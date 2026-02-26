//! STM32F469I-DISCO animated layers example (port of ST LCD_AnimatedPictureFromSDCard).
//!
//! Background gradient on layer 1, animated colored rectangle on layer 2 with
//! color keying so the background shows through. Cycles colors and bounces the
//! rectangle around the screen.
//!
//! Supports both board revisions via runtime panel autodetection (NT35510 / OTM8009A).
//!
//! Build:
//! ```bash
//! cargo build --release --example f469disco-animated-layers --features="stm32f469,stm32-fmc,framebuffer,defmt"
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use stm32f4xx_hal::{self as hal, rcc::Config};

use hal::{
    fmc::FmcExt,
    gpio::alt::fmc as fmc_alt,
    ltdc::{Layer, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use stm32f4xx_hal::display::f469disco as display_init;
#[path = "f469disco/images.rs"]
mod images;

use display_init::{FB_SIZE, HEIGHT, WIDTH};
use images::{fill_gradient, fill_solid, COLOR_KEY};
use stm32_fmc::devices::is42s32400f_6;

const RECT_W: usize = 120;
const RECT_H: usize = 120;

macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        ($(fmc_alt::$alt::from($pin.internal_pull_up(true))),*)
    };
}

fn blit_rect(dst: &mut [u16], dst_w: usize, x0: usize, y0: usize, w: usize, h: usize, color: u16) {
    for y in y0..(y0 + h).min(dst.len() / dst_w) {
        for x in x0..(x0 + w).min(dst_w) {
            dst[y * dst_w + x] = color;
        }
    }
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(Config::hse(8.MHz()).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioc = dp.GPIOC.split(&mut rcc);
    let gpiod = dp.GPIOD.split(&mut rcc);
    let gpioe = dp.GPIOE.split(&mut rcc);
    let gpiof = dp.GPIOF.split(&mut rcc);
    let gpiog = dp.GPIOG.split(&mut rcc);
    let gpioh = dp.GPIOH.split(&mut rcc);
    let gpioi = dp.GPIOI.split(&mut rcc);

    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    #[rustfmt::skip]
    let fmc_pins = fmc_pins! {
        A0: gpiof.pf0, A1: gpiof.pf1, A2: gpiof.pf2, A3: gpiof.pf3,
        A4: gpiof.pf4, A5: gpiof.pf5, A6: gpiof.pf12, A7: gpiof.pf13,
        A8: gpiof.pf14, A9: gpiof.pf15, A10: gpiog.pg0, A11: gpiog.pg1,
        Ba0: gpiog.pg4, Ba1: gpiog.pg5,
        D0: gpiod.pd14, D1: gpiod.pd15, D2: gpiod.pd0, D3: gpiod.pd1,
        D4: gpioe.pe7, D5: gpioe.pe8, D6: gpioe.pe9, D7: gpioe.pe10,
        D8: gpioe.pe11, D9: gpioe.pe12, D10: gpioe.pe13, D11: gpioe.pe14,
        D12: gpioe.pe15, D13: gpiod.pd8, D14: gpiod.pd9, D15: gpiod.pd10,
        D16: gpioh.ph8, D17: gpioh.ph9, D18: gpioh.ph10, D19: gpioh.ph11,
        D20: gpioh.ph12, D21: gpioh.ph13, D22: gpioh.ph14, D23: gpioh.ph15,
        D24: gpioi.pi0, D25: gpioi.pi1, D26: gpioi.pi2, D27: gpioi.pi3,
        D28: gpioi.pi6, D29: gpioi.pi7, D30: gpioi.pi9, D31: gpioi.pi10,
        Nbl0: gpioe.pe0, Nbl1: gpioe.pe1, Nbl2: gpioi.pi4, Nbl3: gpioi.pi5,
        Sdcke0: gpioh.ph2, Sdclk: gpiog.pg8,
        Sdncas: gpiog.pg15, Sdne0: gpioh.ph3,
        Sdnras: gpiof.pf11, Sdnwe: gpioc.pc0,
    };

    defmt::info!("Initializing SDRAM...");
    let mut sdram = dp
        .FMC
        .sdram(fmc_pins, is42s32400f_6::Is42s32400f6 {}, &rcc.clocks);
    let base_ptr = sdram.init(&mut delay) as *mut u16;
    assert!(!base_ptr.is_null());

    let layer1_buf: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr, FB_SIZE) };
    let layer2_buf: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr.add(FB_SIZE), FB_SIZE) };

    fill_gradient(layer1_buf, WIDTH, HEIGHT);
    fill_solid(layer2_buf, COLOR_KEY);

    defmt::info!("Configuring LTDC and initializing display...");
    let (mut display_ctrl, _controller) = display_init::init_display_full(
        dp.DSI,
        dp.LTDC,
        dp.DMA2D,
        &mut rcc,
        &mut delay,
        display_init::BoardHint::Unknown,
        PixelFormat::RGB565,
    );
    display_ctrl.config_layer(Layer::L1, layer1_buf, hal::ltdc::PixelFormat::RGB565);
    display_ctrl.config_layer(Layer::L2, layer2_buf, hal::ltdc::PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.enable_layer(Layer::L2);
    display_ctrl.set_color_keying(Layer::L2, 0x000000);
    display_ctrl.reload();

    defmt::info!("Animated layers running — bouncing rectangle with color keying");

    let colors: [u16; 4] = [0xF800, 0x07E0, 0x001F, 0xFFFF];
    let mut color_idx = 0usize;
    let mut rx: i32 = 100;
    let mut ry: i32 = 200;
    let mut dx: i32 = 4;
    let mut dy: i32 = 3;

    loop {
        if let Some(buf) = display_ctrl.layer_buffer_mut(Layer::L2) {
            fill_solid(buf, COLOR_KEY);
            blit_rect(
                buf,
                WIDTH as usize,
                rx as usize,
                ry as usize,
                RECT_W,
                RECT_H,
                colors[color_idx],
            );
        }

        rx += dx;
        ry += dy;
        if rx <= 0 || rx + RECT_W as i32 >= WIDTH as i32 {
            dx = -dx;
            rx = rx.clamp(0, WIDTH as i32 - RECT_W as i32);
            color_idx = (color_idx + 1) % colors.len();
        }
        if ry <= 0 || ry + RECT_H as i32 >= HEIGHT as i32 {
            dy = -dy;
            ry = ry.clamp(0, HEIGHT as i32 - RECT_H as i32);
            color_idx = (color_idx + 1) % colors.len();
        }

        delay.delay_ms(16u32);
    }
}
