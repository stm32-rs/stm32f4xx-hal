# ❌ CANCELLED — Superseded by upstream-merge.md

> This plan has been cancelled. The correct plan is `upstream-merge.md` which uses a 2-PR strategy with NT35510 as a standalone crate.
> Cancelled on: 2026-02-27

---

# PR #843 Clean Submission — NT35510 Display Support

## TL;DR

> **Quick Summary**: Create 3 clean commits on the `NT35510` branch from working code on `master`, fix bugs found by Metis review, migrate `ft6x06-rs` to existing `ft6x06` crate, add proper attribution, hardware-retest touch functionality, and force-push to update PR #843 for upstream submission.
>
> **Deliverables**:
> - 3 clean commits on `NT35510` branch with proper attribution
> - Feature gate bug fixed (display module compiles on STM32F429)
> - otm8009a made optional dependency
> - lcd-test.rs cleaned of dead `nt35510-only`/`otm8009a-only` cfg blocks
> - lcd-test.rs migrated from `ft6x06-rs` to `ft6x06` (already on origin/master)
> - Touch functionality hardware-verified after ft6x06 migration
> - Stale references removed (`.sisyphus`, deleted `display_init.rs`)
> - Force-pushed to update PR #843
> - Humble PR comment explaining the rebase
>
> **Estimated Effort**: Medium
> **Parallel Execution**: NO — strictly sequential (each commit depends on previous)
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7 → Task 8

---

## Context

### Original Request
Create clean, properly attributed, hardware-verified commits for upstream PR #843 to stm32-rs/stm32f4xx-hal. User wants "nice clean commits and minimal additional noise" and to "appear humble."

### Current State
- **`NT35510` branch**: At `origin/master` (clean slate, PR #843 head branch)
- **`master` branch**: 10 commits ahead of `origin/master` with ALL working code
- **All 5 display examples**: Hardware-verified on real STM32F469I-DISCO (evidence in `.sisyphus/evidence/`)
- **PR #843**: OPEN, targeting `master` on upstream repo

### Metis Review
**Critical bugs found** (MUST fix):
1. Feature gate `#[cfg(all(feature = "dma2d", feature = "ltdc"))]` on display module is wrong — STM32F429 has dma2d+ltdc but no dsihost → compile error. Fix: add `feature = "dsihost"` to gate.
2. `examples/f469disco-lcd-test.rs` line 102 references `.sisyphus/notes/f469-development.md` — doesn't exist upstream.
3. Stale doc comment in `f469disco.rs` line 11 references deleted `display_init.rs`.
4. `otm8009a` as unconditional dependency forces download for all HAL users.

**User decisions**:
- Clean up lcd-test.rs: Remove all `nt35510-only`/`otm8009a-only` cfg blocks, simplify to always use runtime detection
- Keep defmt logging in HAL display module (behind `#[cfg(feature = "defmt")]`)
- Make `otm8009a` an optional dependency (gated behind `dsihost` feature)
- Use 3-commit structure
- Migrate `ft6x06-rs` to existing `ft6x06` crate (eliminate new dev-dependency)
- Hardware retest touch functionality after ft6x06 migration

---

## Work Objectives

### Core Objective
Construct 3 clean, attribution-complete commits on the `NT35510` branch and force-push to update PR #843.

### Concrete Deliverables
- Commit 1: `feat(ltdc): add DSI constructor, framebuffer DrawTarget, and layer management APIs`
- Commit 2: `feat(display): add NT35510 driver and F469DISCO board init helpers`
- Commit 3: `feat(examples): add F469DISCO display examples with runtime panel autodetection`

### Definition of Done
- [ ] `NT35510` branch has exactly 3 commits ahead of `origin/master`
- [ ] `cargo check --features="stm32f469,stm32-fmc,framebuffer,defmt,dsihost"` passes
- [ ] `cargo check --features="stm32f429"` passes (regression test)
- [ ] No references to `.sisyphus`, `notes.md`, `display_init.rs`, `ft6x06-rs`, or `ft6x06_rs` in committed source
- [ ] No `nt35510-only` or `otm8009a-only` in `Cargo.toml` features
- [ ] `otm8009a` is optional in `[dependencies]`
- [ ] `ft6x06` used instead of `ft6x06-rs` in all examples
- [ ] Touch functionality verified on real hardware after ft6x06 migration
- [ ] Attribution comments present on derived files
- [ ] PR #843 updated via force-push

### Must Have
- Feature gate fix (`dsihost` added to display module cfg)
- Attribution on `nt35510.rs` and `f469disco.rs`
- Clean removal of `nt35510-only`/`otm8009a-only` from lcd-test.rs
- Removal of `.sisyphus` comment from lcd-test.rs
- Fix stale `display_init.rs` reference in f469disco.rs doc comment
- Migration from `ft6x06-rs` to `ft6x06` in lcd-test.rs
- Hardware retest confirming touch works after ft6x06 migration

### Must NOT Have (Guardrails)
- Do NOT include `.sisyphus/` files, `notes.md`, `dist/`, `tools/`, `STM32CubeF4/`, `stm32f7xx-hal/` in any commit
- Do NOT use `git add .` or `git add -A`
- Do NOT add SPDX headers or copyright blocks (not the repo's style — use simple `// Based on ...` comments)
- Do NOT modify files outside the identified scope
- Do NOT create more than 3 commits
- Do NOT remove defmt logging from display module (user decision: keep)
- Do NOT push to any remote other than origin
- Do NOT add `ft6x06-rs` as a dependency — use existing `ft6x06` crate

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: NO (no_std embedded target)
- **Automated tests**: None (cross-compiled no_std)
- **Verification method**: `cargo check` with feature combinations

### QA Policy
- Every commit verified with `cargo check` on both `stm32f469` and `stm32f429` targets
- Final diff comparison between NT35510 branch and master to catch content loss
- Grep for forbidden references (`.sisyphus`, `notes.md`, `display_init.rs`, `nt35510-only`, `ft6x06-rs`, `ft6x06_rs`)
- Evidence saved to `.sisyphus/evidence/`
- Hardware retest of touch functionality on real STM32F469I-DISCO after ft6x06 migration (Task 6)

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Sequential — commit pipeline + verification + push):
├── Task 1: Prepare NT35510 branch — checkout and verify clean state [quick]
├── Task 2: Commit 1 — LTDC improvements [deep]
├── Task 3: Commit 2 — Display module with fixes [deep]
├── Task 4: Commit 3 — Examples with cleanup + ft6x06 migration [deep]
├── Task 5: Full verification suite [quick]
├── Task 6: Hardware retest — verify touch after ft6x06 migration [deep]
├── Task 7: Force push NT35510 to origin [quick]
└── Task 8: Post PR comment [quick]

Critical Path: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7 → Task 8
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 2 |
| 2 | 1 | 3 |
| 3 | 2 | 4 |
| 4 | 3 | 5 |
| 5 | 4 | 6 |
| 6 | 5 | 7 |
| 7 | 6 | 8 |
| 8 | 7 | — |

### Agent Dispatch Summary

- **Wave 1**: 8 tasks — T1 → `quick`, T2 → `deep`, T3 → `deep`, T4 → `deep`, T5 → `quick`, T6 → `deep`, T7 → `quick` + `git-master`, T8 → `quick`

---

## TODOs

> Implementation is strictly sequential — each commit depends on the previous.
> EVERY task MUST have: Recommended Agent Profile + QA Scenarios.

- [ ] 1. Prepare NT35510 branch

  **What to do**:
  - Checkout the `NT35510` branch: `git checkout NT35510`
  - Verify it is at the same commit as `origin/master`: `git log --oneline -1 NT35510` should match `git log --oneline -1 origin/master`
  - If not, reset: `git reset --hard origin/master`
  - Verify clean working tree on NT35510: `git status` shows nothing

  **Must NOT do**:
  - Do NOT modify any files in this task
  - Do NOT create any commits

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple git checkout and verification, no code changes
  - **Skills**: [`git-master`]
    - `git-master`: Branch management and verification
  - **Skills Evaluated but Omitted**:
    - None — this is purely a git task

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (first task)
  - **Blocks**: Task 2
  - **Blocked By**: None (can start immediately)

  **References**:
  - `origin/master` is the base for PR #843
  - `NT35510` is the PR head branch (PR #843: https://github.com/stm32-rs/stm32f4xx-hal/pull/843)

  **Acceptance Criteria**:
  - [ ] `git branch --show-current` outputs `NT35510`
  - [ ] `git rev-parse NT35510` equals `git rev-parse origin/master`
  - [ ] `git status` shows clean working tree

  **QA Scenarios:**

  ```
  Scenario: NT35510 branch at origin/master
    Tool: Bash
    Preconditions: Repository cloned with NT35510 and origin/master refs available
    Steps:
      1. Run: git checkout NT35510
      2. Run: git rev-parse NT35510
      3. Run: git rev-parse origin/master
      4. Compare outputs of steps 2 and 3 — they MUST be identical
      5. Run: git status --porcelain
    Expected Result: Step 4 hashes match. Step 5 produces empty output.
    Failure Indicators: Hashes differ, or git status shows uncommitted changes
    Evidence: .sisyphus/evidence/task-1-branch-state.txt
  ```

  **Commit**: NO

---

- [ ] 2. Commit 1 — LTDC improvements: DSI constructor, framebuffer DrawTarget, layer APIs

  **What to do**:
  - Cherry-pick changes from `master` for `src/ltdc.rs` onto `NT35510` branch.
  - **METHOD**: Do NOT use `git cherry-pick`. Instead, manually copy specific content from `master` versions of files onto the `NT35510` working tree, then stage and commit. This avoids conflict resolution issues since the master commits touch multiple files across commit boundaries.
  - Copy `src/ltdc.rs` from master: `git show master:src/ltdc.rs > src/ltdc.rs`
  - Apply ONLY the `Cargo.toml` changes relevant to this commit:
    - Add `[dependencies.embedded-graphics-core]` section (version = "0.4", optional = true)
    - Add `framebuffer = ["dep:embedded-graphics-core"]` to `[features]`
    - NOTE: `otm8009a` changes go in Commit 2, example entries go in Commit 3
  - The ltdc.rs changes include:
    - `new_dsi()` constructor for DSI-driven displays
    - `layer_buffer_mut()` for mutable framebuffer access
    - `set_layer_transparency()`, `set_layer_buffer_address()`, `set_color_keying()` APIs
    - ARGB4444 bugfix: `bytes_per_pixel()` returns 2 (was incorrectly 16)
    - `LtdcFramebuffer<W, C, COLS, ROWS>` struct implementing `DrawTarget` (behind `framebuffer` feature)
  - Stage ONLY: `src/ltdc.rs`, `Cargo.toml`
  - Commit message: `feat(ltdc): add DSI constructor, framebuffer DrawTarget, and layer management APIs`

  **Must NOT do**:
  - Do NOT use `git add .` or `git add -A`
  - Do NOT add display module files (those are Commit 2)
  - Do NOT add example files (those are Commit 3)
  - Do NOT add `otm8009a` dependency changes (those are Commit 2)
  - Do NOT add example `[[example]]` entries (those are Commit 3)
  - Do NOT add SPDX headers

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires careful selective extraction from master, precise Cargo.toml editing, and understanding embedded Rust HAL patterns
  - **Skills**: [`git-master`]
    - `git-master`: Selective file extraction and precise commit staging
  - **Skills Evaluated but Omitted**:
    - `playwright`: No UI work

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Task 3
  - **Blocked By**: Task 1

  **References (CRITICAL — Be Exhaustive)**:

  **Pattern References:**
  - `src/ltdc.rs` on `master` branch — the complete LTDC file with all additions. Copy this entire file.
  - `src/ltdc.rs` on `origin/master` — the baseline to understand what's new

  **API/Type References:**
  - `src/ltdc.rs:362-431` (master) — `new_dsi()` constructor: creates DisplayController without pin/PLLSAI config (DSI host drives pixel clock)
  - `src/ltdc.rs:433-445` (master) — `layer_buffer_mut()`: returns `&mut [T]` slice of layer framebuffer
  - `src/ltdc.rs:447-467` (master) — `set_layer_transparency()`, `set_layer_buffer_address()`: layer blending APIs
  - `src/ltdc.rs:469-492` (master) — `set_color_keying()`: color key enable/disable
  - `src/ltdc.rs:494-548` (master) — `LtdcFramebuffer` struct + `DrawTarget` impl (behind `#[cfg(feature = "framebuffer")]`)
  - `src/ltdc.rs` — `bytes_per_pixel()` for `PixelFormat::ARGB4444` — MUST return 2, not 16 (bugfix)

  **Cargo.toml Changes for this commit:**
  - Add after line 95 (after `stm32_i2s_v12x` section):
    ```
    [dependencies.embedded-graphics-core]
    version = "0.4"
    optional = true
    ```
  - Add to `[features]` section after `ltdc = ["dep:micromath"]` (origin/master line ~520):
    ```
    framebuffer = ["dep:embedded-graphics-core"]
    ```

  **Acceptance Criteria**:
  - [ ] `git log --oneline NT35510 | head -1` shows the commit message containing `feat(ltdc)`
  - [ ] `git log --oneline origin/master..NT35510 | wc -l` outputs `1`
  - [ ] `git diff --stat HEAD~1..HEAD` shows ONLY `src/ltdc.rs` and `Cargo.toml`
  - [ ] `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf` passes
  - [ ] `cargo check --features="stm32f429" --target thumbv7em-none-eabihf` passes (regression — no display module yet)
  - [ ] No `otm8009a` in `[dependencies]` section (only in `[dev-dependencies]`)

  **QA Scenarios:**

  ```
  Scenario: Commit 1 content verification
    Tool: Bash
    Preconditions: NT35510 branch with exactly 1 commit ahead of origin/master
    Steps:
      1. Run: git log --oneline origin/master..NT35510
      2. Verify exactly 1 line of output containing "feat(ltdc)"
      3. Run: git diff --name-only HEAD~1..HEAD
      4. Verify output contains ONLY src/ltdc.rs and Cargo.toml (2 files)
      5. Run: git show HEAD:src/ltdc.rs | grep -c 'fn new_dsi'
      6. Verify output is 1 (new_dsi exists)
      7. Run: git show HEAD:src/ltdc.rs | grep 'ARGB4444 =>' | head -1
      8. Verify the ARGB4444 bytes_per_pixel returns 2 (not 16)
      9. Run: git show HEAD:Cargo.toml | grep -A2 'embedded-graphics-core'
      10. Verify it shows version = "0.4" and optional = true
      11. Run: git show HEAD:Cargo.toml | grep 'framebuffer'
      12. Verify framebuffer feature exists with dep:embedded-graphics-core
    Expected Result: All verifications pass
    Failure Indicators: Wrong number of files, missing new_dsi, wrong ARGB4444 value, missing Cargo.toml entries
    Evidence: .sisyphus/evidence/task-2-commit1-content.txt

  Scenario: Commit 1 compilation check
    Tool: Bash
    Preconditions: Commit 1 applied
    Steps:
      1. Run: cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf 2>&1
      2. Verify exit code 0
      3. Run: cargo check --features="stm32f429" --target thumbv7em-none-eabihf 2>&1
      4. Verify exit code 0 (STM32F429 regression test)
    Expected Result: Both cargo check commands succeed with no errors
    Failure Indicators: Compilation errors on either target
    Evidence: .sisyphus/evidence/task-2-commit1-build.txt
  ```

  **Commit**: YES
  - Message: `feat(ltdc): add DSI constructor, framebuffer DrawTarget, and layer management APIs`
  - Files: `src/ltdc.rs`, `Cargo.toml`
  - Pre-commit: `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf`

---

- [ ] 3. Commit 2 — Display module: NT35510 driver and F469DISCO board init with attribution

  **What to do**:
  - Copy display module files from master onto NT35510 working tree:
    - `mkdir -p src/display`
    - `git show master:src/display/mod.rs > src/display/mod.rs`
    - `git show master:src/display/nt35510.rs > src/display/nt35510.rs`
    - `git show master:src/display/f469disco.rs > src/display/f469disco.rs`
  - Copy `src/lib.rs` from master: `git show master:src/lib.rs > src/lib.rs`
  - **FIX feature gate bug** in `src/lib.rs`: Change line 122 from:
    `#[cfg(all(feature = "dma2d", feature = "ltdc"))]`
    to:
    `#[cfg(all(feature = "dma2d", feature = "ltdc", feature = "dsihost"))]`
    This prevents compile errors on STM32F429 which has dma2d+ltdc but no dsihost.
  - **ADD attribution** to `src/display/nt35510.rs` — add as first line after module doc comment:
    `// Based on work by Stepan Snigirev (diybitcoinhardware/f469-disco, MIT)`
    Follow the pattern in `src/qspi.rs:10`. Single line, no SPDX block.
  - **ADD attribution** to `src/display/f469disco.rs` — add as first line after module doc comment:
    `// Based on STM32CubeF4 BSP LCD driver (STMicroelectronics, BSD-3-Clause)`
  - **FIX stale doc comment** in `src/display/f469disco.rs` line 11: Change reference from
    `examples/f469disco/display_init.rs` to `examples/f469disco-lcd-test.rs` (or remove the line entirely)
  - **Cargo.toml changes for this commit:**
    - Make `otm8009a` an OPTIONAL dependency: change line 47 from `otm8009a = "0.1"` to `otm8009a = { version = "0.1", optional = true }`
    - Add `otm8009a` to the `dsihost` feature: change `dsihost = ["embedded-display-controller"]` to `dsihost = ["embedded-display-controller", "dep:otm8009a"]`
    - NOTE: `otm8009a = "0.1"` in `[dev-dependencies]` stays as-is (already on origin/master)
  - Stage ONLY: `src/display/mod.rs`, `src/display/nt35510.rs`, `src/display/f469disco.rs`, `src/lib.rs`, `Cargo.toml`
  - Commit message: `feat(display): add NT35510 driver and F469DISCO board init helpers`

  **Must NOT do**:
  - Do NOT use `git add .` or `git add -A`
  - Do NOT add example files (those are Commit 3)
  - Do NOT add SPDX headers or copyright blocks — use simple `// Based on ...` single-line comments
  - Do NOT remove defmt logging from display module (user decision: keep it)
  - Do NOT touch `src/ltdc.rs` (that was Commit 1)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires careful attribution placement, feature gate fix, stale reference fix, and Cargo.toml dependency management. Multiple files with interrelated changes.
  - **Skills**: [`git-master`]
    - `git-master`: Selective file extraction and precise staging
  - **Skills Evaluated but Omitted**:
    - `playwright`: No UI work

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Task 4
  - **Blocked By**: Task 2

  **References (CRITICAL — Be Exhaustive):**

  **Pattern References:**
  - `src/qspi.rs:10` (origin/master) — Attribution pattern to follow: `// Based on work by ...` single line, no SPDX
  - `src/display/mod.rs` on `master` — 24 lines, module re-exports. Copy as-is.
  - `src/display/nt35510.rs` on `master` — 182 lines, NT35510 LCD panel driver. Add attribution after doc comment.
  - `src/display/f469disco.rs` on `master` — 408 lines, board-specific display initialization. Add attribution + fix line 11 stale reference.

  **Bug Fix References:**
  - `src/lib.rs:120-123` (master) — Current feature gate is `#[cfg(all(feature = "dma2d", feature = "ltdc"))]` on BOTH `pub mod ltdc` and `pub mod display`. The `ltdc` module gate is correct. The `display` module gate MUST add `feature = "dsihost"` because display imports `DsiHost`.
  - `src/display/f469disco.rs:11` (master) — References deleted `examples/f469disco/display_init.rs`. Fix to reference `examples/f469disco-lcd-test.rs` instead.

  **Cargo.toml References:**
  - Line 47 on master: `otm8009a = "0.1"` — change to `otm8009a = { version = "0.1", optional = true }`
  - `dsihost` feature on origin/master (line ~520): `dsihost = ["embedded-display-controller"]` — add `"dep:otm8009a"`
  - Line 128 on master: `otm8009a = "0.1"` in `[dev-dependencies]` — leave as-is (already on origin/master)

  **External References:**
  - diybitcoinhardware/f469-disco (MIT, Copyright 2020 Stepan Snigirev): https://github.com/diybitcoinhardware/f469-disco
  - STM32CubeF4 BSP LCD driver (BSD-3-Clause, STMicroelectronics)

  **Acceptance Criteria**:
  - [ ] `git log --oneline origin/master..NT35510 | wc -l` outputs `2`
  - [ ] `git diff --name-only HEAD~1..HEAD` shows exactly: `Cargo.toml`, `src/display/f469disco.rs`, `src/display/mod.rs`, `src/display/nt35510.rs`, `src/lib.rs` (5 files)
  - [ ] `grep -c 'Based on' src/display/nt35510.rs` outputs `1`
  - [ ] `grep -c 'Based on' src/display/f469disco.rs` outputs `1`
  - [ ] `grep 'display_init.rs' src/display/f469disco.rs` returns no matches
  - [ ] `grep 'dsihost' src/lib.rs` shows feature gate includes dsihost
  - [ ] `grep 'otm8009a' Cargo.toml | grep 'optional'` shows optional = true in dependencies
  - [ ] `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf` passes
  - [ ] `cargo check --features="stm32f429" --target thumbv7em-none-eabihf` passes (regression — display module gated behind dsihost)

  **QA Scenarios:**

  ```
  Scenario: Attribution and bug fixes verified
    Tool: Bash
    Preconditions: Commit 2 applied on NT35510 branch
    Steps:
      1. Run: grep 'Based on' src/display/nt35510.rs
      2. Verify output contains "Stepan Snigirev" and "diybitcoinhardware/f469-disco" and "MIT"
      3. Run: grep 'Based on' src/display/f469disco.rs
      4. Verify output contains "STM32CubeF4" and "BSD-3-Clause"
      5. Run: grep 'display_init.rs' src/display/f469disco.rs
      6. Verify NO output (stale reference removed/fixed)
      7. Run: grep 'dsihost' src/lib.rs
      8. Verify the display module cfg gate includes dsihost
      9. Run: grep 'otm8009a' Cargo.toml
      10. Verify [dependencies] line has optional = true, [dev-dependencies] line is unchanged
    Expected Result: All 5 checks pass — attribution present, stale ref fixed, feature gate fixed, otm8009a optional
    Failure Indicators: Missing attribution, stale display_init.rs reference present, dsihost not in gate, otm8009a not optional
    Evidence: .sisyphus/evidence/task-3-commit2-fixes.txt

  Scenario: STM32F429 regression — display module does NOT compile on F429
    Tool: Bash
    Preconditions: Commit 2 applied
    Steps:
      1. Run: cargo check --features="stm32f429" --target thumbv7em-none-eabihf 2>&1
      2. Verify exit code 0 — the display module must NOT be pulled in for F429
      3. Run: cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf 2>&1
      4. Verify exit code 0 — F469 with full features must compile
    Expected Result: Both compile successfully. F429 doesn't see display module at all.
    Failure Indicators: F429 build fails with unresolved DsiHost or display imports
    Evidence: .sisyphus/evidence/task-3-commit2-regression.txt
  ```

  **Commit**: YES
  - Message: `feat(display): add NT35510 driver and F469DISCO board init helpers`
  - Files: `src/display/mod.rs`, `src/display/nt35510.rs`, `src/display/f469disco.rs`, `src/lib.rs`, `Cargo.toml`
  - Pre-commit: `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf`

---

- [ ] 4. Commit 3 — Examples with cleanup: all F469DISCO display examples

  **What to do**:
  - Copy example files from master onto NT35510 working tree:
    - `mkdir -p examples/f469disco`
    - `git show master:examples/f469disco-hello-eg.rs > examples/f469disco-hello-eg.rs`
    - `git show master:examples/f469disco-paint.rs > examples/f469disco-paint.rs`
    - `git show master:examples/f469disco-image-slider.rs > examples/f469disco-image-slider.rs`
    - `git show master:examples/f469disco-animated-layers.rs > examples/f469disco-animated-layers.rs`
    - `git show master:examples/f469disco-slideshow.rs > examples/f469disco-slideshow.rs`
    - `git show master:examples/f469disco/images.rs > examples/f469disco/images.rs`
  - Copy `examples/f469disco/nt35510.rs` from master: `git show master:examples/f469disco/nt35510.rs > examples/f469disco/nt35510.rs`
  - Copy `examples/f469disco-lcd-test.rs` from master: `git show master:examples/f469disco-lcd-test.rs > examples/f469disco-lcd-test.rs`

  - **CLEANUP lcd-test.rs** (CRITICAL — 27 matches of dead cfg blocks):
    - Remove ALL `#[cfg(feature = "nt35510-only")]` and `#[cfg(not(feature = "nt35510-only"))]` blocks
    - Remove ALL `#[cfg(feature = "otm8009a-only")]` and `#[cfg(not(feature = "otm8009a-only"))]` blocks
    - Remove the `compile_error!` for mutually exclusive features (line 60-61)
    - Remove the `.sisyphus` comment on line 102
    - Remove header doc comments referencing `nt35510-only`/`otm8009a-only` features (lines 8, 16, 19, 22)
    - Simplify to ALWAYS use runtime detection — the code inside `#[cfg(not(any(feature = "nt35510-only", feature = "otm8009a-only")))]` (line 407+) becomes the only path
    - Remove the `#[cfg(feature = "nt35510-only")]` and `#[cfg(feature = "otm8009a-only")]` forced-controller blocks (lines 389-405)
    - The file should read cleanly without any feature-gated branching for panel selection

  - **MIGRATE lcd-test.rs from `ft6x06-rs` to `ft6x06`** (CRITICAL — eliminate new dev-dep):
    - `ft6x06` v0.1.2 is ALREADY in dev-dependencies on `origin/master` — used by `display-touch.rs`, `f469disco-paint.rs`, `f469disco-image-slider.rs`
    - `ft6x06-rs` v0.3.0 was added by us but is NOT on origin/master — we want to remove this dependency entirely
    - API migration:
      - `use ft6x06_rs::FT6x06;` → `use ft6x06::Ft6X06;`
      - `FT6x06::new(i2c)` (takes ownership) → `Ft6X06::new(&i2c, 0x38, ts_int)` (takes reference + addr + interrupt pin)
      - `touch.get_touch_event()` → `touch.detect_touch(&mut i2c)` + `touch.get_touch(&mut i2c, 1)`
      - Need to add an interrupt pin (use `gpioc.pc1.into_pull_down_input()` — same pattern as `f469disco-paint.rs:153`)
      - Can add `touch.ts_calibration(&mut i2c, &mut delay)` (available in ft6x06 but not ft6x06-rs)
    - Follow the exact same touch initialization pattern used in `examples/f469disco-paint.rs:149-160`
    - The touch loop logic (toggle on rising edge, timeout, LED feedback) stays the same — just the API calls change

  - **CLEANUP nt35510.rs** (4 comments referencing removed features):
    - Remove/rewrite comments on lines 20, 29, 33, 47 that reference `nt35510-only` or `otm8009a-only` features
    - These comments like "Used only for runtime probing; unused when `nt35510-only` or `otm8009a-only` features are enabled" should be simplified to just describe the function's purpose

  - **Cargo.toml changes for this commit:**
    - Do NOT add `ft6x06-rs` — it's being eliminated
    - Add all new `[[example]]` entries for the 5 new examples (paint, hello-eg, image-slider, animated-layers, slideshow)
    - Update `f469disco-lcd-test` required-features from `["stm32f469", "defmt"]` to `["stm32f469", "stm32-fmc", "defmt"]` (needs stm32-fmc now)
    - REMOVE `nt35510-only = []` and `otm8009a-only = []` from `[features]` section

  - Stage ONLY: all example files listed above + `Cargo.toml`
  - Commit message: `feat(examples): add F469DISCO display examples with runtime panel autodetection`

  **Must NOT do**:
  - Do NOT use `git add .` or `git add -A`
  - Do NOT touch `src/ltdc.rs` or `src/display/` (those are Commits 1-2)
  - Do NOT touch `src/lib.rs` (that was Commit 2)
  - Do NOT include `.sisyphus/` files, `notes.md`, or any non-example files beyond Cargo.toml
  - Do NOT leave any reference to `.sisyphus`, `display_init.rs`, `nt35510-only`, or `otm8009a-only` in committed files
  - Do NOT add `ft6x06-rs` as a dev-dependency — use the existing `ft6x06` crate instead
  - Do NOT leave any `ft6x06_rs` or `ft6x06-rs` references in example code

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Extensive file cleanup required — removing 27 cfg-block occurrences across lcd-test.rs, rewriting comments in nt35510.rs, managing multiple Cargo.toml changes. High attention to detail needed.
  - **Skills**: [`git-master`]
    - `git-master`: Multi-file staging and precise commit creation
  - **Skills Evaluated but Omitted**:
    - `playwright`: No UI work

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Task 5
  - **Blocked By**: Task 3

  **References (CRITICAL — Be Exhaustive):**

  **Pattern References:**
  - `examples/f469disco-lcd-test.rs` on `master` — The source file to copy, then clean up. 323 lines on master.
  - `examples/f469disco/nt35510.rs` on `master` — Helper module, 4 comments to rewrite (lines 20, 29, 33, 47)
  - `examples/f469disco-hello-eg.rs` on `master` — 154 lines, copy as-is
  - `examples/f469disco-paint.rs` on `master` — 206 lines, copy as-is
  - `examples/f469disco-image-slider.rs` on `master` — 194 lines, copy as-is
  - `examples/f469disco-animated-layers.rs` on `master` — 171 lines, copy as-is
  - `examples/f469disco-slideshow.rs` on `master` — 158 lines, copy as-is
  - `examples/f469disco/images.rs` on `master` — 123 lines, copy as-is

  **Cleanup References:**
  - `examples/f469disco-lcd-test.rs:8,16,19,22` — Doc comments referencing nt35510-only/otm8009a-only features → REMOVE
  - `examples/f469disco-lcd-test.rs:34,57,60-61,80,82,89,91,104,124,141,143,260,268,389-407` — cfg blocks → REMOVE (keep the runtime-detection code path)
  - `examples/f469disco-lcd-test.rs:102` — `.sisyphus` reference → REMOVE
  - `examples/f469disco/nt35510.rs:20,29,33,47` — Comments referencing removed features → REWRITE

  **ft6x06 Migration References (CRITICAL):**
  - `examples/f469disco-paint.rs:149-160` on `master` — The CANONICAL pattern for ft6x06 touch init: `Ft6X06::new(&i2c, 0x38, ts_int)`, `ts_calibration()`, then polling via `detect_touch()` + `get_touch()`. Copy this pattern into lcd-test.rs.
  - `examples/display-touch.rs:162-198` on `origin/master` — Alternative reference for ft6x06 API: shows `detect_touch()` returning touch count, `get_touch()` returning coordinates
  - `examples/f469disco-lcd-test.rs:56,63-64,289-306,319-377` on `master` — Current ft6x06-rs usage that must be migrated to ft6x06 API
  - Key API differences:
    - `ft6x06::Ft6X06::new(&i2c, 0x38, ts_int)` vs `ft6x06_rs::FT6x06::new(i2c)` — ft6x06 takes I2C by ref, ft6x06-rs takes ownership
    - `detect_touch(&mut i2c)` → `Result<u8>` (number of touches) vs `get_touch_event()` → `Result<Option<TouchEvent>>`
    - `get_touch(&mut i2c, 1)` → touch data with `.x`, `.y`, `.weight`, `.misc` vs `.primary_point.x`, `.primary_point.y`
    - `ts_calibration(&mut i2c, &mut delay)` is available in ft6x06 (was missing in ft6x06-rs v0.3.0)

  **Cargo.toml References:**
  - origin/master line 627: `required-features = ["stm32f469", "defmt"]` for lcd-test → add `"stm32-fmc"`
  - origin/master lines 535-536: `nt35510-only = []` and `otm8009a-only = []` → REMOVE
  - Example entries to add (follow existing pattern, e.g. origin/master line 626+):
    - `f469disco-paint`: required-features = `["stm32f469", "stm32-fmc", "framebuffer", "defmt"]`
    - `f469disco-hello-eg`: required-features = `["stm32f469", "stm32-fmc", "framebuffer", "defmt"]`
    - `f469disco-image-slider`: required-features = `["stm32f469", "stm32-fmc", "defmt"]`
    - `f469disco-animated-layers`: required-features = `["stm32f469", "stm32-fmc", "defmt"]`
    - `f469disco-slideshow`: required-features = `["stm32f469", "stm32-fmc", "defmt"]`

  **Acceptance Criteria**:
  - [ ] `git log --oneline origin/master..NT35510 | wc -l` outputs `3`
  - [ ] `git diff --name-only HEAD~1..HEAD | sort` shows exactly: `Cargo.toml`, `examples/f469disco-animated-layers.rs`, `examples/f469disco-hello-eg.rs`, `examples/f469disco-image-slider.rs`, `examples/f469disco-lcd-test.rs`, `examples/f469disco-paint.rs`, `examples/f469disco-slideshow.rs`, `examples/f469disco/images.rs`, `examples/f469disco/nt35510.rs` (9 files)
  - [ ] `grep -r 'nt35510-only\|otm8009a-only' examples/` returns NO matches
  - [ ] `grep -r '\.sisyphus' examples/` returns NO matches
  - [ ] `grep 'nt35510-only\|otm8009a-only' Cargo.toml` returns NO matches
  - [ ] `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf` passes
  - [ ] `cargo check --features="stm32f429" --target thumbv7em-none-eabihf` passes (regression)
  - [ ] `grep -r 'ft6x06.rs\|ft6x06_rs' examples/ Cargo.toml` returns NO matches (ft6x06-rs fully eliminated)
  - [ ] `grep 'ft6x06' examples/f469disco-lcd-test.rs` shows `ft6x06::Ft6X06` (not `ft6x06_rs`)

  **QA Scenarios:**

  ```
  Scenario: Dead feature references completely removed
    Tool: Bash
    Preconditions: Commit 3 applied on NT35510 branch
    Steps:
      1. Run: grep -rn 'nt35510-only\|otm8009a-only' examples/ Cargo.toml src/
      2. Verify NO output — zero matches anywhere in the codebase
      3. Run: grep -rn '\.sisyphus' examples/ src/
      4. Verify NO output — zero references to .sisyphus
      5. Run: grep -rn 'display_init\.rs' examples/ src/
      6. Verify NO output — zero references to deleted file
    Expected Result: All three greps return empty (exit code 1)
    Failure Indicators: Any match found means cleanup was incomplete
    Evidence: .sisyphus/evidence/task-4-cleanup-verification.txt

  Scenario: All 6 examples present with correct required-features
    Tool: Bash
    Preconditions: Commit 3 applied
    Steps:
      1. Run: ls examples/f469disco-*.rs | wc -l
      2. Verify output is 6 (lcd-test, hello-eg, paint, image-slider, animated-layers, slideshow)
      3. Run: ls examples/f469disco/*.rs | wc -l
      4. Verify output is 2 (images.rs, nt35510.rs)
      5. Run: grep -A1 'name = "f469disco-' Cargo.toml
      6. Verify all 6 examples have [[example]] entries with stm32-fmc in required-features
    Expected Result: 6 top-level examples, 2 helper modules, all with correct Cargo.toml entries
    Failure Indicators: Missing examples, missing Cargo.toml entries, wrong required-features
    Evidence: .sisyphus/evidence/task-4-examples-present.txt

  Scenario: Full build with all features after all 3 commits
    Tool: Bash
    Preconditions: All 3 commits applied on NT35510
    Steps:
      1. Run: cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf 2>&1
      2. Verify exit code 0
      3. Run: cargo check --features="stm32f429" --target thumbv7em-none-eabihf 2>&1
      4. Verify exit code 0
    Expected Result: Both targets compile cleanly
    Failure Indicators: Any compilation error
    Evidence: .sisyphus/evidence/task-4-full-build.txt
  ```

  **Commit**: YES
  - Message: `feat(examples): add F469DISCO display examples with runtime panel autodetection`
  - Files: `examples/f469disco-lcd-test.rs`, `examples/f469disco-hello-eg.rs`, `examples/f469disco-paint.rs`, `examples/f469disco-image-slider.rs`, `examples/f469disco-animated-layers.rs`, `examples/f469disco-slideshow.rs`, `examples/f469disco/images.rs`, `examples/f469disco/nt35510.rs`, `Cargo.toml`
  - Pre-commit: `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf`

---

- [ ] 5. Full verification suite — compile checks, forbidden reference scan, commit structure

  **What to do**:
  - Run comprehensive verification on the final state of the `NT35510` branch:
  - **Commit count**: `git log --oneline origin/master..NT35510 | wc -l` → must be exactly `3`
  - **Commit messages**: Verify the 3 commit messages match the specified format
  - **Compile check (F469)**: `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf`
  - **Compile check (F429 regression)**: `cargo check --features="stm32f429" --target thumbv7em-none-eabihf`
  - **Forbidden references scan**: grep entire committed tree for:
    - `.sisyphus` → must NOT appear in any `src/` or `examples/` file
    - `display_init.rs` → must NOT appear anywhere
    - `nt35510-only` or `otm8009a-only` → must NOT appear in `Cargo.toml` features or any source/example file
    - `notes.md` → must NOT be in any commit
    - `ft6x06-rs` or `ft6x06_rs` → must NOT appear in any example or Cargo.toml (eliminated in favor of existing `ft6x06`)
  - **Diff comparison**: `git diff master..NT35510 -- src/ examples/ Cargo.toml` to verify content is not lost
    The NT35510 branch should have ALL the same src/ and examples/ content as master, minus the cleanup changes (removed cfg blocks, fixed references)
  - **File list**: Verify no unexpected files in any commit (no .sisyphus/, no notes.md, no dist/)
  - Save all verification output to evidence file

  **Must NOT do**:
  - Do NOT modify any files
  - Do NOT create any commits

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Verification-only task — runs commands, checks output, saves evidence
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed, just reading git output

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Task 6
  - **Blocked By**: Task 4

  **References:**
  - All acceptance criteria from Tasks 1-4 should be re-verified here as a comprehensive check
  - The "Must NOT Have" section from Work Objectives defines the forbidden patterns

  **Acceptance Criteria**:
  - [ ] Exactly 3 commits ahead of origin/master
  - [ ] Both cargo check commands pass (F469 and F429)
  - [ ] Zero matches for forbidden references
  - [ ] No unexpected files in any commit
  - [ ] Evidence file saved

  **QA Scenarios:**

  ```
  Scenario: Complete verification of NT35510 branch
    Tool: Bash
    Preconditions: All 3 commits applied on NT35510 branch
    Steps:
      1. Run: git log --oneline origin/master..NT35510
      2. Verify exactly 3 lines, with correct commit message prefixes (feat(ltdc), feat(display), feat(examples))
      3. Run: cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf 2>&1
      4. Verify exit code 0
      5. Run: cargo check --features="stm32f429" --target thumbv7em-none-eabihf 2>&1
      6. Verify exit code 0
      7. Run: grep -rn '\.sisyphus\|display_init\.rs\|nt35510-only\|otm8009a-only\|ft6x06.rs\|ft6x06_rs' src/ examples/ Cargo.toml 2>&1 || echo "CLEAN"
      8. Verify output is "CLEAN"
      9. Run: git diff --stat origin/master..NT35510 -- notes.md .sisyphus/ dist/ tools/
      10. Verify empty output (none of these files are in any commit)
      11. Run: git diff --stat master..NT35510 -- src/ examples/
      12. Review the diff — NT35510 should differ from master ONLY in:
          - lcd-test.rs: ft6x06-rs migrated to ft6x06, removed cfg blocks and .sisyphus comment
          - f469disco.rs: fixed doc comment, added attribution
          - nt35510.rs (display): added attribution
          - lib.rs: fixed feature gate
          - Cargo.toml: otm8009a optional, removed nt35510-only/otm8009a-only features
    Expected Result: All checks pass. Diff between master and NT35510 is minimal and expected.
    Failure Indicators: Wrong commit count, build failures, forbidden references found, unexpected file differences
    Evidence: .sisyphus/evidence/task-5-full-verification.txt
  ```

  **Commit**: NO

---

- [ ] 6. Hardware retest — verify touch works after ft6x06 migration on real STM32F469I-DISCO

  **What to do**:
  - This task runs AFTER the 3 commits are created on NT35510 but BEFORE pushing. It verifies that the ft6x06 migration in lcd-test.rs didn't break touch functionality.
  - SSH into the remote board: `ssh ubuntu@192.168.13.246`
  - Set probe-rs PATH: `export PATH=$PATH:$HOME/.cargo/bin`
  - Flash `f469disco-lcd-test` from the NT35510 branch:
    `cargo run --example f469disco-lcd-test --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf --release`
  - Observe defmt logs for:
    - Display initialization (NT35510 or OTM8009A detection via ADC)
    - Touch controller initialization (`Ft6X06::new` with address 0x38)
    - `ts_calibration` completion
    - Touch events registering when screen is touched (detect_touch count > 0, get_touch coordinates)
  - Touch the screen in multiple locations — verify coordinates change appropriately
  - Verify the toggle-on-touch behavior still works (LED feedback or screen color toggle)
  - If touch does NOT work: STOP. Do NOT push. Report the failure with full defmt log output.
  - If touch works: Save the defmt log output as evidence

  **Must NOT do**:
  - Do NOT push to remote if touch is broken
  - Do NOT modify any code in this task — if touch is broken, report back for diagnosis
  - Do NOT skip the touch verification (the whole point of this task)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Remote hardware interaction via SSH, real-time log observation, manual touch verification. Requires careful attention to defmt output and ability to diagnose failures.
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: No git operations in this task
    - `playwright`: Not a browser task — this is embedded hardware

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Task 7
  - **Blocked By**: Task 5

  **References (CRITICAL):**

  **Touch API References:**
  - `examples/f469disco-paint.rs:149-160` on `master` — The ft6x06 touch init pattern that lcd-test.rs should now match
  - `examples/display-touch.rs:162-198` on `origin/master` — Alternative ft6x06 reference showing detect_touch/get_touch flow
  - The touch controller I2C address is `0x38`, interrupt pin is typically `PC1`

  **Previous Hardware Test Evidence:**
  - `.sisyphus/evidence/` — Previous test runs with ft6x06-rs. Compare behavior: touch should work identically with ft6x06.

  **Remote Board Details:**
  - Host: `ubuntu@192.168.13.246`
  - Chip: STM32F469NIHx
  - Panel: NT35510 (B08 variant)
  - `probe-rs` not on default PATH — must `export PATH=$PATH:$HOME/.cargo/bin`

  **Acceptance Criteria**:
  - [ ] `f469disco-lcd-test` flashes successfully on the board
  - [ ] defmt logs show display initialization completing (panel detected)
  - [ ] defmt logs show touch controller initialization (Ft6X06 with addr 0x38)
  - [ ] Touch events are detected when screen is physically touched
  - [ ] Touch coordinates change when touching different screen locations
  - [ ] Evidence log saved with full defmt output

  **QA Scenarios:**

  ```
  Scenario: Touch functionality works after ft6x06 migration
    Tool: Bash (SSH to remote board)
    Preconditions: NT35510 branch with all 3 commits, remote board powered and connected
    Steps:
      1. SSH: ssh ubuntu@192.168.13.246
      2. Set PATH: export PATH=$PATH:$HOME/.cargo/bin
      3. Navigate to project directory
      4. Run: cargo run --example f469disco-lcd-test --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf --release 2>&1 | tee /tmp/touch-retest.log
      5. Wait for display initialization logs (expect: panel type detected)
      6. Wait for touch init logs (expect: Ft6X06 initialized at 0x38)
      7. Touch the screen physically — observe defmt logs for touch events
      8. Verify touch coordinates appear and change with touch position
      9. Copy log: scp ubuntu@192.168.13.246:/tmp/touch-retest.log .sisyphus/evidence/task-6-touch-retest.log
    Expected Result: Display initializes, touch events register with valid coordinates, toggle behavior works
    Failure Indicators: No touch events in logs, I2C errors, panic, coordinates stuck at 0,0
    Evidence: .sisyphus/evidence/task-6-touch-retest.log
  ```

  **Commit**: NO

---

- [ ] 7. Force push NT35510 branch to origin

  **What to do**:
  - Verify we are on the `NT35510` branch: `git branch --show-current`
  - Force push: `git push --force-with-lease origin NT35510`
  - Verify push succeeded: `git log --oneline origin/NT35510 | head -5`
  - Verify PR #843 now shows the new commits (use `gh pr view 843 --json commits`)

  **Must NOT do**:
  - Do NOT push to any branch other than `NT35510`
  - Do NOT use `--force` — use `--force-with-lease` for safety
  - Do NOT push to `master` branch on remote

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single git push command with verification
  - **Skills**: [`git-master`]
    - `git-master`: Force push with lease and remote verification
  - **Skills Evaluated but Omitted**:
    - None

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Task 8
  - **Blocked By**: Task 6

  **References:**
  - PR #843: https://github.com/stm32-rs/stm32f4xx-hal/pull/843
  - Remote: `origin` (stm32-rs/stm32f4xx-hal or user's fork)

  **Acceptance Criteria**:
  - [ ] `git push --force-with-lease origin NT35510` succeeds
  - [ ] `git rev-parse origin/NT35510` matches `git rev-parse NT35510`
  - [ ] `gh pr view 843` shows updated state

  **QA Scenarios:**

  ```
  Scenario: Force push and PR update verification
    Tool: Bash
    Preconditions: All 3 commits verified on NT35510 branch
    Steps:
      1. Run: git branch --show-current
      2. Verify output is NT35510
      3. Run: git push --force-with-lease origin NT35510
      4. Verify push succeeds (exit code 0)
      5. Run: git rev-parse origin/NT35510
      6. Run: git rev-parse NT35510
      7. Verify outputs of steps 5 and 6 are identical
    Expected Result: Push succeeds, remote matches local
    Failure Indicators: Push rejected, hashes don't match
    Evidence: .sisyphus/evidence/task-7-push-verification.txt
  ```

  **Commit**: NO (this IS the push)

---

- [ ] 8. Post humble PR comment on #843

  **What to do**:
  - Post a comment on PR #843 explaining the rebase and what was done
  - Use `gh pr comment 843 --body "..."` to post the comment
  - **Tone**: Humble, honest. User's words: "my main goal was really mostly just trying to get it to work"
  - **Content should include**:
    - Acknowledgment that this is a force-push / rebase
    - Brief summary of the 3 commits (LTDC improvements, display module, examples)
    - Mention hardware verification on real STM32F469I-DISCO board
    - Credit to the prior work this builds on (diybitcoinhardware/f469-disco, STM32CubeF4 BSP)
    - Humble framing: "My main goal was to get the NT35510 display working on the F469 Discovery board"
    - Note that all examples have been tested on real hardware
  - **Keep it SHORT** — 2-3 paragraphs max. No walls of text.

  **Must NOT do**:
  - Do NOT be boastful or overly technical
  - Do NOT include internal implementation details or debugging history
  - Do NOT mention `.sisyphus` or AI tooling
  - Do NOT create an excessively long comment

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single gh command to post a short comment
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not a git operation, just gh CLI

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (final task)
  - **Blocks**: None
  - **Blocked By**: Task 7

  **References:**
  - PR #843: https://github.com/stm32-rs/stm32f4xx-hal/pull/843
  - diybitcoinhardware/f469-disco: https://github.com/diybitcoinhardware/f469-disco
  - User tone preference: "I want to appear humble and to make it clear that my main goal was just trying to get it to work"

  **Acceptance Criteria**:
  - [ ] `gh pr view 843 --json comments --jq '.comments[-1].body'` shows the posted comment
  - [ ] Comment mentions hardware verification
  - [ ] Comment credits diybitcoinhardware/f469-disco
  - [ ] Comment is ≤3 paragraphs
  - [ ] Comment does NOT mention .sisyphus, AI, or internal tooling

  **QA Scenarios:**

  ```
  Scenario: PR comment posted with correct tone
    Tool: Bash
    Preconditions: PR #843 force-pushed with new commits
    Steps:
      1. Run: gh pr comment 843 --body "<the composed comment>"
      2. Verify exit code 0
      3. Run: gh pr view 843 --json comments --jq '.comments[-1].body'
      4. Verify the comment contains:
         - "hardware" or "real board" (hardware verification mention)
         - "f469-disco" or "diybitcoinhardware" (attribution)
         - Does NOT contain ".sisyphus" or "AI"
      5. Verify comment length is reasonable (< 2000 characters)
    Expected Result: Comment posted successfully with humble tone and proper attribution
    Failure Indicators: Comment not posted, missing attribution, too long, mentions internal tooling
    Evidence: .sisyphus/evidence/task-8-pr-comment.txt
  ```

  **Commit**: NO

---

## Final Verification Wave

> Since this is a strictly sequential pipeline (each commit depends on the previous),
> Task 5 serves as the comprehensive compilation/reference verification, and Task 6 serves as
> the hardware verification for the ft6x06 migration. No separate final wave is needed.
> Tasks 5-6 together verify ALL acceptance criteria from Tasks 1-4.

---

## Commit Strategy

| # | Message | Files | Pre-commit Check |
|---|---------|-------|-----------------|
| 1 | `feat(ltdc): add DSI constructor, framebuffer DrawTarget, and layer management APIs` | `src/ltdc.rs`, `Cargo.toml` | `cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf` |
| 2 | `feat(display): add NT35510 driver and F469DISCO board init helpers` | `src/display/mod.rs`, `src/display/nt35510.rs`, `src/display/f469disco.rs`, `src/lib.rs`, `Cargo.toml` | same |
| 3 | `feat(examples): add F469DISCO display examples with runtime panel autodetection` | 8 example files + `Cargo.toml` | same |

---

## Success Criteria

### Verification Commands
```bash
# Exactly 3 commits
git log --oneline origin/master..NT35510 | wc -l  # Expected: 3

# F469 target compiles
cargo check --features="stm32f469,stm32-fmc,framebuffer,dsihost,defmt" --target thumbv7em-none-eabihf  # Expected: success

# F429 regression (display module NOT pulled in)
cargo check --features="stm32f429" --target thumbv7em-none-eabihf  # Expected: success

# No forbidden references
grep -rn '\.sisyphus\|display_init\.rs\|nt35510-only\|otm8009a-only\|ft6x06.rs\|ft6x06_rs' src/ examples/ Cargo.toml  # Expected: no output

# Attribution present
grep 'Based on' src/display/nt35510.rs src/display/f469disco.rs  # Expected: 2 lines

# otm8009a optional
grep 'otm8009a.*optional' Cargo.toml  # Expected: 1 match

# PR updated
gh pr view 843 --json headRefOid  # Expected: matches git rev-parse NT35510
```

### Final Checklist
- [ ] All "Must Have" present (feature gate fix, attribution, cleanup, stale ref fix, ft6x06 migration)
- [ ] All "Must NOT Have" absent (no .sisyphus, no notes.md, no SPDX blocks, no extra commits, no ft6x06-rs)
- [ ] Both cargo check targets pass (F469 and F429)
- [ ] Touch functionality verified on real hardware after ft6x06 migration
- [ ] PR #843 updated via force-push
- [ ] Humble PR comment posted with attribution
