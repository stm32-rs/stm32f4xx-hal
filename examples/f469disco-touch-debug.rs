//! Touch coordinate debugger for F469I-DISCO
//!
//! Run this and tap the corners/edges of the screen to calibrate touch offset.
//! The logs will show raw coordinates that you can use to calculate offset.
//!
//! Build:
//! cargo build --release --example f469disco-touch-debug --features="stm32f469,stm32-fmc,defmt"

#![no_main]
#![no_std]

use cortex_m_rt::entry;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
use panic_probe as _;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;

use stm32f4xx_hal::{self as hal, rcc::Config};

use hal::{
    fmc::FmcExt,
    gpio::alt::fmc as fmc_alt,
    i2c::I2c,
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use ft6x06::Ft6X06;
use stm32_fmc::devices::is42s32400f_6;

const WIDTH: u16 = 480;
const HEIGHT: u16 = 800;
const FB_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

const DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
    active_width: WIDTH,
    active_height: HEIGHT,
    h_back_porch: 34,
    h_front_porch: 34,
    v_back_porch: 15,
    v_front_porch: 16,
    h_sync: 2,
    v_sync: 1,
    frame_rate: 60,
    h_sync_pol: true,
    v_sync_pol: true,
    no_data_enable_pol: false,
    pixel_clock_pol: true,
};

macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        ($(fmc_alt::$alt::from($pin.internal_pull_up(true))),*)
    };
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hse(8.MHz()).pclk2(32.MHz()).sysclk(180.MHz()));
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
    let mut sdram = dp.FMC.sdram(fmc_pins, is42s32400f_6::Is42s32400f6 {}, &rcc.clocks);
    let base_ptr = sdram.init(&mut delay) as *mut u16;
    assert!(!base_ptr.is_null());

    let buffer: &'static mut [u16] =
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr, FB_SIZE) };
    
    // Clear to white
    for pixel in buffer.iter_mut() {
        *pixel = 0xFFFF;
    }

    // Draw grid: red lines every 100px, blue corners
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let idx = y * WIDTH as usize + x;
            if x % 100 == 0 || y % 100 == 0 {
                buffer[idx] = 0xF800; // Red
            }
            // Mark corners in blue
            if (x < 20 && y < 20) 
                || (x > WIDTH as usize - 20 && y < 20)
                || (x < 20 && y > HEIGHT as usize - 20) 
                || (x > WIDTH as usize - 20 && y > HEIGHT as usize - 20) 
            {
                buffer[idx] = 0x001F; // Blue
            }
        }
    }

    #[cfg(feature = "defmt")]
    defmt::info!("Initializing display...");
    let mut display_ctrl = DisplayController::<u16>::new(
        dp.LTDC,
        dp.DMA2D,
        None,
        PixelFormat::RGB565,
        DISPLAY_CONFIG,
        Some(8.MHz()),
    );
    display_ctrl.config_layer(Layer::L1, buffer, hal::ltdc::PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    #[cfg(feature = "defmt")]
    defmt::info!("Initializing I2C touch controller...");
    let mut i2c = I2c::new(dp.I2C1, (gpiob.pb8, gpiob.pb9), 400.kHz(), &mut rcc);
    let ts_int = gpioc.pc1.into_pull_down_input();
    
    let mut touch = match Ft6X06::new(&i2c, 0x38, ts_int) {
        Ok(t) => {
            #[cfg(feature = "defmt")]
            defmt::info!("FT6X06 touch initialized");
            t
        }
        Err(_) => {
            #[cfg(feature = "defmt")]
            defmt::error!("FT6X06 not detected!");
            loop { cortex_m::asm::wfi(); }
        }
    };

    let _ = touch.ts_calibration(&mut i2c, &mut delay);

    #[cfg(feature = "defmt")]
    defmt::info!("=== TOUCH CALIBRATION ===");
    #[cfg(feature = "defmt")]
    defmt::info!("Tap: TOP-LEFT, TOP-RIGHT, BOTTOM-RIGHT, BOTTOM-LEFT");
    #[cfg(feature = "defmt")]
    defmt::info!("Expected: (0,0), (479,0), (479,799), (0,799)");

    loop {
        match touch.detect_touch(&mut i2c) {
            Ok(0) => {
                delay.delay_ms(10u32);
            }
            Ok(_n) => {
                if let Ok(_point) = touch.get_touch(&mut i2c, 1) {
                    #[cfg(feature = "defmt")]
                    defmt::info!("TOUCH: raw=({},{}) ntouch={}", _point.x, _point.y, _n);
                }
            }
            Err(_) => {}
        }
        delay.delay_ms(50u32);
    }
}
