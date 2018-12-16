use core::cmp::min;

use crate::stm32::{FLASH, RCC};

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
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
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

    pub fn freeze(self) -> Clocks {
        let flash = unsafe { &(*FLASH::ptr()) };
        let rcc = unsafe { &*RCC::ptr() };

        let sysclk = self.sysclk.unwrap_or(HSI);
        let mut hclk = self.hclk.unwrap_or(sysclk);

        assert!(hclk <= sysclk);

        if sysclk == HSI {
            let hpre_bits = match sysclk / hclk {
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
            };

            // Use HSI as source and run everything at the same speed
            rcc.cfgr.modify(|_, w| unsafe {
                w.ppre2()
                    .bits(0)
                    .ppre1()
                    .bits(0)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .hsi()
            });

            Clocks {
                hclk: Hertz(hclk),
                pclk1: Hertz(hclk),
                pclk2: Hertz(hclk),
                ppre1: 1,
                ppre2: 1,
                sysclk: Hertz(sysclk),
            }
        } else {
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

            assert!(sysclk <= sysclk_max && sysclk >= sysclk_min);

            // We're not diving down the hclk so it'll be the same as sysclk
            hclk = sysclk;

            // Sysclk output divisor must be one of 2, 4, 6 or 8
            let sysclk_div = min(8, (432_000_000 / sysclk) & !1);

            // Input divisor from HSI clock, must result to frequency in
            // the range from 1 to 2 MHz
            let pllm = 16;

            // Main scaler, must result in >= 100MHz (>= 192MHz for F401)
            // and <= 432MHz, min 50, max 432
            let plln = sysclk * sysclk_div / (HSI / pllm as u32);

            let pllp = (sysclk_div as u8 / 2) - 1;

            let (ppre1_bits, ppre2_bits) = match hclk {
                45_000_001...90_000_000 => (0b100, 0b011),
                90_000_001...180_000_000 => (0b101, 0b100),
                _ => (0b011, 0b011),
            };

            // Calculate real divisor
            let ppre1 = 1 << (ppre1_bits - 0b011);
            let ppre2 = 1 << (ppre2_bits - 0b011);

            // Calculate new bus clocks
            let pclk1 = hclk / ppre1 as u32;
            let pclk2 = hclk / ppre2 as u32;

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

            // Adjust flash wait states
            unsafe {
                flash
                    .acr
                    .modify(|_, w| w.latency().bits(((sysclk - 1) / flash_latency_step) as u8))
            }

            // use PLL as source
            rcc.pllcfgr.write(|w| unsafe {
                w.pllm()
                    .bits(pllm)
                    .plln()
                    .bits(plln as u16)
                    .pllp()
                    .bits(pllp)
            });

            // Enable PLL
            rcc.cr.write(|w| w.pllon().set_bit());

            // Wait for PLL to stabilise
            while rcc.cr.read().pllrdy().bit_is_clear() {}

            // Set scaling factors and switch clock to PLL
            rcc.cfgr.modify(|_, w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(0)
                    .sw()
                    .pll()
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
