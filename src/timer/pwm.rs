//! Provides basic Pulse-width modulation (PWM) capabilities
//!
//! There are 2 main structures [`Pwm`] and [`PwmHz`]. Both structures implement [`embedded_hal_02::Pwm`] and have some additional API.
//!
//! First one is based on [`FTimer`] with fixed prescaler
//! and easy to use with [`fugit::TimerDurationU32`] for setting pulse width and period without advanced calculations.
//!
//! Second one is based on [`Timer`] with dynamic internally calculated prescaler and require [`fugit::Hertz`] to set period.
//!
//! The main way to run PWM is calling [`FTimer::pwm`] with initial `period`/`frequency` corresponding PWM period.
//! This returns [`PwmManager`] and a tuple of all [`PwmChannel`]s supported by timer.
//! Also there is [`PwmExt`] trait implemented on `pac::TIMx` to simplify creating new structure.
//!
//! ```rust,ignore
//! let (pwm_manager, (pwm_ch1, pwm_ch2, ..)) = dp.TIM1.pwm_us(100.micros(), &clocks);
//! ```
//!
//! Each `PwmChannel` implements [`embedded_hal::pwm::SetDutyCycle`].
//! They are disabled.
//! To enable `PwmChannel` you need to pass one or more regular pins allowed by channel
//! using `with` or `with_open_drain`.
//! Also you can pass complementary pins by `.with_complementary(other_complementary_pin)`.
//! After connecting pins you can dynamically enable main or complementary channels with `enable` and `enable_complementary`
//! and change their polarity with `set_polarity` and `set_complementary_polarity`.
//!
//! ```rust,ignore
//! let mut pwm_c1 = pwm_c1.with(gpioa.pa8).with_complementary(gpioa.pa7);
//! pwm_c1.enable();
//! pwm_c1.enable_complementary();
//! ```
//!
//! By default `PwmChannel` contains information about connected pins to be possible to `release` them.
//! But you can `erase` this information to constuct [`ErasedChannel`] which can be collected to array.
//! Note that this operation is irreversible.
//!
//! `PwmManager` allows you to change PWM `period`/`frequency` and also has methods for advanced PWM control.

use super::sealed::Split;
use super::{
    compute_arr_presc, Advanced, CPin, CenterAlignedMode, FTimer, IdleState, Instance, NCPin, Ocm,
    Polarity, Timer, WithPwm,
};
pub use super::{Ch, C1, C2, C3, C4};
use crate::gpio::{OpenDrain, PushPull};
use crate::rcc::Rcc;
use core::ops::{Deref, DerefMut};
use fugit::{HertzU32 as Hertz, TimerDurationU32};

pub trait PwmExt
where
    Self: Sized + Instance + WithPwm + Split,
{
    fn pwm<const FREQ: u32>(
        self,
        time: TimerDurationU32<FREQ>,
        rcc: &mut Rcc,
    ) -> (PwmManager<Self, FREQ>, Self::Channels);

    fn pwm_hz(self, freq: Hertz, rcc: &mut Rcc) -> (PwmHzManager<Self>, Self::Channels);

    fn pwm_us(
        self,
        time: TimerDurationU32<1_000_000>,
        rcc: &mut Rcc,
    ) -> (PwmManager<Self, 1_000_000>, Self::Channels) {
        self.pwm::<1_000_000>(time, rcc)
    }
}

impl<TIM> PwmExt for TIM
where
    Self: Sized + Instance + WithPwm + Split,
{
    fn pwm<const FREQ: u32>(
        self,
        time: TimerDurationU32<FREQ>,
        rcc: &mut Rcc,
    ) -> (PwmManager<Self, FREQ>, Self::Channels) {
        FTimer::<Self, FREQ>::new(self, rcc).pwm(time)
    }

    fn pwm_hz(self, time: Hertz, rcc: &mut Rcc) -> (PwmHzManager<Self>, Self::Channels) {
        Timer::new(self, rcc).pwm_hz(time)
    }
}

impl<TIM: Instance + WithPwm + Split> Timer<TIM> {
    pub fn pwm_hz(mut self, freq: Hertz) -> (PwmHzManager<TIM>, TIM::Channels) {
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

        (PwmHzManager { timer: self }, TIM::split())
    }
}

impl<TIM: Instance + WithPwm + Split, const FREQ: u32> FTimer<TIM, FREQ> {
    pub fn pwm(mut self, time: TimerDurationU32<FREQ>) -> (PwmManager<TIM, FREQ>, TIM::Channels) {
        // The reference manual is a bit ambiguous about when enabling this bit is really
        // necessary, but since we MUST enable the preload for the output channels then we
        // might as well enable for the auto-reload too
        self.tim.enable_preload(true);

        self.tim.set_auto_reload(time.ticks() - 1).unwrap();

        // Trigger update event to load the registers
        self.tim.trigger_update();

        self.tim.start_pwm();

        (PwmManager { timer: self }, TIM::split())
    }
}

pub struct PwmChannelDisabled<TIM, const C: u8> {
    pub(super) tim: TIM,
}

impl<TIM: crate::Steal, const C: u8> PwmChannelDisabled<TIM, C> {
    pub(crate) fn new() -> Self {
        Self {
            tim: unsafe { TIM::steal() },
        }
    }
}
impl<TIM: Instance + WithPwm + crate::Steal, const C: u8> PwmChannelDisabled<TIM, C>
where
    TIM: CPin<C>,
{
    pub fn with(
        mut self,
        pin: impl Into<TIM::Ch<PushPull>>,
    ) -> PwmChannel<TIM, C, false, PushPull> {
        self.tim.preload_output_channel_in_mode(C, Ocm::PwmMode1);
        PwmChannel {
            tim: self.tim,
            lines: Lines::One(pin.into()),
        }
    }
    pub fn with_open_drain(
        mut self,
        pin: impl Into<TIM::Ch<OpenDrain>>,
    ) -> PwmChannel<TIM, C, false, OpenDrain> {
        self.tim.preload_output_channel_in_mode(C, Ocm::PwmMode1);
        PwmChannel {
            tim: self.tim,
            lines: Lines::One(pin.into()),
        }
    }
}

#[derive(Debug)]
pub enum Lines<P> {
    One(P),
    Two(P, P),
    Three(P, P, P),
    Four(P, P, P, P),
}
impl<P> Lines<P> {
    pub fn and(self, pin: P) -> Self {
        match self {
            Self::One(p) => Self::Two(p, pin),
            Self::Two(p1, p2) => Self::Three(p1, p2, pin),
            Self::Three(p1, p2, p3) => Self::Four(p1, p2, p3, pin),
            Self::Four(_, _, _, _) => unreachable!(),
        }
    }
}

pub struct PwmChannel<TIM: CPin<C>, const C: u8, const COMP: bool = false, Otype = PushPull> {
    pub(super) tim: TIM,
    lines: Lines<TIM::Ch<Otype>>,
    // TODO: add complementary pins
}

impl<TIM: Instance + WithPwm + CPin<C>, const C: u8, const COMP: bool, Otype>
    PwmChannel<TIM, C, COMP, Otype>
{
    pub const fn channel(&self) -> u8 {
        C
    }
    pub fn release(mut self) -> (PwmChannelDisabled<TIM, C>, Lines<TIM::Ch<Otype>>) {
        self.tim.freeze_output_channel(C);
        (PwmChannelDisabled { tim: self.tim }, self.lines)
    }
    pub fn erase(self) -> ErasedChannel<TIM> {
        ErasedChannel {
            _tim: self.tim,
            channel: C,
        }
    }
}
impl<TIM: Instance + CPin<C>, const C: u8, const COMP: bool, Otype>
    PwmChannel<TIM, C, COMP, Otype>
{
    pub fn with(self, pin: impl Into<TIM::Ch<Otype>>) -> Self {
        Self {
            tim: self.tim,
            lines: self.lines.and(pin.into()),
        }
    }
}
impl<TIM: Instance + CPin<C> + NCPin<C>, const C: u8, const COMP: bool, Otype>
    PwmChannel<TIM, C, COMP, Otype>
{
    pub fn with_complementary(
        self,
        pin: impl Into<TIM::ChN<Otype>>,
    ) -> PwmChannel<TIM, C, true, Otype> {
        let _pin = pin.into();
        PwmChannel {
            tim: self.tim,
            lines: self.lines,
        }
    }
}

pub struct ErasedChannel<TIM> {
    _tim: TIM,
    channel: u8,
}

impl<TIM> ErasedChannel<TIM> {
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

macro_rules! ch_impl {
    () => {
        /// Disable PWM channel
        #[inline]
        pub fn disable(&mut self) {
            TIM::enable_channel(self.channel(), false);
        }

        /// Enable PWM channel
        #[inline]
        pub fn enable(&mut self) {
            TIM::enable_channel(self.channel(), true);
        }

        /// Get PWM channel duty cycle
        #[inline]
        pub fn get_duty(&self) -> u16 {
            TIM::read_cc_value(self.channel()) as u16
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
            TIM::set_cc_value(self.channel(), duty as u32)
        }

        /// Set PWM channel polarity
        #[inline]
        pub fn set_polarity(&mut self, p: Polarity) {
            TIM::set_pwm_channel_polarity(self.channel(), p);
        }

        /// Set complementary PWM channel polarity
        #[inline]
        pub fn set_complementary_polarity(&mut self, p: Polarity) {
            TIM::set_pwm_nchannel_polarity(self.channel(), p);
        }
    };
}

macro_rules! chN_impl {
    () => {
        /// Disable complementary PWM channel
        #[inline]
        pub fn disable_complementary(&mut self) {
            TIM::enable_nchannel(self.channel(), false);
        }

        /// Enable complementary PWM channel
        #[inline]
        pub fn enable_complementary(&mut self) {
            TIM::enable_nchannel(self.channel(), true);
        }

        /// Set PWM channel idle state
        #[inline]
        pub fn set_idle_state(&mut self, s: IdleState) {
            TIM::idle_state(self.channel(), false, s);
        }

        /// Set complementary PWM channel idle state
        #[inline]
        pub fn set_complementary_idle_state(&mut self, s: IdleState) {
            TIM::idle_state(self.channel(), true, s);
        }
    };
}

impl<TIM: Instance + WithPwm + CPin<C>, const C: u8, const COMP: bool, Otype>
    PwmChannel<TIM, C, COMP, Otype>
{
    ch_impl!();
}

impl<TIM: Instance + WithPwm + Advanced + CPin<C>, const C: u8, Otype>
    PwmChannel<TIM, C, true, Otype>
{
    chN_impl!();
}

impl<TIM: Instance + WithPwm> ErasedChannel<TIM> {
    ch_impl!();
}

impl<TIM: Instance + WithPwm + Advanced> ErasedChannel<TIM> {
    chN_impl!();
}

pub struct PwmManager<TIM, const FREQ: u32>
where
    TIM: Instance + WithPwm,
{
    pub(super) timer: FTimer<TIM, FREQ>,
}

impl<TIM, const FREQ: u32> PwmManager<TIM, FREQ>
where
    TIM: Instance + WithPwm + Split,
{
    pub fn release(mut self, _channels: TIM::Channels) -> FTimer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.timer
    }
}

impl<TIM, const FREQ: u32> Deref for PwmManager<TIM, FREQ>
where
    TIM: Instance + WithPwm,
{
    type Target = FTimer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, const FREQ: u32> DerefMut for PwmManager<TIM, FREQ>
where
    TIM: Instance + WithPwm,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

pub struct PwmHzManager<TIM>
where
    TIM: Instance + WithPwm,
{
    pub(super) timer: Timer<TIM>,
}

impl<TIM> PwmHzManager<TIM>
where
    TIM: Instance + WithPwm + Split,
{
    pub fn release(mut self, _channels: TIM::Channels) -> Timer<TIM> {
        // stop timer
        self.tim.cr1_reset();
        self.timer
    }
}

impl<TIM> Deref for PwmHzManager<TIM>
where
    TIM: Instance + WithPwm,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM> DerefMut for PwmHzManager<TIM>
where
    TIM: Instance + WithPwm,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM, const FREQ: u32> PwmManager<TIM, FREQ>
where
    TIM: Instance + WithPwm,
{
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
}

impl<TIM> PwmHzManager<TIM>
where
    TIM: Instance + WithPwm,
{
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
}

macro_rules! impl_advanced {
    () => {
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

        /// Sets the alignment mode
        #[inline]
        pub fn set_cms(&mut self, mode: CenterAlignedMode) {
            self.tim.enable_counter(false);
            TIM::set_cms(mode);
            self.tim.enable_counter(true);
        }
    };
}

impl<TIM, const FREQ: u32> PwmManager<TIM, FREQ>
where
    TIM: Instance + WithPwm + Advanced,
{
    impl_advanced!();
}

impl<TIM> PwmHzManager<TIM>
where
    TIM: Instance + WithPwm + Advanced,
{
    impl_advanced!();
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
