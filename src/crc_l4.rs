//! CRC calculation unit
//!
//! Usage example:
//! ```
//! let crc = dp.CRC.constrain(&mut rcc.ahb1);
//!
//! // Lets use the CRC-16-CCITT polynomial
//! let mut crc = crc.polynomial(crc::Polynomial::L16(0x1021)).freeze();
//!
//! let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
//! crc.feed(&data);
//!
//! let result = crc.result();
//! assert!(result == 0x78cb);
//! ```

#![deny(missing_docs)]

use crate::pac::CRC;
use crate::rcc::{self, Enable};
use core::hash::Hasher;

/// Extension trait to constrain the CRC peripheral.
pub trait CrcExt {
    /// Constrains the CRC peripheral to play nicely with the other abstractions
    fn constrain(self, ahb1: &mut rcc::AHB1) -> Config;
}

impl CrcExt for CRC {
    fn constrain(self, ahb1: &mut rcc::AHB1) -> Config {
        // Enable power to CRC unit
        CRC::enable(ahb1);

        // Default values
        Config {
            initial_value: 0xffff_ffff,
            polynomial: Polynomial::L32(0x04c1_1db7),
            input_bit_reversal: None,
            output_bit_reversal: false,
        }
    }
}

/// Polynomial settings.
pub enum Polynomial {
    /// 7-bit polynomial, only the lowest 7 bits are valid
    L7(u8),
    /// 8-bit polynomial
    L8(u8),
    /// 16-bit polynomial
    L16(u16),
    /// 32-bit polynomial
    L32(u32),
}

/// Bit reversal settings.
pub enum BitReversal {
    /// Reverse bits by byte
    ByByte,
    /// Reverse bits by half-word
    ByHalfWord,
    /// Reverse bits by word
    ByWord,
}

/// CRC configuration structure, uses builder pattern.
pub struct Config {
    initial_value: u32,
    polynomial: Polynomial,
    input_bit_reversal: Option<BitReversal>,
    output_bit_reversal: bool,
}

impl Config {
    /// Sets the initial value of the CRC.
    pub fn initial_value(mut self, init: u32) -> Self {
        self.initial_value = init;

        self
    }

    /// Sets the polynomial of the CRC.
    pub fn polynomial(mut self, polynomial: Polynomial) -> Self {
        self.polynomial = polynomial;

        self
    }

    /// Enables bit reversal of the inputs.
    pub fn input_bit_reversal(mut self, rev: BitReversal) -> Self {
        self.input_bit_reversal = Some(rev);

        self
    }

    /// Enables bit reversal of the outputs.
    pub fn output_bit_reversal(mut self, rev: bool) -> Self {
        self.output_bit_reversal = rev;

        self
    }

    /// Freezes the peripheral, making the configuration take effect.
    pub fn freeze(self) -> Crc {
        let crc = unsafe { &(*CRC::ptr()) };

        let (poly, poly_bits, init) = match self.polynomial {
            Polynomial::L7(val) => ((val & 0x7f) as u32, 0b11, self.initial_value & 0x7f),
            Polynomial::L8(val) => (val as u32, 0b10, self.initial_value & 0xff),
            Polynomial::L16(val) => (val as u32, 0b01, self.initial_value & 0xffff),
            Polynomial::L32(val) => (val, 0b00, self.initial_value),
        };

        let in_rev_bits = match self.input_bit_reversal {
            None => 0b00,
            Some(BitReversal::ByByte) => 0b01,
            Some(BitReversal::ByHalfWord) => 0b10,
            Some(BitReversal::ByWord) => 0b11,
        };

        crc.init.write(|w| w.init().bits(init));
        crc.pol.write(|w| w.bits(poly));
        crc.cr.write(|w| {
            w.rev_in().bits(in_rev_bits);
            w.polysize().bits(poly_bits);
            w.reset().set_bit();

            if self.output_bit_reversal {
                w.rev_out().set_bit()
            } else {
                w.rev_out().clear_bit()
            }
        });

        Crc {}
    }
}

/// Constrained CRC peripheral.
pub struct Crc {}

impl Crc {
    /// This will reset the CRC to its initial condition.
    #[inline]
    pub fn reset(&mut self) {
        let crc = unsafe { &(*CRC::ptr()) };

        crc.cr.modify(|_, w| w.reset().set_bit());
    }

    /// This will reset the CRC to its initial condition, however with a specific initial value.
    /// This is very useful if many task are sharing the CRC peripheral, as one can read out the
    /// intermediate result, store it until the next time a task runs, and initialize with the
    /// intermediate result to continue where the task left off.
    #[inline]
    pub fn reset_with_inital_value(&mut self, initial_value: u32) {
        let crc = unsafe { &(*CRC::ptr()) };

        crc.init.write(|w| w.init().bits(initial_value));
        crc.cr.modify(|_, w| w.reset().set_bit());
    }

    /// Feed the CRC with data
    #[inline]
    pub fn feed(&mut self, data: &[u8]) {
        let crc = unsafe { &(*CRC::ptr()) };
        for &byte in data {
            crc.dr8().write(|w| w.dr8().bits(byte));
        }
    }

    /// Get the result of the CRC, depending on the polynomial chosen only a certain amount of the
    /// bits are the result. This will reset the CRC peripheral after use.
    #[inline]
    pub fn result(&mut self) -> u32 {
        let ret = self.peek_result();

        self.reset();

        ret
    }

    /// Get a peed at the result of the CRC, depending on the polynomial chosen only a certain
    /// amount of the bits are the result.
    #[inline]
    pub fn peek_result(&self) -> u32 {
        let crc = unsafe { &(*CRC::ptr()) };

        crc.dr().read().bits()
    }
}

impl Hasher for Crc {
    #[inline]
    fn finish(&self) -> u64 {
        // `peek_result` as `core::hash::Hasher` required that the `finish` method does not reset
        // the hasher.
        self.peek_result() as u64
    }

    #[inline]
    fn write(&mut self, data: &[u8]) {
        self.feed(data);
    }
}
