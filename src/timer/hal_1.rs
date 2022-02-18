//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use core::convert::Infallible;
use embedded_hal_one::delay::blocking::DelayUs;

use super::{Delay, Error, Instance, SysDelay};
use fugit::ExtU32;

impl DelayUs for SysDelay {
    type Error = Infallible;

    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        self.delay(us.micros());

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        self.delay_us(ms * 1_000)
    }
}

impl<TIM: Instance, const FREQ: u32> DelayUs for Delay<TIM, FREQ> {
    type Error = Error;

    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        self.delay(us.micros());
        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        self.delay(ms.millis());
        Ok(())
    }
}
