//! Timers
//!
//! Pins can be used for PWM output in both push-pull mode (`Alternate`) and open-drain mode
//! (`AlternateOD`).
#![allow(non_upper_case_globals)]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::{DCB, DWT, SYST};
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use void::Void;

use crate::bb;
use crate::pac::RCC;
use crate::pwm::{pwm_pin, PwmChannel};

use crate::rcc::{self, Clocks};
use crate::time::Hertz;

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
    pub fn start_count_down<T>(self, timeout: T) -> CountDownTimer<TIM>
    where
        T: Into<Hertz>,
    {
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
        let rvr = self.clk.0 / timeout.into().0 - 1;

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

/// A monotonic non-decreasing timer
///
/// This uses the timer in the debug watch trace peripheral. This means, that if the
/// core is stopped, the timer does not count up. This may be relevant if you are using
/// cortex_m_semihosting::hprintln for debugging in which case the timer will be stopped
/// while printing
#[derive(Clone, Copy)]
pub struct MonoTimer {
    frequency: Hertz,
}

impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(mut dwt: DWT, mut dcb: DCB, clocks: &Clocks) -> Self {
        dcb.enable_trace();
        dwt.enable_cycle_counter();

        // now the CYCCNT counter can't be stopped or reset
        drop(dwt);

        MonoTimer {
            frequency: clocks.hclk(),
        }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(self) -> Hertz {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(self) -> Instant {
        Instant {
            now: DWT::cycle_count(),
        }
    }
}

/// A measurement of a monotonically non-decreasing clock
#[derive(Clone, Copy)]
pub struct Instant {
    now: u32,
}

impl Instant {
    /// Ticks elapsed since the `Instant` was created
    pub fn elapsed(self) -> u32 {
        DWT::cycle_count().wrapping_sub(self.now)
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
        fn read_auto_reload(&self) -> Self::Width;
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
        fn read_cc_value(&self, channel: Channel) -> Self::Width;
        fn set_cc_value(&mut self, channel: Channel, value: Self::Width);
        fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm);
        fn start_pwm(&mut self);
        fn enable_channel(&mut self, channel: Channel, b: bool);
    }
}
pub(crate) use sealed::{General, WithPwm};

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
    ($($TIM:ty: [$bits:ty, $($cnum:ident $(, $aoe:ident)?)?],)+) => {
        $(
            impl Instance for $TIM { }

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
                fn read_auto_reload(&self) -> Self::Width {
                    self.arr.read().bits() as Self::Width
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
        )+
    }
}

macro_rules! with_pwm {
    ($TIM:ty: CH1) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 1;

            #[inline(always)]
            fn read_cc_value(&self, channel: Channel) -> Self::Width {
                match channel {
                    Channel::C1 => {
                        self.ccr1.read().ccr().bits()
                    }
                    _ => 0,
                }
            }

            #[inline(always)]
            fn set_cc_value(&mut self, channel: Channel, value: Self::Width) {
                #[allow(unused_unsafe)]
                match channel {
                    Channel::C1 => {
                        self.ccr1.write(|w| unsafe { w.ccr().bits(value) })
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
            fn enable_channel(&mut self, channel: Channel, b: bool) {
                let c = channel as u8;
                if c < Self::CH_NUMBER {
                    if b {
                        unsafe { bb::set(&self.ccer, c*4) }
                    } else {
                        unsafe { bb::clear(&self.ccer, c*4) }
                    }
                }
            }
        }

        pwm_pin!($TIM, C1, ccr1, 0);
    };
    ($TIM:ty: CH2) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 2;

            #[inline(always)]
            fn read_cc_value(&self, channel: Channel) -> Self::Width {
                match channel {
                    Channel::C1 => {
                        self.ccr1.read().ccr().bits()
                    }
                    Channel::C2 => {
                        self.ccr2.read().ccr().bits()
                    }
                    _ => 0,
                }
            }

            #[inline(always)]
            fn set_cc_value(&mut self, channel: Channel, value: Self::Width) {
                #[allow(unused_unsafe)]
                match channel {
                    Channel::C1 => {
                        self.ccr1.write(|w| unsafe { w.ccr().bits(value) })
                    }
                    Channel::C2 => {
                        self.ccr2.write(|w| unsafe { w.ccr().bits(value) })
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
            fn enable_channel(&mut self, channel: Channel, b: bool) {
                let c = channel as u8;
                if c < Self::CH_NUMBER {
                    if b {
                        unsafe { bb::set(&self.ccer, c*4) }
                    } else {
                        unsafe { bb::clear(&self.ccer, c*4) }
                    }
                }
            }
        }

        pwm_pin!($TIM, C1, ccr1, 0);
        pwm_pin!($TIM, C2, ccr2, 4);
    };
    ($TIM:ty: CH4 $(, $aoe:ident)?) => {
        impl WithPwm for $TIM {
            const CH_NUMBER: u8 = 4;

            #[inline(always)]
            fn read_cc_value(&self, channel: Channel) -> Self::Width {
                let ccr = match channel {
                    Channel::C1 => {
                        &self.ccr1
                    }
                    Channel::C2 => {
                        &self.ccr2
                    }
                    Channel::C3 => {
                        &self.ccr3
                    }
                    Channel::C4 => {
                        &self.ccr4
                    }
                };
                ccr.read().bits() as Self::Width
            }

            #[inline(always)]
            fn set_cc_value(&mut self, channel: Channel, value: Self::Width) {
                let ccr = match channel {
                    Channel::C1 => {
                        &self.ccr1
                    }
                    Channel::C2 => {
                        &self.ccr2
                    }
                    Channel::C3 => {
                        &self.ccr3
                    }
                    Channel::C4 => {
                        &self.ccr4
                    }
                };
                ccr.write(|w| unsafe { w.bits(value as u32) })
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
            fn enable_channel(&mut self, channel: Channel, b: bool) {
                let c = channel as u8;
                if c < Self::CH_NUMBER {
                    if b {
                        unsafe { bb::set(&self.ccer, c*4) }
                    } else {
                        unsafe { bb::clear(&self.ccer, c*4) }
                    }
                }
            }
        }

        pwm_pin!($TIM, C1, ccr1, 0);
        pwm_pin!($TIM, C2, ccr2, 4);
        pwm_pin!($TIM, C3, ccr3, 8);
        pwm_pin!($TIM, C4, ccr4, 12);
    }
}

impl<TIM> CountDownTimer<TIM>
where
    TIM: General,
{
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

#[inline(always)]
pub(crate) const fn compute_arr_presc(freq: u32, clock: u32) -> (u16, u32) {
    let ticks = clock / freq;
    let psc_u32 = (ticks - 1) / (1 << 16);
    let psc = if psc_u32 > u16::MAX as u32 {
        panic!();
    } else {
        psc_u32 as u16
    };
    let arr = ticks / (psc_u32 + 1) - 1;
    (psc, arr)
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

        let (psc, arr) = compute_arr_presc(timeout.into().0, self.clk.0);
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
    crate::pac::TIM1: [u16, CH4, _aoe],
    crate::pac::TIM9: [u16, CH2],
    crate::pac::TIM11: [u16, CH1],
);

// All parts except for F410 add these timers.
#[cfg(not(feature = "stm32f410"))]
hal!(
    crate::pac::TIM5: [u32, CH4],
    crate::pac::TIM2: [u32, CH4],
    crate::pac::TIM3: [u16, CH4],
    crate::pac::TIM4: [u16, CH4],
    crate::pac::TIM10: [u16, CH1],
);

// TIM5 on F410 is 16-bit
#[cfg(feature = "stm32f410")]
hal!(crate::pac::TIM5: [u16, CH4],);

// All parts except F401 and F411.
#[cfg(not(any(feature = "stm32f401", feature = "stm32f411",)))]
hal!(crate::pac::TIM6: [u16,],);

// All parts except F401, F410, F411.
#[cfg(not(any(feature = "stm32f401", feature = "stm32f410", feature = "stm32f411",)))]
hal!(
    crate::pac::TIM7: [u16,],
    crate::pac::TIM8: [u16, CH4, _aoe],
    crate::pac::TIM12: [u16, CH2],
    crate::pac::TIM13: [u16, CH1],
    crate::pac::TIM14: [u16, CH1],
);

use crate::gpio::{self, Alternate};

// Output channels markers
pub trait CPin<C, TIM> {}
pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

macro_rules! channel_impl {
    ( $( $TIM:ident, $C:ident, $PINX:ident, $AF:literal; )+ ) => {
        $(
            impl<Otype> CPin<$C, crate::pac::$TIM> for gpio::$PINX<Alternate<Otype, $AF>> { }
        )+
    };
}

// The approach to PWM channel implementation is to group parts with
// common pins, starting with groupings of the largest number of parts
// and moving to smaller and smaller groupings.  Last, we have individual
// parts to cover exceptions.

// All parts have these PWM pins.
channel_impl!(
    TIM1, C1, PA8, 1;
    TIM1, C2, PA9, 1;
    TIM1, C3, PA10, 1;
    TIM1, C4, PA11, 1;

    TIM5, C1, PA0, 2;
    TIM5, C2, PA1, 2;
    TIM5, C3, PA2, 2;
    TIM5, C4, PA3, 2;

    TIM9, C1, PA2, 3;
    TIM9, C2, PA3, 3;

    TIM11, C1, PB9, 3;
);

// All parts except F410.
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM1, C1, PE9, 1;
    TIM1, C2, PE11, 1;
    TIM1, C3, PE13, 1;
    TIM1, C4, PE14, 1;

    TIM2, C1, PA0, 1;
    TIM2, C2, PA1, 1;
    TIM2, C3, PA2, 1;
    TIM2, C4, PA3, 1;

    TIM2, C2, PB3, 1;
    TIM2, C3, PB10, 1;
    TIM2, C4, PB11, 1;

    TIM2, C1, PA5, 1;
    TIM2, C1, PA15, 1;

    TIM3, C1, PA6, 2;
    TIM3, C2, PA7, 2;
    TIM3, C3, PB0, 2;
    TIM3, C4, PB1, 2;

    TIM3, C1, PB4, 2;
    TIM3, C2, PB5, 2;

    TIM3, C1, PC6, 2;
    TIM3, C2, PC7, 2;
    TIM3, C3, PC8, 2;
    TIM3, C4, PC9, 2;

    TIM4, C1, PB6, 2;
    TIM4, C2, PB7, 2;
    TIM4, C3, PB8, 2;
    TIM4, C4, PB9, 2;

    TIM4, C1, PD12, 2;
    TIM4, C2, PD13, 2;
    TIM4, C3, PD14, 2;
    TIM4, C4, PD15, 2;

    TIM10, C1, PB8, 3;
);

// All parts except F401 and F410.
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM9, C1, PE5, 3;
    TIM9, C2, PE6, 3;
);

// All parts except F401, F410, and F411.
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM8, C1, PC6, 3;
    TIM8, C2, PC7, 3;
    TIM8, C3, PC8, 3;
    TIM8, C4, PC9, 3;

    TIM10, C1, PF6, 3;

    TIM11, C1, PF7, 3;

    TIM12, C1, PB14, 9;
    TIM12, C2, PB15, 9;

    TIM13, C1, PA6, 9;
    TIM13, C1, PF8, 9;  // Not a mistake: TIM13 has only one channel.

    TIM14, C1, PA7, 9;
    TIM14, C1, PF9, 9;  // Not a mistake: TIM14 has only one channel.
);

// STM's "advanced and foundation" lines except F446.
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM5, C1, PH10, 2;
    TIM5, C2, PH11, 2;
    TIM5, C3, PH12, 2;
    TIM5, C4, PI0, 2;

    TIM8, C1, PI5, 3;
    TIM8, C2, PI6, 3;
    TIM8, C3, PI7, 3;
    TIM8, C4, PI2, 3;

    TIM12, C1, PH6, 9;
    TIM12, C2, PH9, 9;
);

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
channel_impl!(
    TIM5, C1, PF3, 2;
    TIM5, C2, PF4, 2;
    TIM5, C3, PF5, 2;
    TIM5, C4, PF10, 2;
);

#[cfg(feature = "stm32f410")]
channel_impl!(
    TIM5, C1, PB12, 2;
    TIM5, C2, PC10, 2;
    TIM5, C3, PC11, 2;
    TIM5, C4, PB11, 2;

    TIM9, C1, PC4, 3;
    TIM9, C2, PC5, 3;

    TIM11, C1, PC13, 3;
);

#[cfg(feature = "stm32f446")]
channel_impl!(
    TIM2, C1, PB8, 1;
    TIM2, C2, PB9, 1;

    TIM2, C4, PB2, 1;
);
