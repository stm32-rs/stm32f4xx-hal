//! I2S (inter-IC Sound) communication using SPI peripherals
//!
//! This module is only available if the `i2s` feature is enabled.

use crate::gpio::{self, NoPin};
use crate::pac::{self, RCC};
use crate::rcc;
use crate::rcc::Clocks;
use fugit::HertzU32 as Hertz;

#[cfg(feature = "stm32_i2s_v12x")]
pub extern crate stm32_i2s_v12x;

// I2S pins are mostly the same as the corresponding SPI pins:
// MOSI -> SD
// NSS -> WS (the current SPI code doesn't define NSS pins)
// SCK -> CK
// The master clock output is separate.

/// A placeholder for when the MCLK pin is not needed
pub type NoMasterClock = NoPin;

/// Trait for SPI peripheral with i2s capability.
pub trait Instance:
    I2sFreq + rcc::Enable + rcc::Reset + gpio::alt::I2sCommon + gpio::alt::I2sMaster
{
}

/// Trait to get I2s frequency at SPI peripheral input.
pub trait I2sFreq {
    fn i2s_freq(clocks: &Clocks) -> Hertz;
}

/// Trait to build an [`I2s`] object from SPI peripheral, pins and clocks
pub trait I2sExt: Sized + Instance {
    fn i2s(
        self,
        pins: (
            impl Into<Self::Ws>,
            impl Into<Self::Ck>,
            impl Into<Self::Mck>,
            impl Into<Self::Sd>,
        ),
        clocks: &Clocks,
    ) -> I2s<Self>;
}

impl<SPI: Instance> I2sExt for SPI {
    fn i2s(
        self,
        pins: (
            impl Into<Self::Ws>,
            impl Into<Self::Ck>,
            impl Into<Self::Mck>,
            impl Into<Self::Sd>,
        ),
        clocks: &Clocks,
    ) -> I2s<Self> {
        I2s::new(self, pins, clocks)
    }
}

/// An I2s wrapper around an SPI object and pins
pub struct I2s<I: Instance> {
    spi: I,
    pins: (I::Ws, I::Ck, I::Mck, I::Sd),
    /// Frequency of clock input to this peripheral from the I2S PLL or related source
    input_clock: Hertz,
}

// Note: for API documenting reason, it's better to keep `(WS, CK, MCLK, SD)` for ctor and dtor
// than replacing by `PINS`
impl<SPI: Instance> I2s<SPI> {
    /// Creates an I2s object around an SPI peripheral and pins
    ///
    /// This function enables and resets the SPI peripheral, but does not configure it.
    ///
    /// The returned I2s object implements `stm32_i2s_v12x::I2sPeripheral`, so it can be used to
    /// configure the peripheral and communicate.
    ///
    /// # Panics
    ///
    /// This function panics if the I2S clock input (from the I2S PLL or similar)
    /// is not configured.
    pub fn new(
        spi: SPI,
        pins: (
            impl Into<SPI::Ws>,
            impl Into<SPI::Ck>,
            impl Into<SPI::Mck>,
            impl Into<SPI::Sd>,
        ),
        clocks: &Clocks,
    ) -> Self {
        let input_clock = SPI::i2s_freq(clocks);
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            // Enable clock, enable reset, clear, reset
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into(), pins.3.into());

        I2s {
            spi,
            pins,
            input_clock,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (SPI, (SPI::Ws, SPI::Ck, SPI::Mck, SPI::Sd)) {
        (self.spi, self.pins)
    }
}

impl<SPI: Instance> I2s<SPI> {
    pub fn ws_pin(&self) -> &SPI::Ws {
        &self.pins.0
    }
    pub fn ws_pin_mut(&mut self) -> &mut SPI::Ws {
        &mut self.pins.0
    }
}

impl<I: Instance> I2s<I> {
    /// Returns the frequency of the clock signal that the SPI peripheral is receiving from the
    /// I2S PLL or similar source
    pub fn input_clock(&self) -> Hertz {
        self.input_clock
    }
}

/// Implements stm32_i2s_v12x::I2sPeripheral for I2s<$SPIX, _> and creates an I2s::$spix function
/// to create and enable the peripheral
///
/// $SPIX: The fully-capitalized name of the SPI peripheral (example: SPI1)
/// $i2sx: The lowercase I2S name of the peripheral (example: i2s1). This is the name of the
/// function that creates an I2s and enables the peripheral clock.
/// $clock: The name of the Clocks function that returns the frequency of the I2S clock input
/// to this SPI peripheral (i2s_cl, i2s_apb1_clk, or i2s2_apb_clk)
macro_rules! i2s {
    ($SPI:ty, $I2s:ident, $clock:ident) => {
        pub type $I2s = I2s<$SPI>;

        impl Instance for $SPI {}

        impl I2sFreq for $SPI {
            fn i2s_freq(clocks: &Clocks) -> Hertz {
                clocks
                    .$clock()
                    .expect("I2S clock input for SPI not enabled")
            }
        }

        #[cfg(feature = "stm32_i2s_v12x")]
        unsafe impl stm32_i2s_v12x::I2sPeripheral for I2s<$SPI> {
            const REGISTERS: *const () = <$SPI>::ptr() as *const _;
            fn i2s_freq(&self) -> u32 {
                self.input_clock.raw()
            }
            fn ws_is_high(&self) -> bool {
                self.ws_pin().is_high()
            }
            fn ws_is_low(&self) -> bool {
                self.ws_pin().is_low()
            }
        }
    };
}

// Actually define the SPI instances that can be used for I2S
// Each one has to be split into two declarations because the F412, F413, F423, and F446
// have two different I2S clocks while other models have only one.

#[cfg(any(feature = "gpio-f410", feature = "gpio-f411"))]
i2s!(pac::SPI1, I2s1, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446",))]
i2s!(pac::SPI1, I2s1, i2s_apb2_clk);

// All STM32F4 models support SPI2/I2S2
#[cfg(not(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446",)))]
i2s!(pac::SPI2, I2s2, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446",))]
i2s!(pac::SPI2, I2s2, i2s_apb1_clk);

// All STM32F4 models except STM32F410 support SPI3/I2S3
#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
i2s!(pac::SPI3, I2s3, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446",))]
i2s!(pac::SPI3, I2s3, i2s_apb1_clk);

#[cfg(feature = "gpio-f411")]
i2s!(pac::SPI4, I2s4, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
i2s!(pac::SPI4, I2s4, i2s_apb2_clk);

#[cfg(any(feature = "gpio-f410", feature = "gpio-f411"))]
i2s!(pac::SPI5, I2s5, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
i2s!(pac::SPI5, I2s5, i2s_apb2_clk);

// DMA support: reuse existing mappings for SPI
#[cfg(feature = "stm32_i2s_v12x")]
mod dma {
    use super::*;
    use crate::dma::traits::{DMASet, PeriAddress};
    use core::ops::Deref;
    use stm32_i2s_v12x::driver::I2sDriver;

    /// I2S DMA reads from and writes to the data register
    unsafe impl<SPI: Instance, MS, TR, STD> PeriAddress for I2sDriver<I2s<SPI>, MS, TR, STD>
    where
        I2s<SPI>: stm32_i2s_v12x::I2sPeripheral,
        SPI: Deref<Target = crate::pac::spi1::RegisterBlock>,
    {
        /// SPI_DR is only 16 bits. Multiple transfers are needed for a 24-bit or 32-bit sample,
        /// as explained in the reference manual.
        type MemSize = u16;

        fn address(&self) -> u32 {
            let registers = &*self.i2s_peripheral().spi;
            &registers.dr as *const _ as u32
        }
    }

    /// DMA is available for I2S based on the underlying implementations for SPI
    unsafe impl<SPI: Instance, MS, TR, STD, STREAM, const CHANNEL: u8, DIR>
        DMASet<STREAM, CHANNEL, DIR> for I2sDriver<I2s<SPI>, MS, TR, STD>
    where
        SPI: DMASet<STREAM, CHANNEL, DIR>,
    {
    }
}
