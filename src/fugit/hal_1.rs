use super::{Delay, Error, Instance};

use embedded_hal_one::delay::blocking::DelayUs;

use fugit::ExtU32;

impl<TIM: Instance, const FREQ: u32> DelayUs for Delay<TIM, FREQ> {
    type Error = Error;

    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        self.delay(us.micros())
    }

    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        self.delay(ms.millis())
    }
}
