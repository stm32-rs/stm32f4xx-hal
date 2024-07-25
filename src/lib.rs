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
pub use crate::pac::interrupt;

pub mod adc;
pub mod bb;
#[cfg(all(feature = "can", any(feature = "can1", feature = "can2")))]
pub mod can;
pub mod crc32;
#[cfg(feature = "dac")]
pub mod dac;
#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c;
pub mod gpio;
pub mod i2c;
pub mod i2s;
#[cfg(all(feature = "usb_fs", feature = "otg-fs"))]
pub mod otg_fs;
#[cfg(all(any(feature = "usb_hs", docsrs), feature = "otg-hs"))]
pub mod otg_hs;

#[cfg(feature = "rng")]
pub mod rng;

pub mod dma;
pub mod dwt;
pub mod flash;
#[cfg(all(feature = "fsmc_lcd", any(feature = "fmc", feature = "fsmc")))]
pub mod fsmc_lcd;
#[cfg(all(feature = "dma2d", feature = "ltdc"))]
pub mod ltdc;
pub mod prelude;
pub mod qei;
#[cfg(feature = "quadspi")]
pub mod qspi;
pub mod rcc;
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
#[cfg(feature = "uart4")]
pub mod uart;
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
