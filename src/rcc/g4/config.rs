use crate::time::Hertz;

/// Prescaler
#[derive(Clone, Copy)]
pub enum Prescaler {
    NotDivided,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
    Div512,
}

/// System clock mux source
pub enum SysClockSrc {
    PLL,
    HSI,
    HSE(Hertz),
}

/// Microcontroller clock output source
pub enum MCOSrc {
    LSI,
    PLL,
    SysClk,
    HSI,
    HSE,
    LSE,
}

/// Low-speed clocks output source
pub enum LSCOSrc {
    LSI,
    LSE,
}

/// PLL clock input source
#[derive(Clone, Copy)]
pub enum PLLSrc {
    HSI,
    HSE(Hertz),
    HSE_BYPASS(Hertz),
}

/// Divider for the PLL clock input (M)
/// This must be set based on the input clock to keep the PLL input frequency within the limits
/// specified in the datasheet.
#[derive(Clone, Copy)]
pub enum PllMDiv {
    DIV_1 = 0,
    DIV_2,
    DIV_3,
    DIV_4,
    DIV_5,
    DIV_6,
    DIV_7,
    DIV_8,
    DIV_9,
    DIV_10,
    DIV_11,
    DIV_12,
    DIV_13,
    DIV_14,
    DIV_15,
    DIV_16,
}

impl PllMDiv {
    pub fn divisor(&self) -> u32 {
        (self.clone() as u32) + 1
    }

    pub fn register_setting(&self) -> u8 {
        self.clone() as u8
    }
}

/// Divider for the PLL Q Output
#[derive(Clone, Copy)]
pub enum PllQDiv {
    DIV_2 = 0,
    DIV_4,
    DIV_6,
    DIV_8,
}

impl PllQDiv {
    pub fn divisor(&self) -> u32 {
        ((self.clone() as u32) + 1) * 2
    }

    pub fn register_setting(&self) -> u8 {
        self.clone() as u8
    }
}

/// Divider for the PLL R Output
#[derive(Clone, Copy)]
pub enum PllRDiv {
    DIV_2 = 0,
    DIV_4,
    DIV_6,
    DIV_8,
}

impl PllRDiv {
    pub fn divisor(&self) -> u32 {
        ((self.clone() as u32) + 1) * 2
    }

    pub fn register_setting(&self) -> u8 {
        self.clone() as u8
    }
}

/// Divider for the PLL P Output
///
/// Note: The P divider has a PLLP register that can be used to set the divider to either 7 or 17.
/// It is a complete mystery why anyone would want to do that instead of using the PLLPDIV register
/// so it's not supported.
#[derive(Clone, Copy)]
pub enum PllPDiv {
    DIV_2 = 2,
    DIV_3,
    DIV_4,
    DIV_5,
    DIV_6,
    DIV_7,
    DIV_8,
    DIV_9,
    DIV_10,
    DIV_11,
    DIV_12,
    DIV_13,
    DIV_14,
    DIV_15,
    DIV_16,
    DIV_17,
    DIV_18,
    DIV_19,
    DIV_20,
    DIV_21,
    DIV_22,
    DIV_23,
    DIV_24,
    DIV_25,
    DIV_26,
    DIV_27,
    DIV_28,
    DIV_29,
    DIV_30,
    DIV_31,
}

impl PllPDiv {
    pub fn divisor(&self) -> u32 {
        self.clone() as u32
    }

    pub fn register_setting(&self) -> u8 {
        self.clone() as u8
    }
}

/// Main PLL multiplication factor for VCO
#[derive(Clone, Copy)]
pub enum PllNMul {
    MUL_8 = 8,
    MUL_9,
    MUL_10,
    MUL_11,
    MUL_12,
    MUL_13,
    MUL_14,
    MUL_15,
    MUL_16,
    MUL_17,
    MUL_18,
    MUL_19,
    MUL_20,
    MUL_21,
    MUL_22,
    MUL_23,
    MUL_24,
    MUL_25,
    MUL_26,
    MUL_27,
    MUL_28,
    MUL_29,
    MUL_30,
    MUL_31,
    MUL_32,
    MUL_33,
    MUL_34,
    MUL_35,
    MUL_36,
    MUL_37,
    MUL_38,
    MUL_39,
    MUL_40,
    MUL_41,
    MUL_42,
    MUL_43,
    MUL_44,
    MUL_45,
    MUL_46,
    MUL_47,
    MUL_48,
    MUL_49,
    MUL_50,
    MUL_51,
    MUL_52,
    MUL_53,
    MUL_54,
    MUL_55,
    MUL_56,
    MUL_57,
    MUL_58,
    MUL_59,
    MUL_60,
    MUL_61,
    MUL_62,
    MUL_63,
    MUL_64,
    MUL_65,
    MUL_66,
    MUL_67,
    MUL_68,
    MUL_69,
    MUL_70,
    MUL_71,
    MUL_72,
    MUL_73,
    MUL_74,
    MUL_75,
    MUL_76,
    MUL_77,
    MUL_78,
    MUL_79,
    MUL_80,
    MUL_81,
    MUL_82,
    MUL_83,
    MUL_84,
    MUL_85,
    MUL_86,
    MUL_87,
    MUL_88,
    MUL_89,
    MUL_90,
    MUL_91,
    MUL_92,
    MUL_93,
    MUL_94,
    MUL_95,
    MUL_96,
    MUL_97,
    MUL_98,
    MUL_99,
    MUL_100,
    MUL_101,
    MUL_102,
    MUL_103,
    MUL_104,
    MUL_105,
    MUL_106,
    MUL_107,
    MUL_108,
    MUL_109,
    MUL_110,
    MUL_111,
    MUL_112,
    MUL_113,
    MUL_114,
    MUL_115,
    MUL_116,
    MUL_117,
    MUL_118,
    MUL_119,
    MUL_120,
    MUL_121,
    MUL_122,
    MUL_123,
    MUL_124,
    MUL_125,
    MUL_126,
    MUL_127,
}

impl PllNMul {
    pub fn multiplier(&self) -> u32 {
        self.clone() as u32
    }

    pub fn register_setting(&self) -> u8 {
        self.clone() as u8
    }
}

/// PLL config
#[derive(Clone, Copy)]
pub struct PllConfig {
    pub mux: PLLSrc,
    pub m: PllMDiv,
    pub n: PllNMul,
    pub r: Option<PllRDiv>,
    pub q: Option<PllQDiv>,
    pub p: Option<PllPDiv>,
}

impl Default for PllConfig {
    fn default() -> PllConfig {
        PllConfig {
            mux: PLLSrc::HSI,
            m: PllMDiv::DIV_2,
            n: PllNMul::MUL_8,
            r: Some(PllRDiv::DIV_2),
            q: None,
            p: None,
        }
    }
}

/// Clocks configutation
pub struct Config {
    pub(crate) sys_mux: SysClockSrc,
    pub(crate) pll_cfg: PllConfig,
    pub(crate) ahb_psc: Prescaler,
    pub(crate) apb1_psc: Prescaler,
    pub(crate) apb2_psc: Prescaler,
}

impl Config {
    pub fn new(mux: SysClockSrc) -> Self {
        Config::default().clock_src(mux)
    }

    pub fn pll() -> Self {
        Config::default().clock_src(SysClockSrc::PLL)
    }

    pub fn hsi() -> Self {
        Config::default().clock_src(SysClockSrc::HSI)
    }

    pub fn clock_src(mut self, mux: SysClockSrc) -> Self {
        self.sys_mux = mux;
        self
    }

    pub fn pll_cfg(mut self, cfg: PllConfig) -> Self {
        self.pll_cfg = cfg;
        self
    }

    pub fn ahb_psc(mut self, psc: Prescaler) -> Self {
        self.ahb_psc = psc;
        self
    }

    pub fn apb1_psc(mut self, psc: Prescaler) -> Self {
        self.apb1_psc = psc;
        self
    }

    pub fn apb2_psc(mut self, psc: Prescaler) -> Self {
        self.apb2_psc = psc;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            sys_mux: SysClockSrc::HSI,
            pll_cfg: PllConfig::default(),
            ahb_psc: Prescaler::NotDivided,
            apb1_psc: Prescaler::NotDivided,
            apb2_psc: Prescaler::NotDivided,
        }
    }
}
