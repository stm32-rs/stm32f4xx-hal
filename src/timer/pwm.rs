//! Provides basic Pulse-width modulation (PWM) capabilities
//!
//! There are 2 main stuctures [`Pwm`] and [`PwmHz`]. Both structures implement [`embedded_hal::Pwm`] and have some additional API.
//!
//! First one is based on [`FTimer`] with fixed prescaler
//! and easy to use with [`fugit::TimerDurationU32`] for setting pulse width and period without advanced calculations.
//!
//! Second one is based on [`Timer`] with dynamic internally calculated prescaler and require [`fugit::Hertz`] to set period.
//!
//! You can [`split`](Pwm::split) any of those structures on independent `PwmChannel`s if you need that implement [`embedded_hal::PwmPin`]
//! but can't change PWM period.
//!
//! Also there is [`PwmExt`] trait implemented on `pac::TIMx` to simplify creating new structure.
//!
//! You need to pass pins you plan to use and initial `time`/`frequency` corresponding PWM period.
//! Pins can be collected in tuples in sequence corresponding to the channel number. Smaller channel number first.
//! Each channel group can contain 1 or several main pins and 0, 1 or several complementary pins. Main pins first.
//!
//! For example:
//! ```plain,ignore
//! ( (CH1, CHN1),    CH2,    ( (CH3_1, CH3_2), CHN3 ) )
//!   | chan. 1 |  |chan. 2|  |       chan. 3        |
//! ```
//! or
//! ```rust,ignore
//! let channels = (gpioa.pa7.into_alternate(), gpioa.pa8.into_alternate()); // error: (CHN1, CH1)
//!
//! let channels = (gpioa.pa8.into_alternate(), gpioa.pa7.into_alternate()); // good: (CH1, CHN1)
//! ```
//!
//! where `CHx` and `CHx_n` are main pins of PWM channel `x` and `CHNx` are complementary pins of PWM channel `x`.
//!
//! After creating structures you can dynamically enable main or complementary channels with `enable` and `enable_complementary`
//! and change their polarity with `set_polarity` and `set_complementary_polarity`.

use super::{
    compute_arr_presc, Advanced, Channel, FTimer, IdleState, Instance, Ocm, Polarity, Timer,
    WithPwm,
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
    const NC1: bool = false;
    const NC2: bool = false;
    const NC3: bool = false;
    const NC4: bool = false;
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

    fn check_complementary_used(c: Channel) -> Channel {
        if (c == Channel::C1 && Self::NC1)
            || (c == Channel::C2 && Self::NC2)
            || (c == Channel::C3 && Self::NC3)
            || (c == Channel::C4 && Self::NC4)
        {
            c
        } else {
            panic!("Unused channel")
        }
    }

    fn split() -> Self::Channels;
}
pub use super::{CPin, Ch, NCPin, C1, C2, C3, C4};

pub struct PwmChannel<TIM, const C: u8, const COMP: bool = false> {
    pub(super) _tim: PhantomData<TIM>,
}

pub trait PwmPin<TIM, const C: u8, const COMP: bool = false> {}

macro_rules! pins_impl {
    ( $( ( $($PINX:ident),+ ), ( $($ENCHX:ident),+ ), ( $($COMP:ident),+ ); )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $($PINX,)+ $(const $COMP: bool,)+> Pins<TIM, ($(Ch<$ENCHX, $COMP>),+)> for ($($PINX),+)
            where
                TIM: Instance + WithPwm,
                $($PINX: PwmPin<TIM, $ENCHX, $COMP>,)+
            {
                $(const $ENCHX: bool = true;)+
                $(const $COMP: bool = true;)+
                type Channels = ($(PwmChannel<TIM, $ENCHX, $COMP>),+);
                fn split() -> Self::Channels {
                    ($(PwmChannel::<TIM, $ENCHX, $COMP>::new()),+)
                }
            }
        )+
    };
}

pins_impl!(
    (P1, P2, P3, P4), (C1, C2, C3, C4), (NC1, NC2, NC3, NC4);
    (P2, P3, P4), (C2, C3, C4), (NC2, NC3, NC4);
    (P1, P3, P4), (C1, C3, C4), (NC1, NC3, NC4);
    (P1, P2, P4), (C1, C2, C4), (NC1, NC2, NC4);
    (P1, P2, P3), (C1, C2, C3), (NC1, NC2, NC3);
    (P3, P4), (C3, C4), (NC3, NC4);
    (P2, P4), (C2, C4), (NC2, NC4);
    (P2, P3), (C2, C3), (NC2, NC3);
    (P1, P4), (C1, C4), (NC1, NC4);
    (P1, P3), (C1, C3), (NC1, NC3);
    (P1, P2), (C1, C2), (NC1, NC2);
    (P1), (C1), (NC1);
    (P2), (C2), (NC2);
    (P3), (C3), (NC3);
    (P4), (C4), (NC4);
);

macro_rules! tuples {
    ( $( $trait:ident, ( $($PX:ident),+ ); )+ ) => {
        $(
            impl<TIM, $($PX,)+ const C: u8> $trait<TIM, C> for ($($PX),+)
            where
                $($PX: CPin<TIM, C>,)+
            {
            }
        )+
    };
}

tuples! {
    CPin, (P1, P2);
    CPin, (P1, P2, P3);
    CPin, (P1, P2, P3, P4);
    NCPin, (P1, P2);
    NCPin, (P1, P2, P3);
    NCPin, (P1, P2, P3, P4);
}

impl<P, TIM, const C: u8> PwmPin<TIM, C> for P where P: CPin<TIM, C> {}
impl<P, NP, TIM, const C: u8> PwmPin<TIM, C, true> for (P, NP)
where
    P: CPin<TIM, C>,
    NP: NCPin<TIM, C>,
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

impl<TIM, const C: u8, const COMP: bool> PwmChannel<TIM, C, COMP> {
    pub(crate) fn new() -> Self {
        Self {
            _tim: core::marker::PhantomData,
        }
    }
}

impl<TIM: Instance + WithPwm, const C: u8, const COMP: bool> PwmChannel<TIM, C, COMP> {
    /// Disable PWM channel
    #[inline]
    pub fn disable(&mut self) {
        TIM::enable_channel(C, false);
    }

    /// Enable PWM channel
    #[inline]
    pub fn enable(&mut self) {
        TIM::enable_channel(C, true);
    }

    /// Set PWM channel polarity
    #[inline]
    pub fn set_polarity(&mut self, p: Polarity) {
        TIM::set_channel_polarity(C, p);
    }

    /// Get PWM channel duty cycle
    #[inline]
    pub fn get_duty(&self) -> u16 {
        TIM::read_cc_value(C) as u16
    }

    /// Get the maximum duty cycle value of the PWM channel
    ///
    /// If `0` returned means max_duty is 2^16
    #[inline]
    pub fn get_max_duty(&self) -> u16 {
        (TIM::read_auto_reload() as u16).wrapping_add(1)
    }

    /// Set PWM channel duty cycle
    #[inline]
    pub fn set_duty(&mut self, duty: u16) {
        TIM::set_cc_value(C, duty as u32)
    }

    /// Set complementary PWM channel polarity
    #[inline]
    pub fn set_complementary_polarity(&mut self, p: Polarity) {
        TIM::set_nchannel_polarity(C, p);
    }
}

impl<TIM: Instance + WithPwm + Advanced, const C: u8> PwmChannel<TIM, C, true> {
    /// Disable complementary PWM channel
    #[inline]
    pub fn disable_complementary(&mut self) {
        TIM::enable_nchannel(C, false);
    }

    /// Enable complementary PWM channel
    #[inline]
    pub fn enable_complementary(&mut self) {
        TIM::enable_nchannel(C, true);
    }

    /// Set PWM channel idle state
    #[inline]
    pub fn set_idle_state(&mut self, s: IdleState) {
        TIM::idle_state(C, false, s);
    }

    /// Set complementary PWM channel idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, s: IdleState) {
        TIM::idle_state(C, true, s);
    }
}

pub struct PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: Timer<TIM>,
    _pins: PhantomData<(P, PINS)>,
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
        PINS::split()
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
    pub fn pwm_hz<P, PINS>(mut self, _pins: PINS, freq: Hertz) -> PwmHz<TIM, P, PINS>
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
            _pins: PhantomData,
        }
    }
}

impl<TIM, P, PINS> PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    /// Enable PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, true)
    }

    /// Disable PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, false)
    }

    /// Set the polarity of the active state for the primary PWM output of the timer on channel `channel`
    #[inline]
    pub fn set_polarity(&mut self, channel: Channel, p: Polarity) {
        TIM::set_channel_polarity(PINS::check_used(channel) as u8, p);
    }

    /// Get the current duty cycle of the timer on channel `channel`
    #[inline]
    pub fn get_duty(&self, channel: Channel) -> u16 {
        TIM::read_cc_value(PINS::check_used(channel) as u8) as u16
    }

    /// Set the duty cycle of the timer on channel `channel`
    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TIM::set_cc_value(PINS::check_used(channel) as u8, duty as u32)
    }

    /// Get the maximum duty cycle value of the timer
    ///
    /// If `0` returned means max_duty is 2^16
    pub fn get_max_duty(&self) -> u16 {
        (TIM::read_auto_reload() as u16).wrapping_add(1)
    }

    /// Get the PWM frequency of the timer in Hertz
    pub fn get_period(&self) -> Hertz {
        let clk = self.clk;
        let psc = self.tim.read_prescaler() as u32;
        let arr = TIM::read_auto_reload();

        // Length in ms of an internal clock pulse
        clk / ((psc + 1) * (arr + 1))
    }

    /// Set the PWM frequency for the timer in Hertz
    pub fn set_period(&mut self, period: Hertz) {
        let clk = self.clk;

        let (psc, arr) = compute_arr_presc(period.raw(), clk.raw());
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr).unwrap();
        self.tim.cnt_reset();
    }

    /// Set the polarity of the active state for the complementary PWM output of the advanced timer on channel `channel`
    #[inline]
    pub fn set_complementary_polarity(&mut self, channel: Channel, p: Polarity) {
        TIM::set_channel_polarity(PINS::check_complementary_used(channel) as u8, p);
    }
}

impl<TIM, P, PINS> PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm + Advanced,
    PINS: Pins<TIM, P>,
{
    /// Enable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(PINS::check_complementary_used(channel) as u8, true)
    }

    /// Disable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(PINS::check_complementary_used(channel) as u8, false)
    }

    /// Set number DTS ticks during that the primary and complementary PWM pins are simultaneously forced to their inactive states
    /// ( see [`Polarity`] setting ) when changing PWM state. This duration when both channels are in an 'off' state  is called 'dead time'.
    ///
    /// This is necessary in applications like motor control or power converters to prevent the destruction of the switching elements by
    /// short circuit in the moment of switching.
    #[inline]
    pub fn set_dead_time(&mut self, dts_ticks: u16) {
        let bits = pack_ceil_dead_time(dts_ticks);
        TIM::set_dtg_value(bits);
    }

    /// Set raw dead time (DTG) bits
    ///
    /// The dead time generation is nonlinear and constrained by the DTS tick duration. DTG register configuration and calculation of
    /// the actual resulting dead time is described in the application note RM0368 from ST Microelectronics
    #[inline]
    pub fn set_dead_time_bits(&mut self, bits: u8) {
        TIM::set_dtg_value(bits);
    }

    /// Return dead time for complementary pins in the unit of DTS ticks
    #[inline]
    pub fn get_dead_time(&self) -> u16 {
        unpack_dead_time(TIM::read_dtg_value())
    }

    /// Get raw dead time (DTG) bits
    #[inline]
    pub fn get_dead_time_bits(&self) -> u8 {
        TIM::read_dtg_value()
    }

    /// Set the pin idle state
    #[inline]
    pub fn set_idle_state(&mut self, channel: Channel, s: IdleState) {
        TIM::idle_state(PINS::check_used(channel) as u8, false, s);
    }

    /// Set the complementary pin idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, channel: Channel, s: IdleState) {
        TIM::idle_state(PINS::check_complementary_used(channel) as u8, true, s);
    }
}

pub struct Pwm<TIM, P, PINS, const FREQ: u32>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: FTimer<TIM, FREQ>,
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

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    /// Enable PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, true)
    }

    /// Disable PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, false)
    }

    /// Set the polarity of the active state for the primary PWM output of the timer on channel `channel`
    #[inline]
    pub fn set_polarity(&mut self, channel: Channel, p: Polarity) {
        TIM::set_channel_polarity(PINS::check_used(channel) as u8, p);
    }

    /// Get the current duty cycle of the timer on channel `channel`
    #[inline]
    pub fn get_duty(&self, channel: Channel) -> u16 {
        TIM::read_cc_value(PINS::check_used(channel) as u8) as u16
    }
    /// Get the current duty cycle of the timer on channel `channel` and convert to a duration
    #[inline]
    pub fn get_duty_time(&self, channel: Channel) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_cc_value(PINS::check_used(channel) as u8))
    }

    /// Set the duty cycle of the timer on channel `channel`
    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TIM::set_cc_value(PINS::check_used(channel) as u8, duty.into())
    }

    /// Set the duty cycle of the timer on channel `channel` from a duration
    #[inline]
    pub fn set_duty_time(&mut self, channel: Channel, duty: TimerDurationU32<FREQ>) {
        TIM::set_cc_value(PINS::check_used(channel) as u8, duty.ticks())
    }

    /// Get the maximum duty cycle value of the timer
    ///
    /// If `0` returned means max_duty is 2^16
    pub fn get_max_duty(&self) -> u16 {
        (TIM::read_auto_reload() as u16).wrapping_add(1)
    }

    /// Get the PWM frequency of the timer as a duration
    pub fn get_period(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_auto_reload() + 1)
    }

    /// Set the PWM frequency for the timer from a duration
    pub fn set_period(&mut self, period: TimerDurationU32<FREQ>) {
        self.tim.set_auto_reload(period.ticks() - 1).unwrap();
        self.tim.cnt_reset();
    }

    /// Set the polarity of the active state for the complementary PWM output of the advanced timer on channel `channel`
    #[inline]
    pub fn set_complementary_polarity(&mut self, channel: Channel, p: Polarity) {
        TIM::set_channel_polarity(PINS::check_complementary_used(channel) as u8, p);
    }
}

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm + Advanced,
    PINS: Pins<TIM, P>,
{
    /// Enable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(PINS::check_complementary_used(channel) as u8, true)
    }

    /// Disable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(PINS::check_complementary_used(channel) as u8, false)
    }

    /// Set number DTS ticks during that the primary and complementary PWM pins are simultaneously forced to their inactive states
    /// ( see [`Polarity`] setting ) when changing PWM state. This duration when both channels are in an 'off' state  is called 'dead time'.
    ///
    /// This is necessary in applications like motor control or power converters to prevent the destruction of the switching elements by
    /// short circuit in the moment of switching.
    #[inline]
    pub fn set_dead_time(&mut self, dts_ticks: u16) {
        let bits = pack_ceil_dead_time(dts_ticks);
        TIM::set_dtg_value(bits);
    }

    /// Set raw dead time (DTG) bits
    ///
    /// The dead time generation is nonlinear and constrained by the DTS tick duration. DTG register configuration and calculation of
    /// the actual resulting dead time is described in the application note RM0368 from ST Microelectronics
    #[inline]
    pub fn set_dead_time_bits(&mut self, bits: u8) {
        TIM::set_dtg_value(bits);
    }

    /// Return dead time for complementary pins in the unit of DTS ticks
    #[inline]
    pub fn get_dead_time(&self) -> u16 {
        unpack_dead_time(TIM::read_dtg_value())
    }

    /// Get raw dead time (DTG) bits
    #[inline]
    pub fn get_dead_time_bits(&self) -> u8 {
        TIM::read_dtg_value()
    }

    /// Set the pin idle state
    #[inline]
    pub fn set_idle_state(&mut self, channel: Channel, s: IdleState) {
        TIM::idle_state(PINS::check_used(channel) as u8, false, s);
    }

    /// Set the complementary pin idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, channel: Channel, s: IdleState) {
        TIM::idle_state(PINS::check_complementary_used(channel) as u8, true, s);
    }
}

/// Convert number dead time ticks to raw DTG register bits.
/// Values greater than 1009 result in maximum dead time of 126 us
const fn pack_ceil_dead_time(dts_ticks: u16) -> u8 {
    match dts_ticks {
        0..=127 => dts_ticks as u8,
        128..=254 => ((((dts_ticks + 1) >> 1) - 64) as u8) | 0b_1000_0000,
        255..=504 => ((((dts_ticks + 7) >> 3) - 32) as u8) | 0b_1100_0000,
        505..=1008 => ((((dts_ticks + 15) >> 4) - 32) as u8) | 0b_1110_0000,
        1009.. => 0xff,
    }
}

/// Convert raw DTG register bits value to number of dead time ticks
const fn unpack_dead_time(bits: u8) -> u16 {
    if bits & 0b_1000_0000 == 0 {
        bits as u16
    } else if bits & 0b_0100_0000 == 0 {
        (((bits & !0b_1000_0000) as u16) + 64) * 2
    } else if bits & 0b_0010_0000 == 0 {
        (((bits & !0b_1100_0000) as u16) + 32) * 8
    } else {
        (((bits & !0b_1110_0000) as u16) + 32) * 16
    }
}
