mod blocking {
    use super::super::{fmpi2c1, Error, FMPI2c};
    use core::ops::Deref;
    use embedded_hal_one::i2c::blocking::{Read, Write, WriteRead};

    impl<I2C, PINS> WriteRead for FMPI2c<I2C, PINS>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        type Error = Error;

        fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
            // Set up current slave address for writing and disable autoending
            self.i2c.cr2.modify(|_, w| {
                w.sadd()
                    .bits(u16::from(addr) << 1)
                    .nbytes()
                    .bits(bytes.len() as u8)
                    .rd_wrn()
                    .clear_bit()
                    .autoend()
                    .clear_bit()
            });

            // Send a START condition
            self.i2c.cr2.modify(|_, w| w.start().set_bit());

            // Wait until the transmit buffer is empty and there hasn't been any error condition
            while {
                let isr = self.i2c.isr.read();
                self.check_and_clear_error_flags(&isr)
                    .map_err(Error::nack_addr)?;
                isr.txis().bit_is_clear() && isr.tc().bit_is_clear()
            } {}

            // Send out all individual bytes
            for c in bytes {
                self.send_byte(*c)?;
            }

            // Wait until data was sent
            while {
                let isr = self.i2c.isr.read();
                self.check_and_clear_error_flags(&isr)
                    .map_err(Error::nack_data)?;
                isr.tc().bit_is_clear()
            } {}

            // Set up current address for reading
            self.i2c.cr2.modify(|_, w| {
                w.sadd()
                    .bits(u16::from(addr) << 1)
                    .nbytes()
                    .bits(buffer.len() as u8)
                    .rd_wrn()
                    .set_bit()
            });

            // Send another START condition
            self.i2c.cr2.modify(|_, w| w.start().set_bit());

            // Send the autoend after setting the start to get a restart
            self.i2c.cr2.modify(|_, w| w.autoend().set_bit());

            // Now read in all bytes
            for c in buffer.iter_mut() {
                *c = self.recv_byte()?;
            }

            // Check and clear flags if they somehow ended up set
            self.check_and_clear_error_flags(&self.i2c.isr.read())
                .map_err(Error::nack_data)?;

            Ok(())
        }
    }

    impl<I2C, PINS> Read for FMPI2c<I2C, PINS>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        type Error = Error;

        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
            // Set up current address for reading
            self.i2c.cr2.modify(|_, w| {
                w.sadd()
                    .bits(u16::from(addr) << 1)
                    .nbytes()
                    .bits(buffer.len() as u8)
                    .rd_wrn()
                    .set_bit()
            });

            // Send a START condition
            self.i2c.cr2.modify(|_, w| w.start().set_bit());

            // Send the autoend after setting the start to get a restart
            self.i2c.cr2.modify(|_, w| w.autoend().set_bit());

            // Now read in all bytes
            for c in buffer.iter_mut() {
                *c = self.recv_byte()?;
            }

            // Check and clear flags if they somehow ended up set
            self.check_and_clear_error_flags(&self.i2c.isr.read())
                .map_err(Error::nack_data)?;

            Ok(())
        }
    }

    impl<I2C, PINS> Write for FMPI2c<I2C, PINS>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        type Error = Error;

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
            // Set up current slave address for writing and enable autoending
            self.i2c.cr2.modify(|_, w| {
                w.sadd()
                    .bits(u16::from(addr) << 1)
                    .nbytes()
                    .bits(bytes.len() as u8)
                    .rd_wrn()
                    .clear_bit()
                    .autoend()
                    .set_bit()
            });

            // Send a START condition
            self.i2c.cr2.modify(|_, w| w.start().set_bit());

            // Send out all individual bytes
            for c in bytes {
                self.send_byte(*c)?;
            }

            // Check and clear flags if they somehow ended up set
            self.check_and_clear_error_flags(&self.i2c.isr.read())
                .map_err(Error::nack_data)?;

            Ok(())
        }
    }
}
