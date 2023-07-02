pub use embedded_hal_one::spi::{Error, ErrorKind, ErrorType, Mode, Phase, Polarity};

use super::Instance;

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

impl<SPI: Instance, const BIDI: bool, W> ErrorType for super::Spi<SPI, BIDI, W> {
    type Error = super::Error;
}

mod nb {
    use super::super::{Error, FrameSize, Instance, Spi};
    use embedded_hal_nb::spi::FullDuplex;

    impl<SPI, const BIDI: bool, W: FrameSize> FullDuplex<W> for Spi<SPI, BIDI, W>
    where
        SPI: Instance,
    {
        fn read(&mut self) -> nb::Result<W, Error> {
            self.read_nonblocking()
        }

        fn write(&mut self, byte: W) -> nb::Result<(), Error> {
            self.write_nonblocking(byte)
        }
    }
}

mod blocking {
    use super::super::{FrameSize, Instance, Spi};
    use embedded_hal_one::spi::{SpiBus, SpiBusFlush, SpiBusRead, SpiBusWrite};

    impl<SPI, const BIDI: bool, W: FrameSize + 'static> SpiBus<W> for Spi<SPI, BIDI, W>
    where
        SPI: Instance,
    {
        fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            self.transfer_in_place(words)
        }

        fn transfer(&mut self, buff: &mut [W], data: &[W]) -> Result<(), Self::Error> {
            self.transfer(buff, data)
        }
    }

    impl<SPI, const BIDI: bool, W> SpiBusFlush for Spi<SPI, BIDI, W>
    where
        SPI: Instance,
    {
        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl<SPI, const BIDI: bool, W: FrameSize + 'static> SpiBusWrite<W> for Spi<SPI, BIDI, W>
    where
        SPI: Instance,
    {
        fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(self.write_nonblocking(*word))?;
                if !BIDI {
                    nb::block!(self.read_nonblocking())?;
                }
            }

            Ok(())
        }
    }

    impl<SPI, const BIDI: bool, W: FrameSize + 'static> SpiBusRead<W> for Spi<SPI, BIDI, W>
    where
        SPI: Instance,
    {
        fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(self.write_nonblocking(W::default()))?;
                *word = nb::block!(self.read_nonblocking())?;
            }

            Ok(())
        }
    }
}
