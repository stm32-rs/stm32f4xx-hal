use crate::signature::FlashSize;
use crate::stm32::FLASH;
use core::{ptr, slice};

/// Flash erase/program error
#[derive(Debug, Clone, Copy)]
pub enum Error {
    ProgrammingSequence,
    ProgrammingParallelism,
    ProgrammingAlignment,
    WriteProtection,
    Operation,
}

impl Error {
    fn read(flash: &FLASH) -> Option<Self> {
        let sr = flash.sr.read();
        if sr.pgserr().bit() {
            Some(Error::ProgrammingSequence)
        } else if sr.pgperr().bit() {
            Some(Error::ProgrammingParallelism)
        } else if sr.pgaerr().bit() {
            Some(Error::ProgrammingAlignment)
        } else if sr.wrperr().bit() {
            Some(Error::WriteProtection)
        } else if sr.operr().bit() {
            Some(Error::Operation)
        } else {
            None
        }
    }
}

/// Flash methods implemented for `stm32::FLASH`
pub trait FlashExt {
    /// Memory-mapped address
    fn address(&self) -> usize;
    /// Size in bytes
    fn len(&self) -> usize;
    /// Returns a read-only view of flash memory
    fn read(&self) -> &[u8] {
        let ptr = self.address() as *const _;
        unsafe { slice::from_raw_parts(ptr, self.len()) }
    }
    /// Unlock flash for erasing/programming until this method's
    /// result is dropped
    fn unlocked(&mut self) -> UnlockedFlash;
}

impl FlashExt for FLASH {
    fn address(&self) -> usize {
        0x0800_0000
    }

    fn len(&self) -> usize {
        FlashSize::get().bytes()
    }

    fn unlocked(&mut self) -> UnlockedFlash {
        unlock(self);
        UnlockedFlash { flash: self }
    }
}

const PSIZE_X8: u8 = 0b00;

/// Result of `FlashExt::unlocked()`
pub struct UnlockedFlash<'a> {
    flash: &'a mut FLASH,
}

/// Automatically lock flash erase/program when leaving scope
impl Drop for UnlockedFlash<'_> {
    fn drop(&mut self) {
        lock(&self.flash);
    }
}

impl UnlockedFlash<'_> {
    /// Erase a flash sector
    ///
    /// Refer to the reference manual to see which sector corresponds
    /// to which memory address.
    pub fn erase(&mut self, sector: u8) -> Result<(), Error> {
        let snb = if sector < 12 { sector } else { sector + 4 };

        #[rustfmt::skip]
        self.flash.cr.modify(|_, w| unsafe {
            w
                // start
                .strt().set_bit()
                .psize().bits(PSIZE_X8)
                // sector number
                .snb().bits(snb)
                // sectore erase
                .ser().set_bit()
                // no programming
                .pg().clear_bit()
        });
        self.wait_ready();
        self.ok()
    }

    /// Program bytes with offset into flash memory,
    /// aligned to 128-bit rows
    pub fn program<'a, I>(&mut self, mut offset: usize, mut bytes: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a u8>,
    {
        let ptr = self.flash.address() as *mut u8;
        let mut bytes_written = 1;
        while bytes_written > 0 {
            bytes_written = 0;
            let amount = 16 - (offset % 16);

            #[rustfmt::skip]
            self.flash.cr.modify(|_, w| unsafe {
                w
                    .psize().bits(PSIZE_X8)
                    // no sector erase
                    .ser().clear_bit()
                    // programming
                    .pg().set_bit()
            });
            for _ in 0..amount {
                match bytes.next() {
                    Some(byte) => {
                        unsafe {
                            ptr::write_volatile(ptr.add(offset), *byte);
                        }
                        offset += 1;
                        bytes_written += 1;
                    }
                    None => break,
                }
            }
            self.wait_ready();
            self.ok()?;
        }
        self.flash.cr.modify(|_, w| w.pg().clear_bit());

        Ok(())
    }

    fn ok(&self) -> Result<(), Error> {
        Error::read(&self.flash).map(Err).unwrap_or(Ok(()))
    }

    fn wait_ready(&self) {
        while self.flash.sr.read().bsy().bit() {}
    }
}

const UNLOCK_KEY1: u32 = 0x45670123;
const UNLOCK_KEY2: u32 = 0xCDEF89AB;

fn unlock(flash: &FLASH) {
    flash.keyr.write(|w| unsafe { w.key().bits(UNLOCK_KEY1) });
    flash.keyr.write(|w| unsafe { w.key().bits(UNLOCK_KEY2) });
    assert!(!flash.cr.read().lock().bit())
}

fn lock(flash: &FLASH) {
    flash.cr.modify(|_, w| w.lock().set_bit());
}
