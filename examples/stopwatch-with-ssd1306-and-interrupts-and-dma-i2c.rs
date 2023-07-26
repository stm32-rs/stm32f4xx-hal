//! A simple stopwatch app running on an SSD1306 display
//!
//! For example, to run on an STM32F411 Nucleo
//! dev board, run the following:
//!
//! ```bash
//! cargo run --features stm32f411 --release --example stopwatch-with-ssd1306-and-interrupts-and-dma-i2c
//! ```
//!
//! Note that `--release` is required to fix link errors for smaller devices.
//!
//! Press the User button on an STM32 Nucleo board to start/stop the timer. Pressing the Reset
//! button will reset the stopwatch to zero.
//!
//! Video of this example running: https://imgur.com/a/lQTQFLy

#![allow(clippy::empty_loop, clippy::new_without_default)]
#![no_std]
#![no_main]

use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use stm32f4xx_hal as hal;

use crate::hal::{
    dma::{Stream1, StreamsTuple},
    gpio::*,
    i2c::dma::{I2CMasterDma, NoDMA, TxDMA},
    i2c::I2c,
    interrupt, pac,
    pac::{DMA1, I2C1},
    prelude::*,
    rcc::{Clocks, Rcc},
    timer::{CounterUs, Event, Flag, Timer},
};
use core::cell::{Cell, RefCell};
use core::fmt::Write;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use cortex_m_rt::entry;
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X12, FONT_9X15},
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use heapless::String;
use ssd1306::{prelude::*, Ssd1306};

pub type I2c1Handle = I2CMasterDma<
    I2C1,                          // Instance of I2C
    TxDMA<I2C1, Stream1<DMA1>, 0>, // Stream and channel used for Tx. First parameter must be same Instance as first generic parameter of I2CMasterDma
    NoDMA,                         // This example don't need Rx
>;

// Set up global state. It's all mutexed up for concurrency safety.
static ELAPSED_MS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0u32));
static TIMER_TIM2: Mutex<RefCell<Option<CounterUs<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static STATE: Mutex<Cell<StopwatchState>> = Mutex::new(Cell::new(StopwatchState::Ready));
static BUTTON: Mutex<RefCell<Option<PC13<Input>>>> = Mutex::new(RefCell::new(None));
// Shared I2C handle. It cannot init statically so Option is required
static I2C1: Mutex<RefCell<Option<I2c1Handle>>> = Mutex::new(RefCell::new(None));

#[derive(Clone, Copy)]
enum StopwatchState {
    Ready,
    Running,
    Stopped,
}

// Custom connector to display
const DISPLAY_BUFFER_SIZE: usize = 128 * 64 / 8 + 1; // Display 128x64 + 1 byte for DataByte
const COMMAND_BUFFER_SIZE: usize = 8;
const I2C_ADDRESS: u8 = 0x3C;

// This atomics variables will protect us from rewriting buffer that current used in transfer
static COMMAND_SEND: AtomicBool = AtomicBool::new(false);
static DRAWING: AtomicBool = AtomicBool::new(false);

pub struct DMAI2cInterface {
    display_buffer: [u8; DISPLAY_BUFFER_SIZE], // Display 128x64 + 1 byte for DataByte
    command_buffer: [u8; COMMAND_BUFFER_SIZE],
}

impl DMAI2cInterface {
    pub fn new() -> Self {
        Self {
            display_buffer: [0x40; DISPLAY_BUFFER_SIZE],
            command_buffer: [0x0; COMMAND_BUFFER_SIZE],
        }
    }
}

impl WriteOnlyDataCommand for DMAI2cInterface {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        while COMMAND_SEND.load(Ordering::SeqCst) {
            core::hint::spin_loop()
        }

        match cmd {
            DataFormat::U8(slice) => {
                self.command_buffer[1..=slice.len()].copy_from_slice(&slice[0..slice.len()]);

                COMMAND_SEND.store(true, Ordering::SeqCst);
                nb::block!(free(|cs| {
                    let mut i2c = I2C1.borrow(cs).borrow_mut();

                    unsafe {
                        i2c.as_mut().unwrap().write_dma(
                            I2C_ADDRESS,
                            &self.command_buffer[..=slice.len()],
                            Some(|_| {
                                COMMAND_SEND.store(false, Ordering::SeqCst);
                            }),
                        )
                    }
                }))
                .ok(); // Ignore errors, Callback will handle it

                Ok(())
            }
            _ => Err(DisplayError::DataFormatNotImplemented),
        }
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        while DRAWING.load(Ordering::SeqCst) {
            core::hint::spin_loop()
        }

        match buf {
            DataFormat::U8(slice) => {
                self.display_buffer[1..=slice.len()].copy_from_slice(&slice[0..slice.len()]);

                DRAWING.store(true, Ordering::SeqCst);
                nb::block!(free(|cs| {
                    let mut i2c = I2C1.borrow(cs).borrow_mut();

                    unsafe {
                        i2c.as_mut().unwrap().write_dma(
                            I2C_ADDRESS,
                            &self.display_buffer[..=slice.len()],
                            Some(|_| DRAWING.store(false, Ordering::SeqCst)),
                        )
                    }
                }))
                .ok(); // Ignore errors, Callback will handle it

                Ok(())
            }
            _ => Err(DisplayError::DataFormatNotImplemented),
        }
    }
}

#[entry]
fn main() -> ! {
    if let (Some(mut dp), Some(cp)) = (pac::Peripherals::take(), cortex_m::Peripherals::take()) {
        let rcc = dp.RCC.constrain();
        let clocks = setup_clocks(rcc);
        let gpiob = dp.GPIOB.split();
        let i2c = I2c::new(dp.I2C1, (gpiob.pb8, gpiob.pb9), 400.kHz(), &clocks);

        // Then convert it to DMA
        let streams = StreamsTuple::new(dp.DMA1);
        let i2c_dma: I2c1Handle = i2c.use_dma_tx(streams.1);
        free(|cs| {
            I2C1.borrow(cs).replace(Some(i2c_dma));
        });

        // We need enable interrupt before init display
        unsafe {
            pac::NVIC::unmask(hal::interrupt::DMA1_STREAM1);
            pac::NVIC::unmask(hal::interrupt::I2C1_ER);
        };

        // On my board it is required to manually toggle Reset Pin of display
        let gpioa = dp.GPIOA.split();
        gpioa.pa8.into_push_pull_output().set_high();

        let mut syscfg = dp.SYSCFG.constrain();

        // Create a button input with an interrupt
        let gpioc = dp.GPIOC.split();
        let mut board_btn = gpioc.pc13.into_pull_up_input();
        board_btn.make_interrupt_source(&mut syscfg);
        board_btn.enable_interrupt(&mut dp.EXTI);
        board_btn.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

        // Safe: it is not changing after init
        let interface = DMAI2cInterface::new();
        let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        disp.init().unwrap();
        disp.flush().unwrap();

        // Create a 1ms periodic interrupt from TIM2
        let mut timer = dp.TIM2.counter(&clocks);
        timer.start(1.secs()).unwrap();
        timer.listen(Event::Update);

        let btn_int_num = board_btn.interrupt(); // hal::pac::Interrupt::EXTI15_10

        free(|cs| {
            TIMER_TIM2.borrow(cs).replace(Some(timer));
            BUTTON.borrow(cs).replace(Some(board_btn));
        });

        // Enable interrupts
        pac::NVIC::unpend(hal::pac::Interrupt::TIM2);
        pac::NVIC::unpend(btn_int_num);

        unsafe {
            pac::NVIC::unmask(btn_int_num);
        };

        let mut delay = Timer::syst(cp.SYST, &clocks).delay();

        loop {
            let elapsed = free(|cs| ELAPSED_MS.borrow(cs).get());

            let mut format_buf = String::<10>::new();
            format_elapsed(&mut format_buf, elapsed);

            disp.clear();

            let state = free(|cs| STATE.borrow(cs).get());
            let state_msg = match state {
                StopwatchState::Ready => "Ready",
                StopwatchState::Running => "",
                StopwatchState::Stopped => "Stopped",
            };

            let text_style = MonoTextStyleBuilder::new()
                .font(&FONT_6X12)
                .text_color(BinaryColor::On)
                .build();

            let text_style_format_buf = MonoTextStyleBuilder::new()
                .font(&FONT_9X15)
                .text_color(BinaryColor::On)
                .build();

            Text::new(state_msg, Point::new(0, 14), text_style)
                .draw(&mut disp)
                .unwrap();
            Text::new(&format_buf, Point::new(0, 28), text_style_format_buf)
                .draw(&mut disp)
                .unwrap();

            disp.flush().unwrap();

            delay.delay_ms(100u32);
        }
    }

    loop {}
}

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            tim2.clear_flags(Flag::Update);
        }

        let cell = ELAPSED_MS.borrow(cs);
        let val = cell.get();
        cell.replace(val + 1);
    });
}

#[interrupt]
fn EXTI15_10() {
    free(|cs| {
        let mut btn_ref = BUTTON.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.deref_mut() {
            // We cheat and don't bother checking _which_ exact interrupt line fired - there's only
            // ever going to be one in this example.
            btn.clear_interrupt_pending_bit();

            let state = STATE.borrow(cs).get();
            // Run the state machine in an ISR - probably not something you want to do in most
            // cases but this one only starts and stops TIM2 interrupts
            match state {
                StopwatchState::Ready => {
                    stopwatch_start(cs);
                    STATE.borrow(cs).replace(StopwatchState::Running);
                }
                StopwatchState::Running => {
                    stopwatch_stop(cs);
                    STATE.borrow(cs).replace(StopwatchState::Stopped);
                }
                StopwatchState::Stopped => {}
            }
        }
    });
}

// Interrupt for handling Tx operations
// We use only write command so we don't DMA1_STREAM0 handler for Rx
#[interrupt]
fn DMA1_STREAM1() {
    free(|cs| {
        let mut i2c = I2C1.borrow(cs).borrow_mut();
        i2c.as_mut().unwrap().handle_dma_interrupt();
    });
}

// Interrupt for errors in I2C
#[interrupt]
fn I2C1_ER() {
    free(|cs| {
        let mut i2c = I2C1.borrow(cs).borrow_mut();
        i2c.as_mut().unwrap().handle_error_interrupt();
    });
}

fn setup_clocks(rcc: Rcc) -> Clocks {
    rcc.cfgr
        .hclk(48.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze()
}

fn stopwatch_start(cs: &CriticalSection) {
    ELAPSED_MS.borrow(cs).replace(0);
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::TIM2);
    }
}

fn stopwatch_stop(_cs: &CriticalSection) {
    pac::NVIC::mask(hal::pac::Interrupt::TIM2);
}

// Formatting requires the heapless crate
fn format_elapsed(buf: &mut String<10>, elapsed: u32) {
    let minutes = elapsed_to_m(elapsed);
    let seconds = elapsed_to_s(elapsed);
    let millis = elapsed_to_ms(elapsed);
    write!(buf, "{minutes}:{seconds:02}.{millis:03}").unwrap();
}

fn elapsed_to_ms(elapsed: u32) -> u32 {
    elapsed % 1000
}

fn elapsed_to_s(elapsed: u32) -> u32 {
    (elapsed - elapsed_to_ms(elapsed)) % 60000 / 1000
}

fn elapsed_to_m(elapsed: u32) -> u32 {
    elapsed / 60000
}
