//! Timers
//!
//! Pins can be used for PWM output in both push-pull mode (`Alternate`) and open-drain mode
//! (`AlternateOD`).
#![allow(non_upper_case_globals)]

use core::convert::TryFrom;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::bb;
use crate::pac::{self, RCC};

use crate::dma::traits::PeriAddress;
use crate::rcc::{self, Clocks};
use fugit::HertzU32 as Hertz;

pub mod counter;
pub use counter::*;
pub mod delay;
pub use delay::*;
mod pins;
pub use pins::*;
pub mod pwm;
pub use pwm::*;
#[cfg(not(feature = "stm32f410"))]
pub mod pwm_input;
#[cfg(not(feature = "stm32f410"))]
pub use pwm_input::PwmInput;
#[cfg(feature = "rtic")]
pub mod monotonic;
#[cfg(feature = "rtic")]
pub use monotonic::*;

mod hal_02;
mod hal_1;

/// Timer wrapper
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

/// Interrupt events
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysEvent {
    /// [Timer] timed out / count down ended
    Update,
}

bitflags::bitflags! {
    pub struct Event: u32 {
        const Update  = 1 << 0;
        const C1 = 1 << 1;
        const C2 = 1 << 2;
        const C3 = 1 << 3;
        const C4 = 1 << 4;
    }
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
    use super::{Channel, Event, Ocm};
    pub trait General {
        type Width: Into<u32> + From<u16>;
        fn max_auto_reload() -> u32;
        unsafe fn set_auto_reload_unchecked(&mut self, arr: u32);
        fn set_auto_reload(&mut self, arr: u32) -> Result<(), super::Error>;
        fn read_auto_reload() -> u32;
        fn enable_preload(&mut self, b: bool);
        fn enable_counter(&mut self);
        fn disable_counter(&mut self);
        fn is_counter_enabled(&self) -> bool;
        fn reset_counter(&mut self);
        fn set_prescaler(&mut self, psc: u16);
        fn read_prescaler(&self) -> u16;
        fn trigger_update(&mut self);
        fn clear_interrupt_flag(&mut self, event: Event);
        fn listen_interrupt(&mut self, event: Event, b: bool);
        fn get_interrupt_flag(&self) -> Event;
        fn read_count(&self) -> Self::Width;
        fn start_one_pulse(&mut self);
        fn start_no_update(&mut self);
        fn cr1_reset(&mut self);
    }

    pub trait WithPwm: General {
        const CH_NUMBER: u8;
        fn read_cc_value(channel: u8) -> u32;
        fn set_cc_value(channel: u8, value: u32);
        fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm);
        fn start_pwm(&mut self);
        fn enable_channel(channel: u8, b: bool);
    }

    pub trait MasterTimer: General {
        type Mms;
        fn master_mode(&mut self, mode: Self::Mms);
    }
}
pub(crate) use sealed::{General, MasterTimer, WithPwm};

pub trait Instance:
    crate::Sealed + rcc::Enable + rcc::Reset + rcc::BusTimerClock + General
{
}

macro_rules! hal {
    ($($TIM:ty: [
        $Timer:ident,
        $bits:ty,
        $(dmar: $memsize:ty,)?
        $(c: ($cnum:ident $(, $aoe:ident)?),)?
        $(m: $timbase:ident,)?
    ],)+) => {
        $(
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
                fn enable_counter(&mut self) {
                    self.cr1.modify(|_, w| w.cen().set_bit());
                }
                #[inline(always)]
                fn disable_counter(&mut self) {
                    self.cr1.modify(|_, w| w.cen().clear_bit());
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
                fn clear_interrupt_flag(&mut self, event: Event) {
                    self.sr.write(|w| unsafe { w.bits(0xffff & !event.bits()) });
                }
                #[inline(always)]
                fn listen_interrupt(&mut self, event: Event, b: bool) {
                    if b {
                        self.dier.modify(|r, w| unsafe { w.bits(r.bits() | event.bits()) });
                    } else {
                        self.dier.modify(|r, w| unsafe { w.bits(r.bits() & !event.bits()) });
                    }
                }
                #[inline(always)]
                fn get_interrupt_flag(&self) -> Event {
                    Event::from_bits_truncate(self.sr.read().bits())
                }
                #[inline(always)]
                fn read_count(&self) -> Self::Width {
                    self.cnt.read().bits() as Self::Width
                }
                #[inline(always)]
                fn start_one_pulse(&mut self) {
                    self.cr1.modify(|_, w| unsafe { w.bits(1 << 3) }.cen().set_bit());
                }
                #[inline(always)]
                fn start_no_update(&mut self) {
                    self.cr1.modify(|_, w| w.cen().set_bit().udis().set_bit());
                }
                #[inline(always)]
                fn cr1_reset(&mut self) {
                    self.cr1.reset();
                }
            }

            $(with_dmar!($TIM, $memsize);)?

            $(
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
        )+
    }
}

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
    ($TIM:ty: CH1) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 1;

            #[inline(always)]
            fn read_cc_value(channel: u8) -> u32 {
                let tim = unsafe { &*<$TIM>::ptr() };
                tim.ccr[channel as usize].read().bits()
            }

            #[inline(always)]
            fn set_cc_value(channel: u8, value: u32) {
                let tim = unsafe { &*<$TIM>::ptr() };
                #[allow(unused_unsafe)]
                tim.ccr[channel as usize].write(|w| unsafe { w.bits(value) })
            }

            #[inline(always)]
            fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm) {
                match channel {
                    Channel::C1 => {
                        self.ccmr1_output()
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().bits(mode as _) );
                    }
                    _ => {},
                }
            }

            #[inline(always)]
            fn start_pwm(&mut self) {
                self.cr1.modify(|_, w| w.cen().set_bit());
            }

            #[inline(always)]
            fn enable_channel(c: u8, b: bool) {
                let tim = unsafe { &*<$TIM>::ptr() };
                if c < Self::CH_NUMBER {
                    if b {
                        unsafe { bb::set(&tim.ccer, c*4) }
                    } else {
                        unsafe { bb::clear(&tim.ccer, c*4) }
                    }
                }
            }
        }
    };
    ($TIM:ty: CH2) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 2;

            #[inline(always)]
            fn read_cc_value(channel: u8) -> u32 {
                let tim = unsafe { &*<$TIM>::ptr() };
                tim.ccr[channel as usize].read().bits()
            }

            #[inline(always)]
            fn set_cc_value(channel: u8, value: u32) {
                let tim = unsafe { &*<$TIM>::ptr() };
                #[allow(unused_unsafe)]
                tim.ccr[channel as usize].write(|w| unsafe { w.bits(value) });
            }

            #[inline(always)]
            fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm) {
                match channel {
                    Channel::C1 => {
                        self.ccmr1_output()
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().bits(mode as _) );
                    }
                    Channel::C2 => {
                        self.ccmr1_output()
                        .modify(|_, w| w.oc2pe().set_bit().oc2m().bits(mode as _) );
                    }
                    _ => {},
                }
            }

            #[inline(always)]
            fn start_pwm(&mut self) {
                self.cr1.modify(|_, w| w.cen().set_bit());
            }

            #[inline(always)]
            fn enable_channel(c: u8, b: bool) {
                let tim = unsafe { &*<$TIM>::ptr() };
                if c < Self::CH_NUMBER {
                    if b {
                        unsafe { bb::set(&tim.ccer, c*4) }
                    } else {
                        unsafe { bb::clear(&tim.ccer, c*4) }
                    }
                }
            }
        }
    };
    ($TIM:ty: CH4 $(, $aoe:ident)?) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 4;

            #[inline(always)]
            fn read_cc_value(channel: u8) -> u32 {
                let tim = unsafe { &*<$TIM>::ptr() };
                tim.ccr[channel as usize].read().bits()
            }

            #[inline(always)]
            fn set_cc_value(channel: u8, value: u32) {
                let tim = unsafe { &*<$TIM>::ptr() };
                #[allow(unused_unsafe)]
                tim.ccr[channel as usize].write(|w| unsafe { w.bits(value) })
            }

            #[inline(always)]
            fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm) {
                match channel {
                    Channel::C1 => {
                        self.ccmr1_output()
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().bits(mode as _) );
                    }
                    Channel::C2 => {
                        self.ccmr1_output()
                        .modify(|_, w| w.oc2pe().set_bit().oc2m().bits(mode as _) );
                    }
                    Channel::C3 => {
                        self.ccmr2_output()
                        .modify(|_, w| w.oc3pe().set_bit().oc3m().bits(mode as _) );
                    }
                    Channel::C4 => {
                        self.ccmr2_output()
                        .modify(|_, w| w.oc4pe().set_bit().oc4m().bits(mode as _) );
                    }
                }
            }

            #[inline(always)]
            fn start_pwm(&mut self) {
                $(let $aoe = self.bdtr.modify(|_, w| w.aoe().set_bit());)?
                self.cr1.modify(|_, w| w.cen().set_bit());
            }

            #[inline(always)]
            fn enable_channel(c: u8, b: bool) {
                let tim = unsafe { &*<$TIM>::ptr() };
                if c < Self::CH_NUMBER {
                    if b {
                        unsafe { bb::set(&tim.ccer, c*4) }
                    } else {
                        unsafe { bb::clear(&tim.ccer, c*4) }
                    }
                }
            }
        }
    }
}

impl<TIM: Instance> Timer<TIM> {
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable and reset the timer peripheral
            TIM::enable(rcc);
            TIM::reset(rcc);
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

    /// Starts listening for an `event`
    ///
    /// Note, you will also have to enable the TIM2 interrupt in the NVIC to start
    /// receiving events.
    pub fn listen(&mut self, event: Event) {
        self.tim.listen_interrupt(event, true);
    }

    /// Clears interrupt associated with `event`.
    ///
    /// If the interrupt is not cleared, it will immediately retrigger after
    /// the ISR has finished.
    pub fn clear_interrupt(&mut self, event: Event) {
        self.tim.clear_interrupt_flag(event);
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        self.tim.listen_interrupt(event, false);
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
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable and reset the timer peripheral
            TIM::enable(rcc);
            TIM::reset(rcc);
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

    /// Creates `Counter` that imlements [embedded_hal::timer::CountDown]
    pub fn counter(self) -> Counter<TIM, FREQ> {
        Counter(self)
    }

    /// Creates `Delay` that imlements [embedded_hal::blocking::delay] traits
    pub fn delay(self) -> Delay<TIM, FREQ> {
        Delay(self)
    }

    /// Releases the TIM peripheral
    pub fn release(self) -> TIM {
        self.tim
    }

    /// Starts listening for an `event`
    ///
    /// Note, you will also have to enable the TIM2 interrupt in the NVIC to start
    /// receiving events.
    pub fn listen(&mut self, event: Event) {
        self.tim.listen_interrupt(event, true);
    }

    /// Clears interrupt associated with `event`.
    ///
    /// If the interrupt is not cleared, it will immediately retrigger after
    /// the ISR has finished.
    pub fn clear_interrupt(&mut self, event: Event) {
        self.tim.clear_interrupt_flag(event);
    }

    pub fn get_interrupt(&mut self) -> Event {
        self.tim.get_interrupt_flag()
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        self.tim.listen_interrupt(event, false);
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

// All F4xx parts have these timers.
hal!(
    pac::TIM9: [Timer9, u16, c: (CH2),],
    pac::TIM11: [Timer11, u16, c: (CH1),],
);

// All parts except for F410 add these timers.
#[cfg(not(feature = "stm32f410"))]
hal!(
    pac::TIM1: [Timer1, u16, dmar: u32, c: (CH4, _aoe), m: tim1,],
    pac::TIM5: [Timer5, u32, dmar: u16, c: (CH4), m: tim5,],
    pac::TIM2: [Timer2, u32, dmar: u16, c: (CH4), m: tim2,],
    pac::TIM3: [Timer3, u16, dmar: u16, c: (CH4), m: tim3,],
    pac::TIM4: [Timer4, u16, dmar: u16, c: (CH4), m: tim3,],
    pac::TIM10: [Timer10, u16, c: (CH1),],
);

// TIM5 on F410 is 16-bit
#[cfg(feature = "stm32f410")]
hal!(
    pac::TIM1: [Timer1, u16, dmar: u16, c: (CH4, _aoe), m: tim1,],
    pac::TIM5: [Timer5, u16, dmar: u16, c: (CH4), m: tim5,],
);

// All parts except F401 and F411.
#[cfg(not(any(feature = "stm32f401", feature = "stm32f411",)))]
hal!(pac::TIM6: [Timer6, u16, m: tim6,],);

// All parts except F401, F410, F411.
#[cfg(not(any(feature = "stm32f401", feature = "stm32f410", feature = "stm32f411",)))]
hal!(
    pac::TIM7: [Timer7, u16, m: tim7,],
    pac::TIM8: [Timer8, u16, dmar: u32, c: (CH4, _aoe), m: tim8,],
    pac::TIM12: [Timer12, u16, c: (CH2),],
    pac::TIM13: [Timer13, u16, c: (CH1),],
    pac::TIM14: [Timer14, u16, c: (CH1),],
);
