# NT35510 External Crate Migration

## TL;DR

> **Quick Summary**: Replace the local `examples/f469disco/nt35510.rs` driver module with the published `nt35510 = "0.1.0"` crate from crates.io across all 5 f469disco examples, then hardware-verify on real STM32F469I-DISCO board.
> 
> **Deliverables**:
> - All 5 f469disco examples compile using external `nt35510` crate instead of local module
> - Hardware-verified on real STM32F469I-DISCO B08 board (NT35510 display)
> - Local `examples/f469disco/nt35510.rs` file deleted
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7

---

## Context

### Original Request
User published the `nt35510` crate to crates.io (v0.1.0) and wants to refactor all f469disco examples to use the external crate instead of the local `#[path = "f469disco/nt35510.rs"] mod nt35510;` pattern. Changes must be hardware-verified on a real STM32F469I-DISCO B08 board before being considered complete.

### Interview Summary
**Key Discussions**:
- **Branch**: Work on `NT35510` branch (not pr2) — user explicitly chose this
- **Test order**: Start with `f469disco-lcd-test.rs` (self-contained, no board.rs dependency) as proof-of-concept
- **Compilation scope**: Only compile f469-related features locally; rely on GitHub CI for full matrix
- **Hardware verification**: Compile locally on Mac, SCP to remote Ubuntu machine, flash via probe-rs
- **Delete policy**: Only delete local `nt35510.rs` after ALL examples pass hardware verification

**Research Findings**:
- `DsiHost` implements `DsiHostCtrlIo` (src/dsi.rs:582) — compatible with external crate's generic interface
- `SysDelay` implements both `DelayNs` (embedded-hal 1.0) and `DelayUs<u32>` (embedded-hal 0.2) — no breaking change
- External crate's `Error` type lacks `defmt::Format` — `defmt::panic!("{:?}", _e)` won't compile
- External crate has extra `InvalidDimensions` error variant — match arms need wildcard or explicit handling
- `f469disco-touch-debug.rs` has NO nt35510 dependency — completely out of scope

### Metis Review
**Identified Gaps** (addressed):
- **defmt incompatibility**: External `Error` lacks `defmt::Format` — resolved by dropping error from defmt panic message
- **Extra error variant**: `InvalidDimensions` in external crate — resolved with wildcard match arm
- **otm8009a contamination risk**: Adding `+ DelayNs` near otm8009a code — guardrail: DO NOT change otm8009a usage
- **Rollback plan**: If hardware tests fail — revert changes, keep local nt35510.rs

---

## Work Objectives

### Core Objective
Migrate all f469disco examples from the local `nt35510.rs` driver module to the published `nt35510 = "0.1.0"` crate, hardware-verify on real hardware, and clean up the now-unused local file.

### Concrete Deliverables
- `Cargo.toml` updated with `nt35510 = "0.1.0"` in `[dev-dependencies]`
- `examples/f469disco-lcd-test.rs` — uses external crate, compiles, hardware-verified
- `examples/f469disco/board.rs` — uses external crate imports and trait bounds
- `examples/f469disco-hello-eg.rs` — local module removed
- `examples/f469disco-paint.rs` — local module removed
- `examples/f469disco-image-slider.rs` — local module removed
- `examples/f469disco-slideshow.rs` — local module removed
- `examples/f469disco/nt35510.rs` — deleted (only after all verification passes)

### Definition of Done
- [ ] `cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost,defmt" --target thumbv7em-none-eabihf` passes
- [ ] `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf` passes (all examples)
- [ ] Hardware flash and run of f469disco-lcd-test on STM32F469I-DISCO B08 — display shows test pattern
- [ ] Hardware flash and run of at least one board.rs-dependent example (f469disco-hello-eg)
- [ ] `examples/f469disco/nt35510.rs` deleted from repository
- [ ] No regressions in f469disco-touch-debug.rs (unmodified, should still compile)

### Must Have
- External `nt35510 = "0.1.0"` crate used by all examples
- Correct `DelayNs` trait bounds where the external crate requires embedded-hal 1.0
- Error type references updated from `Nt35510Error` to `Error`
- Hardware verification on real STM32F469I-DISCO B08 board
- Local `nt35510.rs` file deleted after all verifications pass

### Must NOT Have (Guardrails)
- DO NOT upgrade otm8009a or change its usage beyond adding `+ DelayNs` to shared function signatures
- DO NOT refactor board.rs beyond nt35510 migration (no reformatting, no logic changes)
- DO NOT change f469disco-touch-debug.rs (no nt35510 dependency)
- DO NOT add nt35510 to `[dependencies]` — only `[dev-dependencies]`
- DO NOT use `init_rgb565` or `init_with_config` — use `init()` only (preserves RGB888 behavior)
- DO NOT change the LCD detection/probe algorithm logic
- DO NOT delete `examples/f469disco/nt35510.rs` until ALL examples pass hardware verification
- DO NOT fix pre-existing warnings in unrelated code
- DO NOT add `defmt` as a dependency of the nt35510 crate itself
- DO NOT modify any files in `src/` — this migration is examples-only

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (cargo build/check with target, probe-rs for hardware)
- **Automated tests**: None (embedded examples — verified by compilation + hardware flash)
- **Framework**: cargo build + probe-rs (hardware flash & run)
- **Compilation is the primary test** — if it compiles for the target, the API contract is satisfied

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Compilation**: Use Bash — `cargo build`/`cargo check` with exact features and target
- **Hardware flash**: Use Bash (SSH) — SCP binary, probe-rs download/reset/attach
- **Code inspection**: Use Grep/Read — verify no leftover references to local module

### Remote Board Configuration
- **Host**: `ubuntu@192.168.13.246`
- **Chip**: `STM32F469NIHx`
- **Board revision**: B08 (NT35510 display)
- **SSH pattern**: `ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && <command>"`
- **JTAG recovery**: If `probe-rs` fails with `JtagNoDeviceConnected`, use `--connect-under-reset`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — Cargo.toml + lcd-test migration):
├── Task 1: Add nt35510 dependency + migrate f469disco-lcd-test.rs [quick]

Wave 2 (After Wave 1 — compile + hardware verify lcd-test):
├── Task 2: Compile f469disco-lcd-test (defmt + non-defmt) [quick]
├── Task 3: Hardware flash & verify f469disco-lcd-test [quick]

Wave 3 (After Wave 2 — migrate board.rs + 4 examples):
├── Task 4: Migrate board.rs to external crate [deep]
├── Task 5: Remove #[path] includes from 4 board.rs-dependent examples [quick]

Wave 4 (After Wave 3 — compile all + hardware verify + cleanup):
├── Task 6: Compile all examples + hardware verify hello-eg [quick]
├── Task 7: Delete local nt35510.rs + final verification [quick]

Wave FINAL (After ALL tasks — independent review):
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Real manual QA - hardware [unspecified-high]
├── Task F4: Scope fidelity check [deep]

Critical Path: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7 → F1-F4
```

Note: This migration is largely sequential because each wave depends on the previous one's verification. Tasks 4+5 within Wave 3 can run in parallel. Final verification tasks F1-F4 run in parallel.

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 2, 3 | 1 |
| 2 | 1 | 3 | 2 |
| 3 | 2 | 4, 5 | 2 |
| 4 | 3 | 6 | 3 |
| 5 | 3 | 6 | 3 |
| 6 | 4, 5 | 7 | 4 |
| 7 | 6 | F1-F4 | 4 |
| F1-F4 | 7 | — | FINAL |

### Agent Dispatch Summary

- **Wave 1**: **1 task** — T1 → `quick`
- **Wave 2**: **2 tasks** — T2 → `quick`, T3 → `quick`
- **Wave 3**: **2 tasks (parallel)** — T4 → `deep`, T5 → `quick`
- **Wave 4**: **2 tasks** — T6 → `quick`, T7 → `quick`
- **FINAL**: **4 tasks (parallel)** — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> Implementation tasks follow. EVERY task has: Recommended Agent Profile + QA Scenarios.
> **A task WITHOUT QA Scenarios is INCOMPLETE. No exceptions.**

- [x] 1. Add nt35510 dependency and migrate f469disco-lcd-test.rs

  **What to do**:
  - In `Cargo.toml`, add `nt35510 = "0.1.0"` to `[dev-dependencies]` after the `otm8009a = "0.1"` line (line 127)
  - In `examples/f469disco-lcd-test.rs`:
    - **Remove** lines 17-18: `#[path = "f469disco/nt35510.rs"] mod nt35510;`
    - **Change** `detect_lcd_controller` signature (line 429): replace `delay: &mut impl embedded_hal_02::blocking::delay::DelayUs<u32>` with `delay: &mut impl embedded_hal_02::blocking::delay::DelayUs<u32> + embedded_hal::delay::DelayNs` — the function calls `nt35510.probe()` which needs `DelayNs`, and also calls `delay.delay_us(20_000u32)` which needs the old trait
    - **Rename** error types in 3 match arms:
      - Line 451: `Err(nt35510::Nt35510Error::DsiRead)` → `Err(nt35510::Error::DsiRead)`
      - Line 456: `Err(nt35510::Nt35510Error::DsiWrite)` → `Err(nt35510::Error::DsiWrite)`
      - Line 460: `Err(nt35510::Nt35510Error::ProbeMismatch(id))` → `Err(nt35510::Error::ProbeMismatch(id))`
    - **Fix** defmt panic (line 221): `defmt::panic!("NT35510 init failed: {:?}", _e)` → `defmt::panic!("NT35510 init failed")` — external crate's `Error` does NOT implement `defmt::Format`, so the `{:?}` format (which uses defmt's Debug, not core::fmt::Debug) won't compile. Also change `Err(_e)` to `Err(_)` since `_e` is no longer used.
    - **Add** `use embedded_hal::delay::DelayNs;` import near the top (after existing use statements around line 20)
    - **Important**: The call site at line 219 `nt35510.init(&mut dsi_host, &mut delay)` should work as-is because `DsiHost` implements `DsiHostCtrlIo` and `SysDelay` (which is the concrete type of `delay`) implements `DelayNs`

  **Must NOT do**:
  - DO NOT change otm8009a initialization code
  - DO NOT change LCD detection algorithm logic
  - DO NOT modify any `src/` files
  - DO NOT add nt35510 to `[dependencies]` (only `[dev-dependencies]`)
  - DO NOT use `init_rgb565` or `init_with_config` — keep using `init()`

  **Recommended Agent Profile**:
  > This is a straightforward find-and-replace migration with well-defined changes.
  - **Category**: `quick`
    - Reason: Small, well-defined edits to 2 files with exact line numbers provided
  - **Skills**: []
    - No special skills needed — standard file editing
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed — commit handled separately

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (solo)
  - **Blocks**: Tasks 2, 3
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References** (existing code to follow):
  - `Cargo.toml:127` — `otm8009a = "0.1"` line showing where to add nt35510 dependency (add after this line)
  - `examples/f469disco-lcd-test.rs:17-18` — The `#[path]` module include lines to remove
  - `examples/f469disco-lcd-test.rs:427-430` — `detect_lcd_controller` function signature to modify
  - `examples/f469disco-lcd-test.rs:451-473` — Three error match arms to rename (`Nt35510Error` → `Error`)
  - `examples/f469disco-lcd-test.rs:219-221` — NT35510 init + defmt panic to fix

  **API/Type References** (contracts to implement against):
  - External crate `nt35510` v0.1.0 API: `Nt35510::new()`, `.init(&mut dsi_host, &mut delay)`, `.probe(&mut dsi_host, &mut delay)`, `Error::{DsiRead, DsiWrite, ProbeMismatch(u8), InvalidDimensions}`
  - `embedded_hal::delay::DelayNs` — trait required by external crate's `init()` and `probe()` methods
  - `embedded_hal_02::blocking::delay::DelayUs<u32>` — still needed by `delay.delay_us(20_000u32)` calls within the function

  **Compatibility References** (verified compatible):
  - `src/dsi.rs:582` — `DsiHost` implements `DsiHostCtrlIo` (external crate's generic interface)
  - `src/timer/hal_1.rs:12` — `SysDelay` implements `DelayNs` (embedded-hal 1.0)

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Cargo.toml has correct dependency
    Tool: Bash (grep)
    Preconditions: Task 1 edits applied
    Steps:
      1. Run: grep 'nt35510' Cargo.toml
      2. Verify output contains exactly: nt35510 = "0.1.0"
      3. Verify it appears in [dev-dependencies] section (between line ~100 and line ~134)
      4. Run: grep -c 'nt35510' Cargo.toml — verify count is exactly 1 (only in dev-deps, not in [dependencies])
    Expected Result: Single line `nt35510 = "0.1.0"` in [dev-dependencies] only
    Failure Indicators: Line appears in [dependencies], or count != 1, or version differs
    Evidence: .sisyphus/evidence/task-1-cargo-dep.txt

  Scenario: No references to local nt35510 module in lcd-test
    Tool: Bash (grep)
    Preconditions: Task 1 edits applied
    Steps:
      1. Run: grep -n 'f469disco/nt35510' examples/f469disco-lcd-test.rs
      2. Run: grep -n 'Nt35510Error' examples/f469disco-lcd-test.rs
    Expected Result: Both commands produce empty output (no matches)
    Failure Indicators: Any match found — means old references remain
    Evidence: .sisyphus/evidence/task-1-no-local-refs.txt

  Scenario: Error types correctly renamed
    Tool: Bash (grep)
    Preconditions: Task 1 edits applied
    Steps:
      1. Run: grep -n 'nt35510::Error::' examples/f469disco-lcd-test.rs
      2. Verify exactly 3 matches: DsiRead, DsiWrite, ProbeMismatch(id)
    Expected Result: 3 lines with nt35510::Error:: pattern
    Failure Indicators: Fewer than 3 matches, or old Nt35510Error pattern found
    Evidence: .sisyphus/evidence/task-1-error-types.txt

  Scenario: defmt panic doesn't reference Error type
    Tool: Bash (grep)
    Preconditions: Task 1 edits applied
    Steps:
      1. Run: grep -n 'defmt::panic!' examples/f469disco-lcd-test.rs
      2. Verify the NT35510-related panic does NOT contain {:?} or _e
    Expected Result: `defmt::panic!("NT35510 init failed")` — no format args
    Failure Indicators: Still contains {:?} or _e reference
    Evidence: .sisyphus/evidence/task-1-defmt-panic.txt
  ```

  **Evidence to Capture:**
  - [ ] task-1-cargo-dep.txt — grep output showing nt35510 in dev-deps
  - [ ] task-1-no-local-refs.txt — grep output confirming no local module references
  - [ ] task-1-error-types.txt — grep output showing renamed error types
  - [ ] task-1-defmt-panic.txt — grep output showing fixed defmt panic

  **Commit**: YES
  - Message: `refactor(examples): migrate f469disco-lcd-test to external nt35510 crate`
  - Files: `Cargo.toml`, `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost,defmt" --target thumbv7em-none-eabihf`

---

- [x] 2. Compile f469disco-lcd-test (defmt and non-defmt builds)

  **What to do**:
  - Build with defmt: `cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost,defmt" --target thumbv7em-none-eabihf`
  - Build without defmt: `cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost" --target thumbv7em-none-eabihf`
  - If either fails, diagnose and fix the compilation errors in the files from Task 1
  - Common failure modes:
    - Missing `use embedded_hal::delay::DelayNs;` import
    - Trait bound mismatch — `DelayNs` not in scope or not added to function signature
    - Error type name mismatch — leftover `Nt35510Error` references
    - defmt format issue — leftover `{:?}` with Error type that lacks `defmt::Format`

  **Must NOT do**:
  - DO NOT modify `src/` files to fix compilation — all fixes must be in examples/Cargo.toml
  - DO NOT add feature flags to the nt35510 dependency

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running build commands and potentially fixing small compilation issues
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None relevant

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 3
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - All files modified in Task 1 — these are what's being compiled
  - `examples/f469disco-lcd-test.rs` — full file, focus on import section and detect_lcd_controller

  **Build References**:
  - `.cargo/config.toml` — Cargo build configuration for cross-compilation
  - `memory.x` — Linker script for STM32F469

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Build succeeds with defmt feature
    Tool: Bash
    Preconditions: Task 1 committed
    Steps:
      1. Run: cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost,defmt" --target thumbv7em-none-eabihf
      2. Check exit code is 0
      3. Verify binary exists: ls -la target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test
    Expected Result: Build succeeds, binary file exists at expected path
    Failure Indicators: Compilation errors, missing binary
    Evidence: .sisyphus/evidence/task-2-build-defmt.txt

  Scenario: Build succeeds without defmt feature
    Tool: Bash
    Preconditions: Task 1 committed
    Steps:
      1. Run: cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost" --target thumbv7em-none-eabihf
      2. Check exit code is 0
    Expected Result: Build succeeds without defmt
    Failure Indicators: Compilation errors related to conditional compilation
    Evidence: .sisyphus/evidence/task-2-build-no-defmt.txt
  ```

  **Evidence to Capture:**
  - [ ] task-2-build-defmt.txt — full cargo build output with defmt
  - [ ] task-2-build-no-defmt.txt — full cargo build output without defmt

  **Commit**: NO (compilation verification only, no code changes unless fixes needed)

---

- [x] 3. Hardware flash and verify f469disco-lcd-test on STM32F469I-DISCO

  **What to do**:
  - SCP the compiled binary to remote board:
    `scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/`
  - Flash via probe-rs:
    `ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test && probe-rs reset --chip STM32F469NIHx"`
  - Attach to read defmt output:
    `ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-lcd-test 2>&1"`
  - If flash fails with `JtagNoDeviceConnected`, retry with `--connect-under-reset` flag
  - **Success criteria**: defmt output shows "NT35510 (B08) detected successfully" and "Initializing NT35510" messages, no panics

  **Must NOT do**:
  - DO NOT modify any source files during this task — this is verification only
  - DO NOT proceed to Task 4 if hardware verification fails — debug and fix Task 1 first

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running SSH/SCP commands and checking output
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `playwright`: Not needed — no browser interaction

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (after Task 2)
  - **Blocks**: Tasks 4, 5
  - **Blocked By**: Task 2

  **References**:

  **Hardware References**:
  - Board: STM32F469I-DISCO rev B08 with NT35510 LCD controller
  - SSH host: `ubuntu@192.168.13.246`
  - Chip: `STM32F469NIHx`
  - Recovery: `--connect-under-reset` flag if JTAG fails

  **Expected Output References**:
  - defmt should show: `Auto-detecting LCD controller...` → `NT35510 (B08) detected successfully on attempt 1` → `Initializing NT35510 (B08 revision)`
  - After init: color test pattern display cycle

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Binary flashes successfully to STM32F469I-DISCO
    Tool: Bash (SSH)
    Preconditions: Task 2 build succeeded, binary at target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test
    Steps:
      1. Run: scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/
      2. Run: ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test"
      3. Check exit code is 0
      4. Run: ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs reset --chip STM32F469NIHx"
    Expected Result: SCP succeeds, probe-rs download succeeds, reset succeeds
    Failure Indicators: SSH timeout, probe-rs "JtagNoDeviceConnected" (retry with --connect-under-reset), probe-rs download failure
    Evidence: .sisyphus/evidence/task-3-flash.txt

  Scenario: NT35510 initializes correctly on hardware
    Tool: Bash (SSH)
    Preconditions: Binary flashed successfully
    Steps:
      1. Run: ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-lcd-test 2>&1"
      2. Check output contains "NT35510" and "detected successfully"
      3. Check output does NOT contain "panic" or "PANIC"
    Expected Result: defmt output shows successful NT35510 detection and initialization
    Failure Indicators: Panic message, no output (timeout), "OTM8009A" detected instead of NT35510
    Evidence: .sisyphus/evidence/task-3-hardware-verify.txt
  ```

  **Evidence to Capture:**
  - [ ] task-3-flash.txt — probe-rs download/reset output
  - [ ] task-3-hardware-verify.txt — probe-rs attach defmt output showing NT35510 init

  **Commit**: NO (verification only)

---

- [x] 4. Migrate board.rs to external nt35510 crate

  **What to do**:
  - In `examples/f469disco/board.rs`:
    - **Change** line 29: `use super::nt35510::Nt35510;` → `use nt35510::Nt35510;` (resolves to external crate via Rust 2021 edition)
    - **Add** `use embedded_hal::delay::DelayNs;` import (near line 26, after other use statements)
    - **Add** `+ DelayNs` to trait bounds on these functions:
      - `detect_lcd_controller` (line 115): add `+ DelayNs` to the delay parameter
      - `init_panel` (line 260): add `+ DelayNs` to the delay parameter
      - `init_display_full` (line 330): add `+ DelayNs` to the delay parameter
    - **Rename** error types in match arms:
      - Line 137: `super::nt35510::Nt35510Error::DsiRead` → `nt35510::Error::DsiRead`
      - Line 142: `super::nt35510::Nt35510Error::DsiWrite` → `nt35510::Error::DsiWrite`
      - Line 147: `super::nt35510::Nt35510Error::ProbeMismatch` → `nt35510::Error::ProbeMismatch`
    - **Important**: `init_dsi_with_delay` (line 247) takes `impl DelayMs<u32>` only — does NOT need `+ DelayNs` since it doesn't call nt35510 methods directly. DO NOT modify this function.
    - **Important**: The `init_panel` function (line 260) calls `nt35510.init()` which requires `DelayNs`, so its delay parameter MUST have `+ DelayNs`
    - **Important**: The `init_display_full` function (line 330) calls `init_panel` internally, so it must propagate the `+ DelayNs` bound

  **Must NOT do**:
  - DO NOT modify `init_dsi_with_delay` — it doesn't use nt35510
  - DO NOT change otm8009a usage or imports
  - DO NOT refactor board.rs logic beyond nt35510 migration
  - DO NOT change formatting or restructure the file
  - DO NOT modify any `src/` files

  **Recommended Agent Profile**:
  > This task requires careful analysis of trait bound propagation across multiple functions.
  - **Category**: `deep`
    - Reason: Multiple interconnected function signatures need coordinated trait bound changes; must understand call graph to avoid breaking callers
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed — commit handled in Task 5

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 5)
  - **Parallel Group**: Wave 3 (with Task 5)
  - **Blocks**: Task 6
  - **Blocked By**: Task 3

  **References**:

  **Pattern References** (existing code to follow):
  - `examples/f469disco/board.rs:29` — Current `use super::nt35510::Nt35510;` import to change
  - `examples/f469disco/board.rs:115` — `detect_lcd_controller` function signature (needs `+ DelayNs`)
  - `examples/f469disco/board.rs:137-147` — Error match arms to rename (3 arms: DsiRead, DsiWrite, ProbeMismatch)
  - `examples/f469disco/board.rs:247` — `init_dsi_with_delay` — DO NOT modify (no nt35510 usage)
  - `examples/f469disco/board.rs:260` — `init_panel` function signature (needs `+ DelayNs`)
  - `examples/f469disco/board.rs:330` — `init_display_full` function signature (needs `+ DelayNs`)

  **API/Type References**:
  - Same external crate API as Task 1 — `Nt35510::new()`, `.init()`, `.probe()`, `Error::{...}`
  - `embedded_hal::delay::DelayNs` — required by external crate

  **Compatibility References**:
  - Task 1 already proved the external crate works with `DsiHost` and `SysDelay` — this uses the same types

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: board.rs uses external crate imports
    Tool: Bash (grep)
    Preconditions: Task 4 edits applied
    Steps:
      1. Run: grep -n 'super::nt35510' examples/f469disco/board.rs
      2. Run: grep -n 'use nt35510::Nt35510' examples/f469disco/board.rs
    Expected Result: No matches for super::nt35510, one match for use nt35510::Nt35510
    Failure Indicators: Any super::nt35510 reference remains
    Evidence: .sisyphus/evidence/task-4-imports.txt

  Scenario: Error types correctly renamed in board.rs
    Tool: Bash (grep)
    Preconditions: Task 4 edits applied
    Steps:
      1. Run: grep -n 'Nt35510Error' examples/f469disco/board.rs
      2. Run: grep -n 'nt35510::Error::' examples/f469disco/board.rs
    Expected Result: No Nt35510Error matches, 3 nt35510::Error:: matches
    Failure Indicators: Old error type names remain
    Evidence: .sisyphus/evidence/task-4-error-types.txt

  Scenario: DelayNs trait bound added to correct functions only
    Tool: Bash (grep)
    Preconditions: Task 4 edits applied
    Steps:
      1. Run: grep -n 'DelayNs' examples/f469disco/board.rs
      2. Verify matches appear on: detect_lcd_controller, init_panel, init_display_full signatures + import
      3. Run: grep -n 'init_dsi_with_delay' examples/f469disco/board.rs
      4. Verify init_dsi_with_delay does NOT contain DelayNs
    Expected Result: DelayNs on 3 function signatures + 1 import = 4 matches; init_dsi_with_delay unchanged
    Failure Indicators: DelayNs on init_dsi_with_delay, or missing from required functions
    Evidence: .sisyphus/evidence/task-4-delay-bounds.txt
  ```

  **Evidence to Capture:**
  - [ ] task-4-imports.txt — grep output showing external crate imports
  - [ ] task-4-error-types.txt — grep output showing renamed error types
  - [ ] task-4-delay-bounds.txt — grep output showing DelayNs on correct functions only

  **Commit**: YES (groups with Task 5)
  - Message: `refactor(examples): migrate board.rs and remaining examples to external nt35510 crate`
  - Files: `examples/f469disco/board.rs`, `examples/f469disco-hello-eg.rs`, `examples/f469disco-paint.rs`, `examples/f469disco-image-slider.rs`, `examples/f469disco-slideshow.rs`
  - Pre-commit: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf`

---

- [x] 5. Remove #[path] includes from 4 board.rs-dependent examples

  **What to do**:
  - Remove the `#[path = "f469disco/nt35510.rs"] mod nt35510;` lines from these 4 files:
    - `examples/f469disco-hello-eg.rs` — lines 40-41
    - `examples/f469disco-paint.rs` — lines 45-46
    - `examples/f469disco-image-slider.rs` — lines 39-40
    - `examples/f469disco-slideshow.rs` — lines 36-37
  - Each removal is 2 lines: the `#[path = ...]` attribute and the `mod nt35510;` declaration
  - These examples use board.rs which now imports from the external crate, so they no longer need the local module include
  - **Important**: These examples do NOT directly reference `nt35510::` types in their own code — all nt35510 usage is inside board.rs. So removing the `#[path]` module include is the ONLY change needed.

  **Must NOT do**:
  - DO NOT modify any other lines in these files
  - DO NOT change f469disco-touch-debug.rs (it has no nt35510 reference)
  - DO NOT change f469disco-lcd-test.rs (already done in Task 1)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Removing 2 identical lines from 4 files — trivial mechanical edit
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None relevant

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 4)
  - **Parallel Group**: Wave 3 (with Task 4)
  - **Blocks**: Task 6
  - **Blocked By**: Task 3

  **References**:

  **Pattern References** (exact lines to remove):
  - `examples/f469disco-hello-eg.rs:40-41` — `#[path = "f469disco/nt35510.rs"]` + `mod nt35510;`
  - `examples/f469disco-paint.rs:45-46` — `#[path = "f469disco/nt35510.rs"]` + `mod nt35510;`
  - `examples/f469disco-image-slider.rs:39-40` — `#[path = "f469disco/nt35510.rs"]` + `mod nt35510;`
  - `examples/f469disco-slideshow.rs:36-37` — `#[path = "f469disco/nt35510.rs"]` + `mod nt35510;`

  **Verification Reference**:
  - `examples/f469disco-lcd-test.rs:17-18` — Same pattern was already removed in Task 1 (use as reference)

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: No #[path] nt35510 references remain in any example
    Tool: Bash (grep)
    Preconditions: Task 5 edits applied
    Steps:
      1. Run: grep -rn 'f469disco/nt35510' examples/
      2. Run: grep -rn '#\[path.*nt35510' examples/
    Expected Result: Both commands produce empty output — no matches anywhere
    Failure Indicators: Any match found means a file was missed
    Evidence: .sisyphus/evidence/task-5-no-path-refs.txt

  Scenario: Only the #[path] lines were removed (no collateral damage)
    Tool: Bash (grep)
    Preconditions: Task 5 edits applied
    Steps:
      1. For each of the 4 files, verify line count decreased by exactly 2:
         wc -l examples/f469disco-hello-eg.rs examples/f469disco-paint.rs examples/f469disco-image-slider.rs examples/f469disco-slideshow.rs
      2. Compare with expected counts (original - 2 each)
    Expected Result: Each file has exactly 2 fewer lines than before
    Failure Indicators: More or fewer lines removed — indicates collateral damage or missed lines
    Evidence: .sisyphus/evidence/task-5-line-counts.txt
  ```

  **Evidence to Capture:**
  - [ ] task-5-no-path-refs.txt — grep output confirming zero #[path] references to nt35510
  - [ ] task-5-line-counts.txt — wc -l output showing correct line counts

  **Commit**: YES (groups with Task 4)
  - Message: `refactor(examples): migrate board.rs and remaining examples to external nt35510 crate`
  - Files: (same commit as Task 4)
  - Pre-commit: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf`

---

- [x] 6. Compile all examples and hardware verify f469disco-hello-eg

  **What to do**:
  - Compile ALL f469disco examples together:
    `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf`
  - Build hello-eg for hardware test:
    `cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf`
  - SCP and flash hello-eg to hardware:
    `scp target/thumbv7em-none-eabihf/release/examples/f469disco-hello-eg ubuntu@192.168.13.246:/tmp/`
    `ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-hello-eg && probe-rs reset --chip STM32F469NIHx"`
  - Attach for defmt output:
    `ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-hello-eg 2>&1"`
  - If any compilation fails, diagnose and fix in the files from Tasks 4/5
  - **Note**: hello-eg uses `embedded_graphics` and the `framebuffer` feature — verify these features are included in the build command

  **Must NOT do**:
  - DO NOT modify `src/` files
  - DO NOT proceed to Task 7 if compilation or hardware test fails — fix Tasks 4/5 first

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running build commands, SCP, and SSH for hardware flash
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None relevant

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 4, 5

  **References**:

  **Build References**:
  - Same build toolchain as Tasks 2/3
  - `examples/f469disco-hello-eg.rs` — requires `framebuffer` feature in addition to standard features

  **Hardware References**:
  - Same remote board configuration as Task 3

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All f469disco examples compile together
    Tool: Bash
    Preconditions: Tasks 4 and 5 committed
    Steps:
      1. Run: cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf
      2. Check exit code is 0
    Expected Result: All examples compile without errors
    Failure Indicators: Compilation errors in any example
    Evidence: .sisyphus/evidence/task-6-compile-all.txt

  Scenario: f469disco-hello-eg runs on hardware
    Tool: Bash (SSH)
    Preconditions: hello-eg binary built successfully
    Steps:
      1. Build: cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf
      2. SCP: scp target/thumbv7em-none-eabihf/release/examples/f469disco-hello-eg ubuntu@192.168.13.246:/tmp/
      3. Flash: ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-hello-eg && probe-rs reset --chip STM32F469NIHx"
      4. Attach: ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-hello-eg 2>&1"
      5. Check output shows NT35510 detection and no panics
    Expected Result: hello-eg initializes display and shows text on screen, no panics
    Failure Indicators: Panic, timeout, wrong LCD controller detected
    Evidence: .sisyphus/evidence/task-6-hello-eg-hardware.txt

  Scenario: f469disco-touch-debug still compiles (regression check)
    Tool: Bash
    Preconditions: All previous tasks committed
    Steps:
      1. Run: cargo check --example f469disco-touch-debug --features="stm32f469,stm32-fmc,dsihost" --target thumbv7em-none-eabihf
      2. Check exit code is 0
    Expected Result: touch-debug compiles without errors (was not modified)
    Failure Indicators: Compilation error — means we accidentally affected an unrelated example
    Evidence: .sisyphus/evidence/task-6-touch-debug-regression.txt
  ```

  **Evidence to Capture:**
  - [ ] task-6-compile-all.txt — cargo check output for all examples
  - [ ] task-6-hello-eg-hardware.txt — probe-rs attach output from hello-eg
  - [ ] task-6-touch-debug-regression.txt — cargo check output for touch-debug

  **Commit**: NO (verification only)

---

- [x] 7. Delete local nt35510.rs and final verification

  **What to do**:
  - Delete `examples/f469disco/nt35510.rs` — the local driver file is now fully replaced by the external crate
  - Verify all examples still compile after deletion:
    `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf`
  - Verify no remaining references to the deleted file:
    `grep -rn 'f469disco/nt35510' examples/`
    `grep -rn 'f469disco/nt35510' Cargo.toml`

  **Must NOT do**:
  - DO NOT delete any other files
  - DO NOT modify any other files in this task

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file deletion + verification commands
  - **Skills**: [`git-master`]
    - `git-master`: Clean commit of file deletion
  - **Skills Evaluated but Omitted**:
    - None relevant

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (after Task 6)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 6

  **References**:

  **File to Delete**:
  - `examples/f469disco/nt35510.rs` — 154 lines, the local NT35510 driver that is now replaced by `nt35510 = "0.1.0"` from crates.io

  **Verification References**:
  - All the same compilation commands used in Task 6 — should still pass after deletion since no file imports the local module anymore

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Local nt35510.rs is deleted
    Tool: Bash
    Preconditions: Tasks 1-6 all verified
    Steps:
      1. Run: rm examples/f469disco/nt35510.rs
      2. Run: ls examples/f469disco/nt35510.rs
    Expected Result: "No such file or directory"
    Failure Indicators: File still exists
    Evidence: .sisyphus/evidence/task-7-file-deleted.txt

  Scenario: All examples still compile after deletion
    Tool: Bash
    Preconditions: nt35510.rs deleted
    Steps:
      1. Run: cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf
      2. Check exit code is 0
    Expected Result: All examples compile — no example depends on the deleted file
    Failure Indicators: Compilation error mentioning nt35510.rs — means a reference was missed
    Evidence: .sisyphus/evidence/task-7-compile-after-delete.txt

  Scenario: No references to deleted file remain anywhere
    Tool: Bash (grep)
    Preconditions: nt35510.rs deleted
    Steps:
      1. Run: grep -rn 'f469disco/nt35510' examples/ Cargo.toml
      2. Run: grep -rn 'mod nt35510' examples/ --include='*.rs'
    Expected Result: No matches for the path reference; mod nt35510 only appears inside board.rs submodule context if at all
    Failure Indicators: Any stale path reference to the deleted file
    Evidence: .sisyphus/evidence/task-7-no-stale-refs.txt
  ```

  **Evidence to Capture:**
  - [ ] task-7-file-deleted.txt — ls showing file doesn't exist
  - [ ] task-7-compile-after-delete.txt — cargo check output after deletion
  - [ ] task-7-no-stale-refs.txt — grep output showing no stale references

  **Commit**: YES
  - Message: `chore(examples): remove local nt35510.rs driver (replaced by crates.io crate)`
  - Files: delete `examples/f469disco/nt35510.rs`
  - Pre-commit: `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf`

---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run cargo check). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf`. Review all changed files for: unused imports, dead code warnings, `#[allow(unused)]` additions. Check that no otm8009a code was modified beyond trait bound additions. Verify no `src/` files were touched.
  Output: `Build [PASS/FAIL] | Warnings [N] | Scope [CLEAN/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Flash f469disco-lcd-test to hardware. Verify display shows test pattern (probe-rs attach output should show initialization messages). Flash f469disco-hello-eg. Verify display shows "Hello World". Test f469disco-touch-debug still compiles. Save evidence to `.sisyphus/evidence/final-qa/`.
  Output: `lcd-test [PASS/FAIL] | hello-eg [PASS/FAIL] | touch-debug compile [PASS/FAIL] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git diff). Verify 1:1 — everything in spec was built, nothing beyond spec. Check "Must NOT do" compliance. Verify no files in `src/` were modified. Verify `f469disco-touch-debug.rs` is unchanged. Verify `examples/f469disco/nt35510.rs` was deleted. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **After Task 1**: `refactor(examples): migrate f469disco-lcd-test to external nt35510 crate` — Cargo.toml, examples/f469disco-lcd-test.rs
- **After Task 5**: `refactor(examples): migrate board.rs and remaining examples to external nt35510 crate` — examples/f469disco/board.rs, examples/f469disco-hello-eg.rs, examples/f469disco-paint.rs, examples/f469disco-image-slider.rs, examples/f469disco-slideshow.rs
- **After Task 7**: `chore(examples): remove local nt35510.rs driver (replaced by crates.io crate)` — delete examples/f469disco/nt35510.rs

---

## Success Criteria

### Verification Commands
```bash
# All examples compile
cargo check --features=stm32f469,stm32-fmc,framebuffer,dsihost --examples --target thumbv7em-none-eabihf
# Expected: Finished `dev` profile, no errors

# lcd-test compiles with defmt
cargo build --release --example f469disco-lcd-test --features="stm32f469,stm32-fmc,dsihost,defmt" --target thumbv7em-none-eabihf
# Expected: Finished `release` profile

# No references to local nt35510 module remain
grep -r "f469disco/nt35510.rs" examples/
# Expected: no output (empty)

# Local file is deleted
ls examples/f469disco/nt35510.rs
# Expected: No such file or directory
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All examples compile
- [ ] Hardware verification passed for lcd-test and hello-eg
- [ ] Local nt35510.rs deleted
