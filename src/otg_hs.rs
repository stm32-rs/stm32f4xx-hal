//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.
//!
//! Note that only full-speed mode is supported,
//! external high-speed PHY is not supported.

use crate::pac;

use crate::gpio::alt::otg_hs as alt;
use crate::rcc::{Clocks, Enable, Reset};
use fugit::HertzU32 as Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::UsbPeripheral;

pub struct USB {
    pub usb_global: pac::OTG_HS_GLOBAL,
    pub usb_device: pac::OTG_HS_DEVICE,
    pub usb_pwrclk: pac::OTG_HS_PWRCLK,
    pub pin_dm: alt::Dm,
    pub pin_dp: alt::Dp,
    pub hclk: Hertz,
}

impl USB {
    pub fn new(
        periphs: (pac::OTG_HS_GLOBAL, pac::OTG_HS_DEVICE, pac::OTG_HS_PWRCLK),
        pins: (impl Into<alt::Dm>, impl Into<alt::Dp>),
        clocks: &Clocks,
    ) -> Self {
        Self {
            usb_global: periphs.0,
            usb_device: periphs.1,
            usb_pwrclk: periphs.2,
            pin_dm: pins.0.into(),
            pin_dp: pins.1.into(),
            hclk: clocks.hclk(),
        }
    }
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = pac::OTG_HS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024;

    #[cfg(any(feature = "gpio-f417", feature = "gpio-f427",))]
    const ENDPOINT_COUNT: usize = 6;
    #[cfg(any(feature = "gpio-f446", feature = "gpio-f469"))]
    const ENDPOINT_COUNT: usize = 9;

    fn enable() {
        cortex_m::interrupt::free(|_| {
            unsafe {
                // Enable USB peripheral
                pac::OTG_HS_GLOBAL::enable_unchecked();
                // Reset USB peripheral
                pac::OTG_HS_GLOBAL::reset_unchecked();
            }
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.raw()
    }
}

pub type UsbBusType = UsbBus<USB>;
