//! Delays

use crate::hal::blocking::delay::{DelayMs, DelayUs};

mod syst;

use cortex_m::peripheral::SYST;

use crate::rcc::Clocks;

/// Timer as a delay provider (SysTick by default)
pub struct Delay<TIM = SYST> {
    tim: TIM,
    clocks: Clocks,
}

mod timer;

mod infallible {
    pub trait DelayMs<UXX> {
        fn delay_ms(&mut self, ms: UXX);
    }

    pub trait DelayUs<UXX> {
        fn delay_us(&mut self, us: UXX);
    }
}

impl<TIM> Delay<TIM> {
    pub fn delay_ms<T>(&mut self, ms: T)
    where
        Self: infallible::DelayMs<T>,
    {
        <Self as infallible::DelayMs<T>>::delay_ms(self, ms)
    }
    pub fn delay_us<T>(&mut self, us: T)
    where
        Self: infallible::DelayUs<T>,
    {
        <Self as infallible::DelayUs<T>>::delay_us(self, us)
    }
}

#[cfg(not(feature = "ehal1"))]
impl<TIM, UXX> DelayMs<UXX> for Delay<TIM>
where
    Self: infallible::DelayMs<UXX>,
{
    fn delay_ms(&mut self, ms: UXX) {
        <Self as infallible::DelayMs<UXX>>::delay_ms(self, ms);
    }
}
#[cfg(feature = "ehal1")]
impl<TIM, UXX> DelayMs<UXX> for Delay<TIM>
where
    Self: infallible::DelayMs<UXX>,
{
    type Error = core::convert::Infallible;
    fn delay_ms(&mut self, ms: UXX) -> Result<(), Self::Error> {
        <Self as infallible::DelayMs<UXX>>::delay_ms(self, ms);
        Ok(())
    }
}

#[cfg(not(feature = "ehal1"))]
impl<TIM, UXX> DelayUs<UXX> for Delay<TIM>
where
    Self: infallible::DelayUs<UXX>,
{
    fn delay_us(&mut self, us: UXX) {
        <Self as infallible::DelayUs<UXX>>::delay_us(self, us);
    }
}
#[cfg(feature = "ehal1")]
impl<TIM, UXX> DelayUs<UXX> for Delay<TIM>
where
    Self: infallible::DelayUs<UXX>,
{
    type Error = core::convert::Infallible;
    fn delay_us(&mut self, us: UXX) -> Result<(), Self::Error> {
        <Self as infallible::DelayUs<UXX>>::delay_us(self, us);
        Ok(())
    }
}
