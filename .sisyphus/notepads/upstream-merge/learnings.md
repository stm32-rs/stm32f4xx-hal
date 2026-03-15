## Task 6 Learnings — defmt-optional examples (2026-02-27)

### Key Pattern: defmt-optional in embedded examples

When making defmt optional in embedded examples under `#![deny(warnings)]`:

1. **Panic handler selection** — use cfg-gated `use` statements:
   ```rust
   #[cfg(feature = "defmt")] use defmt_rtt as _;
   #[cfg(feature = "defmt")] use panic_probe as _;
   #[cfg(not(feature = "defmt"))] use panic_halt as _;
   ```

2. **Variables used only in defmt macros** — suppress unused-variable lint:
   ```rust
   for attempt in 0..MAX {
       #[cfg(not(feature = "defmt"))]
       let _ = attempt;
   }
   ```

3. **defmt::panic! vs panic!** — requires dual cfg blocks (NOT `else`):
   ```rust
   Err(e) => {
       #[cfg(not(feature = "defmt"))]
       let _ = e;  // suppress unused when defmt is off
       #[cfg(feature = "defmt")]
       defmt::panic!("msg: {:?}", e);
       #[cfg(not(feature = "defmt"))]
       panic!("msg");
   }
   ```

4. **cfg-gated match arms** — Rust requires all cfg-gated patterns to be in a single
   match block; they cannot be duplicated across cfg blocks. Use cfg inside arm body
   instead of cfg-gating the entire arm when possible.

### Pitfall: Brace corruption from editing

When inserting cfg-gated blocks around existing code via edit tool, be careful of:
- Duplicate closing braces (orphaned from the original `else` arm)
- Duplicate arm bodies when cfg-gating a single match arm that previously had both
  logging and logic in it
- Missing closing braces for match/for-loop when indentation changes during editing

Always run `cargo check` after each structural edit to catch brace issues early,
rather than batching all edits and checking at the end.

### lcd-test was the most complex

`f469disco-lcd-test.rs` had 4 separate locations where brace structure was corrupted:
1. Line ~350: duplicate `delay.delay_us` + orphan `}` in `get_touch` Err arm
2. Line ~305: duplicate `delay.delay_us` + orphan `}` in `detect_touch` Err arm  
3. Line ~370: duplicate Some(point) arm body (orphaned from prior cfg edit)
4. Line ~463: missing close brace for `ProbeMismatch` arm
5. Line ~241: Otm8009a match arm closing brace at wrong indent (missing match-close brace)

## Task 6 Part 2 Learnings — dead_code & clippy fixes (2026-02-27)

### Per-item dead_code annotations in shared example modules

When replacing blanket `#![allow(dead_code)]` with per-item annotations:

1. **Shared modules included via `#[path = "..."] mod foo;`** are compiled separately
   for each example. Items only used internally (called by other module functions)
   are NOT dead code — the compiler tracks internal use.

2. **Enum variants that are matched but never constructed** ARE flagged as dead_code.
   E.g., `BoardHint::NewRevisionLikely` was used in `detect_lcd_controller`'s match
   arms but never constructed by any example — only `Unknown` was used. Solution:
   `#[allow(dead_code)]` on the enum.

3. **Public helper functions not called by any example** need per-item annotation.
   Four helper functions in board.rs (`init_dsi_with_delay`, `init_panel`,
   `init_ltdc_rgb565`, `init_ltdc_argb8888`) exist for API completeness but aren't
   called — they each got `#[allow(dead_code)]`.

4. **Items used as return types or in internal code paths** are NOT dead code even if
   the caller discards the value (e.g., `_controller`).

### clippy::new_without_default for embedded drivers

Embedded device drivers often have `new()` that initializes to a "not yet configured"
state. Adding a `Default` impl would be misleading — the driver isn't meaningfully
"default" until `.init()` is called. Use `#[allow(clippy::new_without_default)]`
with a comment explaining the rationale.

### defmt as dev-dependency

`defmt` is listed as a non-optional `[dev-dependencies]`, meaning it's always available
in example/test builds. Bare `defmt::info!()` calls in example code compile without
the `defmt` feature flag. The feature flag only controls `defmt_rtt` (transport),
`panic_probe`, and `defmt::Format` derives on library types.

## Task 8 Learnings - PR1 core DSI/LTDC split (2026-02-27)

- For branch extraction from a messy feature branch, `git checkout <source-branch> -- <file>` gives a clean file-level transplant without pulling commit history.
- For mixed-scope `Cargo.toml` changes, the safest path is: checkout full file from source branch, then explicitly restore/remove out-of-scope `[[example]]` entries back to upstream values.
- Scope verification should use both `git diff --stat upstream/master..branch` and `git diff --name-only upstream/master..branch`; this quickly catches accidental inclusions like `examples/` or `src/display/`.

## Task 8 Follow-up Verification - PR1 branch readiness (2026-02-27)

- `pr1-core-dsi-ltdc` already existed and is correctly based on `upstream/master` commit `59cbcac` (merge-base matches exactly).
- Final PR1 scope is clean: only `CHANGELOG.md`, `Cargo.toml`, `src/dsi.rs`, and `src/ltdc.rs` differ from `upstream/master`.
- Guard checks to prevent PR2 leakage passed: no `src/display/` diff, no `examples/f469disco*` diff, and no forbidden example entries in `Cargo.toml`.
- Required build check passes for upstream split target: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --target thumbv7em-none-eabihf`.

## Task 9 Learnings - PR2 examples branch prep (2026-02-27)

- Safest transplant flow for branch-split PRs: create target branch from prior PR head, then checkout only scoped files from source feature branch.
- For `Cargo.toml` in split PRs, manual merge of `[[example]]` entries avoids clobbering prior PR feature/dependency cleanup.
- Scope guards that quickly catch leakage: `git diff --name-only <base>`, missing-path check (`ls src/display/` should fail), and explicit file-count checks for expected example sets.
- Required examples build target with `--examples` validated PR2 scope while tolerating the known six pre-existing upstream warnings.
