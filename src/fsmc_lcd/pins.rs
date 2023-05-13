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

use crate::gpio::alt::fsmc as alt;

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

macro_rules! conjure {
    ($($($sb:ident),+;)+) => {
        $(
            #[allow(unused_parens)]
            impl sealed::Conjure for ($(Lcd<$sb>),+) {
                fn conjure() -> Self {
                    ($(Lcd::<$sb>::new()),+)
                }
            }
        )+
    };
}

// Implement Conjure for all non-empty subsets of Lcds
conjure! {
    SubBank1;
    SubBank2;
    SubBank3;
    SubBank4;
    SubBank1, SubBank2;
    SubBank1, SubBank3;
    SubBank1, SubBank4;
    SubBank2, SubBank3;
    SubBank2, SubBank4;
    SubBank3, SubBank4;
    SubBank1, SubBank2, SubBank3;
    SubBank1, SubBank2, SubBank4;
    SubBank1, SubBank3, SubBank4;
    SubBank2, SubBank3, SubBank4;
    SubBank1, SubBank2, SubBank3, SubBank4;
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

macro_rules! chipselect {
    ($($([$sb:ident, $Ne:ident, $i:tt]),+;)+) => {
        $(
            impl ChipSelectPins for ($(alt::$Ne),+) {
                type Lcds = ($(Lcd<$sb>),+);
            }
            impl sealed::Sealed for ($(alt::$Ne),+) {}
        )+
    };
}

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
chipselect! {
    [SubBank1, Ne1, 0], [SubBank2, Ne2, 1];
    [SubBank1, Ne1, 0], [SubBank3, Ne3, 1];
    [SubBank1, Ne1, 0], [SubBank4, Ne4, 1];
    [SubBank2, Ne2, 0], [SubBank3, Ne3, 1];
    [SubBank2, Ne2, 0], [SubBank4, Ne4, 1];
    [SubBank3, Ne3, 0], [SubBank4, Ne4, 1];
    [SubBank1, Ne1, 0], [SubBank2, Ne2, 1], [SubBank3, Ne3, 2];
    [SubBank1, Ne1, 0], [SubBank2, Ne2, 1], [SubBank4, Ne4, 2];
    [SubBank1, Ne1, 0], [SubBank3, Ne3, 1], [SubBank4, Ne4, 2];
    [SubBank2, Ne2, 0], [SubBank3, Ne3, 1], [SubBank4, Ne4, 2];
    [SubBank1, Ne1, 0], [SubBank2, Ne2, 1], [SubBank3, Ne3, 2], [SubBank4, Ne4, 3];
}

/// A set of data pins
///
/// Currently this trait is only implemented for tuples of 16 data pins. In the future,
/// this driver may support 8-bit mode using 8 data pins.
pub trait DataPins: sealed::Sealed {}

#[allow(unused)]
pub struct DataPins16 {
    d0: alt::D0,
    d1: alt::D1,
    d2: alt::D2,
    d3: alt::D3,
    d4: alt::D4,
    d5: alt::D5,
    d6: alt::D6,
    d7: alt::D7,
    d8: alt::D8,
    d9: alt::D9,
    d10: alt::D10,
    d11: alt::D11,
    d12: alt::D12,
    d13: alt::D13,
    d14: alt::D14,
    d15: alt::D15,
}

impl DataPins for DataPins16 {}

impl DataPins16 {
    #[inline(always)]
    pub fn new(
        d0: impl Into<alt::D0>,
        d1: impl Into<alt::D1>,
        d2: impl Into<alt::D2>,
        d3: impl Into<alt::D3>,
        d4: impl Into<alt::D4>,
        d5: impl Into<alt::D5>,
        d6: impl Into<alt::D6>,
        d7: impl Into<alt::D7>,
        d8: impl Into<alt::D8>,
        d9: impl Into<alt::D9>,
        d10: impl Into<alt::D10>,
        d11: impl Into<alt::D11>,
        d12: impl Into<alt::D12>,
        d13: impl Into<alt::D13>,
        d14: impl Into<alt::D14>,
        d15: impl Into<alt::D15>,
    ) -> Self {
        Self {
            d0: d0.into(),
            d1: d1.into(),
            d2: d2.into(),
            d3: d3.into(),
            d4: d4.into(),
            d5: d5.into(),
            d6: d6.into(),
            d7: d7.into(),
            d8: d8.into(),
            d9: d9.into(),
            d10: d10.into(),
            d11: d11.into(),
            d12: d12.into(),
            d13: d13.into(),
            d14: d14.into(),
            d15: d15.into(),
        }
    }
}
impl sealed::Sealed for DataPins16 {}

/// A set of pins used to interface with an LCD
///
/// The `address` and `enable` fields can be individual pins, or tuples of 2, 3, or 4 pins.
#[allow(unused)]
pub struct LcdPins<D, AD, NE> {
    /// The 16-bit data bus
    data: D,
    /// Address pin(s) (data/command)
    address: AD,
    /// Output enable (read enable)
    read_enable: alt::Noe,
    /// Write enable
    write_enable: alt::Nwe,
    /// Chip select / bank enable pin(s)
    chip_select: NE,
}

impl<D, AD, NE> LcdPins<D, AD, NE>
where
    D: DataPins,
    AD: AddressPins,
    NE: ChipSelectPins,
{
    pub fn new(
        data: D,
        address: AD,
        read_enable: impl Into<alt::Noe>,
        write_enable: impl Into<alt::Nwe>,
        chip_select: NE,
    ) -> Self {
        Self {
            data,
            address,
            read_enable: read_enable.into(),
            write_enable: write_enable.into(),
            chip_select,
        }
    }
    pub fn split(self) -> (D, AD, alt::Noe, alt::Nwe, NE) {
        (
            self.data,
            self.address,
            self.read_enable,
            self.write_enable,
            self.chip_select,
        )
    }
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
