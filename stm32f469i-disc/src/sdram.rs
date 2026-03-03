//! 16MByte SDRAM peripheral on F469 discovery board

use crate::hal;
use crate::hal::{fmc::FmcExt, pac::FMC};
use core::mem;
use stm32_fmc::devices::is42s32400f_6;
use stm32_fmc::{AddressPinSet, PinsSdram, SdramPinSet};
use stm32f4xx_hal::rcc::Clocks;

// Re-export FMC alternate pin types for the sdram_pins! macro
#[doc(hidden)]
pub use crate::hal::gpio::alt::fmc as alt;
#[macro_export]
macro_rules! sdram_pins {
    ($c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => {
        (
            // address
            alt::A0::from($f.pf0.internal_pull_up(true)),
            alt::A1::from($f.pf1.internal_pull_up(true)),
            alt::A2::from($f.pf2.internal_pull_up(true)),
            alt::A3::from($f.pf3.internal_pull_up(true)),
            alt::A4::from($f.pf4.internal_pull_up(true)),
            alt::A5::from($f.pf5.internal_pull_up(true)),
            alt::A6::from($f.pf12.internal_pull_up(true)),
            alt::A7::from($f.pf13.internal_pull_up(true)),
            alt::A8::from($f.pf14.internal_pull_up(true)),
            alt::A9::from($f.pf15.internal_pull_up(true)),
            alt::A10::from($g.pg0.internal_pull_up(true)),
            alt::A11::from($g.pg1.internal_pull_up(true)),
            // bank
            alt::Ba0::from($g.pg4.internal_pull_up(true)),
            alt::Ba1::from($g.pg5.internal_pull_up(true)),
            // data
            alt::D0::from($d.pd14.internal_pull_up(true)),
            alt::D1::from($d.pd15.internal_pull_up(true)),
            alt::D2::from($d.pd0.internal_pull_up(true)),
            alt::D3::from($d.pd1.internal_pull_up(true)),
            alt::D4::from($e.pe7.internal_pull_up(true)),
            alt::D5::from($e.pe8.internal_pull_up(true)),
            alt::D6::from($e.pe9.internal_pull_up(true)),
            alt::D7::from($e.pe10.internal_pull_up(true)),
            alt::D8::from($e.pe11.internal_pull_up(true)),
            alt::D9::from($e.pe12.internal_pull_up(true)),
            alt::D10::from($e.pe13.internal_pull_up(true)),
            alt::D11::from($e.pe14.internal_pull_up(true)),
            alt::D12::from($e.pe15.internal_pull_up(true)),
            alt::D13::from($d.pd8.internal_pull_up(true)),
            alt::D14::from($d.pd9.internal_pull_up(true)),
            alt::D15::from($d.pd10.internal_pull_up(true)),
            alt::D16::from($h.ph8.internal_pull_up(true)),
            alt::D17::from($h.ph9.internal_pull_up(true)),
            alt::D18::from($h.ph10.internal_pull_up(true)),
            alt::D19::from($h.ph11.internal_pull_up(true)),
            alt::D20::from($h.ph12.internal_pull_up(true)),
            alt::D21::from($h.ph13.internal_pull_up(true)),
            alt::D22::from($h.ph14.internal_pull_up(true)),
            alt::D23::from($h.ph15.internal_pull_up(true)),
            alt::D24::from($i.pi0.internal_pull_up(true)),
            alt::D25::from($i.pi1.internal_pull_up(true)),
            alt::D26::from($i.pi2.internal_pull_up(true)),
            alt::D27::from($i.pi3.internal_pull_up(true)),
            alt::D28::from($i.pi6.internal_pull_up(true)),
            alt::D29::from($i.pi7.internal_pull_up(true)),
            alt::D30::from($i.pi9.internal_pull_up(true)),
            alt::D31::from($i.pi10.internal_pull_up(true)),
            // NBL
            alt::Nbl0::from($e.pe0.internal_pull_up(true)),
            alt::Nbl1::from($e.pe1.internal_pull_up(true)),
            alt::Nbl2::from($i.pi4.internal_pull_up(true)),
            alt::Nbl3::from($i.pi5.internal_pull_up(true)),
            // Control
            alt::Sdcke0::from($h.ph2.internal_pull_up(true)),
            alt::Sdclk::from($g.pg8.internal_pull_up(true)),
            alt::Sdncas::from($g.pg15.internal_pull_up(true)),
            alt::Sdne0::from($h.ph3.internal_pull_up(true)),
            alt::Sdnras::from($f.pf11.internal_pull_up(true)),
            alt::Sdnwe::from($c.pc0.internal_pull_up(true)),
        )
    };
}
pub use sdram_pins;

/// GPIO pins remaining after SDRAM initialization that VLS needs.
/// 
/// These pins are NOT used by the SDRAM interface and are returned for use by
/// the application (touch interrupt, SDIO).
/// 
/// Note: PH7 (LCD reset) is returned separately from `split_sdram_pins` since it's
/// consumed during display initialization.
pub struct SdramRemainders {
    /// Touch interrupt pin (FT6X06)
    pub pc1: hal::gpio::PC1<hal::gpio::Input>,
    /// SDIO data lines and clock
    pub pc8: hal::gpio::PC8<hal::gpio::Input>,
    pub pc9: hal::gpio::PC9<hal::gpio::Input>,
    pub pc10: hal::gpio::PC10<hal::gpio::Input>,
    pub pc11: hal::gpio::PC11<hal::gpio::Input>,
    pub pc12: hal::gpio::PC12<hal::gpio::Input>,
    /// SDIO command pin
    pub pd2: hal::gpio::PD2<hal::gpio::Input>,
}

/// Split GPIO ports into SDRAM pins and remaining pins for VLS.
/// 
/// This is the function equivalent of the `sdram_pins!` macro, with the addition
/// of returning the remaining pins that VLS needs for touch, SDIO, and LCD reset.
/// 
/// # Arguments
/// * GPIO port parts from `p.GPIOX.split(&mut rcc)`
/// 
/// # Returns
/// * Tuple of SDRAM pins (compatible with `Sdram::new()`)
/// * `SdramRemainders` with pins not used by SDRAM
/// * PH7 pin for LCD reset (consumed by display init)
pub fn split_sdram_pins(
    gpioc: hal::gpio::gpioc::Parts,
    gpiod: hal::gpio::gpiod::Parts,
    gpioe: hal::gpio::gpioe::Parts,
    gpiof: hal::gpio::gpiof::Parts,
    gpiog: hal::gpio::gpiog::Parts,
    gpioh: hal::gpio::gpioh::Parts,
    gpioi: hal::gpio::gpioi::Parts,
) -> (
    // SDRAM pins tuple (same as sdram_pins! macro output)
    (
        // Address pins
        alt::A0, alt::A1, alt::A2, alt::A3, alt::A4, alt::A5,
        alt::A6, alt::A7, alt::A8, alt::A9, alt::A10, alt::A11,
        // Bank pins
        alt::Ba0, alt::Ba1,
        // Data pins
        alt::D0, alt::D1, alt::D2, alt::D3, alt::D4, alt::D5,
        alt::D6, alt::D7, alt::D8, alt::D9, alt::D10, alt::D11,
        alt::D12, alt::D13, alt::D14, alt::D15, alt::D16, alt::D17,
        alt::D18, alt::D19, alt::D20, alt::D21, alt::D22, alt::D23,
        alt::D24, alt::D25, alt::D26, alt::D27, alt::D28, alt::D29,
        alt::D30, alt::D31,
        // NBL pins
        alt::Nbl0, alt::Nbl1, alt::Nbl2, alt::Nbl3,
        // Control pins
        alt::Sdcke0, alt::Sdclk, alt::Sdncas, alt::Sdne0, alt::Sdnras, alt::Sdnwe,
    ),
    // Remaining pins for VLS
    SdramRemainders,
    // LCD reset pin (consumed by init_display)
    hal::gpio::PH7<hal::gpio::Input>,
) {
    // Extract remaining pins BEFORE they're consumed by SDRAM pin extraction
    let remainders = SdramRemainders {
        pc1: gpioc.pc1,
        pc8: gpioc.pc8,
        pc9: gpioc.pc9,
        pc10: gpioc.pc10,
        pc11: gpioc.pc11,
        pc12: gpioc.pc12,
        pd2: gpiod.pd2,
    };
    let ph7 = gpioh.ph7;


    // Extract SDRAM pins (same logic as sdram_pins! macro)
    let sdram_pins = (
        // Address
        alt::A0::from(gpiof.pf0.internal_pull_up(true)),
        alt::A1::from(gpiof.pf1.internal_pull_up(true)),
        alt::A2::from(gpiof.pf2.internal_pull_up(true)),
        alt::A3::from(gpiof.pf3.internal_pull_up(true)),
        alt::A4::from(gpiof.pf4.internal_pull_up(true)),
        alt::A5::from(gpiof.pf5.internal_pull_up(true)),
        alt::A6::from(gpiof.pf12.internal_pull_up(true)),
        alt::A7::from(gpiof.pf13.internal_pull_up(true)),
        alt::A8::from(gpiof.pf14.internal_pull_up(true)),
        alt::A9::from(gpiof.pf15.internal_pull_up(true)),
        alt::A10::from(gpiog.pg0.internal_pull_up(true)),
        alt::A11::from(gpiog.pg1.internal_pull_up(true)),
        // Bank
        alt::Ba0::from(gpiog.pg4.internal_pull_up(true)),
        alt::Ba1::from(gpiog.pg5.internal_pull_up(true)),
        // Data
        alt::D0::from(gpiod.pd14.internal_pull_up(true)),
        alt::D1::from(gpiod.pd15.internal_pull_up(true)),
        alt::D2::from(gpiod.pd0.internal_pull_up(true)),
        alt::D3::from(gpiod.pd1.internal_pull_up(true)),
        alt::D4::from(gpioe.pe7.internal_pull_up(true)),
        alt::D5::from(gpioe.pe8.internal_pull_up(true)),
        alt::D6::from(gpioe.pe9.internal_pull_up(true)),
        alt::D7::from(gpioe.pe10.internal_pull_up(true)),
        alt::D8::from(gpioe.pe11.internal_pull_up(true)),
        alt::D9::from(gpioe.pe12.internal_pull_up(true)),
        alt::D10::from(gpioe.pe13.internal_pull_up(true)),
        alt::D11::from(gpioe.pe14.internal_pull_up(true)),
        alt::D12::from(gpioe.pe15.internal_pull_up(true)),
        alt::D13::from(gpiod.pd8.internal_pull_up(true)),
        alt::D14::from(gpiod.pd9.internal_pull_up(true)),
        alt::D15::from(gpiod.pd10.internal_pull_up(true)),
        alt::D16::from(gpioh.ph8.internal_pull_up(true)),
        alt::D17::from(gpioh.ph9.internal_pull_up(true)),
        alt::D18::from(gpioh.ph10.internal_pull_up(true)),
        alt::D19::from(gpioh.ph11.internal_pull_up(true)),
        alt::D20::from(gpioh.ph12.internal_pull_up(true)),
        alt::D21::from(gpioh.ph13.internal_pull_up(true)),
        alt::D22::from(gpioh.ph14.internal_pull_up(true)),
        alt::D23::from(gpioh.ph15.internal_pull_up(true)),
        alt::D24::from(gpioi.pi0.internal_pull_up(true)),
        alt::D25::from(gpioi.pi1.internal_pull_up(true)),
        alt::D26::from(gpioi.pi2.internal_pull_up(true)),
        alt::D27::from(gpioi.pi3.internal_pull_up(true)),
        alt::D28::from(gpioi.pi6.internal_pull_up(true)),
        alt::D29::from(gpioi.pi7.internal_pull_up(true)),
        alt::D30::from(gpioi.pi9.internal_pull_up(true)),
        alt::D31::from(gpioi.pi10.internal_pull_up(true)),
        // NBL
        alt::Nbl0::from(gpioe.pe0.internal_pull_up(true)),
        alt::Nbl1::from(gpioe.pe1.internal_pull_up(true)),
        alt::Nbl2::from(gpioi.pi4.internal_pull_up(true)),
        alt::Nbl3::from(gpioi.pi5.internal_pull_up(true)),
        // Control
        alt::Sdcke0::from(gpioh.ph2.internal_pull_up(true)),
        alt::Sdclk::from(gpiog.pg8.internal_pull_up(true)),
        alt::Sdncas::from(gpiog.pg15.internal_pull_up(true)),
        alt::Sdne0::from(gpioh.ph3.internal_pull_up(true)),
        alt::Sdnras::from(gpiof.pf11.internal_pull_up(true)),
        alt::Sdnwe::from(gpioc.pc0.internal_pull_up(true)),
    );

    (sdram_pins, remainders, ph7)
}

/// Total SDRAM size in bytes (16 MB)
pub const SDRAM_SIZE_BYTES: usize = 16 * 1024 * 1024;

/// SDRAM memory wrapper providing typed access to the 16MB SDRAM on F469 Discovery.
/// 
/// # Memory Layout
/// 
/// The F469 Discovery has 16MB of SDRAM mapped at address 0xC0000000.
/// This struct provides safe(r) access to this memory region with typed slices.
/// 
/// # Usage
/// 
/// ```ignore
/// let mut sdram = Sdram::new(fmc, sdram_pins!(...), &clocks, &mut delay);
/// 
/// // Get a typed slice for a framebuffer (e.g., 480x800 RGB565 = 768,000 u16s)
/// let fb_buffer: &'static mut [u16] = sdram.as_slice_mut();
/// let fb = LtdcFramebuffer::new(fb_buffer, 480, 800);
/// ```
pub struct Sdram {
    /// Raw pointer to SDRAM base (public for backward compatibility)
    pub mem: *mut u32,
    /// Number of u32 words in SDRAM (4M words = 16MB)
    pub words: usize,
}

impl Sdram {
    /// Initialize SDRAM with the given pins.
    /// 
    /// Returns a new Sdram wrapper after initializing the memory controller.
    pub fn new<BANK: SdramPinSet, ADDR: AddressPinSet, PINS: PinsSdram<BANK, ADDR>>(
        fmc: FMC,
        pins: PINS,
        clocks: &Clocks,
        delay: &mut hal::timer::SysDelay,
    ) -> Self {
        Self {
            mem: fmc
                .sdram(pins, is42s32400f_6::Is42s32400f6 {}, clocks)
                .init(delay),
            words: SDRAM_SIZE_BYTES / core::mem::size_of::<u32>(),
        }
    }
    
    /// Get the raw base pointer as u32.
    /// 
    /// Use this for low-level access when you need the original pointer type.
    pub fn as_mut_ptr(&self) -> *mut u32 {
        self.mem
    }
    
    /// Get the base address of the SDRAM.
    pub fn base_address(&self) -> usize {
        self.mem as usize
    }
    
    /// Get the total size in bytes.
    pub fn size_bytes(&self) -> usize {
        SDRAM_SIZE_BYTES
    }
    
    /// Get a typed mutable slice of the entire SDRAM region.
    /// 
    /// # Safety Invariant
    /// 
    /// This method is safe because:
    /// 1. SDRAM memory is dedicated to this use (not overlapping with other memory)
    /// 2. The 'static lifetime is valid because SDRAM persists for the program's lifetime
    /// 3. The caller must ensure they don't create multiple overlapping mutable slices
    /// 
    /// However, calling this multiple times and storing both results violates Rust's
    /// aliasing rules. Use [`subslice_mut`] for partitioning.
    /// 
    /// # Type Constraints
    /// 
    /// The type T must have size that evenly divides the SDRAM size, and proper alignment.
    /// Common types: u8, u16, u32 for pixel buffers.
    /// 
    /// # Panics
    /// 
    /// Panics if the size of T doesn't evenly divide the SDRAM size.
    pub fn as_slice_mut<T>(&mut self) -> &'static mut [T]
    where
        T: Sized,
    {
        let type_size = mem::size_of::<T>();
        let type_align = mem::align_of::<T>();
        
        assert!(
            SDRAM_SIZE_BYTES % type_size == 0,
            "Type size {} doesn't evenly divide SDRAM size {}",
            type_size,
            SDRAM_SIZE_BYTES
        );
        
        assert!(
            (self.mem as usize) % type_align == 0,
            "SDRAM base address not aligned for type T (align={})",
            type_align
        );
        
        let len = SDRAM_SIZE_BYTES / type_size;
        // SAFETY: SDRAM region is valid for program lifetime, we've verified alignment
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(self.mem as *mut T, len) }
    }
    
    /// Get a typed mutable slice of a portion of the SDRAM region.
    /// 
    /// Use this to partition SDRAM into multiple regions (e.g., multiple framebuffers).
    /// 
    /// # Arguments
    /// 
    /// * `offset_bytes` - Byte offset from the start of SDRAM
    /// * `len_elements` - Number of elements of type T to include
    /// 
    /// # Panics
    /// 
    /// Panics if:
    /// - `offset_bytes` is not aligned for type T
    /// - The region extends beyond SDRAM bounds
    pub fn subslice_mut<T>(&mut self, offset_bytes: usize, len_elements: usize) -> &'static mut [T]
    where
        T: Sized,
    {
        let type_size = mem::size_of::<T>();
        let type_align = mem::align_of::<T>();
        
        let start = (self.mem as usize) + offset_bytes;
        let end = start + len_elements * type_size;
        
        assert!(
            start % type_align == 0,
            "Offset {} not aligned for type T (align={})",
            offset_bytes,
            type_align
        );
        
        assert!(
            end <= (self.mem as usize) + SDRAM_SIZE_BYTES,
            "Region extends beyond SDRAM bounds (offset={}, len={}, available={})",
            offset_bytes,
            len_elements * type_size,
            SDRAM_SIZE_BYTES
        );
        
        // SAFETY: We've verified alignment and bounds
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(start as *mut T, len_elements) }
    }
}
