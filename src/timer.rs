//! Timers
//!
//! Pins can be used for PWM output in both push-pull mode (`Alternate`) and open-drain mode
//! (`AlternateOD`).
#![allow(non_upper_case_globals)]

use core::convert::TryFrom;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use enumflags2::BitFlags;

use crate::bb;
use crate::pac;

use crate::dma::traits::PeriAddress;
use crate::rcc::{self, Clocks};
use fugit::HertzU32 as Hertz;

pub mod counter;
pub use counter::*;
pub mod delay;
pub use delay::*;
pub mod pwm;
pub use pwm::*;
#[cfg(not(feature = "gpio-f410"))]
pub mod pwm_input;
#[cfg(not(feature = "gpio-f410"))]
pub use pwm_input::PwmInput;
#[cfg(feature = "rtic")]
pub mod monotonic;
#[cfg(feature = "rtic")]
pub use monotonic::*;

mod hal_02;
mod hal_1;

/// Timer wrapper.
///
/// This wrapper can be used both for the system timer (SYST) or the
/// general-purpose timers (TIMx).
///
/// Note: If you want to use the timer to sleep a certain amount of time, use
/// [`Delay`](`crate::timer::delay::Delay`).
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) clk: Hertz,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Channel {
    C1 = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
}

pub use crate::gpio::alt::TimCPin as CPin;
pub use crate::gpio::alt::TimNCPin as NCPin;

/// Channel wrapper
pub struct Ch<const C: u8, const COMP: bool>;
pub const C1: u8 = 0;
pub const C2: u8 = 1;
pub const C3: u8 = 2;
pub const C4: u8 = 3;

/// Enum for IO polarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

/// Output Idle state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleState {
    Reset,
    Set,
}

/// SysTick interrupt events
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysEvent {
    /// [Timer] timed out / count down ended
    Update,
}

/// TIM interrupt events
#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    /// Update interrupt enable
    Update = 1 << 0,
    /// Capture/Compare 1 interrupt enable
    C1 = 1 << 1,
    /// Capture/Compare 2 interrupt enable
    C2 = 1 << 2,
    /// Capture/Compare 3 interrupt enable
    C3 = 1 << 3,
    /// Capture/Compare 4 interrupt enable
    C4 = 1 << 4,
    /// COM interrupt enable
    COM = 1 << 5,
    /// Trigger interrupt enable
    Trigger = 1 << 6,
    /// Break interrupt enable
    Break = 1 << 7,
}

/// TIM status flags
#[enumflags2::bitflags]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flag {
    /// Update interrupt flag
    Update = 1 << 0,
    /// Capture/Compare 1 interrupt flag
    C1 = 1 << 1,
    /// Capture/Compare 2 interrupt flag
    C2 = 1 << 2,
    /// Capture/Compare 3 interrupt flag
    C3 = 1 << 3,
    /// Capture/Compare 4 interrupt flag
    C4 = 1 << 4,
    /// COM interrupt flag
    COM = 1 << 5,
    /// Trigger interrupt flag
    Trigger = 1 << 6,
    /// Break interrupt flag
    Break = 1 << 7,
    /// Capture/Compare 1 overcapture flag
    C1Overcapture = 1 << 9,
    /// Capture/Compare 2 overcapture flag
    C2Overcapture = 1 << 10,
    /// Capture/Compare 3 overcapture flag
    C3Overcapture = 1 << 11,
    /// Capture/Compare 4 overcapture flag
    C4Overcapture = 1 << 12,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timer is disabled
    Disabled,
    WrongAutoReload,
}

pub trait TimerExt: Sized {
    /// Non-blocking [Counter] with custom fixed precision
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> Counter<Self, FREQ>;
    /// Non-blocking [Counter] with fixed precision of 1 ms (1 kHz sampling)
    ///
    /// Can wait from 2 ms to 65 sec for 16-bit timer and from 2 ms to 49 days for 32-bit timer.
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    fn counter_ms(self, clocks: &Clocks) -> CounterMs<Self> {
        self.counter::<1_000>(clocks)
    }
    /// Non-blocking [Counter] with fixed precision of 1 μs (1 MHz sampling)
    ///
    /// Can wait from 2 μs to 65 ms for 16-bit timer and from 2 μs to 71 min for 32-bit timer.
    fn counter_us(self, clocks: &Clocks) -> CounterUs<Self> {
        self.counter::<1_000_000>(clocks)
    }
    /// Non-blocking [Counter] with dynamic precision which uses `Hertz` as Duration units
    fn counter_hz(self, clocks: &Clocks) -> CounterHz<Self>;

    /// Blocking [Delay] with custom fixed precision
    fn delay<const FREQ: u32>(self, clocks: &Clocks) -> Delay<Self, FREQ>;
    /// Blocking [Delay] with fixed precision of 1 ms (1 kHz sampling)
    ///
    /// Can wait from 2 ms to 49 days.
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    fn delay_ms(self, clocks: &Clocks) -> DelayMs<Self> {
        self.delay::<1_000>(clocks)
    }
    /// Blocking [Delay] with fixed precision of 1 μs (1 MHz sampling)
    ///
    /// Can wait from 2 μs to 71 min.
    fn delay_us(self, clocks: &Clocks) -> DelayUs<Self> {
        self.delay::<1_000_000>(clocks)
    }
}

impl<TIM: Instance> TimerExt for TIM {
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> Counter<Self, FREQ> {
        FTimer::new(self, clocks).counter()
    }
    fn counter_hz(self, clocks: &Clocks) -> CounterHz<Self> {
        Timer::new(self, clocks).counter_hz()
    }
    fn delay<const FREQ: u32>(self, clocks: &Clocks) -> Delay<Self, FREQ> {
        FTimer::new(self, clocks).delay()
    }
}

pub trait SysTimerExt: Sized {
    /// Creates timer which takes [Hertz] as Duration
    fn counter_hz(self, clocks: &Clocks) -> SysCounterHz;

    /// Creates timer with custom precision (core frequency recommended is known)
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SysCounter<FREQ>;
    /// Creates timer with precision of 1 μs (1 MHz sampling)
    fn counter_us(self, clocks: &Clocks) -> SysCounterUs {
        self.counter::<1_000_000>(clocks)
    }
    /// Blocking [Delay] with custom precision
    fn delay(self, clocks: &Clocks) -> SysDelay;
}

impl SysTimerExt for SYST {
    fn counter_hz(self, clocks: &Clocks) -> SysCounterHz {
        Timer::syst(self, clocks).counter_hz()
    }
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SysCounter<FREQ> {
        Timer::syst(self, clocks).counter()
    }
    fn delay(self, clocks: &Clocks) -> SysDelay {
        Timer::syst_external(self, clocks).delay()
    }
}

impl Timer<SYST> {
    /// Initialize SysTick timer
    pub fn syst(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::Core);
        Self {
            tim,
            clk: clocks.hclk(),
        }
    }

    /// Initialize SysTick timer and set it frequency to `HCLK / 8`
    pub fn syst_external(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::External);
        Self {
            tim,
            clk: clocks.hclk() / 8,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.tim.set_clock_source(SystClkSource::Core);
        self.clk = clocks.hclk();
    }

    pub fn configure_external(&mut self, clocks: &Clocks) {
        self.tim.set_clock_source(SystClkSource::External);
        self.clk = clocks.hclk() / 8;
    }

    pub fn release(self) -> SYST {
        self.tim
    }

    /// Starts listening for an `event`
    pub fn listen(&mut self, event: SysEvent) {
        match event {
            SysEvent::Update => self.tim.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: SysEvent) {
        match event {
            SysEvent::Update => self.tim.disable_interrupt(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Ocm {
    Frozen = 0,
    ActiveOnMatch = 1,
    InactiveOnMatch = 2,
    Toggle = 3,
    ForceInactive = 4,
    ForceActive = 5,
    PwmMode1 = 6,
    PwmMode2 = 7,
}

/// Wrapper type that indicates which register of the contained timer to use for DMA.
pub struct CCR<T, const C: u8>(T);
pub type CCR1<T> = CCR<T, 0>;
pub type CCR2<T> = CCR<T, 1>;
pub type CCR3<T> = CCR<T, 2>;
pub type CCR4<T> = CCR<T, 3>;

/// Wrapper type that indicates which register of the contained timer to use for DMA.
pub struct DMAR<T>(T);

mod sealed {
    use super::{BitFlags, Channel, Event, Flag, IdleState, Ocm, Polarity};
    pub trait General {
        type Width: Into<u32> + From<u16>;
        fn max_auto_reload() -> u32;
        unsafe fn set_auto_reload_unchecked(&mut self, arr: u32);
        fn set_auto_reload(&mut self, arr: u32) -> Result<(), super::Error>;
        fn read_auto_reload() -> u32;
        fn enable_preload(&mut self, b: bool);
        fn enable_counter(&mut self, b: bool);
        fn is_counter_enabled(&self) -> bool;
        fn reset_counter(&mut self);
        fn set_prescaler(&mut self, psc: u16);
        fn read_prescaler(&self) -> u16;
        fn trigger_update(&mut self);
        fn listen_event(
            &mut self,
            disable: Option<BitFlags<Event>>,
            enable: Option<BitFlags<Event>>,
        );
        fn clear_interrupt_flag(&mut self, event: BitFlags<Flag>);
        fn get_interrupt_flag(&self) -> BitFlags<Flag>;
        fn read_count(&self) -> Self::Width;
        fn write_count(&mut self, value: Self::Width);
        fn start_one_pulse(&mut self);
        fn start_free(&mut self, update: bool);
        fn cr1_reset(&mut self);
        fn cnt_reset(&mut self);
    }

    pub trait WithPwmCommon: General {
        const CH_NUMBER: u8;
        const COMP_CH_NUMBER: u8;
        fn read_cc_value(channel: u8) -> u32;
        fn set_cc_value(channel: u8, value: u32);
        fn enable_channel(channel: u8, b: bool);
        fn set_channel_polarity(channel: u8, p: Polarity);
        fn set_nchannel_polarity(channel: u8, p: Polarity);
    }

    pub trait Advanced: WithPwmCommon {
        fn enable_nchannel(channel: u8, b: bool);
        fn set_dtg_value(value: u8);
        fn read_dtg_value() -> u8;
        fn idle_state(channel: u8, comp: bool, s: IdleState);
    }

    pub trait WithPwm: WithPwmCommon {
        fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm);
        fn start_pwm(&mut self);
    }

    pub trait MasterTimer: General {
        type Mms;
        fn master_mode(&mut self, mode: Self::Mms);
    }
}
pub(crate) use sealed::{Advanced, General, MasterTimer, WithPwm, WithPwmCommon};

pub trait Instance:
    crate::Sealed + rcc::Enable + rcc::Reset + rcc::BusTimerClock + General
{
}

macro_rules! hal {
    ($TIM:ty: [
        $Timer:ident,
        $bits:ty,
        $(dmar: $memsize:ty,)?
        $(c: ($cnum:tt $(, $aoe:ident)?),)?
        $(m: $timbase:ident,)?
    ]) => {
        impl Instance for $TIM { }
        pub type $Timer = Timer<$TIM>;

        impl General for $TIM {
            type Width = $bits;

            #[inline(always)]
            fn max_auto_reload() -> u32 {
                <$bits>::MAX as u32
            }
            #[inline(always)]
            unsafe fn set_auto_reload_unchecked(&mut self, arr: u32) {
                self.arr.write(|w| w.bits(arr))
            }
            #[inline(always)]
            fn set_auto_reload(&mut self, arr: u32) -> Result<(), Error> {
                // Note: Make it impossible to set the ARR value to 0, since this
                // would cause an infinite loop.
                if arr > 0 && arr <= Self::max_auto_reload() {
                    Ok(unsafe { self.set_auto_reload_unchecked(arr) })
                } else {
                    Err(Error::WrongAutoReload)
                }
            }
            #[inline(always)]
            fn read_auto_reload() -> u32 {
                let tim = unsafe { &*<$TIM>::ptr() };
                tim.arr.read().bits()
            }
            #[inline(always)]
            fn enable_preload(&mut self, b: bool) {
                self.cr1.modify(|_, w| w.arpe().bit(b));
            }
            #[inline(always)]
            fn enable_counter(&mut self, b: bool) {
                self.cr1.modify(|_, w| w.cen().bit(b));
            }
            #[inline(always)]
            fn is_counter_enabled(&self) -> bool {
                self.cr1.read().cen().is_enabled()
            }
            #[inline(always)]
            fn reset_counter(&mut self) {
                self.cnt.reset();
            }
            #[inline(always)]
            fn set_prescaler(&mut self, psc: u16) {
                self.psc.write(|w| w.psc().bits(psc) );
            }
            #[inline(always)]
            fn read_prescaler(&self) -> u16 {
                self.psc.read().psc().bits()
            }
            #[inline(always)]
            fn trigger_update(&mut self) {
                self.cr1.modify(|_, w| w.urs().set_bit());
                self.egr.write(|w| w.ug().set_bit());
                self.cr1.modify(|_, w| w.urs().clear_bit());
            }
            #[inline(always)]
            fn listen_event(&mut self, disable: Option<BitFlags<Event>>, enable: Option<BitFlags<Event>>) {
                self.dier.modify(|r, w| unsafe { w.bits({
                    let mut bits = r.bits();
                    if let Some(d) = disable {
                        bits &= !(d.bits() as u32);
                    }
                    if let Some(e) = enable {
                        bits |= e.bits() as u32;
                    }
                    bits
                }) });
            }
            #[inline(always)]
            fn clear_interrupt_flag(&mut self, event: BitFlags<Flag>) {
                self.sr.write(|w| unsafe { w.bits(0xffff & !(event.bits() as u32)) });
            }
            #[inline(always)]
            fn get_interrupt_flag(&self) -> BitFlags<Flag> {
                BitFlags::from_bits_truncate(self.sr.read().bits() as u16)
            }
            #[inline(always)]
            fn read_count(&self) -> Self::Width {
                self.cnt.read().bits() as Self::Width
            }
            #[inline(always)]
            fn write_count(&mut self, value:Self::Width) {
                //TODO: remove "unsafe" when possible
                #[allow(unused_unsafe)]
                self.cnt.write(|w|unsafe{w.cnt().bits(value)});
            }
            #[inline(always)]
            fn start_one_pulse(&mut self) {
                self.cr1.modify(|_, w| unsafe { w.bits(1 << 3) }.cen().set_bit());
            }
            #[inline(always)]
            fn start_free(&mut self, update: bool) {
                self.cr1.modify(|_, w| w.cen().set_bit().udis().bit(!update));
            }
            #[inline(always)]
            fn cr1_reset(&mut self) {
                self.cr1.reset();
            }
            #[inline(always)]
            fn cnt_reset(&mut self) {
                self.cnt.reset();
            }
        }

        $(with_dmar!($TIM, $memsize);)?

        $(
            impl WithPwmCommon for $TIM {
                const CH_NUMBER: u8 = $cnum;
                const COMP_CH_NUMBER: u8 = $cnum;

                #[inline(always)]
                fn read_cc_value(c: u8) -> u32 {
                    let tim = unsafe { &*<$TIM>::ptr() };
                    if c < Self::CH_NUMBER {
                        tim.ccr[c as usize].read().bits()
                    } else {
                        0
                    }
                }

                #[inline(always)]
                fn set_cc_value(c: u8, value: u32) {
                    let tim = unsafe { &*<$TIM>::ptr() };
                    if c < Self::CH_NUMBER {
                        #[allow(unused_unsafe)]
                        tim.ccr[c as usize].write(|w| unsafe { w.bits(value) })
                    }
                }

                #[inline(always)]
                fn enable_channel(c: u8, b: bool) {
                    let tim = unsafe { &*<$TIM>::ptr() };
                    if c < Self::CH_NUMBER {
                        unsafe { bb::write(&tim.ccer, c*4, b); }
                    }
                }

                #[inline(always)]
                fn set_channel_polarity(c: u8, p: Polarity) {
                    let tim = unsafe { &*<$TIM>::ptr() };
                    if c < Self::CH_NUMBER {
                        unsafe { bb::write(&tim.ccer, c*4 + 1, p == Polarity::ActiveLow); }
                    }
                }

                #[inline(always)]
                fn set_nchannel_polarity(c: u8, p: Polarity) {
                    let tim = unsafe { &*<$TIM>::ptr() };
                    if c < Self::COMP_CH_NUMBER {
                        unsafe { bb::write(&tim.ccer, c*4 + 3, p == Polarity::ActiveLow); }
                    }
                }
            }

            $(
                impl Advanced for $TIM {
                    fn enable_nchannel(c: u8, b: bool) {
                        let $aoe = ();
                        let tim = unsafe { &*<$TIM>::ptr() };
                        if c < Self::COMP_CH_NUMBER {
                            unsafe { bb::write(&tim.ccer, c*4 + 2, b); }
                        }
                    }
                    fn set_dtg_value(value: u8) {
                        let tim = unsafe { &*<$TIM>::ptr() };
                        tim.bdtr.modify(|_,w| unsafe { w.dtg().bits(value) });
                    }
                    fn read_dtg_value() -> u8 {
                        let tim = unsafe { &*<$TIM>::ptr() };
                        tim.bdtr.read().dtg().bits()
                    }
                    fn idle_state(c: u8, comp: bool, s: IdleState) {
                        let tim = unsafe { &*<$TIM>::ptr() };
                        if !comp {
                            if c < Self::CH_NUMBER {
                                unsafe { bb::write(&tim.cr2, c*2 + 8, s == IdleState::Set); }
                            }
                        } else {
                            if c < Self::COMP_CH_NUMBER {
                                unsafe { bb::write(&tim.cr2, c*2 + 9, s == IdleState::Set); }
                            }
                        }
                    }
                }
            )?

            with_pwm!($TIM: $cnum $(, $aoe)?);
            unsafe impl<const C: u8> PeriAddress for CCR<$TIM, C> {
                #[inline(always)]
                fn address(&self) -> u32 {
                    &self.0.ccr[C as usize] as *const _ as u32
                }

                type MemSize = $bits;
            }
        )?

        $(impl MasterTimer for $TIM {
            type Mms = pac::$timbase::cr2::MMS_A;
            fn master_mode(&mut self, mode: Self::Mms) {
                self.cr2.modify(|_,w| w.mms().variant(mode));
            }
        })?
    };
}
use hal;

macro_rules! with_dmar {
    ($TIM:ty, $memsize:ty) => {
        unsafe impl PeriAddress for DMAR<$TIM> {
            #[inline(always)]
            fn address(&self) -> u32 {
                &self.0.dmar as *const _ as u32
            }

            type MemSize = $memsize;
        }
    };
}

macro_rules! with_pwm {
    ($TIM:ty: [$($Cx:ident, $ccmrx_output:ident, $ocxpe:ident, $ocxm:ident;)+] $(, $aoe:ident)?) => {
        impl WithPwm for $TIM {
            #[inline(always)]
            fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm) {
                match channel {
                    $(
                        Channel::$Cx => {
                            self.$ccmrx_output()
                            .modify(|_, w| w.$ocxpe().set_bit().$ocxm().bits(mode as _) );
                        }
                    )+
                    #[allow(unreachable_patterns)]
                    _ => {},
                }
            }

            #[inline(always)]
            fn start_pwm(&mut self) {
                $(let $aoe = self.bdtr.modify(|_, w| w.aoe().set_bit());)?
                self.cr1.modify(|_, w| w.cen().set_bit());
            }
        }
    };
    ($TIM:ty: 1) => {
        with_pwm!($TIM: [
            C1, ccmr1_output, oc1pe, oc1m;
        ]);
    };
    ($TIM:ty: 2) => {
        with_pwm!($TIM: [
            C1, ccmr1_output, oc1pe, oc1m;
            C2, ccmr1_output, oc2pe, oc2m;
        ]);
    };
    ($TIM:ty: 4 $(, $aoe:ident)?) => {
        with_pwm!($TIM: [
            C1, ccmr1_output, oc1pe, oc1m;
            C2, ccmr1_output, oc2pe, oc2m;
            C3, ccmr2_output, oc3pe, oc3m;
            C4, ccmr2_output, oc4pe, oc4m;
        ] $(, $aoe)?);
    };
}

impl<TIM: Instance> Timer<TIM> {
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            // Enable and reset the timer peripheral
            TIM::enable_unchecked();
            TIM::reset_unchecked();
        }

        Self {
            clk: TIM::timer_clock(clocks),
            tim,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.clk = TIM::timer_clock(clocks);
    }

    pub fn counter_hz(self) -> CounterHz<TIM> {
        CounterHz(self)
    }

    pub fn release(self) -> TIM {
        self.tim
    }
}

impl<TIM: Instance + MasterTimer> Timer<TIM> {
    pub fn set_master_mode(&mut self, mode: TIM::Mms) {
        self.tim.master_mode(mode)
    }
}

/// Timer wrapper for fixed precision timers.
///
/// Uses `fugit::TimerDurationU32` for most of operations
pub struct FTimer<TIM, const FREQ: u32> {
    tim: TIM,
}

/// `FTimer` with precision of 1 μs (1 MHz sampling)
pub type FTimerUs<TIM> = FTimer<TIM, 1_000_000>;

/// `FTimer` with precision of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type FTimerMs<TIM> = FTimer<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> FTimer<TIM, FREQ> {
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            // Enable and reset the timer peripheral
            TIM::enable_unchecked();
            TIM::reset_unchecked();
        }

        let mut t = Self { tim };
        t.configure(clocks);
        t
    }

    /// Calculate prescaler depending on `Clocks` state
    pub fn configure(&mut self, clocks: &Clocks) {
        let clk = TIM::timer_clock(clocks);
        assert!(clk.raw() % FREQ == 0);
        let psc = clk.raw() / FREQ;
        self.tim.set_prescaler(u16::try_from(psc - 1).unwrap());
    }

    /// Creates `Counter` that implements [embedded_hal::timer::CountDown]
    pub fn counter(self) -> Counter<TIM, FREQ> {
        Counter(self)
    }

    /// Creates `Delay` that implements [embedded_hal::blocking::delay] traits
    pub fn delay(self) -> Delay<TIM, FREQ> {
        Delay(self)
    }

    /// Releases the TIM peripheral
    pub fn release(self) -> TIM {
        self.tim
    }
}

impl<TIM: Instance + MasterTimer, const FREQ: u32> FTimer<TIM, FREQ> {
    pub fn set_master_mode(&mut self, mode: TIM::Mms) {
        self.tim.master_mode(mode)
    }
}

#[inline(always)]
pub(crate) const fn compute_arr_presc(freq: u32, clock: u32) -> (u16, u32) {
    let ticks = clock / freq;
    let psc = (ticks - 1) / (1 << 16);
    let arr = ticks / (psc + 1) - 1;
    (psc as u16, arr)
}

impl<TIM: Instance> crate::Listen for Timer<TIM> {
    type Event = Event;
    fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(None, Some(event.into()));
    }
    fn listen_only(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim
            .listen_event(Some(BitFlags::ALL), Some(event.into()));
    }
    fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(Some(event.into()), None);
    }
}

impl<TIM: Instance, const FREQ: u32> crate::Listen for FTimer<TIM, FREQ> {
    type Event = Event;
    fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(None, Some(event.into()));
    }
    fn listen_only(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim
            .listen_event(Some(BitFlags::ALL), Some(event.into()));
    }
    fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(Some(event.into()), None);
    }
}

impl<TIM: Instance> crate::ClearFlags for Timer<TIM> {
    type Flag = Flag;
    fn clear_flags(&mut self, event: impl Into<BitFlags<Flag>>) {
        self.tim.clear_interrupt_flag(event.into());
    }
}

impl<TIM: Instance> crate::ReadFlags for Timer<TIM> {
    type Flag = Flag;
    fn flags(&self) -> BitFlags<Flag> {
        self.tim.get_interrupt_flag()
    }
}

impl<TIM: Instance, const FREQ: u32> crate::ClearFlags for FTimer<TIM, FREQ> {
    type Flag = Flag;
    fn clear_flags(&mut self, event: impl Into<BitFlags<Flag>>) {
        self.tim.clear_interrupt_flag(event.into());
    }
}

impl<TIM: Instance, const FREQ: u32> crate::ReadFlags for FTimer<TIM, FREQ> {
    type Flag = Flag;
    fn flags(&self) -> BitFlags<Flag> {
        self.tim.get_interrupt_flag()
    }
}

#[cfg(not(feature = "gpio-f410"))]
#[cfg(feature = "tim1")]
hal!(pac::TIM1: [Timer1, u16, dmar: u32, c: (4, _aoe), m: tim1,]);
#[cfg(feature = "tim2")]
hal!(pac::TIM2: [Timer2, u32, dmar: u16, c: (4), m: tim2,]);
#[cfg(feature = "tim3")]
hal!(pac::TIM3: [Timer3, u16, dmar: u16, c: (4), m: tim3,]);
#[cfg(feature = "tim4")]
hal!(pac::TIM4: [Timer4, u16, dmar: u16, c: (4), m: tim3,]);
#[cfg(not(feature = "gpio-f410"))]
#[cfg(feature = "tim5")]
hal!(pac::TIM5: [Timer5, u32, dmar: u16, c: (4), m: tim5,]);

// TIM5 on F410 is 16-bit
#[cfg(feature = "gpio-f410")]
#[cfg(feature = "tim1")]
hal!(pac::TIM1: [Timer1, u16, dmar: u16, c: (4, _aoe), m: tim1,]);
#[cfg(feature = "gpio-f410")]
#[cfg(feature = "tim5")]
hal!(pac::TIM5: [Timer5, u16, dmar: u16, c: (4), m: tim5,]);

#[cfg(feature = "tim6")]
hal!(pac::TIM6: [Timer6, u16, m: tim6,]);
#[cfg(feature = "tim7")]
hal!(pac::TIM7: [Timer7, u16, m: tim7,]);
#[cfg(feature = "tim8")]
hal!(pac::TIM8: [Timer8, u16, dmar: u32, c: (4, _aoe), m: tim8,]);
#[cfg(feature = "tim9")]
hal!(pac::TIM9: [Timer9, u16, c: (2),]);
#[cfg(feature = "tim10")]
hal!(pac::TIM10: [Timer10, u16, c: (1),]);
#[cfg(feature = "tim11")]
hal!(pac::TIM11: [Timer11, u16, c: (1),]);
#[cfg(feature = "tim12")]
hal!(pac::TIM12: [Timer12, u16, c: (2),]);
#[cfg(feature = "tim13")]
hal!(pac::TIM13: [Timer13, u16, c: (1),]);
#[cfg(feature = "tim14")]
hal!(pac::TIM14: [Timer14, u16, c: (1),]);
