//! I2S (inter-IC Sound) communication using SPI peripherals
//!
//! This module is only available if the `i2s` feature is enabled.
//!
//! Note: while F413 and F423 have full duplex i2s capability, this mode is not yet availalble for
//! these chips because their `I2S2EXT` and `I2S3EXT` peripherals are missing from their package
//! access crate.

use crate::gpio::{self, NoPin, PinSpeed, Speed};
use crate::pac;
#[allow(unused)]
use crate::rcc::{self, Clocks, Reset};
use fugit::HertzU32 as Hertz;

#[cfg(feature = "i2s")]
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

/// Trait for SPI peripheral that have an extension for full duplex i2s capability.
pub trait DualInstance: Instance + gpio::alt::I2sExtPin {
    /// The I2SEXT peripheral that extend the SPI peripheral
    type I2sExtPeripheral;
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

/// Trait to build an [`DualI2s`] object from SPI peripheral, a I2SEXT peripheral, pins and clocks
pub trait DualI2sExt: Sized + DualInstance {
    fn dual_i2s(
        self,
        i2s_ext: Self::I2sExtPeripheral,
        pins: (
            impl Into<Self::Ws>,
            impl Into<Self::Ck>,
            impl Into<Self::Mck>,
            impl Into<Self::Sd>,
            impl Into<Self::ExtSd>,
        ),
        clocks: &Clocks,
    ) -> DualI2s<Self>;
}

impl<SPI: DualInstance> DualI2sExt for SPI {
    fn dual_i2s(
        self,
        i2s_ext: Self::I2sExtPeripheral,
        pins: (
            impl Into<Self::Ws>,
            impl Into<Self::Ck>,
            impl Into<Self::Mck>,
            impl Into<Self::Sd>,
            impl Into<Self::ExtSd>,
        ),
        clocks: &Clocks,
    ) -> DualI2s<Self> {
        DualI2s::new(self, i2s_ext, pins, clocks)
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
            // Enable clock, enable reset, clear, reset
            SPI::enable_unchecked();
            SPI::reset_unchecked();
        }

        let pins = (
            pins.0.into(),
            // Workaround for corrupted last bit of data issue, see stm32f411 errata
            pins.1.into().speed(Speed::VeryHigh),
            pins.2.into(),
            pins.3.into(),
        );

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

/// Implements stm32_i2s_v12x::I2sPeripheral for I2s<$SPI> and creates an I2s::$spix function
/// to create and enable the peripheral
///
/// $SPI: The fully-capitalized name of the SPI peripheral from pac module (example: SPI1)
/// $I2s: The CamelCase I2S alias name for hal I2s wrapper (example: I2s1).
/// $i2s: module containing the Ws pin definition. (example: i2s1).
/// $clock: The name of the Clocks function that returns the frequency of the I2S clock input
/// to this SPI peripheral (i2s_cl, i2s_apb1_clk, or i2s2_apb_clk)
macro_rules! i2s {
    ($SPI:ty, $I2s:ident, $i2s:ident, $clock:ident) => {
        pub type $I2s = I2s<$SPI>;

        impl Instance for $SPI {}

        impl I2sFreq for $SPI {
            fn i2s_freq(clocks: &Clocks) -> Hertz {
                clocks
                    .$clock()
                    .expect("I2S clock input for SPI not enabled")
            }
        }

        #[cfg(feature = "i2s")]
        impl stm32_i2s_v12x::WsPin for gpio::alt::$i2s::Ws {
            fn is_high(&self) -> bool {
                use crate::gpio::ReadPin;
                <Self as ReadPin>::is_high(self)
            }
            fn is_low(&self) -> bool {
                use crate::gpio::ReadPin;
                <Self as ReadPin>::is_low(self)
            }
        }

        #[cfg(feature = "i2s")]
        unsafe impl stm32_i2s_v12x::I2sPeripheral for I2s<$SPI>
        where
            $SPI: rcc::Reset,
        {
            type WsPin = gpio::alt::$i2s::Ws;
            const REGISTERS: *const () = <$SPI>::ptr() as *const _;
            fn i2s_freq(&self) -> u32 {
                self.input_clock.raw()
            }
            fn ws_pin(&self) -> &Self::WsPin {
                self.ws_pin()
            }
            fn ws_pin_mut(&mut self) -> &mut Self::WsPin {
                self.ws_pin_mut()
            }
            fn rcc_reset(&mut self) {
                unsafe {
                    <$SPI>::reset_unchecked();
                }
            }
        }
    };
}

// Actually define the SPI instances that can be used for I2S
// Each one has to be split into two declarations because the F412, F413, F423, and F446
// have two different I2S clocks while other models have only one.

#[cfg(any(feature = "gpio-f410", feature = "gpio-f411"))]
i2s!(pac::SPI1, I2s1, i2s1, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
i2s!(pac::SPI1, I2s1, i2s1, i2s_apb2_clk);

// All STM32F4 models support SPI2/I2S2
#[cfg(not(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446")))]
i2s!(pac::SPI2, I2s2, i2s2, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
i2s!(pac::SPI2, I2s2, i2s2, i2s_apb1_clk);

// All STM32F4 models except STM32F410 support SPI3/I2S3
#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
i2s!(pac::SPI3, I2s3, i2s3, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
i2s!(pac::SPI3, I2s3, i2s3, i2s_apb1_clk);

#[cfg(feature = "gpio-f411")]
i2s!(pac::SPI4, I2s4, i2s4, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
i2s!(pac::SPI4, I2s4, i2s4, i2s_apb2_clk);

#[cfg(any(feature = "gpio-f410", feature = "gpio-f411"))]
i2s!(pac::SPI5, I2s5, i2s5, i2s_clk);
#[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
i2s!(pac::SPI5, I2s5, i2s5, i2s_apb2_clk);

/// A wrapper around a SPI and a I2SEXT object and pins for full duplex I2S operation
#[allow(clippy::type_complexity)]
pub struct DualI2s<I: DualInstance> {
    spi: I,
    i2s_ext: I::I2sExtPeripheral,
    pins: (I::Ws, I::Ck, I::Mck, I::Sd, I::ExtSd),
    /// Frequency of clock input to this peripheral from the I2S PLL or related source
    input_clock: Hertz,
}

impl<SPI: DualInstance> DualI2s<SPI> {
    /// Creates an DualI2s object around a SPI peripheral, it's I2SEXT extension, and pins
    ///
    /// This function enables and resets the SPI and I2SEXT peripheral, but does not configure it.
    ///
    /// The returned DualI2s object implements `stm32_i2s_v12x::DualI2sPeripheral`, so it can be used to
    /// configure the peripheral and communicate.
    ///
    /// # Panics
    ///
    /// This function panics if the I2S clock input (from the I2S PLL or similar)
    /// is not configured.
    pub fn new(
        spi: SPI,
        i2s_ext: SPI::I2sExtPeripheral,
        pins: (
            impl Into<SPI::Ws>,
            impl Into<SPI::Ck>,
            impl Into<SPI::Mck>,
            impl Into<SPI::Sd>,
            impl Into<SPI::ExtSd>,
        ),
        clocks: &Clocks,
    ) -> Self {
        let input_clock = SPI::i2s_freq(clocks);
        unsafe {
            // Enable clock, enable reset, clear, reset
            // Note: this also affect the I2SEXT peripheral
            SPI::enable_unchecked();
            SPI::reset_unchecked();
        }

        let pins = (
            pins.0.into(),
            // Workaround for corrupted last bit of data issue, see stm32f411 errata
            pins.1.into().speed(Speed::VeryHigh),
            pins.2.into(),
            pins.3.into(),
            pins.4.into(),
        );

        Self {
            spi,
            i2s_ext,
            pins,
            input_clock,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn release(
        self,
    ) -> (
        SPI,
        SPI::I2sExtPeripheral,
        (SPI::Ws, SPI::Ck, SPI::Mck, SPI::Sd, SPI::ExtSd),
    ) {
        (self.spi, self.i2s_ext, self.pins)
    }
}

impl<SPI: DualInstance> DualI2s<SPI> {
    pub fn ws_pin(&self) -> &SPI::Ws {
        &self.pins.0
    }
    pub fn ws_pin_mut(&mut self) -> &mut SPI::Ws {
        &mut self.pins.0
    }
}

impl<I: DualInstance> DualI2s<I> {
    /// Returns the frequency of the clock signal that the SPI peripheral is receiving from the
    /// I2S PLL or similar source
    pub fn input_clock(&self) -> Hertz {
        self.input_clock
    }
}

/// Implements stm32_i2s_v12x::DualI2sPeripheral for DualI2s<$SPI>
///
/// $SPI: The fully-capitalized name of the SPI peripheral from pac module (example: SPI1)
/// $I2SEXT: The fully-capitalized name of the I2SEXT peripheral from pac module (example: I2S3EXT)
/// $DualI2s: The CamelCase I2S alias name for hal I2s wrapper (example: DualI2s1).
/// $i2s: module containing the Ws pin definition. (example: i2s1).
/// $clock: The name of the Clocks function that returns the frequency of the I2S clock input
/// to this SPI peripheral (i2s_cl, i2s_apb1_clk, or i2s2_apb_clk)
#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
macro_rules! dual_i2s {
    ($SPI:ty,$I2SEXT:ty, $DualI2s:ident, $i2s:ident, $clock:ident) => {
        pub type $DualI2s = DualI2s<$SPI>;

        impl DualInstance for $SPI {
            type I2sExtPeripheral = $I2SEXT;
        }

        #[cfg(feature = "i2s")]
        unsafe impl stm32_i2s_v12x::DualI2sPeripheral for DualI2s<$SPI>
        where
            $SPI: rcc::Reset,
        {
            type WsPin = gpio::alt::$i2s::Ws;
            const MAIN_REGISTERS: *const () = <$SPI>::ptr() as *const _;
            const EXT_REGISTERS: *const () = <$I2SEXT>::ptr() as *const _;
            fn i2s_freq(&self) -> u32 {
                self.input_clock.raw()
            }
            fn ws_pin(&self) -> &Self::WsPin {
                self.ws_pin()
            }
            fn ws_pin_mut(&mut self) -> &mut Self::WsPin {
                self.ws_pin_mut()
            }
            fn rcc_reset(&mut self) {
                unsafe {
                    <$SPI>::reset_unchecked();
                }
            }
        }
    };
}

// Actually define objects for dual i2s
// Each one has to be split into two declarations because the F412, F413, F423, and F446
// have two different I2S clocks while other models have only one.
// All STM32F4 models except STM32F410 and STM32F446 have dual i2s support on SPI2 and SPI3
#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
dual_i2s!(pac::SPI2, pac::I2S2EXT, DualI2s2, i2s2, i2s_clk);

// add "gpio-f413" feature here when missing I2SEXT in pac wil be fixed.
#[cfg(feature = "gpio-f412")]
dual_i2s!(pac::SPI2, pac::I2S2EXT, DualI2s2, i2s2, i2s_apb1_clk);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
dual_i2s!(pac::SPI3, pac::I2S3EXT, DualI2s3, i2s3, i2s_clk);

// add "gpio-f413" feature here when missing I2SEXT in pac wil be fixed.
#[cfg(feature = "gpio-f412")]
dual_i2s!(pac::SPI3, pac::I2S3EXT, DualI2s3, i2s3, i2s_apb1_clk);

// DMA support: reuse existing mappings for SPI
#[cfg(feature = "i2s")]
mod dma {
    use super::*;
    use crate::dma::traits::{DMASet, PeriAddress};
    use crate::pac::spi1::RegisterBlock;
    use core::marker::PhantomData;
    use core::ops::Deref;
    use stm32_i2s_v12x::driver::{I2sCore, I2sDriver};
    use stm32_i2s_v12x::transfer::{Ext, Main};
    use stm32_i2s_v12x::DualI2sPeripheral;

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
            self.data_register_address()
        }
    }

    /// DMA is available for I2S based on the underlying implementations for SPI
    unsafe impl<SPI: Instance, MS, TR, STD, STREAM, const CHANNEL: u8, DIR>
        DMASet<STREAM, CHANNEL, DIR> for I2sDriver<I2s<SPI>, MS, TR, STD>
    where
        SPI: DMASet<STREAM, CHANNEL, DIR>,
    {
    }

    pub trait DualI2sDmaTargetExt<I, PART, MS, DIR, STD> {
        fn dma_target(&self) -> DualI2sDmaTarget<I, PART, MS, DIR, STD>;
    }
    impl<I, PART, MS, DIR, STD> DualI2sDmaTargetExt<I, PART, MS, DIR, STD>
        for I2sCore<I, PART, MS, DIR, STD>
    {
        fn dma_target(&self) -> DualI2sDmaTarget<I, PART, MS, DIR, STD> {
            DualI2sDmaTarget {
                _dual_i2s_peripheral: PhantomData,
                _part: PhantomData,
                _ms: PhantomData,
                _dir: PhantomData,
                _std: PhantomData,
            }
        }
    }

    ///  - `I`: The [DualI2sPeripheral] controlled by the I2sCore.
    ///  - `PART`: `Main` or `Ext`. The part of [DualI2sPeripheral] controlled by I2sCore.
    ///  - `MS`: `Master` or `Slave`. The role of the I2sCore. Only a `Main` I2sCore can be Master.
    ///  - `DIR` : `Transmit` or `Receive`. Communication direction.
    ///  - `STD`: I2S standard, eg `Philips`
    pub struct DualI2sDmaTarget<I, PART, MS, DIR, STD> {
        _dual_i2s_peripheral: PhantomData<I>,
        _part: PhantomData<PART>,
        _ms: PhantomData<MS>,
        _dir: PhantomData<DIR>,
        _std: PhantomData<STD>,
    }

    macro_rules! dual_dma {
        ($ext: ty, $reg: ident) => {
            /// I2S DMA reads from and writes to the data register
            unsafe impl<SPIext: DualInstance, MS, TR, STD> PeriAddress
                for DualI2sDmaTarget<DualI2s<SPIext>, $ext, MS, TR, STD>
            where
                DualI2s<SPIext>: DualI2sPeripheral,
            {
                /// SPI_DR is only 16 bits. Multiple transfers are needed for a 24-bit or 32-bit sample,
                /// as explained in the reference manual.
                type MemSize = u16;

                fn address(&self) -> u32 {
                    let reg = unsafe { &*(DualI2s::$reg as *const RegisterBlock) };
                    reg.dr().as_ptr() as u32
                }
            }
        };
    }

    dual_dma!(Main, MAIN_REGISTERS);
    dual_dma!(Ext, EXT_REGISTERS);

    /// DMA is available for I2S based on the underlying implementations for SPI
    unsafe impl<SPIext: DualInstance, PART, MS, TR, STD, STREAM, const CHANNEL: u8, DIR>
        DMASet<STREAM, CHANNEL, DIR> for DualI2sDmaTarget<DualI2s<SPIext>, PART, MS, TR, STD>
    where
        SPIext: DMASet<STREAM, CHANNEL, DIR>,
    {
    }
}

#[cfg(feature = "stm32_i2s_v12x")]
pub use dma::{DualI2sDmaTarget, DualI2sDmaTargetExt};
