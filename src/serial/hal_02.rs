mod nb {
    use super::super::{Error, Instance, Rx, Serial, Tx};
    use embedded_hal::serial::{Read, Write};

    impl<USART, PINS, WORD> Read<WORD> for Serial<USART, PINS, WORD>
    where
        USART: Instance,
        Rx<USART, WORD>: Read<WORD, Error = Error>,
    {
        type Error = Error;

        fn read(&mut self) -> nb::Result<WORD, Self::Error> {
            self.rx.read()
        }
    }

    impl<USART: Instance> Read<u8> for Rx<USART, u8> {
        type Error = Error;

        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            self.read()
        }
    }

    /// Reads 9-bit words from the UART/USART
    ///
    /// If the UART/USART was configured with `WordLength::DataBits9`, the returned value will contain
    /// 9 received data bits and all other bits set to zero. Otherwise, the returned value will contain
    /// 8 received data bits and all other bits set to zero.
    impl<USART: Instance> Read<u16> for Rx<USART, u16> {
        type Error = Error;

        fn read(&mut self) -> nb::Result<u16, Self::Error> {
            self.read()
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
            self.write(word)
        }
        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.flush()
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
            self.write(word)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.flush()
        }
    }
}

mod blocking {
    use super::super::{Error, Instance, Serial, Tx};
    use embedded_hal::blocking::serial::Write;

    impl<USART: Instance> Write<u8> for Tx<USART, u8> {
        type Error = Error;

        fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.bwrite_all(bytes)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.bflush()
        }
    }

    impl<USART: Instance, PINS> Write<u8> for Serial<USART, PINS, u8> {
        type Error = Error;

        fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.tx.bwrite_all(bytes)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.tx.bflush()
        }
    }

    impl<USART: Instance> Write<u16> for Tx<USART, u16> {
        type Error = Error;

        fn bwrite_all(&mut self, slice: &[u16]) -> Result<(), Self::Error> {
            self.bwrite_all(slice)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.bflush()
        }
    }

    impl<USART: Instance, PINS> Write<u16> for Serial<USART, PINS, u16> {
        type Error = Error;

        fn bwrite_all(&mut self, slice: &[u16]) -> Result<(), Self::Error> {
            self.tx.bwrite_all(slice)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.tx.bflush()
        }
    }
}
