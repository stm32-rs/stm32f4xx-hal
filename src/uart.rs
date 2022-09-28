//! Asynchronous serial communication using UART peripherals
//!
//! # Word length
//!
//! By default, the UART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
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
use core::ops::{Deref, DerefMut};

use crate::rcc;
use nb::block;

#[path = "./serial/hal_02.rs"]
mod hal_02;
#[path = "./serial/hal_1.rs"]
mod hal_1;
#[path = "./serial/uart_impls.rs"]
mod uart_impls;

use crate::pac::{self, RCC};

use crate::gpio::NoPin;
use crate::rcc::Clocks;

use crate::dma::traits::PeriAddress;

pub use crate::serial::{config, Event, NoRx, NoTx, Pins, RxPin, TxPin};
pub use config::Config;
/// Serial error
pub use embedded_hal_one::serial::ErrorKind as Error;

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

impl<USART: Instance, WORD> Rx<USART, WORD> {
    fn new() -> Self {
        Self {
            _usart: PhantomData,
            _word: PhantomData,
        }
    }
}

impl<USART: Instance, WORD> Tx<USART, WORD> {
    fn new() -> Self {
        Self {
            _usart: PhantomData,
            _word: PhantomData,
        }
    }
}

impl<USART, PINS, WORD> AsRef<Tx<USART, WORD>> for Serial<USART, PINS, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<USART, WORD> {
        &self.tx
    }
}

impl<USART, PINS, WORD> AsRef<Rx<USART, WORD>> for Serial<USART, PINS, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<USART, WORD> {
        &self.rx
    }
}

impl<USART, PINS, WORD> AsMut<Tx<USART, WORD>> for Serial<USART, PINS, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<USART, WORD> {
        &mut self.tx
    }
}

impl<USART, PINS, WORD> AsMut<Rx<USART, WORD>> for Serial<USART, PINS, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<USART, WORD> {
        &mut self.rx
    }
}

/// Serial receiver containing RX pin
pub struct URx<USART, RX, WORD = u8> {
    inner: Rx<USART, WORD>,
    pin: RX,
}

/// Serial transmitter containing TX pin
pub struct UTx<USART, TX, WORD = u8> {
    inner: Tx<USART, WORD>,
    usart: USART,
    pin: TX,
}

impl<USART: Instance, RX, WORD> URx<USART, RX, WORD> {
    fn new(pin: RX) -> Self {
        Self {
            inner: Rx::new(),
            pin,
        }
    }

    pub fn erase(self) -> Rx<USART, WORD> {
        Rx::new()
    }

    pub fn join<TX>(self, tx: UTx<USART, TX, WORD>) -> Serial<USART, (TX, RX), WORD>
    where
        (TX, RX): Pins<USART>,
    {
        Serial {
            usart: tx.usart,
            pins: (tx.pin, self.pin),
            tx: tx.inner,
            rx: self.inner,
        }
    }
}

impl<USART: Instance, TX, WORD> UTx<USART, TX, WORD> {
    fn new(usart: USART, pin: TX) -> Self {
        Self {
            inner: Tx::new(),
            usart,
            pin,
        }
    }

    pub fn erase(self) -> Tx<USART, WORD> {
        Tx::new()
    }

    pub fn join<RX>(self, rx: URx<USART, RX, WORD>) -> Serial<USART, (TX, RX), WORD>
    where
        (TX, RX): Pins<USART>,
    {
        Serial {
            usart: self.usart,
            pins: (self.pin, rx.pin),
            tx: self.inner,
            rx: rx.inner,
        }
    }
}

impl<USART, TX, WORD> AsRef<Tx<USART, WORD>> for UTx<USART, TX, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<USART, WORD> {
        &self.inner
    }
}

impl<USART, TX, WORD> Deref for UTx<USART, TX, WORD> {
    type Target = Tx<USART, WORD>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<USART, RX, WORD> AsRef<Rx<USART, WORD>> for URx<USART, RX, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<USART, WORD> {
        &self.inner
    }
}

impl<USART, RX, WORD> Deref for URx<USART, RX, WORD> {
    type Target = Rx<USART, WORD>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<USART, TX, WORD> AsMut<Tx<USART, WORD>> for UTx<USART, TX, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<USART, WORD> {
        &mut self.inner
    }
}

impl<USART, TX, WORD> DerefMut for UTx<USART, TX, WORD> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<USART, RX, WORD> AsMut<Rx<USART, WORD>> for URx<USART, RX, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<USART, WORD> {
        &mut self.inner
    }
}

impl<USART, RX, WORD> DerefMut for URx<USART, RX, WORD> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub trait SerialExt: Sized + Instance {
    fn serial<TX, RX, WORD>(
        self,
        pins: (TX, RX),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, (TX, RX), WORD>, config::InvalidConfig>
    where
        (TX, RX): Pins<Self>;
    fn tx<TX, WORD>(
        self,
        tx_pin: TX,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        (TX, NoPin): Pins<Self>;
    fn rx<RX, WORD>(
        self,
        rx_pin: RX,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        (NoPin, RX): Pins<Self>;
}

impl<USART: Instance> SerialExt for USART {
    fn serial<TX, RX, WORD>(
        self,
        pins: (TX, RX),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, (TX, RX), WORD>, config::InvalidConfig>
    where
        (TX, RX): Pins<Self>,
    {
        Serial::new(self, pins, config, clocks)
    }
    fn tx<TX, WORD>(
        self,
        tx_pin: TX,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        (TX, NoPin): Pins<Self>,
    {
        Serial::tx(self, tx_pin, config, clocks)
    }
    fn rx<RX, WORD>(
        self,
        rx_pin: RX,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        (NoPin, RX): Pins<Self>,
    {
        Serial::rx(self, rx_pin, config, clocks)
    }
}

impl<USART, TX, RX, WORD> Serial<USART, (TX, RX), WORD>
where
    (TX, RX): Pins<USART>,
    USART: Instance,
{
    /*
        StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART

        STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
    */
    pub fn new(
        usart: USART,
        mut pins: (TX, RX),
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

        pins.set_alt_mode();

        Ok(Serial {
            usart,
            pins,
            tx: Tx::new(),
            rx: Rx::new(),
        }
        .config_stop(config))
    }

    pub fn release(mut self) -> (USART, (TX, RX)) {
        self.pins.restore_mode();

        (self.usart, (self.pins.0, self.pins.1))
    }
}

impl<USART, TX, WORD> Serial<USART, (TX, NoPin), WORD>
where
    (TX, NoPin): Pins<USART>,
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

impl<USART, RX, WORD> Serial<USART, (NoPin, RX), WORD>
where
    (NoPin, RX): Pins<USART>,
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

impl<USART: Instance, PINS, WORD> Serial<USART, PINS, WORD> {
    pub fn split(self) -> (Tx<USART, WORD>, Rx<USART, WORD>) {
        (self.tx, self.rx)
    }
}

impl<USART: Instance, TX, RX, WORD> Serial<USART, (TX, RX), WORD> {
    pub fn split_nondestructive(self) -> (UTx<USART, TX, WORD>, URx<USART, RX, WORD>) {
        (UTx::new(self.usart, self.pins.0), URx::new(self.pins.1))
    }
}

impl<USART: Instance, PINS> Serial<USART, PINS, u8> {
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

impl<USART: Instance, PINS> Serial<USART, PINS, u16> {
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

impl<USART: Instance, PINS, WORD> Serial<USART, PINS, WORD> {
    fn config_stop(self, config: config::Config) -> Self {
        self.set_stopbits(config.stopbits);
        self
    }

    fn set_stopbits(&self, bits: config::StopBits) {
        use crate::pac::uart4::cr2::STOP_A;
        use config::StopBits;

        unsafe { &(*USART::ptr()) }.cr2.write(|w| {
            w.stop().variant(match bits {
                StopBits::STOP0P5 => STOP_A::Stop1,
                StopBits::STOP1 => STOP_A::Stop1,
                StopBits::STOP1P5 => STOP_A::Stop2,
                StopBits::STOP2 => STOP_A::Stop2,
            })
        });
    }
}

use crate::pac::uart4 as uart_base;

// Implemented by all USART instances
pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + rcc::BusClock {
    #[doc(hidden)]
    fn ptr() -> *const uart_base::RegisterBlock;
}

macro_rules! halUart {
    ($USART:ty, $Serial:ident, $Tx:ident, $Rx:ident) => {
        pub type $Serial<PINS, WORD = u8> = Serial<$USART, PINS, WORD>;
        pub type $Tx<WORD = u8> = Tx<$USART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$USART, WORD>;

        impl Instance for $USART {
            fn ptr() -> *const uart_base::RegisterBlock {
                <$USART>::ptr() as *const _
            }
        }
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
