mod blocking {
    use super::super::{Error, I2c, Instance};
    use embedded_hal::blocking::i2c::{Read, Write, WriteIter, WriteIterRead, WriteRead};

    impl<I2C> WriteRead for I2c<I2C>
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

    impl<I2C> WriteIterRead for I2c<I2C>
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

    impl<I2C> Write for I2c<I2C>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write(addr, bytes)
        }
    }

    impl<I2C> WriteIter for I2c<I2C>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.write_iter(addr, bytes)
        }
    }

    impl<I2C> Read for I2c<I2C>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read(addr, buffer)
        }
    }
}
