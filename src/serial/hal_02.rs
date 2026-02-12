mod nb {
    #[allow(unused)]
    use super::super::RegisterBlockImpl;
    use super::super::{Error, Instance, Rx, Serial, Tx};
    use embedded_hal_02::serial::{Read, Write};

    impl<USART: Instance, WORD> Read<WORD> for Serial<USART>
    where
        Rx<USART>: Read<WORD, Error = Error>,
    {
        type Error = Error;

        #[inline(always)]
        fn read(&mut self) -> nb::Result<WORD, Self::Error> {
            Read::read(&mut self.rx)
        }
    }

    impl<USART: Instance> Read<u8> for Rx<USART> {
        type Error = Error;

        #[inline(always)]
        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            self.usart.read_u8()
        }
    }

    /// Reads 9-bit words from the UART/USART
    ///
    /// If the UART/USART was configured with `WordLength::DataBits9`, the returned value will contain
    /// 9 received data bits and all other bits set to zero. Otherwise, the returned value will contain
    /// 8 received data bits and all other bits set to zero.
    impl<USART: Instance> Read<u16> for Rx<USART> {
        type Error = Error;

        #[inline(always)]
        fn read(&mut self) -> nb::Result<u16, Self::Error> {
            self.usart.read_u16()
        }
    }

    impl<USART: Instance, WORD> Write<WORD> for Serial<USART>
    where
        Tx<USART>: Write<WORD, Error = Error>,
    {
        type Error = Error;

        #[inline(always)]
        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            Write::flush(&mut self.tx)
        }

        #[inline(always)]
        fn write(&mut self, word: WORD) -> nb::Result<(), Self::Error> {
            Write::write(&mut self.tx, word)
        }
    }

    impl<USART: Instance> Write<u8> for Tx<USART> {
        type Error = Error;

        #[inline(always)]
        fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            self.usart.write_u8(word)
        }
        #[inline(always)]
        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.usart.flush()
        }
    }

    /// Writes 9-bit words to the UART/USART
    ///
    /// If the UART/USART was configured with `WordLength::DataBits9`, the 9 least significant bits will
    /// be transmitted and the other 7 bits will be ignored. Otherwise, the 8 least significant bits
    /// will be transmitted and the other 8 bits will be ignored.
    impl<USART: Instance> Write<u16> for Tx<USART> {
        type Error = Error;

        #[inline(always)]
        fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
            self.usart.write_u16(word)
        }

        #[inline(always)]
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

    impl<USART: Instance> Write<u8> for Tx<USART> {
        type Error = Error;

        #[inline(always)]
        fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.usart.bwrite_all_u8(bytes)
        }

        #[inline(always)]
        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()
        }
    }

    impl<USART: Instance> Write<u8> for Serial<USART>
    where
        Tx<USART>: Write<u8, Error = Error>,
    {
        type Error = Error;

        #[inline(always)]
        fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            Write::bwrite_all(&mut self.tx, bytes)
        }

        #[inline(always)]
        fn bflush(&mut self) -> Result<(), Self::Error> {
            Write::bflush(&mut self.tx)
        }
    }

    impl<USART: Instance> Write<u16> for Tx<USART>
    where
        USART: Deref<Target = <USART as crate::Ptr>::RB>,
    {
        type Error = Error;

        #[inline(always)]
        fn bwrite_all(&mut self, slice: &[u16]) -> Result<(), Self::Error> {
            self.usart.bwrite_all_u16(slice)
        }

        #[inline(always)]
        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()
        }
    }

    impl<USART: Instance> Write<u16> for Serial<USART>
    where
        Tx<USART>: Write<u16, Error = Error>,
    {
        type Error = Error;

        #[inline(always)]
        fn bwrite_all(&mut self, bytes: &[u16]) -> Result<(), Self::Error> {
            Write::bwrite_all(&mut self.tx, bytes)
        }

        #[inline(always)]
        fn bflush(&mut self) -> Result<(), Self::Error> {
            Write::bflush(&mut self.tx)
        }
    }
}
