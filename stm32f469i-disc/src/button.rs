//! On-board user button (PA0)

use crate::hal;

/// User button (PA0) as input with pull-down
pub type Button = hal::gpio::PA0<hal::gpio::Input>;

/// Initialize the user button as pull-down input
pub fn init(pa0: hal::gpio::PA0) -> hal::gpio::PA0<hal::gpio::Input> {
    pa0.into_pull_down_input()
}
