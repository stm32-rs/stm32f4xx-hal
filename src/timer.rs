//! Timers

use cast::{u16, u32};
use hal::timer::{CountDown, Periodic};
use nb;
use void::Void;

use rcc::Clocks;
use time::Hertz;

#[cfg(feature = "stm32f412")]
use rcc::{APB1, APB2};
#[cfg(feature = "stm32f412")]
use stm32::{TIM1, TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM8, TIM9, TIM10, TIM11, TIM12, TIM13, TIM14};

/// Hardware timers
pub struct Timer<TIM> {
    clocks: Clocks,
    tim: TIM,
    timeout: Hertz,
}

/// Interrupt events
pub enum Event {
    /// Timer timed out / count down ended
    TimeOut,
}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident, $APB:ident, $pclk:ident, $ppre:ident),)+) => {
        $(
            impl Periodic for Timer<$TIM> {}

            impl CountDown for Timer<$TIM> {
                type Time = Hertz;

                // NOTE(allow) `w.psc().bits()` is safe for TIM{6,7} but not for TIM{2,3,4} due to
                // some SVD omission
                #[allow(unused_unsafe)]
                fn start<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>,
                {
                    // pause
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    // restart counter
                    self.tim.cnt.reset();

                    self.timeout = timeout.into();

                    let frequency = self.timeout.0;
                    let ticks = self.clocks.$pclk().0 * if self.clocks.$ppre() == 1 { 1 } else { 2 }
                        / frequency;

                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                    self.tim.psc.write(|w| unsafe { w.psc().bits(psc) });

                    let arr = u16(ticks / u32(psc + 1)).unwrap();
                    self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                    // start counter
                    self.tim.cr1.modify(|_, w| w.cen().set_bit());
                }

                fn wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.sr.read().uif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.sr.modify(|_, w| w.uif().clear_bit());
                        Ok(())
                    }
                }
            }

            impl Timer<$TIM> {
                // XXX(why not name this `new`?) bummer: constructors need to have different names
                // even if the `$TIM` are non overlapping (compare to the `free` function below
                // which just works)
                /// Configures a TIM peripheral as a periodic count down timer
                pub fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks, apb: &mut $APB) -> Self
                where
                    T: Into<Hertz>,
                {
                    // enable and reset peripheral to a clean slate state
                    apb.enr().modify(|_, w| w.$timXen().set_bit());
                    apb.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim,
                        timeout: Hertz(0),
                    };
                    timer.start(timeout);

                    timer
                }

                /// Starts listening for an `event`
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().set_bit());
                        }
                    }
                }

                /// Stops listening for an `event`
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().clear_bit());
                        }
                    }
                }

                /// Releases the TIM peripheral
                pub fn free(self) -> $TIM {
                    // pause counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }

                /// Mutably borrows the TIM peripheral for direct register access
                pub fn borrow_mut(&mut self) -> &mut $TIM {
                    &mut self.tim
                }
            }
        )+
    }
}

#[cfg(feature = "stm32f412")]
hal! {
     TIM1: ( tim1,  tim1en,  tim1rst, APB2, pclk2, ppre2),
     TIM2: ( tim2,  tim2en,  tim2rst, APB1, pclk1, ppre1),
     TIM3: ( tim3,  tim3en,  tim3rst, APB1, pclk1, ppre1),
     TIM4: ( tim4,  tim4en,  tim4rst, APB1, pclk1, ppre1),
     TIM5: ( tim5,  tim5en,  tim5rst, APB1, pclk1, ppre1),
     TIM6: ( tim6,  tim6en,  tim6rst, APB1, pclk1, ppre1),
     TIM7: ( tim7,  tim7en,  tim7rst, APB1, pclk1, ppre1),
     TIM8: ( tim8,  tim8en,  tim8rst, APB2, pclk2, ppre2),
     TIM9: ( tim9,  tim9en,  tim9rst, APB2, pclk2, ppre2),
    TIM10: (tim10, tim10en, tim10rst, APB2, pclk2, ppre2),
    TIM11: (tim11, tim11en, tim11rst, APB2, pclk2, ppre2),
    TIM12: (tim12, tim12en, tim12rst, APB1, pclk1, ppre1),
    TIM13: (tim13, tim13en, tim13rst, APB1, pclk1, ppre1),
    TIM14: (tim14, tim14en, tim14rst, APB1, pclk1, ppre1),
}
