//! Inter-Integrated Circuit (I2C) bus. Synchronized with the
//! [stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal) implementation,
//! as of 2021-02-25.

use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use crate::rcc::{Clocks, Enable, Reset};
use crate::time::Hertz;
use crate::{gpio, pac};
use core::ops::Deref;

/// I2C error
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,
    /// NACK
    Nack,
    // Overrun, // slave mode only
    // Pec, // SMBUS mode only
    // Timeout, // SMBUS mode only
    // Alert, // SMBUS mode only
}

pub trait Instance:
    crate::Sealed + Deref<Target = pac::i2c1::RegisterBlock> + Enable + Reset + gpio::alt::I2cCommon
{
}

// Implemented by all I2C instances
macro_rules! i2c {
    ($I2C:ty: $I2c:ident) => {
        pub type $I2c = I2c<$I2C>;

        impl Instance for $I2C {}
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

/// I2C peripheral operating in master mode
pub struct I2c<I2C: Instance> {
    i2c: I2C,
    pins: (I2C::Scl, I2C::Sda),
}

pub struct Config {
    presc: u8,
    sclh: u8,
    scll: u8,
    scldel: u8,
    sdadel: u8,
}

impl Config {
    pub fn new(freq: Hertz, clocks: Clocks) -> Self {
        let freq = freq.raw();
        assert!(freq <= 1_000_000);

        // TODO review compliance with the timing requirements of I2C
        // t_I2CCLK = 1 / PCLK1
        // t_PRESC  = (PRESC + 1) * t_I2CCLK
        // t_SCLL   = (SCLL + 1) * t_PRESC
        // t_SCLH   = (SCLH + 1) * t_PRESC
        //
        // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
        // t_SCL ~= t_SYNC1 + t_SYNC2 + t_SCLL + t_SCLH
        let i2cclk = clocks.pclk1().raw();
        let ratio = i2cclk / freq - 4;
        let (presc, scll, sclh, sdadel, scldel) = if freq >= 100_000 {
            // fast-mode or fast-mode plus
            // here we pick SCLL + 1 = 2 * (SCLH + 1)
            let presc = ratio / 387;

            let sclh = ((ratio / (presc + 1)) - 3) / 3;
            let scll = 2 * (sclh + 1) - 1;

            let (sdadel, scldel) = if freq > 400_000 {
                // fast-mode plus
                let sdadel = 0;
                let scldel = i2cclk / 4_000_000 / (presc + 1) - 1;

                (sdadel, scldel)
            } else {
                // fast-mode
                let sdadel = i2cclk / 8_000_000 / (presc + 1);
                let scldel = i2cclk / 2_000_000 / (presc + 1) - 1;

                (sdadel, scldel)
            };

            (presc, scll, sclh, sdadel, scldel)
        } else {
            // standard-mode
            // here we pick SCLL = SCLH
            let presc = ratio / 514;

            let sclh = ((ratio / (presc + 1)) - 2) / 2;
            let scll = sclh;

            let sdadel = i2cclk / 2_000_000 / (presc + 1);
            let scldel = i2cclk / 800_000 / (presc + 1) - 1;

            (presc, scll, sclh, sdadel, scldel)
        };

        macro_rules! u8_or_panic {
            ($value: expr, $message: literal) => {
                match u8::try_from($value) {
                    Ok(value) => value,
                    Err(_) => panic!($message),
                }
            };
        }

        let presc = u8_or_panic!(presc, "I2C pres");
        assert!(presc < 16);

        let scldel = u8_or_panic!(scldel, "I2C scldel");
        assert!(scldel < 16);

        let sdadel = u8_or_panic!(sdadel, "I2C sdadel");
        assert!(sdadel < 16);

        let sclh = u8_or_panic!(sclh, "I2C sclh");
        let scll = u8_or_panic!(scll, "I2C scll");

        Self {
            presc,
            sclh,
            scll,
            scldel,
            sdadel,
        }
    }

    /// For the layout of `timing_bits`, see RM0394 section 37.7.5.
    pub fn with_timing(timing_bits: u32) -> Self {
        Self {
            presc: ((timing_bits >> 28) & 0xf) as u8,
            scldel: ((timing_bits >> 20) & 0xf) as u8,
            sdadel: ((timing_bits >> 16) & 0xf) as u8,
            sclh: ((timing_bits >> 8) & 0xff) as u8,
            scll: (timing_bits & 0xff) as u8,
        }
    }
}

impl<I2C: Instance> I2c<I2C> {
    /// Configures the I2C peripheral to work in master mode
    pub fn new(
        i2c: I2C,
        pins: (impl Into<I2C::Scl>, impl Into<I2C::Sda>),
        config: Config,
        apb1: &mut I2C::Bus,
    ) -> Self {
        I2C::enable(apb1);
        I2C::reset(apb1);
        // Make sure the I2C unit is disabled so we can configure it
        i2c.cr1.modify(|_, w| w.pe().clear_bit());
        // Configure for "fast mode" (400 KHz)
        i2c.timingr.write(|w| {
            w.presc().bits(config.presc);
            w.scll().bits(config.scll);
            w.sclh().bits(config.sclh);
            w.sdadel().bits(config.sdadel);
            w.scldel().bits(config.scldel)
        });

        let pins = (pins.0.into(), pins.1.into());

        // Enable the peripheral
        i2c.cr1.write(|w| w.pe().set_bit());

        I2c { i2c, pins }
    }

    /// Releases the I2C peripheral and associated pins
    pub fn release(self) -> (I2C, (I2C::Scl, I2C::Sda)) {
        (self.i2c, self.pins)
    }
}

/// Sequence to flush the TXDR register. This resets the TXIS and TXE
// flags
macro_rules! flush_txdr {
    ($i2c:expr) => {
        // If a pending TXIS flag is set, write dummy data to TXDR
        if $i2c.isr.read().txis().bit_is_set() {
            $i2c.txdr.write(|w| w.txdata().bits(0));
        }

        // If TXDR is not flagged as empty, write 1 to flush it
        if $i2c.isr.read().txe().is_not_empty() {
            $i2c.isr.write(|w| w.txe().set_bit());
        }
    };
}

macro_rules! busy_wait {
    ($i2c:expr, $flag:ident, $variant:ident) => {
        loop {
            let isr = $i2c.isr.read();

            if isr.$flag().$variant() {
                break;
            } else if isr.berr().is_error() {
                $i2c.icr.write(|w| w.berrcf().set_bit());
                return Err(Error::Bus);
            } else if isr.arlo().is_lost() {
                $i2c.icr.write(|w| w.arlocf().set_bit());
                return Err(Error::Arbitration);
            } else if isr.nackf().bit_is_set() {
                $i2c.icr.write(|w| w.stopcf().set_bit().nackcf().set_bit());
                flush_txdr!($i2c);
                return Err(Error::Nack);
            } else {
                // try again
            }
        }
    };
}

impl<I2C: Instance> Write for I2c<I2C> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        // TODO support transfers of more than 255 bytes
        assert!(bytes.len() < 256);

        // Wait for any previous address sequence to end
        // automatically. This could be up to 50% of a bus
        // cycle (ie. up to 0.5/freq)
        while self.i2c.cr2.read().start().bit_is_set() {}

        // Set START and prepare to send `bytes`. The
        // START bit can be set even if the bus is BUSY or
        // I2C is in slave mode.
        self.i2c.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(u16::from(addr << 1 | 0));
            w.add10().clear_bit();
            w.rd_wrn().write();
            w.nbytes().bits(bytes.len() as u8);
            w.autoend().software()
        });

        for byte in bytes {
            // Wait until we are allowed to send data
            // (START has been ACKed or last byte when
            // through)
            busy_wait!(self.i2c, txis, is_empty);

            // Put byte on the wire
            self.i2c.txdr.write(|w| w.txdata().bits(*byte));
        }

        // Wait until the write finishes
        busy_wait!(self.i2c, tc, is_complete);

        // Stop
        self.i2c.cr2.write(|w| w.stop().set_bit());

        Ok(())
        // Tx::new(&self.i2c)?.write(addr, bytes)
    }
}

impl<I2C: Instance> Read for I2c<I2C> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // TODO support transfers of more than 255 bytes
        assert!(buffer.len() < 256 && buffer.len() > 0);

        // Wait for any previous address sequence to end
        // automatically. This could be up to 50% of a bus
        // cycle (ie. up to 0.5/freq)
        while self.i2c.cr2.read().start().bit_is_set() {}

        // Set START and prepare to receive bytes into
        // `buffer`. The START bit can be set even if the bus
        // is BUSY or I2C is in slave mode.
        self.i2c.cr2.write(|w| {
            w.sadd().bits((addr << 1 | 0) as u16);
            w.rd_wrn().read();
            w.nbytes().bits(buffer.len() as u8);
            w.start().set_bit();
            w.autoend().automatic()
        });

        for byte in buffer {
            // Wait until we have received something
            busy_wait!(self.i2c, rxne, is_not_empty);

            *byte = self.i2c.rxdr.read().rxdata().bits();
        }

        // automatic STOP

        Ok(())
        // Rx::new(&self.i2c)?.read(addr, buffer)
    }
}

impl<I2C: Instance> WriteRead for I2c<I2C> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        // TODO support transfers of more than 255 bytes
        assert!(bytes.len() < 256 && bytes.len() > 0);
        assert!(buffer.len() < 256 && buffer.len() > 0);

        // Wait for any previous address sequence to end
        // automatically. This could be up to 50% of a bus
        // cycle (ie. up to 0.5/freq)
        while self.i2c.cr2.read().start().bit_is_set() {}

        // Set START and prepare to send `bytes`. The
        // START bit can be set even if the bus is BUSY or
        // I2C is in slave mode.
        self.i2c.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(u16::from(addr << 1 | 0));
            w.add10().clear_bit();
            w.rd_wrn().write();
            w.nbytes().bits(bytes.len() as u8);
            w.autoend().software()
        });

        for byte in bytes {
            // Wait until we are allowed to send data
            // (START has been ACKed or last byte went through)
            busy_wait!(self.i2c, txis, is_empty);

            // Put byte on the wire
            self.i2c.txdr.write(|w| w.txdata().bits(*byte));
        }

        // Wait until the write finishes before beginning to read.
        busy_wait!(self.i2c, tc, is_complete);

        // reSTART and prepare to receive bytes into `buffer`
        self.i2c.cr2.write(|w| {
            w.sadd().bits(u16::from(addr << 1 | 1));
            w.add10().clear_bit();
            w.rd_wrn().read();
            w.nbytes().bits(buffer.len() as u8);
            w.start().set_bit();
            w.autoend().automatic()
        });

        for byte in buffer {
            // Wait until we have received something
            busy_wait!(self.i2c, rxne, is_not_empty);

            *byte = self.i2c.rxdr.read().rxdata().bits();
        }

        Ok(())
    }
}
