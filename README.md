stm32f4xx-hal
=============

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
[embedded-hal]: https://github.com/japaric/embedded-hal.git

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
