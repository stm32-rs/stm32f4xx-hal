// RTIC Monotonic impl for the 32-bit timers
use super::{Channel, Event, FTimer, General, Instance, WithPwm};
use crate::pac;
use crate::rcc::Clocks;
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
    fn is_overflow() -> bool {
        TIM::get_irq().contains(Event::Update)
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

mod v1 {
    use super::*;

    impl<TIM, const FREQ: u32> rtic_monotonic::Monotonic for MonoTimer<TIM, FREQ>
    where
        TIM: Instance + General<Width = u32> + WithPwm,
    {
        type Instant = fugit::TimerInstantU32<FREQ>;
        type Duration = fugit::TimerDurationU32<FREQ>;

        unsafe fn reset(&mut self) {
            self.tim.listen_interrupt(Event::C1, true);
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
            self.tim.clear_interrupt_flag(Event::C1);
        }

        #[inline(always)]
        fn zero() -> Self::Instant {
            Self::Instant::from_ticks(0)
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
            let ovf = if Self::is_overflow() {
                TIM::Width::OVF_VALUE
            } else {
                0
            };
            let i = Self::Instant::from_ticks(u64::from(cnt) + ovf + self.ovf);
            i
        }

        fn zero() -> Self::Instant {
            Self::Instant::from_ticks(0)
        }

        unsafe fn reset(&mut self) {
            // Since reset is only called once, we use it to enable the interrupt generation bit.
            self.tim.listen_interrupt(Event::C1, true);
            self.tim.reset_counter();
        }

        fn set_compare(&mut self, instant: Self::Instant) {
            let now = self.now();

            // Since the timer may or may not overflow based on the requested compare val, we check
            // how many ticks are left.
            let val: TIM::Width = match instant.checked_duration_since(now) {
                None => TIM::Width::ONE, // In the past, RTIC will handle this
                Some(x) if x.ticks() <= TIM::Width::OVF_VALUE - 1 => {
                    TIM::Width::cast_u64(instant.duration_since_epoch().ticks())
                } // Will not overflow
                Some(_x) => self.tim.read_count().wrapping_add(TIM::Width::FOR_WRAP), // Will overflow
            };

            TIM::set_cc_value(Channel::C1 as u8, u32::from(val));
        }

        fn clear_compare_flag(&mut self) {
            self.tim.clear_interrupt_flag(Event::C1);
        }

        fn on_interrupt(&mut self) {
            // If there was an overflow, increment the overflow counter.
            if Self::is_overflow() {
                self.tim.clear_interrupt_flag(Event::Update);

                self.ovf += TIM::Width::OVF_VALUE;
            }
        }
    }
}

pub trait Irq {
    const IRQ: pac::Interrupt;
}
#[cfg(feature = "tim2")]
impl Irq for pac::TIM2 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM2;
}
#[cfg(feature = "tim5")]
impl Irq for pac::TIM5 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM5;
}

#[cfg(feature = "async")]
mod v2 {
    use super::*;

    const fn cortex_logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
        ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
    }

    pub(crate) unsafe fn set_monotonic_prio(
        nvic: &mut cortex_m::peripheral::NVIC,
        prio_bits: u8,
        interrupt: impl cortex_m::interrupt::InterruptNumber,
    ) {
        extern "C" {
            static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8;
        }

        let max_prio = RTIC_ASYNC_MAX_LOGICAL_PRIO.max(1).min(1 << prio_bits);

        let hw_prio = cortex_logical2hw(max_prio, prio_bits);

        nvic.set_priority(interrupt, hw_prio);
    }

    macro_rules! queue {
        ($TIM:ty, $QUEUE:ident, $FREQ:literal) => {
            static $QUEUE: rtic_time::TimerQueue<$crate::timer::MonoTimer<$TIM, $FREQ>> =
                rtic_time::TimerQueue::new();

            // Forward timerqueue interface
            impl $crate::timer::MonoTimer<$TIM, $FREQ> {
                /// Timeout at a specific time.
                pub async fn timeout_at<F: core::future::Future>(
                    instant: <Self as rtic_time::Monotonic>::Instant,
                    future: F,
                ) -> Result<F::Output, rtic_time::TimeoutError> {
                    $QUEUE.timeout_at(instant, future).await
                }

                /// Timeout after a specific duration.
                #[inline]
                pub async fn timeout_after<F: core::future::Future>(
                    duration: <Self as rtic_time::Monotonic>::Duration,
                    future: F,
                ) -> Result<F::Output, rtic_time::TimeoutError> {
                    $QUEUE.timeout_after(duration, future).await
                }

                /// Delay for some duration of time.
                #[inline]
                pub async fn delay(duration: <Self as rtic_time::Monotonic>::Duration) {
                    $QUEUE.delay(duration).await;
                }

                /// Delay to some specific time instant.
                #[inline]
                pub async fn delay_until(instant: <Self as rtic_time::Monotonic>::Instant) {
                    $QUEUE.delay_until(instant).await;
                }

                pub fn init(mut self, nvic: &mut cortex_m::peripheral::NVIC) {
                    self.tim.listen_interrupt(Event::C1, true);
                    self.tim.reset_counter();

                    $QUEUE.initialize(self);

                    // SAFETY: We take full ownership of the peripheral and interrupt vector,
                    // plus we are not using any external shared resources so we won't impact
                    // basepri/source masking based critical sections.
                    unsafe {
                        set_monotonic_prio(
                            nvic,
                            pac::NVIC_PRIO_BITS,
                            <$TIM as $crate::timer::monotonic::Irq>::IRQ,
                        );
                        cortex_m::peripheral::NVIC::unmask(
                            <$TIM as crate::timer::monotonic::Irq>::IRQ,
                        );
                    }
                }

                /// Call this from the TIMx interrupt handler
                pub unsafe fn interrupt_handler() {
                    $QUEUE.on_monotonic_interrupt();
                }
            }
        };
    }
    queue!(pac::TIM2, MONO_TIMER2_QUEUE, 1_000_000);
    queue!(pac::TIM5, MONO_TIMER5_QUEUE, 1_000_000);

    impl<TIM, const FREQ: u32> rtic_time::Monotonic for MonoTimer<TIM, FREQ>
    where
        TIM: Instance + General<Width = u32> + WithPwm + Irq,
    {
        type Instant = fugit::TimerInstantU32<FREQ>;
        type Duration = fugit::TimerDurationU32<FREQ>;

        const ZERO: Self::Instant = Self::Instant::from_ticks(0);

        #[inline(always)]
        fn now() -> Self::Instant {
            Self::Instant::from_ticks(TIM::read_cnt())
        }

        fn set_compare(instant: Self::Instant) {
            TIM::set_cc_value(Channel::C1 as u8, instant.duration_since_epoch().ticks());
        }

        fn clear_compare_flag() {
            TIM::clear_irq(Event::C1);
        }

        fn pend_interrupt() {
            cortex_m::peripheral::NVIC::pend(TIM::IRQ);
        }
    }
}
