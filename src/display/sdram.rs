//! SDRAM-backed display framebuffer helpers
//!
//! Provides [`DisplaySdram`], a helper that wraps an FMC-initialised SDRAM
//! region and returns a framebuffer slice suitable for use with
//! [`LtdcFramebuffer`](super::LtdcFramebuffer) or
//! [`DisplayController::config_layer`](crate::ltdc::DisplayController::config_layer).
//!
//! # Example
//!
//! ```rust,ignore
//! use stm32f4xx_hal::display::sdram::DisplaySdram;
//!
//! // `sdram_ptr` obtained from `Sdram::init()`
//! let mut fb = unsafe { DisplaySdram::from_raw(sdram_ptr, 480, 800) };
//! unsafe { fb.clear() };
//! let buf: &'static mut [u16] = unsafe { fb.into_rgb565_buffer() };
//! ```

use core::slice;

/// A display framebuffer residing in external SDRAM.
///
/// This is a thin convenience wrapper that encapsulates the common steps for
/// carving a correctly-sized pixel buffer out of an SDRAM region initialised
/// via the FMC controller.
///
/// The struct itself is zero-cost at runtime — it simply stores the pointer
/// and dimensions so that callers do not need to repeat the arithmetic.
pub struct DisplaySdram {
    ptr: *mut u16,
    width: u16,
    height: u16,
}

impl DisplaySdram {
    /// Create a `DisplaySdram` from a raw SDRAM pointer.
    ///
    /// `ptr` must point to at least `width * height * 2` bytes of valid,
    /// initialised SDRAM memory (Rgb565 = 2 bytes per pixel).
    ///
    /// # Safety
    ///
    /// - `ptr` must be non-null and properly aligned for `u16`.
    /// - The SDRAM region must have been initialised before calling this
    ///   function (e.g. via [`stm32_fmc::Sdram::init`]).
    /// - The caller must ensure exclusive access to the memory region for
    ///   the lifetime of the returned `DisplaySdram`.
    pub unsafe fn from_raw(ptr: *mut u16, width: u16, height: u16) -> Self {
        assert!(!ptr.is_null(), "SDRAM pointer must not be null");
        Self { ptr, width, height }
    }

    /// Create a `DisplaySdram` from a `*mut u32` (as returned by
    /// [`stm32_fmc::Sdram::init`]).
    ///
    /// This is a convenience wrapper that casts the 32-bit pointer to
    /// `*mut u16` for Rgb565 usage.
    ///
    /// # Safety
    ///
    /// Same requirements as [`Self::from_raw`].
    pub unsafe fn from_raw_u32(ptr: *mut u32, width: u16, height: u16) -> Self {
        Self::from_raw(ptr as *mut u16, width, height)
    }

    /// Return the display width in pixels.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Return the display height in pixels.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Return the framebuffer as a `&'static mut [u16]` slice sized to
    /// `width × height` pixels in Rgb565 format.
    ///
    /// This method **consumes** `self` to prevent creating multiple mutable
    /// references to the same SDRAM region.
    ///
    /// # Safety
    ///
    /// This creates a `'static` mutable reference to the SDRAM region.
    /// The caller must guarantee that no other reference to the same
    /// memory exists for the `'static` lifetime.
    pub unsafe fn into_rgb565_buffer(self) -> &'static mut [u16] {
        let len = self.width as usize * self.height as usize;
        slice::from_raw_parts_mut(self.ptr, len)
    }

    /// Clear the framebuffer to black (all zeros).
    ///
    /// # Safety
    ///
    /// The SDRAM region must be initialised and exclusively accessible.
    /// This must not be called after [`Self::into_rgb565_buffer`] has been
    /// used to create a mutable reference to the same memory.
    pub unsafe fn clear(&mut self) {
        let len = self.width as usize * self.height as usize;
        let buf = slice::from_raw_parts_mut(self.ptr, len);
        buf.fill(0);
    }
}
