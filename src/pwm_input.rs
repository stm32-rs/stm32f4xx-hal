use crate::{
    time::Hertz,
    timer::{PinC1, Timer},
};
use cast::u16;

pub trait Pins<TIM> {}

// implement the `Pins` trait wherever PC1 implements PinC1
impl<TIM, PC1> Pins<TIM> for PC1 where PC1: PinC1<TIM> {}

/// Represents a TIMer configured as a PWM input.
/// This peripheral will emit an interrupt on CC2 events, which occurs at two times in this mode:
/// 1. When a new cycle is started: the duty cycle will be `1.00`
/// 2. When the period is captured. the duty cycle will be an observable value.
/// An example interrupt handler is provided:
/// ```
/// use stm32f4xx_hal::pac::TIM8;
/// use stm32f4xx_hal::timer::Timer;
/// use stm32f4xx_hal::pwm_input::PwmInput;
/// use stm32f4xx_hal::gpio::gpioc::PC6;
/// use stm32f4xx_hal::gpio::Alternate;
///
/// type Monitor = PwmInput<TIM8, PC6<Alternate<3>>>;
///
/// fn tim8_cc2(monitor: &Monitor) {
///             let duty_clocks = monitor.get_duty_cycle_clocks();
///             let period_clocks = monitor.get_period_clocks();
///             // check if this interrupt was caused by a capture at the wrong CC2,
///             // peripheral limitation.
///             if !monitor.is_valid_capture(){
///                 return;
///             }
///             let duty = monitor.get_duty_cycle();
/// }
/// ```
pub struct PwmInput<TIM, PINS: Pins<TIM>> {
    tim: TIM,
    clk: Hertz,
    pins: PINS,
}

#[cfg(not(feature = "stm32f410"))]
macro_rules! hal {
    ($($TIM:ident: ($bits:ident),)+) => {
        $(
        // Drag the associated TIM object into scope.
        // Note: its drawn in via the macro to avoid duplicating the feature gate this macro is
        //       expecting to be guarded by.
        use crate::pac::$TIM;

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
            pub fn pwm_input<T, PINS>(self, best_guess: T, pins: PINS) -> PwmInput<$TIM, PINS>
            where
                T: Into<Hertz>,
                PINS: Pins<$TIM>,
            {
                /*
                Borrowed from PWM implementation.
                Sets the TIMer's prescaler such that the TIMer that it ticks at about the best-guess
                 frequency.
                */
                let ticks = self.clk.0 / best_guess.into().0;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                self.tim.psc.write(|w| w.psc().bits(psc));

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
                    w.ic1f()
                        .bits(0)
                        .ic2f()
                        .bits(0)
                        .ic1psc()
                        .bits(0)
                        .ic2psc()
                        .bits(0)
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
                self.tim.cr1.modify(|_, w| w.cen().enabled());

                let Self { tim, clk } = self;

                PwmInput { tim, clk, pins }
            }
        }

        impl<PINS> PwmInput<$TIM, PINS>
        where
            PINS: Pins<$TIM>,
        {
            pub fn reclaim(self) -> (Timer<$TIM>, PINS) {
                // disable timer
                self.tim.cr1.modify(|_, w| w.cen().disabled());
                // decompose elements
                let Self { tim, clk, pins } = self;
                // and return them to the caller
                (Timer { tim, clk }, pins)
            }
            /// Period of PWM signal in terms of clock cycles
            pub fn get_period_clocks(&self) -> $bits {
                self.tim.ccr1.read().ccr().bits()
            }
            /// Duty cycle in terms of clock cycles
            pub fn get_duty_cycle_clocks(&self) -> $bits {
                self.tim.ccr2.read().ccr().bits()
            }
            /// Observed duty cycle as a float in range [0.00, 1.00]
            pub fn get_duty_cycle(&self) -> f32 {
                let period_clocks = self.get_period_clocks();
                if period_clocks == 0 {
                    return 0.0;
                };
                return (self.get_duty_cycle_clocks() as f32 / period_clocks as f32) * 100f32;
            }
            /// Returns whether the timer's duty cycle is a valid observation
            /// (Limitation of how the captures work is extra CC2 interrupts are generated when the
            /// PWM cycle enters a new period).
            pub fn is_valid_capture(&self) -> bool {
                self.get_duty_cycle_clocks() != self.get_period_clocks()
            }
        }
    )+
}}

#[cfg(any(feature = "stm32f411",))]
/* red group */
hal! {
    TIM4: (u16),
    TIM3: (u16),
    TIM2: (u32),
}

/* orange group */
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
hal! {
    TIM2: (u32),
    TIM3: (u16),
    TIM4: (u16),
}
/* green group */
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
hal! {
    TIM8: (u16),
    TIM12: (u16),
}

/* every chip across the series have these timers with support for this feature.
.. except for the 410 which, while the timers support this feature, has a different configuration
   than the rest of the series.
*/
/* yellow group */
#[cfg(not(feature = "stm32f410"))]
hal! {
    TIM1: (u16),
    TIM5: (u32),
    TIM9: (u16),
}
