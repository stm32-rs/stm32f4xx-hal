//! Bit banding
//!
//! Support for the manipulation of peripheral registers through bit-banding.
//! Not all peripherals are mapped to the bit-banding alias region, the peripheral bit-band region
//! is from `0x4000_0000` to `0x400F_FFFF`. Bit-banding allows the manipulation of individual bits
//! atomically.

use core::ptr;

// Start address of the peripheral memory region capable of being addressed by bit-banding
const PERI_ADDRESS_START: usize = 0x4000_0000;
const PERI_ADDRESS_END: usize = 0x400F_FFFF;

const PERI_BIT_BAND_BASE: usize = 0x4200_0000;

/// Clears the bit on the provided register without modifying other bits.
///
/// # Safety
///
/// Some registers have reserved bits which should not be modified.
pub unsafe fn clear<T>(register: *const T, bit: u8) {
    write(register, bit, false);
}

/// Sets the bit on the provided register without modifying other bits.
///
/// # Safety
///
/// Some registers have reserved bits which should not be modified.
pub unsafe fn set<T>(register: *const T, bit: u8) {
    write(register, bit, true);
}

/// Sets or clears the bit on the provided register without modifying other bits.
///
/// # Safety
///
/// Some registers have reserved bits which should not be modified.
pub unsafe fn write<T>(register: *const T, bit: u8, set: bool) {
    let addr = register as usize;

    assert!(addr >= PERI_ADDRESS_START && addr <= PERI_ADDRESS_END);
    assert!(bit < 32);

    let bit = bit as usize;
    let bb_addr = (PERI_BIT_BAND_BASE + (addr - PERI_ADDRESS_START) * 32) + 4 * bit;
    ptr::write_volatile(bb_addr as *mut u32, if set { 1 } else { 0 });
}
