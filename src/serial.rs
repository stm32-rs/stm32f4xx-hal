use core::fmt;
use core::marker::PhantomData;
use core::ptr;

use embedded_hal::blocking;
use embedded_hal::prelude::*;
use embedded_hal::serial;
use nb::block;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
use crate::stm32::{RCC, USART1, USART2, USART6};

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::stm32::USART3;

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::stm32::{UART4, UART5};

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::{UART7, UART8};

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::stm32::{UART10, UART9};

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::PA15;
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpioa::PA8;
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpioa::{PA0, PA1};
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
use crate::gpio::gpioa::{PA10, PA2, PA3, PA9};
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::{PA11, PA12};

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpiob::PB3;
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiob::PB4;
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiob::PB5;
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpiob::{PB10, PB11};
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiob::{PB12, PB13};
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
use crate::gpio::gpiob::{PB6, PB7};
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiob::{PB8, PB9};

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpioc::PC12;
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpioc::PC5;
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpioc::{PC10, PC11};
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
use crate::gpio::gpioc::{PC6, PC7};

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiod::PD10;
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpiod::PD2;
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiod::{PD0, PD1};
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiod::{PD14, PD15};
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
    feature = "stm32f479"
))]
use crate::gpio::gpiod::{PD5, PD6};
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpiod::{PD8, PD9};

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioe::{PE0, PE1};
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpioe::{PE2, PE3};
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioe::{PE7, PE8};

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiof::{PF6, PF7};
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiof::{PF8, PF9};

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiog::{PG0, PG1};
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::gpiog::{PG11, PG12};
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
use crate::gpio::gpiog::{PG14, PG9};

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
use crate::gpio::AF11;
use crate::gpio::{Alternate, AF7, AF8};
use crate::rcc::Clocks;

use crate::dma::traits::PeriAddress;

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,
}

pub mod config {
    use crate::time::Bps;
    use crate::time::U32Ext;

    pub enum WordLength {
        DataBits8,
        DataBits9,
    }

    pub enum Parity {
        ParityNone,
        ParityEven,
        ParityOdd,
    }

    pub enum StopBits {
        #[doc = "1 stop bit"]
        STOP1,
        #[doc = "0.5 stop bits"]
        STOP0P5,
        #[doc = "2 stop bits"]
        STOP2,
        #[doc = "1.5 stop bits"]
        STOP1P5,
    }

    pub enum DmaConfig {
        None,
        Tx,
        Rx,
        TxRx,
    }

    pub struct Config {
        pub baudrate: Bps,
        pub wordlength: WordLength,
        pub parity: Parity,
        pub stopbits: StopBits,
        pub dma: DmaConfig,
    }

    impl Config {
        pub fn baudrate(mut self, baudrate: Bps) -> Self {
            self.baudrate = baudrate;
            self
        }

        pub fn parity_none(mut self) -> Self {
            self.parity = Parity::ParityNone;
            self
        }

        pub fn parity_even(mut self) -> Self {
            self.parity = Parity::ParityEven;
            self
        }

        pub fn parity_odd(mut self) -> Self {
            self.parity = Parity::ParityOdd;
            self
        }

        pub fn wordlength_8(mut self) -> Self {
            self.wordlength = WordLength::DataBits8;
            self
        }

        pub fn wordlength_9(mut self) -> Self {
            self.wordlength = WordLength::DataBits9;
            self
        }

        pub fn stopbits(mut self, stopbits: StopBits) -> Self {
            self.stopbits = stopbits;
            self
        }
    }

    #[derive(Debug)]
    pub struct InvalidConfig;

    impl Default for Config {
        fn default() -> Config {
            let baudrate = 19_200_u32.bps();
            Config {
                baudrate,
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
                dma: DmaConfig::None,
            }
        }
    }
}

pub trait Pins<USART> {}
pub trait PinTx<USART> {}
pub trait PinRx<USART> {}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: PinTx<USART>,
    RX: PinRx<USART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub struct NoTx;
/// A filler type for when the Rx pin is unnecessary
pub struct NoRx;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART1> for NoTx {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART1> for NoRx {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART1> for PA9<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART1> for PA10<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinTx<USART1> for PA15<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinRx<USART1> for PB3<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART1> for PB6<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART1> for PB7<Alternate<AF7>> {}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART2> for NoTx {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART2> for NoRx {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART2> for PA2<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART2> for PA3<Alternate<AF7>> {}
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
    feature = "stm32f479"
))]
impl PinTx<USART2> for PD5<Alternate<AF7>> {}
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
    feature = "stm32f479"
))]
impl PinRx<USART2> for PD6<Alternate<AF7>> {}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<USART3> for NoTx {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<USART3> for NoRx {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<USART3> for PB10<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<USART3> for PB11<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
impl PinRx<USART3> for PC5<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<USART3> for PC10<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<USART3> for PC11<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<USART3> for PD8<Alternate<AF7>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<USART3> for PD9<Alternate<AF7>> {}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<UART4> for NoTx {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<UART4> for NoRx {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<UART4> for PA0<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<UART4> for PA1<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART4> for PA12<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART4> for PA11<Alternate<AF11>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinTx<UART4> for PC10<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinRx<UART4> for PC11<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART4> for PD1<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART4> for PD0<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART4> for PD10<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART4> for PC11<Alternate<AF8>> {}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<UART5> for NoTx {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<UART5> for NoRx {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART5> for PB6<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART5> for PB5<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART5> for PB9<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART5> for PB8<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART5> for PB13<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART5> for PB12<Alternate<AF11>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<UART5> for PC12<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<UART5> for PD2<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f446"))]
impl PinTx<UART5> for PE8<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f446"))]
impl PinRx<UART5> for PE7<Alternate<AF8>> {}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART6> for NoTx {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART6> for NoRx {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinTx<USART6> for PA11<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinRx<USART6> for PA12<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinTx<USART6> for PC6<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
impl PinRx<USART6> for PC7<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinTx<USART6> for PG14<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
impl PinRx<USART6> for PG9<Alternate<AF8>> {}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinTx<UART7> for NoTx {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinRx<UART7> for NoRx {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART7> for PA15<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART7> for PA8<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART7> for PB4<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART7> for PB3<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinTx<UART7> for PE8<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinRx<UART7> for PE7<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinTx<UART7> for PF7<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinRx<UART7> for PF6<Alternate<AF8>> {}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinTx<UART8> for NoTx {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinRx<UART8> for NoRx {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinTx<UART8> for PE1<Alternate<AF8>> {}
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinRx<UART8> for PE0<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART8> for PF9<Alternate<AF8>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART8> for PF8<Alternate<AF8>> {}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART9> for NoTx {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART9> for NoRx {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART9> for PD15<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART9> for PD14<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART9> for PG1<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART9> for PG0<Alternate<AF11>> {}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART10> for NoTx {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART10> for NoRx {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART10> for PE3<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART10> for PE2<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinTx<UART10> for PG12<Alternate<AF11>> {}
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl PinRx<UART10> for PG11<Alternate<AF11>> {}

/// Serial abstraction
pub struct Serial<USART, PINS> {
    usart: USART,
    pins: PINS,
}

/// Serial receiver
pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}

macro_rules! halUsartImpl {
    ($(
        $USARTX:ident: ($usartX:ident, $apbXenr:ident, $rcc_bit:expr, $usartXen:ident, $pclkX:ident),
    )+) => {
        $(
            impl<PINS> Serial<$USARTX, PINS> {
                pub fn $usartX(
                    usart: $USARTX,
                    pins: PINS,
                    config: config::Config,
                    clocks: Clocks,
                ) -> Result<Self, config::InvalidConfig>
                where
                    PINS: Pins<$USARTX>,
                {
                    use self::config::*;
                    use crate::bb;

                    unsafe {
                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        let rcc = &(*RCC::ptr());

                        // Enable clock.
                        bb::set(&rcc.$apbXenr, $rcc_bit);
                    }

                    // Calculate correct baudrate divisor on the fly
                    let div = (clocks.$pclkX().0 + config.baudrate.0 / 2)
                        / config.baudrate.0;
                    usart.brr.write(|w| unsafe { w.bits(div) });

                    // Reset other registers to disable advanced USART features
                    usart.cr2.reset();
                    usart.cr3.reset();

                    // Enable transmission and receiving
                    // and configure frame
                    usart.cr1.write(|w| {
                        w.ue()
                            .set_bit()
                            .te()
                            .set_bit()
                            .re()
                            .set_bit()
                            .m()
                            .bit(match config.wordlength {
                                WordLength::DataBits8 => false,
                                WordLength::DataBits9 => true,
                            }).pce()
                            .bit(match config.parity {
                                Parity::ParityNone => false,
                                _ => true,
                            }).ps()
                            .bit(match config.parity {
                                Parity::ParityOdd => true,
                                _ => false,
                            })
                    });

                    match config.dma {
                        DmaConfig::Tx => {
                            usart.cr3.write(|w| {
                                w.dmat().enabled()
                            })
                        }
                        DmaConfig::Rx => {
                            usart.cr3.write(|w| {
                                w.dmar().enabled()
                            })
                        }
                        DmaConfig::TxRx => {
                            usart.cr3.write(|w| {
                                w
                                    .dmar().enabled()
                                    .dmat().enabled()
                            })
                        }
                        DmaConfig::None => {}
                    }

                    Ok(Serial { usart, pins }.config_stop(config))
                }

                /// Starts listening for an interrupt event
                ///
                /// Note, you will also have to enable the corresponding interrupt
                /// in the NVIC to start receiving events.
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.usart.cr1.modify(|_, w| w.rxneie().set_bit())
                        },
                        Event::Txe => {
                            self.usart.cr1.modify(|_, w| w.txeie().set_bit())
                        },
                        Event::Idle => {
                            self.usart.cr1.modify(|_, w| w.idleie().set_bit())
                        },
                    }
                }

                /// Stop listening for an interrupt event
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.usart.cr1.modify(|_, w| w.rxneie().clear_bit())
                        },
                        Event::Txe => {
                            self.usart.cr1.modify(|_, w| w.txeie().clear_bit())
                        },
                        Event::Idle => {
                            self.usart.cr1.modify(|_, w| w.idleie().clear_bit())
                        },
                    }
                }

                /// Return true if the line idle status is set
                pub fn is_idle(& self) -> bool {
                    unsafe { (*$USARTX::ptr()).sr.read().idle().bit_is_set() }
                }

                /// Return true if the tx register is empty (and can accept data)
                pub fn is_txe(& self) -> bool {
                    unsafe { (*$USARTX::ptr()).sr.read().txe().bit_is_set() }
                }

                /// Return true if the rx register is not empty (and can be read)
                pub fn is_rxne(& self) -> bool {
                    unsafe { (*$USARTX::ptr()).sr.read().rxne().bit_is_set() }
                }

                pub fn split(self) -> (Tx<$USARTX>, Rx<$USARTX>) {
                    (
                        Tx {
                            _usart: PhantomData,
                        },
                        Rx {
                            _usart: PhantomData,
                        },
                    )
                }
                pub fn release(self) -> ($USARTX, PINS) {
                    (self.usart, self.pins)
                }
            }

            impl<PINS> serial::Read<u8> for Serial<$USARTX, PINS> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    let mut rx: Rx<$USARTX> = Rx {
                        _usart: PhantomData,
                    };
                    rx.read()
                }
            }

            impl serial::Read<u8> for Rx<$USARTX> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let sr = unsafe { (*$USARTX::ptr()).sr.read() };

                    // Any error requires the dr to be read to clear
                    if sr.pe().bit_is_set()
                        || sr.fe().bit_is_set()
                        || sr.nf().bit_is_set()
                        || sr.ore().bit_is_set()
                    {
                        unsafe { (*$USARTX::ptr()).dr.read() };
                    }

                    Err(if sr.pe().bit_is_set() {
                        nb::Error::Other(Error::Parity)
                    } else if sr.fe().bit_is_set() {
                        nb::Error::Other(Error::Framing)
                    } else if sr.nf().bit_is_set() {
                        nb::Error::Other(Error::Noise)
                    } else if sr.ore().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.rxne().bit_is_set() {
                        // NOTE(read_volatile) see `write_volatile` below
                        return Ok(unsafe { ptr::read_volatile(&(*$USARTX::ptr()).dr as *const _ as *const _) });
                    } else {
                        nb::Error::WouldBlock
                    })
                }
            }

            unsafe impl PeriAddress for Rx<$USARTX> {
                #[inline(always)]
                fn address(&self) -> u32 {
                    &(unsafe{ &(*$USARTX::ptr()) }.dr) as *const _ as u32
                }

                type MemSize = u8;
            }

            impl<PINS> serial::Write<u8> for Serial<$USARTX, PINS> {
                type Error = Error;

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    let mut tx: Tx<$USARTX> = Tx {
                        _usart: PhantomData,
                    };
                    tx.flush()
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    let mut tx: Tx<$USARTX> = Tx {
                        _usart: PhantomData,
                    };
                    tx.write(byte)
                }
            }

            unsafe impl PeriAddress for Tx<$USARTX> {
                #[inline(always)]
                fn address(&self) -> u32 {
                    &(unsafe{ &(*$USARTX::ptr()) }.dr) as *const _ as u32
                }

                type MemSize = u8;
            }

            impl serial::Write<u8> for Tx<$USARTX> {
                type Error = Error;

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let sr = unsafe { (*$USARTX::ptr()).sr.read() };

                    if sr.tc().bit_is_set() {
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let sr = unsafe { (*$USARTX::ptr()).sr.read() };

                    if sr.txe().bit_is_set() {
                        // NOTE(unsafe) atomic write to stateless register
                        // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
                        unsafe { ptr::write_volatile(&(*$USARTX::ptr()).dr as *const _ as *mut _, byte) }
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }
            }

            impl blocking::serial::Write<u8> for Tx<$USARTX> {
                type Error = Error;

                fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
                    for &b in bytes {
                        loop {
                            match self.write(b) {
                                Err(nb::Error::WouldBlock) => continue,
                                Err(nb::Error::Other(err)) => return Err(err),
                                Ok(()) => break,
                            }
                        }
                    }
                    Ok(())
                }

                fn bflush(&mut self) -> Result<(), Self::Error> {
                    loop {
                        match self.flush() {
                            Ok(()) => return Ok(()),
                            Err(nb::Error::WouldBlock) => continue,
                            Err(nb::Error::Other(err)) => return Err(err),
                        }
                    }
                }
            }

            impl<PINS> blocking::serial::Write<u8> for Serial<$USARTX, PINS> {
                type Error = Error;

                fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
                    let mut tx: Tx<$USARTX> = Tx {
                        _usart: PhantomData,
                    };
                    tx.bwrite_all(bytes)
                }

                fn bflush(&mut self) -> Result<(), Self::Error> {
                    let mut tx: Tx<$USARTX> = Tx {
                        _usart: PhantomData,
                    };
                    tx.bflush()
                }
            }
        )+
    }
}

macro_rules! halUsart {
    ($(
        $USARTX:ident: ($usartX:ident, $apbXenr:ident, $rcc_bit:expr, $usartXen:ident, $pclkX:ident),
    )+) => {
        $(
        impl<PINS> Serial<$USARTX, PINS> {
            fn config_stop(self, config: config::Config) -> Self {
                use crate::stm32::usart1::cr2::STOP_A;
                use self::config::*;

                self.usart.cr2.write(|w| {
                    w.stop().variant(match config.stopbits {
                        StopBits::STOP0P5 => STOP_A::STOP0P5,
                        StopBits::STOP1 => STOP_A::STOP1,
                        StopBits::STOP1P5 => STOP_A::STOP1P5,
                        StopBits::STOP2 => STOP_A::STOP2,
                    })
                });
                self
            }
        }
        )+

        halUsartImpl! {
            $( $USARTX: ($usartX, $apbXenr, $rcc_bit, $usartXen, $pclkX), )+
        }
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
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
macro_rules! halUart {
    ($(
        $USARTX:ident: ($usartX:ident, $apbXenr:ident, $rcc_bit:expr, $usartXen:ident, $pclkX:ident),
    )+) => {
        $(
        impl<PINS> Serial<$USARTX, PINS> {
            fn config_stop(self, config: config::Config) -> Self {
                use crate::stm32::uart4::cr2::STOP_A;
                use self::config::*;

                self.usart.cr2.write(|w| {
                    w.stop().variant(match config.stopbits {
                        StopBits::STOP0P5 => STOP_A::STOP1,
                        StopBits::STOP1 => STOP_A::STOP1,
                        StopBits::STOP1P5 => STOP_A::STOP2,
                        StopBits::STOP2 => STOP_A::STOP2,
                    })
                });
                self
            }
        }
        )+

        halUsartImpl! {
            $( $USARTX: ($usartX, $apbXenr, $rcc_bit, $usartXen, $pclkX), )+
        }
    }
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
    feature = "stm32f479"
))]
halUsart! {
    USART1: (usart1, apb2enr, 4, usart1en, pclk2),
    USART2: (usart2, apb1enr, 17, usart2en, pclk1),
    USART6: (usart6, apb2enr, 5, usart6en, pclk2),
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
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
    feature = "stm32f479"
))]
halUsart! {
    USART3: (usart3, apb1enr, 18, usart3en, pclk1),
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
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
halUart! {
    UART4: (uart4, apb1enr, 19, uart4en, pclk1),
    UART5: (uart5, apb1enr, 20, uart5en, pclk1),
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! {
    UART4: (uart4, apb1enr, 19, uart4en, pclk1),
    UART5: (uart5, apb1enr, 20, uart5en, pclk1),
}

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
halUsart! {
    UART7: (uart7, apb1enr, 30, uart7en, pclk1),
    UART8: (uart8, apb1enr, 31, uart8en, pclk1),
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! {
    UART9: (uart9, apb2enr, 6, uart9en, pclk2),
    UART10: (uart10, apb2enr, 7, uart10en, pclk2),
}

impl<USART, PINS> fmt::Write for Serial<USART, PINS>
where
    Serial<USART, PINS>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| block!(self.write(*c)))
            .map_err(|_| fmt::Error)
    }
}

impl<USART> fmt::Write for Tx<USART>
where
    Tx<USART>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| block!(self.write(*c)))
            .map_err(|_| fmt::Error)
    }
}
