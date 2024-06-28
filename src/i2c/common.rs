#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Address {
    Seven(u8),
    Ten(u16),
}

impl From<u8> for Address {
    fn from(value: u8) -> Self {
        Self::Seven(value)
    }
}

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self::Ten(value)
    }
}

pub use embedded_hal::i2c::NoAcknowledgeSource;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    Overrun,
    NoAcknowledge(NoAcknowledgeSource),
    Timeout,
    // Note: The Bus error type is not currently returned, but is maintained for compatibility.
    Bus,
    Crc,
    ArbitrationLoss,
}

impl Error {
    pub(crate) fn nack_addr(self) -> Self {
        match self {
            Error::NoAcknowledge(NoAcknowledgeSource::Unknown) => {
                Error::NoAcknowledge(NoAcknowledgeSource::Address)
            }
            e => e,
        }
    }
    pub(crate) fn nack_data(self) -> Self {
        match self {
            Error::NoAcknowledge(NoAcknowledgeSource::Unknown) => {
                Error::NoAcknowledge(NoAcknowledgeSource::Data)
            }
            e => e,
        }
    }
}

use embedded_hal::i2c::ErrorKind;
impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match *self {
            Self::Overrun => ErrorKind::Overrun,
            Self::Bus => ErrorKind::Bus,
            Self::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Self::NoAcknowledge(nack) => ErrorKind::NoAcknowledge(nack),
            Self::Crc | Self::Timeout => ErrorKind::Other,
        }
    }
}

pub(crate) type Hal1Operation<'a> = embedded_hal::i2c::Operation<'a>;
pub(crate) type Hal02Operation<'a> = embedded_hal_02::blocking::i2c::Operation<'a>;
