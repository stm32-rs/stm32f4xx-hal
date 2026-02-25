//! STM32F469I-DISCO LCD framebuffer example using embedded-graphics
//!
//! Demonstrates the `LtdcFramebuffer` + `DrawTarget` abstraction on the
//! DSI-connected 480x800 panel. Draws colour bars and rectangles using
//! `embedded-graphics` primitives backed by SDRAM.
//!
//! Build (F469I-DISCO):
//!
//! ```bash
//! cargo build --release --example f469disco-framebuffer --features="stm32f469,stm32-fmc,framebuffer,defmt"
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use core::slice;

use stm32f4xx_hal::{self as hal, rcc::Config};

use hal::{
    dsi::{
        ColorCoding, DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiInterrupts,
        DsiMode, DsiPhyTimers, DsiPllConfig, DsiVideoMode, LaneCount,
    },
    fmc::FmcExt,
    gpio::alt::fmc as fmc_alt,
    ltdc::{DisplayConfig, DisplayController, Layer, LtdcFramebuffer, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use stm32_fmc::devices::is42s32400f_6;

const WIDTH: u16 = 480;
const HEIGHT: u16 = 800;

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

    // 1. Clocks
    let hse_freq = 8.MHz();
    let mut rcc = dp
        .RCC
        .freeze(Config::hse(hse_freq).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    // 2. GPIO
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

    // 3. SDRAM
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

    let mut sdram = dp
        .FMC
        .sdram(fmc_pins, is42s32400f_6::Is42s32400f6 {}, &rcc.clocks);

    let fb_size = WIDTH as usize * HEIGHT as usize;
    let sdram_ptr: *mut u16 = sdram.init(&mut delay) as *mut u16;
    assert!(!sdram_ptr.is_null());
    let fb = unsafe { slice::from_raw_parts_mut(sdram_ptr, fb_size) };
    fb.fill(0); // clear to black

    // 4. DSI
    let dsi_pll_config = unsafe { DsiPllConfig::manual(125, 2, 0, 4) };
    let dsi_config = DsiConfig {
        mode: DsiMode::Video {
            mode: DsiVideoMode::Burst,
        },
        lane_count: LaneCount::DoubleLane,
        channel: DsiChannel::Ch0,
        hse_freq,
        ltdc_freq: 27_429.kHz(),
        interrupts: DsiInterrupts::None,
        color_coding_host: ColorCoding::SixteenBitsConfig1,
        color_coding_wrapper: ColorCoding::SixteenBitsConfig1,
        lp_size: 64,
        vlp_size: 64,
    };

    let mut dsi_host = DsiHost::init(dsi_pll_config, DISPLAY_CONFIG, dsi_config, dp.DSI, &mut rcc)
        .expect("DSI init");

    dsi_host.configure_phy_timers(DsiPhyTimers {
        dataline_hs2lp: 35,
        dataline_lp2hs: 35,
        clock_hs2lp: 35,
        clock_lp2hs: 35,
        dataline_max_read_time: 0,
        stop_wait_time: 10,
    });

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.start();

    // 5. LTDC
    let mut display_ctrl =
        DisplayController::<u16>::new_dsi(dp.LTDC, dp.DMA2D, PixelFormat::RGB565, DISPLAY_CONFIG);

    display_ctrl.config_layer(Layer::L1, fb, PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    // 6. Create LtdcFramebuffer
    let fb2 = unsafe { slice::from_raw_parts_mut(sdram_ptr, fb_size) };
    let mut framebuffer = LtdcFramebuffer::new(fb2, WIDTH, HEIGHT);

    // 7. Draw using embedded-graphics DrawTarget
    defmt::info!("Drawing colour bars via LtdcFramebuffer + DrawTarget");

    let bar_h = HEIGHT as u32 / 4;
    let colors = [Rgb565::RED, Rgb565::GREEN, Rgb565::BLUE, Rgb565::WHITE];

    for (i, &color) in colors.iter().enumerate() {
        Rectangle::new(
            Point::new(0, (i as u32 * bar_h) as i32),
            Size::new(WIDTH as u32, bar_h),
        )
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(&mut framebuffer)
        .unwrap();
    }

    // 8. Draw a rectangle using DrawTarget (embedded-graphics)
    Rectangle::new(Point::new(100, 300), Size::new(280, 200))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::YELLOW))
        .draw(&mut framebuffer)
        .unwrap();
    display_ctrl.reload();

    defmt::info!("Framebuffer demo ready.");
    loop {
        cortex_m::asm::wfi();
    }
}

