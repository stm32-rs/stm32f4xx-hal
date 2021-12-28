#![no_std]
#![allow(non_camel_case_types)]

pub use embedded_hal as hal;

pub use nb;
pub use nb::block;

#[cfg(feature = "stm32f401")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f401 peripherals.
pub use stm32f4::stm32f401 as pac;

#[cfg(feature = "stm32f405")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f405 peripherals.
pub use stm32f4::stm32f405 as pac;

#[cfg(feature = "stm32f407")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f407 peripherals.
pub use stm32f4::stm32f407 as pac;

#[cfg(feature = "stm32f410")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f410 peripherals.
pub use stm32f4::stm32f410 as pac;

#[cfg(feature = "stm32f411")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f411 peripherals.
pub use stm32f4::stm32f411 as pac;

#[cfg(feature = "stm32f412")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f412 peripherals.
pub use stm32f4::stm32f412 as pac;

#[cfg(feature = "stm32f413")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f413 peripherals.
pub use stm32f4::stm32f413 as pac;

#[cfg(feature = "stm32f415")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f405 peripherals.
pub use stm32f4::stm32f405 as pac;

#[cfg(feature = "stm32f417")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f407 peripherals.
pub use stm32f4::stm32f407 as pac;

#[cfg(feature = "stm32f423")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f413 peripherals.
pub use stm32f4::stm32f413 as pac;

#[cfg(feature = "stm32f427")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f427 peripherals.
pub use stm32f4::stm32f427 as pac;

#[cfg(feature = "stm32f429")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f429 peripherals.
pub use stm32f4::stm32f429 as pac;

#[cfg(feature = "stm32f437")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f427 peripherals.
pub use stm32f4::stm32f427 as pac;

#[cfg(feature = "stm32f439")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f429 peripherals.
pub use stm32f4::stm32f429 as pac;

#[cfg(feature = "stm32f446")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f446 peripherals.
pub use stm32f4::stm32f446 as pac;

#[cfg(feature = "stm32f469")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f469 peripherals.
pub use stm32f4::stm32f469 as pac;

#[cfg(feature = "stm32f479")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f469 peripherals.
pub use stm32f4::stm32f469 as pac;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use crate::pac::interrupt;

pub mod adc;
pub mod bb;
#[cfg(all(
    feature = "can",
    any(feature = "can1", feature = "can2",)
))]
pub mod can;
pub mod crc32;
#[cfg(feature = "dac")]
pub mod dac;
pub mod delay;
#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c;
pub mod gpio;
pub mod i2c;
pub mod i2s;
#[cfg(all(feature = "usb_fs", feature = "otg-fs"))]
pub mod otg_fs;
#[cfg(all(
    any(feature = "usb_hs", docsrs),
    feature = "otg-hs",
))]
pub mod otg_hs;

#[cfg(feature = "rng")]
pub mod rng;

pub mod dma;
pub mod dwt;
pub mod flash;
#[cfg(all(
    feature = "fsmc_lcd",
    any(feature = "fmc", feature = "fsmc")
))]
pub mod fsmc_lcd;
pub mod prelude;
pub mod pwm;
#[cfg(not(feature = "stm32f410"))]
pub mod pwm_input;
pub mod qei;
pub mod rcc;
pub mod rtc;
#[cfg(all(feature = "sdio-host", feature = "sdio"))]
pub mod sdio;
pub mod serial;
pub mod signature;
pub mod spi;
pub mod syscfg;
pub mod time;
pub mod timer;
pub mod watchdog;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;
