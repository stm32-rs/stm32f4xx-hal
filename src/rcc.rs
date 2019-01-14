use core::cmp::{max, min};

use crate::stm32::{FLASH, RCC};
use crate::stm32::rcc::pllcfgr::PLLSRCW;
use crate::stm32::rcc::cfgr::SWW;

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
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
                clock_src: ClockSource::HSI,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    pub cfgr: CFGR,
}

struct PLLConfiguration<'a> {
    pllclk: u32,
    source: &'a ClockSource,
}

struct PLLValues {
    pllm: u8,
    plln: u16,
    pllp: u8,
    pllq: u8,
}

struct PLL {
    pllclk: Hertz,
    usbclk: Hertz,
}

impl PLLValues {
    fn pllp_bits(&self) -> u8 {
        (self.pllp as u8 / 2) - 1
    }
}

impl<'a> PLLConfiguration<'a> {
    fn calculate_values(&self) -> PLLValues {
        let src_freq = self.source.frequency();

        let pllm = src_freq / 1_000_000;
        let pllp = min(8, (432_000_000 / self.pllclk) & !1);
        let plln = self.pllclk * pllp / (src_freq / pllm);
        let pllq = max(2, ((src_freq / pllm) * plln) / 48_000_000); // we should never hit the maximum value here, as long as PLLM output is 1MHz

        PLLValues {
            pllm: pllm as u8,
            plln: plln as u16,
            pllp: pllp as u8,
            pllq: pllq as u8,
        }
    }

    fn enable(self/*, rcc: stm32f4::stm32f429::RCC*/) -> PLL {
        let rcc = unsafe { &*RCC::ptr() };

        let clock_ready = match self.source {
            ClockSource::HSI => rcc.cr.read().hsion().bit_is_set() || rcc.cr.read().hsirdy().bit_is_set(),
            ClockSource::HSE(_) => rcc.cr.read().hseon().bit_is_set() || rcc.cr.read().hserdy().bit_is_set()
        };

        // expect clocks to be initialized already
        if !clock_ready {
            panic!("clock not ready");
        }

        let pll_values = self.calculate_values();

        let pllsrc = match self.source {
            ClockSource::HSI => PLLSRCW::HSI,
            ClockSource::HSE(_) => PLLSRCW::HSE
        };

        rcc.pllcfgr.write(|w| unsafe {
            w
                .pllm().bits(pll_values.pllm)
                .plln().bits(pll_values.plln)
                .pllp().bits(pll_values.pllp_bits())
                .pllq().bits(pll_values.pllq)
                .pllsrc().variant(pllsrc)
        });

        // Enable PLL
        rcc.cr.modify(|_, w| w.pllon().set_bit());

        // Wait for PLL to stabilise
        while rcc.cr.read().pllrdy().bit_is_clear() {}

        let plln_output = (self.source.frequency() / pll_values.pllm as u32) * pll_values.plln as u32;

        PLL {
            pllclk: Hertz(plln_output / (&pll_values).pllp as u32),
            usbclk: Hertz(plln_output / pll_values.pllq as u32),
        }
    }
}

const HSI: u32 = 16_000_000; // Hz

enum ClockSource {
    HSI,
    HSE(u32),
}

impl ClockSource {
    fn frequency(&self) -> u32 {
        match self {
            ClockSource::HSI => HSI,
            ClockSource::HSE(freq) => *freq
        }
    }

    fn clock_switch(&self) -> SWW {
        match self {
            ClockSource::HSI => SWW::HSI,
            ClockSource::HSE(_) => SWW::HSE
        }
    }
}

pub struct CFGR {
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
    clock_src: ClockSource,
}

impl CFGR {
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

    pub fn hse<F>(mut self, freq: F) -> Self
        where
            F: Into<Hertz>,
    {
        self.clock_src = ClockSource::HSE(freq.into().0);
        self
    }

    const fn flash_latency(&self, sysclk: u32) -> u8 {
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
        const FLASH_LATENCY_STEP: u32 = 30_000_000;

        #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
        const FLASH_LATENCY_STEP: u32 = 25_000_000;

        ((sysclk - 1) / FLASH_LATENCY_STEP) as u8
    }

    fn hpre_bits(&self, sysclk: u32, hclk: u32) -> u8 {
        match sysclk / hclk {
            0 => unreachable!(),
            1 => 0b0000,
            2 => 0b1000,
            3...5 => 0b1001,
            6...11 => 0b1010,
            12...39 => 0b1011,
            40...95 => 0b1100,
            96...191 => 0b1101,
            192...383 => 0b1110,
            _ => 0b1111,
        }
    }

    fn ppre_bits(&self, hclk: u32) -> (u8, u8) {
        match hclk {
            45_000_001...90_000_000 => (0b100, 0b011),
            90_000_001...180_000_000 => (0b101, 0b100),
            _ => (0b011, 0b011),
        }
    }

    const fn sysclk_max(&self) -> u32 {
        #[cfg(feature = "stm32f401")]
        const SYSCLK_MAX: u32 = 84_000_000;

        #[cfg(any(
            feature = "stm32f405",
            feature = "stm32f407",
            feature = "stm32f415",
            feature = "stm32f417"
        ))]
        const SYSCLK_MAX: u32 = 168_000_000;

        #[cfg(any(
            feature = "stm32f410",
            feature = "stm32f411",
            feature = "stm32f412",
            feature = "stm32f413",
            feature = "stm32f423"
        ))]
        const SYSCLK_MAX: u32 = 100_000_000;

        #[cfg(any(
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f446",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        const SYSCLK_MAX: u32 = 180_000_000;

        SYSCLK_MAX
    }

    const fn sysclk_min(&self) -> u32 {
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
        const SYSCLK_MIN: u32 = 24_000_000;

        #[cfg(any(feature = "stm32f446"))]
        const SYSCLK_MIN: u32 = 12_500_000;

        SYSCLK_MIN
    }

    pub fn freeze(self/*, rcc: stm32f4::stm32f429::RCC*/) -> Clocks {
        let flash = unsafe { &(*FLASH::ptr()) };
        let rcc = unsafe { &*RCC::ptr() };

        let sysclk = self.sysclk.unwrap_or(HSI);
        let hclk = self.hclk.unwrap_or(sysclk);

        assert!(hclk <= sysclk);
        assert!(sysclk <= self.sysclk_max());

        let source_freq = self.clock_src.frequency();
        let sysclk_source = if sysclk == source_freq {
            self.clock_src.clock_switch()
        } else {
            SWW::PLL
        };


        match self.clock_src {
            ClockSource::HSI => {
                rcc.cr.modify(|_, w| w.hsion().set_bit());
                while rcc.cr.read().hsirdy().bit_is_clear() {}
            }
            ClockSource::HSE(_) => {
                rcc.cr.modify(|_, w| w.hseon().set_bit());
                while rcc.cr.read().hserdy().bit_is_clear() {}
            }
        }

        let pll_config = match sysclk_source {
            SWW::PLL => {
                assert!(sysclk >= self.sysclk_min());

                let pll_configuration = PLLConfiguration {
                    source: &self.clock_src,
                    pllclk: sysclk,
                };

                Some(pll_configuration.enable())
            }
            _ => {
                // use PLL to provide 48 MHz clock

                let pll_configuration = PLLConfiguration {
                    source: &self.clock_src,
                    pllclk: 96_000_000
                };

                Some(pll_configuration.enable())
            }
        };

        let (ppre1_bits, ppre2_bits) = self.ppre_bits(hclk);

        // Calculate real divisor
        let ppre1 = 1 << (ppre1_bits - 0b011);
        let ppre2 = 1 << (ppre2_bits - 0b011);

        // Calculate new bus clocks
        let pclk1 = hclk / ppre1 as u32;
        let pclk2 = hclk / ppre2 as u32;

        // Adjust flash wait states
        unsafe {
            flash
                .acr
                .modify(|_, w| w.latency().bits(self.flash_latency(sysclk)))
        }

        rcc.cfgr.modify(|_, w| unsafe {
            w
                .ppre2()
                .bits(ppre2_bits)
                .ppre1()
                .bits(ppre1_bits)
                .hpre()
                .bits(self.hpre_bits(sysclk, hclk))
                .sw()
                .variant(sysclk_source)
        });

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
            usbclk: pll_config.and_then(|c| Some(c.usbclk)),
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
    usbclk: Option<Hertz>,
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
