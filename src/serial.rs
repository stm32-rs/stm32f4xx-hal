//!
//! Asynchronous serial communication using UART/USART peripherals
//!
//! # Word length
//!
//! By default, the UART/USART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
//! the embedded-hal read and write traits with `u8` as the word type.
//!
//! You can also configure the hardware to use 9 data bits with the `Config` `wordlength_9()`
//! function. After creating a `Serial` with this option, use the `with_u16_data()` function to
//! convert the `Serial<_, _, u8>` object into a `Serial<_, _, u16>` that can send and receive
//! `u16`s.
//!
//! In this mode, the `Serial<_, _, u16>`, `Rx<_, u16>`, and `Tx<_, u16>` structs instead implement
//! the embedded-hal read and write traits with `u16` as the word type. You can use these
//! implementations for 9-bit words.
//!

use core::fmt;
use core::marker::PhantomData;

use crate::rcc;
use embedded_hal::blocking;
use embedded_hal::prelude::*;
use embedded_hal::serial;
use nb::block;

use crate::gpio::{Const, SetAlternate};

#[cfg(feature = "gpiod")]
use crate::gpio::gpiod;
#[allow(unused)]
#[cfg(feature = "gpioe")]
use crate::gpio::gpioe;
#[allow(unused)]
#[cfg(feature = "gpiof")]
use crate::gpio::gpiof;
#[allow(unused)]
#[cfg(feature = "gpiog")]
use crate::gpio::gpiog;
use crate::gpio::{gpioa, gpiob, gpioc};

use crate::pac::{RCC, USART1, USART2, USART6};

#[cfg(feature = "usart3")]
use crate::pac::USART3;

#[cfg(feature = "uart10")]
use crate::pac::UART10;
#[cfg(feature = "uart4")]
use crate::pac::UART4;
#[cfg(feature = "uart5")]
use crate::pac::UART5;
#[cfg(feature = "uart7")]
use crate::pac::UART7;
#[cfg(feature = "uart8")]
use crate::pac::UART8;
#[cfg(feature = "uart9")]
use crate::pac::UART9;

use crate::gpio::NoPin;
use crate::rcc::Clocks;

use crate::dma::traits::PeriAddress;

/// Serial error
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
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
            let baudrate = 115_200_u32.bps();
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
pub trait PinTx<USART> {
    type A;
}
pub trait PinRx<USART> {
    type A;
}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: PinTx<USART>,
    RX: PinRx<USART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub type NoTx = NoPin;
/// A filler type for when the Rx pin is unnecessary
pub type NoRx = NoPin;

impl<USART> PinTx<USART> for NoPin
where
    USART: Instance,
{
    type A = Const<0>;
}

impl<USART> PinRx<USART> for NoPin
where
    USART: Instance,
{
    type A = Const<0>;
}

macro_rules! pin {
    ($trait:ident<$USART:ident> for $gpio:ident::$PX:ident<$A:literal>) => {
        impl<MODE> $trait<$USART> for $gpio::$PX<MODE> {
            type A = Const<$A>;
        }
    };
}

pin!(PinTx<USART1> for gpioa::PA9<7>);
pin!(PinRx<USART1> for gpioa::PA10<7>);

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin!(PinTx<USART1> for gpioa::PA15<7>);
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin!(PinRx<USART1> for gpiob::PB3<7>);

pin!(PinTx<USART1> for gpiob::PB6<7>);

pin!(PinRx<USART1> for gpiob::PB7<7>);

pin!(PinTx<USART2> for gpioa::PA2<7>);
pin!(PinRx<USART2> for gpioa::PA3<7>);

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
pin!(PinTx<USART2> for gpiod::PD5<7>);
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
pin!(PinRx<USART2> for gpiod::PD6<7>);

#[cfg(feature = "usart3")]
pin!(PinTx<USART3> for gpiob::PB10<7>);
#[cfg(feature = "usart3")]
pin!(PinRx<USART3> for gpiob::PB11<7>);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
pin!(PinRx<USART3> for gpioc::PC5<7>);
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
pin!(PinTx<USART3> for gpioc::PC10<7>);
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
pin!(PinRx<USART3> for gpioc::PC11<7>);
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
pin!(PinTx<USART3> for gpiod::PD8<7>);
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
pin!(PinRx<USART3> for gpiod::PD9<7>);

#[cfg(feature = "uart4")]
pin!(PinTx<UART4> for gpioa::PA0<8>);
#[cfg(feature = "uart4")]
pin!(PinRx<UART4> for gpioa::PA1<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART4> for gpioa::PA12<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART4> for gpioa::PA11<11>);
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
pin!(PinTx<UART4> for gpioc::PC10<8>);
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
pin!(PinRx<UART4> for gpioc::PC11<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART4> for gpiod::PD1<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART4> for gpiod::PD0<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART4> for gpiod::PD10<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART4> for gpioc::PC11<8>);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART5> for gpiob::PB6<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART5> for gpiob::PB5<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART5> for gpiob::PB9<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART5> for gpiob::PB8<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART5> for gpiob::PB13<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART5> for gpiob::PB12<11>);
#[cfg(feature = "uart5")]
pin!(PinTx<UART5> for gpioc::PC12<8>);
#[cfg(feature = "uart5")]
pin!(PinRx<UART5> for gpiod::PD2<8>);
#[cfg(any(feature = "stm32f446"))]
pin!(PinTx<UART5> for gpioe::PE8<8>);
#[cfg(any(feature = "stm32f446"))]
pin!(PinRx<UART5> for gpioe::PE7<8>);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin!(PinTx<USART6> for gpioa::PA11<8>);
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin!(PinRx<USART6> for gpioa::PA12<8>);

pin!(PinTx<USART6> for gpioc::PC6<8>);

pin!(PinRx<USART6> for gpioc::PC7<8>);
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
pin!(PinTx<USART6> for gpiog::PG14<8>);
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
pin!(PinRx<USART6> for gpiog::PG9<8>);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART7> for gpioa::PA15<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART7> for gpioa::PA8<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART7> for gpiob::PB4<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART7> for gpiob::PB3<8>);
#[cfg(all(feature = "uart7", feature = "gpioe"))]
pin!(PinTx<UART7> for gpioe::PE8<8>);
#[cfg(all(feature = "uart7", feature = "gpioe"))]
pin!(PinRx<UART7> for gpioe::PE7<8>);
#[cfg(all(feature = "uart7", feature = "gpiof"))]
pin!(PinTx<UART7> for gpiof::PF7<8>);
#[cfg(all(feature = "uart7", feature = "gpiof"))]
pin!(PinRx<UART7> for gpiof::PF6<8>);

#[cfg(all(feature = "uart8", feature = "gpioe"))]
pin!(PinTx<UART8> for gpioe::PE1<8>);
#[cfg(all(feature = "uart8", feature = "gpioe"))]
pin!(PinRx<UART8> for gpioe::PE0<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART8> for gpiof::PF9<8>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART8> for gpiof::PF8<8>);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART9> for gpiod::PD15<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART9> for gpiod::PD14<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART9> for gpiog::PG1<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART9> for gpiog::PG0<11>);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART10> for gpioe::PE3<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART10> for gpioe::PE2<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinTx<UART10> for gpiog::PG12<11>);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin!(PinRx<UART10> for gpiog::PG11<11>);

/// Serial abstraction
pub struct Serial<USART, PINS, WORD = u8> {
    usart: USART,
    pins: PINS,
    tx: Tx<USART, WORD>,
    rx: Rx<USART, WORD>,
}

/// Serial receiver
pub struct Rx<USART, WORD = u8> {
    _usart: PhantomData<USART>,
    _word: PhantomData<WORD>,
}

/// Serial transmitter
pub struct Tx<USART, WORD = u8> {
    _usart: PhantomData<USART>,
    _word: PhantomData<WORD>,
}

impl<USART, WORD> Rx<USART, WORD>
where
    USART: Instance,
{
    fn new() -> Self {
        Self {
            _usart: PhantomData,
            _word: PhantomData,
        }
    }

    /// Start listening for an rx not empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) }
    }

    /// Stop listening for the rx not empty interrupt event
    pub fn unlisten(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) }
    }

    /// Start listening for a line idle interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen_idle(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) }
    }

    /// Stop listening for the line idle interrupt event
    pub fn unlisten_idle(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) }
    }

    /// Return true if the line idle status is set
    pub fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().idle().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    /// Clear idle line interrupt flag
    pub fn clear_idle_interrupt(&self) {
        unsafe {
            let _ = (*USART::ptr()).sr.read();
            let _ = (*USART::ptr()).dr.read();
        }
    }
}

impl<USART, WORD> Tx<USART, WORD>
where
    USART: Instance,
{
    fn new() -> Self {
        Self {
            _usart: PhantomData,
            _word: PhantomData,
        }
    }

    /// Start listening for a tx empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) }
    }

    /// Stop listening for the tx empty interrupt event
    pub fn unlisten(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) }
    }

    /// Return true if the tx register is empty (and can accept data)
    pub fn is_tx_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }
}

impl<USART, PINS, WORD> AsRef<Tx<USART, WORD>> for Serial<USART, PINS, WORD> {
    fn as_ref(&self) -> &Tx<USART, WORD> {
        &self.tx
    }
}

impl<USART, PINS, WORD> AsRef<Rx<USART, WORD>> for Serial<USART, PINS, WORD> {
    fn as_ref(&self) -> &Rx<USART, WORD> {
        &self.rx
    }
}

impl<USART, PINS, WORD> AsMut<Tx<USART, WORD>> for Serial<USART, PINS, WORD> {
    fn as_mut(&mut self) -> &mut Tx<USART, WORD> {
        &mut self.tx
    }
}

impl<USART, PINS, WORD> AsMut<Rx<USART, WORD>> for Serial<USART, PINS, WORD> {
    fn as_mut(&mut self) -> &mut Rx<USART, WORD> {
        &mut self.rx
    }
}

impl<USART, TX, RX, WORD, const TXA: u8, const RXA: u8> Serial<USART, (TX, RX), WORD>
where
    TX: PinTx<USART, A = Const<TXA>> + SetAlternate<TXA>,
    RX: PinRx<USART, A = Const<RXA>> + SetAlternate<RXA>,
    USART: Instance,
{
    /*
        StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART

        STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
    */
    pub fn new(
        usart: USART,
        mut pins: (TX, RX),
        config: config::Config,
        clocks: Clocks,
    ) -> Result<Self, config::InvalidConfig> {
        use self::config::*;

        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable clock.
            USART::enable(rcc);
            USART::reset(rcc);
        }

        let pclk_freq = USART::get_frequency(&clocks).0;
        let baud = config.baudrate.0;

        // The frequency to calculate USARTDIV is this:
        //
        // (Taken from STM32F411xC/E Reference Manual,
        // Section 19.3.4, Equation 1)
        //
        // 16 bit oversample: OVER8 = 0
        // 8 bit oversample:  OVER8 = 1
        //
        // USARTDIV =          (pclk)
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // BUT, the USARTDIV has 4 "fractional" bits, which effectively
        // means that we need to "correct" the equation as follows:
        //
        // USARTDIV =      (pclk) * 16
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // When OVER8 is enabled, we can only use the lowest three
        // fractional bits, so we'll need to shift those last four bits
        // right one bit

        // Calculate correct baudrate divisor on the fly
        let (over8, div) = if (pclk_freq / 16) >= baud {
            // We have the ability to oversample to 16 bits, take
            // advantage of it.
            //
            // We also add `baud / 2` to the `pclk_freq` to ensure
            // rounding of values to the closest scale, rather than the
            // floored behavior of normal integer division.
            let div = (pclk_freq + (baud / 2)) / baud;
            (false, div)
        } else if (pclk_freq / 8) >= baud {
            // We are close enough to pclk where we can only
            // oversample 8.
            //
            // See note above regarding `baud` and rounding.
            let div = ((pclk_freq * 2) + (baud / 2)) / baud;

            // Ensure the the fractional bits (only 3) are
            // right-aligned.
            let frac = div & 0xF;
            let div = (div & !0xF) | (frac >> 1);
            (true, div)
        } else {
            return Err(config::InvalidConfig);
        };

        unsafe { (*USART::ptr()).brr.write(|w| w.bits(div)) };

        // Reset other registers to disable advanced USART features
        unsafe { (*USART::ptr()).cr2.reset() };
        unsafe { (*USART::ptr()).cr3.reset() };

        // Enable transmission and receiving
        // and configure frame
        unsafe {
            (*USART::ptr()).cr1.write(|w| {
                w.ue()
                    .set_bit()
                    .over8()
                    .bit(over8)
                    .te()
                    .set_bit()
                    .re()
                    .set_bit()
                    .m()
                    .bit(match config.wordlength {
                        WordLength::DataBits8 => false,
                        WordLength::DataBits9 => true,
                    })
                    .pce()
                    .bit(!matches!(config.parity, Parity::ParityNone))
                    .ps()
                    .bit(matches!(config.parity, Parity::ParityOdd))
            })
        };

        match config.dma {
            DmaConfig::Tx => unsafe { (*USART::ptr()).cr3.write(|w| w.dmat().enabled()) },
            DmaConfig::Rx => unsafe { (*USART::ptr()).cr3.write(|w| w.dmar().enabled()) },
            DmaConfig::TxRx => unsafe {
                (*USART::ptr())
                    .cr3
                    .write(|w| w.dmar().enabled().dmat().enabled())
            },
            DmaConfig::None => {}
        }

        pins.0.set_alt_mode();
        pins.1.set_alt_mode();

        Ok(Serial {
            usart,
            pins,
            tx: Tx::new(),
            rx: Rx::new(),
        }
        .config_stop(config))
    }
    pub fn release(mut self) -> (USART, (TX, RX)) {
        self.pins.0.restore_mode();
        self.pins.1.restore_mode();

        (self.usart, self.pins)
    }
}

impl<USART, TX, WORD, const TXA: u8> Serial<USART, (TX, NoPin), WORD>
where
    TX: PinTx<USART, A = Const<TXA>> + SetAlternate<TXA>,
    USART: Instance,
{
    pub fn tx(
        usart: USART,
        tx_pin: TX,
        config: config::Config,
        clocks: Clocks,
    ) -> Result<Tx<USART, WORD>, config::InvalidConfig> {
        Self::new(usart, (tx_pin, NoPin), config, clocks).map(|s| s.split().0)
    }
}

impl<USART, RX, WORD, const RXA: u8> Serial<USART, (NoPin, RX), WORD>
where
    RX: PinRx<USART, A = Const<RXA>> + SetAlternate<RXA>,
    USART: Instance,
{
    pub fn rx(
        usart: USART,
        rx_pin: RX,
        config: config::Config,
        clocks: Clocks,
    ) -> Result<Rx<USART, WORD>, config::InvalidConfig> {
        Self::new(usart, (NoPin, rx_pin), config, clocks).map(|s| s.split().1)
    }
}

impl<USART, PINS, WORD> Serial<USART, PINS, WORD>
where
    USART: Instance,
{
    /// Starts listening for an interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) },
            Event::Txe => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) },
            Event::Idle => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) },
        }
    }

    /// Stop listening for an interrupt event
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) },
            Event::Txe => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) },
            Event::Idle => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) },
        }
    }

    /// Return true if the line idle status is set
    pub fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().idle().bit_is_set() }
    }

    /// Return true if the tx register is empty (and can accept data)
    pub fn is_tx_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }

    /// Return true if the tx register is empty (and can accept data)
    #[deprecated(since = "0.10.0")]
    pub fn is_txe(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    #[deprecated(since = "0.10.0")]
    pub fn is_rxne(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    /// Clear idle line interrupt flag
    pub fn clear_idle_interrupt(&self) {
        unsafe {
            let _ = (*USART::ptr()).sr.read();
            let _ = (*USART::ptr()).dr.read();
        }
    }

    pub fn split(self) -> (Tx<USART, WORD>, Rx<USART, WORD>) {
        (self.tx, self.rx)
    }
}

impl<USART, PINS> Serial<USART, PINS, u8>
where
    USART: Instance,
{
    /// Converts this Serial into a version that can read and write `u16` values instead of `u8`s
    ///
    /// This can be used with a word length of 9 bits.
    pub fn with_u16_data(self) -> Serial<USART, PINS, u16> {
        Serial {
            usart: self.usart,
            pins: self.pins,
            tx: Tx::new(),
            rx: Rx::new(),
        }
    }
}

impl<USART, PINS> Serial<USART, PINS, u16>
where
    USART: Instance,
{
    /// Converts this Serial into a version that can read and write `u8` values instead of `u16`s
    ///
    /// This can be used with a word length of 8 bits.
    pub fn with_u8_data(self) -> Serial<USART, PINS, u8> {
        Serial {
            usart: self.usart,
            pins: self.pins,
            tx: Tx::new(),
            rx: Rx::new(),
        }
    }
}

impl<USART, PINS, WORD> serial::Read<WORD> for Serial<USART, PINS, WORD>
where
    USART: Instance,
    Rx<USART, WORD>: serial::Read<WORD, Error = Error>,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<WORD, Error> {
        self.rx.read()
    }
}

impl<USART> serial::Read<u8> for Rx<USART, u8>
where
    USART: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        // Delegate to the Read<u16> implementation, then truncate to 8 bits
        Rx::<USART, u16>::new().read().map(|word16| word16 as u8)
    }
}

/// Reads 9-bit words from the UART/USART
///
/// If the UART/USART was configured with `WordLength::DataBits9`, the returned value will contain
/// 9 received data bits and all other bits set to zero. Otherwise, the returned value will contain
/// 8 received data bits and all other bits set to zero.
impl<USART> serial::Read<u16> for Rx<USART, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u16, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        // Any error requires the dr to be read to clear
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*USART::ptr()).dr.read() };
        }

        Err(if sr.pe().bit_is_set() {
            Error::Parity.into()
        } else if sr.fe().bit_is_set() {
            Error::Framing.into()
        } else if sr.nf().bit_is_set() {
            Error::Noise.into()
        } else if sr.ore().bit_is_set() {
            Error::Overrun.into()
        } else if sr.rxne().bit_is_set() {
            // NOTE(unsafe) atomic read from stateless register
            return Ok(unsafe { &*USART::ptr() }.dr.read().dr().bits());
        } else {
            nb::Error::WouldBlock
        })
    }
}

unsafe impl<USART> PeriAddress for Rx<USART, u8>
where
    USART: Instance,
{
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*USART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

impl<USART, PINS, WORD> serial::Write<WORD> for Serial<USART, PINS, WORD>
where
    USART: Instance,
    Tx<USART, WORD>: serial::Write<WORD, Error = Error>,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
    }

    fn write(&mut self, byte: WORD) -> nb::Result<(), Self::Error> {
        self.tx.write(byte)
    }
}

unsafe impl<USART> PeriAddress for Tx<USART, u8>
where
    USART: Instance,
{
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*USART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

impl<USART> serial::Write<u8> for Tx<USART, u8>
where
    USART: Instance,
{
    type Error = Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // Delegate to u16 version
        Tx::<USART, u16>::new().write(u16::from(word))
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // Delegate to u16 version
        Tx::<USART, u16>::new().flush()
    }
}

/// Writes 9-bit words to the UART/USART
///
/// If the UART/USART was configured with `WordLength::DataBits9`, the 9 least significant bits will
/// be transmitted and the other 7 bits will be ignored. Otherwise, the 8 least significant bits
/// will be transmitted and the other 8 bits will be ignored.
impl<USART> serial::Write<u16> for Tx<USART, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            unsafe { &*USART::ptr() }.dr.write(|w| w.dr().bits(word));
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<USART> blocking::serial::Write<u16> for Tx<USART, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, buffer: &[u16]) -> Result<(), Self::Error> {
        for &b in buffer {
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
            match <Self as serial::Write<u16>>::flush(self) {
                Ok(()) => return Ok(()),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(err)) => return Err(err),
            }
        }
    }
}

impl<USART> blocking::serial::Write<u8> for Tx<USART, u8>
where
    USART: Instance,
{
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
            match <Self as serial::Write<u8>>::flush(self) {
                Ok(()) => return Ok(()),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(err)) => return Err(err),
            }
        }
    }
}

impl<USART, PINS> blocking::serial::Write<u16> for Serial<USART, PINS, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, bytes: &[u16]) -> Result<(), Self::Error> {
        self.tx.bwrite_all(bytes)
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        self.tx.bflush()
    }
}

impl<USART, PINS> blocking::serial::Write<u8> for Serial<USART, PINS, u8>
where
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.tx.bwrite_all(bytes)
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        self.tx.bflush()
    }
}

impl<USART, PINS, WORD> Serial<USART, PINS, WORD>
where
    USART: Instance,
{
    fn config_stop(self, config: config::Config) -> Self {
        self.usart.set_stopbits(config.stopbits);
        self
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
use crate::pac::uart4 as uart_base;

#[cfg(not(any(
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
)))]
use crate::pac::usart1 as uart_base;

// Implemented by all USART instances
pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + rcc::GetBusFreq {
    #[doc(hidden)]
    fn ptr() -> *const uart_base::RegisterBlock;
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
}

macro_rules! halUsart {
    ($USARTX:ty: ($usartX:ident)) => {
        impl Instance for $USARTX {
            fn ptr() -> *const uart_base::RegisterBlock {
                <$USARTX>::ptr() as *const _
            }

            fn set_stopbits(&self, bits: config::StopBits) {
                use crate::pac::usart1::cr2::STOP_A;
                use config::StopBits;

                self.cr2.write(|w| {
                    w.stop().variant(match bits {
                        StopBits::STOP0P5 => STOP_A::STOP0P5,
                        StopBits::STOP1 => STOP_A::STOP1,
                        StopBits::STOP1P5 => STOP_A::STOP1P5,
                        StopBits::STOP2 => STOP_A::STOP2,
                    })
                });
            }
        }

        impl<USART, TX, RX, const TXA: u8, const RXA: u8> Serial<USART, (TX, RX)>
        where
            TX: PinTx<USART, A = Const<TXA>> + SetAlternate<TXA>,
            RX: PinRx<USART, A = Const<RXA>> + SetAlternate<RXA>,
            USART: Instance,
        {
            #[deprecated(since = "0.10.0")]
            pub fn $usartX(
                usart: USART,
                pins: (TX, RX),
                config: config::Config,
                clocks: Clocks,
            ) -> Result<Self, config::InvalidConfig> {
                Self::new(usart, pins, config, clocks)
            }
        }
    };
}

// TODO: fix stm32f413 UARTs
#[cfg(any(
    feature = "uart4",
    feature = "uart5",
    feature = "uart7",
    feature = "uart8",
    feature = "uart9",
    feature = "uart10"
))]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423",)))]
macro_rules! halUart {
    ($USARTX:ty: ($usartX:ident)) => {
        impl Instance for $USARTX {
            fn ptr() -> *const uart_base::RegisterBlock {
                <$USARTX>::ptr() as *const _
            }

            fn set_stopbits(&self, bits: config::StopBits) {
                use crate::pac::uart4::cr2::STOP_A;
                use config::StopBits;

                self.cr2.write(|w| {
                    w.stop().variant(match bits {
                        StopBits::STOP0P5 => STOP_A::STOP1,
                        StopBits::STOP1 => STOP_A::STOP1,
                        StopBits::STOP1P5 => STOP_A::STOP2,
                        StopBits::STOP2 => STOP_A::STOP2,
                    })
                });
            }
        }

        impl<USART, TX, RX, const TXA: u8, const RXA: u8> Serial<USART, (TX, RX)>
        where
            TX: PinTx<USART, A = Const<TXA>> + SetAlternate<TXA>,
            RX: PinRx<USART, A = Const<RXA>> + SetAlternate<RXA>,
            USART: Instance,
        {
            #[deprecated(since = "0.10.0")]
            pub fn $usartX(
                usart: USART,
                pins: (TX, RX),
                config: config::Config,
                clocks: Clocks,
            ) -> Result<Self, config::InvalidConfig> {
                Self::new(usart, pins, config, clocks)
            }
        }
    };
}

halUsart! { USART1: (usart1) }
halUsart! { USART2: (usart2) }
halUsart! { USART6: (usart6) }

#[cfg(feature = "usart3")]
halUsart! { USART3: (usart3) }

#[cfg(feature = "uart4")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { UART4: (uart4) }
#[cfg(feature = "uart5")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { UART5: (uart5) }

#[cfg(feature = "uart4")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! { UART4: (uart4) }
#[cfg(feature = "uart5")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! { UART5: (uart5) }

#[cfg(feature = "uart7")]
halUsart! { UART7: (uart7) }
#[cfg(feature = "uart8")]
halUsart! { UART8: (uart8) }
#[cfg(feature = "uart9")]
halUsart! { UART9: (uart9) }
#[cfg(feature = "uart10")]
halUsart! { UART10: (uart10) }

impl<USART, PINS> fmt::Write for Serial<USART, PINS>
where
    Tx<USART>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<USART> fmt::Write for Tx<USART>
where
    Tx<USART>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| block!(self.write(c)))
            .map_err(|_| fmt::Error)
    }
}
