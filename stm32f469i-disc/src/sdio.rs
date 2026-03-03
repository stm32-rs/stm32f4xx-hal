//! SDIO SD card initialization for the STM32F469I-DISCO board.
//!
//! Provides SDIO peripheral setup using the on-board microSD card slot.
//! The slot uses a 4-bit wide SDIO bus on GPIO pins PC8-PC12 and PD2.
//!
//! # Usage
//!
//! ```no_run
//! let (sdio, touch_int) = sdio::init(dp.SDIO, sdram_remainders, &mut rcc);
//! // sdio: Sdio<SdCard> ready for card initialization
//! // touch_int: PC1 available for touch interrupt
//! ```

use crate::hal;
use crate::hal::pac::SDIO;
use crate::hal::rcc::Rcc;
use crate::hal::sdio::{SdCard, Sdio};
use crate::sdram::SdramRemainders;

/// Initialize the SDIO peripheral with 4-bit bus width.
///
/// Configures the SDIO pins (PC8-PC12, PD2) in alternate function mode
/// with appropriate pull resistors matching the VLS reference implementation.
///
/// # Arguments
///
/// * `sdio_pac` - SDIO peripheral from PAC
/// * `remainders` - GPIO pins remaining from SDRAM initialization
/// * `rcc` - RCC register block for clock configuration
///
/// # Returns
///
/// A tuple containing:
/// * `Sdio<SdCard>` - Initialized SDIO host (call `.init()` to detect card)
/// * `PC1<Input>` - Touch interrupt pin (not consumed by SDIO)
pub fn init(
    sdio_pac: SDIO,
    remainders: SdramRemainders,
    rcc: &mut Rcc,
) -> (Sdio<SdCard>, hal::gpio::PC1<hal::gpio::Input>) {
    // Extract and configure SDIO pins from remainders
    // Pin configuration matches VLS reference implementation:
    // - Data lines (D0-D3): internal pull-up enabled
    // - Clock: no pull-up (driven by host)
    // - Command: internal pull-up enabled
    let d0 = remainders.pc8.into_alternate().internal_pull_up(true);
    let d1 = remainders.pc9.into_alternate().internal_pull_up(true);
    let d2 = remainders.pc10.into_alternate().internal_pull_up(true);
    let d3 = remainders.pc11.into_alternate().internal_pull_up(true);
    let clk = remainders.pc12.into_alternate().internal_pull_up(false);
    let cmd = remainders.pd2.into_alternate().internal_pull_up(true);

    // Initialize SDIO peripheral with 4-bit bus
    let sdio = Sdio::new(sdio_pac, (clk, cmd, d0, d1, d2, d3), rcc);

    // Return SDIO host and touch interrupt pin (PC1) configured with pull-down
    // FT6X06 touch interrupt is active-LOW, needs pull-down for defined idle state
    (sdio, remainders.pc1.into_pull_down_input())
}
