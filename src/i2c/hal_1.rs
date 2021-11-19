use embedded_hal_one::i2c::{Error, ErrorKind, NoAcknowledgeSource};

impl Error for super::Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::OVERRUN => ErrorKind::Overrun,
            Self::BUS => ErrorKind::Bus,
            Self::ARBITRATION => ErrorKind::ArbitrationLoss,
            Self::NACK_ADDR => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address),
            Self::NACK_DATA => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Data),
            Self::NACK => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Self::CRC | Self::TIMEOUT => ErrorKind::Other,
        }
    }
}

mod blocking {
    use super::super::{Error, I2c, Instance};
    use embedded_hal_one::i2c::blocking::{Read, Write, WriteIter, WriteIterRead, WriteRead};

    impl<I2C, PINS> WriteRead for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write_read(
            &mut self,
            addr: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read(addr, bytes, buffer)
        }
    }

    impl<I2C, PINS> WriteIterRead for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write_iter_read<B>(
            &mut self,
            addr: u8,
            bytes: B,
            buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.write_iter_read(addr, bytes, buffer)
        }
    }

    impl<I2C, PINS> Write for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write(addr, bytes)
        }
    }

    impl<I2C, PINS> WriteIter for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write_iter<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.write_iter(addr, bytes)
        }
    }

    impl<I2C, PINS> Read for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read(addr, buffer)
        }
    }
}
