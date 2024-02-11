//! HAL for Flexible memory controller (FMC)
//!
//! See the stm32-fmc [usage guide](https://github.com/stm32-rs/stm32-fmc#usage)

// From stm32_fmc

use stm32_fmc::FmcPeripheral;
use stm32_fmc::{AddressPinSet, PinsSdram, Sdram, SdramChip, SdramPinSet, SdramTargetBank};

use crate::pac;
use crate::rcc::{BusClock, Clocks, Enable, Reset};
use fugit::HertzU32 as Hertz;

use crate::gpio::alt::fmc as alt;

/// Storage type for Flexible Memory Controller and its clocks
pub struct FMC {
    pub fmc: pac::FMC,
    hclk: Hertz,
}

/// Extension trait for FMC controller
pub trait FmcExt: Sized {
    fn fmc(self, clocks: &Clocks) -> FMC;

    /// A new SDRAM memory via the Flexible Memory Controller
    fn sdram<
        BANK: SdramPinSet,
        ADDR: AddressPinSet,
        PINS: PinsSdram<BANK, ADDR>,
        CHIP: SdramChip,
    >(
        self,
        pins: PINS,
        chip: CHIP,
        clocks: &Clocks,
    ) -> Sdram<FMC, CHIP> {
        let fmc = self.fmc(clocks);
        Sdram::new(fmc, pins, chip)
    }

    /// A new SDRAM memory via the Flexible Memory Controller
    fn sdram_unchecked<CHIP: SdramChip, BANK: Into<SdramTargetBank>>(
        self,
        bank: BANK,
        chip: CHIP,
        clocks: &Clocks,
    ) -> Sdram<FMC, CHIP> {
        let fmc = self.fmc(clocks);
        Sdram::new_unchecked(fmc, bank, chip)
    }
}

impl FmcExt for pac::FMC {
    /// New FMC instance
    fn fmc(self, clocks: &Clocks) -> FMC {
        FMC {
            fmc: self,
            hclk: pac::FMC::clock(clocks),
        }
    }
}

unsafe impl FmcPeripheral for FMC {
    const REGISTERS: *const () = pac::FMC::ptr() as *const ();

    fn enable(&mut self) {
        // TODO : change it to something safe ...
        unsafe {
            // Enable FMC
            pac::FMC::enable_unchecked();
            // Reset FMC
            pac::FMC::reset_unchecked();
        }
    }

    fn source_clock_hz(&self) -> u32 {
        // FMC block is clocked by HCLK
        self.hclk.raw()
    }
}

impl stm32_fmc::A0 for alt::A0 {}
impl stm32_fmc::A1 for alt::A1 {}
impl stm32_fmc::A2 for alt::A2 {}
impl stm32_fmc::A3 for alt::A3 {}
impl stm32_fmc::A4 for alt::A4 {}
impl stm32_fmc::A5 for alt::A5 {}
impl stm32_fmc::A6 for alt::A6 {}
impl stm32_fmc::A7 for alt::A7 {}
impl stm32_fmc::A8 for alt::A8 {}
impl stm32_fmc::A9 for alt::A9 {}
impl stm32_fmc::A10 for alt::A10 {}
impl stm32_fmc::A11 for alt::A11 {}
impl stm32_fmc::A12 for alt::A12 {}
impl stm32_fmc::A13 for alt::A13 {}
impl stm32_fmc::A14 for alt::A14 {}
impl stm32_fmc::A15 for alt::A15 {}
impl stm32_fmc::A16 for alt::A16 {}
impl stm32_fmc::A17 for alt::A17 {}
impl stm32_fmc::A18 for alt::A18 {}
impl stm32_fmc::A19 for alt::A19 {}
impl stm32_fmc::A20 for alt::A20 {}
impl stm32_fmc::A21 for alt::A21 {}
impl stm32_fmc::A22 for alt::A22 {}
impl stm32_fmc::A23 for alt::A23 {}
impl stm32_fmc::A24 for alt::A24 {}
impl stm32_fmc::BA0 for alt::Ba0 {}
impl stm32_fmc::BA1 for alt::Ba1 {}
impl stm32_fmc::CLK for alt::Clk {}
impl stm32_fmc::D0 for alt::D0 {}
impl stm32_fmc::D1 for alt::D1 {}
impl stm32_fmc::D2 for alt::D2 {}
impl stm32_fmc::D3 for alt::D3 {}
impl stm32_fmc::D4 for alt::D4 {}
impl stm32_fmc::D5 for alt::D5 {}
impl stm32_fmc::D6 for alt::D6 {}
impl stm32_fmc::D7 for alt::D7 {}
impl stm32_fmc::D8 for alt::D8 {}
impl stm32_fmc::D9 for alt::D9 {}
impl stm32_fmc::D10 for alt::D10 {}
impl stm32_fmc::D11 for alt::D11 {}
impl stm32_fmc::D12 for alt::D12 {}
impl stm32_fmc::D13 for alt::D13 {}
impl stm32_fmc::D14 for alt::D14 {}
impl stm32_fmc::D15 for alt::D15 {}
impl stm32_fmc::D16 for alt::D16 {}
impl stm32_fmc::D17 for alt::D17 {}
impl stm32_fmc::D18 for alt::D18 {}
impl stm32_fmc::D19 for alt::D19 {}
impl stm32_fmc::D20 for alt::D20 {}
impl stm32_fmc::D21 for alt::D21 {}
impl stm32_fmc::D22 for alt::D22 {}
impl stm32_fmc::D23 for alt::D23 {}
impl stm32_fmc::D24 for alt::D24 {}
impl stm32_fmc::D25 for alt::D25 {}
impl stm32_fmc::D26 for alt::D26 {}
impl stm32_fmc::D27 for alt::D27 {}
impl stm32_fmc::D28 for alt::D28 {}
impl stm32_fmc::D29 for alt::D29 {}
impl stm32_fmc::D30 for alt::D30 {}
impl stm32_fmc::D31 for alt::D31 {}
impl stm32_fmc::DA0 for alt::Da0 {}
impl stm32_fmc::DA1 for alt::Da1 {}
impl stm32_fmc::DA2 for alt::Da2 {}
impl stm32_fmc::DA3 for alt::Da3 {}
impl stm32_fmc::DA4 for alt::Da4 {}
impl stm32_fmc::DA5 for alt::Da5 {}
impl stm32_fmc::DA6 for alt::Da6 {}
impl stm32_fmc::DA7 for alt::Da7 {}
impl stm32_fmc::DA8 for alt::Da8 {}
impl stm32_fmc::DA9 for alt::Da9 {}
impl stm32_fmc::DA10 for alt::Da10 {}
impl stm32_fmc::DA11 for alt::Da11 {}
impl stm32_fmc::DA12 for alt::Da12 {}
impl stm32_fmc::DA13 for alt::Da13 {}
impl stm32_fmc::DA14 for alt::Da14 {}
impl stm32_fmc::DA15 for alt::Da15 {}
impl stm32_fmc::INT for alt::Int {}
impl stm32_fmc::NBL0 for alt::Nbl0 {}
impl stm32_fmc::NBL1 for alt::Nbl1 {}
impl stm32_fmc::NBL2 for alt::Nbl2 {}
impl stm32_fmc::NBL3 for alt::Nbl3 {}
impl stm32_fmc::NCE for alt::Nce {}
impl stm32_fmc::NE1 for alt::Ne1 {}
impl stm32_fmc::NE2 for alt::Ne2 {}
impl stm32_fmc::NE3 for alt::Ne3 {}
impl stm32_fmc::NE4 for alt::Ne4 {}
impl stm32_fmc::NL for alt::Nl {}
impl stm32_fmc::NOE for alt::Noe {}
impl stm32_fmc::NWAIT for alt::Nwait {}
impl stm32_fmc::NWE for alt::Nwe {}
impl stm32_fmc::SDCKE0 for alt::Sdcke0 {}
impl stm32_fmc::SDCKE1 for alt::Sdcke1 {}
impl stm32_fmc::SDCLK for alt::Sdclk {}
impl stm32_fmc::SDNCAS for alt::Sdncas {}
impl stm32_fmc::SDNE0 for alt::Sdne0 {}
impl stm32_fmc::SDNE1 for alt::Sdne1 {}
impl stm32_fmc::SDNRAS for alt::Sdnras {}
impl stm32_fmc::SDNWE for alt::Sdnwe {}
