mod blocking {
    use super::super::{Error, I2c, Instance};
    use embedded_hal_02::blocking::i2c::{Read, Write, WriteRead};

    impl<I2C: Instance> WriteRead for I2c<I2C> {
        type Error = Error;

        fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
            self.write_read(addr, bytes, buffer)
        }
    }

    impl<I2C: Instance> Read for I2c<I2C> {
        type Error = Error;

        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
            self.read(addr, buffer)
        }
    }

    impl<I2C: Instance> Write for I2c<I2C> {
        type Error = Error;

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
            self.write(addr, bytes)
        }
    }
}
