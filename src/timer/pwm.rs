//! Provides basic Pulse-width modulation (PWM) capabilities
//!
//! There are 2 main structures [`Pwm`] and [`PwmHz`]. Both structures implement [`embedded_hal_02::Pwm`] and have some additional API.
//!
//! First one is based on [`FTimer`] with fixed prescaler
//! and easy to use with [`fugit::TimerDurationU32`] for setting pulse width and period without advanced calculations.
//!
//! Second one is based on [`Timer`] with dynamic internally calculated prescaler and require [`fugit::Hertz`] to set period.
//!
//! You can [`split`](Pwm::split) any of those structures on independent `PwmChannel`s if you need that implement [`embedded_hal_02::PwmPin`]
//! but can't change PWM period.
//!
//! Also there is [`PwmExt`] trait implemented on `pac::TIMx` to simplify creating new structure.
//!
//! You need to pass one or tuple of channels with pins you plan to use and initial `time`/`frequency` corresponding PWM period.
//! Pins can be collected with [`ChannelBuilder`]s in sequence corresponding to the channel number. Smaller channel number first.
//! Each channel group can contain 1 or several main pins and 0, 1 or several complementary pins.
//! Start constructing channel with `new(first_main_pin)`.
//! Then use `.with(other_main_pin)` and `.with_complementary(other_complementary_pin)` accordingly
//! to add advanced pins on same channel.
//!
//! For example:
//! ```rust
//! let channels = (
//!     Channel1::new(gpioa.pa8),
//!     Channel2::new(gpioa.pa9), // use Channel2OD` for `OpenDrain` pin
//! );
//! ```
//! or
//! ```rust,ignore
//! let channels = Channel1::new(gpioa.pa8).with_complementary(gpioa.pa7); // (CH1, CHN1)
//! ```
//!
//! where `CHx` and `CHx_n` are main pins of PWM channel `x` and `CHNx` are complementary pins of PWM channel `x`.
//!
//! After creating structures you can dynamically enable main or complementary channels with `enable` and `enable_complementary`
//! and change their polarity with `set_polarity` and `set_complementary_polarity`.

use super::{
    compute_arr_presc, Advanced, CPin, Channel, FTimer, IdleState, Instance, Ocm, Polarity, Timer,
    WithPwm,
};
pub use super::{Ch, C1, C2, C3, C4};
use crate::gpio::{OpenDrain, PushPull};
use crate::rcc::Clocks;
use core::ops::{Deref, DerefMut};
use fugit::{HertzU32 as Hertz, TimerDurationU32};

pub type Channel1<TIM> = ChannelBuilder<TIM, C1, PushPull>;
pub type Channel2<TIM> = ChannelBuilder<TIM, C2, PushPull>;
pub type Channel3<TIM> = ChannelBuilder<TIM, C3, PushPull>;
pub type Channel4<TIM> = ChannelBuilder<TIM, C4, PushPull>;
pub type Channel1OD<TIM> = ChannelBuilder<TIM, C1, OpenDrain>;
pub type Channel2OD<TIM> = ChannelBuilder<TIM, C2, OpenDrain>;
pub type Channel3OD<TIM> = ChannelBuilder<TIM, C3, OpenDrain>;
pub type Channel4OD<TIM> = ChannelBuilder<TIM, C4, OpenDrain>;

pub enum Lines<P> {
    None,
    One(P),
    Two(P, P),
    Three(P, P, P),
    Four(P, P, P, P),
}

impl<P> Lines<P> {
    pub fn new(pin: P) -> Self {
        Self::One(pin)
    }
    pub fn add(self, pin: P) -> Self {
        match self {
            Self::None => Self::One(pin),
            Self::One(p) => Self::Two(p, pin),
            Self::Two(p1, p2) => Self::Three(p1, p2, pin),
            Self::Three(p1, p2, p3) => Self::Four(p1, p2, p3, pin),
            Self::Four(_, _, _, _) => unreachable!(),
        }
    }
}

pub struct ChannelBuilder<TIM, const C: u8, Otype = PushPull>
where
    TIM: CPin<C>,
{
    lines: Lines<TIM::Ch<Otype>>,
    comp_lines: Lines<TIM::ChN<Otype>>,
}

impl<TIM, Otype, const C: u8> ChannelBuilder<TIM, C, Otype>
where
    TIM: CPin<C>,
{
    pub fn has_complementary(&self) -> bool {
        !matches!(self.comp_lines, Lines::None)
    }

    pub fn new(pin: impl Into<TIM::Ch<Otype>>) -> Self {
        Self {
            lines: Lines::new(pin.into()),
            comp_lines: Lines::None,
        }
    }

    pub fn new_complementary(pin: impl Into<TIM::ChN<Otype>>) -> Self {
        Self {
            lines: Lines::None,
            comp_lines: Lines::new(pin.into()),
        }
    }

    pub fn with(self, pin: impl Into<TIM::Ch<Otype>>) -> Self {
        Self {
            lines: self.lines.add(pin.into()),
            ..self
        }
    }

    pub fn with_complementary(self, pin: impl Into<TIM::ChN<Otype>>) -> Self {
        Self {
            comp_lines: self.comp_lines.add(pin.into()),
            ..self
        }
    }
}

impl<TIM, Otype, const C: u8> sealed::Split for ChannelBuilder<TIM, C, Otype>
where
    TIM: CPin<C>,
{
    type Channels = PwmChannel<TIM, C, Otype>;
    fn split(self) -> Self::Channels {
        PwmChannel {
            _lines: self.lines,
            comp_lines: self.comp_lines,
        }
    }
}

mod sealed {
    pub trait Split {
        type Channels;
        fn split(self) -> Self::Channels;
    }
    macro_rules! split {
        ($($T:ident: $i:tt),+) => {
            impl<$($T),+> Split for ($($T),+)
            where
                $($T: Split,)+
            {
                type Channels = ($($T::Channels),+);
                fn split(self) -> Self::Channels {
                    ($(self.$i.split()),+)
                }
            }
        };
    }
    split!(T1: 0, T2: 1);
    split!(T1: 0, T2: 1, T3: 2);
    split!(T1: 0, T2: 1, T3: 2, T4: 3);
}
#[allow(non_snake_case)]
pub trait Pins<TIM>: sealed::Split {
    fn C1(&self) -> bool {
        false
    }
    fn C2(&self) -> bool {
        false
    }
    fn C3(&self) -> bool {
        false
    }
    fn C4(&self) -> bool {
        false
    }
    fn NC1(&self) -> bool {
        false
    }
    fn NC2(&self) -> bool {
        false
    }
    fn NC3(&self) -> bool {
        false
    }
    fn NC4(&self) -> bool {
        false
    }

    fn check_used(&self, c: Channel) -> Channel {
        if (c == Channel::C1 && self.C1())
            || (c == Channel::C2 && self.C2())
            || (c == Channel::C3 && self.C3())
            || (c == Channel::C4 && self.C4())
        {
            c
        } else {
            panic!("Unused channel")
        }
    }

    fn check_complementary_used(&self, c: Channel) -> Channel {
        if (c == Channel::C1 && self.NC1())
            || (c == Channel::C2 && self.NC2())
            || (c == Channel::C3 && self.NC3())
            || (c == Channel::C4 && self.NC4())
        {
            c
        } else {
            panic!("Unused channel")
        }
    }
}

macro_rules! pins_impl {
    ( $( $(($Otype:ident, $ENCHX:ident, $COMP:ident, $i:tt)),+; )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $($Otype,)+> Pins<TIM> for ($(ChannelBuilder<TIM, $ENCHX, $Otype>),+)
            where
                $(
                    TIM: CPin<$ENCHX>,
                )+
            {
                $(
                    fn $ENCHX(&self) -> bool {
                        true
                    }
                    fn $COMP(&self) -> bool {
                        self.$i.has_complementary()
                    }
                )+
            }
        )+
    };
}

macro_rules! pins_impl1 {
    ( $( ($Otype:ident, $ENCHX:ident, $COMP:ident); )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $Otype> Pins<TIM> for ChannelBuilder<TIM, $ENCHX, $Otype>
            where
                TIM: CPin<$ENCHX>,
            {
                fn $ENCHX(&self) -> bool {
                    true
                }
                fn $COMP(&self) -> bool {
                    self.has_complementary()
                }
            }
        )+
    };
}

pins_impl!(
    (O1, C1, NC1, 0), (O2, C2, NC2, 1), (O3, C3, NC3, 2), (O4, C4, NC4, 3);

                      (O2, C2, NC2, 0), (O3, C3, NC3, 1), (O4, C4, NC4, 2);
    (O1, C1, NC1, 0),                   (O3, C3, NC3, 1), (O4, C4, NC4, 2);
    (O1, C1, NC1, 0), (O2, C2, NC2, 1),                   (O4, C4, NC4, 2);
    (O1, C1, NC1, 0), (O2, C2, NC2, 1), (O3, C3, NC3, 2);

                                        (O3, C3, NC3, 0), (O4, C4, NC4, 1);
                      (O2, C2, NC2, 0),                   (O4, C4, NC4, 1);
                      (O2, C2, NC2, 0), (O3, C3, NC3, 1);
    (O1, C1, NC1, 0),                                     (O4, C4, NC4, 1);
    (O1, C1, NC1, 0),                   (O3, C3, NC3, 1);
    (O1, C1, NC1, 0), (O2, C2, NC2, 1);
);

pins_impl1!(
    (O1, C1, NC1);
    (O2, C2, NC2);
    (O3, C3, NC3);
    (O4, C4, NC4);
);

pub struct PwmChannel<TIM, const C: u8, Otype = PushPull>
where
    TIM: CPin<C>,
{
    _lines: Lines<TIM::Ch<Otype>>,
    comp_lines: Lines<TIM::ChN<Otype>>,
}

impl<TIM, const C: u8, Otype> PwmChannel<TIM, C, Otype>
where
    TIM: CPin<C>,
{
    pub fn has_complementary(&self) -> bool {
        !matches!(self.comp_lines, Lines::None)
    }
}

pub trait PwmExt
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        clocks: &Clocks,
    ) -> Pwm<Self, PINS, FREQ>
    where
        PINS: Pins<Self>;

    fn pwm_hz<PINS>(self, pins: PINS, freq: Hertz, clocks: &Clocks) -> PwmHz<Self, PINS>
    where
        PINS: Pins<Self>;

    fn pwm_us<PINS>(
        self,
        pins: PINS,
        time: TimerDurationU32<1_000_000>,
        clocks: &Clocks,
    ) -> Pwm<Self, PINS, 1_000_000>
    where
        PINS: Pins<Self>,
    {
        self.pwm::<_, 1_000_000>(pins, time, clocks)
    }
}

impl<TIM> PwmExt for TIM
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        clocks: &Clocks,
    ) -> Pwm<TIM, PINS, FREQ>
    where
        PINS: Pins<Self>,
    {
        FTimer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
    }

    fn pwm_hz<PINS>(self, pins: PINS, time: Hertz, clocks: &Clocks) -> PwmHz<TIM, PINS>
    where
        PINS: Pins<Self>,
    {
        Timer::new(self, clocks).pwm_hz(pins, time)
    }
}

impl<TIM, const C: u8, Otype> PwmChannel<TIM, C, Otype>
where
    TIM: CPin<C> + Instance + WithPwm,
{
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
        if self.has_complementary() {
            TIM::set_nchannel_polarity(C, p);
        }
    }
}

impl<TIM, const C: u8, Otype> PwmChannel<TIM, C, Otype>
where
    TIM: CPin<C> + Instance + WithPwm + Advanced,
{
    /// Disable complementary PWM channel
    #[inline]
    pub fn disable_complementary(&mut self) {
        if self.has_complementary() {
            TIM::enable_nchannel(C, false);
        }
    }

    /// Enable complementary PWM channel
    #[inline]
    pub fn enable_complementary(&mut self) {
        if self.has_complementary() {
            TIM::enable_nchannel(C, true);
        }
    }

    /// Set PWM channel idle state
    #[inline]
    pub fn set_idle_state(&mut self, s: IdleState) {
        TIM::idle_state(C, false, s);
    }

    /// Set complementary PWM channel idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, s: IdleState) {
        if self.has_complementary() {
            TIM::idle_state(C, true, s);
        }
    }
}

pub struct PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    timer: Timer<TIM>,
    pins: PINS,
}

impl<TIM, PINS> PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    pub fn release(mut self) -> Timer<TIM> {
        // stop timer
        self.tim.cr1_reset();
        self.timer
    }

    pub fn split(self) -> PINS::Channels {
        self.pins.split()
    }
}

impl<TIM, PINS> Deref for PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, PINS> DerefMut for PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM: Instance + WithPwm> Timer<TIM> {
    pub fn pwm_hz<PINS>(mut self, pins: PINS, freq: Hertz) -> PwmHz<TIM, PINS>
    where
        PINS: Pins<TIM>,
    {
        if pins.C1() {
            self.tim
                .preload_output_channel_in_mode(Channel::C1, Ocm::PwmMode1);
        }
        if pins.C2() && TIM::CH_NUMBER > 1 {
            self.tim
                .preload_output_channel_in_mode(Channel::C2, Ocm::PwmMode1);
        }
        if pins.C3() && TIM::CH_NUMBER > 2 {
            self.tim
                .preload_output_channel_in_mode(Channel::C3, Ocm::PwmMode1);
        }
        if pins.C4() && TIM::CH_NUMBER > 3 {
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

        PwmHz { timer: self, pins }
    }
}

impl<TIM, PINS> PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    /// Enable PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(self.pins.check_used(channel) as u8, true)
    }

    /// Disable PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(self.pins.check_used(channel) as u8, false)
    }

    /// Set the polarity of the active state for the primary PWM output of the timer on channel `channel`
    #[inline]
    pub fn set_polarity(&mut self, channel: Channel, p: Polarity) {
        TIM::set_channel_polarity(self.pins.check_used(channel) as u8, p);
    }

    /// Get the current duty cycle of the timer on channel `channel`
    #[inline]
    pub fn get_duty(&self, channel: Channel) -> u16 {
        TIM::read_cc_value(self.pins.check_used(channel) as u8) as u16
    }

    /// Set the duty cycle of the timer on channel `channel`
    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TIM::set_cc_value(self.pins.check_used(channel) as u8, duty as u32)
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
        TIM::set_nchannel_polarity(self.pins.check_complementary_used(channel) as u8, p);
    }
}

impl<TIM, PINS> PwmHz<TIM, PINS>
where
    TIM: Instance + WithPwm + Advanced,
    PINS: Pins<TIM>,
{
    /// Enable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(self.pins.check_complementary_used(channel) as u8, true)
    }

    /// Disable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(self.pins.check_complementary_used(channel) as u8, false)
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
        TIM::idle_state(self.pins.check_used(channel) as u8, false, s);
    }

    /// Set the complementary pin idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, channel: Channel, s: IdleState) {
        TIM::idle_state(self.pins.check_complementary_used(channel) as u8, true, s);
    }
}

pub struct Pwm<TIM, PINS, const FREQ: u32>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    timer: FTimer<TIM, FREQ>,
    pins: PINS,
}

impl<TIM, PINS, const FREQ: u32> Pwm<TIM, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    pub fn split(self) -> PINS::Channels {
        self.pins.split()
    }

    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.timer
    }
}

impl<TIM, PINS, const FREQ: u32> Deref for Pwm<TIM, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    type Target = FTimer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, PINS, const FREQ: u32> DerefMut for Pwm<TIM, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM: Instance + WithPwm, const FREQ: u32> FTimer<TIM, FREQ> {
    pub fn pwm<PINS>(mut self, pins: PINS, time: TimerDurationU32<FREQ>) -> Pwm<TIM, PINS, FREQ>
    where
        PINS: Pins<TIM>,
    {
        if pins.C1() {
            self.tim
                .preload_output_channel_in_mode(Channel::C1, Ocm::PwmMode1);
        }
        if pins.C2() && TIM::CH_NUMBER > 1 {
            self.tim
                .preload_output_channel_in_mode(Channel::C2, Ocm::PwmMode1);
        }
        if pins.C3() && TIM::CH_NUMBER > 2 {
            self.tim
                .preload_output_channel_in_mode(Channel::C3, Ocm::PwmMode1);
        }
        if pins.C4() && TIM::CH_NUMBER > 3 {
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

        Pwm { timer: self, pins }
    }
}

impl<TIM, PINS, const FREQ: u32> Pwm<TIM, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM>,
{
    /// Enable PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(self.pins.check_used(channel) as u8, true)
    }

    /// Disable PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(self.pins.check_used(channel) as u8, false)
    }

    /// Set the polarity of the active state for the primary PWM output of the timer on channel `channel`
    #[inline]
    pub fn set_polarity(&mut self, channel: Channel, p: Polarity) {
        TIM::set_channel_polarity(self.pins.check_used(channel) as u8, p);
    }

    /// Get the current duty cycle of the timer on channel `channel`
    #[inline]
    pub fn get_duty(&self, channel: Channel) -> u16 {
        TIM::read_cc_value(self.pins.check_used(channel) as u8) as u16
    }
    /// Get the current duty cycle of the timer on channel `channel` and convert to a duration
    #[inline]
    pub fn get_duty_time(&self, channel: Channel) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_cc_value(self.pins.check_used(channel) as u8))
    }

    /// Set the duty cycle of the timer on channel `channel`
    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TIM::set_cc_value(self.pins.check_used(channel) as u8, duty.into())
    }

    /// Set the duty cycle of the timer on channel `channel` from a duration
    #[inline]
    pub fn set_duty_time(&mut self, channel: Channel, duty: TimerDurationU32<FREQ>) {
        TIM::set_cc_value(self.pins.check_used(channel) as u8, duty.ticks())
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
        TIM::set_channel_polarity(self.pins.check_complementary_used(channel) as u8, p);
    }
}

impl<TIM, PINS, const FREQ: u32> Pwm<TIM, PINS, FREQ>
where
    TIM: Instance + WithPwm + Advanced,
    PINS: Pins<TIM>,
{
    /// Enable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(self.pins.check_complementary_used(channel) as u8, true)
    }

    /// Disable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable_complementary(&mut self, channel: Channel) {
        TIM::enable_nchannel(self.pins.check_complementary_used(channel) as u8, false)
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
        TIM::idle_state(self.pins.check_used(channel) as u8, false, s);
    }

    /// Set the complementary pin idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, channel: Channel, s: IdleState) {
        TIM::idle_state(self.pins.check_complementary_used(channel) as u8, true, s);
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
