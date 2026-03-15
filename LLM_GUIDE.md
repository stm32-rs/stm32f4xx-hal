# STM32F469I-DISCO HAL + BSP Integration Guide

## Git Reference

```
Repository: https://github.com/Amperstrand/stm32f4xx-hal
Branch:     pr2-f469disco-examples
Commit:     9c5b5ce4d436d3b6717e809b34b188841ce8fa68
```

## Cargo.toml Configuration

```toml
[dependencies]
# HAL with F469 features
stm32f4xx-hal = { git = "https://github.com/Amperstrand/stm32f4xx-hal", branch = "pr2-f469disco-examples", features = ["stm32f469", "stm32-fmc", "framebuffer", "defmt"] }

# Board Support Package
stm32f469i-disc = { git = "https://github.com/Amperstrand/stm32f4xx-hal", branch = "pr2-f469disco-examples" }

# Touch controller (patched version - clamps ntouch instead of panic)
ft6x06 = { git = "https://github.com/DougAnderson444/ft6x06", rev = "cc352f80b12fd985da4c4847771a26ebc03ece62" }

# Embedded graphics for UI
embedded-graphics = "0.8"
```

---

## Hardware Status

| Component | Status | Notes |
|-----------|--------|-------|
| LTDC/DSI Display | Working | 480x800 NT35510 via SDRAM framebuffer |
| SDRAM | Working | 16MB at 0xC0000000 |
| Touch (FT6X06) | Working | Fixed - PC1 pull-down configured |
| USB OTG FS Serial | Working | CDC-ACM virtual serial port |
| SDIO/SD Card | Working | 4-bit bus, PC8-PC12 + PD2 |
| Button (PA0) | Working | User button, active-high |
| LEDs | Working | PG6 (LED1), PG7 (LED2) |

---

## Bug Fixes in This Branch

### 1. Touch Interrupt Pin (Critical Fix)

**File:** `stm32f469i-disc/src/sdio.rs`

**Problem:** The FT6X06 touch interrupt is active-LOW. Without a pull-down resistor, the pin floats and `wait_touch_interrupt()` never returns.

**Fix:** PC1 is now configured with `into_pull_down_input()` before being returned to the caller.

```rust
// Before (broken):
(sdio, remainders.pc1)

// After (fixed):
(sdio, remainders.pc1.into_pull_down_input())
```

### 2. memory.x Conflict

**File:** `build.rs`

**Problem:** When building a BSP that depends on the HAL, both `build.rs` scripts ran and conflicted.

**Fix:** HAL only generates `memory.x` when `CARGO_PRIMARY_PACKAGE` is set.

```rust
let is_primary = env::var("CARGO_PRIMARY_PACKAGE").is_ok();
if is_primary {
    // Generate memory.x
}
```

---

## BSP Usage Examples

### Display + SDRAM + Touch Initialization

```rust
use stm32f4xx_hal::prelude::*;
use stm32f469i_disc as board;

// Clocks
let (rcc, mut delay) = board::make_clocks(dp.RCC.constrain(), cp.SYST);

// Split GPIO ports
let gpioc = dp.GPIOC.split(&mut rcc);
let gpiod = dp.GPIOD.split(&mut rcc);
let gpioe = dp.GPIOE.split(&mut rcc);
let gpiof = dp.GPIOF.split(&mut rcc);
let gpiog = dp.GPIOG.split(&mut rcc);
let gpioh = dp.GPIOH.split(&mut rcc);
let gpioi = dp.GPIOI.split(&mut rcc);

// Display pipeline: SDRAM -> LTDC -> DSI -> NT35510
let (fb, remainders) = board::lcd::init_display_pipeline(
    dp.FMC, dp.DSI, dp.LTDC, dp.DMA2D,
    gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi,
    &mut rcc, &mut delay,
);

// I2C for touch controller (PB8=SCL, PB9=SDA)
let mut i2c = board::touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);

// SDIO returns (sdio, pc1) - PC1 is touch interrupt, already configured
let (sdio, pc1) = board::sdio::init(dp.SDIO, remainders, &mut rcc);

// Touch controller with calibration
let touchscreen = board::touch::init_touchscreen(&mut i2c, pc1, &mut delay)
    .expect("touchscreen init");

// Button
let button = board::button::init(gpioa.pa0);
```

### Reading Touch

```rust
use ft6x06::Ft6X06;

// Wait for touch (blocking)
touchscreen.wait_touch_interrupt();

// Check number of touches
let n = touchscreen.detect_touch(&mut i2c)?;

// Get first touch point
if n > 0 {
    let point = touchscreen.get_touch(&mut i2c, 1)?;
    let (x, y) = (point.x, point.y);
    // x, y are in display coordinates (0-479, 0-799)
}
```

### USB Serial

```rust
let usb = board::usb::init(
    (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
    gpioa.pa11,
    gpioa.pa12,
    &rcc.clocks,
);
```

---

## Double Buffering (No Flicker)

### Memory Layout

```
SDRAM @ 0xC0000000 (16 MB):
+------------------------+
| Buffer A  (768 KB)     |  0xC0000000
+------------------------+
| Buffer B  (768 KB)     |  0xC0177000
+------------------------+
| Available (~14.5 MB)   |
+------------------------+
```

### Allocation

```rust
const WIDTH: usize = 480;
const HEIGHT: usize = 800;
const FB_SIZE: usize = WIDTH * HEIGHT;  // 384,000 u16s (768 KB)

let base_ptr = sdram.init(&mut delay) as *mut u16;

let buf_a: &'static mut [u16] = unsafe {
    &mut *core::ptr::slice_from_raw_parts_mut(base_ptr, FB_SIZE)
};
let buf_b: &'static mut [u16] = unsafe {
    &mut *core::ptr::slice_from_raw_parts_mut(base_ptr.add(FB_SIZE), FB_SIZE)
};

let buf_a_addr = base_ptr as u32;
let buf_b_addr = unsafe { base_ptr.add(FB_SIZE) } as u32;
```

### Double Buffer Helper

```rust
struct DoubleBuffer {
    front_addr: u32,
    back_addr: u32,
}

impl DoubleBuffer {
    fn new(a: u32, b: u32) -> Self {
        Self { front_addr: a, back_addr: b }
    }

    fn back_buffer(&self) -> &'static mut [u16] {
        unsafe { core::slice::from_raw_parts_mut(self.back_addr as *mut u16, FB_SIZE) }
    }

    fn swap(&mut self, display: &mut LtdcFramebuffer<u16>) {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        display.set_layer_buffer_address(Layer::L1, self.back_addr);
        core::mem::swap(&mut self.front_addr, &mut self.back_addr);
    }
}
```

### Render Loop

```rust
let mut dbl_buf = DoubleBuffer::new(buf_a_addr, buf_b_addr);

display.config_layer(Layer::L1, buf_a, PixelFormat::RGB565);
display.enable_layer(Layer::L1);
display.reload();

loop {
    let back = dbl_buf.back_buffer();

    // Clear
    for pixel in back.iter_mut() {
        *pixel = 0x0000; // Black
    }

    // Draw UI with embedded-graphics
    draw_ui(back);

    // Swap (VSYNC synchronized)
    dbl_buf.swap(&mut display);

    delay.delay_ms(16); // ~60fps
}
```

---

## Key Files in BSP

| File | Purpose |
|------|---------|
| `lcd.rs` | Display init, SDRAM framebuffer, DSI/LTDC config |
| `touch.rs` | FT6X06 init, I2C setup |
| `sdio.rs` | SD card init, returns PC1 for touch |
| `sdram.rs` | SDRAM pin extraction, remainders struct |
| `usb.rs` | USB OTG FS serial port |
| `button.rs` | User button (PA0) |
| `led.rs` | LEDs (PG6, PG7) |

---

## Known Issues

1. **SD Card Init Timeout** - Some cards timeout during initialization. Workaround: reinsert card or add delay.

2. **Touch Calibration** - The FT6X06 `ts_calibration()` runs on first init. It may log errors but still work.

3. **HAL Examples** - Some F469 examples in the HAL reference old APIs. Use BSP examples as reference instead.

---

## Build Commands

```bash
# Build BSP examples
cd stm32f469i-disc
cargo build --release --example display_touch --features defmt

# Build with probe-rs
cargo run --release --example display_touch --features defmt --chip STM32F469NIHx

# Build VLS test
cd ../validating-lightning-signer/vls-signer-stm32
cargo build --release --bin test --features stm32f469
```

---

## Flashing

```bash
# Via probe-rs on remote
scp target/thumbv7em-none-eabihf/release/test ubuntu@192.168.13.246:/tmp/test.elf
ssh ubuntu@192.168.13.246 "probe-rs run --chip STM32F469NIHx /tmp/test.elf"
```

---

## Summary

- Use branch `pr2-f469disco-examples` from Amperstrand/stm32f4xx-hal
- Touch is fixed (PC1 pull-down)
- Display, SDRAM, USB, SDIO all working
- Double buffering available via `set_layer_buffer_address()`
- BSP modules in `stm32f469i-disc` handle board-specific init
