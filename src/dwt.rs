//! Debug and trace and stuff

use crate::rcc::Clocks;
use crate::time::Hertz;
use cortex_m::peripheral::{DCB, DWT};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub trait DwtExt {
    fn constrain(self, dcb: DCB, clocks: Clocks) -> Dwt;
}
impl DwtExt for DWT {
    /// Enable trace unit and cycle counter
    fn constrain(mut self, mut dcb: DCB, clocks: Clocks) -> Dwt {
        dcb.enable_trace();
        self.enable_cycle_counter();
        Dwt {
            dwt: self,
            dcb,
            clocks,
        }
    }
}

pub struct Dwt {
    dwt: DWT,
    dcb: DCB,
    clocks: Clocks,
}
impl Dwt {
    /// Release the dwt and dcb control
    /// # Safety
    /// All instances of Delay and StopWatch become invalid after this
    pub unsafe fn release(self) -> (DWT, DCB) {
        (self.dwt, self.dcb)
    }
    /// Create a delay instance
    pub fn delay(&self) -> Delay {
        Delay {
            clock: self.clocks.hclk(),
        }
    }
    /// Create a stopwatch instance
    /// # Arguments
    /// * `times` - Array which will be holding the timings in ticks (max laps == times.len()-1)
    pub fn stopwatch<'i>(&self, times: &'i mut [u32]) -> StopWatch<'i> {
        StopWatch::new(times, self.clocks.hclk())
    }
    /// Measure cycles it takes to execute f
    pub fn measure<F: FnOnce()>(&self, f: F) -> ClockDuration {
        let mut times: [u32; 2] = [0; 2];
        let mut sw = self.stopwatch(&mut times);
        f();
        sw.lap().lap_time(1).unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct Delay {
    clock: Hertz,
}
impl Delay {
    /// Delay for ClockDuration::ticks
    pub fn delay(duration: ClockDuration) {
        let ticks = duration.ticks as u64;
        Delay::delay_ticks(DWT::get_cycle_count(), ticks);
    }
    /// Delay ticks
    /// NOTE DCB and DWT need to be set up for this to work, so it is private
    fn delay_ticks(mut start: u32, ticks: u64) {
        if ticks < (u32::MAX / 2) as u64 {
            // Simple delay
            let ticks = ticks as u32;
            while (DWT::get_cycle_count().wrapping_sub(start)) < ticks {}
        } else if ticks <= u32::MAX as u64 {
            // Try to avoid race conditions by limiting delay to u32::MAX / 2
            let mut ticks = ticks as u32;
            ticks -= u32::MAX / 2;
            while (DWT::get_cycle_count().wrapping_sub(start)) < u32::MAX / 2 {}
            start -= u32::MAX / 2;
            while (DWT::get_cycle_count().wrapping_sub(start)) < ticks {}
        } else {
            // Delay for ticks, then delay for rest * u32::MAX
            let mut rest = (ticks >> 32) as u32;
            let ticks = (ticks & u32::MAX as u64) as u32;
            loop {
                while (DWT::get_cycle_count().wrapping_sub(start)) < ticks {}
                if rest == 0 {
                    break;
                }
                rest -= 1;
                while (DWT::get_cycle_count().wrapping_sub(start)) > ticks {}
            }
        }
    }
}

// Implement DelayUs/DelayMs for various integer types
impl<T: Into<u64>> DelayUs<T> for Delay {
    fn delay_us(&mut self, us: T) {
        // Convert us to ticks
        let start = DWT::get_cycle_count();
        let ticks = (us.into() * self.clock.0 as u64) / 1_000_000;
        Delay::delay_ticks(start, ticks);
    }
}
impl<T: Into<u64>> DelayMs<T> for Delay {
    fn delay_ms(&mut self, ms: T) {
        // Convert ms to ticks
        let start = DWT::get_cycle_count();
        let ticks = (ms.into() * self.clock.0 as u64) / 1_000;
        Delay::delay_ticks(start, ticks);
    }
}

/// Very simple stopwatch
pub struct StopWatch<'l> {
    times: &'l mut [u32],
    timei: usize,
    clock: Hertz,
}
impl<'l> StopWatch<'l> {
    /// Create a new instance (Private because dwt/dcb should be set up)
    /// # Arguments
    /// * `times` - Array which will be holding the timings (max laps == times.len()-1)
    /// * `clock` - The DWT cycle counters clock
    fn new(times: &'l mut [u32], clock: Hertz) -> Self {
        assert!(times.len() >= 2);
        let mut sw = StopWatch {
            times,
            timei: 0,
            clock,
        };
        sw.reset();
        sw
    }
    /// Returns the numbers of laps recorded
    pub fn lap_count(&self) -> usize {
        self.timei
    }
    /// Resets recorded laps to 0 and sets 0 offset
    pub fn reset(&mut self) {
        self.timei = 0;
        self.times[0] = DWT::get_cycle_count();
    }
    /// Record a new lap
    /// NOTE If lap count exceeds maximum, the last lap is updated
    pub fn lap(&mut self) -> &mut Self {
        let c = DWT::get_cycle_count();
        if self.timei < self.times.len() {
            self.timei += 1;
        }
        self.times[self.timei] = c;
        self
    }
    /// Calculate the time of lap n (n starting with 1)
    /// NOTE Returns None if 'n' is out of range
    pub fn lap_time(&self, n: usize) -> Option<ClockDuration> {
        if (n < 1) || (self.timei < n) {
            None
        } else {
            Some(ClockDuration {
                ticks: self.times[n].wrapping_sub(self.times[n - 1]),
                clock: self.clock,
            })
        }
    }
}

/// Clock difference with capability to calculate SI units (s)
#[derive(Clone, Copy)]
pub struct ClockDuration {
    ticks: u32,
    clock: Hertz,
}
impl ClockDuration {
    /// Returns ticks
    pub fn as_ticks(self) -> u32 {
        self.ticks
    }
    /// Returns calculated milliseconds as integer
    pub fn as_millis(self) -> u64 {
        self.ticks as u64 * 1_000 / self.clock.0 as u64
    }
    /// Returns calculated microseconds as integer
    pub fn as_micros(self) -> u64 {
        self.ticks as u64 * 1_000_000 / self.clock.0 as u64
    }
    /// Returns calculated nanoseconds as integer
    pub fn as_nanos(self) -> u64 {
        self.ticks as u64 * 1_000_000_000 / self.clock.0 as u64
    }
    /// Return calculated seconds as 32-bit float
    pub fn as_secs_f32(self) -> f32 {
        self.ticks as f32 / self.clock.0 as f32
    }
    /// Return calculated seconds as 64-bit float
    pub fn as_secs_f64(self) -> f64 {
        self.ticks as f64 / self.clock.0 as f64
    }
}
