use embedded_hal_one::serial::{Error, ErrorKind};

impl Error for super::Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Overrun => ErrorKind::Overrun,
            Self::Framing => ErrorKind::FrameFormat,
            Self::Parity => ErrorKind::Parity,
            Self::Noise => ErrorKind::Noise,
        }
    }
}

mod nb {
    use super::super::{Error, Instance, Rx, Serial, Tx};
    use embedded_hal_one::serial::nb::{Read, Write};

    impl<USART, PINS, WORD> Read<WORD> for Serial<USART, PINS, WORD>
    where
        USART: Instance,
        Rx<USART, WORD>: Read<WORD, Error = Error>,
    {
        type Error = Error;

        fn read(&mut self) -> nb::Result<WORD, Error> {
            self.rx.read()
        }
    }

    impl<USART: Instance> Read<u8> for Rx<USART, u8> {
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
    impl<USART: Instance> Read<u16> for Rx<USART, u16> {
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

    impl<USART, PINS, WORD> Write<WORD> for Serial<USART, PINS, WORD>
    where
        USART: Instance,
        Tx<USART, WORD>: Write<WORD, Error = Error>,
    {
        type Error = Error;

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.tx.flush()
        }

        fn write(&mut self, byte: WORD) -> nb::Result<(), Self::Error> {
            self.tx.write(byte)
        }
    }

    impl<USART: Instance> Write<u8> for Tx<USART, u8> {
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
    impl<USART: Instance> Write<u16> for Tx<USART, u16> {
        type Error = Error;

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

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            // NOTE(unsafe) atomic read with no side effects
            let sr = unsafe { (*USART::ptr()).sr.read() };

            if sr.tc().bit_is_set() {
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
    }
}

mod blocking {
    use super::super::{Error, Instance, Serial, Tx};
    use embedded_hal_one::serial::{self, blocking::Write};

    impl<USART: Instance> Write<u8> for Tx<USART, u8> {
        type Error = Error;

        fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            for &b in bytes {
                loop {
                    match <Self as serial::nb::Write<u8>>::write(self, b) {
                        Err(nb::Error::WouldBlock) => continue,
                        Err(nb::Error::Other(err)) => return Err(err),
                        Ok(()) => break,
                    }
                }
            }
            Ok(())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            loop {
                match <Self as serial::nb::Write<u8>>::flush(self) {
                    Ok(()) => return Ok(()),
                    Err(nb::Error::WouldBlock) => continue,
                    Err(nb::Error::Other(err)) => return Err(err),
                }
            }
        }
    }

    impl<USART: Instance, PINS> Write<u8> for Serial<USART, PINS, u8> {
        type Error = Error;

        fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.tx.write(bytes)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.tx.flush()
        }
    }

    impl<USART: Instance> Write<u16> for Tx<USART, u16> {
        type Error = Error;

        fn write(&mut self, buffer: &[u16]) -> Result<(), Self::Error> {
            for &b in buffer {
                loop {
                    match <Self as serial::nb::Write<u16>>::write(self, b) {
                        Err(nb::Error::WouldBlock) => continue,
                        Err(nb::Error::Other(err)) => return Err(err),
                        Ok(()) => break,
                    }
                }
            }
            Ok(())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            loop {
                match <Self as serial::nb::Write<u16>>::flush(self) {
                    Ok(()) => return Ok(()),
                    Err(nb::Error::WouldBlock) => continue,
                    Err(nb::Error::Other(err)) => return Err(err),
                }
            }
        }
    }

    impl<USART: Instance, PINS> Write<u16> for Serial<USART, PINS, u16> {
        type Error = Error;

        fn write(&mut self, bytes: &[u16]) -> Result<(), Self::Error> {
            self.tx.write(bytes)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.tx.flush()
        }
    }
}
