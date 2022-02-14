//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use cast::{u16, u32};
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use super::{Delay, Wait};

impl DelayUs<u32> for Delay<SYST> {
    fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.clk.raw() / 8_000_000);

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

impl DelayMs<u32> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayUs<u16> for Delay<SYST> {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl DelayMs<u16> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32(ms));
    }
}

impl DelayUs<u8> for Delay<SYST> {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}

impl DelayMs<u8> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32(ms));
    }
}

impl<TIM> DelayUs<u32> for Delay<TIM>
where
    Self: Wait,
{
    /// Sleep for up to 2^32-1 microseconds (~71 minutes).
    fn delay_us(&mut self, us: u32) {
        // Set up prescaler so that a tick takes exactly 1 µs.
        //
        // For example, if the clock is set to 48 MHz, with a prescaler of 48
        // we'll get ticks that are 1 µs long. This means that we can write the
        // delay value directly to the auto-reload register (ARR).
        let psc = u16(self.clk.raw() / 1_000_000).expect("Prescaler does not fit in u16");
        let arr = us;
        self.wait(psc, arr);
    }
}

impl<TIM> DelayMs<u32> for Delay<TIM>
where
    Self: Wait,
{
    /// Sleep for up to (2^32)/2-1 milliseconds (~24 days).
    /// If the `ms` value is larger than 2147483647, the code will panic.
    fn delay_ms(&mut self, ms: u32) {
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
        let psc = u16(self.clk.raw() / 1000 / 2).expect("Prescaler does not fit in u16");

        // Since PSC = 0.5 ms, double the value for the ARR
        let arr = ms << 1;

        self.wait(psc, arr);
    }
}

impl<TIM> DelayUs<u16> for Delay<TIM>
where
    Self: Wait,
{
    /// Sleep for up to 2^16-1 microseconds (~65 milliseconds).
    fn delay_us(&mut self, us: u16) {
        // See DelayUs<u32> for explanations.
        let psc = u16(self.clk.raw() / 1_000_000).expect("Prescaler does not fit in u16");
        let arr = u32(us);
        self.wait(psc, arr);
    }
}

impl<TIM> DelayMs<u16> for Delay<TIM>
where
    Self: Wait,
{
    /// Sleep for up to (2^16)-1 milliseconds (~65 seconds).
    fn delay_ms(&mut self, ms: u16) {
        // See DelayMs<u32> for explanations. Since the value range is only 16 bit,
        // we don't need an assert here.
        let psc = u16(self.clk.raw() / 1000 / 2).expect("Prescaler does not fit in u16");
        let arr = u32(ms) << 1;
        self.wait(psc, arr);
    }
}
