//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use core::convert::Infallible;
use embedded_hal_one::delay::DelayNs;

use super::{Delay, Instance, PwmChannel, SysDelay, WithPwm};
use fugit::ExtU32Ceil;

impl DelayNs for SysDelay {
    fn delay_ns(&mut self, ns: u32) {
        self.delay(ns.nanos_at_least());
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay(ms.millis_at_least());
    }
}

impl<TIM: Instance, const FREQ: u32> DelayNs for Delay<TIM, FREQ> {
    fn delay_ns(&mut self, ns: u32) {
        self.delay(ns.micros_at_least());
    }

    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros_at_least());
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay(ms.millis_at_least());
    }
}

impl<TIM: Instance + WithPwm, const C: u8> embedded_hal_one::pwm::ErrorType for PwmChannel<TIM, C> {
    type Error = Infallible;
}

impl<TIM: Instance + WithPwm, const C: u8> embedded_hal_one::pwm::SetDutyCycle
    for PwmChannel<TIM, C>
{
    fn max_duty_cycle(&self) -> u16 {
        self.get_max_duty()
    }
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty(duty);
        Ok(())
    }
}
