//! Timers
//!
//! Pins can be used for PWM output in both push-pull mode (`Alternate`) and open-drain mode
//! (`AlternateOD`).

use cast::{u16, u32};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::{DCB, DWT, SYST};
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use void::Void;

use crate::pac::RCC;

use crate::rcc::{self, Clocks};
use crate::time::Hertz;

/// Timer wrapper
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) clk: Hertz,
}

/// Hardware timers
pub struct CountDownTimer<TIM> {
    tim: TIM,
    clk: Hertz,
}

impl<TIM> Timer<TIM>
where
    CountDownTimer<TIM>: CountDown<Time = Hertz>,
{
    /// Starts timer in count down mode at a given frequency
    pub fn start_count_down<T>(self, timeout: T) -> CountDownTimer<TIM>
    where
        T: Into<Hertz>,
    {
        let Self { tim, clk } = self;
        let mut timer = CountDownTimer { tim, clk };
        timer.start(timeout);
        timer
    }
}

impl<TIM> Periodic for CountDownTimer<TIM> {}

/// Interrupt events
pub enum Event {
    /// CountDownTimer timed out / count down ended
    TimeOut,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// CountDownTimer is disabled
    Disabled,
}

impl Timer<SYST> {
    /// Initialize timer
    pub fn syst(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        Self {
            tim: syst,
            clk: clocks.sysclk(),
        }
    }

    pub fn release(self) -> SYST {
        self.tim
    }
}

impl CountDownTimer<SYST> {
    /// Starts listening for an `event`
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::TimeOut => self.tim.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::TimeOut => self.tim.disable_interrupt(),
        }
    }
}

impl CountDown for CountDownTimer<SYST> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let rvr = self.clk.0 / timeout.into().0 - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Cancel for CountDownTimer<SYST> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Self::Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

/// A monotonic non-decreasing timer
///
/// This uses the timer in the debug watch trace peripheral. This means, that if the
/// core is stopped, the timer does not count up. This may be relevant if you are using
/// cortex_m_semihosting::hprintln for debugging in which case the timer will be stopped
/// while printing
#[derive(Clone, Copy)]
pub struct MonoTimer {
    frequency: Hertz,
}

impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(mut dwt: DWT, mut dcb: DCB, clocks: &Clocks) -> Self {
        dcb.enable_trace();
        dwt.enable_cycle_counter();

        // now the CYCCNT counter can't be stopped or reset
        drop(dwt);

        MonoTimer {
            frequency: clocks.hclk(),
        }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(self) -> Hertz {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(self) -> Instant {
        Instant {
            now: DWT::get_cycle_count(),
        }
    }
}

/// A measurement of a monotonically non-decreasing clock
#[derive(Clone, Copy)]
pub struct Instant {
    now: u32,
}

impl Instant {
    /// Ticks elapsed since the `Instant` was created
    pub fn elapsed(self) -> u32 {
        DWT::get_cycle_count().wrapping_sub(self.now)
    }
}

mod private {
    pub trait Sealed {}
}

pub trait Instance: private::Sealed + rcc::Enable + rcc::Reset + rcc::GetBusFreq {}

impl<TIM> Timer<TIM>
where
    TIM: Instance,
{
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable and reset the timer peripheral
            TIM::enable(rcc);
            TIM::reset(rcc);
        }

        Self {
            clk: TIM::get_timer_frequency(clocks),
            tim,
        }
    }
}

macro_rules! hal {
    ($($TIM:ty: ($tim:ident),)+) => {
        $(
            impl private::Sealed for $TIM {}
            impl Instance for $TIM { }

            impl CountDownTimer<$TIM> {
                /// Starts listening for an `event`
                ///
                /// Note, you will also have to enable the TIM2 interrupt in the NVIC to start
                /// receiving events.
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().set_bit());
                        }
                    }
                }

                /// Clears interrupt associated with `event`.
                ///
                /// If the interrupt is not cleared, it will immediately retrigger after
                /// the ISR has finished.
                pub fn clear_interrupt(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Clear interrupt flag
                            self.tim.sr.write(|w| w.uif().clear_bit());
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
                pub fn release(self) -> $TIM {
                    // pause counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }

            impl CountDown for CountDownTimer<$TIM> {
                type Time = Hertz;

                fn start<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>,
                {
                    // pause
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    // reset counter
                    self.tim.cnt.reset();

                    let frequency = timeout.into().0;
                    let ticks = self.clk.0 / frequency;

                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                    self.tim.psc.write(|w| w.psc().bits(psc) );

                    let arr = u16(ticks / u32(psc + 1)).unwrap();
                    self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                    // Trigger update event to load the registers
                    self.tim.cr1.modify(|_, w| w.urs().set_bit());
                    self.tim.egr.write(|w| w.ug().set_bit());
                    self.tim.cr1.modify(|_, w| w.urs().clear_bit());

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

            impl Cancel for CountDownTimer<$TIM>
            {
                type Error = Error;

                fn cancel(&mut self) -> Result<(), Self::Error> {
                    let is_counter_enabled = self.tim.cr1.read().cen().is_enabled();
                    if !is_counter_enabled {
                        return Err(Self::Error::Disabled);
                    }

                    // disable counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    Ok(())
                }
            }
        )+
    }
}

hal! {
    crate::pac::TIM1: (tim1),
    crate::pac::TIM5: (tim5),
    crate::pac::TIM9: (tim9),
    crate::pac::TIM11: (tim11),
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
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
    feature = "stm32f479"
))]
hal! {
    crate::pac::TIM2: (tim2),
    crate::pac::TIM3: (tim3),
    crate::pac::TIM4: (tim4),
    crate::pac::TIM10: (tim10),
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
hal! {
    crate::pac::TIM6: (tim6),
}

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
    feature = "stm32f479"
))]
hal! {
    crate::pac::TIM7: (tim7),
    crate::pac::TIM8: (tim8),
    crate::pac::TIM12: (tim12),
    crate::pac::TIM13: (tim13),
    crate::pac::TIM14: (tim14),
}

#[allow(unused)]
#[cfg(feature = "gpiod")]
use crate::gpio::gpiod::*;
#[allow(unused)]
#[cfg(feature = "gpioe")]
use crate::gpio::gpioe::*;
#[allow(unused)]
#[cfg(feature = "gpiof")]
use crate::gpio::gpiof::*;
#[allow(unused)]
#[cfg(feature = "gpioi")]
use crate::gpio::gpioi::*;
use crate::gpio::{gpioa::*, gpiob::*, Alternate, AlternateOD};
#[allow(unused)]
use crate::gpio::{gpioc::*, gpioh::*};

// Output channels marker traits
pub trait PinC1<TIM> {}
pub trait PinC2<TIM> {}
pub trait PinC3<TIM> {}
pub trait PinC4<TIM> {}

macro_rules! channel_impl {
    ( $( $TIM:ident, $PINC:ident, $PINX:ident, $AF:literal; )+ ) => {
        $(
            impl $PINC<crate::pac::$TIM> for $PINX<Alternate<$AF>> {}
            impl $PINC<crate::pac::$TIM> for $PINX<AlternateOD<$AF>> {}
        )+
    };
}

channel_impl!(
    TIM1, PinC1, PA8, 1;
    TIM1, PinC2, PA9, 1;
    TIM1, PinC3, PA10, 1;
    TIM1, PinC4, PA11, 1;

    TIM5, PinC1, PA0, 2;
    TIM5, PinC2, PA1, 2;
    TIM5, PinC3, PA2, 2;
    TIM5, PinC4, PA3, 2;
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
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
    feature = "stm32f479"
))]
channel_impl!(
    TIM1, PinC1, PE9, 1;
    TIM1, PinC2, PE11, 1;
    TIM1, PinC3, PE13, 1;
    TIM1, PinC4, PE14, 1;

    TIM2, PinC1, PA0, 1;
    TIM2, PinC2, PA1, 1;
    TIM2, PinC3, PA2, 1;
    TIM2, PinC4, PA3, 1;

    TIM2, PinC2, PB3, 1;
    TIM2, PinC3, PB10, 1;
    TIM2, PinC4, PB11, 1;

    TIM2, PinC1, PA5, 1;
    TIM2, PinC1, PA15, 1;

    TIM3, PinC1, PA6, 2;
    TIM3, PinC2, PA7, 2;
    TIM3, PinC3, PB0, 2;
    TIM3, PinC4, PB1, 2;

    TIM3, PinC1, PB4, 2;
    TIM3, PinC2, PB5, 2;

    TIM3, PinC1, PC6, 2;
    TIM3, PinC2, PC7, 2;
    TIM3, PinC3, PC8, 2;
    TIM3, PinC4, PC9, 2;

    TIM4, PinC1, PB6, 2;
    TIM4, PinC2, PB7, 2;
    TIM4, PinC3, PB8, 2;
    TIM4, PinC4, PB9, 2;

    TIM4, PinC1, PD12, 2;
    TIM4, PinC2, PD13, 2;
    TIM4, PinC3, PD14, 2;
    TIM4, PinC4, PD15, 2;
);

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
    feature = "stm32f479"
))]
channel_impl!(
    TIM8, PinC1, PC6, 3;
    TIM8, PinC2, PC7, 3;
    TIM8, PinC3, PC8, 3;
    TIM8, PinC4, PC9, 3;
);

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM5, PinC1, PH10, 2;
    TIM5, PinC2, PH11, 2;
    TIM5, PinC3, PH12, 2;
    TIM5, PinC4, PI0, 2;

    TIM8, PinC1, PI5, 3;
    TIM8, PinC2, PI6, 3;
    TIM8, PinC3, PI7, 3;
    TIM8, PinC4, PI2, 3;
);

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
channel_impl!(
    TIM5, PinC1, PF3, 2;
    TIM5, PinC2, PF4, 2;
    TIM5, PinC3, PF5, 2;
    TIM5, PinC4, PF10, 2;
);

#[cfg(feature = "stm32f410")]
channel_impl!(
    TIM5, PinC1, PB12, 2;
    //TIM5, PinC2, PC10, 2;
    //TIM5, PinC3, PC11, 2;
    TIM5, PinC4, PB11, 2;
);

#[cfg(feature = "stm32f446")]
channel_impl!(
    TIM2, PinC1, PB8, 1;
    TIM2, PinC2, PB9, 1;

    TIM2, PinC4, PB2, 1;
);
