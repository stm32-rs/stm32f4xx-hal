use core::ops::Deref;

use crate::i2c::{Error, Pins};
use crate::pac::{fmpi2c1, FMPI2C1, RCC};
use crate::rcc::{Enable, Reset};
use crate::time::{Hertz, U32Ext};

mod hal_02;
mod hal_1;

/// I2C FastMode+ abstraction
pub struct FMPI2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

pub type FMPI2c1<PINS> = FMPI2c<FMPI2C1, PINS>;

#[derive(Debug, PartialEq)]
pub enum FmpMode {
    Standard { frequency: Hertz },
    Fast { frequency: Hertz },
    FastPlus { frequency: Hertz },
}

impl FmpMode {
    pub fn standard<F: Into<Hertz>>(frequency: F) -> Self {
        Self::Standard {
            frequency: frequency.into(),
        }
    }

    pub fn fast<F: Into<Hertz>>(frequency: F) -> Self {
        Self::Fast {
            frequency: frequency.into(),
        }
    }

    pub fn fast_plus<F: Into<Hertz>>(frequency: F) -> Self {
        Self::FastPlus {
            frequency: frequency.into(),
        }
    }

    pub fn get_frequency(&self) -> Hertz {
        match *self {
            Self::Standard { frequency } => frequency,
            Self::Fast { frequency } => frequency,
            Self::FastPlus { frequency } => frequency,
        }
    }
}

impl<F> From<F> for FmpMode
where
    F: Into<Hertz>,
{
    fn from(frequency: F) -> Self {
        let frequency: Hertz = frequency.into();
        if frequency <= 100_000.hz() {
            Self::Standard { frequency }
        } else if frequency <= 400_000.hz() {
            Self::Fast { frequency }
        } else {
            Self::FastPlus { frequency }
        }
    }
}

impl<PINS> FMPI2c<FMPI2C1, PINS>
where
    PINS: Pins<FMPI2C1>,
{
    pub fn new<M: Into<FmpMode>>(i2c: FMPI2C1, mut pins: PINS, mode: M) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable and reset clock.
            FMPI2C1::enable(rcc);
            FMPI2C1::reset(rcc);

            rcc.dckcfgr2.modify(|_, w| w.fmpi2c1sel().hsi());
        }

        pins.set_alt_mode();

        let i2c = FMPI2c { i2c, pins };
        i2c.i2c_init(mode);
        i2c
    }

    pub fn release(mut self) -> (FMPI2C1, PINS) {
        self.pins.restore_mode();

        (self.i2c, self.pins)
    }
}

impl<I2C, PINS> FMPI2c<I2C, PINS>
where
    I2C: Deref<Target = fmpi2c1::RegisterBlock>,
{
    fn i2c_init<M: Into<FmpMode>>(&self, mode: M) {
        let mode = mode.into();
        use core::cmp;

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1.modify(|_, w| w.pe().clear_bit());

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
            FmpMode::Standard { frequency } => {
                presc = 3;
                scll = cmp::max((((FREQ >> presc) >> 1) / frequency.0) - 1, 255) as u8;
                sclh = scll - 4;
                sdadel = 2;
                scldel = 4;
            }
            FmpMode::Fast { frequency } => {
                presc = 1;
                scll = cmp::max((((FREQ >> presc) >> 1) / frequency.0) - 1, 255) as u8;
                sclh = scll - 6;
                sdadel = 2;
                scldel = 3;
            }
            FmpMode::FastPlus { frequency } => {
                presc = 0;
                scll = cmp::max((((FREQ >> presc) >> 1) / frequency.0) - 4, 255) as u8;
                sclh = scll - 2;
                sdadel = 0;
                scldel = 2;
            }
        }

        // Enable I2C signal generator, and configure I2C for configured speed
        self.i2c.timingr.write(|w| {
            w.presc()
                .bits(presc)
                .scldel()
                .bits(scldel)
                .sdadel()
                .bits(sdadel)
                .sclh()
                .bits(sclh)
                .scll()
                .bits(scll)
        });

        // Enable the I2C processing
        self.i2c.cr1.modify(|_, w| w.pe().set_bit());
    }

    fn check_and_clear_error_flags(&self, isr: &fmpi2c1::isr::R) -> Result<(), Error> {
        // If we received a NACK, then this is an error
        if isr.nackf().bit_is_set() {
            self.i2c
                .icr
                .write(|w| w.stopcf().set_bit().nackcf().set_bit());
            return Err(Error::NACK);
        }

        Ok(())
    }

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        while {
            let isr = self.i2c.isr.read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_addr)?;
            isr.txis().bit_is_clear()
        } {}

        // Push out a byte of data
        self.i2c.txdr.write(|w| unsafe { w.bits(u32::from(byte)) });

        self.check_and_clear_error_flags(&self.i2c.isr.read())
            .map_err(Error::nack_data)?;
        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while {
            let isr = self.i2c.isr.read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_data)?;
            isr.rxne().bit_is_clear()
        } {}

        let value = self.i2c.rxdr.read().bits() as u8;
        Ok(value)
    }

    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
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

    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
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

    pub fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
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
