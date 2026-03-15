# Board Abstraction Refactor: Narrow Waist + BSP Adapter Pattern

## TL;DR

> **Quick Summary**: Refactor vls-signer-stm32 device initialization to implement the "Narrow Waist Board Interface + BSP Adapter" pattern from `board_abstraction_pattern.json`. Each board module (f412, f413, f469) gets an `init_board_io()` function returning a uniform `BoardIo` struct, eliminating `#[cfg]` sprawl in shared code and hiding BSP-private types like `SdramRemainders`.
> 
> **Deliverables**:
> - `BoardIo` struct defined once in `device/mod.rs` using board-exported type aliases (resolved at compile time per feature)
> - `init_board_io()` entry point per board encapsulating all GPIO/peripheral init
> - Refactored `make_devices()` reduced to ~20 lines (thin wrapper)
> - `SdramRemainders` hidden inside f469.rs (BSP leak eliminated)
> - Compile-time guards for board feature selection
> - All three board targets compile cleanly
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: Task 1 → Task 6 → Task 7 → Task 9 (Task 3 demoted to optional hardening)

---

## Context

### Original Request
Refactor vls-signer-stm32 to make clean use of the BSP and HAL while maintaining a light BSP (no external project) for F412 and F413. Follow expert guidance in `board_abstraction_pattern.json`. Changes can be made to HAL, BSP, or VLS. Abandon SD card troubleshooting branch work in favor of clean abstractions.

### Three Git Repositories
1. **stm32f4xx-hal** (HAL) — `/Users/macbook/src/stm32f4xx-hal/`, branch `pr2-f469disco-examples` (commit `8239f21`)
2. **stm32f469i-disc** (BSP) — `/Users/macbook/src/stm32f4xx-hal/stm32f469i-disc/`, subdirectory of HAL
3. **validating-lightning-signer** (VLS) — `/Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/`, branch `main` (commit `0fc59562`)

### Interview Summary
**Key Discussions**:
- Expert guidance (board_abstraction_pattern.json) prescribes 7-step refactoring plan
- SD card troubleshooting abandoned — clean init ordering expected to fix it naturally
- F412/F413 stay HAL-only (no external BSP crate), F469 uses BSP internally
- All three boards have: Display, Touch (ft6x06), SDIO, USB serial, Button
- `SdramRemainders` is the biggest BSP leak — flows from BSP sdram.rs → lcd.rs → f469.rs → mod.rs
- Current `DeviceContext` struct fields must remain stable (setup.rs is 594-line consumer)
- No unit test infrastructure — `no_std` embedded firmware, verification is cross-compilation

**Research Findings**:
- `make_devices()` in `device/mod.rs` (lines 382-556) is primary refactor target — massive `#[cfg]` blocks
- BSP `split_sdram_pins()` returns `(sdram_pins, SdramRemainders, PH7)` consuming 7 GPIO ports
- BSP `init_display_pipeline()` returns `(Display, SdramRemainders)` — remainder leaks to caller
- BSP `sdio::init()` takes `SdramRemainders`, returns `(Sdio<SdCard>, PC1)` — PC1 used for touch interrupt
- Board modules already export type aliases: `I2C`, `TouchInterruptPin`, `DisplayInner`, screen constants
- Missing exports: `Button` type alias, `init_board_io()`, unified peripheral init

### Metis Review
**Identified Gaps** (addressed):
- DeviceContext preservation verified — all 10 fields mapped to BoardIo equivalents
- Optional component handling: all boards have same components, no `Option` wrapping needed (except `sdio` and `rng` which are already `Option`)
- USB serial: already board-agnostic via `SerialDriver::new(USB)` — only needs type alias
- Error handling: embedded firmware uses `unwrap()` throughout — no changes needed
- Touch differences: F412/F413 use PG5 interrupt; F469 uses PC1 (from SdramRemainders) — each board module handles internally
- Button: just a pin type alias per board (PA0 for all, but type path differs)

### External Review & Oracle Consultation (post-Metis)
**Source**: External LLM architectural review + Oracle consultation

**Key decisions (all resolved):**
1. **BoardIo location**: Define ONCE in `device/mod.rs` using board-exported type aliases. Types resolve at compile time per active feature. ✅ Confirmed by Oracle.
2. **Call syntax**: `board::init_board_io(dp, cp)` via existing `use f412 as board` alias pattern. No new module needed — `board::make_clocks()` already uses this pattern (mod.rs lines 52/57/62). ✅ Confirmed.
3. **Task 3 demoted**: BSP SdramRemainders refactoring is optional hardening, not required for the narrow-waist goal. Primary fix: change `pub use` to `use` on f469.rs line 20. The type stays internal to f469.rs because `init_board_io()` consumes it internally. ✅ Oracle recommendation.
4. **DeviceContext untouched**: Zero changes to `DeviceContext` struct definition. `Button` type alias resolves transparently to `PA0<Input>`, so `DeviceContext { button: board_io.button }` compiles without any DeviceContext changes. ✅ Confirmed.
5. **Timer types verbatim**: `Counter<TIM2, 1000000>` is the actual type from DeviceContext (not a placeholder). `FreeTimer` is a wrapper struct (mod.rs lines 186-197). Copy as-is. ✅ Confirmed.

**Additional F469-specific notes:**
- USB serial: All boards use `OTG_FS_*` peripherals — same USB type across boards. `SerialDriver::new()` is already board-agnostic.
- SDIO `Option<Sdio<SdCard>>`: Matches existing DeviceContext shape. All boards produce `Some(sdio)` in `init_board_io()` — `Option` is for future flexibility, not runtime absence.
- Touch PC1 sequencing: PC1 emerges from SDIO init (via SdramRemainders). Sequencing is: split_sdram_pins → display → sdio → touch. Must happen in this order inside f469.rs.

---

## Work Objectives

### Core Objective
Implement the Narrow Waist Board Interface pattern so that `make_devices()` becomes a uniform, cfg-free function that calls `board::init_board_io()` and constructs `DeviceContext` from the returned `BoardIo` struct.

### Concrete Deliverables
- Modified `vls-signer-stm32/src/device/f412.rs` — exports `init_board_io()`, `Button` type alias
- Modified `vls-signer-stm32/src/device/f413.rs` — same exports
- Modified `vls-signer-stm32/src/device/f469.rs` — same exports, SdramRemainders consumed internally, `pub use` downgraded to `use`
- Modified `vls-signer-stm32/src/device/mod.rs` — `BoardIo` struct, cfg-free `make_devices()`, compile-time guards
- (Optional) Modified `stm32f469i-disc/src/sdio.rs` — only if BSP API hardening warranted after primary refactor

### Definition of Done
- [ ] `cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf` succeeds
- [ ] `cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf` succeeds
- [ ] `cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf` succeeds
- [ ] `SdramRemainders` does not appear in `device/mod.rs`
- [ ] No BSP imports (`stm32f469i_disc`) in `device/mod.rs`
- [ ] `#[cfg]` in `device/mod.rs` limited to board module selection only
- [ ] `DeviceContext` struct fields unchanged from pre-refactor

### Must Have
- `BoardIo` struct with named fields defined once in `device/mod.rs` using board-exported type aliases
- `init_board_io()` in each board module returning `BoardIo`
- All GPIO splitting inside board modules (none in shared code)
- Touch init inside board modules (not shared code)
- SDIO init inside f469's board module (SdramRemainders private)
- Compile-time guards for exactly-one-feature
- Stable `DeviceContext` — ZERO textual changes to `DeviceContext` struct definition. Not even type-identical alias substitutions. `setup.rs` requires zero changes.

### Must NOT Have (Guardrails)
- No changes to `setup.rs` — it's a 594-line consumer that must remain stable
- No changes to `DeviceContext` struct definition in `device/mod.rs` — `BoardIo` uses `Button` alias but DeviceContext keeps `PA0<Input>`. Transparent alias means assignment works without changing DeviceContext.
- No changes to `sdcard.rs` FAT logic — reviewed and found adequate as-is (block device adapter with single-block cache is reasonable for embedded)
- No SD card debugging work — explicitly abandoned
- No BSP types (`SdramRemainders`, BSP-specific wrappers) in shared `device/mod.rs`
- No `#[cfg(feature = "stm32f4xx")]` in shared code except board module selection
- No "while I'm here" improvements — strict scope adherence
- No new dependencies — use existing HAL/BSP/VLS dependencies only
- No changes to HAL source code unless absolutely required for type compatibility
- No documentation beyond inline comments explaining the pattern

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (no_std embedded firmware)
- **Automated tests**: None — cross-compilation is the verification
- **Framework**: None
- **Verification method**: `cargo build` for all three targets + grep-based assertions

### QA Policy
Every task MUST include agent-executed QA scenarios using `Bash` (cargo build, grep).
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Compilation**: Use `cargo build` with target/features flags
- **BSP Leak Detection**: Use `grep` to verify no BSP types in shared code
- **Struct Verification**: Use `grep`/`ast_grep_search` to verify struct fields

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — types + BSP prep, MAX PARALLEL):
├── Task 1: Define BoardIo struct and init_board_io() signature in device/mod.rs [deep]
├── Task 2: Add Button type alias to all board modules [quick]
├── Task 3: (OPTIONAL) BSP: Refactor sdio.rs to internalize SdramRemainders [deep] — not on critical path

Wave 2 (Board modules — each independent, MAX PARALLEL):
├── Task 4: Implement init_board_io() for f412.rs (depends: 1, 2) [unspecified-high]
├── Task 5: Implement init_board_io() for f413.rs (depends: 1, 2) [unspecified-high]
├── Task 6: Implement init_board_io() for f469.rs (depends: 1, 2) [deep]

Wave 3 (Integration — refactor shared code):
├── Task 7: Refactor make_devices() to use BoardIo (depends: 4, 5, 6) [deep]
├── Task 8: Add compile-time guards (depends: 7) [quick]

Wave 4 (Verification):
├── Task 9: Cross-compile all three targets (depends: 7, 8) [unspecified-high]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: BSP leak and cfg sprawl verification (unspecified-high)
├── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 6 → Task 7 → Task 9 → F1-F4
Parallel Speedup: ~50% faster than sequential
Max Concurrent: 3 (Waves 1 & 2)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 4, 5, 6, 7 | 1 |
| 2 | — | 4, 5, 6 | 1 |
| 3 | — | — (optional) | 1 (optional) |
| 4 | 1, 2 | 7 | 2 |
| 5 | 1, 2 | 7 | 2 |
| 6 | 1, 2 | 7 | 2 |
| 7 | 4, 5, 6 | 8, 9 | 3 |
| 8 | 7 | 9 | 3 |
| 9 | 7, 8 | F1-F4 | 4 |
| F1-F4 | 9 | — | FINAL |

### Agent Dispatch Summary

- **Wave 1**: **2 required + 1 optional** — T1 → `deep`, T2 → `quick`, T3 → `deep` (optional)
- **Wave 2**: **3** — T4 → `unspecified-high`, T5 → `unspecified-high`, T6 → `deep`
- **Wave 3**: **2** — T7 → `deep`, T8 → `quick`
- **Wave 4**: **1** — T9 → `unspecified-high`
- **FINAL**: **4** — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> Implementation tasks follow. EVERY task has: Agent Profile + Parallelization + QA Scenarios.

- [x] 1. Define BoardIo struct and narrow waist contract in device/mod.rs

  **What to do**:
  - Define a `BoardIo` struct in `device/mod.rs` that captures all peripherals currently scattered across `make_devices()` return paths:
    ```rust
    pub struct BoardIo {
        pub delay: SysDelay,
        pub timer1: FreeTimer,
        pub timer2: Option<Counter<TIM2, 1000000>>,
        pub serial: SerialDriver,
        pub sdio: Option<Sdio<SdCard>>,
        pub disp: Display,
        pub rng: Option<Rng>,
        pub touchscreen: TouchDriver,
        pub i2c: I2C,
        pub button: Button,
    }
    ```
  - The fields must exactly match `DeviceContext` fields so `make_devices()` can construct `DeviceContext` from `BoardIo` trivially
  - Keep the existing board module selection pattern (Pattern A): `#[cfg(feature = "stm32f412")] mod f412; #[cfg(feature = "stm32f412")] use f412 as board;` (and same for f413, f469). Then expand the existing `pub use board::{...}` to include `Button` alongside the existing re-exports (`I2C`, `TouchInterruptPin`, etc.) — this is the ONLY cfg allowed. Do NOT use `pub mod` or `pub use f412::*` — the private `use f412 as board` alias is critical for `board::init_board_io()` calls.
  - Define `pub type Button` in the shared scope OR require each board module to export it (preferred: board modules export it)
  - Do NOT implement `init_board_io()` yet — just define the struct and its imports

  **Must NOT do**:
  - Do not modify `DeviceContext` fields
  - Do not modify `make_devices()` body yet (that's Task 7)
  - Do not touch `setup.rs`

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Defining the foundational struct that all other tasks depend on — must match existing types exactly
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed for code changes

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 2, 3)
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Tasks 4, 5, 6, 7
  - **Blocked By**: None (can start immediately)

  **References** (CRITICAL):

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:367-378` — Current `DeviceContext` struct definition. BoardIo fields must match these exactly. Note: `button` is `PA0<Input>` in DeviceContext — BoardIo uses `Button` alias which resolves transparently to the same type. Do NOT change DeviceContext.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:382-556` — Current `make_devices()` function. Study what it builds per-board to understand what BoardIo must contain.
  - `board_abstraction_pattern.json:60-131` — The `narrow_waist_api` section defining BoardIo contract and `init_board_io` signature.

  **API/Type References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs:1-30` — Existing type aliases (I2C, TouchInterruptPin, DisplayInner, etc.). BoardIo must use these types.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs:1-30` — Same type aliases for f413.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs:1-30` — Same type aliases for f469.

  **External References**:
  - None needed — all types already defined in codebase

  **WHY Each Reference Matters**:
  - `DeviceContext` (mod.rs:367-378): BoardIo fields must be a superset of DeviceContext fields so the refactored make_devices() can trivially construct DeviceContext from BoardIo
  - `make_devices()` (mod.rs:382-556): Shows what peripherals are initialized per board — BoardIo must capture ALL of them
  - `board_abstraction_pattern.json`: The authoritative design document; struct must align with its recommendations
  - Board type aliases: BoardIo uses these types — must import them correctly under cfg

  **Acceptance Criteria**:
  - [ ] `BoardIo` struct defined in `device/mod.rs` with all 10 fields matching DeviceContext
  - [ ] Board module selection uses `#[cfg(feature = "...")]` with `pub use` re-exports
  - [ ] `cargo check -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf` passes (struct compiles)

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: BoardIo struct compiles with f412 types
    Tool: Bash
    Preconditions: Task 1 changes applied to device/mod.rs
    Steps:
      1. Run: cargo check -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0 (warnings acceptable at this stage since BoardIo is defined but unused)
    Expected Result: Compilation succeeds
    Failure Indicators: Type mismatch errors between BoardIo fields and board type aliases
    Evidence: .sisyphus/evidence/task-1-boardio-f412-check.txt

  Scenario: BoardIo field names match DeviceContext exactly
    Tool: Bash (grep)
    Preconditions: Task 1 changes applied
    Steps:
      1. Run: grep -A20 'pub struct BoardIo' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs
      2. Run: grep -A20 'pub struct DeviceContext' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs
      3. Compare field names — must be identical (types may differ for button)
    Expected Result: All 10 fields present with matching names: delay, timer1, timer2, serial, sdio, disp, rng, touchscreen, i2c, button
    Failure Indicators: Missing fields, different field names, wrong field count
    Evidence: .sisyphus/evidence/task-1-boardio-fields.txt
  ```

  **Commit**: YES (groups with Task 2)
  - Message: `refactor(device): define BoardIo struct and Button type aliases`
  - Files: `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`
  - Pre-commit: `cargo check -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf`

- [x] 2. Add Button type alias to all board modules

  **What to do**:
  - In `device/f412.rs`, add: `pub type Button = PA0<Input>;` (PA0 is the user button on STM32F412G-DISCO)
  - In `device/f413.rs`, add: `pub type Button = PA0<Input>;` (PA0 is the user button on STM32F413H-DISCO)
  - In `device/f469.rs`, add: `pub type Button = PA0<Input>;` (PA0 is the user button on STM32F469I-DISCO)
  - Ensure `PA0` and `Input` are properly imported in each module
  - This aligns with the `board_abstraction_pattern.json` requirement for `Button` type alias in each board module

  **Must NOT do**:
  - Do not add any other type aliases yet
  - Do not modify function bodies

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple type alias addition to 3 files — mechanical change
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 1, 3)
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Tasks 4, 5, 6
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs:1-20` — Existing type aliases section. Add `Button` after the existing aliases.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:377` — Current usage: `pub button: PA0<Input>` — this is the type to alias.
  - `board_abstraction_pattern.json:105-107` — Button type alias requirement.

  **WHY Each Reference Matters**:
  - f412.rs aliases section: Must place Button alias alongside existing I2C, DisplayInner aliases for consistency
  - mod.rs button field: Confirms the concrete type that the alias must point to
  - JSON spec: Authoritative requirement that Button must be a board-exported type alias

  **Acceptance Criteria**:
  - [ ] `pub type Button` exists in f412.rs, f413.rs, f469.rs
  - [ ] All three targets pass `cargo check`

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Button type alias present in all board modules
    Tool: Bash (grep)
    Preconditions: Task 2 changes applied
    Steps:
      1. Run: grep 'pub type Button' validating-lightning-signer/vls-signer-stm32/src/device/f412.rs
      2. Run: grep 'pub type Button' validating-lightning-signer/vls-signer-stm32/src/device/f413.rs
      3. Run: grep 'pub type Button' validating-lightning-signer/vls-signer-stm32/src/device/f469.rs
      4. Assert: All three return exactly one match
    Expected Result: Three matches, one per file
    Failure Indicators: Zero or multiple matches in any file
    Evidence: .sisyphus/evidence/task-2-button-aliases.txt
  ```

  **Commit**: YES (groups with Task 1)
  - Message: `refactor(device): define BoardIo struct and Button type aliases`
  - Files: `device/f412.rs`, `device/f413.rs`, `device/f469.rs`
  - Pre-commit: `cargo check` for one target

- [x] 3. (SKIPPED HARDENING) BSP: Refactor sdio.rs to internalize SdramRemainders

  **Status**: OPTIONAL — not on critical path. The primary fix (downgrading `pub use` to `use` in f469.rs line 20) is handled by Task 6. This task is additional BSP API hardening.

  **What to do**:
  - Currently `stm32f469i-disc/src/sdio.rs` `init()` function takes `SdramRemainders` as a parameter. While Task 6 prevents the type from leaking to `device/mod.rs` (by hiding it inside `init_board_io()`), the BSP still exposes it in its public API.
  - **If pursued**: Refactor the BSP to provide a higher-level init function that internally sequences: `split_sdram_pins()` → `init_display_pipeline()` → `sdio::init()` → returns `(Display, Sdio<SdCard>, PC1)` without exposing `SdramRemainders` in any public signature.
  - **Skip condition**: If all three targets compile cleanly after Tasks 1-2 and 4-8, this task can be deferred to a follow-up PR.
  - The existing BSP API works fine — f469.rs just calls the functions in sequence internally. This task is about making the BSP's *own* API cleaner.

  **Must NOT do**:
  - Do not change SDRAM initialization logic (just restructure plumbing)
  - Do not modify BSP display rendering behavior
  - Do not change pin assignments
  - Do not block other tasks on this — it is NOT a dependency for Task 6

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires understanding the full BSP init chain (sdram → display → sdio → touch) and restructuring without breaking anything
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 1, 2) — but can also be skipped entirely
  - **Parallel Group**: Wave 1 (optional participant)
  - **Blocks**: Nothing (no task depends on this)
  - **Blocked By**: None

  **References** (CRITICAL):

  **Pattern References**:
  - `stm32f469i-disc/src/sdram.rs:83-101` — `SdramRemainders` struct definition. Lists remainder pins: PC1, PC8-PC12, PD2.
  - `stm32f469i-disc/src/sdram.rs:115-227` — `split_sdram_pins()` function. Takes 7 GPIO port Parts, returns `(sdram_pins, SdramRemainders, PH7)`.
  - `stm32f469i-disc/src/lcd.rs` — `init_display_pipeline()`. Receives SDRAM + PH7, passes remainders through.
  - `stm32f469i-disc/src/sdio.rs` — `init()`. Takes `SdramRemainders`, extracts SDIO pins (PC8-12, PD2), returns `(Sdio<SdCard>, PC1)`.

  **WHY Each Reference Matters**:
  - `SdramRemainders`: The type being internalized. Must understand its fields to route them.
  - BSP init chain: `split_sdram_pins` → `init_display_pipeline` → `sdio::init` → touch. Must happen in this order.

  **Acceptance Criteria**:
  - [ ] BSP compiles: `cargo check -p stm32f469i-disc`
  - [ ] F469 VLS target still compiles after BSP change
  - [ ] SdramRemainders not in any new public API signature

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: BSP compiles after optional refactoring
    Tool: Bash
    Preconditions: Task 3 changes applied to BSP source
    Steps:
      1. Run: cargo check -p stm32f469i-disc 2>&1
      2. Assert: Exit code 0
      3. Run: cargo check -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf 2>&1
      4. Assert: Exit code 0
    Expected Result: Both BSP and VLS F469 target compile
    Failure Indicators: Type errors from restructured init chain
    Evidence: .sisyphus/evidence/task-3-bsp-compile.txt
  ```

  **Commit**: YES (if pursued)
  - Message: `refactor(bsp): internalize SdramRemainders in board peripheral init`
  - Files: `stm32f469i-disc/src/sdio.rs`, `stm32f469i-disc/src/lib.rs`
  - Pre-commit: `cargo check -p stm32f469i-disc`

- [x] 4. Implement init_board_io() for f412.rs

  **What to do**:
  - Move ALL F412-specific initialization from `make_devices()` in mod.rs into `pub fn init_board_io()` in f412.rs
  - The function signature: `pub fn init_board_io(dp: pac::Peripherals, cp: cortex_m::Peripherals) -> BoardIo`
  - Inside `init_board_io()`, perform:
    1. RCC/clock configuration (currently in shared make_devices)
    2. GPIO port splitting (GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOG as needed)
    3. SysTick delay creation
    4. Timer setup (timer1, timer2)
    5. Display initialization (call existing `init_display()` or equivalent in f412.rs)
    6. I2C initialization for touch
    7. Touch controller initialization (ft6x06)
    8. SDIO initialization
    9. USB serial initialization (SerialDriver)
    10. Button pin configuration
    11. RNG initialization
  - Return `BoardIo` with all fields populated
  - The existing `init_display()` function in f412.rs can be called from within `init_board_io()`
  - Remove the F412-specific `#[cfg]` block logic from `make_devices()` — but don't modify mod.rs yet (Task 7 does that)
  - Actually: don't remove from mod.rs — just ADD the new init_board_io(). mod.rs cleanup is Task 7.

  **Must NOT do**:
  - Do not modify mod.rs (that's Task 7)
  - Do not change F412's display initialization logic
  - Do not change pin assignments or clock configuration

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Substantial code movement requiring careful type tracking across board-specific peripherals
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 5, 6)
  - **Parallel Group**: Wave 2 (with Tasks 5, 6)
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 1, 2 (needs BoardIo struct definition and Button alias)

  **References** (CRITICAL):

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:382-556` — Current `make_devices()`. The F412 path is the `#[cfg(feature = "stm32f412")]` blocks within this function. Extract ALL F412-specific code into f412.rs.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs` — Current f412 module (182 lines). Already has `init_display()`, type aliases. Add `init_board_io()` here.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:1-50` — Imports section. Shows what crates/modules are imported for peripherals — f412.rs will need many of these.

  **API/Type References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:367-378` — `DeviceContext` struct. BoardIo must provide fields to construct this.
  - `validating-lightning-signer/vls-signer-stm32/src/usbserial.rs:1-50` — `SerialDriver::new()` signature. Need to understand what it takes to create SerialDriver.

  **WHY Each Reference Matters**:
  - `make_devices()`: The source of truth for what F412 initialization actually does. Every line in the F412 cfg path must move to f412.rs.
  - `f412.rs`: Destination file. Must understand existing structure to integrate init_board_io() cleanly.
  - `DeviceContext`: Validation target — BoardIo fields must enable constructing DeviceContext.
  - `usbserial.rs`: SerialDriver creation may require specific USB peripheral types.

  **Acceptance Criteria**:
  - [ ] `init_board_io()` function exists in f412.rs
  - [ ] Returns `BoardIo` with all 10 fields populated
  - [ ] `cargo check -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf` passes

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: F412 init_board_io compiles
    Tool: Bash
    Preconditions: Tasks 1, 2, 4 changes applied
    Steps:
      1. Run: cargo check -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0
    Expected Result: F412 target compiles with init_board_io()
    Failure Indicators: Type mismatches, missing imports, wrong field types
    Evidence: .sisyphus/evidence/task-4-f412-check.txt

  Scenario: init_board_io returns all required fields
    Tool: Bash (grep)
    Preconditions: Task 4 changes applied
    Steps:
      1. Run: grep -c 'BoardIo' validating-lightning-signer/vls-signer-stm32/src/device/f412.rs
      2. Assert: At least 2 matches (struct usage + return)
      3. Run: grep 'pub fn init_board_io' validating-lightning-signer/vls-signer-stm32/src/device/f412.rs
      4. Assert: Exactly 1 match
    Expected Result: Function exists and returns BoardIo
    Failure Indicators: Missing function, wrong return type
    Evidence: .sisyphus/evidence/task-4-f412-init-fn.txt
  ```

  **Commit**: YES (groups with Tasks 5, 6)
  - Message: `refactor(device): implement init_board_io() for all boards`
  - Files: `device/f412.rs`
  - Pre-commit: `cargo check` for f412 target

- [x] 5. Implement init_board_io() for f413.rs

  **What to do**:
  - Same as Task 4, but for F413. The F413 is very similar to F412 (nearly identical pin assignments with minor differences).
  - Move ALL F413-specific initialization from `make_devices()` into `pub fn init_board_io()` in f413.rs
  - Signature: `pub fn init_board_io(dp: pac::Peripherals, cp: cortex_m::Peripherals) -> BoardIo`
  - Same init sequence as Task 4: RCC, GPIO split, delay, timers, display, I2C, touch, SDIO, USB serial, button, RNG
  - F413 has slightly different GPIO port usage vs F412 — follow existing f413.rs patterns

  **Must NOT do**:
  - Do not modify mod.rs
  - Do not copy-paste from f412.rs without verifying pin differences
  - Do not change F413's display initialization logic

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Same complexity as Task 4, different board
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 4, 6)
  - **Parallel Group**: Wave 2 (with Tasks 4, 6)
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 1, 2

  **References** (CRITICAL):

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:382-556` — Current `make_devices()`. The F413 path is the `#[cfg(feature = "stm32f413")]` blocks.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs` — Current f413 module (181 lines). Very similar to f412.rs.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs` — Reference for comparison. F413 differs in: FSMC vs FMC for display, some GPIO ports.

  **WHY Each Reference Matters**:
  - `make_devices()`: Source of truth for F413 init. Must move all F413-specific code.
  - `f413.rs`: Destination file. Already has display init logic.
  - `f412.rs`: Comparison — diff against f413.rs to identify exactly where they diverge.

  **Acceptance Criteria**:
  - [ ] `init_board_io()` function exists in f413.rs
  - [ ] Returns `BoardIo` with all 10 fields populated
  - [ ] `cargo check -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf` passes

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: F413 init_board_io compiles
    Tool: Bash
    Preconditions: Tasks 1, 2, 5 changes applied
    Steps:
      1. Run: cargo check -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0
    Expected Result: F413 target compiles
    Failure Indicators: Type mismatches from F413-specific pin/peripheral differences
    Evidence: .sisyphus/evidence/task-5-f413-check.txt
  ```

  **Commit**: YES (groups with Tasks 4, 6)
  - Message: `refactor(device): implement init_board_io() for all boards`
  - Files: `device/f413.rs`
  - Pre-commit: `cargo check` for f413 target

- [x] 6. Implement init_board_io() for f469.rs

  **What to do**:
  - This is the most complex board init because it involves the BSP and the SdramRemainders plumbing.
  - Move ALL F469-specific initialization from `make_devices()` into `pub fn init_board_io()` in f469.rs
  - Signature: `pub fn init_board_io(dp: pac::Peripherals, cp: cortex_m::Peripherals) -> BoardIo`
  - Inside `init_board_io()`, perform:
    1. RCC/clock configuration
    2. GPIO port splitting (all ports needed for SDRAM + other peripherals)
    3. Call BSP functions in sequence: `split_sdram_pins()` → `init_display_pipeline()` → `sdio::init()` — consuming `SdramRemainders` internally within this function
    4. SysTick delay creation
    5. Timer setup
    6. I2C initialization for touch (using BSP's I2C setup or direct HAL)
    7. Touch controller initialization with the touch_int_pin (PC1) from SDIO init
    8. USB serial initialization
    9. Button pin configuration
    10. RNG initialization
  - **CRITICAL first step**: Change line 20 of f469.rs from `pub use stm32f469i_disc::sdram::SdramRemainders;` to `use stm32f469i_disc::sdram::SdramRemainders;` (remove `pub`). This is the primary fix that prevents the BSP type from leaking to mod.rs.
  - **Critical**: SdramRemainders must be consumed entirely within this function. The init sequence inside f469.rs is: `split_sdram_pins()` → `init_display_pipeline()` → `sdio::init()` → touch uses PC1 from sdio result. It must NEVER appear in the function's return type or in mod.rs.
  - The BSP's `stm32f469i_disc` import stays in f469.rs (it's board-specific, not shared)

  **Must NOT do**:
  - Do not expose SdramRemainders in init_board_io()'s return type
  - Do not modify mod.rs (Task 7)
  - Do not change display rendering behavior
  - Do not change SDRAM initialization parameters

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Most complex board init — involves BSP integration, SDRAM pin splitting, and careful type threading. Must understand full init chain.
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 4, 5)
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 1, 2 (needs BoardIo struct and Button alias)

  **References** (CRITICAL):

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:382-556` — Current `make_devices()`. The F469 path involves `#[cfg(feature = "stm32f469")]` blocks including display pipeline, SDRAM, SDIO, touch with PC1.
  - `validating-lightning-signer/vls-signer-stm32/src/device/f469.rs` — Current f469 module (117 lines). Already has `init_display()` that calls BSP's `init_display_pipeline()`.
  - `stm32f469i-disc/src/sdram.rs:83-101` — `SdramRemainders` definition. Must be consumed within f469.rs, never exposed.
  - `stm32f469i-disc/src/sdio.rs` — SDIO init. Takes `SdramRemainders`, returns `(Sdio<SdCard>, PC1)`. Use existing API as-is (Task 3 is optional).
  - `stm32f469i-disc/src/lcd.rs` — Display pipeline. Returns `(Display, SdramRemainders)` — remainders are consumed by sdio::init().
  - `stm32f469i-disc/src/touch.rs` — Touch init. Needs PC1 from the remainder/SDIO plumbing.

  **API/Type References**:
  - `board_abstraction_pattern.json:133-166` — BSP integration guidance section. Core rule: "Treat the BSP as an implementation detail hidden behind the same narrow waist API."
  - `stm32f469i-disc/src/lib.rs` — BSP public API. Use existing exports (Task 3 is optional hardening).

  **WHY Each Reference Matters**:
  - `make_devices()`: F469 path is the most complex with SDRAM, display pipeline, SDIO chain. Every step must move here.
  - `f469.rs`: Destination file. Already partially implemented — extend, don't rewrite from scratch.
  - `SdramRemainders`: THE type that must not leak. Verify it's consumed within f469.rs.
  - BSP modules: Use the existing BSP API. Task 3 (optional hardening) is NOT required — f469.rs calls BSP functions directly in sequence.
  - `board_abstraction_pattern.json`: Authoritative guidance on how BSP should be hidden.

  **Acceptance Criteria**:
  - [ ] `init_board_io()` function exists in f469.rs
  - [ ] Returns `BoardIo` with all 10 fields populated
  - [ ] `SdramRemainders` does NOT appear in f469.rs's public API (it may appear internally)
  - [ ] `cargo check -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf` passes

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: F469 init_board_io compiles with BSP integration
    Tool: Bash
    Preconditions: Tasks 1, 2, 6 changes applied
    Steps:
      1. Run: cargo check -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0
    Expected Result: F469 target compiles with full BSP integration hidden behind BoardIo
    Failure Indicators: Type mismatches between BSP types and BoardIo fields, SdramRemainders leaking
    Evidence: .sisyphus/evidence/task-6-f469-check.txt

  Scenario: SdramRemainders not in public f469 API
    Tool: Bash (grep)
    Preconditions: Task 6 changes applied
    Steps:
      1. Run: grep 'SdramRemainders' validating-lightning-signer/vls-signer-stm32/src/device/f469.rs
      2. If matches found, verify they are only in function bodies (private), not in pub fn signatures or return types
      3. Run: grep 'SdramRemainders' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs
      4. Assert: Zero matches in mod.rs
    Expected Result: SdramRemainders may appear internally in f469.rs but never in mod.rs
    Failure Indicators: SdramRemainders in mod.rs or in pub fn signature in f469.rs
    Evidence: .sisyphus/evidence/task-6-no-remainder-leak.txt
  ```

  **Commit**: YES (groups with Tasks 4, 5)
  - Message: `refactor(device): implement init_board_io() for all boards`
  - Files: `device/f469.rs`
  - Pre-commit: `cargo check` for f469 target

- [x] 7. Refactor make_devices() to use BoardIo narrow waist

  **What to do**:
  - This is the keystone task. Rewrite `make_devices()` in `device/mod.rs` from its current ~170 lines of `#[cfg]` branching to a clean ~20-30 line function:
    ```rust
    pub fn make_devices() -> DeviceContext {
        let dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();
        let board = board::init_board_io(dp, cp);
        DeviceContext {
            delay: board.delay,
            timer1: board.timer1,
            timer2: board.timer2,
            serial: board.serial,
            sdio: board.sdio,
            disp: board.disp,
            rng: board.rng,
            touchscreen: board.touchscreen,
            i2c: board.i2c,
            button: board.button,
        }
    }
    ```
  - Remove ALL `#[cfg(feature = "stm32f4xx")]` blocks within `make_devices()` body
  - The only `#[cfg]` remaining in mod.rs should be the board module selection at the top (Pattern A — matching existing code):
    ```rust
    #[cfg(feature = "stm32f412")]
    mod f412;
    #[cfg(feature = "stm32f412")]
    use f412 as board;
    // ... same for f413, f469

    // Expand existing pub use to include Button:
    pub use board::{
        transform_touch_coords, TouchInterruptPin, CHOICE_BUTTON_POSITIONS,
        CHOICE_TEXT_POSITIONS, CHOICE_TOUCH_POSITIONS, I2C, SCREEN_HEIGHT,
        SCREEN_WIDTH, VCENTER_PIX,
        Button,  // NEW from Task 2
    };
    ```
  - Remove any imports that were only needed for per-board init code (e.g., GPIO types, BSP imports)
  - Keep `DeviceContext` struct definition unchanged
  - Keep any shared utility functions that aren't board-specific
  - The `board::init_board_io` call works because the `use f412 as board` alias (Pattern A) makes all board functions available via `board::`

  **Must NOT do**:
  - Do not change `DeviceContext` struct fields
  - Do not modify `setup.rs`
  - Do not remove board module declarations
  - Do not change function signature of `make_devices()` (same return type)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Highest-risk task — rewriting the central init function. Must carefully preserve all behavior while eliminating complexity.
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (depends on all board modules being complete)
  - **Parallel Group**: Wave 3 (sequential after Wave 2)
  - **Blocks**: Tasks 8, 9
  - **Blocked By**: Tasks 4, 5, 6 (all board modules must have init_board_io() implemented)

  **References** (CRITICAL):

  **Pattern References**:
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:382-556` — Current `make_devices()` in its entirety. This is what you're replacing. Study every line to ensure nothing is lost.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:1-100` — Import section. Many imports can be removed after refactoring (board-specific GPIO types, BSP imports).
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:367-378` — `DeviceContext` struct. Must remain UNCHANGED.
  - `board_abstraction_pattern.json:168-175` — `shared_code_rules` section: cfg_policy and return_shapes.

  **API/Type References**:
  - `validating-lightning-signer/vls-signer-stm32/src/setup.rs` — Primary consumer of `make_devices()`. Do NOT modify. Read to confirm it only uses DeviceContext fields.

  **WHY Each Reference Matters**:
  - `make_devices()`: You are replacing this. Must account for every initialization step — nothing can be silently dropped.
  - Imports: Cleanup opportunity — board-specific imports move to board modules. But be careful not to remove imports still used by other functions in mod.rs.
  - `DeviceContext`: Preservation target. Fields must not change.
  - `setup.rs`: Regression check. If setup.rs compiles, DeviceContext contract is preserved.
  - JSON shared_code_rules: Authoritative rules for what cfg is allowed in shared code.

  **Acceptance Criteria**:
  - [ ] `make_devices()` is <40 lines (down from ~170)
  - [ ] Zero `#[cfg]` inside `make_devices()` body
  - [ ] `DeviceContext` struct fields unchanged
  - [ ] `setup.rs` requires zero modifications
  - [ ] All three targets compile: f412, f413, f469

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All three targets compile after make_devices refactor
    Tool: Bash
    Preconditions: Tasks 1-7 all applied
    Steps:
      1. Run: cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0
      3. Run: cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf 2>&1
      4. Assert: Exit code 0
      5. Run: cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf 2>&1
      6. Assert: Exit code 0
    Expected Result: All three targets build successfully with release optimizations
    Failure Indicators: Missing peripherals, type mismatches, unresolved imports
    Evidence: .sisyphus/evidence/task-7-all-targets-build.txt

  Scenario: No cfg sprawl in make_devices()
    Tool: Bash (grep)
    Preconditions: Task 7 changes applied
    Steps:
      1. Extract make_devices function body (between function signature and closing brace)
      2. Run: grep -c '#\[cfg' in the function body
      3. Assert: 0 matches
    Expected Result: Zero cfg attributes inside make_devices() body
    Failure Indicators: Any #[cfg] inside function body
    Evidence: .sisyphus/evidence/task-7-no-cfg-sprawl.txt

  Scenario: DeviceContext struct unchanged
    Tool: Bash (git diff)
    Preconditions: Task 7 changes applied
    Steps:
      1. Run: git diff HEAD -- validating-lightning-signer/vls-signer-stm32/src/device/mod.rs | grep -A5 -B5 'DeviceContext'
      2. Assert: DeviceContext struct definition has NO changes (only usage changes in make_devices)
    Expected Result: DeviceContext struct definition is identical to pre-refactor
    Failure Indicators: Any field additions, removals, or type changes in DeviceContext
    Evidence: .sisyphus/evidence/task-7-devicecontext-unchanged.txt

  Scenario: setup.rs has zero changes
    Tool: Bash (git diff)
    Preconditions: All tasks applied
    Steps:
      1. Run: git diff HEAD -- validating-lightning-signer/vls-signer-stm32/src/setup.rs
      2. Assert: Empty diff (no changes)
    Expected Result: setup.rs is completely untouched
    Failure Indicators: Any diff output
    Evidence: .sisyphus/evidence/task-7-setup-untouched.txt
  ```

  **Commit**: YES (groups with Task 8)
  - Message: `refactor(device): replace cfg sprawl with BoardIo narrow waist`
  - Files: `device/mod.rs`
  - Pre-commit: `cargo build --release` for all three targets

- [x] 8. Add compile-time guards for board feature selection

  **What to do**:
  - Add compile-time checks to `device/mod.rs` that enforce exactly one board feature is enabled:
    ```rust
    #[cfg(not(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f469")))]
    compile_error!("Enable exactly one board feature: stm32f412 | stm32f413 | stm32f469");
    
    #[cfg(any(
        all(feature = "stm32f412", feature = "stm32f413"),
        all(feature = "stm32f412", feature = "stm32f469"),
        all(feature = "stm32f413", feature = "stm32f469"),
    ))]
    compile_error!("Enable only one board feature at a time.");
    ```
  - Place these at the top of `device/mod.rs`, after imports but before module declarations
  - This directly implements Step 7 from `board_abstraction_pattern.json`

  **Must NOT do**:
  - Do not change any other code in mod.rs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Copy-paste from spec, 10 lines of code
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (depends on Task 7 completing first to avoid merge conflicts)
  - **Parallel Group**: Wave 3 (after Task 7)
  - **Blocks**: Task 9
  - **Blocked By**: Task 7

  **References**:

  **Pattern References**:
  - `board_abstraction_pattern.json:213-226` — `compile_time_guards` section. Contains the exact snippet to use.
  - `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs:1-30` — Top of mod.rs. Place guards here.

  **WHY Each Reference Matters**:
  - JSON spec: Contains the exact guards to implement. Copy verbatim.
  - mod.rs top: Placement location. Must go before module declarations.

  **Acceptance Criteria**:
  - [ ] Both `compile_error!` macros present in mod.rs
  - [ ] Building without any board feature fails with clear error message
  - [ ] Building with one board feature succeeds

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: No-feature build fails with clear error
    Tool: Bash
    Preconditions: Task 8 changes applied
    Steps:
      1. Run: cargo check -p vls-signer-stm32 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code non-zero
      3. Assert: Output contains "Enable exactly one board feature"
    Expected Result: Build fails with helpful compile_error message
    Failure Indicators: Build succeeds (guards not working) or different error message
    Evidence: .sisyphus/evidence/task-8-no-feature-guard.txt

  Scenario: Single-feature build succeeds
    Tool: Bash
    Preconditions: Task 8 changes applied
    Steps:
      1. Run: cargo check -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0
    Expected Result: Build succeeds normally with one feature
    Failure Indicators: Guard accidentally blocks valid builds
    Evidence: .sisyphus/evidence/task-8-single-feature-ok.txt
  ```

  **Commit**: YES (groups with Task 7)
  - Message: `refactor(device): replace cfg sprawl with BoardIo narrow waist`
  - Files: `device/mod.rs`
  - Pre-commit: `cargo check` with each feature individually

- [x] 9. Full cross-compilation verification

  **What to do**:
  - Run full `cargo build --release` for all three targets to verify the complete refactoring
  - This is a verification-only task — no code changes
  - Build each target and capture output as evidence
  - Also run the BSP crate build to verify it still compiles
  - Check binary sizes haven't changed dramatically (major size changes indicate missing/extra code)

  **Must NOT do**:
  - Do not make any code changes in this task

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Verification task requiring compilation of embedded targets
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (must run after all implementation tasks)
  - **Parallel Group**: Wave 4 (sequential, after Wave 3)
  - **Blocks**: Final Verification Wave
  - **Blocked By**: Tasks 7, 8

  **References**:

  **Pattern References**:
  - `board_abstraction_pattern.json:242-248` — Acceptance criteria section. Verify all items.

  **Acceptance Criteria**:
  - [ ] F412 target builds: `cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf`
  - [ ] F413 target builds: `cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf`
  - [ ] F469 target builds: `cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf`
  - [ ] BSP builds: `cargo check -p stm32f469i-disc`

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All targets build with release optimizations
    Tool: Bash
    Preconditions: All Tasks 1-8 complete
    Steps:
      1. Run: cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf 2>&1
      2. Assert: Exit code 0, capture binary size
      3. Run: cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf 2>&1
      4. Assert: Exit code 0, capture binary size
      5. Run: cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf 2>&1
      6. Assert: Exit code 0, capture binary size
      7. Run: cargo check -p stm32f469i-disc 2>&1
      8. Assert: Exit code 0
    Expected Result: All 4 builds succeed
    Failure Indicators: Any build failure
    Evidence: .sisyphus/evidence/task-9-all-builds.txt

  Scenario: No BSP leakage in shared code
    Tool: Bash (grep)
    Preconditions: All tasks complete
    Steps:
      1. Run: grep -c 'SdramRemainders' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs
      2. Assert: Output is "0"
      3. Run: grep -c 'stm32f469i_disc' validating-lightning-signer/vls-signer-stm32/src/device/mod.rs
      4. Assert: Output is "0"
      5. Run: grep -l 'pub fn init_board_io' validating-lightning-signer/vls-signer-stm32/src/device/f4*.rs | wc -l
      6. Assert: Output is "3"
    Expected Result: Zero BSP leaks in shared code, init_board_io in all 3 board modules
    Failure Indicators: Non-zero leak counts, missing init_board_io in any board module
    Evidence: .sisyphus/evidence/task-9-leakage-check.txt

  Scenario: setup.rs and sdcard.rs untouched
    Tool: Bash (git diff)
    Preconditions: All tasks complete
    Steps:
      1. Run: git diff --name-only HEAD 2>/dev/null | grep -E '(setup\.rs|sdcard\.rs)' | wc -l
      2. Assert: Output is "0"
    Expected Result: Neither file was modified
    Failure Indicators: Any changes to setup.rs or sdcard.rs
    Evidence: .sisyphus/evidence/task-9-scope-check.txt
  ```

  **Commit**: NO (verification only)

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, grep for struct/function). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan. Read `board_abstraction_pattern.json` and verify all 7 steps are addressed.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo build` for all three targets. Review all changed files for: unnecessary `unsafe`, dead code, unused imports, `#[allow(...)]` hacks. Check for AI slop: excessive comments, over-abstraction, generic names (data/result/item/temp). Verify consistent code style with existing codebase.
  Output: `Build [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [x] F3. **BSP Leak and Cfg Sprawl Verification** — `unspecified-high`
  Grep for `SdramRemainders` in all VLS files — must only appear in f469.rs. Grep for `stm32f469i_disc` in device/mod.rs — must have zero matches. Count `#[cfg(` occurrences in device/mod.rs — compare before/after, must be significantly reduced. Verify `BoardIo` struct exists in `device/mod.rs` (defined once, not per-board). Verify `init_board_io()` exists in each board module (f412.rs, f413.rs, f469.rs).
  Output: `BSP Leak [CLEAN/N leaks] | Cfg Count [before→after] | BoardIo [mod.rs: YES/NO] | init_board_io [N/3 boards] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT do" compliance: setup.rs unchanged, sdcard.rs unchanged, no new dependencies. Detect cross-task contamination: Task N touching Task M's files. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| After Task(s) | Commit Message | Files | Pre-commit Check |
|---------------|---------------|-------|------------------|
| 1, 2 | `refactor(device): define BoardIo struct and Button type aliases` | `device/mod.rs`, `device/f412.rs`, `device/f413.rs`, `device/f469.rs` | `cargo check` for one target |
# Commit strategy row for Task 3 is conditional (optional)
| 4, 5, 6 | `refactor(device): implement init_board_io() for all boards` | `device/f412.rs`, `device/f413.rs`, `device/f469.rs` | `cargo check` per target |
| 7, 8 | `refactor(device): replace cfg sprawl with BoardIo narrow waist` | `device/mod.rs` | `cargo build --release` all targets |
| 9 | (no commit — verification only) | — | — |
| F1-F4 | (no commit — review only) | — | — |

---

## Success Criteria

### Verification Commands
```bash
# All three targets compile
cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf
cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf
cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf

# No BSP leakage in shared code
grep -c "SdramRemainders" validating-lightning-signer/vls-signer-stm32/src/device/mod.rs  # Expected: 0
grep -c "stm32f469i_disc" validating-lightning-signer/vls-signer-stm32/src/device/mod.rs  # Expected: 0

# BoardIo defined once in mod.rs
grep -c "pub struct BoardIo" validating-lightning-signer/vls-signer-stm32/src/device/mod.rs  # Expected: 1
grep -l "pub struct BoardIo" validating-lightning-signer/vls-signer-stm32/src/device/mod.rs  # Expected: 1 match in mod.rs

# init_board_io exists in each board module
grep -l "pub fn init_board_io" validating-lightning-signer/vls-signer-stm32/src/device/f4*.rs  # Expected: 3 files

# DeviceContext unchanged
grep -A20 "pub struct DeviceContext" validating-lightning-signer/vls-signer-stm32/src/device/mod.rs  # Same fields as before

# setup.rs unchanged
git diff --name-only | grep -c "setup.rs"  # Expected: 0
```

### Final Checklist
- [ ] All "Must Have" items present
- [ ] All "Must NOT Have" items absent
- [ ] All three targets compile with `--release`
- [ ] DeviceContext struct fields identical to pre-refactor
- [ ] setup.rs has zero changes
- [ ] sdcard.rs has zero changes
