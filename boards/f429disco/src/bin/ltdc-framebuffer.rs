//! STM32F429I-DISCO LTDC screen example using LtdcFramebuffer
//!
//! Draws a colour gradient using the LtdcFramebuffer DrawTarget.
//!
//! ## Build
//!
//! ```bash
//! cargo build --release -p f429disco-examples --bin ltdc-framebuffer
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;

use stm32f4xx_hal::{
    display::LtdcFramebuffer,
    ltdc::{BluePins, DisplayConfig, GreenPins, Layer, LtdcPins, PixelFormat, RedPins},
    pac,
    prelude::*,
    rcc::{Config, Rcc},
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};

use static_cell::ConstStaticCell;

const WIDTH: u16 = 480;
const HEIGHT: u16 = 272;

const DISCO_SCREEN_CONFIG: DisplayConfig = DisplayConfig {
    active_width: WIDTH,
    active_height: HEIGHT,
    h_back_porch: 13,
    h_front_porch: 30,
    h_sync: 41,
    v_back_porch: 2,
    v_front_porch: 2,
    v_sync: 10,
    frame_rate: 60,
    h_sync_pol: false,
    v_sync_pol: false,
    no_data_enable_pol: false,
    pixel_clock_pol: false,
};

const FB_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
static FB_LAYER1: ConstStaticCell<[u16; FB_SIZE]> = ConstStaticCell::new([0; FB_SIZE]);

#[entry]
fn main() -> ! {
    let perif = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc_hal: Rcc = perif.RCC.constrain();

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
            gpioi.pi15, gpioj.pj0, gpioj.pj1, gpioj.pj2, gpioj.pj3, gpioj.pj4, gpioj.pj5,
            gpioj.pj6,
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

    gpioh.ph1.into_floating_input();
    let _rcc_hal = rcc_hal.freeze(
        Config::hse(25.MHz())
            .bypass_hse_oscillator()
            .sysclk(216.MHz())
            .hclk(216.MHz()),
    );

    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();

    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    let mut controller = stm32f4xx_hal::ltdc::DisplayController::<u16>::new(
        perif.LTDC,
        perif.DMA2D,
        Some(pins),
        PixelFormat::RGB565,
        DISCO_SCREEN_CONFIG,
        Some(25.MHz()),
    );

    let fb = FB_LAYER1.take();

    // Create the LtdcFramebuffer from the same buffer we pass to the LTDC layer.
    // The LTDC hardware reads from this buffer, and we write to it via LtdcFramebuffer.
    // Both reference the same memory: the LTDC reads it for scanout, we write pixels.
    let fb_ptr = fb.as_mut_ptr();
    controller.config_layer(Layer::L1, fb, PixelFormat::RGB565);
    controller.enable_layer(Layer::L1);
    controller.reload();

    disp_on.set_high();

    // ── Use LtdcFramebuffer for DrawTarget ──────────────────────────────
    // Safety: the LTDC hardware reads from this buffer for scanout, and we
    // write to it. This is the standard single-buffered LTDC usage pattern.
    let fb_ref = unsafe { core::slice::from_raw_parts_mut(fb_ptr, FB_SIZE) };
    let mut display = LtdcFramebuffer::new(fb_ref, WIDTH, HEIGHT);

    // Background
    Rectangle::new(Point::new(0, 0), Size::new(WIDTH as u32, HEIGHT as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 0b11110, 0b11011)))
        .draw(&mut display)
        .ok();

    // Circles
    Circle::new(Point::new(20, 20), 16)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
        .draw(&mut display)
        .ok();

    Circle::new(Point::new(25, 20), 16)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut display)
        .ok();

    // Text
    Text::new(
        "Hello from LtdcFramebuffer!",
        Point::new(100, 100),
        MonoTextStyle::new(&FONT_6X9, RgbColor::WHITE),
    )
    .draw(&mut display)
    .ok();

    controller.reload();

    loop {}
}
