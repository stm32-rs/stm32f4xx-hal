use embedded_hal::i2c::ErrorType;

use super::Instance;

impl<I2C: Instance> ErrorType for super::FMPI2c<I2C> {
    type Error = super::Error;
}

mod blocking {
    use super::super::{fmpi2c1, FMPI2c, Instance};
    use core::ops::Deref;
    use embedded_hal::i2c::Operation;

    impl<I2C: Instance> embedded_hal::i2c::I2c for FMPI2c<I2C>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read(addr, buffer)
        }

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write(addr, bytes)
        }

        fn write_read(
            &mut self,
            addr: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read(addr, bytes, buffer)
        }

        fn transaction(
            &mut self,
            _addr: u8,
            _operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            todo!()
        }
    }
}
