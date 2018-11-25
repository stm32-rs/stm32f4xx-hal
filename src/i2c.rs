use core::ops::Deref;

use crate::stm32::i2c3;
use crate::stm32::rcc;
use crate::stm32::{I2C1, I2C2, RCC};

use hal::blocking::i2c::{Read, Write, WriteRead};

use crate::gpio::gpiob::{PB10, PB11, PB6, PB7, PB8, PB9};
use crate::gpio::{Alternate, AF4};
use crate::rcc::Clocks;
use crate::time::{Hertz, KiloHertz, U32Ext};

/// I2C abstraction
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

pub trait Pins<I2c> {}

impl Pins<I2C1> for (PB6<Alternate<AF4>>, PB7<Alternate<AF4>>) {}
impl Pins<I2C1> for (PB8<Alternate<AF4>>, PB9<Alternate<AF4>>) {}
impl Pins<I2C1> for (PB6<Alternate<AF4>>, PB9<Alternate<AF4>>) {}

impl Pins<I2C2> for (PB10<Alternate<AF4>>, PB11<Alternate<AF4>>) {}

#[derive(Debug)]
pub enum Error {
    OVERRUN,
    NACK,
}

type I2cRegisterBlock = i2c3::RegisterBlock;

trait I2cInit {
    fn i2c(&self) -> &I2cRegisterBlock;

    fn enable_peripheral(&self, rcc: &rcc::RegisterBlock);

    fn get_peripheral_clock(&self, clocks: Clocks) -> Hertz;

    fn i2c_init(&self, speed: KiloHertz, clocks: Clocks) {
        let speed: Hertz = speed.into();

        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };
        self.enable_peripheral(&rcc);

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c().cr1.modify(|_, w| w.pe().clear_bit());

        // Calculate settings for I2C speed modes
        let clock = self.get_peripheral_clock(clocks).0;
        let freq = clock / 1_000_000;
        assert!(freq >= 2 && freq <= 50);

        // Configure bus frequency into I2C peripheral
        self.i2c()
            .cr2
            .write(|w| unsafe { w.freq().bits(freq as u8) });

        let trise = if speed <= 100.khz().into() {
            freq + 1
        } else {
            (freq * 300) / 1000 + 1
        };

        // Configure correct rise times
        self.i2c().trise.write(|w| w.trise().bits(trise as u8));

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
            self.i2c().ccr.write(|w| unsafe {
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
                self.i2c().ccr.write(|w| unsafe {
                    w.f_s().set_bit().duty().clear_bit().ccr().bits(ccr as u16)
                });
            } else {
                let ccr = clock / (speed.0 * 25);
                let ccr = if ccr < 1 { 1 } else { ccr };

                // Set clock to fast mode with appropriate parameters for selected speed (16:9 duty cycle)
                self.i2c().ccr.write(|w| unsafe {
                    w.f_s().set_bit().duty().set_bit().ccr().bits(ccr as u16)
                });
            }
        }

        // Enable the I2C processing
        self.i2c().cr1.modify(|_, w| w.pe().set_bit());
    }
}

trait I2cCommon {
    fn i2c(&self) -> &I2cRegisterBlock;

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        while self.i2c().sr1.read().tx_e().bit_is_clear() {}

        // Push out a byte of data
        self.i2c().dr.write(|w| unsafe { w.bits(u32::from(byte)) });

        // Wait until byte is transferred
        while {
            let sr1 = self.i2c().sr1.read();

            // If we received a NACK, then this is an error
            if sr1.af().bit_is_set() {
                return Err(Error::NACK);
            }

            sr1.btf().bit_is_clear()
        } {}

        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while self.i2c().sr1.read().rx_ne().bit_is_clear() {}
        let value = self.i2c().dr.read().bits() as u8;
        Ok(value)
    }
}

impl<I2C, PINS> I2cCommon for I2c<I2C, PINS>
where
    I2C: Deref<Target = I2cRegisterBlock>,
{
    fn i2c(&self) -> &I2cRegisterBlock {
        &self.i2c
    }
}

impl<I2C, PINS> WriteRead for I2c<I2C, PINS>
where
    I2C: Deref<Target = I2cRegisterBlock>,
{
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write(addr, bytes)?;
        self.read(addr, buffer)?;

        Ok(())
    }
}

impl<I2C, PINS> Write for I2c<I2C, PINS>
where
    I2C: Deref<Target = I2cRegisterBlock>,
{
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        // Send a START condition
        self.i2c.cr1.modify(|_, w| w.start().set_bit());

        // Wait until START condition was generated
        while {
            let sr1 = self.i2c.sr1.read();
            sr1.sb().bit_is_clear()
        } {}

        // Also wait until signalled we're master and everything is waiting for us
        while {
            let sr2 = self.i2c.sr2.read();
            sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
        } {}

        // Set up current address, we're trying to talk to
        self.i2c
            .dr
            .write(|w| unsafe { w.bits(u32::from(addr) << 1) });

        // Wait until address was sent
        while {
            let sr1 = self.i2c.sr1.read();
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
}

impl<I2C, PINS> Read for I2c<I2C, PINS>
where
    I2C: Deref<Target = I2cRegisterBlock>,
{
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Send a START condition and set ACK bit
        self.i2c
            .cr1
            .modify(|_, w| w.start().set_bit().ack().set_bit());

        // Wait until START condition was generated
        while {
            let sr1 = self.i2c.sr1.read();
            sr1.sb().bit_is_clear()
        } {}

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
            let sr1 = self.i2c.sr1.read();
            sr1.addr().bit_is_clear()
        } {}

        // Clear condition by reading SR2
        self.i2c.sr2.read();

        // Receive bytes into buffer
        for c in buffer {
            *c = self.recv_byte()?;
        }

        // Send STOP condition
        self.i2c.cr1.modify(|_, w| w.stop().set_bit());

        // Fallthrough is success
        Ok(())
    }
}

impl<PINS> I2c<I2C1, PINS> {
    pub fn i2c1(i2c: I2C1, pins: PINS, speed: KiloHertz, clocks: Clocks) -> Self
    where
        PINS: Pins<I2C1>,
    {
        let i2c = I2c { i2c, pins };

        i2c.i2c_init(speed, clocks);
        i2c
    }

    pub fn release(self) -> (I2C1, PINS) {
        (self.i2c, self.pins)
    }
}

impl<PINS> I2cInit for I2c<I2C1, PINS> {
    fn i2c(&self) -> &I2cRegisterBlock {
        &self.i2c
    }

    fn enable_peripheral(&self, rcc: &rcc::RegisterBlock) {
        // Enable clock for I2C1
        rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit());

        // Reset I2C1
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());
    }

    fn get_peripheral_clock(&self, clocks: Clocks) -> Hertz {
        clocks.pclk1()
    }
}

impl<PINS> I2c<I2C2, PINS> {
    pub fn i2c2(i2c: I2C2, pins: PINS, speed: KiloHertz, clocks: Clocks) -> Self
    where
        PINS: Pins<I2C2>,
    {
        let i2c = I2c { i2c, pins };

        i2c.i2c_init(speed, clocks);
        i2c
    }

    pub fn release(self) -> (I2C2, PINS) {
        (self.i2c, self.pins)
    }
}

impl<PINS> I2cInit for I2c<I2C2, PINS> {
    fn i2c(&self) -> &I2cRegisterBlock {
        &self.i2c
    }

    fn enable_peripheral(&self, rcc: &rcc::RegisterBlock) {
        // Enable clock for I2C2
        rcc.apb1enr.modify(|_, w| w.i2c2en().set_bit());

        // Reset I2C2
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().clear_bit());
    }

    fn get_peripheral_clock(&self, clocks: Clocks) -> Hertz {
        clocks.pclk1()
    }
}
