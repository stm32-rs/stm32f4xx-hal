//! Timers
//!
//! Pins can be used for PWM output in both push-pull mode (`Alternate`) and open-drain mode
//! (`AlternateOD`).
#![allow(non_upper_case_globals)]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use void::Void;

use crate::bb;
use crate::pac::{self, RCC};

use crate::rcc::{self, Clocks};
use fugit::HertzU32 as Hertz;

mod pins;
pub use pins::*;

/// Timer wrapper
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) clk: Hertz,
}

/// Hardware timers
pub struct CountDownTimer<TIM> {
    tim: TIM,
    clk: Hertz,
}

impl<TIM> Timer<TIM> {
    /// Creates CountDownTimer
    pub fn count_down(self) -> CountDownTimer<TIM> {
        let Self { tim, clk } = self;
        CountDownTimer { tim, clk }
    }
}

impl<TIM> Timer<TIM>
where
    CountDownTimer<TIM>: CountDown<Time = Hertz>,
{
    /// Starts timer in count down mode at a given frequency
    pub fn start_count_down(self, timeout: Hertz) -> CountDownTimer<TIM> {
        let mut timer = self.count_down();
        timer.start(timeout);
        timer
    }
}

impl<TIM> Periodic for CountDownTimer<TIM> {}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Channel {
    C1 = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
}

/// Interrupt events
#[derive(Clone, Copy, PartialEq)]
pub enum SysEvent {
    /// CountDownTimer timed out / count down ended
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
pub enum Error {
    /// CountDownTimer is disabled
    Disabled,
    WrongAutoReload,
}

impl Timer<SYST> {
    /// Initialize timer
    pub fn syst(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        Self {
            tim: syst,
            clk: clocks.sysclk(),
        }
    }

    pub fn release(self) -> SYST {
        self.tim
    }
}

impl CountDownTimer<SYST> {
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

impl CountDown for CountDownTimer<SYST> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let rvr = self.clk.raw() / timeout.into().raw() - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Cancel for CountDownTimer<SYST> {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Self::Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

mod sealed {
    use super::{Channel, Event, Ocm};
    pub trait General {
        type Width: Into<u32> + From<u16>;
        fn max_auto_reload() -> u32;
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

impl<TIM> Timer<TIM>
where
    TIM: Instance,
{
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
}

macro_rules! hal {
    ($($TIM:ty: [
        $Timer:ident,
        $bits:ty,
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
                fn set_auto_reload(&mut self, arr: u32) -> Result<(), Error> {
                    // Note: Make it impossible to set the ARR value to 0, since this
                    // would cause an infinite loop.
                    if arr > 0 && arr <= Self::max_auto_reload() {
                        Ok(self.arr.write(|w| unsafe { w.bits(arr) }))
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
                    self.cr1.write(|w| unsafe { w.bits(1 << 3) }.cen().set_bit());
                }
                #[inline(always)]
                fn cr1_reset(&mut self) {
                    self.cr1.reset();
                }
            }
            $(with_pwm!($TIM: $cnum $(, $aoe)?);)?

            $(impl MasterTimer for $TIM {
                type Mms = pac::$timbase::cr2::MMS_A;
                fn master_mode(&mut self, mode: Self::Mms) {
                    self.cr2.modify(|_,w| w.mms().variant(mode));
                }
            })?
        )+
    }
}

macro_rules! with_pwm {
    ($TIM:ty: CH1) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 1;

            #[inline(always)]
            fn read_cc_value(channel: u8) -> u32 {
                let tim = unsafe { &*<$TIM>::ptr() };
                match channel {
                    0 => {
                        tim.ccr1.read().bits()
                    }
                    _ => 0,
                }
            }

            #[inline(always)]
            fn set_cc_value(channel: u8, value: u32) {
                let tim = unsafe { &*<$TIM>::ptr() };
                #[allow(unused_unsafe)]
                match channel {
                    0 => {
                        tim.ccr1.write(|w| unsafe { w.bits(value) })
                    }
                    _ => {},
                }
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
                self.cr1.write(|w| w.cen().set_bit());
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
                match channel {
                    0 => {
                        tim.ccr1.read().bits()
                    }
                    1 => {
                        tim.ccr2.read().bits()
                    }
                    _ => 0,
                }
            }

            #[inline(always)]
            fn set_cc_value(channel: u8, value: u32) {
                let tim = unsafe { &*<$TIM>::ptr() };
                #[allow(unused_unsafe)]
                match channel {
                    0 => {
                        tim.ccr1.write(|w| unsafe { w.bits(value) })
                    }
                    1 => {
                        tim.ccr2.write(|w| unsafe { w.bits(value) })
                    }
                    _ => {},
                }
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
                self.cr1.write(|w| w.cen().set_bit());
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
                let ccr = match channel {
                    0 => {
                        &tim.ccr1
                    }
                    1 => {
                        &tim.ccr2
                    }
                    2 => {
                        &tim.ccr3
                    }
                    _ => {
                        &tim.ccr4
                    }
                };
                ccr.read().bits()
            }

            #[inline(always)]
            fn set_cc_value(channel: u8, value: u32) {
                let tim = unsafe { &*<$TIM>::ptr() };
                let ccr = match channel {
                    0 => {
                        &tim.ccr1
                    }
                    1 => {
                        &tim.ccr2
                    }
                    2 => {
                        &tim.ccr3
                    }
                    _ => {
                        &tim.ccr4
                    }
                };
                ccr.write(|w| unsafe { w.bits(value) })
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
                self.cr1.write(|w| w.cen().set_bit());
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

impl<TIM: General> CountDownTimer<TIM> {
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

    /// Releases the TIM peripheral
    pub fn release(mut self) -> TIM {
        // pause counter
        self.tim.disable_counter();
        self.tim
    }
}

impl<TIM: General + MasterTimer> CountDownTimer<TIM> {
    pub fn set_master_mode(&mut self, mode: TIM::Mms) {
        self.tim.master_mode(mode)
    }
}

#[inline(always)]
pub(crate) const fn compute_arr_presc(freq: u32, clock: u32) -> (u16, u32) {
    let ticks = clock / freq;
    let psc = (ticks - 1) / (1 << 16);
    assert!(psc <= u16::MAX as u32);
    let arr = ticks / (psc + 1) - 1;
    (psc as u16, arr)
}

impl<TIM> CountDown for CountDownTimer<TIM>
where
    TIM: General,
{
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        // pause
        self.tim.disable_counter();
        // reset counter
        self.tim.reset_counter();

        let (psc, arr) = compute_arr_presc(timeout.into().raw(), self.clk.raw());
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr).unwrap();

        // Trigger update event to load the registers
        self.tim.trigger_update();

        // start counter
        self.tim.enable_counter();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.tim.get_interrupt_flag().contains(Event::Update) {
            self.tim.clear_interrupt_flag(Event::Update);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<TIM> Cancel for CountDownTimer<TIM>
where
    TIM: General,
{
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Self::Error::Disabled);
        }

        // disable counter
        self.tim.disable_counter();
        Ok(())
    }
}

// All F4xx parts have these timers.
hal!(
    pac::TIM9: [Timer9, u16, c: (CH2),],
    pac::TIM11: [Timer11, u16, c: (CH1),],
);

// All parts except for F410 add these timers.
#[cfg(not(feature = "stm32f410"))]
hal!(
    pac::TIM1: [Timer1, u16, c: (CH4, _aoe), m: tim1,],
    pac::TIM5: [Timer5, u32, c: (CH4), m: tim5,],
    pac::TIM2: [Timer2, u32, c: (CH4), m: tim2,],
    pac::TIM3: [Timer3, u16, c: (CH4), m: tim3,],
    pac::TIM4: [Timer4, u16, c: (CH4), m: tim3,],
    pac::TIM10: [Timer10, u16, c: (CH1),],
);

// TIM5 on F410 is 16-bit
#[cfg(feature = "stm32f410")]
hal!(
    pac::TIM1: [Timer1, u16, c: (CH4, _aoe), /*m: tim1,*/], // TODO: fix SVD
    pac::TIM5: [Timer5, u16, c: (CH4), /*m: tim5,*/], // TODO: fix SVD
);

// All parts except F401 and F411.
#[cfg(not(any(feature = "stm32f401", feature = "stm32f411",)))]
hal!(pac::TIM6: [Timer6, u16, /*m: tim7,*/],); // TODO: fix SVD

// All parts except F401, F410, F411.
#[cfg(not(any(feature = "stm32f401", feature = "stm32f410", feature = "stm32f411",)))]
hal!(
    pac::TIM7: [Timer7, u16, /*m: tim7,*/], // TODO: fix SVD
    pac::TIM8: [Timer8, u16, c: (CH4, _aoe), m: tim1,],
    pac::TIM12: [Timer12, u16, c: (CH2),],
    pac::TIM13: [Timer13, u16, c: (CH1),],
    pac::TIM14: [Timer14, u16, c: (CH1),],
);
