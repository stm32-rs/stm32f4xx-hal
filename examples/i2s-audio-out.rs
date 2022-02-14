//!
//! # I2S example for STM32F411
//!
//! This application demonstrates I2S communication with the DAC on an STM32F411E-DISCO board
//!
//! # Hardware required
//!
//! * STM32F407G-DISC1 or STM32F411E-DISCO evaluation board
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

mod cs43l22;

use panic_halt as _;

use cortex_m_rt::entry;

use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity};

use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::nb::block;
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;

use cs43l22::{Cs43L22, Register};

/// Volume in decibels
///
/// Depending on your speakers, you may need to adjust this value.
const VOLUME: i8 = -100;

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
    let clocks = rcc.cfgr.use_hse(8.MHz()).i2s_clk(86.MHz()).freeze();

    let mut delay = Delay::new(cp.SYST, &clocks);

    let i2c = I2c::new(
        dp.I2C1,
        (
            gpiob.pb6.into_alternate_open_drain(),
            gpiob.pb9.into_alternate_open_drain(),
        ),
        100.kHz(),
        &clocks,
    );
    // Shift the address to deal with different ways of representing I2C addresses
    let mut dac = Cs43L22::new(i2c, 0x94 >> 1);

    let mut dac_reset = gpiod.pd4.into_push_pull_output();

    // I2S pins: (WS, CK, MCLK, SD) for I2S3
    let i2s_pins = (
        gpioa.pa4.into_alternate(),
        gpioc.pc10.into_alternate(),
        gpioc.pc7.into_alternate(),
        gpioc.pc12.into_alternate(),
    );
    let hal_i2s = I2s::new(dp.SPI3, i2s_pins, &clocks);
    let i2s_clock = hal_i2s.input_clock();

    // Audio timing configuration:
    // Sample rate 48 kHz
    // 16 bits per sample -> SCK rate 1.536 MHz
    // MCK frequency = 256 * sample rate -> MCK rate 12.228 MHz (also equal to 8 * SCK rate)
    let sample_rate = 48000;

    let i2s = stm32_i2s_v12x::I2s::new(hal_i2s);
    let mut i2s = i2s.configure_master_transmit(MasterConfig::with_sample_rate(
        i2s_clock.raw(),
        sample_rate,
        Data16Frame16,
        FrameFormat::PhilipsI2s,
        Polarity::IdleHigh,
        MasterClock::Enable,
    ));

    // Keep DAC reset low for at least one millisecond
    delay.delay_ms(1u8);
    // Release the DAC from reset
    dac_reset.set_high();
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

    // Reduce the headphone volume something more comfortable
    dac.write(Register::HeadphoneAVol, VOLUME as u8).unwrap();
    dac.write(Register::HeadphoneBVol, VOLUME as u8).unwrap();

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
