use super::{Counter, Delay, Error, Instance, SysCounter};

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use fugit::{ExtU32, MicrosDurationU32, TimerDurationU32};
use void::Void;

impl<TIM: Instance, const FREQ: u32> DelayUs<u32> for Delay<TIM, FREQ> {
    /// Sleep for `us` microseconds
    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros()).unwrap()
    }
}

impl<TIM: Instance, const FREQ: u32> DelayMs<u32> for Delay<TIM, FREQ> {
    /// Sleep for `ms` milliseconds
    fn delay_ms(&mut self, ms: u32) {
        self.delay(ms.millis()).unwrap()
    }
}

impl<TIM: Instance, const FREQ: u32> DelayUs<u16> for Delay<TIM, FREQ> {
    /// Sleep for `us` microseconds
    fn delay_us(&mut self, us: u16) {
        self.delay((us as u32).micros()).unwrap()
    }
}
impl<TIM: Instance, const FREQ: u32> DelayMs<u16> for Delay<TIM, FREQ> {
    /// Sleep for `ms` milliseconds
    fn delay_ms(&mut self, ms: u16) {
        self.delay((ms as u32).millis()).unwrap()
    }
}

use embedded_hal::timer::{Cancel, CountDown, Periodic};

impl<TIM: Instance, const FREQ: u32> Periodic for Counter<TIM, FREQ> {}

impl<TIM: Instance, const FREQ: u32> CountDown for Counter<TIM, FREQ> {
    type Time = TimerDurationU32<FREQ>;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        self.start(timeout.into()).unwrap()
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        match self.wait() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }
}

impl<TIM: Instance, const FREQ: u32> Cancel for Counter<TIM, FREQ> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}

impl CountDown for SysCounter {
    type Time = MicrosDurationU32;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        self.start(timeout.into()).unwrap()
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        match self.wait() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }
}

impl Cancel for SysCounter {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}
