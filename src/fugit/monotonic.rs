// RTIC Monotonic impl for the 32-bit timers
use super::{Instance, Timer};
use crate::rcc::Clocks;
use core::ops::{Deref, DerefMut};
pub use fugit::{self, ExtU32};
use rtic_monotonic::Monotonic;

pub struct MonoTimer<TIM, const FREQ: u32>(Timer<TIM, FREQ>);

impl<TIM, const FREQ: u32> Deref for MonoTimer<TIM, FREQ> {
    type Target = Timer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TIM, const FREQ: u32> DerefMut for MonoTimer<TIM, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `MonoTimer` with sampling of 1 MHz
pub type MonoTimerUs<TIM> = MonoTimer<TIM, 1_000_000>;

impl<TIM: Instance, const FREQ: u32> MonoTimer<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> Timer<TIM, FREQ> {
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

macro_rules! mono {
    ($($TIM:ty,)+) => {
        $(
            impl MonoTimerExt for $TIM {
                fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer<Self, FREQ> {
                    Timer::new(self, clocks).monotonic()
                }
            }

            impl<const FREQ: u32> Timer<$TIM, FREQ> {
                pub fn monotonic(self) -> MonoTimer<$TIM, FREQ> {
                    MonoTimer::<$TIM, FREQ>::_new(self)
                }
            }

            impl<const FREQ: u32> MonoTimer<$TIM, FREQ> {
                fn _new(timer: Timer<$TIM, FREQ>) -> Self {
                    timer.tim.arr.write(|w| unsafe { w.bits(u32::MAX) });
                    timer.tim.egr.write(|w| w.ug().set_bit());
                    timer.tim.sr.modify(|_, w| w.uif().clear_bit());
                    timer.tim.cr1.modify(|_, w| w.cen().set_bit().udis().set_bit());
                    Self(timer)
                }
            }

            impl<const FREQ: u32> Monotonic for MonoTimer<$TIM, FREQ> {
                type Instant = fugit::TimerInstantU32<FREQ>;
                type Duration = fugit::TimerDurationU32<FREQ>;

                unsafe fn reset(&mut self) {
                    self.tim.dier.modify(|_, w| w.cc1ie().set_bit());
                }

                #[inline(always)]
                fn now(&mut self) -> Self::Instant {
                    Self::Instant::from_ticks(self.tim.cnt.read().cnt().bits())
                }

                fn set_compare(&mut self, instant: Self::Instant) {
                    self.tim
                        .ccr1
                        .write(|w| w.ccr().bits(instant.duration_since_epoch().ticks()));
                }

                fn clear_compare_flag(&mut self) {
                    self.tim.sr.modify(|_, w| w.cc1if().clear_bit());
                }

                #[inline(always)]
                fn zero() -> Self::Instant {
                    Self::Instant::from_ticks(0)
                }
            }
        )+
    }
}

mono!(crate::pac::TIM5,);

#[cfg(feature = "tim2")]
mono!(crate::pac::TIM2,);
