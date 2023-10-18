//! CRC32 Calculation Unit
//!
//! This is a hardware accelerated CRC32 calculation unit.
//!
//! It is hardcoded to use the CRC-32 polynomial 0x04C1_1DB7.
//!
//! It operates word-at-a-time, and takes 4 AHB/HCLK cycles per word
//! to calculate. This operation stalls the AHB bus for that time.

use crate::pac::CRC;
use crate::rcc::{Enable, Reset};
use core::mem::MaybeUninit;
use core::ptr::copy_nonoverlapping;

/// A handle to a HAL CRC32 peripheral
pub struct Crc32 {
    periph: CRC,
}

impl Crc32 {
    /// Create a new Crc32 HAL peripheral
    pub fn new(crc: CRC) -> Self {
        unsafe {
            // enable CRC clock.
            CRC::enable_unchecked();
            CRC::reset_unchecked();
        }

        let mut new = Self { periph: crc };
        new.init();

        new
    }

    /// Reset the internal CRC32 state to the default value (0xFFFF_FFFF)
    #[inline(always)]
    pub fn init(&mut self) {
        self.periph.cr().write(|w| w.reset().reset());
    }

    /// Feed words into the CRC engine.
    ///
    /// The resulting calculated CRC (including this and prior data
    /// since the last call to `init()` is returned.
    pub fn update(&mut self, data: &[u32]) -> u32 {
        // Feed each word into the engine
        for word in data {
            self.periph.dr().write(|w| w.bits(*word));
        }
        // Retrieve the resulting CRC
        self.periph.dr().read().bits()
    }

    /// Feed bytes into the CRC engine.
    ///
    /// The resulting calculated CRC (including this and prior data
    /// since the last call to `init()` is returned.
    ///
    /// NOTE: Each four-byte chunk will be copied into a scratch buffer. This
    /// is done to ensure alignment of the data (the CRC engine only processes
    /// full words at a time). If the number of bytes passed in are not a
    /// multiple of four, the MOST significant bytes of the remaining word will
    /// be zeroes.
    ///
    /// This should be taken into consideration if attempting to feed bytes
    /// across multiple parts (that spurious zeroes will be inserted)! To
    /// avoid this, only feed multiples of 4 bytes in before the "final"
    /// part of the message.
    ///
    /// Example: Given the following 7 bytes:
    ///
    /// `[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]`
    ///
    /// The following two words will be fed into the CRC engine:
    ///
    /// 1. `0x4433_2211`
    /// 2. `0x0077_6655`
    pub fn update_bytes(&mut self, data: &[u8]) -> u32 {
        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        // For each full chunk of four bytes...
        chunks.for_each(|chunk| unsafe {
            // Create an uninitialized scratch buffer. We make it uninitialized
            // to avoid re-zeroing this data inside of the loop.
            let mut scratch: MaybeUninit<[u8; 4]> = MaybeUninit::uninit();

            // Copy the (potentially unaligned) bytes from the input chunk to
            // our scratch bytes. We cast the `scratch` buffer from a `*mut [u8; 4]`
            // to a `*mut u8`.
            let src: *const u8 = chunk.as_ptr();
            let dst: *mut u8 = scratch.as_mut_ptr().cast::<u8>();
            copy_nonoverlapping(src, dst, 4);

            // Mark the scratch bytes as initialized, and then convert it to a
            // native-endian u32. Feed this into the CRC peripheral
            self.periph
                .dr()
                .write(|w| w.bits(u32::from_ne_bytes(scratch.assume_init())));
        });

        // If we had a non-multiple of four bytes...
        if !remainder.is_empty() {
            // Create a zero-filled scratch buffer, and copy the data in
            let mut scratch = [0u8; 4];

            // NOTE: We are on a little-endian processor. This means that copying
            // the 0..len range fills the LEAST significant bytes, leaving the
            // MOST significant bytes as zeroes
            scratch[..remainder.len()].copy_from_slice(remainder);
            self.periph
                .dr()
                .write(|w| w.bits(u32::from_ne_bytes(scratch)));
        }

        self.periph.dr().read().bits()
    }

    /// Consume the HAL peripheral, returning the PAC peripheral
    pub fn release(self) -> CRC {
        // Disable CRC clock
        unsafe {
            CRC::disable_unchecked();
        }

        self.periph
    }
}
