use embedded_hal_one::serial::ErrorType;

use super::Instance;

impl<USART: Instance, WORD> ErrorType for super::Serial<USART, WORD> {
    type Error = super::Error;
}

impl<USART: Instance, WORD> ErrorType for super::Rx<USART, WORD> {
    type Error = super::Error;
}

impl<USART: Instance, WORD> ErrorType for super::Tx<USART, WORD> {
    type Error = super::Error;
}

mod nb {
    use core::ops::Deref;

    use super::super::{Error, Instance, RegisterBlockImpl, Rx, Serial, Tx};
    use embedded_hal_nb::serial::{ErrorType, Read, Write};

    impl<USART: Instance, WORD: Copy> Read<WORD> for Serial<USART, WORD>
    where
        Rx<USART, WORD>: Read<WORD> + ErrorType<Error = Self::Error>,
    {
        fn read(&mut self) -> nb::Result<WORD, Error> {
            self.rx.read()
        }
    }

    impl<USART: Instance> Read<u8> for Rx<USART, u8>
    where
        <USART as Instance>::RegisterBlock: RegisterBlockImpl,
    {
        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            unsafe { (*USART::ptr()).read_u8() }
        }
    }

    /// Reads 9-bit words from the UART/USART
    ///
    /// If the UART/USART was configured with `WordLength::DataBits9`, the returned value will contain
    /// 9 received data bits and all other bits set to zero. Otherwise, the returned value will contain
    /// 8 received data bits and all other bits set to zero.
    impl<USART: Instance> Read<u16> for Rx<USART, u16>
    where
        <USART as Instance>::RegisterBlock: RegisterBlockImpl,
    {
        fn read(&mut self) -> nb::Result<u16, Self::Error> {
            unsafe { (*USART::ptr()).read_u16() }
        }
    }

    impl<USART: Instance, WORD: Copy> Write<WORD> for Serial<USART, WORD>
    where
        Tx<USART, WORD>: Write<WORD> + ErrorType<Error = Self::Error>,
    {
        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.tx.flush()
        }

        fn write(&mut self, byte: WORD) -> nb::Result<(), Self::Error> {
            self.tx.write(byte)
        }
    }

    impl<USART: Instance> Write<u8> for Tx<USART, u8>
    where
        <USART as Instance>::RegisterBlock: RegisterBlockImpl,
        USART: Deref<Target = <USART as Instance>::RegisterBlock>,
    {
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
    impl<USART: Instance> Write<u16> for Tx<USART, u16>
    where
        <USART as Instance>::RegisterBlock: RegisterBlockImpl,
        USART: Deref<Target = <USART as Instance>::RegisterBlock>,
    {
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

    use super::super::{Instance, RegisterBlockImpl, Serial, Tx};
    use super::ErrorType;
    use embedded_hal_one::serial::Write;

    impl<USART: Instance, WORD: Copy> Write<WORD> for Serial<USART, WORD>
    where
        Tx<USART, WORD>: Write<WORD> + ErrorType<Error = Self::Error>,
    {
        fn write(&mut self, slice: &[WORD]) -> Result<(), Self::Error> {
            self.tx.write(slice)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.tx.flush()
        }
    }

    impl<USART: Instance> Write<u8> for Tx<USART, u8>
    where
        <USART as Instance>::RegisterBlock: RegisterBlockImpl,
        USART: Deref<Target = <USART as Instance>::RegisterBlock>,
    {
        fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            self.usart.bwrite_all_u8(bytes)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()
        }
    }

    impl<USART: Instance> Write<u16> for Tx<USART, u16>
    where
        <USART as Instance>::RegisterBlock: RegisterBlockImpl,
        USART: Deref<Target = <USART as Instance>::RegisterBlock>,
    {
        fn write(&mut self, slice: &[u16]) -> Result<(), Self::Error> {
            self.usart.bwrite_all_u16(slice)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.usart.bflush()
        }
    }
}
