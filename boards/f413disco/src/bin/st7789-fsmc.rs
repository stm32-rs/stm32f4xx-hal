//! STM32F413H-DISCO ST7789 LCD example via FSMC
//!
//! Draws colour bars on the ST7789H2 display using the FSMC parallel bus
//! interface (16-bit data, 8080 protocol).
//!
//! ## Build
//!
//! ```bash
//! cargo build --release -p f413disco-examples --bin st7789-fsmc
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::{self as hal, rcc::Config};

use hal::{
    fsmc_lcd::{DataPins16, FsmcLcd, LcdPins, Timing},
    gpio::{alt::fsmc as alt, Speed},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use embedded_graphics_07::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use st7789::ST7789;

const WIDTH: u16 = 240;
const HEIGHT: u16 = 240;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("STM32F413H-DISCO ST7789 FSMC example");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(100.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    // ── GPIO setup ──────────────────────────────────────────────────────
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpiod = dp.GPIOD.split(&mut rcc);
    let gpioe = dp.GPIOE.split(&mut rcc);
    let gpiof = dp.GPIOF.split(&mut rcc);
    let gpiog = dp.GPIOG.split(&mut rcc);

    // ── FSMC pins ───────────────────────────────────────────────────────
    let lcd_pins = LcdPins::new(
        DataPins16::new(
            gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
            gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
            gpiod.pd9, gpiod.pd10,
        ),
        alt::Address::from(gpiof.pf0),
        gpiod.pd4,
        gpiod.pd5,
        alt::ChipSelect3::from(gpiog.pg10),
    );

    let rst = gpiob.pb13.into_push_pull_output().speed(Speed::VeryHigh);
    let _te = gpiob.pb14.into_floating_input();

    let read_timing = Timing::default().data(8).address_setup(8).bus_turnaround(0);
    let write_timing = Timing::default().data(3).address_setup(3).bus_turnaround(0);

    let (_fsmc, lcd) = FsmcLcd::new(dp.FSMC, lcd_pins, &read_timing, &write_timing, &mut rcc);

    // ── Initialise ST7789 ───────────────────────────────────────────────
    rprintln!("Initialising ST7789...");
    let mut display = ST7789::new(
        lcd,
        Some(rst),
        Some(gpioe.pe5.into_push_pull_output()),
        WIDTH,
        HEIGHT,
    );
    display.init(&mut delay).unwrap();
    display
        .set_orientation(st7789::Orientation::Portrait)
        .unwrap();

    // ── Draw colour bars ────────────────────────────────────────────────
    // Note: ST7789 implements DrawTarget from embedded-graphics.
    // This drawing code works identically on LTDC boards using LtdcFramebuffer.
    rprintln!("Drawing colour bars...");

    let bar_h = HEIGHT as u32 / 4;
    let colors = [Rgb565::RED, Rgb565::GREEN, Rgb565::BLUE, Rgb565::WHITE];

    for (i, &color) in colors.iter().enumerate() {
        Rectangle::new(
            Point::new(0, (i as u32 * bar_h) as i32),
            Size::new(WIDTH as u32, bar_h),
        )
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(&mut display)
        .unwrap();
    }

    rprintln!("Done. Looping.");
    loop {
        cortex_m::asm::wfi();
    }
}
