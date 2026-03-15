# Fix BSP USB Module & Commit All Improvements

## TL;DR

> **Quick Summary**: Fix the broken USB module in the BSP (`stm32f469i-disc/src/usb.rs`), uncomment it in `lib.rs`, stage the existing `build.rs` fix, commit and push everything, then produce a message with commit hashes for the upstream stm32-rs project to test.
> 
> **Deliverables**:
> - Working BSP USB module that returns HAL's `USB` struct
> - `build.rs` conditional memory.x generation fix committed
> - All pushed to `origin/pr2-f469disco-examples`
> - Upstream notification text with specific commit hash
> 
> **Estimated Effort**: Quick
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 4

---

## Context

### Original Request
Fix the broken BSP USB module, commit all pending fixes (USB + build.rs), push to the branch, and produce a message for the upstream stm32-rs/stm32f4xx-hal project with specific commit hashes for testing.

### Interview Summary
**Key Discussions**:
- `contributions/mod.rs` analyzed: application-level Specter-DIY GUI code with no value for BSP/HAL. References nonexistent `crate::theme::Theme`, has wallet-specific screen IDs. BSP already has better implementations.
- BSP is NOT a separate git repo — it's a subdirectory of the HAL repo, same `.git`, same branch `pr2-f469disco-examples`.
- `stm32f469i-disc/src/usb.rs` is completely broken: calls `UsbBus::new()` with wrong API, missing pins, missing clocks, doesn't compile.
- HAL's USB infrastructure is solid (`src/otg_fs.rs`): proper `USB` struct, `UsbPeripheral` trait impl, STM32F469 gets 6 endpoints.
- `build.rs` has an unstaged fix for conditional `memory.x` generation.
- ft6x06 panic fix already committed (`b32ccf4`), display flicker documented but too invasive for this commit.

**Research Findings**:
- HAL USB pattern from `examples/usb-serial-poll.rs`: `USB::new((periphs), (pa11, pa12), &clocks)` → `UsbBus::new(usb, EP_MEMORY.take())`
- BSP Cargo.toml already enables `usb_fs` feature on the HAL dependency
- PA11/PA12 are dedicated USB pins, not used by any other BSP module (display uses DSI not GPIO-A, touch uses PB8/PB9/PC1)
- No `usb-device` or `usbd-serial` in BSP deps — and that's correct since BSP should just provide the hardware wrapper, not the full USB stack

### Metis Review
**Identified Gaps** (addressed):
- USB abstraction level: Resolved → Option A (return HAL's `USB` struct, no new deps)
- Feature gating: Resolved → `#[cfg(feature = "usb_fs")]` on `pub mod usb;`
- PA11/PA12 conflicts: Resolved → no conflicts (GPIOA not used elsewhere in BSP)
- Acceptance criteria: Resolved → `cargo check -p stm32f469i-disc` must exit 0
- Commit style: Resolved → matches existing `fix(scope): description` pattern

---

## Work Objectives

### Core Objective
Fix the BSP USB module to use the correct HAL API, commit it alongside the build.rs fix, push to the branch, and provide upstream with testable commit references.

### Concrete Deliverables
- `stm32f469i-disc/src/usb.rs` — rewritten to return HAL's `USB` struct
- `stm32f469i-disc/src/lib.rs` — `pub mod usb;` uncommented with feature gate
- `build.rs` — existing fix staged and committed
- Git commit on `origin/pr2-f469disco-examples` with specific hash
- Upstream notification text

### Definition of Done
- [ ] `cargo check -p stm32f469i-disc` exits 0 (no errors)
- [ ] `git log origin/pr2-f469disco-examples --oneline -1` shows new commit
- [ ] Upstream notification text written with commit hash

### Must Have
- USB module compiles and exposes correct API
- build.rs fix included in commit
- Feature-gated USB module (`usb_fs` feature)
- Pushed to remote branch

### Must NOT Have (Guardrails)
- NO new dependencies added to BSP Cargo.toml (no `usb-device`, no `usbd-serial`)
- NO HAL core code changes (only BSP + build.rs)
- NO USB examples (out of scope — user didn't ask)
- NO double-buffering implementation (too invasive, separate effort)
- NO contributions/mod.rs integration (no value, application code)
- NO over-documented code (minimal doc comments matching existing BSP style)
- NO `static mut EP_MEMORY` in BSP — users manage their own endpoint memory
- NO changes to existing examples

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (embedded `no_std` — no unit test runner)
- **Automated tests**: None (cross-compilation check is the verification)
- **Framework**: None

### QA Policy
Every task includes agent-executed QA scenarios. For this embedded Rust project:
- **Compilation**: `cargo check` / `cargo build` with correct features
- **Git state**: `git status`, `git log`, `git diff` verification

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — independent fixes):
├── Task 1: Rewrite BSP usb.rs + uncomment in lib.rs [quick]
├── Task 2: Stage build.rs fix (no code changes, just git add) [quick]

Wave 2 (After Wave 1 — commit + push):
├── Task 3: Verify build, commit all changes, push [quick]
├── Task 4: Write upstream notification with commit hash [quick]

Critical Path: Task 1 → Task 3 → Task 4
Parallel Speedup: Tasks 1+2 run simultaneously
Max Concurrent: 2 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1    | —         | 3      |
| 2    | —         | 3      |
| 3    | 1, 2      | 4      |
| 4    | 3         | —      |

### Agent Dispatch Summary

- **Wave 1**: 2 tasks — T1 → `quick`, T2 → `quick`
- **Wave 2**: 2 tasks — T3 → `quick` + `git-master`, T4 → `quick`

---

## TODOs

- [ ] 1. Rewrite BSP USB Module and Uncomment in lib.rs

  **What to do**:
  - Rewrite `stm32f469i-disc/src/usb.rs` to provide a correct USB initialization function that returns the HAL's `USB` struct
  - The function signature should be: `pub fn init(otg_global: OTG_FS_GLOBAL, otg_device: OTG_FS_DEVICE, otg_pwrclk: OTG_FS_PWRCLK, pa11: impl Into<alt::Dm>, pa12: impl Into<alt::Dp>, clocks: &Clocks) -> USB`
  - Import from `crate::hal::otg_fs::{USB, UsbBus}` and `crate::hal::pac::{OTG_FS_GLOBAL, OTG_FS_DEVICE, OTG_FS_PWRCLK}`
  - Import `crate::hal::gpio::alt::otg_fs as alt` and `crate::hal::rcc::Clocks`
  - Re-export `UsbBus` and `USB` from `crate::hal::otg_fs` for user convenience
  - Keep doc comments matching existing BSP style (see `touch.rs` for reference — `//!` module docs, `///` function docs, usage example in doc comment)
  - In `stm32f469i-disc/src/lib.rs`: change `// pub mod usb; // TODO: Fix USB module for current HAL API` to `#[cfg(feature = "usb_fs")] pub mod usb;`
  - Do NOT add `usb-device`, `usbd-serial`, or `static-cell` to BSP dependencies
  - Do NOT create USB examples

  **Must NOT do**:
  - Add any new dependencies to BSP Cargo.toml
  - Manage static endpoint memory (`EP_MEMORY`) — that's the user's responsibility
  - Change HAL code in `src/otg_fs.rs`
  - Add USB serial example

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small focused edit to 2 files, well-defined pattern to follow
  - **Skills**: []
    - No special skills needed — straightforward Rust file editing
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed — no git operations in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 2)
  - **Parallel Group**: Wave 1 (with Task 2)
  - **Blocks**: Task 3
  - **Blocked By**: None (can start immediately)

  **References** (CRITICAL):

  **Pattern References** (existing code to follow):
  - `stm32f469i-disc/src/touch.rs` — BSP module pattern: doc comments, imports from `crate::hal`, init function that configures board-specific pins. Follow this exact style for module docs (`//!`), function docs (`///`), and usage example format.
  - `examples/usb-serial-poll.rs:28-32` — Correct HAL `USB::new()` call: `USB::new((dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK), (gpioa.pa11, gpioa.pa12), &rcc.clocks)`. This is the API you must use.

  **API/Type References** (contracts to implement against):
  - `src/otg_fs.rs:15-22` — HAL `USB` struct definition: fields are `usb_global`, `usb_device`, `usb_pwrclk`, `pin_dm: alt::Dm`, `pin_dp: alt::Dp`, `hclk: Hertz`
  - `src/otg_fs.rs:24-39` — `USB::new()` constructor signature: `new(periphs: (OTG_FS_GLOBAL, OTG_FS_DEVICE, OTG_FS_PWRCLK), pins: (impl Into<alt::Dm>, impl Into<alt::Dp>), clocks: &Clocks) -> Self`
  - `src/otg_fs.rs:12` — Re-export: `pub use synopsys_usb_otg::UsbBus;`
  - `src/otg_fs.rs:79` — Type alias: `pub type UsbBusType = UsbBus<USB>;`

  **External References** (libraries and frameworks):
  - `synopsys-usb-otg` crate: USB OTG peripheral driver. The BSP should NOT directly depend on this — it's accessed through HAL re-exports.
  - STM32F469I-DISCO schematic: PA11 = USB_DM, PA12 = USB_DP (OTG FS)

  **WHY Each Reference Matters**:
  - `touch.rs` — Copy the exact doc comment style, import patterns, and function structure. The USB init function should look like `init_i2c()` in touch.rs but for USB peripherals.
  - `usb-serial-poll.rs` — This is the AUTHORITATIVE example of how to call `USB::new()`. Do NOT deviate from this pattern.
  - `otg_fs.rs` — The type signatures you must match. The `USB` struct is what your init function returns.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: USB module compiles with usb_fs feature
    Tool: Bash
    Preconditions: Working Rust toolchain with thumbv7em-none-eabihf target
    Steps:
      1. Run `cargo check -p stm32f469i-disc` from HAL root directory
      2. Check exit code
    Expected Result: Exit code 0, no compilation errors
    Failure Indicators: Any error mentioning `usb`, `USB`, `OTG_FS`, `UsbBus`, or unresolved imports
    Evidence: .sisyphus/evidence/task-1-usb-compiles.txt

  Scenario: Feature gate works — module is only included with usb_fs
    Tool: Bash
    Preconditions: None
    Steps:
      1. Run `grep -n 'pub mod usb' stm32f469i-disc/src/lib.rs`
      2. Verify the line contains `#[cfg(feature = "usb_fs")]`
    Expected Result: Line shows `#[cfg(feature = "usb_fs")]` immediately before or on same line as `pub mod usb;`
    Failure Indicators: `pub mod usb;` without cfg gate
    Evidence: .sisyphus/evidence/task-1-feature-gate.txt

  Scenario: No new dependencies added
    Tool: Bash
    Preconditions: None
    Steps:
      1. Run `git diff stm32f469i-disc/Cargo.toml`
      2. Verify output is empty (no changes to Cargo.toml)
    Expected Result: Empty diff — Cargo.toml unchanged
    Failure Indicators: Any additions to [dependencies] section
    Evidence: .sisyphus/evidence/task-1-no-new-deps.txt
  ```

  **Commit**: YES (groups with Task 2, committed in Task 3)
  - Files: `stm32f469i-disc/src/usb.rs`, `stm32f469i-disc/src/lib.rs`

---

- [ ] 2. Stage build.rs Fix

  **What to do**:
  - Run `git add build.rs` to stage the existing modification
  - The modification adds conditional `memory.x` generation — only when building as the primary package (`CARGO_PRIMARY_PACKAGE`), not as a dependency. This prevents the HAL's `memory.x` from conflicting with BSP's own linker script.
  - Do NOT modify the file — it's already correct. Just stage it.

  **Must NOT do**:
  - Modify `build.rs` further
  - Stage any other files (`.elf` files, feedback docs, etc.)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single git command, no code changes
  - **Skills**: []
    - No special skills needed
  - **Skills Evaluated but Omitted**:
    - `git-master`: Overkill for a single `git add`

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 1)
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Task 3
  - **Blocked By**: None (can start immediately)

  **References** (CRITICAL):

  **Pattern References**:
  - `build.rs` (current diff) — The modification is already applied. Run `git diff build.rs` to verify the change adds `CARGO_PRIMARY_PACKAGE` conditional around `memory.x` generation. The change is correct and should be staged as-is.

  **WHY This Reference Matters**:
  - You must verify the diff is what you expect before staging. Don't blindly `git add`.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: build.rs is staged correctly
    Tool: Bash
    Preconditions: build.rs has unstaged modification
    Steps:
      1. Run `git diff build.rs` to verify the modification is the CARGO_PRIMARY_PACKAGE conditional
      2. Run `git add build.rs`
      3. Run `git diff --cached build.rs` to confirm it's now staged
    Expected Result: `git diff --cached build.rs` shows the memory.x conditional change. `git diff build.rs` shows empty (no unstaged changes remain).
    Failure Indicators: Diff shows unexpected changes, or staging fails
    Evidence: .sisyphus/evidence/task-2-buildrs-staged.txt

  Scenario: No unwanted files staged
    Tool: Bash
    Preconditions: After staging build.rs
    Steps:
      1. Run `git diff --cached --name-only`
      2. Verify only `build.rs` appears (Task 1 files added later)
    Expected Result: Only `build.rs` in staged files
    Failure Indicators: `.elf` files, `.md` files, or other untracked files appear
    Evidence: .sisyphus/evidence/task-2-only-buildrs.txt
  ```

  **Commit**: YES (groups with Task 1, committed in Task 3)
  - Files: `build.rs`

---

- [ ] 3. Verify Build, Commit All Changes, and Push

  **What to do**:
  - First verify compilation: `cargo check -p stm32f469i-disc` must exit 0
  - Stage the USB files: `git add stm32f469i-disc/src/usb.rs stm32f469i-disc/src/lib.rs`
  - Verify staged files are exactly: `build.rs`, `stm32f469i-disc/src/usb.rs`, `stm32f469i-disc/src/lib.rs`
  - Create commit with message: `fix(bsp): working USB OTG FS module and build.rs memory.x fix`
  - Push to `origin/pr2-f469disco-examples`
  - Capture the full commit hash with `git log -1 --format="%H"`
  - Record the hash — it will be used in Task 4

  **Must NOT do**:
  - Stage any untracked files (`.elf` files, `.md` notes, `contributions/`, etc.)
  - Force push
  - Create more than one commit
  - Amend the previous commit (`b32ccf4`)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard git operations with clear commands
  - **Skills**: [`git-master`]
    - `git-master`: Ensures proper commit hygiene, correct staging, and safe push
  - **Skills Evaluated but Omitted**:
    - None relevant

  **Parallelization**:
  - **Can Run In Parallel**: NO (must wait for Tasks 1 and 2)
  - **Parallel Group**: Wave 2 (sequential after Wave 1)
  - **Blocks**: Task 4
  - **Blocked By**: Tasks 1, 2

  **References** (CRITICAL):

  **Pattern References**:
  - `git log --oneline -5` — Recent commit style: `fix(ft6x06): patch touch controller panic...`, `feat(examples): add F469-Disco display examples...`, `feat(ltdc): add DSI constructor...`. Follow this `type(scope): description` pattern.
  - Branch: `pr2-f469disco-examples` — current branch, already tracks `origin/pr2-f469disco-examples`

  **WHY Each Reference Matters**:
  - Commit message must match established convention so the upstream PR looks consistent.
  - Push must go to the correct remote branch — `origin` not `upstream`.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: BSP compiles successfully before commit
    Tool: Bash
    Preconditions: Tasks 1 and 2 completed
    Steps:
      1. Run `cargo check -p stm32f469i-disc` from HAL root
      2. Assert exit code 0
    Expected Result: Clean compilation, no warnings about USB module
    Failure Indicators: Any compilation error
    Evidence: .sisyphus/evidence/task-3-build-check.txt

  Scenario: Correct files committed
    Tool: Bash
    Preconditions: Commit created
    Steps:
      1. Run `git show --stat HEAD`
      2. Verify exactly 3 files: `build.rs`, `stm32f469i-disc/src/lib.rs`, `stm32f469i-disc/src/usb.rs`
    Expected Result: 3 files changed, no unexpected files
    Failure Indicators: More or fewer than 3 files, or wrong files listed
    Evidence: .sisyphus/evidence/task-3-commit-stat.txt

  Scenario: Push succeeds to correct remote
    Tool: Bash
    Preconditions: Commit created
    Steps:
      1. Run `git push origin pr2-f469disco-examples`
      2. Run `git log origin/pr2-f469disco-examples -1 --format="%H %s"`
      3. Verify hash matches local HEAD and message starts with "fix(bsp):"
    Expected Result: Push succeeds. Remote HEAD matches local HEAD. Commit message is `fix(bsp): working USB OTG FS module and build.rs memory.x fix`
    Failure Indicators: Push rejected, hash mismatch, or wrong branch
    Evidence: .sisyphus/evidence/task-3-push-result.txt

  Scenario: No dirty state after commit
    Tool: Bash
    Preconditions: Commit and push done
    Steps:
      1. Run `git diff --name-only` (unstaged changes)
      2. Note: untracked files will still exist (.elf, .md notes etc) — that's expected
      3. Only check that `build.rs`, `stm32f469i-disc/src/usb.rs`, `stm32f469i-disc/src/lib.rs` are NOT in the diff
    Expected Result: The 3 committed files do not appear in `git diff`
    Failure Indicators: Any of the 3 files still showing as modified
    Evidence: .sisyphus/evidence/task-3-clean-state.txt
  ```

  **Commit**: YES — this IS the commit task
  - Message: `fix(bsp): working USB OTG FS module and build.rs memory.x fix`
  - Files: `build.rs`, `stm32f469i-disc/src/usb.rs`, `stm32f469i-disc/src/lib.rs`
  - Pre-commit: `cargo check -p stm32f469i-disc`

---

- [ ] 4. Write Upstream Notification with Commit Hash

  **What to do**:
  - After Task 3 push succeeds, get the commit hash from `git log -1 --format="%H"`
  - Write a notification text to `.sisyphus/evidence/upstream-notification.md` that includes:
    - The specific commit hash to test
    - The branch name: `pr2-f469disco-examples`
    - The repo: `Amperstrand/stm32f4xx-hal`
    - What changed: USB module fixed, build.rs conditional memory.x
    - How to test: `cargo check -p stm32f469i-disc`
    - What was already done in prior commits: DSI write commands, LTDC DSI constructor, display examples, ft6x06 panic fix
    - Full commit history for the branch (all 5 commits including the new one)
  - Print the notification text to stdout so the user can copy it

  **Must NOT do**:
  - Write to any file outside `.sisyphus/evidence/`
  - Make additional git commits
  - Push anything

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Text generation from known data, no complex logic
  - **Skills**: []
    - No special skills needed
  - **Skills Evaluated but Omitted**:
    - `writing`: Overkill for a short notification

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (after Task 3)
  - **Blocks**: None
  - **Blocked By**: Task 3

  **References** (CRITICAL):

  **Pattern References**:
  - `BSP-CONTRIBUTION-RESPONSE.md` — existing response template (at repo root). Use similar tone and structure but with SPECIFIC commit hashes.
  - `git log --oneline -5` — full commit list for the branch

  **WHY Each Reference Matters**:
  - The whole point is specific commit references. The upstream project needs exact hashes, not vague "check the branch".

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Notification contains commit hash
    Tool: Bash
    Preconditions: Task 3 completed, hash captured
    Steps:
      1. Read `.sisyphus/evidence/upstream-notification.md`
      2. Verify it contains the full 40-char commit hash from Task 3
      3. Verify it contains `pr2-f469disco-examples`
      4. Verify it contains `Amperstrand/stm32f4xx-hal`
    Expected Result: All three strings present in notification
    Failure Indicators: Missing hash, wrong branch name, or wrong repo name
    Evidence: .sisyphus/evidence/task-4-notification-check.txt

  Scenario: Notification is actionable
    Tool: Bash
    Preconditions: Notification written
    Steps:
      1. Verify notification contains a `cargo check` or `cargo build` command
      2. Verify it lists what changed (USB, build.rs)
    Expected Result: Reader can copy-paste the test command and knows what to look for
    Failure Indicators: Vague instructions like "check the branch"
    Evidence: .sisyphus/evidence/task-4-actionable-check.txt
  ```

  **Commit**: NO
  - This task produces documentation only, not committed to git

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, check git log). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check -p stm32f469i-disc`. Review `stm32f469i-disc/src/usb.rs` for: correct imports, proper doc comments, no `unsafe` blocks, no `as any`/dead code, follows `touch.rs` style. Check `lib.rs` feature gate is correct. Check `build.rs` diff is clean.
  Output: `Build [PASS/FAIL] | Style [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Execute EVERY QA scenario from EVERY task. Follow exact steps, capture evidence. Verify `cargo check` passes, `git log` shows correct commit, `git status` is clean for committed files, notification has real hash.
  Output: `Scenarios [N/N pass] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (`git show HEAD`). Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT do" compliance: no new deps in Cargo.toml, no examples added, no HAL changes. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| # | Message | Files | Pre-commit check |
|---|---------|-------|------------------|
| 1 | `fix(bsp): working USB OTG FS module and build.rs memory.x fix` | `build.rs`, `stm32f469i-disc/src/usb.rs`, `stm32f469i-disc/src/lib.rs` | `cargo check -p stm32f469i-disc` |

---

## Success Criteria

### Verification Commands
```bash
cargo check -p stm32f469i-disc          # Expected: exit 0, no errors
git log --oneline -1                      # Expected: fix(bsp): working USB OTG FS module...
git show --stat HEAD                      # Expected: 3 files changed
git diff --name-only                      # Expected: NO mention of build.rs, usb.rs, or lib.rs
cat .sisyphus/evidence/upstream-notification.md  # Expected: contains commit hash + test instructions
```

### Final Checklist
- [ ] USB module compiles (`cargo check` passes)
- [ ] USB module returns HAL's `USB` struct (not broken `UsbBus::new`)
- [ ] `pub mod usb;` is feature-gated with `#[cfg(feature = "usb_fs")]`
- [ ] `build.rs` fix committed (conditional `memory.x` generation)
- [ ] Single commit with correct message format
- [ ] Pushed to `origin/pr2-f469disco-examples`
- [ ] Upstream notification written with specific commit hash
- [ ] No new BSP dependencies added
- [ ] No HAL core code changed
- [ ] No `.elf` or feedback `.md` files committed
