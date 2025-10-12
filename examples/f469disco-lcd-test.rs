//! This example initializes the STM32F469I-DISCO LCD and displays a test pattern
//! 
//! This example supports both STM32F469I-DISCO board revisions:
//! - B08 revision (NT35510 LCD controller) - auto-detected and preferred
//! - B07 and earlier (OTM8009A LCD controller) - fallback
//! 
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

// NT35510 Constants and Driver (inline implementation for B08 board support)
#[allow(dead_code)]
mod nt35510 {
    use super::hal::dsi::DsiHost;
    use embedded_hal_02::blocking::delay::DelayUs;
    
    // NT35510 Commands (from nt35510_reg.h)
    pub const CMD_NOP: u8 = 0x00;
    pub const CMD_SWRESET: u8 = 0x01;
    pub const CMD_RDDID: u8 = 0x04;
    pub const CMD_RDID1: u8 = 0xDA;  // Read ID1 value (used for detection)
    pub const CMD_RDID2: u8 = 0xDB;  // Read ID2 value
    pub const CMD_RDID3: u8 = 0xDC;  // Read ID3 value
    pub const CMD_SLPOUT: u8 = 0x11;
    pub const CMD_DISPON: u8 = 0x29;
    pub const CMD_DISPOFF: u8 = 0x28;
    pub const CMD_CASET: u8 = 0x2A;
    pub const CMD_RASET: u8 = 0x2B;
    pub const CMD_RAMWR: u8 = 0x2C;
    pub const CMD_MADCTL: u8 = 0x36;
    pub const CMD_COLMOD: u8 = 0x3A;
    pub const CMD_WRDISBV: u8 = 0x51;
    pub const CMD_WRCTRLD: u8 = 0x53;
    pub const CMD_WRCABC: u8 = 0x55;
    pub const CMD_WRCABCMB: u8 = 0x5E;
    
    // NT35510 Register settings  
    pub const COLMOD_RGB565: u8 = 0x55;
    pub const COLMOD_RGB888: u8 = 0x77;
    pub const MADCTR_MODE_PORTRAIT: u8 = 0x00;
    pub const MADCTR_MODE_LANDSCAPE: u8 = 0x60;
    
    pub struct Nt35510 {
        pub initialized: bool,
    }
    
    impl Nt35510 {
        pub fn new() -> Self {
            Self { initialized: false }
        }
        
        /// Try to detect NT35510 by testing command response (similar to C reference)
        pub fn probe<D: DelayUs<u32>>(
            &mut self, 
            dsi_host: &mut DsiHost, 
            _delay: &mut D
        ) -> Result<(), &'static str> {
            use embedded_display_controller::dsi::{DsiReadCommand, DsiHostCtrlIo};
            
            defmt::info!("Probing for NT35510 LCD controller...");
            
            // Try to read RDID1 register - we don't care about the data, just if the command responds
            // This matches the C reference behavior: nt35510_read_reg with length 0
            let mut dummy_data = [0u8; 1];
            match dsi_host.read(DsiReadCommand::DcsShort { arg: CMD_RDID1 }, &mut dummy_data) {
                Ok(_) => {
                    defmt::info!("NT35510 detected - RDID1 command responded");
                    return Ok(());
                },
                Err(e) => {
                    defmt::info!("NT35510 not detected - RDID1 command failed: {:?}", e);
                    return Err("NT35510 not detected");
                }
            }
        }
        
        /// Initialize NT35510 controller
        pub fn init<D: DelayUs<u32>>(
            &mut self, 
            dsi_host: &mut DsiHost, 
            delay: &mut D
        ) -> Result<(), &'static str> {
            defmt::info!("Initializing NT35510...");
            
            // Proprietary initialization sequence
            self.write_reg(dsi_host, 0xF0, &[0x55, 0xAA, 0x52, 0x08, 0x01])?; // LV2: Page 1 enable
            self.write_reg(dsi_host, 0xB0, &[0x03, 0x03, 0x03])?; // AVDD: 5.2V
            self.write_reg(dsi_host, 0xB6, &[0x46, 0x46, 0x46])?; // AVDD: Ratio
            self.write_reg(dsi_host, 0xB1, &[0x03, 0x03, 0x03])?; // AVEE: -5.2V
            self.write_reg(dsi_host, 0xB7, &[0x36, 0x36, 0x36])?; // AVEE: Ratio
            self.write_reg(dsi_host, 0xB2, &[0x00, 0x00, 0x02])?; // VCL: -2.5V
            self.write_reg(dsi_host, 0xB8, &[0x26, 0x26, 0x26])?; // VCL: Ratio
            self.write_reg(dsi_host, 0xBF, &[0x01])?; // VGH: 15V (Free Pump)
            self.write_reg(dsi_host, 0xB3, &[0x09, 0x09, 0x09])?;
            self.write_reg(dsi_host, 0xB9, &[0x36, 0x36, 0x36])?; // VGH: Ratio
            self.write_reg(dsi_host, 0xB5, &[0x08, 0x08, 0x08])?; // VGL_REG: -10V
            self.write_reg(dsi_host, 0xBA, &[0x26, 0x26, 0x26])?; // VGLX: Ratio
            self.write_reg(dsi_host, 0xBC, &[0x00, 0x80, 0x00])?; // VGMP/VGSP: 4.5V/0V
            self.write_reg(dsi_host, 0xBD, &[0x00, 0x80, 0x00])?; // VGMN/VGSN:-4.5V/0V
            self.write_reg(dsi_host, 0xBE, &[0x00, 0x50])?; // VCOM: -1.325V
            
            // Page 0 enable
            self.write_reg(dsi_host, 0xF0, &[0x55, 0xAA, 0x52, 0x08, 0x00])?;
            self.write_reg(dsi_host, 0xB1, &[0xFC, 0x00])?; // Display optional control
            self.write_reg(dsi_host, 0xB6, &[0x03])?; // Set source output data hold time
            self.write_reg(dsi_host, 0xB5, &[0x51])?; // Display resolution control
            self.write_reg(dsi_host, 0xB7, &[0x00, 0x00])?; // Gate EQ control
            self.write_reg(dsi_host, 0xB8, &[0x01, 0x02, 0x02, 0x02])?; // Src EQ control(Mode2)
            self.write_reg(dsi_host, 0xBC, &[0x00, 0x00, 0x00])?;
            self.write_reg(dsi_host, 0xCC, &[0x03, 0x00, 0x00])?;
            self.write_reg(dsi_host, 0xBA, &[0x01])?;
            
            delay.delay_us(200_000); // 200ms delay
            
            // Set orientation (Portrait)
            self.write_cmd(dsi_host, CMD_MADCTL, MADCTR_MODE_PORTRAIT)?;
            
            // Set column address
            self.write_reg(dsi_host, CMD_CASET, &[0x00, 0x00, 0x01, 0xDF])?; // 0-479
            
            // Set row address  
            self.write_reg(dsi_host, CMD_RASET, &[0x00, 0x00, 0x03, 0x1F])?; // 0-799
            
            // Sleep out
            self.write_cmd(dsi_host, CMD_SLPOUT, 0)?;
            delay.delay_us(20_000); // 20ms delay
            
            // Set pixel format to RGB888
            self.write_cmd(dsi_host, CMD_COLMOD, COLMOD_RGB888)?;
            
            // CABC settings
            self.write_cmd(dsi_host, CMD_WRDISBV, 0x7F)?; // brightness
            self.write_cmd(dsi_host, CMD_WRCTRLD, 0x2C)?; // control display
            self.write_cmd(dsi_host, CMD_WRCABC, 0x02)?; // content adaptive brightness
            self.write_cmd(dsi_host, CMD_WRCABCMB, 0xFF)?; // CABC minimum brightness
            
            // Display on
            self.write_cmd(dsi_host, CMD_DISPON, 0)?;
            
            // Memory write (start frame write)
            self.write_cmd(dsi_host, CMD_RAMWR, 0)?;
            
            self.initialized = true;
            defmt::info!("NT35510 initialization complete");
            Ok(())
        }
        
        fn write_cmd(&self, dsi_host: &mut DsiHost, cmd: u8, param: u8) -> Result<(), &'static str> {
            use embedded_display_controller::dsi::{DsiWriteCommand, DsiHostCtrlIo};
            dsi_host.write(DsiWriteCommand::DcsShortP1 { arg: cmd, data: param }).map_err(|_| "DSI write failed")
        }
        
        fn write_reg(&self, dsi_host: &mut DsiHost, reg: u8, data: &[u8]) -> Result<(), &'static str> {
            use embedded_display_controller::dsi::{DsiWriteCommand, DsiHostCtrlIo};
            if data.is_empty() {
                self.write_cmd(dsi_host, reg, 0)
            } else if data.len() == 1 {
                self.write_cmd(dsi_host, reg, data[0])
            } else {
                dsi_host.write(DsiWriteCommand::DcsLongWrite { arg: reg, data }).map_err(|_| "DSI long write failed")
            }
        }
    }
}

// Display configurations for different controllers
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LcdController {
    Nt35510,
    Otm8009a,
}

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 800;

// NT35510 timing (B08 revision)
pub const NT35510_DISPLAY_CONFIG: DisplayConfig = DisplayConfig {
    active_width: WIDTH as _,
    active_height: HEIGHT as _,
    h_back_porch: 150,
    h_front_porch: 150,
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

    // Reset display
    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    // Initialize LTDC with NT35510 configuration (compatible with both controllers)
    // We need DSI communication before controller detection, so we use the more capable timing
    defmt::info!("Initializing LTDC");
    let ltdc_freq = 27_429.kHz();
    let _display = DisplayController::<u32>::new(
        dp.LTDC,
        dp.DMA2D,
        None,
        PixelFormat::ARGB8888,
        NT35510_DISPLAY_CONFIG, // NT35510 timing works for both controllers
        Some(hse_freq),
    );

    // Initialize DSI Host with NT35510-compatible settings
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
        lp_size: 64,    // NT35510 compatible
        vlp_size: 64,   // NT35510 compatible
    };

    defmt::info!("Initializing DSI {:?} {:?}", dsi_config, dsi_pll_config);
    let mut dsi_host = DsiHost::init(
        dsi_pll_config,
        NT35510_DISPLAY_CONFIG,
        dsi_config,
        dp.DSI,
        &mut rcc,
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

    // Now detect which LCD controller is present
    let controller = detect_lcd_controller(&mut dsi_host, &mut delay);
    defmt::info!("Detected LCD controller: {:?}", controller);

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.force_rx_low_power(true);
    dsi_host.enable_color_test(); // Must enable before display initialized

    // Initialize the detected LCD controller
    match controller {
        LcdController::Nt35510 => {
            defmt::info!("Initializing NT35510 (B08 revision)");
            let mut nt35510 = nt35510::Nt35510::new();
            nt35510.init(&mut dsi_host, &mut delay).unwrap();
        },
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
            otm8009a
                .init(&mut dsi_host, otm8009a_config, &mut delay)
                .unwrap();
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
    let mut touch = Ft6X06::new(&i2c, 0x38, ts_int).unwrap();
    
    // Run internal calibration of touchscreen (following display-touch.rs pattern)
    let tsc = touch.ts_calibration(&mut i2c, &mut delay);
    match tsc {
        Err(e) => defmt::warn!("Error from ts_calibration: {}", e),
        Ok(u) => defmt::info!("ts_calibration returned {}", u),
    }

    defmt::info!("Outputting Color/BER test patterns. Touch the screen to see coordinates!");
    
    let mut current_pattern = 0; // 0 for color test, 1 for BER test
    let mut pattern_timer = 0u32;
    let pattern_switch_delay = 500; // Switch patterns every 500 iterations (similar to display-touch.rs timing)
    
    // Start with color test
    dsi_host.enable_color_test();
    defmt::info!("Color test pattern");
    
    loop {
        // Check for touch events (similar to display-touch.rs approach)
        let t = touch.detect_touch(&mut i2c);
        match t {
            Ok(num) if num > 0 => {
                defmt::info!("Number of touches: {}", num);
                // Only get coordinates if touch detected
                if let Ok(point) = touch.get_touch(&mut i2c, 1) {
                    defmt::info!("Touch at x={}, y={} - weight: {}", point.x, point.y, point.weight);
                }
            },
            Ok(_) => {}, // No touches, silent
            Err(_) => {} // I2C error, silent to avoid spam
        }
        
        // Handle pattern switching on a timer (much less frequent than touch checking)
        pattern_timer += 1;
        if pattern_timer >= pattern_switch_delay {
            pattern_timer = 0;
            current_pattern = 1 - current_pattern;
            
            if current_pattern == 0 {
                dsi_host.enable_color_test();
                defmt::info!("Color test pattern");
            } else {
                dsi_host.enable_ber_test();
                defmt::info!("BER test pattern");
            }
        }
        
        // Small delay between iterations (similar to display-touch.rs)
        delay.delay_ms(10u32);
    }
}

/// Detect which LCD controller is present by attempting communication
fn detect_lcd_controller(
    dsi_host: &mut DsiHost,
    delay: &mut impl embedded_hal_02::blocking::delay::DelayUs<u32>
) -> LcdController {
    defmt::info!("Auto-detecting LCD controller...");
    
    // First try NT35510 (B08 revision) - preferred
    let mut nt35510 = nt35510::Nt35510::new();
    match nt35510.probe(dsi_host, delay) {
        Ok(_) => {
            defmt::info!("NT35510 (B08) detected successfully");
            return LcdController::Nt35510;
        },
        Err(e) => {
            defmt::info!("NT35510 detection failed: {}", e);
        }
    }
    
    // If NT35510 detection fails, assume OTM8009A (B07 and earlier)
    defmt::info!("Falling back to OTM8009A (B07 and earlier revisions)");
    LcdController::Otm8009a
}
