//! STM32F469I-DISCO display example using the **external** `otm8009a` driver crate.
//!
//! This example demonstrates the HAL = Transport / Crate = Driver architecture:
//!
//! 1. Configures PLLSAI for the pixel clock.
//! 2. Initialises the SDRAM via FMC so the framebuffer can live in external memory.
//! 3. Instantiates the HAL's generic [`DsiHost`].
//! 4. Passes `&mut DsiHost` to the **external** [`otm8009a::Otm8009A`] driver.
//! 5. Initialises the display through the external driver — no panel-specific code
//!    lives inside the HAL.
//!
//! ## Build
//!
//! ```bash
//! cargo build --release --example stm32f469i_disco_screen \
//!     --features="stm32f469,stm32-fmc,defmt"
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

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
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

use stm32_fmc::devices::is42s32400f_6;

// ── External display driver crate ───────────────────────────────────────────
// The OTM8009A initialisation sequence is provided entirely by an external
// crate that is generic over `DsiHostCtrlIo`.  The HAL only supplies the
// transport (`DsiHost`).
use otm8009a::{Otm8009A, Otm8009AConfig};

// ── Display geometry ────────────────────────────────────────────────────────
const WIDTH: u16 = 480;
const HEIGHT: u16 = 800;

const OTM8009A_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
    active_width: WIDTH,
    active_height: HEIGHT,
    h_back_porch: 20,
    h_front_porch: 20,
    v_back_porch: 10,
    v_front_porch: 10,
    h_sync: 1,
    v_sync: 1,
    frame_rate: 60,
    h_sync_pol: true,
    v_sync_pol: true,
    no_data_enable_pol: false,
    pixel_clock_pol: true,
};

// ── FMC pin helper ──────────────────────────────────────────────────────────
macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        ($(fmc_alt::$alt::from($pin.internal_pull_up(true))),*)
    };
}

// ── Entry point ─────────────────────────────────────────────────────────────
#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    // ── 1. Clock tree ───────────────────────────────────────────────────
    let hse_freq = 8.MHz();
    let mut rcc = dp
        .RCC
        .freeze(Config::hse(hse_freq).pclk2(32.MHz()).sysclk(180.MHz()));
    let mut delay = cp.SYST.delay(&rcc.clocks);

    // ── 2. GPIO setup ───────────────────────────────────────────────────
    let gpioc = dp.GPIOC.split(&mut rcc);
    let gpiod = dp.GPIOD.split(&mut rcc);
    let gpioe = dp.GPIOE.split(&mut rcc);
    let gpiof = dp.GPIOF.split(&mut rcc);
    let gpiog = dp.GPIOG.split(&mut rcc);
    let gpioh = dp.GPIOH.split(&mut rcc);
    let gpioi = dp.GPIOI.split(&mut rcc);

    // LCD hardware reset
    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    // ── 3. Initialise SDRAM via FMC ─────────────────────────────────────
    // The framebuffer will reside in external SDRAM (16 MB starting at 0xC000_0000).
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
    let sdram_ptr: *mut u32 = sdram.init(&mut delay);
    // Safety: `sdram.init` returns a pointer to the start of the initialised SDRAM bank
    // which is always non-null (0xC000_0000 on the STM32F469I-DISCO). The region is 16 MiB
    // (4M × 32-bit words), which is larger than our framebuffer (480 × 800 = 384 000 words).
    assert!(!sdram_ptr.is_null(), "SDRAM init returned null pointer");
    let fb = unsafe { slice::from_raw_parts_mut(sdram_ptr, fb_size) };
    // Clear framebuffer to black
    for pixel in fb.iter_mut() {
        *pixel = 0xFF00_0000; // opaque black (ARGB8888)
    }

    // ── 4. Initialise DSI Host (HAL transport layer) ────────────────────
    let ltdc_freq = 27_429.kHz();

    // Safety: these PLL parameters are specific to the STM32F469I-DISCO board with 8 MHz HSE.
    // VCO = (8 MHz / IDF=2) × 2 × NDIV=125 = 1 000 MHz
    // PHY clock = VCO / (2 × ODF=1) = 500 MHz → lane byte clock = 62.5 MHz
    // ECKDIV=4 → TX escape clock ≈ 15.6 MHz (must be < 20 MHz per spec)
    let dsi_pll_config = unsafe { DsiPllConfig::manual(125, 2, 0 /* div1 */, 4) };

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

    defmt::info!("Initialising DSI host...");
    let mut dsi_host = match DsiHost::init(
        dsi_pll_config,
        OTM8009A_DISPLAY_CONFIG,
        dsi_config,
        dp.DSI,
        &mut rcc,
    ) {
        Ok(host) => host,
        Err(e) => defmt::panic!("DSI host init failed: {:?}", e),
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
    dsi_host.enable_bus_turn_around();

    // ── 5. Initialise LTDC (pixel engine) ───────────────────────────────
    // PLLSAI is NOT configured here because the DSI PLL provides the pixel clock.
    defmt::info!("Initialising LTDC for DSI output...");
    let mut display = DisplayController::<u32>::new_dsi(
        dp.LTDC,
        dp.DMA2D,
        PixelFormat::ARGB8888,
        OTM8009A_DISPLAY_CONFIG,
    );

    // Attach the SDRAM-backed framebuffer to layer 1
    display.config_layer(Layer::L1, fb, PixelFormat::ARGB8888);
    display.enable_layer(Layer::L1);
    display.reload();

    // ── 6. Initialise the display via the *external* OTM8009A crate ─────
    // This is the key architectural point: the HAL provides the DsiHost
    // (transport), and the external `otm8009a` crate provides the driver
    // (panel-specific init sequence).  The driver is generic over
    // `DsiHostCtrlIo`, which `DsiHost` implements.
    defmt::info!("Initialising OTM8009A via external driver crate...");
    let otm_config = Otm8009AConfig {
        frame_rate: otm8009a::FrameRate::_60Hz,
        mode: otm8009a::Mode::Portrait,
        color_map: otm8009a::ColorMap::Rgb,
        cols: WIDTH,
        rows: HEIGHT,
    };
    let mut otm = Otm8009A::new();
    if let Err(e) = otm.init(&mut dsi_host, otm_config, &mut delay) {
        defmt::panic!("OTM8009A init failed: {:?}", e);
    }

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.force_rx_low_power(true);

    defmt::info!("Display ready — drawing colour bars");

    // ── 7. Draw a simple colour-bar pattern ─────────────────────────────
    let bar_height = HEIGHT as usize / 4;
    let colors: [u32; 4] = [
        0xFFFF_0000, // red
        0xFF00_FF00, // green
        0xFF00_00FF, // blue
        0xFFFF_FFFF, // white
    ];
    for (i, &color) in colors.iter().enumerate() {
        let y_start = i * bar_height;
        let y_end = if i == 3 {
            HEIGHT as usize
        } else {
            (i + 1) * bar_height
        };
        for y in y_start..y_end {
            for x in 0..WIDTH as usize {
                display.draw_pixel(Layer::L1, x, y, color);
            }
        }
    }
    display.reload();

    defmt::info!("Done. Looping.");
    loop {
        cortex_m::asm::wfi();
    }
}
