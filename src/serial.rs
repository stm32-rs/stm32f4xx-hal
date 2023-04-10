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
mod hal_1;

use crate::gpio;

use crate::pac::{self, RCC};

use crate::gpio::NoPin;
use crate::rcc::Clocks;

use crate::dma::traits::PeriAddress;

/// Serial error
pub use embedded_hal_one::serial::ErrorKind as Error;

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
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WordLength {
        DataBits8,
        DataBits9,
    }

    /// Parity generation and checking. If odd or even parity is selected, the
    /// underlying USART will be configured to send/receive the parity bit in
    /// addtion to the data bits.
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Wrapper around `STOP_A`
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DmaConfig {
        None,
        Tx,
        Rx,
        TxRx,
    }

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        pub fn dma(mut self, dma: DmaConfig) -> Self {
            self.dma = dma;
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

pub use config::Config;

/// A filler type for when the Tx pin is unnecessary
pub type NoTx = NoPin;
/// A filler type for when the Rx pin is unnecessary
pub type NoRx = NoPin;

/// Serial abstraction
pub struct Serial<USART: Instance, WORD = u8> {
    tx: Tx<USART, WORD>,
    rx: Rx<USART, WORD>,
}

impl<USART: Instance, WORD> Rx<USART, WORD> {
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
}

impl<USART: Instance> Rx<USART, u8> {
    fn with_u16_data(self) -> Rx<USART, u16> {
        Rx::new(self.pin)
    }
}

impl<USART: Instance> Rx<USART, u16> {
    fn with_u8_data(self) -> Rx<USART, u8> {
        Rx::new(self.pin)
    }
}

/// Trait for [`Rx`] interrupt handling.
pub trait RxISR {
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool;

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool;

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self);
}

impl<USART: Instance, WORD> RxISR for Rx<USART, WORD> {
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().idle().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self) {
        unsafe {
            let _ = (*USART::ptr()).sr.read();
            let _ = (*USART::ptr()).dr.read();
        }
    }
}

impl<USART: Instance, WORD> Tx<USART, WORD> {
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
}

impl<USART: Instance> Tx<USART, u8> {
    fn with_u16_data(self) -> Tx<USART, u16> {
        Tx::new(self.usart, self.pin)
    }
}

impl<USART: Instance> Tx<USART, u16> {
    fn with_u8_data(self) -> Tx<USART, u8> {
        Tx::new(self.usart, self.pin)
    }
}

/// Trait for [`Tx`] interrupt handling.
pub trait TxISR {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool;
}

impl<USART: Instance, WORD> TxISR for Tx<USART, WORD> {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }
}

impl<USART: Instance, WORD> AsRef<Tx<USART, WORD>> for Serial<USART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<USART, WORD> {
        &self.tx
    }
}

impl<USART: Instance, WORD> AsRef<Rx<USART, WORD>> for Serial<USART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<USART, WORD> {
        &self.rx
    }
}

impl<USART: Instance, WORD> AsMut<Tx<USART, WORD>> for Serial<USART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<USART, WORD> {
        &mut self.tx
    }
}

impl<USART: Instance, WORD> AsMut<Rx<USART, WORD>> for Serial<USART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<USART, WORD> {
        &mut self.rx
    }
}

/// Serial receiver containing RX pin
pub struct Rx<USART: Instance, WORD = u8> {
    _word: PhantomData<(USART, WORD)>,
    pin: USART::RxPin,
}

/// Serial transmitter containing TX pin
pub struct Tx<USART: Instance, WORD = u8> {
    _word: PhantomData<WORD>,
    usart: USART,
    pin: USART::TxPin,
}

impl<USART: Instance, WORD> Rx<USART, WORD> {
    fn new(pin: USART::RxPin) -> Self {
        Self {
            _word: PhantomData,
            pin,
        }
    }

    pub fn join<TX>(self, tx: Tx<USART, WORD>) -> Serial<USART, WORD> {
        Serial { tx, rx: self }
    }
}

impl<USART: Instance, WORD> Tx<USART, WORD> {
    fn new(usart: USART, pin: USART::TxPin) -> Self {
        Self {
            _word: PhantomData,
            usart,
            pin,
        }
    }

    pub fn join(self, rx: Rx<USART, WORD>) -> Serial<USART, WORD> {
        Serial { tx: self, rx }
    }
}

pub trait SerialExt: Sized + Instance {
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::TxPin>, impl Into<Self::RxPin>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig>;

    fn tx<WORD>(
        self,
        tx_pin: impl Into<Self::TxPin>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::RxPin>;

    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::RxPin>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::TxPin>;
}

impl<USART: Instance> SerialExt for USART {
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::TxPin>, impl Into<Self::RxPin>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig> {
        Serial::new(self, pins, config, clocks)
    }
    fn tx<WORD>(
        self,
        tx_pin: impl Into<Self::TxPin>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::RxPin>,
    {
        Serial::tx(self, tx_pin, config, clocks)
    }
    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::RxPin>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::TxPin>,
    {
        Serial::rx(self, rx_pin, config, clocks)
    }
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    /*
        StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART

        STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
    */
    pub fn new(
        usart: USART,
        pins: (impl Into<USART::TxPin>, impl Into<USART::RxPin>),
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

        let pclk_freq = USART::clock(clocks).raw();
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

        Ok(Serial {
            tx: Tx::new(usart, pins.0.into()),
            rx: Rx::new(pins.1.into()),
        }
        .config_stop(config))
    }

    pub fn release<TX, RX, E>(self) -> Result<(USART, (TX, RX)), E>
    where
        TX: TryFrom<USART::TxPin, Error = E>,
        RX: TryFrom<USART::RxPin, Error = E>,
    {
        Ok((
            self.tx.usart,
            (self.tx.pin.try_into()?, self.rx.pin.try_into()?),
        ))
    }
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    pub fn tx(
        usart: USART,
        tx_pin: impl Into<USART::TxPin>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<USART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<USART::RxPin>,
    {
        Self::new(usart, (tx_pin, NoPin), config, clocks).map(|s| s.split().0)
    }
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    pub fn rx(
        usart: USART,
        rx_pin: impl Into<USART::RxPin>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<USART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<USART::TxPin>,
    {
        Self::new(usart, (NoPin, rx_pin), config, clocks).map(|s| s.split().1)
    }
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
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

    pub fn split(self) -> (Tx<USART, WORD>, Rx<USART, WORD>) {
        (self.tx, self.rx)
    }
}

impl<USART: Instance, WORD> RxISR for Serial<USART, WORD> {
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool {
        self.rx.is_idle()
    }

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool {
        self.rx.is_rx_not_empty()
    }

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self) {
        self.rx.clear_idle_interrupt();
    }
}

impl<USART: Instance, WORD> TxISR for Serial<USART, WORD> {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }
}

impl<USART: Instance> Serial<USART, u8> {
    /// Converts this Serial into a version that can read and write `u16` values instead of `u8`s
    ///
    /// This can be used with a word length of 9 bits.
    pub fn with_u16_data(self) -> Serial<USART, u16> {
        Serial {
            tx: self.tx.with_u16_data(),
            rx: self.rx.with_u16_data(),
        }
    }
}

impl<USART: Instance> Serial<USART, u16> {
    /// Converts this Serial into a version that can read and write `u8` values instead of `u16`s
    ///
    /// This can be used with a word length of 8 bits.
    pub fn with_u8_data(self) -> Serial<USART, u8> {
        Serial {
            tx: self.tx.with_u8_data(),
            rx: self.rx.with_u8_data(),
        }
    }
}

unsafe impl<USART: Instance> PeriAddress for Rx<USART, u8> {
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*USART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

unsafe impl<USART: Instance> PeriAddress for Tx<USART, u8> {
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*USART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    fn config_stop(self, config: config::Config) -> Self {
        self.tx.usart.set_stopbits(config.stopbits);
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
    type TxPin;
    type RxPin;

    #[doc(hidden)]
    fn ptr() -> *const uart_base::RegisterBlock;
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
}

macro_rules! halUsart {
    ($USART:ty, $usart:ident, $Serial:ident, $Tx:ident, $Rx:ident) => {
        pub type $Serial<WORD = u8> = Serial<$USART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$USART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$USART, WORD>;

        impl Instance for $USART {
            type TxPin = gpio::alt::$usart::Tx;
            type RxPin = gpio::alt::$usart::Rx;

            fn ptr() -> *const uart_base::RegisterBlock {
                <$USART>::ptr() as *const _
            }

            fn set_stopbits(&self, bits: config::StopBits) {
                use crate::pac::usart1::cr2::STOP_A;
                use config::StopBits;

                self.cr2.write(|w| {
                    w.stop().variant(match bits {
                        StopBits::STOP0P5 => STOP_A::Stop0p5,
                        StopBits::STOP1 => STOP_A::Stop1,
                        StopBits::STOP1P5 => STOP_A::Stop1p5,
                        StopBits::STOP2 => STOP_A::Stop2,
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
    ($USART:ty, $usart:ident, $Serial:ident, $Tx:ident, $Rx:ident) => {
        pub type $Serial<WORD = u8> = Serial<$USART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$USART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$USART, WORD>;

        impl Instance for $USART {
            type TxPin = gpio::alt::$usart::Tx;
            type RxPin = gpio::alt::$usart::Rx;

            fn ptr() -> *const uart_base::RegisterBlock {
                <$USART>::ptr() as *const _
            }

            fn set_stopbits(&self, bits: config::StopBits) {
                use crate::pac::uart4::cr2::STOP_A;
                use config::StopBits;

                self.cr2.write(|w| {
                    w.stop().variant(match bits {
                        StopBits::STOP0P5 => STOP_A::Stop1,
                        StopBits::STOP1 => STOP_A::Stop1,
                        StopBits::STOP1P5 => STOP_A::Stop2,
                        StopBits::STOP2 => STOP_A::Stop2,
                    })
                });
            }
        }
    };
}

halUsart! { pac::USART1, usart1, Serial1, Rx1, Tx1 }
halUsart! { pac::USART2, usart2, Serial2, Rx2, Tx2 }
halUsart! { pac::USART6, usart6, Serial6, Rx6, Tx6 }

#[cfg(feature = "usart3")]
halUsart! { pac::USART3, usart3, Serial3, Rx3, Tx3 }

#[cfg(feature = "uart4")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { pac::UART4, uart4, Serial4, Rx4, Tx4 }
#[cfg(feature = "uart5")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { pac::UART5, uart5, Serial5, Rx5, Tx5 }

//#[cfg(feature = "uart4")]
//#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
//halUsart! { pac::UART4, uart4, Serial4, Rx4, Tx4 }
#[cfg(feature = "uart5")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
halUsart! { pac::UART5, uart5, Serial5, Rx5, Tx5 }

#[cfg(feature = "uart7")]
halUsart! { pac::UART7, uart7, Serial7, Rx7, Tx7 }
#[cfg(feature = "uart8")]
halUsart! { pac::UART8, uart8, Serial8, Rx8, Tx8 }
#[cfg(feature = "uart9")]
halUsart! { pac::UART9, uart9, Serial9, Rx9, Tx9 }
#[cfg(feature = "uart10")]
halUsart! { pac::UART10, uart10, Serial10, Rx10, Tx10 }

impl<USART: Instance> fmt::Write for Serial<USART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<USART: Instance> fmt::Write for Tx<USART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| block!(self.write(c)))
            .map_err(|_| fmt::Error)
    }
}

impl<USART: Instance> Rx<USART, u8> {
    fn read(&mut self) -> nb::Result<u8, Error> {
        // Delegate to the Read<u16> implementation, then truncate to 8 bits
        unsafe {
            (&mut *(self as *mut Self as *mut Rx<USART, u16>))
                .read()
                .map(|word16| word16 as u8)
        }
    }
}

impl<USART: Instance> Rx<USART, u16> {
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
            Error::FrameFormat.into()
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

impl<USART: Instance> Tx<USART, u8> {
    fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        // Delegate to u16 version
        unsafe { (&mut *(self as *mut Self as *mut Tx<USART, u16>)).write(u16::from(word)) }
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        // Delegate to u16 version
        unsafe { (&mut *(self as *mut Self as *mut Tx<USART, u16>)).flush() }
    }

    fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for &b in bytes {
            nb::block!(self.write(b))?;
        }
        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Error> {
        nb::block!(self.flush())
    }
}

impl<USART: Instance> Tx<USART, u16> {
    fn write(&mut self, word: u16) -> nb::Result<(), Error> {
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

    fn flush(&mut self) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn bwrite_all(&mut self, buffer: &[u16]) -> Result<(), Error> {
        for &b in buffer {
            nb::block!(self.write(b))?;
        }
        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Error> {
        nb::block!(self.flush())
    }
}
