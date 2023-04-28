//! # I2S transfer example
//!
//! This application show I2s transfer usage for audio output. WARNING! This example generates very
//! loud sine tones (full scale), so turn down the volume before running this example.
//!
//! # Hardware required
//!
//! * a STM32F411 based board
//! * I2S DAC, eg PCM5102 from TI
//!
//! # Hardware Wiring
//!
//! The wiring assume using a PCM5102 module that can be found on Aliexpress, ebay, Amazon...
//!
//! ## Stm32
//!
//! | stm32 | PCM5102 |
//! |-------|---------|
//! | pa4   | LCK     |
//! | pc10  | BCK     |
//! | pc12  | DIN     |
//!
//!
//! ## PCM5102 module
//!
//! | Pin   | Connected to    |
//! |-------|-----------------|
//! | BCK   | pc10            |
//! | DIN   | pc12            |
//! | LCK   | pa4             |
//! | GND   | Gnd             |
//! | VIN   | +3V3            |
//! | FLT   | Gnd or +3V3     |
//! | DEMP  | Gnd             |
//! | XSMT  | +3V3            |
//! | A3V3  |                 |
//! | AGND  | audio out gnd   |
//! | ROUT  | audio out left  |
//! | LROUT | audio out right |
//!
//! Notes: on the module (not the chip) A3V3 is connected to VIN and AGND is connected to GND
//!
//!
//! Expected behavior: two different sine tone.

#![no_std]
#![no_main]

use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::gpio::NoPin;
use stm32f4xx_hal::i2s::stm32_i2s_v12x::transfer::*;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::nb::block;
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::*;

const SAMPLE_RATE: u32 = 48_000;

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
    rtt_init_print!();
    let dp = Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let rcc = dp.RCC.constrain();
    // The 61440 kHz frequency can be divided to get exactly 48 kHz sample rate even when
    // generating master clock
    let clocks = rcc
        .cfgr
        .use_hse(8u32.MHz())
        .sysclk(96.MHz())
        .i2s_clk(61440.kHz())
        .freeze();

    let i2s_pins = (gpioa.pa4, gpioc.pc10, NoPin, gpioc.pc12);
    let i2s = I2s::new(dp.SPI3, i2s_pins, &clocks);
    let i2s_config = I2sTransferConfig::new_master()
        .transmit()
        .standard(Philips)
        .data_format(Data32Channel32)
        .request_frequency(SAMPLE_RATE);
    let mut i2s_transfer = I2sTransfer::new(i2s, i2s_config);
    rprintln!("Actual sample rate is {}", i2s_transfer.sample_rate());

    let sine_375_1sec = SINE_375
        .iter()
        .map(|&x| {
            let x = (x as i32) << 16;
            (x, x)
        })
        .cycle()
        .take(SAMPLE_RATE as usize);
    let sine_750_1sec = SINE_750
        .iter()
        .map(|&x| {
            let x = (x as i32) << 16;
            (x, x)
        })
        .cycle()
        .take(SAMPLE_RATE as usize);

    loop {
        // Play 375.1 Hz using non blocking api
        for sample in sine_375_1sec.clone() {
            block!(i2s_transfer.write(sample)).ok();
        }
        // Play 750 Hz using blocking api
        i2s_transfer.write_iter(sine_750_1sec.clone());
    }
}

use core::panic::PanicInfo;
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
