# STM32F469 HAL + BSP Testing Guide

This document provides the configuration and instructions for testing the STM32F469 HAL and BSP crates from the `stm32f4xx-hal` repository.

## Repository Information

- **Repository**: `https://github.com/Amperstrand/stm32f4xx-hal`
- **Branch**: `pr2-f469disco-examples`
- **Head Commit**: `1495022`

## Architecture Overview

### Layering

```
┌──────────────────────────────────────────────────────────────────┐
│  APPLICATION (your firmware)                                      │
├──────────────────────────────────────────────────────────────────┤
│  stm32f469i-disc (BSP crate)                                      │
│  - Board-specific pin assignments for 32F469IDISCOVERY            │
│  - NT35510/OTM8009A display panel detection                       │
│  - FT6X06 touch controller initialization                         │
│  - SDRAM, SDIO, LED, button helpers                               │
│  - USB OTG FS board wiring                                        │
├──────────────────────────────────────────────────────────────────┤
│  stm32f4xx-hal (HAL crate)                                        │
│  - Generic MCU peripherals (LTDC, DSI, I2C, SPI, etc.)            │
│  - DSI host primitives and LTDC framebuffer                       │
│  - embedded-graphics DrawTarget implementation                    │
│  - No board-specific code                                         │
├──────────────────────────────────────────────────────────────────┤
│  External Driver Crates                                           │
│  - nt35510 (display controller)                                   │
│  - otm8009a (display controller)                                  │
│  - ft6x06 (touch controller) - PINNED TO FIX COMMIT               │
└──────────────────────────────────────────────────────────────────┘
```

### What Belongs Where

| Component | HAL (`stm32f4xx-hal`) | BSP (`stm32f469i-disc`) |
|-----------|----------------------|------------------------|
| DSI host primitives | ✅ | ❌ |
| LTDC peripheral driver | ✅ | ❌ |
| `LtdcFramebuffer<DrawTarget>` | ✅ | ❌ |
| Display panel detection | ❌ | ✅ |
| NT35510 init sequence | ❌ | ✅ |
| OTM8009A init sequence | ❌ | ✅ |
| FT6X06 touch wiring | ❌ | ✅ |
| SDRAM pin configuration | ❌ | ✅ |
| SDIO board wiring | ❌ | ✅ |
| LED/button aliases | ❌ | ✅ |

## Cargo Configuration

### Recommended `Cargo.toml`

```toml
[dependencies]
# Cortex-M runtime
cortex-m = "0.7"
cortex-m-rt = "0.7"

# HAL crate - generic STM32F4 peripheral abstractions
[dependencies.stm32f4xx-hal]
git = "https://github.com/Amperstrand/stm32f4xx-hal"
branch = "pr2-f469disco-examples"
features = [
    "stm32f469",      # MCU feature
    "stm32-fmc",      # SDRAM via FMC
    "dsihost",        # DSI + LTDC support
    "framebuffer",    # LtdcFramebuffer DrawTarget
    "usb_fs",         # USB OTG FS
]

# BSP crate - STM32F469I-DISCO board support
[dependencies.stm32f469i-disc]
git = "https://github.com/Amperstrand/stm32f4xx-hal"
branch = "pr2-f469disco-examples"
default-features = false
features = [
    "framebuffer",    # Enables HAL framebuffer feature
    "usb_fs",         # Enables HAL USB FS feature
]

# Optional: defmt for logging
[dependencies.defmt]
version = "1.0"
optional = true

[features]
default = ["defmt"]
defmt = ["dep:defmt", "stm32f4xx-hal/defmt", "stm32f469i-disc/defmt"]
```

### Feature Reference

| Feature | Crate | Purpose |
|---------|-------|---------|
| `stm32f469` | HAL | Enables STM32F469NI PAC and HAL support |
| `stm32-fmc` | HAL | FMC peripheral for SDRAM |
| `dsihost` | HAL | DSI host + LTDC support |
| `framebuffer` | HAL | `LtdcFramebuffer` implementing `embedded-graphics::DrawTarget` |
| `usb_fs` | HAL | USB OTG FS support |
| `sdio-host` | HAL | SDIO peripheral support |
| `sdio` | HAL | SD card support |
| `defmt` | Both | Logging via defmt |

## Critical: FT6X06 Touch Driver Fix

The FT6X06 touch controller driver has a known bug where it panics on spurious multi-touch values. This repository includes a patched version.

**Fixed commit**: `cc352f80b12fd985da4c4847771a26ebc03ece62`  
**Source**: https://github.com/Srg213/ft6x06/pull/5

The fix changes:
```rust
// BEFORE (crashes on spurious values):
assert!(ntouch <= FT6X06_MAX_NB_TOUCH as u8);

// AFTER (clamps safely):
Ok(core::cmp::min(ntouch, FT6X06_MAX_NB_TOUCH as u8))
```

This fix is already pinned in both `stm32f4xx-hal/Cargo.toml` and `stm32f469i-disc/Cargo.toml`. Do not override it.

## BSP Module Reference

The `stm32f469i-disc` crate provides these modules:

### `lcd` - Display Pipeline

```rust
use stm32f469i_disc::lcd::{
    self as board,
    FB_SIZE, HEIGHT, WIDTH,
    BoardHint,
    init_display_full,
};

// Initialize DSI, detect panel (NT35510/OTM8009A), configure LTDC
let (mut display_ctrl, controller) = board::init_display_full(
    dp.DSI,
    dp.LTDC,
    dp.DMA2D,
    &mut rcc,
    &mut delay,
    board::BoardHint::Unknown,
    stm32f4xx_hal::ltdc::PixelFormat::RGB565,
);

// Configure layer with SDRAM framebuffer
display_ctrl.config_layer(Layer::L1, buffer, hal::ltdc::PixelFormat::RGB565);
display_ctrl.enable_layer(Layer::L1);
display_ctrl.reload();
```

### `sdram` - 16MB SDRAM

```rust
use stm32f469i_disc::sdram::{Sdram, SdramRemainders, split_sdram_pins, SDRAM_SIZE_BYTES};

// Split GPIO ports into SDRAM pins + remainders
let (sdram_pins, remainders, lcd_reset) = split_sdram_pins(
    gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi,
);

// Initialize SDRAM
let mut sdram = Sdram::new(dp.FMC, sdram_pins, &clocks, &mut delay);

// Get framebuffer slice
let buffer: &'static mut [u16] = sdram.as_slice_mut();
```

### `touch` - FT6X06 Touch Controller

```rust
use stm32f469i_disc::touch::{init_i2c, init_ft6x06, FT6X06_I2C_ADDR};

// Initialize I2C1 (PB8=SCL, PB9=SDA)
let mut i2c = init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);

// Initialize touch controller
let ts_int = remainders.pc1.into_pull_down_input();
let mut touch = init_ft6x06(&i2c, ts_int).expect("FT6X06 not detected");

// Read touches
if let Ok(num) = touch.detect_touch(&mut i2c) {
    if num > 0 {
        if let Ok(point) = touch.get_touch(&mut i2c, 1) {
            let x = point.x;
            let y = point.y;
        }
    }
}
```

### `sdio` - SD Card

```rust
use stm32f469i_disc::sdio::init;

// Initialize SDIO (uses PC8-PC12, PD2)
let (sdio, touch_int) = init(dp.SDIO, remainders, &mut rcc);
// sdio: Sdio<SdCard> - call .init() to detect card
// touch_int: PC1 available for touch interrupt
```

### `led` - On-board LEDs

```rust
use stm32f469i_disc::led::{Leds, LedColor};

let mut leds = Leds::new(gpiod, gpiog, gpiok);
leds[LedColor::Green].on();
leds[LedColor::Red].toggle();
```

### `button` - User Button

```rust
use stm32f469i_disc::button::{Button, init};

let button: Button = init(pa0);
if button.is_high() {
    // Button pressed
}
```

## Build Commands

### HAL + BSP (Basic Test)

```bash
# Check HAL with F469 features
cargo check -p stm32f4xx-hal \
  --features stm32f469,stm32-fmc,dsihost,framebuffer

# Check BSP crate
cargo check -p stm32f469i-disc
```

### Display Examples

```bash
# Hello world (shapes and text)
cargo build --example f469disco-hello-eg --release \
  --features stm32f469,stm32-fmc,framebuffer,dsihost,defmt

# Paint app (touch drawing)
cargo build --example f469disco-paint --release \
  --features stm32f469,stm32-fmc,framebuffer,dsihost,defmt

# Image slider (touch swipe)
cargo build --example f469disco-image-slider --release \
  --features stm32f469,stm32-fmc,framebuffer,dsihost,defmt

# Slideshow (dual layer crossfade)
cargo build --example f469disco-slideshow --release \
  --features stm32f469,stm32-fmc,framebuffer,dsihost,defmt
```

### Verify FT6X06 Fix

```bash
cargo tree -p ft6x06
# Should show:
# ft6x06 v0.1.2 (https://github.com/DougAnderson444/ft6x06?rev=cc352f80...)
```

## Hardware Requirements

- **Board**: STM32F469I-DISCO (32F469IDISCOVERY)
- **Display**: On-board 480×800 DSI LCD (NT35510 or OTM8009A, auto-detected)
- **Touch**: On-board FT6X06 capacitive touch
- **Memory**: 16MB SDRAM onboard (used for framebuffer)

## Pin Reference

| Function | Pin | Notes |
|----------|-----|-------|
| LCD Reset | PH7 | Active low |
| Touch I2C SCL | PB8 | I2C1 |
| Touch I2C SDA | PB9 | I2C1 |
| Touch Interrupt | PC1 | Optional, polling works |
| User Button | PA0 | Active high |
| LED Green (LD1) | PG6 | |
| LED Orange (LD2) | PD4 | |
| LED Red (LD3) | PD5 | |
| LED Blue (LD4) | PK3 | |
| USB OTG FS | PA11/PA12 | |

## Common Issues

### 1. Black Screen

- Check LCD reset timing (PH7: low 20ms, high 10ms)
- Verify SDRAM initialization before display init
- Ensure DSI clock configuration is correct

### 2. Touch Crashes

- Verify ft6x06 is pinned to `cc352f80` (run `cargo tree -p ft6x06`)
- Do NOT use crates.io `ft6x06 = "0.1.2"` without the patch

### 3. Display Flicker

- Use dual buffering with `set_layer_buffer_address()`
- Ensure SDRAM timing is correct
- Check LTDC vsync synchronization

## For VLS (Validating Lightning Signer) Projects

```toml
[dependencies.stm32f4xx-hal]
git = "https://github.com/Amperstrand/stm32f4xx-hal"
branch = "pr2-f469disco-examples"
default-features = false
features = [
    "stm32f469",
    "stm32-fmc",
    "dsihost",
    "framebuffer",
    "sdio",
    "sdio-host",
    "otg-fs",
    "usb_fs",
]

[dependencies.stm32f469i-disc]
git = "https://github.com/Amperstrand/stm32f4xx-hal"
branch = "pr2-f469disco-examples"
default-features = false
features = ["framebuffer", "usb_fs"]
```

## Commit History for Reference

| Commit | Description |
|--------|-------------|
| `1495022` | `feat(bsp): add stm32f469 board support modules` |
| `cfe2656` | `refactor(f469): use BSP lcd helpers and pin ft6x06 fix` |
| `3a79c39` | `docs(bsp): add F469 cleanup plan and pin ft6x06 crash fix` |
| `97f9a51` | `feat(bsp): add init_display_pipeline() and HSE_FREQ constant` |
| `50e65d3` | `feat(bsp): working USB OTG FS module with correct HAL API` |
| `b32ccf4` | `fix(ft6x06): patch touch controller panic on spurious multi-touch values` |
| `dc928d7` | `feat(examples): add F469-Disco display examples with NT35510/OTM8009A auto-detection` |

## Summary

To test the STM32F469 HAL and BSP:

1. Add both `stm32f4xx-hal` and `stm32f469i-disc` as dependencies from branch `pr2-f469disco-examples`
2. Enable features: `stm32f469`, `stm32-fmc`, `dsihost`, `framebuffer`
3. Use BSP modules (`lcd`, `sdram`, `touch`, etc.) for board-specific initialization
4. Use HAL for generic peripheral access
5. The FT6X06 touch fix is already pinned - don't override it
6. Build and flash any F469 example to verify display/touch functionality
