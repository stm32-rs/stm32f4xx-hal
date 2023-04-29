//! # Reset and Clock Control
//!
//! The most important function this module
//! delivers is the clock configuration.
//!
//! To configure the clock, we first have to obtain the
//! device peripherals.
//!
//! ```
//! # use cortex_m_rt::entry;
//! # use stm32f3xx-hal::{prelude::*, time::rate::*};
//!
//! # #[entry]
//! # fn main() -> ! {
//! // Get our peripherals
//! let dp = pac::Peripherals::take().unwrap();
//!
//! let mut flash = dp.FLASH.constrain();
//! let mut rcc = dp.RCC.constrain();
//! # }
//! ```
//!
//! After that we can configure the clock
//!
//! ```
//! # use cortex_m_rt::entry;
//! # use stm32f3xx-hal::{prelude::*, time::rate::*};
//! #
//! # #[entry]
//! # fn main() -> ! {
//! # let dp = pac::Peripherals::take().unwrap();
//!
//! # let mut flash = dp.FLASH.constrain();
//! # let mut rcc = dp.RCC.constrain();
//! let clocks = rcc.cfgr
//!     // Using the external oscillator
//!     // Set the frequency to that of the external oscillator
//!     .use_hse(8.MHz())
//!     // Set the frequency for the AHB bus,
//!     // which the root of every following clock peripheral
//!     .hclk(48.MHz())
//!     // The sysclk is equivalent to the core clock
//!     .sysclk(48.MHz())
//!     // The following are peripheral clocks, which are both
//!     // needed to configure specific peripherals.
//!     // Looking at the peripheral function parameters
//!     // should give more insight, which peripheral clock is needed.
//!     .pclk1(12.MHz())
//!     .pclk2(12.MHz())
//!     // Freeze / apply the configuration and setup all clocks
//!     .freeze(&mut flash.acr);
//! # }
//! ```
//!
//! All fields can be omitted and will internally be set to a calculated default.
//! For more details read the documentation of the [`CFGR`] methods to
//! find out how to setup the clock.

use crate::pac::{
    rcc::{self, cfgr, cfgr2},
    RCC,
};

use fugit::HertzU32 as Hertz;
use fugit::RateExtU32;

use core::convert::TryInto;

use crate::flash::ACR;

impl crate::Sealed for RCC {}

/// Extension trait that constrains the []`RCC`] peripheral
pub trait RccExt: crate::Sealed {
    /// Constrains the [`RCC`] peripheral.
    ///
    /// Consumes the [`pac::RCC`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::RCC`]: `crate::pac::RCC`
    /// [`HAL`]: `crate`
    fn constrain(self) -> Rcc;
}

mod enable;

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb: AHB { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            bdcr: BDCR { _0: () },
            cfgr: CFGR::default(),
        }
    }
}

/// Constrained RCC peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](RccExt::constrain) function
/// on the [`RCC`](crate::pac::RCC) struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// ```
pub struct Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub ahb: AHB,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    /// RCC Backup Domain
    pub bdcr: BDCR,
    /// Clock configuration
    pub cfgr: CFGR,
}

/// AMBA High-performance Bus (AHB) registers
///
/// An instance of this struct is acquired from the [`RCC`](crate::pac::RCC) struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// use_ahb(&mut rcc.ahb)
/// ```
pub struct AHB {
    _0: (),
}

/// Advanced Peripheral Bus 1 (APB1) registers
///
/// An instance of this struct is acquired from the [`RCC`](crate::pac::RCC) struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// use_apb1(&mut rcc.apb1)
/// ```
pub struct APB1 {
    _0: (),
}

/// Advanced Peripheral Bus 2 (APB2) registers
///
/// An instance of this struct is acquired from the [`RCC`](crate::pac::RCC) struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// use_apb2(&mut rcc.apb2)
/// ```
pub struct APB2 {
    _0: (),
}

macro_rules! bus_struct {
    ($($busX:ident => ($EN:ident, $en:ident, $RST:ident, $rst:ident),)+) => {
        $(
            impl $busX {
                fn new() -> Self {
                    Self { _0: () }
                }

                #[allow(unused)]
                fn enr(&self) -> &rcc::$EN {
                    // NOTE(unsafe) this proxy grants exclusive access to this register
                    unsafe { &(*RCC::ptr()).$en }
                }

                #[allow(unused)]
                fn rstr(&self) -> &rcc::$RST {
                    // NOTE(unsafe) this proxy grants exclusive access to this register
                    unsafe { &(*RCC::ptr()).$rst }
                }
            }
        )+
    };
}

bus_struct! {
    AHB => (AHBENR, ahbenr, AHBRSTR, ahbrstr),
    APB1 => (APB1ENR, apb1enr, APB1RSTR, apb1rstr),
    APB2 => (APB2ENR, apb2enr, APB2RSTR, apb2rstr),
}

/// Bus associated to peripheral
pub trait RccBus: crate::Sealed {
    /// The underlying bus peripheral
    type Bus;
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
    fn is_disabled() -> bool;

    /// Enables peripheral
    ///
    /// # Safety
    ///
    /// Takes access to RCC internally, so you have to make sure
    /// you don't have race condition accessing RCC registers
    unsafe fn enable_unchecked();

    /// Disables peripheral
    ///
    /// # Safety
    ///
    /// Takes access to RCC internally, so you have to make sure
    /// you don't have race condition accessing RCC registers
    unsafe fn disable_unchecked();
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

/// Frequency on bus that peripheral is connected in
pub trait BusClock {
    /// Calculates frequency depending on `Clock` state
    fn clock(clocks: &Clocks) -> Hertz;
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

impl BusClock for AHB {
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

/// Frequency on bus that timer is connected in
pub trait BusTimerClock {
    /// Calculates base frequency of timer depending on `Clock` state
    fn timer_clock(clocks: &Clocks) -> Hertz;
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

impl BusTimerClock for APB1 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        let pclk_mul = if clocks.ppre1 > 1 { 2 } else { 1 };
        clocks.pclk1 * pclk_mul
    }
}
impl BusTimerClock for APB2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        let pclk_mul = if clocks.ppre2 > 1 { 2 } else { 1 };
        clocks.pclk2 * pclk_mul
    }
}

/// Frequency of interal hardware RC oscillator (HSI OSC)
pub const HSI: Hertz = Hertz::Hz(8_000_000);
/// Frequency of external 32.768 kHz oscillator (LSE OSC)
pub const LSE: Hertz = Hertz::Hz(32_768);

// some microcontrollers do not have USB
#[cfg(any(feature = "stm32f301", feature = "stm32f318", feature = "stm32f334",))]
mod usb_clocking {
    use crate::rcc::PllConfig;

    pub(crate) fn is_valid(
        _sysclk: u32,
        _hse: Option<u32>,
        _pclk1: u32,
        _pll_config: &Option<PllConfig>,
    ) -> (bool, bool) {
        (false, false)
    }

    pub(crate) fn set_usbpre<W>(w: &mut W, _: bool) -> &mut W {
        w
    }
}

#[cfg(not(any(feature = "stm32f301", feature = "stm32f318", feature = "stm32f334",)))]
mod usb_clocking {
    use crate::pac::rcc::cfgr;
    use crate::rcc::PllConfig;

    /// Check for all clock options to be
    pub(crate) fn is_valid(
        sysclk: u32,
        hse: Option<u32>,
        pclk1: u32,
        pll_config: &Option<PllConfig>,
    ) -> (cfgr::USBPRE_A, bool) {
        // the USB clock is only valid if an external crystal is used, the PLL is enabled, and the
        // PLL output frequency is a supported one.
        // usbpre == false: divide clock by 1.5, otherwise no division
        let usb_ok = hse.is_some() && pll_config.is_some();
        // The APB1 clock must have a minimum frequency of 10 MHz to avoid data overrun/underrun
        // problems. [RM0316 32.5.2]
        if pclk1 >= 10_000_000 {
            match (usb_ok, sysclk) {
                (true, 72_000_000) => (cfgr::USBPRE_A::Div15, true),
                (true, 48_000_000) => (cfgr::USBPRE_A::Div1, true),
                _ => (cfgr::USBPRE_A::Div1, false),
            }
        } else {
            (cfgr::USBPRE_A::Div1, false)
        }
    }

    pub(crate) fn set_usbpre(w: &mut cfgr::W, usb_prescale: cfgr::USBPRE_A) -> &mut cfgr::W {
        w.usbpre().variant(usb_prescale)
    }
}

/// Backup Domain Control register (RCC_BDCR)
pub struct BDCR {
    _0: (),
}

impl BDCR {
    #[allow(unused)]
    pub(crate) fn bdcr(&mut self) -> &rcc::BDCR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).bdcr }
    }
}

/// Clock configuration
///
/// An instance of this struct is acquired from the [`RCC`](crate::pac::RCC) struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// use_cfgr(&mut rcc.cfgr)
/// ```
pub struct CFGR {
    hse: Option<u32>,
    hse_bypass: bool,
    pll_bypass: bool,
    css: bool,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

impl Default for CFGR {
    fn default() -> Self {
        Self {
            hse: None,
            hse_bypass: false,
            pll_bypass: true,
            css: false,
            hclk: None,
            pclk1: None,
            pclk2: None,
            sysclk: None,
        }
    }
}

pub(crate) struct PllConfig {
    src: cfgr::PLLSRC_A,
    mul: cfgr::PLLMUL_A,
    div: Option<cfgr2::PREDIV_A>,
}

/// Determine the [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
///
/// This function is based on the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
// TODO(Sh3Rm4n): As num-traits is a indirecty dependecy of this crate through embedded-time,
// use its implementation instead.
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Convert pll multiplier into equivalent register field type
fn into_pll_mul(mul: u8) -> cfgr::PLLMUL_A {
    match mul {
        2 => cfgr::PLLMUL_A::Mul2,
        3 => cfgr::PLLMUL_A::Mul3,
        4 => cfgr::PLLMUL_A::Mul4,
        5 => cfgr::PLLMUL_A::Mul5,
        6 => cfgr::PLLMUL_A::Mul6,
        7 => cfgr::PLLMUL_A::Mul7,
        8 => cfgr::PLLMUL_A::Mul8,
        9 => cfgr::PLLMUL_A::Mul9,
        10 => cfgr::PLLMUL_A::Mul10,
        11 => cfgr::PLLMUL_A::Mul11,
        12 => cfgr::PLLMUL_A::Mul12,
        13 => cfgr::PLLMUL_A::Mul13,
        14 => cfgr::PLLMUL_A::Mul14,
        15 => cfgr::PLLMUL_A::Mul15,
        16 => cfgr::PLLMUL_A::Mul16,
        _ => crate::unreachable!(),
    }
}

/// Convert pll divisor into equivalent register field type
fn into_pre_div(div: u8) -> cfgr2::PREDIV_A {
    match div {
        1 => cfgr2::PREDIV_A::Div1,
        2 => cfgr2::PREDIV_A::Div2,
        3 => cfgr2::PREDIV_A::Div3,
        4 => cfgr2::PREDIV_A::Div4,
        5 => cfgr2::PREDIV_A::Div5,
        6 => cfgr2::PREDIV_A::Div6,
        7 => cfgr2::PREDIV_A::Div7,
        8 => cfgr2::PREDIV_A::Div8,
        9 => cfgr2::PREDIV_A::Div9,
        10 => cfgr2::PREDIV_A::Div10,
        11 => cfgr2::PREDIV_A::Div11,
        12 => cfgr2::PREDIV_A::Div12,
        13 => cfgr2::PREDIV_A::Div13,
        14 => cfgr2::PREDIV_A::Div14,
        15 => cfgr2::PREDIV_A::Div15,
        16 => cfgr2::PREDIV_A::Div16,
        _ => crate::unreachable!(),
    }
}

impl CFGR {
    /// Uses `HSE` (external oscillator) instead of `HSI` (internal RC oscillator) as the clock source.
    pub fn use_hse(mut self, freq: Hertz) -> Self {
        self.hse = Some(freq.raw());
        self
    }

    /// Set this to disallow bypass the PLLCLK for the systemclock generation.
    pub fn use_pll(mut self) -> Self {
        self.pll_bypass = false;
        self
    }

    /// Enable `HSE` bypass.
    ///
    /// Uses user provided clock signal instead of an external oscillator.
    /// `OSC_OUT` pin is free and can be used as GPIO.
    ///
    /// No effect if `HSE` is not enabled.
    pub fn bypass_hse(mut self) -> Self {
        self.hse_bypass = true;
        self
    }

    /// Enable `CSS` (Clock Security System).
    ///
    /// System clock is automatically switched to `HSI` and an interrupt (`CSSI`) is generated
    /// when `HSE` clock failure is detected.
    ///
    /// No effect if `HSE` is not enabled.
    pub fn enable_css(mut self) -> Self {
        self.css = true;
        self
    }

    /// Sets a frequency for the AHB bus.
    pub fn hclk(mut self, freq: Hertz) -> Self {
        self.hclk = Some(freq.raw());
        self
    }

    /// Sets a frequency for the `APB1` bus
    ///
    /// - Maximal supported frequency: 36 Mhz
    ///
    /// If not manually set, it will be set to [`CFGR::sysclk`] frequency
    /// or [`CFGR::sysclk`] frequency / 2, if [`CFGR::sysclk`] > 36 Mhz
    pub fn pclk1(mut self, freq: Hertz) -> Self {
        self.pclk1 = Some(freq.raw());
        self
    }

    /// Sets a frequency for the `APB2` bus
    ///
    /// # Resolution and Limits
    ///
    /// - Maximal supported frequency with HSE: 72 Mhz
    /// - Maximal supported frequency without HSE: 64 Mhz
    pub fn pclk2(mut self, freq: Hertz) -> Self {
        self.pclk2 = Some(freq.raw());
        self
    }

    /// Sets the system (core) frequency
    ///
    /// # Resolution and Limits
    ///
    /// - Maximal supported frequency with `HSE`: 72 Mhz
    /// - Maximal supported frequency without `HSE`: 64 Mhz
    ///
    /// If [`CFGR::use_hse`] is not set, `HSI / 2` will be used.
    /// Only multiples of (HSI / 2) (4 Mhz) are allowed.
    ///
    /// This is true for devices **except** the following devices,
    /// as these allow finer resolutions
    /// even when using the internal oscillator:
    ///
    ///     [stm32f302xd,stm32f302xe,stm32f303xd,stm32f303xe,stm32f398]
    pub fn sysclk(mut self, freq: Hertz) -> Self {
        self.sysclk = Some(freq.raw());
        self
    }

    /// Calculate the values for the pll multiplier (`PLLMUL`) and the pll divisior (`PLLDIV`).
    ///
    /// These values are chosen depending on the chosen system clock (SYSCLK) and the frequency of the
    /// oscillator clock (`HSE` / `HSI`).
    ///
    /// For these devices, `PLL_SRC` can selected between the internal oscillator (`HSI`) and
    /// the external oscillator (`HSE`).
    ///
    /// HSI is divided by 2 before its transferred to `PLL_SRC`.
    /// HSE can be divided between `1..16`, before it is transferred to `PLL_SRC`.
    /// After this system clock frequency (`SYSCLK`) can be changed via multiplier.
    /// The value can be multiplied with `2..16`.
    ///
    /// To determine the optimal values, if `HSE` is chosen as `PLL_SRC`, the greatest common divisor
    /// is calculated and the limitations of the possible values are taken into consideration.
    ///
    /// `HSI` is simpler to calculate, but the possible system clocks are less than `HSE`, because the
    /// division is not configurable.
    #[cfg(not(feature = "gpio-f303e"))]
    fn calc_pll(&self, sysclk: u32) -> (u32, PllConfig) {
        let pllsrcclk = self.hse.unwrap_or(HSI.to_Hz() / 2);
        // Get the optimal value for the pll divisor (PLL_DIV) and multiplier (PLL_MUL)
        // Only for HSE PLL_DIV can be changed
        let (pll_mul, pll_div): (u32, Option<u32>) = if self.hse.is_some() {
            // Get the optimal value for the pll divisor (PLL_DIV) and multiplier (PLL_MUL)
            // with the greatest common divisor calculation.
            let common_divisor = gcd(sysclk, pllsrcclk);
            let mut multiplier = sysclk / common_divisor;
            let mut divisor = pllsrcclk / common_divisor;

            // Check if the multiplier can be represented by PLL_MUL
            if multiplier == 1 {
                // PLL_MUL minimal value is 2
                multiplier *= 2;
                divisor *= 2;
            }

            // PLL_MUL maximal value is 16
            crate::assert!(multiplier <= 16);

            // PRE_DIV maximal value is 16
            crate::assert!(divisor <= 16);

            (multiplier, Some(divisor))
        }
        // HSI division is always divided by 2 and has no adjustable division
        else {
            let pll_mul = sysclk / pllsrcclk;
            crate::assert!(pll_mul <= 16);
            (pll_mul, None)
        };

        let sysclk = (pllsrcclk / pll_div.unwrap_or(1)) * pll_mul;
        crate::assert!(sysclk <= 72_000_000);

        let pll_src = if self.hse.is_some() {
            cfgr::PLLSRC_A::HseDivPrediv
        } else {
            cfgr::PLLSRC_A::HsiDiv2
        };

        // Convert into register bit field types
        let pll_mul_bits = into_pll_mul(pll_mul as u8);
        let pll_div_bits = pll_div.map(|pll_div| into_pre_div(pll_div as u8));

        (
            sysclk,
            PllConfig {
                src: pll_src,
                mul: pll_mul_bits,
                div: pll_div_bits,
            },
        )
    }

    /// Calculate the values for the pll multiplier (`PLLMUL`) and the pll divisor (`PLLDIV`).
    ///
    /// These values are chosen depending on the chosen system clock (`SYSCLK`) and the frequency of the oscillator
    /// clk (`HSI` / `HSE`).
    ///
    /// For these devices, `PLL_SRC` can be set to choose between the internal oscillator (HSI) and
    /// the external oscillator (`HSE`).
    /// After this the system clock frequency (`SYSCLK`) can be changed via a division and a
    /// multiplication block.
    /// It can be divided from with values `1..16`  and multiplied from `2..16`.
    ///
    /// To determine the optimal values, the greatest common divisor is calculated and the
    /// limitations of the possible values are taken into considiration.
    #[cfg(feature = "gpio-f303e")]
    fn calc_pll(&self, sysclk: u32) -> (u32, PllConfig) {
        let pllsrcclk = self.hse.unwrap_or(HSI.integer());

        let (pll_mul, pll_div) = {
            // Get the optimal value for the pll divisor (PLL_DIV) and multiplcator (PLL_MUL)
            // with the greatest common divisor calculation.
            let common_divisor = gcd(sysclk, pllsrcclk);
            let mut multiplier = sysclk / common_divisor;
            let mut divisor = pllsrcclk / common_divisor;

            // Check if the multiplier can be represented by PLL_MUL
            if multiplier == 1 {
                // PLL_MUL minimal value is 2
                multiplier *= 2;
                divisor *= 2;
            }

            // PLL_MUL maximal value is 16
            crate::assert!(multiplier <= 16);

            // PRE_DIV maximal value is 16
            crate::assert!(divisor <= 16);

            (multiplier, divisor)
        };

        let sysclk = (pllsrcclk / pll_div) * pll_mul;
        crate::assert!(sysclk <= 72_000_000);

        // Select hardware clock source of the PLL
        // TODO Check whether HSI_DIV2 could be useful
        let pll_src = if self.hse.is_some() {
            cfgr::PLLSRC_A::HseDivPrediv
        } else {
            cfgr::PLLSRC_A::HseDivPrediv
        };

        // Convert into register bit field types
        let pll_mul_bits = into_pll_mul(pll_mul as u8);
        let pll_div_bits = into_pre_div(pll_div as u8);

        (
            sysclk,
            PllConfig {
                src: pll_src,
                mul: pll_mul_bits,
                div: Some(pll_div_bits),
            },
        )
    }

    /// Get the system clock, the system clock source and the pll_options, if needed.
    ///
    /// The system clock source is determined by the chosen system clock and the provided hardware
    /// clock.
    /// This function does only chose the PLL if needed, otherwise it will use the oscillator clock as system clock.
    ///
    /// Calls [`CFGR::calc_pll`] internally.
    fn get_sysclk(&self) -> (u32, cfgr::SW_A, Option<PllConfig>) {
        // If a sysclk is given, check if the PLL has to be used,
        // else select the system clock source, which is either HSI or HSE.
        match (self.sysclk, self.hse) {
            // No need to use the PLL
            // PLL is needed for USB, but we can make this assumption, to not use PLL here,
            // because the two valid USB clocks, 72 Mhz and 48 Mhz, can't be generated
            // directly from neither the internal rc (8 Mhz)  nor the external
            // Oscillator (max 32 Mhz), without using the PLL.
            (Some(sysclk), Some(hse)) if sysclk == hse && self.pll_bypass => {
                (hse, cfgr::SW_A::Hse, None)
            }
            // No need to use the PLL
            (Some(sysclk), None) if sysclk == HSI.to_Hz() && self.pll_bypass => {
                (HSI.to_Hz(), cfgr::SW_A::Hsi, None)
            }
            (Some(sysclk), _) => {
                let (sysclk, pll_config) = self.calc_pll(sysclk);
                (sysclk, cfgr::SW_A::Pll, Some(pll_config))
            }
            // Use HSE as system clock
            (None, Some(hse)) => (hse, cfgr::SW_A::Hse, None),
            // Use HSI as system clock
            (None, None) => (HSI.to_Hz(), cfgr::SW_A::Hsi, None),
        }
    }

    /// Freezes the clock configuration, making it effective
    ///
    /// This function internally calculates the specific.
    /// divisors for the different clock peripheries.
    ///
    /// # Panics
    ///
    /// If any of the set frequencies via [`sysclk`](CFGR::sysclk), [`hclk`](CFGR::hclk), [`pclk1`](CFGR::pclk1) or [`pclk2`](CFGR::pclk2)
    /// are invalid or can not be reached because of e.g. to low frequencies
    /// of the former, as [`sysclk`](CFGR::sysclk) depends on the configuration of [`hclk`](CFGR::hclk)
    /// this function will panic.
    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        let (sysclk, sysclk_source, pll_config) = self.get_sysclk();

        let (hpre_bits, hpre) =
            self.hclk
                .map_or((cfgr::HPRE_A::Div1, 1), |hclk| match sysclk / hclk {
                    0 => crate::unreachable!(),
                    1 => (cfgr::HPRE_A::Div1, 1),
                    2 => (cfgr::HPRE_A::Div2, 2),
                    3..=5 => (cfgr::HPRE_A::Div4, 4),
                    6..=11 => (cfgr::HPRE_A::Div8, 8),
                    12..=39 => (cfgr::HPRE_A::Div16, 16),
                    40..=95 => (cfgr::HPRE_A::Div64, 64),
                    96..=191 => (cfgr::HPRE_A::Div128, 128),
                    192..=383 => (cfgr::HPRE_A::Div256, 256),
                    _ => (cfgr::HPRE_A::Div512, 512),
                });

        let hclk: u32 = sysclk / hpre;

        crate::assert!(hclk <= 72_000_000);

        let (mut ppre1_bits, mut ppre1) =
            self.pclk1
                .map_or((cfgr::PPRE1_A::Div1, 1), |pclk1| match hclk / pclk1 {
                    0 => crate::unreachable!(),
                    1 => (cfgr::PPRE1_A::Div1, 1),
                    2 => (cfgr::PPRE1_A::Div2, 2),
                    3..=5 => (cfgr::PPRE1_A::Div4, 4),
                    6..=11 => (cfgr::PPRE1_A::Div8, 8),
                    _ => (cfgr::PPRE1_A::Div16, 16),
                });

        let mut pclk1 = hclk / u32::from(ppre1);

        // This ensures, that no panic happens, when
        // pclk1 is not manually set.
        // As hclk highest value is 72.MHz()
        // dividing by 2 should always be sufficient
        if self.pclk1.is_none() && pclk1 > 36_000_000 {
            ppre1_bits = cfgr::PPRE1_A::Div2;
            ppre1 = 2;
            pclk1 = hclk / u32::from(ppre1);
        }

        crate::assert!(pclk1 <= 36_000_000);

        let (ppre2_bits, ppre2) =
            self.pclk2
                .map_or((cfgr::PPRE2_A::Div1, 1), |pclk2| match hclk / pclk2 {
                    0 => crate::unreachable!(),
                    1 => (cfgr::PPRE2_A::Div1, 1),
                    2 => (cfgr::PPRE2_A::Div2, 2),
                    3..=5 => (cfgr::PPRE2_A::Div4, 4),
                    6..=11 => (cfgr::PPRE2_A::Div8, 8),
                    _ => (cfgr::PPRE2_A::Div16, 16),
                });

        let pclk2 = hclk / u32::from(ppre2);

        crate::assert!(pclk2 <= 72_000_000);

        // Adjust flash wait states according to the
        // HCLK frequency (cpu core clock)
        acr.acr().modify(|_, w| {
            if hclk <= 24_000_000 {
                w.latency().ws0()
            } else if hclk <= 48_000_000 {
                w.latency().ws1()
            } else {
                w.latency().ws2()
            }
        });

        let (usbpre, usbclk_valid) = usb_clocking::is_valid(sysclk, self.hse, pclk1, &pll_config);

        let rcc = unsafe { &*RCC::ptr() };

        // enable HSE and wait for it to be ready
        if self.hse.is_some() {
            rcc.cr.modify(|_, w| {
                w.hsebyp().bit(self.hse_bypass);
                w.csson().bit(self.css);
                w.hseon().on()
            });

            while rcc.cr.read().hserdy().is_not_ready() {}
        }

        // enable PLL and wait for it to be ready
        if let Some(pll_config) = pll_config {
            rcc.cfgr.modify(|_, w| {
                w.pllmul()
                    .variant(pll_config.mul)
                    .pllsrc()
                    .variant(pll_config.src)
            });

            if let Some(pll_div) = pll_config.div {
                rcc.cfgr2.modify(|_, w| w.prediv().variant(pll_div));
            };

            rcc.cr.modify(|_, w| w.pllon().on());

            while rcc.cr.read().pllrdy().is_not_ready() {}
        };

        // set prescalers and clock source
        rcc.cfgr.modify(|_, w| {
            usb_clocking::set_usbpre(w, usbpre);

            w.ppre2()
                .variant(ppre2_bits)
                .ppre1()
                .variant(ppre1_bits)
                .hpre()
                .variant(hpre_bits)
                .sw()
                .variant(sysclk_source)
        });

        Clocks {
            hclk: hclk.Hz(),
            pclk1: pclk1.Hz(),
            pclk2: pclk2.Hz(),
            ppre1,
            ppre2,
            sysclk: sysclk.Hz(),
            usbclk_valid,
            pll_bypass: self.pll_bypass,
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed.
/// This struct can be obtained via the [freeze](CFGR::freeze) method of the [CFGR](CFGR) struct.
#[derive(Debug, Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
    usbclk_valid: bool,
    pll_bypass: bool,
}

// TODO(Sh3Rm4n) Add defmt support for embedded-time!
#[cfg(feature = "defmt")]
impl defmt::Format for Clocks {
    fn format(&self, f: defmt::Formatter) {
        // Format as hexadecimal.
        defmt::write!(
            f,
            "Clocks {{ hclk: {} Hz, pclk1: {} Hz, pclk2: {} Hz, ppre1: {:b}, ppre2: {:b}, sysclk: {} Hz, usbclk_valid: {}, pll_bypass: {} }}",
            self.hclk.integer(),
            self.pclk1.integer(),
            self.pclk2.integer(),
            self.ppre1,
            self.ppre2,
            self.sysclk.integer(),
            self.usbclk_valid,
            self.pll_bypass,
        );
    }
}

// TODO(Sh3Rm4n): Think of some way to generlize APB1 and APB2 as types to then implement a method,
// with which the ppre or pclk can be obtained by passing in the type of APB.
// With that in place, some places of macro magic are not needed anymore.
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

    /// Returns the PLL clock if configured, else it returns `None`.
    ///
    /// The PLL clock is a source of the system clock, but it is not necessarily configured to be one.
    pub fn pllclk(&self) -> Option<Hertz> {
        if self.pll_bypass {
            None
        } else {
            // The PLLCLK is the same as the sysclk, beccause
            // the sysclk is using it as a source.
            Some(self.sysclk())
        }
    }

    /// Returns whether the USBCLK clock frequency is valid for the USB peripheral
    ///
    /// If the microcontroller does support USB, 48 Mhz or 72 Mhz have to be used
    /// and the [`CFGR::use_hse`] must be set.
    ///
    /// The APB1 / [`CFGR::pclk1`] clock must have a minimum frequency of 10 MHz to avoid data
    /// overrun/underrun problems. [RM0316 32.5.2][RM0316]
    ///
    /// [RM0316]: https://www.st.com/resource/en/reference_manual/dm00043574.pdf
    pub fn usbclk_valid(&self) -> bool {
        self.usbclk_valid
    }
}
