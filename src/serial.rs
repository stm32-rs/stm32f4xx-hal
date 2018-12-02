use core::fmt;
use core::marker::PhantomData;
use core::ptr;

use hal;
use hal::prelude::*;
use nb::block;

#[cfg(any(feature = "stm32f401", feature = "stm32f411"))]
use stm32::{RCC, USART1, USART2, USART6};

#[cfg(feature = "stm32f407")]
use stm32::{RCC, UART4, UART5, USART1, USART2, USART3, USART6};

#[cfg(feature = "stm32f412")]
use stm32::{RCC, USART1, USART2, USART3, USART6};

#[cfg(feature = "stm32f429")]
use stm32::{RCC, UART4, UART5, UART7, UART8, USART1, USART2, USART3, USART6};

#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
use stm32::usart6::cr2::STOPW;

#[cfg(any(feature = "stm32f401", feature = "stm32f412", feature = "stm32f411"))]
use stm32::usart1::cr2::STOPW;

use gpio::gpioa::{PA10, PA2, PA3, PA9};
use gpio::gpiob::{PB6, PB7};
use gpio::gpioc::{PC6, PC7};
use gpio::gpiod::{PD5, PD6};

#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
use gpio::gpioa::{PA0, PA1};
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
use gpio::gpiob::{PB10, PB11};
#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
use gpio::gpioc::PC12;
#[cfg(feature = "stm32f412")]
use gpio::gpioc::PC5;
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
use gpio::gpioc::{PC10, PC11};
#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
use gpio::gpiod::PD2;
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
use gpio::gpiod::{PD8, PD9};
#[cfg(feature = "stm32f429")]
use gpio::gpioe::{PE0, PE1};
#[cfg(feature = "stm32f429")]
use gpio::gpiof::{PF6, PF7};
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
use gpio::gpiog::{PG14, PG9};
use gpio::{Alternate, AF7, AF8};
use rcc::Clocks;

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
}

pub mod config {
    use time::Bps;
    use time::U32Ext;

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

    pub struct Config {
        pub baudrate: Bps,
        pub wordlength: WordLength,
        pub parity: Parity,
        pub stopbits: StopBits,
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
            }
        }
    }
}

pub trait Pins<USART> {}

impl Pins<USART1> for (PA9<Alternate<AF7>>, PA10<Alternate<AF7>>) {}
impl Pins<USART1> for (PB6<Alternate<AF7>>, PB7<Alternate<AF7>>) {}

impl Pins<USART2> for (PA2<Alternate<AF7>>, PA3<Alternate<AF7>>) {}
impl Pins<USART2> for (PD5<Alternate<AF7>>, PD6<Alternate<AF7>>) {}

#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
impl Pins<USART3> for (PB10<Alternate<AF7>>, PB11<Alternate<AF7>>) {}
#[cfg(any(feature = "stm32f412"))]
impl Pins<USART3> for (PB10<Alternate<AF7>>, PC5<Alternate<AF7>>) {}
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
impl Pins<USART3> for (PC10<Alternate<AF7>>, PC11<Alternate<AF7>>) {}
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
impl Pins<USART3> for (PD8<Alternate<AF7>>, PD9<Alternate<AF7>>) {}

#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
impl Pins<UART4> for (PA0<Alternate<AF8>>, PA1<Alternate<AF8>>) {}
#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
impl Pins<UART4> for (PC10<Alternate<AF8>>, PC11<Alternate<AF8>>) {}

#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
impl Pins<UART5> for (PC12<Alternate<AF8>>, PD2<Alternate<AF8>>) {}

impl Pins<USART6> for (PC6<Alternate<AF8>>, PC7<Alternate<AF8>>) {}
#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
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

macro_rules! halUsart {
    ($(
        $USARTX:ident: ($usartX:ident, $apbXenr:ident, $usartXen:ident,  $pclkX:ident),
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

                    // NOTE(unsafe) This executes only during initialisation
                    let rcc = unsafe { &(*RCC::ptr()) };

                    // Enable clock for USART
                    rcc.$apbXenr.modify(|_, w| w.$usartXen().set_bit());

                    // Calculate correct baudrate divisor on the fly
                    let div = (clocks.$pclkX().0 * 25) / (4 * config.baudrate.0);
                    let mantissa = div / 100;
                    let fraction = ((div - mantissa * 100) * 16 + 50) / 100;
                    usart
                        .brr
                        .write(|w| unsafe { w.bits(mantissa << 4 | fraction) });

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

                    usart.cr2.write(|w| {
                        w.stop().variant(match config.stopbits {
                            StopBits::STOP0P5 => STOPW::STOP0P5,
                            StopBits::STOP1 => STOPW::STOP1,
                            StopBits::STOP1P5 => STOPW::STOP1P5,
                            StopBits::STOP2 => STOPW::STOP2,
                        })
                    });
                    Ok(Serial { usart, pins })
                }

                /// Starts listening for an interrupt event
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.usart.cr1.modify(|_, w| w.rxneie().set_bit())
                        },
                        Event::Txe => {
                            self.usart.cr1.modify(|_, w| w.txeie().set_bit())
                        },
                    }
                }

                /// Starts listening for an interrupt event
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.usart.cr1.modify(|_, w| w.rxneie().clear_bit())
                        },
                        Event::Txe => {
                            self.usart.cr1.modify(|_, w| w.txeie().clear_bit())
                        },
                    }
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

            impl hal::serial::Read<u8> for Rx<$USARTX> {
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

            impl hal::serial::Write<u8> for Tx<$USARTX> {
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
        )+
    }
}

halUsart! {
    USART1: (usart1, apb2enr, usart1en, pclk2),
    USART2: (usart2, apb1enr, usart2en, pclk1),
    USART6: (usart6, apb2enr, usart6en, pclk2),
}

#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
halUsart! {
    USART3: (usart3, apb1enr, usart3en, pclk1),
}

impl<USART> fmt::Write for Tx<USART>
where
    Tx<USART>: hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = s
            .as_bytes()
            .into_iter()
            .map(|c| block!(self.write(*c)))
            .last();
        Ok(())
    }
}
