//! Pin definitions for the Flexible Static Memory Controller / Flexible Memory Controller
//!
//! Note: This file only includes pins for these functions:
//! * NOE (read enable)
//! * NWE (write enable)
//! * NEx (chip select)
//! * Ax (address)
//! * Dx (data 0 through 15)
//!
//! # Naming conventions
//!
//! For signal names, this module uses:
//! * Chip select instead of enable
//! * Address instead of data/command
//! * Read enable instead of output enable
//! * Write enable

use crate::gpio::alt::fmc as alt;
use core::marker::PhantomData;

use super::sealed;
use super::{Lcd, SubBank1};
use crate::fsmc_lcd::{SubBank2, SubBank3, SubBank4};

/// One, two, three, or four address pins
pub trait AddressPins: sealed::Sealed {}

// Implement AddressPins for one address pin and tuples of two, three, and four
impl AddressPins for alt::Address {}
impl AddressPins for (alt::Address, alt::Address) {}
impl sealed::Sealed for (alt::Address, alt::Address) {}
impl AddressPins for (alt::Address, alt::Address, alt::Address) {}
impl sealed::Sealed for (alt::Address, alt::Address, alt::Address) {}
impl AddressPins for (alt::Address, alt::Address, alt::Address, alt::Address) {}
impl sealed::Sealed for (alt::Address, alt::Address, alt::Address, alt::Address) {}

// Implement Conjure for all non-empty subsets of Lcds
impl sealed::Conjure for Lcd<SubBank1> {
    fn conjure() -> Self {
        Lcd {
            _sub_bank: PhantomData,
        }
    }
}
impl sealed::Conjure for Lcd<SubBank2> {
    fn conjure() -> Self {
        Lcd {
            _sub_bank: PhantomData,
        }
    }
}
impl sealed::Conjure for Lcd<SubBank3> {
    fn conjure() -> Self {
        Lcd {
            _sub_bank: PhantomData,
        }
    }
}
impl sealed::Conjure for Lcd<SubBank4> {
    fn conjure() -> Self {
        Lcd {
            _sub_bank: PhantomData,
        }
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank2>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank3>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank2>, Lcd<SubBank3>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank2>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank3>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank3>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank3>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank2>, Lcd<SubBank3>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}
impl sealed::Conjure for (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank3>, Lcd<SubBank4>) {
    fn conjure() -> Self {
        (
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
            Lcd {
                _sub_bank: PhantomData,
            },
        )
    }
}

/// One, two, three, or four chip select pins
///
/// Due to trait system limitations, this trait is only implemented for pins wrapped in the
/// `ChipSelect1`, `ChipSelect2`, `ChipSelect3`, and `ChipSelect4` wrappers.
///
/// This trait is implemented for all non-empty subsets of the 4 possible chip select signals.
/// The pins must be in order.
///
/// # Example types that implement `ChipSelectPins`
///
/// Wrapped single pins:
/// * `ChipSelect1<PD7<Alternate<12>>>`
/// * `ChipSelect2<PG9<Alternate<12>>>`
/// * `ChipSelect3<PG10<Alternate<12>>>`
/// * `ChipSelect4<PG12<Alternate<12>>>`
///
/// Tuples of wrapped pins:
/// * `(ChipSelect1<PD7<Alternate<12>>>, ChipSelect2<PG9<Alternate<12>>>)`
/// * `(ChipSelect1<PD7<Alternate<12>>>, ChipSelect4<PG4<Alternate<12>>>)`
/// * `(ChipSelect1<PD7<Alternate<12>>>, ChipSelect2<PG9<Alternate<12>>>, ChipSelect3<PG10<Alternate<12>>>, ChipSelect4<PG12<Alternate<12>>>)`
pub trait ChipSelectPins: sealed::Sealed {
    /// One, two, three, or four `Lcd<_>` objects associated with the sub-bank(s) that these pin(s)
    /// control
    type Lcds: sealed::Conjure;
}

// The set of 4 chip selects has 15 subsets (excluding the empty set):
// 1
// 2
// 3
// 4
// 1, 2
// 1, 3
// 1, 4
// 2, 3
// 2, 4
// 3, 4
// 1, 2, 3
// 1, 2, 4
// 1, 3, 4
// 2, 3, 4
// 1, 2, 3, 4

impl ChipSelectPins for alt::Ne1 {
    type Lcds = Lcd<SubBank1>;
}
impl ChipSelectPins for alt::Ne2 {
    type Lcds = Lcd<SubBank2>;
}
impl ChipSelectPins for alt::Ne3 {
    type Lcds = Lcd<SubBank3>;
}
impl ChipSelectPins for alt::Ne4 {
    type Lcds = Lcd<SubBank4>;
}
impl ChipSelectPins for (alt::Ne1, alt::Ne2) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne2) {}
impl ChipSelectPins for (alt::Ne1, alt::Ne3) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank3>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne3) {}
impl ChipSelectPins for (alt::Ne1, alt::Ne4) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne4) {}
impl ChipSelectPins for (alt::Ne2, alt::Ne3) {
    type Lcds = (Lcd<SubBank2>, Lcd<SubBank3>);
}
impl sealed::Sealed for (alt::Ne2, alt::Ne3) {}
impl ChipSelectPins for (alt::Ne2, alt::Ne4) {
    type Lcds = (Lcd<SubBank2>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne2, alt::Ne4) {}
impl ChipSelectPins for (alt::Ne3, alt::Ne4) {
    type Lcds = (Lcd<SubBank3>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne3, alt::Ne4) {}
impl ChipSelectPins for (alt::Ne1, alt::Ne2, alt::Ne3) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank3>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne2, alt::Ne3) {}
impl ChipSelectPins for (alt::Ne1, alt::Ne2, alt::Ne4) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne2, alt::Ne4) {}
impl ChipSelectPins for (alt::Ne1, alt::Ne3, alt::Ne4) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank3>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne3, alt::Ne4) {}
impl ChipSelectPins for (alt::Ne2, alt::Ne3, alt::Ne4) {
    type Lcds = (Lcd<SubBank2>, Lcd<SubBank3>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne2, alt::Ne3, alt::Ne4) {}
impl ChipSelectPins for (alt::Ne1, alt::Ne2, alt::Ne3, alt::Ne4) {
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank3>, Lcd<SubBank4>);
}
impl sealed::Sealed for (alt::Ne1, alt::Ne2, alt::Ne3, alt::Ne4) {}

/// A set of data pins
///
/// Currently this trait is only implemented for tuples of 16 data pins. In the future,
/// this driver may support 8-bit mode using 8 data pins.
pub trait DataPins: sealed::Sealed {}

impl DataPins
    for (
        alt::D0,
        alt::D1,
        alt::D2,
        alt::D3,
        alt::D4,
        alt::D5,
        alt::D6,
        alt::D7,
        alt::D8,
        alt::D9,
        alt::D10,
        alt::D11,
        alt::D12,
        alt::D13,
        alt::D14,
        alt::D15,
    )
{
}
impl sealed::Sealed
    for (
        alt::D0,
        alt::D1,
        alt::D2,
        alt::D3,
        alt::D4,
        alt::D5,
        alt::D6,
        alt::D7,
        alt::D8,
        alt::D9,
        alt::D10,
        alt::D11,
        alt::D12,
        alt::D13,
        alt::D14,
        alt::D15,
    )
{
}

/// A set of pins used to interface with an LCD
///
/// The `address` and `enable` fields can be individual pins, or tuples of 2, 3, or 4 pins.
pub struct LcdPins<D, AD, NE> {
    /// The 16-bit data bus
    pub data: D,
    /// Address pin(s) (data/command)
    pub address: AD,
    /// Output enable (read enable)
    pub read_enable: alt::Noe,
    /// Write enable
    pub write_enable: alt::Nwe,
    /// Chip select / bank enable pin(s)
    pub chip_select: NE,
}

/// A set of pins that can be used with the FSMC
///
/// This trait is implemented for the `LcdPins` struct that contains 16 data pins, 1 through 4
/// address pins, 1 through 4 chip select / bank enable pins, an output enable pin, and a write
/// enable pin.
pub trait Pins: sealed::Sealed {
    /// One, two, three, or four `Lcd<_>` objects associated with the sub-bank(s) that the chip
    /// select pin pin(s) control
    type Lcds: sealed::Conjure;
}

impl<D, AD, NE> Pins for LcdPins<D, AD, NE>
where
    D: DataPins,
    AD: AddressPins,
    NE: ChipSelectPins,
{
    type Lcds = NE::Lcds;
}

impl<D, AD, NE> sealed::Sealed for LcdPins<D, AD, NE>
where
    D: DataPins,
    AD: AddressPins,
    NE: ChipSelectPins,
{
}
