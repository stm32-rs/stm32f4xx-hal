//! Delays

mod syst;

use cortex_m::peripheral::SYST;

use crate::time::Hertz;

/// Timer as a delay provider (SysTick by default)
pub struct Delay<T = SYST> {
    tim: T,
    clk: Hertz,
}

impl<T> Delay<T> {
    /// Releases the timer resource
    pub fn release(self) -> T {
        self.tim
    }
}

mod timer;
