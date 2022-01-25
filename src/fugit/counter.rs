use super::{Error, Event, Instance, Timer};

use core::ops::{Deref, DerefMut};
use fugit::{TimerDurationU32, TimerInstantU32};

/// Periodic non-blocking timer that imlements [embedded_hal::timer::CountDown]
pub struct Counter<TIM, const FREQ: u32>(pub(super) Timer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Counter<T, FREQ> {
    type Target = Timer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Counter<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Counter` with sampling of 1 MHz
pub type CounterUs<TIM> = Counter<TIM, 1_000_000>;

/// `Counter` with sampling of 1 kHz
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type CounterMs<TIM> = Counter<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> Counter<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> Timer<TIM, FREQ> {
        // stop counter
        self.tim.cr1_reset();
        self.0
    }

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(self.tim.read_count().into())
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        // pause
        self.tim.disable_counter();
        // reset counter
        self.tim.reset_counter();

        self.tim.set_auto_reload(timeout.ticks() - 1)?;

        // Trigger update event to load the registers
        self.tim.trigger_update();

        // start counter
        self.tim.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tim.get_interrupt_flag().contains(Event::Update) {
            self.tim.clear_interrupt_flag(Event::Update);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        // disable counter
        self.tim.disable_counter();
        Ok(())
    }
}

impl<TIM: Instance, const FREQ: u32> fugit_timer::Timer<FREQ> for Counter<TIM, FREQ> {
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<FREQ> {
        Self::now(self)
    }

    fn start(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }
}
