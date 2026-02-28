//! STM32F469I-DISCO paint example (port of ST LCD_Paint).
//!
//! Touch-driven drawing with color palette and brush. Uses RGB565 framebuffer
//! in SDRAM and FT6X06 touch controller over I2C.
//!
//! Supports both board revisions via runtime panel autodetection (NT35510 / OTM8009A).
//!
//! Build:
//! ```bash
//! cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt"
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use fugit::MillisDurationU32;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use panic_probe as _;

use stm32f4xx_hal::{self as hal, rcc::Config};

use hal::{
    fmc::FmcExt,
    gpio::alt::fmc as fmc_alt,
    i2c::I2c,
    ltdc::{Layer, LtdcFramebuffer, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    watchdog::IndependentWatchdog,
};

use embedded_graphics_core::{
    geometry::{Point, Size},
    pixelcolor::{IntoStorage, Rgb565, RgbColor},
    prelude::DrawTarget,
    primitives::Rectangle,
};

use ft6x06::Ft6X06;

#[path = "f469disco/board.rs"]
mod board;

use board::{FB_SIZE, HEIGHT, WIDTH};
use stm32_fmc::devices::is42s32400f_6;

const PALETTE_H: i32 = 48;
const BRUSH_R: i32 = 8;

/// Touch calibration offsets (adjust for your panel if needed)
/// The FT6X06 on F469I-DISCO may have a physical offset.
/// Positive values shift touch right/down, negative shifts left/up.
const TOUCH_X_OFFSET: i32 = 0;
const TOUCH_Y_OFFSET: i32 = 0;
const PALETTE: [Rgb565; 8] = [
    Rgb565::BLACK,
    Rgb565::RED,
    Rgb565::GREEN,
    Rgb565::BLUE,
    Rgb565::new(31, 63, 0),
    Rgb565::new(0, 31, 31),
    Rgb565::new(31, 0, 31),
    Rgb565::WHITE,
];

macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        ($(fmc_alt::$alt::from($pin.internal_pull_up(true))),*)
    };
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();
    let mut iwdg = IndependentWatchdog::new(dp.IWDG);
    iwdg.start(MillisDurationU32::from_ticks(2000));
    #[cfg(feature = "defmt")]
    defmt::info!("IWDG watchdog started (2s timeout)");

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

    let buffer: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr, FB_SIZE) };
    let mut fb = LtdcFramebuffer::new(buffer, WIDTH, HEIGHT);

    fb.clear(Rgb565::WHITE).ok();
    for (i, &c) in PALETTE.iter().enumerate() {
        let x = (i as i32) * (WIDTH as i32 / 8);
        let _ = fb.fill_solid(
            &Rectangle::new(
                Point::new(x, 0),
                Size::new((WIDTH / 8) as u32, PALETTE_H as u32),
            ),
            c,
        );
    }
    let buffer = fb.into_inner();

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
    display_ctrl.config_layer(Layer::L1, buffer, hal::ltdc::PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    #[cfg(feature = "defmt")]
    defmt::info!("Initializing I2C touch controller...");
    let mut i2c = I2c::new(dp.I2C1, (gpiob.pb8, gpiob.pb9), 400.kHz(), &mut rcc);
    // PC0 is used by FMC SDNWE; PC1 serves as a placeholder for the FT6X06
    // interrupt pin. Touch detection works via I2C polling regardless.
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

    let mut current_color = Rgb565::RED;
    #[cfg(feature = "defmt")]
    let mut garbage_count: u32 = 0;
    let mut loop_count: u32 = 0;
    #[cfg(feature = "defmt")]
    defmt::info!("Paint ready — touch to draw, tap palette bar to change color");

    loop {
        iwdg.feed();
        loop_count = loop_count.wrapping_add(1);

        let mut td_status = [0u8; 1];
        let ntouch = match touch.as_mut() {
            Some(_) => match i2c.write_read(0x38u8, &[0x02u8], &mut td_status) {
                Ok(()) => td_status[0] & 0x0F,
                Err(_) => {
                    delay.delay_ms(10u32);
                    continue;
                }
            },
            None => {
                delay.delay_ms(10u32);
                continue;
            }
        };

        if ntouch == 0 {
            delay.delay_ms(10u32);
            continue;
        }

        if ntouch > 2 {
            #[cfg(feature = "defmt")]
            {
                garbage_count += 1;
                defmt::warn!(
                    "ft6x06: invalid ntouch={}, skipping (total={})",
                    ntouch,
                    garbage_count
                );
            }
            delay.delay_ms(10u32);
            continue;
        }

        if let Some(touch) = touch.as_mut() {
            if let Ok(point) = touch.get_touch(&mut i2c, 1) {
                let x = (point.x as i32 + TOUCH_X_OFFSET).clamp(0, WIDTH as i32 - 1);
                let y = (point.y as i32 + TOUCH_Y_OFFSET).clamp(0, HEIGHT as i32 - 1);

                #[cfg(feature = "defmt")]
                defmt::info!("touch: raw=({}, {}) adj=({}, {})", point.x, point.y, x, y);
                if y < PALETTE_H {
                    let idx = (x * 8 / WIDTH as i32).clamp(0, 7);
                    current_color = PALETTE[idx as usize];
                } else if let Some(buf) = display_ctrl.layer_buffer_mut(Layer::L1) {
                    let c = current_color.into_storage();
                    let w = WIDTH as i32;
                    let h = HEIGHT as i32;
                    let x0 = (x - BRUSH_R).max(0);
                    let y0 = (y - BRUSH_R).max(0);
                    let x1 = (x + BRUSH_R).min(w);
                    let y1 = (y + BRUSH_R).min(h);
                    for py in y0..y1 {
                        for px in x0..x1 {
                            buf[py as usize * (WIDTH as usize) + px as usize] = c;
                        }
                    }
                }
            }
        }

        // Periodic DSI error register diagnostic (every ~100 iterations)
        #[cfg(feature = "dsihost")]
        if loop_count % 100 == 0 {
            // SAFETY: DSI peripheral is not borrowed here; we only read status registers.
            let dsi = unsafe { &*stm32f4xx_hal::pac::DSI::ptr() };
            let isr0 = dsi.isr0().read().bits();
            let isr1 = dsi.isr1().read().bits();
            if isr0 != 0 || isr1 != 0 {
                #[cfg(feature = "defmt")]
                defmt::warn!("DSI errors: ISR0={:#010x} ISR1={:#010x}", isr0, isr1);
                // ISR flags are read-only hardware status; they auto-clear when the
                // error condition resolves. FIR registers are for forcing (injecting)
                // flags, NOT clearing — do not write to them.
            }
        }

        delay.delay_ms(10u32);
    }
}
