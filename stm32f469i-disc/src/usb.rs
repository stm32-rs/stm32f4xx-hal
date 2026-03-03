//! USB OTG FS initialization for STM32F469I-DISCO board
//!
//! Provides USB peripheral setup using the OTG FS interface.
//! Uses PA11 (DM) and PA12 (DP) pins.
//!
//! # Usage
//!
//! ```no_run
//! let gpioa = dp.GPIOA.split(&mut rcc);
//! let usb = usb::init(
//!     (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
//!     gpioa.pa11,
//!     gpioa.pa12,
//!     &rcc.clocks,
//! );
//! // Pass to SerialDriver or use with UsbBus::new(usb, ep_memory)
//! ```

use crate::hal;
use crate::hal::otg_fs::USB;

/// Initialize the USB OTG FS peripheral.
///
/// Configures PA11 (DM) and PA12 (DP) in alternate function mode
/// for USB device operation.
///
/// # Arguments
///
/// * `periphs` - Tuple of (OTG_FS_GLOBAL, OTG_FS_DEVICE, OTG_FS_PWRCLK)
/// * `pa11` - USB DM pin (PA11)
/// * `pa12` - USB DP pin (PA12)
/// * `clocks` - System clocks reference
///
/// # Returns
///
/// A `USB` struct ready for use with `UsbBus::new(usb, ep_memory)`.
pub fn init(
    periphs: (
        hal::pac::OTG_FS_GLOBAL,
        hal::pac::OTG_FS_DEVICE,
        hal::pac::OTG_FS_PWRCLK,
    ),
    pa11: hal::gpio::PA11,
    pa12: hal::gpio::PA12,
    clocks: &hal::rcc::Clocks,
) -> USB {
    USB::new(periphs, (pa11, pa12), clocks)
}
