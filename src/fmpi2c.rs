use core::ops::Deref;

use crate::gpio;

use crate::pac::fmpi2c1 as i2c1;
use crate::pac::{self, RCC};
use crate::rcc::{BusClock, Enable, Reset};
use fugit::{HertzU32 as Hertz, RateExtU32};

#[path = "i2c/common.rs"]
mod common;
pub use common::{Address, Error, NoAcknowledgeSource};
use common::{Hal02Operation, Hal1Operation};

// Old names
pub use I2c as FmpI2c;
pub use Mode as FmpMode;

#[path = "i2c/hal_02.rs"]
mod hal_02;
#[path = "i2c/hal_1.rs"]
mod hal_1;

pub trait Instance:
    crate::Sealed
    + crate::Ptr<RB = i2c1::RegisterBlock>
    + Deref<Target = Self::RB>
    + Enable
    + Reset
    + BusClock
    + gpio::alt::I2cCommon
{
    fn clock_hsi(rcc: &crate::pac::rcc::RegisterBlock);
}

macro_rules! i2c {
    ($I2C:ty, $i2csel:ident, $I2Calias:ident) => {
        pub type $I2Calias = I2c<$I2C>;

        impl Instance for $I2C {
            fn clock_hsi(rcc: &crate::pac::rcc::RegisterBlock) {
                rcc.dckcfgr2().modify(|_, w| w.$i2csel().hsi());
            }
        }
    };
}

#[cfg(feature = "fmpi2c1")]
i2c!(pac::FMPI2C1, fmpi2c1sel, FMPI2c1);

/// I2C FastMode+ abstraction
pub struct I2c<I2C: Instance> {
    i2c: I2C,
    pins: (I2C::Scl, I2C::Sda),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Standard { frequency: Hertz },
    Fast { frequency: Hertz },
    FastPlus { frequency: Hertz },
}

impl Mode {
    pub fn standard(frequency: Hertz) -> Self {
        Self::Standard { frequency }
    }

    pub fn fast(frequency: Hertz) -> Self {
        Self::Fast { frequency }
    }

    pub fn fast_plus(frequency: Hertz) -> Self {
        Self::FastPlus { frequency }
    }

    pub fn get_frequency(&self) -> Hertz {
        match *self {
            Self::Standard { frequency } => frequency,
            Self::Fast { frequency } => frequency,
            Self::FastPlus { frequency } => frequency,
        }
    }
}

impl From<Hertz> for Mode {
    fn from(frequency: Hertz) -> Self {
        let k100: Hertz = 100.kHz();
        let k400: Hertz = 400.kHz();
        if frequency <= k100 {
            Self::Standard { frequency }
        } else if frequency <= k400 {
            Self::Fast { frequency }
        } else {
            Self::FastPlus { frequency }
        }
    }
}

pub trait I2cExt: Sized + Instance {
    fn i2c(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
    ) -> I2c<Self>;
}

impl<I2C: Instance> I2cExt for I2C {
    fn i2c(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
    ) -> I2c<Self> {
        I2c::new(self, pins, mode)
    }
}

impl<I2C: Instance> I2c<I2C> {
    pub fn new(
        i2c: I2C,
        pins: (impl Into<I2C::Scl>, impl Into<I2C::Sda>),
        mode: impl Into<Mode>,
    ) -> Self {
        unsafe {
            // Enable and reset clock.
            I2C::enable_unchecked();
            I2C::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(mode);
        i2c
    }

    pub fn release(self) -> (I2C, (I2C::Scl, I2C::Sda)) {
        (self.i2c, self.pins)
    }
}

impl<I2C: Instance> I2c<I2C> {
    fn i2c_init(&self, mode: impl Into<Mode>) {
        let mode = mode.into();

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1().modify(|_, w| w.pe().clear_bit());

        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
        let rcc = unsafe { &(*RCC::ptr()) };
        I2C::clock_hsi(rcc);

        // Calculate settings for I2C speed modes
        let presc;
        let scldel;
        let sdadel;
        let sclh;
        let scll;

        // We're using the HSI clock to keep things simple so this is going to be always 16 MHz
        const FREQ: u32 = 16_000_000;

        // Normal I2C speeds use a different scaling than fast mode below and fast mode+ even more
        // below
        match mode {
            Mode::Standard { frequency } => {
                presc = 3;
                scll = crate::max_u32((((FREQ >> presc) >> 1) / frequency.raw()) - 1, 255) as u8;
                sclh = scll - 4;
                sdadel = 2;
                scldel = 4;
            }
            Mode::Fast { frequency } => {
                presc = 1;
                scll = crate::max_u32((((FREQ >> presc) >> 1) / frequency.raw()) - 1, 255) as u8;
                sclh = scll - 6;
                sdadel = 2;
                scldel = 3;
            }
            Mode::FastPlus { frequency } => {
                presc = 0;
                scll = crate::max_u32((((FREQ >> presc) >> 1) / frequency.raw()) - 4, 255) as u8;
                sclh = scll - 2;
                sdadel = 0;
                scldel = 2;
            }
        }

        // Enable I2C signal generator, and configure I2C for configured speed
        self.i2c.timingr().write(|w| {
            w.presc().set(presc);
            w.scldel().set(scldel);
            w.sdadel().set(sdadel);
            w.sclh().set(sclh);
            w.scll().set(scll)
        });

        // Enable the I2C processing
        self.i2c.cr1().modify(|_, w| w.pe().set_bit());
    }

    #[inline(always)]
    fn check_and_clear_error_flags(&self, isr: &i2c1::isr::R) -> Result<(), Error> {
        // If we received a NACK, then this is an error
        if isr.nackf().bit_is_set() {
            self.i2c
                .icr()
                .write(|w| w.stopcf().clear_bit_by_one().nackcf().clear_bit_by_one());
            return Err(Error::NoAcknowledge(NoAcknowledgeSource::Unknown));
        }

        Ok(())
    }

    #[inline(always)]
    fn end_transaction(&self) -> Result<(), Error> {
        // Check and clear flags if they somehow ended up set
        self.check_and_clear_error_flags(&self.i2c.isr().read())
            .map_err(Error::nack_data)?;
        Ok(())
    }

    /// Sends START and Address for writing
    #[inline(always)]
    fn prepare_write(&self, addr: Address, datalen: usize) -> Result<(), Error> {
        // Set up current slave address for writing and disable autoending
        self.i2c.cr2().modify(|_, w| {
            match addr {
                Address::Seven(addr) => {
                    w.add10().clear_bit();
                    w.sadd().set(u16::from(addr) << 1);
                }
                Address::Ten(addr) => {
                    w.add10().set_bit();
                    w.sadd().set(addr);
                }
            }
            w.nbytes().set(datalen as u8);
            w.rd_wrn().clear_bit();
            w.autoend().clear_bit()
        });

        // Send a START condition
        self.i2c.cr2().modify(|_, w| w.start().set_bit());

        // Wait until address was sent
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_addr)?;
            isr.txis().bit_is_clear() && isr.tc().bit_is_clear()
        } {}

        Ok(())
    }

    /// Sends START and Address for reading
    fn prepare_read(
        &self,
        addr: Address,
        buflen: usize,
        first_transaction: bool,
    ) -> Result<(), Error> {
        // Set up current address for reading
        self.i2c.cr2().modify(|_, w| {
            match addr {
                Address::Seven(addr) => {
                    w.add10().clear_bit();
                    w.sadd().set(u16::from(addr) << 1);
                }
                Address::Ten(addr) => {
                    w.add10().set_bit();
                    w.head10r().bit(!first_transaction);
                    w.sadd().set(addr);
                }
            }
            w.nbytes().set(buflen as u8);
            w.rd_wrn().set_bit()
        });

        // Send a START condition
        self.i2c.cr2().modify(|_, w| w.start().set_bit());

        // Send the autoend after setting the start to get a restart
        self.i2c.cr2().modify(|_, w| w.autoend().set_bit());

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
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_addr)?;
            isr.txis().bit_is_clear()
        } {}

        // Push out a byte of data
        self.i2c
            .txdr()
            .write(|w| unsafe { w.bits(u32::from(byte)) });

        self.end_transaction()
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_data)?;
            isr.rxne().bit_is_clear()
        } {}

        let value = self.i2c.rxdr().read().bits() as u8;
        Ok(value)
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        // Receive bytes into buffer
        for c in buffer {
            *c = self.recv_byte()?;
        }

        Ok(())
    }

    pub fn read(&mut self, addr: impl Into<Address>, buffer: &mut [u8]) -> Result<(), Error> {
        self.prepare_read(addr.into(), buffer.len(), true)?;
        self.read_bytes(buffer)?;

        self.end_transaction()
    }

    pub fn write(&mut self, addr: impl Into<Address>, bytes: &[u8]) -> Result<(), Error> {
        self.prepare_write(addr.into(), bytes.len())?;
        self.write_bytes(bytes.iter().cloned())?;

        self.end_transaction()
    }

    pub fn write_read(
        &mut self,
        addr: impl Into<Address>,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        let addr = addr.into();
        self.prepare_write(addr, bytes.len())?;
        self.write_bytes(bytes.iter().cloned())?;

        // Wait until data was sent
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_data)?;
            isr.tc().bit_is_clear()
        } {}

        self.read(addr, buffer)
    }

    pub fn transaction<'a>(
        &mut self,
        addr: impl Into<Address>,
        mut ops: impl Iterator<Item = Hal1Operation<'a>>,
    ) -> Result<(), Error> {
        let addr = addr.into();
        if let Some(mut prev_op) = ops.next() {
            // 1. Generate Start for operation
            match &prev_op {
                Hal1Operation::Read(buf) => self.prepare_read(addr, buf.len(), true)?,
                Hal1Operation::Write(data) => self.prepare_write(addr, data.len())?,
            };

            for op in ops {
                // 2. Execute previous operations.
                match &mut prev_op {
                    Hal1Operation::Read(rb) => self.read_bytes(rb)?,
                    Hal1Operation::Write(wb) => self.write_bytes(wb.iter().cloned())?,
                };
                // 3. If operation changes type we must generate new start
                match (&prev_op, &op) {
                    (Hal1Operation::Read(_), Hal1Operation::Write(data)) => {
                        self.prepare_write(addr, data.len())?
                    }
                    (Hal1Operation::Write(_), Hal1Operation::Read(buf)) => {
                        self.prepare_read(addr, buf.len(), false)?
                    }
                    _ => {} // No changes if operation have not changed
                }

                prev_op = op;
            }

            // 4. Now, prev_op is last command use methods variations that will generate stop
            match prev_op {
                Hal1Operation::Read(rb) => self.read_bytes(rb)?,
                Hal1Operation::Write(wb) => self.write_bytes(wb.iter().cloned())?,
            };

            self.end_transaction()?;
        }

        // Fallthrough is success
        Ok(())
    }

    pub fn transaction_slice(
        &mut self,
        addr: impl Into<Address>,
        ops_slice: &mut [Hal1Operation<'_>],
    ) -> Result<(), Error> {
        let addr = addr.into();
        transaction_impl!(self, addr, ops_slice, Hal1Operation);
        // Fallthrough is success
        Ok(())
    }

    fn transaction_slice_hal_02(
        &mut self,
        addr: impl Into<Address>,
        ops_slice: &mut [Hal02Operation<'_>],
    ) -> Result<(), Error> {
        let addr = addr.into();
        transaction_impl!(self, addr, ops_slice, Hal02Operation);
        // Fallthrough is success
        Ok(())
    }
}

macro_rules! transaction_impl {
    ($self:ident, $addr:ident, $ops_slice:ident, $Operation:ident) => {
        let i2c = $self;
        let addr = $addr;
        let mut ops = $ops_slice.iter_mut();

        if let Some(mut prev_op) = ops.next() {
            // 1. Generate Start for operation
            match &prev_op {
                $Operation::Read(buf) => i2c.prepare_read(addr, buf.len(), true)?,
                $Operation::Write(data) => i2c.prepare_write(addr, data.len())?,
            };

            for op in ops {
                // 2. Execute previous operations.
                match &mut prev_op {
                    $Operation::Read(rb) => i2c.read_bytes(rb)?,
                    $Operation::Write(wb) => i2c.write_bytes(wb.iter().cloned())?,
                };
                // 3. If operation changes type we must generate new start
                match (&prev_op, &op) {
                    ($Operation::Read(_), $Operation::Write(data)) => {
                        i2c.prepare_write(addr, data.len())?
                    }
                    ($Operation::Write(_), $Operation::Read(buf)) => {
                        i2c.prepare_read(addr, buf.len(), false)?
                    }
                    _ => {} // No changes if operation have not changed
                }

                prev_op = op;
            }

            // 4. Now, prev_op is last command use methods variations that will generate stop
            match prev_op {
                $Operation::Read(rb) => i2c.read_bytes(rb)?,
                $Operation::Write(wb) => i2c.write_bytes(wb.iter().cloned())?,
            };

            i2c.end_transaction()?;
        }
    };
}
use transaction_impl;

// Note: implementation is from f0xx-hal
// TODO: check error handling. See https://github.com/stm32-rs/stm32f0xx-hal/pull/95/files
