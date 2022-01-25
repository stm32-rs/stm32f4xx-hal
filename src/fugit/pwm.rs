use super::{Channel, Instance, Ocm, Timer, WithPwm};
use crate::rcc::Clocks;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use fugit::TimerDurationU32;

pub use crate::pwm::{Pins, PwmChannel};
pub use crate::timer::{CPin, C1, C2, C3, C4};

pub trait PwmExt<P, PINS>
where
    Self: Sized + Instance + WithPwm,
    PINS: Pins<Self, P>,
{
    fn pwm<const FREQ: u32>(
        self,
        clocks: &Clocks,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
    ) -> Pwm<Self, P, PINS, FREQ>;

    fn pwm_us(
        self,
        clocks: &Clocks,
        pins: PINS,
        time: TimerDurationU32<1_000_000>,
    ) -> Pwm<Self, P, PINS, 1_000_000> {
        self.pwm::<1_000_000>(clocks, pins, time)
    }
}

impl<TIM, P, PINS> PwmExt<P, PINS> for TIM
where
    Self: Sized + Instance + WithPwm,
    PINS: Pins<Self, P>,
{
    fn pwm<const FREQ: u32>(
        self,
        clocks: &Clocks,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
    ) -> Pwm<TIM, P, PINS, FREQ> {
        Timer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
    }
}

pub struct Pwm<TIM, P, PINS, const FREQ: u32>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: Timer<TIM, FREQ>,
    _pins: PhantomData<(P, PINS)>,
}

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn split(self) -> PINS::Channels {
        PINS::split()
    }

    pub fn release(mut self) -> Timer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.timer
    }
}

impl<TIM, P, PINS, const FREQ: u32> Deref for Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    type Target = Timer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, P, PINS, const FREQ: u32> DerefMut for Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM: Instance + WithPwm, const FREQ: u32> Timer<TIM, FREQ> {
    pub fn pwm<P, PINS>(
        mut self,
        _pins: PINS,
        time: TimerDurationU32<FREQ>,
    ) -> Pwm<TIM, P, PINS, FREQ>
    where
        PINS: Pins<TIM, P>,
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

        self.tim.set_auto_reload(time.ticks() - 1).unwrap();

        // Trigger update event to load the registers
        self.tim.trigger_update();

        self.tim.start_pwm();

        Pwm {
            timer: self,
            _pins: PhantomData,
        }
    }
}

impl<TIM, P, PINS, const FREQ: u32> embedded_hal::Pwm for Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    type Channel = Channel;
    type Duty = u16;
    type Time = TimerDurationU32<FREQ>;

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
        Self::Time::from_ticks(self.tim.read_auto_reload().into() + 1)
    }

    fn set_period<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        self.tim.set_auto_reload(period.into().ticks() - 1).unwrap();
    }
}
