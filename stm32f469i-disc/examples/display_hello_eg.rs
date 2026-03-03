//! Hello embedded-graphics on the STM32F469I-DISCO display.
//!
//! Renders text and colored shapes using the BSP `lcd` module and
//! `LtdcFramebuffer` as the DrawTarget.
//!
//! Run: `cargo run --release --example display_hello_eg --features framebuffer`

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

use stm32f469i_disc as board;

use board::hal::gpio::alt::fmc as alt;
use board::hal::ltdc::{Layer, LtdcFramebuffer, PixelFormat};
use board::hal::pac::{CorePeripherals, Peripherals};
use board::hal::{prelude::*, rcc};
use board::lcd;
use board::sdram::{sdram_pins, Sdram};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::Text,
};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(rcc::Config::hse(8.MHz()).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let _gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let gpiod = dp.GPIOD.split(&mut rcc);
    let gpioe = dp.GPIOE.split(&mut rcc);
    let gpiof = dp.GPIOF.split(&mut rcc);
    let gpiog = dp.GPIOG.split(&mut rcc);
    let gpioh = dp.GPIOH.split(&mut rcc);
    let gpioi = dp.GPIOI.split(&mut rcc);

    // LCD reset
    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    // Initialize SDRAM for framebuffer
    defmt::info!("Initializing SDRAM...");
    let sdram = Sdram::new(
        dp.FMC,
        sdram_pins! {gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi},
        &rcc.clocks,
        &mut delay,
    );

    let buffer: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(sdram.mem as *mut u16, lcd::FB_SIZE) };
    let mut fb = LtdcFramebuffer::new(buffer, lcd::WIDTH, lcd::HEIGHT);

    // Draw with embedded-graphics
    fb.clear(Rgb565::BLACK).ok();

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("Hello embedded-graphics!", Point::new(40, 60), text_style)
        .draw(&mut fb)
        .ok();

    Rectangle::new(Point::new(50, 100), Size::new(200, 100))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut fb)
        .ok();

    Circle::new(Point::new(300, 100), 80)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
        .draw(&mut fb)
        .ok();

    Triangle::new(
        Point::new(50, 400),
        Point::new(200, 300),
        Point::new(350, 400),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
    .draw(&mut fb)
    .ok();

    let buffer = fb.into_inner();

    // Initialize display
    defmt::info!("Initializing display...");
    let (mut display_ctrl, _controller) = lcd::init_display_full(
        dp.DSI,
        dp.LTDC,
        dp.DMA2D,
        &mut rcc,
        &mut delay,
        lcd::BoardHint::Unknown,
        PixelFormat::RGB565,
    );
    display_ctrl.config_layer(Layer::L1, buffer, PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    defmt::info!("Hello embedded-graphics! Display ready.");
    loop {
        cortex_m::asm::wfi();
    }
}
