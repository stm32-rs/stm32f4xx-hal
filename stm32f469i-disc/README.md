stm32f469i-disc
===============
Board support package for the STM32F469I-DISCOVERY kit.
Quick Start
-----------

```toml
[dependencies.stm32f469i-disc]
git = "https://github.com/Amperstrand/stm32f4xx-hal"
features = ["defmt"]
```

```bash
cargo run --release --example gpio_hal_blinky
```


Module Overview
---------------
- `lcd` - Display with auto-detection
- `led` - On-board LEDs
- `sdram` - 16MB SDRAM
- `touch` - FT6X06 controller
- `sdio` - SD card
- `button` - User button
- `usb` - USB OTG FS

Documentation Links
-------------------
- [USB Guide](docs/USB-GUIDE.md) - USB OTG FS setup and CDC-ACM
- [Pin Consumption](docs/PIN-CONSUMPTION.md) - Which pins SDRAM consumes
- [Testing Guide](../STM32F469_HAL_BSP_TESTING.md) - Full HAL/BSP testing instructions

Peripheral Support
------------------
- [x] Green, Orange, Red, Blue user LEDs
- [x] 16MB SDRAM on FMC interface
- [x] NT35510/OTM8009A LCD with DSI interface (auto-detected)
- [x] FT6X06 touch controller (I2C)
- [ ] Other on-board peripherals

Examples
--------
- `gpio_hal_blinky` — Cycle through user LEDs
- `fmc_sdram_test` — Read/write SDRAM test with pattern verification
- `display_dsi_lcd` — Rolling gradient animation on DSI display
- `display_hello_eg` — Text and shapes using embedded-graphics
- `display_touch` — Touch input with swipe gesture detection
- `usb_cdc_serial` — USB CDC-ACM virtual serial port echo test

Building
--------
```bash
# Basic example (no special features)
cargo build --example gpio_hal_blinky

# SDRAM test
cargo build --example fmc_sdram_test

# Display examples (requires framebuffer feature)
cargo build --release --example display_dsi_lcd
cargo build --release --example display_hello_eg --features framebuffer
cargo build --release --example display_touch
```

Running
-------
```bash
cargo run --release --example display_touch
```

Credits
-------
Thanks to the authors of [stm32f429i-disc](https://github.com/stm32-rs/stm32f429i-disc.git) and [stm32f407g-disc](https://github.com/stm32-rs/stm32f407g-disc.git) crates for solid starting points.

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
