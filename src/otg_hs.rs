//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.
//!
//! Note that only full-speed mode is supported,
//! external high-speed PHY is not supported.

use crate::pac;

use crate::gpio::alt::otg_hs as alt;
use crate::rcc::{Enable, Reset};
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

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = pac::OTG_HS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024;

    #[cfg(any(
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
    ))]
    const ENDPOINT_COUNT: usize = 6;
    #[cfg(any(feature = "stm32f446", feature = "stm32f469", feature = "stm32f479"))]
    const ENDPOINT_COUNT: usize = 9;

    fn enable() {
        let rcc = unsafe { &*pac::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            pac::OTG_HS_GLOBAL::enable(rcc);
            // Reset USB peripheral
            pac::OTG_HS_GLOBAL::reset(rcc);
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.raw()
    }
}

pub type UsbBusType = UsbBus<USB>;
