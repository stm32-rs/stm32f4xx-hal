//! DSI LCD test — rolling gradient on the STM32F469I-DISCO display.
//!
//! Uses the BSP `lcd` module for display initialization, supporting both
//! NT35510 (B08) and OTM8009A (B07) panels automatically.
//!
//! Run: `cargo run --release --example display_dsi_lcd`

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

use stm32f469i_disc as board;

use board::hal::gpio::alt::fmc as alt;
use board::hal::ltdc::{Layer, PixelFormat};
use board::hal::{pac, prelude::*, rcc};
use board::lcd;
use board::sdram::{sdram_pins, Sdram};

fn hue2rgb(hue: u32, level: u32) -> u32 {
    let hue = hue % 360;
    let sector: u32 = hue / 60;
    let fraction = hue % 60;
    let none = 0;
    let full = level;
    let rise = (level * fraction) / 60;
    let fall = (level * (60 - fraction)) / 60;
    let rgb = match sector {
        0 => (full, rise, none),
        1 => (fall, full, none),
        2 => (none, full, rise),
        3 => (none, fall, full),
        4 => (rise, none, full),
        5 => (full, none, fall),
        _ => (none, none, none),
    };
    rgb.2 | (rgb.1 << 8) | (rgb.0 << 16)
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(rcc::Config::hse(8.MHz()).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);
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
    let fb = unsafe { core::slice::from_raw_parts_mut(sdram.mem, sdram.words) };

    // Initialize display using BSP lcd module
    defmt::info!("Initializing display...");
    let (mut display_ctrl, _controller) = lcd::init_display_full(
        dp.DSI,
        dp.LTDC,
        dp.DMA2D,
        &mut rcc,
        &mut delay,
        lcd::BoardHint::Unknown,
        PixelFormat::ARGB8888,
    );
    display_ctrl.config_layer(Layer::L1, fb, PixelFormat::ARGB8888);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    defmt::info!("Display ready — rolling gradient");

    // Rolling gradient animation
    let mut hue = 0u32;
    let ratio = 3;
    let speed = 3;
    loop {
        let mut addr = 0;
        for row in 0..lcd::HEIGHT as u32 {
            let rgb = hue2rgb((hue + row) / ratio, 255);
            for _col in 0..lcd::WIDTH as u32 {
                fb[addr] = rgb;
                addr += 1;
            }
        }
        hue += speed * if gpioa.pa0.is_high() { 5 } else { 1 };
        delay.delay_ms(15u32);
    }
}
