use crate::stm32::rcc::cfgr::{HPRE_A, SW_A};
use crate::stm32::RCC;

use crate::time::Hertz;

#[cfg(not(feature = "stm32f410"))]
use pll::I2sPll;
use pll::MainPll;
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
use pll::SaiPll;

mod pll;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            cfgr: CFGR {
                hse: None,
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
                pll48clk: false,
                i2s_ckin: None,
                #[cfg(any(
                    feature = "stm32f401",
                    feature = "stm32f405",
                    feature = "stm32f407",
                    feature = "stm32f410",
                    feature = "stm32f411",
                    feature = "stm32f415",
                    feature = "stm32f417",
                    feature = "stm32f427",
                    feature = "stm32f429",
                    feature = "stm32f437",
                    feature = "stm32f439",
                    feature = "stm32f469",
                    feature = "stm32f479"
                ))]
                i2s_clk: None,
                #[cfg(any(
                    feature = "stm32f412",
                    feature = "stm32f413",
                    feature = "stm32f423",
                    feature = "stm32f446",
                ))]
                i2s_apb1_clk: None,
                #[cfg(any(
                    feature = "stm32f412",
                    feature = "stm32f413",
                    feature = "stm32f423",
                    feature = "stm32f446",
                ))]
                i2s_apb2_clk: None,
                #[cfg(any(
                    feature = "stm32f413",
                    feature = "stm32f423",
                    feature = "stm32f427",
                    feature = "stm32f429",
                    feature = "stm32f437",
                    feature = "stm32f439",
                    feature = "stm32f446",
                    feature = "stm32f469",
                    feature = "stm32f479",
                ))]
                sai1_clk: None,
                #[cfg(any(
                    feature = "stm32f413",
                    feature = "stm32f423",
                    feature = "stm32f427",
                    feature = "stm32f429",
                    feature = "stm32f437",
                    feature = "stm32f439",
                    feature = "stm32f446",
                    feature = "stm32f469",
                    feature = "stm32f479",
                ))]
                sai2_clk: None,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    pub cfgr: CFGR,
}

/// Built-in high speed clock frequency
pub const HSI: u32 = 16_000_000; // Hz

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
/// Minimum system clock frequency
pub const SYSCLK_MIN: u32 = 24_000_000;

#[cfg(any(feature = "stm32f446"))]
/// Minimum system clock frequency
pub const SYSCLK_MIN: u32 = 12_500_000;

#[cfg(feature = "stm32f401")]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 84_000_000;

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417"
))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 168_000_000;

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 100_000_000;

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 180_000_000;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
/// Maximum APB2 peripheral clock frequency
pub const PCLK2_MAX: u32 = SYSCLK_MAX;

#[cfg(not(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
)))]
/// Maximum APB2 peripheral clock frequency
pub const PCLK2_MAX: u32 = SYSCLK_MAX / 2;

/// Maximum APB1 peripheral clock frequency
pub const PCLK1_MAX: u32 = PCLK2_MAX / 2;

pub struct CFGR {
    hse: Option<u32>,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
    pll48clk: bool,

    i2s_ckin: Option<u32>,
    #[cfg(any(
        feature = "stm32f401",
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479"
    ))]
    i2s_clk: Option<u32>,
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    i2s_apb1_clk: Option<u32>,
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    i2s_apb2_clk: Option<u32>,
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    sai1_clk: Option<u32>,
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    sai2_clk: Option<u32>,
}

impl CFGR {
    /// Uses HSE (external oscillator) instead of HSI (internal RC oscillator) as the clock source.
    /// Will result in a hang if an external oscillator is not connected or it fails to start.
    pub fn use_hse<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hse = Some(freq.into().0);
        self
    }

    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk1<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk1 = Some(freq.into().0);
        self
    }

    pub fn pclk2<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk2 = Some(freq.into().0);
        self
    }

    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
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
    pub fn i2s_ckin<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.i2s_ckin = Some(freq.into().0);
        self
    }

    /// Selects an I2S clock frequency and enables the I2S clock.
    #[cfg(any(
        feature = "stm32f401",
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479"
    ))]
    pub fn i2s_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.i2s_clk = Some(freq.into().0);
        self
    }

    /// Selects an I2S clock frequency for the first set of I2S instancesand enables the I2S clock.
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    pub fn i2s_apb1_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.i2s_apb1_clk = Some(freq.into().0);
        self
    }

    /// Selects an I2S clock frequency for the second set of I2S instances and enables the I2S clock.
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    pub fn i2s_apb2_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.i2s_apb2_clk = Some(freq.into().0);
        self
    }

    /// Selects a SAIA clock frequency and enables the SAIA clock.
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    pub fn saia_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sai1_clk = Some(freq.into().0);
        self
    }

    /// Selects a SAIB clock frequency and enables the SAIB clock.
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    pub fn saib_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sai2_clk = Some(freq.into().0);
        self
    }

    /// Selects a SAI1 clock frequency and enables the SAI1 clock.
    #[cfg(any(feature = "stm32f446",))]
    pub fn sai1_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sai1_clk = Some(freq.into().0);
        self
    }

    /// Selects a SAI2 clock frequency and enables the SAI2 clock.
    #[cfg(any(feature = "stm32f446",))]
    pub fn sai2_clk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sai2_clk = Some(freq.into().0);
        self
    }
    #[cfg(feature = "stm32f410")]
    #[inline(always)]
    fn pll_setup(&self, pllsrcclk: u32, pllsysclk: Option<u32>) -> PllSetup {
        let i2s_clocks = self.i2s_clocks();

        let main_pll = if let Some(i2s_clk) = i2s_clocks.pll_i2s_clk {
            // The I2S frequency is generated by the main PLL. The frequency needs to be accurate,
            // so we need an expensive full PLL configuration search.
            MainPll::setup_with_i2s(
                pllsrcclk,
                self.hse.is_some(),
                pllsysclk,
                self.pll48clk,
                i2s_clk,
            )
        } else {
            MainPll::fast_setup(pllsrcclk, self.hse.is_some(), pllsysclk, self.pll48clk)
        };

        PllSetup {
            use_pll: main_pll.use_pll,
            pllsysclk: main_pll.pllsysclk,
            pll48clk: main_pll.pll48clk,
            i2s: i2s_clocks.real(main_pll.plli2sclk, self.i2s_ckin),
        }
    }

    #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
    #[inline(always)]
    fn pll_setup(&self, pllsrcclk: u32, pllsysclk: Option<u32>) -> PllSetup {
        let rcc = unsafe { &*RCC::ptr() };

        let i2s_clocks = self.i2s_clocks();
        let sai_clocks = self.sai_clocks();

        let main_pll = MainPll::fast_setup(pllsrcclk, self.hse.is_some(), pllsysclk, self.pll48clk);

        let (i2s_pll, real_sai_clk) = if let Some(i2s_clk) = i2s_clocks.pll_i2s_clk {
            // Currently, we only support generating SAI/PLL clocks with the I2S PLL. This is only
            // really usable when the frequencies are identical or the I2S frequency is a multiple of
            // the SAI frequency. Therefore, we just optimize the PLL for the I2S frequency and then
            // derive the SAI frequency from the I2S frequency.
            let i2s_pll = I2sPll::setup(pllsrcclk, Some(i2s_clk));

            if let Some(sai_clk) = sai_clocks.pll_sai_clk {
                let div = u32::min(
                    u32::max((i2s_pll.plli2sclk.unwrap() + (sai_clk >> 1)) / sai_clk, 1),
                    31,
                );
                rcc.dckcfgr.modify(|_, w| w.plli2sdivr().bits(div as u8));
                let real_sai_clk = sai_clk / div;
                (i2s_pll, Some(real_sai_clk))
            } else {
                (i2s_pll, None)
            }
        } else if let Some(pll_sai_clk) = sai_clocks.pll_sai_clk {
            // We try all divider values to get the best approximation of the requested frequency.
            // NOTE: STM32F413/423 have a different divider range than other models!
            let (i2s_pll, real_sai_clk, div) = (1..31)
                .map(|div| {
                    let i2s_pll = I2sPll::setup(pllsrcclk, Some(pll_sai_clk * div));
                    let real_clk = i2s_pll.plli2sclk.unwrap() / div;
                    (i2s_pll, real_clk, div)
                })
                .min_by_key(|(_, real_clk, _)| (*real_clk as i32 - pll_sai_clk as i32).abs())
                .unwrap();
            rcc.dckcfgr.modify(|_, w| w.plli2sdivr().bits(div as u8));
            (i2s_pll, Some(real_sai_clk))
        } else {
            (I2sPll::unused(), None)
        };

        PllSetup {
            use_pll: main_pll.use_pll,
            use_i2spll: i2s_pll.use_pll,
            pllsysclk: main_pll.pllsysclk,
            pll48clk: main_pll.pll48clk,
            i2s: i2s_clocks.real(i2s_pll.plli2sclk, self.i2s_ckin),
            sai: sai_clocks.real(real_sai_clk, self.i2s_ckin),
        }
    }

    #[cfg(any(feature = "stm32f411", feature = "stm32f412", feature = "stm32f446"))]
    #[inline(always)]
    fn pll_setup(&self, pllsrcclk: u32, pllsysclk: Option<u32>) -> PllSetup {
        let i2s_clocks = self.i2s_clocks();
        #[cfg(feature = "stm32f446")]
        let sai_clocks = self.sai_clocks();

        // All PLLs are completely independent.
        let main_pll = MainPll::fast_setup(pllsrcclk, self.hse.is_some(), pllsysclk, self.pll48clk);
        let i2s_pll = I2sPll::setup(pllsrcclk, i2s_clocks.pll_i2s_clk);
        #[cfg(feature = "stm32f446")]
        let sai_pll = SaiPll::setup(pllsrcclk, sai_clocks.pll_sai_clk);

        PllSetup {
            use_pll: main_pll.use_pll,
            use_i2spll: i2s_pll.use_pll,
            #[cfg(feature = "stm32f446")]
            use_saipll: sai_pll.use_pll,
            pllsysclk: main_pll.pllsysclk,
            pll48clk: main_pll.pll48clk,
            i2s: i2s_clocks.real(i2s_pll.plli2sclk, self.i2s_ckin),
            #[cfg(feature = "stm32f446")]
            sai: sai_clocks.real(sai_pll.sai_clk, self.i2s_ckin),
        }
    }

    #[cfg(any(
        feature = "stm32f401",
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479"
    ))]
    #[inline(always)]
    fn pll_setup(&self, pllsrcclk: u32, pllsysclk: Option<u32>) -> PllSetup {
        let i2s_clocks = self.i2s_clocks();
        #[cfg(any(
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        let sai_clocks = self.sai_clocks();

        // We have separate PLLs, but they share the "M" divider.
        let main_pll = MainPll::fast_setup(pllsrcclk, self.hse.is_some(), pllsysclk, self.pll48clk);
        let i2s_pll = I2sPll::setup_shared_m(pllsrcclk, main_pll.m, i2s_clocks.pll_i2s_clk);
        #[cfg(any(
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        let sai_pll =
            SaiPll::setup_shared_m(pllsrcclk, main_pll.m.or(i2s_pll.m), sai_clocks.pll_sai_clk);

        PllSetup {
            use_pll: main_pll.use_pll,
            use_i2spll: i2s_pll.use_pll,
            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            use_saipll: sai_pll.use_pll,
            pllsysclk: main_pll.pllsysclk,
            pll48clk: main_pll.pll48clk,
            i2s: i2s_clocks.real(i2s_pll.plli2sclk, self.i2s_ckin),
            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            sai: sai_clocks.real(sai_pll.sai_clk, self.i2s_ckin),
        }
    }

    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    fn sai_clocks(&self) -> SaiClocks {
        let sai1_ext = self.sai1_clk.is_some() && self.sai1_clk == self.i2s_ckin;
        #[cfg(not(feature = "stm32f446"))]
        let sai2_ext = self.sai2_clk.is_some() && self.sai2_clk == self.i2s_ckin;
        // Not the PLL output, but the target clock after the divider.
        let pll_sai_clk = if sai1_ext { None } else { self.sai1_clk };
        // The STM32F446 only supports I2S_CKIN for SAI1.
        #[cfg(feature = "stm32f446")]
        let pll_sai_clk2 = self.sai2_clk;
        #[cfg(not(feature = "stm32f446"))]
        let pll_sai_clk2 = if sai2_ext { None } else { self.sai2_clk };
        if pll_sai_clk.is_some() && pll_sai_clk2.is_some() && pll_sai_clk != pll_sai_clk2 {
            panic!("only one SAI PLL frequency implemented");
        }
        SaiClocks {
            sai1_ext,
            #[cfg(not(feature = "stm32f446"))]
            sai2_ext,
            pll_sai_clk,
        }
    }

    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
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

    #[cfg(not(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    )))]
    fn i2s_clocks(&self) -> I2sClocks {
        let i2s_ext = self.i2s_clk.is_some() && self.i2s_clk == self.i2s_ckin;
        let pll_i2s_clk = if i2s_ext { None } else { self.i2s_clk };
        I2sClocks {
            i2s_ext,
            pll_i2s_clk,
        }
    }

    fn flash_setup(sysclk: u32) {
        use crate::stm32::FLASH;

        #[cfg(any(
            feature = "stm32f401",
            feature = "stm32f405",
            feature = "stm32f407",
            feature = "stm32f410",
            feature = "stm32f411",
            feature = "stm32f412",
            feature = "stm32f415",
            feature = "stm32f417",
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f446",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        let flash_latency_step = 30_000_000;

        #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
        let flash_latency_step = 25_000_000;

        unsafe {
            let flash = &(*FLASH::ptr());
            // Adjust flash wait states
            flash.acr.modify(|_, w| {
                w.latency().bits(((sysclk - 1) / flash_latency_step) as u8);
                w.prften().set_bit();
                w.icen().set_bit();
                w.dcen().set_bit()
            })
        }
    }

    /// Initialises the hardware according to CFGR state returning a Clocks instance.
    /// Panics if overclocking is attempted.
    pub fn freeze(self) -> Clocks {
        self.freeze_internal(false)
    }

    /// Initialises the hardware according to CFGR state returning a Clocks instance.
    /// Allows overclocking.
    pub unsafe fn freeze_unchecked(self) -> Clocks {
        self.freeze_internal(true)
    }

    pub fn freeze_internal(self, unchecked: bool) -> Clocks {
        let rcc = unsafe { &*RCC::ptr() };

        //let (use_pll, sysclk_on_pll, sysclk, pll48clk) = self.pll_setup();
        let pllsrcclk = self.hse.unwrap_or(HSI);
        let sysclk = self.sysclk.unwrap_or(pllsrcclk);
        let sysclk_on_pll = sysclk != pllsrcclk;

        let plls = self.pll_setup(pllsrcclk, if sysclk_on_pll { Some(sysclk) } else { None });
        let sysclk = if sysclk_on_pll {
            plls.pllsysclk.unwrap()
        } else {
            sysclk
        };

        assert!(unchecked || !sysclk_on_pll || sysclk <= SYSCLK_MAX && sysclk >= SYSCLK_MIN);

        let hclk = self.hclk.unwrap_or(sysclk);
        let (hpre_bits, hpre_div) = match (sysclk + hclk - 1) / hclk {
            0 => unreachable!(),
            1 => (HPRE_A::DIV1, 1),
            2 => (HPRE_A::DIV2, 2),
            3..=5 => (HPRE_A::DIV4, 4),
            6..=11 => (HPRE_A::DIV8, 8),
            12..=39 => (HPRE_A::DIV16, 16),
            40..=95 => (HPRE_A::DIV64, 64),
            96..=191 => (HPRE_A::DIV128, 128),
            192..=383 => (HPRE_A::DIV256, 256),
            _ => (HPRE_A::DIV512, 512),
        };

        // Calculate real AHB clock
        let hclk = sysclk / hpre_div;

        let pclk1 = self
            .pclk1
            .unwrap_or_else(|| core::cmp::min(PCLK1_MAX, hclk));
        let (ppre1_bits, ppre1) = match (hclk + pclk1 - 1) / pclk1 {
            0 => unreachable!(),
            1 => (0b000, 1),
            2 => (0b100, 2),
            3..=5 => (0b101, 4),
            6..=11 => (0b110, 8),
            _ => (0b111, 16),
        };

        // Calculate real APB1 clock
        let pclk1 = hclk / u32::from(ppre1);

        assert!(unchecked || pclk1 <= PCLK1_MAX);

        let pclk2 = self
            .pclk2
            .unwrap_or_else(|| core::cmp::min(PCLK2_MAX, hclk));
        let (ppre2_bits, ppre2) = match (hclk + pclk2 - 1) / pclk2 {
            0 => unreachable!(),
            1 => (0b000, 1),
            2 => (0b100, 2),
            3..=5 => (0b101, 4),
            6..=11 => (0b110, 8),
            _ => (0b111, 16),
        };

        // Calculate real APB2 clock
        let pclk2 = hclk / u32::from(ppre2);

        assert!(unchecked || pclk2 <= PCLK2_MAX);

        Self::flash_setup(sysclk);

        if self.hse.is_some() {
            // enable HSE and wait for it to be ready
            rcc.cr.modify(|_, w| w.hseon().set_bit());
            while rcc.cr.read().hserdy().bit_is_clear() {}
        }

        if plls.use_pll {
            // Enable PLL
            rcc.cr.modify(|_, w| w.pllon().set_bit());

            // Enable voltage regulator overdrive if HCLK is above the limit
            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            if hclk > 168_000_000 {
                // Enable clock for PWR peripheral
                rcc.apb1enr.modify(|_, w| w.pwren().set_bit());

                let pwr = unsafe { &*crate::stm32::PWR::ptr() };
                pwr.cr.modify(|_, w| w.oden().set_bit());
                while pwr.csr.read().odrdy().bit_is_clear() {}
                pwr.cr.modify(|_, w| w.odswen().set_bit());
                while pwr.csr.read().odswrdy().bit_is_clear() {}
            }

            // Wait for PLL to stabilise
            while rcc.cr.read().pllrdy().bit_is_clear() {}
        }

        #[cfg(not(feature = "stm32f410"))]
        if plls.use_i2spll {
            // Enable PLL.
            rcc.cr.modify(|_, w| w.plli2son().set_bit());

            // Wait for PLL to stabilise
            while rcc.cr.read().plli2srdy().bit_is_clear() {}
        }

        #[cfg(any(
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f446",
            feature = "stm32f469",
            feature = "stm32f479",
        ))]
        if plls.use_saipll {
            // Enable PLL.
            rcc.cr.modify(|_, w| w.pllsaion().set_bit());

            // Wait for PLL to stabilise
            while rcc.cr.read().pllsairdy().bit_is_clear() {}
        }

        // Select I2S and SAI clocks
        plls.i2s.config_clocksel();
        #[cfg(any(
            feature = "stm32f413",
            feature = "stm32f423",
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f446",
            feature = "stm32f469",
            feature = "stm32f479",
        ))]
        plls.sai.config_clocksel();

        // Set scaling factors
        rcc.cfgr.modify(|_, w| unsafe {
            w.ppre2()
                .bits(ppre2_bits)
                .ppre1()
                .bits(ppre1_bits)
                .hpre()
                .variant(hpre_bits)
        });

        // Wait for the new prescalers to kick in
        // "The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after write"
        cortex_m::asm::delay(16);

        // Select system clock source
        rcc.cfgr.modify(|_, w| {
            w.sw().variant(if sysclk_on_pll {
                SW_A::PLL
            } else if self.hse.is_some() {
                SW_A::HSE
            } else {
                SW_A::HSI
            })
        });

        let clocks = Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
            pll48clk: plls.pll48clk.map(|clk| Hertz(clk)),

            #[cfg(not(any(
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446",
            )))]
            i2s_clk: plls.i2s.i2s_clk.map(|clk| Hertz(clk)),
            #[cfg(any(
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446",
            ))]
            i2s_apb1_clk: plls.i2s.i2s_apb1_clk.map(|clk| Hertz(clk)),
            #[cfg(any(
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446",
            ))]
            i2s_apb2_clk: plls.i2s.i2s_apb2_clk.map(|clk| Hertz(clk)),

            #[cfg(any(
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479",
            ))]
            saia_clk: plls.sai.sai1_clk.map(|clk| Hertz(clk)),
            #[cfg(any(
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479",
            ))]
            saib_clk: plls.sai.sai2_clk.map(|clk| Hertz(clk)),
            #[cfg(feature = "stm32f446")]
            sai1_clk: plls.sai.sai1_clk.map(|clk| Hertz(clk)),
            #[cfg(feature = "stm32f446")]
            sai2_clk: plls.sai.sai2_clk.map(|clk| Hertz(clk)),
        };

        if self.pll48clk {
            assert!(clocks.is_pll48clk_valid());
        }

        clocks
    }
}

struct PllSetup {
    use_pll: bool,
    #[cfg(not(feature = "stm32f410"))]
    use_i2spll: bool,
    #[cfg(any(
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    use_saipll: bool,

    pllsysclk: Option<u32>,
    pll48clk: Option<u32>,

    i2s: RealI2sClocks,

    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    sai: RealSaiClocks,
}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
struct I2sClocks {
    /// True if the clock for the APB1 I2S instances is identical to I2S_CKIN.
    i2s_apb1_ext: bool,
    /// True if the clock for the APB2 I2S instances is identical to I2S_CKIN.
    i2s_apb2_ext: bool,
    /// Target for the I2S PLL output.
    pll_i2s_clk: Option<u32>,
}

#[cfg(not(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
)))]
struct I2sClocks {
    /// True if the clock for I2S is identical to I2S_CKIN.
    i2s_ext: bool,
    /// Target for the I2S PLL output.
    pll_i2s_clk: Option<u32>,
}

impl I2sClocks {
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    fn real(&self, pll_i2s_clk: Option<u32>, i2s_ckin: Option<u32>) -> RealI2sClocks {
        RealI2sClocks {
            i2s_apb1_ext: self.i2s_apb1_ext,
            i2s_apb2_ext: self.i2s_apb2_ext,
            i2s_apb1_clk: if self.i2s_apb1_ext {
                i2s_ckin
            } else {
                pll_i2s_clk
            },
            i2s_apb2_clk: if self.i2s_apb2_ext {
                i2s_ckin
            } else {
                pll_i2s_clk
            },
        }
    }

    #[cfg(not(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    )))]
    fn real(&self, pll_i2s_clk: Option<u32>, i2s_ckin: Option<u32>) -> RealI2sClocks {
        RealI2sClocks {
            i2s_ext: self.i2s_ext,
            i2s_clk: if self.i2s_ext { i2s_ckin } else { pll_i2s_clk },
        }
    }
}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
struct RealI2sClocks {
    i2s_apb1_ext: bool,
    i2s_apb2_ext: bool,
    i2s_apb1_clk: Option<u32>,
    i2s_apb2_clk: Option<u32>,
}

#[cfg(not(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
)))]
struct RealI2sClocks {
    i2s_ext: bool,
    i2s_clk: Option<u32>,
}

impl RealI2sClocks {
    fn config_clocksel(&self) {
        let rcc = unsafe { &*RCC::ptr() };

        #[cfg(not(any(
            feature = "stm32f410",
            feature = "stm32f412",
            feature = "stm32f413",
            feature = "stm32f423",
            feature = "stm32f446",
        )))]
        if self.i2s_ext {
            rcc.cfgr.modify(|_, w| w.i2ssrc().ckin());
        } else {
            rcc.cfgr.modify(|_, w| w.i2ssrc().plli2s());
        }
        #[cfg(feature = "stm32f410")]
        if self.i2s_ext {
            rcc.dckcfgr.modify(|_, w| w.i2ssrc().i2s_ckin());
        } else {
            rcc.dckcfgr.modify(|_, w| w.i2ssrc().pllclkr());
        }
        #[cfg(any(
            feature = "stm32f412",
            feature = "stm32f413",
            feature = "stm32f423",
            feature = "stm32f446",
        ))]
        if self.i2s_apb1_ext {
            rcc.dckcfgr.modify(|_, w| w.i2s1src().i2s_ckin());
        } else {
            rcc.dckcfgr.modify(|_, w| w.i2s1src().plli2sr());
        }
        #[cfg(any(
            feature = "stm32f412",
            feature = "stm32f413",
            feature = "stm32f423",
            feature = "stm32f446",
        ))]
        if self.i2s_apb2_ext {
            rcc.dckcfgr.modify(|_, w| w.i2s2src().i2s_ckin());
        } else {
            rcc.dckcfgr.modify(|_, w| w.i2s2src().plli2sr());
        }
    }
}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
struct SaiClocks {
    /// True if the clock for SAI1 (STM32F446) or SAIA (all other models) is identical to I2S_CKIN.
    sai1_ext: bool,
    /// True if the clock for SAIB is identical to I2S_CKIN.
    #[cfg(not(feature = "stm32f446"))]
    sai2_ext: bool,
    /// Target for the SAI clock as generated by PLL and SAI clock divider.
    pll_sai_clk: Option<u32>,
}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
impl SaiClocks {
    fn real(&self, pll_sai_clk: Option<u32>, i2s_ckin: Option<u32>) -> RealSaiClocks {
        RealSaiClocks {
            sai1_ext: self.sai1_ext,
            #[cfg(not(feature = "stm32f446"))]
            sai2_ext: self.sai2_ext,
            sai1_clk: if self.sai1_ext { i2s_ckin } else { pll_sai_clk },
            #[cfg(not(feature = "stm32f446"))]
            sai2_clk: if self.sai2_ext { i2s_ckin } else { pll_sai_clk },
            #[cfg(feature = "stm32f446")]
            sai2_clk: pll_sai_clk,
        }
    }
}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
struct RealSaiClocks {
    sai1_ext: bool,
    #[cfg(not(feature = "stm32f446"))]
    sai2_ext: bool,
    sai1_clk: Option<u32>,
    sai2_clk: Option<u32>,
}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
impl RealSaiClocks {
    fn config_clocksel(&self) {
        let rcc = unsafe { &*RCC::ptr() };

        // Configure SAI clocks.
        #[cfg(not(feature = "stm32f446",))]
        if self.sai1_ext {
            rcc.dckcfgr.modify(|_, w| w.sai1asrc().i2s_ckin());
        } else {
            rcc.dckcfgr.modify(|_, w| w.sai1asrc().pllsai());
        }
        #[cfg(not(feature = "stm32f446",))]
        if self.sai2_ext {
            rcc.dckcfgr.modify(|_, w| w.sai1bsrc().i2s_ckin());
        } else {
            rcc.dckcfgr.modify(|_, w| w.sai1bsrc().pllsai());
        }
        #[cfg(feature = "stm32f446")]
        if self.sai1_ext {
            rcc.dckcfgr.modify(|_, w| w.sai1src().i2s_ckin());
        } else {
            rcc.dckcfgr.modify(|_, w| w.sai1src().pllsai());
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
    pll48clk: Option<Hertz>,

    #[cfg(not(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    )))]
    i2s_clk: Option<Hertz>,
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    i2s_apb1_clk: Option<Hertz>,
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    i2s_apb2_clk: Option<Hertz>,

    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    saia_clk: Option<Hertz>,
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    saib_clk: Option<Hertz>,
    #[cfg(feature = "stm32f446")]
    sai1_clk: Option<Hertz>,
    #[cfg(feature = "stm32f446")]
    sai2_clk: Option<Hertz>,
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

    /// Returns the prescaler of the APB1
    pub fn ppre1(&self) -> u8 {
        self.ppre1
    }

    /// Returns the prescaler of the APB2
    pub fn ppre2(&self) -> u8 {
        self.ppre2
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
            .map(|freq| (48_000_000 - freq.0 as i32).abs() <= 120_000)
            .unwrap_or(false)
    }

    /// Returns the frequency of the I2S clock.
    #[cfg(not(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    )))]
    pub fn i2s_clk(&self) -> Option<Hertz> {
        self.i2s_clk
    }
    /// Returns the frequency of the first I2S clock (for the I2S peripherals on APB1).
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    pub fn i2s_apb1_clk(&self) -> Option<Hertz> {
        self.i2s_apb1_clk
    }
    /// Returns the frequency of the second I2S clock (for the I2S peripherals on APB2).
    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    pub fn i2s_apb2_clk(&self) -> Option<Hertz> {
        self.i2s_apb2_clk
    }

    /// Returns the frequency of the SAI A clock.
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    pub fn saia_clk(&self) -> Option<Hertz> {
        self.saia_clk
    }
    /// Returns the frequency of the SAI B clock.
    #[cfg(any(
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    pub fn saib_clk(&self) -> Option<Hertz> {
        self.saib_clk
    }
    /// Returns the frequency of the SAI1 clock.
    #[cfg(feature = "stm32f446")]
    pub fn sai1_clk(&self) -> Option<Hertz> {
        self.sai1_clk
    }
    /// Returns the frequency of the SAI2 clock.
    #[cfg(feature = "stm32f446")]
    pub fn sai2_clk(&self) -> Option<Hertz> {
        self.sai2_clk
    }
}