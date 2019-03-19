//! Device electronic signature
//!
//! (stored in flash memory)

use core::str::from_utf8_unchecked;

macro_rules! define_ptr_type {
    ($name: ident, $ptr: expr) => (
        impl $name {
            fn ptr() -> *const Self {
                $ptr as *const _
            }

            /// Returns a wrapped reference to the value in flash memory
            pub fn get() -> &'static Self {
                unsafe { &*Self::ptr() }
            }
        }
    )
}

/// Uniqure Device ID register
#[derive(Hash, Debug)]
#[repr(C)]
pub struct Uid {
    x: u16,
    y: u16,
    waf_lot: [u8; 8],
}
define_ptr_type!(Uid, 0x1FFF_7A10);

impl Uid {
    /// X coordinate on wafer
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Y coordinate on wafer
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Wafer number
    pub fn waf_num(&self) -> u8 {
        self.waf_lot[0].into()
    }

    /// Lot number
    pub unsafe fn lot_num(&self) -> &str {
        from_utf8_unchecked(&self.waf_lot[1..])
    }
}

/// Size of integrated flash
#[derive(Debug)]
#[repr(C)]
pub struct FlashSize(u16);
define_ptr_type!(FlashSize, 0x1FFF_7A22);

impl FlashSize {
    /// Read flash size in kilobytes
    pub fn kilo_bytes(&self) -> u16 {
        self.0
    }

    /// Read flash size in bytes
    pub fn bytes(&self) -> usize {
        usize::from(self.kilo_bytes()) * 1024
    }
}
