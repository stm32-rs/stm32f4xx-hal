//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use embedded_hal_one::delay::DelayUs;

use super::{Delay, Instance, SysDelay};
use fugit::ExtU32;

impl DelayUs for SysDelay {
    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros());
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl<TIM: Instance, const FREQ: u32> DelayUs for Delay<TIM, FREQ> {
    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros());
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay(ms.millis());
    }
}
