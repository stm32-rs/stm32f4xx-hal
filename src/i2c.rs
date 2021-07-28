use core::ops::Deref;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

use crate::pac::i2c1;
use crate::rcc::{Enable, Reset};

#[cfg(feature = "i2c3")]
use crate::pac::I2C3;
use crate::pac::{I2C1, I2C2, RCC};

#[allow(unused)]
#[cfg(feature = "gpiof")]
use crate::gpio::gpiof;
#[allow(unused)]
use crate::gpio::{gpioa, gpiob, gpioc, gpioh};

use crate::gpio::AlternateOD;

use crate::rcc::Clocks;
use crate::time::{Hertz, U32Ext};

#[derive(Debug, Eq, PartialEq)]
pub enum DutyCycle {
    Ratio2to1,
    Ratio16to9,
}

#[derive(Debug, PartialEq)]
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
    pub fn standard<F: Into<Hertz>>(frequency: F) -> Self {
        Self::Standard {
            frequency: frequency.into(),
        }
    }

    pub fn fast<F: Into<Hertz>>(frequency: F, duty_cycle: DutyCycle) -> Self {
        Self::Fast {
            frequency: frequency.into(),
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

impl<F> From<F> for Mode
where
    F: Into<Hertz>,
{
    fn from(frequency: F) -> Self {
        let frequency: Hertz = frequency.into();
        if frequency <= 100_000.hz() {
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

pub trait Pins<I2c> {}
pub trait PinScl<I2c> {}
pub trait PinSda<I2c> {}

impl<I2c, SCL, SDA> Pins<I2c> for (SCL, SDA)
where
    SCL: PinScl<I2c>,
    SDA: PinSda<I2c>,
{
}

impl PinScl<I2C1> for gpiob::PB6<AlternateOD<4>> {}
impl PinSda<I2C1> for gpiob::PB7<AlternateOD<4>> {}
impl PinScl<I2C1> for gpiob::PB8<AlternateOD<4>> {}
impl PinSda<I2C1> for gpiob::PB9<AlternateOD<4>> {}

#[cfg(any(feature = "stm32f446"))]
impl PinSda<I2C2> for gpiob::PB3<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C2> for gpiob::PB3<AlternateOD<9>> {}
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C2> for gpiob::PB9<AlternateOD<9>> {}
impl PinScl<I2C2> for gpiob::PB10<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C2> for gpiob::PB11<AlternateOD<4>> {}
#[cfg(any(feature = "stm32f446"))]
impl PinSda<I2C2> for gpioc::PC12<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C2> for gpiof::PF1<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C2> for gpiof::PF0<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C2> for gpioh::PH4<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C2> for gpioh::PH5<AlternateOD<4>> {}

#[cfg(feature = "i2c3")]
impl PinScl<I2C3> for gpioa::PA8<AlternateOD<4>> {}
#[cfg(any(feature = "stm32f446"))]
impl PinSda<I2C3> for gpiob::PB4<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C3> for gpiob::PB4<AlternateOD<9>> {}
#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C3> for gpiob::PB8<AlternateOD<9>> {}

#[cfg(feature = "i2c3")]
impl PinSda<I2C3> for gpioc::PC9<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C3> for gpioh::PH7<AlternateOD<4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C3> for gpioh::PH8<AlternateOD<4>> {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    OVERRUN,
    NACK,
    TIMEOUT,
    // Note: The BUS error type is not currently returned, but is maintained for backwards
    // compatibility.
    BUS,
    CRC,
    ARBITRATION,
}

pub trait Instance: crate::Sealed + Deref<Target = i2c1::RegisterBlock> + Enable + Reset {}

impl Instance for I2C1 {}
impl Instance for I2C2 {}

#[cfg(feature = "i2c3")]
impl Instance for I2C3 {}

impl<I2C, SCL, SDA> I2c<I2C, (SCL, SDA)>
where
    I2C: Instance,
    SCL: PinScl<I2C>,
    SDA: PinSda<I2C>,
{
    pub fn new<M: Into<Mode>>(i2c: I2C, pins: (SCL, SDA), mode: M, clocks: Clocks) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable and reset clock.
            I2C::enable(rcc);
            I2C::reset(rcc);
        }

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(mode, clocks.pclk1());
        i2c
    }
}

impl<I2C, PINS> I2c<I2C, PINS>
where
    I2C: Instance,
{
    fn i2c_init<M: Into<Mode>>(&self, mode: M, pclk: Hertz) {
        let mode = mode.into();
        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Calculate settings for I2C speed modes
        let clock = pclk.0;
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
                let ccr = (clock / (frequency.0 * 2)).max(4);

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
                    let ccr = (clock / (frequency.0 * 3)).max(1);

                    // Set clock to fast mode with appropriate parameters for selected speed (2:1 duty cycle)
                    self.i2c.ccr.write(|w| unsafe {
                        w.f_s().set_bit().duty().clear_bit().ccr().bits(ccr as u16)
                    });
                }
                DutyCycle::Ratio16to9 => {
                    let ccr = (clock / (frequency.0 * 25)).max(1);

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
            return Err(Error::TIMEOUT);
        }

        if sr1.pecerr().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.pecerr().clear_bit());
            return Err(Error::CRC);
        }

        if sr1.ovr().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.ovr().clear_bit());
            return Err(Error::OVERRUN);
        }

        if sr1.af().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.af().clear_bit());
            return Err(Error::NACK);
        }

        if sr1.arlo().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.arlo().clear_bit());
            return Err(Error::ARBITRATION);
        }

        // The errata indicates that BERR may be incorrectly detected. It recommends ignoring and
        // clearing the BERR bit instead.
        if sr1.berr().bit_is_set() {
            self.i2c.sr1.modify(|_, w| w.berr().clear_bit());
        }

        Ok(sr1)
    }

    /// Perform an I2C software reset
    fn reset(&mut self) {
        self.i2c.cr1.write(|w| w.pe().set_bit().swrst().set_bit());
        self.i2c.cr1.reset();
        self.init();
    }

    /// Generate START condition
    fn send_start(&mut self) {
        self.i2c.cr1.modify(|_, w| w.start().set_bit());
    }

    /// Sends the (7-Bit) address on the I2C bus. The 8th bit on the bus is set
    /// depending on wether it is a read or write transfer.
    fn send_addr(&self, addr: u8, read: bool) {
        self.i2c
            .dr
            .write(|w| w.dr().bits(addr << 1 | (if read { 1 } else { 0 })));
    }

    /// Generate STOP condition
    fn send_stop(&self) {
        self.i2c.cr1.modify(|_, w| w.stop().set_bit());
    }

    /// Check if STOP condition is generated
    fn wait_for_stop(&mut self) {
        while self.i2c.cr1.read().stop().is_stop() {}
    }

    fn send_start_and_wait(&mut self) -> Result<(), Error> {
        // Send a START condition
        self.send_start();

        // Wait until START condition was generated
        while self.check_and_clear_error_flags()?.sb().bit_is_clear() {}

        Ok(())
    }

    fn send_addr_and_wait(&mut self, addr: u8, read: bool) -> Result<(), Error> {
        // Set up current address, we're trying to talk to
        self.send_addr(addr, read);
        loop {
            match self.check_and_clear_error_flags() {
                Ok(sr1) => {
                    // Wait for the address to be acknowledged
                    if sr1.addr().bit_is_set() {
                        break Ok(());
                    }
                }
                Err(Error::NACK) => {
                    self.send_stop();
                    break Err(Error::NACK);
                }
                Err(e) => break Err(e),
            }
        }
    }

    /// Clears the I2C ADDR pending flag
    fn clear_addr_flag(&self) {
        self.i2c.sr1.read();
        self.i2c.sr2.read();
    }

    fn write_bytes_and_wait(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.clear_addr_flag();

        self.i2c.dr.write(|w| w.dr().bits(bytes[0]));

        for byte in &bytes[1..] {
            while self.check_and_clear_error_flags()?.tx_e().bit_is_clear() {}
            self.i2c.dr.write(|w| w.dr().bits(*byte));
        }
        while self.check_and_clear_error_flags()?.btf().bit_is_clear() {}

        Ok(())
    }

    fn write_without_stop(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.send_start_and_wait()?;

        // Also wait until signalled we're master and everything is waiting for us
        while {
            self.check_and_clear_error_flags()?;

            let sr2 = self.i2c.sr2.read();
            sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
        } {}

        self.send_addr_and_wait(addr, false)?;

        let ret = self.write_bytes_and_wait(bytes);
        if ret == Err(Error::NACK) {
            self.send_stop();
        }
        ret
    }

    pub fn release(self) -> (I2C, PINS) {
        (self.i2c, self.pins)
    }
}

impl<I2C, PINS> Write for I2c<I2C, PINS>
where
    I2C: Instance,
{
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write_without_stop(addr, bytes)?;
        self.send_stop();
        self.wait_for_stop();

        Ok(())
    }
}

impl<I2C, PINS> Read for I2c<I2C, PINS>
where
    I2C: Instance,
{
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
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

        self.send_addr_and_wait(addr, true)?;

        match buffer.len() {
            0 => {
                self.clear_addr_flag();
                self.send_stop();
            }
            1 => {
                self.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                self.clear_addr_flag();
                self.send_stop();

                while self.check_and_clear_error_flags()?.rx_ne().bit_is_clear() {}
                buffer[0] = self.i2c.dr.read().dr().bits();
            }
            2 => {
                self.i2c
                    .cr1
                    .modify(|_, w| w.ack().clear_bit().pos().set_bit());
                self.clear_addr_flag();

                while self.check_and_clear_error_flags()?.btf().bit_is_clear() {}
                self.send_stop();
                buffer[0] = self.i2c.dr.read().dr().bits();
                buffer[1] = self.i2c.dr.read().dr().bits();
            }
            buffer_len => {
                self.i2c.cr1.modify(|_, w| w.ack().set_bit());
                self.clear_addr_flag();

                let (first_bytes, last_3_bytes) = buffer.split_at_mut(buffer_len - 3);

                for byte in first_bytes {
                    while self.check_and_clear_error_flags()?.rx_ne().bit_is_clear() {}
                    *byte = self.i2c.dr.read().dr().bits();
                    // TODO: check btf flag
                }

                while self.check_and_clear_error_flags()?.btf().bit_is_clear() {}
                self.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                last_3_bytes[0] = self.i2c.dr.read().dr().bits();
                while self.check_and_clear_error_flags()?.btf().bit_is_clear() {}
                self.send_stop();
                last_3_bytes[1] = self.i2c.dr.read().dr().bits();
                last_3_bytes[2] = self.i2c.dr.read().dr().bits();
            }
        }

        self.wait_for_stop();

        // Fallthrough is success
        Ok(())
    }
}

impl<I2C, PINS> WriteRead for I2c<I2C, PINS>
where
    I2C: Instance,
{
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        if !bytes.is_empty() {
            self.write_without_stop(addr, bytes)?;
        }

        if !buffer.is_empty() {
            self.read(addr, buffer)?;
        } else if !bytes.is_empty() {
            self.send_stop();
            self.wait_for_stop();
        }

        Ok(())
    }
}
