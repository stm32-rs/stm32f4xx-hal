use super::{compute_arr_presc, Error, Event, Instance, Timer};
use core::ops::{Deref, DerefMut};
use fugit::HertzU32 as Hertz;

/// Hardware timers
pub struct CounterHz<TIM>(Timer<TIM>);

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
