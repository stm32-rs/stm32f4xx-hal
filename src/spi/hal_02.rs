pub use embedded_hal::spi::{Mode, Phase, Polarity};

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

mod nb {
    use super::super::{Error, FrameSize, Instance, Spi};
    use embedded_hal::spi::FullDuplex;

    impl<SPI, PINS, const BIDI: bool, W: FrameSize> FullDuplex<W> for Spi<SPI, PINS, BIDI, W>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn read(&mut self) -> nb::Result<W, Error> {
            if BIDI {
                self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
            }
            self.check_read()
        }

        fn send(&mut self, byte: W) -> nb::Result<(), Error> {
            if BIDI {
                self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
            }
            self.check_send(byte)
        }
    }
}

mod blocking {
    use super::super::{Error, Instance, Spi};
    use embedded_hal::blocking::spi::{Operation, Transactional, Transfer, Write, WriteIter};
    use embedded_hal::spi::FullDuplex;

    impl<SPI, PINS, const BIDI: bool> Transfer<u8> for Spi<SPI, PINS, BIDI, u8>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
            for word in words.iter_mut() {
                nb::block!(self.send(*word))?;
                *word = nb::block!(self.read())?;
            }

            Ok(words)
        }
    }

    impl<SPI, PINS, const BIDI: bool> Transfer<u16> for Spi<SPI, PINS, BIDI, u16>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u16]) -> Result<&'w [u16], Self::Error> {
            for word in words.iter_mut() {
                nb::block!(self.send(*word))?;
                *word = nb::block!(self.read())?;
            }

            Ok(words)
        }
    }

    impl<SPI, PINS, const BIDI: bool> Write<u8> for Spi<SPI, PINS, BIDI, u8>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.spi_write(words.iter().copied())
        }
    }

    impl<SPI, PINS, const BIDI: bool> WriteIter<u8> for Spi<SPI, PINS, BIDI, u8>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            self.spi_write(words)
        }
    }

    impl<SPI, PINS, const BIDI: bool> Write<u16> for Spi<SPI, PINS, BIDI, u16>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
            self.spi_write(words.iter().copied())
        }
    }

    impl<SPI, PINS, const BIDI: bool> WriteIter<u16> for Spi<SPI, PINS, BIDI, u16>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u16>,
        {
            for word in words.into_iter() {
                nb::block!(self.send(word))?;
                if !BIDI {
                    nb::block!(self.read())?;
                }
            }

            Ok(())
        }
    }

    impl<SPI, PINS, const BIDI: bool, W: Copy + 'static> Transactional<W> for Spi<SPI, PINS, BIDI, W>
    where
        Self: Transfer<W, Error = Error> + Write<W, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Error> {
            for op in operations {
                match op {
                    Operation::Write(w) => self.write(w)?,
                    Operation::Transfer(t) => self.transfer(t).map(|_| ())?,
                }
            }

            Ok(())
        }
    }
}
