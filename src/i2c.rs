use core::ops::Deref;

use crate::pac::{self, i2c1};
use crate::rcc::{Enable, Reset};

use crate::gpio;

use crate::rcc::Clocks;
use embedded_hal_one::i2c::blocking::Operation;
use fugit::{HertzU32 as Hertz, RateExtU32};

mod hal_02;
mod hal_1;

#[cfg(feature = "dma")]
pub mod dma;

#[derive(Debug, Eq, PartialEq)]
pub enum DutyCycle {
    Ratio2to1,
    Ratio16to9,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Standard {
        frequency: Hertz,
    },
    Fast {
        frequency: Hertz,
        duty_cycle: DutyCycle,
    },
}

impl Mode {
    pub fn standard(frequency: Hertz) -> Self {
        Self::Standard { frequency }
    }

    pub fn fast(frequency: Hertz, duty_cycle: DutyCycle) -> Self {
        Self::Fast {
            frequency,
            duty_cycle,
        }
    }

    pub fn get_frequency(&self) -> Hertz {
        match *self {
            Self::Standard { frequency } => frequency,
            Self::Fast { frequency, .. } => frequency,
        }
    }
}

impl From<Hertz> for Mode {
    fn from(frequency: Hertz) -> Self {
        let k100: Hertz = 100.kHz();
        if frequency <= k100 {
            Self::Standard { frequency }
        } else {
            Self::Fast {
                frequency,
                duty_cycle: DutyCycle::Ratio2to1,
            }
        }
    }
}

/// I2C abstraction
pub struct I2c<I2C: Instance> {
    i2c: I2C,
    pins: (I2C::Scl, I2C::Sda),
}

pub use embedded_hal_one::i2c::NoAcknowledgeSource;

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

pub trait Instance:
    crate::Sealed + Deref<Target = i2c1::RegisterBlock> + Enable + Reset + gpio::alt::I2cCommon
{
    #[doc(hidden)]
    fn ptr() -> *const i2c1::RegisterBlock;
}

// Implemented by all I2C instances
macro_rules! i2c {
    ($I2C:ty: $I2c:ident) => {
        pub type $I2c = I2c<$I2C>;

        impl Instance for $I2C {
            fn ptr() -> *const i2c1::RegisterBlock {
                <$I2C>::ptr() as *const _
            }
        }
    };
}

i2c! { pac::I2C1: I2c1 }
#[cfg(feature = "i2c2")]
i2c! { pac::I2C2: I2c2 }
#[cfg(feature = "i2c3")]
i2c! { pac::I2C3: I2c3 }
#[cfg(feature = "i2c4")]
i2c! { pac::I2C4: I2c4 }
#[cfg(feature = "i2c5")]
i2c! { pac::I2C5: I2c5 }
#[cfg(feature = "i2c6")]
i2c! { pac::I2C6: I2c6 }

pub trait I2cExt: Sized + Instance {
    fn i2c(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> I2c<Self>;
}

impl<I2C: Instance> I2cExt for I2C {
    fn i2c(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> I2c<Self> {
        I2c::new(self, pins, mode, clocks)
    }
}

impl<I2C> I2c<I2C>
where
    I2C: Instance,
{
    pub fn new(
        i2c: I2C,
        pins: (impl Into<I2C::Scl>, impl Into<I2C::Sda>),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> Self {
        unsafe {
            // Enable and reset clock.
            I2C::enable_unchecked();
            I2C::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(mode, clocks.pclk1());
        i2c
    }

    pub fn release(self) -> (I2C, (I2C::Scl, I2C::Sda)) {
        (self.i2c, self.pins)
    }
}

impl<I2C: Instance> I2c<I2C> {
    fn i2c_init(&self, mode: impl Into<Mode>, pclk: Hertz) {
        let mode = mode.into();
        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Calculate settings for I2C speed modes
        let clock = pclk.raw();
        let clc_mhz = clock / 1_000_000;
        assert!((2..=50).contains(&clc_mhz));

        // Configure bus frequency into I2C peripheral
        self.i2c
            .cr2
            .write(|w| unsafe { w.freq().bits(clc_mhz as u8) });

        let trise = match mode {
            Mode::Standard { .. } => clc_mhz + 1,
            Mode::Fast { .. } => clc_mhz * 300 / 1000 + 1,
        };

        // Configure correct rise times
        self.i2c.trise.write(|w| w.trise().bits(trise as u8));

        match mode {
            // I2C clock control calculation
            Mode::Standard { frequency } => {
                let ccr = (clock / (frequency.raw() * 2)).max(4);

                // Set clock to standard mode with appropriate parameters for selected speed
                self.i2c.ccr.write(|w| unsafe {
                    w.f_s()
                        .clear_bit()
                        .duty()
                        .clear_bit()
                        .ccr()
                        .bits(ccr as u16)
                });
            }
            Mode::Fast {
                frequency,
                duty_cycle,
            } => match duty_cycle {
                DutyCycle::Ratio2to1 => {
                    let ccr = (clock / (frequency.raw() * 3)).max(1);

                    // Set clock to fast mode with appropriate parameters for selected speed (2:1 duty cycle)
                    self.i2c.ccr.write(|w| unsafe {
                        w.f_s().set_bit().duty().clear_bit().ccr().bits(ccr as u16)
                    });
                }
                DutyCycle::Ratio16to9 => {
                    let ccr = (clock / (frequency.raw() * 25)).max(1);

                    // Set clock to fast mode with appropriate parameters for selected speed (16:9 duty cycle)
                    self.i2c.ccr.write(|w| unsafe {
                        w.f_s().set_bit().duty().set_bit().ccr().bits(ccr as u16)
                    });
                }
            },
        }

        // Enable the I2C processing
        self.i2c.cr1.modify(|_, w| w.pe().set_bit());
    }

    fn check_and_clear_error_flags(&self) -> Result<i2c1::sr1::R, Error> {
        // Note that flags should only be cleared once they have been registered. If flags are
        // cleared otherwise, there may be an inherent race condition and flags may be missed.
        let sr1 = self.i2c.sr1.read();

        if sr1.timeout().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.timeout().clear_bit());
            return Err(Error::Timeout);
        }

        if sr1.pecerr().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.pecerr().clear_bit());
            return Err(Error::Crc);
        }

        if sr1.ovr().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.ovr().clear_bit());
            return Err(Error::Overrun);
        }

        if sr1.af().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.af().clear_bit());
            return Err(Error::NoAcknowledge(NoAcknowledgeSource::Unknown));
        }

        if sr1.arlo().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.arlo().clear_bit());
            return Err(Error::ArbitrationLoss);
        }

        // The errata indicates that BERR may be incorrectly detected. It recommends ignoring and
        // clearing the BERR bit instead.
        if sr1.berr().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.berr().clear_bit());
        }

        Ok(sr1)
    }

    /// Sends START and Address for writing
    #[inline(always)]
    fn prepare_write(&self, addr: u8) -> Result<(), Error> {
        // Send a START condition
        self.i2c.cr1.modify(|_, w| w.start().set_bit());

        // Wait until START condition was generated
        while self.check_and_clear_error_flags()?.sb().bit_is_clear() {}

        // Also wait until signalled we're master and everything is waiting for us
        loop {
            self.check_and_clear_error_flags()?;

            let sr2 = self.i2c.sr2.read();
            if !(sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()) {
                break;
            }
        }

        // Set up current address, we're trying to talk to
        self.i2c
            .dr
            .write(|w| unsafe { w.bits(u32::from(addr) << 1) });

        // Wait until address was sent
        loop {
            // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
            let sr1 = self
                .check_and_clear_error_flags()
                .map_err(Error::nack_addr)?;

            // Wait for the address to be acknowledged
            if sr1.addr().bit_is_set() {
                break;
            }
        }

        // Clear condition by reading SR2
        self.i2c.sr2.read();

        Ok(())
    }

    /// Sends START and Address for reading
    fn prepare_read(&self, addr: u8) -> Result<(), Error> {
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
            self.check_and_clear_error_flags()
                .map_err(Error::nack_addr)?;
            if self.i2c.sr1.read().addr().bit_is_set() {
                break;
            }
        }

        // Clear condition by reading SR2
        self.i2c.sr2.read();

        Ok(())
    }

    fn write_bytes(&mut self, bytes: impl Iterator<Item = u8>) -> Result<(), Error> {
        // Send bytes
        for c in bytes {
            self.send_byte(c)?;
        }

        // Fallthrough is success
        Ok(())
    }

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
        while self
            .check_and_clear_error_flags()
            .map_err(Error::nack_addr)?
            .tx_e()
            .bit_is_clear()
        {}

        // Push out a byte of data
        self.i2c.dr.write(|w| unsafe { w.bits(u32::from(byte)) });

        // Wait until byte is transferred
        // Check for any potential error conditions.
        while self
            .check_and_clear_error_flags()
            .map_err(Error::nack_data)?
            .btf()
            .bit_is_clear()
        {}

        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        loop {
            // Check for any potential error conditions.
            self.check_and_clear_error_flags()
                .map_err(Error::nack_data)?;

            if self.i2c.sr1.read().rx_ne().bit_is_set() {
                break;
            }
        }

        let value = self.i2c.dr.read().bits() as u8;
        Ok(value)
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        // Receive bytes into buffer
        for c in buffer {
            *c = self.recv_byte()?;
        }

        Ok(())
    }

    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        if buffer.is_empty() {
            return Err(Error::Overrun);
        }

        self.prepare_read(addr)?;
        self.read_wo_prepare(buffer)
    }

    /// Reads like normal but does'n generate start and don't send address
    fn read_wo_prepare(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        if let Some((last, buffer)) = buffer.split_last_mut() {
            // Read all bytes but not last
            self.read_bytes(buffer)?;

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
            Err(Error::Overrun)
        }
    }

    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.prepare_write(addr)?;
        self.write_wo_prepare(bytes)
    }

    /// Writes like normal but does'n generate start and don't send address
    fn write_wo_prepare(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.write_bytes(bytes.iter().cloned())?;

        // Send a STOP condition
        self.i2c.cr1.modify(|_, w| w.stop().set_bit());

        // Wait for STOP condition to transmit.
        while self.i2c.cr1.read().stop().bit_is_set() {}

        // Fallthrough is success
        Ok(())
    }

    pub fn write_iter<B>(&mut self, addr: u8, bytes: B) -> Result<(), Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.prepare_write(addr)?;
        self.write_bytes(bytes.into_iter())?;

        // Send a STOP condition
        self.i2c.cr1.modify(|_, w| w.stop().set_bit());

        // Wait for STOP condition to transmit.
        while self.i2c.cr1.read().stop().bit_is_set() {}

        // Fallthrough is success
        Ok(())
    }

    pub fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.prepare_write(addr)?;
        self.write_bytes(bytes.iter().cloned())?;
        self.read(addr, buffer)
    }

    pub fn write_iter_read<B>(&mut self, addr: u8, bytes: B, buffer: &mut [u8]) -> Result<(), Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.prepare_write(addr)?;
        self.write_bytes(bytes.into_iter())?;
        self.read(addr, buffer)
    }

    pub fn transaction<'a>(
        &mut self,
        addr: u8,
        mut ops: impl Iterator<Item = Operation<'a>>,
    ) -> Result<(), Error> {
        if let Some(mut prev_op) = ops.next() {
            // 1. Generate Start for operation
            match &prev_op {
                Operation::Read(_) => self.prepare_read(addr)?,
                Operation::Write(_) => self.prepare_write(addr)?,
            };

            for op in ops {
                // 2. Execute previous operations.
                match &mut prev_op {
                    Operation::Read(rb) => self.read_bytes(rb)?,
                    Operation::Write(wb) => self.write_bytes(wb.iter().cloned())?,
                };
                // 3. If operation changes type we must generate new start
                match (&prev_op, &op) {
                    (Operation::Read(_), Operation::Write(_)) => self.prepare_write(addr)?,
                    (Operation::Write(_), Operation::Read(_)) => self.prepare_read(addr)?,
                    _ => {} // No changes if operation have not changed
                }

                prev_op = op;
            }

            // 4. Now, prev_op is last command use methods variations that will generate stop
            match prev_op {
                Operation::Read(rb) => self.read_wo_prepare(rb)?,
                Operation::Write(wb) => self.write_wo_prepare(wb)?,
            };
        }

        // Fallthrough is success
        Ok(())
    }

    pub fn transaction_slice(
        &mut self,
        addr: u8,
        ops_slice: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        let mut ops = ops_slice.iter_mut();

        if let Some(mut prev_op) = ops.next() {
            // 1. Generate Start for operation
            match &prev_op {
                Operation::Read(_) => self.prepare_read(addr)?,
                Operation::Write(_) => self.prepare_write(addr)?,
            };

            for op in ops {
                // 2. Execute previous operations.
                match &mut prev_op {
                    Operation::Read(rb) => self.read_bytes(rb)?,
                    Operation::Write(wb) => self.write_bytes(wb.iter().cloned())?,
                };
                // 3. If operation changes type we must generate new start
                match (&prev_op, &op) {
                    (Operation::Read(_), Operation::Write(_)) => self.prepare_write(addr)?,
                    (Operation::Write(_), Operation::Read(_)) => self.prepare_read(addr)?,
                    _ => {} // No changes if operation have not changed
                }

                prev_op = op;
            }

            // 4. Now, prev_op is last command use methods variations that will generate stop
            match prev_op {
                Operation::Read(rb) => self.read_wo_prepare(rb)?,
                Operation::Write(wb) => self.write_wo_prepare(wb)?,
            };
        }

        // Fallthrough is success
        Ok(())
    }
}
