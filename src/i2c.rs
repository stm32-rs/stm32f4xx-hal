use core::ops::Deref;

use crate::pac::{self, i2c1};
use crate::rcc::{Enable, Reset};

use crate::gpio::{Const, OpenDrain, PinA, SetAlternate};
use crate::pac::RCC;

use crate::rcc::Clocks;
use fugit::{HertzU32 as Hertz, RateExtU32};

mod hal_02;
mod hal_1;

mod dma;
// For DMA mapping
pub(crate) use dma::{Rx, Tx};

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
pub struct I2c<I2C: Instance, PINS> {
    i2c: I2C,
    pins: PINS,
}

pub struct Scl;
impl crate::Sealed for Scl {}
pub struct Sda;
impl crate::Sealed for Sda {}

pub trait Pins<I2C> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}

impl<I2C, SCL, SDA, const SCLA: u8, const SDAA: u8> Pins<I2C> for (SCL, SDA)
where
    SCL: PinA<Scl, I2C, A = Const<SCLA>> + SetAlternate<SCLA, OpenDrain>,
    SDA: PinA<Sda, I2C, A = Const<SDAA>> + SetAlternate<SDAA, OpenDrain>,
{
    fn set_alt_mode(&mut self) {
        self.0.set_alt_mode();
        self.1.set_alt_mode();
    }
    fn restore_mode(&mut self) {
        self.0.restore_mode();
        self.1.restore_mode();
    }
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

pub trait Instance: crate::Sealed + Deref<Target = i2c1::RegisterBlock> + Enable + Reset {
    #[doc(hidden)]
    fn ptr() -> *const i2c1::RegisterBlock;
}

// Implemented by all I2C instances
macro_rules! i2c {
    ($I2C:ty: $I2c:ident) => {
        pub type $I2c<PINS> = I2c<$I2C, PINS>;

        impl Instance for $I2C {
            fn ptr() -> *const i2c1::RegisterBlock {
                <$I2C>::ptr() as *const _
            }
        }
    };
}

i2c! { pac::I2C1: I2c1 }
i2c! { pac::I2C2: I2c2 }

#[cfg(feature = "i2c3")]
i2c! { pac::I2C3: I2c3 }

pub trait I2cExt: Sized + Instance {
    fn i2c<SCL, SDA>(
        self,
        pins: (SCL, SDA),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> I2c<Self, (SCL, SDA)>
    where
        (SCL, SDA): Pins<Self>;
}

impl<I2C: Instance> I2cExt for I2C {
    fn i2c<SCL, SDA>(
        self,
        pins: (SCL, SDA),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> I2c<Self, (SCL, SDA)>
    where
        (SCL, SDA): Pins<Self>,
    {
        I2c::new(self, pins, mode, clocks)
    }
}

impl<I2C, SCL, SDA> I2c<I2C, (SCL, SDA)>
where
    I2C: Instance,
    (SCL, SDA): Pins<I2C>,
{
    pub fn new(i2c: I2C, mut pins: (SCL, SDA), mode: impl Into<Mode>, clocks: &Clocks) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable and reset clock.
            I2C::enable(rcc);
            I2C::reset(rcc);
        }

        pins.set_alt_mode();

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(mode, clocks.pclk1());
        i2c
    }

    pub fn release(mut self) -> (I2C, (SCL, SDA)) {
        self.pins.restore_mode();

        (self.i2c, (self.pins.0, self.pins.1))
    }
}

impl<I2C: Instance, PINS> I2c<I2C, PINS> {
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

    fn write_bytes(&mut self, addr: u8, bytes: impl Iterator<Item = u8>) -> Result<(), Error> {
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

    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
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
                self.check_and_clear_error_flags()
                    .map_err(Error::nack_addr)?;
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
            Err(Error::Overrun)
        }
    }

    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.write_bytes(addr, bytes.iter().cloned())?;

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
        self.write_bytes(addr, bytes.into_iter())?;

        // Send a STOP condition
        self.i2c.cr1.modify(|_, w| w.stop().set_bit());

        // Wait for STOP condition to transmit.
        while self.i2c.cr1.read().stop().bit_is_set() {}

        // Fallthrough is success
        Ok(())
    }

    pub fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.write_bytes(addr, bytes.iter().cloned())?;
        self.read(addr, buffer)
    }

    pub fn write_iter_read<B>(&mut self, addr: u8, bytes: B, buffer: &mut [u8]) -> Result<(), Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.write_bytes(addr, bytes.into_iter())?;
        self.read(addr, buffer)
    }
}
