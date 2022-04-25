//! I2S (inter-IC Sound) communication using SPI peripherals
//!
//! This module is only available if the `i2s` feature is enabled.

use crate::gpio::marker::{Interruptable, Readable};
use crate::gpio::{Const, NoPin, Pin, PinA, PushPull, SetAlternate};
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

/// A marker for pin that can be used as SD (serial data)
pub struct Sd;

/// A marker for pin that can be used as WS (word select, left/right clock)
pub struct Ws;
impl Readable for Ws {}
impl Interruptable for Ws {}

/// A marker for pin that can be used as CK (bit clock)
pub struct Ck;

/// A marker for pin that can be used as MCK (master clock output)
pub struct Mck;
impl crate::Sealed for Mck {}

/// A placeholder for when the MCLK pin is not needed
pub type NoMasterClock = NoPin;

/// A trait for WS pin.
///
/// This allow to use a WsPin trought any type implementing Pins
pub trait WsPin {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

impl<const P: char, const N: u8> WsPin for Pin<P, N, Ws> {
    fn is_high(&self) -> bool {
        self.is_high()
    }
    fn is_low(&self) -> bool {
        self.is_low()
    }
}

/// prevent usage of the inner trait outside the crate since it allow pin state violation
mod pins {
    /// A set of pins configured for I2S communication: (WS, CK, MCLK, SD)
    ///
    /// NoMasterClock can be used instead of the master clock pin.
    pub trait Pins<SPI> {
        /// WS pin in I2S alternate type state.
        type WsPin: super::WsPin;
        fn set_alt_mode(&mut self);
        fn restore_mode(&mut self);
        /// Get WS pin.
        fn ws_pin(&self) -> &Self::WsPin;
        /// Get WS pin mutably.
        fn ws_pin_mut(&mut self) -> &mut Self::WsPin;
    }
}
use pins::*;

impl<
        SPI,
        const WSP: char,
        const WSN: u8,
        WSM,
        const WSA: u8,
        CK,
        const CKA: u8,
        MCLK,
        const MCLKA: u8,
        SD,
        const SDA: u8,
    > Pins<SPI> for (Pin<WSP, WSN, WSM>, CK, MCLK, SD)
where
    Pin<WSP, WSN, WSM>: PinA<Ws, SPI, A = Const<WSA>> + SetAlternate<WSA, PushPull>,
    CK: PinA<Ck, SPI, A = Const<CKA>> + SetAlternate<CKA, PushPull>,
    MCLK: PinA<Mck, SPI, A = Const<MCLKA>> + SetAlternate<MCLKA, PushPull>,
    SD: PinA<Sd, SPI, A = Const<SDA>> + SetAlternate<SDA, PushPull>,
{
    type WsPin = Pin<WSP, WSN, Ws>;
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
    fn ws_pin(&self) -> &Self::WsPin {
        unsafe { &*(&self.0 as *const _ as *const Self::WsPin) }
    }
    fn ws_pin_mut(&mut self) -> &mut Self::WsPin {
        unsafe { &mut *(&mut self.0 as *mut _ as *mut Self::WsPin) }
    }
}

/// Trait for SPI peripheral with i2s capability.
pub trait Instance: I2sFreq + rcc::Enable + rcc::Reset {}

/// Trait to get I2s frequency at SPI peripheral input.
pub trait I2sFreq {
    fn i2s_freq(clocks: &Clocks) -> Hertz;
}

/// Trait to build an [`I2s`] object from SPI peripheral, pins and clocks
pub trait I2sExt: Sized + Instance {
    fn i2s<WS, CK, MCLK, SD>(
        self,
        pins: (WS, CK, MCLK, SD),
        clocks: &Clocks,
    ) -> I2s<Self, (WS, CK, MCLK, SD)>
    where
        (WS, CK, MCLK, SD): Pins<Self>;
}

impl<SPI: Instance> I2sExt for SPI {
    fn i2s<WS, CK, MCLK, SD>(
        self,
        pins: (WS, CK, MCLK, SD),
        clocks: &Clocks,
    ) -> I2s<Self, (WS, CK, MCLK, SD)>
    where
        (WS, CK, MCLK, SD): Pins<Self>,
    {
        I2s::new(self, pins, clocks)
    }
}

/// An I2s wrapper around an SPI object and pins
pub struct I2s<I, PINS> {
    spi: I,
    pins: PINS,
    /// Frequency of clock input to this peripheral from the I2S PLL or related source
    input_clock: Hertz,
}

// Note: for API documenting reason, it's better to keep `(WS, CK, MCLK, SD)` for ctor and dtor
// than replacing by `PINS`
impl<SPI, WS, CK, MCLK, SD> I2s<SPI, (WS, CK, MCLK, SD)>
where
    SPI: Instance,
    (WS, CK, MCLK, SD): Pins<SPI>,
{
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
    pub fn new(spi: SPI, mut pins: (WS, CK, MCLK, SD), clocks: &Clocks) -> Self {
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

    pub fn release(mut self) -> (SPI, (WS, CK, MCLK, SD)) {
        self.pins.restore_mode();

        (
            self.spi,
            (self.pins.0, self.pins.1, self.pins.2, self.pins.3),
        )
    }
}

impl<SPI, PINS: Pins<SPI>> I2s<SPI, PINS> {
    pub fn ws_pin(&self) -> &PINS::WsPin {
        self.pins.ws_pin()
    }
    pub fn ws_pin_mut(&mut self) -> &mut PINS::WsPin {
        self.pins.ws_pin_mut()
    }
}

impl<I, PINS> I2s<I, PINS> {
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
        unsafe impl<PINS: Pins<$SPI>> stm32_i2s_v12x::I2sPeripheral for I2s<$SPI, PINS> {
            const REGISTERS: *const () = <$SPI>::ptr() as *const _;
            fn i2s_freq(&self) -> u32 {
                self.input_clock.raw()
            }
            fn ws_is_high(&self) -> bool {
                self.pins.ws_pin().is_high()
            }
            fn ws_is_low(&self) -> bool {
                self.pins.ws_pin().is_low()
            }
        }
    };
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

// DMA support: reuse existing mappings for SPI
#[cfg(feature = "stm32_i2s_v12x")]
mod dma {
    use super::*;
    use crate::dma::traits::{DMASet, PeriAddress};
    use core::ops::Deref;
    use stm32_i2s_v12x::driver::I2sDriver;

    /// I2S DMA reads from and writes to the data register
    unsafe impl<SPI, PINS, MS, TR, STD> PeriAddress for I2sDriver<I2s<SPI, PINS>, MS, TR, STD>
    where
        I2s<SPI, PINS>: stm32_i2s_v12x::I2sPeripheral,
        PINS: Pins<SPI>,
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
    unsafe impl<SPI, PINS, MS, TR, STD, STREAM, const CHANNEL: u8, DIR> DMASet<STREAM, CHANNEL, DIR>
        for I2sDriver<I2s<SPI, PINS>, MS, TR, STD>
    where
        SPI: DMASet<STREAM, CHANNEL, DIR>,
    {
    }
}
