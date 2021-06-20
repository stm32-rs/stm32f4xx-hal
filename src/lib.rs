#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "This crate requires one of the following device features enabled:
        stm32f401
        stm32f405
        stm32f407
        stm32f410
        stm32f411
        stm32f412
        stm32f413
        stm32f415
        stm32f417
        stm32f423
        stm32f427
        stm32f429
        stm32f437
        stm32f439
        stm32f446
        stm32f469
        stm32f479"
);

pub use embedded_hal as hal;

pub use nb;
pub use nb::block;

#[cfg(feature = "stm32f401")]
pub use stm32f4::stm32f401 as pac;

#[cfg(feature = "stm32f405")]
pub use stm32f4::stm32f405 as pac;

#[cfg(feature = "stm32f407")]
pub use stm32f4::stm32f407 as pac;

#[cfg(feature = "stm32f410")]
pub use stm32f4::stm32f410 as pac;

#[cfg(feature = "stm32f411")]
pub use stm32f4::stm32f411 as pac;

#[cfg(feature = "stm32f412")]
pub use stm32f4::stm32f412 as pac;

#[cfg(feature = "stm32f413")]
pub use stm32f4::stm32f413 as pac;

#[cfg(feature = "stm32f415")]
pub use stm32f4::stm32f405 as pac;

#[cfg(feature = "stm32f417")]
pub use stm32f4::stm32f407 as pac;

#[cfg(feature = "stm32f423")]
pub use stm32f4::stm32f413 as pac;

#[cfg(feature = "stm32f427")]
pub use stm32f4::stm32f427 as pac;

#[cfg(feature = "stm32f429")]
pub use stm32f4::stm32f429 as pac;

#[cfg(feature = "stm32f437")]
pub use stm32f4::stm32f427 as pac;

#[cfg(feature = "stm32f439")]
pub use stm32f4::stm32f429 as pac;

#[cfg(feature = "stm32f446")]
pub use stm32f4::stm32f446 as pac;

#[cfg(feature = "stm32f469")]
pub use stm32f4::stm32f469 as pac;

#[cfg(feature = "stm32f479")]
pub use stm32f4::stm32f469 as pac;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use crate::pac::interrupt;

#[cfg(feature = "device-selected")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub mod bb;
#[cfg(all(
    feature = "device-selected",
    feature = "can",
    any(feature = "can1", feature = "can2",)
))]
pub mod can;
#[cfg(feature = "device-selected")]
pub mod crc32;
#[cfg(all(feature = "device-selected", feature = "dac"))]
pub mod dac;
#[cfg(feature = "device-selected")]
pub mod delay;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(all(feature = "device-selected", feature = "i2s"))]
pub mod i2s;
#[cfg(all(feature = "device-selected", feature = "usb_fs", feature = "otg-fs"))]
pub mod otg_fs;
#[cfg(all(
    feature = "device-selected",
    any(feature = "usb_hs", docsrs),
    feature = "otg-hs",
))]
pub mod otg_hs;

#[cfg(all(feature = "device-selected", feature = "rng"))]
pub mod rng;

#[cfg(feature = "device-selected")]
pub use pac as stm32;

#[cfg(feature = "device-selected")]
pub mod dma;
#[cfg(feature = "device-selected")]
pub mod dwt;
#[cfg(all(
    feature = "device-selected",
    feature = "fsmc_lcd",
    any(feature = "fmc", feature = "fsmc")
))]
pub mod fsmc_lcd;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
pub mod pwm;
#[cfg(feature = "device-selected")]
pub mod qei;
#[cfg(feature = "device-selected")]
pub mod rcc;
#[cfg(feature = "device-selected")]
pub mod rtc;
#[cfg(all(feature = "device-selected", feature = "sdio-host", feature = "sdio"))]
pub mod sdio;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
pub mod signature;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub mod syscfg;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod timer;
#[cfg(feature = "device-selected")]
pub mod watchdog;
