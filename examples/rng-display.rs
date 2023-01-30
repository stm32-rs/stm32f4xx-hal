//! Generate random numbers using the RNG peripheral and display the values.
//! This example is specifically tuned to run correctly on the
//! stm32f4-discovery board (model STM32F407G-DISC1)
//! For example:
//!
//! ```bash
//! cargo run --release --features stm32f407 --example rng-display
//! ```
//!
//! Note that this example requires the `--release` build flag because it is too
//! large to fit in the default `memory.x` file provided with this crate.

#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

#[cfg(not(debug_assertions))]
use panic_halt as _;
#[cfg(debug_assertions)]
use panic_semihosting as _;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use hal::{i2c::I2c, pac, prelude::*};
use rand_core::RngCore;

use core::fmt::Write;
use heapless::String;

// dimensions of SSD1306 OLED display known to work
pub const SCREEN_WIDTH: i32 = 128;
pub const SCREEN_HEIGHT: i32 = 64;
pub const FONT_HEIGHT: i32 = 8;
/// height of embedded font, in pixels
pub const VCENTER_PIX: i32 = (SCREEN_HEIGHT - FONT_HEIGHT) / 2;
pub const HINSET_PIX: i32 = 20;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();

        // Clock configuration is critical for RNG to work properly; otherwise
        // RNG_SR CECS bit will constantly report an error (if RNG_CLK < HCLK/16)
        // here we pick a simple clock configuration that ensures the pll48clk,
        // from which RNG_CLK is derived, is about 48 MHz
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz()) //discovery board has 8 MHz crystal for HSE
            .sysclk(128.MHz())
            .freeze();

        let mut delay_source = cp.SYST.delay(&clocks);

        // Set up I2C1: SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        // as per the STM32F407 datasheet. Pin assignment as per the
        // stm32f4-discovery (ST32F407G-DISC1) board.
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate().set_open_drain();
        let sda = gpiob.pb9.into_alternate().set_open_drain();
        let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);

        // Set up the display
        let interface = I2CDisplayInterface::new(i2c);
        let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        disp.init().unwrap();

        // enable the RNG peripheral and its clock
        // this will panic if the clock configuration is unsuitable
        let mut rand_source = dp.RNG.constrain(&clocks);
        let mut format_buf = String::<20>::new();
        loop {
            //display clear
            disp.clear();

            //this will continuously report an error if RNG_CLK < HCLK/16
            let rand_val = rand_source.next_u32();

            format_buf.clear();
            if write!(&mut format_buf, "{rand_val}").is_ok() {
                let text_style = MonoTextStyleBuilder::new()
                    .font(&FONT_5X8)
                    .text_color(BinaryColor::On)
                    .build();

                Text::new(&format_buf, Point::new(HINSET_PIX, VCENTER_PIX), text_style)
                    .draw(&mut disp)
                    .unwrap();
            }
            disp.flush().unwrap();
            //delay a little while between refreshes so the display is readable
            delay_source.delay_ms(100u8);
        }
    }

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
