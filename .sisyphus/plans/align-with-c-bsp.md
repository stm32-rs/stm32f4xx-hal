# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# Align DSI/LTDC with C BSP Known-Working Pattern

## TL;DR

> **Quick Summary**: Fix DSI register hardcodes (`NullPacketSize`, `NumberOfChunks`) by making them configurable in `DsiConfig`, switch the LCD test example from `DisplayController::new()` (which auto-computes wrong PLLSAI) to `new_dsi()` + manual PLLSAI setup, and restore NT35510 ST spec timings. All changes align with the STM32F469I-DISCO C BSP reference.
> 
> **Deliverables**:
> - `DsiConfig` struct gains `null_packet_size: u16` and `number_of_chunks: u16` fields
> - `DsiHost::init()` uses configurable fields instead of hardcoded values
> - `f469disco-lcd-test.rs` uses `DisplayController::new_dsi()` + explicit PLLSAI setup
> - NT35510 timings restored to ST spec values (v_sync=120, v_back_porch=150, v_front_porch=150)
> - All 6 f469disco examples compile
> - Hardware verified on remote board
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 2 waves + final verification
> **Critical Path**: Task 1 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7

---

## Context

### Original Request
User asked to study the `f469-disco-revc-board-support.md` (C BSP reference) and make the Rust HAL approach align with the known-working C BSP patterns, timings, and settings — while maintaining the spirit of the HAL crate.

### Interview Summary
**Key Discussions**:
- **Root cause identified**: `DisplayController::new()` auto-computes PLLSAI from display timings. With NT35510 ST spec timings (total_height=1219), it computes 40.125 MHz instead of the correct 27.429 MHz. C BSP uses a FIXED PLLSAI (PLLN=384, PLLR=7, DIVR=2) for both display panels.
- **HAL enhancement vs example-only**: User chose HAL enhancement — changes go into `src/dsi.rs` and `src/ltdc.rs`, not just examples.
- **DSI registers**: User chose to fix both `NullPacketSize` (0→0xFFF) and `NumberOfChunks` (1→0) to match C BSP.
- **Verification**: User chose to flash and verify on the remote board (ubuntu@192.168.13.246).

**Research Findings**:
- `DisplayController::new_dsi()` already exists at `src/ltdc.rs:368` — it skips PLLSAI auto-calc and GPIO pin setup, which is exactly what DSI examples need.
- The shared `examples/f469disco/display_init.rs` module already uses `new_dsi()` (line 190). The 5 examples using it work without explicit PLLSAI config.
- C BSP uses identical `numc=0, npsize=0xFFF` for BOTH OTM8009A and NT35510 panels.

### Metis Review
**Identified Gaps** (addressed):
- **Breaking change**: Adding fields to `DsiConfig` breaks existing code constructing it. Addressed by using plain `u16` fields — callers must add them, but it's a clear compile error with an easy fix.
- **Regression risk in other examples**: The DSI register fix propagates through `DsiHost::init()` to all 6 examples. `display_init.rs` module at line 109 must also add the new `DsiConfig` fields.
- **Phasing**: DSI register fix should be independently testable from PLLSAI/timing changes. Plan structures tasks so they can be committed separately.
- **PLLSAI for `display_init.rs` examples**: They already work without explicit PLLSAI via `new_dsi()`. No PLLSAI changes needed for those examples.
- **Rollback criteria**: If ST spec timings still fail even with fixed PLLSAI, revert timing task independently; DSI register fix stands on its own.

---

## Work Objectives

### Core Objective
Align the Rust HAL's DSI configuration with the C BSP reference by making hardcoded DSI packet parameters configurable and fixing the PLLSAI clock setup for DSI-connected displays.

### Concrete Deliverables
- `src/dsi.rs`: `DsiConfig` struct with `null_packet_size` and `number_of_chunks` fields; `DsiHost::init()` uses them
- `examples/f469disco-lcd-test.rs`: Uses `new_dsi()` + manual PLLSAI; NT35510 timings restored to ST spec
- `examples/f469disco/display_init.rs`: Updated `DsiConfig` construction with new fields
- All 6 f469disco examples compile cleanly
- Verified working on remote STM32F469I-DISCO board

### Definition of Done
- [ ] `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"` passes
- [ ] `cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"` passes
- [ ] Remote board shows correct display output (verified via defmt logs)

### Must Have
- `null_packet_size` and `number_of_chunks` as configurable `u16` fields in `DsiConfig`
- `DsiHost::init()` uses the config fields instead of hardcoded `1` and `0`
- `f469disco-lcd-test.rs` uses `DisplayController::new_dsi()` instead of `DisplayController::new()`
- Explicit PLLSAI register configuration in `f469disco-lcd-test.rs` (PLLN=384, PLLR=7, DIVR=2)
- NT35510 timings match ST spec: v_sync=120, v_back_porch=150, v_front_porch=150

### Must NOT Have (Guardrails)
- DO NOT remove or modify `DisplayController::new()` — it is used by non-DSI examples (e.g., `ltdc-screen`)
- DO NOT change `DISPLAY_CONFIG` timings in `examples/f469disco/display_init.rs` — those OTM8009A-compatible tight timings are shared by 5 working examples
- DO NOT modify `examples/f469disco/nt35510.rs` — the NT35510 driver is already working
- DO NOT use `Option<u16>` or builder pattern for the new `DsiConfig` fields — plain `u16` keeps the struct simple, `const`-constructible, and `Copy`
- DO NOT add PLLSAI configuration to `display_init.rs` examples — they work without it via `new_dsi()`
- DO NOT combine DSI register fix with timing restoration in the same commit — they must be independently revertable
- DO NOT add excessive documentation or abstractions — this is an embedded HAL, keep it minimal

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (embedded target, no unit test harness)
- **Automated tests**: None (cross-compilation target `thumbv7em-none-eabihf`)
- **Framework**: N/A
- **Verification method**: Compile checks + on-hardware flash + defmt log inspection

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Compile verification**: `cargo check` / `cargo build --release` for all affected examples
- **Hardware verification**: Flash via `probe-rs` on remote board, capture defmt logs via `probe-rs attach`
- **Regression check**: All 6 f469disco examples must still compile

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — HAL changes, independent):
├── Task 1: Add null_packet_size/number_of_chunks to DsiConfig [quick]
├── Task 2: (reserved for Wave 2 dependency resolution)

Wave 2 (After Wave 1 — example changes, depend on HAL):
├── Task 3: Update display_init.rs DsiConfig construction [quick]
├── Task 4: Switch f469disco-lcd-test to new_dsi() + PLLSAI setup [deep]
├── Task 5: Restore NT35510 ST spec timings [quick]

Wave 3 (After Wave 2 — compile verification):
├── Task 6: Compile-check ALL f469disco examples [quick]

Wave 4 (After Wave 3 — hardware verification):
├── Task 7: Flash and verify on remote board [unspecified-high]

Wave FINAL (After ALL tasks — independent review):
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Real manual QA — reflash and verify [unspecified-high]
├── Task F4: Scope fidelity check [deep]

Critical Path: Task 1 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7 → F1-F4
Parallel Speedup: Tasks 3, 4, 5 can run in parallel after Task 1
Max Concurrent: 3 (Wave 2)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 3, 4, 5 | 1 |
| 3 | 1 | 6 | 2 |
| 4 | 1 | 5, 6 | 2 |
| 5 | 4 | 6 | 2 |
| 6 | 3, 4, 5 | 7 | 3 |
| 7 | 6 | F1-F4 | 4 |
| F1-F4 | 7 | — | FINAL |

### Agent Dispatch Summary

- **Wave 1**: 1 task — T1 → `quick`
- **Wave 2**: 3 tasks — T3 → `quick`, T4 → `deep`, T5 → `quick`
- **Wave 3**: 1 task — T6 → `quick`
- **Wave 4**: 1 task — T7 → `unspecified-high`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> Implementation + Test = ONE Task. Never separate.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.
> **A task WITHOUT QA Scenarios is INCOMPLETE. No exceptions.**

- [ ] 1. Add `null_packet_size` and `number_of_chunks` fields to `DsiConfig`; use them in `DsiHost::init()`

  **What to do**:
  - Add two new fields to the `DsiConfig` struct at `src/dsi.rs:143`:
    ```rust
    pub null_packet_size: u16,
    pub number_of_chunks: u16,
    ```
  - Replace the hardcoded value at `src/dsi.rs:318` (`dsi.vccr().modify(|_, w| unsafe { w.numc().bits(1) })`) with:
    ```rust
    dsi.vccr().modify(|_, w| unsafe { w.numc().bits(dsi_config.number_of_chunks) });
    ```
  - Replace the hardcoded value at `src/dsi.rs:321` (`dsi.vnpcr().modify(|_, w| unsafe { w.npsize().bits(0) })`) with:
    ```rust
    dsi.vnpcr().modify(|_, w| unsafe { w.npsize().bits(dsi_config.null_packet_size) });
    ```
  - The values used in both examples should match C BSP: `null_packet_size: 0xFFF, number_of_chunks: 0`

  **Must NOT do**:
  - DO NOT use `Option<u16>` — plain `u16` keeps struct `Copy` and `const`-constructible
  - DO NOT add a `Default` impl — let callers explicitly choose values
  - DO NOT change any other fields in `DsiConfig`
  - DO NOT touch the `DsiHost::init()` function signature

  **Recommended Agent Profile**:
  > Select category + skills based on task domain.
  - **Category**: `quick`
    - Reason: Straightforward struct field addition + two line replacements in a single file
  - **Skills**: `[]`
  - **Skills Evaluated but Omitted**:
    - None relevant — this is a simple Rust struct/register modification

  **Parallelization**:
  - **Can Run In Parallel**: NO (sole Wave 1 task)
  - **Parallel Group**: Wave 1 (alone)
  - **Blocks**: Tasks 3, 4, 5 (all example updates need the new `DsiConfig` fields)
  - **Blocked By**: None (can start immediately)

  **References** (CRITICAL — Be Exhaustive):

  **Pattern References** (existing code to follow):
  - `src/dsi.rs:143-154` — Current `DsiConfig` struct definition; add new fields after `vlp_size: u8` at line 153
  - `src/dsi.rs:318` — Current hardcoded `numc().bits(1)` — replace `1` with `dsi_config.number_of_chunks`
  - `src/dsi.rs:321` — Current hardcoded `npsize().bits(0)` — replace `0` with `dsi_config.null_packet_size`
  - `src/dsi.rs:312-321` — Context: the surrounding TODO comments explain what these registers do

  **API/Type References** (contracts to implement against):
  - `src/dsi.rs:130-139` — `ColorCoding` enum shows the derive pattern used (`cfg_attr(feature = "defmt", derive(defmt::Format))`, `Clone, Copy, Debug, PartialEq, Eq`) — `u16` implements all these automatically
  - `src/dsi.rs:156-163` — `DsiHost::init()` signature showing `dsi_config: DsiConfig` parameter — the config is already passed by value

  **External References**:
  - `f469-disco-revc-board-support.md` — C BSP uses `NullPacketSize = 0xFFF` and `NumberOfChunks = 0` for both display panels

  **WHY Each Reference Matters**:
  - `dsi.rs:143-154`: You need to see the existing field layout to add new fields in the right place with consistent style
  - `dsi.rs:318,321`: These are the exact two lines to modify — the register writes that currently use hardcoded values
  - `dsi.rs:312-321`: The TODO comment at line 312 says "Unhardcode?" — we're finally doing that
  - `dsi.rs:156-163`: Confirms `dsi_config` is passed to `init()` and accessible where the register writes happen

  **Acceptance Criteria**:
  - [ ] `DsiConfig` struct has `pub null_packet_size: u16` field
  - [ ] `DsiConfig` struct has `pub number_of_chunks: u16` field
  - [ ] `dsi.rs:318` no longer has hardcoded `1` — uses `dsi_config.number_of_chunks`
  - [ ] `dsi.rs:321` no longer has hardcoded `0` — uses `dsi_config.null_packet_size`
  - [ ] `DsiConfig` still derives `Clone, Copy, Debug, PartialEq, Eq` and `defmt::Format`

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: DsiConfig compiles with new fields
    Tool: Bash
    Preconditions: src/dsi.rs modified with new fields
    Steps:
      1. Run: grep -n 'null_packet_size' src/dsi.rs
      2. Assert: output shows field declaration in struct AND usage in register write
      3. Run: grep -n 'number_of_chunks' src/dsi.rs
      4. Assert: output shows field declaration in struct AND usage in register write
      5. Run: grep -n 'bits(1)' src/dsi.rs — should NOT match in vccr context
      6. Run: grep -n 'bits(0)' src/dsi.rs — should NOT match in vnpcr context
    Expected Result: New fields exist; old hardcoded values removed from numc/npsize lines
    Failure Indicators: grep still finds hardcoded `bits(1)` for numc or `bits(0)` for npsize
    Evidence: .sisyphus/evidence/task-1-dsiconfig-fields.txt
  ```

  **Commit**: YES (groups with Task 3)
  - Message: `feat(dsi): make NullPacketSize and NumberOfChunks configurable in DsiConfig`
  - Files: `src/dsi.rs`
  - Pre-commit: N/A (compile check deferred to Task 3 which updates callers)

- [ ] 3. Update `display_init.rs` DsiConfig construction with new fields

  **What to do**:
  - Add `null_packet_size: 0xFFF` and `number_of_chunks: 0` to the `DsiConfig` construction in `examples/f469disco/display_init.rs` at line 109-122
  - Add the same fields to the `DsiConfig` construction in `examples/f469disco-lcd-test.rs` at lines 201-214
  - These values match the C BSP reference for both OTM8009A and NT35510 panels

  **Must NOT do**:
  - DO NOT change ANY other fields in either `DsiConfig` construction
  - DO NOT change `DISPLAY_CONFIG` timings in `display_init.rs` (lines 28-42)
  - DO NOT change the DSI PLL config or PHY timers in either file

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Adding two field values to two existing struct constructions — trivial
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 4, 5 — but this is simplest so likely finishes first)
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Task 6 (compile check needs all callers updated)
  - **Blocked By**: Task 1 (new fields must exist in struct)

  **References**:

  **Pattern References**:
  - `examples/f469disco/display_init.rs:109-122` — Current `DsiConfig` construction; add `null_packet_size: 0xFFF, number_of_chunks: 0,` after `vlp_size: 64,` at line 121
  - `examples/f469disco-lcd-test.rs:201-214` — Current `DsiConfig` construction; add same fields after `vlp_size: 64,` at line 213

  **External References**:
  - `f469-disco-revc-board-support.md` — C BSP NullPacketSize=0xFFF, NumberOfChunks=0

  **WHY Each Reference Matters**:
  - `display_init.rs:109-122`: This is one of exactly TWO places in the codebase that construct `DsiConfig`. Miss this and 5 examples won't compile.
  - `f469disco-lcd-test.rs:201-214`: The other construction site. Both must be updated.

  **Acceptance Criteria**:
  - [ ] `display_init.rs` DsiConfig has `null_packet_size: 0xFFF`
  - [ ] `display_init.rs` DsiConfig has `number_of_chunks: 0`
  - [ ] `f469disco-lcd-test.rs` DsiConfig has `null_packet_size: 0xFFF`
  - [ ] `f469disco-lcd-test.rs` DsiConfig has `number_of_chunks: 0`
  - [ ] `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"` passes
  - [ ] `cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"` passes

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Both DsiConfig constructions compile with new fields
    Tool: Bash
    Preconditions: Task 1 complete (struct has new fields)
    Steps:
      1. Run: cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
      2. Assert: exit code 0 (no compile errors)
      3. Run: cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
      4. Assert: exit code 0
    Expected Result: Both examples compile cleanly
    Failure Indicators: Compile error about missing fields or type mismatches
    Evidence: .sisyphus/evidence/task-3-compile-check.txt

  Scenario: display_init.rs DISPLAY_CONFIG timings are untouched
    Tool: Bash
    Preconditions: display_init.rs has been modified
    Steps:
      1. Run: grep -A 14 'pub const DISPLAY_CONFIG' examples/f469disco/display_init.rs
      2. Assert: v_back_porch is 15, v_front_porch is 16, v_sync is 1 (unchanged)
    Expected Result: Only DsiConfig section was modified, not DISPLAY_CONFIG
    Failure Indicators: DISPLAY_CONFIG timing values differ from original
    Evidence: .sisyphus/evidence/task-3-no-timing-regression.txt
  ```

  **Commit**: YES (groups with Task 1)
  - Message: `feat(dsi): make NullPacketSize and NumberOfChunks configurable in DsiConfig`
  - Files: `src/dsi.rs`, `examples/f469disco/display_init.rs`, `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo check --example f469disco-lcd-test --features="stm32f469,defmt" && cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"`

---

- [ ] 4. Switch `f469disco-lcd-test.rs` from `DisplayController::new()` to `new_dsi()` + explicit PLLSAI setup

  **What to do**:
  - Replace the `DisplayController::<u32>::new()` call at `examples/f469disco-lcd-test.rs:246-253` with `DisplayController::<u32>::new_dsi()`
  - The current code is:
    ```rust
    let _display = DisplayController::<u32>::new(
        dp.LTDC,
        dp.DMA2D,
        None,
        PixelFormat::ARGB8888,
        controller.display_config(),
        Some(hse_freq),
    );
    ```
  - Replace with manual PLLSAI configuration followed by `new_dsi()`:
    ```rust
    // Configure PLLSAI for LTDC pixel clock: 384MHz VCO / 7 / 2 = 27.429 MHz
    // Matches C BSP fixed configuration for both NT35510 and OTM8009A panels.
    // We do this manually because DisplayController::new() auto-computes PLLSAI
    // from display timings, which gives wrong results for NT35510's large v_sync/v_porch.
    let rcc_regs = unsafe { &(*stm32f4xx_hal::pac::RCC::ptr()) };
    rcc_regs.pllsaicfgr().write(|w| unsafe {
        w.pllsain().bits(384);
        w.pllsair().bits(7)
    });
    rcc_regs.dckcfgr().modify(|_, w| w.pllsaidivr().set(0b00)); // /2
    rcc_regs.cr().modify(|_, w| w.pllsaion().on());
    while rcc_regs.cr().read().pllsairdy().is_not_ready() {}

    let _display = DisplayController::<u32>::new_dsi(
        dp.LTDC,
        dp.DMA2D,
        PixelFormat::ARGB8888,
        controller.display_config(),
    );
    ```
  - Note: `new_dsi()` takes 4 args (no `pins: Option<LtdcPins>`, no `hse: Option<Hertz>`)
  - The import `use stm32f4xx_hal::pac::RCC` may be needed (check if `pac` is already imported)
  - The `hse_freq` variable is still needed earlier for DSI init, so don't remove it

  **Must NOT do**:
  - DO NOT modify `DisplayController::new()` in `src/ltdc.rs`
  - DO NOT modify `DisplayController::new_dsi()` in `src/ltdc.rs`
  - DO NOT remove the `hse_freq` variable — it's used by DSI host init
  - DO NOT change any code after the `_display` assignment (panel init, drawing, etc.)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires understanding the PLLSAI register layout and unsafe register access patterns in the existing codebase
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 3, but Task 5 depends on this)
  - **Parallel Group**: Wave 2 (with Tasks 3, 5)
  - **Blocks**: Task 5 (timing restoration should follow the switch to `new_dsi()`)
  - **Blocked By**: Task 1 (new `DsiConfig` fields needed)

  **References**:

  **Pattern References**:
  - `examples/f469disco-lcd-test.rs:246-253` — Current `DisplayController::new()` call to replace
  - `examples/f469disco/display_init.rs:189-190` — Working `new_dsi()` usage pattern: `DisplayController::<u16>::new_dsi(ltdc, dma2d, PixelFormat::RGB565, DISPLAY_CONFIG)` — follow this pattern but with `u32` and `ARGB8888`
  - `src/ltdc.rs:368-373` — `new_dsi()` function signature: `pub fn new_dsi(ltdc: LTDC, dma2d: DMA2D, pixel_format: PixelFormat, config: DisplayConfig)`
  - `src/ltdc.rs:300-309` — Existing PLLSAI register setup pattern in `new()` — copy this pattern for manual PLLSAI in the example

  **API/Type References**:
  - `src/ltdc.rs:218-225` — `new()` signature (for comparison: takes `pins`, `hse` — `new_dsi()` does not)

  **External References**:
  - `f469-disco-revc-board-support.md` — C BSP PLLSAI: PLLN=384, PLLR=7, DIVR=2, result=27.429 MHz

  **WHY Each Reference Matters**:
  - `f469disco-lcd-test.rs:246-253`: The exact code being replaced — copy surrounding context to ensure clean edit
  - `display_init.rs:189-190`: Proves `new_dsi()` works in practice — match this calling convention
  - `ltdc.rs:300-309`: Shows the EXACT register access pattern for PLLSAI — copy this unsafe block
  - `ltdc.rs:368-373`: Confirms `new_dsi()` parameter list

  **Acceptance Criteria**:
  - [ ] `f469disco-lcd-test.rs` no longer calls `DisplayController::new()`
  - [ ] `f469disco-lcd-test.rs` calls `DisplayController::new_dsi()` with 4 args
  - [ ] PLLSAI registers are set before `new_dsi()`: PLLN=384, PLLR=7, DIVR=0b00 (/2)
  - [ ] PLLSAI is enabled and waited for (pllsaion + pllsairdy loop)
  - [ ] `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"` passes

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: new_dsi() call compiles and PLLSAI setup is present
    Tool: Bash
    Preconditions: f469disco-lcd-test.rs has been modified
    Steps:
      1. Run: grep -n 'new_dsi' examples/f469disco-lcd-test.rs
      2. Assert: at least one match showing DisplayController::new_dsi() call
      3. Run: grep -n 'DisplayController.*::new(' examples/f469disco-lcd-test.rs
      4. Assert: NO matches (old new() call removed)
      5. Run: grep -n 'pllsain.*384' examples/f469disco-lcd-test.rs
      6. Assert: one match showing PLLN=384
      7. Run: grep -n 'pllsair.*7' examples/f469disco-lcd-test.rs
      8. Assert: one match showing PLLR=7
      9. Run: cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
      10. Assert: exit code 0
    Expected Result: Old new() replaced with new_dsi(); PLLSAI setup present and correct
    Failure Indicators: old new() still present, or PLLSAI values wrong, or compile error
    Evidence: .sisyphus/evidence/task-4-new-dsi-switch.txt
  ```

  **Commit**: YES (groups with Task 5)
  - Message: `fix(example): use new_dsi() with fixed PLLSAI and ST spec NT35510 timings`
  - Files: `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"`

- [ ] 5. Restore NT35510 ST spec timings in `f469disco-lcd-test.rs`

  **What to do**:
  - Change the `NT35510_DISPLAY_CONFIG` constant in `examples/f469disco-lcd-test.rs:105-119` to use ST spec timings:
    ```rust
    pub const NT35510_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
        active_width: WIDTH as _,
        active_height: HEIGHT as _,
        h_back_porch: 34,
        h_front_porch: 34,
        v_back_porch: 150,   // ST spec NT35510 timing
        v_front_porch: 150,  // ST spec NT35510 timing
        h_sync: 2,
        v_sync: 120,         // ST spec NT35510 timing
        frame_rate: 60,
        h_sync_pol: true,
        v_sync_pol: true,
        no_data_enable_pol: false,
        pixel_clock_pol: true,
    };
    ```
  - Update the comments: remove the old "Using OTM8009A-compatible tight timings" comments (lines 101-103) and replace with accurate description
  - The key changes are: `v_back_porch: 15 → 150`, `v_front_porch: 16 → 150`, `v_sync: 1 → 120`

  **Must NOT do**:
  - DO NOT change `OTM8009A_DISPLAY_CONFIG` (lines 125-139) — those timings are correct for OTM8009A
  - DO NOT change `DISPLAY_CONFIG` in `examples/f469disco/display_init.rs` — that's a different module used by other examples
  - DO NOT change `h_back_porch`, `h_front_porch`, `h_sync`, or `frame_rate` — those are the same in both C BSP configs
  - DO NOT change any of the polarity fields

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Changing 3 numeric constants and updating comments — trivial
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO (depends on Task 4 — should be applied to the file after Task 4's changes)
  - **Parallel Group**: Wave 2 (after Task 4)
  - **Blocks**: Task 6 (compile check)
  - **Blocked By**: Task 4 (the `new_dsi()` switch should happen first so the timing change is tested with the correct PLLSAI)

  **References**:

  **Pattern References**:
  - `examples/f469disco-lcd-test.rs:100-119` — Current NT35510 config with OTM8009A tight timings and comments explaining why
  - `examples/f469disco-lcd-test.rs:125-139` — OTM8009A config (DO NOT TOUCH — but useful for comparison)

  **External References**:
  - `f469-disco-revc-board-support.md` — C BSP NT35510 timings: VSA=120, VBP=150, VFP=150, HSA=2, HBP=34, HFP=34

  **WHY Each Reference Matters**:
  - `f469disco-lcd-test.rs:100-119`: The exact code block to modify — 3 values change, comments change
  - `f469-disco-revc-board-support.md`: Authoritative source for the correct NT35510 timing values

  **Acceptance Criteria**:
  - [ ] `NT35510_DISPLAY_CONFIG.v_sync` is `120`
  - [ ] `NT35510_DISPLAY_CONFIG.v_back_porch` is `150`
  - [ ] `NT35510_DISPLAY_CONFIG.v_front_porch` is `150`
  - [ ] Comments no longer say "OTM8009A-compatible tight timings"
  - [ ] `OTM8009A_DISPLAY_CONFIG` is unchanged
  - [ ] `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"` passes

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: NT35510 timings match ST spec
    Tool: Bash
    Preconditions: f469disco-lcd-test.rs modified
    Steps:
      1. Run: grep -A 15 'NT35510_DISPLAY_CONFIG' examples/f469disco-lcd-test.rs | head -20
      2. Assert: v_back_porch is 150, v_front_porch is 150, v_sync is 120
      3. Run: grep -A 15 'OTM8009A_DISPLAY_CONFIG' examples/f469disco-lcd-test.rs | head -20
      4. Assert: v_back_porch is 15, v_front_porch is 16, v_sync is 1 (UNCHANGED)
      5. Run: cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
      6. Assert: exit code 0
    Expected Result: NT35510 has ST spec timings; OTM8009A timings untouched
    Failure Indicators: Wrong timing values or OTM8009A config was modified
    Evidence: .sisyphus/evidence/task-5-nt35510-timings.txt
  ```

  **Commit**: YES (groups with Task 4)
  - Message: `fix(example): use new_dsi() with fixed PLLSAI and ST spec NT35510 timings`
  - Files: `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"`

- [ ] 6. Compile-check ALL f469disco examples (regression gate)

  **What to do**:
  - Run `cargo check` on ALL 6 f469disco examples to verify no regressions:
    ```bash
    cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
    cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo check --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo check --example f469disco-image-slider --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo check --example f469disco-animated-layers --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo check --example f469disco-slideshow --features="stm32f469,stm32-fmc,framebuffer,defmt"
    ```
  - If any fail, investigate and fix the cause (likely a missing `DsiConfig` field)
  - This is a pure verification task — no code changes expected

  **Must NOT do**:
  - DO NOT change source code in this task unless a compile error is found
  - If a fix is needed, it should be applied to the correct task's files

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running 6 compile commands and checking exit codes
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO (must wait for all Wave 2 tasks)
  - **Parallel Group**: Wave 3 (alone)
  - **Blocks**: Task 7 (no point flashing if it doesn't compile)
  - **Blocked By**: Tasks 3, 4, 5

  **References**:
  - `Cargo.toml` — For correct feature flags for each example
  - All 6 examples in `examples/` directory

  **Acceptance Criteria**:
  - [ ] All 6 `cargo check` commands exit with code 0
  - [ ] No warnings related to the changed code (unused imports, etc.)

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All 6 f469disco examples compile
    Tool: Bash
    Preconditions: Tasks 1, 3, 4, 5 complete
    Steps:
      1. Run: cargo check --example f469disco-lcd-test --features="stm32f469,defmt" 2>&1
      2. Assert: exit code 0
      3. Run: cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      4. Assert: exit code 0
      5. Run: cargo check --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      6. Assert: exit code 0
      7. Run: cargo check --example f469disco-image-slider --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      8. Assert: exit code 0
      9. Run: cargo check --example f469disco-animated-layers --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      10. Assert: exit code 0
      11. Run: cargo check --example f469disco-slideshow --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      12. Assert: exit code 0
    Expected Result: All 6 examples compile without errors
    Failure Indicators: Any cargo check exits non-zero
    Evidence: .sisyphus/evidence/task-6-all-examples-compile.txt
  ```

  **Commit**: NO (verification only)

- [ ] 7. Flash and verify on remote board

  **What to do**:
  - Build `f469disco-lcd-test` in release mode:
    ```bash
    cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"
    ```
  - Copy to remote board and flash:
    ```bash
    scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/
    ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test && probe-rs reset --chip STM32F469NIHx"
    ```
  - Capture defmt logs to verify correct operation:
    ```bash
    ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-lcd-test 2>&1"
    ```
  - Expected log output should include:
    - `"Detected LCD controller: Nt35510"` (or `Otm8009a` depending on board)
    - `"Initializing NT35510"` (or OTM8009A)
    - `"Outputting Color/BER test patterns"` (or similar test pattern message)
    - NO panics, no hardfaults
  - If the display doesn't work (garbled/no output), but the program runs without panic:
    - This means the hypothesis (ST spec timings + fixed PLLSAI) was wrong
    - In that case: revert Task 5 (timing change), keep Tasks 1-4 (DSI register fix + `new_dsi()` switch)
    - Re-flash with tight timings to verify the DSI register fix alone works

  **Must NOT do**:
  - DO NOT modify any source code in this task — it's verification only
  - DO NOT skip the defmt log capture — it's the primary evidence

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Requires SSH to remote board, flashing embedded hardware, interpreting defmt logs
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO (must be sequential after compile check)
  - **Parallel Group**: Wave 4 (alone)
  - **Blocks**: Final verification wave
  - **Blocked By**: Task 6 (must compile first)

  **References**:
  - Remote board: `ubuntu@192.168.13.246`, chip: `STM32F469NIHx`
  - `.sisyphus/notes/f469-development.md` — Previous flash/verify patterns and expected log output
  - `examples/f469disco-lcd-test.rs` — The example being flashed

  **Acceptance Criteria**:
  - [ ] `cargo build --release` succeeds for `f469disco-lcd-test`
  - [ ] `probe-rs download` succeeds on remote board
  - [ ] defmt logs show LCD controller detected (Nt35510 or Otm8009a)
  - [ ] defmt logs show test patterns being output
  - [ ] No panic or hardfault in logs

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Build, flash, and verify display works
    Tool: Bash (SSH to remote board)
    Preconditions: Task 6 passed (all examples compile)
    Steps:
      1. Run: cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"
      2. Assert: exit code 0, binary exists at target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test
      3. Run: scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/
      4. Assert: exit code 0
      5. Run: ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test && probe-rs reset --chip STM32F469NIHx"
      6. Assert: exit code 0 (flash succeeded)
      7. Run: ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-lcd-test 2>&1" | tee .sisyphus/evidence/task-7-defmt-logs.txt
      8. Assert: output contains "Detected LCD controller" (not a panic)
      9. Assert: output contains "test pattern" or "Color" or "BER" (display is outputting)
      10. Assert: output does NOT contain "panicked" or "HardFault"
    Expected Result: Program boots, detects display, outputs test patterns
    Failure Indicators: Panic, HardFault, or no output within 15s timeout
    Evidence: .sisyphus/evidence/task-7-defmt-logs.txt

  Scenario: Rollback verification (if ST spec timings fail)
    Tool: Bash
    Preconditions: Main scenario failed — program runs but display is garbled
    Steps:
      1. Revert NT35510 timings to tight values (v_sync=1, v_back_porch=15, v_front_porch=16)
      2. Keep all other changes (DsiConfig fields, new_dsi(), PLLSAI setup)
      3. Rebuild, reflash, and verify
      4. Assert: display works with tight timings + new_dsi() + fixed PLLSAI
    Expected Result: Tight timings still work with the new_dsi()/PLLSAI approach
    Failure Indicators: Display broken even with tight timings after switch to new_dsi()
    Evidence: .sisyphus/evidence/task-7-rollback-verification.txt
  ```

  **Commit**: NO (verification only — commits done in Tasks 1/3 and 4/5)


## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run compile command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --features="stm32f469,defmt"` on all examples. Review all changed files for: `as any`, empty catches, commented-out code, unused imports. Check for AI slop: excessive comments, over-abstraction, generic names. Verify `DsiConfig` still derives `Clone, Copy, Debug, PartialEq, Eq` and `defmt::Format`.
  Output: `Build [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Build and flash `f469disco-lcd-test` to remote board. Capture defmt logs for 15 seconds. Verify: DSI init succeeds, LCD controller detected, display patterns output, no panics. Also build and flash `f469disco-hello-eg` to verify regression-free. Save evidence to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Regression [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual git diff. Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance: `DisplayController::new()` untouched, `display_init.rs` DISPLAY_CONFIG timings untouched, `nt35510.rs` untouched. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Commit 1** (after Tasks 1, 3): `feat(dsi): make NullPacketSize and NumberOfChunks configurable in DsiConfig`
  - Files: `src/dsi.rs`, `examples/f469disco/display_init.rs`, `examples/f469disco-lcd-test.rs` (DsiConfig only)
  - Pre-commit: `cargo check --example f469disco-lcd-test --features="stm32f469,defmt" && cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"`

- **Commit 2** (after Tasks 4, 5): `fix(example): use new_dsi() with fixed PLLSAI and ST spec NT35510 timings`
  - Files: `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"`

---

## Success Criteria

### Verification Commands
```bash
# All examples compile
cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
cargo check --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
cargo check --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt"

# Build for flash
cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"

# Flash and verify on remote board
scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test && probe-rs reset --chip STM32F469NIHx"
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-lcd-test 2>&1"
# Expected: logs show "Detected LCD controller: Nt35510" and "Outputting Color/BER test patterns" with no panic
```

### Final Checklist
- [ ] `null_packet_size` and `number_of_chunks` in `DsiConfig` ✓
- [ ] `DsiHost::init()` uses config fields, not hardcoded values ✓
- [ ] `f469disco-lcd-test.rs` uses `DisplayController::new_dsi()` ✓
- [ ] PLLSAI configured to PLLN=384, PLLR=7, DIVR=2 (27.429 MHz) ✓
- [ ] NT35510 timings: v_sync=120, v_back_porch=150, v_front_porch=150 ✓
- [ ] `DisplayController::new()` unchanged ✓
- [ ] `display_init.rs` DISPLAY_CONFIG timings unchanged ✓
- [ ] All 6 f469disco examples compile ✓
- [ ] Display works on remote board ✓
