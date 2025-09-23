#![no_main]
#![no_std]

// Required
extern crate panic_semihosting;

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};

use static_cell::ConstStaticCell;
use stm32f4xx_hal::{
    ltdc::{BluePins, GreenPins, Layer, LtdcPins, PixelFormat, RedPins},
    pac,
    prelude::*,
    rcc::{Config, Rcc},
};

mod screen;

// DIMENSIONS
const WIDTH: u16 = 480;
const HEIGHT: u16 = 272;

// Graphics framebuffer
const FB_GRAPHICS_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
static FB_LAYER1: ConstStaticCell<[u16; FB_GRAPHICS_SIZE]> =
    ConstStaticCell::new([0; FB_GRAPHICS_SIZE]);

#[entry]
fn main() -> ! {
    let perif = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc_hal: Rcc = perif.RCC.constrain();

    // Set up pins
    let _gpioa = perif.GPIOA.split(&mut rcc_hal);
    let _gpiob = perif.GPIOB.split(&mut rcc_hal);
    let gpioe = perif.GPIOE.split(&mut rcc_hal);
    let gpiog = perif.GPIOG.split(&mut rcc_hal);
    let gpioh = perif.GPIOH.split(&mut rcc_hal);
    let gpioi = perif.GPIOI.split(&mut rcc_hal);
    let gpioj = perif.GPIOJ.split(&mut rcc_hal);
    let gpiok = perif.GPIOK.split(&mut rcc_hal);

    let pins = LtdcPins::new(
        RedPins::new(
            gpioi.pi15, gpioj.pj0, gpioj.pj1, gpioj.pj2, gpioj.pj3, gpioj.pj4, gpioj.pj5, gpioj.pj6,
        ),
        GreenPins::new(
            gpioj.pj7, gpioj.pj8, gpioj.pj9, gpioj.pj10, gpioj.pj11, gpiok.pk0, gpiok.pk1,
            gpiok.pk2,
        ),
        BluePins::new(
            gpioe.pe4, gpioj.pj13, gpioj.pj14, gpioj.pj15, gpiog.pg12, gpiok.pk4, gpiok.pk5,
            gpiok.pk6,
        ),
        gpioi.pi10,
        gpioi.pi9,
        gpiok.pk7,
        gpioi.pi14,
    );

    // HSE osc out in High Z
    gpioh.ph1.into_floating_input();
    let _rcc_hal = rcc_hal.freeze(
        Config::hse(25.MHz())
            .bypass_hse_oscillator()
            .sysclk(216.MHz())
            .hclk(216.MHz()),
    );

    // LCD enable: set it low first to avoid LCD bleed while setting up timings
    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();

    // LCD backlight enable
    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    let mut display = screen::Stm32F7DiscoDisplay::new(perif.LTDC, perif.DMA2D, pins);
    display
        .controller
        .config_layer(Layer::L1, FB_LAYER1.take(), PixelFormat::RGB565);

    display.controller.enable_layer(Layer::L1);
    display.controller.reload();

    let display = &mut display;

    // LCD enable: activate LCD !
    disp_on.set_high();

    Rectangle::new(Point::new(0, 0), Size::new(479, 271))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 0b11110, 0b11011)))
        .draw(display)
        .ok();

    let c1 = Circle::new(Point::new(20, 20), 2 * 8)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 63, 0)));

    let c2 = Circle::new(Point::new(25, 20), 2 * 8)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(31, 0, 0)));

    let t = Text::new(
        "Hello Rust!",
        Point::new(100, 100),
        MonoTextStyle::new(&FONT_6X9, RgbColor::WHITE),
    );

    c1.draw(display).ok();
    c2.draw(display).ok();
    t.draw(display).ok();

    for i in 0..300 {
        Circle::new(Point::new(20 + i, 20), 2 * 8)
            .into_styled(PrimitiveStyle::with_fill(RgbColor::GREEN))
            .draw(display)
            .ok();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
