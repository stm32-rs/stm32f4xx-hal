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
use nb::block;

mod hal_02;

use crate::gpio::{Const, PinA, PushPull, SetAlternate};

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

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum WordLength {
        DataBits8,
        DataBits9,
    }

    /// Parity generation and checking. If odd or even parity is selected, the
    /// underlying USART will be configured to send/receive the parity bit in
    /// addtion to the data bits.
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Parity {
        /// No parity bit will be added/checked.
        ParityNone,
        /// The MSB transmitted/received will be generated/checked to have a
        /// even number of bits set.
        ParityEven,
        /// The MSB transmitted/received will be generated/checked to have a
        /// odd number of bits set.
        ParityOdd,
    }

    /// Stop Bit configuration parameter for serial.
    ///
    /// Wrapper around [`STOP_A`]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy, Debug, PartialEq)]
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

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum DmaConfig {
        None,
        Tx,
        Rx,
        TxRx,
    }

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[non_exhaustive]
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

    impl<T: Into<Bps>> From<T> for Config {
        fn from(b: T) -> Config {
            Config {
                baudrate: b.into(),
                ..Default::default()
            }
        }
    }
}

pub struct TxPin;
impl crate::Sealed for TxPin {}
pub struct RxPin;
impl crate::Sealed for RxPin {}

pub trait Pins<USART> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}
impl<USART, TX, RX, const TXA: u8, const RXA: u8> Pins<USART> for (TX, RX)
where
    TX: PinA<TxPin, USART, A = Const<TXA>> + SetAlternate<PushPull, TXA>,
    RX: PinA<RxPin, USART, A = Const<RXA>> + SetAlternate<PushPull, RXA>,
{
    fn set_alt_mode(&mut self) {
        self.0.set_alt_mode();
        self.1.set_alt_mode();
    }
    fn restore_mode(&mut self) {
        self.0.restore_mode();
        self.1.restore_mode();
    }
}

/// A filler type for when the Tx pin is unnecessary
pub type NoTx = NoPin;
/// A filler type for when the Rx pin is unnecessary
pub type NoRx = NoPin;

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

impl<USART, PINS, WORD> Serial<USART, PINS, WORD>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    /*
        StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART

        STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
    */
    pub fn new(
        usart: USART,
        mut pins: PINS,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Self, config::InvalidConfig> {
        use self::config::*;

        let config = config.into();
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable clock.
            USART::enable(rcc);
            USART::reset(rcc);
        }

        let pclk_freq = USART::clock(clocks).0;
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

        pins.set_alt_mode();

        Ok(Serial {
            usart,
            pins,
            tx: Tx::new(),
            rx: Rx::new(),
        }
        .config_stop(config))
    }
    pub fn release(mut self) -> (USART, PINS) {
        self.pins.restore_mode();

        (self.usart, self.pins)
    }
}

impl<USART, TX, WORD, const TXA: u8> Serial<USART, (TX, NoPin), WORD>
where
    TX: PinA<TxPin, USART, A = Const<TXA>> + SetAlternate<PushPull, TXA>,
    USART: Instance,
{
    pub fn tx(
        usart: USART,
        tx_pin: TX,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<USART, WORD>, config::InvalidConfig> {
        Self::new(usart, (tx_pin, NoPin), config, clocks).map(|s| s.split().0)
    }
}

impl<USART, RX, WORD, const RXA: u8> Serial<USART, (NoPin, RX), WORD>
where
    RX: PinA<RxPin, USART, A = Const<RXA>> + SetAlternate<PushPull, RXA>,
    USART: Instance,
{
    pub fn rx(
        usart: USART,
        rx_pin: RX,
        config: impl Into<config::Config>,
        clocks: &Clocks,
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
pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + rcc::BusClock {
    #[doc(hidden)]
    fn ptr() -> *const uart_base::RegisterBlock;
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
}

macro_rules! halUsart {
    ($USARTX:ty) => {
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
    ($USARTX:ty) => {
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
    };
}

halUsart! { USART1 }
halUsart! { USART2 }
halUsart! { USART6 }

#[cfg(feature = "usart3")]
halUsart! { USART3 }

#[cfg(feature = "uart4")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { UART4 }
#[cfg(feature = "uart5")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { UART5 }

#[cfg(feature = "uart4")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! { UART4 }
#[cfg(feature = "uart5")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! { UART5 }

#[cfg(feature = "uart7")]
halUsart! { UART7 }
#[cfg(feature = "uart8")]
halUsart! { UART8 }
#[cfg(feature = "uart9")]
halUsart! { UART9 }
#[cfg(feature = "uart10")]
halUsart! { UART10 }

impl<USART, PINS> fmt::Write for Serial<USART, PINS>
where
    Tx<USART>: embedded_hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<USART> fmt::Write for Tx<USART>
where
    Tx<USART>: embedded_hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use embedded_hal::serial::Write;
        s.bytes()
            .try_for_each(|c| block!(self.write(c)))
            .map_err(|_| fmt::Error)
    }
}
