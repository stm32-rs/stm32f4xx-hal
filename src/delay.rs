//! Delays

use cast::u32;
use core::convert::Infallible;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::rcc::Clocks;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// System timer (SysTick) as a delay provider
pub struct Delay {
    clocks: Clocks,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clocks: Clocks) -> Self {
        syst.set_clock_source(SystClkSource::External);

        Delay { syst, clocks }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    type Error = Infallible;

    fn try_delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        self.try_delay_us(ms * 1_000)?;
        Ok(())
    }
}

impl DelayMs<u16> for Delay {
    type Error = Infallible;

    fn try_delay_ms(&mut self, ms: u16) -> Result<(), Self::Error> {
        self.try_delay_ms(u32(ms))?;
        Ok(())
    }
}

impl DelayMs<u8> for Delay {
    type Error = Infallible;

    fn try_delay_ms(&mut self, ms: u8) -> Result<(), Self::Error> {
        self.try_delay_ms(u32(ms))?;
        Ok(())
    }
}

impl DelayUs<u32> for Delay {
    type Error = Infallible;

    fn try_delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.clocks.hclk().0 / 8_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }

        Ok(())
    }
}

impl DelayUs<u16> for Delay {
    type Error = Infallible;

    fn try_delay_us(&mut self, us: u16) -> Result<(), Self::Error> {
        self.try_delay_us(u32(us))
    }
}

impl DelayUs<u8> for Delay {
    type Error = Infallible;

    fn try_delay_us(&mut self, us: u8) -> Result<(), Self::Error> {
        self.try_delay_us(u32(us))
    }
}
