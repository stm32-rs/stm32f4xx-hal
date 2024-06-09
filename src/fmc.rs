//! HAL for Flexible memory controller (FMC)
//!
//! See the stm32-fmc [usage guide](https://github.com/stm32-rs/stm32-fmc#usage)

// From stm32_fmc

use stm32_fmc::FmcPeripheral;
use stm32_fmc::{AddressPinSet, PinsSdram, Sdram, SdramChip, SdramPinSet, SdramTargetBank};

use crate::rcc::{BusClock, Clocks, Enable, Reset};
use fugit::HertzU32 as Hertz;

use crate::gpio::alt::fmc as alt;

#[cfg(feature = "fmc")]
use crate::pac::FMC as FMC_PER;
#[cfg(feature = "fsmc")]
use crate::pac::FSMC as FMC_PER;

/// Storage type for Flexible Memory Controller and its clocks
pub struct FMC {
    pub fmc: FMC_PER,
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

impl FmcExt for FMC_PER {
    /// New FMC instance
    fn fmc(self, clocks: &Clocks) -> FMC {
        FMC {
            fmc: self,
            hclk: FMC_PER::clock(clocks),
        }
    }
}

unsafe impl FmcPeripheral for FMC {
    const REGISTERS: *const () = FMC_PER::ptr() as *const ();

    fn enable(&mut self) {
        // TODO : change it to something safe ...
        unsafe {
            // Enable FMC
            FMC_PER::enable_unchecked();
            // Reset FMC
            FMC_PER::reset_unchecked();
        }
    }

    fn source_clock_hz(&self) -> u32 {
        // FMC block is clocked by HCLK
        self.hclk.raw()
    }
}

macro_rules! pins {
    ($($F:ident: $P:ident;)+) => {
        $(
            impl stm32_fmc::$F for alt::$P {}
        )+
    }
}

pins! {
    A0: A0;
    A1: A1;
    A2: A2;
    A3: A3;
    A4: A4;
    A5: A5;
    A6: A6;
    A7: A7;
    A8: A8;
    A9: A9;
    A10: A10;
    A11: A11;
    A12: A12;
    A13: A13;
    A14: A14;
    A15: A15;
    A16: A16;
    A17: A17;
    A18: A18;
    A19: A19;
    A20: A20;
    A21: A21;
    A22: A22;
    A23: A23;
    A24: A24;
    CLK: Clk;
    D0: D0;
    D1: D1;
    D2: D2;
    D3: D3;
    D4: D4;
    D5: D5;
    D6: D6;
    D7: D7;
    D8: D8;
    D9: D9;
    D10: D10;
    D11: D11;
    D12: D12;
    D13: D13;
    D14: D14;
    D15: D15;
    DA0: Da0;
    DA1: Da1;
    DA2: Da2;
    DA3: Da3;
    DA4: Da4;
    DA5: Da5;
    DA6: Da6;
    DA7: Da7;
    DA8: Da8;
    DA9: Da9;
    DA10: Da10;
    DA11: Da11;
    DA12: Da12;
    DA13: Da13;
    DA14: Da14;
    DA15: Da15;
    NBL0: Nbl0;
    NBL1: Nbl1;
    NE1: Ne1;
    NE2: Ne2;
    NE3: Ne3;
    NE4: Ne4;
    NL: Nl;
    NOE: Noe;
    NWAIT: Nwait;
    NWE: Nwe;
}

#[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
pins! {
    BA0: Ba0;
    BA1: Ba1;
    SDCKE0: Sdcke0;
    SDCKE1: Sdcke1;
    SDCLK: Sdclk;
    SDNCAS: Sdncas;
    SDNE0: Sdne0;
    SDNE1: Sdne1;
    SDNRAS: Sdnras;
    SDNWE: Sdnwe;
}

#[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
pins! {
    D16: D16;
    D17: D17;
    D18: D18;
    D19: D19;
    D20: D20;
    D21: D21;
    D22: D22;
    D23: D23;
    D24: D24;
    D25: D25;
    D26: D26;
    D27: D27;
    D28: D28;
    D29: D29;
    D30: D30;
    D31: D31;
    NBL2: Nbl2;
    NBL3: Nbl3;
}

#[cfg(feature = "gpio-f469")]
pins! {
    INT: Int;
}

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
pins! {
    NCE: Nce3;
}
#[cfg(any(feature = "gpio-f417", feature = "gpio-f427"))]
pins! {
    NCE: Nce2;
}
