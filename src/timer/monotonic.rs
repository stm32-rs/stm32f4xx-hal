// RTIC Monotonic impl for the 32-bit timers
use super::{Channel, Event, FTimer, Flag, General, Instance, WithPwm};
use crate::rcc::Clocks;
use crate::ReadFlags;
use core::ops::{Deref, DerefMut};
pub use fugit::{self, ExtU32};
use systick_monotonic::Systick;

pub struct MonoTimer<TIM, const FREQ: u32>(FTimer<TIM, FREQ>);

impl<TIM, const FREQ: u32> Deref for MonoTimer<TIM, FREQ> {
    type Target = FTimer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TIM, const FREQ: u32> DerefMut for MonoTimer<TIM, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `MonoTimer` with precision of 1 μs (1 MHz sampling)
pub type MonoTimerUs<TIM> = MonoTimer<TIM, 1_000_000>;
pub type MonoTimer64Us<TIM> = MonoTimer64<TIM, 1_000_000>;

impl<TIM: Instance, const FREQ: u32> MonoTimer<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.0
    }
}

pub trait MonoTimerExt: Sized {
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer<Self, FREQ>;
    fn monotonic_us(self, clocks: &Clocks) -> MonoTimer<Self, 1_000_000> {
        self.monotonic::<1_000_000>(clocks)
    }
}

impl<TIM> MonoTimerExt for TIM
where
    Self: Instance + General<Width = u32> + WithPwm,
{
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer<Self, FREQ> {
        FTimer::new(self, clocks).monotonic()
    }
}

pub trait MonoTimer64Ext: Sized {
    fn monotonic64<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer64<Self, FREQ>;
    fn monotonic64_us(self, clocks: &Clocks) -> MonoTimer64<Self, 1_000_000> {
        self.monotonic64::<1_000_000>(clocks)
    }
}

impl<TIM> MonoTimer64Ext for TIM
where
    Self: Instance + General + WithPwm,
{
    fn monotonic64<const FREQ: u32>(self, clocks: &Clocks) -> MonoTimer64<Self, FREQ> {
        FTimer::new(self, clocks).monotonic64()
    }
}

pub trait SysMonoTimerExt: Sized {
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> Systick<FREQ>;
    fn monotonic_us(self, clocks: &Clocks) -> Systick<1_000_000> {
        self.monotonic::<1_000_000>(clocks)
    }
}

impl SysMonoTimerExt for crate::pac::SYST {
    fn monotonic<const FREQ: u32>(self, clocks: &Clocks) -> Systick<FREQ> {
        Systick::new(self, clocks.hclk().raw())
    }
}

impl<TIM, const FREQ: u32> FTimer<TIM, FREQ>
where
    TIM: Instance + General<Width = u32> + WithPwm,
{
    pub fn monotonic(mut self) -> MonoTimer<TIM, FREQ> {
        unsafe {
            self.tim.set_auto_reload_unchecked(TIM::max_auto_reload());
        }
        self.tim.trigger_update();
        self.tim.start_free(false);
        MonoTimer(self)
    }
}

impl<TIM, const FREQ: u32> FTimer<TIM, FREQ>
where
    TIM: Instance + General + WithPwm,
{
    pub fn monotonic64(mut self) -> MonoTimer64<TIM, FREQ> {
        unsafe {
            self.tim.set_auto_reload_unchecked(TIM::max_auto_reload());
        }
        self.tim.trigger_update();
        self.tim.start_free(true);
        MonoTimer64 {
            ftimer: self,
            ovf: 0,
        }
    }
}

impl<TIM, const FREQ: u32> rtic_monotonic::Monotonic for MonoTimer<TIM, FREQ>
where
    TIM: Instance + General<Width = u32> + WithPwm,
{
    type Instant = fugit::TimerInstantU32<FREQ>;
    type Duration = fugit::TimerDurationU32<FREQ>;

    unsafe fn reset(&mut self) {
        self.tim.listen_event(None, Some(Event::C1.into()));
        self.tim.reset_counter();
    }

    #[inline(always)]
    fn now(&mut self) -> Self::Instant {
        Self::Instant::from_ticks(self.tim.read_count())
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        TIM::set_cc_value(Channel::C1 as u8, instant.duration_since_epoch().ticks());
    }

    fn clear_compare_flag(&mut self) {
        self.tim.clear_interrupt_flag(Flag::C1.into());
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }
}

/// Extended TIM15/16 (16-bits) to 64 bits
pub struct MonoTimer64<TIM, const FREQ: u32> {
    ftimer: FTimer<TIM, FREQ>,
    ovf: u64,
}

impl<TIM, const FREQ: u32> Deref for MonoTimer64<TIM, FREQ> {
    type Target = FTimer<TIM, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.ftimer
    }
}

impl<TIM, const FREQ: u32> DerefMut for MonoTimer64<TIM, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ftimer
    }
}

impl<TIM: Instance, const FREQ: u32> MonoTimer64<TIM, FREQ> {
    fn is_overflow(&self) -> bool {
        self.flags().contains(Flag::Update)
    }
}

/// Use Compare channel 1 for Monotonic
impl<TIM, const FREQ: u32> rtic_monotonic::Monotonic for MonoTimer64<TIM, FREQ>
where
    TIM: Instance + General + WithPwm,
    TIM::Width: ArrWidth,
    u64: From<TIM::Width>,
    u32: From<TIM::Width>,
{
    // Since we are counting overflows we can't let RTIC disable the interrupt.
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    type Instant = fugit::TimerInstantU64<FREQ>;
    type Duration = fugit::TimerDurationU64<FREQ>;

    fn now(&mut self) -> Self::Instant {
        let cnt = self.tim.read_count();

        // If the overflow bit is set, we add this to the timer value. It means the `on_interrupt`
        // has not yet happened, and we need to compensate here.
        let ovf = if self.is_overflow() {
            TIM::Width::OVF_VALUE
        } else {
            0
        };
        Self::Instant::from_ticks(u64::from(cnt) + ovf + self.ovf)
    }

    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }

    unsafe fn reset(&mut self) {
        // Since reset is only called once, we use it to enable the interrupt generation bit.
        self.tim.listen_event(None, Some(Event::C1.into()));
        self.tim.reset_counter();
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        let now = self.now();

        // Since the timer may or may not overflow based on the requested compare val, we check
        // how many ticks are left.
        let val: TIM::Width = match instant.checked_duration_since(now) {
            None => TIM::Width::ONE, // In the past, RTIC will handle this
            Some(x) if x.ticks() < TIM::Width::OVF_VALUE => {
                TIM::Width::cast_u64(instant.duration_since_epoch().ticks())
            } // Will not overflow
            Some(_x) => self.tim.read_count().wrapping_add(TIM::Width::FOR_WRAP), // Will overflow
        };

        TIM::set_cc_value(Channel::C1 as u8, u32::from(val));
    }

    fn clear_compare_flag(&mut self) {
        self.tim.clear_interrupt_flag(Flag::C1.into());
    }

    fn on_interrupt(&mut self) {
        // If there was an overflow, increment the overflow counter.
        if self.is_overflow() {
            self.tim.clear_interrupt_flag(Flag::Update.into());

            self.ovf += TIM::Width::OVF_VALUE;
        }
    }
}

trait ArrWidth {
    const OVF_VALUE: u64;
    const FOR_WRAP: Self;
    const ONE: Self;
    fn cast_u64(from: u64) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
}

impl ArrWidth for u16 {
    const OVF_VALUE: u64 = 0x1_0000;
    const FOR_WRAP: Self = 0xfffe;
    const ONE: Self = 1;
    fn cast_u64(from: u64) -> Self {
        from as _
    }
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl ArrWidth for u32 {
    const OVF_VALUE: u64 = 0x1_0000_0000;
    const FOR_WRAP: Self = 0xffff_fffe;
    const ONE: Self = 1;
    fn cast_u64(from: u64) -> Self {
        from as _
    }
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}
