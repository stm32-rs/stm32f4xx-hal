# Response: STM32F469I-DISCO BSP Support Available

Thank you for your interest in adding display and touch support to the stm32f4xx-hal BSP. **Good news — this work has already been implemented and is ready for use!**

---

## What's Available

A complete **STM32F469I-DISCO Board Support Package** exists in the Amperstrand fork:

📦 **Repository**: https://github.com/Amperstrand/stm32f4xx-hal  
🌿 **Branch**: `pr2-f469disco-examples`  
🔖 **Latest Commit**: `b32ccf4` — `fix(ft6x06): patch touch controller panic`

---

## Features Implemented

| Feature | Status | Details |
|---------|--------|---------|
| **DSI + LTDC Display** | ✅ Complete | Full initialization sequence for NT35510/OTM8009A |
| **Panel Auto-Detection** | ✅ Complete | Runtime probe detects B07 vs B08+ boards |
| **Touch Controller** | ✅ Complete | FT6X06 with panic fix applied |
| **16MB SDRAM** | ✅ Complete | FMC interface at 0 0xC0000000 |
| **Examples** | ✅ Complete | 4 working examples included |

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies.stm32f4xx-hal]
version = "0.23.0"
git = "https://github.com/Amperstrand/stm32f4xx-hal"
branch = "pr2-f469disco-examples"
features = ["stm32f469", "stm32-fmc", "dsihost", "sdio-host"]
```

---

## What's Included

### HAL-Level Drivers
- `src/dsi.rs` — MIPI DSI host driver (713 lines)
- `src/ltdc.rs` — LTDC display controller

### Panel Drivers (in `nt35510/` crate)
- NT35510 LCD controller driver
- OTM8009A support via existing crate

### BSP Crate (`stm32f469i-disc/`)
- `lcd.rs` — Full DSI+LTDC init with auto-detection
- `touch.rs` — FT6X06 I2C setup
- `sdram.rs` — 16MB FMC SDRAM
- **4 examples**: display_dsi_lcd, display_hello_eg, display_touch, fmc_sdram_test

---

## Touch Controller Fix

The ft6x06 crate v0.1.2 panics on spurious multi-touch values. We've patched this at the dependency level:

```toml
[patch.crates-io]
ft6x06 = { git = "https://github.com/DougAnderson444/ft6x06", branch = "main" }
```

This applies [PR #5](https://github.com/Srg213/ft6x06/pull/5) which clamps touch count instead of asserting.

---

## Testing

All code has been tested on real STM32F469NIHx hardware via probe-rs:

```bash
# Build examples
cd stm32f469i-disc
cargo build --release --example display_dsi_lcd
cargo build --release --example display_touch

# Flash and run
scp target/thumbv7em-none-eabihf/release/examples/display_dsi_lcd ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 "probe-rs run --chip STM32F469NIHx /tmp/display_dsi_lcd"
```

---

## Known Limitations

- **USB VCP**: Not yet implemented (noted as TODO in BSP crate)
- **Display Flicker**: Known issue without double-buffering; workaround via dirty region tracking

---

## Request

Please test the branch `pr2-f469disco-examples` and provide feedback. If the implementation looks good, we'd be happy to rebase onto upstream `main` for merging.

**Commit to test**: `b32ccf4168b13e26856a52ed862a2dbd491fb805`
