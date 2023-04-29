//! Multi device hardware abstraction on top of the peripheral access API for the STMicro STM32F4 series microcontrollers.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
#![no_std]
#![allow(non_camel_case_types)]

use enumflags2::{BitFlag, BitFlags};

pub use embedded_hal as hal;
pub use embedded_hal_02 as hal_02;

pub use nb;
pub use nb::block;

#[cfg(feature = "svd-f215")]
pub use stm32f2::stm32f215 as pac;

#[cfg(feature = "svd-f217")]
pub use stm32f2::stm32f217 as pac;

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

// Enable use of interrupt macro
pub use crate::pac::interrupt;

#[cfg(any(feature = "f4", feature = "f7"))]
pub mod adc;
#[cfg(feature = "l4")]
#[path = "adc/l4.rs"]
pub mod adc;
#[cfg(feature = "bb")]
pub mod bb;
#[cfg(all(feature = "can", any(feature = "can1", feature = "can2")))]
pub mod can;
#[cfg(feature = "l4")]
#[path = "crc_l4.rs"]
pub mod crc;
#[cfg(feature = "f4")]
pub mod crc32;
#[cfg(feature = "dac")]
pub mod dac;
#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c;
pub mod gpio;
#[cfg(feature = "i2c_v1")]
pub mod i2c;
#[cfg(feature = "i2c_v2")]
#[path = "fmpi2c.rs"]
pub mod i2c;
#[cfg(feature = "f4")]
pub mod i2s;
#[cfg(all(feature = "usb_fs", feature = "otg-fs"))]
pub mod otg_fs;
#[cfg(all(any(feature = "usb_hs", docsrs), feature = "otg-hs"))]
pub mod otg_hs;

#[cfg(feature = "f4")]
#[cfg(feature = "rng")]
pub mod rng;

#[cfg(feature = "dma")]
pub mod dma;
pub mod dwt;
#[cfg(feature = "f4")]
#[path = "flash/f4.rs"]
pub mod flash;
#[cfg(feature = "f7")]
#[path = "flash/f7.rs"]
pub mod flash;
#[cfg(feature = "l4")]
#[path = "flash/l4.rs"]
pub mod flash;
#[cfg(any(feature = "fmc", feature = "fsmc"))]
#[cfg(feature = "f7")]
pub mod fmc;
#[cfg(all(feature = "fsmc_lcd", any(feature = "fmc", feature = "fsmc")))]
pub mod fsmc_lcd;
#[cfg(feature = "l4")]
pub mod lptimer;
#[cfg(all(feature = "dma2d", feature = "ltdc"))]
pub mod ltdc;
pub mod prelude;
#[cfg(feature = "l4")]
#[path = "pwr/l4.rs"]
pub mod pwr;
pub mod qei;
#[cfg(feature = "quadspi")]
pub mod qspi;
pub mod rcc;
#[cfg(feature = "f4")]
pub mod rtc;
#[cfg(feature = "sai")]
pub mod sai;
#[cfg(all(feature = "sdio-host", feature = "sdio"))]
pub mod sdio;
pub mod serial;
pub mod signature;
pub mod spi;
pub mod syscfg;
pub mod time;
pub mod timer;
#[cfg(feature = "l4")]
pub mod tsc;
#[cfg(feature = "uart4")]
pub mod uart;
#[cfg(feature = "usb")]
#[cfg(any(feature = "svd-l412", feature = "svd-l4x2", feature = "svd-l4x3",))]
pub mod usb;
pub mod watchdog;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}

pub trait ReadFlags {
    /// Enum of bit flags
    type Flag: BitFlag;

    /// Get all interrupts flags a once.
    fn flags(&self) -> BitFlags<Self::Flag>;
}

pub trait ClearFlags {
    /// Enum of manually clearable flags
    type Flag: BitFlag;

    /// Clear interrupts flags with `Self::Flags`s
    ///
    /// If event flag is not cleared, it will immediately retrigger interrupt
    /// after interrupt handler has finished.
    fn clear_flags(&mut self, flags: impl Into<BitFlags<Self::Flag>>);

    /// Clears all interrupts flags
    #[inline(always)]
    fn clear_all_flags(&mut self) {
        self.clear_flags(BitFlags::ALL)
    }
}

pub trait Listen {
    /// Enum of bit flags associated with events
    type Event: BitFlag;

    /// Start listening for `Event`s
    ///
    /// Note, you will also have to enable the appropriate interrupt in the NVIC to start
    /// receiving events.
    fn listen(&mut self, event: impl Into<BitFlags<Self::Event>>);

    /// Start listening for `Event`s, stop all other
    ///
    /// Note, you will also have to enable the appropriate interrupt in the NVIC to start
    /// receiving events.
    fn listen_only(&mut self, event: impl Into<BitFlags<Self::Event>>);

    /// Stop listening for `Event`s
    fn unlisten(&mut self, event: impl Into<BitFlags<Self::Event>>);

    /// Start listening all `Event`s
    #[inline(always)]
    fn listen_all(&mut self) {
        self.listen(BitFlags::ALL)
    }

    /// Stop listening all `Event`s
    #[inline(always)]
    fn unlisten_all(&mut self) {
        self.unlisten(BitFlags::ALL)
    }
}

pub trait Ptr {
    /// RegisterBlock structure
    type RB;
    /// Return the pointer to the register block
    fn ptr() -> *const Self::RB;
}

pub trait Steal {
    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances, for example by only
    /// accessing read-only or write-only registers, or by consuming the
    /// original peripheral and using critical sections to coordinate
    /// access between multiple new instances.
    ///
    /// Additionally the HAL may rely on only one
    /// peripheral instance existing to ensure memory safety; ensure
    /// no stolen instances are passed to such software.
    unsafe fn steal() -> Self;
}
