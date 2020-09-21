//! Generate random numbers using the RNG peripheral and display the values.
//! This example is specifically tuned to run correctly on the
//! stm32f4-discovery board (model STM32F407G-DISC1)
//! This example requires the `rt` feature to be enabled. For example:
//!
//! ```bash
//! cargo run --release --features stm32f407,rt  --example rng-display
//! ```
//!
//! Note that this example requires the `--release` build flag because it is too
//! large to fit in the default `memory.x` file provided with this crate.

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
    fonts::Font6x8, fonts::Text, pixelcolor::BinaryColor, prelude::*, style::TextStyleBuilder,
};

use ssd1306::{prelude::*, Builder, I2CDIBuilder};

use hal::{i2c::I2c, prelude::*, stm32};
use rand_core::RngCore;

use arrayvec::ArrayString;
use core::fmt;

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
        stm32::Peripherals::take(),
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
            .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
            .sysclk(128.mhz())
            .freeze();

        let mut delay_source = hal::delay::Delay::new(cp.SYST, clocks);

        // Set up I2C1: SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        // as per the STM32F407 datasheet. Pin assignment as per the
        // stm32f4-discovery (ST32F407G-DISC1) board.
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

        // Set up the display
        let interface = I2CDIBuilder::new().init(i2c);
        let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
        disp.init().unwrap();

        // enable the RNG peripheral and its clock
        // this will panic if the clock configuration is unsuitable
        let mut rand_source = dp.RNG.constrain(clocks);
        let mut format_buf = ArrayString::<[u8; 20]>::new();
        loop {
            //display clear
            disp.clear();

            //this will continuously report an error if RNG_CLK < HCLK/16
            let rand_val = rand_source.next_u32();

            format_buf.clear();
            if fmt::write(&mut format_buf, format_args!("{}", rand_val)).is_ok() {
                let text_style = TextStyleBuilder::new(Font6x8)
                    .text_color(BinaryColor::On)
                    .build();

                Text::new(format_buf.as_str(), Point::new(HINSET_PIX, VCENTER_PIX))
                    .into_styled(text_style)
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
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
