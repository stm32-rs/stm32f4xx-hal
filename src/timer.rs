//! Timers

use cast::{u16, u32};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use void::Void;

use crate::stm32::RCC;
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
use crate::stm32::TIM6;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
use crate::stm32::{TIM1, TIM11, TIM5, TIM9};
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
use crate::stm32::{TIM10, TIM2, TIM3, TIM4};
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
use crate::stm32::{TIM12, TIM13, TIM14, TIM7, TIM8};

use crate::rcc::Clocks;
use crate::time::Hertz;

/// Hardware timers
pub struct Timer<TIM> {
    clocks: Clocks,
    tim: TIM,
}

/// Interrupt events
pub enum Event {
    /// Timer timed out / count down ended
    TimeOut,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Timer is disabled
    Disabled,
}

impl Timer<SYST> {
    /// Configures the SYST clock as a periodic count down timer
    pub fn syst<T>(mut syst: SYST, timeout: T, clocks: Clocks) -> Self
    where
        T: Into<Hertz>,
    {
        syst.set_clock_source(SystClkSource::Core);
        let mut timer = Timer { tim: syst, clocks };
        timer.start(timeout);
        timer
    }

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

impl CountDown for Timer<SYST> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let rvr = self.clocks.sysclk().0 / timeout.into().0 - 1;

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

impl Cancel for Timer<SYST> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Self::Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl Periodic for Timer<SYST> {}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident, $apbenr:ident, $apbrstr:ident, $pclk:ident, $ppre:ident),)+) => {
        $(
            impl Timer<$TIM> {
                /// Configures a TIM peripheral as a periodic count down timer
                pub fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks) -> Self
                where
                    T: Into<Hertz>,
                {
                    // enable and reset peripheral to a clean slate state
                    let rcc = unsafe { &(*RCC::ptr()) };
                    rcc.$apbenr.modify(|_, w| w.$timXen().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim,
                    };
                    timer.start(timeout);

                    timer
                }

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

            impl CountDown for Timer<$TIM> {
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
                    let pclk_mul = if self.clocks.$ppre() == 1 { 1 } else { 2 };
                    let ticks = self.clocks.$pclk().0 * pclk_mul / frequency;

                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                    self.tim.psc.write(|w| w.psc().bits(psc) );

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

            impl Cancel for Timer<$TIM>
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

            impl Periodic for Timer<$TIM> {}
        )+
    }
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    TIM1: (tim1, tim1en, tim1rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM5: (tim5, tim5en, tim5rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM9: (tim9, tim9en, tim9rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM11: (tim11, tim11en, tim11rst, apb2enr, apb2rstr, pclk2, ppre2),
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
    TIM2: (tim2, tim2en, tim2rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM3: (tim3, tim3en, tim3rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM4: (tim4, tim4en, tim4rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM10: (tim10, tim10en, tim10rst, apb2enr, apb2rstr, pclk2, ppre2),
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
    TIM6: (tim6, tim6en, tim6rst, apb1enr, apb1rstr, pclk1, ppre1),
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
    TIM7: (tim7, tim7en, tim7rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM8: (tim8, tim8en, tim8rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM12: (tim12, tim12en, tim12rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM13: (tim13, tim13en, tim13rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM14: (tim14, tim14en, tim14rst, apb1enr, apb1rstr, pclk1, ppre1),
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
use crate::gpio::gpiob::*;

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
use crate::gpio::gpioc::*;

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
use crate::gpio::gpiod::*;

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
use crate::gpio::gpioe::*;

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiof::*;

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
use crate::gpio::gpioh::*;

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
use crate::gpio::gpioi::*;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
use crate::gpio::AF1;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
use crate::gpio::AF2;

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
use crate::gpio::AF3;

use crate::gpio::{gpioa::*, Alternate};

// Output channels marker traits
pub trait PinC1<TIM> {}
pub trait PinC2<TIM> {}
pub trait PinC3<TIM> {}
pub trait PinC4<TIM> {}

macro_rules! channel_impl {
    ( $( $TIM:ident, $PINC:ident, $PINX:ident, $MODE:ident<$AF:ident>; )+ ) => {
        $(
            impl $PINC<$TIM> for $PINX<$MODE<$AF>> {}
        )+
    };
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    TIM1, PinC1, PA8, Alternate<AF1>;
    TIM1, PinC2, PA9, Alternate<AF1>;
    TIM1, PinC3, PA10, Alternate<AF1>;
    TIM1, PinC4, PA11, Alternate<AF1>;

    TIM5, PinC1, PA0, Alternate<AF2>;
    TIM5, PinC2, PA1, Alternate<AF2>;
    TIM5, PinC3, PA2, Alternate<AF2>;
    TIM5, PinC4, PA3, Alternate<AF2>;
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
    TIM1, PinC1, PE9, Alternate<AF1>;
    TIM1, PinC2, PE11, Alternate<AF1>;
    TIM1, PinC3, PE13, Alternate<AF1>;
    TIM1, PinC4, PE14, Alternate<AF1>;

    TIM2, PinC1, PA0, Alternate<AF1>;
    TIM2, PinC2, PA1, Alternate<AF1>;
    TIM2, PinC3, PA2, Alternate<AF1>;
    TIM2, PinC4, PA3, Alternate<AF1>;

    TIM2, PinC2, PB3, Alternate<AF1>;
    TIM2, PinC3, PB10, Alternate<AF1>;
    TIM2, PinC4, PB11, Alternate<AF1>;

    TIM2, PinC1, PA5, Alternate<AF1>;
    TIM2, PinC1, PA15, Alternate<AF1>;

    TIM3, PinC1, PA6, Alternate<AF2>;
    TIM3, PinC2, PA7, Alternate<AF2>;
    TIM3, PinC3, PB0, Alternate<AF2>;
    TIM3, PinC4, PB1, Alternate<AF2>;

    TIM3, PinC1, PB4, Alternate<AF2>;
    TIM3, PinC2, PB5, Alternate<AF2>;

    TIM3, PinC1, PC6, Alternate<AF2>;
    TIM3, PinC2, PC7, Alternate<AF2>;
    TIM3, PinC3, PC8, Alternate<AF2>;
    TIM3, PinC4, PC9, Alternate<AF2>;

    TIM4, PinC1, PB6, Alternate<AF2>;
    TIM4, PinC2, PB7, Alternate<AF2>;
    TIM4, PinC3, PB8, Alternate<AF2>;
    TIM4, PinC4, PB9, Alternate<AF2>;

    TIM4, PinC1, PD12, Alternate<AF2>;
    TIM4, PinC2, PD13, Alternate<AF2>;
    TIM4, PinC3, PD14, Alternate<AF2>;
    TIM4, PinC4, PD15, Alternate<AF2>;
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
    TIM8, PinC1, PC6, Alternate<AF3>;
    TIM8, PinC2, PC7, Alternate<AF3>;
    TIM8, PinC3, PC8, Alternate<AF3>;
    TIM8, PinC4, PC9, Alternate<AF3>;
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
    TIM5, PinC1, PH10, Alternate<AF2>;
    TIM5, PinC2, PH11, Alternate<AF2>;
    TIM5, PinC3, PH12, Alternate<AF2>;
    TIM5, PinC4, PI0, Alternate<AF2>;

    TIM8, PinC1, PI5, Alternate<AF3>;
    TIM8, PinC2, PI6, Alternate<AF3>;
    TIM8, PinC3, PI7, Alternate<AF3>;
    TIM8, PinC4, PI2, Alternate<AF3>;
);

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
channel_impl!(
    TIM5, PinC1, PF3, Alternate<AF2>;
    TIM5, PinC2, PF4, Alternate<AF2>;
    TIM5, PinC3, PF5, Alternate<AF2>;
    TIM5, PinC4, PF10, Alternate<AF2>;
);

#[cfg(feature = "stm32f410")]
channel_impl!(
    TIM5, PinC1, PB12, Alternate<AF2>;
    //TIM5, PinC2, PC10, Alternate<AF2>;
    //TIM5, PinC3, PC11, Alternate<AF2>;
    TIM5, PinC4, PB11, Alternate<AF2>;
);

#[cfg(feature = "stm32f446")]
channel_impl!(
    TIM2, PinC1, PB8, Alternate<AF1>;
    TIM2, PinC2, PB9, Alternate<AF1>;

    TIM2, PinC4, PB2, Alternate<AF1>;
);
