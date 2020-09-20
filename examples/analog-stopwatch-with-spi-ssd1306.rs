//! This is an advanced example of stopwatch-with-ssd1306-and-interrupts.rs
//!
//! This example requires the `rt` feature to be enabled. For this example, you need stm32f429i-disco (development kit) and ssd1306 (SPI).
//!
//! You could find more details about this example here: http://www.mcu.by/rust-embedded-stopwatch/

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f4xx_hal as hal;

use crate::hal::{
    gpio::{gpioa::PA0, Edge, ExtiPin, Input, PullDown},
    interrupt, pac,
    prelude::*,
    rcc::{Clocks, Rcc},
    spi::Spi,
    stm32,
    timer::{Event, Timer},
};

use arrayvec::ArrayString;
use core::cell::{Cell, RefCell};
use core::fmt;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, CriticalSection, Mutex};

use hal::spi::{Mode, Phase, Polarity};

use core::f32::consts::{FRAC_PI_2, PI};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    egcircle,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitive_style,
    primitives::{Circle, Line},
    style::TextStyleBuilder,
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};
use micromath::F32Ext;

use ssd1306::{prelude::*, Builder};

// Set up global state. It's all mutexed up for concurrency safety.
static ELAPSED_MS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0u32));
static ELAPSED_RESET_MS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0u32));
static TIMER_TIM2: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));
static STATE: Mutex<Cell<StopwatchState>> = Mutex::new(Cell::new(StopwatchState::Ready));
static BUTTON: Mutex<RefCell<Option<PA0<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));

/// The center of the clock face
const CENTER: Point = Point::new(64, 40);

/// The radius of the clock face
const SIZE: u32 = 23;

/// Start at the top of the circle
const START: f32 = -FRAC_PI_2;

#[derive(Clone, Copy)]
enum StopwatchState {
    Ready,
    Running,
    Stopped,
}

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    dp.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    let rcc = dp.RCC.constrain();

    let clocks = setup_clocks(rcc);

    let gpioa = dp.GPIOA.split();
    let gpioe = dp.GPIOE.split();

    let mut board_btn = gpioa.pa0.into_pull_down_input();
    board_btn.make_interrupt_source(&mut dp.SYSCFG);
    board_btn.enable_interrupt(&mut dp.EXTI);
    board_btn.trigger_on_edge(&mut dp.EXTI, Edge::FALLING);

    //spi4
    //sck  - pe2
    //miso - pe5
    //mosi - pe6
    //cs - pe4
    //dc - pe3

    let sck = gpioe.pe2.into_alternate_af5();
    let miso = gpioe.pe5.into_alternate_af5();
    let mosi = gpioe.pe6.into_alternate_af5();

    let spi = Spi::spi4(
        dp.SPI4,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        stm32f4xx_hal::time::KiloHertz(2000).into(),
        clocks,
    );

    // Set up the LEDs. On the stm32f429i-disco they are connected to pin PG13 and PG14.
    let gpiog = dp.GPIOG.split();
    let mut led3 = gpiog.pg13.into_push_pull_output();
    let mut led4 = gpiog.pg14.into_push_pull_output();

    let dc = gpioe.pe3.into_push_pull_output();
    let mut ss = gpioe.pe4.into_push_pull_output();
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    ss.set_high().unwrap();
    delay.delay_ms(100_u32);
    ss.set_low().unwrap();

    // Set up the display
    let interface = SPIInterfaceNoCS::new(spi, dc);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    disp.init().unwrap();
    disp.flush().unwrap();

    // Create a 1ms periodic interrupt from TIM2
    let mut timer = Timer::tim2(dp.TIM2, 1.khz(), clocks);
    timer.listen(Event::TimeOut);

    free(|cs| {
        TIMER_TIM2.borrow(cs).replace(Some(timer));
        BUTTON.borrow(cs).replace(Some(board_btn));
    });

    // Enable interrupts
    stm32::NVIC::unpend(hal::stm32::Interrupt::TIM2);
    stm32::NVIC::unpend(hal::stm32::Interrupt::EXTI0);
    unsafe {
        stm32::NVIC::unmask(hal::stm32::Interrupt::EXTI0);
    };

    let mut state_led = false;
    loop {
        let elapsed = free(|cs| ELAPSED_MS.borrow(cs).get());

        let mut format_buf = ArrayString::<[u8; 10]>::new();
        format_elapsed(&mut format_buf, elapsed);

        disp.clear();

        let state = free(|cs| STATE.borrow(cs).get());
        let state_msg = match state {
            StopwatchState::Ready => "Ready",
            StopwatchState::Running => "Running",
            StopwatchState::Stopped => "Stopped",
        };

        state_led = !state_led;

        match state {
            StopwatchState::Ready => {
                led3.set_high().unwrap();
                led4.set_low().unwrap();
            }
            StopwatchState::Running => {
                if state_led {
                    led4.set_low().unwrap();
                    led3.set_high().unwrap();
                } else {
                    led4.set_low().unwrap();
                    led3.set_low().unwrap();
                }
            }
            StopwatchState::Stopped => {
                led3.set_low().unwrap();
                led4.set_high().unwrap();
            }
        };

        let text_style = TextStyleBuilder::new(Font6x8)
            .text_color(BinaryColor::On)
            .build();

        Text::new(state_msg, Point::new(0, 0))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        Text::new(format_buf.as_str(), Point::new((128 / 2) - 1, 0))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        draw_face().draw(&mut disp).unwrap();
        draw_seconds_hand(elapsed_to_s(elapsed))
            .draw(&mut disp)
            .unwrap();

        disp.flush().unwrap();

        delay.delay_ms(100u32);
    }
}

fn setup_clocks(rcc: Rcc) -> Clocks {
    return rcc
        .cfgr
        .hclk(180.mhz())
        .sysclk(180.mhz())
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .freeze();
}

#[interrupt]
fn EXTI0() {
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
                    ELAPSED_RESET_MS.borrow(cs).replace(0);
                    stopwatch_start(cs);
                    STATE.borrow(cs).replace(StopwatchState::Running);
                }
                StopwatchState::Running => {
                    ELAPSED_RESET_MS.borrow(cs).replace(0);
                    stopwatch_stop(cs);
                    stopwatch_reset_start(cs);
                    STATE.borrow(cs).replace(StopwatchState::Stopped);
                }
                StopwatchState::Stopped => {
                    let cell_reset = ELAPSED_RESET_MS.borrow(cs);
                    let val_reset = cell_reset.get();

                    if val_reset > 500_u32 {
                        ELAPSED_MS.borrow(cs).replace(0);
                        stopwatch_reset_stop(cs);
                        STATE.borrow(cs).replace(StopwatchState::Ready);
                    } else {
                        stopwatch_reset_stop(cs);
                        stopwatch_continue(cs);
                        STATE.borrow(cs).replace(StopwatchState::Running);
                    }
                }
            }
        }
    });
}

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            tim2.clear_interrupt(Event::TimeOut);
        }

        let cell = ELAPSED_MS.borrow(cs);
        let cell_reset = ELAPSED_RESET_MS.borrow(cs);
        let val = cell.get();
        let val_reset = cell_reset.get();

        match STATE.borrow(cs).get() {
            StopwatchState::Ready => {
                cell.replace(val + 1);
            }
            StopwatchState::Running => {
                cell.replace(val + 1);
            }
            StopwatchState::Stopped => {
                let mut btn_ref = BUTTON.borrow(cs).borrow_mut();
                if let Some(ref mut btn) = btn_ref.deref_mut() {
                    if btn.is_high().unwrap() {
                        cell_reset.replace(val_reset + 1);
                    }
                }
            }
        }
    });
}

fn stopwatch_start<'cs>(cs: &'cs CriticalSection) {
    ELAPSED_MS.borrow(cs).replace(0);
    unsafe {
        stm32::NVIC::unmask(hal::stm32::Interrupt::TIM2);
    }
}

fn stopwatch_continue<'cs>(_cs: &'cs CriticalSection) {
    unsafe {
        stm32::NVIC::unmask(hal::stm32::Interrupt::TIM2);
    }
}

fn stopwatch_stop<'cs>(_cs: &'cs CriticalSection) {
    stm32::NVIC::mask(hal::stm32::Interrupt::TIM2);
}

fn stopwatch_reset_start<'cs>(cs: &'cs CriticalSection) {
    ELAPSED_RESET_MS.borrow(cs).replace(0);
    unsafe {
        stm32::NVIC::unmask(hal::stm32::Interrupt::TIM2);
    }
}

fn stopwatch_reset_stop<'cs>(_cs: &'cs CriticalSection) {
    stm32::NVIC::mask(hal::stm32::Interrupt::TIM2);
}

// Formatting requires the arrayvec crate
fn format_elapsed(buf: &mut ArrayString<[u8; 10]>, elapsed: u32) {
    let minutes = elapsed_to_m(elapsed);
    let seconds = elapsed_to_s(elapsed);
    let millis = elapsed_to_ms(elapsed);
    fmt::write(
        buf,
        format_args!("{}:{:02}.{:03}", minutes, seconds, millis),
    )
    .unwrap();
}

fn elapsed_to_ms(elapsed: u32) -> u32 {
    return elapsed % 1000;
}

fn elapsed_to_s(elapsed: u32) -> u32 {
    return (elapsed - elapsed_to_ms(elapsed)) % 60000 / 1000;
}

fn elapsed_to_m(elapsed: u32) -> u32 {
    return elapsed / 60000;
}

/// Convert a polar coordinate (angle/distance) into an (X, Y) coordinate centered around `CENTER`
fn polar(angle: f32, radius: f32) -> Point {
    CENTER + Point::new((angle.cos() * radius) as i32, (angle.sin() * radius) as i32)
}

/// Draw a circle and 12 tics as a simple clock face
fn draw_face() -> impl Iterator<Item = Pixel<BinaryColor>> {
    let tic_len = 3.0;

    // Use the circle macro to create the outer face
    let face = egcircle!(
        center = CENTER,
        radius = SIZE,
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1)
    );

    // Create 12 `Line`s starting from the outer edge and drawing inwards by `tic_len` pixels
    let tics = (0..12).into_iter().map(move |index| {
        // Start angle around the circle, in radians
        let angle = START + (PI * 2.0 / 12.0) * index as f32;

        // Start point on circumference
        let start = polar(angle, SIZE as f32);

        // End point; start point offset by `tic_len` pixels towards the circle center
        let end = polar(angle, SIZE as f32 - tic_len);

        Line::new(start, end)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter()
    });

    // Create a single iterator of pixels, first iterating over the circle, then over the 12 lines
    // generated
    face.into_iter().chain(tics.flatten())
}

/// Draw the seconds hand given a seconds value (0 - 59)
fn draw_seconds_hand(seconds: u32) -> impl Iterator<Item = Pixel<BinaryColor>> {
    // Convert seconds into a position around the circle in radians
    let seconds_radians = ((seconds as f32 / 60.0) * 2.0 * PI) + START;

    let end = polar(seconds_radians, SIZE as f32);

    // Basic line hand
    let hand = Line::new(CENTER, end).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

    // Decoration position
    let decoration_position = polar(seconds_radians, SIZE as f32 - 23.0);

    // Decoration style
    let decoration_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::Off)
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    // Add a fancy circle near the end of the hand
    let decoration = Circle::new(decoration_position, 3).into_styled(decoration_style);

    hand.into_iter().chain(&decoration)
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
