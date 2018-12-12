#![no_std]
#![allow(non_camel_case_types)]

extern crate bare_metal;
extern crate cast;
extern crate cortex_m;
pub extern crate embedded_hal as hal;
extern crate void;

pub extern crate nb;
pub use nb::block;

pub extern crate stm32f4;

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
pub use stm32f4::interrupt;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pub mod delay;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pub mod gpio;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f429",
    feature = "stm32f411"
))]
pub mod i2c;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f429",
    feature = "stm32f411"
))]
pub mod prelude;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pub mod rcc;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f429",
    feature = "stm32f411"
))]
pub mod serial;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pub mod spi;
pub mod time;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pub mod timer;
