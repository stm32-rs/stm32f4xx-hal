# F412/F413 Mipidsi Migration

## TL;DR

> **Quick Summary**: Fix F412 and F413 compilation by migrating from deprecated `st7789` crate to `mipidsi v0.7.1`, fixing the gpiod scoping bug in SDIO init, and correcting the hardcoded RAM_SIZE/HEAP_SIZE that is wrong for F412 (256K RAM, not 320K).
>
> **Deliverables**:
> - F412 test binary compiles cleanly
> - F413 test binary compiles cleanly
> - F469 test binary still compiles (regression check)
> - Changes committed to VLS repo
>
> **Estimated Effort**: Low (5 files, straightforward API migration + bug fixes)
> **Parallel Execution**: NO — sequential (Cargo.toml → f412.rs → f413.rs → mod.rs fixes → verify)
> **Critical Path**: Update deps → Migrate f412 → Migrate f413 → Fix mod.rs (gpiod + RAM_SIZE) → Build verify

---

## Context

### Root Cause Analysis

The F412 and F413 boards fail to compile with 9 errors:
- **E0425**: `gpiod` not in scope at line 369 (SDIO block references gpiod before it's split)
- **E0599**: No `clear` method on ST7789 (method moved to DrawTarget trait)
- **E0277** (7x): `ST7789<...>` doesn't implement `DrawTarget` from embedded-graphics 0.8

The root cause is **dependency version conflict**:
- `st7789 v0.7.0` depends on `embedded-graphics-core v0.3.3`
- `embedded-graphics v0.8.x` requires `embedded-graphics-core v0.4.x`
- Two incompatible versions of `DrawTarget` trait exist in the dependency graph

### Solution

Replace `st7789` with `mipidsi v0.7.1`:
- Uses `embedded-graphics-core v0.4.0` ✅
- Uses `display-interface v0.4.1` ✅ (matches HAL FSMC LCD output)
- Uses `embedded-hal v0.2.7` ✅ (HAL implements both eh 0.2 and 1.0)
- `SysDelay` implements `DelayUs<u32>` from eh 0.2 ✅

### API Migration Reference

| Aspect | st7789 v0.7.0 | mipidsi v0.7.1 |
|--------|---------------|----------------|
| Import | `st7789::{Orientation, ST7789}` | `mipidsi::{Builder, Orientation, models::ST7789}` |
| Constructor | `ST7789::new(di, Some(rst), Some(bl), 240, 240)` | `Builder::st7789(di).with_display_size(240, 240)` |
| Init | `disp.init(delay)` | `.init(&mut delay, Some(rst))` |
| Orientation | `disp.set_orientation(Orientation::Portrait)` | `.with_orientation(Orientation::Portrait(false))` before init |
| Type | `ST7789<DI, RST, BL>` (3 type params) | `Display<DI, ST7789, RST>` (no BL in type) |
| Backlight | Managed internally | Managed externally (set_high after init) |
| Portrait | `Orientation::Portrait` | `Orientation::Portrait(false)` |
| PortraitSwapped | `Orientation::PortraitSwapped` | `Orientation::PortraitInverted(false)` |

### Guardrails

- **MUST NOT**: Modify F469 code (it doesn't use st7789/mipidsi)
- **MUST NOT**: Change display behavior (same 240x240 ST7789 LCD)
- **MUST NOT**: Add new features — only fix compilation and correct existing bugs
- **MUST NOT**: Change any HAL code
- **MUST NOT**: Work on demo_signer — test binary only

---

## Work Objectives

### Definition of Done
- [x] F412 test binary compiles: `cargo build --no-default-features --features stm32f412 --release --bin test` exits 0
- [x] F413 test binary compiles: `cargo build --no-default-features --features stm32f413 --release --bin test` exits 0
- [x] F469 test binary still compiles: `cargo build --features stm32f469 --release --bin test` exits 0
- [x] RAM_SIZE/HEAP_SIZE correct per-board (F412: 256K/222K, F413/F469: 320K/286K)
- [x] Changes committed to VLS repo
---

## TODOs

- [x] 1. Update Cargo.toml: Replace st7789 with mipidsi

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/Cargo.toml`:
    - Line 18: Change `st7789 = { version = "0.7.0", default-features = false, features = ["graphics"], optional = true }` to `mipidsi = { version = "0.7.1", optional = true }`
    - Line 58: Change `dep:st7789` to `dep:mipidsi`
    - Line 59: Change `dep:st7789` to `dep:mipidsi`
  - The `display-interface` optional dep (line 15) can remain or be removed (mipidsi brings it transitively)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Acceptance Criteria**:
  - [ ] `grep -E "st7789|mipidsi" Cargo.toml` shows only `mipidsi` entries

  **Commit**: Groups with tasks 2-4

- [x] 2. Update f412.rs: Migrate to mipidsi API

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs`:
    - Line 18: Change `use st7789::{Orientation, ST7789};` to `use mipidsi::{Builder, Orientation, models::ST7789};`
    - Line 31: Change `pub type DisplayInner = ST7789<Lcd<LcdSubBank, u16>, LcdResetPin, BacklightPin>;` to `pub type DisplayInner = mipidsi::Display<Lcd<LcdSubBank, u16>, ST7789, LcdResetPin>;`
    - Lines 118-133: Rewrite display init:
      ```rust
      let lcd_reset = pd11.into_push_pull_output().speed(Speed::VeryHigh);
      let mut backlight_control = pf5.into_push_pull_output();

      // Touchscreen reset...
      let mut ts_reset = pf12.into_push_pull_output().speed(Speed::VeryHigh);
      long_hard_reset(&mut ts_reset, delay).expect("long hard reset");

      // FSMC LCD interface...
      let write_timing = Timing::default().data(3).address_setup(3).bus_turnaround(0);
      let read_timing = Timing::default().data(8).address_setup(8).bus_turnaround(0);

      info!("setup display");
      let (_fsmc, interface) = FsmcLcd::new(fsmc, lcd_pins, &read_timing, &write_timing, rcc);

      let mut disp = Builder::st7789(interface)
          .with_display_size(240, 240)
          .with_orientation(Orientation::Portrait(false))
          .init(delay, Some(lcd_reset))
          .unwrap();
      
      // Turn on backlight (managed externally in mipidsi)
      backlight_control.set_high();

      Display { inner: disp }
      ```

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Acceptance Criteria**:
  - [ ] No `st7789::` references in file
  - [ ] `DisplayInner` type uses `mipidsi::Display`
  - [ ] Backlight set high after init

  **Commit**: Groups with tasks 1, 3, 4

- [x] 3. Update f413.rs: Migrate to mipidsi API

  **What to do**:
  - Same pattern as f412.rs, but:
    - Use `Orientation::PortraitInverted(false)` instead of `Portrait(false)` (maps from st7789's `PortraitSwapped`)
    - F413 uses different pins (pb13 for reset, pe5 for backlight)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Acceptance Criteria**:
  - [ ] No `st7789::` references in file
  - [ ] Uses `PortraitInverted(false)` orientation

  **Commit**: Groups with tasks 1, 2, 4

- [x] 4. Fix mod.rs gpiod scoping for F412/F413 SDIO

  **What to do** (Option B - move SDIO inside board cfg blocks):
  - In `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`:
  
    1. **DELETE** the premature SDIO block (lines 357-372):
       ```rust
       // DELETE THIS ENTIRE BLOCK:
       #[cfg(not(feature = "stm32f469"))]
       let sdio: Sdio<SdCard> = { ... };
       ```
  
    2. **In F412 block** (inside the cfg block, after ts_int), ADD:
       ```rust
       let sdio: Sdio<SdCard> = {
           info!("SDIO setup");
           let d0 = gpioc.pc8.into_alternate().internal_pull_up(true);
           let d1 = gpioc.pc9.into_alternate().internal_pull_up(true);
           let d2 = gpioc.pc10.into_alternate().internal_pull_up(true);
           let d3 = gpioc.pc11.into_alternate().internal_pull_up(true);
           let clk = gpioc.pc12.into_alternate().internal_pull_up(false);
           let cmd = gpiod.pd2.into_alternate().internal_pull_up(true);  // F412 uses PD2
           Sdio::new(p.SDIO, (clk, cmd, d0, d1, d2, d3), &mut rcc)
       };
       ```
  
    3. **In F413 block** (inside the cfg block, after ts_int), ADD:
       ```rust
       let sdio: Sdio<SdCard> = {
           info!("SDIO setup");
           let d0 = gpioc.pc8.into_alternate().internal_pull_up(true);
           let d1 = gpioc.pc9.into_alternate().internal_pull_up(true);
           let d2 = gpioc.pc10.into_alternate().internal_pull_up(true);
           let d3 = gpioc.pc11.into_alternate().internal_pull_up(true);
           let clk = gpioc.pc12.into_alternate().internal_pull_up(false);
           let cmd = gpioa.pa6.into_alternate().internal_pull_up(true);  // F413 uses PA6!
           Sdio::new(p.SDIO, (clk, cmd, d0, d1, d2, d3), &mut rcc)
       };
       ```
  
    4. **Update return tuples**: Change `(disp, i2c, ts_int)` to `(disp, i2c, ts_int, sdio)` in both F412 and F413 blocks
    5. **Update F412/F413 let bindings**: Change `let (disp, mut i2c, ts_int) = {` to `let (disp, mut i2c, ts_int, sdio) = {`
    6. **Remove the F469-specific sdio binding** at line 318 if it exists as a separate declaration

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Acceptance Criteria**:
  - [ ] `gpiod.pd2` is accessible when used (not "cannot find value" error)
  - [ ] F412 uses `gpiod.pd2` for SDIO cmd
  - [ ] F413 uses `gpioa.pa6` for SDIO cmd

  **Commit**: Groups with tasks 1, 2, 3, 5

- [x] 5. Fix RAM_SIZE/HEAP_SIZE for F412 (latent memory bug)

  **What to do**:
  - In `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`:
    - Lines 85-88: Replace hardcoded constants with per-board cfg values:
      ```rust
      /// Total configured RAM size from `memory.x` (build.rs generates per-chip)
      #[cfg(feature = "stm32f412")]
      const RAM_SIZE: usize = 256 * 1024;
      #[cfg(not(feature = "stm32f412"))]
      const RAM_SIZE: usize = 320 * 1024;
      
      /// Size of the heap in bytes
      #[cfg(feature = "stm32f412")]
      pub const HEAP_SIZE: usize = 222 * 1024;  // 256K - ~34K data+stack
      #[cfg(not(feature = "stm32f412"))]
      pub const HEAP_SIZE: usize = 286 * 1024;  // 320K - ~34K data+stack
      ```
    - These values match what `build.rs` generates for `memory.x` (256K for F412, 320K for F413/F469)
    - The heap reduction (286K → 222K = 64K less) matches the RAM reduction (320K → 256K = 64K less)
    - Stack/data allocation remains the same ~34K across all boards

  **Context**:
  - `build.rs` already generates the correct `memory.x` with 256K for F412, but the Rust constant was wrong
  - `init_allocator()` uses RAM_SIZE to compute stack_size: `stack_size = RAM_SIZE - data_size - HEAP_SIZE`
  - With the old values on F412: 320K - data - 286K = would claim memory beyond physical RAM

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Acceptance Criteria**:
  - [ ] `RAM_SIZE` is 256K when `stm32f412` feature is active
  - [ ] `HEAP_SIZE` is 222K when `stm32f412` feature is active
  - [ ] F413/F469 values unchanged (320K/286K)
  - [ ] `init_allocator()` log message would show correct values per board

  **Commit**: Groups with tasks 1-4

- [x] 6. Verify all 3 targets compile

  **What to do**:
  - Run build for all 3 targets:
    ```bash
    cd validating-lightning-signer/vls-signer-stm32
    cargo build --no-default-features --features stm32f412 --release --bin test
    cargo build --no-default-features --features stm32f413 --release --bin test
    cargo build --features stm32f469 --release --bin test
    ```
  - All must exit with code 0

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Acceptance Criteria**:
  - [ ] F412 build exits 0
  - [ ] F413 build exits 0
  - [ ] F469 build exits 0

- [x] 7. Commit changes to VLS repo

  **What to do**:
  - Stage all changed files
  - Commit with message: `fix(vls): migrate F412/F413 from st7789 to mipidsi, fix RAM_SIZE`
  - Push to origin

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Commit Message**: `fix(vls): migrate F412/F413 from st7789 to mipidsi, fix RAM_SIZE`

---

## Files to Modify

| File | Changes |
|------|---------|
| `validating-lightning-signer/vls-signer-stm32/Cargo.toml` | Replace st7789 with mipidsi dep |
| `validating-lightning-signer/vls-signer-stm32/src/device/f412.rs` | Migrate to mipidsi API |
| `validating-lightning-signer/vls-signer-stm32/src/device/f413.rs` | Migrate to mipidsi API |
| `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs` | Fix gpiod scoping for SDIO + RAM_SIZE/HEAP_SIZE per-board |

---

## Verification Commands

```bash
# F412 compilation
cd validating-lightning-signer/vls-signer-stm32
cargo build --no-default-features --features stm32f412 --release --bin test
# Expected: exit code 0

# F413 compilation
cargo build --no-default-features --features stm32f413 --release --bin test
# Expected: exit code 0

# F469 regression check
cargo build --features stm32f469 --release --bin test
# Expected: exit code 0
```

---

## Success Criteria

- [x] F412 test binary compiles cleanly
- [x] F413 test binary compiles cleanly
- [x] F469 test binary still compiles (no regression)
- [x] RAM_SIZE/HEAP_SIZE correct for each board
- [x] All changes committed to VLS repo
- [x] No st7789 references remain in codebase
