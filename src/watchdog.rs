//! Watchdog peripherals

use crate::{
    pac::{DBGMCU, IWDG},
    time::MilliSeconds,
};
use core::fmt;
use embedded_hal::watchdog::{Watchdog, WatchdogEnable};

/// Wraps the Independent Watchdog (IWDG) peripheral
pub struct IndependentWatchdog {
    iwdg: IWDG,
}

#[cfg(feature = "defmt")]
impl defmt::Format for IndependentWatchdog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IndependentWatchdog");
    }
}

impl fmt::Debug for IndependentWatchdog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IndependentWatchdog")
    }
}

const MAX_PR: u8 = 0b110;
const MAX_RL: u16 = 0xFFF;
const KR_ACCESS: u16 = 0x5555;
const KR_RELOAD: u16 = 0xAAAA;
const KR_START: u16 = 0xCCCC;

impl IndependentWatchdog {
    /// Wrap and start the watchdog
    pub fn new(iwdg: IWDG) -> Self {
        IndependentWatchdog { iwdg }
    }

    /// Debug independent watchdog stopped when core is halted
    pub fn stop_on_debug(&self, dbgmcu: &DBGMCU, stop: bool) {
        dbgmcu.apb1_fz.modify(|_, w| w.dbg_iwdg_stop().bit(stop));
    }

    fn setup(&self, timeout_ms: u32) {
        let mut pr = 0;
        while pr < MAX_PR && Self::timeout_period(pr, MAX_RL) < timeout_ms {
            pr += 1;
        }

        let max_period = Self::timeout_period(pr, MAX_RL);
        let max_rl = u32::from(MAX_RL);
        let rl = (timeout_ms * max_rl / max_period).min(max_rl) as u16;

        self.access_registers(|iwdg| {
            iwdg.pr.modify(|_, w| w.pr().bits(pr));
            iwdg.rlr.modify(|_, w| w.rl().bits(rl));
        });
    }

    fn is_pr_updating(&self) -> bool {
        self.iwdg.sr.read().pvu().bit()
    }

    /// Returns the interval in ms
    pub fn interval(&self) -> MilliSeconds {
        while self.is_pr_updating() {}

        let pr = self.iwdg.pr.read().pr().bits();
        let rl = self.iwdg.rlr.read().rl().bits();
        let ms = Self::timeout_period(pr, rl);
        MilliSeconds(ms)
    }

    /// pr: Prescaler divider bits, rl: reload value
    ///
    /// Returns ms
    fn timeout_period(pr: u8, rl: u16) -> u32 {
        let divider: u32 = match pr {
            0b000 => 4,
            0b001 => 8,
            0b010 => 16,
            0b011 => 32,
            0b100 => 64,
            0b101 => 128,
            0b110 => 256,
            0b111 => 256,
            _ => unreachable!(),
        };
        (u32::from(rl) + 1) * divider / 32
    }

    fn access_registers<A, F: FnMut(&IWDG) -> A>(&self, mut f: F) -> A {
        // Unprotect write access to registers
        self.iwdg.kr.write(|w| unsafe { w.key().bits(KR_ACCESS) });
        let a = f(&self.iwdg);

        // Protect again
        self.iwdg.kr.write(|w| unsafe { w.key().bits(KR_RELOAD) });
        a
    }
}

impl WatchdogEnable for IndependentWatchdog {
    type Time = MilliSeconds;

    fn start<T: Into<Self::Time>>(&mut self, period: T) {
        self.setup(period.into().0);

        self.iwdg.kr.write(|w| unsafe { w.key().bits(KR_START) });
    }
}

impl Watchdog for IndependentWatchdog {
    fn feed(&mut self) {
        self.iwdg.kr.write(|w| unsafe { w.key().bits(KR_RELOAD) });
    }
}
