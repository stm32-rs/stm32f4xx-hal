//!
//! Asynchronous serial communication using USART peripherals
//!
//! # Word length
//!
//! By default, the UART/USART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
//! the embedded-hal read and write traits with `u8` as the word type.
//!
//! You can also configure the hardware to use 9 data bits with the `Config` `wordlength_9()`
//! function. After creating a `Serial` with this option, use the `with_u16_data()` function to
//! convert the `Serial<_, u8>` object into a `Serial<_, u16>` that can send and receive `u16`s.
//!
//! In this mode, the `Serial<_, u16>`, `Rx<_, u16>`, and `Tx<_, u16>` structs instead implement
//! the embedded-hal read and write traits with `u16` as the word type. You can use these
//! implementations for 9-bit words.

use core::fmt;
use enumflags2::BitFlags;

#[allow(unused)]
use crate::pacext::uart::UartRB;
mod hal_02;
mod hal_1;

mod uart_impls;
use uart_impls::RegisterBlockImpl;

use crate::gpio::{self, PushPull};

use crate::pac;

use crate::rcc::{self, Rcc};

pub mod dma;
use crate::dma::{
    traits::{DMASet, PeriAddress},
    MemoryToPeripheral, PeripheralToMemory,
};

/// Serial error kind
///
/// This represents a common set of serial operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common serial errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum Error {
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// Received data does not conform to the peripheral configuration.
    /// Can be caused by a misconfigured device on either end of the serial line.
    FrameFormat,
    /// Parity check failed.
    Parity,
    /// Serial line is too noisy to read valid data.
    Noise,
    /// A different error occurred. The original error may contain more information.
    Other,
}

/// UART interrupt events
#[enumflags2::bitflags]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum Event {
    /// IDLE interrupt enable
    Idle = 1 << 4,
    /// RXNE interrupt enable
    RxNotEmpty = 1 << 5,
    /// Transmission complete interrupt enable
    TransmissionComplete = 1 << 6,
    /// TXE interrupt enable
    TxEmpty = 1 << 7,
    /// PE interrupt enable
    ParityError = 1 << 8,
}

/// UART/USART status flags
#[enumflags2::bitflags]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum Flag {
    /// Parity error
    ParityError = 1 << 0,
    /// Framing error
    FramingError = 1 << 1,
    /// Noise detected flag
    Noise = 1 << 2,
    /// Overrun error
    Overrun = 1 << 3,
    /// IDLE line detected
    Idle = 1 << 4,
    /// Read data register not empty
    RxNotEmpty = 1 << 5,
    /// Transmission complete
    TransmissionComplete = 1 << 6,
    /// Transmit data register empty
    TxEmpty = 1 << 7,
    /// LIN break detection flag
    LinBreak = 1 << 8,
    /// CTS flag
    Cts = 1 << 9,
}

/// UART clearable flags
#[enumflags2::bitflags]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum CFlag {
    /// Read data register not empty
    RxNotEmpty = 1 << 5,
    /// Transmission complete
    TransmissionComplete = 1 << 6,
    /// LIN break detection flag
    LinBreak = 1 << 8,
}

pub mod config;

pub use config::Config;

pub use gpio::alt::SerialAsync as CommonPins;

// Implemented by all USART/UART instances
pub trait Instance:
    crate::Sealed
    + crate::Ptr<RB: RegisterBlockImpl>
    + crate::Steal
    + core::ops::Deref<Target = Self::RB>
    + rcc::Enable
    + rcc::Reset
    + rcc::BusClock
    + CommonPins
{
    #[doc(hidden)]
    #[inline(always)]
    fn peri_address() -> u32 {
        unsafe { &*Self::PTR }.peri_address()
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

/// Trait for [`Tx`] interrupt handling.
pub trait TxISR {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool;
}

/// Trait for listening [`Rx`] interrupt events.
pub trait RxListen {
    /// Start listening for an rx not empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen(&mut self);

    /// Stop listening for the rx not empty interrupt event
    fn unlisten(&mut self);

    /// Start listening for a line idle interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen_idle(&mut self);

    /// Stop listening for the line idle interrupt event
    fn unlisten_idle(&mut self);
}

/// Trait for listening [`Tx`] interrupt event.
pub trait TxListen {
    /// Start listening for a tx empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen(&mut self);

    /// Stop listening for the tx empty interrupt event
    fn unlisten(&mut self);
}

/// Serial abstraction
pub struct Serial<USART: CommonPins> {
    tx: Tx<USART>,
    rx: Rx<USART>,
}

/// Serial receiver containing RX pin
pub struct Rx<USART: CommonPins> {
    usart: USART,
    pin: Option<USART::Rx<PushPull>>,
}

/// Serial transmitter containing TX pin
pub struct Tx<USART: CommonPins> {
    usart: USART,
    pin: Option<USART::Tx<PushPull>>,
}

pub trait SerialExt: Sized + Instance {
    fn serial(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Serial<Self>, config::InvalidConfig>;

    fn tx(
        self,
        tx_pin: impl Into<Self::Tx<PushPull>>,
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Tx<Self>, config::InvalidConfig>;

    fn rx(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Rx<Self>, config::InvalidConfig>;
}

impl<USART: Instance> Serial<USART> {
    pub fn new(
        uart: USART,
        pins: (
            impl Into<USART::Tx<PushPull>>,
            impl Into<USART::Rx<PushPull>>,
        ),
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Self, config::InvalidConfig> {
        Self::_new(uart, (Some(pins.0), Some(pins.1)), config, rcc)
    }
    fn _new(
        uart: USART,
        pins: (
            Option<impl Into<USART::Tx<PushPull>>>,
            Option<impl Into<USART::Rx<PushPull>>>,
        ),
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Self, config::InvalidConfig> {
        use self::config::*;

        let config = config.into();
        // Enable clock.
        USART::enable(rcc);
        USART::reset(rcc);

        let pclk_freq = USART::clock(&rcc.clocks).raw();
        let baud = config.baudrate.0;

        if !USART::RB::IRDA && config.irda != IrdaMode::None {
            return Err(config::InvalidConfig);
        }

        let (over8, div) = if config.irda != IrdaMode::None {
            let div = (pclk_freq + (baud / 2)) / baud;
            (false, div)
        } else {
            calculate_brr(pclk_freq, baud)?
        };

        uart.brr().write(|w| unsafe { w.bits(div as u16) });

        // Reset other registers to disable advanced USART features
        uart.cr2().reset();
        uart.cr3().reset();
        // IrDA configuration - see STM32F411xC/E (RM0383) sections:
        // 19.3.12 "IrDA SIR ENDEC block"
        // 19.6.7 "Guard time and prescaler register (USART_GTPR)"
        if config.irda != IrdaMode::None && config.stopbits != StopBits::STOP1 {
            return Err(config::InvalidConfig);
        }

        uart.configure_irda(config.irda, pclk_freq);

        // Enable transmission and receiving
        // and configure frame

        uart.cr1().write(|w| {
            w.ue().set_bit();
            w.over8().bit(over8);
            w.te().set_bit();
            w.re().set_bit();
            w.m().bit(config.wordlength == WordLength::DataBits9);
            w.pce().bit(config.parity != Parity::ParityNone);
            w.ps().bit(config.parity == Parity::ParityOdd)
        });

        uart.enable_dma(config.dma);

        let serial = Serial {
            tx: Tx::new(uart, pins.0.map(Into::into)),
            rx: Rx::new(unsafe { USART::steal() }, pins.1.map(Into::into)),
        };
        serial.tx.usart.set_stopbits(config.stopbits);
        Ok(serial)
    }
}

fn calculate_brr(pclk_freq: u32, baud: u32) -> Result<(bool, u32), config::InvalidConfig> {
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
    if (pclk_freq / 16) >= baud {
        // We have the ability to oversample to 16 bits, take
        // advantage of it.
        //
        // We also add `baud / 2` to the `pclk_freq` to ensure
        // rounding of values to the closest scale, rather than the
        // floored behavior of normal integer division.
        let div = (pclk_freq + (baud / 2)) / baud;
        Ok((false, div))
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
        Ok((true, div))
    } else {
        Err(config::InvalidConfig)
    }
}

impl<UART: CommonPins> Serial<UART> {
    pub fn split(self) -> (Tx<UART>, Rx<UART>) {
        (self.tx, self.rx)
    }

    #[allow(clippy::type_complexity)]
    pub fn release(
        self,
    ) -> (
        UART,
        (Option<UART::Tx<PushPull>>, Option<UART::Rx<PushPull>>),
    ) {
        (self.tx.usart, (self.tx.pin, self.rx.pin))
    }
}

macro_rules! halUsart {
    ($USART:ty, $Serial:ident, $Rx:ident, $Tx:ident) => {
        pub type $Serial = Serial<$USART>;
        pub type $Tx = Tx<$USART>;
        pub type $Rx = Rx<$USART>;

        impl Instance for $USART {}
    };
}
pub(crate) use halUsart;

halUsart! { pac::USART1, Serial1, Rx1, Tx1 }
halUsart! { pac::USART2, Serial2, Rx2, Tx2 }
halUsart! { pac::USART6, Serial6, Rx6, Tx6 }

#[cfg(feature = "usart3")]
halUsart! { pac::USART3, Serial3, Rx3, Tx3 }

#[cfg(feature = "uart4")]
macro_rules! halUart {
    ($UART:ty, $Serial:ident, $Rx:ident, $Tx:ident) => {
        pub type $Serial = Serial<$UART>;
        pub type $Tx = Tx<$UART>;
        pub type $Rx = Rx<$UART>;

        impl Instance for $UART {}
    };
}

#[cfg(feature = "uart4")]
halUart! { pac::UART4, Serial4, Rx4, Tx4 }
#[cfg(feature = "uart5")]
halUart! { pac::UART5, Serial5, Rx5, Tx5 }
#[cfg(feature = "uart7")]
halUart! { pac::UART7, Serial7, Rx7, Tx7 }
#[cfg(feature = "uart8")]
halUart! { pac::UART8, Serial8, Rx8, Tx8 }
#[cfg(feature = "uart9")]
halUart! { pac::UART9, Serial9, Rx9, Tx9 }
#[cfg(feature = "uart10")]
halUart! { pac::UART10, Serial10, Rx10, Tx10 }

impl<UART: CommonPins> Rx<UART> {
    pub(crate) fn new(usart: UART, pin: Option<UART::Rx<PushPull>>) -> Self {
        Self { usart, pin }
    }

    pub fn join(self, tx: Tx<UART>) -> Serial<UART> {
        Serial { tx, rx: self }
    }
}

impl<UART: CommonPins> Tx<UART> {
    pub(crate) fn new(usart: UART, pin: Option<UART::Tx<PushPull>>) -> Self {
        Self { usart, pin }
    }

    pub fn join(self, rx: Rx<UART>) -> Serial<UART> {
        Serial { tx: self, rx }
    }
}

impl<UART: Instance> AsRef<Tx<UART>> for Serial<UART> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<UART> {
        &self.tx
    }
}

impl<UART: Instance> AsRef<Rx<UART>> for Serial<UART> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<UART> {
        &self.rx
    }
}

impl<UART: Instance> AsMut<Tx<UART>> for Serial<UART> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<UART> {
        &mut self.tx
    }
}

impl<UART: Instance> AsMut<Rx<UART>> for Serial<UART> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<UART> {
        &mut self.rx
    }
}

impl<UART: Instance> RxISR for Serial<UART>
where
    Rx<UART>: RxISR,
{
    fn is_idle(&self) -> bool {
        self.rx.is_idle()
    }

    fn is_rx_not_empty(&self) -> bool {
        self.rx.is_rx_not_empty()
    }

    /// This clears `Idle`, `Overrun`, `Noise`, `FrameError` and `ParityError` flags
    fn clear_idle_interrupt(&self) {
        self.rx.clear_idle_interrupt();
    }
}

impl<UART: Instance> RxISR for Rx<UART> {
    fn is_idle(&self) -> bool {
        self.usart.is_idle()
    }

    fn is_rx_not_empty(&self) -> bool {
        self.usart.is_rx_not_empty()
    }

    /// This clears `Idle`, `Overrun`, `Noise`, `FrameError` and `ParityError` flags
    fn clear_idle_interrupt(&self) {
        self.usart.clear_idle_interrupt();
    }
}

impl<UART: Instance> TxISR for Serial<UART>
where
    Tx<UART>: TxISR,
{
    fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }
}

impl<UART: Instance> TxISR for Tx<UART> {
    fn is_tx_empty(&self) -> bool {
        self.usart.is_tx_empty()
    }
}

impl<UART: Instance> RxListen for Rx<UART> {
    fn listen(&mut self) {
        self.usart.listen_rxne()
    }

    fn unlisten(&mut self) {
        self.usart.unlisten_rxne()
    }

    fn listen_idle(&mut self) {
        self.usart.listen_idle()
    }

    fn unlisten_idle(&mut self) {
        self.usart.unlisten_idle()
    }
}

impl<UART: Instance> TxListen for Tx<UART> {
    fn listen(&mut self) {
        self.usart.listen_txe()
    }

    fn unlisten(&mut self) {
        self.usart.unlisten_txe()
    }
}

impl<UART: Instance> crate::ClearFlags for Serial<UART> {
    type Flag = CFlag;

    #[inline(always)]
    fn clear_flags(&mut self, flags: impl Into<BitFlags<Self::Flag>>) {
        self.tx.usart.clear_flags(flags.into())
    }
}

impl<UART: Instance> crate::ReadFlags for Serial<UART> {
    type Flag = Flag;

    #[inline(always)]
    fn flags(&self) -> BitFlags<Self::Flag> {
        self.tx.usart.flags()
    }
}

impl<UART: Instance> crate::Listen for Serial<UART> {
    type Event = Event;

    #[inline(always)]
    fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tx.usart.listen_event(None, Some(event.into()));
    }

    #[inline(always)]
    fn listen_only(&mut self, event: impl Into<BitFlags<Self::Event>>) {
        self.tx
            .usart
            .listen_event(Some(BitFlags::ALL), Some(event.into()));
    }

    #[inline(always)]
    fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tx.usart.listen_event(Some(event.into()), None);
    }
}

impl<UART: Instance> fmt::Write for Serial<UART>
where
    Tx<UART>: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<UART: Instance> fmt::Write for Tx<UART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| nb::block!(self.usart.write_u8(c)))
            .map_err(|_| fmt::Error)
    }
}

impl<UART: Instance> SerialExt for UART {
    fn serial(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Serial<Self>, config::InvalidConfig> {
        Serial::new(self, pins, config, rcc)
    }
    fn tx(
        self,
        tx_pin: impl Into<Self::Tx<PushPull>>,
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Tx<Self>, config::InvalidConfig> {
        Serial::tx(self, tx_pin, config, rcc)
    }
    fn rx(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Rx<Self>, config::InvalidConfig> {
        Serial::rx(self, rx_pin, config, rcc)
    }
}

impl<UART: Instance> Serial<UART> {
    pub fn tx(
        usart: UART,
        tx_pin: impl Into<UART::Tx<PushPull>>,
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Tx<UART>, config::InvalidConfig> {
        Self::_new(
            usart,
            (Some(tx_pin), None::<UART::Rx<PushPull>>),
            config,
            rcc,
        )
        .map(|s| s.split().0)
    }
}

impl<UART: Instance> Serial<UART> {
    pub fn rx(
        usart: UART,
        rx_pin: impl Into<UART::Rx<PushPull>>,
        config: impl Into<config::Config>,
        rcc: &mut Rcc,
    ) -> Result<Rx<UART>, config::InvalidConfig> {
        Self::_new(
            usart,
            (None::<UART::Tx<PushPull>>, Some(rx_pin)),
            config,
            rcc,
        )
        .map(|s| s.split().1)
    }
}

unsafe impl<UART: Instance> PeriAddress for Rx<UART> {
    #[inline(always)]
    fn address(&self) -> u32 {
        self.usart.peri_address()
    }

    type MemSize = u8;
}

unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory>
    for Rx<UART>
where
    UART: DMASet<STREAM, CHANNEL, PeripheralToMemory>,
{
}

unsafe impl<UART: Instance> PeriAddress for Tx<UART> {
    #[inline(always)]
    fn address(&self) -> u32 {
        self.usart.peri_address()
    }

    type MemSize = u8;
}

unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral>
    for Tx<UART>
where
    UART: DMASet<STREAM, CHANNEL, MemoryToPeripheral>,
{
}
