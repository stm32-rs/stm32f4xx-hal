//! Provides the core functionality of the Input Capture mode.
//!
//! The main way to enable the Input Capture mode is by calling
//! ```rust,ignore
//! Timer::new(dp.TIM5, &clocks).capture_hz(24.MHz());
//! ```
//! In the `capture_hz` method, the desired timer counter frequency is specified.
//! For high accuracy, it is recommended to use 32-bit timers (TIM2, TIM5) and to select the highest possible frequency, ideally the maximum frequency equal to the timer's clock frequency.
//! This returns a `CaptureHzManager` and a tuple of all `CaptureChannel`s supported by the timer. Additionally, the [`CaptureExt`] trait is implemented for `pac::TIMx` to simplify the creation of a new structure.
//!
//! ```rust,ignore
//! let (cc_manager, (cc_ch1, cc_ch2, ...)) = dp.TIM5.capture_hz(24.MHz(), &clocks);
//! ```
//!
//! To enable a [`CaptureChannel`], you need to pass one or more valid pins supported by the channel using the `with` method.
//!
//! [`CaptureHzManager`] also provides additional methods for managing the Input Capture mode, such as `set_prescaler` and `set_filter`.

use super::sealed::{Split, SplitCapture};
use super::{
    CPin, CaptureFilter, CaptureMode, CapturePolarity, CapturePrescaler, Instance, Timer,
    WithCapture,
};
pub use super::{Ch, C1, C2, C3, C4};
use crate::gpio::PushPull;
use crate::rcc::Rcc;
use core::ops::{Deref, DerefMut};
use fugit::HertzU32 as Hertz;

pub trait CaptureExt
where
    Self: Sized + Instance + WithCapture + SplitCapture,
{
    fn capture_hz(
        self,
        freq: Hertz,
        rcc: &mut Rcc,
    ) -> (CaptureHzManager<Self>, Self::CaptureChannels);
}

impl<TIM> CaptureExt for TIM
where
    Self: Sized + Instance + WithCapture + SplitCapture,
{
    fn capture_hz(
        self,
        time: Hertz,
        rcc: &mut Rcc,
    ) -> (CaptureHzManager<Self>, Self::CaptureChannels) {
        Timer::new(self, rcc).capture_hz(time)
    }
}

impl<TIM: Instance + WithCapture + SplitCapture> Timer<TIM> {
    // At a timer clock frequency of 100 MHz,
    // the frequency should be in the range from 2000 Hz to the timer clock frequency.
    // It is recommended to use 32-bit timers (TIM2, TIM5).
    pub fn capture_hz(mut self, freq: Hertz) -> (CaptureHzManager<TIM>, TIM::CaptureChannels) {
        // The reference manual is a bit ambiguous about when enabling this bit is really
        // necessary, but since we MUST enable the preload for the output channels then we
        // might as well enable for the auto-reload too
        self.tim.enable_preload(true);

        let psc = self.clk.raw() / freq.raw();
        assert!(self.clk.raw() % freq.raw() == 0);
        assert!(
            psc <= u16::MAX.into(),
            "PSC value {} exceeds 16-bit limit (65535)",
            psc
        );

        self.tim.set_prescaler(psc as u16 - 1);
        self.tim.set_auto_reload(TIM::max_auto_reload()).unwrap();

        // Trigger update event to load the registers
        self.tim.trigger_update();

        self.tim.start_capture();

        (CaptureHzManager { timer: self }, TIM::split_capture())
    }
}

pub struct CaptureChannelDisabled<TIM, const C: u8> {
    pub(super) tim: TIM,
}

impl<TIM: crate::Steal, const C: u8> CaptureChannelDisabled<TIM, C> {
    pub(crate) fn new() -> Self {
        Self {
            tim: unsafe { TIM::steal() },
        }
    }
}
impl<TIM: Instance + WithCapture + crate::Steal, const C: u8> CaptureChannelDisabled<TIM, C>
where
    TIM: CPin<C>,
{
    pub fn with(
        mut self,
        pin: impl Into<TIM::Ch<PushPull>>,
    ) -> CaptureChannel<TIM, C, false, PushPull> {
        self.tim.preload_capture(C, CaptureMode::InputCapture);
        CaptureChannel {
            tim: self.tim,
            pin: pin.into(),
        }
    }
}

pub struct CaptureChannel<TIM: CPin<C>, const C: u8, const COMP: bool = false, Otype = PushPull> {
    pub(super) tim: TIM,
    pin: TIM::Ch<Otype>,
    // TODO: add complementary pins
}

impl<TIM: Instance + WithCapture + CPin<C>, const C: u8, const COMP: bool, Otype>
    CaptureChannel<TIM, C, COMP, Otype>
{
    pub const fn channel(&self) -> u8 {
        C
    }
    pub fn release(mut self) -> (CaptureChannelDisabled<TIM, C>, TIM::Ch<Otype>) {
        self.disable();
        (CaptureChannelDisabled { tim: self.tim }, self.pin)
    }
    pub fn erase(self) -> CaptureErasedChannel<TIM> {
        CaptureErasedChannel {
            _tim: self.tim,
            channel: C,
        }
    }

    pub fn set_prescaler(&mut self, psc: CapturePrescaler) {
        self.tim.prescaler_capture(C, psc);
    }

    pub fn set_filter(&mut self, filter: CaptureFilter) {
        self.tim.filter_capture(C, filter);
    }
}

pub struct CaptureErasedChannel<TIM> {
    _tim: TIM,
    channel: u8,
}

impl<TIM> CaptureErasedChannel<TIM> {
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

macro_rules! ch_impl {
    () => {
        /// Disable input capture channel
        #[inline]
        pub fn disable(&mut self) {
            TIM::enable_channel(self.channel(), false);
        }

        /// Enable input capture channel
        #[inline]
        pub fn enable(&mut self) {
            TIM::enable_channel(self.channel(), true);
        }

        /// Get capture value
        #[inline]
        pub fn get_capture(&self) -> u32 {
            TIM::read_cc_value(self.channel())
        }

        /// Set input capture channel polarity
        #[inline]
        pub fn set_polarity(&mut self, p: CapturePolarity) {
            TIM::set_capture_channel_polarity(self.channel(), p);
        }
    };
}

impl<TIM: Instance + WithCapture + CPin<C>, const C: u8, const COMP: bool, Otype>
    CaptureChannel<TIM, C, COMP, Otype>
{
    ch_impl!();
}

impl<TIM: Instance + WithCapture> CaptureErasedChannel<TIM> {
    ch_impl!();
}

pub struct CaptureHzManager<TIM>
where
    TIM: Instance + WithCapture,
{
    pub(super) timer: Timer<TIM>,
}

impl<TIM> CaptureHzManager<TIM>
where
    TIM: Instance + WithCapture + Split,
{
    pub fn release(mut self, _channels: TIM::Channels) -> Timer<TIM> {
        // stop timer
        self.tim.cr1_reset();
        self.timer
    }
}

impl<TIM> Deref for CaptureHzManager<TIM>
where
    TIM: Instance + WithCapture,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM> DerefMut for CaptureHzManager<TIM>
where
    TIM: Instance + WithCapture,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM> CaptureHzManager<TIM>
where
    TIM: Instance + WithCapture,
{
    /// Get the PWM frequency of the timer in Hertz
    pub fn get_timer_clock(&self) -> u32 {
        let clk = self.clk;
        let psc = self.tim.read_prescaler() as u32;

        // The frequency of the timer counter increment
        (clk / (psc + 1)).raw()
    }

    /// Set the frequency of the timer counter increment
    pub fn set_timer_clock(&mut self, freq: Hertz) {
        let clk = self.clk;
        let psc = clk.raw() / freq.raw();
        assert!(self.clk.raw() % freq.raw() == 0);
        assert!(
            psc <= u16::MAX.into(),
            "PSC value {} exceeds 16-bit limit (65535)",
            psc
        );

        self.tim.set_prescaler(psc as u16 - 1);
        self.tim.set_auto_reload(TIM::max_auto_reload()).unwrap();
        self.tim.cnt_reset();
    }

    pub fn get_max_auto_reload(&mut self) -> u32 {
        TIM::max_auto_reload()
    }
}
