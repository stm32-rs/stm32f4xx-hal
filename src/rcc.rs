use crate::stm32::RCC;
use crate::stm32::rcc::cfgr::{HPREW, SWW};

use crate::time::Hertz;

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
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    pub cfgr: CFGR,
}

const HSI: u32 = 16_000_000; // Hz

pub struct CFGR {
    hse: Option<u32>,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
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

    fn pll_setup(&self) -> (bool, u32)
    {
        let rcc = unsafe { &*RCC::ptr() };

        let pllsrcclk = self.hse.unwrap_or(HSI);
        let sysclk = self.sysclk.unwrap_or(pllsrcclk);

        // Sysclk output divisor must be one of 2, 4, 6 or 8
        let sysclk_div = core::cmp::min(8, (432_000_000 / sysclk) & !1);

        // Input divisor from PLL source clock, must result to frequency in
        // the range from 1 to 2 MHz
        let pllm_min = (pllsrcclk + 1_999_999) / 2_000_000;
        let pllm_max = pllsrcclk / 1_000_000;

        // Find the lowest pllm value that minimize the difference between
        // requested sysclk and actual sysclk.
        let pllm = (pllm_min..=pllm_max).min_by_key(|pllm| {
            let vco_in = pllsrcclk / pllm;
            let plln = sysclk * sysclk_div / vco_in;
            sysclk - (vco_in * plln / sysclk_div)
        }).unwrap();

        let vco_in = pllsrcclk / pllm;
        assert!(vco_in >= 1_000_000 && vco_in <= 2_000_000);

        // Main scaler, must result in >= 100MHz (>= 192MHz for F401)
        // and <= 432MHz, min 50, max 432
        let plln = sysclk * sysclk_div / vco_in;

        let pllp = (sysclk_div / 2) - 1;

        // Calculate real system clock
        let sysclk = vco_in * plln / sysclk_div;

        if sysclk != pllsrcclk {
            // use PLL as source
            rcc.pllcfgr.write(|w| unsafe {
                w.pllm()
                    .bits(pllm as u8)
                    .plln()
                    .bits(plln as u16)
                    .pllp()
                    .bits(pllp as u8)
                    .pllsrc()
                    .bit(self.hse.is_some())
            });

            (true, sysclk)
        } else {
            (false, pllsrcclk)
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
            flash.acr.modify(|_, w|
                w.latency().bits(((sysclk - 1) / flash_latency_step) as u8)
            )
        }
    }

    pub fn freeze(self) -> Clocks {
        let rcc = unsafe { &*RCC::ptr() };

        let (use_pll, sysclk) = self.pll_setup();

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
        let sysclk_min = 24_000_000;

        #[cfg(any(feature = "stm32f446"))]
        let sysclk_min = 12_500_000;

        #[cfg(feature = "stm32f401")]
        let sysclk_max = 84_000_000;

        #[cfg(any(
            feature = "stm32f405",
            feature = "stm32f407",
            feature = "stm32f415",
            feature = "stm32f417"
        ))]
        let sysclk_max = 168_000_000;

        #[cfg(any(
            feature = "stm32f410",
            feature = "stm32f411",
            feature = "stm32f412",
            feature = "stm32f413",
            feature = "stm32f423"
        ))]
        let sysclk_max = 100_000_000;

        #[cfg(any(
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f446",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        let sysclk_max = 180_000_000;

        assert!(!use_pll || sysclk <= sysclk_max && sysclk >= sysclk_min);

        let hclk = self.hclk.unwrap_or(sysclk);
        let (hpre_bits, hpre_div) = match (sysclk + hclk - 1) / hclk {
            0 => unreachable!(),
            1 => (HPREW::DIV1, 1),
            2 => (HPREW::DIV2, 2),
            3...5 => (HPREW::DIV4, 4),
            6...11 => (HPREW::DIV8, 8),
            12...39 => (HPREW::DIV16, 16),
            40...95 => (HPREW::DIV64, 64),
            96...191 => (HPREW::DIV128, 128),
            192...383 => (HPREW::DIV256, 256),
            _ => (HPREW::DIV512, 512),
        };

        // Calculate real AHB clock
        let hclk = sysclk / hpre_div;

        #[cfg(any(
            feature = "stm32f401",
            feature = "stm32f405",
            feature = "stm32f407",
            feature = "stm32f415",
            feature = "stm32f417"
        ))]
        let (pclk1_max, pclk2_max) = (42_000_000, 84_000_000);
        #[cfg(any(
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f446",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        let (pclk1_max, pclk2_max) = (45_000_000, 90_000_000);
        #[cfg(any(
            feature = "stm32f410",
            feature = "stm32f411",
            feature = "stm32f412",
            feature = "stm32f413",
            feature = "stm32f423"
        ))]
        let (pclk1_max, pclk2_max) = (50_000_000, 100_000_000);

        let pclk1 = self.pclk1.unwrap_or_else(|| core::cmp::min(pclk1_max, hclk));
        let (ppre1_bits, ppre1) = match (hclk + pclk1 - 1) / pclk1 {
            0 => unreachable!(),
            1 => (0b000, 1),
            2 => (0b100, 2),
            3...5 => (0b101, 4),
            6...11 => (0b110, 8),
            _ => (0b111, 16),
        };

        // Calculate real APB1 clock
        let pclk1 = hclk / u32::from(ppre1);

        assert!(pclk1 <= pclk1_max);

        let pclk2 = self.pclk2.unwrap_or_else(|| core::cmp::min(pclk2_max, hclk));
        let (ppre2_bits, ppre2) = match (hclk + pclk2 - 1) / pclk2 {
            0 => unreachable!(),
            1 => (0b000, 1),
            2 => (0b100, 2),
            3...5 => (0b101, 4),
            6...11 => (0b110, 8),
            _ => (0b111, 16),
        };

        // Calculate real APB2 clock
        let pclk2 = hclk / u32::from(ppre2);

        assert!(pclk2 <= pclk2_max);

        Self::flash_setup(sysclk);

        if self.hse.is_some() {
            // enable HSE and wait for it to be ready
            rcc.cr.modify(|_, w| w.hseon().set_bit());
            while rcc.cr.read().hserdy().bit_is_clear() {}
        }

        if use_pll {
            // Enable PLL
            rcc.cr.modify(|_, w| w.pllon().set_bit());

            // Wait for PLL to stabilise
            while rcc.cr.read().pllrdy().bit_is_clear() {}
        }

        // Set scaling factors and select system clock source
        rcc.cfgr.modify(|_, w| unsafe {
            w.ppre2()
                .bits(ppre2_bits)
                .ppre1()
                .bits(ppre1_bits)
                .hpre()
                .variant(hpre_bits)
                .sw()
                .variant(if use_pll {
                    SWW::PLL
                } else if self.hse.is_some() {
                    SWW::HSE
                } else {
                    SWW::HSI
                })
        });

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
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
}
