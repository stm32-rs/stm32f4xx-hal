//! Clock configuration.
//!
//! This module provides functionality to configure the RCC to generate the requested clocks.
//!
//! # Example
//!
//! ```
//! let dp = pac::Peripherals::take().unwrap();
//! let rcc = dp.RCC.constrain();
//! let clocks = rcc
//!     .cfgr
//!     .use_hse(8.MHz())
//!     .sysclk(168.MHz())
//!     .pclk1(24.MHz())
//!     .i2s_clk(86.MHz())
//!     .require_pll48clk()
//!     .freeze();
//!     // Test that the I2S clock is suitable for 48000kHz audio.
//!     assert!(clocks.i2s_clk().unwrap() == 48.MHz().into());
//! ```
//!
//! # Limitations
//!
//! Unlike the clock configuration tool provided by ST, the code does not extensively search all
//! possible configurations. Instead, it often relies on an iterative approach to reduce
//! computational complexity. On most MCUs the code will first generate a configuration for the 48
//! MHz clock and the system clock without taking other requested clocks into account, even if the
//! accuracy of these clocks is affected. **If you specific accuracy requirements, you should
//! always check the resulting frequencies!**
//!
//! Whereas the hardware often supports flexible clock source selection and many clocks can be
//! sourced from multiple PLLs, the code implements a fixed mapping between PLLs and clocks. The 48
//! MHz clock is always generated by the main PLL, the I2S clocks are always generated by the I2S
//! PLL (unless a matching external clock input is provided), and similarly the SAI clocks are
//! always generated by the SAI PLL. It is therefore not possible to, for example, specify two
//! different I2S frequencies unless you also provide a matching I2S_CKIN signal for one of them.
//!
//! Some MCUs have limited clock generation hardware and do not provide either I2S or SAI PLLs even
//! though I2S or SAI are available. On the STM32F410, the I2S clock is generated by the main PLL,
//! and on the STM32F413/423 SAI clocks are generated by the I2S PLL. On these MCUs, the actual
//! frequencies may substantially deviate from the requested frequencies.
#[cfg(feature = "f2")]
mod f2;
#[cfg(feature = "f2")]
pub use f2::*;

#[cfg(feature = "f4")]
mod f4;
#[cfg(feature = "f4")]
pub use f4::*;

#[cfg(feature = "f7")]
mod f7;
#[cfg(feature = "f7")]
pub use f7::*;

use fugit::HertzU32 as Hertz;

/// Bus associated to peripheral
pub trait RccBus: crate::Sealed {
    /// Bus type;
    type Bus;
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
