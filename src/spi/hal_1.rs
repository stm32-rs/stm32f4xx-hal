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

impl<SPI, PINS, const BIDI: bool> ErrorType for super::Spi<SPI, PINS, BIDI> {
    type Error = super::Error;
}

mod nb {
    use super::super::{Error, Instance, Spi};
    use embedded_hal_one::spi::nb::FullDuplex;

    impl<SPI, PINS, const BIDI: bool> FullDuplex<u8> for Spi<SPI, PINS, BIDI>
    where
        SPI: Instance,
    {
        fn read(&mut self) -> nb::Result<u8, Error> {
            if BIDI {
                self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
            }
            self.check_read()
        }

        fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
            if BIDI {
                self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
            }
            self.check_send(byte)
        }
    }
}

mod blocking {
    use super::super::{Error, Instance, Spi};
    use embedded_hal_one::spi::{
        blocking::{SpiBus, SpiBusFlush, SpiBusRead, SpiBusWrite},
        nb::FullDuplex,
    };

    impl<SPI, PINS, const BIDI: bool, W: Copy + Default + 'static> SpiBus<W> for Spi<SPI, PINS, BIDI>
    where
        Self: FullDuplex<W, Error = Error> + SpiBusWrite<W>,
        SPI: Instance,
    {
        fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(<Self as FullDuplex<W>>::write(self, *word))?;
                *word = nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }

        fn transfer(&mut self, buff: &mut [W], data: &[W]) -> Result<(), Self::Error> {
            assert_eq!(data.len(), buff.len());

            for (d, b) in data.iter().cloned().zip(buff.iter_mut()) {
                nb::block!(<Self as FullDuplex<W>>::write(self, d))?;
                *b = nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, const BIDI: bool> SpiBusFlush for Spi<SPI, PINS, BIDI>
    where
        SPI: Instance,
    {
        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl<SPI, PINS, const BIDI: bool, W: Copy + 'static> SpiBusWrite<W> for Spi<SPI, PINS, BIDI>
    where
        Self: FullDuplex<W, Error = Error>,
        SPI: Instance,
    {
        fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(<Self as FullDuplex<W>>::write(self, *word))?;
                if !BIDI {
                    nb::block!(<Self as FullDuplex<W>>::read(self))?;
                }
            }

            Ok(())
        }
    }

    impl<SPI, PINS, const BIDI: bool, W: Copy + Default + 'static> SpiBusRead<W>
        for Spi<SPI, PINS, BIDI>
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
}
