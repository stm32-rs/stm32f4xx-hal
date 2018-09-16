#![no_std]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![feature(const_fn)]
#![allow(non_camel_case_types)]
#![feature(never_type)]

extern crate bare_metal;
extern crate cast;
extern crate cortex_m;
pub extern crate embedded_hal as hal;

#[macro_use]
pub extern crate nb;
pub use nb::block;
pub extern crate stm32f4;

#[cfg(feature = "stm32f407")]
pub use stm32f4::stm32f407 as stm32;

#[cfg(feature = "stm32f429")]
pub use stm32f4::stm32f429 as stm32;

pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod prelude;
pub mod rcc;
pub mod serial;
pub mod time;
