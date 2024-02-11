//! Reset and clock control.

use core::cmp::min;

mod enable;

use super::{BusClock, BusTimerClock, RccBus};

#[cfg_attr(test, allow(unused_imports))]
use micromath::F32Ext;

use crate::pac::{rcc, FLASH, PWR, RCC};
use fugit::{HertzU32 as Hertz, RateExtU32};

/// Typical output frequency of the HSI oscillator.
const HSI_FREQUENCY: Hertz = Hertz::from_raw(16_000_000);

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb1: AHB1::new(),
            ahb2: AHB2::new(),
            ahb3: AHB3::new(),
            apb1: APB1::new(),
            apb2: APB2::new(),
            bdcr: BDCR::new(),
            cfgr: CFGR {
                hse: None,
                hse_bypass: false,
                hclk: None,
                sysclk: None,
                pclk1: None,
                pclk2: None,
                lse: None,
                lsi: None,
                use_pll: false,
                pll48clk: None,
                pllm: 2,
                plln: 50,
                pllp: PLLP::Div2,
                pllq: 2,
                use_pllsai: false,
                pllsain: 192,
                pllsaip: PLLSAIP::Div2,
                pllsaiq: 2,
                use_plli2s: false,
                plli2sr: 2,
                plli2sq: 2,
                plli2sn: 192,
                mco1: MCO1::Hsi,
                mco1pre: MCOPRE::Div1_no_div,
                mco2: MCO2::Sysclk,
                mco2pre: MCOPRE::Div1_no_div,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// Advanced High-Performance Bus 1 (AHB1) registers
    pub ahb1: AHB1,
    /// Advanced High-Performance Bus 2 (AHB2) registers
    pub ahb2: AHB2,
    /// Advanced High-Performance Bus 3 (AHB3) registers
    pub ahb3: AHB3,

    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    /// RCC Backup Domain
    pub bdcr: BDCR,
    pub cfgr: CFGR,
}

macro_rules! bus_struct {
    ($($busX:ident => ($EN:ident, $en:ident, $LPEN:ident, $lpen:ident, $RST:ident, $rst:ident, $doc:literal),)+) => {
        $(
            #[doc = $doc]
            pub struct $busX {
                _0: (),
            }

            impl $busX {
                pub(crate) fn new() -> Self {
                    Self { _0: () }
                }

                pub(crate) fn enr(&self) -> &rcc::$EN {
                    // NOTE(unsafe) this proxy grants exclusive access to this register
                    unsafe { &(*RCC::ptr()).$en }
                }

                pub(crate) fn lpenr(&self) -> &rcc::$LPEN {
                    // NOTE(unsafe) this proxy grants exclusive access to this register
                    unsafe { &(*RCC::ptr()).$lpen }
                }

                pub(crate) fn rstr(&self) -> &rcc::$RST {
                    // NOTE(unsafe) this proxy grants exclusive access to this register
                    unsafe { &(*RCC::ptr()).$rst }
                }
            }
        )+
    };
}

bus_struct! {
    APB1 => (APB1ENR, apb1enr, APB1LPENR, apb1lpenr, APB1RSTR, apb1rstr, "Advanced Peripheral Bus 1 (APB1) registers"),
    APB2 => (APB2ENR, apb2enr, APB2LPENR, apb2lpenr, APB2RSTR, apb2rstr, "Advanced Peripheral Bus 2 (APB2) registers"),
    AHB1 => (AHB1ENR, ahb1enr, AHB1LPENR, ahb1lpenr, AHB1RSTR, ahb1rstr, "Advanced High-performance Bus 1 (AHB1) registers"),
    AHB2 => (AHB2ENR, ahb2enr, AHB2LPENR, ahb2lpenr, AHB2RSTR, ahb2rstr, "Advanced High-performance Bus 2 (AHB2) registers"),
    AHB3 => (AHB3ENR, ahb3enr, AHB3LPENR, ahb3lpenr, AHB3RSTR, ahb3rstr, "Advanced High-performance Bus 3 (AHB3) registers"),
}

/// Backup Domain Control register (RCC_BDCR)
pub struct BDCR {
    _0: (),
}

impl BDCR {
    pub(crate) fn new() -> Self {
        Self { _0: () }
    }
}

/*impl HSEClock {
    /// Provide HSE frequency.
    ///
    /// # Panics
    ///
    /// Panics if the frequency is outside the valid range. The frequency must be between
    /// 4 MHz and 26 MHz in oscillator mode and between 1 MHz and 50 MHz in bypass mode.
    pub fn new(freq: Hertz, mode: HSEClockMode) -> Self {
        let valid_range = match mode {
            // Source: Datasheet DS12536 Rev 2, Table 38
            HSEClockMode::Oscillator => Hertz::MHz(4)..=Hertz::MHz(26),
            // Source: Datasheet DS12536 Rev 2, Table 40
            HSEClockMode::Bypass => Hertz::MHz(1)..=Hertz::MHz(50),
        };
        assert!(valid_range.contains(&freq));

        HSEClock { freq, mode }
    }
}*/

/// LSE clock mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEClockMode {
    /// Enable LSE oscillator to use external crystal or ceramic resonator.
    Oscillator,
    /// Bypass LSE oscillator to use external clock source.
    /// Use this if an external oscillator is used which is not connected to `OSC32_IN` such as a MEMS resonator.
    Bypass,
}

/// LSE Clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LSEClock {
    /// Input frequency.
    freq: Hertz,
    /// Mode.
    mode: LSEClockMode,
}

impl LSEClock {
    /// Provide LSE frequency.
    pub fn new(mode: LSEClockMode) -> Self {
        // Sets the LSE clock source to 32.768 kHz.
        LSEClock {
            freq: 32_768.Hz(),
            mode,
        }
    }
}

/// PLL P division factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLP {
    Div2 = 0b00,
    Div4 = 0b01,
    Div6 = 0b10,
    Div8 = 0b11,
}

/// MCO prescaler
///
/// Value on reset: No division
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCOPRE {
    /// No division
    Div1_no_div,
    /// Division by 2
    Div2,
    /// Division by 3
    Div3,
    /// Division by 4
    Div4,
    /// Division by 5
    Div5,
}

/// PLL48CLK clock source selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL48CLK {
    /// 48 MHz clock from PLLQ is selected
    Pllq,
    /// 48 MHz clock from PLLSAI is selected
    Pllsai,
}

/// PLLSAIP division factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAIP {
    Div2 = 0b00,
    Div4 = 0b01,
    Div6 = 0b10,
    Div8 = 0b11,
}

/// Microcontroller clock output 1
///
/// Value on reset: HSI
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCO1 {
    /// HSI clock selected
    Hsi,
    /// LSE oscillator selected
    Lse,
    /// HSE oscillator clock selected
    Hse,
    /// PLL clock selected
    Pll,
}

/// Microcontroller clock output 2
///
/// Value on reset: SYSCLK
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCO2 {
    /// System clock (SYSCLK) selected
    Sysclk,
    /// PLLI2S clock selected
    Plli2s,
    /// HSE oscillator clock selected
    Hse,
    /// PLL clock selected
    Pll,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VOSscale {
    PwrScale1,
    PwrScale2,
    PwrScale3,
}

impl Default for VOSscale {
    fn default() -> Self {
        VOSscale::PwrScale3
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct InternalRCCConfig {
    hpre: u8,
    ppre1: u8,
    ppre2: u8,
    flash_waitstates: u8,
    overdrive: bool,
    vos_scale: VOSscale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct FreqRequest {
    p: Option<(u32, u32)>,
    q: Option<(u32, u32)>,
}

/// Clock configuration register.
#[derive(Debug, PartialEq, Eq)]
pub struct CFGR {
    hse: Option<u32>,
    hse_bypass: bool,
    hclk: Option<u32>,
    sysclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    lse: Option<LSEClock>,
    lsi: Option<Hertz>,
    use_pll: bool,
    pll48clk: Option<PLL48CLK>,
    pllm: u8,
    plln: u16,
    pllp: PLLP,
    pllq: u8,
    use_pllsai: bool,
    pllsain: u16,
    pllsaip: PLLSAIP,
    pllsaiq: u8,
    use_plli2s: bool,
    plli2sr: u8,
    plli2sq: u8,
    plli2sn: u16,
    mco1: MCO1,
    mco1pre: MCOPRE,
    mco2: MCO2,
    mco2pre: MCOPRE,
}

impl CFGR {
    /// Configures the HSE oscillator.
    pub fn use_hse(mut self, hse: Hertz) -> Self {
        self.hse = Some(hse.raw());
        self
    }

    /// Sets HCLK frequency.
    ///
    /// The HCLK is used for the AHB bus, core, memory and DMA.
    ///
    /// # Panics
    ///
    /// Panics if the frequency is larger than 216 MHz.
    pub fn hclk(mut self, freq: Hertz) -> Self {
        assert!(freq.raw() <= 216_000_000);

        self.hclk = Some(freq.raw());
        self
    }

    /// Sets the SYSCLK frequency.
    ///
    /// This sets the SYSCLK frequency and sets up the USB clock if defined.
    /// The provided frequency must be between 12.5 Mhz and 216 Mhz.
    /// 12.5 Mhz is the VCO minimum frequency and SYSCLK PLLP divider limitation.
    /// If the ethernet peripheral is on, the user should set a frequency higher than 25 Mhz.
    ///
    /// # Panics
    ///
    /// Panics if the frequency is not between 12.5 MHz and 216 MHz.
    pub fn sysclk(mut self, sysclk: Hertz) -> Self {
        assert!((12_500_000..=216_000_000).contains(&sysclk.raw()));

        self.sysclk = Some(sysclk.raw());
        self
    }

    /// Sets the PCLK1 clock (APB1 clock).
    ///
    /// If this method isn't called the maximum allowed frequency is used for PCLK1.
    ///
    /// # Panics
    ///
    /// Panics if the frequency is not between 12.5 MHz and 54 MHz.
    pub fn pclk1(mut self, freq: Hertz) -> Self {
        assert!((12_500_000..=54_000_000).contains(&freq.raw()));

        self.pclk1 = Some(freq.raw());
        self
    }

    /// Sets PCLK2 clock (APB2 clock).
    ///
    /// If this method isn't called the maximum allowed frequency is used for PCLK2.
    ///
    /// # Panics
    ///
    /// Panics if the frequency is not between 12.5 MHz and 108 MHz.
    pub fn pclk2(mut self, freq: Hertz) -> Self {
        assert!((12_500_000..=108_000_000).contains(&freq.raw()));

        self.pclk2 = Some(freq.raw());
        self
    }

    /// Sets the LSE clock source to 32.768 kHz.
    pub fn lse(mut self, lse: LSEClock) -> Self {
        self.lse = Some(lse);
        self
    }

    /// Sets the LSI clock source to 32 kHz.
    ///
    /// Be aware that the tolerance is up to ±47% (Min 17 kHz, Typ 32 kHz, Max 47 kHz).
    pub fn lsi(mut self) -> Self {
        self.lsi = Some(32.kHz());
        self
    }

    /// Sets the SYSCLK clock source to the main PLL.
    ///
    /// Note: `sysclk` must be specified or `use_pll48clk` must be set to true, otherwise `use_pll` is reset to false.
    pub fn use_pll(mut self) -> Self {
        self.use_pll = true;
        self
    }

    /// Sets the 48 MHz clock source.
    pub fn use_pll48clk(mut self, pll48clk: PLL48CLK) -> Self {
        self.pll48clk = Some(pll48clk);
        self
    }

    /// Sets the common PLL division factor.
    ///
    /// # Panics
    ///
    /// Panics if the division factor isn't between 2 and 63.
    pub fn pllm(mut self, pllm: u8) -> Self {
        assert!((2..=63).contains(&pllm));
        self.pllm = pllm;
        self
    }

    /// Sets the PLL multiplication factor for the main PLL.
    ///
    /// # Panics
    ///
    /// Panics if the multiplication factor isn't between 50 and 432 (inclusive).
    pub fn plln(mut self, plln: u16) -> Self {
        assert!((50..=432).contains(&plln));
        self.plln = plln;
        self
    }

    /// Sets the PLL division factor for the main PLL.
    pub fn pllp(mut self, pllp: PLLP) -> Self {
        self.pllp = pllp;
        self
    }

    /// Sets the PLL division factor for the 48 MHz clock.
    /// # Panics
    ///
    /// Panics if the division factor isn't between 2 and 15 (inclusive).
    pub fn pllq(mut self, pllq: u8) -> Self {
        assert!((2..=15).contains(&pllq));
        self.pllq = pllq;
        self
    }

    /// Enables the PLLSAI clock source.
    pub fn use_pllsai(mut self) -> Self {
        self.use_pllsai = true;
        self
    }

    /// Sets the PLLSAIN multiplication factor for PLLSAI.
    ///
    /// # Panics
    ///
    /// Panics if the multiplication factor isn't between 50 and 432.
    pub fn pllsain(mut self, pllsain: u16) -> Self {
        assert!((50..=432).contains(&pllsain));
        self.pllsain = pllsain;
        self
    }

    /// Sets the PLLSAIP division factor for PLLSAI.
    pub fn pllsaip(mut self, pllsaip: PLLSAIP) -> Self {
        self.pllsaip = pllsaip;
        self
    }

    /// Sets the PLLSAIQ division factor for PLLSAIS.
    ///
    /// # Panics
    ///
    /// Panics if the division factor isn't between 2 and 15.
    pub fn pllsaiq(mut self, pllsaiq: u8) -> Self {
        assert!((2..=15).contains(&pllsaiq));
        self.pllsaiq = pllsaiq;
        self
    }

    /// Enables the PLLI2S clock source.
    pub fn use_plli2s(mut self) -> Self {
        self.use_plli2s = true;
        self
    }

    /// Sets the PLLI2SN multiplication factor for PLLI2S.
    ///
    /// # Panics
    ///
    /// Panics if the multiplication factor isn't between 50 and 432.
    pub fn plli2sn(mut self, plli2sn: u16) -> Self {
        assert!((50..=432).contains(&plli2sn));
        self.plli2sn = plli2sn;
        self
    }

    /// Sets the PLLI2SQ division factor for PLLI2S.
    ///
    /// # Panics
    ///
    /// Panics if the division factor isn't between 2 and 15.
    pub fn plli2sq(mut self, plli2sq: u8) -> Self {
        assert!((2..=15).contains(&plli2sq));
        self.plli2sq = plli2sq;
        self
    }

    /// Sets the PLLI2SR division factor for PLLI2S.
    ///
    /// # Panics
    ///
    /// Panics if the division factor isn't between 2 and 7.
    pub fn plli2sr(mut self, plli2sr: u8) -> Self {
        assert!((2..=7).contains(&plli2sr));
        self.plli2sr = plli2sr;
        self
    }

    /// Sets the MCO1 source
    pub fn mco1(mut self, mco1: MCO1) -> Self {
        self.mco1 = mco1;
        self
    }

    /// Sets the MCO1 division factors
    pub fn mco1pre(mut self, mco1pre: MCOPRE) -> Self {
        self.mco1pre = mco1pre;
        self
    }

    /// Sets the MCO2 source
    pub fn mco2(mut self, mco2: MCO2) -> Self {
        self.mco2 = mco2;
        self
    }

    /// Sets the MCO2 division factors
    pub fn mco2pre(mut self, mco2pre: MCOPRE) -> Self {
        self.mco2pre = mco2pre;
        self
    }

    /// Output clock calculation
    fn calculate_clocks(&self) -> (Clocks, InternalRCCConfig) {
        let mut config = InternalRCCConfig::default();

        let base_clk = u64::from(self.hse.unwrap_or(HSI_FREQUENCY.raw()));

        let mut sysclk = base_clk;

        let mut pll48clk_valid = false;

        if self.use_pll {
            sysclk = base_clk * self.plln as u64
                / self.pllm as u64
                / match self.pllp {
                    PLLP::Div2 => 2,
                    PLLP::Div4 => 4,
                    PLLP::Div6 => 6,
                    PLLP::Div8 => 8,
                };
        }

        // Check if pll48clk is valid
        if let Some(pll48clk) = self.pll48clk {
            match pll48clk {
                PLL48CLK::Pllq => {
                    pll48clk_valid = {
                        let pll48clk =
                            base_clk * self.plln as u64 / self.pllm as u64 / self.pllq as u64;
                        (48_000_000 - 120_000..=48_000_000 + 120_000).contains(&pll48clk)
                    }
                }
                PLL48CLK::Pllsai => {
                    pll48clk_valid = {
                        if self.use_pllsai {
                            let pll48clk = base_clk * self.pllsain as u64
                                / self.pllm as u64
                                / match self.pllsaip {
                                    PLLSAIP::Div2 => 2,
                                    PLLSAIP::Div4 => 4,
                                    PLLSAIP::Div6 => 6,
                                    PLLSAIP::Div8 => 8,
                                };
                            (48_000_000 - 120_000..=48_000_000 + 120_000).contains(&pll48clk)
                        } else {
                            false
                        }
                    }
                }
            }
        }

        // SYSCLK, must be <= 216 Mhz. By default, HSI/HSE frequency is chosen
        assert!(sysclk <= 216_000_000);
        let sysclk = sysclk as u32;

        // HCLK. By default, SYSCLK frequency is chosen. Because of the method
        // of clock multiplication and division, even if `sysclk` is set to be
        // the same as `hclk`, it can be slightly inferior to `sysclk` after
        // pllm, pllp... calculations
        let mut hclk: u32 = min(sysclk, self.hclk.unwrap_or(sysclk));

        // Configure HPRE.
        let hpre_val: f32 = (sysclk as f32 / hclk as f32).ceil();

        // The real value of hpre is computed to be as near as possible to the
        // desired value, this leads to a quantization error
        let (hpre_val, hpre): (f32, u8) = match hpre_val as u32 {
            0 => unreachable!(),
            1 => (1.0, 0b000),
            2 => (2.0, 0b1000),
            3..=5 => (4.0, 0b1001),
            6..=11 => (8.0, 0b1010),
            12..=39 => (16.0, 0b1011),
            40..=95 => (64.0, 0b1100),
            96..=191 => (128.0, 0b1101),
            192..=383 => (256.0, 0b1110),
            _ => (512.0, 0b1111),
        };
        config.hpre = hpre;
        // update hclk with the real value
        hclk = (sysclk as f32 / hpre_val).floor() as u32;

        // PCLK1 (APB1). Must be <= 54 Mhz. By default, min(hclk, 54Mhz) is
        // chosen
        // Add limits dependens on OD follows by DS Table 16.
        let max_pclk1 = if sysclk <= 180_000_000 {
            45_000_000
        } else {
            54_000_000
        };
        let mut pclk1: u32 = min(max_pclk1, self.pclk1.unwrap_or(hclk));
        // PCLK2 (APB2). Must be <= 108 Mhz. By default, min(hclk, 108Mhz) is
        // chosen
        // Add limits dependens on OD follows by DS Table 16.
        let max_pclk2 = if sysclk <= 180_000_000 {
            90_000_000
        } else {
            108_000_000
        };
        let mut pclk2: u32 = min(max_pclk2, self.pclk2.unwrap_or(hclk));

        // Configure PPRE1
        let mut ppre1_val: u32 = (hclk as f32 / pclk1 as f32).ceil() as u32;
        config.ppre1 = match ppre1_val {
            0 => unreachable!(),
            1 => {
                ppre1_val = 1;
                0b000
            }
            2 => {
                ppre1_val = 2;
                0b100
            }
            3..=6 => {
                ppre1_val = 4;
                0b101
            }
            7..=12 => {
                ppre1_val = 8;
                0b110
            }
            _ => {
                ppre1_val = 16;
                0b111
            }
        };
        // update pclk1 with the real value
        pclk1 = hclk / ppre1_val;

        // Configure PPRE2
        let mut ppre2_val: u32 = (hclk as f32 / pclk2 as f32).ceil() as u32;
        config.ppre2 = match ppre2_val {
            0 => unreachable!(),
            1 => {
                ppre2_val = 1;
                0b000
            }
            2 => {
                ppre2_val = 2;
                0b100
            }
            3..=6 => {
                ppre2_val = 4;
                0b101
            }
            7..=12 => {
                ppre2_val = 8;
                0b110
            }
            _ => {
                ppre2_val = 16;
                0b111
            }
        };
        // update pclk2 with the real value
        pclk2 = hclk / ppre2_val;

        // Assumes TIMPRE bit of RCC_DCKCFGR1 is reset (0)
        let timclk1 = if ppre1_val == 1 { pclk1 } else { 2 * pclk1 };
        let timclk2 = if ppre2_val == 1 { pclk2 } else { 2 * pclk2 };

        // Adjust flash wait states
        config.flash_waitstates = if sysclk <= 30_000_000 {
            0b0000
        } else if sysclk <= 60_000_000 {
            0b0001
        } else if sysclk <= 90_000_000 {
            0b0010
        } else if sysclk <= 120_000_000 {
            0b0011
        } else if sysclk <= 150_000_000 {
            0b0100
        } else if sysclk <= 180_000_000 {
            0b0101
        } else if sysclk <= 210_000_000 {
            0b0110
        } else {
            0b0111
        };
        // Adjust power state and overdrive mode
        // Configure follows by RM 4.1.4
        // Values getted from DS Table 16. General operating conditions
        config.vos_scale = if sysclk <= 144_000_000 {
            VOSscale::PwrScale3
        } else if sysclk <= 168_000_000 {
            VOSscale::PwrScale2
        } else {
            VOSscale::PwrScale1
        };
        // For every frequency higher than 180 need to enable overdrive
        // Follows by DS Table 16.
        config.overdrive = sysclk > 180_000_000;

        let clocks = Clocks {
            hclk: hclk.Hz(),
            pclk1: pclk1.Hz(),
            pclk2: pclk2.Hz(),
            sysclk: sysclk.Hz(),
            timclk1: timclk1.Hz(),
            timclk2: timclk2.Hz(),
            pll48clk_valid,
            hse: self.hse.map(|hse| hse.Hz()),
            lse: self.lse.map(|lse| lse.freq),
            lsi: self.lsi,
        };

        (clocks, config)
    }

    /// Calculate the PLL M, N, P and Q values from the provided clock and requested options.
    fn calculate_mnpq(
        f_pll_clock_input: u32,
        freq_req: FreqRequest,
    ) -> Option<(u32, u32, Option<u32>, Option<u32>)> {
        let mut m = 2;
        let mut n = 432;
        let mut p = None;
        let mut q = None;

        if freq_req.p.is_none() && freq_req.q.is_none() {
            return None;
        }

        loop {
            if m > 63 {
                return None;
            }
            let f_vco_input = f_pll_clock_input / m;
            if f_vco_input < 1_000_000 {
                return None;
            }
            if f_vco_input > 2_000_000 || n < 50 {
                m += 1;
                n = 432;
                continue;
            }
            let f_vco_clock = (f_pll_clock_input as u64 * n as u64 / m as u64) as u32;
            if f_vco_clock < 50_000_000 {
                m += 1;
                n = 432;
                continue;
            }
            if f_vco_clock > 432_000_000 {
                n -= 1;
                continue;
            }

            if let Some((p_freq_min, p_freq_max)) = freq_req.p {
                let mut div = None;
                for div_p in &[2, 4, 6, 8] {
                    let f_pll_clock_output = f_vco_clock / div_p;
                    if f_pll_clock_output >= p_freq_min && f_pll_clock_output <= p_freq_max {
                        div = Some(*div_p)
                    }
                }
                if div.is_some() {
                    p = div;
                    if freq_req.q.is_none() {
                        break;
                    }
                } else {
                    n -= 1;
                    continue;
                }
            }

            if let Some((q_freq_min, q_freq_max)) = freq_req.q {
                let mut div = None;
                for div_q in 2..=15 {
                    let f_usb_clock_output = f_vco_clock / div_q;
                    if f_usb_clock_output >= q_freq_min && f_usb_clock_output <= q_freq_max {
                        div = Some(div_q)
                    }
                }
                if div.is_some() {
                    q = div;
                    break;
                } else {
                    n -= 1;
                    continue;
                }
            }
        }

        Some((m, n, p, q))
    }

    fn pll_configure(&mut self) {
        let base_clk = self.hse.unwrap_or(HSI_FREQUENCY.raw());

        let sysclk = if let Some(clk) = self.sysclk {
            clk
        } else {
            base_clk
        };

        let p = if base_clk == sysclk {
            None
        } else {
            Some((sysclk - 1, sysclk + 1))
        };

        let q = if let Some(PLL48CLK::Pllq) = self.pll48clk {
            Some((48_000_000 - 120_000, 48_000_000 + 120_000))
        } else {
            None
        };

        if p.is_none() && q.is_none() {
            // We don't need PLL
            self.use_pll = false;
            return;
        }

        // We check if (pllm, plln, pllp) allow to obtain the requested Sysclk,
        // so that we don't have to calculate them
        let p_ok = (sysclk as u64)
            == (base_clk as u64 * self.plln as u64
                / self.pllm as u64
                / match self.pllp {
                    PLLP::Div2 => 2,
                    PLLP::Div4 => 4,
                    PLLP::Div6 => 6,
                    PLLP::Div8 => 8,
                });
        if p_ok && q.is_none() {
            return;
        }

        if let Some((m, n, p, q)) = CFGR::calculate_mnpq(base_clk, FreqRequest { p, q }) {
            self.pllm = m as u8;
            self.plln = n as u16;
            if let Some(p) = p {
                self.use_pll = true;
                self.pllp = match p {
                    2 => PLLP::Div2,
                    4 => PLLP::Div4,
                    6 => PLLP::Div6,
                    8 => PLLP::Div8,
                    _ => unreachable!(),
                };
            }
            if let Some(q) = q {
                self.pllq = q as u8;
            }
        } else {
            panic!("couldn't calculate {} from {}", sysclk, base_clk);
        }
    }

    /// Configures the default clock settings.
    ///
    /// Set SYSCLK as 216 Mhz and setup USB clock if defined.
    pub fn set_defaults(self) -> Self {
        self.sysclk(216.MHz())
    }

    /// Configure the "mandatory" clocks (`sysclk`, `hclk`, `pclk1` and `pclk2')
    /// and return them via the `Clocks` struct.
    ///
    /// The user shouldn't call freeze more than once as the clocks parameters
    /// cannot be changed after the clocks have started.
    ///
    /// The implementation makes the following choice: HSI is always chosen over
    /// HSE except when HSE is provided. When HSE is provided, HSE is used
    /// wherever it is possible.
    pub fn freeze(mut self) -> Clocks {
        let flash = unsafe { &(*FLASH::ptr()) };
        let rcc = unsafe { &(*RCC::ptr()) };
        let pwr = unsafe { &(*PWR::ptr()) };

        self.pll_configure();

        let (clocks, config) = self.calculate_clocks();

        // Switch to fail-safe clock settings.
        // This is useful when booting from a bootloader that alters clock tree configuration.
        // Turn on HSI
        rcc.cr.modify(|_, w| w.hsion().set_bit());
        while rcc.cr.read().hsirdy().bit_is_clear() {}
        // Switch to HSI
        rcc.cfgr.modify(|_, w| w.sw().hsi());

        // Configure HSE if provided
        if self.hse.is_some() {
            // Configure the HSE mode
            rcc.cr.modify(|_, w| {
                if self.hse_bypass {
                    w.hsebyp().bypassed()
                } else {
                    w.hsebyp().not_bypassed()
                }
            });
            // Start HSE
            rcc.cr.modify(|_, w| w.hseon().on());
            while rcc.cr.read().hserdy().is_not_ready() {}
        }

        // Enable sequence follows by RM 4.1.4 Entering Overdrive mode.
        if self.use_pll || self.pll48clk.is_some() {
            // Disable PLL
            // Since the main-PLL configuration parameters cannot be changed once PLL is enabled, it is
            // recommended to configure PLL before enabling it (selection of the HSI or HSE oscillator as
            // PLL clock source, and configuration of division factors M, N, P, and Q).
            rcc.cr.modify(|_, w| w.pllon().off());

            rcc.pllcfgr.modify(|_, w| unsafe {
                w.pllm().bits(self.pllm);
                w.plln().bits(self.plln);
                w.pllp().bits(self.pllp as u8);
                w.pllq().bits(self.pllq);
                w.pllsrc().bit(self.hse.is_some())
            });

            // Enable PWR domain and setup VOSscale and Overdrive options
            rcc.apb1enr.modify(|_, w| w.pwren().set_bit());

            pwr.cr1.modify(|_, w| match config.vos_scale {
                VOSscale::PwrScale3 => w.vos().scale3(),
                VOSscale::PwrScale2 => w.vos().scale2(),
                VOSscale::PwrScale1 => w.vos().scale1(),
            });

            // Enable PLL
            rcc.cr.modify(|_, w| w.pllon().on());

            // Wait for PLL to stabilise
            while rcc.cr.read().pllrdy().is_not_ready() {}

            //Over-drive
            if config.overdrive {
                // Entering Over-drive mode
                //enable the Over-drive mode
                pwr.cr1.modify(|_, w| w.oden().set_bit());

                //wait for the ODRDY flag to be set
                while !pwr.csr1.read().odrdy().bit_is_set() {}

                //switch the voltage regulator from Normal mode to Over-drive mode
                pwr.cr1.modify(|_, w| w.odswen().set_bit());

                //Wait for the ODSWRDY flag in the PWR_CSR1 to be set.
                while !pwr.csr1.read().odswrdy().bit_is_set() {}
            }
        }

        // Configure LSE if provided
        if self.lse.is_some() {
            // Configure the LSE mode
            match self.lse.as_ref().unwrap().mode {
                LSEClockMode::Bypass => rcc.bdcr.modify(|_, w| w.lsebyp().bypassed()),
                LSEClockMode::Oscillator => rcc.bdcr.modify(|_, w| w.lsebyp().not_bypassed()),
            }
            // Enable the LSE.
            rcc.bdcr.modify(|_, w| w.lseon().on());
            while rcc.bdcr.read().lserdy().is_not_ready() {}
        }

        if self.lsi.is_some() {
            rcc.csr.modify(|_, w| w.lsion().on());
            while rcc.csr.read().lsirdy().is_not_ready() {}
        }

        if self.use_pllsai {
            let pllsain_freq = match self.hse {
                Some(hse) => hse as u64 / self.pllm as u64 * self.pllsain as u64,
                None => 16_000_000 / self.pllm as u64 * self.pllsain as u64,
            };
            let pllsaip_freq = pllsain_freq
                / match self.pllsaip {
                    PLLSAIP::Div2 => 2,
                    PLLSAIP::Div4 => 4,
                    PLLSAIP::Div6 => 6,
                    PLLSAIP::Div8 => 8,
                };
            // let pllsaiq_freq = pllsain_freq / self.pllsaiq as u64;

            // The reference manual (RM0410 Rev 4, Page 212), says the following
            // "Caution: The software has to set these bits correctly to ensure that the VCO output frequency is between 100 and 432 MHz.",
            // but STM32CubeMX states 192 MHz as the minimum. SSo the stricter requirement was chosen.
            assert!((192_000_000..=432_000_000).contains(&pllsain_freq));
            assert!(pllsaip_freq <= 48_000_000);

            rcc.pllsaicfgr.modify(|_, w| unsafe {
                w.pllsain().bits(self.pllsain);
                w.pllsaip().bits(self.pllsaip as u8);
                w.pllsaiq().bits(self.pllsaiq)
            });
            rcc.cr.modify(|_, w| w.pllsaion().on());
        }

        if let Some(pll48clk) = self.pll48clk {
            match pll48clk {
                PLL48CLK::Pllq => rcc.dckcfgr2.modify(|_, w| w.ck48msel().bit(false)),
                PLL48CLK::Pllsai => rcc.dckcfgr2.modify(|_, w| w.ck48msel().bit(true)),
            }
        }

        if self.use_plli2s {
            let plli2sn_freq = match self.hse {
                Some(hse) => hse as u64 / self.pllm as u64 * self.plli2sn as u64,
                None => 16_000_000 / self.pllm as u64 * self.plli2sn as u64,
            };
            let plli2sr_freq = plli2sn_freq / self.plli2sr as u64;
            let plli2sq_freq = plli2sn_freq / self.plli2sq as u64;

            assert!((192_000_000..=432_000_000).contains(&plli2sn_freq));
            assert!(plli2sr_freq <= 216_000_000);
            assert!(plli2sq_freq <= 216_000_000);

            rcc.plli2scfgr.modify(|_, w| unsafe {
                w.plli2sn().bits(self.plli2sn);
                w.plli2sr().bits(self.plli2sr);
                w.plli2sq().bits(self.plli2sq)
            });
            rcc.cr.modify(|_, w| w.plli2son().on());
        }

        rcc.cfgr.modify(|_, w| {
            w.mco1()
                .variant(self.mco1.into())
                .mco1pre()
                .variant(self.mco1pre.into());
            w.mco2()
                .variant(self.mco2.into())
                .mco2pre()
                .variant(self.mco2pre.into())
        });

        flash
            .acr
            .write(|w| w.latency().bits(config.flash_waitstates));

        // Configure HCLK, PCLK1, PCLK2
        rcc.cfgr.modify(|_, w| unsafe {
            w.ppre1()
                .bits(config.ppre1)
                .ppre2()
                .bits(config.ppre2)
                .hpre()
                .bits(config.hpre)
        });

        // Select SYSCLK source
        if self.use_pll {
            rcc.cfgr.modify(|_, w| w.sw().pll());
            while !rcc.cfgr.read().sws().is_pll() {}
        } else if self.hse.is_some() {
            rcc.cfgr.modify(|_, w| w.sw().hse());
            while !rcc.cfgr.read().sws().is_hse() {}
        } else {
            rcc.cfgr.modify(|_, w| w.sw().hsi());
            while !rcc.cfgr.read().sws().is_hsi() {}
        }

        // As requested by user manual we need to wait 16 ticks before the right
        // predivision is applied
        cortex_m::asm::delay(16);

        clocks
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    sysclk: Hertz,
    timclk1: Hertz,
    timclk2: Hertz,
    pll48clk_valid: bool,
    hse: Option<Hertz>,
    lse: Option<Hertz>,
    lsi: Option<Hertz>,
}

impl Clocks {
    /// Returns the frequency of the AHB1
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the frequency for timers on APB1
    pub fn timclk1(&self) -> Hertz {
        self.timclk1
    }

    /// Returns the frequency for timers on APB1
    pub fn timclk2(&self) -> Hertz {
        self.timclk2
    }

    /// Returns true if the PLL48 clock is within USB
    /// specifications. It is required to use the USB functionality.
    pub fn is_pll48clk_valid(&self) -> bool {
        // USB specification allow +-0.25%
        self.pll48clk_valid
    }

    /// Returns the frequency of the `HSE` if `Some`, else `None`.
    pub fn hse(&self) -> Option<Hertz> {
        self.hse
    }

    /// Returns the frequency of the `LSE` if `Some`, else `None`.
    pub fn lse(&self) -> Option<Hertz> {
        self.lse
    }

    /// Returns the frequency of the `LSI` if `Some`, else `None`.
    pub fn lsi(&self) -> Option<Hertz> {
        self.lsi
    }
}

impl BusClock for AHB1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.hclk
    }
}

impl BusClock for AHB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.hclk
    }
}

impl BusClock for AHB3 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.hclk
    }
}

impl BusClock for APB1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk1
    }
}

impl BusClock for APB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk2
    }
}

impl BusTimerClock for APB1 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.timclk1
    }
}

impl BusTimerClock for APB2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.timclk2
    }
}

impl From<MCO1> for crate::pac::rcc::cfgr::MCO1_A {
    fn from(input: MCO1) -> Self {
        match input {
            MCO1::Hsi => Self::Hsi,
            MCO1::Lse => Self::Lse,
            MCO1::Hse => Self::Hse,
            MCO1::Pll => Self::Pll,
        }
    }
}

impl From<MCO2> for crate::pac::rcc::cfgr::MCO2_A {
    fn from(input: MCO2) -> Self {
        match input {
            MCO2::Sysclk => Self::Sysclk,
            MCO2::Plli2s => Self::Plli2s,
            MCO2::Hse => Self::Hse,
            MCO2::Pll => Self::Pll,
        }
    }
}

impl From<MCOPRE> for crate::pac::rcc::cfgr::MCO2PRE_A {
    fn from(input: MCOPRE) -> Self {
        match input {
            MCOPRE::Div1_no_div => Self::Div1,
            MCOPRE::Div2 => Self::Div2,
            MCOPRE::Div3 => Self::Div3,
            MCOPRE::Div4 => Self::Div4,
            MCOPRE::Div5 => Self::Div5,
        }
    }
}

/// Enable/disable peripheral
pub trait Enable: RccBus {
    /// Enables peripheral
    fn enable(bus: &mut Self::Bus);

    /// Disables peripheral
    fn disable(bus: &mut Self::Bus);

    /// Check if peripheral enabled
    fn is_enabled() -> bool;

    /// Check if peripheral disabled
    fn is_disabled() -> bool {
        !Self::is_enabled()
    }

    /// # Safety
    ///
    /// Enables peripheral. Takes access to RCC internally
    unsafe fn enable_unchecked();

    /// # Safety
    ///
    /// Disables peripheral. Takes access to RCC internally
    unsafe fn disable_unchecked();
}

/// Enable/disable peripheral in low power mode
pub trait LPEnable: RccBus {
    /// Enables peripheral in low power mode
    fn low_power_enable(bus: &mut Self::Bus);

    /// Disables peripheral in low power mode
    fn low_power_disable(bus: &mut Self::Bus);

    /// Check if peripheral enabled in low power mode
    fn is_low_power_enabled() -> bool;

    /// Check if peripheral disabled in low power mode
    fn is_low_power_disabled() -> bool {
        !Self::is_low_power_enabled()
    }

    /// # Safety
    ///
    /// Enables peripheral in low power mode. Takes access to RCC internally
    unsafe fn low_power_enable_unchecked();

    /// # Safety
    ///
    /// Disables peripheral in low power mode. Takes access to RCC internally
    unsafe fn low_power_disable_unchecked();
}

/// Reset peripheral
pub trait Reset: RccBus {
    /// Resets peripheral
    fn reset(bus: &mut Self::Bus);

    /// # Safety
    ///
    /// Resets peripheral. Takes access to RCC internally
    unsafe fn reset_unchecked();
}

#[cfg(test)]
mod tests {
    use fugit::{HertzU32 as Hertz, RateExtU32};

    use super::{FreqRequest, CFGR};

    fn build_request(sysclk: u32, use_pll48clk: bool) -> FreqRequest {
        let p = Some((sysclk - 1, sysclk + 1));
        let q = if use_pll48clk {
            Some((48_000_000 - 120_000, 48_000_000 + 120_000))
        } else {
            None
        };
        FreqRequest { p, q }
    }

    fn check(hse: u32, sysclk: u32, use_pll48clk: bool) {
        let request = build_request(sysclk, use_pll48clk);
        let (m, n, p, q) =
            CFGR::calculate_mnpq(hse, request).expect("Can't calculate PLL parameters");

        let pll_in = hse;

        if m < 2 || m > 63 {
            panic!("Invalid PLL M value: {}", m);
        }

        let vco_in = pll_in / m;
        if vco_in < 1_000_000 || vco_in > 2_000_000 {
            panic!("Invalid VCO input frequency: {}", vco_in);
        }

        if n < 50 || n > 432 {
            panic!("Invalid PLL N value: {}", n);
        }

        let vco = ((pll_in as u64) * (n as u64) / (m as u64)) as u32;
        if vco < 100_000_000 || vco > 432_000_000 {
            panic!("Invalid VCO frequency: {}", vco);
        }

        let p = p.expect("PLL P value should be defined!");
        if [2, 4, 6, 8].iter().find(|v| **v == p).is_none() {
            panic!("Invalid PLL P value: {}", p);
        }

        let p_freq = vco / p;
        if p_freq > 216_000_000 {
            panic!("Invalid PLL P frequency: {}", p_freq);
        }
        if p_freq < (sysclk - 1) || p_freq > (sysclk + 1) {
            panic!(
                "Invalid PLL P frequency: {} (requested sysclk {})",
                p_freq, sysclk
            );
        }

        if use_pll48clk && q.is_none() {
            panic!("PLL Q value should be defined!");
        }
        if let Some(q) = q {
            if q < 2 || q > 15 {
                panic!("Invalid PLL Q value: {}", q);
            }
            if use_pll48clk {
                let q_freq = vco / q;
                if q_freq < (48_000_000 - 120_000) || q_freq > (48_000_000 + 120_000) {
                    panic!("Invalid PLL Q frequency: {}", q_freq);
                }
            }
        }
    }

    #[test]
    fn test_pll_calc1() {
        check(25_000_000, 48_000_000, false);
    }

    #[test]
    fn test_pll_calc1_usb() {
        check(25_000_000, 48_000_000, true);
    }

    #[test]
    fn test_pll_calc2() {
        check(12_000_000, 48_000_000, false);
    }

    #[test]
    fn test_pll_calc2_usb() {
        check(12_000_000, 48_000_000, true);
    }

    #[test]
    fn test_pll_calc3() {
        check(12_000_000, 216_000_000, false);
    }

    #[test]
    fn test_pll_calc3_usb() {
        check(12_000_000, 216_000_000, true);
    }

    #[test]
    fn test_rcc_calc1() {
        use super::{HSEClock, HSEClockMode, MCO1, MCO2, MCOPRE, PLL48CLK, PLLP, PLLSAIP};

        let cfgr = CFGR {
            hse: None,
            hse_bypass: false,
            hclk: None,
            sysclk: None,
            pclk1: None,
            pclk2: None,
            lse: None,
            lsi: None,
            use_pll: false,
            pll48clk: None,
            pllm: 2,
            plln: 50,
            pllp: PLLP::Div2,
            pllq: 2,
            use_pllsai: false,
            pllsain: 192,
            pllsaip: PLLSAIP::Div2,
            pllsaiq: 2,
            use_plli2s: false,
            plli2sr: 2,
            plli2sq: 2,
            plli2sn: 192,
            mco1: MCO1::Hsi,
            mco1pre: MCOPRE::Div1_no_div,
            mco2: MCO2::Sysclk,
            mco2pre: MCOPRE::Div1_no_div,
        };

        let mut cfgr = cfgr
            .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
            .use_pll()
            .use_pll48clk(PLL48CLK::Pllq)
            .sysclk(216.MHz());
        cfgr.pll_configure();

        assert_eq!(cfgr.hse.unwrap().freq, Hertz::MHz(25));

        let (clocks, _config) = cfgr.calculate_clocks();
        assert_eq!(clocks.sysclk().raw(), 216_000_000);
        assert!(clocks.is_pll48clk_valid());
    }

    #[test]
    fn test_rcc_calc2() {
        use super::{HSEClock, HSEClockMode, MCO1, MCO2, MCOPRE, PLL48CLK, PLLP, PLLSAIP};

        let cfgr = CFGR {
            hse: None,
            hse_bypass: false,
            hclk: None,
            sysclk: None,
            pclk1: None,
            pclk2: None,
            lse: None,
            lsi: None,
            use_pll: false,
            pll48clk: None,
            pllm: 2,
            plln: 50,
            pllp: PLLP::Div2,
            pllq: 2,
            use_pllsai: false,
            pllsain: 192,
            pllsaip: PLLSAIP::Div2,
            pllsaiq: 2,
            use_plli2s: false,
            plli2sr: 2,
            plli2sq: 2,
            plli2sn: 192,
            mco1: MCO1::Hsi,
            mco1pre: MCOPRE::Div1_no_div,
            mco2: MCO2::Sysclk,
            mco2pre: MCOPRE::Div1_no_div,
        };

        let mut cfgr = cfgr
            .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
            .use_pll48clk(PLL48CLK::Pllq)
            .sysclk(216.MHz());
        cfgr.pll_configure();

        assert_eq!(cfgr.hse.unwrap().freq, Hertz::MHz(25));

        let (clocks, _config) = cfgr.calculate_clocks();
        assert_eq!(clocks.sysclk().raw(), 216_000_000);
        assert!(clocks.is_pll48clk_valid());
    }

    #[test]
    fn test_rcc_calc3() {
        use super::{HSEClock, HSEClockMode, MCO1, MCO2, MCOPRE, PLL48CLK, PLLP, PLLSAIP};

        let cfgr = CFGR {
            hse: None,
            hse_bypass: false,
            hclk: None,
            sysclk: None,
            pclk1: None,
            pclk2: None,
            lse: None,
            lsi: None,
            use_pll: false,
            pll48clk: None,
            pllm: 2,
            plln: 50,
            pllp: PLLP::Div2,
            pllq: 2,
            use_pllsai: false,
            pllsain: 192,
            pllsaip: PLLSAIP::Div2,
            pllsaiq: 2,
            use_plli2s: false,
            plli2sr: 2,
            plli2sq: 2,
            plli2sn: 192,
            mco1: MCO1::Hsi,
            mco1pre: MCOPRE::Div1_no_div,
            mco2: MCO2::Sysclk,
            mco2pre: MCOPRE::Div1_no_div,
        };

        let mut cfgr = cfgr
            .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
            .use_pll48clk(PLL48CLK::Pllq)
            .set_defaults();
        cfgr.pll_configure();

        assert_eq!(cfgr.hse.unwrap().freq, Hertz::MHz(25));

        let (clocks, _config) = cfgr.calculate_clocks();
        assert_eq!(clocks.sysclk().raw(), 216_000_000);
        assert!(clocks.is_pll48clk_valid());
    }

    #[test]
    fn test_rcc_default() {
        use super::{MCO1, MCO2, MCOPRE, PLLP, PLLSAIP};

        let mut cfgr = CFGR {
            hse: None,
            hse_bypass: false,
            hclk: None,
            sysclk: None,
            pclk1: None,
            pclk2: None,
            lse: None,
            lsi: None,
            use_pll: false,
            pll48clk: None,
            pllm: 2,
            plln: 50,
            pllp: PLLP::Div2,
            pllq: 2,
            use_pllsai: false,
            pllsain: 192,
            pllsaip: PLLSAIP::Div2,
            pllsaiq: 2,
            use_plli2s: false,
            plli2sr: 2,
            plli2sq: 2,
            plli2sn: 192,
            mco1: MCO1::Hsi,
            mco1pre: MCOPRE::Div1_no_div,
            mco2: MCO2::Sysclk,
            mco2pre: MCOPRE::Div1_no_div,
        };

        cfgr.pll_configure();
        assert!(!cfgr.use_pll);
        let (clocks, _config) = cfgr.calculate_clocks();
        assert_eq!(clocks.sysclk().raw(), 16_000_000);
    }
}
