//! Embedded RGB565 image data for F469-DISCO display examples.
//!
//! Provides small sprites and pattern generators so demos can run without
//! an SD card or external flash.

/// RGB565 color key used for transparent pixels in overlay sprites.
#[allow(dead_code)]
pub const COLOR_KEY: u16 = 0x0000;

/// 32×32 sprite: solid red (for color-key overlay demo).
#[allow(dead_code)]
pub const SPRITE_RED_32X32: [u16; 32 * 32] = [0xF800u16; 32 * 32];

/// 32×32 sprite: solid green.
#[allow(dead_code)]
pub const SPRITE_GREEN_32X32: [u16; 32 * 32] = [0x07E0u16; 32 * 32];

/// 32×32 sprite: solid blue.
#[allow(dead_code)]
pub const SPRITE_BLUE_32X32: [u16; 32 * 32] = [0x001Fu16; 32 * 32];

/// 32×32 sprite: white center, black border (color key on black).
#[allow(dead_code)]
pub const SPRITE_WHITE_CIRCLE_32X32: [u16; 32 * 32] = {
    let mut buf = [COLOR_KEY; 32 * 32];
    let white = 0xFFFFu16;
    let mut y = 0usize;
    while y < 32 {
        let mut x = 0usize;
        while x < 32 {
            let dx = x as i32 - 16;
            let dy = y as i32 - 16;
            if dx * dx + dy * dy <= 8 * 8 {
                buf[y * 32 + x] = white;
            }
            x += 1;
        }
        y += 1;
    }
    buf
};

/// Fill a RGB565 buffer with vertical color bars (red, green, blue, white).
#[inline]
#[allow(dead_code)]
pub fn fill_color_bars(buffer: &mut [u16], width: u16, height: u16) {
    let w = width as usize;
    let h = height as usize;
    let q = h / 4;
    let colors = [0xF800u16, 0x07E0, 0x001F, 0xFFFF]; // R, G, B, W
    for y in 0..h {
        let band = (y / q).min(3);
        let c = colors[band];
        for x in 0..w {
            buffer[y * w + x] = c;
        }
    }
}

/// Fill buffer with a single RGB565 color.
#[inline]
pub fn fill_solid(buffer: &mut [u16], color: u16) {
    buffer.fill(color);
}

/// Fill buffer with gradient from top to bottom (red -> green -> blue).
#[inline]
pub fn fill_gradient(buffer: &mut [u16], width: u16, height: u16) {
    let w = width as usize;
    let h = height as usize;
    for y in 0..h {
        let t = y as u32 * 255 / h.max(1) as u32;
        let r = (255 - t).min(255) as u16;
        let g = if t <= 128 {
            (t * 2).min(255)
        } else {
            (255 - (t - 128) * 2).min(255)
        };
        let b = t.min(255) as u16;
        let r5 = (r >> 3) & 0x1F;
        let g6 = ((g as u16) >> 2) & 0x3F;
        let b5 = (b >> 3) & 0x1F;
        let c = (r5 << 11) | (g6 << 5) | b5;
        for x in 0..w {
            buffer[y * w + x] = c;
        }
    }
}

/// Pattern selector for slideshow-style demos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SlidePattern {
    ColorBars,
    SolidRed,
    SolidGreen,
    SolidBlue,
    Gradient,
}

impl SlidePattern {
    /// Fill the given buffer with this pattern.
    #[allow(dead_code)]
    pub fn fill(&self, buffer: &mut [u16], width: u16, height: u16) {
        match self {
            SlidePattern::ColorBars => fill_color_bars(buffer, width, height),
            SlidePattern::SolidRed => fill_solid(buffer, 0xF800),
            SlidePattern::SolidGreen => fill_solid(buffer, 0x07E0),
            SlidePattern::SolidBlue => fill_solid(buffer, 0x001F),
            SlidePattern::Gradient => fill_gradient(buffer, width, height),
        }
    }

    /// All patterns in order for cycling.
    #[allow(dead_code)]
    pub const ALL: [SlidePattern; 5] = [
        SlidePattern::ColorBars,
        SlidePattern::SolidRed,
        SlidePattern::SolidGreen,
        SlidePattern::SolidBlue,
        SlidePattern::Gradient,
    ];
}
