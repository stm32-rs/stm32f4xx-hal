//! Device electronic signature
//!
//! (stored in flash memory)

use core::str::from_utf8_unchecked;

/// The VDDA in millivolts used at the factory during calibration
pub const VDDA_CALIB: u32 = 3300;

/// The nominal value of the VREFINT sampled at the factory during calibration
pub const VREFINT_CALIB: u32 = (1210 * 4095) / VDDA_CALIB;

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
        self.waf_lot[0]
    }

    /// Lot number
    pub fn lot_num(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.waf_lot[1..]) }
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

/// ADC VREF calibration value is stored in at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VrefCal(u16);
define_ptr_type!(VrefCal, 0x1FFF_7A2A);

impl VrefCal {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}

/// A temperature reading taken at 30°C stored at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VtempCal30(u16);
define_ptr_type!(VtempCal30, 0x1FFF_7A2C);

impl VtempCal30 {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}

/// A temperature reading taken at 110°C stored at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VtempCal110(u16);
define_ptr_type!(VtempCal110, 0x1FFF_7A2E);

impl VtempCal110 {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}
