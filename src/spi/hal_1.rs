pub use embedded_hal_one::spi::{Error, ErrorKind, ErrorType, Mode, Phase, Polarity};

impl From<Polarity> for super::Polarity {
    fn from(p: Polarity) -> Self {
        match p {
            Polarity::IdleLow => Self::IdleLow,
            Polarity::IdleHigh => Self::IdleHigh,
        }
    }
}

impl From<Phase> for super::Phase {
    fn from(p: Phase) -> Self {
        match p {
            Phase::CaptureOnFirstTransition => Self::CaptureOnFirstTransition,
            Phase::CaptureOnSecondTransition => Self::CaptureOnSecondTransition,
        }
    }
}

impl From<Mode> for super::Mode {
    fn from(m: Mode) -> Self {
        Self {
            polarity: m.polarity.into(),
            phase: m.phase.into(),
        }
    }
}

impl Error for super::Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Overrun => ErrorKind::Overrun,
            Self::ModeFault => ErrorKind::ModeFault,
            Self::Crc => ErrorKind::Other,
        }
    }
}

impl<SPI, PINS, TRANSFER_MODE> ErrorType for super::Spi<SPI, PINS, TRANSFER_MODE> {
    type Error = super::Error;
}

mod nb {
    use super::super::{Error, Instance, Spi, TransferModeBidi, TransferModeNormal};
    use embedded_hal_one::spi::nb::FullDuplex;

    impl<SPI, PINS> FullDuplex<u8> for Spi<SPI, PINS, TransferModeNormal>
    where
        SPI: Instance,
    {
        fn read(&mut self) -> nb::Result<u8, Error> {
            self.check_read()
        }

        fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
            self.check_send(byte)
        }
    }

    impl<SPI, PINS> FullDuplex<u8> for Spi<SPI, PINS, TransferModeBidi>
    where
        SPI: Instance,
    {
        fn read(&mut self) -> nb::Result<u8, Error> {
            self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
            self.check_read()
        }

        fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
            self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
            self.check_send(byte)
        }
    }
}

mod blocking {
    use super::super::{Error, Instance, Spi, TransferModeBidi, TransferModeNormal};
    use embedded_hal_one::spi::{
        blocking::{Operation, Read, Transactional, Transfer, TransferInplace, Write, WriteIter},
        nb::FullDuplex,
    };

    impl<SPI, PINS, TRANSFER_MODE, W: Copy + 'static> TransferInplace<W>
        for Spi<SPI, PINS, TRANSFER_MODE>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn transfer_inplace(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            for word in words.iter_mut() {
                nb::block!(<Self as FullDuplex<W>>::write(self, *word))?;
                *word = nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, TRANSFER_MODE, W: Copy + 'static> Transfer<W> for Spi<SPI, PINS, TRANSFER_MODE>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn transfer(&mut self, buff: &mut [W], data: &[W]) -> Result<(), Self::Error> {
            assert_eq!(data.len(), buff.len());

            for i in 0..data.len() {
                nb::block!(<Self as FullDuplex<W>>::write(self, data[i]))?;
                buff[i] = nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, W: Copy + 'static> Write<W> for Spi<SPI, PINS, TransferModeNormal>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(<Self as FullDuplex<W>>::write(self, *word))?;
                nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, W: Copy + 'static> Write<W> for Spi<SPI, PINS, TransferModeBidi>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(<Self as FullDuplex<W>>::write(self, *word))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, W: Copy + 'static> WriteIter<W> for Spi<SPI, PINS, TransferModeNormal>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                nb::block!(<Self as FullDuplex<W>>::write(self, word))?;
                nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, W: Copy + 'static> WriteIter<W> for Spi<SPI, PINS, TransferModeBidi>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                nb::block!(<Self as FullDuplex<W>>::write(self, word))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, TRANSFER_MODE, W: Copy + Default + 'static> Read<W>
        for Spi<SPI, PINS, TRANSFER_MODE>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(<Self as FullDuplex<W>>::write(self, W::default()))?;
                *word = nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, TRANSFER_MODE, W: Copy + 'static> Transactional<W> for Spi<SPI, PINS, TRANSFER_MODE>
    where
        Self: Write<W, Error = Error>
            + Read<W, Error = Error>
            + TransferInplace<W, Error = Error>
            + Transfer<W, Error = Error>,
    {
        fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Error> {
            for op in operations {
                match op {
                    Operation::Write(w) => self.write(w)?,
                    Operation::TransferInplace(t) => self.transfer_inplace(t)?,
                    Operation::Read(r) => self.read(r)?,
                    Operation::Transfer(w, r) => self.transfer(w, r)?,
                }
            }

            Ok(())
        }
    }
}
