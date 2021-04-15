//! Bare-bones driver for configuring a CS43L22 digital-analog converter

use stm32f4xx_hal::hal::blocking::i2c::{Read, Write};

/// Interface to the I2C control port of a Cirrus Logic CS43L22 DAC
pub struct Cs43L22<I> {
    /// I2C interface
    i2c: I,
    /// Address of DAC
    address: u8,
}

impl<I> Cs43L22<I>
where
    I: Write + Read,
{
    pub fn new(i2c: I, address: u8) -> Self {
        Cs43L22 { i2c, address }
    }

    /// Does basic configuration as specified in the datasheet
    pub fn basic_setup(&mut self) -> Result<(), <I as Write>::Error> {
        // Settings from section 4.11 of the datasheet
        self.write(Register::Magic00, 0x99)?;
        self.write(Register::Magic47, 0x80)?;
        self.write(Register::Magic32, 0x80)?;
        self.write(Register::Magic32, 0x00)?;
        self.write(Register::Magic00, 0x00)
    }

    /// Writes the value of one register
    pub fn write(&mut self, register: Register, value: u8) -> Result<(), <I as Write>::Error> {
        // Set auto-increment bit
        let map = (register as u8) | 0x80;
        self.i2c.write(self.address, &[map, value])
    }

    /// Reads the value of one register
    #[allow(dead_code)]
    pub fn read(
        &mut self,
        register: Register,
    ) -> Result<u8, CombinedI2cError<<I as Read>::Error, <I as Write>::Error>> {
        let mut values = [0u8];
        self.read_multiple(register, &mut values)?;
        Ok(values[0])
    }
    /// Reads the values of zero or more consecutive registers
    #[allow(dead_code)]
    pub fn read_multiple(
        &mut self,
        register: Register,
        values: &mut [u8],
    ) -> Result<(), CombinedI2cError<<I as Read>::Error, <I as Write>::Error>> {
        // Two transactions: set the memory address pointer, then read
        // An empty write sets the address
        // Set auto-increment bit
        let map = (register as u8) | 0x80;
        self.i2c
            .write(self.address, &[map])
            .map_err(CombinedI2cError::Write)?;
        self.i2c
            .read(self.address, values)
            .map_err(CombinedI2cError::Read)
    }
}

#[derive(Debug)]
pub enum CombinedI2cError<R, W> {
    Read(R),
    Write(W),
}

/// CS43L22 registers
#[allow(dead_code)]
pub enum Register {
    /// This is used in the specified startup sequence, but its actual content is not documented.
    Magic00 = 0x00,
    Id = 0x01,
    PowerCtl1 = 0x02,
    PowerCtl2 = 0x04,
    ClockingCtl = 0x05,
    InterfaceCtl1 = 0x06,
    InterfaceCtl2 = 0x07,
    PassthroughASelect = 0x08,
    PassthroughBSelect = 0x09,
    AnalogZcSr = 0x0a,
    PassthroughGangCtl = 0x0c,
    PlaybackCtl1 = 0x0d,
    MiscCtl = 0x0e,
    PlaybackCtl2 = 0x0f,
    PassthroughAVol = 0x14,
    PassthroughBVol = 0x15,
    PcmAVol = 0x1a,
    PcmBVol = 0x1b,
    BeepFreqOnTime = 0x1c,
    BeepVolOffTime = 0x1d,
    BeepToneCfg = 0x1e,
    ToneCtl = 0x1f,
    MasterAVol = 0x20,
    MasterBVol = 0x21,
    HeadphoneAVol = 0x22,
    HeadphoneBVol = 0x23,
    SpeakerAVol = 0x24,
    SpeakerBVol = 0x25,
    ChannelMixer = 0x26,
    LimitCtl1 = 0x27,
    LimitClt2 = 0x28,
    LimitAttack = 0x29,
    Status = 0x2e,
    BatteryComp = 0x2f,
    VpBatteryLevel = 0x30,
    SpeakerStatus = 0x31,
    /// This is used in the specified startup sequence, but its actual content is not documented.
    Magic32 = 0x32,
    ChargePumpFreq = 0x34,
    /// This is used in the specified startup sequence, but its actual content is not documented.
    Magic47 = 0x47,
}
