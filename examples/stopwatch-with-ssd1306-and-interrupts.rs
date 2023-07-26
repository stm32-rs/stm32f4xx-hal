//! A simple stopwatch app running on an SSD1306 display
//!
//! For example, to run on an STM32F411 Nucleo
//! dev board, run the following:
//!
//! ```bash
//! cargo run --features stm32f411 --release --example stopwatch-with-ssd1306-and-interrupts
//! ```
//!
//! Note that `--release` is required to fix link errors for smaller devices.
//!
//! Press the User button on an STM32 Nucleo board to start/stop the timer. Pressing the Reset
//! button will reset the stopwatch to zero.
//!
//! Video of this example running: https://imgur.com/a/lQTQFLy

#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use stm32f4xx_hal as hal;

use crate::hal::{
    gpio::{Edge, Input, PC13},
    i2c::I2c,
    interrupt, pac,
    prelude::*,
    rcc::{Clocks, Rcc},
    timer::{CounterUs, Event, Flag, Timer},
};
use core::cell::{Cell, RefCell};
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use cortex_m_rt::entry;
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
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

// Set up global state. It's all mutexed up for concurrency safety.
static ELAPSED_MS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0u32));
static TIMER_TIM2: Mutex<RefCell<Option<CounterUs<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static STATE: Mutex<Cell<StopwatchState>> = Mutex::new(Cell::new(StopwatchState::Ready));
static BUTTON: Mutex<RefCell<Option<PC13<Input>>>> = Mutex::new(RefCell::new(None));

#[derive(Clone, Copy)]
enum StopwatchState {
    Ready,
    Running,
    Stopped,
}

#[entry]
fn main() -> ! {
    if let (Some(mut dp), Some(cp)) = (pac::Peripherals::take(), cortex_m::Peripherals::take()) {
        let rcc = dp.RCC.constrain();
        let clocks = setup_clocks(rcc);
        let gpiob = dp.GPIOB.split();
        let i2c = I2c::new(dp.I2C1, (gpiob.pb8, gpiob.pb9), 400.kHz(), &clocks);

        let mut syscfg = dp.SYSCFG.constrain();

        // Create a button input with an interrupt
        let gpioc = dp.GPIOC.split();
        let mut board_btn = gpioc.pc13.into_pull_up_input();
        board_btn.make_interrupt_source(&mut syscfg);
        board_btn.enable_interrupt(&mut dp.EXTI);
        board_btn.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

        let interface = I2CDisplayInterface::new(i2c);
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

            Text::new(state_msg, Point::zero(), text_style)
                .draw(&mut disp)
                .unwrap();
            Text::new(&format_buf, Point::new(0, 14), text_style_format_buf)
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
