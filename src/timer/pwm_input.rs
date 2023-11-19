use super::{CPin, General, Instance, Timer, WithPwm};
use crate::gpio::PushPull;
use crate::pac;
use core::convert::TryFrom;
use core::ops::{Deref, DerefMut};
use fugit::HertzU32 as Hertz;

/// Represents a TIMer configured as a PWM input.
/// This peripheral will emit an interrupt on CC2 events, which occurs at two times in this mode:
/// 1. When a new cycle is started: the duty cycle will be `1.00`
/// 2. When the period is captured. the duty cycle will be an observable value.
/// An example interrupt handler is provided:
/// ```
/// use stm32f4xx_hal::{pac::TIM8, pwm_input::PwmInput};
///
/// type Monitor = PwmInput<TIM8>;
///
/// fn tim8_cc2(monitor: &Monitor) {
///     let duty_clocks = monitor.get_duty_cycle_clocks();
///     let period_clocks = monitor.get_period_clocks();
///     // check if this interrupt was caused by a capture at the wrong CC2,
///     // peripheral limitation.
///     if !monitor.is_valid_capture() {
///         return;
///     }
///     let duty = monitor.get_duty_cycle();
/// }
/// ```
pub struct PwmInput<TIM>
where
    TIM: Instance + WithPwm + CPin<0>,
{
    timer: Timer<TIM>,
    _pins: TIM::Ch<PushPull>,
}

impl<TIM> Deref for PwmInput<TIM>
where
    TIM: Instance + WithPwm + CPin<0>,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM> DerefMut for PwmInput<TIM>
where
    TIM: Instance + WithPwm + CPin<0>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM> PwmInput<TIM>
where
    TIM: Instance + WithPwm + CPin<0>,
{
    pub fn release(mut self) -> Timer<TIM> {
        self.tim.cr1_reset();
        self.timer
    }
}

#[cfg(not(feature = "gpio-f410"))]
macro_rules! hal {
    ($TIM:ty) => {
        impl Timer<$TIM> {
            /// Configures this timer for PWM input. Accepts the `best_guess` frequency of the signal
            /// Note: this should be as close as possible to the frequency of the PWM waveform for best
            /// accuracy.
            ///
            /// This device will emit an interrupt on CC1, which occurs at two times in this mode:
            /// 1. When a new cycle is started: the duty cycle will be `1.00`
            /// 2. When the period is captured. the duty cycle will be an observable value.
            /// See the pwm input example for an suitable interrupt handler.
            #[allow(unused_unsafe)] //for some chips the operations are considered safe.
            pub fn pwm_input(
                mut self,
                best_guess: Hertz,
                pins: impl Into<<$TIM as CPin<0>>::Ch<PushPull>>,
            ) -> PwmInput<$TIM> {
                let pins = pins.into();

                /*
                Borrowed from PWM implementation.
                Sets the TIMer's prescaler such that the TIMer that it ticks at about the best-guess
                 frequency.
                */
                let ticks = self.clk.raw() / best_guess.raw();
                let psc = u16::try_from((ticks - 1) / (1 << 16)).unwrap();
                self.tim.set_prescaler(psc);

                // Seemingly this needs to be written to
                // self.tim.arr.write(|w| w.arr().bits(u16::MAX));

                /*
                For example, one can measure the period (in TIMx_CCR1 register) and the duty cycle (in
                TIMx_CCR2 register) of the PWM applied on TI1 using the following procedure (depending
                on CK_INT frequency and prescaler value):

                from RM0390 16.3.7
                 */

                // Select the active input for TIMx_CCR1: write the CC1S bits to 01 in the TIMx_CCMR1
                // register (TI1 selected).
                self.tim
                    .ccmr1_input()
                    .modify(|_, w| unsafe { w.cc1s().bits(0b01) });

                // Select the active polarity for TI1FP1 (used both for capture in TIMx_CCR1 and counter
                // clear): write the CC1P and CC1NP bits to ‘0’ (active on rising edge).

                self.tim
                    .ccer
                    .modify(|_, w| w.cc1p().clear_bit().cc2p().clear_bit());

                // disable filters and disable the input capture prescalers.
                self.tim.ccmr1_input().modify(|_, w| unsafe {
                    w.ic1f().bits(0).ic2f().bits(0);
                    w.ic1psc().bits(0).ic2psc().bits(0)
                });

                // Select the active input for TIMx_CCR2: write the CC2S bits to 10 in the TIMx_CCMR1
                // register (TI1 selected)
                self.tim
                    .ccmr1_input()
                    .modify(|_, w| unsafe { w.cc2s().bits(0b10) });

                // Select the active polarity for TI1FP2 (used for capture in TIMx_CCR2): write the CC2P
                // and CC2NP bits to ‘1’ (active on falling edge).
                self.tim
                    .ccer
                    .modify(|_, w| w.cc2p().set_bit().cc2np().set_bit());

                // Select the valid trigger input: write the TS bits to 101 in the TIMx_SMCR register
                // (TI1FP1 selected).
                self.tim.smcr.modify(|_, w| unsafe { w.ts().bits(0b101) });

                // Configure the slave mode controller in reset mode: write the SMS bits to 100 in the
                // TIMx_SMCR register.
                self.tim.smcr.modify(|_, w| unsafe { w.sms().bits(0b100) });

                // Enable the captures: write the CC1E and CC2E bits to ‘1’ in the TIMx_CCER register.
                self.tim
                    .ccer
                    .modify(|_, w| w.cc1e().set_bit().cc2e().set_bit());

                // enable interrupts.
                self.tim.dier.modify(|_, w| w.cc2ie().set_bit());
                // enable the counter.
                self.tim.enable_counter(true);

                PwmInput {
                    timer: self,
                    _pins: pins,
                }
            }
        }

        impl PwmInput<$TIM> {
            /// Period of PWM signal in terms of clock cycles
            pub fn get_period_clocks(&self) -> <$TIM as General>::Width {
                self.tim.ccr1().read().ccr().bits()
            }
            /// Duty cycle in terms of clock cycles
            pub fn get_duty_cycle_clocks(&self) -> <$TIM as General>::Width {
                self.tim.ccr2().read().ccr().bits()
            }
            /// Observed duty cycle as a float in range [0.00, 1.00]
            pub fn get_duty_cycle(&self) -> f32 {
                let period_clocks = self.get_period_clocks();
                if period_clocks == 0 {
                    return 0.;
                };
                (self.get_duty_cycle_clocks() as f32 / period_clocks as f32) * 100f32
            }
            /// Returns whether the timer's duty cycle is a valid observation
            /// (Limitation of how the captures work is extra CC2 interrupts are generated when the
            /// PWM cycle enters a new period).
            pub fn is_valid_capture(&self) -> bool {
                self.get_duty_cycle_clocks() != self.get_period_clocks()
            }
        }
    };
}

#[cfg(feature = "tim1")]
hal! { pac::TIM1 }
#[cfg(feature = "tim2")]
hal! { pac::TIM2 }
#[cfg(feature = "tim3")]
hal! { pac::TIM3 }
#[cfg(feature = "tim4")]
hal! { pac::TIM4 }
#[cfg(feature = "tim5")]
hal! { pac::TIM5 }
#[cfg(feature = "tim8")]
hal! { pac::TIM8 }
#[cfg(feature = "tim9")]
hal! { pac::TIM9 }
#[cfg(feature = "tim12")]
hal! { pac::TIM12 }
