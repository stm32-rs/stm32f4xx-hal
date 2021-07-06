//! System timer (SysTick) as a delay provider.

use cast::u32;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use super::infallible::{DelayMs, DelayUs};
use crate::rcc::Clocks;

use super::Delay;

impl Delay<SYST> {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut tim: SYST, clocks: Clocks) -> Self {
        tim.set_clock_source(SystClkSource::External);
        Self { tim, clocks }
    }

    #[deprecated(since = "0.10.0", note = "Please use release instead")]
    pub fn free(self) -> SYST {
        self.release()
    }

    /// Releases the system timer (SysTick) resource
    pub fn release(self) -> SYST {
        self.tim
    }
}

impl DelayMs<u32> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32(ms));
    }
}

impl DelayMs<u8> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32(ms));
    }
}

impl DelayUs<u32> for Delay<SYST> {
    fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.clocks.hclk().0 / 8_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.tim.set_reload(current_rvr);
            self.tim.clear_current();
            self.tim.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.tim.has_wrapped() {}

            self.tim.disable_counter();
        }
    }
}

impl DelayUs<u16> for Delay<SYST> {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl DelayUs<u8> for Delay<SYST> {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}
