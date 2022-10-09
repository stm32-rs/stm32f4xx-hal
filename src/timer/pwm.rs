use super::{
    compute_arr_presc, Advanced, Channel, FTimer, Instance, Ocm, Polarity, Timer, WithPwm,
};
use crate::rcc::Clocks;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use fugit::{HertzU32 as Hertz, TimerDurationU32};

pub trait Pins<TIM, P> {
    const C1: bool = false;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    type Channels;
    type ChannelPins;

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

    fn split(self) -> Self::Channels;
    fn split_nondestructive(self) -> Self::ChannelPins;
}
pub use super::{CPin, Ch, C1, C2, C3, C4};

pub struct PwmChannel<TIM, const C: u8> {
    _tim: PhantomData<TIM>,
}

pub struct PwmChannelPin<TIM, const C: u8, PIN> {
    pin: PIN,
    channel: PwmChannel<TIM, C>,
}

impl<TIM, const C: u8, PIN> Deref for PwmChannelPin<TIM, C, PIN> {
    type Target = PwmChannel<TIM, C>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

impl<TIM, const C: u8, PIN> DerefMut for PwmChannelPin<TIM, C, PIN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channel
    }
}

macro_rules! pins_impl {
    ( $( ( $($PINX:ident),+ ), ( $($pinx:ident),+ ), ( $($ENCHX:ident),+ ), ( $($CHNUM:literal),+ ); )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $($PINX,)+> Pins<TIM, ($(Ch<$ENCHX>),+)> for ($($PINX),+)
            where
                TIM: Instance + WithPwm,
                $($PINX: CPin<TIM, $ENCHX>,)+
            {
                $(const $ENCHX: bool = true;)+
                type Channels = ($(PwmChannel<TIM, $ENCHX>),+);
                type ChannelPins = ($(PwmChannelPin<TIM, $ENCHX, $PINX>),+);
                fn split(self) -> Self::Channels {
                    ($(PwmChannel::<TIM, $ENCHX>::new()),+)
                }
                fn split_nondestructive(self) -> Self::ChannelPins {
                    let ($($pinx),+) = self;
                    ($(PwmChannelPin::<TIM, $ENCHX, $PINX>::new($pinx)),+)
                }
            }
        )+
    };
}

pins_impl!(
    (P1, P2, P3, P4), (p1, p2, p3, p4), (C1, C2, C3, C4), (0, 1, 2, 3);
    (P2, P3, P4), (p2, p3, p4), (C2, C3, C4), (1, 2, 3);
    (P1, P3, P4), (p1, p3, p4), (C1, C3, C4), (0, 2, 3);
    (P1, P2, P4), (p1, p2, p4), (C1, C2, C4), (0, 1, 3);
    (P1, P2, P3), (p1, p2, p3), (C1, C2, C3), (0, 1, 2);
    (P3, P4), (p3, p4), (C3, C4), (2, 3);
    (P2, P4), (p2, p4), (C2, C4), (1, 3);
    (P2, P3), (p2, p3), (C2, C3), (1, 2);
    (P1, P4), (p1, p4), (C1, C4), (0, 3);
    (P1, P3), (p1, p3), (C1, C3), (0, 2);
    (P1, P2), (p1, p2), (C1, C2), (0, 1);
    (P1), (p1), (C1), (0);
    (P2), (p2), (C2), (1);
    (P3), (p3), (C3), (2);
    (P4), (p4), (C4), (3);
);

// Several pins on 1 channel
impl<TIM, P1, P2, const C: u8> CPin<TIM, C> for (P1, P2)
where
    P1: CPin<TIM, C>,
    P2: CPin<TIM, C>,
{
}
impl<TIM, P1, P2, P3, const C: u8> CPin<TIM, C> for (P1, P2, P3)
where
    P1: CPin<TIM, C>,
    P2: CPin<TIM, C>,
    P3: CPin<TIM, C>,
{
}
#[cfg(feature = "gpio-f446")]
impl<TIM, P1, P2, P3, P4, const C: u8> CPin<TIM, C> for (P1, P2, P3, P4)
where
    P1: CPin<TIM, C>,
    P2: CPin<TIM, C>,
    P3: CPin<TIM, C>,
    P4: CPin<TIM, C>,
{
}

pub trait PwmExt
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<P, PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        clocks: &Clocks,
    ) -> Pwm<Self, P, PINS, FREQ>
    where
        PINS: Pins<Self, P>;

    fn pwm_hz<P, PINS>(self, pins: PINS, freq: Hertz, clocks: &Clocks) -> PwmHz<Self, P, PINS>
    where
        PINS: Pins<Self, P>;

    fn pwm_us<P, PINS>(
        self,
        pins: PINS,
        time: TimerDurationU32<1_000_000>,
        clocks: &Clocks,
    ) -> Pwm<Self, P, PINS, 1_000_000>
    where
        PINS: Pins<Self, P>,
    {
        self.pwm::<_, _, 1_000_000>(pins, time, clocks)
    }
}

impl<TIM> PwmExt for TIM
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<P, PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        clocks: &Clocks,
    ) -> Pwm<TIM, P, PINS, FREQ>
    where
        PINS: Pins<Self, P>,
    {
        FTimer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
    }

    fn pwm_hz<P, PINS>(self, pins: PINS, time: Hertz, clocks: &Clocks) -> PwmHz<TIM, P, PINS>
    where
        PINS: Pins<Self, P>,
    {
        Timer::new(self, clocks).pwm_hz(pins, time)
    }
}

impl<TIM: Instance + WithPwm, const C: u8, PIN> PwmChannelPin<TIM, C, PIN>
where
    PIN: CPin<TIM, C>,
{
    pub(crate) fn new(pin: PIN) -> Self {
        Self {
            pin,
            channel: PwmChannel::new(),
        }
    }

    pub fn erase() -> PwmChannel<TIM, C> {
        PwmChannel::new()
    }

    pub fn release(mut self) -> PIN {
        self.disable();
        self.pin
    }
}

impl<TIM: Instance + WithPwm, const C: u8> PwmChannel<TIM, C> {
    pub(crate) fn new() -> Self {
        Self {
            _tim: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn disable(&mut self) {
        TIM::enable_channel(C, false);
    }

    #[inline]
    pub fn enable(&mut self) {
        TIM::enable_channel(C, true);
    }

    #[inline]
    pub fn set_polarity(&mut self, p: Polarity) {
        TIM::set_channel_polarity(C, p);
    }

    #[inline]
    pub fn set_complementary_polarity(&mut self, p: Polarity) {
        TIM::set_nchannel_polarity(C, p);
    }

    #[inline]
    pub fn get_duty(&self) -> u16 {
        TIM::read_cc_value(C) as u16
    }

    /// If `0` returned means max_duty is 2^16
    #[inline]
    pub fn get_max_duty(&self) -> u16 {
        (TIM::read_auto_reload() as u16).wrapping_add(1)
    }

    #[inline]
    pub fn set_duty(&mut self, duty: u16) {
        TIM::set_cc_value(C, duty as u32)
    }
}

impl<TIM: Instance + WithPwm + Advanced, const C: u8> PwmChannel<TIM, C> {
    #[inline]
    pub fn disable_complementary(&mut self) {
        TIM::enable_nchannel(C, false);
    }
    #[inline]
    pub fn enable_complementary(&mut self) {
        TIM::enable_nchannel(C, true);
    }
}

pub struct PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: Timer<TIM>,
    pins: PINS,
    _marker: PhantomData<P>,
}

impl<TIM, P, PINS> PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn release(mut self) -> Timer<TIM> {
        // stop timer
        self.tim.cr1_reset();
        self.timer
    }

    pub fn split(self) -> PINS::Channels {
        self.pins.split()
    }

    pub fn split_nondestructive(self) -> PINS::ChannelPins {
        self.pins.split_nondestructive()
    }
}

impl<TIM, P, PINS> Deref for PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, P, PINS> DerefMut for PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM: Instance + WithPwm> Timer<TIM> {
    pub fn pwm_hz<P, PINS>(mut self, pins: PINS, freq: Hertz) -> PwmHz<TIM, P, PINS>
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

        let (psc, arr) = compute_arr_presc(freq.raw(), self.clk.raw());
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr).unwrap();

        // Trigger update event to load the registers
        self.tim.trigger_update();

        self.tim.start_pwm();

        PwmHz {
            timer: self,
            pins,
            _marker: PhantomData,
        }
    }
}

impl<TIM, P, PINS> PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, true)
    }

    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, false)
    }

    pub fn get_duty(&self, channel: Channel) -> u16 {
        TIM::read_cc_value(PINS::check_used(channel) as u8) as u16
    }

    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TIM::set_cc_value(PINS::check_used(channel) as u8, duty as u32)
    }

    /// If `0` returned means max_duty is 2^16
    pub fn get_max_duty(&self) -> u16 {
        (TIM::read_auto_reload() as u16).wrapping_add(1)
    }

    pub fn get_period(&self) -> Hertz {
        let clk = self.clk;
        let psc = self.tim.read_prescaler() as u32;
        let arr = TIM::read_auto_reload();

        // Length in ms of an internal clock pulse
        clk / ((psc + 1) * (arr + 1))
    }

    pub fn set_period(&mut self, period: Hertz) {
        let clk = self.clk;

        let (psc, arr) = compute_arr_presc(period.raw(), clk.raw());
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr).unwrap();
    }
}

pub struct Pwm<TIM, P, PINS, const FREQ: u32>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: FTimer<TIM, FREQ>,
    pins: PINS,
    _marker: PhantomData<P>,
}

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn split(self) -> PINS::Channels {
        self.pins.split()
    }

    pub fn split_nondestructive(self) -> PINS::ChannelPins {
        self.pins.split_nondestructive()
    }

    pub fn release(mut self) -> FTimer<TIM, FREQ> {
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
    type Target = FTimer<TIM, FREQ>;
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

impl<TIM: Instance + WithPwm, const FREQ: u32> FTimer<TIM, FREQ> {
    pub fn pwm<P, PINS>(
        mut self,
        pins: PINS,
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
            pins,
            _marker: PhantomData,
        }
    }
}

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, true)
    }

    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, false)
    }

    pub fn get_duty(&self, channel: Channel) -> u16 {
        TIM::read_cc_value(PINS::check_used(channel) as u8) as u16
    }

    pub fn get_duty_time(&self, channel: Channel) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_cc_value(PINS::check_used(channel) as u8))
    }

    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TIM::set_cc_value(PINS::check_used(channel) as u8, duty.into())
    }

    pub fn set_duty_time(&mut self, channel: Channel, duty: TimerDurationU32<FREQ>) {
        TIM::set_cc_value(PINS::check_used(channel) as u8, duty.ticks())
    }

    /// If `0` returned means max_duty is 2^16
    pub fn get_max_duty(&self) -> u16 {
        (TIM::read_auto_reload() as u16).wrapping_add(1)
    }

    pub fn get_period(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_auto_reload() + 1)
    }

    pub fn set_period(&mut self, period: TimerDurationU32<FREQ>) {
        self.tim.set_auto_reload(period.ticks() - 1).unwrap();
    }
}
