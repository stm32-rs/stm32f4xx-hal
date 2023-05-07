#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(feature = "device-selected"))]
compile_error!("This crate requires one of stm32xxxx device features enabled");

#[cfg(feature = "device-selected")]
pub use embedded_hal as hal;

#[cfg(feature = "device-selected")]
pub use nb;
#[cfg(feature = "device-selected")]
pub use nb::block;

#[cfg(feature = "svd-f0x0")]
pub use stm32f0::stm32f0x0 as pac;

#[cfg(feature = "svd-f0x1")]
pub use stm32f0::stm32f0x1 as pac;

#[cfg(feature = "svd-f0x2")]
pub use stm32f0::stm32f0x2 as pac;

#[cfg(feature = "svd-f0x8")]
pub use stm32f0::stm32f0x8 as pac;

#[cfg(feature = "svd-f301")]
pub use stm32f3::stm32f301 as pac;

#[cfg(feature = "svd-f302")]
pub use stm32f3::stm32f302 as pac;

#[cfg(feature = "svd-f303")]
pub use stm32f3::stm32f303 as pac;

#[cfg(feature = "svd-f373")]
pub use stm32f3::stm32f373 as pac;

#[cfg(feature = "svd-f3x4")]
pub use stm32f3::stm32f3x4 as pac;

#[cfg(feature = "svd-f401")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f401 peripherals.
pub use stm32f4::stm32f401 as pac;

#[cfg(feature = "svd-f405")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f405/f415 peripherals.
pub use stm32f4::stm32f405 as pac;

#[cfg(feature = "svd-f407")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f407/f417 peripherals.
pub use stm32f4::stm32f407 as pac;

#[cfg(feature = "svd-f410")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f410 peripherals.
pub use stm32f4::stm32f410 as pac;

#[cfg(feature = "svd-f411")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f411 peripherals.
pub use stm32f4::stm32f411 as pac;

#[cfg(feature = "svd-f412")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f412 peripherals.
pub use stm32f4::stm32f412 as pac;

#[cfg(feature = "svd-f413")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f413/f423 peripherals.
pub use stm32f4::stm32f413 as pac;

#[cfg(feature = "svd-f427")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f427/f437 peripherals.
pub use stm32f4::stm32f427 as pac;

#[cfg(feature = "svd-f429")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f429/f439 peripherals.
pub use stm32f4::stm32f429 as pac;

#[cfg(feature = "svd-f446")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f446 peripherals.
pub use stm32f4::stm32f446 as pac;

#[cfg(feature = "svd-f469")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the stm32f469/f479 peripherals.
pub use stm32f4::stm32f469 as pac;

#[cfg(feature = "svd-f7x2")]
pub use stm32f7::stm32f7x2 as pac;

#[cfg(feature = "svd-f7x3")]
pub use stm32f7::stm32f7x3 as pac;

#[cfg(feature = "svd-f730")]
pub use stm32f7::stm32f730 as pac;

#[cfg(feature = "svd-f745")]
pub use stm32f7::stm32f745 as pac;

#[cfg(feature = "svd-f7x6")]
pub use stm32f7::stm32f7x6 as pac;

#[cfg(feature = "svd-f765")]
pub use stm32f7::stm32f765 as pac;

#[cfg(feature = "svd-f7x7")]
pub use stm32f7::stm32f7x7 as pac;

#[cfg(feature = "svd-f7x9")]
pub use stm32f7::stm32f7x9 as pac;

#[cfg(feature = "svd-g431")]
pub use stm32g4::stm32g431 as pac;

#[cfg(feature = "svd-g441")]
pub use stm32g4::stm32g441 as pac;

#[cfg(feature = "svd-g471")]
pub use stm32g4::stm32g471 as pac;

#[cfg(feature = "svd-g473")]
pub use stm32g4::stm32g473 as pac;

#[cfg(feature = "svd-g474")]
pub use stm32g4::stm32g474 as pac;

#[cfg(feature = "svd-g483")]
pub use stm32g4::stm32g483 as pac;

#[cfg(feature = "svd-g484")]
pub use stm32g4::stm32g484 as pac;

#[cfg(feature = "svd-g491")]
pub use stm32g4::stm32g491 as pac;

#[cfg(feature = "svd-g4a1")]
pub use stm32g4::stm32g4a1 as pac;

#[cfg(feature = "svd-l4x1")]
pub use stm32l4::stm32l4x1 as pac;

#[cfg(feature = "svd-l412")]
pub use stm32l4::stm32l412 as pac;

#[cfg(feature = "svd-l4x2")]
pub use stm32l4::stm32l4x2 as pac;

#[cfg(feature = "svd-l4x3")]
pub use stm32l4::stm32l4x3 as pac;

#[cfg(feature = "svd-l4x5")]
pub use stm32l4::stm32l4x5 as pac;

#[cfg(feature = "svd-l4x6")]
pub use stm32l4::stm32l4x6 as pac;

#[cfg(feature = "svd-l4r9")]
pub use stm32l4::stm32l4r9 as pac;

// Enable use of interrupt macro
pub use crate::pac::interrupt;

#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
#[path = "adc/f4.rs"]
pub mod adc;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f7")]
#[path = "adc/f7.rs"]
pub mod adc;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
#[path = "adc/l4.rs"]
pub mod adc;

#[cfg(feature = "device-selected")]
#[cfg(feature = "bb")]
pub mod bb;

#[cfg(all(
    feature = "device-selected",
    feature = "can",
    any(feature = "can1", feature = "can2",)
))]
pub mod can;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
#[path = "crc_l4.rs"]
pub mod crc;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
pub mod crc32;
#[cfg(all(feature = "device-selected", feature = "dac"))]
pub mod dac;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
pub mod i2c;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f7")]
#[path = "i2c/f7.rs"]
pub mod i2c;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
#[path = "i2c/l4.rs"]
pub mod i2c;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
pub mod i2s;
#[cfg(all(feature = "device-selected", feature = "usb_fs", feature = "otg-fs"))]
pub mod otg_fs;
#[cfg(all(
    feature = "device-selected",
    any(feature = "usb_hs", docsrs),
    feature = "otg-hs",
))]
pub mod otg_hs;

#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
#[cfg(feature = "rng")]
pub mod rng;

#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
pub mod dma;
#[cfg(feature = "device-selected")]
pub mod dwt;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
#[path = "flash/f4.rs"]
pub mod flash;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f7")]
#[path = "flash/f7.rs"]
pub mod flash;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
#[path = "flash/l4.rs"]
pub mod flash;
#[cfg(feature = "device-selected")]
#[cfg(any(feature = "fmc", feature = "fsmc"))]
#[cfg(feature = "f7")]
pub mod fmc;
#[cfg(all(
    feature = "device-selected",
    feature = "fsmc_lcd",
    any(feature = "fmc", feature = "fsmc")
))]
pub mod fsmc_lcd;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
pub mod lptimer;
#[cfg(feature = "device-selected")]
#[cfg(feature = "ltdc")]
#[cfg(feature = "f7")]
pub mod ltdc;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
#[path = "pwr/l4.rs"]
pub mod pwr;
#[cfg(feature = "device-selected")]
pub mod qei;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f7")]
#[cfg(feature = "dma")]
#[path = "qspi/f7.rs"]
pub mod qspi;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
#[path = "qspi/l4.rs"]
pub mod qspi;
#[cfg(feature = "device-selected")]
pub mod rcc;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f4")]
pub mod rtc;
#[cfg(all(feature = "device-selected", feature = "sdio-host", feature = "sdio"))]
pub mod sdio;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
#[cfg(not(feature = "f3"))]
pub mod signature;
#[cfg(feature = "device-selected")]
#[cfg(feature = "f3")]
pub mod signature_f3;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(all(feature = "device-selected"))]
pub mod syscfg;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod timer;
#[cfg(feature = "device-selected")]
#[cfg(feature = "l4")]
pub mod tsc;
#[cfg(feature = "device-selected")]
#[cfg(feature = "uart4")]
pub mod uart;
#[cfg(feature = "device-selected")]
#[cfg(feature = "usb")]
#[cfg(any(feature = "svd-l412", feature = "svd-l4x2", feature = "svd-l4x3",))]
pub mod usb;
#[cfg(feature = "device-selected")]
pub mod watchdog;

#[cfg(feature = "device-selected")]
mod sealed {
    pub trait Sealed {}
}
#[cfg(feature = "device-selected")]
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}

#[allow(unused)]
use assert;
#[allow(unused)]
use unreachable;
