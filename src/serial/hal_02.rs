mod nb {
    #[allow(unused)]
    use super::super::RegisterBlockImpl;
    use super::super::{Error, Instance, Rx, Serial, Tx};
    use embedded_hal_02::serial::{Read, Write};

    impl<USART: Instance, WORD> Read<WORD> for Serial<USART, WORD>
    where
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
            self.usart.read_u8()
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
            self.usart.read_u16()
        }
    }

    impl<USART: Instance, WORD> Write<WORD> for Serial<USART, WORD>
    where
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
        type Error = Error;

        fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
            self.usart.write_u16(word)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.usart.flush()
        }
    }
}

mod blocking {
    use core::ops::Deref;

    #[allow(unused)]
    use super::super::RegisterBlockImpl;
    use super::super::{Error, Instance, Serial, Tx};
    use embedded_hal_02::blocking::serial::Write;

    impl<USART: Instance> Write<u8> for Tx<USART, u8> {
        type Error = Error;

        fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.usart.bwrite_all_u8(bytes)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()
        }
    }

    impl<USART: Instance> Write<u8> for Serial<USART, u8>
    where
        Tx<USART, u8>: Write<u8, Error = Error>,
    {
        type Error = Error;

        fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.tx.bwrite_all(bytes)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.tx.bflush()
        }
    }

    impl<USART: Instance> Write<u16> for Tx<USART, u16>
    where
        USART: Deref<Target = <USART as crate::Ptr>::RB>,
    {
        type Error = Error;

        fn bwrite_all(&mut self, slice: &[u16]) -> Result<(), Self::Error> {
            self.usart.bwrite_all_u16(slice)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()
        }
    }

    impl<USART: Instance> Write<u16> for Serial<USART, u16>
    where
        Tx<USART, u16>: Write<u16, Error = Error>,
    {
        type Error = Error;

        fn bwrite_all(&mut self, bytes: &[u16]) -> Result<(), Self::Error> {
            self.tx.bwrite_all(bytes)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.tx.bflush()
        }
    }
}
