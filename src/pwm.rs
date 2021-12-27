use crate::{
    bb,
    time::Hertz,
    timer::{General, Timer},
};
use cast::u16;
use core::{marker::PhantomData, mem::MaybeUninit};

pub trait Pins<TIM, P> {
    const C1: bool = false;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    type Channels;
}
pub use crate::timer::{CPin, C1, C2, C3, C4};

pub struct PwmChannel<TIM, CHANNEL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
}

macro_rules! pins_impl {
    ( $( ( $($PINX:ident),+ ), ( $($ENCHX:ident),* ); )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $($PINX,)+> Pins<TIM, ($($ENCHX),+)> for ($($PINX),+)
            where
                $($PINX: CPin<$ENCHX, TIM>,)+
            {
                $(const $ENCHX: bool = true;)+
                type Channels = ($(PwmChannel<TIM, $ENCHX>),+);
            }
        )+
    };
}

pins_impl!(
    (P1, P2, P3, P4), (C1, C2, C3, C4);
    (P2, P3, P4), (C2, C3, C4);
    (P1, P3, P4), (C1, C3, C4);
    (P1, P2, P4), (C1, C2, C4);
    (P1, P2, P3), (C1, C2, C3);
    (P3, P4), (C3, C4);
    (P2, P4), (C2, C4);
    (P2, P3), (C2, C3);
    (P1, P4), (C1, C4);
    (P1, P3), (C1, C3);
    (P1, P2), (C1, C2);
    (P1), (C1);
    (P2), (C2);
    (P3), (C3);
    (P4), (C4);
);

impl<C, TIM, P1: CPin<C, TIM>, P2: CPin<C, TIM>> CPin<C, TIM> for (P1, P2) {}
impl<C, TIM, P1: CPin<C, TIM>, P2: CPin<C, TIM>, P3: CPin<C, TIM>> CPin<C, TIM> for (P1, P2, P3) {}
impl<C, TIM, P1: CPin<C, TIM>, P2: CPin<C, TIM>, P3: CPin<C, TIM>, P4: CPin<C, TIM>> CPin<C, TIM>
    for (P1, P2, P3, P4)
{
}

macro_rules! brk {
    (TIM1, $tim:ident) => {
        $tim.bdtr.modify(|_, w| w.aoe().set_bit());
    };
    (TIM8, $tim:ident) => {
        $tim.bdtr.modify(|_, w| w.aoe().set_bit());
    };
    ($_other:ident, $_tim:ident) => {};
}

macro_rules! pwm_pin {
    ($TIMX:ty, $C:ty, $ccr: ident, $bit:literal) => {
        impl PwmChannel<$TIMX, $C> {
            //NOTE(unsafe) atomic write with no side effects
            #[inline]
            pub fn disable(&mut self) {
                unsafe { bb::clear(&(*<$TIMX>::ptr()).ccer, $bit) }
            }

            //NOTE(unsafe) atomic write with no side effects
            #[inline]
            pub fn enable(&mut self) {
                unsafe { bb::set(&(*<$TIMX>::ptr()).ccer, $bit) }
            }

            //NOTE(unsafe) atomic read with no side effects
            #[inline]
            pub fn get_duty(&self) -> u16 {
                unsafe { (*<$TIMX>::ptr()).$ccr.read().bits() as u16 }
            }

            //NOTE(unsafe) atomic read with no side effects
            #[inline]
            pub fn get_max_duty(&self) -> u16 {
                unsafe { (*<$TIMX>::ptr()).arr.read().bits() as u16 }
            }

            //NOTE(unsafe) atomic write with no side effects
            #[inline]
            pub fn set_duty(&mut self, duty: u16) {
                unsafe { (*<$TIMX>::ptr()).$ccr.write(|w| w.bits(duty.into())) }
            }
        }

        impl embedded_hal::PwmPin for PwmChannel<$TIMX, $C> {
            type Duty = u16;

            fn disable(&mut self) {
                self.disable()
            }
            fn enable(&mut self) {
                self.enable()
            }
            fn get_duty(&self) -> Self::Duty {
                self.get_duty()
            }
            fn get_max_duty(&self) -> Self::Duty {
                self.get_max_duty()
            }
            fn set_duty(&mut self, duty: Self::Duty) {
                self.set_duty(duty)
            }
        }
    };
}

macro_rules! pwm_all_channels {
    ($($TIMX:ident,)+) => {
        $(
            impl Timer<crate::pac::$TIMX> {
                pub fn pwm<P, PINS, T>(mut self, _pins: PINS, freq: T) -> PINS::Channels
                where
                    PINS: Pins<crate::pac::$TIMX, P>,
                    T: Into<Hertz>,
                {
                    if PINS::C1 {
                        self.tim.ccmr1_output()
                            .modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1() );
                    }
                    if PINS::C2 {
                        self.tim.ccmr1_output()
                            .modify(|_, w| w.oc2pe().set_bit().oc2m().pwm_mode1() );
                    }
                    if PINS::C3 {
                        self.tim.ccmr2_output()
                            .modify(|_, w| w.oc3pe().set_bit().oc3m().pwm_mode1() );
                    }
                    if PINS::C4 {
                        self.tim.ccmr2_output()
                            .modify(|_, w| w.oc4pe().set_bit().oc4m().pwm_mode1() );
                    }

                    // The reference manual is a bit ambiguous about when enabling this bit is really
                    // necessary, but since we MUST enable the preload for the output channels then we
                    // might as well enable for the auto-reload too
                    self.tim.cr1.modify(|_, w| w.arpe().set_bit());

                    let ticks = self.clk.0 / freq.into().0;
                    let psc = (ticks - 1) / (1 << 16);
                    self.tim.set_prescaler(u16(psc).unwrap());
                    let arr = ticks / (psc + 1);
                    self.tim.set_auto_reload(arr).unwrap();

                    // Trigger update event to load the registers
                    self.tim.trigger_update();

                    let _tim = &self.tim;
                    brk!($TIMX, _tim);
                    self.tim.cr1.write(|w|
                        w.cms()
                            .bits(0b00)
                            .dir()
                            .clear_bit()
                            .opm()
                            .clear_bit()
                            .cen()
                            .set_bit()
                    );
                    //NOTE(unsafe) `PINS::Channels` is a ZST
                    unsafe { MaybeUninit::uninit().assume_init() }
                }
            }

            pwm_pin!(crate::pac::$TIMX, C1, ccr1, 0);
            pwm_pin!(crate::pac::$TIMX, C2, ccr2, 4);
            pwm_pin!(crate::pac::$TIMX, C3, ccr3, 8);
            pwm_pin!(crate::pac::$TIMX, C4, ccr4, 12);
        )+
    };
}

macro_rules! pwm_2_channels {
    ($($TIMX:ident,)+) => {
        $(
            impl Timer<crate::pac::$TIMX> {
                pub fn pwm<P, PINS, T>(mut self, _pins: PINS, freq: T) -> PINS::Channels
                where
                    PINS: Pins<crate::pac::$TIMX, P>,
                    T: Into<Hertz>,
                {
                    if PINS::C1 {
                        self.tim.ccmr1_output().modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1());

                    }
                    if PINS::C2 {
                        self.tim.ccmr1_output().modify(|_, w| w.oc2pe().set_bit().oc2m().pwm_mode1());

                    }

                    // The reference manual is a bit ambiguous about when enabling this bit is really
                    // necessary, but since we MUST enable the preload for the output channels then we
                    // might as well enable for the auto-reload too
                    self.tim.cr1.modify(|_, w| w.arpe().set_bit());

                    let ticks = self.clk.0 / freq.into().0;
                    let psc = (ticks - 1) / (1 << 16);
                    self.tim.set_prescaler(u16(psc).unwrap());
                    let arr = ticks / (psc + 1);
                    self.tim.set_auto_reload(arr).unwrap();

                    // Trigger update event to load the registers
                    self.tim.trigger_update();

                    self.tim.cr1.write(|w|
                        w.opm()
                            .clear_bit()
                            .cen()
                            .set_bit()
                    );
                    //NOTE(unsafe) `PINS::Channels` is a ZST
                    unsafe { MaybeUninit::uninit().assume_init() }
                }
            }

            pwm_pin!(crate::pac::$TIMX, C1, ccr1, 0);
            pwm_pin!(crate::pac::$TIMX, C2, ccr2, 4);
        )+
    };
}

macro_rules! pwm_1_channel {
    ($($TIMX:ident,)+) => {
        $(
            impl Timer<crate::pac::$TIMX> {
                pub fn pwm<P, PINS, T>(mut self, _pins: PINS, freq: T) -> PINS::Channels
                where
                    PINS: Pins<crate::pac::$TIMX, P>,
                    T: Into<Hertz>,
                {
                    if PINS::C1 {
                        self.tim.ccmr1_output()
                            .modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1());

                    }

                    // The reference manual is a bit ambiguous about when enabling this bit is really
                    // necessary, but since we MUST enable the preload for the output channels then we
                    // might as well enable for the auto-reload too
                    self.tim.cr1.modify(|_, w| w.arpe().set_bit());

                    let ticks = self.clk.0 / freq.into().0;
                    let psc = (ticks - 1) / (1 << 16);
                    self.tim.set_prescaler(u16(psc).unwrap());
                    let arr = ticks / (psc + 1);
                    self.tim.set_auto_reload(arr).unwrap();

                    // Trigger update event to load the registers
                    self.tim.trigger_update();

                    self.tim.cr1.write(|w|
                        w.cen()
                            .set_bit()
                    );
                    //NOTE(unsafe) `PINS::Channels` is a ZST
                    unsafe { MaybeUninit::uninit().assume_init() }
                }
            }

            pwm_pin!(crate::pac::$TIMX, C1, ccr1, 0);
        )+
    };
}

pwm_all_channels!(TIM1, TIM5,);

pwm_2_channels!(TIM9,);

pwm_1_channel!(TIM11,);

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
pwm_all_channels!(TIM2, TIM3, TIM4,);

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
pwm_1_channel!(TIM10,);

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
pwm_all_channels!(TIM8,);

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
pwm_2_channels!(TIM12,);

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
pwm_1_channel!(TIM13, TIM14,);
