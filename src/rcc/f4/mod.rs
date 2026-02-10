use crate::pac::rcc::cfgr::{HPRE, SW};
use crate::pac::RCC;

use super::*;

use fugit::HertzU32 as Hertz;
use fugit::RateExtU32;

mod pll;

/// Built-in high speed clock frequency
pub const HSI: u32 = 16_000_000; // Hz

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
/// Minimum system clock frequency
pub const SYSCLK_MIN: u32 = 24_000_000;

#[cfg(feature = "gpio-f446")]
/// Minimum system clock frequency
pub const SYSCLK_MIN: u32 = 12_500_000;

#[cfg(feature = "gpio-f401")]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 84_000_000;

#[cfg(feature = "gpio-f417")]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 168_000_000;

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 100_000_000;

#[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 180_000_000;

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
))]
/// Maximum APB2 peripheral clock frequency
pub const PCLK2_MAX: u32 = SYSCLK_MAX;

#[cfg(not(any(
    feature = "gpio-f401",
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
)))]
/// Maximum APB2 peripheral clock frequency
pub const PCLK2_MAX: u32 = SYSCLK_MAX / 2;

/// Maximum APB1 peripheral clock frequency
pub const PCLK1_MAX: u32 = PCLK2_MAX / 2;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    hse: Option<u32>,
    hse_bypass: bool,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
    pll48clk: bool,

    i2s_ckin: Option<u32>,

    #[cfg(not(feature = "rcc_i2s_apb"))]
    i2s_clk: Option<u32>,
    #[cfg(feature = "rcc_i2s_apb")]
    i2s_apb1_clk: Option<u32>,
    #[cfg(feature = "rcc_i2s_apb")]
    i2s_apb2_clk: Option<u32>,

    #[cfg(feature = "sai")]
    sai1_clk: Option<u32>,
    #[cfg(feature = "sai")]
    sai2_clk: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Config {
    pub const DEFAULT: Self = Self {
        hse: None,
        hse_bypass: false,
        hclk: None,
        pclk1: None,
        pclk2: None,
        sysclk: None,
        pll48clk: false,

        i2s_ckin: None,

        #[cfg(not(feature = "rcc_i2s_apb"))]
        i2s_clk: None,
        #[cfg(feature = "rcc_i2s_apb")]
        i2s_apb1_clk: None,
        #[cfg(feature = "rcc_i2s_apb")]
        i2s_apb2_clk: None,

        #[cfg(feature = "sai")]
        sai1_clk: None,
        #[cfg(feature = "sai")]
        sai2_clk: None,
    };

    pub fn hsi() -> Self {
        Self::DEFAULT
    }

    pub fn hse(freq: Hertz) -> Self {
        Self::DEFAULT.use_hse(freq)
    }

    /// Uses HSE (external oscillator) instead of HSI (internal RC oscillator) as the clock source.
    /// Will result in a hang if an external oscillator is not connected or it fails to start.
    pub fn use_hse(mut self, freq: Hertz) -> Self {
        self.hse = Some(freq.raw());
        self
    }

    /// Bypasses the high-speed external oscillator and uses an external clock input on the OSC_IN
    /// pin.
    ///
    /// For this configuration, the OSC_IN pin should be connected to a clock source with a
    /// frequency specified in the call to use_hse(), and the OSC_OUT pin should not be connected.
    ///
    /// This function has no effect unless use_hse() is also called.
    pub fn bypass_hse_oscillator(self) -> Self {
        Self {
            hse_bypass: true,
            ..self
        }
    }

    pub fn hclk(mut self, freq: Hertz) -> Self {
        self.hclk = Some(freq.raw());
        self
    }

    pub fn pclk1(mut self, freq: Hertz) -> Self {
        self.pclk1 = Some(freq.raw());
        self
    }

    pub fn pclk2(mut self, freq: Hertz) -> Self {
        self.pclk2 = Some(freq.raw());
        self
    }

    pub fn sysclk(mut self, freq: Hertz) -> Self {
        self.sysclk = Some(freq.raw());
        self
    }

    pub fn require_pll48clk(mut self) -> Self {
        self.pll48clk = true;
        self
    }

    /// Declares that the selected frequency is available at the I2S clock input pin (I2S_CKIN).
    ///
    /// If this frequency matches the requested SAI or I2S frequencies, the external I2S clock is
    /// used to generate the clocks.
    pub fn i2s_ckin(mut self, freq: Hertz) -> Self {
        self.i2s_ckin = Some(freq.raw());
        self
    }
}

#[cfg(not(feature = "rcc_i2s_apb"))]
impl Config {
    /// Selects an I2S clock frequency and enables the I2S clock.
    pub fn i2s_clk(mut self, freq: Hertz) -> Self {
        self.i2s_clk = Some(freq.raw());
        self
    }
}

#[cfg(feature = "rcc_i2s_apb")]
impl Config {
    /// Selects an I2S clock frequency for the first set of I2S instancesand enables the I2S clock.
    pub fn i2s_apb1_clk(mut self, freq: Hertz) -> Self {
        self.i2s_apb1_clk = Some(freq.raw());
        self
    }

    /// Selects an I2S clock frequency for the second set of I2S instances and enables the I2S clock.
    pub fn i2s_apb2_clk(mut self, freq: Hertz) -> Self {
        self.i2s_apb2_clk = Some(freq.raw());
        self
    }
}

#[cfg(feature = "sai")]
#[cfg(not(feature = "sai2"))]
impl Config {
    /// Selects a SAIA clock frequency and enables the SAIA clock.
    pub fn saia_clk(mut self, freq: Hertz) -> Self {
        self.sai1_clk = Some(freq.raw());
        self
    }

    /// Selects a SAIB clock frequency and enables the SAIB clock.
    pub fn saib_clk(mut self, freq: Hertz) -> Self {
        self.sai2_clk = Some(freq.raw());
        self
    }
}

#[cfg(feature = "sai2")]
impl Config {
    /// Selects a SAI1 clock frequency and enables the SAI1 clock.
    pub fn sai1_clk(mut self, freq: Hertz) -> Self {
        self.sai1_clk = Some(freq.raw());
        self
    }

    /// Selects a SAI2 clock frequency and enables the SAI2 clock.
    pub fn sai2_clk(mut self, freq: Hertz) -> Self {
        self.sai2_clk = Some(freq.raw());
        self
    }
}

#[cfg(feature = "sai")]
impl Config {
    fn sai_clocks(&self) -> SaiClocks {
        let sai1_ext = self.sai1_clk.is_some() && self.sai1_clk == self.i2s_ckin;
        #[cfg(not(feature = "sai2"))]
        let sai2_ext = self.sai2_clk.is_some() && self.sai2_clk == self.i2s_ckin;
        // Not the PLL output, but the target clock after the divider.
        let pll_sai_clk = if sai1_ext { None } else { self.sai1_clk };
        // The STM32F446 only supports I2S_CKIN for SAI1.
        #[cfg(feature = "sai2")]
        let pll_sai_clk2 = self.sai2_clk;
        #[cfg(not(feature = "sai2"))]
        let pll_sai_clk2 = if sai2_ext { None } else { self.sai2_clk };
        if pll_sai_clk.is_some() && pll_sai_clk2.is_some() && pll_sai_clk != pll_sai_clk2 {
            panic!("only one SAI PLL frequency implemented");
        }
        SaiClocks {
            sai1_ext,
            #[cfg(not(feature = "sai2"))]
            sai2_ext,
            pll_sai_clk,
        }
    }
}

impl Config {
    #[cfg(feature = "rcc_i2s_apb")]
    fn i2s_clocks(&self) -> I2sClocks {
        let i2s_apb1_ext = self.i2s_apb1_clk.is_some() && self.i2s_apb1_clk == self.i2s_ckin;
        let i2s_apb2_ext = self.i2s_apb2_clk.is_some() && self.i2s_apb2_clk == self.i2s_ckin;
        let pll_i2s_clk = if i2s_apb1_ext {
            None
        } else {
            self.i2s_apb1_clk
        };
        let pll_i2s_clk2 = if i2s_apb2_ext {
            None
        } else {
            self.i2s_apb2_clk
        };
        if pll_i2s_clk.is_some() && pll_i2s_clk2.is_some() && pll_i2s_clk != pll_i2s_clk2 {
            panic!("only one I2S PLL frequency implemented");
        }
        I2sClocks {
            i2s_apb1_ext,
            i2s_apb2_ext,
            pll_i2s_clk,
        }
    }

    #[cfg(not(feature = "rcc_i2s_apb"))]
    fn i2s_clocks(&self) -> I2sClocks {
        let i2s_ext = self.i2s_clk.is_some() && self.i2s_clk == self.i2s_ckin;
        let pll_i2s_clk = if i2s_ext { None } else { self.i2s_clk };
        I2sClocks {
            i2s_ext,
            pll_i2s_clk,
        }
    }
}

impl Rcc {
    fn flash_setup(sysclk: u32) {
        use crate::pac::FLASH;

        #[cfg(any(
            feature = "gpio-f401",
            feature = "gpio-f417",
            feature = "gpio-f410",
            feature = "gpio-f411",
            feature = "gpio-f412",
            feature = "gpio-f427",
            feature = "gpio-f446",
            feature = "gpio-f469",
        ))]
        let flash_latency_step = 30_000_000;

        #[cfg(feature = "gpio-f413")]
        let flash_latency_step = 25_000_000;

        unsafe {
            let flash = &(*FLASH::ptr());
            // Adjust flash wait states
            flash.acr().modify(|_, w| {
                w.latency().bits(((sysclk - 1) / flash_latency_step) as u8);
                w.prften().set_bit();
                w.icen().set_bit();
                w.dcen().set_bit()
            });
        }
    }

    /// Apply clock configuration
    pub fn freeze(self, rcc_cfg: Config) -> Self {
        self.freeze_internal(rcc_cfg, false)
    }

    /// Initialises the hardware according to Config state returning a Clocks instance.
    /// Allows overclocking.
    ///
    /// # Safety
    ///
    /// This method does not check if the clocks are bigger or smaller than the officially
    /// recommended.
    pub unsafe fn freeze_unchecked(self, rcc_cfg: Config) -> Self {
        self.freeze_internal(rcc_cfg, true)
    }

    fn freeze_internal(self, rcc_cfg: Config, unchecked: bool) -> Self {
        let rcc = &self.rb;

        let pllsrcclk = rcc_cfg.hse.unwrap_or(HSI);
        let sysclk = rcc_cfg.sysclk.unwrap_or(pllsrcclk);
        let sysclk_on_pll = sysclk != pllsrcclk;

        let plls = pll::PllSetup::from_cfgr(&rcc_cfg, pllsrcclk, sysclk_on_pll.then_some(sysclk));
        let sysclk = if sysclk_on_pll {
            plls.pllsysclk.unwrap()
        } else {
            sysclk
        };

        assert!(unchecked || !sysclk_on_pll || (SYSCLK_MIN..=SYSCLK_MAX).contains(&sysclk));

        let hclk = rcc_cfg.hclk.unwrap_or(sysclk);
        let (hpre_bits, hpre_div) = match (sysclk + hclk - 1) / hclk {
            0 => unreachable!(),
            1 => (HPRE::Div1, 1),
            2 => (HPRE::Div2, 2),
            3..=5 => (HPRE::Div4, 4),
            6..=11 => (HPRE::Div8, 8),
            12..=39 => (HPRE::Div16, 16),
            40..=95 => (HPRE::Div64, 64),
            96..=191 => (HPRE::Div128, 128),
            192..=383 => (HPRE::Div256, 256),
            _ => (HPRE::Div512, 512),
        };

        // Calculate real AHB clock
        let hclk = sysclk / hpre_div;

        let pclk1 = rcc_cfg
            .pclk1
            .unwrap_or_else(|| crate::min_u32(PCLK1_MAX, hclk));
        let (ppre1_bits, ppre1) = match (hclk + pclk1 - 1) / pclk1 {
            0 => unreachable!(),
            1 => (0b000, 1u8),
            2 => (0b100, 2),
            3..=5 => (0b101, 4),
            6..=11 => (0b110, 8),
            _ => (0b111, 16),
        };

        // Calculate real APB1 clock
        let pclk1 = hclk / u32::from(ppre1);

        assert!(unchecked || pclk1 <= PCLK1_MAX);

        let pclk2 = rcc_cfg
            .pclk2
            .unwrap_or_else(|| crate::min_u32(PCLK2_MAX, hclk));
        let (ppre2_bits, ppre2) = match (hclk + pclk2 - 1) / pclk2 {
            0 => unreachable!(),
            1 => (0b000, 1u8),
            2 => (0b100, 2),
            3..=5 => (0b101, 4),
            6..=11 => (0b110, 8),
            _ => (0b111, 16),
        };

        // Calculate real APB2 clock
        let pclk2 = hclk / u32::from(ppre2);

        assert!(unchecked || pclk2 <= PCLK2_MAX);

        Self::flash_setup(sysclk);

        if rcc_cfg.hse.is_some() {
            // enable HSE and wait for it to be ready
            rcc.cr().modify(|_, w| {
                if rcc_cfg.hse_bypass {
                    w.hsebyp().bypassed();
                }
                w.hseon().set_bit()
            });
            while rcc.cr().read().hserdy().bit_is_clear() {}
        }

        if plls.use_pll {
            // Enable PLL
            rcc.cr().modify(|_, w| w.pllon().set_bit());

            // Enable voltage regulator overdrive if HCLK is above the limit
            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            if hclk > 168_000_000 {
                // Enable clock for PWR peripheral
                rcc.apb1enr().modify(|_, w| w.pwren().set_bit());

                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();

                let pwr = unsafe { &*crate::pac::PWR::ptr() };
                pwr.cr().modify(|_, w| w.oden().set_bit());
                while pwr.csr().read().odrdy().bit_is_clear() {}
                pwr.cr().modify(|_, w| w.odswen().set_bit());
                while pwr.csr().read().odswrdy().bit_is_clear() {}
            }

            // Wait for PLL to stabilise
            while rcc.cr().read().pllrdy().bit_is_clear() {}
        }

        #[cfg(not(feature = "gpio-f410"))]
        if plls.use_i2spll {
            // Enable PLL.
            rcc.cr().modify(|_, w| w.plli2son().set_bit());

            // Wait for PLL to stabilise
            while rcc.cr().read().plli2srdy().bit_is_clear() {}
        }

        #[cfg(feature = "sai")]
        #[cfg(not(feature = "gpio-f413"))]
        if plls.use_saipll {
            // Enable PLL.
            rcc.cr().modify(|_, w| w.pllsaion().set_bit());

            // Wait for PLL to stabilise
            while rcc.cr().read().pllsairdy().bit_is_clear() {}
        }

        // Select I2S and SAI clocks
        plls.i2s.config_clocksel();
        #[cfg(feature = "sai")]
        plls.sai.config_clocksel();

        // Set scaling factors
        rcc.cfgr().modify(|_, w| unsafe {
            w.ppre2().bits(ppre2_bits);
            w.ppre1().bits(ppre1_bits);
            w.hpre().variant(hpre_bits)
        });

        // Wait for the new prescalers to kick in
        // "The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after write"
        cortex_m::asm::delay(16);

        // Select system clock source
        rcc.cfgr().modify(|_, w| {
            w.sw().variant(if sysclk_on_pll {
                SW::Pll
            } else if rcc_cfg.hse.is_some() {
                SW::Hse
            } else {
                SW::Hsi
            })
        });

        let pclk_mul = if ppre1 == 1 { 1 } else { 2 };
        let timclk1 = Hertz::from_raw(pclk1 * pclk_mul);

        let pclk_mul = if ppre2 == 1 { 1 } else { 2 };
        let timclk2 = Hertz::from_raw(pclk2 * pclk_mul);

        let clocks = Clocks {
            hclk: hclk.Hz(),
            pclk1: pclk1.Hz(),
            pclk2: pclk2.Hz(),
            timclk1,
            timclk2,
            sysclk: sysclk.Hz(),
            pll48clk: plls.pll48clk.map(Hertz::from_raw),

            #[cfg(not(feature = "rcc_i2s_apb"))]
            i2s_clk: plls.i2s.i2s_clk.map(Hertz::from_raw),
            #[cfg(feature = "rcc_i2s_apb")]
            i2s_apb1_clk: plls.i2s.apb1.i2s_clk.map(Hertz::from_raw),
            #[cfg(feature = "rcc_i2s_apb")]
            i2s_apb2_clk: plls.i2s.apb2.i2s_clk.map(Hertz::from_raw),

            #[cfg(feature = "sai")]
            #[cfg(not(feature = "sai2"))]
            saia_clk: plls.sai.sai1_clk.map(Hertz::from_raw),
            #[cfg(feature = "sai")]
            #[cfg(not(feature = "sai2"))]
            saib_clk: plls.sai.sai2_clk.map(Hertz::from_raw),
            #[cfg(feature = "sai2")]
            sai1_clk: plls.sai.sai1_clk.map(Hertz::from_raw),
            #[cfg(feature = "sai2")]
            sai2_clk: plls.sai.sai2_clk.map(Hertz::from_raw),
        };

        if rcc_cfg.pll48clk {
            assert!(clocks.is_pll48clk_valid());
        }

        Self {
            rb: self.rb,
            clocks,
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct I2sClocks {
    /// True if the clock for the APB1 I2S instances is identical to I2S_CKIN.
    #[cfg(feature = "rcc_i2s_apb")]
    i2s_apb1_ext: bool,
    /// True if the clock for the APB2 I2S instances is identical to I2S_CKIN.
    #[cfg(feature = "rcc_i2s_apb")]
    i2s_apb2_ext: bool,
    /// True if the clock for I2S is identical to I2S_CKIN.
    #[cfg(not(feature = "rcc_i2s_apb"))]
    i2s_ext: bool,
    /// Target for the I2S PLL output.
    pll_i2s_clk: Option<u32>,
}

impl I2sClocks {
    fn real(&self, pll_i2s_clk: Option<u32>, i2s_ckin: Option<u32>) -> RealI2sClocks {
        #[cfg(feature = "rcc_i2s_apb")]
        let clk = RealI2sClocks {
            apb1: RealI2sClock {
                i2s_ext: self.i2s_apb1_ext,
                i2s_clk: if self.i2s_apb1_ext {
                    i2s_ckin
                } else {
                    pll_i2s_clk
                },
            },
            apb2: RealI2sClock {
                i2s_ext: self.i2s_apb2_ext,
                i2s_clk: if self.i2s_apb2_ext {
                    i2s_ckin
                } else {
                    pll_i2s_clk
                },
            },
        };
        #[cfg(not(feature = "rcc_i2s_apb"))]
        let clk = RealI2sClocks {
            i2s_ext: self.i2s_ext,
            i2s_clk: if self.i2s_ext { i2s_ckin } else { pll_i2s_clk },
        };
        clk
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RealI2sClock {
    i2s_ext: bool,
    i2s_clk: Option<u32>,
}

#[cfg(feature = "rcc_i2s_apb")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RealI2sClocks {
    apb1: RealI2sClock,
    apb2: RealI2sClock,
}

#[cfg(not(feature = "rcc_i2s_apb"))]
use RealI2sClock as RealI2sClocks;

impl RealI2sClocks {
    fn config_clocksel(&self) {
        let rcc = unsafe { &*RCC::ptr() };

        #[cfg(not(feature = "gpio-f410"))]
        #[cfg(not(feature = "rcc_i2s_apb"))]
        rcc.cfgr().modify(|_, w| {
            if self.i2s_ext {
                w.i2ssrc().ckin()
            } else {
                w.i2ssrc().plli2s()
            }
        });
        #[cfg(feature = "gpio-f410")]
        rcc.dckcfgr().modify(|_, w| {
            if self.i2s_ext {
                w.i2ssrc().i2s_ckin()
            } else {
                w.i2ssrc().pllclkr()
            }
        });
        #[cfg(feature = "rcc_i2s_apb")]
        rcc.dckcfgr().modify(|_, w| {
            if self.apb1.i2s_ext {
                w.i2s1src().i2s_ckin()
            } else {
                w.i2s1src().plli2sr()
            };
            if self.apb2.i2s_ext {
                w.i2s2src().i2s_ckin()
            } else {
                w.i2s2src().plli2sr()
            }
        });
    }
}

#[cfg(feature = "sai")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct SaiClocks {
    /// True if the clock for SAI1 (STM32F446) or SAIA (all other models) is identical to I2S_CKIN.
    sai1_ext: bool,
    /// True if the clock for SAIB is identical to I2S_CKIN.
    #[cfg(not(feature = "sai2"))]
    sai2_ext: bool,
    /// Target for the SAI clock as generated by PLL and SAI clock divider.
    pll_sai_clk: Option<u32>,
}

#[cfg(feature = "sai")]
impl SaiClocks {
    fn real(&self, pll_sai_clk: Option<u32>, i2s_ckin: Option<u32>) -> RealSaiClocks {
        RealSaiClocks {
            sai1_ext: self.sai1_ext,
            #[cfg(not(feature = "sai2"))]
            sai2_ext: self.sai2_ext,
            sai1_clk: if self.sai1_ext { i2s_ckin } else { pll_sai_clk },
            #[cfg(not(feature = "sai2"))]
            sai2_clk: if self.sai2_ext { i2s_ckin } else { pll_sai_clk },
            #[cfg(feature = "sai2")]
            sai2_clk: pll_sai_clk,
        }
    }
}

#[cfg(feature = "sai")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RealSaiClocks {
    sai1_ext: bool,
    #[cfg(not(feature = "sai2"))]
    sai2_ext: bool,
    sai1_clk: Option<u32>,
    sai2_clk: Option<u32>,
}

#[cfg(feature = "sai")]
impl RealSaiClocks {
    fn config_clocksel(&self) {
        let rcc = unsafe { &*RCC::ptr() };

        // Configure SAI clocks.
        #[cfg(not(feature = "sai2"))]
        rcc.dckcfgr().modify(|_, w| {
            if self.sai1_ext {
                w.sai1asrc().i2s_ckin()
            } else {
                w.sai1asrc().pllsai()
            };
            if self.sai2_ext {
                w.sai1bsrc().i2s_ckin()
            } else {
                w.sai1bsrc().pllsai()
            }
        });

        #[cfg(feature = "sai2")]
        rcc.dckcfgr().modify(|_, w| {
            if self.sai1_ext {
                w.sai1src().i2s_ckin()
            } else {
                w.sai1src().pllsai()
            }
        });
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Clocks {
    pub(super) hclk: Hertz,
    pub(super) pclk1: Hertz,
    pub(super) pclk2: Hertz,
    pub(super) timclk1: Hertz,
    pub(super) timclk2: Hertz,
    pub(super) sysclk: Hertz,
    pub(super) pll48clk: Option<Hertz>,

    #[cfg(not(feature = "rcc_i2s_apb"))]
    pub(super) i2s_clk: Option<Hertz>,
    #[cfg(feature = "rcc_i2s_apb")]
    pub(super) i2s_apb1_clk: Option<Hertz>,
    #[cfg(feature = "rcc_i2s_apb")]
    pub(super) i2s_apb2_clk: Option<Hertz>,

    #[cfg(feature = "sai")]
    #[cfg(not(feature = "sai2"))]
    pub(super) saia_clk: Option<Hertz>,
    #[cfg(feature = "sai")]
    #[cfg(not(feature = "sai2"))]
    pub(super) saib_clk: Option<Hertz>,
    #[cfg(feature = "sai2")]
    pub(super) sai1_clk: Option<Hertz>,
    #[cfg(feature = "sai2")]
    pub(super) sai2_clk: Option<Hertz>,
}

impl Default for Clocks {
    fn default() -> Clocks {
        let freq = HSI.Hz();
        Clocks {
            hclk: freq,
            pclk1: freq,
            pclk2: freq,
            timclk1: freq,
            timclk2: freq,
            sysclk: freq,
            pll48clk: None,
            #[cfg(not(feature = "rcc_i2s_apb"))]
            i2s_clk: None,
            #[cfg(feature = "rcc_i2s_apb")]
            i2s_apb1_clk: None,
            #[cfg(feature = "rcc_i2s_apb")]
            i2s_apb2_clk: None,

            #[cfg(feature = "sai")]
            #[cfg(not(feature = "sai2"))]
            saia_clk: None,
            #[cfg(feature = "sai")]
            #[cfg(not(feature = "sai2"))]
            saib_clk: None,
            #[cfg(feature = "sai2")]
            sai1_clk: None,
            #[cfg(feature = "sai2")]
            sai2_clk: None,
        }
    }
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

    /// Returns the frequency for timers on APB1
    pub fn timclk1(&self) -> Hertz {
        self.timclk1
    }

    /// Returns the frequency for timers on APB1
    pub fn timclk2(&self) -> Hertz {
        self.timclk2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the frequency of the PLL48 clock line
    pub fn pll48clk(&self) -> Option<Hertz> {
        self.pll48clk
    }

    /// Returns true if the PLL48 clock is within USB
    /// specifications. It is required to use the USB functionality.
    pub fn is_pll48clk_valid(&self) -> bool {
        // USB specification allows +-0.25%
        self.pll48clk
            .map(|freq| 48_000_000_u32.abs_diff(freq.raw()) <= 120_000)
            .unwrap_or_default()
    }

    /// Returns the frequency of the I2S clock.
    #[cfg(not(feature = "rcc_i2s_apb"))]
    pub fn i2s_clk(&self) -> Option<Hertz> {
        self.i2s_clk
    }
    /// Returns the frequency of the first I2S clock (for the I2S peripherals on APB1).
    #[cfg(feature = "rcc_i2s_apb")]
    pub fn i2s_apb1_clk(&self) -> Option<Hertz> {
        self.i2s_apb1_clk
    }
    /// Returns the frequency of the second I2S clock (for the I2S peripherals on APB2).
    #[cfg(feature = "rcc_i2s_apb")]
    pub fn i2s_apb2_clk(&self) -> Option<Hertz> {
        self.i2s_apb2_clk
    }

    /// Returns the frequency of the SAI A clock.
    #[cfg(feature = "sai")]
    #[cfg(not(feature = "sai2"))]
    pub fn saia_clk(&self) -> Option<Hertz> {
        self.saia_clk
    }
    /// Returns the frequency of the SAI B clock.
    #[cfg(feature = "sai")]
    #[cfg(not(feature = "sai2"))]
    pub fn saib_clk(&self) -> Option<Hertz> {
        self.saib_clk
    }
    /// Returns the frequency of the SAI1 clock.
    #[cfg(feature = "sai2")]
    pub fn sai1_clk(&self) -> Option<Hertz> {
        self.sai1_clk
    }
    /// Returns the frequency of the SAI2 clock.
    #[cfg(feature = "sai2")]
    pub fn sai2_clk(&self) -> Option<Hertz> {
        self.sai2_clk
    }
}
