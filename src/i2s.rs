//! I2S (inter-IC Sound) communication using SPI peripherals
//!
//! This module is only available if the `i2s` feature is enabled.

use stm32_i2s_v12x::{Instance, RegisterBlock};

use crate::pac::RCC;
use crate::time::Hertz;
use crate::{bb, rcc::Clocks, spi};

// I2S pins are mostly the same as the corresponding SPI pins:
// MOSI -> SD
// NSS -> WS (the current SPI code doesn't define NSS pins)
// SCK -> CK
// The master clock output is separate.

/// A pin that can be used as SD (serial data)
pub trait PinSd<SPI> {}
/// A pin that can be used as WS (word select, left/right clock)
pub trait PinWs<SPI> {}
/// A pin that can be used as CK (bit clock)
pub trait PinCk<SPI> {}
/// A pin that can be used as MCK (master clock output)
pub trait PinMck<SPI> {}

/// Each MOSI pin can also be used as SD
impl<P, SPI> PinSd<SPI> for P where P: spi::PinMosi<SPI> {}
/// Each SCK pin can also be used as CK
impl<P, SPI> PinCk<SPI> for P where P: spi::PinSck<SPI> {}

/// A placeholder for when the MCLK pin is not needed
pub struct NoMasterClock;

mod sealed {
    pub trait Sealed {}
}

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
        ($($PER:ident => $pin:ident<$af:ident>,)+) => {
            $(
                impl crate::i2s::sealed::Sealed for $pin<crate::gpio::Alternate<$af>> {}
                impl crate::i2s::PinMck<$PER> for $pin<crate::gpio::Alternate<$af>> {}
            )+
        };
    }

    mod common {
        use crate::gpio::{gpioc::PC6, AF5};
        use crate::pac::SPI2;
        // All STM32F4 models support PC6<AF5> for SPI2/I2S2
        pin_mck! { SPI2 => PC6<AF5>, }
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
            AF5, AF6,
        };
        use crate::pac::{SPI2, SPI3};
        pin_mck! {
            SPI2 => PA3<AF5>,
            SPI2 => PA6<AF6>,
            SPI3 => PB10<AF6>,
        }
    }

    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    mod pc4_af5 {
        use crate::gpio::{gpioc::PC4, AF5};
        use crate::pac::SPI1;
        pin_mck! { SPI1 => PC4<AF5>, }
    }

    // On all models except the STM32F410, PC7<AF6> is the master clock output from I2S3.
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
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    mod i2s3_pc7_af6 {
        use crate::gpio::{gpioc::PC7, AF6};
        use crate::pac::SPI3;
        pin_mck! { SPI3 => PC7<AF6>, }
    }

    // On the STM32F410, PC7<AF6> is the master clock output from I2S1 instead of I2S3.
    // Also, PB10<AF6> is the master clock output from I2S1 instead of I2S3.
    #[cfg(feature = "stm32f410")]
    mod i2s1_pc7_af6 {
        use crate::gpio::{gpiob::PB10, gpioc::PC7, AF6};
        use crate::pac::SPI1;
        pin_mck! {
            SPI1 => PC7<AF6>,
            SPI1 => PB10<AF6>,
        }
    }
}

/// Word select (WS) pins
mod ws_pins {
    macro_rules! pin_ws {
        ($($PER:ident => $pin:ident<$af:ident>,)+) => {
            $(
                impl crate::i2s::sealed::Sealed for $pin<crate::gpio::Alternate<$af>> {}
                impl crate::i2s::PinWs<$PER> for $pin<crate::gpio::Alternate<$af>> {}
            )+
        };
    }

    mod common {
        use crate::gpio::{
            gpiob::{PB12, PB9},
            AF5,
        };
        use crate::pac::SPI2;
        // All STM32F4 models support these pins
        pin_ws! {
            SPI2 => PB9<AF5>,
            SPI2 => PB12<AF5>,
        }
    }

    /// Pins available on all models except the STM32F410
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
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    mod not_f410 {
        use crate::gpio::{
            gpioa::{PA15, PA4},
            AF6,
        };
        use crate::pac::SPI3;
        pin_ws! {
            SPI3 => PA4<AF6>,
            SPI3 => PA15<AF6>,
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
        use crate::gpio::{
            gpioa::{PA15, PA4},
            AF5,
        };
        use crate::pac::SPI1;
        pin_ws! {
            SPI1 => PA4<AF5>,
            SPI1 => PA15<AF5>,
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
            AF5, AF6,
        };
        use crate::pac::{SPI4, SPI5};
        pin_ws! {
            SPI4 => PB12<AF6>,
            SPI4 => PE4<AF5>,
            SPI4 => PE11<AF5>,
            SPI5 => PE4<AF6>,
            SPI5 => PE11<AF6>,
        }
    }

    #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
    mod pa11 {
        use crate::gpio::{gpioa::PA11, AF5};
        use crate::pac::SPI2;
        pin_ws! { SPI2 => PA11<AF5>, }
    }

    #[cfg(any(
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pb1 {
        use crate::gpio::{gpiob::PB1, AF6};
        use crate::pac::SPI5;
        pin_ws! { SPI5 => PB1<AF6>, }
    }

    #[cfg(feature = "stm32f446")]
    mod pb4_pd1 {
        use crate::gpio::{gpiob::PB4, gpiod::PD1, AF7};
        use crate::pac::SPI2;
        pin_ws! {
            SPI2 => PB4<AF7>,
            SPI2 => PD1<AF7>,
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
        use crate::gpio::{gpioi::PI0, AF5};
        use crate::pac::SPI2;
        pin_ws! { SPI2 => PI0<AF5>, }
    }
}

// All STM32F4 models use the same bits in APB1ENR, APB2ENR, APB1RSTR, and APB2RSTR to enable
// and reset the SPI peripherals.
// SPI1: APB2 bit 12
// SPI2: APB1 bit 14
// SPI3: APB1 bit 15
// SPI4: APB2 bit 13
// SPI5: APB2 bit 20

/// Implements Instance for I2s<$SPIX, _> and creates an I2s::$spix function to create and enable
/// the peripheral
///
/// $SPIX: The fully-capitalized name of the SPI peripheral (example: SPI1)
/// $i2sx: The lowercase I2S name of the peripheral (example: i2s1). This is the name of the
/// function that creates an I2s and enables the peripheral clock.
/// $clock: The name of the Clocks function that returns the frequency of the I2S clock input
/// to this SPI peripheral (i2s_cl, i2s_apb1_clk, or i2s2_apb_clk)
/// $apbxenr: The lowercase name of the RCC peripheral enable register (apb1enr or apb2enr)
/// $apbxrstr: The lowercase name of the RCC peripheral reset register (apb1rstr or apb2rstr)
/// $rcc_bit: The index (starting at 0) in $apbxenr and $apbxrstr of the enable and reset bits
/// for this SPI peripheral
macro_rules! i2s {
    ($SPIX:ty, $i2sx:ident, $clock:ident, $apbxenr:ident, $apbxrstr:ident, $rcc_bit:expr) => {
        impl<PINS> I2s<$SPIX, PINS>
        where
            PINS: Pins<$SPIX>,
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
            pub fn $i2sx(spi: $SPIX, pins: PINS, clocks: Clocks) -> Self {
                let input_clock = clocks
                    .$clock()
                    .expect("I2S clock input for SPI not enabled");
                unsafe {
                    // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                    let rcc = &(*RCC::ptr());
                    // Enable clock, enable reset, clear, reset
                    bb::set(&rcc.$apbxenr, $rcc_bit);

                    // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                    cortex_m::asm::dsb();

                    bb::set(&rcc.$apbxrstr, $rcc_bit);
                    bb::clear(&rcc.$apbxrstr, $rcc_bit);
                }
                I2s {
                    _spi: spi,
                    _pins: pins,
                    input_clock,
                }
            }
        }
        impl PinMck<$SPIX> for NoMasterClock {}
        unsafe impl<PINS> Instance for I2s<$SPIX, PINS>
        where
            PINS: Pins<$SPIX>,
        {
            const REGISTERS: *mut RegisterBlock = <$SPIX>::ptr() as *mut _;
        }
    };
}

// Actually define the SPI instances that can be used for I2S
// Each one has to be split into two declarations because the F412, F413, F423, and F446
// have two different I2S clocks while other models have only one.

#[cfg(any(feature = "stm32f410", feature = "stm32f411"))]
i2s!(crate::pac::SPI1, i2s1, i2s_clk, apb2enr, apb2rstr, 12);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(crate::pac::SPI1, i2s1, i2s_apb2_clk, apb2enr, apb2rstr, 12);

// All STM32F4 models support SPI2/I2S2
#[cfg(not(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
)))]
i2s!(crate::pac::SPI2, i2s2, i2s_clk, apb1enr, apb1rstr, 14);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(crate::pac::SPI2, i2s2, i2s_apb1_clk, apb1enr, apb1rstr, 14);

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
i2s!(crate::pac::SPI3, i2s3, i2s_clk, apb1enr, apb1rstr, 15);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
i2s!(crate::pac::SPI3, i2s3, i2s_apb1_clk, apb1enr, apb1rstr, 15);

#[cfg(feature = "stm32f411")]
i2s!(crate::pac::SPI4, i2s4, i2s_clk, apb2enr, apb2rstr, 13);
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
i2s!(crate::pac::SPI4, i2s4, i2s_apb2_clk, apb2enr, apb2rstr, 13);

#[cfg(any(feature = "stm32f410", feature = "stm32f411"))]
i2s!(crate::pac::SPI5, i2s5, i2s_clk, apb2enr, apb2rstr, 20);
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
i2s!(crate::pac::SPI5, i2s5, i2s_apb2_clk, apb2enr, apb2rstr, 20);

/// An I2s wrapper around an SPI object and pins
pub struct I2s<I, PINS> {
    _spi: I,
    _pins: PINS,
    /// Frequency of clock input to this peripheral from the I2S PLL or related source
    input_clock: Hertz,
}

impl<I, PINS> I2s<I, PINS>
where
    PINS: Pins<I>,
{
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
    unsafe impl<SPI, PINS, MODE, STREAM, CHANNEL, DIR> DMASet<STREAM, CHANNEL, DIR>
        for stm32_i2s_v12x::I2s<I2s<SPI, PINS>, MODE>
    where
        SPI: DMASet<STREAM, CHANNEL, DIR>,
        PINS: Pins<SPI>,
    {
    }
}
