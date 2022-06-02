// RTIC Monotonic impl for the 32-bit timers
use super::{Channel, Event, FTimer, General, Instance, WithPwm};
use crate::rcc::Clocks;
use core::ops::{Deref, DerefMut};
pub use fugit::{self, ExtU32};
use rtic_monotonic::Monotonic;
use systick_monotonic::Systick;

pub struct MonoTimer<TIM, const FREQ: u32>(FTimer<TIM, FREQ>);

impl<TIM, const FREQ: u32> Deref for MonoTimer<TIM, FREQ> {
    type Target = FTimer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TIM, const FREQ: u32> DerefMut for MonoTimer<TIM, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `MonoTimer` with precision of 1 μs (1 MHz sampling)
pub type MonoTimerUs<TIM> = MonoTimer<TIM, 1_000_000>;

impl<TIM: Instance, const FREQ: u32> MonoTimer<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.0
    }
}

pub trait MonoTimerExt: Sized {
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer<Self, FREQ>;
    fn monotonic_us(self, clocks: &Clocks) -> MonoTimer<Self, 1_000_000> {
        self.monotonic::<1_000_000>(clocks)
    }
}

impl<TIM> MonoTimerExt for TIM
where
    Self: Instance + General<Width = u32> + WithPwm,
{
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer<Self, FREQ> {
        FTimer::new(self, clocks).monotonic()
    }
}

pub trait SysMonoTimerExt: Sized {
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> Systick<FREQ>;
    fn monotonic_us(self, clocks: &Clocks) -> Systick<1_000_000> {
        self.monotonic::<1_000_000>(clocks)
    }
}

impl SysMonoTimerExt for crate::pac::SYST {
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> Systick<FREQ> {
        Systick::new(self, clocks.hclk().raw())
    }
}

impl<TIM, const FREQ: u32> FTimer<TIM, FREQ>
where
    TIM: Instance + General<Width = u32> + WithPwm,
{
    pub fn monotonic(mut self) -> MonoTimer<TIM, FREQ> {
        unsafe {
            self.tim.set_auto_reload_unchecked(TIM::max_auto_reload());
        }
        self.tim.trigger_update();
        self.tim.start_no_update();
        MonoTimer(self)
    }
}

impl<TIM, const FREQ: u32> Monotonic for MonoTimer<TIM, FREQ>
where
    TIM: Instance + General<Width = u32> + WithPwm,
{
    type Instant = fugit::TimerInstantU32<FREQ>;
    type Duration = fugit::TimerDurationU32<FREQ>;

    unsafe fn reset(&mut self) {
        self.tim.listen_interrupt(Event::C1, true);
    }

    #[inline(always)]
    fn now(&mut self) -> Self::Instant {
        Self::Instant::from_ticks(self.tim.read_count())
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        TIM::set_cc_value(Channel::C1 as u8, instant.duration_since_epoch().ticks());
    }

    fn clear_compare_flag(&mut self) {
        self.tim.clear_interrupt_flag(Event::C1);
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }
}
