use super::{Error, SysEvent};

use crate::{pac::SYST, rcc::Clocks, time::Hertz};
use fugit::{MicrosDurationU32, TimerInstantU32};

impl crate::timer::Timer<SYST> {
    /// Creates SysCounter
    pub fn counter(self) -> SysCounter {
        let Self { tim, clk } = self;
        SysCounter::_new(tim, clk)
    }
}

/// SysTick timer with sampling of 1MHz
pub struct SysCounter {
    tim: SYST,
    mhz: u32,
}

pub trait SysCounterExt: Sized {
    fn counter_us(self, clocks: &Clocks) -> SysCounter;
}

impl SysCounterExt for SYST {
    fn counter_us(self, clocks: &Clocks) -> SysCounter {
        SysCounter::new(self, clocks)
    }
}

impl SysCounter {
    pub fn new(tim: SYST, clocks: &Clocks) -> Self {
        Self::_new(tim, clocks.sysclk())
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.mhz = clocks.sysclk().0 / 1_000_000;
    }

    fn _new(tim: SYST, clk: Hertz) -> Self {
        Self {
            tim,
            mhz: clk.0 / 1_000_000,
        }
    }

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

    pub fn now(&self) -> TimerInstantU32<1_000_000> {
        TimerInstantU32::from_ticks(SYST::get_current() / self.mhz)
    }

    pub fn start(&mut self, timeout: MicrosDurationU32) -> Result<(), Error> {
        let rvr = timeout.ticks() * self.mhz - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl fugit_timer::Timer<1_000_000> for SysCounter {
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<1_000_000> {
        Self::now(self)
    }

    fn start(&mut self, duration: MicrosDurationU32) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}
