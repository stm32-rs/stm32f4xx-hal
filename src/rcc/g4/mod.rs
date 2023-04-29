use crate::pac::{self, rcc, FLASH, PWR, RCC};
use crate::time::Hertz;
use fugit::RateExtU32;

mod clockout;
mod config;
mod enable;

pub use clockout::*;
pub use config::*;

pub trait Instance: crate::Sealed + Enable + Reset + BusClock {}

/// HSI speed
pub const HSI_FREQ: u32 = 16_000_000;

/// Clock frequencies
#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    /// System frequency
    pub sysclk: Hertz,
    /// Core frequency
    pub core_clk: Hertz,
    /// AHB frequency
    pub hclk: Hertz,
    /// APB 1 frequency
    pub pclk1: Hertz,
    /// APB 1 timers frequency (Timers 2-7)
    pub timclk1: Hertz,
    /// APB 2 frequency
    pub pclk2: Hertz,
    /// APB 2 timers frequency (Timers 1, 8, 20, 15, 16, 17 and HRTIM1)
    pub timclk2: Hertz,
    /// PLL frequency
    pub pll_clk: PLLClocks,
}

impl Clocks {
    /// Returns the frequency of the AHB
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

    /// Returns the frequency for timers on APB2
    pub fn timclk2(&self) -> Hertz {
        self.timclk2
    }
}

/// PLL Clock frequencies
#[derive(Clone, Copy, Debug)]
pub struct PLLClocks {
    /// R frequency
    pub r: Option<Hertz>,
    /// Q frequency
    pub q: Option<Hertz>,
    /// P frequency
    pub p: Option<Hertz>,
}

impl Default for Clocks {
    fn default() -> Clocks {
        let freq = HSI_FREQ.Hz();
        Clocks {
            sysclk: freq,
            hclk: freq,
            core_clk: freq,
            pclk1: freq,
            timclk1: freq,
            pclk2: freq,
            timclk2: freq,
            pll_clk: PLLClocks {
                r: None,
                q: None,
                p: None,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// Clock configuration
    pub clocks: Clocks,
    pub(crate) rb: RCC,
}

impl Rcc {
    /// Apply clock configuration
    pub fn freeze(self, rcc_cfg: Config) -> Self {
        let pll_clk = self.config_pll(rcc_cfg.pll_cfg);

        let (sysclk, sw_bits) = match rcc_cfg.sys_mux {
            SysClockSrc::HSI => {
                self.enable_hsi();
                (HSI_FREQ.Hz(), 0b01)
            }
            SysClockSrc::HSE(freq) => {
                self.enable_hse(false);
                (freq, 0b10)
            }
            SysClockSrc::PLL => {
                // If PLL is selected as sysclock then the r output should have been configured
                if pll_clk.r.is_none() {
                    panic!("PLL output selected as sysclock but PLL output R is not configured")
                }
                (pll_clk.r.unwrap(), 0b11)
            }
        };

        let sys_freq = sysclk.raw();
        let (ahb_freq, ahb_psc_bits) = match rcc_cfg.ahb_psc {
            Prescaler::Div2 => (sys_freq / 2, 0b1000),
            Prescaler::Div4 => (sys_freq / 4, 0b1001),
            Prescaler::Div8 => (sys_freq / 8, 0b1010),
            Prescaler::Div16 => (sys_freq / 16, 0b1011),
            Prescaler::Div64 => (sys_freq / 64, 0b1100),
            Prescaler::Div128 => (sys_freq / 128, 0b1101),
            Prescaler::Div256 => (sys_freq / 256, 0b1110),
            Prescaler::Div512 => (sys_freq / 512, 0b1111),
            _ => (sysclk.raw(), 0b0000),
        };
        let (apb1_freq, apb1_psc_bits) = match rcc_cfg.apb1_psc {
            Prescaler::Div2 => (sysclk.raw() / 2, 0b100),
            Prescaler::Div4 => (sysclk.raw() / 4, 0b101),
            Prescaler::Div8 => (sysclk.raw() / 8, 0b110),
            Prescaler::Div16 => (sysclk.raw() / 16, 0b111),
            _ => (sysclk.raw(), 0b000),
        };
        let (apb2_freq, apb2_psc_bits) = match rcc_cfg.apb2_psc {
            Prescaler::Div2 => (sysclk.raw() / 2, 0b100),
            Prescaler::Div4 => (sysclk.raw() / 4, 0b101),
            Prescaler::Div8 => (sysclk.raw() / 8, 0b110),
            Prescaler::Div16 => (sysclk.raw() / 16, 0b111),
            _ => (sysclk.raw(), 0b000),
        };

        unsafe {
            // Adjust flash wait states
            let flash = &(*FLASH::ptr());
            flash.acr.modify(|_, w| {
                w.latency().bits(if sysclk.raw() <= 24_000_000 {
                    0b000
                } else if sysclk.raw() <= 48_000_000 {
                    0b001
                } else {
                    0b010
                })
            })
        }

        self.rb.cfgr.modify(|_, w| unsafe {
            w.hpre().bits(ahb_psc_bits);
            w.ppre1().bits(apb1_psc_bits);
            w.ppre2().bits(apb2_psc_bits);
            w.sw().bits(sw_bits)
        });

        while self.rb.cfgr.read().sws().bits() != sw_bits {}

        // From RM:
        // The timer clock frequencies are automatically defined by hardware. There are two cases:
        // 1. If the APB prescaler equals 1, the timer clock frequencies are set to the same
        // frequency as that of the APB domain.
        // 2. Otherwise, they are set to twice (×2) the frequency of the APB domain.
        let timclk1 = match rcc_cfg.apb1_psc {
            Prescaler::NotDivided => apb1_freq,
            _ => apb1_freq * 2,
        };

        let timclk2 = match rcc_cfg.apb2_psc {
            Prescaler::NotDivided => apb2_freq,
            _ => apb2_freq * 2,
        };

        Rcc {
            rb: self.rb,
            clocks: Clocks {
                pll_clk,
                sysclk,
                core_clk: ahb_freq.Hz(),
                hclk: ahb_freq.Hz(),
                pclk1: apb1_freq.Hz(),
                timclk1: timclk1.Hz(),
                pclk2: apb2_freq.Hz(),
                timclk2: timclk2.Hz(),
            },
        }
    }

    pub fn unlock_rtc(&mut self) {
        self.rb.apb1enr1.modify(|_, w| w.pwren().set_bit());
        let pwr = unsafe { &(*PWR::ptr()) };
        pwr.cr1.modify(|_, w| w.dbp().set_bit());
    }

    fn config_pll(&self, pll_cfg: PllConfig) -> PLLClocks {
        // Disable PLL
        self.rb.cr.modify(|_, w| w.pllon().clear_bit());
        while self.rb.cr.read().pllrdy().bit_is_set() {}

        // Enable the input clock feeding the PLL
        let (pll_input_freq, pll_src_bits) = match pll_cfg.mux {
            PLLSrc::HSI => {
                self.enable_hsi();
                (HSI_FREQ, 0b10)
            }
            PLLSrc::HSE(freq) => {
                self.enable_hse(false);
                (freq.raw(), 0b11)
            }
            PLLSrc::HSE_BYPASS(freq) => {
                self.enable_hse(true);
                (freq.raw(), 0b11)
            }
        };

        // Calculate the frequency of the internal PLL VCO.
        let pll_freq = pll_input_freq / pll_cfg.m.divisor() * pll_cfg.n.multiplier();

        // Calculate the output frequencies for the P, Q, and R outputs
        let p = pll_cfg
            .p
            .map(|p| ((pll_freq / p.divisor()).Hz(), p.register_setting()));

        let q = pll_cfg
            .q
            .map(|q| ((pll_freq / q.divisor()).Hz(), q.register_setting()));

        let r = pll_cfg
            .r
            .map(|r| ((pll_freq / r.divisor()).Hz(), r.register_setting()));

        // Set the M input divider, the N multiplier for the PLL, and the PLL source.
        self.rb.pllcfgr.modify(|_, w| unsafe {
            // Set N, M, and source
            let w = w.plln().bits(pll_cfg.n.register_setting());
            w.pllm().bits(pll_cfg.m.register_setting());
            w.pllsrc().bits(pll_src_bits);

            // Set and enable P if requested
            let w = match p {
                Some((_, register_setting)) => {
                    w.pllpdiv().bits(register_setting).pllpen().set_bit()
                }
                None => w,
            };

            // Set and enable Q if requested
            let w = match q {
                Some((_, register_setting)) => w.pllq().bits(register_setting).pllqen().set_bit(),
                None => w,
            };

            // Set and enable R if requested
            let w = match r {
                Some((_, register_setting)) => w.pllr().bits(register_setting).pllren().set_bit(),
                None => w,
            };

            w
        });

        // Enable PLL
        self.rb.cr.modify(|_, w| w.pllon().set_bit());
        while self.rb.cr.read().pllrdy().bit_is_clear() {}

        PLLClocks {
            r: r.map(|r| r.0),
            q: q.map(|q| q.0),
            p: p.map(|p| p.0),
        }
    }

    pub(crate) fn enable_hsi(&self) {
        self.rb.cr.modify(|_, w| w.hsion().set_bit());
        while self.rb.cr.read().hsirdy().bit_is_clear() {}
    }

    pub(crate) fn enable_hse(&self, bypass: bool) {
        self.rb
            .cr
            .modify(|_, w| w.hseon().set_bit().hsebyp().bit(bypass));
        while self.rb.cr.read().hserdy().bit_is_clear() {}
    }

    pub(crate) fn enable_lse(&self, bypass: bool) {
        self.rb
            .bdcr
            .modify(|_, w| w.lseon().set_bit().lsebyp().bit(bypass));
        while self.rb.bdcr.read().lserdy().bit_is_clear() {}
    }

    pub(crate) fn enable_lsi(&self) {
        self.rb.csr.modify(|_, w| w.lsion().set_bit());
        while self.rb.csr.read().lsirdy().bit_is_clear() {}
    }
}

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;

    /// Constrains the `RCC` peripheral and apply clock configuration
    fn freeze(self, rcc_cfg: Config) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            rb: self,
            clocks: Clocks::default(),
        }
    }

    fn freeze(self, rcc_cfg: Config) -> Rcc {
        self.constrain().freeze(rcc_cfg)
    }
}

use crate::pac::rcc::RegisterBlock as RccRB;

pub struct AHB1 {
    _0: (),
}
impl AHB1 {
    #[inline(always)]
    fn enr(rcc: &RccRB) -> &rcc::AHB1ENR {
        &rcc.ahb1enr
    }
    #[inline(always)]
    fn rstr(rcc: &RccRB) -> &rcc::AHB1RSTR {
        &rcc.ahb1rstr
    }
    #[inline(always)]
    fn smenr(rcc: &RccRB) -> &rcc::AHB1SMENR {
        &rcc.ahb1smenr
    }
}

pub struct AHB2 {
    _0: (),
}
impl AHB2 {
    #[inline(always)]
    fn enr(rcc: &RccRB) -> &rcc::AHB2ENR {
        &rcc.ahb2enr
    }
    #[inline(always)]
    fn rstr(rcc: &RccRB) -> &rcc::AHB2RSTR {
        &rcc.ahb2rstr
    }
    #[inline(always)]
    fn smenr(rcc: &RccRB) -> &rcc::AHB2SMENR {
        &rcc.ahb2smenr
    }
}

pub struct AHB3 {
    _0: (),
}
impl AHB3 {
    #[inline(always)]
    fn enr(rcc: &RccRB) -> &rcc::AHB3ENR {
        &rcc.ahb3enr
    }
    #[inline(always)]
    fn rstr(rcc: &RccRB) -> &rcc::AHB3RSTR {
        &rcc.ahb3rstr
    }
    #[inline(always)]
    fn smenr(rcc: &RccRB) -> &rcc::AHB3SMENR {
        &rcc.ahb3smenr
    }
}

pub struct APB1_1 {
    _0: (),
}
impl APB1_1 {
    #[inline(always)]
    fn enr(rcc: &RccRB) -> &rcc::APB1ENR1 {
        &rcc.apb1enr1
    }
    #[inline(always)]
    fn rstr(rcc: &RccRB) -> &rcc::APB1RSTR1 {
        &rcc.apb1rstr1
    }
    #[inline(always)]
    fn smenr(rcc: &RccRB) -> &rcc::APB1SMENR1 {
        &rcc.apb1smenr1
    }
}

pub struct APB1_2 {
    _0: (),
}
impl APB1_2 {
    #[inline(always)]
    fn enr(rcc: &RccRB) -> &rcc::APB1ENR2 {
        &rcc.apb1enr2
    }
    #[inline(always)]
    fn rstr(rcc: &RccRB) -> &rcc::APB1RSTR2 {
        &rcc.apb1rstr2
    }
    #[inline(always)]
    fn smenr(rcc: &RccRB) -> &rcc::APB1SMENR2 {
        &rcc.apb1smenr2
    }
}

pub struct APB2 {
    _0: (),
}
impl APB2 {
    #[inline(always)]
    fn enr(rcc: &RccRB) -> &rcc::APB2ENR {
        &rcc.apb2enr
    }
    #[inline(always)]
    fn rstr(rcc: &RccRB) -> &rcc::APB2RSTR {
        &rcc.apb2rstr
    }
    #[inline(always)]
    fn smenr(rcc: &RccRB) -> &rcc::APB2SMENR {
        &rcc.apb2smenr
    }
}

/// Bus associated to peripheral
pub trait RccBus: crate::Sealed {
    /// Bus type;
    type Bus;
}

/// Enable/disable peripheral
pub trait Enable: RccBus {
    fn enable(rcc: &RccRB);
    fn disable(rcc: &RccRB);

    /// # Safety
    ///
    /// Enables peripheral. Takes access to RCC internally
    unsafe fn enable_unchecked() {
        let rcc = &*pac::RCC::ptr();
        Self::enable(rcc);
    }

    /// # Safety
    ///
    /// Disables peripheral. Takes access to RCC internally
    unsafe fn disable_unchecked() {
        let rcc = &*pac::RCC::ptr();
        Self::disable(rcc);
    }
}

/// Reset peripheral
pub trait Reset: RccBus {
    fn reset(rcc: &RccRB);

    /// # Safety
    ///
    /// Resets peripheral. Takes access to RCC internally
    unsafe fn reset_unchecked() {
        let rcc = pac::RCC::ptr();
        Self::reset(&*rcc);
    }
}

/// Frequency on bus that peripheral is connected in
pub trait BusClock {
    /// Calculates frequency depending on `Clock` state
    fn clock(clocks: &Clocks) -> Hertz;
}

/// Frequency on bus that timer is connected in
pub trait BusTimerClock {
    /// Calculates base frequency of timer depending on `Clock` state
    fn timer_clock(clocks: &Clocks) -> Hertz;
}

impl<T> BusClock for T
where
    T: RccBus,
    T::Bus: BusClock,
{
    fn clock(clocks: &Clocks) -> Hertz {
        T::Bus::clock(clocks)
    }
}

impl<T> BusTimerClock for T
where
    T: RccBus,
    T::Bus: BusTimerClock,
{
    fn timer_clock(clocks: &Clocks) -> Hertz {
        T::Bus::timer_clock(clocks)
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

impl BusClock for APB1_1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk1
    }
}
impl BusTimerClock for APB1_1 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.timclk1
    }
}

impl BusClock for APB1_2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk1
    }
}
impl BusTimerClock for APB1_2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.timclk1
    }
}

impl BusClock for APB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk2
    }
}
impl BusTimerClock for APB2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.timclk2
    }
}
