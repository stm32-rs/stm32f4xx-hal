# USB Refactor Plan: Move USB Init into Board-Specific CFG Blocks

## Objective
Move USB initialization from shared code (lines 399-408) into each board's cfg block.
- **F469**: Use BSP `stm32f469i_disc::usb::init()`
- **F412/F413**: Keep inline construction (just relocate it)

## File to Modify
Only: `validating-lightning-signer/vls-signer-stm32/src/device/mod.rs`

---

## Current State

### Shared USB Code (lines 399-408) - TO REMOVE
```rust
// ── USB serial ──
info!("setup serial driver");
let serial = SerialDriver::new(USB {
    usb_global: p.OTG_FS_GLOBAL,
    usb_device: p.OTG_FS_DEVICE,
    usb_pwrclk: p.OTG_FS_PWRCLK,
    pin_dm: gpioa.pa11.into_alternate().into(),
    pin_dp: gpioa.pa12.into_alternate().into(),
    hclk: rcc.clocks.hclk(),
});
```

### Current Block Returns
- **F412** (line 418): `let (disp, mut i2c, ts_int, sdio) = { ... }`
- **F413** (line 454): `let (disp, mut i2c, ts_int, sdio) = { ... }`
- **F469** (line 489): `let (disp, i2c, touchscreen, sdio, button) = { ... }`

---

## Target State

### F412 Block (lines 417-450)
**Line 418 change:**
```rust
// FROM:
let (disp, mut i2c, ts_int, sdio) = {
// TO:
let (disp, mut i2c, ts_int, sdio, serial) = {
```

**Add before line 449 (before `(disp, i2c, ts_int, sdio)`):**
```rust
        // USB serial
        info!("setup serial driver");
        let serial = SerialDriver::new(USB {
            usb_global: p.OTG_FS_GLOBAL,
            usb_device: p.OTG_FS_DEVICE,
            usb_pwrclk: p.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate().into(),
            pin_dp: gpioa.pa12.into_alternate().into(),
            hclk: rcc.clocks.hclk(),
        });
```

**Line 449 change:**
```rust
// FROM:
        (disp, i2c, ts_int, sdio)
// TO:
        (disp, i2c, ts_int, sdio, serial)
```

### F413 Block (lines 453-486)
**Line 454 change:**
```rust
// FROM:
let (disp, mut i2c, ts_int, sdio) = {
// TO:
let (disp, mut i2c, ts_int, sdio, serial) = {
```

**Add before line 485 (before `(disp, i2c, ts_int, sdio)`):**
```rust
        // USB serial
        info!("setup serial driver");
        let serial = SerialDriver::new(USB {
            usb_global: p.OTG_FS_GLOBAL,
            usb_device: p.OTG_FS_DEVICE,
            usb_pwrclk: p.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate().into(),
            pin_dp: gpioa.pa12.into_alternate().into(),
            hclk: rcc.clocks.hclk(),
        });
```

**Line 485 change:**
```rust
// FROM:
        (disp, i2c, ts_int, sdio)
// TO:
        (disp, i2c, ts_int, sdio, serial)
```

### F469 Block (lines 488-520)
**Line 489 change:**
```rust
// FROM:
let (disp, i2c, touchscreen, sdio, button) = {
// TO:
let (disp, i2c, touchscreen, sdio, button, serial) = {
```

**Add before line 519 (before `(disp, i2c, touchscreen, sdio, button)`):**
```rust
        // USB serial via BSP
        info!("setup serial driver");
        let serial = SerialDriver::new(
            stm32f469i_disc::usb::init(
                (p.OTG_FS_GLOBAL, p.OTG_FS_DEVICE, p.OTG_FS_PWRCLK),
                gpioa.pa11,
                gpioa.pa12,
                &rcc.clocks,
            )
        );
```

**Line 519 change:**
```rust
// FROM:
        (disp, i2c, touchscreen, sdio, button)
// TO:
        (disp, i2c, touchscreen, sdio, button, serial)
```

### Remove Shared USB Code (lines 399-408)
**DELETE these lines entirely:**
```rust
    // ── USB serial ──
    info!("setup serial driver");
    let serial = SerialDriver::new(USB {
        usb_global: p.OTG_FS_GLOBAL,
        usb_device: p.OTG_FS_DEVICE,
        usb_pwrclk: p.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate().into(),
        pin_dp: gpioa.pa12.into_alternate().into(),
        hclk: rcc.clocks.hclk(),
    });
```

---

## Order of Operations

1. **Edit F412 block** - add USB init, update tuple
2. **Edit F413 block** - add USB init, update tuple
3. **Edit F469 block** - add BSP USB init, update tuple
4. **Remove shared USB code** (lines 399-408)

---

## Verification Steps

```bash
# Build F412 (may fail due to legacy HAL API issues)
cargo build --features stm32f412 --release --bin demo_signer -p vls-signer-stm32

# Build F413 (may fail due to legacy HAL API issues)
cargo build --features stm32f413 --release --bin demo_signer -p vls-signer-stm32

# Build F469 (must succeed)
cargo build --features stm32f469 --release --bin demo_signer -p vls-signer-stm32
```

---

## Line Reference Summary (Before Changes)

| Element | Lines |
|---------|-------|
| Shared USB code (DELETE) | 399-408 |
| F412 block start | 417 |
| F412 tuple return | 449-450 |
| F413 block start | 453 |
| F413 tuple return | 485-486 |
| F469 block start | 488 |
| F469 tuple return | 519-520 |
| `serial` used in DeviceContext | 539 |

---

## Key Constraints

1. `gpioa` is split at line 387 (before all cfg blocks) - available to all
2. `p.OTG_FS_*` peripherals need to be captured before cfg blocks since `p` is partially moved
3. `rcc.clocks` is accessed via reference - no ownership issues
4. Keep `SerialDriver::new()` call - only change what's passed to it
5. The `serial` variable must exist for all boards (used at line 539)
