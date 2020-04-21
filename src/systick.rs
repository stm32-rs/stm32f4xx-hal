//! SYSTEMTIME based on systick

use cortex_m::interrupt::free;
use cortex_m::peripheral::SYST;
use cortex_m_rt::exception;

use crate::rcc::Clocks;
use crate::time::Hertz;
use crate::timer::{Event, Timer};

use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub trait SysTickTime {
    fn to_systemtime<T>(self, timeout: T, clocks: Clocks) -> SysTime
    where
        T: Into<Hertz>;
}
impl SysTickTime for SYST {
    fn to_systemtime<T>(self, timeout_hz: T, clocks: Clocks) -> SysTime
    where
        T: Into<Hertz>,
    {
        let timeout_hz = timeout_hz.into();

        let mut systime = SysTickTimeStatic {
            countdown: Timer::syst(self, timeout_hz, clocks),
            systick: 0,
            tick_to_ns: 1_000_000_000 / timeout_hz.0,
        };

        systime.countdown.listen(Event::TimeOut);

        free(|_| unsafe {
            SYSTIME = Some(systime);
        });

        SysTime {}
    }
}

struct SysTickTimeStatic {
    countdown: Timer<SYST>,
    systick: u64,
    tick_to_ns: u32,
}

// there can only be one!
static mut SYSTIME: Option<SysTickTimeStatic> = None;

pub struct SysTime {}
impl SysTime {
    /// return time in ns
    pub fn as_nanos(&self) -> u64 {
        let mut tick_to_ns = 0u32;
        free(|_| unsafe {
            if let Some(systime) = &SYSTIME {
                tick_to_ns = systime.tick_to_ns;
                (&systime.systick as *const u64).read_volatile()
            } else {
                0
            }
        }) * tick_to_ns as u64
    }
    /// return time in us
    pub fn as_micros(&self) -> u64 {
        self.as_nanos() / 1_000_000
    }
    /// return time in ms
    pub fn as_millis(&self) -> u64 {
        self.as_nanos() / 1_000_000
    }
    /// return time in seconds, as f64
    pub fn as_secs_f64(&self) -> f64 {
        self.as_nanos() as f64 / 1_000_000_000f64
    }
    /// return time in seconds, as f32
    pub fn as_secs_f32(&self) -> f32 {
        self.as_nanos() as f32 / 1_000_000_000f32
    }

    /// delay n ns
    ///  note: this function depends on the systick interrupt,
    ///        so do not use it from other interrupts (with higher priority).
    fn delay_ns_(&self, ns: u64) {
        let timeout = self.as_nanos() + ns;
        while timeout >= self.as_nanos() {}
    }
}
// Implement DelayUs/DelayMs for various integer types
macro_rules! impl_DelayIntT {
    (for $($t:ty),+) => {$(
        impl DelayMs<$t> for SysTime {
            fn delay_ms(&mut self, ms: $t) {
                self.delay_ns_(ms as u64 * 1_000_000);
            }
        }
        impl DelayUs<$t> for SysTime {
            fn delay_us(&mut self, us: $t) {
                self.delay_ns_(us as u64 * 1_000);
            }
        }
    )*}
}
impl_DelayIntT!(for usize,u64,u32,u16,u8,i64,i32,i16,i8);

// there can only be one!
#[exception]
fn SysTick() {
    free(|_| unsafe {
        if let Some(systime) = &mut SYSTIME {
            systime.systick += 1;
        }
    });
}
