//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use cast::u16;
use core::convert::Infallible;
use cortex_m::peripheral::SYST;
use embedded_hal_one::delay::blocking::DelayUs;

use super::{Delay, Wait};

impl DelayUs for Delay<SYST> {
    type Error = Infallible;

    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.clk.0 / 8_000_000);

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

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        self.delay_us(ms * 1_000)
    }
}

impl<TIM> DelayUs for Delay<TIM>
where
    Self: Wait,
{
    type Error = Infallible;

    /// Sleep for up to 2^32-1 microseconds (~71 minutes).
    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        // Set up prescaler so that a tick takes exactly 1 µs.
        //
        // For example, if the clock is set to 48 MHz, with a prescaler of 48
        // we'll get ticks that are 1 µs long. This means that we can write the
        // delay value directly to the auto-reload register (ARR).
        let psc = u16(self.clk.0 / 1_000_000).expect("Prescaler does not fit in u16");
        let arr = us;
        self.wait(psc, arr);

        Ok(())
    }

    /// Sleep for up to (2^32)/2-1 milliseconds (~24 days).
    /// If the `ms` value is larger than 2147483647, the code will panic.
    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        // See next section for explanation why the usable range is reduced.
        assert!(ms <= 2_147_483_647); // (2^32)/2-1

        // Set up prescaler so that a tick takes exactly 0.5 ms.
        //
        // For example, if the clock is set to 48 MHz, with a prescaler of 24'000
        // we'll get ticks that are 0.5 ms long. This means that we can write the
        // delay value multipled by two to the auto-reload register (ARR).
        //
        // Note that we cannot simply use a prescaler value where the tick corresponds
        // to 1 ms, because then a clock of 100 MHz would correspond to a prescaler
        // value of 100'000, which doesn't fit in the 16-bit PSC register.
        //
        // Unfortunately this means that only one half of the full 32-bit range
        // can be used, but 24 days should be plenty of usable delay time.
        let psc = u16(self.clk.0 / 1000 / 2).expect("Prescaler does not fit in u16");

        // Since PSC = 0.5 ms, double the value for the ARR
        let arr = ms << 1;

        self.wait(psc, arr);

        Ok(())
    }
}
