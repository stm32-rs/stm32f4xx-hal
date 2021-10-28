//!
//! # I2S example for STM32F411
//!
//! This application demonstrates I2S communication with the DAC on an STM32F411E-DISCO board.
//! Unlike `i2s-audio-out`, this example uses DMA instead of writing one sample at a time.
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
//! Expected behavior: the speakers/headphones emit a continuous 750 Hz tone
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

use core::cell::RefCell;

use panic_halt as _;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity, TransmitMode};

use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::dma::config::DmaConfig;
use stm32f4xx_hal::dma::MemoryToPeripheral;
use stm32f4xx_hal::dma::{Stream5, StreamsTuple, Transfer};
use stm32f4xx_hal::gpio::{
    gpioa::PA4,
    gpioc::{PC10, PC12, PC7},
    Alternate, PushPull,
};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::pac::{interrupt, Interrupt};
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::pac::{DMA1, SPI3};
use stm32f4xx_hal::prelude::*;

use cs43l22::{Cs43L22, Register};

/// Volume in decibels
///
/// Depending on your speakers, you may need to adjust this value.
const VOLUME: i8 = -100;

const SINE_SAMPLES: usize = 64;

/// A sine wave spanning 64 samples
///
/// With a sample rate of 48 kHz, this produces a 750 Hz tone.
const SINE_750: [i16; SINE_SAMPLES] = [
    0, 3211, 6392, 9511, 12539, 15446, 18204, 20787, 23169, 25329, 27244, 28897, 30272, 31356,
    32137, 32609, 32767, 32609, 32137, 31356, 30272, 28897, 27244, 25329, 23169, 20787, 18204,
    15446, 12539, 9511, 6392, 3211, 0, -3211, -6392, -9511, -12539, -15446, -18204, -20787, -23169,
    -25329, -27244, -28897, -30272, -31356, -32137, -32609, -32767, -32609, -32137, -31356, -30272,
    -28897, -27244, -25329, -23169, -20787, -18204, -15446, -12539, -9511, -6392, -3211,
];

#[entry]
fn main() -> ! {
    // Make a copy of SINE_750 with three differences:
    // 1. It's a way to get a &'static mut
    // 2. Its type is u16 instead of i16
    // 3. Each sample is repeated (for the left and right channels)
    static mut SINE_750_MUT: [u16; SINE_SAMPLES * 2] = [0; SINE_SAMPLES * 2];

    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    // The 86 MHz frequency can be divided to get a sample rate very close to 48 kHz.
    let clocks = rcc.cfgr.use_hse(8.mhz()).i2s_clk(86.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let mut delay = Delay::new(cp.SYST, &clocks);

    let i2c = I2c::new(
        dp.I2C1,
        (
            gpiob.pb6.into_alternate_open_drain(),
            gpiob.pb9.into_alternate_open_drain(),
        ),
        100.khz(),
        clocks,
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
    let hal_i2s = I2s::new(dp.SPI3, i2s_pins, clocks);
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
    i2s.set_dma_enabled(true);

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

    {
        // Copy samples from flash into the buffer that DMA will use
        let mut dest_iter = SINE_750_MUT.iter_mut();
        for sample in SINE_750.iter() {
            // Duplicate sample for the left and right channels
            let left = dest_iter.next().unwrap();
            let right = dest_iter.next().unwrap();
            *left = *sample as u16;
            *right = *sample as u16;
        }
    }

    // Set up DMA: DMA 1 stream 5 channel 0 memory -> peripheral
    let dma1_streams = StreamsTuple::new(dp.DMA1);
    let dma_config = DmaConfig::default()
        .memory_increment(true)
        .transfer_complete_interrupt(true);
    let mut dma_transfer: I2sDmaTransfer =
        Transfer::init_memory_to_peripheral(dma1_streams.5, i2s, SINE_750_MUT, None, dma_config);

    dma_transfer.start(|i2s| i2s.enable());
    // Hand off transfer to interrupt handler
    cortex_m::interrupt::free(|cs| *G_TRANSFER.borrow(cs).borrow_mut() = Some(dma_transfer));
    // Enable interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::DMA1_STREAM5);
    }

    loop {
        // Don't WFI. That can cause problems attaching the debugger.
    }
}

type I2sDmaTransfer = Transfer<
    Stream5<DMA1>,
    stm32_i2s_v12x::I2s<
        I2s<
            SPI3,
            (
                PA4<Alternate<PushPull, 6>>,
                PC10<Alternate<PushPull, 6>>,
                PC7<Alternate<PushPull, 6>>,
                PC12<Alternate<PushPull, 6>>,
            ),
        >,
        TransmitMode<Data16Frame16>,
    >,
    MemoryToPeripheral,
    &'static mut [u16; SINE_SAMPLES * 2],
    0,
>;

/// DMA transfer handoff from main() to interrupt handler
static G_TRANSFER: Mutex<RefCell<Option<I2sDmaTransfer>>> = Mutex::new(RefCell::new(None));

/// This interrupt handler runs when DMA 1 finishes a transfer to the I2S peripheral
#[interrupt]
fn DMA1_STREAM5() {
    static mut TRANSFER: Option<I2sDmaTransfer> = None;

    let transfer = TRANSFER.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TRANSFER.borrow(cs).replace(None).unwrap())
    });

    transfer.clear_transfer_complete_interrupt();
    unsafe {
        transfer
            .next_transfer_with(|buffer, _active_buffer| {
                // Transfer again with the same buffer
                (buffer, ())
            })
            .unwrap();
    }
}
