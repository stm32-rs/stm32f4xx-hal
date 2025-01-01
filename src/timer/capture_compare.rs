use super::sealed::{Split, SplitCc};
use super::{CPin, Ccm, Instance, Polarity, Timer, WithCc};
pub use super::{Ch, C1, C2, C3, C4};
use crate::gpio::PushPull;
use crate::rcc::Clocks;
use core::ops::{Deref, DerefMut};
use fugit::HertzU32 as Hertz;

pub trait CcExt
where
    Self: Sized + Instance + WithCc + SplitCc,
{
    fn capture_compare_hz(
        self,
        freq: Hertz,
        clocks: &Clocks,
    ) -> (CcHzManager<Self>, Self::CcChannels);
}

impl<TIM> CcExt for TIM
where
    Self: Sized + Instance + WithCc + SplitCc,
{
    fn capture_compare_hz(
        self,
        time: Hertz,
        clocks: &Clocks,
    ) -> (CcHzManager<Self>, Self::CcChannels) {
        Timer::new(self, clocks).capture_compare_hz(time)
    }
}

impl<TIM: Instance + WithCc + SplitCc> Timer<TIM> {
    // At a timer clock frequency of 100 MHz,
    // the frequency should be in the range from 2000 Hz to the timer clock frequency.
    // It is recommended to use 32-bit timers (TIM2, TIM5).
    pub fn capture_compare_hz(
        mut self,
        freq: Hertz,
    ) -> (CcHzManager<TIM>, TIM::CcChannels) {
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

        self.tim.start_capture_compare();

        (CcHzManager { timer: self }, TIM::split_cc())
    }
}

pub struct CcChannelDisabled<TIM, const C: u8> {
    pub(super) tim: TIM,
}

impl<TIM: crate::Steal, const C: u8> CcChannelDisabled<TIM, C> {
    pub(crate) fn new() -> Self {
        Self {
            tim: unsafe { TIM::steal() },
        }
    }
}
impl<TIM: Instance + WithCc + crate::Steal, const C: u8>
    CcChannelDisabled<TIM, C>
where
    TIM: CPin<C>,
{
    pub fn with(
        mut self,
        pin: impl Into<TIM::Ch<PushPull>>,
    ) -> CcChannel<TIM, C, false, PushPull> {
        self.tim.preload_capture_compare(C, Ccm::InputCapture);
        CcChannel {
            tim: self.tim,
            lines: CaptureLines::One(pin.into()),
        }
    }
}

#[derive(Debug)]
pub enum CaptureLines<P> {
    One(P),
    Two(P, P),
    Three(P, P, P),
    Four(P, P, P, P),
}
impl<P> CaptureLines<P> {
    pub fn and(self, pin: P) -> Self {
        match self {
            Self::One(p) => Self::Two(p, pin),
            Self::Two(p1, p2) => Self::Three(p1, p2, pin),
            Self::Three(p1, p2, p3) => Self::Four(p1, p2, p3, pin),
            Self::Four(_, _, _, _) => unreachable!(),
        }
    }
}

pub struct CcChannel<
    TIM: CPin<C>,
    const C: u8,
    const COMP: bool = false,
    Otype = PushPull,
> {
    pub(super) tim: TIM,
    lines: CaptureLines<TIM::Ch<Otype>>,
    // TODO: add complementary pins
}

impl<TIM: Instance + WithCc + CPin<C>, const C: u8, const COMP: bool, Otype>
    CcChannel<TIM, C, COMP, Otype>
{
    pub const fn channel(&self) -> u8 {
        C
    }
    pub fn release(
        mut self,
    ) -> (
        CcChannelDisabled<TIM, C>,
        CaptureLines<TIM::Ch<Otype>>,
    ) {
        self.disable();
        (CcChannelDisabled { tim: self.tim }, self.lines)
    }
    pub fn erase(self) -> CaptureErasedChannel<TIM> {
        CaptureErasedChannel {
            _tim: self.tim,
            channel: C,
        }
    }
}
impl<TIM: Instance + CPin<C>, const C: u8, const COMP: bool, Otype>
    CcChannel<TIM, C, COMP, Otype>
{
    pub fn with(self, pin: impl Into<TIM::Ch<Otype>>) -> Self {
        Self {
            tim: self.tim,
            lines: self.lines.and(pin.into()),
        }
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
        /// Disable input capture/output compare channel
        #[inline]
        pub fn disable(&mut self) {
            TIM::enable_channel(self.channel(), false);
        }

        /// Enable input capture/output compare channel
        #[inline]
        pub fn enable(&mut self) {
            TIM::enable_channel(self.channel(), true);
        }

        /// Get capture value
        #[inline]
        pub fn get_capture(&self) -> u32 {
            TIM::read_cc_value(self.channel())
        }

        /// Set Input capture/Output compare channel polarity
        #[inline]
        pub fn set_polarity(&mut self, p: Polarity) {
            TIM::set_channel_polarity(self.channel(), p);
        }
    };
}

impl<TIM: Instance + WithCc + CPin<C>, const C: u8, const COMP: bool, Otype>
    CcChannel<TIM, C, COMP, Otype>
{
    ch_impl!();
}

impl<TIM: Instance + WithCc> CaptureErasedChannel<TIM> {
    ch_impl!();
}

pub struct CcHzManager<TIM>
where
    TIM: Instance + WithCc,
{
    pub(super) timer: Timer<TIM>,
}

impl<TIM> CcHzManager<TIM>
where
    TIM: Instance + WithCc + Split,
{
    pub fn release(mut self, _channels: TIM::Channels) -> Timer<TIM> {
        // stop timer
        self.tim.cr1_reset();
        self.timer
    }
}

impl<TIM> Deref for CcHzManager<TIM>
where
    TIM: Instance + WithCc,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM> DerefMut for CcHzManager<TIM>
where
    TIM: Instance + WithCc,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM> CcHzManager<TIM>
where
    TIM: Instance + WithCc,
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
        self.tim.set_auto_reload(1 << 16).unwrap();
        self.tim.cnt_reset();
    }
}
