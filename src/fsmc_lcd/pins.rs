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

use core::marker::PhantomData;

use super::sealed;
use super::{Lcd, SubBank1};
use crate::fsmc_lcd::{SubBank2, SubBank3, SubBank4};

/// A pin that can be used for data bus 0
pub trait PinD0: sealed::Sealed {}
/// A pin that can be used for data bus 1
pub trait PinD1: sealed::Sealed {}
/// A pin that can be used for data bus 2
pub trait PinD2: sealed::Sealed {}
/// A pin that can be used for data bus 3
pub trait PinD3: sealed::Sealed {}
/// A pin that can be used for data bus 4
pub trait PinD4: sealed::Sealed {}
/// A pin that can be used for data bus 5
pub trait PinD5: sealed::Sealed {}
/// A pin that can be used for data bus 6
pub trait PinD6: sealed::Sealed {}
/// A pin that can be used for data bus 7
pub trait PinD7: sealed::Sealed {}
/// A pin that can be used for data bus 8
pub trait PinD8: sealed::Sealed {}
/// A pin that can be used for data bus 9
pub trait PinD9: sealed::Sealed {}
/// A pin that can be used for data bus 10
pub trait PinD10: sealed::Sealed {}
/// A pin that can be used for data bus 11
pub trait PinD11: sealed::Sealed {}
/// A pin that can be used for data bus 12
pub trait PinD12: sealed::Sealed {}
/// A pin that can be used for data bus 13
pub trait PinD13: sealed::Sealed {}
/// A pin that can be used for data bus 14
pub trait PinD14: sealed::Sealed {}
/// A pin that can be used for data bus 15
pub trait PinD15: sealed::Sealed {}

/// A pin that can be used for the output enable (read enable, NOE) signal
pub trait PinReadEnable: sealed::Sealed {}
/// A pin that can be used for the write enable (NOE) signal
pub trait PinWriteEnable: sealed::Sealed {}
/// A pin that can be used as one bit of the memory address
///
/// This is used to switch between data and command mode.
pub trait PinAddress: sealed::Sealed {}

/// A pin that can be used to enable a memory device on sub-bank 1
pub trait PinChipSelect1: sealed::Sealed {}
/// A pin that can be used to enable a memory device on sub-bank 2
pub trait PinChipSelect2: sealed::Sealed {}
/// A pin that can be used to enable a memory device on sub-bank 3
pub trait PinChipSelect3: sealed::Sealed {}
/// A pin that can be used to enable a memory device on sub-bank 4
pub trait PinChipSelect4: sealed::Sealed {}

/// One, two, three, or four address pins
pub trait AddressPins: sealed::Sealed {}

// Implement AddressPins for one address pin and tuples of two, three, and four
impl<A> AddressPins for A where A: PinAddress {}
impl<A1: PinAddress, A2: PinAddress> AddressPins for (A1, A2) {}
impl<A1: PinAddress, A2: PinAddress> sealed::Sealed for (A1, A2) {}
impl<A1: PinAddress, A2: PinAddress, A3: PinAddress> AddressPins for (A1, A2, A3) {}
impl<A1: PinAddress, A2: PinAddress, A3: PinAddress> sealed::Sealed for (A1, A2, A3) {}
impl<A1: PinAddress, A2: PinAddress, A3: PinAddress, A4: PinAddress> AddressPins
    for (A1, A2, A3, A4)
{
}
impl<A1: PinAddress, A2: PinAddress, A3: PinAddress, A4: PinAddress> sealed::Sealed
    for (A1, A2, A3, A4)
{
}

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

/// Wrapper for a pin that implements PinChipSelect1
///
/// This is required to avoid conflicting trait implementations.
pub struct ChipSelect1<P>(pub P);
/// Wrapper for a pin that implements PinChipSelect2
///
/// This is required to avoid conflicting trait implementations.
pub struct ChipSelect2<P>(pub P);
/// Wrapper for a pin that implements PinChipSelect3
///
/// This is required to avoid conflicting trait implementations.
pub struct ChipSelect3<P>(pub P);
/// Wrapper for a pin that implements PinChipSelect4
///
/// This is required to avoid conflicting trait implementations.
pub struct ChipSelect4<P>(pub P);

impl<CS1: PinChipSelect1> ChipSelectPins for ChipSelect1<CS1> {
    type Lcds = Lcd<SubBank1>;
}
impl<CS1: PinChipSelect1> sealed::Sealed for ChipSelect1<CS1> {}
impl<CS2: PinChipSelect2> ChipSelectPins for ChipSelect2<CS2> {
    type Lcds = Lcd<SubBank2>;
}
impl<CS2: PinChipSelect2> sealed::Sealed for ChipSelect2<CS2> {}
impl<CS3: PinChipSelect3> ChipSelectPins for ChipSelect3<CS3> {
    type Lcds = Lcd<SubBank3>;
}
impl<CS3: PinChipSelect3> sealed::Sealed for ChipSelect3<CS3> {}
impl<CS4: PinChipSelect4> ChipSelectPins for ChipSelect4<CS4> {
    type Lcds = Lcd<SubBank4>;
}
impl<CS4: PinChipSelect4> sealed::Sealed for ChipSelect4<CS4> {}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2> ChipSelectPins
    for (ChipSelect1<CS1>, ChipSelect2<CS2>)
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>);
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2> sealed::Sealed
    for (ChipSelect1<CS1>, ChipSelect2<CS2>)
{
}
impl<CS1: PinChipSelect1, CS3: PinChipSelect3> ChipSelectPins
    for (ChipSelect1<CS1>, ChipSelect3<CS3>)
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank3>);
}
impl<CS1: PinChipSelect1, CS3: PinChipSelect3> sealed::Sealed
    for (ChipSelect1<CS1>, ChipSelect3<CS3>)
{
}
impl<CS1: PinChipSelect1, CS4: PinChipSelect4> ChipSelectPins
    for (ChipSelect1<CS1>, ChipSelect4<CS4>)
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank4>);
}
impl<CS1: PinChipSelect1, CS4: PinChipSelect4> sealed::Sealed
    for (ChipSelect1<CS1>, ChipSelect4<CS4>)
{
}
impl<CS2: PinChipSelect2, CS3: PinChipSelect3> ChipSelectPins
    for (ChipSelect2<CS2>, ChipSelect3<CS3>)
{
    type Lcds = (Lcd<SubBank2>, Lcd<SubBank3>);
}
impl<CS2: PinChipSelect2, CS3: PinChipSelect3> sealed::Sealed
    for (ChipSelect2<CS2>, ChipSelect3<CS3>)
{
}
impl<CS2: PinChipSelect2, CS4: PinChipSelect4> ChipSelectPins
    for (ChipSelect2<CS2>, ChipSelect4<CS4>)
{
    type Lcds = (Lcd<SubBank2>, Lcd<SubBank4>);
}
impl<CS2: PinChipSelect2, CS4: PinChipSelect4> sealed::Sealed
    for (ChipSelect2<CS2>, ChipSelect4<CS4>)
{
}
impl<CS3: PinChipSelect3, CS4: PinChipSelect4> ChipSelectPins
    for (ChipSelect3<CS3>, ChipSelect4<CS4>)
{
    type Lcds = (Lcd<SubBank3>, Lcd<SubBank4>);
}
impl<CS3: PinChipSelect3, CS4: PinChipSelect4> sealed::Sealed
    for (ChipSelect3<CS3>, ChipSelect4<CS4>)
{
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2, CS3: PinChipSelect3> ChipSelectPins
    for (ChipSelect1<CS1>, ChipSelect2<CS2>, ChipSelect3<CS3>)
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank3>);
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2, CS3: PinChipSelect3> sealed::Sealed
    for (ChipSelect1<CS1>, ChipSelect2<CS2>, ChipSelect3<CS3>)
{
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2, CS4: PinChipSelect4> ChipSelectPins
    for (ChipSelect1<CS1>, ChipSelect2<CS2>, ChipSelect4<CS4>)
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank4>);
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2, CS4: PinChipSelect4> sealed::Sealed
    for (ChipSelect1<CS1>, ChipSelect2<CS2>, ChipSelect4<CS4>)
{
}
impl<CS1: PinChipSelect1, CS3: PinChipSelect3, CS4: PinChipSelect4> ChipSelectPins
    for (ChipSelect1<CS1>, ChipSelect3<CS3>, ChipSelect4<CS4>)
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank3>, Lcd<SubBank4>);
}
impl<CS1: PinChipSelect1, CS3: PinChipSelect3, CS4: PinChipSelect4> sealed::Sealed
    for (ChipSelect1<CS1>, ChipSelect3<CS3>, ChipSelect4<CS4>)
{
}
impl<CS2: PinChipSelect2, CS3: PinChipSelect3, CS4: PinChipSelect4> ChipSelectPins
    for (ChipSelect2<CS2>, ChipSelect3<CS3>, ChipSelect4<CS4>)
{
    type Lcds = (Lcd<SubBank2>, Lcd<SubBank3>, Lcd<SubBank4>);
}
impl<CS2: PinChipSelect2, CS3: PinChipSelect3, CS4: PinChipSelect4> sealed::Sealed
    for (ChipSelect2<CS2>, ChipSelect3<CS3>, ChipSelect4<CS4>)
{
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2, CS3: PinChipSelect3, CS4: PinChipSelect4>
    ChipSelectPins
    for (
        ChipSelect1<CS1>,
        ChipSelect2<CS2>,
        ChipSelect3<CS3>,
        ChipSelect4<CS4>,
    )
{
    type Lcds = (Lcd<SubBank1>, Lcd<SubBank2>, Lcd<SubBank3>, Lcd<SubBank4>);
}
impl<CS1: PinChipSelect1, CS2: PinChipSelect2, CS3: PinChipSelect3, CS4: PinChipSelect4>
    sealed::Sealed
    for (
        ChipSelect1<CS1>,
        ChipSelect2<CS2>,
        ChipSelect3<CS3>,
        ChipSelect4<CS4>,
    )
{
}

/// A set of data pins
///
/// Currently this trait is only implemented for tuples of 16 data pins. In the future,
/// this driver may support 8-bit mode using 8 data pins.
pub trait DataPins: sealed::Sealed {}

impl<D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15> DataPins
    for (
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
    )
where
    D0: PinD0,
    D1: PinD1,
    D2: PinD2,
    D3: PinD3,
    D4: PinD4,
    D5: PinD5,
    D6: PinD6,
    D7: PinD7,
    D8: PinD8,
    D9: PinD9,
    D10: PinD10,
    D11: PinD11,
    D12: PinD12,
    D13: PinD13,
    D14: PinD14,
    D15: PinD15,
{
}
impl<D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15> sealed::Sealed
    for (
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
    )
where
    D0: PinD0,
    D1: PinD1,
    D2: PinD2,
    D3: PinD3,
    D4: PinD4,
    D5: PinD5,
    D6: PinD6,
    D7: PinD7,
    D8: PinD8,
    D9: PinD9,
    D10: PinD10,
    D11: PinD11,
    D12: PinD12,
    D13: PinD13,
    D14: PinD14,
    D15: PinD15,
{
}

/// A set of pins used to interface with an LCD
///
/// The `address` and `enable` fields can be individual pins, or tuples of 2, 3, or 4 pins.
pub struct LcdPins<D, AD, NOE, NWE, NE> {
    /// The 16-bit data bus
    pub data: D,
    /// Address pin(s) (data/command)
    pub address: AD,
    /// Output enable (read enable)
    pub read_enable: NOE,
    /// Write enable
    pub write_enable: NWE,
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

impl<D, AD, NOE, NWE, NE> Pins for LcdPins<D, AD, NOE, NWE, NE>
where
    D: DataPins,
    AD: AddressPins,
    NOE: PinReadEnable,
    NWE: PinWriteEnable,
    NE: ChipSelectPins,
{
    type Lcds = NE::Lcds;
}

impl<D, AD, NOE, NWE, NE> sealed::Sealed for LcdPins<D, AD, NOE, NWE, NE>
where
    D: DataPins,
    AD: AddressPins,
    NOE: PinReadEnable,
    NWE: PinWriteEnable,
    NE: ChipSelectPins,
{
}

/// Pins available on all STM32F4 models that have an FSMC/FMC
mod common_pins {
    use super::sealed::Sealed;
    use super::{
        PinAddress, PinChipSelect1, PinChipSelect2, PinChipSelect3, PinChipSelect4, PinD0, PinD1,
        PinD10, PinD11, PinD12, PinD13, PinD14, PinD15, PinD2, PinD3, PinD4, PinD5, PinD6, PinD7,
        PinD8, PinD9, PinReadEnable, PinWriteEnable,
    };
    use crate::gpio::{
        Pin, AF12, PD0, PD1, PD10, PD11, PD12, PD13, PD14, PD15, PD4, PD5, PD7, PD8, PD9, PE10,
        PE11, PE12, PE13, PE14, PE15, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9, PF0, PF1, PF12, PF13,
        PF14, PF15, PF2, PF3, PF4, PF5, PG0, PG1, PG10, PG12, PG13, PG2, PG3, PG4, PG5, PG9,
    };

    impl<const P: char, const N: u8, MODE> Sealed for Pin<P, N, MODE> {}

    // All FSMC/FMC pins use AF12

    impl PinD2 for PD0<AF12> {}
    impl PinD3 for PD1<AF12> {}
    impl PinReadEnable for PD4<AF12> {}
    impl PinWriteEnable for PD5<AF12> {}
    impl PinChipSelect1 for PD7<AF12> {}
    impl PinD13 for PD8<AF12> {}
    impl PinD14 for PD9<AF12> {}
    impl PinD15 for PD10<AF12> {}
    impl PinAddress for PD11<AF12> {}
    impl PinAddress for PD12<AF12> {}
    impl PinAddress for PD13<AF12> {}
    impl PinD0 for PD14<AF12> {}
    impl PinD1 for PD15<AF12> {}
    impl PinAddress for PE2<AF12> {}
    impl PinAddress for PE3<AF12> {}
    impl PinAddress for PE4<AF12> {}
    impl PinAddress for PE5<AF12> {}
    impl PinAddress for PE6<AF12> {}
    impl PinD4 for PE7<AF12> {}
    impl PinD5 for PE8<AF12> {}
    impl PinD6 for PE9<AF12> {}
    impl PinD7 for PE10<AF12> {}
    impl PinD8 for PE11<AF12> {}
    impl PinD9 for PE12<AF12> {}
    impl PinD10 for PE13<AF12> {}
    impl PinD11 for PE14<AF12> {}
    impl PinD12 for PE15<AF12> {}

    impl PinAddress for PF0<AF12> {}
    impl PinAddress for PF1<AF12> {}
    impl PinAddress for PF2<AF12> {}
    impl PinAddress for PF3<AF12> {}
    impl PinAddress for PF4<AF12> {}
    impl PinAddress for PF5<AF12> {}
    impl PinAddress for PF12<AF12> {}
    impl PinAddress for PF13<AF12> {}
    impl PinAddress for PF14<AF12> {}
    impl PinAddress for PF15<AF12> {}
    impl PinAddress for PG0<AF12> {}
    impl PinAddress for PG1<AF12> {}
    impl PinAddress for PG2<AF12> {}
    impl PinAddress for PG3<AF12> {}
    impl PinAddress for PG4<AF12> {}
    impl PinAddress for PG5<AF12> {}
    impl PinChipSelect2 for PG9<AF12> {}
    impl PinChipSelect3 for PG10<AF12> {}
    impl PinChipSelect4 for PG12<AF12> {}
    impl PinAddress for PG13<AF12> {}
    // PG14<Alternate<12> can be used as address 25 (A25), but that pin is not available here.
    // Because external addresses are in units of 16 bits, external address line 25 can never
    // be high. The internal memory address would overflow into the next sub-bank.
}

/// Additional pins available on some models
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
mod extra_pins {
    use super::{
        PinAddress, PinChipSelect4, PinD0, PinD1, PinD13, PinD2, PinD3, PinD4, PinD5, PinD6, PinD7,
        PinReadEnable, PinWriteEnable,
    };
    use crate::gpio::{
        AF10, AF12, PA2, PA3, PA4, PA5, PB12, PB14, PC11, PC12, PC2, PC3, PC4, PC5, PC6, PD2,
    };

    // Most FSMC/FMC pins use AF12, but a few use AF10

    impl PinD4 for PA2<AF12> {}
    impl PinD5 for PA3<AF12> {}
    impl PinD6 for PA4<AF12> {}
    impl PinD7 for PA5<AF12> {}
    impl PinD13 for PB12<AF12> {}
    impl PinD0 for PB14<AF10> {}
    impl PinWriteEnable for PC2<AF12> {}
    impl PinAddress for PC3<AF12> {}
    impl PinChipSelect4 for PC4<AF12> {}
    impl PinReadEnable for PC5<AF12> {}
    impl PinD1 for PC6<AF10> {}
    impl PinD2 for PC11<AF10> {}
    impl PinD3 for PC12<AF10> {}
    impl PinWriteEnable for PD2<AF10> {}
}
