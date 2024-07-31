use embedded_graphics::{
    pixelcolor::{Rgb565, RgbColor},
    prelude::{DrawTarget, OriginDimensions, Pixel, Size},
};

use stm32f4xx_hal::{
    ltdc::{DisplayConfig, DisplayController, Layer, LtdcPins, PixelFormat, SupportedWord},
    pac::{DMA2D, LTDC},
    prelude::*,
};

/// STM32F7-DISCO board display
pub const DISCO_SCREEN_CONFIG: DisplayConfig = DisplayConfig {
    active_width: 480,
    active_height: 272,
    h_back_porch: 13,
    h_front_porch: 30,
    h_sync: 41,
    v_back_porch: 2,
    v_front_porch: 2,
    v_sync: 10,
    frame_rate: 60,
    h_sync_pol: false,
    v_sync_pol: false,
    no_data_enable_pol: false,
    pixel_clock_pol: false,
};

pub struct Stm32F7DiscoDisplay<T: 'static + SupportedWord> {
    pub controller: DisplayController<T>,
}

impl<T: 'static + SupportedWord> Stm32F7DiscoDisplay<T> {
    pub fn new(ltdc: LTDC, dma2d: DMA2D, pins: LtdcPins) -> Stm32F7DiscoDisplay<T> {
        let controller = DisplayController::new(
            ltdc,
            dma2d,
            Some(pins),
            PixelFormat::RGB565,
            DISCO_SCREEN_CONFIG,
            Some(25.MHz()),
        );

        Stm32F7DiscoDisplay { controller }
    }
}

impl DrawTarget for Stm32F7DiscoDisplay<u16> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (480,272)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let (x @ 0..=479, y @ 0..=271) = coord.into() {
                let value: u16 = (color.b() as u16 & 0x1F)
                    | ((color.g() as u16 & 0x3F) << 5)
                    | ((color.r() as u16 & 0x1F) << 11);

                self.controller
                    .draw_pixel(Layer::L1, x as usize, y as usize, value);
            }
        }

        Ok(())
    }
}

impl OriginDimensions for Stm32F7DiscoDisplay<u16> {
    fn size(&self) -> Size {
        Size::new(480, 272)
    }
}
