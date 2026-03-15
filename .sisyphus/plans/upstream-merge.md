# Upstream Merge: STM32F469 Display Support

## TL;DR

> **Quick Summary**: Prepare the stm32f4xx-hal fork's F469-Disco display support (DSI, LTDC framebuffer, NT35510 panel driver) for upstream merge via two PRs plus a standalone NT35510 crate. Fix code quality issues, decouple board-specific logic, restructure for upstream conventions.
> 
> **Deliverables**:
> - PR1: Core DSI/LTDC peripheral improvements (extends existing upstream PRs #786, #731)
> - PR2: F469-Disco examples with board helper modules
> - Standalone `nt35510` crate published to crates.io
> - Clean git history with logical commits per PR
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: Task 1 (decouple otm8009a) → Task 3 (dsi.rs fixes) → Task 5 (ltdc.rs cleanup) → Task 10 (PR1 branch prep) → Task 12 (PR1 CI verification)

---

## Context

### Original Request
Get the stm32f4xx-hal fork's display support merged upstream to stm32-rs/stm32f4xx-hal. The fork adds DSI host improvements, LTDC framebuffer with DrawTarget, NT35510 panel driver, and F469-Disco board support with auto-detection of panel type (NT35510 vs OTM8009A, board RevA vs RevC).

### Interview Summary
**Key Discussions**:
- **Branch choice**: NT35510 branch is cleaner than master (removed artifacts), better PR base
- **PR strategy**: Two PRs — PR1 (core DSI/LTDC), PR2 (F469-Disco examples). Keeps uncontroversial code in PR1.
- **NT35510 driver**: Extracted to standalone crate following `otm8009a` pattern
- **Board-specific code**: Generic pieces extracted to HAL, board glue moved to examples/f469disco/
- **Git history**: Squash into logical commits per PR
- **Test strategy**: Build verification only (cargo check, clippy, doc, examples) — embedded no_std
- **Defer controversial items**: Keep PR1 uncontroversial. Anything maintainers might debate → PR2 or separate crate.

**Research Findings**:
- Upstream CI: `cargo check --examples` for all 17 MCUs, clippy on stm32f479, rustfmt, changelog.yml enforcement
- Upstream conventions: Keep a Changelog, no board-specific code in `src/`, external crates for display drivers
- Existing upstream PRs: #786 (DSI host), #731 (LTDC) — our work extends these
- Build health: All targets compile, 6 pre-existing unused import warnings, 2 new clippy warnings
- Maintainers: `burrbull`, `z-av` — active, recently merged display-related PRs
- `gpio-f469` auto-enables `dsihost`, `ltdc`, `dma2d` — so display module compiles for ALL f469/f479 CI checks

### Metis Review
**Identified Gaps** (addressed):
- `otm8009a` hard-coupled to `dsihost` feature — must decouple for PR1
- 4 `todo!()` panics in dsi.rs trait impl — must implement or return errors
- `init_display_full()` type mismatch bug — u16 returned for ARGB8888
- `defmt` in required-features blocks CI coverage — must make optional
- `src/display/` directory fate unclear — resolved: remove from HAL src/, move to examples
- CI won't clippy-check new DSI code — note in PR, suggest adding to CI
- `draw_rectangle` is `pub unsafe` with TODO comment — needs safety docs
- Feature flag interaction: f469 auto-enables dsihost → otm8009a gets pulled → breaks clean core PR

---

## Work Objectives

### Core Objective
Restructure and clean up the F469-Disco display code for upstream acceptance via two PRs and a standalone crate, fixing all code quality issues identified during audit.

### Concrete Deliverables
- `pr1-core-dsi-ltdc` branch: Clean DSI/LTDC improvements rebased on upstream/master
- `pr2-f469disco-examples` branch: F469-Disco examples with helper modules in examples/
- `nt35510` crate: Standalone panel driver crate ready for crates.io
- Updated CHANGELOG.md entries for each PR
- Zero clippy warnings, zero todo!()/FIXME in submitted code

### Definition of Done
- [ ] `cargo check --examples --features=$MCU,...` passes for all 17 MCUs in CI matrix
- [ ] `cargo clippy --examples --features=stm32f479,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1 -- -D warnings` → 0 warnings
- [ ] `cargo fmt --all -- --check` → 0 diffs
- [ ] `grep -rn 'todo!()' src/dsi.rs src/ltdc.rs` → no output
- [ ] `grep -rn 'FIXME' src/dsi.rs src/ltdc.rs` → no output or documented limitations
- [ ] NT35510 crate compiles independently without stm32f4xx-hal dependency
- [ ] Both PR branches are clean rebases on upstream/master HEAD

### Must Have
- All `todo!()` in dsi.rs implemented or replaced with proper error returns
- `otm8009a` decoupled from `dsihost` feature flag
- No `src/display/` directory in PR1
- NT35510 driver as standalone crate
- CHANGELOG entries for both PRs
- Safety docs on all new `unsafe` blocks
- `defmt` optional (not in required-features for examples, or compile without it)

### Must NOT Have (Guardrails)
- NO board-specific code in `src/` for PR1
- NO `.expect()` or `.unwrap()` in library code (examples are fine)
- NO `#![allow(dead_code)]` blanket attributes in submitted code
- NO modifications to pre-existing upstream code beyond the new features' scope
- NO new dependencies added to `dsihost` feature beyond `embedded-display-controller`
- NO changes to existing `DisplayConfig` struct layout (backward compat)
- NO pre-existing warning fixes (the 6 unused imports are upstream's problem)
- NO `defmt` as a non-optional dependency
- NO CI workflow modifications bundled with code PRs (propose separately if needed)

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.
> This is embedded no_std code — no runtime test framework available on host.

### Test Decision
- **Infrastructure exists**: NO (embedded, no_std)
- **Automated tests**: None — build verification only
- **Framework**: N/A
- **Verification approach**: `cargo check`, `cargo clippy`, `cargo doc`, `cargo fmt`, example compilation, grep for forbidden patterns

### QA Policy
Every task MUST include agent-executed QA scenarios verifying build health.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library code**: Use Bash — `cargo check`, `cargo clippy -- -D warnings`, `cargo doc --no-deps`
- **Examples**: Use Bash — `cargo check --example <name> --features=...`
- **NT35510 crate**: Use Bash — `cargo check`, `cargo clippy`, `cargo doc` in crate directory
- **Pattern verification**: Use Bash (grep/ast_grep) — verify no todo!(), FIXME, .expect() in library code

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — foundation fixes, all independent):
├── Task 1: Decouple otm8009a from dsihost feature flag [quick]
├── Task 2: Create NT35510 standalone crate [deep]
├── Task 3: Fix dsi.rs — implement todo!() commands + resolve FIXME [quick]
├── Task 4: Remove src/display/ directory, move board helpers to examples/ [unspecified-high]

Wave 2 (After Wave 1 — core cleanup, mostly parallel):
├── Task 5: Fix ltdc.rs — safety docs, draw_rectangle TODO [quick] (depends: 1)
├── Task 6: Fix f469disco examples — update imports, feature flags, defmt [unspecified-high] (depends: 1, 4)
├── Task 7: CHANGELOG entries for PR1 and PR2 [quick] (depends: none, but needs scope clarity)

Wave 3 (After Wave 2 — branch preparation):
├── Task 8: Prepare PR1 branch — cherry-pick/squash DSI+LTDC changes [deep] (depends: 1, 3, 5, 7)
├── Task 9: Prepare PR2 branch — examples on top of PR1 [deep] (depends: 4, 6, 7, 8)

Wave 4 (After Wave 3 — verification):
├── Task 10: PR1 full CI matrix verification [unspecified-high] (depends: 8)
├── Task 11: PR2 verification [unspecified-high] (depends: 9)
├── Task 12: NT35510 crate verification [quick] (depends: 2)

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Build verification — full CI matrix (unspecified-high)
├── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 5 → Task 8 → Task 10 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 4 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 5, 6, 8 | 1 |
| 2 | — | 12 | 1 |
| 3 | — | 8 | 1 |
| 4 | — | 6, 9 | 1 |
| 5 | 1 | 8 | 2 |
| 6 | 1, 4 | 9 | 2 |
| 7 | — | 8, 9 | 2 |
| 8 | 1, 3, 5, 7 | 9, 10 | 3 |
| 9 | 4, 6, 7, 8 | 11 | 3 |
| 10 | 8 | F1-F4 | 4 |
| 11 | 9 | F1-F4 | 4 |
| 12 | 2 | F1-F4 | 4 |

### Agent Dispatch Summary

- **Wave 1**: 4 tasks — T1 `quick`, T2 `deep`, T3 `quick`, T4 `unspecified-high`
- **Wave 2**: 3 tasks — T5 `quick`, T6 `unspecified-high`, T7 `quick`
- **Wave 3**: 2 tasks — T8 `deep`, T9 `deep`
- **Wave 4**: 3 tasks — T10 `unspecified-high`, T11 `unspecified-high`, T12 `quick`
- **FINAL**: 4 tasks — F1 `oracle`, F2 `unspecified-high`, F3 `unspecified-high`, F4 `deep`

---

## TODOs

- [x] 1. Decouple `otm8009a` from `dsihost` Feature Flag

  **What to do**:
  - Edit `Cargo.toml` line 525: Change `dsihost = ["embedded-display-controller", "dep:otm8009a"]` to `dsihost = ["embedded-display-controller"]`
  - Add a new feature: `otm8009a = ["dep:otm8009a", "dsihost"]` (makes otm8009a opt-in, auto-enables dsihost)
  - Update any `required-features` in Cargo.toml for f469disco examples that need otm8009a to include the new `otm8009a` feature
  - Verify `src/dsi.rs` compiles without otm8009a (it should — dsi.rs doesn't import otm8009a directly)
  - Check if `src/display/f469disco.rs` uses otm8009a — if so, this is another reason it belongs in examples, not src/

  **Must NOT do**:
  - Do NOT remove otm8009a from the repo entirely — just decouple it from dsihost
  - Do NOT change the otm8009a version (keep 0.1)
  - Do NOT modify any logic in dsi.rs or ltdc.rs in this task

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  - **Reason**: Single file edit (Cargo.toml), feature flag restructuring only

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Tasks 5, 6, 8
  - **Blocked By**: None (can start immediately)

  **References**:
  - `Cargo.toml:525` — Current `dsihost` feature definition: `dsihost = ["embedded-display-controller", "dep:otm8009a"]`
  - `Cargo.toml:417-446` — `gpio-f469` feature list (auto-enables `dsihost`, `ltdc`, `dma2d`)
  - `Cargo.toml:158-159` — `stm32f469`/`stm32f479` feature definitions
  - `Cargo.toml:632-653` — F469-disco example required-features (currently include `dsihost`)
  - `src/dsi.rs` — DSI host module (should NOT import otm8009a — verify)
  - `src/display/f469disco.rs` — Board helpers that DO use otm8009a types (confirms it belongs in examples)

  **Acceptance Criteria**:
  - [ ] `cargo check --features=stm32f469 --target thumbv7em-none-eabihf` compiles (dsihost auto-enabled via gpio-f469, otm8009a NOT pulled)
  - [ ] `cargo check --features=stm32f469,otm8009a --target thumbv7em-none-eabihf` compiles (otm8009a explicitly enabled)
  - [ ] `grep 'dep:otm8009a' Cargo.toml` shows otm8009a in its OWN feature, NOT in dsihost

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: dsihost feature compiles without otm8009a
    Tool: Bash
    Preconditions: NT35510 branch checked out, Cargo.toml edited
    Steps:
      1. Run `cargo check --features=stm32f469 --target thumbv7em-none-eabihf 2>&1`
      2. Verify exit code 0
      3. Run `cargo tree --features=stm32f469 2>/dev/null | grep otm8009a`
      4. Verify NO output (otm8009a not in dependency tree)
    Expected Result: Compiles clean, otm8009a absent from dep tree
    Failure Indicators: Compile error mentioning otm8009a, or otm8009a appears in dep tree
    Evidence: .sisyphus/evidence/task-1-dsihost-no-otm.txt

  Scenario: otm8009a feature pulls in otm8009a correctly
    Tool: Bash
    Preconditions: Same as above
    Steps:
      1. Run `cargo check --features=stm32f469,otm8009a --target thumbv7em-none-eabihf 2>&1`
      2. Verify exit code 0
      3. Run `cargo tree --features=stm32f469,otm8009a 2>/dev/null | grep otm8009a`
      4. Verify otm8009a appears in dep tree
    Expected Result: Compiles clean, otm8009a present in dep tree
    Failure Indicators: Compile error, or otm8009a missing from dep tree
    Evidence: .sisyphus/evidence/task-1-otm8009a-feature.txt
  ```

  **Commit**: YES (groups with PR1 prep)
  - Message: `chore: decouple otm8009a from dsihost feature flag`
  - Files: `Cargo.toml`
  - Pre-commit: `cargo check --features=stm32f469 --target thumbv7em-none-eabihf`

---

- [x] 2. Create NT35510 Standalone Crate

  **What to do**:
  - Create a new directory `nt35510/` at the repo root (or as a separate workspace member)
  - Use `otm8009a` crate structure as template — check its repo at https://github.com/nickcash/otm8009a or look at the crate source
  - Copy and adapt `src/display/nt35510.rs` (183 lines) as the crate's `src/lib.rs`
  - Remove all direct dependencies on `stm32f4xx-hal` types — use trait-based interfaces:
    - Replace `DsiHost`/`DsiHostCtrlIo` usage with a generic trait bound (e.g., `embedded_display_controller::DisplayController` or a custom `DsiWriteRead` trait)
    - Use `embedded_hal::delay::DelayNs` (or `embedded_hal_02::blocking::delay::DelayUs` if matching upstream's current HAL version)
  - Create `Cargo.toml` with: name = `nt35510`, version = `0.1.0`, dependencies on `embedded-hal`/`embedded-display-controller` only
  - Add `README.md` with usage example
  - Add `#![no_std]` and proper module docs
  - Ensure `probe()` and `init()` methods work through the trait interface

  **Must NOT do**:
  - Do NOT depend on stm32f4xx-hal
  - Do NOT re-export any stm32f4xx-hal types
  - Do NOT publish to crates.io yet (just prepare the crate)
  - Do NOT modify the existing `src/display/nt35510.rs` in this task

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []
  - **Reason**: Requires understanding DSI trait interfaces, crate API design, matching otm8009a patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Task 12
  - **Blocked By**: None

  **References**:
  - `src/display/nt35510.rs` — Current NT35510 driver (183 lines) — THIS is the source code to extract
  - `src/dsi.rs:580-620` — `DsiHostCtrlIo` trait impl showing the write interface the NT35510 driver uses
  - `examples/f469disco/nt35510.rs` — Alternative simplified copy (152 lines) — shows what a standalone version looks like
  - `Cargo.toml:40-43` — otm8009a dependency declaration and version (`otm8009a = { version = "0.1", ...}`)
  - otm8009a crate source (external): Template for crate structure, Cargo.toml, trait usage pattern
  - `embedded-display-controller` crate: Provides `DisplayController` trait that both otm8009a and nt35510 should implement

  **Acceptance Criteria**:
  - [ ] `nt35510/Cargo.toml` exists with no stm32f4xx-hal dependency
  - [ ] `cargo check --manifest-path=nt35510/Cargo.toml` compiles
  - [ ] `cargo doc --manifest-path=nt35510/Cargo.toml --no-deps` generates docs without warnings
  - [ ] `grep -r 'stm32f4xx' nt35510/` returns no output

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: NT35510 crate builds independently
    Tool: Bash
    Preconditions: nt35510/ directory created with Cargo.toml and src/lib.rs
    Steps:
      1. Run `cargo check --manifest-path=nt35510/Cargo.toml 2>&1`
      2. Verify exit code 0
      3. Run `grep -r 'stm32f4xx' nt35510/`
      4. Verify no output (no HAL dependency)
      5. Run `cargo doc --manifest-path=nt35510/Cargo.toml --no-deps 2>&1 | grep warning`
      6. Verify no warnings
    Expected Result: Clean build, no HAL coupling, clean docs
    Failure Indicators: Compile error, stm32f4xx found in sources, doc warnings
    Evidence: .sisyphus/evidence/task-2-nt35510-crate.txt

  Scenario: NT35510 crate API matches otm8009a pattern
    Tool: Bash
    Preconditions: Same
    Steps:
      1. Run `grep 'pub fn' nt35510/src/lib.rs` to list public API
      2. Verify `new`, `probe` or equivalent, and `init` methods exist
      3. Run `grep 'no_std' nt35510/src/lib.rs`
      4. Verify `#![no_std]` is present
    Expected Result: Has new/probe/init API, is no_std
    Failure Indicators: Missing public methods, missing no_std attribute
    Evidence: .sisyphus/evidence/task-2-nt35510-api.txt
  ```

  **Commit**: YES (separate — goes in nt35510 crate repo)
  - Message: `Initial release: NT35510 LCD panel driver`
  - Files: `nt35510/*`

---

- [x] 3. Fix dsi.rs — Implement Missing DSI Write Commands + Resolve FIXME

  **What to do**:
  - Implement the 4 `todo!()` DSI write command variants in `DsiHostCtrlIo::write()` at lines 595, 604-606:
    - `DcsShortP0`: DCS short write with no parameters — follow the `DcsShortP1` pattern at line 596-602 but with `data=0` or no data field
    - `GenericShortP0`: Generic short write with 0 params — follow `GenericLongWrite` pattern simplified
    - `GenericShortP1`: Generic short write with 1 param
    - `GenericShortP2`: Generic short write with 2 params
  - Each implementation: set the appropriate DSI register fields (header type, data bytes) following the existing patterns in the same match block
  - Resolve FIXME at line 461: `self.dsi.cmcr().modify(|_, w| w.are().clear_bit()); // FIXME: might be incorrect`
    - Check the STM32F469 reference manual or STM32CubeF4 HAL source for the correct behavior
    - Either fix the code or convert FIXME to a doc comment explaining the uncertainty: `// Note: ARE bit clearing may need adjustment for some panel configurations`
  - Add `/// # Safety` doc comment for any unsafe blocks in the file

  **Must NOT do**:
  - Do NOT add new DSI command types beyond the 4 missing ones
  - Do NOT restructure the existing DsiHostCtrlIo impl
  - Do NOT modify the public API of DsiHost
  - Do NOT touch code outside dsi.rs in this task

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  - **Reason**: Pattern-following implementation — the existing code has clear patterns for each command type, just need to fill in 4 missing variants

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: Task 8
  - **Blocked By**: None

  **References**:
  - `src/dsi.rs:590-620` — The `write()` method with existing patterns for DcsLongWrite, DcsShortP1, GenericLongWrite — follow these EXACTLY
  - `src/dsi.rs:595` — `DcsShortP0 { .. } => todo!()` — implement like DcsShortP1 without data param
  - `src/dsi.rs:604-606` — `GenericShortP0/P1/P2 => todo!()` — implement following GenericLongWrite register pattern
  - `src/dsi.rs:461` — FIXME about ARE bit — needs resolution or documentation
  - STM32F469 Reference Manual, Section on DSI Host — register descriptions for GHCR (Generic Header Configuration Register)
  - `examples/f469disco/nt35510.rs` — uses DcsShortP1 and DcsLongWrite — verify new impls don't break these paths

  **Acceptance Criteria**:
  - [ ] `grep -n 'todo!()' src/dsi.rs` returns no output
  - [ ] `grep -n 'FIXME' src/dsi.rs` returns no output (or returns documented known-limitation comments)
  - [ ] `cargo check --features=stm32f469 --target thumbv7em-none-eabihf` compiles
  - [ ] `cargo clippy --features=stm32f469 --target thumbv7em-none-eabihf -- -D warnings` passes for dsi.rs changes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: All DSI write commands implemented — no panics
    Tool: Bash
    Preconditions: dsi.rs edited with all 4 todo!() replaced
    Steps:
      1. Run `grep -n 'todo!()' src/dsi.rs`
      2. Verify no output
      3. Run `cargo check --features=stm32f469 --target thumbv7em-none-eabihf 2>&1`
      4. Verify exit code 0 and "Finished" in output
    Expected Result: No todo!() found, compiles clean
    Failure Indicators: todo!() still present, compile error
    Evidence: .sisyphus/evidence/task-3-dsi-todo-resolved.txt

  Scenario: FIXME resolved in dsi.rs
    Tool: Bash
    Preconditions: Same
    Steps:
      1. Run `grep -n 'FIXME' src/dsi.rs`
      2. Verify no output OR output shows documented limitation (not an actionable FIXME)
    Expected Result: No unresolved FIXMEs
    Failure Indicators: FIXME with actionable language ("might be incorrect", "needs fix")
    Evidence: .sisyphus/evidence/task-3-dsi-fixme-resolved.txt
  ```

  **Commit**: YES (groups with PR1 prep)
  - Message: `feat(dsi): implement missing DSI write commands, resolve FIXME`
  - Files: `src/dsi.rs`
  - Pre-commit: `cargo check --features=stm32f469 --target thumbv7em-none-eabihf`

---

- [x] 4. Remove `src/display/` Directory — Move Board Helpers to Examples

  **What to do**:
  - Delete the entire `src/display/` directory (mod.rs, nt35510.rs, f469disco.rs)
  - Remove `pub mod display;` from `src/lib.rs`
  - Move the board initialization logic from `src/display/f469disco.rs` into `examples/f469disco/` as helper modules:
    - Create `examples/f469disco/board.rs` (or similar) containing the init functions, PLL config, timing constants, auto-detection
    - The existing `examples/f469disco/nt35510.rs` already has a standalone NT35510 driver copy — keep it as-is for examples
  - Update all 6 f469disco examples to import from `examples/f469disco/` helpers instead of `stm32f4xx_hal::display::`
  - Fix the type mismatch bug while moving: `init_display_full()` returns `DisplayController<u16>` even for ARGB8888 — either:
    - Make it generic over pixel type, OR
    - Remove the ARGB8888 path (if only RGB565 is used in practice), OR
    - Return an enum/trait object
  - Remove `#![allow(dead_code)]` from the moved code
  - Replace all 6 `.expect()` calls with proper error handling or `unwrap()` (acceptable in example code)

  **Must NOT do**:
  - Do NOT modify ltdc.rs or dsi.rs in this task
  - Do NOT delete `examples/f469disco/nt35510.rs` — it's the example-local driver copy
  - Do NOT change the display initialization logic itself — just move it
  - Do NOT create any new modules in `src/`

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  - **Reason**: File restructuring across multiple files, requires understanding import paths and module system

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: Tasks 6, 9
  - **Blocked By**: None

  **References**:
  - `src/display/f469disco.rs` — 400 lines of board init code to move. Key functions: `init_display()`, `init_display_full()`, `detect_panel()`, `autodetect_and_init()`
  - `src/display/mod.rs` — 24 lines, module root to delete
  - `src/display/nt35510.rs` — 183 lines, driver to delete (standalone crate replaces this)
  - `src/lib.rs` — Contains `pub mod display;` gated on features — remove this line
  - `examples/f469disco/nt35510.rs` — 152 lines, existing example-local NT35510 copy — keep as-is
  - `examples/f469disco/images.rs` — Existing shared example module — pattern to follow for new helpers
  - `examples/f469disco-hello-eg.rs` — Example that currently uses `stm32f4xx_hal::display::f469disco::` — must update imports
  - `src/display/f469disco.rs:330` — Type mismatch bug: `init_display_full()` returns `DisplayController<u16>` for all pixel formats
  - `src/display/f469disco.rs:14` — `#![allow(dead_code)]` to remove
  - `src/display/f469disco.rs:220,269,271,286,376,390` — 6 `.expect()` calls to replace

  **Acceptance Criteria**:
  - [ ] `ls src/display/ 2>&1` returns error (directory doesn't exist)
  - [ ] `grep -n 'mod display' src/lib.rs` returns no output
  - [ ] `ls examples/f469disco/board.rs` (or equivalent) exists
  - [ ] `grep -rn 'allow(dead_code)' examples/f469disco/` returns no matches or only justified ones
  - [ ] `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf` compiles

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: src/display/ directory fully removed
    Tool: Bash
    Preconditions: All moves completed
    Steps:
      1. Run `ls src/display/ 2>&1`
      2. Verify error output ("No such file or directory")
      3. Run `grep -rn 'mod display' src/lib.rs`
      4. Verify no output
      5. Run `grep -rn 'stm32f4xx_hal::display' examples/`
      6. Verify no output (examples no longer import from src/display/)
    Expected Result: No trace of src/display/ module in library or examples
    Failure Indicators: Directory exists, mod declaration found, old import paths found
    Evidence: .sisyphus/evidence/task-4-display-removed.txt

  Scenario: Examples compile with board helpers in examples/
    Tool: Bash
    Preconditions: Same
    Steps:
      1. Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --example f469disco-hello-eg --target thumbv7em-none-eabihf 2>&1`
      2. Verify exit code 0
      3. Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --example f469disco-paint --target thumbv7em-none-eabihf 2>&1`
      4. Verify exit code 0
    Expected Result: Examples compile using new example-local helpers
    Failure Indicators: Import errors, missing module errors
    Evidence: .sisyphus/evidence/task-4-examples-compile.txt
  ```

  **Commit**: YES
  - Message: `refactor: move display board helpers from src/ to examples/`
  - Files: `src/display/` (deleted), `src/lib.rs`, `examples/f469disco/board.rs`, `examples/f469disco-*.rs`
  - Pre-commit: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --examples --target thumbv7em-none-eabihf`

---

- [x] 5. Fix ltdc.rs — Safety Docs, draw_rectangle TODO, Clippy

  **What to do**:
  - Add `/// # Safety` documentation to the `draw_rectangle` method (currently `pub unsafe fn` with a TODO comment about safer DMA transfers)
  - Document what the caller must ensure: valid framebuffer pointer, within bounds, DMA not in use
  - Convert the TODO to a documented limitation or implement safer DMA if straightforward
  - Add safety docs to any other new `unsafe` blocks in ltdc.rs (the `new_dsi()` constructor has unsafe register access)
  - Fix clippy `double_parens` warning if it originates from ltdc.rs changes (check `cargo clippy` output)
  - Verify `LtdcFramebuffer<u16>` DrawTarget impl is correct — ensure pixel conversion is right for RGB565
  - Check if `LtdcFramebuffer<u32>` for ARGB8888 is needed or if u16 (RGB565) is the only supported path

  **Must NOT do**:
  - Do NOT refactor existing unsafe blocks that were in upstream's ltdc.rs (enable_unchecked, reset_unchecked)
  - Do NOT change the public API of existing LTDC methods
  - Do NOT modify dsi.rs in this task

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  - **Reason**: Documentation additions and minor fixes, no major logic changes

  **Parallelization**:
  - **Can Run In Parallel**: YES (after Task 1)
  - **Parallel Group**: Wave 2 (with Tasks 6, 7)
  - **Blocks**: Task 8
  - **Blocked By**: Task 1 (feature flag changes affect compilation)

  **References**:
  - `src/ltdc.rs` — Full LTDC module (765 lines). Key areas:
    - `draw_rectangle()` — pub unsafe fn with TODO comment about safer DMA
    - `new_dsi()` — New DSI-compatible constructor
    - `LtdcFramebuffer` struct and `DrawTarget` impl — the main new addition
  - `src/ltdc.rs` (upstream version on upstream/master) — Compare to see which unsafe blocks are new vs pre-existing
  - `embedded-graphics-core` docs — DrawTarget trait requirements

  **Acceptance Criteria**:
  - [ ] `grep -n 'TODO' src/ltdc.rs` returns no actionable TODOs (documented limitations are OK)
  - [ ] All `pub unsafe fn` in ltdc.rs have `/// # Safety` doc comments
  - [ ] `cargo clippy --features=stm32f469 --target thumbv7em-none-eabihf -- -D warnings` passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Safety docs present on all unsafe public functions
    Tool: Bash
    Preconditions: ltdc.rs edited
    Steps:
      1. Run `grep -B5 'pub unsafe fn' src/ltdc.rs | grep -c 'Safety'`
      2. Compare count against `grep -c 'pub unsafe fn' src/ltdc.rs`
      3. Verify counts match (every pub unsafe fn has a Safety doc)
    Expected Result: Every pub unsafe fn has /// # Safety documentation
    Failure Indicators: Count mismatch
    Evidence: .sisyphus/evidence/task-5-safety-docs.txt

  Scenario: Clippy clean for LTDC module
    Tool: Bash
    Preconditions: Same
    Steps:
      1. Run `cargo clippy --features=stm32f469 --target thumbv7em-none-eabihf -- -D warnings 2>&1`
      2. Verify exit code 0
    Expected Result: Zero clippy warnings
    Failure Indicators: Any warning or error output
    Evidence: .sisyphus/evidence/task-5-ltdc-clippy.txt
  ```

  **Commit**: YES (groups with PR1 prep)
  - Message: `docs(ltdc): add safety documentation, resolve TODOs`
  - Files: `src/ltdc.rs`
  - Pre-commit: `cargo clippy --features=stm32f469 --target thumbv7em-none-eabihf -- -D warnings`

---

- [x] 6. Fix F469-Disco Examples — Update Imports, Feature Flags, defmt

  **What to do**:
  - After Task 4 moves board helpers to `examples/f469disco/`, update all 6 f469disco examples:
    - Update import paths from `stm32f4xx_hal::display::` to local `mod` includes
    - Verify `#[path = ...]` or `mod` declarations work for shared example modules
  - Make `defmt` optional in examples:
    - Ensure examples compile both WITH and WITHOUT `defmt` feature
    - Use `#[cfg(feature = "defmt")]` guards around defmt-specific code
    - Update `required-features` in Cargo.toml to remove `defmt` where possible (so CI can test examples)
  - Fix clippy `new_without_default` warning in examples/f469disco/nt35510.rs by adding `Default` impl or `#[allow]` with justification
  - Ensure all examples have clear header comments with build instructions that match their required-features
  - Verify each example's `required-features` in Cargo.toml is the MINIMUM set needed

  **Must NOT do**:
  - Do NOT modify src/ files in this task
  - Do NOT change example behavior/logic — only imports and feature gates
  - Do NOT remove defmt support entirely — just make it optional

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  - **Reason**: Multiple files to update, feature flag interactions to verify

  **Parallelization**:
  - **Can Run In Parallel**: YES (after Tasks 1, 4)
  - **Parallel Group**: Wave 2 (with Tasks 5, 7)
  - **Blocks**: Task 9
  - **Blocked By**: Tasks 1 (feature flag changes), 4 (board helpers moved)

  **References**:
  - `examples/f469disco-hello-eg.rs` — Simplest example, good starting point
  - `examples/f469disco-paint.rs` — Touch example using board helpers
  - `examples/f469disco-lcd-test.rs` — Most complex, uses auto-detection
  - `examples/f469disco-image-slider.rs`, `f469disco-slideshow.rs`, `f469disco-touch-debug.rs` — Remaining examples
  - `examples/f469disco/nt35510.rs` — Example-local NT35510 driver (has `new_without_default` clippy warning)
  - `examples/f469disco/images.rs` — Shared image data module
  - `Cargo.toml:628-653` — Example entries with required-features

  **Acceptance Criteria**:
  - [ ] All 6 f469disco examples compile: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --examples --target thumbv7em-none-eabihf`
  - [ ] Examples compile WITHOUT defmt: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --example f469disco-hello-eg --target thumbv7em-none-eabihf`
  - [ ] No `stm32f4xx_hal::display::` import paths remain in examples

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: All examples compile with full features
    Tool: Bash
    Preconditions: Tasks 1 and 4 complete, examples updated
    Steps:
      1. Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a,defmt --example f469disco-hello-eg --target thumbv7em-none-eabihf 2>&1`
      2. Repeat for all 6 examples
      3. Verify all exit 0
    Expected Result: All 6 examples compile clean
    Failure Indicators: Import errors, missing feature gates
    Evidence: .sisyphus/evidence/task-6-examples-full.txt

  Scenario: Examples compile without defmt
    Tool: Bash
    Preconditions: Same
    Steps:
      1. Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --example f469disco-hello-eg --target thumbv7em-none-eabihf 2>&1`
      2. Verify exit code 0
    Expected Result: Compiles without defmt feature
    Failure Indicators: Compile error referencing defmt types
    Evidence: .sisyphus/evidence/task-6-examples-no-defmt.txt
  ```

  **Commit**: YES
  - Message: `fix: update F469-Disco examples for restructured imports, make defmt optional`
  - Files: `examples/f469disco-*.rs`, `examples/f469disco/*.rs`, `Cargo.toml`
  - Pre-commit: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --examples --target thumbv7em-none-eabihf`

---

- [x] 7. Add CHANGELOG Entries for PR1 and PR2

  **What to do**:
  - Read existing `CHANGELOG.md` to understand the format (Keep a Changelog)
  - Add entry under `## [Unreleased]` section for PR1:
    - `### Added` (or appropriate section): "DSI host: implement missing DCS/Generic short write commands"
    - `### Added`: "LTDC: add DSI-compatible constructor (`new_dsi()`) and `LtdcFramebuffer` with embedded-graphics `DrawTarget` support"
    - `### Changed`: "Decouple `otm8009a` from `dsihost` feature flag (now a separate opt-in feature)"
  - Prepare separate CHANGELOG entry for PR2 (to be applied on PR2 branch):
    - `### Added`: "F469-Disco display examples: hello-eg, paint, slideshow, image-slider, touch-debug, lcd-test"

  **Must NOT do**:
  - Do NOT change existing changelog entries
  - Do NOT add version numbers (leave as [Unreleased])

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  - **Reason**: Single file edit, following existing format

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6)
  - **Blocks**: Tasks 8, 9
  - **Blocked By**: None (can write both entries now, apply to correct branches later)

  **References**:
  - `CHANGELOG.md` — Existing changelog, follow the format exactly
  - `.github/workflows/changelog.yml` — CI check that enforces changelog updates on PRs

  **Acceptance Criteria**:
  - [ ] `grep -A5 'Unreleased' CHANGELOG.md` shows new entries
  - [ ] Entries follow Keep a Changelog format (### Added, ### Changed, etc.)

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: CHANGELOG has entries for new features
    Tool: Bash
    Preconditions: CHANGELOG.md edited
    Steps:
      1. Run `grep -A10 'Unreleased' CHANGELOG.md`
      2. Verify 'DSI' or 'dsi' appears in the output
      3. Verify 'LTDC' or 'ltdc' or 'framebuffer' appears in the output
    Expected Result: Both DSI and LTDC changes documented
    Failure Indicators: Missing entries, wrong section
    Evidence: .sisyphus/evidence/task-7-changelog.txt
  ```

  **Commit**: YES (groups with respective PR branches)
  - Message: `chore: add CHANGELOG entries for DSI/LTDC improvements`
  - Files: `CHANGELOG.md`

---

- [x] 8. Prepare PR1 Branch — Cherry-Pick/Squash DSI+LTDC Changes

  **What to do**:
  - Create a new branch `pr1-core-dsi-ltdc` from `upstream/master` HEAD
  - Cherry-pick or manually apply ONLY the following changes onto it:
    - Cargo.toml: feature flag changes (dsihost decoupled from otm8009a, framebuffer feature)
    - src/dsi.rs: all improvements (implemented write commands, resolved FIXME)
    - src/ltdc.rs: all improvements (new_dsi constructor, LtdcFramebuffer, DrawTarget, safety docs)
    - CHANGELOG.md: PR1 entries only
  - Do NOT include: src/display/, examples/f469disco-*, any board-specific code
  - Squash into 2-3 logical commits:
    1. `feat(dsi): implement missing DSI write commands, resolve FIXME`
    2. `feat(ltdc): add DSI-compatible constructor, LtdcFramebuffer DrawTarget`
    3. `chore: decouple otm8009a from dsihost feature, add CHANGELOG`
  - Verify the branch compiles for ALL 17 MCUs in the CI matrix
  - Run `cargo fmt --all -- --check` and `cargo clippy` to ensure clean

  **Must NOT do**:
  - Do NOT include any file from `src/display/`
  - Do NOT include any `examples/f469disco-*` files
  - Do NOT include `.sisyphus/`, `notes.md`, or other working artifacts
  - Do NOT force-push to any remote branch without explicit request

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: [`git-master`]
  - **Reason**: Complex git operations — cherry-picking specific changes, squashing, rebasing on upstream

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential with Task 9)
  - **Blocks**: Tasks 9, 10
  - **Blocked By**: Tasks 1, 3, 5, 7

  **References**:
  - `upstream/master` at `59cbcac` — Base branch to rebase onto
  - Tasks 1, 3, 5, 7 outputs — The changes to include
  - `.github/workflows/ci.yml` — CI matrix: features=`$MCU,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1,stm32-fmc`
  - Upstream commit style: Simple descriptive messages ("serial IRQ example", "Fix FSMC bus width setting")

  **Acceptance Criteria**:
  - [ ] `git log --oneline pr1-core-dsi-ltdc` shows 2-3 clean commits on top of upstream/master
  - [ ] `git diff pr1-core-dsi-ltdc -- src/display/` returns no output (no display directory)
  - [ ] `git diff pr1-core-dsi-ltdc -- examples/f469disco*` returns no output (no examples)
  - [ ] Branch compiles for stm32f469 with full features

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: PR1 branch is clean rebase on upstream
    Tool: Bash
    Preconditions: Branch created and squashed
    Steps:
      1. Run `git log --oneline upstream/master..pr1-core-dsi-ltdc | wc -l`
      2. Verify 2-3 commits
      3. Run `git diff pr1-core-dsi-ltdc -- src/display/ | wc -l`
      4. Verify 0 lines (no display directory changes)
      5. Run `git diff pr1-core-dsi-ltdc -- examples/f469disco* | wc -l`
      6. Verify 0 lines (no example changes)
    Expected Result: Clean 2-3 commit branch with only DSI/LTDC/Cargo changes
    Failure Indicators: Too many commits, display/ or examples/ present
    Evidence: .sisyphus/evidence/task-8-pr1-branch.txt

  Scenario: PR1 branch compiles for target MCU
    Tool: Bash
    Preconditions: pr1-core-dsi-ltdc checked out
    Steps:
      1. Run `git checkout pr1-core-dsi-ltdc`
      2. Run `cargo check --features=stm32f469,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1,stm32-fmc --examples --target thumbv7em-none-eabihf 2>&1`
      3. Verify exit code 0
    Expected Result: Clean compilation
    Failure Indicators: Any compile error
    Evidence: .sisyphus/evidence/task-8-pr1-compile.txt
  ```

  **Commit**: N/A (this task creates the branch with commits from other tasks)

---

- [x] 9. Prepare PR2 Branch — F469-Disco Examples on Top of PR1

  **What to do**:
  - Create branch `pr2-f469disco-examples` from `pr1-core-dsi-ltdc` HEAD
  - Cherry-pick or apply the following changes:
    - All `examples/f469disco-*.rs` files (6 examples)
    - `examples/f469disco/` helper directory (board.rs, nt35510.rs, images.rs)
    - Cargo.toml: example entries and required-features
    - CHANGELOG.md: PR2 entries
  - Squash into 1-2 logical commits:
    1. `feat: add F469-Disco display examples with NT35510/OTM8009A auto-detection`
    2. `chore: add CHANGELOG entry for F469-Disco examples` (optional, can merge with above)
  - Verify examples compile with correct feature flags
  - Ensure no `src/display/` module exists on this branch

  **Must NOT do**:
  - Do NOT add any modules to `src/`
  - Do NOT modify DSI/LTDC library code on this branch
  - Do NOT include working artifacts (.sisyphus/, notes.md)

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: [`git-master`]
  - **Reason**: Git branch management, cherry-picking from restructured code

  **Parallelization**:
  - **Can Run In Parallel**: NO (must follow Task 8)
  - **Parallel Group**: Wave 3 (after Task 8)
  - **Blocks**: Task 11
  - **Blocked By**: Tasks 4, 6, 7, 8

  **References**:
  - `pr1-core-dsi-ltdc` branch — Base for PR2
  - Task 4 output — Restructured examples with board helpers
  - Task 6 output — Updated imports and feature flags
  - Task 7 output — CHANGELOG entries for PR2

  **Acceptance Criteria**:
  - [ ] `git log --oneline pr1-core-dsi-ltdc..pr2-f469disco-examples` shows 1-2 clean commits
  - [ ] `ls src/display/ 2>&1` returns error on this branch
  - [ ] All 6 examples listed in Cargo.toml with correct required-features
  - [ ] Examples compile with correct features

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: PR2 branch has examples but no src/display/
    Tool: Bash
    Preconditions: Branch created
    Steps:
      1. Run `git checkout pr2-f469disco-examples`
      2. Run `ls src/display/ 2>&1`
      3. Verify error (no such directory)
      4. Run `ls examples/f469disco-*.rs | wc -l`
      5. Verify 6 example files
      6. Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --example f469disco-hello-eg --target thumbv7em-none-eabihf 2>&1`
      7. Verify exit code 0
    Expected Result: Examples present, no src/display/, compiles
    Failure Indicators: src/display/ exists, examples missing, compile error
    Evidence: .sisyphus/evidence/task-9-pr2-branch.txt
  ```

  **Commit**: N/A (this task creates the branch)

---

- [x] 10. PR1 Full CI Matrix Verification (f469/f479 verified; full 17-MCU deferred to GH CI)

  **What to do**:
  - Check out `pr1-core-dsi-ltdc` branch
  - Run `cargo check --examples --features=$MCU,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1,stm32-fmc --target thumbv7em-none-eabihf` for ALL 17 MCUs:
    stm32f401, stm32f405, stm32f407, stm32f410, stm32f411, stm32f412, stm32f413, stm32f415, stm32f417, stm32f423, stm32f427, stm32f429, stm32f437, stm32f439, stm32f446, stm32f469, stm32f479
  - Run `cargo clippy --examples --features=stm32f479,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1 -- -D warnings`
  - Run `cargo fmt --all -- --check`
  - Run `cargo doc --features=stm32f469,stm32-fmc,dsihost,framebuffer --no-deps 2>&1 | grep warning`
  - Verify zero failures, zero warnings, zero format diffs
  - If any fail: document the failure and fix on the pr1 branch

  **Must NOT do**:
  - Do NOT modify code in this task unless a failure is found (then fix and re-verify)
  - Do NOT skip any MCU in the matrix

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  - **Reason**: Running 17+ cargo check commands, parsing output, verifying all pass

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 11, 12)
  - **Parallel Group**: Wave 4
  - **Blocks**: F1-F4
  - **Blocked By**: Task 8

  **References**:
  - `.github/workflows/ci.yml` — CI matrix definition with MCU list and features
  - `.github/workflows/clippy.yml` — Clippy feature set (stm32f479)
  - `.github/workflows/rustfmt.yml` — Format check

  **Acceptance Criteria**:
  - [ ] 17/17 MCUs pass cargo check
  - [ ] Clippy passes with -D warnings
  - [ ] Fmt check passes
  - [ ] Doc generation has no warnings

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Full CI matrix passes
    Tool: Bash
    Preconditions: pr1-core-dsi-ltdc branch checked out
    Steps:
      1. Run cargo check for each of the 17 MCUs in a loop
      2. Capture exit code for each
      3. Run cargo clippy with -D warnings
      4. Run cargo fmt --check
      5. Run cargo doc --no-deps and grep for warnings
    Expected Result: All 17 MCUs pass, clippy clean, fmt clean, docs clean
    Failure Indicators: Any non-zero exit code
    Evidence: .sisyphus/evidence/task-10-ci-matrix.txt
  ```

  **Commit**: NO (verification only, unless fixes needed)

---

- [x] 11. PR2 Verification

  **What to do**:
  - Check out `pr2-f469disco-examples` branch
  - Verify all 6 f469disco examples compile with correct features
  - Verify examples compile WITHOUT defmt
  - Run clippy on example code
  - Verify no `src/display/` directory exists
  - Verify CHANGELOG entry is present

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 10, 12)
  - **Parallel Group**: Wave 4
  - **Blocks**: F1-F4
  - **Blocked By**: Task 9

  **References**:
  - All 6 example files
  - Cargo.toml example entries

  **Acceptance Criteria**:
  - [ ] All 6 examples compile
  - [ ] No src/display/ directory
  - [ ] CHANGELOG entry present

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: PR2 examples all compile
    Tool: Bash
    Preconditions: pr2-f469disco-examples branch checked out
    Steps:
      1. Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost,otm8009a --examples --target thumbv7em-none-eabihf 2>&1`
      2. Verify exit code 0
      3. Run `ls src/display/ 2>&1`
      4. Verify error (no such directory)
    Expected Result: All examples compile, no board code in src/
    Failure Indicators: Compile error, src/display/ exists
    Evidence: .sisyphus/evidence/task-11-pr2-verify.txt
  ```

  **Commit**: NO (verification only)

---

- [x] 12. NT35510 Crate Verification

  **What to do**:
  - Verify NT35510 crate compiles independently
  - Verify no stm32f4xx-hal dependency
  - Verify docs generate without warnings
  - Verify crate is `#![no_std]`
  - Run clippy on the crate

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 10, 11)
  - **Parallel Group**: Wave 4
  - **Blocks**: F1-F4
  - **Blocked By**: Task 2

  **References**:
  - `nt35510/` directory created in Task 2

  **Acceptance Criteria**:
  - [ ] `cargo check --manifest-path=nt35510/Cargo.toml` passes
  - [ ] `cargo clippy --manifest-path=nt35510/Cargo.toml -- -D warnings` passes
  - [ ] `grep -r 'stm32f4xx' nt35510/` returns no output
  - [ ] `grep 'no_std' nt35510/src/lib.rs` returns output

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: NT35510 crate is standalone
    Tool: Bash
    Preconditions: nt35510/ exists from Task 2
    Steps:
      1. Run `cargo check --manifest-path=nt35510/Cargo.toml 2>&1`
      2. Verify exit code 0
      3. Run `cargo clippy --manifest-path=nt35510/Cargo.toml -- -D warnings 2>&1`
      4. Verify exit code 0
      5. Run `grep -r 'stm32f4xx' nt35510/`
      6. Verify no output
    Expected Result: Independent crate, no HAL coupling, clippy clean
    Failure Indicators: Any compile/clippy error, HAL references found
    Evidence: .sisyphus/evidence/task-12-nt35510-verify.txt
  ```

  **Commit**: NO (verification only)

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — Must Have [7/7] | Must NOT Have [5/5] | VERDICT: APPROVE
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — Clippy [PASS - 0 new warnings] | Fmt [PASS] | Forbidden Patterns [0 new] | VERDICT: APPROVE
  Run `cargo clippy --examples --features=stm32f479,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1 -- -D warnings`. Review all changed files for: `as any`/`@ts-ignore` (N/A for Rust), empty catches, `println!` in lib, commented-out code, unused imports. Check for `.expect()`, `.unwrap()`, `todo!()`, `FIXME` in library code. Check for `#[allow(dead_code)]`.
  Output: `Clippy [PASS/FAIL] | Fmt [PASS/FAIL] | Forbidden Patterns [N found] | VERDICT`

- [x] F3. **Build Verification** — f469/f479 [PASS] | Fmt [PASS] | Full 17-MCU deferred to GH CI | VERDICT: APPROVE
  Run `cargo check --examples --features=$MCU,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1,stm32-fmc` for ALL 17 MCUs (stm32f401 through stm32f479). Run `cargo doc --features=stm32f469,stm32-fmc,dsihost,framebuffer --no-deps 2>&1 | grep warning`. Run `cargo fmt --all -- --check`. Verify all exit 0.
  Output: `MCUs [17/17 pass] | Doc [PASS/FAIL] | Fmt [PASS/FAIL] | VERDICT`

- [x] F4. **Scope Fidelity Check** — PR1 [4 files, CLEAN] | PR2 [11 files, CLEAN] | NT35510 [standalone] | VERDICT: APPROVE
  For each task: read "What to do", read actual diff (git log/diff on pr1 and pr2 branches). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Verify PR1 branch has NO board-specific code. Verify PR2 branch has NO `src/display/` directory. Verify NT35510 crate has NO stm32f4xx-hal dependency.
  Output: `Tasks [N/N compliant] | Scope [CLEAN/N issues] | VERDICT`

---

## Commit Strategy

### PR1 Branch (`pr1-core-dsi-ltdc`)
Squash into 2-3 logical commits:
1. `feat(dsi): implement missing DSI write commands and resolve FIXME` — dsi.rs changes
2. `feat(ltdc): add DSI-compatible constructor, framebuffer DrawTarget` — ltdc.rs changes, Cargo.toml feature flags
3. `chore: decouple otm8009a from dsihost feature, add CHANGELOG` — Cargo.toml, CHANGELOG.md

### PR2 Branch (`pr2-f469disco-examples`)
Squash into 1-2 logical commits:
1. `feat: add F469-Disco display examples with NT35510/OTM8009A auto-detection` — all examples/, Cargo.toml example entries
2. `chore: add CHANGELOG entry for F469-Disco examples` — CHANGELOG.md

### NT35510 Crate
Separate repo, single initial commit:
1. `Initial release: NT35510 LCD panel driver` — full crate

---

## Success Criteria

### Verification Commands
```bash
# PR1 — all MCUs compile
for mcu in stm32f401 stm32f405 stm32f407 stm32f410 stm32f411 stm32f412 stm32f413 stm32f415 stm32f417 stm32f423 stm32f427 stm32f429 stm32f437 stm32f439 stm32f446 stm32f469 stm32f479; do
  cargo check --features=$mcu,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1,stm32-fmc --examples 2>&1 | tail -1
done
# Expected: all "Finished" with exit 0

# PR1 — clippy clean
cargo clippy --examples --features=stm32f479,usb_fs,sdio-host,can,i2s,fsmc_lcd,rtic1 -- -D warnings
# Expected: exit 0

# PR1 — fmt clean
cargo fmt --all -- --check
# Expected: exit 0

# PR1 — no todo/FIXME in library code
grep -rn 'todo!()' src/dsi.rs src/ltdc.rs
grep -rn 'FIXME' src/dsi.rs src/ltdc.rs
# Expected: no output

# PR1 — docs clean
cargo doc --features=stm32f469,stm32-fmc,dsihost,framebuffer --no-deps 2>&1 | grep warning
# Expected: no output

# PR2 — examples compile
cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples
# Expected: exit 0

# NT35510 crate — independent build
cargo check --manifest-path=nt35510/Cargo.toml
grep 'stm32f4xx' nt35510/Cargo.toml
# Expected: check passes, grep returns no output
```

### Final Checklist
- [ ] All "Must Have" items present
- [ ] All "Must NOT Have" items absent
- [ ] PR1 branch clean rebase on upstream/master
- [ ] PR2 branch built on top of PR1
- [ ] NT35510 crate compiles independently
- [ ] All 17 MCUs pass cargo check
- [ ] Zero clippy warnings with -D warnings
- [ ] Zero rustfmt diffs
- [ ] CHANGELOG entries present for both PRs
