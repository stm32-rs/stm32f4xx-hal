mod nb {
    use super::super::{Error, Instance, RegisterBlockImpl, Rx, Serial, Tx};
    use embedded_hal_nb::serial::{ErrorKind, Read, Write};

    impl embedded_hal_nb::serial::Error for Error {
        fn kind(&self) -> ErrorKind {
            match self {
                Error::Overrun => ErrorKind::Overrun,
                Error::FrameFormat => ErrorKind::FrameFormat,
                Error::Parity => ErrorKind::Parity,
                Error::Noise => ErrorKind::Noise,
                Error::Other => ErrorKind::Other,
            }
        }
    }

    impl<USART: Instance, WORD> embedded_hal_nb::serial::ErrorType for Serial<USART, WORD> {
        type Error = Error;
    }
    impl<USART: Instance, WORD> embedded_hal_nb::serial::ErrorType for Rx<USART, WORD> {
        type Error = Error;
    }
    impl<USART: Instance, WORD> embedded_hal_nb::serial::ErrorType for Tx<USART, WORD> {
        type Error = Error;
    }

    impl<USART: Instance, WORD: Copy> Read<WORD> for Serial<USART, WORD>
    where
        Rx<USART, WORD>: Read<WORD, Error = Error>,
    {
        fn read(&mut self) -> nb::Result<WORD, Self::Error> {
            self.rx.read()
        }
    }

    impl<USART: Instance> Read<u8> for Rx<USART, u8> {
        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            self.usart.read_u8()
        }
    }

    /// Reads 9-bit words from the UART/USART
    ///
    /// If the UART/USART was configured with `WordLength::DataBits9`, the returned value will contain
    /// 9 received data bits and all other bits set to zero. Otherwise, the returned value will contain
    /// 8 received data bits and all other bits set to zero.
    impl<USART: Instance> Read<u16> for Rx<USART, u16> {
        fn read(&mut self) -> nb::Result<u16, Self::Error> {
            self.usart.read_u16()
        }
    }

    impl<USART: Instance, WORD: Copy> Write<WORD> for Serial<USART, WORD>
    where
        Tx<USART, WORD>: Write<WORD, Error = Error>,
    {
        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.tx.flush()
        }

        fn write(&mut self, byte: WORD) -> nb::Result<(), Self::Error> {
            self.tx.write(byte)
        }
    }

    impl<USART: Instance> Write<u8> for Tx<USART, u8> {
        fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            self.usart.write_u8(word)
        }
        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.usart.flush()
        }
    }

    /// Writes 9-bit words to the UART/USART
    ///
    /// If the UART/USART was configured with `WordLength::DataBits9`, the 9 least significant bits will
    /// be transmitted and the other 7 bits will be ignored. Otherwise, the 8 least significant bits
    /// will be transmitted and the other 8 bits will be ignored.
    impl<USART: Instance> Write<u16> for Tx<USART, u16> {
        fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
            self.usart.write_u16(word)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.usart.flush()
        }
    }
}

mod io {
    use core::ops::Deref;

    use super::super::{Error, Instance, RegisterBlockImpl, Rx, Serial, Tx};
    use embedded_io::Write;

    impl embedded_io::Error for Error {
        // TODO: fix error conversion
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }

    impl<USART: Instance, WORD> embedded_io::ErrorType for Serial<USART, WORD> {
        type Error = Error;
    }

    impl<USART: Instance, WORD> embedded_io::ErrorType for Tx<USART, WORD> {
        type Error = Error;
    }

    impl<USART: Instance, WORD> embedded_io::ErrorType for Rx<USART, WORD> {
        type Error = Error;
    }

    impl<USART: Instance> Write for Tx<USART, u8>
    where
        <USART as crate::Ptr>::RB: RegisterBlockImpl,
        USART: Deref<Target = <USART as crate::Ptr>::RB>,
    {
        fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
            let mut i = 0;
            for byte in bytes.iter() {
                match self.usart.write_u8(*byte) {
                    Ok(_) => {
                        i += 1;
                    }
                    Err(nb::Error::WouldBlock) => {
                        return Ok(i);
                    }
                    Err(nb::Error::Other(e)) => {
                        return Err(e);
                    }
                }
            }
            Ok(i)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()?;
            Ok(())
        }
    }

    impl<USART: Instance> Write for Serial<USART, u8>
    where
        Tx<USART, u8>: Write<Error = Error>,
    {
        fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
            self.tx.write(bytes)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.tx.flush()
        }
    }
}
