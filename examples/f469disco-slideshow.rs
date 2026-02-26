//! STM32F469I-DISCO slideshow example (port of ST LCD_PicturesFromSDCard).
//!
//! Crossfade between patterns using dual LTDC layers and layer transparency.
//! L1 is always at full opacity; only L2's alpha is animated.
//! No SD card: uses embedded pattern generators.
//!
//! Supports both board revisions via runtime panel autodetection (NT35510 / OTM8009A).
//!
//! Build:
//! ```bash
//! cargo build --release --example f469disco-slideshow --features="stm32f469,stm32-fmc,framebuffer,defmt"
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
use images::SlidePattern;
use stm32_fmc::devices::is42s32400f_6;

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

    let patterns = SlidePattern::ALL;
    SlidePattern::ColorBars.fill(layer1_buf, WIDTH, HEIGHT);
    SlidePattern::SolidRed.fill(layer2_buf, WIDTH, HEIGHT);

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
    display_ctrl.set_layer_transparency(Layer::L1, 255);
    display_ctrl.set_layer_transparency(Layer::L2, 0);
    display_ctrl.reload();

    defmt::info!("Slideshow running — crossfading between {} patterns", patterns.len());

    // Crossfade: L1 stays at full opacity. Only L2's alpha is animated.
    // L2 alpha=0 → L1 visible. L2 alpha=255 → L2 visible, L1 hidden.
    let mut idx = 0usize;
    const FADE_STEPS: u32 = 32;
    const STEP_MS: u32 = 30;
    const HOLD_MS: u32 = 2000;

    loop {
        for step in 0..=FADE_STEPS {
            let alpha = (step * 255 / FADE_STEPS) as u8;
            display_ctrl.set_layer_transparency(Layer::L2, alpha);
            delay.delay_ms(STEP_MS);
        }
        delay.delay_ms(HOLD_MS);

        idx = (idx + 1) % patterns.len();
        if let Some(buf) = display_ctrl.layer_buffer_mut(Layer::L1) {
            patterns[idx].fill(buf, WIDTH, HEIGHT);
        }

        for step in 0..=FADE_STEPS {
            let alpha = 255 - (step * 255 / FADE_STEPS) as u8;
            display_ctrl.set_layer_transparency(Layer::L2, alpha);
            delay.delay_ms(STEP_MS);
        }
        delay.delay_ms(HOLD_MS);

        idx = (idx + 1) % patterns.len();
        if let Some(buf) = display_ctrl.layer_buffer_mut(Layer::L2) {
            patterns[idx].fill(buf, WIDTH, HEIGHT);
        }
    }
}
