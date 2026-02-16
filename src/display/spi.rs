//! SPI display transport
//!
//! Provides [`SpiDisplay`], a transport layer that implements the
//! [`WriteOnlyDataCommand`](display_interface::WriteOnlyDataCommand) trait from the
//! [`display-interface`](https://crates.io/crates/display-interface) crate.
//!
//! A Data/Command (DC) output pin is used to distinguish command bytes
//! (DC low) from data bytes (DC high).
//!
//! # Example
//!
//! ```rust,ignore
//! use stm32f4xx_hal::display::SpiDisplay;
//!
//! // `spi_device` implements `embedded_hal::spi::SpiDevice`
//! // `dc` implements `embedded_hal::digital::OutputPin`
//! let mut display = SpiDisplay::new(spi_device, dc);
//! ```

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;

/// SPI display transport with a Data/Command (DC) pin.
///
/// Implements [`display_interface::WriteOnlyDataCommand`] so it can be passed
/// directly to display driver crates such as `st7789`, `ili9341`, or `ssd1306`.
///
/// The DC pin selects the transfer mode:
/// - **Low** → command mode
/// - **High** → data mode
pub struct SpiDisplay<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC> SpiDisplay<SPI, DC> {
    /// Create a new SPI display transport.
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }

    /// Consume the transport and return the SPI device and DC pin.
    pub fn release(self) -> (SPI, DC) {
        (self.spi, self.dc)
    }
}

/// Size of the stack buffer used when converting `u16` data to bytes.
const BUF_SIZE: usize = 64;

// Generates both a private `send_*` helper and the `WriteOnlyDataCommand` impl
// for a given `display-interface` version. The two crate versions expose
// identical APIs but define distinct types, so the macro avoids duplication.
macro_rules! impl_display_interface {
    ($display_interface:ident) => {
        impl<SPI, DC> $display_interface::WriteOnlyDataCommand for SpiDisplay<SPI, DC>
        where
            SPI: SpiDevice,
            DC: OutputPin,
        {
            fn send_commands(
                &mut self,
                cmd: $display_interface::DataFormat<'_>,
            ) -> Result<(), $display_interface::DisplayError> {
                self.dc
                    .set_low()
                    .map_err(|_| $display_interface::DisplayError::DCError)?;
                send_spi_data(&mut self.spi, cmd)
            }

            fn send_data(
                &mut self,
                buf: $display_interface::DataFormat<'_>,
            ) -> Result<(), $display_interface::DisplayError> {
                self.dc
                    .set_high()
                    .map_err(|_| $display_interface::DisplayError::DCError)?;
                send_spi_data(&mut self.spi, buf)
            }
        }

        fn send_spi_data<SPI: SpiDevice>(
            spi: &mut SPI,
            data: $display_interface::DataFormat<'_>,
        ) -> Result<(), $display_interface::DisplayError> {
            use $display_interface::DataFormat;

            match data {
                DataFormat::U8(buf) => spi
                    .write(buf)
                    .map_err(|_| $display_interface::DisplayError::BusWriteError),

                // U16 is defined by display-interface as "same endianness as the
                // system" – native byte order is therefore the correct encoding.
                DataFormat::U16(buf) => {
                    let mut byte_buf = [0u8; BUF_SIZE];
                    for chunk in buf.chunks(BUF_SIZE / 2) {
                        for (i, v) in chunk.iter().enumerate() {
                            let bytes = v.to_ne_bytes();
                            byte_buf[i * 2] = bytes[0];
                            byte_buf[i * 2 + 1] = bytes[1];
                        }
                        spi.write(&byte_buf[..chunk.len() * 2])
                            .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                    }
                    Ok(())
                }

                DataFormat::U16BE(buf) => {
                    let mut byte_buf = [0u8; BUF_SIZE];
                    for chunk in buf.chunks(BUF_SIZE / 2) {
                        for (i, v) in chunk.iter().enumerate() {
                            let bytes = v.to_be_bytes();
                            byte_buf[i * 2] = bytes[0];
                            byte_buf[i * 2 + 1] = bytes[1];
                        }
                        spi.write(&byte_buf[..chunk.len() * 2])
                            .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                    }
                    Ok(())
                }

                DataFormat::U16LE(buf) => {
                    let mut byte_buf = [0u8; BUF_SIZE];
                    for chunk in buf.chunks(BUF_SIZE / 2) {
                        for (i, v) in chunk.iter().enumerate() {
                            let bytes = v.to_le_bytes();
                            byte_buf[i * 2] = bytes[0];
                            byte_buf[i * 2 + 1] = bytes[1];
                        }
                        spi.write(&byte_buf[..chunk.len() * 2])
                            .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                    }
                    Ok(())
                }

                DataFormat::U8Iter(iter) => {
                    let mut byte_buf = [0u8; BUF_SIZE];
                    let mut i = 0;
                    for v in iter {
                        byte_buf[i] = v;
                        i += 1;
                        if i == BUF_SIZE {
                            spi.write(&byte_buf)
                                .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                            i = 0;
                        }
                    }
                    if i > 0 {
                        spi.write(&byte_buf[..i])
                            .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                    }
                    Ok(())
                }

                DataFormat::U16BEIter(iter) => {
                    let mut byte_buf = [0u8; BUF_SIZE];
                    let mut i = 0;
                    for v in iter {
                        if i + 2 > BUF_SIZE {
                            spi.write(&byte_buf[..i])
                                .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                            i = 0;
                        }
                        let bytes = v.to_be_bytes();
                        byte_buf[i] = bytes[0];
                        byte_buf[i + 1] = bytes[1];
                        i += 2;
                    }
                    if i > 0 {
                        spi.write(&byte_buf[..i])
                            .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                    }
                    Ok(())
                }

                DataFormat::U16LEIter(iter) => {
                    let mut byte_buf = [0u8; BUF_SIZE];
                    let mut i = 0;
                    for v in iter {
                        if i + 2 > BUF_SIZE {
                            spi.write(&byte_buf[..i])
                                .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                            i = 0;
                        }
                        let bytes = v.to_le_bytes();
                        byte_buf[i] = bytes[0];
                        byte_buf[i + 1] = bytes[1];
                        i += 2;
                    }
                    if i > 0 {
                        spi.write(&byte_buf[..i])
                            .map_err(|_| $display_interface::DisplayError::BusWriteError)?;
                    }
                    Ok(())
                }

                _ => Err($display_interface::DisplayError::DataFormatNotImplemented),
            }
        }
    };
}

// --- display-interface 0.5 (current) ----------------------------------------
mod di_05 {
    use super::*;
    impl_display_interface!(display_interface);
}

// --- display-interface 0.4 (legacy) -----------------------------------------
mod di_04 {
    use super::*;
    impl_display_interface!(display_interface_04);
}
