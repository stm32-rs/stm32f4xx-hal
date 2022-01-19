// RTIC Monotonic impl for the 32-bit timers
use crate::{rcc::Clocks, timer::Timer};
use cast::u16;
pub use fugit::{self, ExtU32};
use rtic_monotonic::Monotonic;

pub struct MonoTimer<T, const FREQ: u32>(T);

macro_rules! mono {
    ($($TIM:ty,)+) => {
        $(
            impl Timer<$TIM> {
                pub fn monotonic<const FREQ: u32>(self) -> MonoTimer<$TIM, FREQ> {
                    MonoTimer::<$TIM, FREQ>::_new(self)
                }
            }

            impl<const FREQ: u32> MonoTimer<$TIM, FREQ> {
                pub fn new(timer: $TIM, clocks: &Clocks) -> Self {
                    Timer::<$TIM>::new(timer, clocks).monotonic()
                }

                fn _new(timer: Timer<$TIM>) -> Self {
                    let Timer { tim, clk } = timer;
                    assert!(clk.0 % FREQ == 0);
                    let prescaler = clk.0 / FREQ - 1;
                    tim.psc.write(|w| w.psc().bits(u16(prescaler).unwrap()));
                    tim.arr.write(|w| unsafe { w.bits(u32::MAX) });
                    tim.egr.write(|w| w.ug().set_bit());
                    tim.sr.modify(|_, w| w.uif().clear_bit());
                    tim.cr1.modify(|_, w| w.cen().set_bit().udis().set_bit());
                    Self(tim)
                }
            }

            impl<const FREQ: u32> Monotonic for MonoTimer<$TIM, FREQ> {
                type Instant = fugit::TimerInstantU32<FREQ>;
                type Duration = fugit::TimerDurationU32<FREQ>;

                unsafe fn reset(&mut self) {
                    self.0.dier.modify(|_, w| w.cc1ie().set_bit());
                }

                #[inline(always)]
                fn now(&mut self) -> Self::Instant {
                    Self::Instant::from_ticks(self.0.cnt.read().cnt().bits())
                }

                fn set_compare(&mut self, instant: Self::Instant) {
                    self.0
                        .ccr1
                        .write(|w| w.ccr().bits(instant.duration_since_epoch().ticks()));
                }

                fn clear_compare_flag(&mut self) {
                    self.0.sr.modify(|_, w| w.cc1if().clear_bit());
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
