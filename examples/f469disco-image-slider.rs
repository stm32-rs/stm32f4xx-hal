//! STM32F469I-DISCO image slider example (simplified port of ST LCD_DSI_ImagesSlider).
//!
//! Touch swipe left/right to cycle through full-screen patterns. Uses dual
//! buffers and set_layer_buffer_address to switch images. Patterns are
//! generated in SDRAM — no QSPI or external storage needed.
//!
//! Supports both board revisions via runtime panel autodetection (NT35510 / OTM8009A).
//!
//! Build:
//! ```bash
//! cargo build --release --example f469disco-image-slider --features="stm32f469,stm32-fmc,framebuffer,defmt"
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use panic_probe as _;

use stm32f4xx_hal::{self as hal, rcc::Config};

use hal::{
    fmc::FmcExt,
    gpio::alt::fmc as fmc_alt,
    i2c::I2c,
    ltdc::{Layer, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use ft6x06::Ft6X06;

#[path = "f469disco/board.rs"]
mod board;
#[path = "f469disco/images.rs"]
mod images;

use board::{FB_SIZE, HEIGHT, WIDTH};
use images::SlidePattern;
use stm32_fmc::devices::is42s32400f_6;

const SWIPE_THRESHOLD: i32 = 80;

macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        ($(fmc_alt::$alt::from($pin.internal_pull_up(true))),*)
    };
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(Config::hse(8.MHz()).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpiob = dp.GPIOB.split(&mut rcc);
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

    #[cfg(feature = "defmt")]
    defmt::info!("Initializing SDRAM...");
    let mut sdram = dp
        .FMC
        .sdram(fmc_pins, is42s32400f_6::Is42s32400f6 {}, &rcc.clocks);
    let base_ptr = sdram.init(&mut delay) as *mut u16;
    assert!(!base_ptr.is_null());

    let buf_a: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr, FB_SIZE) };
    let buf_b: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr.add(FB_SIZE), FB_SIZE) };

    let patterns = SlidePattern::ALL;
    SlidePattern::ColorBars.fill(buf_a, WIDTH, HEIGHT);
    SlidePattern::SolidRed.fill(buf_b, WIDTH, HEIGHT);

    #[cfg(feature = "defmt")]
    defmt::info!("Configuring LTDC and initializing display...");
    let (mut display_ctrl, _controller) = board::init_display_full(
        dp.DSI,
        dp.LTDC,
        dp.DMA2D,
        &mut rcc,
        &mut delay,
        board::BoardHint::Unknown,
        PixelFormat::RGB565,
    );
    display_ctrl.config_layer(Layer::L1, buf_a, hal::ltdc::PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    #[cfg(feature = "defmt")]
    defmt::info!("Initializing I2C touch controller...");
    let mut i2c = I2c::new(dp.I2C1, (gpiob.pb8, gpiob.pb9), 400.kHz(), &mut rcc);
    let ts_int = gpioc.pc1.into_pull_down_input();
    let mut touch = Ft6X06::new(&i2c, 0x38, ts_int).ok();
    if let Some(t) = touch.as_mut() {
        let _ = t.ts_calibration(&mut i2c, &mut delay);
        #[cfg(feature = "defmt")]
        defmt::info!("FT6X06 touch initialized");
    } else {
        #[cfg(feature = "defmt")]
        defmt::warn!("FT6X06 touch not detected");
    }

    let mut idx = 0usize;
    let mut showing_a = true;
    let mut touch_start_x: Option<i32> = None;
    let mut touch_last_x: Option<i32> = None;
    let buf_a_addr = base_ptr as u32;
    let buf_b_addr = unsafe { base_ptr.add(FB_SIZE) } as u32;

    #[cfg(feature = "defmt")]
    defmt::info!(
        "Image slider ready — swipe left/right to change pattern ({} patterns)",
        patterns.len()
    );

    loop {
        let num = touch
            .as_mut()
            .and_then(|t| t.detect_touch(&mut i2c).ok())
            .unwrap_or(0);

        if num > 0 {
            if let Some(touch) = touch.as_mut() {
                if let Ok(point) = touch.get_touch(&mut i2c, 1) {
                    let x = point.x as i32;
                    touch_last_x = Some(x);
                    if touch_start_x.is_none() {
                        touch_start_x = Some(x);
                    }
                }
            }
        } else if let (Some(start_x), Some(end_x)) = (touch_start_x.take(), touch_last_x.take()) {
            let delta = end_x - start_x;
            let new_idx;
            if delta >= SWIPE_THRESHOLD {
                new_idx = (idx + patterns.len() - 1) % patterns.len();
            } else if delta <= -SWIPE_THRESHOLD {
                new_idx = (idx + 1) % patterns.len();
            } else {
                continue;
            }

            idx = new_idx;
            if showing_a {
                let buf_b_slice =
                    unsafe { core::slice::from_raw_parts_mut(buf_b_addr as *mut u16, FB_SIZE) };
                patterns[idx].fill(buf_b_slice, WIDTH, HEIGHT);
                display_ctrl.set_layer_buffer_address(Layer::L1, buf_b_addr);
            } else {
                let buf_a_slice =
                    unsafe { core::slice::from_raw_parts_mut(buf_a_addr as *mut u16, FB_SIZE) };
                patterns[idx].fill(buf_a_slice, WIDTH, HEIGHT);
                display_ctrl.set_layer_buffer_address(Layer::L1, buf_a_addr);
            }
            showing_a = !showing_a;
            #[cfg(feature = "defmt")]
            defmt::info!("Switched to pattern {}", idx);
        }

        delay.delay_ms(20u32);
    }
}
