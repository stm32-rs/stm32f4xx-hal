//! Delays

use super::{Error, FTimer, General, Instance, Timer};
use crate::pac;
use core::ops::{Deref, DerefMut};
use cortex_m::peripheral::SYST;
use fugit::TimerDurationU32;

/// Timer as a delay provider (SysTick by default)
pub struct Delay<TIM = SYST>(Timer<TIM>);

impl<T> Deref for Delay<T> {
    type Target = Timer<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Delay<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Delay<SYST> {
    /// Releases the timer resource
    pub fn release(self) -> Timer<SYST> {
        self.0
    }
}

impl<TIM: Instance> Delay<TIM> {
    /// Releases the timer resource
    pub fn release(mut self) -> Timer<TIM> {
        self.tim.cr1_reset();
        self.0
    }
}

impl<TIM> Timer<TIM> {
    pub fn delay(self) -> Delay<TIM> {
        Delay(self)
    }
}

mod sealed {
    pub trait Wait {
        fn wait(&mut self, prescaler: u16, auto_reload_register: u32);
    }
}
pub(super) use sealed::Wait;

macro_rules! hal {
    ($($TIM:ty,)+) => {
        $(
            impl Wait for Delay<$TIM> {
                fn wait(&mut self, prescaler: u16, auto_reload_register: u32) {
                    // Write Prescaler (PSC)
                    self.tim.set_prescaler(prescaler - 1);

                    // Write Auto-Reload Register (ARR)
                    // Note: Make it impossible to set the ARR value to 0, since this
                    // would cause an infinite loop.
                    self.tim.set_auto_reload(auto_reload_register - 1).unwrap();

                    // Trigger update event (UEV) in the event generation register (EGR)
                    // in order to immediately apply the config
                    self.tim.trigger_update();

                    // Configure the counter in one-pulse mode (counter stops counting at
                    // the next updateevent, clearing the CEN bit) and enable the counter.
                    self.tim.start_one_pulse();

                    // Wait for CEN bit to clear
                    while self.tim.is_counter_enabled() { /* wait */ }
                }
            }
        )+
    }
}

hal! {
    pac::TIM5,
}

#[cfg(feature = "tim2")]
hal! {
    pac::TIM2,
}

/// Periodic non-blocking timer that imlements [embedded_hal::blocking::delay] traits
pub struct FDelay<TIM, const FREQ: u32>(pub(super) FTimer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for FDelay<T, FREQ> {
    type Target = FTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for FDelay<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `FDelay` with precision of 1 μs (1 MHz sampling)
pub type DelayUs<TIM> = FDelay<TIM, 1_000_000>;

/// `FDelay` with precision of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type DelayMs<TIM> = FDelay<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> FDelay<TIM, FREQ> {
    /// Sleep for given time
    pub fn delay(&mut self, time: TimerDurationU32<FREQ>) -> Result<(), Error> {
        let mut ticks = time.ticks() - 1;
        while ticks > 0 {
            let reload = ticks.min(TIM::max_auto_reload());
            ticks -= reload;

            // Write Auto-Reload Register (ARR)
            self.tim.set_auto_reload(reload)?;

            // Trigger update event (UEV) in the event generation register (EGR)
            // in order to immediately apply the config
            self.tim.trigger_update();

            // Configure the counter in one-pulse mode (counter stops counting at
            // the next updateevent, clearing the CEN bit) and enable the counter.
            self.tim.start_one_pulse();

            // Wait for CEN bit to clear
            while self.tim.is_counter_enabled() { /* wait */ }
        }

        Ok(())
    }

    pub fn max_delay(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::max_auto_reload())
    }

    /// Releases the TIM peripheral
    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.0
    }
}

impl<TIM: Instance, const FREQ: u32> fugit_timer::Delay<FREQ> for FDelay<TIM, FREQ> {
    type Error = Error;

    fn delay(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.delay(duration)
    }
}
