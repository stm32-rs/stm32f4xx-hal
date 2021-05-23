use core::cmp;
use core::mem;

use crate::hal::blocking::rng;
use crate::pac;
use crate::pac::RNG;
use crate::rcc::{Clocks, Enable, Reset};
use crate::time::U32Ext;
use core::num::NonZeroU32;
use core::ops::Shl;
use rand_core::RngCore;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    /// The RNG_CLK was not correctly detected (fRNG_CLK< fHCLK/16).
    /// See CECS in RNG peripheral documentation.
    ClockError = 2,
    /// RNG detected more than 64 consecutive bits of the same value (0 or 1) OR
    /// more than 32 consecutive 01 pairs.
    /// See SECS in RNG peripheral documentation.
    SeedError = 4,
}

impl From<ErrorKind> for rand_core::Error {
    fn from(err: ErrorKind) -> rand_core::Error {
        let err_code = NonZeroU32::new(rand_core::Error::CUSTOM_START + err as u32).unwrap();
        rand_core::Error::from(err_code)
    }
}

pub trait RngExt {
    fn constrain(self, clocks: Clocks) -> Rng;
}

impl RngExt for RNG {
    /// Enable RNG_CLK and the RNG peripheral.
    /// Note that clocks must already be configured such that RNG_CLK is not less than 1/16 HCLK,
    /// otherwise all reads of the RNG would return a ClockError (CECS error).
    /// This function will panic if pll48clk < 1/16 hclk.
    fn constrain(self, clocks: Clocks) -> Rng {
        let rcc = unsafe { &*pac::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // enable RNG_CLK (peripheral clock)
            RNG::enable(rcc);
            RNG::reset(rcc);

            // verify the clock configuration is valid
            let hclk = clocks.hclk();
            let rng_clk = clocks.pll48clk().unwrap_or_else(|| 0u32.hz());
            assert!(rng_clk.0 >= (hclk.0 / 16));

            // enable the RNG peripheral
            self.cr.modify(|_, w| w.rngen().set_bit());
        });

        Rng { rb: self }
    }
}

pub struct Rng {
    rb: RNG,
}

impl Rng {
    /// Returns 32 bits of random data from RNDATA, or error.
    /// May fail if, for example RNG_CLK is misconfigured.
    fn next_random_word(&mut self) -> Result<u32, ErrorKind> {
        loop {
            let status = self.rb.sr.read();
            if status.cecs().bit() {
                return Err(ErrorKind::ClockError);
            }
            if status.secs().bit() {
                return Err(ErrorKind::SeedError);
            }
            if status.drdy().bit() {
                return Ok(self.rb.dr.read().rndata().bits());
            }
        }
    }

    pub fn release(self) -> RNG {
        self.rb
    }
}

impl rng::Read for Rng {
    type Error = rand_core::Error;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.try_fill_bytes(buffer)
    }
}

impl RngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        self.next_random_word().unwrap()
    }

    fn next_u64(&mut self) -> u64 {
        let w1 = self.next_u32();
        let w2 = self.next_u32();
        (w1 as u64).shl(32) | (w2 as u64)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.try_fill_bytes(dest).unwrap()
    }

    /// Fills buffer with random values, or returns an error
    fn try_fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), rand_core::Error> {
        const BATCH_SIZE: usize = 4 / mem::size_of::<u8>();
        let mut i = 0_usize;
        while i < buffer.len() {
            let random_word = self.next_random_word()?;
            let bytes = random_word.to_ne_bytes();
            let n = cmp::min(BATCH_SIZE, buffer.len() - i);
            buffer[i..i + n].copy_from_slice(&bytes[..n]);
            i += n;
        }
        Ok(())
    }
}
