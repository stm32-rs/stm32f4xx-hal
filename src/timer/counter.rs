use super::{compute_arr_presc, Error, Event, FTimer, Instance, Timer};
use core::ops::{Deref, DerefMut};
use fugit::HertzU32 as Hertz;

/// Hardware timers
pub struct CounterHz<TIM>(pub(super) Timer<TIM>);

impl<T> Deref for CounterHz<T> {
    type Target = Timer<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CounterHz<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<TIM: Instance> CounterHz<TIM> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> Timer<TIM> {
        // stop timer
        self.tim.cr1_reset();
        self.0
    }
}

impl<TIM: Instance> CounterHz<TIM> {
    pub fn start(&mut self, timeout: Hertz) -> Result<(), Error> {
        // pause
        self.tim.disable_counter();
        // reset counter
        self.tim.reset_counter();

        let (psc, arr) = compute_arr_presc(timeout.raw(), self.clk.raw());
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr)?;

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
use fugit::{TimerDurationU32, TimerInstantU32};

/// Periodic non-blocking timer that imlements [embedded_hal::timer::CountDown]
pub struct Counter<TIM, const FREQ: u32>(pub(super) FTimer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Counter<T, FREQ> {
    type Target = FTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Counter<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Counter` with precision of 1 μs (1 MHz sampling)
pub type CounterUs<TIM> = Counter<TIM, 1_000_000>;

/// `Counter` with precision of of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type CounterMs<TIM> = Counter<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> Counter<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> FTimer<TIM, FREQ> {
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
