//!
//! # I2S example for STM32F411
//!
//! This application demonstrates I2S communication with the DAC on an STM32F411E-DISCO board
//!
//! # Hardware required
//!
//! * STM32F411E-DISCO evaluation board
//! * Headphones or speakers with a headphone plug
//!
//! # Procedure
//!
//! 1. Connect the headphones or speakers to the headphone jack on the evaluation board
//!    (warning: the DAC may produce a powerful signal that becomes a very loud sound.
//!    Set the speaker volume to minimum, or do not put on the headphones.)
//! 2. Load this compiled application on the microcontroller and run it
//!
//! Expected behavior: the speakers/headphones emit 1 second of 375 Hz tone followed by 1 second of
//! 750 Hz tone, repeating indefinitely.
//!
//! # Pins and addresses
//!
//! * PD4 -> DAC ~RESET (pulled low)
//!
//! * PB9 -> SDA (pulled high)
//! * PB6 -> SCL (pulled high)
//!
//! * PC7 -> MCLK
//! * PC10 -> SCK (bit clock)
//! * PC12 -> SD
//! * PA4 -> WS
//!
//! DAC I2C address 0x94
//!

#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::hal::blocking::i2c::{Read, Write};
use stm32f4xx_hal::nb::block;
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;

use cortex_m_rt::entry;
use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity};
use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;

/// A sine wave spanning 64 samples
///
/// With a sample rate of 48 kHz, this produces a 750 Hz tone.
const SINE_750: [i16; 64] = [
    0, 3211, 6392, 9511, 12539, 15446, 18204, 20787, 23169, 25329, 27244, 28897, 30272, 31356,
    32137, 32609, 32767, 32609, 32137, 31356, 30272, 28897, 27244, 25329, 23169, 20787, 18204,
    15446, 12539, 9511, 6392, 3211, 0, -3211, -6392, -9511, -12539, -15446, -18204, -20787, -23169,
    -25329, -27244, -28897, -30272, -31356, -32137, -32609, -32767, -32609, -32137, -31356, -30272,
    -28897, -27244, -25329, -23169, -20787, -18204, -15446, -12539, -9511, -6392, -3211,
];

/// A sine wave spanning 128 samples
///
/// With a sample rate of 48 kHz, this produces a 375 Hz tone.
const SINE_375: [i16; 128] = [
    0, 1607, 3211, 4807, 6392, 7961, 9511, 11038, 12539, 14009, 15446, 16845, 18204, 19519, 20787,
    22004, 23169, 24278, 25329, 26318, 27244, 28105, 28897, 29621, 30272, 30851, 31356, 31785,
    32137, 32412, 32609, 32727, 32767, 32727, 32609, 32412, 32137, 31785, 31356, 30851, 30272,
    29621, 28897, 28105, 27244, 26318, 25329, 24278, 23169, 22004, 20787, 19519, 18204, 16845,
    15446, 14009, 12539, 11038, 9511, 7961, 6392, 4807, 3211, 1607, 0, -1607, -3211, -4807, -6392,
    -7961, -9511, -11038, -12539, -14009, -15446, -16845, -18204, -19519, -20787, -22004, -23169,
    -24278, -25329, -26318, -27244, -28105, -28897, -29621, -30272, -30851, -31356, -31785, -32137,
    -32412, -32609, -32727, -32767, -32727, -32609, -32412, -32137, -31785, -31356, -30851, -30272,
    -29621, -28897, -28105, -27244, -26318, -25329, -24278, -23169, -22004, -20787, -19519, -18204,
    -16845, -15446, -14009, -12539, -11038, -9511, -7961, -6392, -4807, -3211, -1607,
];

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let rcc = dp.RCC.constrain();
    // The 86 MHz frequency can be divided to get a sample rate very close to 48 kHz.
    let clocks = rcc.cfgr.use_hse(8.mhz()).i2s_clk(86.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (
            gpiob.pb6.into_alternate_af4_open_drain(),
            gpiob.pb9.into_alternate_af4_open_drain(),
        ),
        100.khz(),
        clocks,
    );
    let mut dac = Cs43L22 {
        i2c,
        // Shift the address to deal with different ways of representing I2C addresses
        address: 0x94 >> 1,
    };

    let mut dac_reset = gpiod.pd4.into_push_pull_output();

    // I2S pins: (WS, CK, MCLK, SD) for I2S3
    let i2s_pins = (
        gpioa.pa4.into_alternate_af6(),
        gpioc.pc10.into_alternate_af6(),
        gpioc.pc7.into_alternate_af6(),
        gpioc.pc12.into_alternate_af6(),
    );
    let hal_i2s = I2s::i2s3(dp.SPI3, i2s_pins, clocks);
    let i2s_clock = hal_i2s.input_clock();

    // Audio timing configuration:
    // Sample rate 48 kHz
    // 16 bits per sample -> SCK rate 1.536 MHz
    // MCK frequency = 256 * sample rate -> MCK rate 12.228 MHz (also equal to 8 * SCK rate)
    let sample_rate = 48000;

    let i2s = stm32_i2s_v12x::I2s::new(hal_i2s);
    let mut i2s = i2s.configure_master_transmit(MasterConfig::with_sample_rate(
        i2s_clock.0,
        sample_rate,
        Data16Frame16,
        FrameFormat::PhilipsI2s,
        Polarity::IdleHigh,
        MasterClock::Enable,
    ));

    // Keep DAC reset low for at least one millisecond
    delay.delay_ms(1u8);
    // Release the DAC from reset
    dac_reset.set_high().unwrap();
    // Wait at least 550 ns before starting I2C communication
    delay.delay_us(1u8);

    dac.basic_setup().unwrap();
    // Clocking control from the table in section 4.6 of the datasheet:
    // Auto mode: disabled
    // Speed mode: 01 (single-speed)
    // 8 kHz, 16 kHz, or 32 kHz sample rate: no
    // 27 MHz video clock: no
    // Internal MCLK/LRCLCK ratio: 00
    // MCLK divide by 2: no
    dac.write(Register::ClockingCtl, 0b0_01_0_0_00_0).unwrap();
    // Interface control:
    // Slave mode
    // SCLK not inverted
    // DSP mode disabled
    // Interface format I2S
    // Word length 16 bits
    dac.write(Register::InterfaceCtl1, 0b0_0_0_0_01_11).unwrap();

    // Reduce the headphone volume to make the demo less annoying
    let headphone_volume = -30i8 as u8;
    dac.write(Register::HeadphoneAVol, headphone_volume)
        .unwrap();
    dac.write(Register::HeadphoneBVol, headphone_volume)
        .unwrap();

    // Power up DAC
    dac.write(Register::PowerCtl1, 0b1001_1110).unwrap();

    // Start sending samples
    i2s.enable();
    let sine_375_1sec = SINE_375.iter().cloned().cycle().take(sample_rate as usize);
    let sine_750_1sec = SINE_750.iter().cloned().cycle().take(sample_rate as usize);

    loop {
        // Play one second of each tone
        for sample in sine_375_1sec.clone() {
            // Transmit the same sample on the left and right channels
            block!(i2s.transmit(sample)).unwrap();
            block!(i2s.transmit(sample)).unwrap();
        }
        for sample in sine_750_1sec.clone() {
            // Transmit the same sample on the left and right channels
            block!(i2s.transmit(sample)).unwrap();
            block!(i2s.transmit(sample)).unwrap();
        }
    }
}

/// Interface to the I2C control port of a Cirrus Logic CS43L22 DAC
struct Cs43L22<I> {
    /// I2C interface
    i2c: I,
    /// Address of DAC
    address: u8,
}

impl<I> Cs43L22<I>
where
    I: Write + Read,
{
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
    fn write(&mut self, register: Register, value: u8) -> Result<(), <I as Write>::Error> {
        // Set auto-increment bit
        let map = (register as u8) | 0x80;
        self.i2c.write(self.address, &[map, value])
    }

    /// Reads the value of one register
    #[allow(dead_code)]
    fn read(
        &mut self,
        register: Register,
    ) -> Result<u8, CombinedI2cError<<I as Read>::Error, <I as Write>::Error>> {
        let mut values = [0u8];
        self.read_multiple(register, &mut values)?;
        Ok(values[0])
    }
    /// Reads the values of zero or more consecutive registers
    #[allow(dead_code)]
    fn read_multiple(
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
enum CombinedI2cError<R, W> {
    Read(R),
    Write(W),
}

/// CS43L22 registers
#[allow(dead_code)]
enum Register {
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
