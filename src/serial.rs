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
use core::ops::Deref;

use crate::rcc;
use nb::block;

mod hal_02;
mod hal_1;
mod uart_impls;

use crate::gpio::{self, PushPull};

use crate::pac;

use crate::gpio::NoPin;
use crate::rcc::Clocks;

#[cfg(features = "dma")]
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

pub mod config;

pub use config::Config;

/// A filler type for when the Tx pin is unnecessary
pub use gpio::NoPin as NoTx;
/// A filler type for when the Rx pin is unnecessary
pub use gpio::NoPin as NoRx;

pub use gpio::alt::SerialAsync as CommonPins;

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

/// Serial abstraction
pub struct Serial<USART: CommonPins, WORD = u8> {
    tx: Tx<USART, WORD>,
    rx: Rx<USART, WORD>,
}

/// Serial receiver containing RX pin
pub struct Rx<USART: CommonPins, WORD = u8> {
    _word: PhantomData<(USART, WORD)>,
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
        NoPin: Into<Self::Rx<PushPull>>;

    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Tx<PushPull>>;
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    pub fn new(
        usart: USART,
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

        usart.brr.write(|w| unsafe { w.bits(div) });

        // Reset other registers to disable advanced USART features
        usart.cr2.reset();
        usart.cr3.reset();

        // Enable transmission and receiving
        // and configure frame

        usart.cr1.write(|w| {
            w.ue().set_bit();
            w.over8().bit(over8);
            w.te().set_bit();
            w.re().set_bit();
            #[cfg(feature = "uart_v1")]
            w.m().bit(config.wordlength == WordLength::DataBits9);
            #[cfg(feature = "uart_v2")]
            w.m1().bit(config.wordlength == WordLength::DataBits9);
            w.pce().bit(config.parity != Parity::ParityNone);
            w.ps().bit(config.parity == Parity::ParityOdd)
        });

        match config.dma {
            DmaConfig::Tx => usart.cr3.write(|w| w.dmat().enabled()),
            DmaConfig::Rx => usart.cr3.write(|w| w.dmar().enabled()),
            DmaConfig::TxRx => usart.cr3.write(|w| w.dmar().enabled().dmat().enabled()),
            DmaConfig::None => {}
        }

        Ok(Serial {
            tx: Tx::new(usart, pins.0.into()),
            rx: Rx::new(pins.1.into()),
        }
        .config_stop(config))
    }

    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (USART, (USART::Tx<PushPull>, USART::Rx<PushPull>)) {
        (self.tx.usart, (self.tx.pin, self.rx.pin))
    }
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    fn config_stop(self, config: config::Config) -> Self {
        self.tx.usart.set_stopbits(config.stopbits);
        self
    }
}

use crate::pac::usart1 as uart_base;

// Implemented by all USART instances
pub trait Instance:
    crate::Sealed
    + Deref<Target = uart_base::RegisterBlock>
    + rcc::Enable
    + rcc::Reset
    + rcc::BusClock
    + CommonPins
{
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
pub(crate) use halUsart;

#[cfg(feature = "usart1")]
halUsart! { pac::USART1, usart1, Serial1, Rx1, Tx1 }
#[cfg(feature = "usart2")]
halUsart! { pac::USART2, usart2, Serial2, Rx2, Tx2 }
#[cfg(feature = "usart3")]
#[cfg(not(feature = "l4"))]
halUsart! { pac::USART3, usart3, Serial3, Rx3, Tx3 }
#[cfg(feature = "usart4")]
halUsart! { pac::USART4, usart4, Serial4, Rx4, Tx4 }
#[cfg(feature = "usart5")]
halUsart! { pac::USART5, usart5, Serial5, Rx5, Tx5 }
#[cfg(feature = "usart6")]
halUsart! { pac::USART6, usart6, Serial6, Rx6, Tx6 }
#[cfg(feature = "usart7")]
halUsart! { pac::USART7, usart7, Serial7, Rx7, Tx7 }
#[cfg(feature = "usart8")]
halUsart! { pac::USART8, usart8, Serial8, Rx8, Tx8 }
#[cfg(feature = "usart10")]
halUsart! { pac::USART10, usart10, Serial10, Rx10, Tx10 }
