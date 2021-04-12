/// Private implementation details used in the fsmc_lcd module and the pins submodule

pub trait Sealed {}

/// Private supertrait of SubBank
pub trait SealedSubBank {
    /// The address of the beginning of this sub-bank's address space
    const BASE_ADDRESS: usize;
    /// The address in memory used to communicate with the LCD controller with the data/command
    /// signal set to command (low)
    const COMMAND_ADDRESS: usize = Self::BASE_ADDRESS;
    /// The address in memory used to communicate with the LCD controller with the data/command
    /// signal set to data (high)
    const DATA_ADDRESS: usize = make_data_address(Self::BASE_ADDRESS);
}

/// A trait similar to Default, but private to this crate
///
/// This is used to create `Lcd` objects and tuples of `Lcd`s.
pub trait Conjure {
    /// Creates something out of thin air
    fn conjure() -> Self;
}

/// Converts a command address into a data address
///
/// The data address will result in all external address signals being set high.
const fn make_data_address(base: usize) -> usize {
    // Bits 26 and 27 select the sub-bank, don't change them.
    // Bits 25 through 1 become address signals 24 through 0, set these high.
    // Bit 0 is not used with 16-bit addressing.
    base | 0x3fffffe
}
