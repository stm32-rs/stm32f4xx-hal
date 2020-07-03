//! USB OTG full-speed peripheral
//!
//! Requires the `usb_fs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.

use crate::stm32;

use crate::gpio::{
    gpioa::{PA11, PA12},
    Alternate, AF10,
};
use crate::time::Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::UsbPeripheral;

pub struct USB {
    pub usb_global: stm32::OTG_FS_GLOBAL,
    pub usb_device: stm32::OTG_FS_DEVICE,
    pub usb_pwrclk: stm32::OTG_FS_PWRCLK,
    pub pin_dm: PA11<Alternate<AF10>>,
    pub pin_dp: PA12<Alternate<AF10>>,
    pub hclk: Hertz,
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = stm32::OTG_FS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = false;
    const FIFO_DEPTH_WORDS: usize = 320;

    #[cfg(any(
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
    ))]
    const ENDPOINT_COUNT: usize = 4;
    #[cfg(feature = "stm32f446")]
    const ENDPOINT_COUNT: usize = 6;

    fn enable() {
        let rcc = unsafe { &*stm32::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.ahb2enr.modify(|_, w| w.otgfsen().set_bit());

            // Reset USB peripheral
            rcc.ahb2rstr.modify(|_, w| w.otgfsrst().set_bit());
            rcc.ahb2rstr.modify(|_, w| w.otgfsrst().clear_bit());
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.0
    }
}

pub type UsbBusType = UsbBus<USB>;
