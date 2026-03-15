# BSP Migration: Finalize F469 — Full Peripheral Abstraction

## TL;DR

> **Quick Summary**: Commit existing SDRAM migration, then systematically move all board-specific peripheral initialization from VLS into BSP modules: SDIO, touch (refactored), touchscreen calibration, user button, and USB serial pins. End state: VLS F469 block calls only BSP functions for hardware init, with clean abstractions that scale to future multi-board support.
> 
> **Deliverables**:
> - Committed SDRAM migration in BSP and VLS repos
> - New BSP `sdio.rs` module (SDIO pin setup + init)
> - Refactored BSP `touch.rs` (individual pins, calibration included)
> - New BSP `button.rs` module (user button PA0)
> - New BSP `usb.rs` module (USB OTG FS pin setup)
> - Updated BSP `lib.rs` and `Cargo.toml`
> - Updated BSP `display_touch.rs` example (uses new touch API)
> - Updated VLS `mod.rs` and `f469.rs` to use all BSP modules
> - All three repos building cleanly
> 
> **Estimated Effort**: Medium (multiple coordinated modules across 2 repos)
> **Parallel Execution**: YES — 3 waves + final verification
> **Critical Path**: T1 → T3 → T6 → T10 → T12 → T13 → F1-F4

---

## Context

### Original Request
Finalize the porting of vls-signer-stm32 making changes to the HAL, the BSP, and the VLS projects as appropriate. Focus is only on F469 — no F412/F413. End state: fully functional vls-signer-stm32 for F469 with clean abstractions in the stm32f4xx HAL and the BSP. User wants maximum reasonable abstraction in BSP, designed for future multi-board support.

### Interview Summary
**Key Discussions**:
- SDRAM migration is already done in code but needs committing in both BSP and VLS repos
- SDIO pin setup is still inline in VLS — needs BSP module
- BSP touch.rs has a design flaw: takes whole GPIO port Parts instead of individual pins, preventing VLS from reusing other pins on same ports. Needs refactor to individual pins.
- VLS `make_touchscreen()` (Ft6X06 creation + calibration) should move to BSP touch module — I2C addr 0x38 is board-fixed
- User button PA0 is trivial but should have BSP module for consistency
- USB serial PA11/PA12 pin config should move to BSP — board-specific pins
- HAL repo needs NO changes
- Three separate git repos with path deps: BSP → HAL, VLS → BSP → HAL
- Future: user wants to add more boards via new BSPs — APIs must accept individual pins, not port Parts

**Research Findings**:
- Touch pin (PC1) used at RUNTIME for `wait_touch_interrupt()` — lives inside `Ft6X06` driver for program lifetime
- BSP `init_i2c()` takes `gpiob::Parts` + `gpioc::Parts` → prevents VLS pin reuse. Fix: accept `PB8`, `PB9` individually.
- BSP `init_ft6x06()` only creates driver, no calibration. VLS `make_touchscreen()` adds `ts_calibration()`. BSP should include calibration.
- STM32 BSPs use individual module inits (not monolithic), no `Board` trait — flexibility over convenience
- `USB` struct in HAL takes `OTG_FS_GLOBAL`, `OTG_FS_DEVICE`, `OTG_FS_PWRCLK`, `pin_dm: alt::Dm`, `pin_dp: alt::Dp`, `hclk: Hertz`
- `SerialDriver::new(usb: USB)` in VLS — takes the constructed `USB` struct
- BSP `display_touch.rs` example uses old `touch::init_i2c(dp.I2C1, gpiob, gpioc, &mut rcc)` API — must be updated

### Gap Analysis (self-review)
**Addressed**:
- BSP Cargo.toml needs `sdio`, `sdio-host`, `usb_fs` features — included in Task 3
- BSP touch.rs refactor must update `display_touch.rs` example — included in Task 6
- Duplicate comment on lines 523-524 of VLS mod.rs — included in cleanup
- Commit ordering: BSP changes must be committed before VLS changes (VLS depends on BSP)
- SDIO init consumes `SdramRemainders` and returns `pc1` separately — touch init must happen after SDIO init
- VLS `f469.rs` has `init_i2c()` and `init_touch_int()` functions that become redundant after BSP touch refactor — must be removed
- `make_touchscreen()` in VLS mod.rs is shared code (not `#[cfg]`-gated) but only makes sense for boards with FT6X06 — BSP handles it per-board

---

## Work Objectives

### Core Objective
Move ALL board-specific peripheral initialization from VLS into BSP modules, leaving VLS with only application-level logic. The F469 block in `make_devices()` should call only BSP functions and HAL peripheral constructors (for non-board-specific peripherals like RNG and timers).

### Concrete Deliverables
- `stm32f469i-disc/src/sdram.rs` — committed (already modified, uncommitted)
- `stm32f469i-disc/src/sdio.rs` — new file
- `stm32f469i-disc/src/touch.rs` — refactored (individual pins, calibration)
- `stm32f469i-disc/src/button.rs` — new file
- `stm32f469i-disc/src/usb.rs` — new file
- `stm32f469i-disc/src/lib.rs` — updated with new modules
- `stm32f469i-disc/Cargo.toml` — updated with new features
- `stm32f469i-disc/examples/display_touch.rs` — updated for new touch API
- `vls-signer-stm32/src/device/mod.rs` — updated to use BSP modules
- `vls-signer-stm32/src/device/f469.rs` — simplified (remove redundant init functions)

### Definition of Done
- [ ] `cargo build --features stm32f469 --release` succeeds in VLS repo
- [ ] `cargo build` succeeds in BSP repo
- [ ] `cargo build --release --example display_touch` succeeds in BSP repo
- [ ] `cargo build --release --example display_dsi_lcd` succeeds in BSP repo
- [ ] No inline peripheral pin setup remains in VLS F469 block (except RNG, timers)
- [ ] All three repos have clean git status (untracked files don't count)

### Must Have
- SDIO pin configuration centralized in BSP
- Touch I2C init accepting individual pins (PB8, PB9), not port Parts
- Touchscreen creation with calibration in BSP
- User button init in BSP
- USB OTG FS pin setup in BSP
- BSP `display_touch.rs` example updated for new touch API
- SdramRemainders consumed by SDIO init for type-safe pin ownership
- All BSP and VLS repos have clean, well-described commits
- Cross-compilation succeeds for `thumbv7em-none-eabihf`

### Must NOT Have (Guardrails)
- **NO changes to F412/F413 code** — out of scope, leave existing `#[cfg]` blocks untouched
- **NO changes to HAL repo** — HAL is clean, no modifications needed
- **NO `Board` trait or unifying abstraction layer** — BSPs use concrete modules, not traits
- **NO changes to `SerialDriver` struct or implementation** — only extract `USB` struct construction to BSP
- **NO changes to `make_touchscreen()` for F412/F413** — only F469 touch path changes
- **NO changes to VLS touch/button/serial runtime usage** — only initialization moves to BSP
- **NO new external crate dependencies** — just enable existing HAL features and use existing ft6x06 crate
- **NO changes to `DeviceContext` struct** — same fields, same types, different init path
- **NO monolithic board init function** — keep individual module inits (matches STM32 BSP convention)

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (embedded `no_std`, no test harness)
- **Automated tests**: None — verification is cross-compilation
- **Framework**: N/A

### QA Policy
Every task includes cross-compilation verification as the primary QA method.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **All tasks**: Use Bash — `cargo build` with appropriate features and target
- **Commit tasks**: Use Bash — `git diff --stat`, `git log --oneline -1`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — commit SDRAM + create all BSP modules):
├── Task 1: Commit SDRAM migration in BSP repo [quick + git-master]
├── Task 2: Commit SDRAM migration in VLS repo [quick + git-master]
├── Task 3: Create BSP sdio.rs module + update Cargo.toml [unspecified-high]
├── Task 4: Create BSP button.rs module [quick]
├── Task 5: Create BSP usb.rs module [quick]
└── Task 6: Refactor BSP touch.rs + add calibration + update example [unspecified-high]

Wave 2 (After Wave 1 — verify BSP, update VLS, commit BSP):
├── Task 7: Verify all BSP builds (lib + all examples) [quick]
├── Task 8: Commit all BSP module additions [quick + git-master]
├── Task 9: Update VLS f469.rs — remove redundant init functions [unspecified-high]
├── Task 10: Update VLS mod.rs — use BSP modules for all peripherals [unspecified-high]
└── Task 11: Remove VLS make_touchscreen() or gate it for non-F469 [unspecified-high]

Wave 3 (After Wave 2 — commit VLS, full verification):
├── Task 12: Commit VLS changes [quick + git-master]
└── Task 13: Full cross-compilation verification [quick]

Wave FINAL (After ALL tasks — independent review):
├── Task F1: Plan compliance audit [quick]
├── Task F2: Code quality review [quick]
├── Task F3: Build verification from clean state [quick]
└── Task F4: Scope fidelity check [quick]

Critical Path: T1 → T3 → T7 → T8 → T10 → T12 → T13 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 6 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 3, 6, 7 |
| 2 | — | 9, 10 |
| 3 | 1 | 7, 10 |
| 4 | — | 7, 10 |
| 5 | — | 7, 10 |
| 6 | 1 | 7, 10, 11 |
| 7 | 1, 3, 4, 5, 6 | 8 |
| 8 | 7 | 9, 10, 11 |
| 9 | 2, 8 | 12 |
| 10 | 2, 8 | 12 |
| 11 | 2, 6, 8 | 12 |
| 12 | 9, 10, 11 | 13 |
| 13 | 12 | F1-F4 |
| F1-F4 | 13 | — |

### Agent Dispatch Summary

- **Wave 1**: 6 tasks — T1 → `quick` + `git-master`, T2 → `quick` + `git-master`, T3 → `unspecified-high`, T4 → `quick`, T5 → `quick`, T6 → `unspecified-high`
- **Wave 2**: 5 tasks — T7 → `quick`, T8 → `quick` + `git-master`, T9 → `unspecified-high`, T10 → `unspecified-high`, T11 → `unspecified-high`
- **Wave 3**: 2 tasks — T12 → `quick` + `git-master`, T13 → `quick`
- **FINAL**: 4 tasks — F1-F4 → `quick`

---

## TODOs
> Implementation + Test = ONE Task. Never separate.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.
> **A task WITHOUT QA Scenarios is INCOMPLETE. No exceptions.**

- [ ] 1. Commit SDRAM Migration in BSP Repo

  **What to do**:
  - Verify the existing uncommitted changes in `stm32f469i-disc/src/sdram.rs` compile correctly
  - Run `cargo build` in BSP repo to confirm no regressions
  - Stage `src/sdram.rs` and commit with descriptive message
  - Verify commit landed cleanly

  **Must NOT do**:
  - Do NOT modify any code — this is a commit-only task
  - Do NOT touch any file other than `src/sdram.rs`
  - Do NOT run `cargo fmt` or make style changes

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Commit workflow, proper message formatting
  - **Skills Evaluated but Omitted**:
    - None — this is a simple git commit task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4, 5, 6)
  - **Blocks**: Tasks 3, 6, 7 (BSP modules depending on sdram types)
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/sdram.rs` — The file to commit. Contains: `SdramRemainders` struct (holds PC1, PC8-PC12, PD2 pins after SDRAM init), `split_sdram_pins()` (macro-free pin splitter), `Sdram` wrapper struct with `init()` method.

  **WHY Each Reference Matters**:
  - The executor needs to verify the file compiles before committing — read it to understand the scope of changes
  - `SdramRemainders` is consumed by the SDIO module (Task 3) — this commit must land first

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds in `/Users/macbook/src/stm32f4xx-hal/stm32f469i-disc/`
  - [ ] `git log -1 --oneline` in BSP repo shows the new commit
  - [ ] `git diff --cached` is empty (nothing left staged)
  - [ ] `git diff src/sdram.rs` is empty (file is committed)

  **QA Scenarios:**

  ```
  Scenario: BSP compiles after SDRAM commit
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Run `cargo build` — should compile without errors
      2. Run `git log -1 --oneline` — should show commit with "sdram" in message
      3. Run `git status --short src/sdram.rs` — should show nothing (clean)
    Expected Result: Build succeeds, commit exists, sdram.rs is clean
    Failure Indicators: Compilation errors, missing commit, sdram.rs still shows as modified
    Evidence: .sisyphus/evidence/task-1-bsp-sdram-commit.txt

  Scenario: Existing BSP examples still build
    Tool: Bash
    Preconditions: BSP repo has the SDRAM commit
    Steps:
      1. Run `cargo build --release --example display_dsi_lcd` — should succeed
      2. Run `cargo build --release --example fmc_sdram_test` — should succeed (if it exists)
    Expected Result: All existing examples compile without errors
    Failure Indicators: Any compilation failure in examples
    Evidence: .sisyphus/evidence/task-1-bsp-examples-build.txt
  ```

  **Commit**: YES
  - Message: `feat(sdram): add SdramRemainders, split_sdram_pins, and Sdram wrapper`
  - Files: `src/sdram.rs`
  - Pre-commit: `cargo build`

- [ ] 2. Commit SDRAM Migration in VLS Repo

  **What to do**:
  - Verify the existing uncommitted changes in VLS `src/device/f469.rs` and `src/device/mod.rs` compile correctly
  - Run `cargo build --features stm32f469 --release` in VLS repo to confirm no regressions
  - Stage `src/device/f469.rs` and `src/device/mod.rs` and commit with descriptive message
  - NOTE: VLS depends on BSP via path dependency, so BSP SDRAM commit (Task 1) must exist first. However, since the current code already compiles with the uncommitted BSP changes, this task CAN run in parallel with Task 1 — the uncommitted BSP sdram.rs is already on disk.

  **Must NOT do**:
  - Do NOT modify any code — this is a commit-only task
  - Do NOT touch files other than `src/device/f469.rs` and `src/device/mod.rs`
  - Do NOT commit unrelated untracked files in the VLS repo (e.g., `docs/` directory)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Commit workflow, staging specific files
  - **Skills Evaluated but Omitted**:
    - None — simple git commit task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4, 5, 6)
  - **Blocks**: Tasks 9, 10, 11 (VLS updates that build on these committed changes)
  - **Blocked By**: None (can start immediately — BSP changes already on disk)

  **References**:

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` — Contains `init_display()` which uses BSP `sdram::split_sdram_pins()`, `sdram::Sdram`, and `lcd::Lcd`. Lines ~25-85. These are the SDRAM-related changes to commit.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` — Contains `make_devices()` at lines ~380-547. The F469 block (lines ~487-520) calls `board::init_display()`. Changes here relate to SDRAM consumption path.

  **WHY Each Reference Matters**:
  - The executor must verify ONLY SDRAM-related changes are staged — not the later modifications we'll make for SDIO/touch/button/USB
  - If `mod.rs` has changes beyond SDRAM, only the SDRAM-related hunks should be committed. Use `git add -p` if needed to stage selectively.

  **Acceptance Criteria**:
  - [ ] `cargo build --features stm32f469 --release` succeeds in VLS repo
  - [ ] `git log -1 --oneline` in VLS repo shows the new commit
  - [ ] `git diff src/device/f469.rs src/device/mod.rs` shows only non-SDRAM changes remaining (SDIO inline code, touch, button, USB)

  **QA Scenarios:**

  ```
  Scenario: VLS compiles after SDRAM commit
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32
    Steps:
      1. Run `cargo build --features stm32f469 --release` — should compile without errors
      2. Run `git log -1 --oneline` — should show commit with "SDRAM" or "sdram" in message
      3. Run `git diff --stat src/device/f469.rs src/device/mod.rs` — should show remaining (non-SDRAM) changes
    Expected Result: Build succeeds, commit exists, remaining changes are for future tasks
    Failure Indicators: Compilation errors, commit missing, all changes accidentally committed
    Evidence: .sisyphus/evidence/task-2-vls-sdram-commit.txt
  ```

  **Commit**: YES
  - Message: `feat(f469): use BSP SDRAM abstractions instead of inline pin setup`
  - Files: `src/device/f469.rs`, `src/device/mod.rs` (SDRAM-related hunks only)
  - Pre-commit: `cargo build --features stm32f469 --release`

- [ ] 3. Create BSP `sdio.rs` Module + Update `Cargo.toml`

  **What to do**:
  - Create `stm32f469i-disc/src/sdio.rs` with a public init function that:
    - Accepts `pac::SDIO` peripheral, `SdramRemainders` struct, and `&mut Rcc`
    - Extracts SDIO pins (PC8, PC9, PC10, PC11, PC12, PD2) from `SdramRemainders`
    - Returns the remaining pin PC1 (for touch interrupt) alongside the initialized `Sdio<SdCard>`
    - Pin configuration must EXACTLY match the current inline VLS code at `mod.rs:487-520` (the F469 block)
  - Update `stm32f469i-disc/Cargo.toml`:
    - Add `"sdio-host"` to the `features` list in the `[dependencies.stm32f4xx-hal]` section
    - Add `"sdio"` to the features if not already present
  - Add `pub mod sdio;` to `stm32f469i-disc/src/lib.rs`
  - The function signature should be:
    ```rust
    pub fn init(
        sdio_pac: pac::SDIO,
        remainders: sdram::SdramRemainders,
        rcc: &mut Rcc,
    ) -> (Sdio<SdCard>, hal::gpio::PC1<hal::gpio::Input>)
    ```

  **Must NOT do**:
  - Do NOT add SD card detection or hot-plug support
  - Do NOT add error handling beyond what the HAL `Sdio::new()` provides
  - Do NOT change the SDIO clock or bus width from current VLS defaults
  - Do NOT modify `sdram.rs`

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
    - No special skills needed — standard Rust embedded module creation
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task (commit is Task 8)

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4, 5, 6)
  - **Blocks**: Tasks 7 (BSP verification), 10 (VLS integration)
  - **Blocked By**: Task 1 (needs SDRAM commit for clean SdramRemainders type). NOTE: In practice, since sdram.rs is already on disk (just uncommitted), this CAN start in parallel with Task 1.

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/sdram.rs:SdramRemainders` — The struct consumed by this module. Fields: `pc1` (touch), `pc8-pc12` (SDIO D0-D3, CLK), `pd2` (SDIO CMD). Study this struct to understand what pins are available.
  - `stm32f469i-disc/src/lcd.rs` — Existing BSP module pattern to follow. Look at: imports, function signatures, how it takes individual peripherals and returns initialized hardware.

  **API/Type References**:
  - `stm32f4xx-hal/src/sdio.rs` — HAL SDIO driver. Key: `Sdio::new<PINS: Pins>(sdio: SDIO, pins: PINS, rcc: &mut Rcc)`. The `Pins` trait is implemented for tuples of correctly-typed GPIO pins.
  - HAL sdio `Pins` trait — Check which pin tuples satisfy `Pins`. Likely `(PD2<alt::...>, PC8<alt::...>, PC9<alt::...>, PC10<alt::...>, PC11<alt::...>, PC12<alt::...>)` in the correct alt mode.

  **Code to Extract (current VLS inline SDIO setup)**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~493-520 — The F469 block has inline SDIO pin configuration. This is the EXACT code to move into BSP. Look for `pc8`, `pc9`, `pc10`, `pc11`, `pc12`, `pd2` pin setup and `Sdio::new()` call.

  **External References**:
  - `sdio-host` crate — provides `SdCard` type. BSP Cargo.toml may need to re-export or depend on this.

  **WHY Each Reference Matters**:
  - `SdramRemainders` struct tells you exactly which pins are available and their types after SDRAM init
  - `lcd.rs` shows the BSP coding style: how to structure imports, function signatures, and return types
  - VLS inline code shows the exact pin alternate function modes and Sdio::new() call pattern
  - HAL `sdio.rs` shows the `Pins` trait requirements for type-safe pin passing

  **Acceptance Criteria**:
  - [ ] File `stm32f469i-disc/src/sdio.rs` exists
  - [ ] `pub mod sdio;` in `stm32f469i-disc/src/lib.rs`
  - [ ] `Cargo.toml` has `"sdio-host"` feature enabled for stm32f4xx-hal
  - [ ] `cargo build` succeeds in BSP repo
  - [ ] Function signature matches: takes `pac::SDIO`, `SdramRemainders`, `&mut Rcc`; returns `(Sdio<SdCard>, PC1)`

  **QA Scenarios:**

  ```
  Scenario: BSP sdio module compiles and has correct API
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Run `cargo build` — should succeed with new sdio module
      2. Verify file exists: `ls src/sdio.rs` — should show the file
      3. Verify module exported: `grep 'pub mod sdio' src/lib.rs` — should find the declaration
      4. Verify Cargo.toml updated: `grep 'sdio' Cargo.toml` — should show sdio-host feature
    Expected Result: Module exists, compiles, is exported, and Cargo.toml has required features
    Failure Indicators: Missing file, compilation errors, missing module export, missing Cargo feature
    Evidence: .sisyphus/evidence/task-3-bsp-sdio-module.txt

  Scenario: sdio.rs function signature is correct
    Tool: Bash
    Preconditions: src/sdio.rs exists
    Steps:
      1. Run `grep -n 'pub fn' src/sdio.rs` — should show init function taking SdramRemainders
      2. Run `grep -n 'SdramRemainders' src/sdio.rs` — should show usage of the type
      3. Run `grep -n 'PC1' src/sdio.rs` — should show PC1 in return type
    Expected Result: Function accepts SdramRemainders, returns tuple with Sdio and PC1
    Failure Indicators: Wrong parameter types, missing SdramRemainders consumption, PC1 not returned
    Evidence: .sisyphus/evidence/task-3-sdio-signature.txt
  ```

  **Commit**: NO (groups with Task 8)

- [ ] 4. Create BSP `button.rs` Module

  **What to do**:
  - Create `stm32f469i-disc/src/button.rs` with a trivial init function that:
    - Accepts `PA0` pin (in any input state)
    - Configures it as pull-down input
    - Returns the configured pin
  - Add `pub mod button;` to `stm32f469i-disc/src/lib.rs`
  - Function signature:
    ```rust
    pub fn init(pa0: hal::gpio::PA0) -> hal::gpio::PA0<hal::gpio::Input>
    ```
  - This module should be under 30 lines including imports
  - Follow the BSP style from `sdram.rs` and `lcd.rs` (minimal, functional, no overengineering)

  **Must NOT do**:
  - Do NOT add debouncing logic
  - Do NOT add interrupt configuration
  - Do NOT add button event handling or state machines
  - Do NOT use traits or generics — this is a concrete pin wrapper

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
    - No special skills — trivial module, under 30 lines
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 5, 6)
  - **Blocks**: Tasks 7 (BSP verification), 10 (VLS integration)
  - **Blocked By**: None (can start immediately — no dependency on SDRAM)

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/led.rs` — Simplest existing BSP module. Follow its structure for imports and style.
  - `stm32f469i-disc/src/sdram.rs` — Shows how to import HAL types (`use stm32f4xx_hal as hal; use hal::gpio::*;`)

  **Code to Extract (current VLS button init)**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` line ~533 — `let button = gpioa.pa0.into_pull_down_input();` — this is the EXACT code to move into BSP. One line.

  **WHY Each Reference Matters**:
  - `led.rs` is the simplest BSP module — copy its file structure
  - VLS line 533 shows the exact pin configuration: PA0 pull-down input

  **Acceptance Criteria**:
  - [ ] File `stm32f469i-disc/src/button.rs` exists
  - [ ] `pub mod button;` in `stm32f469i-disc/src/lib.rs`
  - [ ] `cargo build` succeeds in BSP repo
  - [ ] Module is under 30 lines
  - [ ] Function takes PA0 and returns PA0<Input>

  **QA Scenarios:**

  ```
  Scenario: BSP button module compiles and is minimal
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Run `cargo build` — should succeed
      2. Run `wc -l src/button.rs` — should be under 30 lines
      3. Run `grep 'pub fn' src/button.rs` — should show init function
      4. Run `grep 'pub mod button' src/lib.rs` — should find the declaration
    Expected Result: Module exists, compiles, is under 30 lines, and is exported
    Failure Indicators: Missing file, compilation errors, over 30 lines, missing export
    Evidence: .sisyphus/evidence/task-4-bsp-button-module.txt

  Scenario: No over-engineering in button module
    Tool: Bash
    Preconditions: src/button.rs exists
    Steps:
      1. Run `grep -c 'trait\|impl\|struct\|enum\|macro' src/button.rs` — should be 0 or minimal
      2. Run `grep -c 'debounce\|interrupt\|event\|state' src/button.rs` — should be 0
    Expected Result: No traits, structs, debouncing, interrupts, or event handling
    Failure Indicators: Any trait definitions, debounce logic, interrupt config, or event handling
    Evidence: .sisyphus/evidence/task-4-button-no-overengineering.txt
  ```

  **Commit**: NO (groups with Task 8)

- [ ] 5. Create BSP `usb.rs` Module

  **What to do**:
  - Create `stm32f469i-disc/src/usb.rs` with a function that:
    - Accepts USB OTG FS peripheral parts (`OTG_FS_GLOBAL`, `OTG_FS_DEVICE`, `OTG_FS_PWRCLK`), PA11, PA12, and `&Clocks`
    - Constructs and returns the HAL `USB` struct
    - Pin configuration must EXACTLY match current VLS code: PA11 → USB DM (alternate), PA12 → USB DP (alternate)
  - Add `pub mod usb;` to `stm32f469i-disc/src/lib.rs`
  - Function signature:
    ```rust
    pub fn init(
        otg_fs_global: pac::OTG_FS_GLOBAL,
        otg_fs_device: pac::OTG_FS_DEVICE,
        otg_fs_pwrclk: pac::OTG_FS_PWRCLK,
        pa11: hal::gpio::PA11,
        pa12: hal::gpio::PA12,
        clocks: &hal::rcc::Clocks,
    ) -> hal::otg_fs::USB
    ```
  - Alternatively, use the HAL `USB::new()` constructor if it provides a cleaner API
  - Update BSP `Cargo.toml` to enable `"usb_fs"` feature for stm32f4xx-hal

  **Must NOT do**:
  - Do NOT implement USB device stack or any USB protocol handling
  - Do NOT implement `SerialDriver` or any serial communication
  - Do NOT add USB interrupts or DMA
  - Do NOT touch `usbserial.rs` in VLS

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
    - No special skills — straightforward pin/peripheral aggregation
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 4, 6)
  - **Blocks**: Tasks 7 (BSP verification), 10 (VLS integration)
  - **Blocked By**: None (can start immediately — no SDRAM dependency)

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/lcd.rs` — Existing BSP module that aggregates peripherals. Follow its style.

  **API/Type References**:
  - `stm32f4xx-hal/src/otg_fs.rs` — HAL USB struct definition. Key fields: `usb_global`, `usb_device`, `usb_pwrclk`, `pin_dm: alt::Dm`, `pin_dp: alt::Dp`, `hclk: Hertz`. Also has `USB::new()` constructor.

  **Code to Extract (current VLS USB setup)**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~399-408 — USB struct construction:
    ```rust
    let serial = SerialDriver::new(USB {
        usb_global: p.OTG_FS_GLOBAL,
        usb_device: p.OTG_FS_DEVICE,
        usb_pwrclk: p.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate().into(),
        pin_dp: gpioa.pa12.into_alternate().into(),
        hclk: rcc.clocks.hclk(),
    });
    ```
    BSP module should construct the `USB` struct. VLS will then pass it to `SerialDriver::new()`.

  **WHY Each Reference Matters**:
  - HAL `otg_fs.rs` shows the exact type needed (`USB` struct) and its `new()` constructor alternative
  - VLS lines 399-408 show the exact pin alternate modes and peripheral construction pattern to replicate
  - `lcd.rs` provides the BSP coding style template

  **Acceptance Criteria**:
  - [ ] File `stm32f469i-disc/src/usb.rs` exists
  - [ ] `pub mod usb;` in `stm32f469i-disc/src/lib.rs`
  - [ ] `Cargo.toml` has `"usb_fs"` feature enabled for stm32f4xx-hal
  - [ ] `cargo build` succeeds in BSP repo
  - [ ] Function returns `hal::otg_fs::USB` struct, nothing more

  **QA Scenarios:**

  ```
  Scenario: BSP usb module compiles and has correct API
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Run `cargo build` — should succeed with new usb module
      2. Run `grep 'pub fn' src/usb.rs` — should show init function
      3. Run `grep 'pub mod usb' src/lib.rs` — should find the declaration
      4. Run `grep 'usb_fs' Cargo.toml` — should show feature enabled
    Expected Result: Module exists, compiles, is exported, returns USB struct
    Failure Indicators: Missing file, compilation errors, missing export, USB stack implementation
    Evidence: .sisyphus/evidence/task-5-bsp-usb-module.txt

  Scenario: No USB stack implementation
    Tool: Bash
    Preconditions: src/usb.rs exists
    Steps:
      1. Run `grep -c 'UsbBus\|UsbDevice\|EndpointIn\|EndpointOut\|serial\|cdc' src/usb.rs` — should be 0
      2. Run `wc -l src/usb.rs` — should be under 40 lines
    Expected Result: No USB protocol code, module is small
    Failure Indicators: USB device stack types, serial protocol implementation, file over 40 lines
    Evidence: .sisyphus/evidence/task-5-usb-no-stack.txt
  ```

  **Commit**: NO (groups with Task 8)

- [ ] 6. Refactor BSP `touch.rs` + Add Calibration + Update Example

  **What to do**:
  This is the most complex BSP task. Three changes in one:

  **6a. Refactor `init_i2c()` to accept individual pins:**
  - Change signature from `init_i2c(i2c: I2C1, gpiob: gpiob::Parts, gpioc: gpioc::Parts, rcc: &mut Rcc)`
  - To: `init_i2c(i2c: I2C1, pb8: PB8, pb9: PB9, rcc: &mut Rcc) -> I2c<I2C1>`
  - Remove all code that splits `gpiob` and `gpioc` — caller passes specific pins
  - The function no longer returns `PC1` — SDIO module (Task 3) already handles PC1 via `SdramRemainders`

  **6b. Add `init_touchscreen()` function with calibration:**
  - New function that combines the current `init_ft6x06()` with VLS `make_touchscreen()` calibration
  - Signature:
    ```rust
    pub fn init_touchscreen<T: InputPin>(
        i2c: &I2c<I2C1>,
        ts_int: T,
    ) -> Option<Ft6X06<I2c<I2C1>, T>>
    ```
  - Inside: call `Ft6X06::new(i2c, FT6X06_I2C_ADDR, ts_int)`, then `ts_calibration()`, then return the driver
  - The calibration code to move from VLS is in `mod.rs:289-303` (`make_touchscreen()` function)
  - Keep existing `init_ft6x06()` for backward compatibility (it's used by `display_touch.rs` example too) OR merge them

  **6c. Update `display_touch.rs` example:**
  - Change from `touch::init_i2c(dp.I2C1, gpiob, gpioc, &mut rcc)` to the new individual-pin signature
  - The example uses `sdram_pins!` macro (not `split_sdram_pins`), so it has its own SDRAM init path
  - Adjust the example to pass `gpiob.pb8` and `gpiob.pb9` individually
  - Adjust how `PC1` is obtained (likely from `gpioc.pc1` directly since this example doesn't use `SdramRemainders`)

  **Must NOT do**:
  - Do NOT change calibration values or algorithm
  - Do NOT add touch filtering or gesture recognition
  - Do NOT change the `Ft6X06` driver behavior
  - Do NOT change I2C address (0x38)
  - Do NOT remove `init_ft6x06()` if `display_touch.rs` still needs it (check first)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
    - No special skills — Rust embedded refactoring, but complex enough for unspecified-high
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 4, 5)
  - **Blocks**: Tasks 7 (BSP verification), 10, 11 (VLS integration depends on new touch API)
  - **Blocked By**: Task 1 (conceptually, for SdramRemainders type). In practice can start immediately since sdram.rs is on disk.

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/touch.rs` — The file being refactored. Currently 74 lines. Key functions: `init_i2c()` (lines ~20-47, takes Parts), `init_ft6x06()` (lines ~49-74, creates Ft6X06 driver). `FT6X06_I2C_ADDR = 0x38` constant.

  **Code to Move (VLS make_touchscreen + calibration)**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~289-303 — `make_touchscreen()` function. Creates `Ft6X06::new()`, calls `ts_calibration()`, returns the driver wrapped in `TouchDriver`. The calibration call is the key piece to move to BSP.

  **Example to Update**:
  - `stm32f469i-disc/examples/display_touch.rs` — Lines ~40-60 use old `touch::init_i2c(dp.I2C1, gpiob, gpioc, &mut rcc)` API. Must be updated to pass individual pins. This example uses `sdram_pins!` macro for SDRAM, so PC1 comes from gpioc split, not SdramRemainders.

  **API/Type References**:
  - `ft6x06` crate: `Ft6X06::new(i2c, addr, int_pin) -> Self`, `ts_calibration(&mut self)` method
  - HAL I2C: `I2c::new(i2c: I2C1, pins: (impl Into<SclPin>, impl Into<SdaPin>), mode, clocks: &Clocks)` — check exact pin types for PB8/PB9

  **WHY Each Reference Matters**:
  - Current `touch.rs` shows what to refactor and what to keep
  - VLS `make_touchscreen()` contains the calibration code to absorb
  - `display_touch.rs` is the only consumer besides VLS — must be updated or it won't compile
  - The ft6x06 crate API tells you exactly what the driver needs

  **Acceptance Criteria**:
  - [ ] `init_i2c()` takes `PB8`, `PB9` individually — NOT `gpiob::Parts` or `gpioc::Parts`
  - [ ] New `init_touchscreen()` function exists that creates Ft6X06 + runs calibration
  - [ ] `FT6X06_I2C_ADDR` constant preserved (0x38)
  - [ ] `display_touch.rs` example compiles: `cargo build --release --example display_touch`
  - [ ] `cargo build` succeeds in BSP repo
  - [ ] No references to `gpiob::Parts` or `gpioc::Parts` remain in `touch.rs`

  **QA Scenarios:**

  ```
  Scenario: Refactored touch module compiles with individual pins
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Run `cargo build` — should succeed with refactored touch module
      2. Run `grep -c 'Parts' src/touch.rs` — should be 0 (no more Parts references)
      3. Run `grep 'init_i2c' src/touch.rs` — should show PB8, PB9 in signature
      4. Run `grep 'init_touchscreen\|calibrat' src/touch.rs` — should find calibration function
    Expected Result: Module compiles, uses individual pins, has calibration
    Failure Indicators: Parts references remain, compilation errors, missing calibration
    Evidence: .sisyphus/evidence/task-6-touch-refactor.txt

  Scenario: display_touch example compiles with new API
    Tool: Bash
    Preconditions: touch.rs has been refactored
    Steps:
      1. Run `cargo build --release --example display_touch` — should succeed
      2. Run `grep 'init_i2c' examples/display_touch.rs` — should show individual pins, not Parts
    Expected Result: Example compiles, uses new API
    Failure Indicators: Compilation error in example, old API still referenced
    Evidence: .sisyphus/evidence/task-6-display-touch-example.txt
  ```

  **Commit**: NO (groups with Task 8)

- [ ] 7. Verify All BSP Builds (Library + All Examples)

  **What to do**:
  - Run a comprehensive build check of the entire BSP repo to catch any cross-module issues:
    - `cargo build` (library)
    - `cargo build --release --example display_touch`
    - `cargo build --release --example display_dsi_lcd`
    - `cargo build --release --example fmc_sdram_test` (if it exists)
  - Fix any compilation errors found
  - This task is a GATE — nothing in Wave 2 proceeds until all BSP builds pass

  **Must NOT do**:
  - Do NOT change any module's public API at this point — only fix compilation issues
  - Do NOT add new functionality
  - Do NOT modify examples beyond what's needed to compile

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (first task, gate for rest of wave)
  - **Blocks**: Task 8 (commit)
  - **Blocked By**: Tasks 1, 3, 4, 5, 6 (all BSP module creation tasks)

  **References**:

  **Pattern References**:
  - All BSP source files: `src/lib.rs`, `src/sdram.rs`, `src/sdio.rs`, `src/touch.rs`, `src/button.rs`, `src/usb.rs`
  - All BSP examples: `examples/display_touch.rs`, `examples/display_dsi_lcd.rs`

  **WHY Each Reference Matters**:
  - Comprehensive build catches inter-module type mismatches that per-module builds might miss
  - Examples are the real integration tests for BSP modules

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds
  - [ ] `cargo build --release --example display_touch` succeeds
  - [ ] `cargo build --release --example display_dsi_lcd` succeeds
  - [ ] All build output captured as evidence

  **QA Scenarios:**

  ```
  Scenario: Full BSP compilation check
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc. All module tasks (1, 3-6) complete.
    Steps:
      1. Run `cargo build` — should succeed
      2. Run `cargo build --release --example display_touch` — should succeed
      3. Run `cargo build --release --example display_dsi_lcd` — should succeed
    Expected Result: All builds pass with zero errors
    Failure Indicators: Any compilation error in library or examples
    Evidence: .sisyphus/evidence/task-7-bsp-full-build.txt
  ```

  **Commit**: NO (gate task, commit is Task 8)

- [ ] 8. Commit All BSP Module Additions

  **What to do**:
  - Stage all new and modified BSP files from Tasks 3-6:
    - `src/sdio.rs` (new)
    - `src/button.rs` (new)
    - `src/usb.rs` (new)
    - `src/touch.rs` (modified)
    - `src/lib.rs` (modified — new module declarations)
    - `Cargo.toml` (modified — new features)
    - `examples/display_touch.rs` (modified — new touch API)
  - Commit with a single descriptive message
  - Verify commit landed cleanly

  **Must NOT do**:
  - Do NOT modify any code — this is a commit-only task
  - Do NOT squash with the SDRAM commit (Task 1) — keep them separate
  - Do NOT commit any files not listed above

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Multi-file staging, commit workflow
  - **Skills Evaluated but Omitted**:
    - None — simple commit task

  **Parallelization**:
  - **Can Run In Parallel**: NO (sequential gate)
  - **Parallel Group**: Wave 2 (after Task 7)
  - **Blocks**: Tasks 9, 10, 11 (VLS updates depend on committed BSP)
  - **Blocked By**: Task 7 (all BSP builds must pass first)

  **References**:

  **Files to Commit**:
  - `stm32f469i-disc/src/sdio.rs` — New SDIO module from Task 3
  - `stm32f469i-disc/src/button.rs` — New button module from Task 4
  - `stm32f469i-disc/src/usb.rs` — New USB module from Task 5
  - `stm32f469i-disc/src/touch.rs` — Refactored from Task 6
  - `stm32f469i-disc/src/lib.rs` — New module exports
  - `stm32f469i-disc/Cargo.toml` — New features
  - `stm32f469i-disc/examples/display_touch.rs` — Updated for new touch API

  **WHY Each Reference Matters**:
  - Executor needs the complete list of files to stage — missing any file would leave the BSP in a broken state between commits

  **Acceptance Criteria**:
  - [ ] `git log -1 --oneline` shows the new commit
  - [ ] `git diff --cached` is empty
  - [ ] All 7 files listed above are in the commit
  - [ ] `cargo build` still succeeds after commit

  **QA Scenarios:**

  ```
  Scenario: BSP commit contains all module files
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc. Task 7 passed.
    Steps:
      1. Run `git add src/sdio.rs src/button.rs src/usb.rs src/touch.rs src/lib.rs Cargo.toml examples/display_touch.rs`
      2. Run `git commit -m "feat: add sdio, button, usb modules; refactor touch for individual pins"`
      3. Run `git log -1 --stat` — should show all 7 files
      4. Run `git status --short` — should show clean (no modified tracked files)
    Expected Result: Commit contains exactly the 7 files, repo is clean
    Failure Indicators: Missing files in commit, extra unintended files, dirty working tree
    Evidence: .sisyphus/evidence/task-8-bsp-commit.txt
  ```

  **Commit**: YES
  - Message: `feat: add sdio, button, usb modules; refactor touch for individual pins`
  - Files: `src/sdio.rs`, `src/button.rs`, `src/usb.rs`, `src/touch.rs`, `src/lib.rs`, `Cargo.toml`, `examples/display_touch.rs`
  - Pre-commit: `cargo build && cargo build --release --example display_touch`

- [ ] 9. Update VLS `f469.rs` — Remove Redundant Init Functions

  **What to do**:
  - Remove `init_i2c()` function from `f469.rs` — BSP `touch::init_i2c()` now handles this with individual pins
  - Remove `init_touch_int()` function from `f469.rs` — PC1 is now obtained from SDIO init via `SdramRemainders`
  - Keep all other functions in `f469.rs`:
    - `init_display()` — KEEP (uses BSP sdram + lcd)
    - `touch_to_grid()` — KEEP (application-level coordinate mapping)
    - `transform_touch_coords()` — KEEP (application-level)
    - `make_clocks()` — KEEP (clock configuration)
    - Type aliases (`I2C`, `TouchInterruptPin`, `DisplayInner`) — KEEP
  - Update imports if needed (remove unused imports from deleted functions)

  **Must NOT do**:
  - Do NOT change `init_display()` — it's already using BSP correctly
  - Do NOT change `touch_to_grid()` or `transform_touch_coords()` — application-level logic stays in VLS
  - Do NOT change `make_clocks()` — clock config stays in VLS
  - Do NOT change type aliases
  - Do NOT touch F412/F413 files

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
    - No special skills — Rust refactoring with careful function removal
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 10, 11, after Tasks 7, 8)
  - **Blocks**: Task 12 (VLS commit)
  - **Blocked By**: Tasks 2, 8 (needs SDRAM commit in VLS + BSP modules committed)

  **References**:

  **Code to Remove**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` function `init_i2c()` — Currently at lines ~100-120. Creates `I2c::new()` with `gpiob.pb8`, `gpiob.pb9`. Redundant because BSP `touch::init_i2c()` now does this.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` function `init_touch_int()` — Currently at lines ~122-130. Configures PC1 as pull-down input. Redundant because PC1 now comes from BSP `sdio::init()` return value.

  **Code to Keep (verify NOT deleted)**:
  - `f469.rs::init_display()` — Already using BSP, keep as-is
  - `f469.rs::touch_to_grid()` and `transform_touch_coords()` — App-level coordinate mapping
  - `f469.rs::make_clocks()` — Board-specific clock configuration (HSE 8MHz, sysclk 180MHz)
  - Type aliases: `I2C`, `TouchInterruptPin`, `DisplayInner`

  **WHY Each Reference Matters**:
  - Must verify exactly which functions to remove vs keep — removing the wrong function breaks everything
  - Type aliases must be kept because VLS code elsewhere references them

  **Acceptance Criteria**:
  - [ ] `init_i2c()` function no longer exists in `f469.rs`
  - [ ] `init_touch_int()` function no longer exists in `f469.rs`
  - [ ] `init_display()`, `touch_to_grid()`, `transform_touch_coords()`, `make_clocks()` still exist
  - [ ] All type aliases still present
  - [ ] No unused import warnings

  **QA Scenarios:**

  ```
  Scenario: Redundant VLS f469 functions removed
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32
    Steps:
      1. Run `grep -n 'fn init_i2c' src/device/f469.rs` — should return nothing (function removed)
      2. Run `grep -n 'fn init_touch_int' src/device/f469.rs` — should return nothing (function removed)
      3. Run `grep -n 'fn init_display\|fn touch_to_grid\|fn transform_touch\|fn make_clocks' src/device/f469.rs` — should show all 4 functions still present
      4. Run `grep -n 'type I2C\|type TouchInterruptPin\|type DisplayInner' src/device/f469.rs` — should show all type aliases present
    Expected Result: Only redundant functions removed, all keepers preserved
    Failure Indicators: init_i2c or init_touch_int still present, OR keeper functions missing
    Evidence: .sisyphus/evidence/task-9-vls-f469-cleanup.txt
  ```

  **Commit**: NO (groups with Task 12)

- [ ] 10. Update VLS `mod.rs` — Use BSP Modules for All Peripherals

  **What to do**:
  This is the most impactful VLS change. Update `make_devices()` F469 block to use all BSP modules:

  **10a. Replace inline SDIO setup with BSP call:**
  - Find the inline SDIO pin configuration in the F469 block (lines ~493-520)
  - Replace with: `let (sdio, pc1) = stm32f469i_disc::sdio::init(p.SDIO, remainders, &mut rcc);`
  - Where `remainders` comes from the `split_sdram_pins()` call earlier in the F469 block

  **10b. Replace touch init with BSP calls:**
  - Replace `board::init_i2c(dp.I2C1, gpiob, gpioc, &mut rcc)` with `stm32f469i_disc::touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc)`
  - Replace `make_touchscreen()` call with `stm32f469i_disc::touch::init_touchscreen(&i2c, pc1)` (where `pc1` comes from SDIO init)
  - Remove or gate `make_touchscreen()` function if it's no longer needed for F469 (keep for F412/F413 if they still use it)

  **10c. Replace USB setup with BSP call:**
  - Replace the inline `USB { usb_global: ..., pin_dm: gpioa.pa11.into_alternate().into(), ... }` construction
  - With: `let usb = stm32f469i_disc::usb::init(p.OTG_FS_GLOBAL, p.OTG_FS_DEVICE, p.OTG_FS_PWRCLK, gpioa.pa11, gpioa.pa12, &rcc.clocks);`
  - Then: `let serial = SerialDriver::new(usb);`

  **10d. Replace button init with BSP call:**
  - Replace `let button = gpioa.pa0.into_pull_down_input();`
  - With: `let button = stm32f469i_disc::button::init(gpioa.pa0);`

  **10e. Clean up duplicate comments:**
  - Remove duplicate comment on lines ~523-524

  **10f. Handle make_touchscreen() for non-F469 boards:**
  - If F412/F413 still call `make_touchscreen()`, keep the function but gate the F469 path to use BSP
  - The function currently creates `Ft6X06::new()` + `ts_calibration()` — for F469, BSP now does this
  - Check if `make_touchscreen()` is used outside the F469 `#[cfg]` block. If so, keep it for those boards.

  **Must NOT do**:
  - Do NOT change `DeviceContext` struct — same fields, different init path
  - Do NOT change `SerialDriver::new()` call beyond passing BSP-constructed USB
  - Do NOT change RNG or timer initialization — those stay in VLS (not board-specific)
  - Do NOT touch F412/F413 `#[cfg]` blocks
  - Do NOT change the order of initialization if it matters for hardware (SDRAM → SDIO → touch is the correct order)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
    - No special skills — complex Rust refactoring requiring careful attention to pin ownership
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 9, 11)
  - **Blocks**: Task 12 (VLS commit)
  - **Blocked By**: Tasks 2, 8 (needs SDRAM commit + BSP modules committed)

  **References**:

  **Code to Replace (current VLS mod.rs F469 block)**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~487-535 — The entire F469 `#[cfg]` block in `make_devices()`. This is the PRIMARY reference. Read it line by line.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~399-408 — USB serial construction (inline `USB` struct). This code is OUTSIDE the F469 cfg block but should be brought under F469 gating when using BSP.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~289-303 — `make_touchscreen()` function. Check if it's still needed for F412/F413.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` line ~533 — Button init: `let button = gpioa.pa0.into_pull_down_input();`

  **BSP APIs to Call**:
  - `stm32f469i_disc::sdio::init(pac::SDIO, SdramRemainders, &mut Rcc) -> (Sdio<SdCard>, PC1)`
  - `stm32f469i_disc::touch::init_i2c(I2C1, PB8, PB9, &mut Rcc) -> I2c<I2C1>`
  - `stm32f469i_disc::touch::init_touchscreen(&I2c<I2C1>, PC1) -> Option<Ft6X06<...>>`
  - `stm32f469i_disc::usb::init(OTG_FS_GLOBAL, OTG_FS_DEVICE, OTG_FS_PWRCLK, PA11, PA12, &Clocks) -> USB`
  - `stm32f469i_disc::button::init(PA0) -> PA0<Input>`

  **Pin Ownership Flow** (CRITICAL — executor must understand this):
  - `gpioa` is split early. `pa0` → button, `pa11`/`pa12` → USB. Rest unused.
  - `gpiob` is split. `pb8`/`pb9` → touch I2C. Rest unused by this board.
  - `gpioc` pins come from `SdramRemainders` (after sdram init uses most of GPIOC). `pc1` → touch interrupt (via SDIO init return), `pc8-12` → SDIO.
  - `gpiod` — `pd2` comes from `SdramRemainders` → SDIO CMD.

  **WHY Each Reference Matters**:
  - The F469 block is the heart of what's changing — executor must read every line
  - USB construction is outside the cfg block — may need restructuring
  - make_touchscreen() may still be needed for other boards — check before removing
  - Pin ownership flow prevents double-use compilation errors

  **Acceptance Criteria**:
  - [ ] F469 block calls `stm32f469i_disc::sdio::init()` instead of inline SDIO setup
  - [ ] F469 block calls `stm32f469i_disc::touch::init_i2c()` with individual pins
  - [ ] F469 block calls `stm32f469i_disc::touch::init_touchscreen()` with calibration
  - [ ] F469 block calls `stm32f469i_disc::usb::init()` for USB struct construction
  - [ ] F469 block calls `stm32f469i_disc::button::init()` for user button
  - [ ] `DeviceContext` struct is unchanged
  - [ ] `SerialDriver::new()` still receives a `USB` struct
  - [ ] F412/F413 `#[cfg]` blocks are completely untouched
  - [ ] `cargo build --features stm32f469 --release` succeeds

  **QA Scenarios:**

  ```
  Scenario: VLS F469 uses all BSP modules
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32. BSP commit (Task 8) exists.
    Steps:
      1. Run `cargo build --features stm32f469 --release` — should succeed
      2. Run `grep -c 'stm32f469i_disc::sdio' src/device/mod.rs` — should be >= 1
      3. Run `grep -c 'stm32f469i_disc::touch' src/device/mod.rs` — should be >= 1
      4. Run `grep -c 'stm32f469i_disc::usb' src/device/mod.rs` — should be >= 1
      5. Run `grep -c 'stm32f469i_disc::button' src/device/mod.rs` — should be >= 1
    Expected Result: Build succeeds, all BSP module calls present
    Failure Indicators: Compilation errors, missing BSP module calls, inline setup still present
    Evidence: .sisyphus/evidence/task-10-vls-bsp-integration.txt

  Scenario: No inline peripheral setup remains in F469 block
    Tool: Bash
    Preconditions: mod.rs has been updated
    Steps:
      1. Inspect the F469 cfg block in mod.rs — should NOT contain `.into_alternate()` calls for SDIO/USB pins
      2. Run `grep -n 'into_alternate' src/device/mod.rs` — any matches should be OUTSIDE the F469 block (F412/F413)
      3. Run `grep -n 'into_pull_down_input' src/device/mod.rs` — should NOT be in F469 block (button uses BSP now)
    Expected Result: No inline pin configuration in F469 block
    Failure Indicators: Pin configuration code still inline in F469 block
    Evidence: .sisyphus/evidence/task-10-no-inline-setup.txt

  Scenario: F412/F413 blocks untouched
    Tool: Bash
    Preconditions: mod.rs has been updated
    Steps:
      1. Run `git diff src/device/f412.rs` — should show no changes
      2. Run `git diff src/device/f413.rs` — should show no changes
    Expected Result: Zero changes to F412/F413 files
    Failure Indicators: Any diff output for these files
    Evidence: .sisyphus/evidence/task-10-no-f412-f413-changes.txt
  ```

  **Commit**: NO (groups with Task 12)

- [ ] 11. Remove `make_touchscreen()` from VLS — Touchscreen Init Belongs in BSP

  **What to do**:
  - **Design principle**: Each BSP should own its board's touchscreen initialization (creation + calibration). Long-term, when F412/F413 get their own BSPs, those BSPs will each have their own `init_touchscreen()`. Therefore, `make_touchscreen()` should NOT live in VLS application code.
  - Remove the `make_touchscreen()` function from `mod.rs` (lines ~289-303)
  - The F469 path already uses BSP `touch::init_touchscreen()` (done in Task 10)
  - For F412/F413:
    - Check if they currently call `make_touchscreen()`. If so, INLINE the function body directly into their `#[cfg]` blocks as a temporary measure (so removing the shared function doesn't break them)
    - This is a stopgap until those boards get proper BSPs — it's better than keeping shared touchscreen init in VLS
    - Alternatively, if `make_touchscreen()` is NOT called by F412/F413, simply remove it
  - Keep the `TouchDriver` struct (lines ~305-307) — it's used by approver/setup at runtime
  - Clean up any dead code or unused imports

  **Must NOT do**:
  - Do NOT change F412/F413 touchscreen BEHAVIOR — only restructure where the code lives
  - Do NOT remove `TouchDriver` struct — it's used at runtime
  - Do NOT change touch coordinate mapping (`touch_to_grid`, etc.)
  - Do NOT change calibration values or algorithm

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
    - Requires careful analysis of cfg-gated code paths
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not committing in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 9, 10)
  - **Blocks**: Task 12 (VLS commit)
  - **Blocked By**: Tasks 2, 6, 8 (needs SDRAM commit + BSP touch refactor committed)

  **References**:

  **Code to Remove/Inline**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~289-303 — `make_touchscreen()` function body. Contains: `Ft6X06::new()`, `ts_calibration()`, wraps in `TouchDriver`
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` lines ~305-307 — `TouchDriver` struct (KEEP)
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` line ~529-531 — Where `make_touchscreen()` is called. Check if inside `#[cfg(feature = "stm32f469")]` or shared code.

  **Code to Check for F412/F413 Usage**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs` — Search for `make_touchscreen` calls
  - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs` — Search for `make_touchscreen` calls
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` — Search for ALL `make_touchscreen` call sites

  **WHY Each Reference Matters**:
  - Must find ALL callers before removing the function — inlining into F412/F413 blocks if needed
  - `TouchDriver` is referenced in approver.rs and setup.rs for runtime touch operations

  **Acceptance Criteria**:
  - [ ] `make_touchscreen()` function no longer exists as a shared function in `mod.rs`
  - [ ] If F412/F413 called it: their code now has the Ft6X06 creation inline in their own `#[cfg]` blocks
  - [ ] `TouchDriver` struct preserved
  - [ ] `cargo build --features stm32f469 --release` succeeds
  - [ ] No unused function/import warnings related to touchscreen

  **QA Scenarios:**

  ```
  Scenario: make_touchscreen removed, no regressions
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32
    Steps:
      1. Run `cargo build --features stm32f469 --release` — should succeed
      2. Run `grep -n 'fn make_touchscreen' src/device/mod.rs` — should return nothing (function removed)
      3. Run `grep -n 'TouchDriver' src/device/mod.rs` — should find the struct definition (preserved)
      4. Run `cargo build --features stm32f469 --release 2>&1 | grep -i 'warning.*unused\|warning.*dead_code'` — should return nothing related to touchscreen
    Expected Result: Function removed, struct preserved, builds clean
    Failure Indicators: Function still exists, struct missing, compilation errors, dead code warnings
    Evidence: .sisyphus/evidence/task-11-touchscreen-handling.txt
  ```

  **Commit**: NO (groups with Task 12)

- [ ] 12. Commit All VLS Changes

  **What to do**:
  - Verify VLS compiles: `cargo build --features stm32f469 --release`
  - Stage all modified VLS files:
    - `src/device/f469.rs` (modified — redundant functions removed)
    - `src/device/mod.rs` (modified — BSP module calls, touchscreen handling)
  - Commit with a single descriptive message
  - Do NOT commit untracked files (e.g., `docs/` directory)

  **Must NOT do**:
  - Do NOT modify any code — this is a commit-only task
  - Do NOT commit `docs/` or other untracked files
  - Do NOT amend the SDRAM commit (Task 2)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Selective staging, commit workflow
  - **Skills Evaluated but Omitted**:
    - None — simple commit task

  **Parallelization**:
  - **Can Run In Parallel**: NO (sequential)
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 13 (final verification)
  - **Blocked By**: Tasks 9, 10, 11 (all VLS modification tasks)

  **References**:

  **Files to Commit**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` — Redundant functions removed (Task 9)
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` — BSP module integration (Tasks 10, 11)

  **WHY Each Reference Matters**:
  - These are the ONLY files to commit — do NOT include anything else

  **Acceptance Criteria**:
  - [ ] `cargo build --features stm32f469 --release` succeeds before commit
  - [ ] `git log -1 --oneline` shows the new commit
  - [ ] `git diff src/device/f469.rs src/device/mod.rs` is empty after commit
  - [ ] `git diff src/device/f412.rs src/device/f413.rs` shows no changes (untouched)

  **QA Scenarios:**

  ```
  Scenario: VLS commit is clean and complete
    Tool: Bash
    Preconditions: Working directory is /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32. Tasks 9-11 complete.
    Steps:
      1. Run `cargo build --features stm32f469 --release` — must succeed
      2. Run `git add src/device/f469.rs src/device/mod.rs`
      3. Run `git commit -m "feat(f469): use BSP modules for all peripheral init"`
      4. Run `git log -1 --stat` — should show exactly 2 files
      5. Run `git status --short` — should show only `?? docs/` (untracked)
    Expected Result: Commit contains exactly 2 files, build passes, repo clean except untracked docs
    Failure Indicators: Extra files in commit, build failure, unexpected dirty files
    Evidence: .sisyphus/evidence/task-12-vls-commit.txt
  ```

  **Commit**: YES
  - Message: `feat(f469): use BSP modules for all peripheral init`
  - Files: `src/device/f469.rs`, `src/device/mod.rs`
  - Pre-commit: `cargo build --features stm32f469 --release`

- [ ] 13. Full Cross-Compilation Verification

  **What to do**:
  - Run ALL verification commands from scratch to confirm the entire migration is complete:
  - BSP repo:
    - `cargo build` (library)
    - `cargo build --release --example display_touch`
    - `cargo build --release --example display_dsi_lcd`
  - VLS repo:
    - `cargo build --features stm32f469 --release`
  - Git status checks:
    - BSP: `git status --short` — should be clean
    - VLS: `git status --short` — only `?? docs/` untracked
  - Git log checks:
    - BSP: `git log --oneline -3` — should show SDRAM commit + module commit
    - VLS: `git log --oneline -3` — should show SDRAM commit + BSP integration commit
  - Verify no inline peripheral setup in VLS F469 block
  - Verify BSP has all 5 modules: sdram, sdio, touch, button, usb

  **Must NOT do**:
  - Do NOT make any code changes — this is verification only
  - Do NOT commit anything

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: No git operations beyond status/log queries

  **Parallelization**:
  - **Can Run In Parallel**: NO (final gate)
  - **Parallel Group**: Wave 3 (after Task 12)
  - **Blocks**: F1-F4 (final review wave)
  - **Blocked By**: Task 12 (all commits must be done)

  **References**:

  **All BSP Source Files**:
  - `stm32f469i-disc/src/lib.rs` — Module exports (should have: sdram, sdio, touch, button, usb, lcd, led)
  - `stm32f469i-disc/src/sdram.rs`, `src/sdio.rs`, `src/touch.rs`, `src/button.rs`, `src/usb.rs`

  **All VLS Modified Files**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` — F469 block should call BSP modules
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` — Should have `init_display()`, `touch_to_grid()`, `transform_touch_coords()`, `make_clocks()` only

  **WHY Each Reference Matters**:
  - Final verification must check EVERY deliverable from the plan

  **Acceptance Criteria**:
  - [ ] All 4 build commands succeed
  - [ ] BSP git is clean (no modified/untracked source files)
  - [ ] VLS git shows only `docs/` as untracked
  - [ ] BSP has 4 commits total (2 new: SDRAM + modules)
  - [ ] VLS has 2 new commits (SDRAM + BSP integration)
  - [ ] BSP lib.rs exports: sdram, sdio, touch, button, usb, lcd, led

  **QA Scenarios:**

  ```
  Scenario: Complete migration verification
    Tool: Bash
    Preconditions: All tasks 1-12 complete
    Steps:
      1. In BSP: `cargo build` — PASS
      2. In BSP: `cargo build --release --example display_touch` — PASS
      3. In BSP: `cargo build --release --example display_dsi_lcd` — PASS
      4. In VLS: `cargo build --features stm32f469 --release` — PASS
      5. In BSP: `git status --short` — empty
      6. In VLS: `git status --short` — only `?? docs/`
      7. In BSP: `git log --oneline -2` — shows 2 new commits
      8. In VLS: `git log --oneline -2` — shows 2 new commits
      9. Run `grep 'pub mod' stm32f469i-disc/src/lib.rs` — should show sdram, sdio, touch, button, usb, lcd, led
    Expected Result: All builds pass, repos clean, commits present, all modules exported
    Failure Indicators: Any build failure, dirty git status, missing commits, missing module
    Evidence: .sisyphus/evidence/task-13-final-verification.txt

  Scenario: No inline peripheral setup in VLS
    Tool: Bash
    Preconditions: Task 12 complete
    Steps:
      1. Read the F469 cfg block in VLS mod.rs
      2. Verify it calls BSP modules: sdio::init, touch::init_i2c, touch::init_touchscreen, usb::init, button::init
      3. Verify no `.into_alternate()` calls in F469 block (should be in BSP now)
      4. Verify no inline `USB { ... }` struct construction in F469 block
    Expected Result: F469 block is clean, calling only BSP functions
    Failure Indicators: Any inline pin setup or peripheral construction remaining
    Evidence: .sisyphus/evidence/task-13-no-inline-setup.txt
  ```

  **Commit**: NO (verification only)

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `quick`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read files, check git log). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `quick`
  Review all changed/new files across both repos. Check for: unused imports, unnecessary `unsafe`, missing doc comments on public items, AI slop (excessive comments, over-abstraction, generic variable names). Verify no `#[allow(unused)]` or `#[allow(dead_code)]` suppression. Check BSP modules follow existing style (see `sdram.rs`, `lcd.rs` for reference).
  Output: `Files [N clean/N issues] | VERDICT`

- [ ] F3. **Build Verification from Clean State** — `quick`
  Run `cargo clean` then full rebuild in both repos to ensure no stale artifacts:
  1. `cargo clean && cargo build` in BSP repo
  2. `cargo clean && cargo build --release --example display_touch` in BSP repo
  3. `cargo clean && cargo build --features stm32f469 --release` in VLS repo
  Output: `BSP lib [PASS/FAIL] | BSP examples [PASS/FAIL] | VLS [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `quick`
  For each task: read git diff, verify 1:1 match with task spec. Check:
  - No F412/F413 code was touched
  - No HAL files were modified
  - No `DeviceContext` struct changed
  - No `SerialDriver` implementation changed
  - No Board trait was created
  - All changes are within scope
  Output: `Tasks [N/N compliant] | Out-of-scope changes [CLEAN/N issues] | VERDICT`

---

## Commit Strategy

| # | Repo | Message | Files |
|---|------|---------|-------|
| 1 | BSP | `feat(sdram): add SdramRemainders, split_sdram_pins, and Sdram wrapper` | `src/sdram.rs` |
| 2 | VLS | `feat(f469): use BSP SDRAM abstractions instead of inline pin setup` | `src/device/f469.rs`, `src/device/mod.rs` |
| 3 | BSP | `feat: add sdio, button, usb modules; refactor touch for individual pins` | `src/sdio.rs`, `src/button.rs`, `src/usb.rs`, `src/touch.rs`, `src/lib.rs`, `Cargo.toml`, `examples/display_touch.rs` |
| 4 | VLS | `feat(f469): use BSP modules for all peripheral init` | `src/device/f469.rs`, `src/device/mod.rs` |

---

## Success Criteria

### Verification Commands
```bash
# BSP builds
cd /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
cargo build                                      # Expected: success
cargo build --release --example fmc_sdram_test   # Expected: success
cargo build --release --example display_dsi_lcd  # Expected: success
cargo build --release --example display_touch    # Expected: success

# VLS builds
cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32
cargo build --features stm32f469 --release       # Expected: success

# Git status clean
git -C /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc status --short          # Expected: empty
git -C /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer status --short  # Expected: only ?? docs/
```

### Final Checklist
- [ ] All "Must Have" present: SDIO, touch, button, USB in BSP; VLS uses BSP modules
- [ ] All "Must NOT Have" absent: no F412/F413 changes, no HAL changes, no Board trait
- [ ] 4 commits across 2 repos (BSP: 2, VLS: 2)
- [ ] BSP has clean modular abstractions: sdram, sdio, touch, button, usb
- [ ] VLS F469 block calls only BSP functions for board-specific hardware
- [ ] BSP display_touch example works with refactored touch API
