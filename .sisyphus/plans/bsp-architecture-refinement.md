# BSP Architecture Refinement — Display Pipeline & Constants

## TL;DR

> **Quick Summary**: Move the remaining board-specific display initialization from VLS f469.rs into the BSP as a single high-level function. Also migrate screen dimension constants and add HSE frequency constant to BSP.
> 
> **Deliverables**:
> - BSP: New `lcd::init_display_pipeline()` function encapsulating SDRAM→LCD reset→framebuffer→DSI/LTDC→layer config
> - BSP: `pub const HSE_FREQ` in lib.rs
> - VLS: f469.rs simplified from 70-line `init_display()` to ~10-line thin wrapper
> - VLS: Screen constants imported from BSP instead of locally defined
> 
> **Estimated Effort**: Short (3-5 focused tasks)
> **Parallel Execution**: YES — 2 waves
> **Critical Path**: Task 1 (BSP function) → Task 3 (VLS integration) → Task 5 (cross-compile) → F1-F4

---

## Context

### Original Request
User asked for a broader architecture review of HAL/BSP/VLS boundaries after completing the initial BSP migration for STM32F469. Research across three agents (codebase mapping, VLS audit, ecosystem conventions) identified three remaining items where board-specific knowledge lives in VLS instead of BSP.

### Interview Summary
**Key Discussions**:
- Ecosystem conventions: BSPs provide constants + pin mappings, not mandatory side effects
- Clock config stays in VLS (app decides sysclk), BSP provides HSE constant only
- No `default_clock_config()` convenience function
- Display init is 90% board knowledge — strong candidate for BSP
- USB, touch, SDIO, button already correctly in BSP

**Research Findings**:
- `rp-pico`, `nrf52840-dk`, `stm32f407g-disc` BSPs all use modular per-peripheral init
- BSPs typically provide frequency constants, not clock init functions
- BSP `lcd.rs` already defines `WIDTH=480`, `HEIGHT=800`, `FB_SIZE` — VLS just needs to import them
- VLS f469.rs `init_display()` is 70 lines; items 1-6 (GPIO split, SDRAM init, LCD reset, framebuffer alloc, DSI+LTDC init, layer config) are all board knowledge; only item 7 (Display wrapper) is app-specific

### Metis Review
**Identified Gaps** (addressed):
- Return type of new BSP function: BSP returns `(LtdcFramebuffer<u16>, SdramRemainders)`, VLS wraps in its Display type
- Screen constants already exist in BSP lcd.rs — no need to create new ones, just import
- F412/F413 still define their own SCREEN_WIDTH/HEIGHT (240×240) — must not touch those
- `make_clocks()` should use BSP HSE_FREQ constant for consistency (included as nice-to-have in VLS task)

---

## Work Objectives

### Core Objective
Collapse the 70-line VLS `init_display()` function into a single BSP call plus a thin VLS wrapper, and import board constants from BSP instead of defining them locally.

### Concrete Deliverables
- `stm32f469i-disc/src/lcd.rs`: New `init_display_pipeline()` function
- `stm32f469i-disc/src/lib.rs`: `pub const HSE_FREQ` export
- `vls-signer-stm32/src/device/f469.rs`: Simplified `init_display()` using BSP function, imported constants
- Two git commits (one per repo)

### Definition of Done
- [ ] `cargo build --release` succeeds in BSP repo
- [ ] `cargo build --features stm32f469 --release` succeeds in VLS repo
- [ ] `cargo build --features stm32f412 --release` succeeds in VLS repo (no regression)
- [ ] `cargo build --features stm32f413 --release` succeeds in VLS repo (no regression)
- [ ] VLS f469.rs `init_display()` is ≤15 lines (down from 70)
- [ ] No local SCREEN_WIDTH/SCREEN_HEIGHT definitions in VLS f469.rs

### Must Have
- BSP function encapsulates: GPIO split → SDRAM init → LCD reset → framebuffer alloc → DSI+LTDC init → layer config
- BSP function returns `(LtdcFramebuffer<u16>, SdramRemainders)` — VLS wraps in Display type
- HSE_FREQ constant exported from BSP lib.rs
- VLS imports SCREEN_WIDTH/SCREEN_HEIGHT from BSP lcd module
- LCD reset timing preserved exactly (20ms low, 10ms high)
- F412/F413 code completely untouched

### Must NOT Have (Guardrails)
- NO changes to F412/F413 code paths (f412.rs, f413.rs)
- NO changes to HAL repo
- NO `default_clock_config()` or clock init function in BSP
- NO Board trait or generic board abstraction
- NO changes to DeviceContext struct
- NO changes to BSP's existing `lcd::init_display_full()` signature
- NO new abstraction layers — just one new function consolidating existing BSP calls
- NO documentation bloat — minimal doc comments on new function

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (no unit tests in BSP or VLS-signer-stm32)
- **Automated tests**: None
- **Framework**: N/A
- **Primary verification**: Cross-compilation for all three board features

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Build verification**: Use Bash — `cargo build` with appropriate features
- **Code verification**: Use grep/read — verify function signatures, imports, line counts

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — BSP changes, independent):
├── Task 1: Add init_display_pipeline() to BSP lcd.rs [deep]
├── Task 2: Add HSE_FREQ constant to BSP lib.rs [quick]

Wave 2 (After Wave 1 — VLS changes depend on BSP):
├── Task 3: Simplify VLS f469.rs to use BSP display function + import constants [deep]
├── Task 4: Commit both repos [quick]

Wave 3 (After Wave 2 — verification):
├── Task 5: Cross-compile verification for all 3 board features [quick]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
├── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 3 → Task 4 → Task 5 → F1-F4
Parallel Speedup: ~40% faster than sequential
Max Concurrent: 2 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 3, 4 | 1 |
| 2 | — | 3, 4 | 1 |
| 3 | 1, 2 | 4 | 2 |
| 4 | 3 | 5 | 2 |
| 5 | 4 | F1-F4 | 3 |
| F1-F4 | 5 | — | FINAL |

### Agent Dispatch Summary

- **Wave 1**: 2 tasks — T1 → `deep`, T2 → `quick`
- **Wave 2**: 2 tasks — T3 → `deep`, T4 → `quick`
- **Wave 3**: 1 task — T5 → `quick`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs


- [ ] 1. Add `init_display_pipeline()` to BSP lcd.rs

  **What to do**:
  - Add a new public function `init_display_pipeline()` to `stm32f469i-disc/src/lcd.rs` that encapsulates the entire SDRAM→LCD-reset→framebuffer→DSI/LTDC→layer-config pipeline
  - Function signature:
    ```rust
    pub fn init_display_pipeline(
        fmc: pac::FMC,
        dsi: pac::DSI,
        ltdc: pac::LTDC,
        dma2d: pac::DMA2D,
        gpioc: stm32f4xx_hal::gpio::gpioc::Parts,
        gpiod: stm32f4xx_hal::gpio::gpiod::Parts,
        gpioe: stm32f4xx_hal::gpio::gpioe::Parts,
        gpiof: stm32f4xx_hal::gpio::gpiof::Parts,
        gpiog: stm32f4xx_hal::gpio::gpiog::Parts,
        gpioh: stm32f4xx_hal::gpio::gpioh::Parts,
        gpioi: stm32f4xx_hal::gpio::gpioi::Parts,
        rcc: &mut Rcc,
        delay: &mut SysDelay,
    ) -> (LtdcFramebuffer<u16>, SdramRemainders)
    ```
  - Implementation steps (move from VLS f469.rs lines 109-152):
    1. Call `sdram::split_sdram_pins()` to get SDRAM pins + remainders + PH7
    2. LCD reset via PH7: set low → delay 20ms → set high → delay 10ms
    3. Initialize SDRAM via `sdram::Sdram::new()`
    4. Allocate framebuffer: `sdram.subslice_mut(0, FB_SIZE)` using BSP's `FB_SIZE` constant
    5. Create `LtdcFramebuffer::new()`, clear to black, extract inner buffer
    6. Call existing `init_display_full()` for DSI+LTDC+panel detection
    7. Configure Layer::L1 with buffer and RGB565
    8. Enable layer, reload, get mutable buffer reference
    9. Return `(LtdcFramebuffer::new(buffer, WIDTH, HEIGHT), remainders)`
  - Place the function after the existing `init_display_full()` function (near end of lcd.rs)
  - Use `WIDTH`, `HEIGHT`, `FB_SIZE` constants already defined in lcd.rs (lines 43-47)
  - Add necessary imports: `use crate::sdram::{self, SdramRemainders};` and timer/delay types

  **Must NOT do**:
  - Do NOT modify the existing `init_display_full()` function signature or behavior
  - Do NOT return a VLS `Display` type — return raw `LtdcFramebuffer<u16>`
  - Do NOT add a `Display` type to the BSP — that's an app-level concern
  - Do NOT change the LCD reset timing (must remain exactly 20ms low, 10ms high)
  - Do NOT change pixel format — always RGB565

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Moving 40+ lines of embedded hardware init code between repos requires careful type matching and understanding of the SDRAM/DSI/LTDC pipeline
  - **Skills**: `[]`
    - No special skills needed — pure Rust embedded code movement

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 2)
  - **Blocks**: Task 3 (VLS integration), Task 4 (commit)
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References** (code to move FROM):
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs:93-155` — The entire `init_display()` function. Lines 109-152 contain the board-specific logic to extract and move. Lines 153-155 (Display wrapper) stay in VLS.

  **Pattern References** (code to integrate WITH):
  - `stm32f469i-disc/src/lcd.rs:43-47` — `WIDTH`, `HEIGHT`, `FB_SIZE` constants — use these instead of function parameters for dimensions
  - `stm32f469i-disc/src/lcd.rs` (full file) — The existing `init_display_full()` function. New function will call it internally at step 6.
  - `stm32f469i-disc/src/sdram.rs` — `split_sdram_pins()`, `Sdram::new()`, `SdramRemainders` — all already public, just need to call them

  **API/Type References**:
  - `stm32f4xx-hal/src/ltdc.rs` — `LtdcFramebuffer<u16>`, `Layer`, `PixelFormat`, `DisplayController` types
  - `stm32f4xx-hal/src/rcc/mod.rs` — `Rcc` type (passed as `&mut Rcc`)
  - `stm32f4xx-hal/src/timer/mod.rs` — `SysDelay` type (passed as `&mut SysDelay`)
  - `embedded_hal_02::blocking::delay::DelayMs` — Used for LCD reset timing

  **WHY Each Reference Matters**:
  - VLS f469.rs:93-155 is the SOURCE code to move — extract lines 109-152 as the function body
  - BSP lcd.rs constants (WIDTH/HEIGHT/FB_SIZE) replace VLS's local SCREEN_WIDTH/SCREEN_HEIGHT in framebuffer sizing
  - BSP sdram.rs functions are already called from VLS — same calls, just moved into BSP
  - HAL ltdc.rs defines the return type `LtdcFramebuffer<u16>` — must match exactly

  **Acceptance Criteria**:
  - [ ] Function `pub fn init_display_pipeline(...)` exists in `stm32f469i-disc/src/lcd.rs`
  - [ ] Function returns `(LtdcFramebuffer<u16>, SdramRemainders)`
  - [ ] Function uses `FB_SIZE` constant (not hardcoded dimensions)
  - [ ] LCD reset timing: 20ms low, 10ms high (grep for `delay_ms` calls)
  - [ ] `cargo build --release` in BSP repo succeeds (exit 0)

  **QA Scenarios:**

  ```
  Scenario: BSP compiles with new function
    Tool: Bash
    Preconditions: BSP repo at /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Run `cargo build --release` in BSP directory
      2. Verify exit code is 0
      3. Grep lcd.rs for `pub fn init_display_pipeline` — must find exactly 1 match
      4. Grep lcd.rs for `-> (LtdcFramebuffer<u16>, SdramRemainders)` — must find exactly 1 match
      5. Grep lcd.rs for `delay_ms(delay, 20u32)` and `delay_ms(delay, 10u32)` — must find both
    Expected Result: Build succeeds, function signature correct, reset timing preserved
    Failure Indicators: Build error, missing function, wrong return type, missing delay calls
    Evidence: .sisyphus/evidence/task-1-bsp-build.txt

  Scenario: New function does not break existing API
    Tool: Bash
    Preconditions: BSP repo clean
    Steps:
      1. Grep lcd.rs for `pub fn init_display_full` — must still exist (exactly 1 match)
      2. Grep lcd.rs for `pub const WIDTH` — must still exist
      3. Grep lcd.rs for `pub const HEIGHT` — must still exist
      4. Grep lcd.rs for `pub const FB_SIZE` — must still exist
    Expected Result: All existing public API intact
    Failure Indicators: Any existing public item missing or renamed
    Evidence: .sisyphus/evidence/task-1-api-preserved.txt
  ```

  **Evidence to Capture:**
  - [ ] task-1-bsp-build.txt — cargo build output
  - [ ] task-1-api-preserved.txt — grep results for existing API

  **Commit**: YES (groups with Task 2)
  - Message: `feat(bsp): add init_display_pipeline() and HSE_FREQ constant`
  - Files: `src/lcd.rs`, `src/lib.rs`
  - Pre-commit: `cargo build --release`

- [ ] 2. Add `HSE_FREQ` constant to BSP lib.rs

  **What to do**:
  - Add a public constant to `stm32f469i-disc/src/lib.rs` for the HSE crystal frequency:
    ```rust
    /// HSE crystal frequency on the STM32F469I-DISCO board (8 MHz).
    pub const HSE_FREQ_MHZ: u32 = 8;
    ```
  - Place it after the module declarations (after line 16, `pub mod usb;`)
  - Use `u32` with MHz value for simplicity — VLS will use it as `stm32f469i_disc::HSE_FREQ_MHZ.MHz()` with the HAL's `.MHz()` extension trait
  - NOTE: Using `u32` instead of `Hertz` avoids pulling `stm32f4xx_hal::time::Hertz` into the BSP's top-level namespace and keeps the constant simple

  **Must NOT do**:
  - Do NOT add a `default_clock_config()` function — user explicitly excluded this
  - Do NOT add `RECOMMENDED_SYSCLK` or any other clock-related constants beyond HSE
  - Do NOT change any existing code in lib.rs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single constant addition to a 16-line file — trivial change
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Task 3 (VLS uses this constant), Task 4 (commit)
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `stm32f469i-disc/src/lib.rs:1-16` — Full file. Add constant after line 16.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs:67` — Current hardcoded `8.MHz()` in `make_clocks()` — this is the value being extracted

  **External References**:
  - [32F469IDISCOVERY User Manual (UM1932)](https://www.st.com/resource/en/user_manual/um1932-discovery-kit-with-stm32f469ni-mcu-stmicroelectronics.pdf) — Section 6.3 confirms 8MHz HSE crystal (X3)
  - `rp-pico` BSP convention: defines `pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;` in lib.rs — same pattern

  **WHY Each Reference Matters**:
  - BSP lib.rs is where to add the constant — need to see current structure
  - VLS f469.rs:67 shows the current inline `8.MHz()` that Task 3 will replace with this constant
  - rp-pico pattern confirms this is the idiomatic approach

  **Acceptance Criteria**:
  - [ ] `pub const HSE_FREQ_MHZ: u32 = 8;` exists in `stm32f469i-disc/src/lib.rs`
  - [ ] Constant has a doc comment mentioning 8 MHz and STM32F469I-DISCO
  - [ ] `cargo build --release` in BSP repo succeeds

  **QA Scenarios:**

  ```
  Scenario: HSE_FREQ_MHZ constant is exported
    Tool: Bash
    Preconditions: BSP repo at /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc
    Steps:
      1. Grep lib.rs for `pub const HSE_FREQ_MHZ` — must find exactly 1 match
      2. Grep lib.rs for `= 8;` on the same line — verify value is 8
      3. Run `cargo build --release` — must exit 0
    Expected Result: Constant present with value 8, BSP compiles
    Failure Indicators: Missing constant, wrong value, build failure
    Evidence: .sisyphus/evidence/task-2-hse-const.txt

  Scenario: No existing code changed
    Tool: Bash
    Preconditions: BSP repo
    Steps:
      1. Run `git diff src/lib.rs` in BSP repo
      2. Verify diff shows ONLY additions (no deletions, no modifications to existing lines)
    Expected Result: Only new lines added
    Failure Indicators: Any existing line modified or deleted
    Evidence: .sisyphus/evidence/task-2-diff.txt
  ```

  **Evidence to Capture:**
  - [ ] task-2-hse-const.txt — grep + build output
  - [ ] task-2-diff.txt — git diff showing only additions

  **Commit**: YES (groups with Task 1)
  - Message: `feat(bsp): add init_display_pipeline() and HSE_FREQ constant`
  - Files: `src/lib.rs` (combined with Task 1's lcd.rs commit)
  - Pre-commit: `cargo build --release`

---

- [ ] 3. Simplify VLS f469.rs to use BSP display pipeline and import constants

  **What to do**:
  - Modify `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` to use the new BSP functions:
  - **Replace screen constants** (lines 33-34):
    - Remove `pub const SCREEN_WIDTH: u16 = 480;` and `pub const SCREEN_HEIGHT: u16 = 800;`
    - Add: `pub use stm32f469i_disc::lcd::WIDTH as SCREEN_WIDTH;` and `pub use stm32f469i_disc::lcd::HEIGHT as SCREEN_HEIGHT;`
    - This preserves the `SCREEN_WIDTH`/`SCREEN_HEIGHT` names that `mod.rs` and other code already use
  - **Update `make_clocks()`** (line 67):
    - Replace `Config::hse(8.MHz())` with `Config::hse(stm32f469i_disc::HSE_FREQ_MHZ.MHz())`
    - This uses the BSP constant instead of an inline magic number
  - **Collapse `init_display()`** (lines 93-156) from 70 lines to ~10 lines:
    - Keep the same function signature: `pub fn init_display(...) -> (Display, SdramRemainders)`
    - Replace body with:
      1. Call `stm32f469i_disc::lcd::init_display_pipeline(fmc, dsi, ltdc, dma2d, gpioc..gpioi, rcc, delay)` — gets `(LtdcFramebuffer<u16>, SdramRemainders)`
      2. Wrap the framebuffer in `Display { inner: fb }`
      3. Return `(disp, remainders)`
    - Remove the `info!("setup display (F469 DSI)")` log line (BSP function will log internally if needed)
  - **Clean up imports**: Remove any imports that become unused after the collapse (e.g., `Rgb565`, `embedded_hal_02::blocking::delay::DelayMs` if no longer used directly)
  - **Do NOT remove these imports** that are still used by other code: `LtdcFramebuffer`, `Layer`, `PixelFormat` (may still be needed by mod.rs references)

  **Must NOT do**:
  - Do NOT change the `init_display()` function signature — same parameters, same return type `(Display, SdramRemainders)`
  - Do NOT touch f412.rs, f413.rs, or mod.rs — ONLY f469.rs
  - Do NOT change any UI layout constants (VCENTER_PIX, BUTTON_*, CHOICE_*) — they derive from SCREEN_WIDTH/HEIGHT which remain the same values
  - Do NOT change `touch_to_grid()` or `transform_touch_coords()`
  - Do NOT change `make_clocks()` behavior — only replace the literal `8` with the BSP constant

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires careful refactoring of a 70-line function while preserving the exact same external behavior and API surface, across repository boundaries
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (sequential, after Tasks 1 & 2)
  - **Blocks**: Task 4 (commit)
  - **Blocked By**: Task 1 (init_display_pipeline must exist), Task 2 (HSE_FREQ_MHZ must exist)

  **References**:

  **Pattern References** (code to modify):
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs:33-34` — Local SCREEN_WIDTH/SCREEN_HEIGHT constants to replace with BSP imports
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs:66-70` — `make_clocks()` function with inline `8.MHz()` to replace
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs:93-156` — Full `init_display()` function to collapse

  **Pattern References** (BSP API to call):
  - `stm32f469i-disc/src/lcd.rs` — New `init_display_pipeline()` function (created in Task 1) — call this to replace lines 109-152
  - `stm32f469i-disc/src/lib.rs` — `HSE_FREQ_MHZ` constant (created in Task 2) — use in make_clocks()
  - `stm32f469i-disc/src/lcd.rs:43-44` — `pub const WIDTH: u16 = 480` and `pub const HEIGHT: u16 = 800` — import as SCREEN_WIDTH/SCREEN_HEIGHT

  **Context References** (to understand the caller):
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:494-534` — F469 cfg block that calls `board::init_display()` — this code must NOT change, so the signature must stay identical
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:1-30` — Imports at top of mod.rs, uses `board::SCREEN_WIDTH` etc. — re-export must preserve these names

  **WHY Each Reference Matters**:
  - f469.rs:33-34 are the constants being replaced — must maintain same names via `pub use ... as`
  - f469.rs:66-70 is `make_clocks()` — only the `8.MHz()` literal changes, rest stays identical
  - f469.rs:93-156 is the big win — 70 lines to ~10 lines
  - mod.rs:494-534 is the CALLER — proves the function signature must remain unchanged
  - BSP lcd.rs and lib.rs provide the new APIs — must match exactly what Task 1 & 2 created

  **Acceptance Criteria**:
  - [ ] No `pub const SCREEN_WIDTH` or `pub const SCREEN_HEIGHT` definition in f469.rs (replaced by `pub use`)
  - [ ] `pub use stm32f469i_disc::lcd::WIDTH as SCREEN_WIDTH` exists in f469.rs
  - [ ] `pub use stm32f469i_disc::lcd::HEIGHT as SCREEN_HEIGHT` exists in f469.rs
  - [ ] `make_clocks()` uses `stm32f469i_disc::HSE_FREQ_MHZ.MHz()` instead of `8.MHz()`
  - [ ] `init_display()` function body is ≤15 lines (count lines between `{` and `}`)
  - [ ] `init_display()` function signature unchanged: same parameters, returns `(Display, SdramRemainders)`
  - [ ] `cargo build --features stm32f469 --release` in VLS repo succeeds
  - [ ] `cargo build --features stm32f412 --release` in VLS repo succeeds (no regression)
  - [ ] `cargo build --features stm32f413 --release` in VLS repo succeeds (no regression)

  **QA Scenarios:**

  ```
  Scenario: VLS compiles for all 3 board features
    Tool: Bash
    Preconditions: VLS repo at /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32
    Steps:
      1. Run `cargo build --features stm32f469 --release` — must exit 0
      2. Run `cargo build --features stm32f412 --release` — must exit 0
      3. Run `cargo build --features stm32f413 --release` — must exit 0
    Expected Result: All 3 builds succeed with exit code 0
    Failure Indicators: Any build fails, type mismatch errors, unresolved imports
    Evidence: .sisyphus/evidence/task-3-vls-builds.txt

  Scenario: Screen constants are imported not defined
    Tool: Bash/Grep
    Preconditions: VLS repo
    Steps:
      1. Grep f469.rs for `pub const SCREEN_WIDTH` — must find 0 matches
      2. Grep f469.rs for `pub const SCREEN_HEIGHT` — must find 0 matches
      3. Grep f469.rs for `pub use stm32f469i_disc::lcd::WIDTH as SCREEN_WIDTH` — must find 1 match
      4. Grep f469.rs for `pub use stm32f469i_disc::lcd::HEIGHT as SCREEN_HEIGHT` — must find 1 match
    Expected Result: Constants imported from BSP, not locally defined
    Failure Indicators: Local const definitions still present, or BSP imports missing
    Evidence: .sisyphus/evidence/task-3-constants-check.txt

  Scenario: init_display() is collapsed
    Tool: Bash/Grep
    Preconditions: VLS repo
    Steps:
      1. Count lines in init_display() function body (between opening `{` and closing `}`)
      2. Verify ≤15 lines
      3. Grep f469.rs for `init_display_pipeline` — must find ≥1 match (the BSP call)
    Expected Result: Function collapsed to ≤15 lines, calls BSP pipeline
    Failure Indicators: Function still >15 lines, or no call to BSP function
    Evidence: .sisyphus/evidence/task-3-function-size.txt

  Scenario: HSE constant used in make_clocks
    Tool: Grep
    Preconditions: VLS repo
    Steps:
      1. Grep f469.rs for `8.MHz()` — must find 0 matches
      2. Grep f469.rs for `HSE_FREQ_MHZ` — must find ≥1 match
    Expected Result: No hardcoded 8MHz, uses BSP constant
    Failure Indicators: Hardcoded value still present
    Evidence: .sisyphus/evidence/task-3-hse-usage.txt

  Scenario: F412 and F413 files completely untouched
    Tool: Bash
    Preconditions: VLS repo
    Steps:
      1. Run `git diff src/device/f412.rs` — must be empty
      2. Run `git diff src/device/f413.rs` — must be empty
      3. Run `git diff src/device/mod.rs` — must be empty
    Expected Result: Zero changes to f412.rs, f413.rs, and mod.rs
    Failure Indicators: Any diff output for these files
    Evidence: .sisyphus/evidence/task-3-no-regression.txt
  ```

  **Evidence to Capture:**
  - [ ] task-3-vls-builds.txt — cargo build output for all 3 features
  - [ ] task-3-constants-check.txt — grep results
  - [ ] task-3-function-size.txt — line count of init_display()
  - [ ] task-3-hse-usage.txt — grep for hardcoded vs BSP constant
  - [ ] task-3-no-regression.txt — git diff showing F412/F413/mod.rs untouched

  **Commit**: YES
  - Message: `refactor(f469): use BSP display pipeline and import screen constants`
  - Files: `src/device/f469.rs`
  - Pre-commit: `cargo build --features stm32f469 --release && cargo build --features stm32f412 --release && cargo build --features stm32f413 --release`

- [ ] 4. Commit changes to both repositories

  **What to do**:
  - **BSP repo** (`/Users/macbook/src/stm32f4xx-hal/stm32f469i-disc/`):
    - `git add src/lcd.rs src/lib.rs`
    - `git commit -m "feat(bsp): add init_display_pipeline() and HSE_FREQ constant"`
  - **VLS repo** (`/Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32/`):
    - `git add src/device/f469.rs`
    - `git commit -m "refactor(f469): use BSP display pipeline and import screen constants"`
  - Verify both commits succeed (exit code 0)

  **Must NOT do**:
  - Do NOT push to any remote
  - Do NOT commit any files other than those listed
  - Do NOT amend any previous commits
  - Do NOT touch f412.rs, f413.rs, or mod.rs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Two simple git commit commands
  - **Skills**: `['git-master']`
    - `git-master`: Ensures proper commit hygiene

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (after Task 3)
  - **Blocks**: Task 5 (verification)
  - **Blocked By**: Task 3 (all code changes must be complete before commit)

  **References**:
  - Task 1 output: BSP lcd.rs and lib.rs changes
  - Task 3 output: VLS f469.rs changes

  **Acceptance Criteria**:
  - [ ] BSP repo has a new commit with message matching `feat(bsp): add init_display_pipeline*`
  - [ ] VLS repo has a new commit with message matching `refactor(f469): use BSP display pipeline*`
  - [ ] `git status` in both repos shows clean working tree

  **QA Scenarios:**

  ```
  Scenario: Both commits created successfully
    Tool: Bash
    Preconditions: Tasks 1-3 complete
    Steps:
      1. In BSP repo: `git log -1 --oneline` — must contain "init_display_pipeline"
      2. In VLS repo: `git log -1 --oneline` — must contain "BSP display pipeline"
      3. In BSP repo: `git status` — must show clean working tree
      4. In VLS repo: `git status` — must show clean working tree
    Expected Result: Both commits present, both repos clean
    Failure Indicators: Missing commit, dirty working tree, wrong commit message
    Evidence: .sisyphus/evidence/task-4-commits.txt
  ```

  **Evidence to Capture:**
  - [ ] task-4-commits.txt — git log and git status output for both repos

  **Commit**: N/A (this task IS the commit)

- [ ] 5. Cross-compile verification for all board features

  **What to do**:
  - Clean build verification from scratch for all supported board features:
    1. BSP: `cargo build --release` in `stm32f469i-disc/`
    2. VLS F469: `cargo build --features stm32f469 --release` in `vls-signer-stm32/`
    3. VLS F412: `cargo build --features stm32f412 --release` in `vls-signer-stm32/`
    4. VLS F413: `cargo build --features stm32f413 --release` in `vls-signer-stm32/`
  - All 4 builds must succeed (exit code 0)
  - This is the final verification that nothing was broken

  **Must NOT do**:
  - Do NOT make any code changes — this is verification only
  - Do NOT skip any of the 4 builds

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running 4 build commands and checking exit codes
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (after Task 4)
  - **Blocks**: F1-F4 (final verification wave)
  - **Blocked By**: Task 4 (commits must be made before final verification)

  **References**:
  - `.sisyphus/plans/bsp-architecture-refinement.md` — Success Criteria section for exact commands
  - `stm32f469i-disc/Cargo.toml` — BSP crate configuration
  - `validating-lightning-signer/vls-signer-stm32/Cargo.toml` — VLS crate with feature flags

  **Acceptance Criteria**:
  - [ ] `cargo build --release` in BSP → exit 0
  - [ ] `cargo build --features stm32f469 --release` in VLS → exit 0
  - [ ] `cargo build --features stm32f412 --release` in VLS → exit 0
  - [ ] `cargo build --features stm32f413 --release` in VLS → exit 0

  **QA Scenarios:**

  ```
  Scenario: All 4 builds succeed
    Tool: Bash
    Preconditions: Tasks 1-4 complete, commits made
    Steps:
      1. `cargo build --release` in /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc — exit 0
      2. `cargo build --features stm32f469 --release` in /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32 — exit 0
      3. `cargo build --features stm32f412 --release` in /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32 — exit 0
      4. `cargo build --features stm32f413 --release` in /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32 — exit 0
    Expected Result: All 4 builds complete with exit code 0, no warnings about missing items
    Failure Indicators: Any non-zero exit code, unresolved symbol errors, type mismatches
    Evidence: .sisyphus/evidence/task-5-cross-compile.txt

  Scenario: No unexpected file changes
    Tool: Bash
    Preconditions: After all builds
    Steps:
      1. `git status` in BSP repo — must show clean working tree
      2. `git status` in VLS repo — must show clean working tree
    Expected Result: Builds did not produce any uncommitted changes
    Failure Indicators: Generated files, modified sources, new untracked files
    Evidence: .sisyphus/evidence/task-5-clean-state.txt
  ```

  **Evidence to Capture:**
  - [ ] task-5-cross-compile.txt — all 4 build outputs with exit codes
  - [ ] task-5-clean-state.txt — git status for both repos

  **Commit**: NO (verification only)

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, grep for function/constant). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo build --release` in BSP. Run `cargo build --features stm32f469 --release` in VLS. Review all changed files for: `as any`/`@ts-ignore` equivalents, empty error handling, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Verify BSP compiles independently. Verify VLS compiles with stm32f469. Verify VLS compiles with stm32f412. Verify VLS compiles with stm32f413. Check that `init_display_pipeline` function signature matches plan. Check that VLS f469.rs init_display is ≤15 lines. Save evidence.
  Output: `Builds [N/N pass] | Signatures [N/N] | Line Count [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Commit 1** (BSP repo): `feat(bsp): add init_display_pipeline() and HSE_FREQ constant` — lcd.rs, lib.rs
- **Commit 2** (VLS repo): `refactor(f469): use BSP display pipeline and import screen constants` — f469.rs

---

## Success Criteria

### Verification Commands
```bash
# BSP compiles
cd /Users/macbook/src/stm32f4xx-hal/stm32f469i-disc && cargo build --release
# Expected: exit 0

# VLS compiles for F469
cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f469 --release
# Expected: exit 0

# VLS compiles for F412 (no regression)
cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f412 --release
# Expected: exit 0

# VLS compiles for F413 (no regression)
cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/vls-signer-stm32 && cargo build --features stm32f413 --release
# Expected: exit 0
```

### Final Checklist
- [ ] BSP exports `init_display_pipeline()` in lcd.rs
- [ ] BSP exports `HSE_FREQ` in lib.rs
- [ ] VLS f469.rs imports SCREEN_WIDTH/SCREEN_HEIGHT from BSP
- [ ] VLS f469.rs init_display() is ≤15 lines
- [ ] F412/F413 files unchanged (git diff shows 0 changes)
- [ ] All 3 board features compile
