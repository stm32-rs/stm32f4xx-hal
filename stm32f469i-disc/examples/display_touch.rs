//! Touch swipe example for STM32F469I-DISCO
//!
//! Demonstrates the FT6X06 touch controller by detecting swipes left/right
//! and cycling through patterns on the display.
//!
//! Run: `cargo run --release --example display_touch`

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

use stm32f469i_disc as board;

use board::hal::ltdc::{Layer, PixelFormat};
use board::hal::{pac, prelude::*, rcc};
use board::lcd;
use board::sdram::{alt, sdram_pins, Sdram};
use board::touch;

/// Convert hue to RGB565 color (16-bit)
/// Creates a gradient effect with the given hue base
fn hue_to_rgb565(hue: u32, level: u32) -> u16 {
    let hue = hue % 360;
    let sector: u32 = hue / 60;
    let fraction = hue % 60;
    let none = 0;
    let full = level;
    let rise = (level * fraction) / 60;
    let fall = (level * (60 - fraction)) / 60;
    
    let (r, g, b) = match sector {
        0 => (full, rise, none),
        1 => (fall, full, none),
        2 => (none, full, rise),
        3 => (none, fall, full),
        4 => (rise, none, full),
        5 => (full, none, fall),
        _ => (none, none, none),
    };
    
    // Convert 8-bit RGB to RGB565: R5-G6-B5
    let r5 = (r >> 3) as u16;
    let g6 = (g >> 2) as u16;
    let b5 = (b >> 3) as u16;
    (r5 << 11) | (g6 << 5) | b5
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(rcc::Config::hse(8.MHz()).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpiob = dp.GPIOB.split(&mut rcc);
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

    // Initialize touch controller I2C (individual pins, not Parts structs)
    // and extract touch interrupt pin BEFORE passing gpioc to sdram_pins!
    defmt::info!("Initializing touch controller I2C...");
    let mut i2c = touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);
    let ts_int = gpioc.pc1.into_pull_down_input();

    // Initialize SDRAM
    defmt::info!("Initializing SDRAM...");
    let sdram = Sdram::new(
        dp.FMC,
        sdram_pins! {gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi},
        &rcc.clocks,
        &mut delay,
    );

    // Get raw pointer to framebuffer (avoids borrow checker issues with config_layer)
    let fb_ptr: *mut u16 = sdram.mem as *mut u16;
    
    // Initialize display with RGB565 (compatible with DisplayController<u16>)
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
    
    // Create static slice for config_layer (this is safe because SDRAM is static)
    let fb: &'static mut [u16] = unsafe {
        core::slice::from_raw_parts_mut(fb_ptr, lcd::FB_SIZE)
    };
    display_ctrl.config_layer(Layer::L1, fb, PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    // Initialize touch controller (no calibration for this simple example)
    defmt::info!("Initializing touch controller...");
    let mut touch_ctrl = touch::init_ft6x06(&i2c, ts_int);
    if touch_ctrl.is_some() {
        defmt::info!("FT6X06 touch controller initialized");
    } else {
        defmt::warn!("FT6X06 touch controller not detected");
    }

    let mut pattern_num = 0u32;
    let mut touch_start_x: Option<i32> = None;

    loop {
        // Fill screen with current pattern using raw pointer
        let hue_base = pattern_num * 60;
        for row in 0..lcd::HEIGHT as usize {
            let hue = hue_base + row as u32;
            for col in 0..lcd::WIDTH as usize {
                let rgb565 = hue_to_rgb565(hue, 255);
                unsafe {
                    *fb_ptr.add(row * lcd::WIDTH as usize + col) = rgb565;
                }
            }
        }

        // Check for touch input
        if let Some(t) = touch_ctrl.as_mut() {
            if let Ok(num) = t.detect_touch(&mut i2c) {
                if num > 0 {
                    if let Ok(point) = t.get_touch(&mut i2c, 1) {
                        let x = point.x as i32;
                        if touch_start_x.is_none() {
                            touch_start_x = Some(x);
                        }

                        // Detect swipe
                        if let Some(start_x) = touch_start_x {
                            let delta = x - start_x;
                            // Swipe right: delta > 50 (go to previous pattern)
                            if delta > 50 {
                                pattern_num = pattern_num.saturating_sub(1);
                                touch_start_x = None;
                                defmt::info!("Swipe right -> pattern {}", pattern_num);
                            }
                            // Swipe left: delta < -50 (go to next pattern)
                            else if delta < -50 {
                                pattern_num = pattern_num.saturating_add(1);
                                touch_start_x = None;
                                defmt::info!("Swipe left -> pattern {}", pattern_num);
                            }
                        }
                    }
                } else {
                    // No touch detected
                    touch_start_x = None;
                }
            }
        }

        delay.delay_ms(15u32);
    }
}
