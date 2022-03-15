//! I2S (inter-IC Sound) communication using SPI peripherals
//!
//! This module is only available if the `i2s` feature is enabled.

use crate::gpio::{Const, NoPin, PinA, PushPull, SetAlternate};
#[cfg(feature = "stm32_i2s_v12x")]
use stm32_i2s_v12x::RegisterBlock;

use crate::pac::{self, RCC};
use crate::rcc;
use crate::{rcc::Clocks, spi};
use fugit::HertzU32 as Hertz;

// I2S pins are mostly the same as the corresponding SPI pins:
// MOSI -> SD
// NSS -> WS (the current SPI code doesn't define NSS pins)
// SCK -> CK
// The master clock output is separate.

/// A pin that can be used as SD (serial data)
///
/// Each MOSI pin can also be used as SD
pub type Sd = spi::Mosi;

/// A pin that can be used as WS (word select, left/right clock)
pub type Ws = spi::Nss;

/// A pin that can be used as CK (bit clock)
///
/// Each SCK pin can also be used as CK
pub type Ck = spi::Sck;

/// A pin that can be used as MCK (master clock output)
pub struct Mck;
impl crate::Sealed for Mck {}

/// A pin that can be as SD with extended I2s instance
pub struct ExtSd;
impl crate::Sealed for ExtSd {}

/// A placeholder for when the MCLK pin is not needed
pub type NoMasterClock = NoPin;

/// A set of pins configured for simple or full-duplex I2S communication:
/// - (WS, CK, MCLK, SD) for simple I2s
/// - (WS, CK, MCLK, SD, EXTSD) for Dual I2s
///
/// NoMasterClock can be used instead of the master clock pin.
pub trait Pins<I> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}

/// Pins for simple I2s communication
impl<SPI, WS, CK, MCLK, SD, const WSA: u8, const CKA: u8, const MCLKA: u8, const SDA: u8> Pins<SPI>
    for (WS, CK, MCLK, SD)
where
    WS: PinA<Ws, SPI, A = Const<WSA>> + SetAlternate<WSA, PushPull>,
    CK: PinA<Ck, SPI, A = Const<CKA>> + SetAlternate<CKA, PushPull>,
    MCLK: PinA<Mck, SPI, A = Const<MCLKA>> + SetAlternate<MCLKA, PushPull>,
    SD: PinA<Sd, SPI, A = Const<SDA>> + SetAlternate<SDA, PushPull>,
{
    fn set_alt_mode(&mut self) {
        self.0.set_alt_mode();
        self.1.set_alt_mode();
        self.2.set_alt_mode();
        self.3.set_alt_mode();
    }
    fn restore_mode(&mut self) {
        self.0.restore_mode();
        self.1.restore_mode();
        self.2.restore_mode();
        self.3.restore_mode();
    }
}

/// Pins for Dual I2s communication
impl<
        SPI,
        I2SEXT,
        WS,
        CK,
        MCLK,
        SD,
        EXTSD,
        const WSA: u8,
        const CKA: u8,
        const MCLKA: u8,
        const SDA: u8,
        const EXTSDA: u8,
    > Pins<(SPI, I2SEXT)> for (WS, CK, MCLK, SD, EXTSD)
where
    WS: PinA<Ws, SPI, A = Const<WSA>> + SetAlternate<WSA, PushPull>,
    CK: PinA<Ck, SPI, A = Const<CKA>> + SetAlternate<CKA, PushPull>,
    MCLK: PinA<Mck, SPI, A = Const<MCLKA>> + SetAlternate<MCLKA, PushPull>,
    SD: PinA<Sd, SPI, A = Const<SDA>> + SetAlternate<SDA, PushPull>,
    EXTSD: PinA<ExtSd, I2SEXT, A = Const<EXTSDA>> + SetAlternate<EXTSDA, PushPull>,
{
    fn set_alt_mode(&mut self) {
        self.0.set_alt_mode();
        self.1.set_alt_mode();
        self.2.set_alt_mode();
        self.3.set_alt_mode();
        self.4.set_alt_mode();
    }
    fn restore_mode(&mut self) {
        self.0.restore_mode();
        self.1.restore_mode();
        self.2.restore_mode();
        self.3.restore_mode();
        self.4.restore_mode();
    }
}

pub trait Instance: I2sFreq + rcc::Enable + rcc::Reset {}

pub trait I2sFreq {
    fn i2s_freq(clocks: &Clocks) -> Hertz;
}

/// Implements stm32_i2s_v12x::Instance for I2s<$SPIX, _> and creates an I2s::$spix function to create and enable
/// the peripheral
///
/// $SPIX: The fully-capitalized name of the SPI peripheral (example: SPI1)
/// $i2sx: The lowercase I2S name of the peripheral (example: i2s1). This is the name of the
/// function that creates an I2s and enables the peripheral clock.
/// $clock: The name of the Clocks function that returns the frequency of the I2S clock input
/// to this SPI peripheral (i2s_cl, i2s_apb1_clk, or i2s2_apb_clk)
macro_rules! i2s {
    ($SPI:ty, $I2s:ident, $clock:ident) => {
        pub type $I2s<PINS> = I2s<$SPI, PINS>;

        impl Instance for $SPI {}

        impl I2sFreq for $SPI {
            fn i2s_freq(clocks: &Clocks) -> Hertz {
                clocks
                    .$clock()
                    .expect("I2S clock input for SPI not enabled")
            }
        }

        #[cfg(feature = "stm32_i2s_v12x")]
        unsafe impl<PINS> stm32_i2s_v12x::Instance for I2s<$SPI, PINS> {
            const REGISTERS: *mut RegisterBlock = <$SPI>::ptr() as *mut _;
        }
    };
}

pub trait I2sExt: Sized + Instance {
    fn i2s<PINS: Pins<Self>>(self, pins: PINS, clocks: &Clocks) -> I2s<Self, PINS>;
}

impl<SPI: Instance> I2sExt for SPI {
    fn i2s<PINS: Pins<Self>>(self, pins: PINS, clocks: &Clocks) -> I2s<Self, PINS> {
        I2s::new(self, pins, clocks)
    }
}

impl<SPI, PINS> I2s<SPI, PINS>
where
    SPI: Instance,
    PINS: Pins<SPI>,
{
    /// Creates an I2s object around an SPI peripheral and pins
    ///
    /// This function enables and resets the SPI peripheral, but does not configure it.
    ///
    /// The returned I2s object implements [stm32_i2s_v12x::Instance], so it can be used
    /// to configure the peripheral and communicate.
    ///
    /// # Panics
    ///
    /// This function panics if the I2S clock input (from the I2S PLL or similar)
    /// is not configured.
    pub fn new(spi: SPI, mut pins: PINS, clocks: &Clocks) -> Self {
        let input_clock = SPI::i2s_freq(clocks);
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            // Enable clock, enable reset, clear, reset
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.set_alt_mode();

        I2s {
            spi,
            pins,
            input_clock,
        }
    }

    pub fn release(mut self) -> (SPI, PINS) {
        self.pins.restore_mode();

        (self.spi, self.pins)
    }
}

// Actually define the SPI instances that can be used for I2S
// Each one has to be split into two declarations because the F412, F413, F423, and F446
// have two different I2S clocks while other models have only one.

#[cfg(any(feature = "stm32f410", feature = "stm32f411"))]
i2s!(pac::SPI1, I2s1, i2s_clk);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(pac::SPI1, I2s1, i2s_apb2_clk);

// All STM32F4 models support SPI2/I2S2
#[cfg(not(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
)))]
i2s!(pac::SPI2, I2s2, i2s_clk);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(pac::SPI2, I2s2, i2s_apb1_clk);

// All STM32F4 models except STM32F410 support SPI3/I2S3
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479",
))]
i2s!(pac::SPI3, I2s3, i2s_clk);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(pac::SPI3, I2s3, i2s_apb1_clk);

#[cfg(feature = "stm32f411")]
i2s!(pac::SPI4, I2s4, i2s_clk);
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
i2s!(pac::SPI4, I2s4, i2s_apb2_clk);

#[cfg(any(feature = "stm32f410", feature = "stm32f411"))]
i2s!(pac::SPI5, I2s5, i2s_clk);
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
i2s!(pac::SPI5, I2s5, i2s_apb2_clk);

/// An I2s wrapper around an SPI object and pins
pub struct I2s<I, PINS> {
    spi: I,
    pins: PINS,
    /// Frequency of clock input to this peripheral from the I2S PLL or related source
    input_clock: Hertz,
}

impl<I, PINS> I2s<I, PINS> {
    /// Returns the frequency of the clock signal that the SPI peripheral is receiving from the
    /// I2S PLL or similar source
    pub fn input_clock(&self) -> Hertz {
        self.input_clock
    }
}

/// An full duplex I2s wrapper around SPI and I2SEXT object and pins
pub struct DualI2s<I, PINS> {
    interfaces: I,
    pins: PINS,
    /// Frequency of clock input to this peripheral from the I2S PLL or related source
    input_clock: Hertz,
}

impl<SPI, I2SEXT, PINS> DualI2s<(SPI, I2SEXT), PINS>
where
    SPI: Instance,
    PINS: Pins<(SPI, I2SEXT)>,
{
    /// Creates a DualI2s object around a SPI and I2SEXT peripherals and pins
    ///
    /// This function enables and resets SPI and I2SEXT peripherals, but does not configure it.
    ///
    /// The returned DualI2s object implements [stm32_i2s_v12x::DualInstance], so it can be used
    /// to configure the peripheral and communicate.
    ///
    /// # Panics
    ///
    /// This function panics if the I2S clock input (from the I2S PLL or similar)
    /// is not configured.
    pub fn new(interfaces: (SPI, I2SEXT), mut pins: PINS, clocks: &Clocks) -> Self {
        let input_clock = SPI::i2s_freq(clocks);
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            // Enable clock, enable reset, clear, reset
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.set_alt_mode();

        DualI2s {
            interfaces,
            pins,
            input_clock,
        }
    }

    pub fn release(mut self) -> ((SPI, I2SEXT), PINS) {
        self.pins.restore_mode();

        (self.interfaces, self.pins)
    }
}

/// Implements stm32_i2s_v12x::DualInstance for `DualI2s<($SPI,$I2SEXT), _>.
///
/// $SPI: The fully-capitalized name of the SPI peripheral (example: SPI1)
/// $I2SEXT: The fully-capitalized name of the I2SEXT peripheral (example: I2S2EXT)
macro_rules! dual_i2s {
    (($SPI:ty, $I2SEXT:ty)) => {
        #[cfg(feature = "stm32_i2s_v12x")]
        unsafe impl<PINS> stm32_i2s_v12x::DualInstance for DualI2s<($SPI, $I2SEXT), PINS> {
            const REGISTERS: (
                *mut stm32_i2s_v12x::RegisterBlock,
                *mut stm32_i2s_v12x::RegisterBlock,
            ) = (
                <$SPI>::ptr() as *mut stm32_i2s_v12x::RegisterBlock,
                <$I2SEXT>::ptr() as *mut stm32_i2s_v12x::RegisterBlock,
            );
        }
    };
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
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
dual_i2s!((pac::SPI2, pac::I2S2EXT));

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
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
dual_i2s!((pac::SPI3, pac::I2S3EXT));

// DMA support: reuse existing mappings for SPI
#[cfg(feature = "stm32_i2s_v12x")]
mod dma {
    use super::*;
    use crate::dma::traits::{DMASet, PeriAddress};
    use core::ops::Deref;

    /// I2S DMA reads from and writes to the data register
    unsafe impl<SPI, PINS, MODE> PeriAddress for stm32_i2s_v12x::I2s<I2s<SPI, PINS>, MODE>
    where
        I2s<SPI, PINS>: stm32_i2s_v12x::Instance,
        PINS: Pins<SPI>,
        SPI: Deref<Target = crate::pac::spi1::RegisterBlock>,
    {
        /// SPI_DR is only 16 bits. Multiple transfers are needed for a 24-bit or 32-bit sample,
        /// as explained in the reference manual.
        type MemSize = u16;

        fn address(&self) -> u32 {
            let registers = &*self.instance().spi;
            &registers.dr as *const _ as u32
        }
    }

    /// DMA is available for I2S based on the underlying implementations for SPI
    unsafe impl<SPI, PINS, MODE, STREAM, DIR, const CHANNEL: u8> DMASet<STREAM, DIR, CHANNEL>
        for stm32_i2s_v12x::I2s<I2s<SPI, PINS>, MODE>
    where
        SPI: DMASet<STREAM, DIR, CHANNEL>,
    {
    }
}
