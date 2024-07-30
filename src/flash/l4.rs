//! Flash memory module
//!
//! Example usage of flash programming interface:
//!
//! ```
//! fn program_region(mut flash: flash::Parts) -> Result<(), flash::Error> {
//!     // Unlock the flashing module
//!     let mut prog = flash.keyr.unlock_flash(&mut flash.sr, &mut flash.cr)?;
//!
//!     let page = flash::FlashPage(5);
//!
//!     // Perform the erase and programing operation
//!     prog.erase_page(page)?;
//!     let data = [
//!         0x1111_1112_1113_1114,
//!         0x2221_2222_2223_2224,
//!         0x3331_3332_3333_3334,
//!     ];
//!     prog.write_native(page.to_address(), &data)?;
//!
//!     // Check result (not needed, but done for this example)
//!     let addr = page.to_address() as *const u64;
//!     assert!(unsafe { core::ptr::read(addr) } == data[0]);
//!     assert!(unsafe { core::ptr::read(addr.offset(1)) } == data[1]);
//!     assert!(unsafe { core::ptr::read(addr.offset(2)) } == data[2]);
//!
//!     Ok(())
//! }
//! ```

use crate::pac::{flash, FLASH};
use core::convert::TryInto;
use core::{mem, ops::Drop, ptr};

/// Flash page representation where each flash page represents a region of 2048 bytes. The flash
/// controller can only erase on a page basis.
#[derive(Copy, Clone, Debug)]
pub struct FlashPage(pub usize);

/// Flash operation error
#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// Flash controller is not done yet
    Busy,
    /// Error detected (by command execution, or because no command could be executed)
    Illegal,
    /// Set during read if ECC decoding logic detects correctable or uncorrectable error
    EccError,
    /// Page number is out of range
    PageOutOfRange,
    /// (Legal) command failed
    Failure,
}

/// A type alias for the result of a Flash operation.
pub type FResult = core::result::Result<(), Error>;

pub trait Read {
    /// Native type of the flash for reading with the correct alignment of the memory and size
    ///
    /// Can be `u8`, `u16`, `u32`, ..., or any user defined type
    type NativeType;

    /// Read from the flash memory using the native interface
    fn read_native(&self, address: usize, array: &mut [Self::NativeType]);

    /// Read a buffer of bytes from memory
    fn read(&self, address: usize, buf: &mut [u8]);
}

pub trait WriteErase {
    /// Native type of the flash for writing with the correct alignment and size
    ///
    /// Can be `u8`, `u16`, `u32`, ..., or any user defined type
    type NativeType;

    /// check flash status
    fn status(&self) -> FResult;

    /// Erase specified flash page.
    fn erase_page(&mut self, page: FlashPage) -> FResult;

    /// The smallest possible write, depends on platform
    fn write_native(&mut self, address: usize, array: &[Self::NativeType]) -> FResult;

    /// Read a buffer of bytes to memory, this uses the native writes internally and if it's not
    /// the same length and a set of native writes the write will be padded to fill a native write.
    fn write(&mut self, address: usize, data: &[u8]) -> FResult;
}

/// Extension trait to constrain the FLASH peripheral
pub trait FlashExt {
    /// Constrains the FLASH peripheral to play nicely with the other abstractions
    fn constrain(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR {},
            pdkeyr: PDKEYR {},
            keyr: KEYR {},
            optkeyr: OPTKEYR {},
            sr: SR {},
            cr: CR {},
            eccr: ECCR {},
            pcrop1sr: PCROP1SR {},
            pcrop1er: PCROP1ER {},
            wrp1ar: WRP1AR {},
            wrp1br: WRP1BR {},
        }
    }
}

/// Constrained FLASH peripheral
pub struct Parts {
    /// Opaque ACR register
    pub acr: ACR,
    /// Opaque PDKEYR register
    pub pdkeyr: PDKEYR,
    /// Opaque KEYR register
    pub keyr: KEYR,
    /// Opaque OPTKEYR register
    pub optkeyr: OPTKEYR,
    /// Opaque SR register
    pub sr: SR,
    /// Opaque SR register
    pub cr: CR,
    /// Opaque ECCR register
    pub eccr: ECCR,
    /// Opaque PCROP1SR register
    pub pcrop1sr: PCROP1SR,
    /// Opaque PCROP1ER register
    pub pcrop1er: PCROP1ER,
    /// Opaque WRP1AR register
    pub wrp1ar: WRP1AR,
    /// Opaque WRP1BR register
    pub wrp1br: WRP1BR,
}

macro_rules! generate_register {
    ($a:ident, $b:ident, $name:expr) => {
        #[doc = "Opaque "]
        #[doc = $name]
        #[doc = " register"]
        pub struct $a;

        impl $a {
            #[allow(unused)]
            pub(crate) fn $b(&mut self) -> &flash::$a {
                // NOTE(unsafe) this proxy grants exclusive access to this register
                unsafe { &(*FLASH::ptr()).$b }
            }
        }
    };

    ($a:ident, $b:ident) => {
        generate_register!($a, $b, stringify!($a));
    };
}

generate_register!(ACR, acr);
generate_register!(PDKEYR, pdkeyr);
generate_register!(KEYR, keyr);
generate_register!(OPTKEYR, optkeyr);
generate_register!(SR, sr);
generate_register!(CR, cr);
generate_register!(ECCR, eccr);
generate_register!(PCROP1SR, pcrop1sr);
generate_register!(PCROP1ER, pcrop1er);
generate_register!(WRP1AR, wrp1ar);
generate_register!(WRP1BR, wrp1br);

const FLASH_KEY1: u32 = 0x4567_0123;
const FLASH_KEY2: u32 = 0xCDEF_89AB;

impl KEYR {
    /// Unlock the flash registers via KEYR to access the flash programming
    pub fn unlock_flash<'a>(
        &'a mut self,
        sr: &'a mut SR,
        cr: &'a mut CR,
    ) -> Result<FlashProgramming<'a>, Error> {
        let keyr = self.keyr();
        unsafe {
            keyr.write(|w| w.bits(FLASH_KEY1));
            keyr.write(|w| w.bits(FLASH_KEY2));
        }

        if cr.cr().read().lock().bit_is_clear() {
            Ok(FlashProgramming { sr, cr })
        } else {
            Err(Error::Failure)
        }
    }
}

impl FlashPage {
    /// This gives the starting address of a flash page in physical address
    pub const fn to_address(&self) -> usize {
        0x0800_0000 + self.0 * 2048
    }
}

/// Flash programming interface
pub struct FlashProgramming<'a> {
    sr: &'a mut SR,
    cr: &'a mut CR,
}

impl<'a> Drop for FlashProgramming<'a> {
    fn drop(&mut self) {
        // Lock on drop
        self.lock();
    }
}

impl<'a> Read for FlashProgramming<'a> {
    type NativeType = u8;

    #[inline]
    fn read_native(&self, address: usize, array: &mut [Self::NativeType]) {
        let mut address = address as *const Self::NativeType;

        for data in array {
            unsafe {
                *data = ptr::read(address);
                address = address.add(1);
            }
        }
    }

    #[inline]
    fn read(&self, address: usize, buf: &mut [u8]) {
        self.read_native(address, buf);
    }
}

impl<'a> WriteErase for FlashProgramming<'a> {
    type NativeType = u64;

    fn status(&self) -> FResult {
        let sr = unsafe { &(*FLASH::ptr()).sr }.read();

        if sr.bsy().bit_is_set() {
            Err(Error::Busy)
        } else if sr.pgaerr().bit_is_set() || sr.progerr().bit_is_set() || sr.wrperr().bit_is_set()
        {
            Err(Error::Illegal)
        } else {
            Ok(())
        }
    }

    fn erase_page(&mut self, page: FlashPage) -> FResult {
        match page.0 {
            0..=255 => {
                self.cr.cr().modify(|_, w| unsafe {
                    w.bker()
                        .clear_bit()
                        .pnb()
                        .bits(page.0 as u8)
                        .per()
                        .set_bit()
                });
            }
            256..=511 => {
                self.cr.cr().modify(|_, w| unsafe {
                    w.bker()
                        .set_bit()
                        .pnb()
                        .bits((page.0 - 256) as u8)
                        .per()
                        .set_bit()
                });
            }
            _ => {
                return Err(Error::PageOutOfRange);
            }
        }

        self.cr.cr().modify(|_, w| w.start().set_bit());

        let res = self.wait();

        self.cr.cr().modify(|_, w| w.per().clear_bit());

        res
    }

    fn write_native(&mut self, address: usize, array: &[Self::NativeType]) -> FResult {
        // NB: The check for alignment of the address, and that the flash is erased is made by the
        // flash controller. The `wait` function will return the proper error codes.
        let mut address = address as *mut u32;

        self.cr.cr().modify(|_, w| w.pg().set_bit());

        for dword in array {
            unsafe {
                ptr::write_volatile(address, *dword as u32);
                ptr::write_volatile(address.add(1), (*dword >> 32) as u32);

                address = address.add(2);
            }

            self.wait()?;

            if self.sr.sr().read().eop().bit_is_set() {
                self.sr.sr().modify(|_, w| w.eop().clear_bit());
            }
        }

        self.cr.cr().modify(|_, w| w.pg().clear_bit());

        Ok(())
    }

    fn write(&mut self, address: usize, data: &[u8]) -> FResult {
        let address_offset = address % mem::align_of::<Self::NativeType>();
        let unaligned_size = (mem::size_of::<Self::NativeType>() - address_offset)
            % mem::size_of::<Self::NativeType>();

        if unaligned_size > 0 {
            let unaligned_data = &data[..unaligned_size];
            // Handle unaligned address data, make it into a native write
            let mut data = 0xffff_ffff_ffff_ffffu64;
            for b in unaligned_data {
                data = (data >> 8) | ((*b as Self::NativeType) << 56);
            }

            let unaligned_address = address - address_offset;
            let native = &[data];
            self.write_native(unaligned_address, native)?;
        }

        // Handle aligned address data
        let aligned_data = &data[unaligned_size..];
        let mut aligned_address = if unaligned_size > 0 {
            address - address_offset + mem::size_of::<Self::NativeType>()
        } else {
            address
        };

        let mut chunks = aligned_data.chunks_exact(mem::size_of::<Self::NativeType>());

        while let Some(exact_chunk) = chunks.next() {
            // Write chunks
            let native = &[Self::NativeType::from_ne_bytes(
                exact_chunk.try_into().unwrap(),
            )];
            self.write_native(aligned_address, native)?;
            aligned_address += mem::size_of::<Self::NativeType>();
        }

        let rem = chunks.remainder();

        if !rem.is_empty() {
            let mut data = 0xffff_ffff_ffff_ffffu64;
            // Write remainder
            for b in rem.iter().rev() {
                data = (data << 8) | *b as Self::NativeType;
            }

            let native = &[data];
            self.write_native(aligned_address, native)?;
        }

        Ok(())
    }
}

impl<'a> FlashProgramming<'a> {
    /// Lock the flash memory controller
    fn lock(&mut self) {
        self.cr.cr().modify(|_, w| w.lock().set_bit());
    }

    /// Wait till last flash operation is complete
    fn wait(&mut self) -> FResult {
        while self.sr.sr().read().bsy().bit_is_set() {}

        self.status()
    }

    /// Erase all flash pages, note that this will erase the current running program if it is not
    /// called from a program running in RAM.
    pub fn erase_all_pages(&mut self) -> FResult {
        self.cr.cr().modify(|_, w| w.mer1().set_bit());
        self.cr.cr().modify(|_, w| w.start().set_bit());

        let res = self.wait();

        self.cr.cr().modify(|_, w| w.mer1().clear_bit());

        res
    }
}
