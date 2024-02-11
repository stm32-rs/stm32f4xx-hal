//! Flash memory

use crate::pac::FLASH;
use nb::block;

/// Base address of flash memory on AXIM interface.
const FLASH_BASE: *mut u8 = 0x800_0000 as *mut u8;

/// The last valid flash address in any STM32F7 device
const MAX_FLASH_ADDRESS: *mut u8 = 0x81F_FFFF as *mut u8;

/// Flash programming error.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Busy,
    Locked,
    EraseSequence,
    ProgrammingParallelism,
    ProgrammingAlignment,
    WriteProtection,
}

/// Embedded flash memory.
pub struct Flash {
    registers: FLASH,
}

impl Flash {
    /// Creates a new Flash instance.
    pub fn new(flash: FLASH) -> Self {
        Self { registers: flash }
    }

    /// Unlocks the flash memory.
    pub fn unlock(&mut self) {
        if !self.is_locked() {
            // don't try to unlock the flash if it's already unlocked, because
            // trying to unlock the flash twice causes a HardFault
            return;
        }

        self.registers.keyr.write(|w| w.key().bits(0x45670123));
        self.registers.keyr.write(|w| w.key().bits(0xCDEF89AB));
    }

    /// Locks the flash memory.
    pub fn lock(&mut self) {
        self.registers.cr.modify(|_, w| w.lock().set_bit());
    }

    /// Returns `true` if the flash memory is locked.
    fn is_locked(&self) -> bool {
        self.registers.cr.read().lock().is_locked()
    }

    /// Returns `true` if a flash operation is in progress.
    fn is_busy(&self) -> bool {
        self.registers.sr.read().bsy().bit_is_set()
    }

    /// Starts a sector erase sequence.
    ///
    /// The returned `EraseSequence` object can be used to wait for the completion of the
    /// erase sequence by blocking on the `wait` method.
    pub fn erase_sector(&mut self, sector_number: u8) -> Result<EraseSequence<'_>, Error> {
        EraseSequence::new_erase_sector(self, sector_number)
    }

    /// Erases a flash sector.
    ///
    /// This method blocks until the sector is erased or an error occurred.
    pub fn blocking_erase_sector(&mut self, sector_number: u8) -> Result<(), Error> {
        let mut sequence = self.erase_sector(sector_number)?;
        block!(sequence.wait())
    }

    /// Starts a mass erases of the flash memory.
    ///
    /// The returned `EraseSequence` object can be used to wait for the completion of the
    /// erase sequence by blocking on the `wait` method.
    pub fn mass_erase(&mut self) -> Result<EraseSequence<'_>, Error> {
        EraseSequence::new_mass_erase(self)
    }

    /// Mass erases the flash memory.
    ///
    /// This method blocks until the flash is erased or an error occurred.
    pub fn blocking_mass_erase(&mut self) -> Result<(), Error> {
        let mut sequence = self.mass_erase()?;
        block!(sequence.wait())
    }

    /// Starts a programming sequence.
    ///
    /// Note that you must block on the `wait` method in the returned `ProgrammingSequence` object
    /// in order to program all bytes.
    pub fn program<'a, 'b>(
        &'a mut self,
        start_offset: usize,
        data: &'b [u8],
    ) -> Result<ProgrammingSequence<'a, 'b>, Error> {
        ProgrammingSequence::new(self, start_offset, data)
    }

    /// Programs a block of flash memory.
    ///
    /// This method blocks until the block is programed or an error occurred.
    pub fn blocking_program(&mut self, start_offset: usize, data: &[u8]) -> Result<(), Error> {
        let mut sequence = self.program(start_offset, data)?;
        block!(sequence.wait())
    }

    /// Releases the flash peripheral.
    pub fn free(self) -> FLASH {
        self.registers
    }

    /// Returns an error if the flash is locked or busy.
    fn check_locked_or_busy(&self) -> Result<(), Error> {
        if self.is_locked() {
            Err(Error::Locked)
        } else if self.is_busy() {
            Err(Error::Busy)
        } else {
            Ok(())
        }
    }

    /// Checks the error flags.
    fn check_errors(&self) -> Result<(), Error> {
        let sr = self.registers.sr.read();

        if sr.erserr().bit_is_set() {
            Err(Error::EraseSequence)
        } else if sr.pgperr().bit_is_set() {
            Err(Error::ProgrammingParallelism)
        } else if sr.pgaerr().bit_is_set() {
            Err(Error::ProgrammingAlignment)
        } else if sr.wrperr().bit_is_set() {
            Err(Error::WriteProtection)
        } else {
            Ok(())
        }
    }

    /// Clears all error flags.
    fn clear_errors(&mut self) {
        self.registers.sr.write(|w| {
            w.erserr().set_bit();
            w.pgperr().set_bit();
            w.pgaerr().set_bit();
            w.wrperr().set_bit()
        });
    }
}

/// Erase sequence.
pub struct EraseSequence<'a> {
    flash: &'a mut Flash,
}

impl<'a> EraseSequence<'a> {
    /// Creates a sector erase sequence.
    fn new_erase_sector(flash: &'a mut Flash, sector_number: u8) -> Result<Self, Error> {
        flash.check_locked_or_busy()?;
        flash.clear_errors();

        //TODO: This should check if sector_number is valid for this device

        flash.registers.cr.modify(|_, w| unsafe {
            #[cfg(any(
                feature = "stm32f765",
                feature = "stm32f767",
                feature = "stm32f769",
                feature = "stm32f777",
                feature = "stm32f778",
                feature = "stm32f779",
            ))]
            w.mer1().clear_bit().mer2().clear_bit();
            #[cfg(not(any(
                feature = "stm32f765",
                feature = "stm32f767",
                feature = "stm32f769",
                feature = "stm32f777",
                feature = "stm32f778",
                feature = "stm32f779",
            )))]
            w.mer().clear_bit();
            w.ser().set_bit();
            w.snb().bits(sector_number)
        });
        flash.registers.cr.modify(|_, w| w.strt().start());

        Ok(Self { flash })
    }

    /// Creates a mass erase sequence.
    fn new_mass_erase(flash: &'a mut Flash) -> Result<Self, Error> {
        flash.check_locked_or_busy()?;
        flash.clear_errors();

        flash.registers.cr.modify(|_, w| {
            #[cfg(any(
                feature = "stm32f765",
                feature = "stm32f767",
                feature = "stm32f769",
                feature = "stm32f777",
                feature = "stm32f778",
                feature = "stm32f779",
            ))]
            w.mer1().set_bit().mer2().set_bit();
            #[cfg(not(any(
                feature = "stm32f765",
                feature = "stm32f767",
                feature = "stm32f769",
                feature = "stm32f777",
                feature = "stm32f778",
                feature = "stm32f779",
            )))]
            w.mer().clear_bit();
            w.ser().clear_bit()
        });

        flash.registers.cr.modify(|_, w| w.strt().start());

        Ok(Self { flash })
    }

    /// Waits until the erase sequence is finished.
    pub fn wait(&mut self) -> nb::Result<(), Error> {
        self.flash.check_errors().map_err(nb::Error::from)?;

        if self.flash.is_busy() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

/// Programming sequence.
pub struct ProgrammingSequence<'a, 'b> {
    flash: &'a mut Flash,
    data: &'b [u8],
    address: *mut u8,
}

impl<'a, 'b> ProgrammingSequence<'a, 'b> {
    /// Creates a programming sequence.
    fn new(flash: &'a mut Flash, start_offset: usize, data: &'b [u8]) -> Result<Self, Error> {
        flash.check_locked_or_busy()?;
        flash.clear_errors();

        flash
            .registers
            .cr
            .modify(|_, w| w.psize().psize8().pg().set_bit());

        let address = unsafe { FLASH_BASE.add(start_offset) };

        Ok(Self {
            flash,
            data,
            address,
        })
    }

    /// Waits until the programming sequence is finished.
    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.flash.is_busy() {
            return Err(nb::Error::WouldBlock);
        }

        if let Err(error) = self.flash.check_errors() {
            // make sure programing mode is disabled when an error occurred
            self.flash.registers.cr.modify(|_, w| w.pg().clear_bit());

            return Err(error.into());
        }

        if let Some((first, rest)) = self.data.split_first() {
            if self.address >= FLASH_BASE && self.address <= MAX_FLASH_ADDRESS {
                unsafe {
                    core::ptr::write_volatile(self.address, *first);
                }
            }

            // ensure data is written byte by byte to prevent programming parallelism errors
            cortex_m::asm::dmb();

            self.address = unsafe { self.address.add(1) };
            self.data = rest;

            Err(nb::Error::WouldBlock)
        } else {
            self.flash.registers.cr.modify(|_, w| w.pg().clear_bit());

            Ok(())
        }
    }
}
