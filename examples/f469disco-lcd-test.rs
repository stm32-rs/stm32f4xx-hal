//! This example initializes the STM32F469I-DISCO LCD and displays a test pattern
//!
//! This example supports both STM32F469I-DISCO board revisions:
//! - B08 revision (NT35510 LCD controller) - auto-detected and preferred
//! - B07 and earlier (OTM8009A LCD controller) - fallback
//!   (see STM32F469I-DISCO user manual UM1932 for revision split)
//!
//! ## Display Controller Detection
//!
//! This example automatically detects the LCD controller at runtime by
//! reading the RDID1 register (0xDA) via DSI:
//!
//! | Controller | Board Revision | RDID1 Value |
//! |------------|----------------|-------------|
//! | NT35510    | B08            | 0x00        |
//! | OTM8009A   | B07, B01       | 0x40        |
//!
//! ### Detection Approach
//!
//! 1. Initialize DSI with conservative timing compatible with both controllers
//! 2. Read RDID1 register to identify controller
//! 3. Configure LTDC with optimal timing for detected controller
//! 4. Initialize the appropriate display driver
//!
//! ### Known Limitations
//!
//! - DSI must be initialized before probing, requiring initial timing parameters
//! - We use NT35510-compatible timing for the probe phase
//! - If detection fails, the example will panic with an error message
//!
//! ### Hardware Compatibility
//!
//! Tested on:
//! - STM32F469I-DISCO B08 (NT35510)
//! - STM32F469I-DISCO B07 (OTM8009A)
//!
//! ### Notes from controller research
//!
//! - ST's standalone OTM8009A driver reads ID1 (`0xDA`) and expects `0x40`:
//!   <https://github.com/STMicroelectronics/stm32-otm8009a>
//! - ST's standalone NT35510 driver reads ID2 (`0xDB`) and expects `0x80`:
//!   <https://github.com/STMicroelectronics/stm32-nt35510>
//! - This example intentionally probes NT35510 using RDID1 (`0xDA == 0x00`) because it
//!   has proven reliable on F469I-DISCO B08 boards in this repository.
//!
//! Run as:
//! cargo run --release --example f469disco-lcd-test --features="stm32f469,defmt"

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

#[path = "f469disco/nt35510.rs"]
mod nt35510;

use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

use stm32f4xx_hal::{self as hal, rcc::Config};

use crate::hal::{
    dsi::{
        ColorCoding, DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiInterrupts,
        DsiMode, DsiPhyTimers, DsiPllConfig, DsiVideoMode, LaneCount,
    },
    i2c::I2c,
    ltdc::{DisplayConfig, DisplayController, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use ft6x06::Ft6X06;
use otm8009a::{Otm8009A, Otm8009AConfig};

const TOUCH_ERROR_LOG_THROTTLE: u8 = 16;
const TOUCH_MAX_RETRIES: u8 = 3;
/// After this many consecutive detect_touch failures, stop polling with a one-time warning.
const TOUCH_DISABLE_THRESHOLD: u16 = 100;

// Display configurations for different controllers
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LcdController {
    Nt35510,
    Otm8009a,
}

impl LcdController {
    fn display_config(self) -> DisplayConfig {
        match self {
            Self::Nt35510 => NT35510_DISPLAY_CONFIG,
            Self::Otm8009a => OTM8009A_DISPLAY_CONFIG,
        }
    }
}

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 800;

// NT35510 timing (B08 / Rev C):
// - Vertical values follow ST's BSP + specter-diy validated hardware settings
//   for NT35510 panels (VSYNC=120, VBP=150, VFP=150).
pub const NT35510_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
    active_width: WIDTH as _,
    active_height: HEIGHT as _,
    h_back_porch: 34,
    h_front_porch: 34,
    v_back_porch: 150,
    v_front_porch: 150,
    h_sync: 2,
    v_sync: 120,
    frame_rate: 60,
    h_sync_pol: true,
    v_sync_pol: true,
    no_data_enable_pol: false,
    pixel_clock_pol: true,
};

// OTM8009A timing (B07 and earlier revisions)
// Values from STMicroelectronics/stm32-otm8009a otm8009a.h
// Tested on KoD KM-040TMP-02-0621 WVGA display
pub const OTM8009A_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
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

// DSI probe timing: Use NT35510 timing for initial DSI setup.
// This works for both NT35510 and OTM8009A during the detection phase.
// After detection, LTDC is configured with optimal timing for the
// detected controller. The probe timing is conservative enough to
// work with both display types for reading the RDID1 register.
const DSI_PROBE_DISPLAY_CONFIG: DisplayConfig = NT35510_DISPLAY_CONFIG;

// DSI LP sizes:
// - NT35510 needs larger LP packet sizes (64)
// - OTM8009A may prefer 4 in single-controller builds; 64 is used here so one
//   runtime-detecting binary can operate on B08+ and B07 boards.
const DSI_LP_SIZE: u8 = 64;
const DSI_VLP_SIZE: u8 = 64;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let hse_freq = 8.MHz();
    let mut rcc = dp
        .RCC
        .freeze(Config::hse(hse_freq).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioh = dp.GPIOH.split(&mut rcc);

    // Reset display (PH7/XRES) in push-pull mode.
    // This is required by NT35510-based B08 boards and remains compatible with
    // OTM8009A boards in our testing.
    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    let ltdc_freq = 27_429.kHz();

    // Initialize DSI Host with probe-compatible settings.
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
        lp_size: DSI_LP_SIZE,
        vlp_size: DSI_VLP_SIZE,
    };

    defmt::info!("Initializing DSI {:?} {:?}", dsi_config, dsi_pll_config);
    let mut dsi_host = match DsiHost::init(
        dsi_pll_config,
        DSI_PROBE_DISPLAY_CONFIG,
        dsi_config,
        dp.DSI,
        &mut rcc,
    ) {
        Ok(host) => host,
        Err(e) => defmt::panic!("DSI host initialization failed: {:?}", e),
    };

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

    let controller = detect_lcd_controller(&mut dsi_host, &mut delay);
    defmt::info!("Detected LCD controller: {:?}", controller);

    defmt::info!("Initializing LTDC for detected controller");
    let _display = DisplayController::<u32>::new_dsi(
        dp.LTDC,
        dp.DMA2D,
        PixelFormat::ARGB8888,
        controller.display_config(),
    );

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.force_rx_low_power(true);
    dsi_host.enable_color_test(); // Must enable before display initialized

    // Initialize the detected LCD controller
    match controller {
        LcdController::Nt35510 => {
            defmt::info!("Initializing NT35510 (B08 revision)");
            let mut nt35510 = nt35510::Nt35510::new();
            if let Err(e) = nt35510.init(&mut dsi_host, &mut delay) {
                defmt::panic!("NT35510 init failed: {:?}", e);
            }
        }
        LcdController::Otm8009a => {
            defmt::info!("Initializing OTM8009A (B07 and earlier revisions)");
            let otm8009a_config = Otm8009AConfig {
                frame_rate: otm8009a::FrameRate::_60Hz,
                mode: otm8009a::Mode::Portrait,
                color_map: otm8009a::ColorMap::Rgb,
                cols: WIDTH as u16,
                rows: HEIGHT as u16,
            };
            let mut otm8009a = Otm8009A::new();
            if let Err(e) = otm8009a.init(&mut dsi_host, otm8009a_config, &mut delay) {
                defmt::panic!("OTM8009A init failed: {:?}", e);
            }
        }
    }

    // ========== INITIALIZE TOUCHSCREEN ==========
    defmt::info!("Initializing touchscreen");
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    let scl = gpiob.pb8;
    let sda = gpiob.pb9;
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &mut rcc);

    let ts_int = gpioc.pc0.into_pull_down_input();
    let mut touch = match Ft6X06::new(&i2c, 0x38, ts_int) {
        Ok(touch) => Some(touch),
        Err(_) => {
            defmt::warn!("Touch controller unavailable");
            None
        }
    };

    // Run internal calibration of touchscreen (following display-touch.rs pattern)
    if let Some(touch) = touch.as_mut() {
        let tsc = touch.ts_calibration(&mut i2c, &mut delay);
        match tsc {
            Err(_) => defmt::warn!("Error from ts_calibration"),
            Ok(u) => defmt::info!("ts_calibration returned {}", u),
        }
    } else {
        defmt::warn!("Touch initialization failed; running display pattern without touch input");
    }

    defmt::info!("Outputting Color/BER test patterns. Touch to toggle test mode.");

    let mut current_pattern_is_color = true;
    let mut pattern_timer = 0u32;
    let pattern_switch_delay = 500;
    let mut touch_error_throttle = 0u8;
    let mut touch_consecutive_failures: u16 = 0;

    dsi_host.enable_color_test();

    loop {
        if let Some(touch) = touch.as_mut() {
            if touch_consecutive_failures < TOUCH_DISABLE_THRESHOLD {
                let mut detected_touches = None;
                for attempt in 0..TOUCH_MAX_RETRIES {
                    match touch.detect_touch(&mut i2c) {
                        Ok(num) => {
                            detected_touches = Some(num);
                            break;
                        }
                        Err(_) => {
                            touch_error_throttle = touch_error_throttle.wrapping_add(1);
                            if touch_error_throttle % TOUCH_ERROR_LOG_THROTTLE == 0 {
                                defmt::warn!(
                                    "detect_touch read error (attempt {})",
                                    attempt + 1
                                );
                            }
                            delay.delay_us(500u32);
                        }
                    }
                }

                let Some(num) = detected_touches else {
                    touch_consecutive_failures =
                        touch_consecutive_failures.saturating_add(1);
                    if touch_consecutive_failures == TOUCH_DISABLE_THRESHOLD {
                        defmt::warn!(
                            "Touch controller unresponsive after {} consecutive failures; disabling polling",
                            TOUCH_DISABLE_THRESHOLD
                        );
                    } else {
                        touch_error_throttle = touch_error_throttle.wrapping_add(1);
                        if touch_error_throttle % TOUCH_ERROR_LOG_THROTTLE == 0 {
                            defmt::warn!(
                                "detect_touch timed out after {} attempts",
                                TOUCH_MAX_RETRIES
                            );
                        }
                    }
                    pattern_loop_housekeeping(
                        &mut dsi_host,
                        &mut current_pattern_is_color,
                        &mut pattern_timer,
                        pattern_switch_delay,
                    );
                    delay.delay_ms(10u32);
                    continue;
                };

                // Reset consecutive failure counter on any successful detect
                touch_consecutive_failures = 0;

                if num > 0 {
                    defmt::info!("Number of touches: {}", num);

                    let mut touch_point = None;
                    for attempt in 0..TOUCH_MAX_RETRIES {
                        match touch.get_touch(&mut i2c, 1) {
                            Ok(point) => {
                                touch_point = Some(point);
                                break;
                            }
                            Err(_) => {
                                touch_error_throttle =
                                    touch_error_throttle.wrapping_add(1);
                                if touch_error_throttle % TOUCH_ERROR_LOG_THROTTLE == 0 {
                                    defmt::warn!(
                                        "get_touch read error (attempt {})",
                                        attempt + 1
                                    );
                                }
                                delay.delay_us(500u32);
                            }
                        }
                    }

                    match touch_point {
                        Some(point) => {
                            defmt::info!(
                                "Touch at x={}, y={} - weight: {}",
                                point.x,
                                point.y,
                                point.weight
                            );
                            current_pattern_is_color = !current_pattern_is_color;
                            if current_pattern_is_color {
                                dsi_host.enable_color_test();
                            } else {
                                dsi_host.enable_ber_test();
                            }
                        }
                        None => {
                            touch_error_throttle = touch_error_throttle.wrapping_add(1);
                            if touch_error_throttle % TOUCH_ERROR_LOG_THROTTLE == 0 {
                                defmt::warn!(
                                    "get_touch timed out after {} attempts",
                                    TOUCH_MAX_RETRIES
                                );
                            }
                        }
                    }
                }
            }
        }

        pattern_loop_housekeeping(
            &mut dsi_host,
            &mut current_pattern_is_color,
            &mut pattern_timer,
            pattern_switch_delay,
        );

        delay.delay_ms(10u32);
    }
}

fn pattern_loop_housekeeping(
    dsi_host: &mut DsiHost,
    current_pattern_is_color: &mut bool,
    pattern_timer: &mut u32,
    pattern_switch_delay: u32,
) {
    *pattern_timer += 1;
    if *pattern_timer >= pattern_switch_delay {
        *pattern_timer = 0;
        *current_pattern_is_color = !*current_pattern_is_color;

        if *current_pattern_is_color {
            dsi_host.enable_color_test();
        } else {
            dsi_host.enable_ber_test();
        }
    }
}

fn detect_lcd_controller(
    dsi_host: &mut DsiHost,
    delay: &mut impl embedded_hal_02::blocking::delay::DelayUs<u32>,
) -> LcdController {
    defmt::info!("Auto-detecting LCD controller...");

    const PROBE_RETRIES: u8 = 3;
    let mut nt35510 = nt35510::Nt35510::new();
    for attempt in 1..=PROBE_RETRIES {
        match nt35510.probe(dsi_host, delay) {
            Ok(_) => {
                defmt::info!("NT35510 (B08) detected successfully on attempt {}", attempt);
                return LcdController::Nt35510;
            }
            Err(nt35510::Nt35510Error::DsiRead) => {
                defmt::warn!("NT35510 probe attempt {} failed: DSI read error", attempt);
            }
            Err(nt35510::Nt35510Error::DsiWrite) => {
                defmt::warn!("NT35510 probe attempt {} failed: DSI write error", attempt);
            }
            Err(nt35510::Nt35510Error::ProbeMismatch(id)) => {
                defmt::info!(
                    "NT35510 probe attempt {} mismatch: RDID1=0x{:02x}",
                    attempt,
                    id
                );
            }
        }

        delay.delay_us(5_000u32);
    }

    defmt::info!("Display detection failed after {} attempts", PROBE_RETRIES);
    defmt::info!("This could indicate:");
    defmt::info!("  1. OTM8009A display (B07 and earlier boards)");
    defmt::info!("  2. Display not connected or powered");
    defmt::info!("  3. DSI initialization issue");
    defmt::info!("Falling back to OTM8009A (B07 and earlier revisions)");
    LcdController::Otm8009a
}
