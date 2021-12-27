mod blocking {
    use super::super::{Error, I2c, I2cCommon, Instance};
    use embedded_hal::blocking::i2c::{Read, Write, WriteIter, WriteIterRead, WriteRead};

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
            self.write_bytes(addr, bytes.iter().cloned())?;
            self.read(addr, buffer)?;

            Ok(())
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
            self.write_bytes(addr, bytes.into_iter())?;
            self.read(addr, buffer)?;

            Ok(())
        }
    }

    impl<I2C, PINS> Write for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write_bytes(addr, bytes.iter().cloned())?;

            // Send a STOP condition
            self.i2c.cr1.modify(|_, w| w.stop().set_bit());

            // Wait for STOP condition to transmit.
            while self.i2c.cr1.read().stop().bit_is_set() {}

            // Fallthrough is success
            Ok(())
        }
    }

    impl<I2C, PINS> WriteIter for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn write<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.write_bytes(addr, bytes.into_iter())?;

            // Send a STOP condition
            self.i2c.cr1.modify(|_, w| w.stop().set_bit());

            // Wait for STOP condition to transmit.
            while self.i2c.cr1.read().stop().bit_is_set() {}

            // Fallthrough is success
            Ok(())
        }
    }

    impl<I2C, PINS> Read for I2c<I2C, PINS>
    where
        I2C: Instance,
    {
        type Error = Error;

        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            if let Some((last, buffer)) = buffer.split_last_mut() {
                // Send a START condition and set ACK bit
                self.i2c
                    .cr1
                    .modify(|_, w| w.start().set_bit().ack().set_bit());

                // Wait until START condition was generated
                while self.i2c.sr1.read().sb().bit_is_clear() {}

                // Also wait until signalled we're master and everything is waiting for us
                while {
                    let sr2 = self.i2c.sr2.read();
                    sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
                } {}

                // Set up current address, we're trying to talk to
                self.i2c
                    .dr
                    .write(|w| unsafe { w.bits((u32::from(addr) << 1) + 1) });

                // Wait until address was sent
                loop {
                    self.check_and_clear_error_flags()?;
                    if self.i2c.sr1.read().addr().bit_is_set() {
                        break;
                    }
                }

                // Clear condition by reading SR2
                self.i2c.sr2.read();

                // Receive bytes into buffer
                for c in buffer {
                    *c = self.recv_byte()?;
                }

                // Prepare to send NACK then STOP after next byte
                self.i2c
                    .cr1
                    .modify(|_, w| w.ack().clear_bit().stop().set_bit());

                // Receive last byte
                *last = self.recv_byte()?;

                // Wait for the STOP to be sent.
                while self.i2c.cr1.read().stop().bit_is_set() {}

                // Fallthrough is success
                Ok(())
            } else {
                Err(Error::OVERRUN)
            }
        }
    }
}
