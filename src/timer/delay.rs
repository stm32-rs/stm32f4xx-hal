//! Delays

use super::{General, Instance, Timer};
use crate::pac;
use core::ops::{Deref, DerefMut};
use cortex_m::peripheral::SYST;

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
