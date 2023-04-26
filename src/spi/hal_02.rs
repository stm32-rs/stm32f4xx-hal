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

    impl<SPI, const BIDI: bool, W: FrameSize> FullDuplex<W> for Spi<SPI, BIDI, W>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn read(&mut self) -> nb::Result<W, Error> {
            self.read_nonblocking()
        }

        fn send(&mut self, byte: W) -> nb::Result<(), Error> {
            self.write_nonblocking(byte)
        }
    }
}

mod blocking {
    use super::super::{Error, Instance, Spi};
    use embedded_hal::blocking::spi::{Operation, Transactional, Transfer, Write, WriteIter};

    impl<SPI, const BIDI: bool> Transfer<u8> for Spi<SPI, BIDI, u8>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
            self.transfer_in_place(words)?;

            Ok(words)
        }
    }

    impl<SPI, const BIDI: bool> Transfer<u16> for Spi<SPI, BIDI, u16>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u16]) -> Result<&'w [u16], Self::Error> {
            self.transfer_in_place(words)?;
            Ok(words)
        }
    }

    impl<SPI, const BIDI: bool> Write<u8> for Spi<SPI, BIDI, u8>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.write(words)
        }
    }

    impl<SPI, const BIDI: bool> WriteIter<u8> for Spi<SPI, BIDI, u8>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            for word in words.into_iter() {
                nb::block!(self.write_nonblocking(word))?;
                if !BIDI {
                    nb::block!(self.read_nonblocking())?;
                }
            }

            Ok(())
        }
    }

    impl<SPI, const BIDI: bool> Write<u16> for Spi<SPI, BIDI, u16>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
            self.write(words)
        }
    }

    impl<SPI, const BIDI: bool> WriteIter<u16> for Spi<SPI, BIDI, u16>
    where
        SPI: Instance,
    {
        type Error = Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u16>,
        {
            for word in words.into_iter() {
                nb::block!(self.write_nonblocking(word))?;
                if !BIDI {
                    nb::block!(self.read_nonblocking())?;
                }
            }

            Ok(())
        }
    }

    impl<SPI, const BIDI: bool, W: Copy + 'static> Transactional<W> for Spi<SPI, BIDI, W>
    where
        Self: Transfer<W, Error = Error> + Write<W, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn exec(&mut self, operations: &mut [Operation<'_, W>]) -> Result<(), Error> {
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
