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

impl<SPI, PINS, const BIDI: bool, W> ErrorType for super::Spi<SPI, PINS, BIDI, W> {
    type Error = super::Error;
}

mod nb {
    use super::super::{Error, FrameSize, Instance, Spi};
    use embedded_hal_one::spi::nb::FullDuplex;

    impl<SPI, PINS, const BIDI: bool, W: FrameSize> FullDuplex<W> for Spi<SPI, PINS, BIDI, W>
    where
        SPI: Instance,
    {
        fn read(&mut self) -> nb::Result<W, Error> {
            if BIDI {
                self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
            }
            self.check_read()
        }

        fn write(&mut self, byte: W) -> nb::Result<(), Error> {
            if BIDI {
                self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
            }
            self.check_send(byte)
        }
    }
}

mod blocking {
    use super::super::{FrameSize, Instance, Spi};
    use embedded_hal_one::spi::{
        blocking::{SpiBus, SpiBusFlush, SpiBusRead, SpiBusWrite},
        nb::FullDuplex,
    };

    impl<SPI, PINS, const BIDI: bool, W: FrameSize + 'static> SpiBus<W> for Spi<SPI, PINS, BIDI, W>
    where
        SPI: Instance,
    {
        fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(<Self as FullDuplex<W>>::write(self, *word))?;
                *word = nb::block!(<Self as FullDuplex<W>>::read(self))?;
            }

            Ok(())
        }

        fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error> {
            let mut iter_r = read.iter_mut();
            let mut iter_w = write.iter().cloned();
            loop {
                match (iter_r.next(), iter_w.next()) {
                    (Some(r), Some(w)) => {
                        nb::block!(<Self as FullDuplex<W>>::write(self, w))?;
                        *r = nb::block!(<Self as FullDuplex<W>>::read(self))?;
                    }
                    (Some(r), None) => {
                        nb::block!(<Self as FullDuplex<W>>::write(self, W::default()))?;
                        *r = nb::block!(<Self as FullDuplex<W>>::read(self))?;
                    }
                    (None, Some(w)) => {
                        nb::block!(<Self as FullDuplex<W>>::write(self, w))?;
                        let _ = nb::block!(<Self as FullDuplex<W>>::read(self))?;
                    }
                    (None, None) => break,
                }
            }
            Ok(())
        }
    }

    impl<SPI, PINS, const BIDI: bool, W> SpiBusFlush for Spi<SPI, PINS, BIDI, W>
    where
        SPI: Instance,
    {
        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl<SPI, PINS, const BIDI: bool, W: FrameSize + 'static> SpiBusWrite<W> for Spi<SPI, PINS, BIDI, W>
    where
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

    impl<SPI, PINS, const BIDI: bool, W: FrameSize + 'static> SpiBusRead<W> for Spi<SPI, PINS, BIDI, W>
    where
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
