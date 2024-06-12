//! # Hardware random number generator.
//!
//!
//! The build in random number generator (RNG) of an STM32F4 uses analog noise to
//! provide random 32-bit values.
//!
//! Notes:
//! - It takes 40 periods of `RNG_CLK` to generate a new random value.
//! - The RNG requires the `PLL48_CLK` to be active ([more details](RngExt::constrain))
//!
//! For more details, see reference manual chapter 24.
//!
//! Minimal working example:
//! ```
//! let dp = pac::Peripherals::take().unwrap();
//! let rcc = dp.RCC.constrain();
//! let clocks = rcc.cfgr.require_pll48clk().freeze();
//! let mut rand_source = dp.RNG.constrain(clocks);
//! let rand_val = rand_source.next_u32();
//! ```
//!
//! A full example can be found [in the examples folder on github](https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/rng-display.rs)
use core::cmp;
use core::mem;

use crate::pac::RNG;
use crate::rcc::{Enable, Rcc, Reset};
use core::num::NonZeroU32;
use core::ops::Shl;
use embedded_hal_02::blocking::rng;

/// Random number generator specific errors
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

impl From<ErrorKind> for rand_core_06::Error {
    fn from(err: ErrorKind) -> rand_core_06::Error {
        let err_code = NonZeroU32::new(rand_core_06::Error::CUSTOM_START + err as u32).unwrap();
        rand_core_06::Error::from(err_code)
    }
}

/// Helper trait to implement the `constrain` method for the
/// [RNG peripheral](crate::pac::RNG) which is how the [Rng] struct is
/// created.
///
/// Usage:
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// let clocks = rcc.cfgr.require_pll48clk().freeze();
/// let mut rand_source = dp.RNG.constrain(clocks);
/// ```
pub trait RngExt {
    /// Enables the hardware random generator and provides the [Rng] struct.
    ///
    /// The datasheet states, that the `RNG_CLK` must not be less than 1/16 HCLK
    /// (HCLK is the CPU clock), otherwise all reads of the RNG would return a
    /// ClockError (CECS error).
    /// As the `RNG_CLK` always seems to be connected to the `PLL48_CLK` and the
    /// maximum value of `HCLK` is 168MHz, this is always true as long as the `PLL48_CLK` is enabled.
    /// This can be done with the [require_pll48clk](crate::rcc::Config::require_pll48clk) function.
    ///
    /// See reference manual section 24.4.2 for more details
    ///
    /// # Panics
    ///
    /// This function will panic if `PLL48_CLK < 1/16 HCLK`.
    fn constrain(self, rcc: &mut Rcc) -> Rng;
}

impl RngExt for RNG {
    fn constrain(self, rcc: &mut Rcc) -> Rng {
        cortex_m::interrupt::free(|_| {
            // need set enable pll for this operation
            if rcc.cr().read().pllrdy().bit_is_clear() {
                rcc.cr().modify(|_, w| w.pllon().set_bit());
                // wait till pll is ready
                while rcc.cr().read().pllrdy().bit_is_clear() {}
            }

            // enable RNG_CLK (peripheral clock)
            // enable RNG_CLK (peripheral clock)
            RNG::enable(rcc);
            // give RNG_CLK time to start
            RNG::is_enabled();
            RNG::reset(rcc);

            // enable the RNG peripheral
            self.cr().modify(|_, w| w.rngen().set_bit());
            // hardware check for clock is used
            // instead of software calculation, which may be inaccurate.
            // until data is available we will check for CECS flag, if it is set
            // means that clock error occured
            while !self.sr().read().drdy().bit() {
                assert!(!self.sr().read().cecs().bit());
            }
        });

        Rng { rb: self }
    }
}

/// Random number provider which provides access to all [rand_core::RngCore]
/// functions.
///
/// Example use:
///
/// ```
/// use rand_core::RngCore;
///
/// // ...
///
/// let mut rand_source = dp.RNG.constrain(clocks);
/// let rand_u32: u32 = rand_source.next_u32();
/// let rand_u64: u64 = rand_source.next_u64();
/// ```
pub struct Rng {
    rb: RNG,
}

impl Rng {
    /// Returns 32 bits of random data from RNDATA, or error.
    /// May fail if, for example RNG_CLK is misconfigured.
    fn next_random_word(&mut self) -> Result<u32, ErrorKind> {
        loop {
            let status = self.rb.sr().read();
            if status.cecs().bit() {
                return Err(ErrorKind::ClockError);
            }
            if status.secs().bit() {
                return Err(ErrorKind::SeedError);
            }
            if status.drdy().bit() {
                return Ok(self.rb.dr().read().rndata().bits());
            }
        }
    }

    fn try_fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), ErrorKind> {
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

    /// Releases ownership of the [`RNG`] peripheral object
    /// (after which `self` can't be used anymore).
    pub fn release(self) -> RNG {
        self.rb
    }
}

impl rng::Read for Rng {
    type Error = rand_core_06::Error;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.try_fill_bytes(buffer)?;
        Ok(())
    }
}

impl rand_core::RngCore for Rng {
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
}

impl rand_core_06::RngCore for Rng {
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
    fn try_fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), rand_core_06::Error> {
        self.try_fill_bytes(buffer)?;
        Ok(())
    }
}
