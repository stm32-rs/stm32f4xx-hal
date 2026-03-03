//! LCD display initialization for the STM32F469I-DISCO board.
//!
//! Provides the complete DSI + LTDC bring-up sequence supporting both
//! board revisions:
//! - B08 and later: NT35510 LCD controller
//! - B07 and earlier: OTM8009A LCD controller
//!
//! The panel is auto-detected at runtime via DSI probe reads.
//!
//! # Usage
//!
//! ```no_run
//! let (mut display_ctrl, _controller) = lcd::init_display_full(
//!     dp.DSI, dp.LTDC, dp.DMA2D,
//!     &mut rcc, &mut delay,
//!     lcd::BoardHint::Unknown,
//!     PixelFormat::RGB565,
//! );
//! display_ctrl.config_layer(Layer::L1, buffer, PixelFormat::RGB565);
//! display_ctrl.enable_layer(Layer::L1);
//! display_ctrl.reload();
//! ```

// Based on STM32CubeF4 BSP LCD driver (STMicroelectronics, BSD-3-Clause)

use crate::hal::{
    dsi::{
        ColorCoding, DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiInterrupts,
        DsiMode, DsiPhyTimers, DsiPllConfig, DsiVideoMode, LaneCount,
    },
    ltdc::{DisplayConfig, DisplayController, PixelFormat},
    pac::{DMA2D, DSI, LTDC},
    prelude::*,
    rcc::Rcc,
};
#[cfg(feature = "framebuffer")]
use crate::hal::{
    ltdc::{Layer, LtdcFramebuffer},
    pac,
    timer::SysDelay,
};
#[cfg(feature = "framebuffer")]
use crate::sdram::{self, SdramRemainders};

#[cfg(feature = "framebuffer")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, RgbColor},
};
use embedded_hal::delay::DelayNs;
use embedded_hal_02::blocking::delay::{DelayMs, DelayUs};
use nt35510::Nt35510;
use otm8009a::{Otm8009A, Otm8009AConfig};

/// Panel width in pixels (portrait).
pub const WIDTH: u16 = 480;
/// Panel height in pixels (portrait).
pub const HEIGHT: u16 = 800;
/// Framebuffer size in pixels.
pub const FB_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

/// NT35510 display timing (B08 revision).
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

/// OTM8009A display timing (B07 and earlier revisions).
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

/// Default display config (works for both panel types).
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
pub enum BoardHint {
    /// FT6X06 at 0x38 found — likely NT35510 (newer revision).
    NewRevisionLikely,
    /// Legacy touch at 0x2A found — likely OTM8009A (older revision).
    LegacyRevisionLikely,
    /// No reliable hint available.
    Unknown,
}

/// Detect which LCD controller is connected via DSI probe.
///
/// Uses 3 probe retries with delays. Tracks read/write errors and mismatches.
/// Uses the board hint to inform the fallback decision.
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
                defmt::info!("NT35510 (B08) detected on attempt {}", attempt);
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
        #[cfg(feature = "defmt")]
        {
            let mismatch_id = first_mismatch_id.unwrap_or(0xFF);
            defmt::info!(
                "Consistent non-NT35510 response (id=0x{:02x}, count={}); falling back to OTM8009A",
                mismatch_id,
                mismatch_count
            );
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
/// After calling this, wait 20ms before any panel communication.
/// Prefer [`init_dsi_with_delay`] which includes the delay.
pub fn init_dsi(dsi: DSI, rcc: &mut Rcc) -> DsiHost {
    let hse_freq = 8.MHz();
    let ltdc_freq = 27_429.kHz();
    // VCO = (8MHz HSE / 2 IDF) * 2 * 125 = 1000MHz
    // 1000MHz VCO / (2 * 1 ODF * 8) = 62.5MHz
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

/// Initialize DSI host and wait for panel link to settle (20ms).
pub fn init_dsi_with_delay(dsi: DSI, rcc: &mut Rcc, delay: &mut impl DelayMs<u32>) -> DsiHost {
    let dsi_host = init_dsi(dsi, rcc);
    delay.delay_ms(20u32);
    dsi_host
}

/// Detect and initialize the LCD panel, then switch DSI to high-speed mode.
pub fn init_panel(
    dsi_host: &mut DsiHost,
    delay: &mut (impl DelayUs<u32> + DelayMs<u32> + DelayNs),
    board_hint: BoardHint,
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
            defmt::info!("Initializing OTM8009A (B07 and earlier)...");
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
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    #[cfg(feature = "defmt")]
    defmt::info!("Panel initialized, DSI in high-speed mode");
    controller
}

/// Create the LTDC display controller for RGB565.
pub fn init_ltdc_rgb565(ltdc: LTDC, dma2d: DMA2D) -> DisplayController<u16> {
    DisplayController::<u16>::new_dsi(ltdc, dma2d, PixelFormat::RGB565, DISPLAY_CONFIG)
}

/// Create the LTDC display controller for ARGB8888.
pub fn init_ltdc_argb8888(ltdc: LTDC, dma2d: DMA2D) -> DisplayController<u32> {
    DisplayController::<u32>::new_dsi(ltdc, dma2d, PixelFormat::ARGB8888, DISPLAY_CONFIG)
}

/// Full display initialization following the proven lcd-test sequence.
///
/// Handles the complete init sequence in the correct order:
/// 1. DSI host init
/// 2. 20ms delay for panel link settle
/// 3. LCD controller detection
/// 4. LTDC initialization (before panel init — this is critical)
/// 5. Panel initialization
/// 6. Switch DSI to high-speed mode
///
/// Returns `(DisplayController, LcdController)`.
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
    let display_ctrl = match pixel_format {
        PixelFormat::RGB565 => DisplayController::<u16>::new(
            ltdc,
            dma2d,
            None,
            pixel_format,
            controller.display_config(),
            Some(hse_freq),
        ),
        _ => DisplayController::<u16>::new_dsi(
            ltdc,
            dma2d,
            pixel_format,
            controller.display_config(),
        ),
    };

    // Step 5: Set command mode and init panel
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInLowPower);
    dsi_host.force_rx_low_power(true);

    match controller {
        LcdController::Nt35510 => {
            #[cfg(feature = "defmt")]
            defmt::info!("Initializing NT35510 (B08 revision)...");
            let mut panel = Nt35510::new();
            panel.init(&mut dsi_host, delay).unwrap();
        }
        LcdController::Otm8009a => {
            #[cfg(feature = "defmt")]
            defmt::info!("Initializing OTM8009A (B07 and earlier)...");
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
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    #[cfg(feature = "defmt")]
    defmt::info!("Display initialized successfully");

    (display_ctrl, controller)
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "framebuffer")]
pub fn init_display_pipeline(
    fmc: pac::FMC,
    dsi: pac::DSI,
    ltdc: pac::LTDC,
    dma2d: pac::DMA2D,
    gpioc: stm32f4xx_hal::gpio::gpioc::Parts,
    gpiod: stm32f4xx_hal::gpio::gpiod::Parts,
    gpioe: stm32f4xx_hal::gpio::gpioe::Parts,
    gpiof: stm32f4xx_hal::gpio::gpiof::Parts,
    gpiog: stm32f4xx_hal::gpio::gpiog::Parts,
    gpioh: stm32f4xx_hal::gpio::gpioh::Parts,
    gpioi: stm32f4xx_hal::gpio::gpioi::Parts,
    rcc: &mut Rcc,
    delay: &mut SysDelay,
) -> (LtdcFramebuffer<u16>, SdramRemainders) {
    let (sdram_pins, remainders, ph7) =
        sdram::split_sdram_pins(gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi);

    let mut lcd_reset = ph7.into_push_pull_output();
    lcd_reset.set_low();
    embedded_hal_02::blocking::delay::DelayMs::<u32>::delay_ms(delay, 20u32);
    lcd_reset.set_high();
    embedded_hal_02::blocking::delay::DelayMs::<u32>::delay_ms(delay, 10u32);

    let mut sdram = sdram::Sdram::new(fmc, sdram_pins, &rcc.clocks, delay);
    let buffer: &'static mut [u16] = sdram.subslice_mut(0, FB_SIZE);
    let mut fb = LtdcFramebuffer::new(buffer, WIDTH, HEIGHT);
    fb.clear(Rgb565::BLACK).ok();
    let buffer = fb.into_inner();

    let (mut display_ctrl, _lcd_controller) = init_display_full(
        dsi,
        ltdc,
        dma2d,
        rcc,
        delay,
        BoardHint::Unknown,
        PixelFormat::RGB565,
    );

    display_ctrl.config_layer(Layer::L1, buffer, PixelFormat::RGB565);
    display_ctrl.enable_layer(Layer::L1);
    display_ctrl.reload();

    let buffer = display_ctrl
        .layer_buffer_mut(Layer::L1)
        .expect("layer L1 buffer");
    let buffer: &'static mut [u16] = unsafe { core::mem::transmute(buffer) };

    (LtdcFramebuffer::new(buffer, WIDTH, HEIGHT), remainders)
}
