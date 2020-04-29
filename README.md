stm32f4xx-hal
=============

[![Crates.io](https://img.shields.io/crates/v/stm32f4xx-hal.svg)](https://crates.io/crates/stm32f4xx-hal)
[![Released API docs](https://docs.rs/stm32f4xx-hal/badge.svg)](https://docs.rs/stm32f4xx-hal)

_stm32f4xx-hal_ contains a multi device hardware abstraction on top of the
peripheral access API for the STMicro STM32F4 series microcontrollers. The
selection of the MCU is done by feature gates, typically specified by board
support crates. Currently supported configurations are:

* stm32f401
* stm32f405
* stm32f407
* stm32f410
* stm32f411
* stm32f412
* stm32f413
* stm32f415
* stm32f417
* stm32f423
* stm32f427
* stm32f429
* stm32f437
* stm32f439
* stm32f446
* stm32f469
* stm32f479

The idea behind this crate is to gloss over the slight differences in the
various peripherals available on those MCUs so a HAL can be written for all
chips in that same family without having to cut and paste crates for every
single model.

Collaboration on this crate is highly welcome as are pull requests!

This crate relies on Adam Greigs fantastic [stm32f4][] crate to provide
appropriate register definitions and implements a partial set of the
[embedded-hal][] traits.

Some of the implementation was shamelessly adapted from the [stm32f103xx-hal][]
crate by Jorge Aparicio.

[stm32f4]: https://crates.io/crates/stm32f4
[stm32f103xx-hal]: https://github.com/japaric/stm32f103xx-hal
[embedded-hal]: https://github.com/rust-embedded/embedded-hal

Setting up your project
-------

Check if the BSP for your board exists in the
[stm32-rs](https://github.com/stm32-rs) page.
If it exists, the `stm32f4xx-hal` crate should be already included, so you can
use the bsp as BSP for your project.

Otherwise, create a new Rust project as you usually do with `cargo init`. The
"hello world" of embedded development is usually to blink a LED. The code to do
so is available in [examples/delay-blinky.rs](examples/delay-blinky.rs).
Copy that file to the `main.rs` of your project.

You also need to add some dependencies to your `Cargo.toml`:

```toml
[dependencies]
embedded-hal = "0.2"
nb = "0.1.2"
cortex-m = "0.6"
cortex-m-rt = "0.6"
# Panic behaviour, see https://crates.io/keywords/panic-impl for alternatives
panic-halt = "0.2"

[dependencies.stm32f4xx-hal]
version = "0.8"
features = ["rt", "stm32f407"] # replace the model of your microcontroller here
```

We also need to tell Rust how to link our executable and how to lay out the
result in memory. To accomplish all this, copy [.cargo/config](.cargo/config)
and [memory.x](memory.x) from the `stm32f4xx-hal` repository to your project and make sure the sizes match up with the datasheet. Also note that there might be different kinds of memory which are not equal; to be on the safe side only specify the size of the first block at the specified address.

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
