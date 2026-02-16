//! LTDC framebuffer abstraction with `embedded-graphics` support
//!
//! Provides [`LtdcFramebuffer`], a thin wrapper around a memory-mapped pixel
//! buffer that implements [`embedded_graphics_core::draw_target::DrawTarget`]
//! for `Rgb565` colour format.
//!
//! # Example
//!
//! ```rust,ignore
//! use stm32f4xx_hal::display::LtdcFramebuffer;
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::primitives::{Circle, PrimitiveStyle};
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // `fb` is a &'static mut [u16] backed by SDRAM
//! let mut display = LtdcFramebuffer::new(fb, 480, 800);
//!
//! Circle::new(Point::new(100, 100), 50)
//!     .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
//!     .draw(&mut display)
//!     .unwrap();
//! ```

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb565},
    primitives::Rectangle,
    Pixel,
};

/// A framebuffer-backed draw target for LTDC displays.
///
/// Wraps an SDRAM (or SRAM) backed `&'static mut [u16]` buffer and
/// presents it as an [`embedded_graphics_core::draw_target::DrawTarget`]
/// using the [`Rgb565`] pixel format (16-bit, 5-6-5 layout).
///
/// The framebuffer is expected to be configured as LTDC layer 1 via
/// [`DisplayController::config_layer`](crate::ltdc::DisplayController::config_layer).
///
/// # Double buffering
///
/// This abstraction manages a **single** framebuffer. True double-buffering
/// (swapping between two SDRAM regions to avoid tearing) requires:
///
/// 1. Two framebuffer regions in SDRAM
/// 2. Swapping the LTDC layer base address during vertical blanking
/// 3. Coordinating writes to the non-displayed buffer
///
/// ```rust,ignore
/// // Allocate two buffers
/// let fb0 = unsafe { slice::from_raw_parts_mut(sdram_ptr, FB_SIZE) };
/// let fb1 = unsafe { slice::from_raw_parts_mut(sdram_ptr.add(FB_SIZE), FB_SIZE) };
///
/// // Draw into the back buffer, then swap LTDC layer address during VBlank
/// // LTDC.layer[x].CFBAR = fb0.as_ptr() or fb1.as_ptr()
/// ```
pub struct LtdcFramebuffer {
    buffer: &'static mut [u16],
    width: u16,
    height: u16,
}

impl LtdcFramebuffer {
    /// Create a new framebuffer draw target.
    ///
    /// # Panics
    ///
    /// Panics if `buffer.len() != width as usize * height as usize`.
    pub fn new(buffer: &'static mut [u16], width: u16, height: u16) -> Self {
        assert_eq!(
            buffer.len(),
            width as usize * height as usize,
            "framebuffer size mismatch"
        );
        Self {
            buffer,
            width,
            height,
        }
    }

    /// Return the active display width in pixels.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Return the active display height in pixels.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get a raw pointer to the start of the framebuffer.
    ///
    /// This is useful when configuring the LTDC layer via
    /// [`DisplayController::config_layer`](crate::ltdc::DisplayController::config_layer).
    pub fn as_ptr(&self) -> *const u16 {
        self.buffer.as_ptr()
    }

    /// Borrow the underlying pixel buffer.
    pub fn as_slice(&self) -> &[u16] {
        self.buffer
    }

    /// Mutably borrow the underlying pixel buffer.
    pub fn as_mut_slice(&mut self) -> &mut [u16] {
        self.buffer
    }
}

impl DrawTarget for LtdcFramebuffer {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let w = self.width as i32;
        let h = self.height as i32;

        for Pixel(coord, color) in pixels {
            let x = coord.x;
            let y = coord.y;
            if x >= 0 && x < w && y >= 0 && y < h {
                self.buffer[x as usize + self.width as usize * y as usize] = color.into_storage();
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let value = color.into_storage();
        let w = self.width as i32;
        let h = self.height as i32;

        // Clamp the rectangle to the display bounds
        let x_start = area.top_left.x.max(0).min(w) as usize;
        let y_start = area.top_left.y.max(0).min(h) as usize;
        let x_end = (area.top_left.x + area.size.width as i32).max(0).min(w) as usize;
        let y_end = (area.top_left.y + area.size.height as i32).max(0).min(h) as usize;

        let stride = self.width as usize;
        for y in y_start..y_end {
            let row_start = y * stride + x_start;
            let row_end = y * stride + x_end;
            self.buffer[row_start..row_end].fill(value);
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buffer.fill(color.into_storage());
        Ok(())
    }
}

impl OriginDimensions for LtdcFramebuffer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}
