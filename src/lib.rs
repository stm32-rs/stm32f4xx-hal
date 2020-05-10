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
pub use stm32f4::stm32f401 as stm32;

#[cfg(feature = "stm32f405")]
pub use stm32f4::stm32f405 as stm32;

#[cfg(feature = "stm32f407")]
pub use stm32f4::stm32f407 as stm32;

#[cfg(feature = "stm32f410")]
pub use stm32f4::stm32f410 as stm32;

#[cfg(feature = "stm32f411")]
pub use stm32f4::stm32f411 as stm32;

#[cfg(feature = "stm32f412")]
pub use stm32f4::stm32f412 as stm32;

#[cfg(feature = "stm32f413")]
pub use stm32f4::stm32f413 as stm32;

#[cfg(feature = "stm32f415")]
pub use stm32f4::stm32f405 as stm32;

#[cfg(feature = "stm32f417")]
pub use stm32f4::stm32f407 as stm32;

#[cfg(feature = "stm32f423")]
pub use stm32f4::stm32f413 as stm32;

#[cfg(feature = "stm32f427")]
pub use stm32f4::stm32f427 as stm32;

#[cfg(feature = "stm32f429")]
pub use stm32f4::stm32f429 as stm32;

#[cfg(feature = "stm32f437")]
pub use stm32f4::stm32f427 as stm32;

#[cfg(feature = "stm32f439")]
pub use stm32f4::stm32f429 as stm32;

#[cfg(feature = "stm32f446")]
pub use stm32f4::stm32f446 as stm32;

#[cfg(feature = "stm32f469")]
pub use stm32f4::stm32f469 as stm32;

#[cfg(feature = "stm32f479")]
pub use stm32f4::stm32f469 as stm32;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use crate::stm32::interrupt;

#[cfg(feature = "device-selected")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub mod bb;
#[cfg(feature = "device-selected")]
pub mod delay;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(all(
    feature = "usb_fs",
    any(
        feature = "stm32f401",
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f411",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
    )
))]
pub mod otg_fs;
#[cfg(all(
    any(feature = "usb_hs", docsrs),
    any(
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
    )
))]
pub mod otg_hs;

#[cfg(all(
    feature = "device-selected",
    not(any(
        feature = "stm32f401",
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f446",
    ))
))]
pub mod rng;

#[cfg(feature = "device-selected")]
pub mod dwt;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
pub mod pwm;
#[cfg(feature = "device-selected")]
pub mod qei;
#[cfg(feature = "device-selected")]
pub mod rcc;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
pub mod signature;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod timer;
#[cfg(feature = "device-selected")]
pub mod watchdog;
