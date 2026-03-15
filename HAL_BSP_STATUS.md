# STM32F4xx HAL + STM32F469I-DISCO BSP Status

## Git Reference

```
Branch:  pr2-f469disco-examples
Commit:  9c5b5ce4d436d3b6717e809b34b188841ce8fa68
Repo:    github.com/Amperstrand/stm32f4xx-hal
```

## What's Working

| Feature | Status |
|---------|--------|
| LTDC/DSI display (480x800) | OK |
| SDRAM framebuffer | OK |
| Touch (FT6X06) | OK (fixed) |
| USB serial | OK |
| SDIO/SD card | OK |
| Button (PA0) | OK |

## Bug Fixes Applied

1. **Touch interrupt pin** (`stm32f469i-disc/src/sdio.rs`):
   - PC1 now configured with `into_pull_down_input()`
   - FT6X06 interrupt is active-LOW, needs pull-down for defined idle state
   - Without this, `wait_touch_interrupt()` never returns

2. **memory.x conflict** (`build.rs`):
   - HAL only generates memory.x when built as primary package
   - Prevents conflict when BSP has its own memory.x

## Double Buffering

See `DOUBLE_BUFFER_GUIDE.md` for complete example.

Key points:
- Two buffers in SDRAM at 0xC0000000
- Use `set_layer_buffer_address()` to swap
- Swap is VSYNC-synchronized automatically

## BSP Usage

```rust
// Display + SDRAM
let (disp, remainders) = stm32f469i_disc::lcd::init_display_pipeline(
    fmc, dsi, ltdc, dma2d, gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi,
    &mut rcc, &mut delay,
);

// Touch
let mut i2c = stm32f469i_disc::touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);
let (sdio, pc1) = stm32f469i_disc::sdio::init(dp.SDIO, remainders, &mut rcc);
let touchscreen = stm32f469i_disc::touch::init_touchscreen(&mut i2c, pc1, &mut delay);
```

## What's Left / Known Issues

1. **SD card detection** - Some cards timeout during init. May need delay or reinsert.

2. **VLS integration** - Update Cargo.toml to point to this branch.

3. **Examples** - HAL examples for F469 could use cleanup (some reference old API).

4. **Upstream PR** - Branch is ready to submit as PR to upstream stm32f4xx-hal.
