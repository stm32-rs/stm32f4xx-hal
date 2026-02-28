//! STM32F469I-DISCO display initialization helpers.
//!
//! This module exposes the proven F469 DISCO LCD bring-up sequence as
//! reusable helpers for the f469disco examples:
//! - DSI host init with correct PLL, PHY timers and link settle delay
//! - Runtime NT35510 vs OTM8009A detection using DSI ID reads
//! - LTDC init using timing derived from detected controller
//! - Panel init for both NT35510 (HAL driver) and OTM8009A (external crate)
//!
//! It is based on the `f469disco-lcd-test` example and the proven LCD bring-up sequence.
// Based on STM32CubeF4 BSP LCD driver (STMicroelectronics, BSD-3-Clause)
// Per-item #[allow(dead_code)] is used below for shared helpers that are not
// called by every example but exist for API completeness.

use stm32f4xx_hal::{
    dsi::{
        ColorCoding, DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiInterrupts,
        DsiMode, DsiPhyTimers, DsiPllConfig, DsiVideoMode, LaneCount,
    },
    ltdc::{DisplayConfig, DisplayController, PixelFormat},
    pac::{DMA2D, DSI, LTDC},
    prelude::*,
    rcc::Rcc,
};

use embedded_hal::delay::DelayNs;
use embedded_hal_02::blocking::delay::{DelayMs, DelayUs};
use otm8009a::{Otm8009A, Otm8009AConfig};

use nt35510::Nt35510;

/// Panel width in pixels (portrait).
pub const WIDTH: u16 = 480;
/// Panel height in pixels (portrait).
pub const HEIGHT: u16 = 800;
/// Framebuffer size in pixels.
pub const FB_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

// NT35510 timing (B08 revision) - using OTM8009A-compatible tight timings.
// These timings are proven to work on NT35510 displays and also on OTM8009A.
pub const NT35510_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
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

// OTM8009A timing (B07 and earlier revisions).
pub const OTM8009A_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
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

/// Default display config (NT35510 timings, works for both).
pub const DISPLAY_CONFIG: DisplayConfig = NT35510_DISPLAY_CONFIG;

/// Detected / selected LCD controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdController {
    Nt35510,
    Otm8009a,
}

impl LcdController {
    /// Return the LTDC timing configuration for this controller.
    pub fn display_config(self) -> DisplayConfig {
        match self {
            LcdController::Nt35510 => NT35510_DISPLAY_CONFIG,
            LcdController::Otm8009a => OTM8009A_DISPLAY_CONFIG,
        }
    }
}

/// Hint about board revision from external probes (e.g. touch controller I2C).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(dead_code)] // Variants used by board detection; not all constructed by every example
pub enum BoardHint {
    /// FT6X06 at 0x38 found → likely NT35510 (newer revision).
    NewRevisionLikely,
    /// Legacy touch at 0x2A found → likely OTM8009A (older revision).
    LegacyRevisionLikely,
    /// No reliable hint available.
    Unknown,
}

/// Detect which LCD controller is connected via DSI probe.
///
/// Uses the proven detection algorithm from `f469disco-lcd-test`:
/// - 3 probe retries with delays
/// - Tracks read/write errors and mismatches
/// - Uses board hint to inform fallback decision
pub fn detect_lcd_controller(
    dsi_host: &mut DsiHost,
    delay: &mut (impl DelayUs<u32> + DelayMs<u32> + DelayNs),
    board_hint: BoardHint,
) -> LcdController {
    const PROBE_RETRIES: u8 = 3;
    embedded_hal_02::blocking::delay::DelayUs::<u32>::delay_us(delay, 20_000u32);

    let mut nt35510 = Nt35510::new();
    let mut mismatch_count = 0u8;
    let mut first_mismatch_id: Option<u8> = None;
    let mut consistent_mismatch = true;
    let mut read_error_count = 0u8;
    let mut write_error_count = 0u8;

    for attempt in 1..=PROBE_RETRIES {
        #[cfg(not(feature = "defmt"))]
        let _ = attempt;
        match nt35510.probe(dsi_host, delay) {
            Ok(_) => {
                #[cfg(feature = "defmt")]
                defmt::info!("NT35510 (B08) detected successfully on attempt {}", attempt);
                return LcdController::Nt35510;
            }
            Err(nt35510::Error::DsiRead) => {
                read_error_count = read_error_count.saturating_add(1);
                #[cfg(feature = "defmt")]
                defmt::warn!("NT35510 probe attempt {} failed: DSI read error", attempt);
            }
            Err(nt35510::Error::DsiWrite) => {
                write_error_count = write_error_count.saturating_add(1);
                #[cfg(feature = "defmt")]
                defmt::warn!("NT35510 probe attempt {} failed: DSI write error", attempt);
            }
            Err(nt35510::Error::ProbeMismatch(id)) => {
                mismatch_count = mismatch_count.saturating_add(1);
                match first_mismatch_id {
                    None => first_mismatch_id = Some(id),
                    Some(first) if first != id => consistent_mismatch = false,
                    Some(_) => {}
                }
                #[cfg(feature = "defmt")]
                defmt::info!(
                    "NT35510 probe attempt {} mismatch: RDID2=0x{:02x}",
                    attempt,
                    id
                );
            }
            Err(nt35510::Error::InvalidDimensions) => {
                #[cfg(feature = "defmt")]
                defmt::warn!(
                    "NT35510 probe attempt {} failed: invalid dimensions",
                    attempt
                );
            }
        }
        embedded_hal_02::blocking::delay::DelayUs::<u32>::delay_us(delay, 5_000u32);
    }

    let fallback_to_otm = match board_hint {
        BoardHint::LegacyRevisionLikely => mismatch_count >= 1 && consistent_mismatch,
        BoardHint::NewRevisionLikely => mismatch_count >= PROBE_RETRIES && consistent_mismatch,
        BoardHint::Unknown => mismatch_count >= 2 && consistent_mismatch,
    };

    if fallback_to_otm {
        let mismatch_id = first_mismatch_id.unwrap_or(0xFF);
        #[cfg(not(feature = "defmt"))]
        let _ = mismatch_id;
        #[cfg(feature = "defmt")]
        {
            defmt::info!(
                "Consistent non-NT35510 probe response (id=0x{:02x}, count={}); falling back to OTM8009A",
                mismatch_id,
                mismatch_count
            );
            defmt::info!("Falling back to OTM8009A (B07 and earlier revisions)");
        }
        LcdController::Otm8009a
    } else {
        #[cfg(feature = "defmt")]
        defmt::warn!(
            "Probe inconclusive (mismatch={}, read_err={}, write_err={}); defaulting to NT35510",
            mismatch_count,
            read_error_count,
            write_error_count
        );
        LcdController::Nt35510
    }
}

/// Initialize the DSI host with F469-DISCO settings.
///
/// **IMPORTANT**: After calling this, you must wait 20ms before any panel communication.
/// Use [`init_dsi_with_delay`] for the complete sequence, or call `delay.delay_ms(20)` yourself.
///
/// Returns the configured [`DsiHost`] ready for panel init.
pub fn init_dsi(dsi: DSI, rcc: &mut Rcc) -> DsiHost {
    let hse_freq = 8.MHz();
    let ltdc_freq = 27_429.kHz();
    let dsi_pll_config = unsafe { DsiPllConfig::manual(125, 2, 0, 4) };
    let dsi_config = DsiConfig {
        mode: DsiMode::Video {
            mode: DsiVideoMode::Burst,
        },
        lane_count: LaneCount::DoubleLane,
        channel: DsiChannel::Ch0,
        hse_freq,
        ltdc_freq,
        interrupts: DsiInterrupts::None,
        color_coding_host: ColorCoding::SixteenBitsConfig1,
        color_coding_wrapper: ColorCoding::SixteenBitsConfig1,
        lp_size: 64,
        vlp_size: 64,
    };

    #[cfg(feature = "defmt")]
    defmt::info!("Initializing DSI...");
    let mut dsi_host = DsiHost::init(dsi_pll_config, DISPLAY_CONFIG, dsi_config, dsi, rcc).unwrap();

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
    dsi_host.enable_bus_turn_around();

    dsi_host
}

/// Initialize DSI host and wait for panel link to settle.
///
/// This is the preferred entry point – it includes the critical 20ms delay
/// after DSI start that allows the panel link to settle before communication.
#[allow(dead_code)]
pub fn init_dsi_with_delay(dsi: DSI, rcc: &mut Rcc, delay: &mut impl DelayMs<u32>) -> DsiHost {
    let dsi_host = init_dsi(dsi, rcc);
    delay.delay_ms(20u32); // Critical: allow panel link to settle
    dsi_host
}

/// Detect and initialize the LCD panel, then switch DSI to high-speed mode.
///
/// Handles both NT35510 (B08) and OTM8009A (B07 and earlier) panels.
/// Uses runtime detection with fallback logic.
#[allow(dead_code)]
pub fn init_panel(
    dsi_host: &mut DsiHost,
    delay: &mut (impl DelayUs<u32> + DelayMs<u32> + DelayNs),
    board_hint: BoardHint,
    _rgb565: bool,
) -> LcdController {
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInLowPower);
    dsi_host.force_rx_low_power(true);

    let controller = detect_lcd_controller(dsi_host, delay, board_hint);

    match controller {
        LcdController::Nt35510 => {
            #[cfg(feature = "defmt")]
            defmt::info!("Initializing NT35510 (B08 revision)...");
            let mut panel = Nt35510::new();
            panel.init(dsi_host, delay).unwrap();
        }
        LcdController::Otm8009a => {
            #[cfg(feature = "defmt")]
            defmt::info!("Initializing OTM8009A (B07 and earlier revisions)...");
            let otm_config = Otm8009AConfig {
                frame_rate: otm8009a::FrameRate::_60Hz,
                mode: otm8009a::Mode::Portrait,
                color_map: otm8009a::ColorMap::Rgb,
                cols: WIDTH,
                rows: HEIGHT,
            };
            let mut otm = Otm8009A::new();
            otm.init(dsi_host, otm_config, delay).unwrap();
        }
    }

    dsi_host.force_rx_low_power(false);
    #[cfg(feature = "defmt")]
    defmt::info!("force_rx_low_power cleared — DSI HS mode fully active");
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    #[cfg(feature = "defmt")]
    defmt::info!("Panel initialized, DSI in high-speed mode");
    controller
}

/// Create and configure the LTDC display controller for RGB565.
#[allow(dead_code)]
pub fn init_ltdc_rgb565(ltdc: LTDC, dma2d: DMA2D) -> DisplayController<u16> {
    DisplayController::<u16>::new_dsi(ltdc, dma2d, PixelFormat::RGB565, DISPLAY_CONFIG)
}

/// Create and configure the LTDC display controller for ARGB8888.
#[allow(dead_code)]
pub fn init_ltdc_argb8888(ltdc: LTDC, dma2d: DMA2D) -> DisplayController<u32> {
    DisplayController::<u32>::new_dsi(ltdc, dma2d, PixelFormat::ARGB8888, DISPLAY_CONFIG)
}

// =============================================================================
// FULL INITIALIZATION HELPERS
// These match the proven sequence from f469disco-lcd-test.rs
// =============================================================================

/// Full display initialization following the proven lcd-test sequence.
///
/// This function handles the complete init sequence in the correct order:
/// 1. DSI host init
/// 2. 20ms delay for panel link settle
/// 3. LCD controller detection
/// 4. LTDC initialization (BEFORE panel init - this is critical!)
/// 5. Panel initialization
/// 6. Switch DSI to high-speed mode
///
/// Returns `(display_controller, lcd_controller)` on success.
pub fn init_display_full(
    dsi: DSI,
    ltdc: LTDC,
    dma2d: DMA2D,
    rcc: &mut Rcc,
    delay: &mut (impl DelayUs<u32> + DelayMs<u32> + DelayNs),
    board_hint: BoardHint,
    pixel_format: PixelFormat,
) -> (DisplayController<u16>, LcdController) {
    let hse_freq = 8.MHz();

    // Step 1: DSI host init
    let mut dsi_host = init_dsi(dsi, rcc);

    // Step 2: Critical delay for panel link
    embedded_hal_02::blocking::delay::DelayMs::<u32>::delay_ms(delay, 20u32);

    // Step 3: Detect LCD controller
    let controller = detect_lcd_controller(&mut dsi_host, delay, board_hint);
    #[cfg(feature = "defmt")]
    defmt::info!("Detected LCD controller: {:?}", controller);

    // Step 4: Initialize LTDC BEFORE panel init
    // This matches the proven lcd-test.rs sequence
    let display_ctrl = match pixel_format {
        PixelFormat::RGB565 => DisplayController::<u16>::new(
            ltdc,
            dma2d,
            None,
            pixel_format,
            controller.display_config(),
            Some(hse_freq),
        ),
        _ => {
            // Fall back to new_dsi for other formats (uncommon)
            DisplayController::<u16>::new_dsi(
                ltdc,
                dma2d,
                pixel_format,
                controller.display_config(),
            )
        }
    };

    // Step 5: Set command mode and init panel
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInLowPower);
    dsi_host.force_rx_low_power(true);

    match controller {
        LcdController::Nt35510 => {
            #[cfg(feature = "defmt")]
            defmt::info!("Initializing NT35510 (B08 revision)...");
            let mut panel = Nt35510::new();
            // For RGB565 framebuffer, we still use RGB888 panel mode (LTDC handles conversion)
            panel.init(&mut dsi_host, delay).unwrap();
        }
        LcdController::Otm8009a => {
            #[cfg(feature = "defmt")]
            defmt::info!("Initializing OTM8009A (B07 and earlier revisions)...");
            let otm_config = Otm8009AConfig {
                frame_rate: otm8009a::FrameRate::_60Hz,
                mode: otm8009a::Mode::Portrait,
                color_map: otm8009a::ColorMap::Rgb,
                cols: WIDTH,
                rows: HEIGHT,
            };
            let mut otm = Otm8009A::new();
            otm.init(&mut dsi_host, otm_config, delay).unwrap();
        }
    }

    // Step 6: Switch to high-speed mode
    dsi_host.force_rx_low_power(false);
    #[cfg(feature = "defmt")]
    defmt::info!("force_rx_low_power cleared — DSI HS mode fully active");
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    #[cfg(feature = "defmt")]
    defmt::info!("Display initialized successfully");

    (display_ctrl, controller)
}
