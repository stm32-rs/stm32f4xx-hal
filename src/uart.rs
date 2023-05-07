//!
//! Asynchronous serial communication using UART peripherals
//!
//! # Word length
//!
//! By default, the UART/UART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
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

use crate::rcc;
use nb::block;

#[allow(clippy::duplicate_mod)]
#[path = "./serial/hal_02.rs"]
mod hal_02;
#[allow(clippy::duplicate_mod)]
#[path = "./serial/hal_1.rs"]
mod hal_1;
#[allow(clippy::duplicate_mod)]
#[path = "./serial/uart_impls.rs"]
mod uart_impls;

use crate::pac;

use crate::gpio::{NoPin, PushPull};
use crate::rcc::Clocks;

#[allow(unused)]
#[cfg(feature = "dma")]
use crate::dma::traits::PeriAddress;

pub use crate::serial::{config, CommonPins, Event, NoRx, NoTx, RxISR, TxISR};
pub use config::Config;
/// Serial error
pub use embedded_hal_one::serial::ErrorKind as Error;

/// Serial abstraction
pub struct Serial<UART: CommonPins, WORD = u8> {
    tx: Tx<UART, WORD>,
    rx: Rx<UART, WORD>,
}

/// Serial receiver containing RX pin
pub struct Rx<UART: CommonPins, WORD = u8> {
    _word: PhantomData<(UART, WORD)>,
    pin: UART::Rx<PushPull>,
}

/// Serial transmitter containing TX pin
pub struct Tx<UART: CommonPins, WORD = u8> {
    _word: PhantomData<WORD>,
    usart: UART,
    pin: UART::Tx<PushPull>,
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

impl<UART: Instance, WORD> Serial<UART, WORD> {
    /*
        StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART
        STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
    */
    pub fn new(
        usart: UART,
        pins: (impl Into<UART::Tx<PushPull>>, impl Into<UART::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Self, config::InvalidConfig> {
        use self::config::*;

        let config = config.into();
        unsafe {
            // Enable clock.
            UART::enable_unchecked();
            UART::reset_unchecked();
        }

        let pclk_freq = UART::clock(clocks).raw();
        let baud = config.baudrate.0;

        // The frequency to calculate UARTDIV is this:
        //
        // (Taken from STM32F411xC/E Reference Manual,
        // Section 19.3.4, Equation 1)
        //
        // 16 bit oversample: OVER8 = 0
        // 8 bit oversample:  OVER8 = 1
        //
        // UARTDIV =          (pclk)
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // BUT, the UARTDIV has 4 "fractional" bits, which effectively
        // means that we need to "correct" the equation as follows:
        //
        // UARTDIV =      (pclk) * 16
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

        unsafe { (*UART::ptr()).brr.write(|w| w.bits(div)) };

        // Reset other registers to disable advanced UART features
        unsafe { (*UART::ptr()).cr2.reset() };
        unsafe { (*UART::ptr()).cr3.reset() };

        // Enable transmission and receiving
        // and configure frame
        unsafe {
            (*UART::ptr()).cr1.write(|w| {
                w.ue().set_bit();
                w.over8().bit(over8);
                w.te().set_bit();
                w.re().set_bit();
                #[cfg(feature = "f4")]
                w.m().bit(config.wordlength == WordLength::DataBits9);
                #[cfg(feature = "l4")]
                w.m1().bit(config.wordlength == WordLength::DataBits9);
                w.pce().bit(config.parity != Parity::ParityNone);
                w.ps().bit(config.parity == Parity::ParityOdd)
            })
        };

        match config.dma {
            DmaConfig::Tx => unsafe { (*UART::ptr()).cr3.write(|w| w.dmat().enabled()) },
            DmaConfig::Rx => unsafe { (*UART::ptr()).cr3.write(|w| w.dmar().enabled()) },
            DmaConfig::TxRx => unsafe {
                (*UART::ptr())
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

    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (UART, (UART::Tx<PushPull>, UART::Rx<PushPull>)) {
        (self.tx.usart, (self.tx.pin, self.rx.pin))
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD> {
    fn config_stop(self, config: config::Config) -> Self {
        self.tx.usart.set_stopbits(config.stopbits);
        self
    }
}

use crate::pac::uart4 as uart_base;

// Implemented by all UART instances
pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + rcc::BusClock + CommonPins {
    #[doc(hidden)]
    fn ptr() -> *const uart_base::RegisterBlock;
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
}

#[cfg(not(any(feature = "stm32f413", feature = "stm32f423",)))]
macro_rules! halUart {
    ($UART:ty, $usart:ident, $Serial:ident, $Tx:ident, $Rx:ident) => {
        pub type $Serial<WORD = u8> = Serial<$UART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$UART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$UART, WORD>;

        impl Instance for $UART {
            fn ptr() -> *const uart_base::RegisterBlock {
                <$UART>::ptr() as *const _
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

#[cfg(feature = "uart4")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { pac::UART4, uart4, Serial4, Rx4, Tx4 }
#[cfg(feature = "uart5")]
#[cfg(not(any(feature = "stm32f413", feature = "stm32f423")))]
halUart! { pac::UART5, uart5, Serial5, Rx5, Tx5 }

#[cfg(feature = "uart4")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
impl Instance for pac::UART4 {
    fn ptr() -> *const uart_base::RegisterBlock {
        pac::UART4::ptr() as *const _
    }

    fn set_stopbits(&self, _bits: config::StopBits) {
        todo!()
    }
}

#[cfg(feature = "uart5")]
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
crate::serial::halUsart! { pac::UART5, uart5, Serial5, Rx5, Tx5 }

#[cfg(feature = "uart7")]
crate::serial::halUsart! { pac::UART7, uart7, Serial7, Rx7, Tx7 }
#[cfg(feature = "uart8")]
crate::serial::halUsart! { pac::UART8, uart8, Serial8, Rx8, Tx8 }
#[cfg(feature = "uart9")]
crate::serial::halUsart! { pac::UART9, uart9, Serial9, Rx9, Tx9 }
#[cfg(feature = "uart10")]
crate::serial::halUsart! { pac::UART10, uart10, Serial10, Rx10, Tx10 }
