use cast::{u16, u32};
use core::convert::Infallible;
use core::{marker::PhantomData, mem::MaybeUninit};

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
use crate::{bb, hal, rcc::Clocks, stm32::RCC, time::Hertz};

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
use crate::stm32::{TIM12, TIM13, TIM14, TIM8};

pub trait Pins<TIM, P> {
    const C1: bool = false;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    type Channels;
}
use crate::timer::PinC1;
use crate::timer::PinC2;
use crate::timer::PinC3;
use crate::timer::PinC4;

pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

pub struct PwmChannels<TIM, CHANNELS> {
    _channel: PhantomData<CHANNELS>,
    _tim: PhantomData<TIM>,
}

macro_rules! pins_impl {
    ( $( ( $($PINX:ident),+ ), ( $($TRAIT:ident),+ ), ( $($ENCHX:ident),* ); )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $($PINX,)+> Pins<TIM, ($($ENCHX),+)> for ($($PINX),+)
            where
                $($PINX: $TRAIT<TIM>,)+
            {
                $(const $ENCHX: bool = true;)+
                type Channels = ($(PwmChannels<TIM, $ENCHX>),+);
            }
        )+
    };
}

pins_impl!(
    (P1, P2, P3, P4), (PinC1, PinC2, PinC3, PinC4), (C1, C2, C3, C4);
    (P2, P3, P4), (PinC2, PinC3, PinC4), (C2, C3, C4);
    (P1, P3, P4), (PinC1, PinC3, PinC4), (C1, C3, C4);
    (P1, P2, P4), (PinC1, PinC2, PinC4), (C1, C2, C4);
    (P1, P2, P3), (PinC1, PinC2, PinC3), (C1, C2, C3);
    (P3, P4), (PinC3, PinC4), (C3, C4);
    (P2, P4), (PinC2, PinC4), (C2, C4);
    (P2, P3), (PinC2, PinC3), (C2, C3);
    (P1, P4), (PinC1, PinC4), (C1, C4);
    (P1, P3), (PinC1, PinC3), (C1, C3);
    (P1, P2), (PinC1, PinC2), (C1, C2);
    (P1), (PinC1), (C1);
    (P2), (PinC2), (C2);
    (P3), (PinC3), (C3);
    (P4), (PinC4), (C4);
);

impl<TIM, P1: PinC1<TIM>, P2: PinC1<TIM>> PinC1<TIM> for (P1, P2) {}
impl<TIM, P1: PinC2<TIM>, P2: PinC2<TIM>> PinC2<TIM> for (P1, P2) {}
impl<TIM, P1: PinC3<TIM>, P2: PinC3<TIM>> PinC3<TIM> for (P1, P2) {}
impl<TIM, P1: PinC4<TIM>, P2: PinC4<TIM>> PinC4<TIM> for (P1, P2) {}

impl<TIM, P1: PinC1<TIM>, P2: PinC1<TIM>, P3: PinC1<TIM>> PinC1<TIM> for (P1, P2, P3) {}
impl<TIM, P1: PinC2<TIM>, P2: PinC2<TIM>, P3: PinC2<TIM>> PinC2<TIM> for (P1, P2, P3) {}
impl<TIM, P1: PinC3<TIM>, P2: PinC3<TIM>, P3: PinC3<TIM>> PinC3<TIM> for (P1, P2, P3) {}
impl<TIM, P1: PinC4<TIM>, P2: PinC4<TIM>, P3: PinC4<TIM>> PinC4<TIM> for (P1, P2, P3) {}

impl<TIM, P1: PinC1<TIM>, P2: PinC1<TIM>, P3: PinC1<TIM>, P4: PinC1<TIM>> PinC1<TIM>
    for (P1, P2, P3, P4)
{
}
impl<TIM, P1: PinC2<TIM>, P2: PinC2<TIM>, P3: PinC2<TIM>, P4: PinC2<TIM>> PinC2<TIM>
    for (P1, P2, P3, P4)
{
}
impl<TIM, P1: PinC3<TIM>, P2: PinC3<TIM>, P3: PinC3<TIM>, P4: PinC3<TIM>> PinC3<TIM>
    for (P1, P2, P3, P4)
{
}
impl<TIM, P1: PinC4<TIM>, P2: PinC4<TIM>, P3: PinC4<TIM>, P4: PinC4<TIM>> PinC4<TIM>
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

macro_rules! pwm_all_channels {
    ($($TIMX:ident: ($timX:ident, $apbenr:ident, $apbrstr:ident, $bit:expr, $pclk:ident, $ppre:ident),)+) => {
        $(
            pub fn $timX<P, PINS, T>(tim: $TIMX, _pins: PINS, clocks: Clocks, freq: T) -> PINS::Channels
            where
                PINS: Pins<$TIMX, P>,
                T: Into<Hertz>,
            {
                {
                    unsafe {
                        //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
                        let rcc = &(*RCC::ptr());
                        // Enable and reset the timer peripheral, it's the same bit position for both registers
                        bb::set(&rcc.$apbenr, $bit);
                        bb::set(&rcc.$apbrstr, $bit);
                        bb::clear(&rcc.$apbrstr, $bit);
                    }
                }
                if PINS::C1 {
                    tim.ccmr1_output()
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1() );
                }
                if PINS::C2 {
                    tim.ccmr1_output()
                        .modify(|_, w| w.oc2pe().set_bit().oc2m().pwm_mode1() );
                }
                if PINS::C3 {
                    tim.ccmr2_output()
                        .modify(|_, w| w.oc3pe().set_bit().oc3m().pwm_mode1() );
                }
                if PINS::C4 {
                    tim.ccmr2_output()
                        .modify(|_, w| w.oc4pe().set_bit().oc4m().pwm_mode1() );
                }

                // The reference manual is a bit ambiguous about when enabling this bit is really
                // necessary, but since we MUST enable the preload for the output channels then we
                // might as well enable for the auto-reload too
                tim.cr1.modify(|_, w| w.arpe().set_bit());

                let clk = clocks.$pclk().0 * if clocks.$ppre() == 1 { 1 } else { 2 };
                let ticks = clk / freq.into().0;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                tim.psc.write(|w| w.psc().bits(psc) );
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                // Trigger update event to load the registers
                tim.cr1.modify(|_, w| w.urs().set_bit());
                tim.egr.write(|w| w.ug().set_bit());
                tim.cr1.modify(|_, w| w.urs().clear_bit());

                brk!($TIMX, tim);
                tim.cr1.write(|w|
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

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C1> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr1.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C2> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 4) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 4) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr2.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C3> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 8) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 8) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr3.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr3.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C4> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 12) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 12) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr4.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr4.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }
        )+
    };
}

macro_rules! pwm_2_channels {
    ($($TIMX:ident: ($timX:ident, $apbenr:ident, $apbrstr:ident, $bit:expr, $pclk:ident, $ppre:ident),)+) => {
        $(
            pub fn $timX<P, PINS, T>(tim: $TIMX, _pins: PINS, clocks: Clocks, freq: T) -> PINS::Channels
            where
                PINS: Pins<$TIMX, P>,
                T: Into<Hertz>,
            {
                {
                    unsafe {
                        //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
                        let rcc = &(*RCC::ptr());
                        // Enable and reset the timer peripheral, it's the same bit position for both registers
                        bb::set(&rcc.$apbenr, $bit);
                        bb::set(&rcc.$apbrstr, $bit);
                        bb::clear(&rcc.$apbrstr, $bit);
                    }
                }
                if PINS::C1 {
                    //NOTE(unsafe) 6 is a valid value to write to oc1m
                    unsafe {
                        tim.ccmr1_output().modify(|_, w| w.oc1pe().set_bit().oc1m().bits(6));
                    }
                }
                if PINS::C2 {
                    //NOTE(unsafe) 6 is a valid value to write to oc2m
                    unsafe {
                        tim.ccmr1_output().modify(|_, w| w.oc2pe().set_bit().oc2m().bits(6));
                    }
                }

                // The reference manual is a bit ambiguous about when enabling this bit is really
                // necessary, but since we MUST enable the preload for the output channels then we
                // might as well enable for the auto-reload too
                tim.cr1.modify(|_, w| w.arpe().set_bit());

                let clk = clocks.$pclk().0 * if clocks.$ppre() == 1 { 1 } else { 2 };
                let ticks = clk / freq.into().0;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                tim.psc.write(|w| w.psc().bits(psc) );
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                // Trigger update event to load the registers
                tim.cr1.modify(|_, w| w.urs().set_bit());
                tim.egr.write(|w| w.ug().set_bit());
                tim.cr1.modify(|_, w| w.urs().clear_bit());

                tim.cr1.write(|w|
                    w.opm()
                        .clear_bit()
                        .cen()
                        .set_bit()
                );
                //NOTE(unsafe) `PINS::Channels` is a ZST
                unsafe { MaybeUninit::uninit().assume_init() }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C1> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr1.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C2> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 4) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 4) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr2.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }
        )+
    };
}

macro_rules! pwm_1_channel {
    ($($TIMX:ident: ($timX:ident, $apbenr:ident, $apbrstr:ident, $bit:expr, $pclk:ident, $ppre:ident),)+) => {
        $(
            pub fn $timX<P, PINS, T>(tim: $TIMX, _pins: PINS, clocks: Clocks, freq: T) -> PINS::Channels
            where
                PINS: Pins<$TIMX, P>,
                T: Into<Hertz>,
            {
                {
                    unsafe {
                        //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
                        let rcc = &(*RCC::ptr());
                        // Enable and reset the timer peripheral, it's the same bit position for both registers
                        bb::set(&rcc.$apbenr, $bit);
                        bb::set(&rcc.$apbrstr, $bit);
                        bb::clear(&rcc.$apbrstr, $bit);
                    }
                }
                if PINS::C1 {
                    //NOTE(unsafe) 6 is a valid value to write to oc1m
                    unsafe {
                        tim.ccmr1_output()
                            .modify(|_, w| w.oc1pe().set_bit().oc1m().bits(6));
                    }
                }

                // The reference manual is a bit ambiguous about when enabling this bit is really
                // necessary, but since we MUST enable the preload for the output channels then we
                // might as well enable for the auto-reload too
                tim.cr1.modify(|_, w| w.arpe().set_bit());

                let clk = clocks.$pclk().0 * if clocks.$ppre() == 1 { 1 } else { 2 };
                let ticks = clk / freq.into().0;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                tim.psc.write(|w| w.psc().bits(psc) );
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                // Trigger update event to load the registers
                tim.cr1.modify(|_, w| w.urs().set_bit());
                tim.egr.write(|w| w.ug().set_bit());
                tim.cr1.modify(|_, w| w.urs().clear_bit());

                tim.cr1.write(|w|
                    w.cen()
                        .set_bit()
                );
                //NOTE(unsafe) `PINS::Channels` is a ZST
                unsafe { MaybeUninit::uninit().assume_init() }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C1> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr1.read().ccr().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr().bits(duty.into())) };
                    Ok(())
                }
            }
        )+
    };
}

#[cfg(feature = "stm32f410")]
macro_rules! pwm_tim5_f410 {
    ($($TIMX:ident: ($timX:ident, $apbenr:ident, $apbrstr:ident, $bit:expr, $pclk:ident, $ppre:ident),)+) => {
        $(
            pub fn $timX<P, PINS, T>(tim: $TIMX, _pins: PINS, clocks: Clocks, freq: T) -> PINS::Channels
            where
                PINS: Pins<$TIMX, P>,
                T: Into<Hertz>,
            {
                {
                    unsafe {
                        //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
                        let rcc = &(*RCC::ptr());
                        // Enable and reset the timer peripheral, it's the same bit position for both registers
                        bb::set(&rcc.$apbenr, $bit);
                        bb::set(&rcc.$apbrstr, $bit);
                        bb::clear(&rcc.$apbrstr, $bit);
                    }
                }
                if PINS::C1 {
                    tim.ccmr1_output()
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1() );
                }
                if PINS::C2 {
                    tim.ccmr1_output()
                        .modify(|_, w| w.oc2pe().set_bit().oc2m().pwm_mode1() );
                }
                if PINS::C3 {
                    tim.ccmr2_output()
                        .modify(|_, w| w.oc3pe().set_bit().oc3m().pwm_mode1() );
                }
                if PINS::C4 {
                    tim.ccmr2_output()
                        .modify(|_, w| w.oc4pe().set_bit().oc4m().pwm_mode1() );
                }

                // The reference manual is a bit ambiguous about when enabling this bit is really
                // necessary, but since we MUST enable the preload for the output channels then we
                // might as well enable for the auto-reload too
                tim.cr1.modify(|_, w| w.arpe().set_bit());

                let clk = clocks.$pclk().0 * if clocks.$ppre() == 1 { 1 } else { 2 };
                let ticks = clk / freq.into().0;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                tim.psc.write(|w| w.psc().bits(psc) );
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                tim.arr.write(|w| unsafe { w.arr_l().bits(arr) });

                // Trigger update event to load the registers
                tim.cr1.modify(|_, w| w.urs().set_bit());
                tim.egr.write(|w| w.ug().set_bit());
                tim.cr1.modify(|_, w| w.urs().clear_bit());

                tim.cr1.write(|w|
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

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C1> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 0) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr1.read().ccr1_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr1_l().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C2> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 4) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 4) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr2.read().ccr1_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr1_l().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C3> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 8) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 8) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr3.read().ccr1_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr3.write(|w| w.ccr1_l().bits(duty.into())) };
                    Ok(())
                }
            }

            impl hal::pwm::PwmPin for PwmChannels<$TIMX, C4> {
                type Error = Infallible;
                type Duty = u16;

                //NOTE(unsafe) atomic write with no side effects
                fn try_disable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::clear(&(*$TIMX::ptr()).ccer, 12) };
                    Ok(())
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_enable(&mut self) -> Result<(), Self::Error> {
                    unsafe { bb::set(&(*$TIMX::ptr()).ccer, 12) };
                    Ok(())
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).ccr4.read().ccr1_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic read with no side effects
                fn try_get_max_duty(&self) -> Result<u16, Self::Error> {
                    Ok(unsafe { (*$TIMX::ptr()).arr.read().arr_l().bits() as u16 })
                }

                //NOTE(unsafe) atomic write with no side effects
                fn try_set_duty(&mut self, duty: u16) -> Result<(), Self::Error> {
                    unsafe { (*$TIMX::ptr()).ccr4.write(|w| w.ccr1_l().bits(duty.into())) };
                    Ok(())
                }
            }
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
pwm_all_channels!(
    TIM1: (tim1, apb2enr, apb2rstr, 0u8, pclk2, ppre2),
);

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
pwm_2_channels!(
    TIM9: (tim9, apb2enr, apb2rstr, 16u8, pclk2, ppre2),
);

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
pwm_1_channel!(
    TIM11: (tim11, apb2enr, apb2rstr, 18u8, pclk2, ppre2),
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
pwm_all_channels!(
    TIM2: (tim2, apb1enr, apb1rstr, 0u8, pclk1, ppre1),
    TIM3: (tim3, apb1enr, apb1rstr, 1u8, pclk1, ppre1),
    TIM4: (tim4, apb1enr, apb1rstr, 2u8, pclk1, ppre1),
    TIM5: (tim5, apb1enr, apb1rstr, 3u8, pclk1, ppre1),
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
pwm_1_channel!(
    TIM10: (tim10, apb2enr, apb2rstr, 17u8, pclk2, ppre2),
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
pwm_all_channels!(
    TIM8: (tim8, apb2enr, apb2rstr, 1u8, pclk2, ppre2),
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
pwm_2_channels!(
    TIM12: (tim12, apb1enr, apb1rstr, 6u8, pclk1, ppre1),
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
pwm_1_channel!(
    TIM13: (tim13, apb1enr, apb1rstr, 7u8, pclk1, ppre1),
    TIM14: (tim14, apb1enr, apb1rstr, 8u8, pclk1, ppre1),
);

#[cfg(feature = "stm32f410")]
pwm_tim5_f410!(
    TIM5: (tim5, apb1enr, apb1rstr, 3u8, pclk1, ppre1),
);
