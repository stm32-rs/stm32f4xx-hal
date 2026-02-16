stm32f4xx-hal
=============

[![Crates.io](https://img.shields.io/crates/d/stm32f4xx-hal.svg)](https://crates.io/crates/stm32f4xx-hal)
[![Crates.io](https://img.shields.io/crates/v/stm32f4xx-hal.svg)](https://crates.io/crates/stm32f4xx-hal)
[![Released API docs](https://docs.rs/stm32f4xx-hal/badge.svg)](https://docs.rs/stm32f4xx-hal)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![dependency status](https://deps.rs/repo/github/stm32-rs/stm32f4xx-hal/status.svg)](https://deps.rs/repo/github/stm32-rs/stm32f4xx-hal)
[![Continuous integration](https://github.com/stm32-rs/stm32f4xx-hal/workflows/Continuous%20integration/badge.svg)](https://github.com/stm32-rs/stm32f4xx-hal)

_stm32f4xx-hal_ contains a multi device hardware abstraction on top of the
peripheral access API for the STMicro STM32F4 series microcontrollers. The
selection of the MCU is done by feature gates, typically specified by board
support crates. Currently supported configurations are:

<table>
<tr>
<td>

* stm32f401
* stm32f405
* stm32f407
* stm32f410
* stm32f411
* stm32f412
<td>

* stm32f413
* stm32f415
* stm32f417
* stm32f423
* stm32f427
* stm32f429
<td>

* stm32f437
* stm32f439
* stm32f446
* stm32f469
* stm32f479
</tr>
</table>

The idea behind this crate is to gloss over the slight differences in the
various peripherals available on those MCUs so a HAL can be written for all
chips in that same family without having to cut and paste crates for every
single model.

### Other optional features

* `rtic1` — support [RTICv1 framework](https://crates.io/crates/cortex-m-rtic).
* `rtic2` — support [RTICv2 framework](https://crates.io/crates/rtic) (incompatible with `rtic1`, require nightly compiller).
* `defmt` — implementation of `defmt::Format` for public enums and structures. See [defmt](https://crates.io/crates/defmt).
* `can` — bxCAN peripheral support. See [bxcan](https://crates.io/crates/bxcan).
* `i2s` — I2S peripheral support. See [stm32_i2s_v12x](https://crates.io/crates/stm32_i2s_v12x).
* `usb_fs` or `usb_hs` — USB OTG FS/HS peripheral support. See [synopsys-usb-otg](https://crates.io/crates/synopsys-usb-otg).
* `fsmc_lcd` — LCD support via FMC/FSMC peripheral. See [display-interface](https://crates.io/crates/display-interface).
* `sdio-host` — SDIO peripheral support. See [sdio-host](https://crates.io/crates/sdio-host).
* `dsihost` — DSI host support. See [embedded-display-controller](https://crates.io/crates/embedded-display-controller).
* `framebuffer` — LTDC framebuffer abstraction with `DrawTarget`. See [embedded-graphics-core](https://crates.io/crates/embedded-graphics-core).

Collaboration on this crate is highly welcome as are pull requests!

This crate relies on Adam Greigs fantastic [stm32f4][] crate to provide
appropriate register definitions and implements a partial set of the
[embedded-hal][] traits.

Some of the implementation was shamelessly adapted from the [stm32f1xx-hal][]
crate originally started by Jorge Aparicio.

[stm32f4]: https://crates.io/crates/stm32f4
[stm32f1xx-hal]: https://github.com/stm32-rs/stm32f1xx-hal
[embedded-hal]: https://github.com/rust-embedded/embedded-hal

## Board Example Crates

This repository includes structured example crates for STM32 Discovery boards
in the `boards/` directory:

| Board Crate | MCU | Display | Interface |
|-------------|-----|---------|-----------|
| `boards/f469disco` | STM32F469NI | OTM8009A / NT35510 | DSI + LTDC |
| `boards/f429disco` | STM32F429ZI | ILI9341 | LTDC |
| `boards/f413disco` | STM32F413ZH | ST7789H2 | FSMC |

### Building Board Examples

```bash
# F469I-DISCO (DSI display with SDRAM framebuffer)
cargo build --release -p f469disco-examples --bin lcd-framebuffer

# F413H-DISCO (ST7789 via FSMC parallel bus)
cargo build --release -p f413disco-examples --bin st7789-fsmc

# F429I-DISCO (LTDC framebuffer)
cargo build --release -p f429disco-examples --bin ltdc-framebuffer
```

### Cross-Board Portability

All board examples use `DrawTarget` from embedded-graphics, enabling portable
drawing code:

```rust
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle};

fn draw_ui<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    Rectangle::new(Point::new(10, 10), Size::new(100, 50))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(display)
        .unwrap();
}
```

### Build Orchestration

Use the xtask tool to check all board/example combinations:

```bash
cargo xtask check-all
cargo xtask run-example --board f469disco --example f469disco-lcd-test
```

## Setting up your project

Check if the BSP for your board exists in the
[stm32-rs](https://github.com/stm32-rs) page.
If it exists, the `stm32f4xx-hal` crate should be already included, so you can
use the bsp as BSP for your project.

### Manually

Otherwise, create a new Rust project as you usually do with `cargo init`. The
"hello world" of embedded development is usually to blink a LED. The code to do
so is available in [examples/delay-syst-blinky.rs](examples/delay-syst-blinky.rs).
Copy that file to the `main.rs` of your project.

You also need to add some dependencies to your `Cargo.toml`:

```toml
[dependencies]
embedded-hal = "1.0"
nb = "1"
cortex-m = "0.7"
cortex-m-rt = "0.7"
# Panic behaviour, see https://crates.io/keywords/panic-impl for alternatives
panic-halt = "1.0"

[dependencies.stm32f4xx-hal]
version = "0.23.0"
features = ["stm32f407"] # replace the model of your microcontroller here
                         # and add other required features
```

We also need to tell Rust how to link our executable and how to lay out the
result in memory. To accomplish all this, copy [.cargo/config](.cargo/config.toml)
and [memory.x](memory.x) from the `stm32f4xx-hal` repository to your project and make sure the sizes match up with the datasheet. Also note that there might be different kinds of memory which are not equal; to be on the safe side only specify the size of the first block at the specified address.

### Fast start

To create empty project faster you could use `cargo generate` command. See [stm32-template](https://github.com/burrbull/stm32-template/).
```
$ cargo generate --git https://github.com/burrbull/stm32-template/
```
Note that you need to know your chip full name.

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
