# Dependency Update + F412/F413 Re-enablement

## TL;DR

> **Quick Summary**: Update all vls-signer-stm32 dependencies to latest safe versions (including mipidsi 0.7.1→0.8.0), update F412/F413 display init code for mipidsi 0.8 API, re-label F412/F413 from "Legacy" to "Untested", verify all three board targets compile and pass clippy, then commit and push.
> 
> **Deliverables**:
> - All 11 dependency bumps applied to Cargo.toml
> - F412/F413 display init code updated for mipidsi 0.8.0 API
> - README.md and Cargo.toml comments updated: "Legacy" → "Untested"
> - All 3 targets (F469, F412, F413) compile cleanly
> - All 3 targets pass `cargo clippy`
> - Changes committed and pushed
> 
> **Estimated Effort**: Short (4 files, well-researched API migration)
> **Parallel Execution**: YES — 2 waves
> **Critical Path**: Baseline check → Cargo.toml updates → F412/F413 code fix → Compile verify → Clippy → README update → Commit & push

---

## Context

### Original Request
Update all dependencies in vls-signer-stm32 to their latest safe versions, migrate mipidsi from 0.7.1 to 0.8.0, re-enable F412/F413 as "untested" boards, verify all three board targets compile and pass clippy, mark F412/F413 as untested in docs, then git commit and push.

### Interview Summary
**Key Discussions**:
- mipidsi 0.8.0 chosen over 0.9/0.10 because 0.8 uses `display-interface v0.5` which the HAL already supports, while 0.9+ would require a custom adapter
- Test strategy: compile-check + clippy (no unit tests — embedded target, no hardware available for F412/F413)
- st7789 crate deprecated; mipidsi is the maintained successor by the same author

**Research Findings**:
- All 9 non-mipidsi dep bumps verified as non-breaking via changelog review
- alloc-cortex-m 0.4.1→0.4.4 is a critical security fix (RUSTSEC-2022-0063)
- usbd-serial 0.2.0→0.2.2 fixes a Zero-Length Packet bug causing host hangs
- mipidsi 0.7→0.8 migration guide documented exact API changes (Builder::new, display_size, orientation, reset_pin, init takes &mut delay)
- HAL's `Lcd<S, u16>` implements display-interface v0.5 `WriteOnlyDataCommand` ✅
- HAL's `SysDelay` implements `DelayNs` (embedded-hal 1.0) ✅
- HAL's GPIO pins implement both embedded-hal 0.2 and 1.0 `OutputPin` ✅

### Metis Review
**Identified Gaps** (addressed):
- Execution order: baseline F469 check before any changes (prevents false negatives)
- display-interface dep: update from 0.4.1 to 0.5.0 or verify it's transitively satisfied
- Orientation enum: verify still exported from mipidsi root in 0.8
- Display type params: verify mipidsi 0.8 `Display<DI, MODEL, RST>` still has 3 params
- backlight_control.set_high(): may need explicit trait import if ambiguous between eh 0.2 and 1.0

---

## Work Objectives

### Core Objective
Update all vls-signer-stm32 dependencies to their latest safe versions and re-enable F412/F413 board support as "untested" targets.

### Concrete Deliverables
- Updated `Cargo.toml` with 11 dependency version bumps + comment change
- Updated `f412.rs` and `f413.rs` with mipidsi 0.8.0 API
- Updated `README.md` with "Untested" status for F412/F413
- All 3 board targets compile and pass clippy

### Definition of Done
- [ ] `cargo check --no-default-features --features stm32f412 --release` exits 0
- [ ] `cargo check --no-default-features --features stm32f413 --release` exits 0
- [ ] `cargo check --features stm32f469 --release` exits 0
- [ ] `cargo clippy --no-default-features --features stm32f412 --release` exits 0
- [ ] `cargo clippy --no-default-features --features stm32f413 --release` exits 0
- [ ] `cargo clippy --features stm32f469 --release` exits 0
- [ ] Changes committed and pushed

### Must Have
- F469 must NOT break (it's the production target)
- mipidsi updated to exactly 0.8.0 (not 0.9 or higher)
- display-interface updated to 0.5.0 to match mipidsi 0.8's transitive dep
- F412/F413 marked as "Untested" (not "Legacy", not "Supported")
- All safe dependency bumps applied

### Must NOT Have (Guardrails)
- NO changes to f469.rs or any F469-specific code
- NO changes to any HAL code (stm32f4xx-hal crate itself)
- NO mipidsi version higher than 0.8.x
- NO changes to feature flags in Cargo.toml (stm32f412/f413 feature definitions stay the same)
- NO new dependencies added
- NO functional behavior changes (same display init, same orientation, same 240x240 size)

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (embedded target, no test harness)
- **Automated tests**: None (compile-check + clippy only)
- **Framework**: N/A
- **Rationale**: This is a `no_std` embedded target cross-compiled to `thumbv7em-none-eabihf`. There is no test runner. Verification = compilation + clippy.

### QA Policy
Every task includes compile-check verification as QA scenario.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.txt`.

- **Compilation**: Use Bash — `cargo check --features <target> --release`
- **Lint**: Use Bash — `cargo clippy --features <target> --release`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 0 (Baseline — verify current state):
└── Task 1: Baseline F469 compile check [quick]

Wave 1 (After Wave 0 — all dep updates + code changes, PARALLEL):
├── Task 2: Update Cargo.toml dependencies [quick]
│   (blocks: Tasks 3, 4, 5)
│   NOTE: Task 2 must complete before 3/4/5 can compile,
│         but 3/4/5 can be written in parallel with 2

Wave 2 (After Task 2 — code changes, PARALLEL):
├── Task 3: Update f412.rs for mipidsi 0.8.0 API (depends: 2) [quick]
├── Task 4: Update f413.rs for mipidsi 0.8.0 API (depends: 2) [quick]
└── Task 5: Update README.md + Cargo.toml comments (depends: 2) [quick]

Wave 3 (After Wave 2 — verification):
├── Task 6: Compile-check all 3 targets (depends: 3, 4, 5) [quick]
└── Task 7: Clippy all 3 targets (depends: 6) [quick]

Wave 4 (After Wave 3 — commit):
└── Task 8: Git commit and push (depends: 7) [quick]

Wave FINAL (After ALL tasks — independent review):
├── Task F1: Plan compliance audit (oracle)
└── Task F2: Scope fidelity check (deep)

Critical Path: Task 1 → Task 2 → Task 3/4 → Task 6 → Task 7 → Task 8
Parallel Speedup: ~30% (Wave 2 has 3 parallel tasks)
Max Concurrent: 3 (Wave 2)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 2 | 0 |
| 2 | 1 | 3, 4, 5 | 1 |
| 3 | 2 | 6 | 2 |
| 4 | 2 | 6 | 2 |
| 5 | 2 | 6 | 2 |
| 6 | 3, 4, 5 | 7 | 3 |
| 7 | 6 | 8 | 3 |
| 8 | 7 | F1, F2 | 4 |
| F1 | 8 | — | FINAL |
| F2 | 8 | — | FINAL |

### Agent Dispatch Summary

- **Wave 0**: **1** — T1 → `quick`
- **Wave 1**: **1** — T2 → `quick`
- **Wave 2**: **3** — T3 → `quick`, T4 → `quick`, T5 → `quick`
- **Wave 3**: **2** — T6 → `quick`, T7 → `quick`
- **Wave 4**: **1** — T8 → `quick` + `git-master`
- **FINAL**: **2** — F1 → `oracle`, F2 → `deep`

---

## TODOs


- [ ] 1. Baseline F469 Compile Check

  **What to do**:
  - Run `cargo check --features stm32f469 --release` to verify F469 compiles BEFORE any changes
  - This establishes a known-good baseline. If this fails, there's a pre-existing issue unrelated to our changes

  **Must NOT do**:
  - Do NOT modify any files
  - Do NOT proceed to other tasks if this fails

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 0 (solo)
  - **Blocks**: Task 2
  - **Blocked By**: None (start immediately)

  **References**:
  - `validating-lightning-signer/vls-signer-stm32/Cargo.toml` — Current dependency versions to verify
  - Working directory: `validating-lightning-signer/vls-signer-stm32`

  **Acceptance Criteria**:
  - [ ] `cargo check --features stm32f469 --release` exits 0

  **QA Scenarios:**

  ```
  Scenario: F469 baseline compilation
    Tool: Bash
    Preconditions: No files modified, working directory is validating-lightning-signer/vls-signer-stm32
    Steps:
      1. Run: cargo check --features stm32f469 --release
      2. Capture exit code and output
    Expected Result: Exit code 0, no errors
    Failure Indicators: Non-zero exit code, any "error[E" in output
    Evidence: .sisyphus/evidence/task-1-baseline-f469.txt
  ```

  **Commit**: NO

- [ ] 2. Update Cargo.toml Dependencies

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/Cargo.toml`, update the following dependency versions:
    - Line 11: `nb = "1"` → `nb = "1.1"`
    - Line 12: `cortex-m = { version = "0.7", ...}` → `cortex-m = { version = "0.7.7", ...}`
    - Line 13: `cortex-m-rt = "0.7"` → `cortex-m-rt = "0.7.5"`
    - Line 14: `alloc-cortex-m = { version = "0.4.1" }` → `alloc-cortex-m = { version = "0.4.4" }` (**SECURITY: RUSTSEC-2022-0063**)
    - Line 15: `display-interface = { version = "0.4.1", optional = true }` → `display-interface = { version = "0.5.0", optional = true }`
    - Line 17: `embedded-graphics = "0.8.1"` → `embedded-graphics = "0.8.2"`
    - Line 18: `mipidsi = { version = "0.7.1", optional = true }` → `mipidsi = { version = "0.8.0", optional = true }`
    - Line 19: `rtt-target = { version = "0.6.1" }` → `rtt-target = { version = "0.6.2" }`
    - Line 24: `usb-device = "0.3.1"` → `usb-device = "0.3.2"`
    - Line 25: `usbd-serial = "0.2.0"` → `usbd-serial = "0.2.2"` (**BUG FIX: ZLP host hang**)
    - Line 26: `fugit = "0.3"` → `fugit = "0.3.9"`
  - Also update the comment on line 57:
    - `# Legacy features (not yet updated for current HAL API):` → `# Untested features (no hardware available for verification):`

  **Must NOT do**:
  - Do NOT change any feature definitions (stm32f412, stm32f413 feature lists stay identical)
  - Do NOT add new dependencies
  - Do NOT change stm32f4 version (0.16.0 — pinned to HAL)
  - Do NOT change embedded-hal, embedded-hal-02, profont, panic-probe, rand_core, log, serde_json, hex, ft6x06, stm32f469i-disc, stm32-fmc, fatfs, or stm32f4xx-hal deps

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (solo)
  - **Blocks**: Tasks 3, 4, 5
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/Cargo.toml` — The file to edit, all changes are version string bumps

  **External References**:
  - RUSTSEC-2022-0063: alloc-cortex-m unsoundness fix in 0.4.4
  - usbd-serial 0.2.2 changelog: ZLP bug fix preventing host hangs
  - mipidsi 0.8.0: uses display-interface 0.5.0 + embedded-hal 1.0

  **WHY Each Reference Matters**:
  - Cargo.toml is the only file to edit — all changes are version bumps, no structural changes
  - The comment change on line 57 re-labels F412/F413 from "Legacy" to "Untested"

  **Acceptance Criteria**:
  - [ ] `nb` version is `"1.1"`
  - [ ] `cortex-m` version is `"0.7.7"`
  - [ ] `cortex-m-rt` version is `"0.7.5"`
  - [ ] `alloc-cortex-m` version is `"0.4.4"`
  - [ ] `display-interface` version is `"0.5.0"`
  - [ ] `embedded-graphics` version is `"0.8.2"`
  - [ ] `mipidsi` version is `"0.8.0"`
  - [ ] `rtt-target` version is `"0.6.2"`
  - [ ] `usb-device` version is `"0.3.2"`
  - [ ] `usbd-serial` version is `"0.2.2"`
  - [ ] `fugit` version is `"0.3.9"`
  - [ ] Line 57 comment says "Untested" not "Legacy"
  - [ ] NO other deps changed

  **QA Scenarios:**

  ```
  Scenario: Verify all version bumps applied correctly
    Tool: Bash
    Preconditions: Cargo.toml has been edited
    Steps:
      1. Run: grep 'nb = ' Cargo.toml — expect '"1.1"'
      2. Run: grep 'alloc-cortex-m' Cargo.toml — expect '"0.4.4"'
      3. Run: grep 'mipidsi' Cargo.toml — expect '"0.8.0"'
      4. Run: grep 'usbd-serial' Cargo.toml — expect '"0.2.2"'
      5. Run: grep 'display-interface' Cargo.toml — expect '"0.5.0"'
      6. Run: grep 'Legacy' Cargo.toml — expect no matches
      7. Run: grep 'Untested' Cargo.toml — expect 1 match on the comment line
    Expected Result: All version strings match expected values, no "Legacy" text remains
    Failure Indicators: Any version mismatch, or "Legacy" still present
    Evidence: .sisyphus/evidence/task-2-cargo-versions.txt
  ```

  **Commit**: YES (groups with Tasks 3, 4, 5, as Task 8)
  - Files: `validating-lightning-signer/vls-signer-stm32/Cargo.toml`

- [ ] 3. Update f412.rs for mipidsi 0.8.0 API

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs`, update the mipidsi display initialization from 0.7 API to 0.8 API:
  - **Line 154-158** — Replace the display builder block:
    ```rust
    // CURRENT (mipidsi 0.7.1):
    let mut disp = Builder::st7789(interface)
        .with_display_size(240, 240)
        .with_orientation(Orientation::Portrait(false))
        .init(delay, Some(lcd_reset))
        .unwrap();

    // NEW (mipidsi 0.8.0):
    let mut disp = Builder::new(ST7789, interface)
        .display_size(240, 240)
        .orientation(Orientation::Portrait(false))
        .reset_pin(lcd_reset)
        .init(&mut *delay)
        .unwrap();
    ```
  - **Note on `&mut *delay`**: The `delay` parameter is `&mut SysDelay`. mipidsi 0.8's `init()` takes `&mut impl DelayNs`. Since `delay` is already `&mut SysDelay` and `SysDelay` implements `DelayNs`, passing `delay` directly (or `&mut *delay`) should work. The dereference-and-reborrow ensures the right trait impl is selected.
  - **Line 18**: Import may stay the same — `use mipidsi::{models::ST7789, Builder, Orientation};` — verify `Orientation` is still at `mipidsi::Orientation` in 0.8 (it is per migration guide).
  - **Line 31**: `DisplayInner` type alias may need updating if mipidsi 0.8 changes the `Display` type signature. Per research, it should still be `mipidsi::Display<DI, MODEL, RST>` with 3 type params. Verify at compile time.

  **Must NOT do**:
  - Do NOT change FSMC LCD pin setup (lines 131-152) — those are HAL API calls, not mipidsi
  - Do NOT change backlight_control.set_high() (line 161) — backlight management is external
  - Do NOT change touch or I2C init
  - Do NOT change constants or layout values

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 4, 5)
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Task 6
  - **Blocked By**: Task 2

  **References**:

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs:154-158` — Current mipidsi 0.7 builder code to replace
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs:18` — Current import line
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs:31` — DisplayInner type alias

  **External References**:
  - mipidsi 0.7→0.8 migration guide: Constructor changes from `Builder::st7789(di)` to `Builder::new(ST7789, di)`, `.with_display_size()` to `.display_size()`, `.with_orientation()` to `.orientation()`, `.init(delay, Some(rst))` to `.reset_pin(rst).init(&mut delay)`

  **WHY Each Reference Matters**:
  - Lines 154-158 are the ONLY lines that call mipidsi API — this is the sole change needed
  - Line 18 import may need verification but likely stays the same
  - Line 31 type alias depends on mipidsi's `Display` type signature — verify at compile

  **Acceptance Criteria**:
  - [ ] `Builder::new(ST7789, interface)` used instead of `Builder::st7789(interface)`
  - [ ] `.display_size(240, 240)` used instead of `.with_display_size(240, 240)`
  - [ ] `.orientation(...)` used instead of `.with_orientation(...)`
  - [ ] `.reset_pin(lcd_reset).init(&mut *delay)` used instead of `.init(delay, Some(lcd_reset))`
  - [ ] No other lines in f412.rs changed

  **QA Scenarios:**

  ```
  Scenario: Verify f412.rs mipidsi 0.8 API migration
    Tool: Bash
    Preconditions: f412.rs has been edited
    Steps:
      1. Run: grep 'Builder::new' f412.rs — expect match
      2. Run: grep 'Builder::st7789' f412.rs — expect NO match
      3. Run: grep 'with_display_size' f412.rs — expect NO match
      4. Run: grep 'with_orientation' f412.rs — expect NO match
      5. Run: grep 'display_size' f412.rs — expect match
      6. Run: grep 'reset_pin' f412.rs — expect match
    Expected Result: All new API patterns present, no old API patterns
    Failure Indicators: Any old mipidsi 0.7 API pattern found
    Evidence: .sisyphus/evidence/task-3-f412-migration.txt
  ```

  **Commit**: YES (groups with Tasks 2, 4, 5, as Task 8)
  - Files: `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs`


- [ ] 4. Update f413.rs for mipidsi 0.8.0 API

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs`, update the mipidsi display initialization from 0.7 API to 0.8 API:
  - **Lines 152-156** — Replace the display builder block:
    ```rust
    // CURRENT (mipidsi 0.7.1):
    let mut disp = Builder::st7789(interface)
        .with_display_size(240, 240)
        .with_orientation(Orientation::PortraitInverted(false))
        .init(delay, Some(lcd_reset))
        .unwrap();

    // NEW (mipidsi 0.8.0):
    let mut disp = Builder::new(ST7789, interface)
        .display_size(240, 240)
        .orientation(Orientation::PortraitInverted(false))
        .reset_pin(lcd_reset)
        .init(&mut *delay)
        .unwrap();
    ```
  - **Same notes as Task 3** regarding `&mut *delay`, import line, and DisplayInner type alias.
  - **Key difference from f412**: F413 uses `Orientation::PortraitInverted(false)` (not `Portrait`)

  **Must NOT do**:
  - Do NOT change FSMC LCD pin setup (lines 129-150)
  - Do NOT change backlight_control.set_high() (line 159)
  - Do NOT change FMPI2C init or touch init
  - Do NOT change constants or layout values

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 3, 5)
  - **Parallel Group**: Wave 2 (with Tasks 3, 5)
  - **Blocks**: Task 6
  - **Blocked By**: Task 2

  **References**:

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs:152-156` — Current mipidsi 0.7 builder code to replace
  - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs:19` — Current import line
  - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs:32` — DisplayInner type alias
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs` — Use f412.rs Task 3 changes as pattern (identical except orientation)

  **External References**:
  - Same mipidsi 0.7→0.8 migration guide as Task 3

  **WHY Each Reference Matters**:
  - Lines 152-156 are the ONLY lines that call mipidsi API
  - f412.rs (after Task 3) serves as a direct template — copy the pattern, change `Portrait` to `PortraitInverted`

  **Acceptance Criteria**:
  - [ ] `Builder::new(ST7789, interface)` used instead of `Builder::st7789(interface)`
  - [ ] `.display_size(240, 240)` used instead of `.with_display_size(240, 240)`
  - [ ] `.orientation(Orientation::PortraitInverted(false))` used
  - [ ] `.reset_pin(lcd_reset).init(&mut *delay)` used instead of `.init(delay, Some(lcd_reset))`
  - [ ] No other lines in f413.rs changed

  **QA Scenarios:**

  ```
  Scenario: Verify f413.rs mipidsi 0.8 API migration
    Tool: Bash
    Preconditions: f413.rs has been edited
    Steps:
      1. Run: grep 'Builder::new' f413.rs — expect match
      2. Run: grep 'Builder::st7789' f413.rs — expect NO match
      3. Run: grep 'PortraitInverted' f413.rs — expect match
      4. Run: grep 'with_display_size' f413.rs — expect NO match
      5. Run: grep 'reset_pin' f413.rs — expect match
    Expected Result: All new API patterns present, no old API patterns
    Failure Indicators: Any old mipidsi 0.7 API pattern found
    Evidence: .sisyphus/evidence/task-4-f413-migration.txt
  ```

  **Commit**: YES (groups with Tasks 2, 3, 5, as Task 8)
  - Files: `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs`

- [ ] 5. Update README.md: Legacy → Untested

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/README.md`:
    - **Line 3**: Change "F412/F413 support is legacy and not yet updated for the current stm32f4xx-hal API." → "F412/F413 support is included but untested (no hardware available for verification)."
    - **Lines 8-10**: Change section header and entries:
      ```
      // CURRENT:
      Legacy (not yet updated):
      - **STM32F412 Discovery** (32F412GDISCOVERY) — 240×240 ST7789 LCD, FSMC interface
      - **STM32F413 Discovery** (32F413HDISCOVERY) — 240×240 ST7789 LCD, FSMC interface

      // NEW:
      Untested (compiles but no hardware verification):
      - **STM32F412 Discovery** (32F412GDISCOVERY) ⚠️ — 240×240 ST7789 LCD, FSMC interface
      - **STM32F413 Discovery** (32F413HDISCOVERY) ⚠️ — 240×240 ST7789 LCD, FSMC interface
      ```
    - **Lines 170-182**: Update the "Legacy Board Support" section:
      ```
      // CURRENT:
      #### Legacy Board Support
      
      F412 and F413 Discovery boards are no longer actively maintained...
      relies on an older stm32f4xx-hal API that has changed...

      // NEW:
      #### Untested Board Support
      
      F412 and F413 Discovery boards compile with the current HAL API but have not been
      verified on physical hardware. They use the mipidsi display driver with FSMC LCD interface.
      If you have F412 or F413 Discovery hardware and can test, contributions are welcome.
      ```

  **Must NOT do**:
  - Do NOT change F469 documentation or status
  - Do NOT change setup instructions, build commands, or test instructions
  - Do NOT change any other sections

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 3, 4)
  - **Parallel Group**: Wave 2 (with Tasks 3, 4)
  - **Blocks**: Task 6
  - **Blocked By**: Task 2

  **References**:
  - `validating-lightning-signer/vls-signer-stm32/README.md` — Lines 3, 8-10, 170-182 need updating

  **Acceptance Criteria**:
  - [ ] No occurrence of "Legacy" in README.md
  - [ ] "Untested" appears in README.md for F412/F413 sections
  - [ ] F469 status unchanged (still shows ✅)
  - [ ] ⚠️ emoji used for F412/F413 board entries

  **QA Scenarios:**

  ```
  Scenario: Verify README updates
    Tool: Bash
    Preconditions: README.md has been edited
    Steps:
      1. Run: grep -i 'legacy' README.md — expect NO matches
      2. Run: grep 'Untested' README.md — expect matches for F412/F413 sections
      3. Run: grep '✅' README.md — expect F469 still has checkmark
      4. Run: grep '⚠️' README.md — expect F412 and F413 entries have warning
    Expected Result: Legacy removed, Untested added, F469 unchanged
    Failure Indicators: "Legacy" still present, or F469 status changed
    Evidence: .sisyphus/evidence/task-5-readme-update.txt
  ```

  **Commit**: YES (groups with Tasks 2, 3, 4, as Task 8)
  - Files: `validating-lightning-signer/vls-signer-stm32/README.md`

- [ ] 6. Compile-check All 3 Targets

  **What to do**:
  - Run `cargo check` for all 3 board targets to verify everything compiles after all changes:
    ```bash
    cd validating-lightning-signer/vls-signer-stm32
    cargo check --features stm32f469 --release
    cargo check --no-default-features --features stm32f412 --release
    cargo check --no-default-features --features stm32f413 --release
    ```
  - If any target fails to compile, identify the error and report it back — do NOT try to fix it (that would be scope of the blocked task)
  - **Likely compile issues to watch for**:
    - mipidsi 0.8 `Display` type might have different type params → fix DisplayInner type alias in f412.rs/f413.rs
    - `Orientation` might be in a different module path in mipidsi 0.8 → fix import
    - `init()` return type might be different → check if `.unwrap()` still works
    - Ambiguous `.set_high()` method if both eh 0.2 and 1.0 `OutputPin` are in scope → add explicit trait import

  **Must NOT do**:
  - Do NOT modify files during this task (verification only)
  - If compile errors occur in f412.rs or f413.rs, report them so the executing agent can fix Tasks 3/4

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential with Task 7)
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 3, 4, 5

  **References**:
  - All files from Tasks 2-5 — this task validates their correctness

  **Acceptance Criteria**:
  - [ ] `cargo check --features stm32f469 --release` exits 0
  - [ ] `cargo check --no-default-features --features stm32f412 --release` exits 0
  - [ ] `cargo check --no-default-features --features stm32f413 --release` exits 0

  **QA Scenarios:**

  ```
  Scenario: All 3 targets compile successfully
    Tool: Bash
    Preconditions: Tasks 2-5 complete, all files updated
    Steps:
      1. Run: cargo check --features stm32f469 --release 2>&1
      2. Verify exit code 0
      3. Run: cargo check --no-default-features --features stm32f412 --release 2>&1
      4. Verify exit code 0
      5. Run: cargo check --no-default-features --features stm32f413 --release 2>&1
      6. Verify exit code 0
    Expected Result: All 3 exit code 0, no "error[E" in output
    Failure Indicators: Non-zero exit code on any target
    Evidence: .sisyphus/evidence/task-6-compile-check.txt

  Scenario: F469 compile is not regressed
    Tool: Bash
    Preconditions: Baseline from Task 1 passed
    Steps:
      1. Compare Task 1 evidence (baseline) with this task's F469 output
      2. Verify no new warnings introduced
    Expected Result: F469 compiles as cleanly as baseline
    Evidence: .sisyphus/evidence/task-6-f469-regression.txt
  ```

  **Commit**: NO (verification only)

- [ ] 7. Clippy All 3 Targets

  **What to do**:
  - Run `cargo clippy` for all 3 board targets:
    ```bash
    cd validating-lightning-signer/vls-signer-stm32
    cargo clippy --features stm32f469 --release -- -D warnings
    cargo clippy --no-default-features --features stm32f412 --release -- -D warnings
    cargo clippy --no-default-features --features stm32f413 --release -- -D warnings
    ```
  - If clippy warnings appear, fix them in the relevant files (f412.rs, f413.rs, or Cargo.toml)
  - Common clippy issues after migration:
    - Unused imports (if mipidsi 0.8 re-exports differently)
    - Unnecessary `mut` if API changed mutability requirements
    - `unwrap()` usage (might suggest `expect()` — keep `unwrap()` to match existing codebase style)

  **Must NOT do**:
  - Do NOT fix clippy warnings in f469.rs or any non-touched files
  - Do NOT suppress warnings with `#[allow]` — fix them properly

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (depends on Task 6 passing)
  - **Parallel Group**: Wave 3 (after Task 6)
  - **Blocks**: Task 8
  - **Blocked By**: Task 6

  **References**:
  - All files from Tasks 2-5
  - Existing clippy style: the codebase uses `#[allow(clippy::too_many_arguments)]` on init_display — match this style

  **Acceptance Criteria**:
  - [ ] `cargo clippy --features stm32f469 --release -- -D warnings` exits 0
  - [ ] `cargo clippy --no-default-features --features stm32f412 --release -- -D warnings` exits 0
  - [ ] `cargo clippy --no-default-features --features stm32f413 --release -- -D warnings` exits 0

  **QA Scenarios:**

  ```
  Scenario: All 3 targets pass clippy with no warnings
    Tool: Bash
    Preconditions: Task 6 passed (all targets compile)
    Steps:
      1. Run: cargo clippy --features stm32f469 --release -- -D warnings 2>&1
      2. Verify exit code 0
      3. Run: cargo clippy --no-default-features --features stm32f412 --release -- -D warnings 2>&1
      4. Verify exit code 0
      5. Run: cargo clippy --no-default-features --features stm32f413 --release -- -D warnings 2>&1
      6. Verify exit code 0
    Expected Result: All 3 exit code 0, zero warnings
    Failure Indicators: Non-zero exit code or any warning text in output
    Evidence: .sisyphus/evidence/task-7-clippy.txt
  ```

  **Commit**: NO (verification only, or YES if clippy fixes were needed — group with Task 8)

- [ ] 8. Git Commit and Push

  **What to do**:
  - Stage all changed files:
    - `validating-lightning-signer/vls-signer-stm32/Cargo.toml`
    - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs`
    - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs`
    - `validating-lightning-signer/vls-signer-stm32/README.md`
    - Plus `Cargo.lock` if it was updated by dependency resolution
  - Verify no unintended files are staged (especially no HAL files)
  - Commit with message: `chore(vls): update deps to latest safe versions, migrate mipidsi 0.7→0.8, re-enable F412/F413 as untested`
  - Push to origin

  **Must NOT do**:
  - Do NOT commit any files outside `validating-lightning-signer/vls-signer-stm32/`
  - Do NOT force push
  - Do NOT amend previous commits

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (solo)
  - **Blocks**: F1, F2
  - **Blocked By**: Task 7

  **References**:
  - Git working directory is the repo root (`/Users/macbook/src/stm32f4xx-hal`)
  - The VLS subproject is at `validating-lightning-signer/` (a subdirectory, NOT a git submodule)

  **Acceptance Criteria**:
  - [ ] `git status` shows clean working tree after commit
  - [ ] `git log -1 --oneline` shows the expected commit message
  - [ ] `git push` succeeds
  - [ ] Only 4-5 files in the commit (Cargo.toml, f412.rs, f413.rs, README.md, optionally Cargo.lock)

  **QA Scenarios:**

  ```
  Scenario: Clean commit with correct scope
    Tool: Bash
    Preconditions: Tasks 2-7 complete, all changes verified
    Steps:
      1. Run: git add -A validating-lightning-signer/vls-signer-stm32/
      2. Run: git diff --cached --name-only — verify only expected files
      3. Run: git commit -m 'chore(vls): update deps to latest safe versions, migrate mipidsi 0.7→0.8, re-enable F412/F413 as untested'
      4. Run: git push
      5. Run: git status — verify clean
    Expected Result: Commit created and pushed, working tree clean
    Failure Indicators: Unexpected files in diff, push rejected, working tree not clean
    Evidence: .sisyphus/evidence/task-8-commit.txt
  ```

  **Commit**: This IS the commit task
  - Message: `chore(vls): update deps to latest safe versions, migrate mipidsi 0.7→0.8, re-enable F412/F413 as untested`
  - Files: Cargo.toml, f412.rs, f413.rs, README.md (+ Cargo.lock if changed)

---

## Final Verification Wave

> 2 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (run cargo check, grep for version strings). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Task 8**: `chore(vls): update deps to latest safe versions, migrate mipidsi 0.7→0.8, re-enable F412/F413 as untested` — Cargo.toml, f412.rs, f413.rs, README.md

---

## Success Criteria

### Verification Commands
```bash
cd validating-lightning-signer/vls-signer-stm32
cargo check --features stm32f469 --release                          # Expected: exit 0
cargo check --no-default-features --features stm32f412 --release    # Expected: exit 0
cargo check --no-default-features --features stm32f413 --release    # Expected: exit 0
cargo clippy --features stm32f469 --release -- -D warnings          # Expected: exit 0
cargo clippy --no-default-features --features stm32f412 --release -- -D warnings  # Expected: exit 0
cargo clippy --no-default-features --features stm32f413 --release -- -D warnings  # Expected: exit 0
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All 3 targets compile
- [ ] All 3 targets pass clippy
- [ ] F412/F413 labeled "Untested" in README and Cargo.toml
- [ ] Changes committed and pushed
