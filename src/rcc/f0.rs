use crate::pac::RCC;
use crate::time::Hertz;

/// Extension trait that sets up the `RCC` peripheral
pub trait RccExt {
    /// Configure the clocks of the RCC peripheral
    fn configure(self) -> CFGR;
}

impl RccExt for RCC {
    fn configure(self) -> CFGR {
        CFGR {
            hclk: None,
            pclk: None,
            sysclk: None,
            clock_src: SysClkSource::HSI,
            /// CRS is only available on devices with HSI48
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f071",
                feature = "stm32f072",
                feature = "stm32f078",
                feature = "stm32f091",
                feature = "stm32f098",
            ))]
            crs: None,
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f072",
                feature = "stm32f078",
            ))]
            usb_src: USBClockSource::HSI48,
            #[cfg(feature = "stm32f070")]
            usb_src: USBClockSource::Disabled,
            rcc: self,
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    pub clocks: Clocks,
    pub(crate) regs: RCC,
}

pub enum HSEBypassMode {
    /// Not bypassed: for crystals
    NotBypassed,
    /// Bypassed: for external clock sources
    Bypassed,
}
#[cfg(any(
    feature = "stm32f042",
    feature = "stm32f048",
    feature = "stm32f070", // Doesn't have HSI48
    feature = "stm32f072",
    feature = "stm32f078",
))]
#[allow(clippy::upper_case_acronyms)]
pub enum USBClockSource {
    #[cfg(feature = "stm32f070")]
    /// USB peripheral's tranceiver is disabled
    Disabled,
    #[cfg(not(feature = "stm32f070"))]
    /// HSI48 is used as USB peripheral tranceiver clock
    HSI48,
    /// PLL output is used as USB peripheral tranceiver clock
    PLL,
}
/// RCC for F0x0 devices
#[cfg(any(feature = "stm32f030", feature = "stm32f070",))]
mod inner {
    use crate::pac::{rcc::cfgr::SW_A, RCC};

    pub(super) const HSI: u32 = 8_000_000; // Hz

    // Does PLLSRC have two bits?
    #[cfg(any(
        feature = "stm32f030x4",
        feature = "stm32f030x6",
        feature = "stm32f030x8",
        // Reference Manual states HSI_PREDIV is not available on stm32f070/, but it actually
        // seems to work, and also is referenced in its CMSIS header files stm32f070x6.h and stm32f070xb.h
        // But, for now be conservative and mark it as not available.
        feature = "stm32f070",
        // It's also listed in the CMSIS headers for the f030xc,
        feature = "stm32f030xc"
    ))]
    pub(super) const RCC_PLLSRC_PREDIV1_SUPPORT: bool = false;

    #[allow(clippy::upper_case_acronyms)]
    pub(super) enum SysClkSource {
        HSI,
        /// High-speed external clock(freq,bypassed)
        HSE(u32, super::HSEBypassMode),
    }

    pub(super) fn get_freq(c_src: &SysClkSource) -> u32 {
        // Select clock source based on user input and capability
        // Highest selected frequency source available takes precedent.
        match c_src {
            SysClkSource::HSE(freq, _) => *freq,
            _ => HSI,
        }
    }

    pub(super) fn enable_clock(rcc: &mut RCC, c_src: &SysClkSource) {
        // Enable the requested clock
        match c_src {
            SysClkSource::HSE(_, bypassed) => {
                match bypassed {
                    super::HSEBypassMode::NotBypassed => {
                        rcc.cr
                            .modify(|_, w| w.csson().on().hseon().on().hsebyp().not_bypassed());
                    }
                    super::HSEBypassMode::Bypassed => {
                        rcc.cr
                            .modify(|_, w| w.csson().on().hseon().on().hsebyp().bypassed());
                    }
                }
                while !rcc.cr.read().hserdy().bit_is_set() {}
            }
            SysClkSource::HSI => {
                rcc.cr.write(|w| w.hsion().set_bit());
                while rcc.cr.read().hsirdy().bit_is_clear() {}
            }
        }
    }

    pub(super) fn enable_pll(
        rcc: &mut RCC,
        c_src: &SysClkSource,
        pllmul_bits: u8,
        ppre_bits: u8,
        hpre_bits: u8,
    ) {
        let pllsrc_bit: bool = match c_src {
            SysClkSource::HSI => false,
            SysClkSource::HSE(_, _) => true,
        };

        // Set PLL source and multiplier
        rcc.cfgr
            .modify(|_, w| w.pllsrc().bit(pllsrc_bit).pllmul().bits(pllmul_bits));

        rcc.cr.modify(|_, w| w.pllon().set_bit());
        while rcc.cr.read().pllrdy().bit_is_clear() {}

        rcc.cfgr
            .modify(|_, w| unsafe { w.ppre().bits(ppre_bits).hpre().bits(hpre_bits).sw().pll() });
    }

    pub(super) fn get_sww(c_src: &SysClkSource) -> SW_A {
        match c_src {
            SysClkSource::HSI => SW_A::HSI,
            SysClkSource::HSE(_, _) => SW_A::HSE,
        }
    }
}
/// RCC for F0x1, F0x2, F0x8 devices
#[cfg(any(
    feature = "stm32f031",
    feature = "stm32f038",
    feature = "stm32f042",
    feature = "stm32f048",
    feature = "stm32f051",
    feature = "stm32f058",
    feature = "stm32f071",
    feature = "stm32f072",
    feature = "stm32f078",
    feature = "stm32f091",
    feature = "stm32f098",
))]
mod inner {
    use crate::pac::{rcc::cfgr::SW_A, RCC};

    pub(super) const HSI: u32 = 8_000_000; // Hz
    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f071",
        feature = "stm32f072",
        feature = "stm32f078",
        feature = "stm32f091",
        feature = "stm32f098",
    ))]
    pub(super) const HSI48: u32 = 48_000_000; // Hz

    // Does PLLSRC have two bits?
    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f071",
        feature = "stm32f072",
        feature = "stm32f078",
        feature = "stm32f091",
        feature = "stm32f098",
    ))]
    pub(super) const RCC_PLLSRC_PREDIV1_SUPPORT: bool = true;
    #[cfg(any(
        feature = "stm32f031",
        feature = "stm32f038",
        feature = "stm32f051",
        feature = "stm32f058"
    ))]
    pub(super) const RCC_PLLSRC_PREDIV1_SUPPORT: bool = false;

    #[allow(clippy::upper_case_acronyms)]
    pub(super) enum SysClkSource {
        HSI,
        /// High-speed external clock(freq,bypassed)
        HSE(u32, super::HSEBypassMode),
        #[cfg(any(
            feature = "stm32f042",
            feature = "stm32f048",
            feature = "stm32f071",
            feature = "stm32f072",
            feature = "stm32f078",
            feature = "stm32f091",
            feature = "stm32f098",
        ))]
        HSI48,
    }

    pub(super) fn get_freq(c_src: &SysClkSource) -> u32 {
        // Select clock source based on user input and capability
        // Highest selected frequency source available takes precedent.
        match c_src {
            SysClkSource::HSE(freq, _) => *freq,
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f071",
                feature = "stm32f072",
                feature = "stm32f078",
                feature = "stm32f091",
                feature = "stm32f098",
            ))]
            SysClkSource::HSI48 => HSI48,
            _ => HSI,
        }
    }

    pub(super) fn enable_clock(rcc: &mut RCC, c_src: &SysClkSource) {
        // Enable the requested clock
        match c_src {
            SysClkSource::HSE(_, bypassed) => {
                match bypassed {
                    super::HSEBypassMode::NotBypassed => {
                        rcc.cr
                            .modify(|_, w| w.csson().on().hseon().on().hsebyp().not_bypassed());
                    }
                    super::HSEBypassMode::Bypassed => {
                        rcc.cr
                            .modify(|_, w| w.csson().on().hseon().on().hsebyp().bypassed());
                    }
                }

                while !rcc.cr.read().hserdy().bit_is_set() {}
            }
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f071",
                feature = "stm32f072",
                feature = "stm32f078",
                feature = "stm32f091",
                feature = "stm32f098",
            ))]
            SysClkSource::HSI48 => {
                rcc.cr2.modify(|_, w| w.hsi48on().set_bit());
                while rcc.cr2.read().hsi48rdy().bit_is_clear() {}
            }
            SysClkSource::HSI => {
                rcc.cr.write(|w| w.hsion().set_bit());
                while rcc.cr.read().hsirdy().bit_is_clear() {}
            }
        }
    }

    pub(super) fn enable_pll(
        rcc: &mut RCC,
        c_src: &SysClkSource,
        pllmul_bits: u8,
        ppre_bits: u8,
        hpre_bits: u8,
    ) {
        let pllsrc_bit: u8 = match (c_src, RCC_PLLSRC_PREDIV1_SUPPORT) {
            (SysClkSource::HSI, false) => 0b00, // HSI/2
            (SysClkSource::HSI, true) => 0b01,  // HSI/PREDIV
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f071",
                feature = "stm32f072",
                feature = "stm32f078",
                feature = "stm32f091",
                feature = "stm32f098",
            ))]
            (SysClkSource::HSI48, _) => 0b11,
            (SysClkSource::HSE(_, _), _) => 0b10,
        };

        // Set PLL source and multiplier
        rcc.cfgr
            .modify(|_, w| w.pllsrc().bits(pllsrc_bit).pllmul().bits(pllmul_bits));

        rcc.cr.modify(|_, w| w.pllon().set_bit());
        while rcc.cr.read().pllrdy().bit_is_clear() {}

        rcc.cfgr
            .modify(|_, w| unsafe { w.ppre().bits(ppre_bits).hpre().bits(hpre_bits).sw().pll() });
    }

    pub(super) fn get_sww(c_src: &SysClkSource) -> SW_A {
        match c_src {
            SysClkSource::HSI => SW_A::HSI,
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f071",
                feature = "stm32f072",
                feature = "stm32f078",
                feature = "stm32f091",
                feature = "stm32f098",
            ))]
            SysClkSource::HSI48 => SW_A::HSI48,
            SysClkSource::HSE(_, _) => SW_A::HSE,
        }
    }
}

use self::inner::SysClkSource;

pub struct CFGR {
    hclk: Option<u32>,
    pclk: Option<u32>,
    sysclk: Option<u32>,
    clock_src: SysClkSource,
    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f070",
        feature = "stm32f072",
        feature = "stm32f078",
    ))]
    usb_src: USBClockSource,
    /// CRS is only available on devices with HSI48
    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f071",
        feature = "stm32f072",
        feature = "stm32f078",
        feature = "stm32f091",
        feature = "stm32f098",
    ))]
    crs: Option<crate::pac::CRS>,
    rcc: RCC,
}

impl CFGR {
    pub fn hse<F>(mut self, freq: F, bypass: HSEBypassMode) -> Self
    where
        F: Into<Hertz>,
    {
        self.clock_src = SysClkSource::HSE(freq.into().0, bypass);
        self
    }

    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f071",
        feature = "stm32f072",
        feature = "stm32f078",
        feature = "stm32f091",
        feature = "stm32f098",
    ))]
    pub fn hsi48(mut self) -> Self {
        self.clock_src = SysClkSource::HSI48;
        self
    }

    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk = Some(freq.into().0);
        self
    }

    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
        self
    }
    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f070",
        feature = "stm32f072",
        feature = "stm32f078",
    ))]
    /// Set the USB clock source (only valid for STM32F0xx with USB)
    pub fn usbsrc(mut self, src: USBClockSource) -> Self {
        self.usb_src = src;
        self
    }

    #[cfg(any(
        feature = "stm32f042",
        feature = "stm32f048",
        feature = "stm32f071",
        feature = "stm32f072",
        feature = "stm32f078",
        feature = "stm32f091",
        feature = "stm32f098",
    ))]
    pub fn enable_crs(mut self, crs: crate::pac::CRS) -> Self {
        self.crs = Some(crs);
        self
    }

    pub fn freeze(mut self, flash: &mut crate::pac::FLASH) -> Rcc {
        // Default to lowest frequency clock on all systems.
        let sysclk = self.sysclk.unwrap_or(self::inner::HSI);

        let r_sysclk; // The "real" sysclock value, calculated below
        let pllmul_bits;

        // Select clock source based on user input and capability
        // Highest selected frequency source available takes precedent.
        // For F04x, F07x, F09x parts, use HSI48 if requested.
        let src_clk_freq = self::inner::get_freq(&self.clock_src);

        // Pll check
        if sysclk == src_clk_freq {
            // Bypass pll if src clk and requested sysclk are the same, to save power.
            // The only reason to override this behaviour is if the sysclk source were HSI, and you
            // were running the USB off the PLL...
            pllmul_bits = None;
            r_sysclk = src_clk_freq;
        } else {
            // FIXME: This assumes reset value of prediv (/1).
            //        There is no logic to set plldiv to any value other than 1.
            // Note that for some models, HSI is fixed by hardware to divide by two.
            let pllprediv = match (&self.clock_src, self::inner::RCC_PLLSRC_PREDIV1_SUPPORT) {
                (self::inner::SysClkSource::HSI, false) => 2,
                (_, _) => 1,
            };
            // Find PLL multiplier that creates freq closest to target
            let pllmul = (2 * pllprediv * self.sysclk.unwrap_or(src_clk_freq) + src_clk_freq)
                / src_clk_freq
                / 2;
            let pllmul = core::cmp::min(core::cmp::max(pllmul, 2), 16);
            r_sysclk = pllmul * src_clk_freq / pllprediv;

            pllmul_bits = Some(pllmul as u8 - 2)
        }

        let hpre_bits = self
            .hclk
            .map(|hclk| match r_sysclk / hclk {
                0 => unreachable!(),
                1 => 0b0111,
                2 => 0b1000,
                3..=5 => 0b1001,
                6..=11 => 0b1010,
                12..=39 => 0b1011,
                40..=95 => 0b1100,
                96..=191 => 0b1101,
                192..=383 => 0b1110,
                _ => 0b1111,
            })
            .unwrap_or(0b0111);

        let hclk = r_sysclk / (1 << (hpre_bits - 0b0111));

        let ppre_bits = self
            .pclk
            .map(|pclk| match hclk / pclk {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3..=5 => 0b101,
                6..=11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre: u8 = 1 << (ppre_bits - 0b011);
        let pclk = hclk / cast::u32(ppre);

        // adjust flash wait states
        unsafe {
            flash.acr.write(|w| {
                w.latency().bits(if r_sysclk <= 24_000_000 {
                    0b000
                } else if r_sysclk <= 48_000_000 {
                    0b001
                } else {
                    0b010
                })
            })
        }

        // Enable the requested clock
        self::inner::enable_clock(&mut self.rcc, &self.clock_src);

        // Only need to set USBSW if MCU has USB HW
        #[cfg(feature = "stm32f070")]
        {
            match self.usb_src {
                USBClockSource::Disabled => self.rcc.cfgr3.modify(|_, w| w.usbsw().clear_bit()),
                USBClockSource::PLL => self.rcc.cfgr3.modify(|_, w| w.usbsw().set_bit()),
            }
        }
        #[cfg(any(
            feature = "stm32f042",
            feature = "stm32f048",
            feature = "stm32f072",
            feature = "stm32f078",
        ))]
        match self.usb_src {
            USBClockSource::HSI48 => self.rcc.cfgr3.modify(|_, w| w.usbsw().clear_bit()),
            USBClockSource::PLL => self.rcc.cfgr3.modify(|_, w| w.usbsw().set_bit()),
        }
        // Set up rcc based on above calculated configuration.

        // Enable PLL
        if let Some(pllmul_bits) = pllmul_bits {
            self::inner::enable_pll(
                &mut self.rcc,
                &self.clock_src,
                pllmul_bits,
                ppre_bits,
                hpre_bits,
            );
        } else {
            let sw_var = self::inner::get_sww(&self.clock_src);

            // CRS is only available on devices with HSI48
            #[cfg(any(
                feature = "stm32f042",
                feature = "stm32f048",
                feature = "stm32f071",
                feature = "stm32f072",
                feature = "stm32f078",
                feature = "stm32f091",
                feature = "stm32f098",
            ))]
            match self.crs {
                Some(crs) => {
                    self.rcc.apb1enr.modify(|_, w| w.crsen().set_bit());

                    // Initialize clock recovery
                    // Set autotrim enabled.
                    crs.cr.modify(|_, w| w.autotrimen().set_bit());
                    // Enable CR
                    crs.cr.modify(|_, w| w.cen().set_bit());
                }
                _ => {}
            }

            // use HSI as source
            self.rcc.cfgr.modify(|_, w| unsafe {
                w.ppre()
                    .bits(ppre_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .variant(sw_var)
            });
        }
        Rcc {
            clocks: Clocks {
                hclk: Hertz(hclk),
                pclk: Hertz(pclk),
                sysclk: Hertz(sysclk),
            },
            regs: self.rcc,
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk: Hertz,
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB
    pub fn pclk(&self) -> Hertz {
        self.pclk
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
