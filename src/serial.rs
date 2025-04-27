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
use core::marker::PhantomData;
use enumflags2::BitFlags;

use crate::pacext::uart::UartRB;
mod hal_02;
mod hal_1;

mod uart_impls;
use uart_impls::RegisterBlockImpl;

use crate::gpio::{self, PushPull};

use crate::pac;

use crate::gpio::NoPin;
use crate::rcc::{self, Clocks};

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
    /// Clear to send flag
    Cts = 1 << 9,
}

pub mod config;

pub use config::Config;

/// A filler type for when the Tx pin is unnecessary
pub use gpio::NoPin as NoTx;
/// A filler type for when the Rx pin is unnecessary
pub use gpio::NoPin as NoRx;

pub use gpio::alt::{SerialAsync as CommonPins, SerialFlowControl};

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
        unsafe { &*Self::ptr() }.peri_address()
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
pub struct Serial<USART: CommonPins, WORD = u8> {
    tx: Tx<USART, WORD>,
    rx: Rx<USART, WORD>,
}

/// Serial receiver containing RX pin
pub struct Rx<USART: CommonPins, WORD = u8> {
    _word: PhantomData<WORD>,
    usart: USART,
    pin: USART::Rx<PushPull>,
}

/// Serial transmitter containing TX pin
pub struct Tx<USART: CommonPins, WORD = u8> {
    _word: PhantomData<WORD>,
    usart: USART,
    pin: USART::Tx<PushPull>,
}

pub trait SerialExt: Sized + Instance {
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig>;

    fn tx<WORD>(
        self,
        tx_pin: impl Into<Self::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Rx<PushPull>>,
    {
        self.serial((tx_pin, NoPin::new()), config, clocks)
            .map(|s| s.split().0)
    }

    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Tx<PushPull>>,
    {
        self.serial((NoPin::new(), rx_pin), config, clocks)
            .map(|s| s.split().1)
    }
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    pub fn new(
        uart: USART,
        pins: (
            impl Into<USART::Tx<PushPull>>,
            impl Into<USART::Rx<PushPull>>,
        ),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Self, config::InvalidConfig> {
        use self::config::*;

        let config = config.into();
        unsafe {
            // Enable clock.
            USART::enable_unchecked();
            USART::reset_unchecked();
        }

        let pclk_freq = USART::clock(clocks).raw();
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
            tx: Tx::new(uart, pins.0.into()),
            rx: Rx::new(unsafe { USART::steal() }, pins.1.into()),
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

impl<UART: CommonPins, WORD> Serial<UART, WORD> {
    pub fn split(self) -> (Tx<UART, WORD>, Rx<UART, WORD>) {
        (self.tx, self.rx)
    }

    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (UART, (UART::Tx<PushPull>, UART::Rx<PushPull>)) {
        (self.tx.usart, (self.tx.pin, self.rx.pin))
    }
}

macro_rules! halUsart {
    ($USART:ty, $Serial:ident, $Rx:ident, $Tx:ident) => {
        pub type $Serial<WORD = u8> = Serial<$USART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$USART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$USART, WORD>;

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
        pub type $Serial<WORD = u8> = Serial<$UART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$UART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$UART, WORD>;

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

impl<UART: CommonPins> Rx<UART, u8> {
    pub(crate) fn with_u16_data(self) -> Rx<UART, u16> {
        Rx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins> Rx<UART, u16> {
    pub(crate) fn with_u8_data(self) -> Rx<UART, u8> {
        Rx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins> Tx<UART, u8> {
    pub(crate) fn with_u16_data(self) -> Tx<UART, u16> {
        Tx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins> Tx<UART, u16> {
    pub(crate) fn with_u8_data(self) -> Tx<UART, u8> {
        Tx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins, WORD> Rx<UART, WORD> {
    pub(crate) fn new(usart: UART, pin: UART::Rx<PushPull>) -> Self {
        Self {
            _word: PhantomData,
            usart,
            pin,
        }
    }

    pub fn join(self, tx: Tx<UART, WORD>) -> Serial<UART, WORD> {
        Serial { tx, rx: self }
    }
}

impl<UART: CommonPins, WORD> Tx<UART, WORD> {
    pub(crate) fn new(usart: UART, pin: UART::Tx<PushPull>) -> Self {
        Self {
            _word: PhantomData,
            usart,
            pin,
        }
    }

    pub fn join(self, rx: Rx<UART, WORD>) -> Serial<UART, WORD> {
        Serial { tx: self, rx }
    }
}

impl<UART: Instance, WORD> AsRef<Tx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<UART, WORD> {
        &self.tx
    }
}

impl<UART: Instance, WORD> AsRef<Rx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<UART, WORD> {
        &self.rx
    }
}

impl<UART: Instance, WORD> AsMut<Tx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<UART, WORD> {
        &mut self.tx
    }
}

impl<UART: Instance, WORD> AsMut<Rx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<UART, WORD> {
        &mut self.rx
    }
}

impl<UART: Instance> Serial<UART, u8> {
    /// Converts this Serial into a version that can read and write `u16` values instead of `u8`s
    ///
    /// This can be used with a word length of 9 bits.
    pub fn with_u16_data(self) -> Serial<UART, u16> {
        Serial {
            tx: self.tx.with_u16_data(),
            rx: self.rx.with_u16_data(),
        }
    }
}

impl<UART: Instance> Serial<UART, u16> {
    /// Converts this Serial into a version that can read and write `u8` values instead of `u16`s
    ///
    /// This can be used with a word length of 8 bits.
    pub fn with_u8_data(self) -> Serial<UART, u8> {
        Serial {
            tx: self.tx.with_u8_data(),
            rx: self.rx.with_u8_data(),
        }
    }
}

pub trait InstanceFC: Instance<RB = pac::usart1::RegisterBlock> + SerialFlowControl {}
impl InstanceFC for pac::USART1 {}
impl InstanceFC for pac::USART2 {}
#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
impl InstanceFC for pac::USART6 {}

impl<UART: InstanceFC, WORD> Serial<UART, WORD> {
    pub fn with_rts(self, rts: impl Into<UART::Rts>) -> Self {
        self.rx.usart.cr3().modify(|_, w| w.rtse().set_bit());
        let _rts = rts.into();
        self
    }
    pub fn with_cts(self, cts: impl Into<UART::Cts>) -> Self {
        self.tx.usart.cr3().modify(|_, w| w.ctse().set_bit());
        let _cts = cts.into();
        self
    }
}
impl<UART: InstanceFC, WORD> Serial<UART, WORD> {
    pub fn enable_request_to_send(&mut self) {
        self.rx.enable_request_to_send();
    }
    pub fn disable_request_to_send(&mut self) {
        self.rx.disable_request_to_send();
    }
    pub fn enable_clear_to_send(&mut self) {
        self.tx.enable_clear_to_send();
    }
    pub fn disable_clear_to_send(&mut self) {
        self.tx.disable_clear_to_send();
    }
    pub fn listen_clear_to_send(&mut self) {
        self.tx.listen_clear_to_send();
    }
    pub fn unlisten_clear_to_send(&mut self) {
        self.tx.unlisten_clear_to_send();
    }
}
impl<UART: InstanceFC, WORD> Rx<UART, WORD> {
    pub fn enable_request_to_send(&mut self) {
        self.usart.cr3().modify(|_, w| w.rtse().set_bit());
    }
    pub fn disable_request_to_send(&mut self) {
        self.usart.cr3().modify(|_, w| w.rtse().clear_bit());
    }
}
impl<UART: InstanceFC, WORD> Tx<UART, WORD> {
    pub fn enable_clear_to_send(&mut self) {
        self.usart.cr3().modify(|_, w| w.ctse().set_bit());
    }
    pub fn disable_clear_to_send(&mut self) {
        self.usart.cr3().modify(|_, w| w.ctse().clear_bit());
    }
    fn listen_clear_to_send(&mut self) {
        self.usart.cr3().modify(|_, w| w.ctsie().set_bit());
    }

    fn unlisten_clear_to_send(&mut self) {
        self.usart.cr3().modify(|_, w| w.ctsie().clear_bit());
    }
}

impl<UART: Instance, WORD> RxISR for Serial<UART, WORD>
where
    Rx<UART, WORD>: RxISR,
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

impl<UART: Instance, WORD> RxISR for Rx<UART, WORD> {
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

impl<UART: Instance, WORD> TxISR for Serial<UART, WORD>
where
    Tx<UART, WORD>: TxISR,
{
    fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }
}

impl<UART: Instance, WORD> TxISR for Tx<UART, WORD> {
    fn is_tx_empty(&self) -> bool {
        self.usart.is_tx_empty()
    }
}

impl<UART: Instance, WORD> RxListen for Rx<UART, WORD> {
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

impl<UART: Instance, WORD> TxListen for Tx<UART, WORD> {
    fn listen(&mut self) {
        self.usart.listen_txe()
    }

    fn unlisten(&mut self) {
        self.usart.unlisten_txe()
    }
}

impl<UART: Instance, WORD> crate::ClearFlags for Serial<UART, WORD> {
    type Flag = CFlag;

    #[inline(always)]
    fn clear_flags(&mut self, flags: impl Into<BitFlags<Self::Flag>>) {
        self.tx.usart.clear_flags(flags.into())
    }
}

impl<UART: Instance, WORD> crate::ReadFlags for Serial<UART, WORD> {
    type Flag = Flag;

    #[inline(always)]
    fn flags(&self) -> BitFlags<Self::Flag> {
        self.tx.usart.flags()
    }
}

impl<UART: Instance, WORD> crate::Listen for Serial<UART, WORD> {
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
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig> {
        Serial::new(self, pins, config, clocks)
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD> {
    pub fn tx(
        usart: UART,
        tx_pin: impl Into<UART::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<UART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<UART::Rx<PushPull>>,
    {
        Self::new(usart, (tx_pin, NoPin::new()), config, clocks).map(|s| s.split().0)
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD> {
    pub fn rx(
        usart: UART,
        rx_pin: impl Into<UART::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<UART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<UART::Tx<PushPull>>,
    {
        Self::new(usart, (NoPin::new(), rx_pin), config, clocks).map(|s| s.split().1)
    }
}

unsafe impl<UART: Instance> PeriAddress for Rx<UART, u8> {
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

unsafe impl<UART: Instance> PeriAddress for Tx<UART, u8> {
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
