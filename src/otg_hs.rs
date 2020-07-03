//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.
//!
//! Note that only full-speed mode is supported,
//! external high-speed PHY is not supported.

use crate::stm32;

use crate::gpio::{
    gpiob::{PB14, PB15},
    Alternate, AF12,
};
use crate::time::Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::UsbPeripheral;

pub struct USB {
    pub usb_global: stm32::OTG_HS_GLOBAL,
    pub usb_device: stm32::OTG_HS_DEVICE,
    pub usb_pwrclk: stm32::OTG_HS_PWRCLK,
    pub pin_dm: PB14<Alternate<AF12>>,
    pub pin_dp: PB15<Alternate<AF12>>,
    pub hclk: Hertz,
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = stm32::OTG_HS_GLOBAL::ptr() as *const ();

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
    #[cfg(feature = "stm32f446")]
    const ENDPOINT_COUNT: usize = 9;

    fn enable() {
        let rcc = unsafe { &*stm32::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.ahb1enr.modify(|_, w| w.otghsen().set_bit());

            // Reset USB peripheral
            rcc.ahb1rstr.modify(|_, w| w.otghsrst().set_bit());
            rcc.ahb1rstr.modify(|_, w| w.otghsrst().clear_bit());
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.0
    }
}

pub type UsbBusType = UsbBus<USB>;
