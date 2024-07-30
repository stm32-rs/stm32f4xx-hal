//! USB peripheral
//!
//! Requires the `stm32-usbd` feature.
//!
//! See <https://github.com/stm32-rs/stm32l4xx-hal/tree/master/examples>
//! for usage examples.

use crate::gpio::alt::usb as alt;
use crate::pac;
use crate::rcc::{Enable, Reset};
use stm32_usbd::UsbPeripheral;

pub use stm32_usbd::UsbBus;

pub struct USB {
    pub usb: pac::USB,
    pub pin_dm: alt::Dm,
    pub pin_dp: alt::Dp,
}

impl USB {
    pub fn new(usb: pac::USB, pins: (impl Into<alt::Dm>, impl Into<alt::Dp>)) -> Self {
        Self {
            usb,
            pin_dm: pins.0.into(),
            pin_dp: pins.1.into(),
        }
    }
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = pac::USB::ptr() as *const ();
    const DP_PULL_UP_FEATURE: bool = true;
    const EP_MEMORY: *const () = 0x4000_6c00 as _;
    const EP_MEMORY_SIZE: usize = 1024;
    const EP_MEMORY_ACCESS_2X16: bool = true;

    fn enable() {
        cortex_m::interrupt::free(|_| unsafe {
            // Enable USB peripheral
            pac::USB::enable_unchecked();

            // Reset USB peripheral
            pac::USB::reset_unchecked();
        });
    }

    fn startup_delay() {
        // There is a chip specific startup delay. For STM32F103xx it's 1µs and this should wait for
        // at least that long.
        cortex_m::asm::delay(72);
    }
}

pub type UsbBusType = UsbBus<USB>;
