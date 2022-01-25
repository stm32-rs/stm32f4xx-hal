use crate::{
    time::{Hertz, U32Ext},
    timer::{compute_arr_presc, Channel, Instance, Ocm, Timer, WithPwm},
};
use core::marker::PhantomData;

pub trait Pins<TIM, P> {
    const C1: bool = false;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    type Channels;

    fn check_used(c: Channel) -> Channel {
        if (c == Channel::C1 && Self::C1)
            || (c == Channel::C2 && Self::C2)
            || (c == Channel::C3 && Self::C3)
            || (c == Channel::C4 && Self::C4)
        {
            c
        } else {
            panic!("Unused channel")
        }
    }

    fn split() -> Self::Channels;
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
                fn split() -> Self::Channels {
                    ($(PwmChannel::<TIM, $ENCHX> { _channel: PhantomData, _tim: PhantomData }),+)
                }
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

macro_rules! pwm_pin {
    ($TIMX:ty, $C:ty, $ccr: ident, $bit:literal) => {
        impl PwmChannel<$TIMX, $C> {
            #[inline]
            pub fn disable(&mut self) {
                //NOTE(unsafe) atomic write with no side effects
                unsafe { bb::clear(&(*<$TIMX>::ptr()).ccer, $bit) }
            }

            #[inline]
            pub fn enable(&mut self) {
                //NOTE(unsafe) atomic write with no side effects
                unsafe { bb::set(&(*<$TIMX>::ptr()).ccer, $bit) }
            }

            #[inline]
            pub fn get_duty(&self) -> u16 {
                //NOTE(unsafe) atomic read with no side effects
                unsafe { (*<$TIMX>::ptr()).$ccr.read().bits() as u16 }
            }

            /// If `0` returned means max_duty is 2^16
            #[inline]
            pub fn get_max_duty(&self) -> u16 {
                //NOTE(unsafe) atomic read with no side effects
                unsafe { ((*<$TIMX>::ptr()).arr.read().bits() as u16).wrapping_add(1) }
            }

            #[inline]
            pub fn set_duty(&mut self, duty: u16) {
                //NOTE(unsafe) atomic write with no side effects
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

pub(crate) use pwm_pin;

impl<TIM: Instance + WithPwm> Timer<TIM> {
    pub fn pwm<P, PINS, T>(mut self, _pins: PINS, freq: T) -> Pwm<TIM, P, PINS>
    where
        PINS: Pins<TIM, P>,
        T: Into<Hertz>,
    {
        if PINS::C1 {
            self.tim
                .preload_output_channel_in_mode(Channel::C1, Ocm::PwmMode1);
        }
        if PINS::C2 && TIM::CH_NUMBER > 1 {
            self.tim
                .preload_output_channel_in_mode(Channel::C2, Ocm::PwmMode1);
        }
        if PINS::C3 && TIM::CH_NUMBER > 2 {
            self.tim
                .preload_output_channel_in_mode(Channel::C3, Ocm::PwmMode1);
        }
        if PINS::C4 && TIM::CH_NUMBER > 3 {
            self.tim
                .preload_output_channel_in_mode(Channel::C4, Ocm::PwmMode1);
        }

        // The reference manual is a bit ambiguous about when enabling this bit is really
        // necessary, but since we MUST enable the preload for the output channels then we
        // might as well enable for the auto-reload too
        self.tim.enable_preload(true);

        let (psc, arr) = compute_arr_presc(freq.into().0, self.clk.0);
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr).unwrap();

        // Trigger update event to load the registers
        self.tim.trigger_update();

        self.tim.start_pwm();

        Pwm {
            clk: self.clk,
            tim: self.tim,
            _pins: PhantomData,
        }
    }
}

pub struct Pwm<TIM, P, PINS>
where
    PINS: Pins<TIM, P>,
{
    clk: Hertz,
    tim: TIM,
    _pins: PhantomData<(P, PINS)>,
}

impl<TIM, P, PINS> Pwm<TIM, P, PINS>
where
    PINS: Pins<TIM, P>,
{
    pub fn split(self) -> PINS::Channels {
        PINS::split()
    }
}

impl<TIM, P, PINS> embedded_hal::Pwm for Pwm<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    type Channel = Channel;
    type Duty = u16;
    type Time = Hertz;

    fn enable(&mut self, channel: Self::Channel) {
        self.tim.enable_channel(PINS::check_used(channel), true)
    }

    fn disable(&mut self, channel: Self::Channel) {
        self.tim.enable_channel(PINS::check_used(channel), false)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let duty: u32 = self.tim.read_cc_value(PINS::check_used(channel)).into();
        duty as u16
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.tim
            .set_cc_value(PINS::check_used(channel), duty.into())
    }

    /// If `0` returned means max_duty is 2^16
    fn get_max_duty(&self) -> Self::Duty {
        let arr: u32 = self.tim.read_auto_reload().into();
        (arr as u16).wrapping_add(1)
    }

    fn get_period(&self) -> Self::Time {
        let clk = self.clk;
        let psc = self.tim.read_prescaler() as u32;
        let arr: u32 = self.tim.read_auto_reload().into();

        // Length in ms of an internal clock pulse
        (clk.0 / (psc * arr)).hz()
    }

    fn set_period<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        let clk = self.clk;

        let (psc, arr) = compute_arr_presc(period.into().0, clk.0);
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr).unwrap();
    }
}
