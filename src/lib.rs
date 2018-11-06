#![no_std]
#![allow(non_camel_case_types)]

extern crate bare_metal;
extern crate cast;
extern crate void;
extern crate cortex_m;
pub extern crate embedded_hal as hal;

pub extern crate nb;
pub use nb::block;

pub extern crate stm32f4;

#[cfg(feature = "stm32f401")]
pub use stm32f4::stm32f401 as stm32;

#[cfg(feature = "stm32f407")]
pub use stm32f4::stm32f407 as stm32;

#[cfg(feature = "stm32f412")]
pub use stm32f4::stm32f412 as stm32;

#[cfg(feature = "stm32f429")]
pub use stm32f4::stm32f429 as stm32;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use stm32f4::interrupt;

#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod delay;
#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod gpio;
#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod i2c;
#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod prelude;
#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod rcc;
#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod serial;
pub mod time;
#[cfg(any(feature = "stm32f401", feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
pub mod timer;
