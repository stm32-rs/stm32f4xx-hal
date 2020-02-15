//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.
//!
//! Note that only full-speed mode is supported,
//! external high-speed PHY is not supported.

use crate::stm32;

use crate::gpio::{Alternate, AF12, gpiob::{PB14, PB15}};

use synopsys_usb_otg::UsbPeripheral;
pub use synopsys_usb_otg::UsbBus;

pub struct USB {
    pub usb_global: stm32::OTG_HS_GLOBAL,
    pub usb_device: stm32::OTG_HS_DEVICE,
    pub usb_pwrclk: stm32::OTG_HS_PWRCLK,
    pub pin_dm: PB14<Alternate<AF12>>,
    pub pin_dp: PB15<Alternate<AF12>>,
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = stm32::OTG_HS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024;

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
}

pub type UsbBusType = UsbBus<USB>;
