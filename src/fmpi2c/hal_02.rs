mod blocking {
    use super::super::{fmpi2c1, Error, FMPI2c};
    use core::ops::Deref;
    use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

    impl<I2C, PINS> WriteRead for FMPI2c<I2C, PINS>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        type Error = Error;

        fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
            self.write_read(addr, bytes, buffer)
        }
    }

    impl<I2C, PINS> Read for FMPI2c<I2C, PINS>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        type Error = Error;

        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
            self.read(addr, buffer)
        }
    }

    impl<I2C, PINS> Write for FMPI2c<I2C, PINS>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        type Error = Error;

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
            self.write(addr, bytes)
        }
    }
}
