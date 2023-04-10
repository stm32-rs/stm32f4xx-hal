//!
//! LCD interface using the Flexible Memory Controller (FMC) / Flexible Static Memory Controller (FSMC)
//!
//! This module is only available if the `fsmc_lcd` feature is enabled and the target
//! microcontroller has an FMC or FSMC
//!
//! This driver is compatible with many LCD driver chips that support the Intel 8080 interface
//! (also called Display Bus Interface (DBI) type B) with a 16-bit data bus.
//!
//! Here are some examples of compatible LCD drivers:
//! * Sitronix ST7735S
//! * Sitronix ST7789VI
//! * Ilitek ILI9327
//! * Ilitek ILI9320
//! * Himax HX8357-B
//!
//! Higher-level driver code can add support for specific LCD driver
//! integrated circuits.
//!
//! For an overview of how this interface works, see [application note AN2790](https://www.st.com/content/ccc/resource/technical/document/application_note/85/ad/ef/0f/a3/a6/49/9a/CD00201397.pdf/files/CD00201397.pdf/jcr:content/translations/en.CD00201397.pdf).
//!
//! # Pins
//!
//! To interface with 1-4 LCDs, you will need:
//! * 16 data pins (0 through 15), shared among all the LCDs
//! * One NEx (chip select) pin per LCD (up to 4)
//! * One Ax (address) pin, shared among all the LCDs. This pin is used to select data or command
//!   mode.
//!     * You can also supply up to 4 address pins, but they will always have the same output.
//!     * Caution: Due to hardware limitations, address line A25 cannot be used.
//! * One NOE (read enable) pin, shared among all the LCDs
//! * One NWE (write enable) pin, shared among all the LCDs
//!
//! # Timing
//!
//! Because the correct timing depends on the specific LCD controller and the wiring between the
//! microcontroller and LCD controller, this driver does not try to calculate the correct
//! timing settings. Instead, it exposes the access modes and timing options that the STM32F4
//! hardware supports.
//!
//! The default access mode is mode C. For an example timing diagram, refer to reference manual
//! [RM0090](https://www.st.com/resource/en/reference_manual/dm00031020.pdf),
//! figures 443 and 444 (on page 1562), or your microcontroller reference manual.
//!
//! Access modes A, B, and D are also supported.
//!
//! # Basic operation
//!
//! 1. Create an `LcdPins` object containing the pins used to communicate with the LCD
//!
//! 2. Create default `Timing` objects for the write and read timing
//!     
//!     a. (Optional) Adjust the timing to make read and write operations faster, within the limits
//!        of the wiring and LCD controller
//!
//! 3. Pass the FSMC peripheral object, pins, read timing, and write timing to `FsmcLcd::new`.
//!    This function will return an `FsmcLcd` and one or more `Lcd` objects.
//!
//! 4. Use the returned `Lcd` object(s) to configure the controller(s) and display graphics

mod display_interface_impl;
mod pins;
mod sealed;
mod timing;

use core::marker::PhantomData;

pub use self::pins::{AddressPins, ChipSelectPins, DataPins, LcdPins, Pins};
pub use self::timing::{AccessMode, Timing};

use crate::pac::RCC;
use crate::rcc::{Enable, Reset};

// Use the FMC or FSMC, whichever is available, and treat it like an FSMC
#[cfg(feature = "fmc")]
use crate::pac::fmc as fsmc;
#[cfg(feature = "fsmc")]
use crate::pac::fsmc;
#[cfg(feature = "fmc")]
use crate::pac::FMC as FSMC;
#[cfg(feature = "fsmc")]
use crate::pac::FSMC;

/// A sub-bank of bank 1, with its own chip select output
pub trait SubBank: sealed::SealedSubBank {}
/// Sub-bank 1
pub struct SubBank1(());
impl sealed::SealedSubBank for SubBank1 {
    const BASE_ADDRESS: usize = 0x6000_0000;
}
impl SubBank for SubBank1 {}
/// Sub-bank 2
pub struct SubBank2(());
impl sealed::SealedSubBank for SubBank2 {
    const BASE_ADDRESS: usize = 0x6400_0000;
}
impl SubBank for SubBank2 {}
/// Sub-bank 3
pub struct SubBank3(());
impl sealed::SealedSubBank for SubBank3 {
    const BASE_ADDRESS: usize = 0x6800_0000;
}
impl SubBank for SubBank3 {}
/// Sub-bank 4
pub struct SubBank4(());
impl sealed::SealedSubBank for SubBank4 {
    const BASE_ADDRESS: usize = 0x6c00_0000;
}
impl SubBank for SubBank4 {}

/// An FMC or FSMC configured as an LCD interface
pub struct FsmcLcd<PINS> {
    pins: PINS,
    fsmc: FSMC,
}

impl<PINS> FsmcLcd<PINS>
where
    PINS: Pins,
{
    /// Configures the FSMC/FMC to interface with an LCD using the provided pins
    ///
    /// The same timings will be used for all connected LCDs.
    ///
    /// The return type includes an `FsmcLcd` and either an `Lcd` or a tuple of up to four `Lcd`s,
    /// depending on the pins given.
    ///
    /// The returned `FsmcLcd` can be used later to release the FSMC and pins for other uses,
    /// or it can be ignored.
    ///
    /// # Return type examples
    ///
    /// ## One enable/chip select pin
    ///
    /// If you pass an `LcdPins` object with the `enable` field containing a single `ChipSelect1`
    /// object with a pin, this function will return an `FsmcLcd` and an `Lcd<SubBank1>`.
    ///
    /// ### Multiple enable/chip select pins
    ///
    /// If you pass an `LcdPins` object with the `enable` field containing a tuple of 2-4
    /// `ChipSelectX` objects, this function will return an `FsmcLcd` and a tuple of `Lcd<_>`
    /// objects. Each `Lcd` is associated with one chip select pin, and can be controlled
    /// independently.
    ///
    /// # Examples
    ///
    /// The `stm32f4xx-hal` repository has a `st7789-lcd` example that draws graphics on an LCD
    /// and runs on the STM32F412G-DISCO board.
    ///
    /// Up to four LCDs can be controlled separately using four chip select pins
    ///
    /// ```ignore
    /// let lcd_pins = LcdPins {
    ///     data: (
    ///         gpiod.pd14.into(),
    ///         gpiod.pd15.into(),
    ///         gpiod.pd0.into(),
    ///         gpiod.pd1.into(),
    ///         gpioe.pe7.into(),
    ///         gpioe.pe8.into(),
    ///         gpioe.pe9.into(),
    ///         gpioe.pe10.into(),
    ///         gpioe.pe11.into(),
    ///         gpioe.pe12.into(),
    ///         gpioe.pe13.into(),
    ///         gpioe.pe14.into(),
    ///         gpioe.pe15.into(),
    ///         gpiod.pd8.into(),
    ///         gpiod.pd9.into(),
    ///         gpiod.pd10.into(),
    ///     ),
    ///     // Four address pins, one for each LCD
    ///     // All of them will have the same output
    ///     address: (
    ///         gpiof.pf0.into(),
    ///         gpioe.pe2.into(),
    ///         gpioe.pe3.into(),
    ///         gpiof.pf14.into(),
    ///     ),
    ///     read_enable: gpiod.pd4.into(),
    ///     write_enable: gpiod.pd5.into(),
    ///     // Four chip select pins, one for each LCD, controlled independently
    ///     chip_select: (
    ///         ChipSelect1::from(gpiod.pd7),
    ///         ChipSelect2::from(gpiog.pg9),
    ///         ChipSelect3::from(gpiog.pg10),
    ///         ChipSelect4::from(gpiog.pg12),
    ///     ),
    /// };
    ///
    /// let (_fsmc, mut lcds) = FsmcLcd::new(dp.FSMC, lcd_pins, &Timing::default(), &Timing::default());
    /// // lcds is a tuple of four `Lcd` objects. Each one can be accessed independently.
    /// // This is just a basic example of some things that can be done.
    /// lcds.0.write_command(37);
    /// lcds.1.write_command(38);
    /// lcds.2.write_command(39);
    /// lcds.3.write_command(40);
    /// ```
    pub fn new(
        fsmc: FSMC,
        pins: PINS,
        read_timing: &Timing,
        write_timing: &Timing,
    ) -> (Self, PINS::Lcds) {
        use self::sealed::Conjure;
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable the FSMC/FMC peripheral
            FSMC::enable(rcc);
            FSMC::reset(rcc);
        }

        // Configure memory type and basic interface settings
        // The reference manuals are sometimes unclear on the distinction between banks
        // and sub-banks of bank 1. This driver uses addresses in the different sub-banks of
        // bank 1. The configuration registers for "bank x" (like FMC_BCRx) actually refer to
        // sub-banks, not banks. We need to configure and enable all four of them.
        configure_bcr1(&fsmc.bcr1);
        configure_bcr(&fsmc.bcr2);
        configure_bcr(&fsmc.bcr3);
        configure_bcr(&fsmc.bcr4);
        configure_btr(&fsmc.btr1, read_timing);
        configure_btr(&fsmc.btr2, read_timing);
        configure_btr(&fsmc.btr3, read_timing);
        configure_btr(&fsmc.btr4, read_timing);
        configure_bwtr(&fsmc.bwtr1, write_timing);
        configure_bwtr(&fsmc.bwtr2, write_timing);
        configure_bwtr(&fsmc.bwtr3, write_timing);
        configure_bwtr(&fsmc.bwtr4, write_timing);

        (FsmcLcd { pins, fsmc }, PINS::Lcds::conjure())
    }

    /// Reunites this FsmcLcd and all its associated LCDs, and returns the FSMC and pins for other
    /// uses
    ///
    /// This function also resets and disables the FSMC.
    pub fn release(self, _lcds: PINS::Lcds) -> (FSMC, PINS) {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // All STM32F4 models with an FSMC or FMC use bit 0 in AHB3ENR and AHB3RSTR.
            // Reset FSMC/FMC
            FSMC::reset(rcc);
            // Disable the FSMC/FMC peripheral
            FSMC::disable(rcc);
        }

        (self.fsmc, self.pins)
    }
}

/// Configures an SRAM/NOR-Flash chip-select control register for LCD interface use
fn configure_bcr1(bcr: &fsmc::BCR1) {
    bcr.write(|w| {
        w
            // The write fifo and WFDIS bit are missing from some models.
            // Where present, the FIFO is enabled by default.
            // ------------
            // Disable synchronous writes
            .cburstrw()
            .disabled()
            // Don't split burst transactions (doesn't matter for LCD mode)
            .cpsize()
            .no_burst_split()
            // Ignore wait signal (asynchronous mode)
            .asyncwait()
            .disabled()
            // Enable extended mode, for different read and write timings
            .extmod()
            .enabled()
            // Ignore wait signal (synchronous mode)
            .waiten()
            .disabled()
            // Allow write operations
            .wren()
            .enabled()
            // Default wait timing
            .waitcfg()
            .before_wait_state()
            // Default wait polarity
            .waitpol()
            .active_low()
            // Disable burst reads
            .bursten()
            .disabled()
            // Enable NOR flash operations
            .faccen()
            .enabled()
            // 16-bit bus width
            .mwid()
            .bits16()
            // NOR flash mode (compatible with LCD controllers)
            .mtyp()
            .flash()
            // Address and data not multiplexed
            .muxen()
            .disabled()
            // Enable this memory bank
            .mbken()
            .enabled()
    })
}

/// Configures an SRAM/NOR-Flash chip-select control register for LCD interface use
///
/// This is equivalent to `configure_bcr1`, but without the `WFDIS` and `CCLKEN` bits that are
/// present in BCR1 only.
fn configure_bcr(bcr: &fsmc::BCR) {
    bcr.write(|w| {
        w
            // Disable synchronous writes
            .cburstrw()
            .disabled()
            // Don't split burst transactions (doesn't matter for LCD mode)
            .cpsize()
            .no_burst_split()
            // Ignore wait signal (asynchronous mode)
            .asyncwait()
            .disabled()
            // Enable extended mode, for different read and write timings
            .extmod()
            .enabled()
            // Ignore wait signal (synchronous mode)
            .waiten()
            .disabled()
            // Allow write operations
            .wren()
            .enabled()
            // Default wait timing
            .waitcfg()
            .before_wait_state()
            // Default wait polarity
            .waitpol()
            .active_low()
            // Disable burst reads
            .bursten()
            .disabled()
            // Enable NOR flash operations
            .faccen()
            .enabled()
            // 16-bit bus width
            .mwid()
            .bits16()
            // NOR flash mode (compatible with LCD controllers)
            .mtyp()
            .flash()
            // Address and data not multiplexed
            .muxen()
            .disabled()
            // Enable this memory bank
            .mbken()
            .enabled()
    })
}

/// Configures a read timing register
fn configure_btr(btr: &fsmc::BTR, read_timing: &Timing) {
    btr.write(|w| unsafe {
        w.accmod()
            .variant(read_timing.access_mode.as_read_variant())
            .busturn()
            .bits(read_timing.bus_turnaround)
            .datast()
            .bits(read_timing.data)
            .addhld()
            .bits(read_timing.address_hold)
            .addset()
            .bits(read_timing.address_setup)
    })
}
/// Configures a write timing register
fn configure_bwtr(bwtr: &fsmc::BWTR, write_timing: &Timing) {
    bwtr.write(|w| unsafe {
        w.accmod()
            .variant(write_timing.access_mode.as_write_variant())
            .busturn()
            .bits(write_timing.bus_turnaround)
            .datast()
            .bits(write_timing.data)
            .addhld()
            .bits(write_timing.address_hold)
            .addset()
            .bits(write_timing.address_setup)
    })
}

/// An interface to an LCD controller using one sub-bank
///
/// This struct provides low-level read and write commands that can be used to implement
/// drivers for LCD controllers. Each function corresponds to exactly one transaction on the bus.
pub struct Lcd<S> {
    /// Phantom S
    ///
    /// S determines the chip select signal to use, and the addresses used with that signal.
    _sub_bank: PhantomData<S>,
}

impl<S> Lcd<S>
where
    S: SubBank,
{
    /// Writes a value with the data/command (address) signals set high
    pub fn write_data(&mut self, value: u16) {
        unsafe {
            core::ptr::write_volatile(S::DATA_ADDRESS as *mut u16, value);
        }
    }

    /// Writes a value with the data/command (address) signals set low
    pub fn write_command(&mut self, value: u16) {
        unsafe {
            core::ptr::write_volatile(S::COMMAND_ADDRESS as *mut u16, value);
        }
    }

    /// Reads a value with the data/command (address) signals set high
    pub fn read_data(&self) -> u16 {
        unsafe { core::ptr::read_volatile(S::DATA_ADDRESS as *const u16) }
    }

    /// Reads a value with the data/command (address) signals set low
    pub fn read_command(&self) -> u16 {
        unsafe { core::ptr::read_volatile(S::COMMAND_ADDRESS as *const u16) }
    }
}
