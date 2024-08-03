//! This example initializes the STM32F469I-DISCO LCD and displays a test pattern
//! Run as:
//! cargo run --release --example f469disco-lcd-test --features="stm32f469,defmt"

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

use stm32f4xx_hal as hal;

use crate::hal::{
    dsi::{
        ColorCoding, DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiInterrupts,
        DsiMode, DsiPhyTimers, DsiPllConfig, DsiVideoMode, LaneCount,
    },
    ltdc::{DisplayConfig, DisplayController, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use otm8009a::{Otm8009A, Otm8009AConfig};

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 800;

pub const DISPLAY_CONFIGURATION: DisplayConfig = DisplayConfig {
    active_width: WIDTH as _,
    active_height: HEIGHT as _,
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

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let hse_freq = 8.MHz();
    let clocks = rcc
        .cfgr
        .use_hse(hse_freq)
        .pclk2(32.MHz())
        .sysclk(180.MHz())
        .freeze();
    let mut delay = cp.SYST.delay(&clocks);

    let gpioh = dp.GPIOH.split();

    // Reset display
    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    // Initialize LTDC, needed to provide pixel clock to DSI
    defmt::info!("Initializing LTDC");
    let ltdc_freq = 27_429.kHz();
    let _display = DisplayController::<u32>::new(
        dp.LTDC,
        dp.DMA2D,
        None,
        PixelFormat::ARGB8888,
        DISPLAY_CONFIGURATION,
        Some(hse_freq),
    );

    // Initialize DSI Host
    // VCO = (8MHz HSE / 2 IDF) * 2 * 125 = 1000MHz
    // 1000MHz VCO / (2 * 1 ODF * 8) = 62.5MHz
    let dsi_pll_config = unsafe {
        DsiPllConfig::manual(125, 2, 0 /*div1*/, 4)
    };

    let dsi_config = DsiConfig {
        mode: DsiMode::Video {
            mode: DsiVideoMode::Burst,
        },
        lane_count: LaneCount::DoubleLane,
        channel: DsiChannel::Ch0,
        hse_freq,
        ltdc_freq,
        interrupts: DsiInterrupts::None,
        color_coding_host: ColorCoding::TwentyFourBits,
        color_coding_wrapper: ColorCoding::TwentyFourBits,
        lp_size: 4,
        vlp_size: 4,
    };

    defmt::info!("Initializing DSI {:?} {:?}", dsi_config, dsi_pll_config);
    let mut dsi_host = DsiHost::init(
        dsi_pll_config,
        DISPLAY_CONFIGURATION,
        dsi_config,
        dp.DSI,
        &clocks,
    )
    .unwrap();

    dsi_host.configure_phy_timers(DsiPhyTimers {
        dataline_hs2lp: 35,
        dataline_lp2hs: 35,
        clock_hs2lp: 35,
        clock_lp2hs: 35,
        dataline_max_read_time: 0,
        stop_wait_time: 10,
    });

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInLowPower);
    dsi_host.start();
    dsi_host.enable_bus_turn_around(); // Must be before read attempts
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.force_rx_low_power(true);
    dsi_host.enable_color_test(); // Must enable before display initialized

    defmt::info!("Initializing OTM8009A");
    let otm8009a_config = Otm8009AConfig {
        frame_rate: otm8009a::FrameRate::_60Hz,
        mode: otm8009a::Mode::Portrait,
        color_map: otm8009a::ColorMap::Rgb,
        cols: WIDTH as u16,
        rows: HEIGHT as u16,
    };
    let mut otm8009a = Otm8009A::new();
    otm8009a
        .init(&mut dsi_host, otm8009a_config, &mut delay)
        .unwrap();

    defmt::info!("Outputting Color/BER test patterns...");
    let delay_ms = 5000u32;
    loop {
        dsi_host.enable_color_test();
        delay.delay_ms(delay_ms);
        dsi_host.enable_ber_test();
        delay.delay_ms(delay_ms);
    }
}
