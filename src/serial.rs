use core::fmt::{Result, Write};
use core::marker::PhantomData;
use core::ptr;

use hal;
use hal::prelude::*;
use nb;

#[cfg(feature = "stm32f407")]
use stm32::{RCC, UART4, UART5, USART1, USART2, USART3, USART6};

#[cfg(feature = "stm32f429")]
use stm32::{RCC, UART4, UART5, UART7, UART8, USART1, USART2, USART3, USART6};

use gpio::gpioa::{PA0, PA1, PA10, PA2, PA3, PA9};
use gpio::gpiob::{PB10, PB11, PB6, PB7};
use gpio::gpioc::{PC10, PC11, PC12, PC6, PC7};
use gpio::gpiod::{PD2, PD5, PD6, PD8, PD9};
#[cfg(feature = "stm32f429")]
use gpio::gpioe::{PE0, PE1};
#[cfg(feature = "stm32f429")]
use gpio::gpiof::{PF6, PF7};
use gpio::gpiog::{PG14, PG9};
use gpio::{Alternate, AF7, AF8};
use rcc::Clocks;
use time::Bps;

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

pub trait Pins<USART> {}

impl Pins<USART1> for (PA9<Alternate<AF7>>, PA10<Alternate<AF7>>) {}
impl Pins<USART1> for (PB6<Alternate<AF7>>, PB7<Alternate<AF7>>) {}

impl Pins<USART2> for (PA2<Alternate<AF7>>, PA3<Alternate<AF7>>) {}
impl Pins<USART2> for (PD5<Alternate<AF7>>, PD6<Alternate<AF7>>) {}

impl Pins<USART3> for (PB10<Alternate<AF7>>, PB11<Alternate<AF7>>) {}
impl Pins<USART3> for (PC10<Alternate<AF7>>, PC11<Alternate<AF7>>) {}
impl Pins<USART3> for (PD8<Alternate<AF7>>, PD9<Alternate<AF7>>) {}

impl Pins<UART4> for (PA0<Alternate<AF8>>, PA1<Alternate<AF8>>) {}
impl Pins<UART4> for (PC10<Alternate<AF8>>, PC11<Alternate<AF8>>) {}

impl Pins<UART5> for (PC12<Alternate<AF8>>, PD2<Alternate<AF8>>) {}

impl Pins<USART6> for (PC6<Alternate<AF8>>, PC7<Alternate<AF8>>) {}
impl Pins<USART6> for (PG14<Alternate<AF8>>, PG9<Alternate<AF8>>) {}

#[cfg(feature = "stm32f429")]
impl Pins<UART7> for (PF7<Alternate<AF8>>, PF6<Alternate<AF8>>) {}
#[cfg(feature = "stm32f429")]
impl Pins<UART8> for (PE1<Alternate<AF8>>, PE0<Alternate<AF8>>) {}

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

/// USART1
impl<PINS> Serial<USART1, PINS> {
    pub fn usart1(usart: USART1, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<USART1>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for USART
        rcc.apb2enr.modify(|_, w| w.usart1en().set_bit());

        // Calculate correct baudrate divisor on the fly
        let div = (clocks.pclk2().0 * 25) / (4 * baud_rate.0);
        let mantissa = div / 100;
        let fraction = ((div - mantissa * 100) * 16 + 50) / 100;
        usart
            .brr
            .write(|w| unsafe { w.bits(mantissa << 4 | fraction) });

        // Reset other registers to disable advanced USART features
        usart.cr2.reset();
        usart.cr3.reset();

        // Enable transmission and receiving
        usart
            .cr1
            .write(|w| w.ue().set_bit().te().set_bit().re().set_bit());

        Serial { usart, pins }
    }

    pub fn split(self) -> (Tx<USART1>, Rx<USART1>) {
        (
            Tx {
                _usart: PhantomData,
            },
            Rx {
                _usart: PhantomData,
            },
        )
    }

    pub fn release(self) -> (USART1, PINS) {
        (self.usart, self.pins)
    }
}

impl hal::serial::Read<u8> for Rx<USART1> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART1::ptr()).sr.read() };

        // Any error requires the dr to be read to clear
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*USART1::ptr()).dr.read() };
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
            return Ok(unsafe { ptr::read_volatile(&(*USART1::ptr()).dr as *const _ as *const _) });
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl hal::serial::Write<u8> for Tx<USART1> {
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART1::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART1::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
            unsafe { ptr::write_volatile(&(*USART1::ptr()).dr as *const _ as *mut _, byte) }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// USART2
impl<PINS> Serial<USART2, PINS> {
    pub fn usart2(usart: USART2, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<USART2>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for USART
        rcc.apb1enr.modify(|_, w| w.usart2en().set_bit());

        // Calculate correct baudrate divisor on the fly
        let div = (clocks.pclk1().0 * 25) / (4 * baud_rate.0);
        let mantissa = div / 100;
        let fraction = ((div - mantissa * 100) * 16 + 50) / 100;
        usart
            .brr
            .write(|w| unsafe { w.bits(mantissa << 4 | fraction) });

        // Reset other registers to disable advanced USART features
        usart.cr2.reset();
        usart.cr3.reset();

        // Enable transmission and receiving
        usart
            .cr1
            .write(|w| w.ue().set_bit().te().set_bit().re().set_bit());

        Serial { usart, pins }
    }

    pub fn split(self) -> (Tx<USART2>, Rx<USART2>) {
        (
            Tx {
                _usart: PhantomData,
            },
            Rx {
                _usart: PhantomData,
            },
        )
    }
    pub fn release(self) -> (USART2, PINS) {
        (self.usart, self.pins)
    }
}

impl hal::serial::Read<u8> for Rx<USART2> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART2::ptr()).sr.read() };

        // Any error requires the dr to be read to clear
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*USART2::ptr()).dr.read() };
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
            return Ok(unsafe { ptr::read_volatile(&(*USART2::ptr()).dr as *const _ as *const _) });
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl hal::serial::Write<u8> for Tx<USART2> {
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART2::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART2::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
            unsafe { ptr::write_volatile(&(*USART2::ptr()).dr as *const _ as *mut _, byte) }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// USART3
impl<PINS> Serial<USART3, PINS> {
    pub fn usart3(usart: USART3, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<USART3>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for USART
        rcc.apb1enr.modify(|_, w| w.usart3en().set_bit());

        // Calculate correct baudrate divisor on the fly
        let div = (clocks.pclk1().0 * 25) / (4 * baud_rate.0);
        let mantissa = div / 100;
        let fraction = ((div - mantissa * 100) * 16 + 50) / 100;
        usart
            .brr
            .write(|w| unsafe { w.bits(mantissa << 4 | fraction) });

        // Reset other registers to disable advanced USART features
        usart.cr2.reset();
        usart.cr3.reset();

        // Enable transmission and receiving
        usart
            .cr1
            .write(|w| w.ue().set_bit().te().set_bit().re().set_bit());

        Serial { usart, pins }
    }

    pub fn split(self) -> (Tx<USART3>, Rx<USART3>) {
        (
            Tx {
                _usart: PhantomData,
            },
            Rx {
                _usart: PhantomData,
            },
        )
    }
    pub fn release(self) -> (USART3, PINS) {
        (self.usart, self.pins)
    }
}

impl hal::serial::Read<u8> for Rx<USART3> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART3::ptr()).sr.read() };

        // Any error requires the dr to be read to clear
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*USART3::ptr()).dr.read() };
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
            return Ok(unsafe { ptr::read_volatile(&(*USART3::ptr()).dr as *const _ as *const _) });
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl hal::serial::Write<u8> for Tx<USART3> {
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART3::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART3::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
            unsafe { ptr::write_volatile(&(*USART3::ptr()).dr as *const _ as *mut _, byte) }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<USART> Write for Tx<USART>
where
    Tx<USART>: hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> Result {
        let _ = s
            .as_bytes()
            .into_iter()
            .map(|c| block!(self.write(*c)))
            .last();
        Ok(())
    }
}
