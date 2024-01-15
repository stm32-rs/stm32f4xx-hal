//! Interface to the LCD-TFT display controller
//!
//! <div class="warning">Not tested yet</div>

#[cfg_attr(test, allow(unused_imports))]
use micromath::F32Ext;

use crate::{
    gpio::{alt::ltdc as alt, PinSpeed, Speed},
    pac::{DMA2D, LTDC, RCC},
    rcc::{Enable, Reset},
};
use fugit::HertzU32 as Hertz;

/// Display configuration constants
pub struct DisplayConfig {
    pub active_width: u16,
    pub active_height: u16,
    pub h_back_porch: u16,
    pub h_front_porch: u16,
    pub v_back_porch: u16,
    pub v_front_porch: u16,
    pub h_sync: u16,
    pub v_sync: u16,
    pub frame_rate: u16,
    /// `false`: active low, `true`: active high
    pub h_sync_pol: bool,
    /// `false`: active low, `true`: active high
    pub v_sync_pol: bool,
    /// `false`: active low, `true`: active high
    pub no_data_enable_pol: bool,
    /// `false`: active low, `true`: active high
    pub pixel_clock_pol: bool,
}

/// Accessible layers
/// * `L1`: layer 1
/// * `L2`: layer 2
#[derive(Clone, Copy, Debug)]
pub enum Layer {
    L1 = 0,
    L2 = 1,
}

#[allow(unused)]
pub struct RedPins {
    r0: alt::R0,
    r1: alt::R1,
    r2: alt::R2,
    r3: alt::R3,
    r4: alt::R4,
    r5: alt::R5,
    r6: alt::R6,
    r7: alt::R7,
}

#[allow(unused)]
pub struct GreenPins {
    g0: alt::G0,
    g1: alt::G1,
    g2: alt::G2,
    g3: alt::G3,
    g4: alt::G4,
    g5: alt::G5,
    g6: alt::G6,
    g7: alt::G7,
}

#[allow(unused)]
pub struct BluePins {
    b0: alt::B0,
    b1: alt::B1,
    b2: alt::B2,
    b3: alt::B3,
    b4: alt::B4,
    b5: alt::B5,
    b6: alt::B6,
    b7: alt::B7,
}

impl RedPins {
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn new(
        r0: impl Into<alt::R0>,
        r1: impl Into<alt::R1>,
        r2: impl Into<alt::R2>,
        r3: impl Into<alt::R3>,
        r4: impl Into<alt::R4>,
        r5: impl Into<alt::R5>,
        r6: impl Into<alt::R6>,
        r7: impl Into<alt::R7>,
    ) -> Self {
        Self {
            r0: r0.into().speed(Speed::VeryHigh),
            r1: r1.into().speed(Speed::VeryHigh),
            r2: r2.into().speed(Speed::VeryHigh),
            r3: r3.into().speed(Speed::VeryHigh),
            r4: r4.into().speed(Speed::VeryHigh),
            r5: r5.into().speed(Speed::VeryHigh),
            r6: r6.into().speed(Speed::VeryHigh),
            r7: r7.into().speed(Speed::VeryHigh),
        }
    }
}

impl GreenPins {
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn new(
        g0: impl Into<alt::G0>,
        g1: impl Into<alt::G1>,
        g2: impl Into<alt::G2>,
        g3: impl Into<alt::G3>,
        g4: impl Into<alt::G4>,
        g5: impl Into<alt::G5>,
        g6: impl Into<alt::G6>,
        g7: impl Into<alt::G7>,
    ) -> Self {
        Self {
            g0: g0.into().speed(Speed::VeryHigh),
            g1: g1.into().speed(Speed::VeryHigh),
            g2: g2.into().speed(Speed::VeryHigh),
            g3: g3.into().speed(Speed::VeryHigh),
            g4: g4.into().speed(Speed::VeryHigh),
            g5: g5.into().speed(Speed::VeryHigh),
            g6: g6.into().speed(Speed::VeryHigh),
            g7: g7.into().speed(Speed::VeryHigh),
        }
    }
}

impl BluePins {
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn new(
        b0: impl Into<alt::B0>,
        b1: impl Into<alt::B1>,
        b2: impl Into<alt::B2>,
        b3: impl Into<alt::B3>,
        b4: impl Into<alt::B4>,
        b5: impl Into<alt::B5>,
        b6: impl Into<alt::B6>,
        b7: impl Into<alt::B7>,
    ) -> Self {
        Self {
            b0: b0.into().speed(Speed::VeryHigh),
            b1: b1.into().speed(Speed::VeryHigh),
            b2: b2.into().speed(Speed::VeryHigh),
            b3: b3.into().speed(Speed::VeryHigh),
            b4: b4.into().speed(Speed::VeryHigh),
            b5: b5.into().speed(Speed::VeryHigh),
            b6: b6.into().speed(Speed::VeryHigh),
            b7: b7.into().speed(Speed::VeryHigh),
        }
    }
}

#[allow(unused)]
pub struct LtdcPins {
    r: RedPins,
    g: GreenPins,
    b: BluePins,
    hsync: alt::Hsync,
    vsync: alt::Vsync,
    de: alt::De,
    clk: alt::Clk,
}

impl LtdcPins {
    #[inline(always)]
    pub fn new(
        r: RedPins,
        g: GreenPins,
        b: BluePins,
        hsync: impl Into<alt::Hsync>,
        vsync: impl Into<alt::Vsync>,
        de: impl Into<alt::De>,
        clk: impl Into<alt::Clk>,
    ) -> Self {
        Self {
            r,
            g,
            b,
            hsync: hsync.into().speed(Speed::VeryHigh),
            vsync: vsync.into().speed(Speed::VeryHigh),
            de: de.into().speed(Speed::VeryHigh),
            clk: clk.into().speed(Speed::VeryHigh),
        }
    }
}

pub struct DisplayController<T: 'static + SupportedWord> {
    /// ltdc instance
    _ltdc: LTDC,
    /// dma2d instance
    _dma2d: DMA2D,
    /// Configuration structure
    config: DisplayConfig,
    /// layer 1 buffer
    buffer1: Option<&'static mut [T]>,
    /// layer 2 buffer
    buffer2: Option<&'static mut [T]>,
    /// Pixels format in the layers
    pixel_format: PixelFormat,
}

impl<T: 'static + SupportedWord> DisplayController<T> {
    /// Create and configure the DisplayController
    pub fn new(
        ltdc: LTDC,
        dma2d: DMA2D,
        _pins: LtdcPins,
        pixel_format: PixelFormat,
        config: DisplayConfig,
        hse: Option<Hertz>,
    ) -> DisplayController<T> {
        // Screen constants
        let total_width: u16 =
            config.h_sync + config.h_back_porch + config.active_width + config.h_front_porch - 1;
        let total_height: u16 =
            config.v_sync + config.v_back_porch + config.active_height + config.v_front_porch - 1;
        let lcd_clk: u32 =
            (total_width as u32) * (total_height as u32) * (config.frame_rate as u32);

        // TODO : change it to something safe ...
        unsafe {
            // Enable LTDC
            LTDC::enable_unchecked();
            // Reset LTDC peripheral
            LTDC::reset_unchecked();

            // Enable DMA2D
            DMA2D::enable_unchecked();
            // Reset DMA2D
            DMA2D::reset_unchecked();
        }

        // Get base clock and PLLM divisor
        let base_clk = if let Some(hse) = &hse {
            hse.raw()
        } else {
            // If no HSE is provided, we use the HSI clock at 16 MHz
            16_000_000
        };
        let rcc = unsafe { &(*RCC::ptr()) };
        let pllm: u8 = rcc.pllcfgr().read().pllm().bits();

        // There are 24 combinations possible for a divisor with PLLR and DIVR
        // We find the one that is the closest possible to the target value
        // while respecting all the conditions
        let vco_in_mhz: f32 = (base_clk as f32 / pllm as f32) / 1_000_000.0;
        let lcd_clk_mhz = (lcd_clk as f32) / 1_000_000.0;
        let allowed_pllr = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let allowed_divr = [2.0, 4.0, 8.0, 16.0];
        let mut best_pllr: f32 = allowed_pllr[0];
        let mut best_divr: f32 = allowed_divr[0];
        let mut best_plln: f32 = 100.0;
        let mut best_error: f32 = (vco_in_mhz * best_plln) / (best_pllr * best_divr);
        let mut error: f32;
        let mut plln: f32;

        for pllr in &allowed_pllr {
            for divr in &allowed_divr {
                plln = ((lcd_clk_mhz * divr * pllr) / vco_in_mhz).floor();
                error = lcd_clk_mhz - (vco_in_mhz * plln) / (pllr * divr);

                // We have to make sure that the VCO_OUT is in range [100, 432]
                // MHz Because VCO_IN is in range [1, 2] Mhz, the condition
                // PLLN in range [50, 432] is automatically satisfied
                if 100.0 <= vco_in_mhz * plln
                    && vco_in_mhz * plln <= 432.0
                    && error >= 0.0
                    && error < best_error
                {
                    best_pllr = *pllr;
                    best_divr = *divr;
                    best_plln = plln;
                    best_error = error;
                }
            }
        }

        let pllsaidivr: u8 = match best_divr as u16 {
            2 => 0b00,
            4 => 0b01,
            8 => 0b10,
            16 => 0b11,
            _ => unreachable!(),
        };

        // // Write PPLSAI configuration
        rcc.pllsaicfgr().write(|w| unsafe {
            w.pllsain().bits(best_plln as u16);
            w.pllsair().bits(best_pllr as u8)
        });
        rcc.dckcfgr().modify(|_, w| w.pllsaidivr().set(pllsaidivr));

        // Enable PLLSAI and wait for it
        rcc.cr().modify(|_, w| w.pllsaion().on());
        while rcc.cr().read().pllsairdy().is_not_ready() {}

        // Configure LTDC Timing registers
        ltdc.sscr().write(|w| {
            w.hsw().set(config.h_sync - 1);
            w.vsh().set(config.v_sync - 1)
        });
        ltdc.bpcr().write(|w| {
            w.ahbp().set(config.h_sync + config.h_back_porch - 1);
            w.avbp().set(config.v_sync + config.v_back_porch - 1)
        });
        ltdc.awcr().write(|w| {
            w.aaw()
                .set(config.h_sync + config.h_back_porch + config.active_width - 1);
            w.aah()
                .set(config.v_sync + config.v_back_porch + config.active_height - 1)
        });
        ltdc.twcr().write(|w| {
            w.totalw().set(total_width);
            w.totalh().set(total_height)
        });

        // Configure LTDC signals polarity
        ltdc.gcr().write(|w| {
            w.hspol().bit(config.h_sync_pol);
            w.vspol().bit(config.v_sync_pol);
            w.depol().bit(config.no_data_enable_pol);
            w.pcpol().bit(config.pixel_clock_pol)
        });

        // Set blue background color
        ltdc.bccr().write(|w| unsafe { w.bits(0xAAAAAAAA) });

        // TODO: configure interupts

        // Reload ltdc config immediatly
        ltdc.srcr().modify(|_, w| w.imr().set_bit());
        // Turn display ON
        ltdc.gcr()
            .modify(|_, w| w.ltdcen().set_bit().den().set_bit());

        // Reload ltdc config immediatly
        ltdc.srcr().modify(|_, w| w.imr().set_bit());

        DisplayController {
            _ltdc: ltdc,
            _dma2d: dma2d,
            config,
            buffer1: None,
            buffer2: None,
            pixel_format,
        }
    }

    /// Configure the layer
    ///
    /// Note : the choice is made (for the sake of simplicity) to make the layer
    /// as big as the screen
    ///
    /// Color Keying and CLUT are not yet supported
    pub fn config_layer(
        &mut self,
        layer: Layer,
        buffer: &'static mut [T],
        pixel_format: PixelFormat,
    ) {
        let _layer = self._ltdc.layer(layer as usize);

        let height = self.config.active_height;
        let width = self.config.active_width;
        assert!(buffer.len() == height as usize * width as usize);

        // Horizontal and vertical window (coordinates include porches): where
        // in the time frame the layer values should be sent
        let h_win_start = self.config.h_sync + self.config.h_back_porch - 1;
        let v_win_start = self.config.v_sync + self.config.v_back_porch - 1;

        _layer.whpcr().write(|w| {
            w.whstpos().set(h_win_start + 1);
            w.whsppos().set(h_win_start + width)
        });
        _layer.wvpcr().write(|w| {
            w.wvstpos().set(v_win_start + 1);
            w.wvsppos().set(v_win_start + height)
        });

        // Set pixel format
        _layer.pfcr().write(|w| {
            w.pf().set(match &pixel_format {
                PixelFormat::ARGB8888 => 0b000,
                // PixelFormat::RGB888 => 0b001,
                PixelFormat::RGB565 => 0b010,
                PixelFormat::ARGB1555 => 0b011,
                PixelFormat::ARGB4444 => 0b100,
                PixelFormat::L8 => 0b101,
                PixelFormat::AL44 => 0b110,
                PixelFormat::AL88 => 0b111,
                // _ => unimplemented!(),
            })
        });

        // Set global alpha value to 1 (255/255). Used for layer blending.
        _layer.cacr().write(|w| w.consta().set(0xFF));

        // Set default color to plain (not transparent) red (for debug
        // purposes). The default color is used outside the defined layer window
        // or when a layer is disabled.
        _layer.dccr().write(|w| unsafe { w.bits(0xFFFF0000) });

        // Blending factor: how the layer is combined with the layer below it
        // (layer 2 with layer 1 or layer 1 with background). Here it is set so
        // that the blending factor does not take the pixel alpha value, just
        // the global value of the layer
        _layer.bfcr().write(|w| w.bf1().constant().bf2().constant());

        // Color frame buffer start address
        _layer
            .cfbar()
            .write(|w| w.cfbadd().set(buffer.as_ptr() as u32));

        // Color frame buffer line length (active*byte per pixel + 3), and pitch
        let byte_per_pixel: u16 = match &pixel_format {
            PixelFormat::ARGB8888 => 4,
            // PixelFormat::RGB888 => 24, unsupported for now because u24 does not exist
            PixelFormat::RGB565 => 2,
            PixelFormat::ARGB1555 => 2,
            PixelFormat::ARGB4444 => 16,
            PixelFormat::L8 => 1,
            PixelFormat::AL44 => 1,
            PixelFormat::AL88 => 2,
            // _ => unimplemented!(),
        };
        _layer.cfblr().write(|w| {
            w.cfbp()
                .set(width * byte_per_pixel)
                .cfbll()
                .set(width * byte_per_pixel + 3)
        });

        // Frame buffer number of lines
        _layer.cfblnr().write(|w| w.cfblnbr().set(height));

        // No Color Lookup table (CLUT)
        _layer.cr().modify(|_, w| w.cluten().clear_bit());

        // Config DMA2D hardware acceleration : pixel format, no CLUT
        self._dma2d.fgpfccr().write(|w| unsafe {
            w.bits(match &pixel_format {
                PixelFormat::ARGB8888 => 0b000,
                // PixelFormat::RGB888 => 0b0001, unsupported for now because u24 does not exist
                PixelFormat::RGB565 => 0b0010,
                PixelFormat::ARGB1555 => 0b0011,
                PixelFormat::ARGB4444 => 0b0100,
                PixelFormat::L8 => 0b0101,
                PixelFormat::AL44 => 0b0110,
                PixelFormat::AL88 => 0b0111,
                // PixelFormat::L4 => 0b1000, unsupported for now
                // PixelFormat::A8 => 0b1001,
                // PixelFormat::A4 => 0b1010
                // _ => unimplemented!(),
            })
        });

        match &layer {
            Layer::L1 => self.buffer1 = Some(buffer),
            Layer::L2 => self.buffer2 = Some(buffer),
        }
    }

    /// Enable the layer
    pub fn enable_layer(&self, layer: Layer) {
        self._ltdc
            .layer(layer as usize)
            .cr()
            .modify(|_, w| w.len().set_bit());
    }

    /// Draw a pixel at position (x,y) on the given layer
    pub fn draw_pixel(&mut self, layer: Layer, x: usize, y: usize, color: T) {
        if x >= self.config.active_width as usize || y >= self.config.active_height as usize {
            panic!("Invalid (x,y) pixel position");
        }

        (match layer {
            Layer::L1 => self.buffer1.as_mut().unwrap(),
            Layer::L2 => self.buffer2.as_mut().unwrap(),
        })[x + self.config.active_width as usize * y] = color;
    }

    /// Draw hardware accelerated rectangle
    ///
    /// # Safety
    ///
    /// TODO: use safer DMA transfers
    pub unsafe fn draw_rectangle(
        &mut self,
        layer: Layer,
        top_left: (usize, usize),
        bottom_right: (usize, usize),
        color: u32,
    ) {
        // Output color format
        self._dma2d.opfccr().write(|w| {
            w.cm().bits(match &self.pixel_format {
                PixelFormat::ARGB8888 => 0b000,
                // PixelFormat::RGB888 => 0b001, unsupported for now
                PixelFormat::RGB565 => 0b010,
                PixelFormat::ARGB1555 => 0b011,
                PixelFormat::ARGB4444 => 0b100,
                _ => unreachable!(),
            })
        });

        // Output color
        self._dma2d.ocolr().write_with_zero(|w| w.bits(color));

        // Destination memory address
        let offset: isize = (top_left.0 + self.config.active_width as usize * top_left.1) as isize;
        self._dma2d.omar().write_with_zero(|w| {
            w.bits(
                (match layer {
                    Layer::L1 => self.buffer1.as_ref(),
                    Layer::L2 => self.buffer2.as_ref(),
                })
                .unwrap()
                .as_ptr()
                .offset(offset) as u32,
            )
        });

        // Pixels per line and number of lines
        self._dma2d.nlr().write(|w| {
            w.pl().bits((bottom_right.0 - top_left.0) as u16);
            w.nl().bits((bottom_right.1 - top_left.1) as u16)
        });

        // Line offset
        self._dma2d.oor().write(|w| {
            w.lo()
                .bits(top_left.0 as u16 + self.config.active_width - bottom_right.0 as u16)
        });

        // Start transfert: register to memory mode
        self._dma2d
            .cr()
            .modify(|_, w| w.mode().bits(0b11).start().set_bit());
    }

    /// Reload display controller immediatly
    pub fn reload(&self) {
        // Reload ltdc config immediatly
        self._ltdc.srcr().modify(|_, w| w.imr().set_bit());
    }
}

/// Available PixelFormats to work with
///
/// Notes :
/// * `L8`: 8-bit luminance or CLUT
/// * `AL44`: 4-bit alpha + 4-bit luminance
/// * `AL88`: 8-bit alpha + 8-bit luminance
pub enum PixelFormat {
    ARGB8888,
    // RGB888(u24) unsupported for now because u24 does not exist
    RGB565,
    ARGB1555,
    ARGB4444,
    L8,
    AL44,
    AL88,
}

pub trait SupportedWord {}
impl SupportedWord for u8 {}
impl SupportedWord for u16 {}
impl SupportedWord for u32 {}
