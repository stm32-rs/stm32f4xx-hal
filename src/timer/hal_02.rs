//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use embedded_hal_02::{
    blocking::delay::{DelayMs, DelayUs},
    timer::{Cancel, CountDown, Periodic},
};
use fugit::{ExtU32Ceil, HertzU32 as Hertz, TimerDurationU32};
use void::Void;

use super::{
    CPin, Channel, Counter, CounterHz, Delay, Error, Instance, Pins, Pwm, PwmChannel, PwmHz,
    SysCounter, SysCounterHz, SysDelay, WithPwm,
};

impl DelayUs<u32> for SysDelay {
    #[inline]
    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros_at_least())
    }
}

impl DelayMs<u32> for SysDelay {
    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        self.delay(ms.millis_at_least());
    }
}

impl DelayUs<u16> for SysDelay {
    #[inline]
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayMs<u16> for SysDelay {
    #[inline]
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayUs<u8> for SysDelay {
    #[inline]
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}

impl DelayMs<u8> for SysDelay {
    #[inline]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

impl<TIM> Periodic for CounterHz<TIM> {}
impl Periodic for SysCounterHz {}
impl<const FREQ: u32> Periodic for SysCounter<FREQ> {}

impl CountDown for SysCounterHz {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
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

impl Cancel for SysCounterHz {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}

impl<TIM: Instance> CountDown for CounterHz<TIM> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
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

impl<TIM: Instance> Cancel for CounterHz<TIM> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}

impl<const FREQ: u32> CountDown for SysCounter<FREQ> {
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

impl<const FREQ: u32> Cancel for SysCounter<FREQ> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}

impl<TIM, const C: u8, Otype> embedded_hal_02::PwmPin for PwmChannel<TIM, C, Otype>
where
    TIM: CPin<C> + Instance + WithPwm,
{
    type Duty = u16;

    fn disable(&mut self) {
        self.disable()
    }
    fn enable(&mut self) {
        self.enable()
    }
    fn get_duty(&self) -> Self::Duty {
        self.get_duty()
    }
    fn get_max_duty(&self) -> Self::Duty {
        self.get_max_duty()
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        self.set_duty(duty)
    }
}

impl<TIM, PINS> embedded_hal_02::Pwm for PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    type Channel = Channel;
    type Duty = u16;
    type Time = Hertz;

    fn enable(&mut self, channel: Self::Channel) {
        self.enable(channel)
    }

    fn disable(&mut self, channel: Self::Channel) {
        self.disable(channel)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        self.get_duty(channel)
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.set_duty(channel, duty)
    }

    /// If `0` returned means max_duty is 2^16
    fn get_max_duty(&self) -> Self::Duty {
        self.get_max_duty()
    }

    fn get_period(&self) -> Self::Time {
        self.get_period()
    }

    fn set_period<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        self.set_period(period.into())
    }
}

impl<TIM: Instance, const FREQ: u32> DelayUs<u32> for Delay<TIM, FREQ> {
    /// Sleep for `us` microseconds
    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros_at_least())
    }
}

impl<TIM: Instance, const FREQ: u32> DelayMs<u32> for Delay<TIM, FREQ> {
    /// Sleep for `ms` milliseconds
    fn delay_ms(&mut self, ms: u32) {
        self.delay(ms.millis_at_least())
    }
}

impl<TIM: Instance, const FREQ: u32> DelayUs<u16> for Delay<TIM, FREQ> {
    /// Sleep for `us` microseconds
    fn delay_us(&mut self, us: u16) {
        self.delay((us as u32).micros_at_least())
    }
}
impl<TIM: Instance, const FREQ: u32> DelayMs<u16> for Delay<TIM, FREQ> {
    /// Sleep for `ms` milliseconds
    fn delay_ms(&mut self, ms: u16) {
        self.delay((ms as u32).millis_at_least())
    }
}

impl<TIM: Instance, const FREQ: u32> DelayUs<u8> for Delay<TIM, FREQ> {
    /// Sleep for `us` microseconds
    fn delay_us(&mut self, us: u8) {
        self.delay((us as u32).micros_at_least())
    }
}
impl<TIM: Instance, const FREQ: u32> DelayMs<u8> for Delay<TIM, FREQ> {
    /// Sleep for `ms` milliseconds
    fn delay_ms(&mut self, ms: u8) {
        self.delay((ms as u32).millis_at_least())
    }
}

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

impl<TIM, PINS, const FREQ: u32> embedded_hal_02::Pwm for Pwm<TIM, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    type Channel = Channel;
    type Duty = u16;
    type Time = TimerDurationU32<FREQ>;

    fn enable(&mut self, channel: Self::Channel) {
        self.enable(channel)
    }

    fn disable(&mut self, channel: Self::Channel) {
        self.disable(channel)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        self.get_duty(channel)
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.set_duty(channel, duty)
    }

    /// If `0` returned means max_duty is 2^16
    fn get_max_duty(&self) -> Self::Duty {
        self.get_max_duty()
    }

    fn get_period(&self) -> Self::Time {
        self.get_period()
    }

    fn set_period<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        self.set_period(period.into())
    }
}
