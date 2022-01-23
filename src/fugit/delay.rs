use super::{Error, Instance, Timer};
use core::ops::{Deref, DerefMut};
use fugit::TimerDurationU32;

/// Periodic non-blocking timer that imlements [embedded_hal::blocking::delay] traits
pub struct Delay<TIM, const FREQ: u32>(pub(super) Timer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Delay<T, FREQ> {
    type Target = Timer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Delay<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Delay` with sampling of 1 MHz
pub type DelayUs<TIM> = Delay<TIM, 1_000_000>;

/// `Delay` with sampling of 1 kHz
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type DelayMs<TIM> = Delay<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> Delay<TIM, FREQ> {
    /// Sleep for given time
    pub fn delay(&mut self, time: TimerDurationU32<FREQ>) -> Result<(), Error> {
        // Write Auto-Reload Register (ARR)
        self.tim.set_auto_reload(time.ticks() - 1)?;

        // Trigger update event (UEV) in the event generation register (EGR)
        // in order to immediately apply the config
        self.tim.trigger_update();

        // Configure the counter in one-pulse mode (counter stops counting at
        // the next updateevent, clearing the CEN bit) and enable the counter.
        self.tim.start_one_pulse();

        // Wait for CEN bit to clear
        while self.tim.is_counter_enabled() { /* wait */ }

        Ok(())
    }

    pub fn max_delay(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::max_auto_reload())
    }

    /// Releases the TIM peripheral
    pub fn release(mut self) -> Timer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.0
    }
}

impl<TIM: Instance, const FREQ: u32> fugit_timer::Delay<FREQ> for Delay<TIM, FREQ> {
    type Error = Error;

    fn delay(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.delay(duration)
    }
}
