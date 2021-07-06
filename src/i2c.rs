use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use core::ops::Deref;

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
use crate::time::{Hertz, KiloHertz, U32Ext};

/// I2C abstraction
pub struct I2c<I2C: Instance, PINS> {
    i2c: I2C,
    pins: PINS,
}

#[cfg(feature = "fmpi2c1")]
/// I2C FastMode+ abstraction
pub struct FMPI2c<I2C, PINS> {
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

#[cfg(feature = "fmpi2c1")]
use crate::{gpio::gpiod, pac::fmpi2c1, pac::FMPI2C1};

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpioc::PC6<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinSda<FMPI2C1> for gpioc::PC7<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinSda<FMPI2C1> for gpiob::PB3<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiob::PB10<AlternateOD<9>> {}

#[cfg(feature = "fmpi2c1")]
impl PinSda<FMPI2C1> for gpiob::PB14<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiob::PB15<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiod::PD12<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiob::PB13<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiod::PD14<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiod::PD15<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiof::PF14<AlternateOD<4>> {}

#[cfg(feature = "fmpi2c1")]
impl PinScl<FMPI2C1> for gpiof::PF15<AlternateOD<4>> {}

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

mod private {
    pub trait Sealed {}
}

pub trait Instance: private::Sealed + Deref<Target = i2c1::RegisterBlock> + Enable + Reset {}

impl private::Sealed for I2C1 {}
impl Instance for I2C1 {}
impl private::Sealed for I2C2 {}
impl Instance for I2C2 {}

#[cfg(feature = "i2c3")]
impl private::Sealed for I2C3 {}
#[cfg(feature = "i2c3")]
impl Instance for I2C3 {}

#[cfg(feature = "fmpi2c1")]
impl<PINS> FMPI2c<FMPI2C1, PINS>
where
    PINS: Pins<FMPI2C1>,
{
    pub fn new(i2c: FMPI2C1, pins: PINS, speed: KiloHertz) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable and reset clock.
            FMPI2C1::enable(rcc);
            FMPI2C1::reset(rcc);

            rcc.dckcfgr2.modify(|_, w| w.fmpi2c1sel().hsi());
        }

        let i2c = FMPI2c { i2c, pins };
        i2c.i2c_init(speed);
        i2c
    }
}

impl<I2C, PINS> I2c<I2C, PINS>
where
    I2C: Instance,
    PINS: Pins<I2C>,
{
    pub fn new(i2c: I2C, pins: PINS, speed: KiloHertz, clocks: Clocks) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable and reset clock.
            I2C::enable(rcc);
            I2C::reset(rcc);
        }

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(speed, clocks.pclk1());
        i2c
    }
}

impl<I2C, PINS> I2c<I2C, PINS>
where
    I2C: Instance,
{
    fn i2c_init(&self, speed: KiloHertz, pclk: Hertz) {
        let speed: Hertz = speed.into();

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Calculate settings for I2C speed modes
        let clock = pclk.0;
        let freq = clock / 1_000_000;
        assert!((2..=50).contains(&freq));

        // Configure bus frequency into I2C peripheral
        self.i2c.cr2.write(|w| unsafe { w.freq().bits(freq as u8) });

        let trise = if speed <= 100.khz().into() {
            freq + 1
        } else {
            (freq * 300) / 1000 + 1
        };

        // Configure correct rise times
        self.i2c.trise.write(|w| w.trise().bits(trise as u8));

        // I2C clock control calculation
        if speed <= 100.khz().into() {
            let ccr = {
                let ccr = clock / (speed.0 * 2);
                if ccr < 4 {
                    4
                } else {
                    ccr
                }
            };

            // Set clock to standard mode with appropriate parameters for selected speed
            self.i2c.ccr.write(|w| unsafe {
                w.f_s()
                    .clear_bit()
                    .duty()
                    .clear_bit()
                    .ccr()
                    .bits(ccr as u16)
            });
        } else {
            const DUTYCYCLE: u8 = 0;
            if DUTYCYCLE == 0 {
                let ccr = clock / (speed.0 * 3);
                let ccr = if ccr < 1 { 1 } else { ccr };

                // Set clock to fast mode with appropriate parameters for selected speed (2:1 duty cycle)
                self.i2c.ccr.write(|w| unsafe {
                    w.f_s().set_bit().duty().clear_bit().ccr().bits(ccr as u16)
                });
            } else {
                let ccr = clock / (speed.0 * 25);
                let ccr = if ccr < 1 { 1 } else { ccr };

                // Set clock to fast mode with appropriate parameters for selected speed (16:9 duty cycle)
                self.i2c.ccr.write(|w| unsafe {
                    w.f_s().set_bit().duty().set_bit().ccr().bits(ccr as u16)
                });
            }
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

    pub fn release(self) -> (I2C, PINS) {
        (self.i2c, self.pins)
    }
}

trait I2cCommon {
    fn write_bytes(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error>;

    fn send_byte(&self, byte: u8) -> Result<(), Error>;

    fn recv_byte(&self) -> Result<u8, Error>;
}

impl<I2C, PINS> I2cCommon for I2c<I2C, PINS>
where
    I2C: Instance,
{
    fn write_bytes(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        // Send a START condition
        self.i2c.cr1.modify(|_, w| w.start().set_bit());

        // Wait until START condition was generated
        while self.check_and_clear_error_flags()?.sb().bit_is_clear() {}

        // Also wait until signalled we're master and everything is waiting for us
        while {
            self.check_and_clear_error_flags()?;

            let sr2 = self.i2c.sr2.read();
            sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
        } {}

        // Set up current address, we're trying to talk to
        self.i2c
            .dr
            .write(|w| unsafe { w.bits(u32::from(addr) << 1) });

        // Wait until address was sent
        while {
            // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
            let sr1 = self.check_and_clear_error_flags()?;

            // Wait for the address to be acknowledged
            sr1.addr().bit_is_clear()
        } {}

        // Clear condition by reading SR2
        self.i2c.sr2.read();

        // Send bytes
        for c in bytes {
            self.send_byte(*c)?;
        }

        // Fallthrough is success
        Ok(())
    }

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        while {
            // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
            self.check_and_clear_error_flags()?.tx_e().bit_is_clear()
        } {}

        // Push out a byte of data
        self.i2c.dr.write(|w| unsafe { w.bits(u32::from(byte)) });

        // Wait until byte is transferred
        while {
            // Check for any potential error conditions.
            self.check_and_clear_error_flags()?.btf().bit_is_clear()
        } {}

        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while {
            // Check for any potential error conditions.
            self.check_and_clear_error_flags()?;

            self.i2c.sr1.read().rx_ne().bit_is_clear()
        } {}

        let value = self.i2c.dr.read().bits() as u8;
        Ok(value)
    }
}

impl<I2C, PINS> WriteRead for I2c<I2C, PINS>
where
    I2C: Instance,
{
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write_bytes(addr, bytes)?;
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
        self.write_bytes(addr, bytes)?;

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
            while {
                self.check_and_clear_error_flags()?;
                self.i2c.sr1.read().addr().bit_is_clear()
            } {}

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

#[cfg(feature = "fmpi2c1")]
impl<I2C, PINS> FMPI2c<I2C, PINS>
where
    I2C: Deref<Target = fmpi2c1::RegisterBlock>,
{
    fn i2c_init(&self, speed: KiloHertz) {
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
        if speed <= 100.khz() {
            presc = 3;
            scll = cmp::max((((FREQ >> presc) >> 1) / speed.0) - 1, 255) as u8;
            sclh = scll - 4;
            sdadel = 2;
            scldel = 4;
        } else if speed <= 400.khz() {
            presc = 1;
            scll = cmp::max((((FREQ >> presc) >> 1) / speed.0) - 1, 255) as u8;
            sclh = scll - 6;
            sdadel = 2;
            scldel = 3;
        } else {
            presc = 0;
            scll = cmp::max((((FREQ >> presc) >> 1) / speed.0) - 4, 255) as u8;
            sclh = scll - 2;
            sdadel = 0;
            scldel = 2;
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

    pub fn release(self) -> (I2C, PINS) {
        (self.i2c, self.pins)
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
            self.check_and_clear_error_flags(&isr)?;
            isr.txis().bit_is_clear()
        } {}

        // Push out a byte of data
        self.i2c.txdr.write(|w| unsafe { w.bits(u32::from(byte)) });

        self.check_and_clear_error_flags(&self.i2c.isr.read())?;
        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while {
            let isr = self.i2c.isr.read();
            self.check_and_clear_error_flags(&isr)?;
            isr.rxne().bit_is_clear()
        } {}

        let value = self.i2c.rxdr.read().bits() as u8;
        Ok(value)
    }
}

#[cfg(feature = "fmpi2c1")]
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
            self.check_and_clear_error_flags(&isr)?;
            isr.txis().bit_is_clear() && isr.tc().bit_is_clear()
        } {}

        // Send out all individual bytes
        for c in bytes {
            self.send_byte(*c)?;
        }

        // Wait until data was sent
        while {
            let isr = self.i2c.isr.read();
            self.check_and_clear_error_flags(&isr)?;
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
        self.check_and_clear_error_flags(&self.i2c.isr.read())?;

        Ok(())
    }
}

#[cfg(feature = "fmpi2c1")]
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
        self.check_and_clear_error_flags(&self.i2c.isr.read())?;

        Ok(())
    }
}

#[cfg(feature = "fmpi2c1")]
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
        self.check_and_clear_error_flags(&self.i2c.isr.read())?;

        Ok(())
    }
}
