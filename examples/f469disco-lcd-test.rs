//! This example initializes the STM32F469I-DISCO LCD and displays a test pattern
//!
//! This example supports both STM32F469I-DISCO board revisions:
//! - B08 revision (NT35510 LCD controller) - auto-detected and preferred
//! - B07 and earlier (OTM8009A LCD controller) - fallback
//!
//! Runtime auto-detection is used by default.
//!
//! ## Build
//! ```bash
//! cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
//! ```
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

const FT6X06_I2C_ADDR_NEW: u8 = 0x38;
const FT6X06_I2C_ADDR_LEGACY: u8 = 0x2A;
// Timeout to force release when FT6x06 doesn't report release (~300ms at 10ms poll)
const TOUCH_TIMEOUT_TICKS: u8 = 30;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardHint {
    NewRevisionLikely,
    LegacyRevisionLikely,
    Unknown,
}

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

// NT35510 timing (B08 revision)
pub const NT35510_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
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

const DSI_PROBE_DISPLAY_CONFIG: DisplayConfig = NT35510_DISPLAY_CONFIG;

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
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);  // For touch interrupt (PC0)
    let _gpiod = dp.GPIOD.split(&mut rcc);  // For future LD2 (PD4), LD3 (PD5)
    let gpiog = dp.GPIOG.split(&mut rcc);  // For LD1 (PG6)
    let mut led = gpiog.pg6.into_push_pull_output();
    led.set_low();  // Start off

let scl = gpiob.pb8;
    let sda = gpiob.pb9;
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &mut rcc);

    let mut probe_touch_addr = |addr| {
        let mut id = [0u8; 1];
        i2c.read(addr, &mut id).is_ok()
            || i2c.write_read(addr, &[0xA8], &mut id).is_ok()
            || i2c.write_read(addr, &[0x00], &mut id).is_ok()
    };

    let board_hint = if probe_touch_addr(FT6X06_I2C_ADDR_NEW) {
        BoardHint::NewRevisionLikely
    } else if probe_touch_addr(FT6X06_I2C_ADDR_LEGACY) {
        BoardHint::LegacyRevisionLikely
    } else {
        BoardHint::Unknown
    };
    defmt::info!("Board hint from touch probe: {:?}", board_hint);

    // Reset display
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
        lp_size: 64,  // Increased for NT35510 compatibility; also works for OTM8009A
        vlp_size: 64, // Increased for NT35510 compatibility; also works for OTM8009A
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
    delay.delay_ms(20u32); // Allow panel link to settle after DSI start before probing

    let controller = detect_lcd_controller(&mut dsi_host, &mut delay, board_hint);
    defmt::info!("Detected LCD controller: {:?}", controller);

    defmt::info!("Initializing LTDC for detected controller");
    let _display = DisplayController::<u32>::new_dsi(
        dp.LTDC,
        dp.DMA2D,
        PixelFormat::ARGB8888,
        controller.display_config(),
    );

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInLowPower);
    dsi_host.force_rx_low_power(true);

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

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.enable_color_test();

    // ========== INITIALIZE TOUCHSCREEN ==========
    defmt::info!("Initializing touchscreen");
    let ts_int = gpioc.pc0.into_pull_down_input();
    let touch_addr = match board_hint {
        BoardHint::LegacyRevisionLikely => FT6X06_I2C_ADDR_LEGACY,
        BoardHint::NewRevisionLikely | BoardHint::Unknown => FT6X06_I2C_ADDR_NEW,
    };
    let mut touch = match Ft6X06::new(&i2c, touch_addr, ts_int) {
        Ok(touch) => Some(touch),
        Err(_) => {
            defmt::warn!("Touch controller unavailable at 0x{:02x}", touch_addr);
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
    let mut prev_num: u8 = 0;
    let mut touch_hold_ticks: u8 = 0;  // Timeout counter for forced release
    let mut total_toggles = 0u32;
    let mut touch_err_count = 0u32;

    dsi_host.enable_color_test();

loop {
        if let Some(touch) = touch.as_mut() {
            // Detect number of touches
            let num = match touch.detect_touch(&mut i2c) {
                Ok(n) => n,
                Err(_) => {
                    touch_err_count += 1;
                    if touch_err_count % 64 == 1 {
                        defmt::warn!("detect_touch error (count={})", touch_err_count);
                    }
                    prev_num = 0;
                    touch_hold_ticks = 0;
                    delay.delay_ms(20u32);
                    continue;
                }
            };

            // Always read touch data when touching to clear FT6x06 internal state
            if num > 0 {
                match touch.get_touch(&mut i2c, 1) {
Ok(point) => {
                        // Enhanced touch logging with quadrant info
                        let quadrant = match (point.x < 240, point.y < 400) {
                            (true, true) => "top-left",
                            (false, true) => "top-right",
                            (true, false) => "bottom-left",
                            (false, false) => "bottom-right",
                        };
                        defmt::info!("Touch: x={}, y={}, w={}, q={}", point.x, point.y, point.weight, quadrant);

                        // LED on while touching
                        led.set_high();
                        // Toggle on rising edge: was not touching, now touching
                        if prev_num == 0 {
                            touch_hold_ticks = 0;  // Reset on new touch
                            current_pattern_is_color = !current_pattern_is_color;
                            total_toggles += 1;
                            if current_pattern_is_color {
                                dsi_host.enable_color_test();
                            } else {
                                dsi_host.enable_ber_test();
                            }
                            defmt::info!("Toggle #{} -> {} at ({},{})",
                                total_toggles,
                                if current_pattern_is_color { "Color" } else { "BER" },
                                point.x, point.y
                            );
                        }
                    }
                    Err(_) => {
                        touch_err_count += 1;
                        if touch_err_count % 64 == 1 {
                            defmt::warn!("get_touch error (count={})", touch_err_count);
                        }
                    }
                }
} else if prev_num > 0 {
                defmt::info!("Touch released (held {}ms)", touch_hold_ticks * 10);
                led.set_low();  // LED off on release
            }

// Timeout: force release if FT6x06 stuck reporting num>0
if num > 0 {
                touch_hold_ticks = touch_hold_ticks.saturating_add(1);
                if touch_hold_ticks >= TOUCH_TIMEOUT_TICKS {
                    defmt::debug!("Touch timeout, forcing release");
                    prev_num = 0;  // Force release state for next poll
                    touch_hold_ticks = 0;
                } else {
                    prev_num = num;
                }
            } else {
                touch_hold_ticks = 0;
                prev_num = num;
            }
        }

        delay.delay_ms(10u32);
    }
}

fn detect_lcd_controller(
    dsi_host: &mut DsiHost,
    delay: &mut impl embedded_hal_02::blocking::delay::DelayUs<u32>,
    board_hint: BoardHint,
) -> LcdController {
    defmt::info!(
        "Auto-detecting LCD controller with hint {:?}...",
        board_hint
    );

    const PROBE_RETRIES: u8 = 3;
    delay.delay_us(20_000u32);
    let mut nt35510 = nt35510::Nt35510::new();
    let mut mismatch_count = 0u8;
    let mut first_mismatch_id: Option<u8> = None;
    let mut consistent_mismatch = true;
    let mut read_error_count = 0u8;
    let mut write_error_count = 0u8;
    for attempt in 1..=PROBE_RETRIES {
        match nt35510.probe(dsi_host, delay) {
            Ok(_) => {
                defmt::info!("NT35510 (B08) detected successfully on attempt {}", attempt);
                return LcdController::Nt35510;
            }
            Err(nt35510::Nt35510Error::DsiRead) => {
                read_error_count = read_error_count.saturating_add(1);
                defmt::warn!("NT35510 probe attempt {} failed: DSI read error", attempt);
            }
            Err(nt35510::Nt35510Error::DsiWrite) => {
                write_error_count = write_error_count.saturating_add(1);
                defmt::warn!("NT35510 probe attempt {} failed: DSI write error", attempt);
            }
            Err(nt35510::Nt35510Error::ProbeMismatch(id)) => {
                mismatch_count = mismatch_count.saturating_add(1);
                match first_mismatch_id {
                    None => first_mismatch_id = Some(id),
                    Some(first) if first != id => consistent_mismatch = false,
                    Some(_) => {}
                }
                defmt::info!(
                    "NT35510 probe attempt {} mismatch: RDID2=0x{:02x}",
                    attempt,
                    id
                );
            }
        }

        delay.delay_us(5_000u32);
    }

    let fallback_to_otm = match board_hint {
        BoardHint::LegacyRevisionLikely => mismatch_count >= 1 && consistent_mismatch,
        BoardHint::NewRevisionLikely => mismatch_count >= PROBE_RETRIES && consistent_mismatch,
        BoardHint::Unknown => mismatch_count >= 2 && consistent_mismatch,
    };

    if fallback_to_otm {
        let mismatch_id = first_mismatch_id.unwrap_or(0xFF);
        defmt::info!(
            "Consistent non-NT35510 probe response (id=0x{:02x}, count={}); falling back to OTM8009A",
            mismatch_id,
            mismatch_count
        );
        defmt::info!("Falling back to OTM8009A (B07 and earlier revisions)");
        LcdController::Otm8009a
    } else {
        defmt::warn!(
            "Probe inconclusive (mismatch={}, read_err={}, write_err={}); defaulting to NT35510",
            mismatch_count,
            read_error_count,
            write_error_count
        );
        LcdController::Nt35510
    }
}
