use crate::gpio::gpiob::{PB5, PB6};
use crate::gpio::gpioc::{PC0, PC2, PC3};
use crate::gpio::gpioh::{PH2, PH3, PH5, PH6, PH7};
use crate::gpio::GpioExt;
use crate::stm32::FMC;

pub use crate::stm32::fmc::{
    sdcmr::MODE_AW,
    sdcr::{CAS_A, MWID_A, NB_A, NC_A, NR_A, RPIPE_A, SDCLK_A},
    SDCMR, SDCR, SDRTR, SDSR, SDTR,
};

type FMCAF = crate::gpio::Alternate<crate::gpio::AF12>;
macro_rules! fmc_gpio {
    ($gpioX:ident.$pXY:ident) => {
        $gpioX
            .$pXY
            .into_alternate_af12()
            .set_speed(crate::gpio::Speed::High)
            .internal_pull_up(false)
    };
}

/// SDRAM address width
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SDRAMAddressWidth {
    /// 11-bit address size (A0 - A10)
    AW11 = 0,
    /// 12-bit address size (A0 - A11)
    AW12 = 1,
    /// 13-bit address size (A0 - A12)
    AW13 = 2,
}

/// Number of byte-enable pins (NBLx)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SDRAMByteEnableSelection {
    /// no byte-enable pins
    None = 0,
    /// two byte-enable pins (16-bit, NBL0 and NBL1)
    NBL2 = 1,
    /// four byte-enable pins (32-bit, NBL0 through NBL3)
    NBL4 = 2,
}

/// SDRAM data width
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SDRAMDataWidth {
    /// 8-bit data bus
    DW8,
    /// 16-bit data bus
    DW16 {
        /// whether or not to use NBL0 and NBL1
        /// (i.e., true is the same as SDRAMByteEnableSelection::NBL2)
        byte_enable_16: bool,
    },
    /// 32-bit data bus
    DW32 {
        /// how many NBLx pins to use
        byte_enable_selection: SDRAMByteEnableSelection,
    },
}

impl From<SDRAMDataWidth> for MWID_A {
    fn from(sdw: SDRAMDataWidth) -> Self {
        match sdw {
            SDRAMDataWidth::DW8 => MWID_A::BITS8,
            SDRAMDataWidth::DW16 { .. } => MWID_A::BITS16,
            SDRAMDataWidth::DW32 { .. } => MWID_A::BITS32,
        }
    }
}

/// (internal) which SDRAM bank to use
/// determined based on which combination of
/// SDCKE and SDNE pins are used (i.e., 0+0 uses Bank1, 1+1 uses Bank2)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SDRAMInternalBankSelection {
    Bank1,
    Bank2,
}

/// SDRAM CAS Latency selection
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CASLatency {
    /// 1 memory clock cycle
    CL1,
    /// 2 memory clock cycles
    CL2,
    /// 3 memory clock cycles
    CL3,
}

/// SDRAM Common Clock selection
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SDRAMCommonClock {
    /// SDRAM Common Clock disabled
    Disabled,
    /// 2 HCLK Cycles
    HCLK2,
    /// 3 HCLK Cycles
    HCLK3,
}

/// SDRAM Common Read Pipe Delay selection
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SDRAMCommonReadPipeDelay {
    /// 0 HCLK Cycles
    HCLK0,
    /// 1 HCLK Cycle
    HCLK1,
    /// 2 HCLK Cycles
    HCLK2,
}

macro_rules! fmc_pinswappable {
    ($SD_PIN_NAME:ident, [ $($GPIO_PIN_TYPE:ty),+ ]) => {
        pub trait $SD_PIN_NAME {
            fn set_fmc_speed_and_pullup(self) -> Self;
        }
        $(
            impl $SD_PIN_NAME for $GPIO_PIN_TYPE {
                fn set_fmc_speed_and_pullup(self) -> Self {
                    self.set_speed(crate::gpio::Speed::High).internal_pull_up(false)
                }
            }
        )+
    }
}

fmc_pinswappable!(PinSDNWE,  [PC0<FMCAF>, PH5<FMCAF>]);
fmc_pinswappable!(PinSDCKE0, [PH2<FMCAF>, PC3<FMCAF>]);
fmc_pinswappable!(PinSDCKE1, [PH7<FMCAF>, PB5<FMCAF>]);
fmc_pinswappable!(PinSDNE0,  [PH3<FMCAF>, PC2<FMCAF>]);
fmc_pinswappable!(PinSDNE1,  [PH6<FMCAF>, PB6<FMCAF>]);

pub trait SDRAMClockAndChipEnable {
    fn sdram_bank_selection() -> SDRAMInternalBankSelection;
    fn set_fmc_speed_and_pullup(self) -> Self;
}

pub struct UseSDRAMBank1<SDCKE0: PinSDCKE0, SDNE0: PinSDNE0>(pub SDCKE0, pub SDNE0);
pub struct UseSDRAMBank2<SDCKE1: PinSDCKE1, SDNE1: PinSDNE1>(pub SDCKE1, pub SDNE1);

impl<SDCKE0: PinSDCKE0, SDNE0: PinSDNE0> SDRAMClockAndChipEnable for UseSDRAMBank1<SDCKE0, SDNE0> {
    fn sdram_bank_selection() -> SDRAMInternalBankSelection {
        SDRAMInternalBankSelection::Bank1
    }

    fn set_fmc_speed_and_pullup(self) -> Self {
        UseSDRAMBank1(
            self.0.set_fmc_speed_and_pullup(),
            self.1.set_fmc_speed_and_pullup(),
        )
    }
}

impl<SDCKE1: PinSDCKE1, SDNE1: PinSDNE1> SDRAMClockAndChipEnable for UseSDRAMBank2<SDCKE1, SDNE1> {
    fn sdram_bank_selection() -> SDRAMInternalBankSelection {
        SDRAMInternalBankSelection::Bank2
    }

    fn set_fmc_speed_and_pullup(self) -> Self {
        UseSDRAMBank2(
            self.0.set_fmc_speed_and_pullup(),
            self.1.set_fmc_speed_and_pullup(),
        )
    }
}

pub struct SDRAMConfig {
    /// width of the address bus
    pub address_width: SDRAMAddressWidth,

    /// width of the data bus
    pub data_width: SDRAMDataWidth,

    pub number_of_column_bits: NC_A,
    pub number_of_row_bits: NR_A,

    /// number of banks in module (BA0/BA1 pins)
    pub module_bank_count: NB_A,

    pub sdclk_period: SDCLK_A,
    pub rpipe_delay: RPIPE_A,
    pub write_protection: bool,
    pub read_burst: bool,
    pub timing: SDRAMTiming,
}

/// Timing configurations for SDRAM. All units in SDRAM clocks (confirm?)
/// todo: use a nibble type for these if possible...
pub struct SDRAMTiming {
    pub cas_latency: CAS_A,
    /// LOAD MODE REGISTER command to ACTIVE command delay (sometimes called t_mrd).
    /// "Load mode register to active delay" in CubeMX
    pub t_mrd: u8,
    /// Exit SELF_REFRESH-to-ACTIVE command delay
    /// "Exit self-refresh delay" in CubeMX
    pub t_xsr: u8,
    /// Self refresh time (as named in CubeMX, tRAS in some datasheets)
    pub t_ras: u8,
    /// ACTIVE-to-ACTIVE command period (sometimes called tRC)
    /// "SDRAM common row cycle delay" in CubeMX
    pub t_rc: u8,
    /// WRITE recovery time (as named in CubeMX, tWR in datasheets)
    pub t_wr: u8,
    /// PRECHARGE command period (tRP in datasheets)
    /// "SDRAM common row precharge delay" in CubeMX
    pub t_rp: u8,
    /// ACTIVE-to-READ or Write delay (tRCD in datasheets)
    /// "Row to column delay" in CubeMX
    pub t_rcd: u8,
}

impl SDRAMConfig {
    fn configure_gpio<CCE: SDRAMClockAndChipEnable, SDNWE: PinSDNWE>(
        &self,
        cce: CCE,
        sdnwe: SDNWE,
    ) {
        let peripherals = unsafe { crate::stm32::Peripherals::steal() };
        let gpiod = peripherals.GPIOD.split();
        let gpioe = peripherals.GPIOE.split();
        let gpiof = peripherals.GPIOF.split();
        let gpiog = peripherals.GPIOG.split();
        let gpioh = peripherals.GPIOH.split();
        let gpioi = peripherals.GPIOI.split();

        // always configure pin for address lines A0-A10 (minimum address size is 11-bit)
        fmc_gpio!(gpiof.pf0); // PF0 -> A0
        fmc_gpio!(gpiof.pf1); // PF1 -> A1
        fmc_gpio!(gpiof.pf2); // PF2 -> A2
        fmc_gpio!(gpiof.pf3); // PF3 -> A3
        fmc_gpio!(gpiof.pf4); // PF4 -> A4
        fmc_gpio!(gpiof.pf5); // PF5 -> A5
        fmc_gpio!(gpiof.pf12); // PF12 -> A6
        fmc_gpio!(gpiof.pf13); // PF13 -> A7
        fmc_gpio!(gpiof.pf14); // PF14 -> A8
        fmc_gpio!(gpiof.pf15); // PF15 -> A9
        fmc_gpio!(gpiog.pg0); // PG0  -> A10

        // configure gpio line A11 for 12 or 13 bit
        if self.address_width >= SDRAMAddressWidth::AW12 {
            fmc_gpio!(gpiog.pg1); // PG1 -> A11
        }

        // configure gpio line A12 for 13-bit
        if self.address_width == SDRAMAddressWidth::AW13 {
            fmc_gpio!(gpiog.pg2); // PG2 -> A12
        }

        // we either have BA0 only or BA0 + BA1 (Bank Access)
        // this is determined by SDRAMModuleBankCount
        fmc_gpio!(gpiog.pg4); // always have BA0

        // BA1 if we need it
        if self.module_bank_count == NB_A::NB4 {
            fmc_gpio!(gpiog.pg5);
        }

        // always have D0-D7
        fmc_gpio!(gpiod.pd14); // PD14 -> D0
        fmc_gpio!(gpiod.pd15); // PD15 -> D1
        fmc_gpio!(gpiod.pd0); // PD0 -> D2
        fmc_gpio!(gpiod.pd1); // PD1 -> D3
        fmc_gpio!(gpioe.pe7); // PE7 -> D4
        fmc_gpio!(gpioe.pe8); // PE8 -> D5
        fmc_gpio!(gpioe.pe9); // PE9 -> D6
        fmc_gpio!(gpioe.pe10); // PE10 -> D7

        // figure out what other data lines we need and which NBL lines we need
        let (data_width, normalized_nbl) = match self.data_width {
            SDRAMDataWidth::DW8 => (8, SDRAMByteEnableSelection::None),
            SDRAMDataWidth::DW16 { byte_enable_16 } => (
                16u8,
                if byte_enable_16 {
                    SDRAMByteEnableSelection::NBL2
                } else {
                    SDRAMByteEnableSelection::None
                },
            ),
            SDRAMDataWidth::DW32 {
                byte_enable_selection,
            } => (32u8, byte_enable_selection),
        };

        if data_width >= 16 {
            fmc_gpio!(gpioe.pe11); // PE11 -> D8
            fmc_gpio!(gpioe.pe12); // PE12 -> D9
            fmc_gpio!(gpioe.pe13); // PE13 -> D10
            fmc_gpio!(gpioe.pe14); // PE14 -> D11
            fmc_gpio!(gpioe.pe15); // PE15 -> D12
            fmc_gpio!(gpiod.pd8); // PD8 -> D13
            fmc_gpio!(gpiod.pd9); // PD9 -> D14
            fmc_gpio!(gpiod.pd10); // PD10 -> D15
        }

        if data_width == 32 {
            fmc_gpio!(gpioh.ph8); // PH8 -> D16
            fmc_gpio!(gpioh.ph9); // PH9 -> D17
            fmc_gpio!(gpioh.ph10); // PH10 -> D18
            fmc_gpio!(gpioh.ph11); // PH11 -> D19
            fmc_gpio!(gpioh.ph12); // PH12 -> D20
            fmc_gpio!(gpioh.ph13); // PH13 -> D21
            fmc_gpio!(gpioh.ph14); // PH14 -> D22
            fmc_gpio!(gpioh.ph15); // PH15 -> D23
            fmc_gpio!(gpioi.pi0); // PI0 -> D24
            fmc_gpio!(gpioi.pi1); // PI1 -> D25
            fmc_gpio!(gpioi.pi2); // PI2 -> D26
            fmc_gpio!(gpioi.pi3); // PI3 -> D27
            fmc_gpio!(gpioi.pi6); // PI6 -> D28
            fmc_gpio!(gpioi.pi7); // PI0 -> D29
            fmc_gpio!(gpioi.pi9); // PI9 -> D30
            fmc_gpio!(gpioi.pi10); // PI10 -> D31
        }

        if normalized_nbl >= SDRAMByteEnableSelection::NBL2 {
            fmc_gpio!(gpioe.pe0); // PE0 -> NBL0
            fmc_gpio!(gpioe.pe1); // PE1 -> NBL1
        }

        if normalized_nbl == SDRAMByteEnableSelection::NBL4 {
            fmc_gpio!(gpioi.pi4); // PI4 -> NBL2
            fmc_gpio!(gpioi.pi5); // PI5 -> NBL3
        }

        fmc_gpio!(gpiog.pg8); // PG8 -> SDCLK
        fmc_gpio!(gpiof.pf11); // PF11 -> SDNRAS
        fmc_gpio!(gpiog.pg15); // PG15 -> SDNCAS

        // note that SDCKEx, SDNEx, and and SDNWE pins will have already been configured
        // as AFs in order to construct an SDRAMConfig anyway, but nothing in the type system
        // guarantees that the appropriate speed and pullup was set
        cce.set_fmc_speed_and_pullup();
        sdnwe.set_fmc_speed_and_pullup();
    }

    fn configure_sdcr<CCE: SDRAMClockAndChipEnable>(&self) {
        let peripherals = unsafe { crate::stm32::Peripherals::steal() };
        let fmc = peripherals.FMC;
        let mwid = self.data_width.into();

        // going off the HAL code, seems like if you use Bank1, you just set everything in SDCR1
        // but if you use Bank2 then sdclk, rburst, and rpipe go in SDCR1 and everything else into SDCR2
        // todo: STM32 LL api clears all of these (i.e., sets to 0 regardless of what the startup default is),
        // todo: including parts of SDCR2 that we don't touch here. This should probably be verified...

        // apply remaining values to the appropriate SDCR
        match CCE::sdram_bank_selection() {
            SDRAMInternalBankSelection::Bank1 => {
                fmc.sdcr1.write_with_zero(|w| {
                    w.sdclk()
                        .variant(self.sdclk_period)
                        .rburst()
                        .bit(self.read_burst)
                        .rpipe()
                        .variant(self.rpipe_delay)
                        .nc()
                        .variant(self.number_of_column_bits)
                        .nr()
                        .variant(self.number_of_row_bits)
                        .mwid()
                        .variant(mwid)
                        .nb()
                        .variant(self.module_bank_count)
                        .cas()
                        .variant(self.timing.cas_latency)
                        .wp()
                        .bit(self.write_protection)
                });
            }
            SDRAMInternalBankSelection::Bank2 => {
                fmc.sdcr1.modify(|_, w| {
                    w.sdclk()
                        .variant(self.sdclk_period)
                        .rburst()
                        .bit(self.read_burst)
                        .rpipe()
                        .variant(self.rpipe_delay)
                });
                fmc.sdcr2.write_with_zero(|w| {
                    w.nc()
                        .variant(self.number_of_column_bits)
                        .nr()
                        .variant(self.number_of_row_bits)
                        .mwid()
                        .variant(mwid)
                        .nb()
                        .variant(self.module_bank_count)
                        .cas()
                        .variant(self.timing.cas_latency)
                        .wp()
                        .bit(self.write_protection)
                });
            }
        };
    }

    fn configure_sdtr<CCE: SDRAMClockAndChipEnable>(&self) {
        let peripherals = unsafe { crate::stm32::Peripherals::steal() };
        let fmc = peripherals.FMC;

        // similar to SDCR above, TRC and TRP always get set in Bank 1 even if you're using bank 2
        // also we do a bunch of "-1" to the values specified in timing, this is to maintain
        // parity with HAL (i.e., you specify actual desired timings, HAL makes sure the FMC uses them)
        match CCE::sdram_bank_selection() {
            SDRAMInternalBankSelection::Bank1 => {
                fmc.sdtr1.write_with_zero(|w| {
                    w.trc()
                        .bits(self.timing.t_rc - 1)
                        .trp()
                        .bits(self.timing.t_rp - 1)
                        .tmrd()
                        .bits(self.timing.t_mrd - 1)
                        .txsr()
                        .bits(self.timing.t_xsr - 1)
                        .tras()
                        .bits(self.timing.t_ras - 1)
                        .twr()
                        .bits(self.timing.t_wr - 1)
                        .trcd()
                        .bits(self.timing.t_rcd - 1)
                });
            }
            SDRAMInternalBankSelection::Bank2 => {
                fmc.sdtr1.modify(|_, w| {
                    w.trc()
                        .bits(self.timing.t_rc - 1)
                        .trp()
                        .bits(self.timing.t_rp - 1)
                });
                fmc.sdtr2.write_with_zero(|w| {
                    w.tmrd()
                        .bits(self.timing.t_mrd - 1)
                        .txsr()
                        .bits(self.timing.t_xsr - 1)
                        .tras()
                        .bits(self.timing.t_ras - 1)
                        .twr()
                        .bits(self.timing.t_wr - 1)
                        .trcd()
                        .bits(self.timing.t_rcd - 1)
                });
            }
        };
    }

    pub fn configure<CCE: SDRAMClockAndChipEnable, SDNWE: PinSDNWE>(
        self,
        clock_and_chip_enable: CCE,
        sdnwe: SDNWE,
    ) -> SDRAM {
        let peripherals = unsafe { crate::stm32::Peripherals::steal() };
        let rcc = peripherals.RCC;

        // Enable the FMC peripheral clock, appears to be the only bit in AHB3ENR
        rcc.ahb3enr.modify(|_, w| w.fmcen().enabled());

        self.configure_gpio(clock_and_chip_enable, sdnwe);
        self.configure_sdcr::<CCE>();
        self.configure_sdtr::<CCE>();

        SDRAM {
            bank_selection: CCE::sdram_bank_selection(),
        }
    }
}

pub struct SDRAM {
    bank_selection: SDRAMInternalBankSelection,
}

impl SDRAM {
    pub fn send_command(&mut self, cmd: SDRAMCommand) {
        unsafe {
            let fmc = &*FMC::ptr();
            let (ctb1, ctb2) = match cmd.target_bank {
                SDRAMCommandTargetBank::None => (false, false),
                SDRAMCommandTargetBank::Bank1 => (true, false),
                SDRAMCommandTargetBank::Bank2 => (false, true),
                SDRAMCommandTargetBank::Both => (true, true),
            };
            fmc.sdcmr.write_with_zero(|w| {
                w.mode()
                    .variant(cmd.command_mode)
                    .ctb1()
                    .bit(ctb1)
                    .ctb2()
                    .bit(ctb2)
                    .nrfs()
                    .bits(cmd.auto_refresh_number)
                    .mrd()
                    .bits(cmd.mode_register_definition)
            });
            loop {
                if !fmc.sdsr.read().busy().is_busy() {
                    break;
                }
            }
        }
    }

    pub fn is_write_protect(&self) -> bool {
        let fmc = unsafe { &*FMC::ptr() };
        let sdcr = match self.bank_selection {
            SDRAMInternalBankSelection::Bank1 => &fmc.sdcr1,
            SDRAMInternalBankSelection::Bank2 => &fmc.sdcr2,
        };
        sdcr.read().wp().bit()
    }

    pub fn set_write_protect(&mut self, enabled: bool) {
        let fmc = unsafe { &*FMC::ptr() };
        let sdcr = match self.bank_selection {
            SDRAMInternalBankSelection::Bank1 => &fmc.sdcr1,
            SDRAMInternalBankSelection::Bank2 => &fmc.sdcr2,
        };
        sdcr.modify(|_, w| w.wp().bit(enabled))
    }

    pub fn set_autorefresh_number(&mut self, nrfs: u8) {
        let fmc = unsafe { &*FMC::ptr() };
        fmc.sdcmr.modify(|_, w| w.nrfs().bits(nrfs))
    }

    pub fn program_refresh_rate(&mut self, refresh_rate: u16) {
        let fmc = unsafe { &*FMC::ptr() };
        fmc.sdrtr.modify(|_, w| w.count().bits(refresh_rate))
    }

    pub fn sliced<'a, T: Sized>(&self, memory_length: usize) -> &'a mut [T] {
        // todo: this is true for f427/f429, dunno how it translates to other devices with FMC
        // todo: confirm that using SDRAM bank 1 means accessing A000_0000-CFFF_FFFF (FMC Block 5)
        // todo: Can't help but feel like it should be 0x80000000 (Block 4)...
        let base_address: u32 = match self.bank_selection {
            SDRAMInternalBankSelection::Bank1 => 0xA000_0000,
            SDRAMInternalBankSelection::Bank2 => 0xD000_0000,
        };

        unsafe {
            core::slice::from_raw_parts_mut(
                base_address as *mut T,
                memory_length / core::mem::size_of::<T>(),
            )
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SDRAMCommandTargetBank {
    None,
    Bank1,
    Bank2,
    Both,
}

pub struct SDRAMCommand {
    pub command_mode: MODE_AW,
    pub target_bank: SDRAMCommandTargetBank, // todo: might be redundant, as we have SDRAM::BankSelection to determine this...
    pub auto_refresh_number: u8,
    pub mode_register_definition: u16,
}
