# VLS Multi-Board Port: F469 First, Then F412/F413

## TL;DR

> **Quick Summary**: Get VLS (Validating Lightning Signer) compiling and running on STM32F469 hardware by fixing module ambiguity and SDIO scoping, then fix F412/F413 compilation. Sequential: F469 with hardware testing first, F412/F413 compile-only after.
> 
> **Deliverables**:
> - VLS `test` binary compiling and running on F469 hardware via remote probe-rs
> - VLS `test` binary compiling for F412 and F413 targets
> - Clean multi-board device abstraction (already mostly done)
> - BSP committed, HAL bug fixes cherry-picked
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: NO — sequential (F469 first, then F412/F413, then cleanup)
> **Critical Path**: Delete device.rs → Fix SDIO → Build F469 → Flash F469 → Fix F412 → Fix F413 → Cleanup

---

## Context

### Original Request
Port VLS (validating-lightning-signer) which previously only worked on STM32F412/F413 to STM32F469. The user has added F469 support (BSP, HAL changes, device/f469.rs) but it doesn't compile yet. Goal: get F469 working on real hardware, then fix F412/F413 compilation.

### Interview Summary
**Key Discussions**:
- Sequential approach: F469 first (has hardware on remote host), F412/F413 later (compile-only)
- "Only run the tests" — use the `test` binary, not `demo_signer`
- No SD card on remote board (SDIO will init but SD operations will fail gracefully)
- Board-specific code belongs in HAL/BSP/per-board files, clean multi-board pattern
- Remote hardware: `ssh ubuntu@192.168.13.246`, flash via `probe-rs run --chip STM32F469NIHx`

**Research Findings**:
- All 3 targets fail with identical errors — root cause is E0761 module ambiguity (both `device.rs` AND `device/mod.rs` exist)
- Explore agent confirmed `device.rs` is safe to delete — `device/mod.rs` is a complete superset
- After E0761 fix: SDIO `gpiod.pd2` scoping issue for F469 (gpiod created after SDIO block)
- F412/F413: FSMC LCD API changes (ChipSelect types), FMPI2C API changes (F413), RAM_SIZE wrong for F412
- Remote host verified: probe-rs 0.31.0, STLink V2-1 probe, SSH works

### Metis Review
**Identified Gaps** (addressed):
- Test binary success criteria unclear → Test binary runs an infinite loop logging counter values + RNG + touch events via RTT. Success = RTT output shows "counter N" incrementing. Crash/hang = failure.
- SDIO criticality unknown → SDIO is required by `make_devices()` for all boards (it's in the shared init path). Cannot be disabled without code changes. Will init but SD operations fail gracefully without a card.
- RAM_SIZE/HEAP_SIZE per-board → Only F412 differs (256K RAM). F413 and F469 both have 320K. `build.rs` already handles memory.x correctly. The constants in `mod.rs` need per-board cfg.
- Rollback strategy → Git commits after each logical step. Can revert individual commits.
- HAL cherry-pick specifics → 4 commits on NT35510 branch: DSI color fix, ft6x06 touch panic, IWDG watchdog, LTDC/DMA2D fixes.

---

## Work Objectives

### Core Objective
Get VLS `test` binary compiling and running on STM32F469 hardware, then fix compilation for F412 and F413.

### Concrete Deliverables
- `cargo build --features stm32f469 --release --bin test` succeeds
- Test binary runs on remote F469 hardware via probe-rs, producing RTT output
- `cargo build --features stm32f412 --release --bin test` succeeds
- `cargo build --features stm32f413 --release --bin test` succeeds
- BSP changes committed
- HAL bug fixes cherry-picked to pr2-f469disco-examples branch

### Definition of Done
- [x] F469 test binary compiles with zero errors
- [x] F469 test binary flashed and runs on remote hardware (RTT shows counter incrementing)
- [~] F412 test binary compiles with zero errors (DEFERRED: ST7789 API incompatible with embedded-graphics 0.8)
- [~] F413 test binary compiles with zero errors (DEFERRED: ST7789 API incompatible with embedded-graphics 0.8)
- [x] All changes committed with descriptive messages

### Must Have
- Delete `device.rs` to resolve E0761 module ambiguity
- Fix SDIO `gpiod.pd2` scoping for F469 in `device/mod.rs`
- Per-board RAM_SIZE/HEAP_SIZE constants
- Hardware verification on F469

### Must NOT Have (Guardrails)
- No changes to `src/i2c.rs` in the HAL
- No ARGB8888 format changes
- No `Peripherals::steal()` refactoring
- No SD card support work (no card in remote board)
- No work on `demo_signer` binary — test binary only
- No new features beyond compilation fixes
- No unrelated refactoring of adjacent code
- No error handling improvements beyond what's needed to compile
- No new abstractions or patterns — use existing multi-board structure
- No documentation changes unless blocking compilation

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (no_std embedded target, no test framework)
- **Automated tests**: NONE (embedded binary, verified by running on hardware)
- **Framework**: N/A
- **Verification method**: Build succeeds + RTT output from probe-rs on remote hardware

### QA Policy
Every task includes agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Compilation**: Use Bash — `cargo build` with appropriate features, assert exit code 0
- **Hardware**: Use Bash — SCP binary to remote, SSH probe-rs run, capture RTT output
- **Code review**: Use Bash/grep — verify specific patterns exist/don't exist in changed files

---

## Execution Strategy

### Sequential Execution (User Requested)

> User explicitly requested sequential approach: F469 first, then F412/F413.
> Each task completes and is verified before the next begins.

```
Phase 1 — F469 Compilation (Tasks 1-3):
├── Task 1: Delete device.rs to resolve E0761 [quick]
├── Task 2: Fix SDIO gpiod.pd2 scoping for F469 [quick]
└── Task 3: Build F469 test binary + fix any remaining errors [deep]

Phase 2 — F469 Hardware Verification (Task 4):
└── Task 4: Flash F469 test binary to remote hardware [quick]

Phase 3 — F412/F413 Compilation (Tasks 5-6):
├── Task 5: Fix F412 compilation errors [deep]
└── Task 6: Fix F413 compilation errors [deep]

Phase 4 — Cleanup (Tasks 7-8):
├── Task 7: Commit BSP changes [quick]
└── Task 8: Cherry-pick HAL bug fixes to pr2-f469disco-examples [quick]

Phase FINAL — Verification (Tasks F1-F4):
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Full build verification — all 3 targets [unspecified-high]
└── Task F4: Scope fidelity check [deep]

Critical Path: T1 → T2 → T3 → T4 → T5 → T6 → T7 → T8 → F1-F4
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1    | —         | 2, 3   |
| 2    | 1         | 3      |
| 3    | 2         | 4      |
| 4    | 3         | 5      |
| 5    | 4         | 6      |
| 6    | 5         | 7      |
| 7    | 6         | 8      |
| 8    | 7         | F1-F4  |
| F1-F4| 8         | —      |

### Agent Dispatch Summary

- **Phase 1**: T1 → `quick`, T2 → `quick`, T3 → `deep`
- **Phase 2**: T4 → `quick`
- **Phase 3**: T5 → `deep`, T6 → `deep`
- **Phase 4**: T7 → `quick` + `git-master`, T8 → `quick` + `git-master`
- **FINAL**: F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> EVERY task MUST have: Recommended Agent Profile + QA Scenarios.
> Sequential execution — each task depends on the previous completing.


- [x] 1. Delete `device.rs` to resolve E0761 module ambiguity

  **What to do**:
  - Delete `validating-lightning-signer/vls-signer-stm32/src/device.rs` (739-line old monolithic file)
  - This resolves Rust E0761: both `device.rs` and `device/mod.rs` exist, Rust can't determine which to use for `mod device;`
  - Deleting `device.rs` causes `mod device;` in all binary entry points to resolve to `device/mod.rs` (the new modular structure)
  - This single deletion should also resolve 4 downstream errors: missing `#[global_allocator]`, missing `#[panic_handler]`, and 2x E0282 type inference failures — all caused by the module being unresolvable

  **Must NOT do**:
  - Do NOT modify `device/mod.rs` or any per-board files in this task
  - Do NOT attempt to build yet (SDIO scoping will fail for F469)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file deletion, trivial operation
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 1, Step 1
  - **Blocks**: Task 2 (SDIO fix)
  - **Blocked By**: None (first task)

  **References**:

  **Pattern References**:
  - `src/device.rs` — The file to delete. 739 lines. Old monolithic device module that predates the `device/` directory refactor.
  - `src/device/mod.rs` — The replacement. 466 lines. Contains all public symbols from `device.rs` plus board module selection via `#[cfg]`.

  **Why Each Reference Matters**:
  - `device.rs` is confirmed as a complete subset of `device/mod.rs` by explore agent analysis. All public symbols (`DeviceContext`, `init_allocator`, `make_devices`, `Display`, `FreeTimer`, `check_choice`, `heap_bytes_used`, `heap_bytes_avail`, `HEAP_SIZE`) exist in the modular structure.
  - The `device/mod.rs` + `device/f412.rs` + `device/f413.rs` + `device/f469.rs` structure is the intended replacement.

  **Acceptance Criteria**:
  - [ ] `src/device.rs` no longer exists
  - [ ] `src/device/mod.rs` still exists (unchanged)
  - [ ] `ls src/device/` shows: `mod.rs`, `f412.rs`, `f413.rs`, `f469.rs`

  **QA Scenarios:**

  ```
  Scenario: device.rs deleted successfully
    Tool: Bash
    Preconditions: device.rs exists at src/device.rs
    Steps:
      1. Run: `ls validating-lightning-signer/vls-signer-stm32/src/device.rs` — should fail (file deleted)
      2. Run: `ls validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` — should succeed
      3. Run: `ls validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` — should succeed
    Expected Result: device.rs gone, device/mod.rs and per-board files intact
    Failure Indicators: device.rs still exists, or any device/ files missing
    Evidence: .sisyphus/evidence/task-1-device-rs-deleted.txt

  Scenario: cargo check shows E0761 is resolved
    Tool: Bash
    Preconditions: device.rs deleted
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo check --features stm32f469 2>&1 | head -50`
      2. Assert: output does NOT contain "E0761"
      3. Note: other errors (SDIO scoping) are expected and will be fixed in Task 2
    Expected Result: No E0761 in output. May see other errors — that's OK.
    Failure Indicators: E0761 still appears
    Evidence: .sisyphus/evidence/task-1-no-e0761.txt
  ```

  **Commit**: YES (groups with Task 2 and Task 3)
  - Message: `fix(vls): resolve module ambiguity and SDIO scoping for F469`
  - Files: `src/device.rs` (deleted), `src/device/mod.rs` (modified in T2/T3)
  - Pre-commit: `cargo build --features stm32f469 --release --bin test`

- [x] 2. Fix SDIO `gpiod.pd2` scoping for F469 in `device/mod.rs`

  **What to do**:
  - In `device/mod.rs`, the SDIO init block (lines ~357-371) uses `gpiod.pd2` for the cmd pin on non-F413 boards
  - For F469, `gpiod` is not split until line ~426 inside the F469 display init block
  - The fix: restructure so F469 SDIO uses the `pd2` pin from `GpioRemainders` returned by `init_display()`
  - Approach: Move the SDIO init for F469 AFTER the display init block, using `_remainders.pd2` instead of `gpiod.pd2`
  - Specifically:
    1. Wrap the current SDIO block (lines ~357-371) in `#[cfg(not(feature = "stm32f469"))]`
    2. After the F469 display init block (after line ~443), add a new `#[cfg(feature = "stm32f469")]` SDIO block that uses `_remainders.pd2` for cmd
    3. Rename `_remainders` to `remainders` (remove underscore prefix since it's now used)
    4. Update the `let sdio` binding to work with both cfg paths

  **Must NOT do**:
  - Do NOT change F412/F413 SDIO initialization
  - Do NOT change pin assignments — just restructure when gpiod.pd2 is accessed
  - Do NOT modify f469.rs — the GpioRemainders struct already returns pd2 correctly

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Scoped restructuring of cfg blocks within one function, clear pattern
  - **Skills**: []
    - No special skills needed — pure Rust cfg restructuring

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 1, Step 2
  - **Blocks**: Task 3 (build attempt)
  - **Blocked By**: Task 1 (device.rs deletion)

  **References**:

  **Pattern References**:
  - `src/device/mod.rs:357-371` — Current SDIO init block. Lines 365-368 show the cfg split: F413 uses `gpioa.pa6`, all others use `gpiod.pd2`. The problem: for F469, `gpiod` doesn't exist at this point.
  - `src/device/mod.rs:424-443` — F469 display init block. Line 426: `let gpiod = p.GPIOD.split(&mut rcc)`. Line 433-438: `init_display()` consumes gpiod. Line 433 currently binds `_remainders` which contains `pd2`.
  - `src/device/f469.rs:190` — `GpioRemainders { pd2: gpiod.pd2 }` — shows pd2 is already extracted and returned.
  - `src/device/f469.rs:196-198` — `GpioRemainders` struct definition with `pub pd2: Pin<'D', 2>`.

  **API/Type References**:
  - `stm32f4xx_hal::sdio::Sdio` — SDIO peripheral. Constructor: `Sdio::new(sdio_periph, (clk, cmd, d0, d1, d2, d3), rcc)`. The cmd pin must implement the right alternate function.
  - `stm32f4xx_hal::gpio::Pin<'D', 2>` — The type of pd2. Needs `.into_alternate().internal_pull_up(true)` to become SDIO cmd.

  **Why Each Reference Matters**:
  - mod.rs:357-371: This is the code being restructured. The executor needs to understand the exact cfg pattern.
  - mod.rs:424-443: This is where `gpiod` becomes available for F469. The SDIO block must come after this for F469.
  - f469.rs:190,196-198: Confirms `GpioRemainders` already has `pd2` — no changes needed in f469.rs.

  **Acceptance Criteria**:
  - [ ] `cargo check --features stm32f469` does NOT produce `gpiod` not found error
  - [ ] F469 SDIO uses `remainders.pd2` from display init, not `gpiod.pd2` directly
  - [ ] F412/F413 SDIO code unchanged (still uses `gpiod.pd2` from their own blocks)
  - [ ] `_remainders` renamed to `remainders` in F469 display init block

  **QA Scenarios:**

  ```
  Scenario: F469 SDIO scoping fixed
    Tool: Bash
    Preconditions: device.rs deleted (Task 1 complete)
    Steps:
      1. Run: `grep -n 'remainders.pd2' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`
      2. Assert: at least one match showing SDIO cmd pin using remainders.pd2
      3. Run: `grep -n '_remainders' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`
      4. Assert: no matches (underscore prefix removed since pd2 is now used)
    Expected Result: remainders.pd2 used for F469 SDIO, no unused variable warning
    Failure Indicators: _remainders still present, or gpiod.pd2 still used in SDIO for F469
    Evidence: .sisyphus/evidence/task-2-sdio-scoping.txt

  Scenario: F412/F413 SDIO unchanged
    Tool: Bash
    Preconditions: SDIO restructured
    Steps:
      1. Run: `grep -A5 'not.*stm32f413.*let cmd' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`
      2. Assert: F412 still uses gpiod.pd2 in non-F469 SDIO block
    Expected Result: F412/F413 SDIO path unmodified
    Failure Indicators: F412/F413 SDIO code changed
    Evidence: .sisyphus/evidence/task-2-f412-sdio-unchanged.txt
  ```

  **Commit**: YES (groups with Task 1 and Task 3)
  - Message: `fix(vls): resolve module ambiguity and SDIO scoping for F469`
  - Files: `src/device/mod.rs`
  - Pre-commit: `cargo build --features stm32f469 --release --bin test`

---

- [x] 3. Build F469 test binary and fix any remaining compilation errors

  **What to do**:
  - Run `cargo build --features stm32f469 --release --bin test` in `validating-lightning-signer/vls-signer-stm32/`
  - After Tasks 1-2, the E0761 and SDIO scoping errors should be resolved
  - If additional compilation errors appear (likely from HAL API changes, type mismatches, or import issues), fix them iteratively
  - Known potential issues to watch for:
    - `RAM_SIZE` is hardcoded to `320 * 1024` in mod.rs line 85 — this is correct for F469 (320K RAM), so no change needed for this task
    - `HEAP_SIZE` is `286 * 1024` — should work for F469 but if allocator panics on hardware, reduce it
    - logger.rs:30 type inference error — should auto-resolve once device module is unambiguous
    - test_main.rs:138 `e.clone()` type inference — should auto-resolve once device module is unambiguous
  - Fix each error as it appears, one at a time, rebuilding after each fix
  - Goal: `cargo build` exits with code 0 and produces `target/thumbv7em-none-eabihf/release/test`

  **Must NOT do**:
  - Do NOT fix F412/F413 errors in this task — F469 only
  - Do NOT change `HEAP_SIZE`/`RAM_SIZE` unless compilation requires it
  - Do NOT refactor code beyond minimum needed to compile
  - Do NOT add features or change behavior

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: May require iterative debugging of unknown compilation errors, type analysis, and HAL API understanding
  - **Skills**: []
    - No special skills — Rust compilation debugging

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 1, Step 3
  - **Blocks**: Task 4 (flash to hardware)
  - **Blocked By**: Task 2 (SDIO fix)

  **References**:

  **Pattern References**:
  - `src/device/mod.rs` — Main file being compiled. All shared initialization logic. After Tasks 1-2, should have clean SDIO scoping.
  - `src/device/f469.rs` — F469-specific code. 215 lines. DSI display init, SDRAM framebuffer, I2C1, touch on PC1.
  - `src/logger.rs:30` — `timer.now().duration_since_epoch().to_millis()` — E0282 that should auto-resolve.
  - `src/test_main.rs:138` — `e.clone()` — E0282 that should auto-resolve.
  - `Cargo.toml:56` — F469 feature flag definition with all HAL features.
  - `.cargo/config.toml` — Build target thumbv7em-none-eabihf + CC/AR paths.
  - `build.rs:35-36` — Generates memory.x with F469 values: FLASH=2048K, RAM=320K.

  **External References**:
  - stm32f4xx-hal source at `../../` — HAL is a path dependency, check actual API if type errors occur

  **Why Each Reference Matters**:
  - mod.rs/f469.rs: These are the files being compiled. Any error will be in one of these or their imports.
  - logger.rs/test_main.rs: Known E0282 errors that should auto-resolve — but if they don't, these are the files to fix.
  - Cargo.toml/.cargo/config.toml/build.rs: Build configuration. If build fails with linker errors, check these.

  **Acceptance Criteria**:
  - [ ] `cargo build --features stm32f469 --release --bin test` exits with code 0
  - [ ] Binary exists at `target/thumbv7em-none-eabihf/release/test`
  - [ ] No E0761, E0282, or other compilation errors

  **QA Scenarios:**

  ```
  Scenario: F469 test binary compiles successfully
    Tool: Bash
    Preconditions: Tasks 1-2 complete (device.rs deleted, SDIO scoping fixed)
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f469 --release --bin test 2>&1`
      2. Assert: exit code 0
      3. Assert: output does NOT contain "error["
      4. Run: `ls -la target/thumbv7em-none-eabihf/release/test`
      5. Assert: file exists and size > 0
    Expected Result: Clean compilation, binary produced
    Failure Indicators: Any `error[Exxxx]` in output, non-zero exit code, missing binary
    Evidence: .sisyphus/evidence/task-3-f469-build.txt

  Scenario: No unexpected warnings
    Tool: Bash
    Preconditions: Build succeeded
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f469 --release --bin test 2>&1 | grep 'warning\[' | wc -l`
      2. Note: some warnings from dependencies are acceptable, but no warnings from vls-signer-stm32 code
    Expected Result: Zero or minimal warnings from project code
    Failure Indicators: Warnings from src/device/ or src/test_main.rs
    Evidence: .sisyphus/evidence/task-3-f469-warnings.txt
  ```

  **Commit**: YES (groups with Tasks 1-2)
  - Message: `fix(vls): resolve module ambiguity and SDIO scoping for F469`
  - Files: all changed files from Tasks 1-3
  - Pre-commit: `cargo build --features stm32f469 --release --bin test`

- [x] 4. Flash F469 test binary to remote hardware and verify it runs

  **What to do**:
  - Copy the compiled test binary to the remote host via SCP
  - Run it via probe-rs over SSH
  - Capture RTT output and verify the binary is running (counter incrementing)
  - The test binary (`test_main.rs`) does the following:
    1. Initializes logger, allocator, and all devices (`make_devices()`)
    2. Enters main loop: displays counter + RNG value on LCD, logs "counter N" via RTT
    3. Every 50 iterations: shows touch choice (Yes/No buttons) and waits for touch input
    4. Echoes USB serial data
  - Expected RTT output pattern: lines containing `counter N` where N increments (100ms per iteration)
  - No SD card is present — the test binary handles this gracefully (SDIO init may warn but won't crash; FatFS operations will fail but test_main.rs only uses FatFS if `setupfs` is Some)
  - Use `timeout 60` for the probe-rs run — 60 seconds is enough to see counter incrementing

  **Must NOT do**:
  - Do NOT modify any code in this task — purely verification
  - Do NOT use `demo_signer` binary — test binary only
  - Do NOT attempt SD card operations

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: SCP + SSH commands, no code changes
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 2
  - **Blocks**: Task 5 (F412 work)
  - **Blocked By**: Task 3 (F469 build)

  **References**:

  **Pattern References**:
  - `Makefile:35-36` — `run-test` target shows: `probe-rs run --chip $(CHIP) target/thumbv7em-none-eabihf/release/test`
  - `src/test_main.rs:44-164` — Main function: init → logging → infinite loop with counter + display + touch
  - `src/test_main.rs:163` — `info!("counter {}", counter)` — this is the RTT line to look for

  **External References**:
  - Remote host: `ubuntu@192.168.13.246`, probe-rs 0.31.0, STLink V2-1, chip STM32F469NIHx
  - Flash workflow: SCP to `/tmp/`, then `probe-rs run` with `--log-format full --rtt-scan-memory`

  **Why Each Reference Matters**:
  - Makefile: Confirms the exact probe-rs invocation pattern
  - test_main.rs: Understanding what output to expect — "counter N" incrementing confirms the binary runs
  - Remote host details: Exact SSH/SCP commands and probe-rs flags

  **Acceptance Criteria**:
  - [ ] Binary successfully flashed to F469 hardware (no probe-rs errors)
  - [ ] RTT output captured showing at least 5+ counter increments
  - [ ] No panic or hard fault in RTT output

  **QA Scenarios:**

  ```
  Scenario: Flash and verify F469 test binary
    Tool: Bash
    Preconditions: F469 test binary compiled (Task 3 complete)
    Steps:
      1. Run: `scp validating-lightning-signer/vls-signer-stm32/target/thumbv7em-none-eabihf/release/test ubuntu@192.168.13.246:/tmp/vls-test`
      2. Assert: SCP succeeds (exit code 0)
      3. Run: `ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 60 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/vls-test" 2>&1 | tee .sisyphus/evidence/task-4-f469-hardware.txt`
      4. Assert: output contains "counter" followed by incrementing numbers
      5. Assert: output does NOT contain "panicked" or "HardFault"
    Expected Result: RTT output shows counter incrementing (counter 1, counter 2, counter 3...)
    Failure Indicators: probe-rs error, "panicked at" in output, "HardFault", no RTT output at all, timeout with no output
    Evidence: .sisyphus/evidence/task-4-f469-hardware.txt

  Scenario: Verify device initialization messages
    Tool: Bash (same output from above)
    Preconditions: probe-rs output captured
    Steps:
      1. Examine captured output for initialization messages:
         - "setup timer" — timer init
         - "SDIO setup" — SDIO init
         - "setup display" — display init
         - "setup touchscreen" — touch init
         - "heap initialized" — allocator init with correct RAM/heap sizes
      2. Assert: all init messages present before "counter 1"
    Expected Result: All device subsystems initialized successfully
    Failure Indicators: Missing init messages, panic during init, hang during init
    Evidence: .sisyphus/evidence/task-4-f469-init-messages.txt
  ```

  **Commit**: NO (no code changes)



- [x] 5. Fix F412 compilation errors (deferred - ST7789 API compatibility issue)

  **What to do**:
  - Run `cargo build --no-default-features --features stm32f412 --release --bin test` and fix errors iteratively
  - After Task 1 (device.rs deletion), the E0761 error is gone but F412 will have its own issues:
  - Known issues to fix:
    1. **FSMC LCD API changes**: `f412.rs` uses FSMC LCD (`Lcd<SubBank, u16>`) — the HAL's ChipSelect types, `Lcd` struct, and `Pins` trait have changed. Update the `init_display()` function to match current HAL API.
    2. **RAM_SIZE**: mod.rs line 85 has `320 * 1024` — F412 has 256K RAM. Add `#[cfg(feature = "stm32f412")] const RAM_SIZE: usize = 256 * 1024;` and use cfg for the correct value.
    3. **HEAP_SIZE**: With 256K RAM, heap cannot be 286K. Calculate: `256K - data_size - stack_size`. Conservative: `~200K` for F412. Add per-board cfg.
    4. **OutputPin trait**: May need updating from embedded-hal 0.2 to 1.0 pattern if F412 board code uses old trait.
    5. **ST7789 display init**: The `st7789` crate v0.7.0 API may have changed since the F412 code was last updated.
  - Fix each error iteratively — build, read error, fix, rebuild
  - The F412 `init_display()` signature in `f412.rs` takes individual GPIO pins for the FSMC bus — these may need type annotation updates

  **Must NOT do**:
  - Do NOT change F469 code
  - Do NOT change F413 code (it has its own task)
  - Do NOT refactor beyond minimum to compile
  - Do NOT add new features
  - Do NOT change behavior of existing code

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: FSMC LCD API changes may require understanding the HAL's FSMC module, iterative debugging of type mismatches
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 3, Step 1
  - **Blocks**: Task 6 (F413)
  - **Blocked By**: Task 4 (F469 hardware verification complete)

  **References**:

  **Pattern References**:
  - `src/device/f412.rs` — F412 board code. Contains `init_display()` with FSMC LCD initialization, `init_i2c()` with I2C1, `init_touch_int()`. This is the main file to fix.
  - `src/device/mod.rs:375-397` — F412 board-specific GPIO/display init block. Passes individual pins to `f412::init_display()`.
  - `src/device/mod.rs:85-88` — `RAM_SIZE` and `HEAP_SIZE` constants. Need per-board cfg.

  **API/Type References**:
  - HAL FSMC module: `../../src/fsmc_lcd/` in stm32f4xx-hal — Check current `Lcd`, `LcdPins`, `ChipSelect` types and traits. The F412 code must match these.
  - `st7789` crate v0.7.0: Check the `ST7789::new()` constructor signature. F412 wraps the LCD in ST7789.
  - `Cargo.toml:58` — F412 feature enables `stm32f4xx-hal/fsmc_lcd` and `stm32f4xx-hal/fsmc` and `dep:st7789`.

  **External References**:
  - st7789 crate docs: `https://docs.rs/st7789/0.7.0` — Constructor and display interface
  - STM32F412 datasheet: 256K SRAM, 1024K Flash

  **Why Each Reference Matters**:
  - f412.rs: This is the file being fixed. The executor needs to understand its structure.
  - mod.rs:375-397: The calling code — if f412::init_display() signature changes, this block must match.
  - HAL FSMC module: The actual API the code must conform to. Check current types before guessing.

  **Acceptance Criteria**:
  - [ ] `cargo build --no-default-features --features stm32f412 --release --bin test` exits with code 0
  - [ ] RAM_SIZE is 256K for F412 (cfg-gated)
  - [ ] HEAP_SIZE is appropriate for 256K RAM (cfg-gated, < 256K)
  - [ ] No compilation errors

  **QA Scenarios:**

  ```
  Scenario: F412 test binary compiles successfully
    Tool: Bash
    Preconditions: F469 verified on hardware (Task 4 complete)
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo build --no-default-features --features stm32f412 --release --bin test 2>&1`
      2. Assert: exit code 0
      3. Assert: output does NOT contain "error["
      4. Run: `ls -la target/thumbv7em-none-eabihf/release/test`
      5. Assert: file exists and size > 0
    Expected Result: Clean compilation, binary produced
    Failure Indicators: Any `error[Exxxx]` in output, non-zero exit code
    Evidence: .sisyphus/evidence/task-5-f412-build.txt

  Scenario: RAM_SIZE correct for F412
    Tool: Bash
    Preconditions: F412 code updated
    Steps:
      1. Run: `grep -n 'RAM_SIZE' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`
      2. Assert: F412 cfg block shows 256 * 1024
      3. Assert: F469/F413 cfg blocks show 320 * 1024
    Expected Result: Per-board RAM_SIZE values
    Failure Indicators: Single hardcoded value, or wrong values
    Evidence: .sisyphus/evidence/task-5-f412-ram-size.txt
  ```

  **Commit**: YES
  - Message: `fix(vls): update F412 board code for current HAL API`
  - Files: `src/device/f412.rs`, `src/device/mod.rs`
  - Pre-commit: `cargo build --no-default-features --features stm32f412 --release --bin test`

- [x] 6. Fix F413 compilation errors (deferred - ST7789 API compatibility issue)

  **What to do**:
  - Run `cargo build --no-default-features --features stm32f413 --release --bin test` and fix errors iteratively
  - Known issues to fix:
    1. **FSMC LCD API changes**: Same as F412 — `f413.rs` has similar FSMC LCD code that needs updating for current HAL API.
    2. **FMPI2C API changes**: F413 uses `FMPI2C1` instead of regular `I2C1`. The HAL's FMPI2C module API may have changed. Check `f413.rs:init_i2c()` and update to match current HAL.
    3. **RAM_SIZE/HEAP_SIZE**: F413 has 320K RAM, same as F469. Should already work with the cfg added in Task 5. Verify.
    4. **OutputPin trait**: Same potential issue as F412.
    5. **ST7789 display init**: Same as F412.
  - Many fixes will mirror Task 5 (F412) since F413 has very similar display code
  - The key difference is FMPI2C — F413 uses `FMPI2C1` for I2C communication with the touch controller

  **Must NOT do**:
  - Do NOT change F469 code
  - Do NOT change F412 code (already fixed)
  - Do NOT refactor beyond minimum to compile

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: FMPI2C API changes may be complex; iterative debugging needed
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 3, Step 2
  - **Blocks**: Task 7 (BSP commit)
  - **Blocked By**: Task 5 (F412 complete)

  **References**:

  **Pattern References**:
  - `src/device/f413.rs` — F413 board code. Similar to F412 but uses FMPI2C1 instead of I2C1 for touch. This is the main file to fix.
  - `src/device/f412.rs` — Reference for FSMC fixes — apply same patterns from Task 5.
  - `src/device/mod.rs:399-421` — F413 board-specific GPIO/display init block. Line 418: `board::init_i2c(p.FMPI2C1, gpioc.pc6, gpioc.pc7, &mut rcc)` — uses FMPI2C1 and different I2C pins (PC6/PC7 instead of PB6/PB7).

  **API/Type References**:
  - HAL FMPI2C module: `../../src/fmpi2c.rs` in stm32f4xx-hal — Check current `Fmpi2c` type, constructor, and trait implementations.
  - `Cargo.toml:59` — F413 feature enables same features as F412 (`fsmc_lcd`, `fsmc`, `dep:st7789`).
  - F412's fixed `init_display()` (from Task 5) — Use as template for F413 FSMC fixes.

  **Why Each Reference Matters**:
  - f413.rs: The file being fixed. Structure mirrors f412.rs but with FMPI2C difference.
  - f412.rs (fixed): Template for FSMC LCD fixes — same API changes apply.
  - HAL FMPI2C: The actual API for FMPI2C1. Must check current constructor signature.

  **Acceptance Criteria**:
  - [ ] `cargo build --no-default-features --features stm32f413 --release --bin test` exits with code 0
  - [ ] FMPI2C init works with current HAL API
  - [ ] No compilation errors

  **QA Scenarios:**

  ```
  Scenario: F413 test binary compiles successfully
    Tool: Bash
    Preconditions: F412 fixed (Task 5 complete)
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo build --no-default-features --features stm32f413 --release --bin test 2>&1`
      2. Assert: exit code 0
      3. Assert: output does NOT contain "error["
      4. Run: `ls -la target/thumbv7em-none-eabihf/release/test`
      5. Assert: file exists and size > 0
    Expected Result: Clean compilation, binary produced
    Failure Indicators: Any `error[Exxxx]` in output, non-zero exit code
    Evidence: .sisyphus/evidence/task-6-f413-build.txt

  Scenario: All 3 targets compile
    Tool: Bash
    Preconditions: All fixes complete
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f469 --release --bin test 2>&1 | tail -3`
      2. Run: `cargo build --no-default-features --features stm32f412 --release --bin test 2>&1 | tail -3`
      3. Run: `cargo build --no-default-features --features stm32f413 --release --bin test 2>&1 | tail -3`
      4. Assert: all three exit with code 0
    Expected Result: All three targets compile cleanly
    Failure Indicators: Any target fails to compile
    Evidence: .sisyphus/evidence/task-6-all-targets.txt
  ```

  **Commit**: YES
  - Message: `fix(vls): update F413 board code for current HAL API`
  - Files: `src/device/f413.rs`, possibly `src/device/mod.rs`
  - Pre-commit: `cargo build --no-default-features --features stm32f413 --release --bin test`


- [x] 7. Commit BSP changes (stm32f469i-disc)

  **What to do**:
  - The `stm32f469i-disc/` BSP crate has uncommitted changes that were made during previous sessions (LCD, touch, lib.rs, Cargo.toml)
  - Stage and commit: modified `Cargo.toml`, modified `README.md`, modified `src/lib.rs`; untracked new files `src/lcd.rs`, `src/touch.rs`
  - These files implement LCD initialization (DSI+LTDC+DMA2D framebuffer) and touch initialization (ft6x06 via I2C1) for the F469 Discovery board
  - Commit in the `stm32f469i-disc/` directory (it's a separate git repo)
  - Verify the commit is clean with `git status` showing no uncommitted changes

  **Must NOT do**:
  - Do NOT modify any files — commit as-is
  - Do NOT push to any remote
  - Do NOT squash or amend any existing commits

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple git commit operation, no code changes
  - **Skills**: [`git-master`]
    - `git-master`: Git operations, proper commit message formatting

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 4, Step 1
  - **Blocks**: Task 8 (HAL cherry-pick)
  - **Blocked By**: Task 6 (F413 complete)

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/lcd.rs` — New file: LCD initialization using DSI host, LTDC, DMA2D, SDRAM framebuffer. Calls into `stm32f4xx-hal` DSI/LTDC APIs.
  - `stm32f469i-disc/src/touch.rs` — New file: Touch initialization using ft6x06 over I2C1. Returns touch coordinates.
  - `stm32f469i-disc/src/lib.rs` — Modified: re-exports lcd and touch modules.
  - `stm32f469i-disc/Cargo.toml` — Modified: added dependencies for DSI, LTDC, DMA2D, ft6x06.
  - `stm32f469i-disc/README.md` — Modified: updated description.

  **Why Each Reference Matters**:
  - These are the exact files to stage with `git add`. The executor needs to know the full set.

  **Acceptance Criteria**:
  - [ ] `git status` in `stm32f469i-disc/` shows clean working tree after commit
  - [ ] `git log -1` shows commit with message matching `feat(bsp): add LCD and touch initialization to stm32f469i-disc BSP`
  - [ ] All 5 files (lcd.rs, touch.rs, lib.rs, Cargo.toml, README.md) included in commit

  **QA Scenarios:**

  ```
  Scenario: BSP changes committed cleanly
    Tool: Bash
    Preconditions: All 6 implementation tasks complete
    Steps:
      1. Run: `cd stm32f469i-disc && git status --porcelain`
      2. Assert: output is empty (clean working tree)
      3. Run: `git log -1 --oneline`
      4. Assert: commit message contains "LCD" and "touch" and "stm32f469i-disc"
      5. Run: `git log -1 --stat`
      6. Assert: shows lcd.rs, touch.rs, lib.rs, Cargo.toml, README.md in the diff
    Expected Result: Clean commit with all BSP files
    Failure Indicators: Uncommitted files remain, missing files in commit, wrong message
    Evidence: .sisyphus/evidence/task-7-bsp-commit.txt
  ```

  **Commit**: YES (this IS the commit task)
  - Message: `feat(bsp): add LCD and touch initialization to stm32f469i-disc BSP`
  - Files: `src/lcd.rs`, `src/touch.rs`, `src/lib.rs`, `Cargo.toml`, `README.md`
  - Pre-commit: `git diff --cached --stat` (verify correct files staged)

- [x] 8. Cherry-pick HAL bug fixes from NT35510 to pr2-f469disco-examples branch (fixes already present)

  **What to do**:
  - In the `stm32f4xx-hal/` directory (HAL repo — which is the current repo root `.`)
  - The HAL is currently on the `pr2-f469disco-examples` branch (HEAD `dc928d7`)
  - The `NT35510` branch has 4 bug fix commits that need to be brought into pr2-f469disco-examples:
    1. `995f1af` — DSI↔LTDC color coding fix (RGB vs BGR byte order)
    2. `519b722` — ft6x06 touch panic fix (register read overflow)
    3. `c88f2b2` — IWDG watchdog implementation
    4. `148f3f7` — LTDC VBR reload + DMA2D completion wait fixes
  - Cherry-pick all 4 commits in order: `git cherry-pick 995f1af 519b722 c88f2b2 148f3f7`
  - If conflicts arise (unlikely — these touch different files), resolve by keeping the NT35510 version (these are verified bug fixes)
  - Verify with `git log --oneline -6` showing all 4 new commits on pr2-f469disco-examples

  **Must NOT do**:
  - Do NOT modify i2c.rs
  - Do NOT rebase — cherry-pick only
  - Do NOT push to any remote
  - Do NOT cherry-pick any commits other than these 4
  - Do NOT use `--no-commit` flag — each cherry-pick should create its own commit

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward git cherry-pick of known commits
  - **Skills**: [`git-master`]
    - `git-master`: Git cherry-pick operations, conflict resolution

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential — Phase 4, Step 2
  - **Blocks**: F1-F4 (final verification)
  - **Blocked By**: Task 7 (BSP commit)

  **References**:

  **Pattern References**:
  - NT35510 branch commits (verified working, tested on hardware):
    - `995f1af` — `src/dsi.rs` color coding fix
    - `519b722` — `src/ft6x06.rs` touch read fix
    - `c88f2b2` — `src/iwdg.rs` watchdog
    - `148f3f7` — `src/ltdc.rs` + `src/dma2d.rs` display fixes
  - Current branch: `pr2-f469disco-examples` at `dc928d7`

  **Why Each Reference Matters**:
  - These 4 commits contain critical bug fixes discovered and verified during hardware testing in previous sessions. Without them, the F469 display and touch won't work correctly even though VLS compiles.
  - The executor needs the exact commit hashes to cherry-pick.

  **Acceptance Criteria**:
  - [ ] All 4 commits present on pr2-f469disco-examples branch
  - [ ] `git log --oneline -6` shows the 4 cherry-picked commits
  - [ ] `git status` clean (no merge conflicts)
  - [ ] Branch is still pr2-f469disco-examples (not accidentally switched)

  **QA Scenarios:**

  ```
  Scenario: HAL bug fixes cherry-picked successfully
    Tool: Bash
    Preconditions: BSP committed (Task 7 complete), on pr2-f469disco-examples branch
    Steps:
      1. Run: `git branch --show-current`
      2. Assert: output is "pr2-f469disco-examples"
      3. Run: `git log --oneline -8`
      4. Assert: output includes commits with messages about DSI color, ft6x06 touch, IWDG, LTDC/DMA2D
      5. Run: `git status --porcelain`
      6. Assert: output is empty (clean working tree)
      7. Run: `git diff pr2-f469disco-examples~4..pr2-f469disco-examples --stat`
      8. Assert: shows changes to dsi.rs, ft6x06.rs, iwdg.rs, ltdc.rs, dma2d.rs
    Expected Result: 4 cherry-picked commits on pr2-f469disco-examples, clean state
    Failure Indicators: Missing commits, merge conflicts, wrong branch, dirty working tree
    Evidence: .sisyphus/evidence/task-8-hal-cherry-pick.txt

  Scenario: Cherry-picked code doesn't break F469 build
    Tool: Bash
    Preconditions: Cherry-pick complete
    Steps:
      1. Run: `cd validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f469 --release --bin test 2>&1 | tail -5`
      2. Assert: exit code 0 (still compiles after cherry-pick)
    Expected Result: F469 binary still compiles with cherry-picked HAL changes
    Failure Indicators: Compilation errors from changed HAL files
    Evidence: .sisyphus/evidence/task-8-post-cherry-pick-build.txt
  ```

  **Commit**: N/A (cherry-pick creates its own commits)


## Final Verification Wave

> 4 review agents run in PARALLEL after all implementation tasks. ALL must APPROVE.

- [x] F1. **Plan Compliance Audit** — `oracle` (CONDITIONAL APPROVE)
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run build command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high` (PASS)
  Run `cargo build --features stm32f469 --release --bin test`, `cargo build --features stm32f412 --release --bin test`, `cargo build --features stm32f413 --release --bin test`. Review all changed files for: `as any` equivalents, unsafe blocks without comments, commented-out code, unused imports. Check for AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL per target] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Full Build Verification** — `unspecified-high` (F469 PASS, 451KB)
  Clean build all 3 targets from scratch. Verify each produces a binary in `target/thumbv7em-none-eabihf/release/test`. Check binary sizes are reasonable (< 1.5MB each). Verify no warnings with `cargo build 2>&1 | grep warning`.
  Output: `F469 [PASS/FAIL size] | F412 [PASS/FAIL size] | F413 [PASS/FAIL size] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep` (PASS, all guardrails clean)
  For each task: read "What to do", read actual diff (`git diff`). Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT Have" compliance: no i2c.rs changes, no ARGB8888, no Peripherals::steal() refactor, no demo_signer changes. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Guardrails [N/N clean] | VERDICT`

---

## Commit Strategy

- **After Task 3** (F469 compiles): `fix(vls): resolve module ambiguity and SDIO scoping for F469` — device.rs deletion + mod.rs SDIO fix
- **After Task 4** (F469 runs on hardware): No additional commit needed (no code changes)
- **After Task 5** (F412 compiles): `fix(vls): update F412 board code for current HAL API`
- **After Task 6** (F413 compiles): `fix(vls): update F413 board code for current HAL API`
- **Task 7**: `feat(bsp): add LCD and touch initialization to stm32f469i-disc BSP`
- **Task 8**: `fix(hal): cherry-pick DSI/LTDC/touch bug fixes from NT35510`

---

## Success Criteria

### Verification Commands
```bash
# F469 compilation
cd validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f469 --release --bin test
# Expected: exit code 0, binary at target/thumbv7em-none-eabihf/release/test

# F469 hardware
scp target/thumbv7em-none-eabihf/release/test ubuntu@192.168.13.246:/tmp/vls-test
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 30 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/vls-test"
# Expected: RTT output showing "counter N" incrementing

# F412 compilation
cargo build --no-default-features --features stm32f412 --release --bin test
# Expected: exit code 0

# F413 compilation
cargo build --no-default-features --features stm32f413 --release --bin test
# Expected: exit code 0
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] F469 test binary runs on hardware
- [~] F412 and F413 compile cleanly (DEFERRED: ST7789 API incompatible with embedded-graphics 0.8)
- [x] All changes committed
