//! Device electronic signature
//!
//! (stored in flash memory)
use core::fmt;
use core::str;

use core::convert::TryInto;

macro_rules! define_ptr_type {
    ($name: ident, $ptr: expr) => {
        impl $name {
            fn ptr() -> *const Self {
                $ptr as *const _
            }

            /// Returns a wrapped reference to the value in flash memory
            pub fn get() -> &'static Self {
                unsafe { &*Self::ptr() }
            }
        }
    };
}

/// Uniqure Device ID register
#[repr(C, packed)]
pub struct Uid {
    x: u16,
    y: u16,
    waf: u8,
    lot: [u8; 7],
}
define_ptr_type!(Uid, 0x1FFF_F7AC);

#[cfg(feature = "defmt")]
impl defmt::Format for Uid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Peripheral {{ x: {:x}, y: {:x}, waf: {}, lum: {}}}",
            { self.x },
            { self.y },
            { self.waf },
            { self.lot_number() },
        );
    }
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Uid")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("waf", &self.waf)
            .field("lot", &self.lot_number())
            .finish()
    }
}

fn bcd_to_num(bcd_num: u16) -> u16 {
    bcd_num
        .to_ne_bytes()
        .iter()
        .enumerate()
        .map(|(i, byte)| (i * 2, (*byte & 0xF0) >> 4, *byte & 0x0F))
        .map(|(i, high_nibble, low_nibble)| {
            let i: u32 = i.try_into().unwrap_or_default(); // This should never overflow
            u16::from(high_nibble) * 10u16.pow(i + 1) + u16::from(low_nibble) * 10u16.pow(i)
        })
        .sum()
}

impl Uid {
    /// X coordinate on wafer in BCD format
    pub fn x_bcd(&self) -> u16 {
        self.x
    }

    /// X coordinate on wafer
    pub fn x(&self) -> u16 {
        bcd_to_num(self.x)
    }

    /// Y coordinate on wafer in BCD format
    pub fn y_bcd(&self) -> u16 {
        self.y
    }

    /// Y coordinate on wafer
    pub fn y(&self) -> u16 {
        bcd_to_num(self.y)
    }

    /// Wafer number
    pub fn wafer_number(&self) -> u8 {
        self.waf
    }

    /// Lot number
    pub fn lot_number(&self) -> &str {
        // Lets ignore the last byte, because it is a '\0' character.
        unsafe { str::from_utf8_unchecked(&self.lot[..6]) }
    }
}

/// Size of integrated flash
#[derive(Debug)]
#[repr(C)]
pub struct FlashSize(u16);
define_ptr_type!(FlashSize, 0x1FFF_F7CC);

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
