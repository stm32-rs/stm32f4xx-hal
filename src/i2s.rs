//! I2S (inter-IC Sound) communication using SPI peripherals
//!
//! This module is only available if the `i2s` feature is enabled.

use crate::gpio::{Const, NoPin, PushPull, SetAlternate};
use stm32_i2s_v12x::{Instance, RegisterBlock};

use crate::pac::RCC;
use crate::rcc;
use crate::time::Hertz;
use crate::{rcc::Clocks, spi};

// I2S pins are mostly the same as the corresponding SPI pins:
// MOSI -> SD
// NSS -> WS (the current SPI code doesn't define NSS pins)
// SCK -> CK
// The master clock output is separate.

/// A pin that can be used as SD (serial data)
pub trait PinSd<SPI> {
    type A;
}
/// A pin that can be used as WS (word select, left/right clock)
pub trait PinWs<SPI> {
    type A;
}
/// A pin that can be used as CK (bit clock)
pub trait PinCk<SPI> {
    type A;
}
/// A pin that can be used as MCK (master clock output)
pub trait PinMck<SPI> {
    type A;
}

impl<SPI> PinMck<SPI> for NoPin
where
    SPI: Instance,
{
    type A = Const<0>;
}

/// Each MOSI pin can also be used as SD
impl<P, SPI, const MOSIA: u8> PinSd<SPI> for P
where
    P: spi::PinMosi<SPI, A = Const<MOSIA>>,
{
    type A = Const<MOSIA>;
}
/// Each SCK pin can also be used as CK
impl<P, SPI, const SCKA: u8> PinCk<SPI> for P
where
    P: spi::PinSck<SPI, A = Const<SCKA>>,
{
    type A = Const<SCKA>;
}

/// A placeholder for when the MCLK pin is not needed
pub type NoMasterClock = NoPin;

/// A set of pins configured for I2S communication: (WS, CK, MCLK, SD)
///
/// NoMasterClock can be used instead of the master clock pin.
pub trait Pins<SPI> {}

impl<SPI, PWS, PCK, PMCLK, PSD> Pins<SPI> for (PWS, PCK, PMCLK, PSD)
where
    PWS: PinWs<SPI>,
    PCK: PinCk<SPI>,
    PMCLK: PinMck<SPI>,
    PSD: PinSd<SPI>,
{
}

/// Master clock (MCK) pins
mod mck_pins {
    macro_rules! pin_mck {
        ($($PER:ident => $pin:ident<$af:literal>,)+) => {
            $(
                impl<MODE> crate::i2s::PinMck<$PER> for $pin<MODE> {
                    type A = crate::gpio::Const<$af>;
                }
            )+
        };
    }

    mod common {
        use crate::gpio::gpioc::PC6;
        use crate::pac::SPI2;
        // All STM32F4 models support PC6<5> for SPI2/I2S2
        pin_mck! { SPI2 => PC6<5>, }
    }

    #[cfg(any(
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pa3_pa6_pb10 {
        use crate::gpio::{
            gpioa::{PA3, PA6},
            gpiob::PB10,
        };
        use crate::pac::{SPI2, SPI3};
        pin_mck! {
            SPI2 => PA3<5>,
            SPI2 => PA6<6>,
            SPI3 => PB10<6>,
        }
    }

    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    mod pc4_af5 {
        use crate::gpio::gpioc::PC4;
        use crate::pac::SPI1;
        pin_mck! { SPI1 => PC4<5>, }
    }

    // On all models except the STM32F410, PC7<6> is the master clock output from I2S3.
    #[cfg(feature = "spi3")]
    mod i2s3_pc7_af6 {
        use crate::gpio::gpioc::PC7;
        use crate::pac::SPI3;
        pin_mck! { SPI3 => PC7<6>, }
    }

    // On the STM32F410, PC7<6> is the master clock output from I2S1 instead of I2S3.
    // Also, PB10<6> is the master clock output from I2S1 instead of I2S3.
    #[cfg(feature = "stm32f410")]
    mod i2s1_pc7_af6 {
        use crate::gpio::{gpiob::PB10, gpioc::PC7};
        use crate::pac::SPI1;
        pin_mck! {
            SPI1 => PC7<6>,
            SPI1 => PB10<6>,
        }
    }
}

/// Word select (WS) pins
mod ws_pins {
    macro_rules! pin_ws {
        ($($PER:ident => $pin:ident<$af:literal>,)+) => {
            $(
                impl<MODE> crate::i2s::PinWs<$PER> for $pin<MODE> {
                    type A = crate::gpio::Const<$af>;
                }
            )+
        };
    }

    mod common {
        use crate::gpio::gpiob::{PB12, PB9};
        use crate::pac::SPI2;
        // All STM32F4 models support these pins
        pin_ws! {
            SPI2 => PB9<5>,
            SPI2 => PB12<5>,
        }
    }

    /// Pins available on all models except the STM32F410
    #[cfg(feature = "spi3")]
    mod not_f410 {
        use crate::gpio::gpioa::{PA15, PA4};
        use crate::pac::SPI3;
        pin_ws! {
            SPI3 => PA4<6>,
            SPI3 => PA15<6>,
        }
    }
    #[cfg(any(
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    mod pa4_af5_pa15_af5 {
        use crate::gpio::gpioa::{PA15, PA4};
        use crate::pac::SPI1;
        pin_ws! {
            SPI1 => PA4<5>,
            SPI1 => PA15<5>,
        }
    }
    #[cfg(any(
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pb12_pe4_pe11 {
        use crate::gpio::{
            gpiob::PB12,
            gpioe::{PE11, PE4},
        };
        use crate::pac::{SPI4, SPI5};
        pin_ws! {
            SPI4 => PB12<6>,
            SPI4 => PE4<5>,
            SPI4 => PE11<5>,
            SPI5 => PE4<6>,
            SPI5 => PE11<6>,
        }
    }

    #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
    mod pa11 {
        use crate::gpio::gpioa::PA11;
        use crate::pac::SPI2;
        pin_ws! { SPI2 => PA11<5>, }
    }

    #[cfg(any(
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pb1 {
        use crate::gpio::gpiob::PB1;
        use crate::pac::SPI5;
        pin_ws! { SPI5 => PB1<6>, }
    }

    #[cfg(feature = "stm32f446")]
    mod pb4_pd1 {
        use crate::gpio::{gpiob::PB4, gpiod::PD1};
        use crate::pac::SPI2;
        pin_ws! {
            SPI2 => PB4<7>,
            SPI2 => PD1<7>,
        }
    }

    #[cfg(any(
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    mod pi0 {
        use crate::gpio::gpioi::PI0;
        use crate::pac::SPI2;
        pin_ws! { SPI2 => PI0<5>, }
    }
}

pub trait I2sFreq {
    fn i2s_freq(clocks: &Clocks) -> Hertz;
}

/// Implements Instance for I2s<$SPIX, _> and creates an I2s::$spix function to create and enable
/// the peripheral
///
/// $SPIX: The fully-capitalized name of the SPI peripheral (example: SPI1)
/// $i2sx: The lowercase I2S name of the peripheral (example: i2s1). This is the name of the
/// function that creates an I2s and enables the peripheral clock.
/// $clock: The name of the Clocks function that returns the frequency of the I2S clock input
/// to this SPI peripheral (i2s_cl, i2s_apb1_clk, or i2s2_apb_clk)
macro_rules! i2s {
    ($SPIX:ty, $clock:ident) => {
        impl I2sFreq for $SPIX {
            fn i2s_freq(clocks: &Clocks) -> Hertz {
                clocks
                    .$clock()
                    .expect("I2S clock input for SPI not enabled")
            }
        }

        unsafe impl<PINS> Instance for I2s<$SPIX, PINS> {
            const REGISTERS: *mut RegisterBlock = <$SPIX>::ptr() as *mut _;
        }
    };
}

impl<SPI, WS, CK, MCLK, SD, const WSA: u8, const CKA: u8, const MCLKA: u8, const SDA: u8>
    I2s<SPI, (WS, CK, MCLK, SD)>
where
    SPI: I2sFreq + rcc::Enable + rcc::Reset,
    WS: PinWs<SPI, A = Const<WSA>> + SetAlternate<PushPull, WSA>,
    CK: PinCk<SPI, A = Const<CKA>> + SetAlternate<PushPull, CKA>,
    MCLK: PinMck<SPI, A = Const<MCLKA>> + SetAlternate<PushPull, MCLKA>,
    SD: PinSd<SPI, A = Const<SDA>> + SetAlternate<PushPull, SDA>,
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
    pub fn new(spi: SPI, mut pins: (WS, CK, MCLK, SD), clocks: &Clocks) -> Self {
        let input_clock = SPI::i2s_freq(clocks);
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            // Enable clock, enable reset, clear, reset
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.0.set_alt_mode();
        pins.1.set_alt_mode();
        pins.2.set_alt_mode();
        pins.3.set_alt_mode();

        I2s {
            _spi: spi,
            _pins: pins,
            input_clock,
        }
    }
}

// Actually define the SPI instances that can be used for I2S
// Each one has to be split into two declarations because the F412, F413, F423, and F446
// have two different I2S clocks while other models have only one.

#[cfg(any(feature = "stm32f410", feature = "stm32f411"))]
i2s!(crate::pac::SPI1, i2s_clk);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(crate::pac::SPI1, i2s_apb2_clk);

// All STM32F4 models support SPI2/I2S2
#[cfg(not(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
)))]
i2s!(crate::pac::SPI2, i2s_clk);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(crate::pac::SPI2, i2s_apb1_clk);

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
i2s!(crate::pac::SPI3, i2s_clk);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(crate::pac::SPI3, i2s_apb1_clk);

#[cfg(feature = "stm32f411")]
i2s!(crate::pac::SPI4, i2s_clk);
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
i2s!(crate::pac::SPI4, i2s_apb2_clk);

#[cfg(any(feature = "stm32f410", feature = "stm32f411"))]
i2s!(crate::pac::SPI5, i2s_clk);
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
i2s!(crate::pac::SPI5, i2s_apb2_clk);

/// An I2s wrapper around an SPI object and pins
pub struct I2s<I, PINS> {
    _spi: I,
    _pins: PINS,
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

// DMA support: reuse existing mappings for SPI
mod dma {
    use super::*;
    use crate::dma::traits::{DMASet, PeriAddress};
    use core::ops::Deref;

    /// I2S DMA reads from and writes to the data register
    unsafe impl<SPI, PINS, MODE> PeriAddress for stm32_i2s_v12x::I2s<I2s<SPI, PINS>, MODE>
    where
        I2s<SPI, PINS>: Instance,
        PINS: Pins<SPI>,
        SPI: Deref<Target = crate::pac::spi1::RegisterBlock>,
    {
        /// SPI_DR is only 16 bits. Multiple transfers are needed for a 24-bit or 32-bit sample,
        /// as explained in the reference manual.
        type MemSize = u16;

        fn address(&self) -> u32 {
            let registers = &*self.instance()._spi;
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
