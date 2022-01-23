use super::{General, Timer};
use crate::rcc::Clocks;
use core::mem::MaybeUninit;
use fugit::TimerDurationU32;

pub use crate::pwm::{Pins, PwmChannel};
pub use crate::timer::{CPin, C1, C2, C3, C4};

pub trait PwmExt<P, PINS>
where
    Self: Sized,
    PINS: Pins<Self, P>,
{
    fn pwm<const FREQ: u32>(
        self,
        clocks: &Clocks,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
    ) -> PINS::Channels;

    fn pwm_us(
        self,
        clocks: &Clocks,
        pins: PINS,
        time: TimerDurationU32<1_000_000>,
    ) -> PINS::Channels {
        self.pwm::<1_000_000>(clocks, pins, time)
    }
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

macro_rules! pwm_all_channels {
    ($($TIMX:ident,)+) => {
        $(
            impl<P, PINS> PwmExt<P, PINS> for crate::pac::$TIMX where
                Self: Sized,
                PINS: Pins<Self, P>
            {
                fn pwm<const FREQ: u32>(self, clocks: &Clocks, pins: PINS, time: TimerDurationU32<FREQ>) -> PINS::Channels {
                    Timer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
                }
            }

            impl<const FREQ: u32> Timer<crate::pac::$TIMX, FREQ> {
                pub fn pwm<P, PINS>(mut self, _pins: PINS, time: TimerDurationU32<FREQ>) -> PINS::Channels
                where
                    PINS: Pins<crate::pac::$TIMX, P>,
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

                    self.tim.set_auto_reload(time.ticks() - 1).unwrap();

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
        )+
    };
}

macro_rules! pwm_2_channels {
    ($($TIMX:ident,)+) => {
        $(
            impl<P, PINS> PwmExt<P, PINS> for crate::pac::$TIMX where
                Self: Sized,
                PINS: Pins<Self, P>
            {
                fn pwm<const FREQ: u32>(self, clocks: &Clocks, pins: PINS, time: TimerDurationU32<FREQ>) -> PINS::Channels {
                    Timer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
                }
            }

            impl<const FREQ: u32> Timer<crate::pac::$TIMX, FREQ> {
                pub fn pwm<P, PINS>(mut self, _pins: PINS, time: TimerDurationU32<FREQ>) -> PINS::Channels
                where
                    PINS: Pins<crate::pac::$TIMX, P>,
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

                    self.tim.set_auto_reload(time.ticks() - 1).unwrap();

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
        )+
    };
}

macro_rules! pwm_1_channel {
    ($($TIMX:ident,)+) => {
        $(
            impl<P, PINS> PwmExt<P, PINS> for crate::pac::$TIMX where
                Self: Sized,
                PINS: Pins<Self, P>
            {
                fn pwm<const FREQ: u32>(self, clocks: &Clocks, pins: PINS, time: TimerDurationU32<FREQ>) -> PINS::Channels {
                    Timer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
                }
            }

            impl<const FREQ: u32> Timer<crate::pac::$TIMX,FREQ> {
                pub fn pwm<P, PINS>(mut self, _pins: PINS, time: TimerDurationU32<FREQ>) -> PINS::Channels
                where
                    PINS: Pins<crate::pac::$TIMX, P>,
                {
                    if PINS::C1 {
                        self.tim.ccmr1_output()
                            .modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1());

                    }

                    // The reference manual is a bit ambiguous about when enabling this bit is really
                    // necessary, but since we MUST enable the preload for the output channels then we
                    // might as well enable for the auto-reload too
                    self.tim.cr1.modify(|_, w| w.arpe().set_bit());

                    self.tim.set_auto_reload(time.ticks() - 1).unwrap();

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
