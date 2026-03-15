#![cfg_attr(not(test), no_std)]
//! Standalone `no_std` driver for NT35510 DSI LCD controller panels.
//!
//! The default configuration is tested on STM32F469I-DISCO:
//! portrait mode, RGB565, 480x800.
//! Landscape mode uses MADCTL rotation matching the OTM8009A pattern,
//! but is currently untested.

mod regs;

pub use regs::*;

use embedded_display_controller::dsi::{DsiHostCtrlIo, DsiReadCommand, DsiWriteCommand};
use embedded_hal::delay::DelayNs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    DsiRead,
    DsiWrite,
    ProbeMismatch(u8),
    InvalidDimensions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Portrait orientation (480x800). Tested on STM32F469I-DISCO.
    Portrait,
    /// Landscape orientation (800x480). Untested.
    Landscape,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorFormat {
    /// 16-bit RGB565. Tested on STM32F469I-DISCO.
    Rgb565,
    /// 24-bit RGB888. Tested on STM32F469I-DISCO.
    Rgb888,
}

/// Configuration for the NT35510 panel.
///
/// Default values match the STM32F469I-DISCO board configuration
/// (portrait mode, RGB565, 480x800).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nt35510Config {
    pub mode: Mode,
    pub color_format: ColorFormat,
    /// Display width in pixels (before rotation).
    pub cols: u16,
    /// Display height in pixels (before rotation).
    pub rows: u16,
}

impl Default for Nt35510Config {
    fn default() -> Self {
        Self {
            mode: Mode::Portrait,
            color_format: ColorFormat::Rgb565,
            cols: 480,
            rows: 800,
        }
    }
}

pub struct Nt35510 {
    initialized: bool,
}

impl Default for Nt35510 {
    fn default() -> Self {
        Self::new()
    }
}

impl Nt35510 {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    /// Probe whether an NT35510 is connected by reading its ID registers.
    ///
    /// Returns `Ok(())` if the panel responds with expected NT35510 IDs.
    /// Returns `Err(Error::ProbeMismatch(id))` if a different panel responds.
    /// Returns `Err(Error::DsiRead)` if DSI reads fail entirely.
    pub fn probe<D: DelayNs>(
        &mut self,
        dsi_host: &mut impl DsiHostCtrlIo,
        _delay: &mut D,
    ) -> Result<(), Error> {
        match self.read_id(dsi_host, NT35510_CMD_RDID2) {
            Ok(id) if id == NT35510_ID2_EXPECTED => return Ok(()),
            Ok(id) => return Err(Error::ProbeMismatch(id)),
            Err(_) => {}
        }

        match self.read_id(dsi_host, NT35510_CMD_RDID1) {
            Ok(id) if id == NT35510_ID1_EXPECTED => Ok(()),
            Ok(id) => Err(Error::ProbeMismatch(id)),
            Err(_) => Err(Error::DsiRead),
        }
    }

    /// Check if an NT35510 panel is connected by reading ID registers.
    /// Returns `Ok(true)` if NT35510 is detected and `Ok(false)` otherwise.
    pub fn id_matches(&mut self, dsi_host: &mut impl DsiHostCtrlIo) -> Result<bool, Error> {
        if let Ok(id) = self.read_id(dsi_host, NT35510_CMD_RDID2) {
            return Ok(id == NT35510_ID2_EXPECTED);
        }

        match self.read_id(dsi_host, NT35510_CMD_RDID1) {
            Ok(id) => Ok(id == NT35510_ID1_EXPECTED),
            Err(_) => Err(Error::DsiRead),
        }
    }

    /// Initialize the panel in RGB888 (24-bit) mode.
    pub fn init<D: DelayNs>(
        &mut self,
        dsi_host: &mut impl DsiHostCtrlIo,
        delay: &mut D,
    ) -> Result<(), Error> {
        let config = Nt35510Config {
            color_format: ColorFormat::Rgb888,
            ..Nt35510Config::default()
        };
        self.init_with_config(dsi_host, delay, config)
    }

    /// Initialize the panel in RGB565 (16-bit) mode.
    pub fn init_rgb565<D: DelayNs>(
        &mut self,
        dsi_host: &mut impl DsiHostCtrlIo,
        delay: &mut D,
    ) -> Result<(), Error> {
        self.init_with_config(dsi_host, delay, Nt35510Config::default())
    }

    /// Initialize the panel with an explicit configuration.
    pub fn init_with_config<D: DelayNs>(
        &mut self,
        dsi_host: &mut impl DsiHostCtrlIo,
        delay: &mut D,
        config: Nt35510Config,
    ) -> Result<(), Error> {
        if self.initialized {
            return Ok(());
        }

        if config.cols == 0 || config.rows == 0 {
            return Err(Error::InvalidDimensions);
        }

        self.write_reg(
            dsi_host,
            NT35510_CMD_SETEXTC,
            &[0x55, 0xAA, 0x52, 0x08, 0x01],
        )?;
        self.write_reg(dsi_host, NT35510_CMD_B0, &[0x03, 0x03, 0x03])?;
        self.write_reg(dsi_host, NT35510_CMD_B6, &[0x46, 0x46, 0x46])?;
        self.write_reg(dsi_host, NT35510_CMD_B1, &[0x03, 0x03, 0x03])?;
        self.write_reg(dsi_host, NT35510_CMD_B7, &[0x36, 0x36, 0x36])?;
        self.write_reg(dsi_host, NT35510_CMD_B2, &[0x00, 0x00, 0x02])?;
        self.write_reg(dsi_host, NT35510_CMD_B8, &[0x26, 0x26, 0x26])?;
        self.write_reg(dsi_host, NT35510_CMD_BF, &[0x01])?;
        self.write_reg(dsi_host, NT35510_CMD_B3, &[0x09, 0x09, 0x09])?;
        self.write_reg(dsi_host, NT35510_CMD_B9, &[0x36, 0x36, 0x36])?;
        self.write_reg(dsi_host, NT35510_CMD_B5, &[0x08, 0x08, 0x08])?;
        self.write_reg(dsi_host, NT35510_CMD_BA, &[0x26, 0x26, 0x26])?;
        self.write_reg(dsi_host, NT35510_CMD_BC, &[0x00, 0x80, 0x00])?;
        self.write_reg(dsi_host, NT35510_CMD_BD, &[0x00, 0x80, 0x00])?;
        self.write_reg(dsi_host, NT35510_CMD_BE, &[0x00, 0x50])?;

        self.write_reg(
            dsi_host,
            NT35510_CMD_SETEXTC,
            &[0x55, 0xAA, 0x52, 0x08, 0x00],
        )?;
        self.write_reg(dsi_host, NT35510_CMD_B1, &[0xFC, 0x00])?;
        self.write_reg(dsi_host, NT35510_CMD_B6, &[0x03, 0x03])?;
        self.write_reg(dsi_host, NT35510_CMD_B5, &[0x50, 0x50])?;
        self.write_reg(dsi_host, NT35510_CMD_B7, &[0x00, 0x00])?;
        self.write_reg(dsi_host, NT35510_CMD_B8, &[0x01, 0x02, 0x02, 0x02])?;
        self.write_reg(dsi_host, NT35510_CMD_BC, &[0x00, 0x00, 0x00])?;
        self.write_reg(dsi_host, NT35510_CMD_CC, &[0x03, 0x00, 0x00])?;
        self.write_reg(dsi_host, NT35510_CMD_BA, &[0x01, 0x01])?;

        let colmod = match config.color_format {
            ColorFormat::Rgb565 => NT35510_COLMOD_RGB565,
            ColorFormat::Rgb888 => NT35510_COLMOD_RGB888,
        };
        let madctl = match config.mode {
            Mode::Portrait => NT35510_MADCTL_PORTRAIT,
            Mode::Landscape => NT35510_MADCTL_LANDSCAPE,
        };

        let last_col = (config.cols - 1).to_be_bytes();
        let last_row = (config.rows - 1).to_be_bytes();
        let caset = [0x00, 0x00, last_col[0], last_col[1]];
        let raset = [0x00, 0x00, last_row[0], last_row[1]];

        delay.delay_us(200_000);
        self.write_cmd(dsi_host, NT35510_CMD_SLPOUT, 0x00)?;
        delay.delay_us(120_000);
        self.write_cmd(dsi_host, NT35510_CMD_COLMOD, colmod)?;
        self.write_cmd(dsi_host, NT35510_CMD_MADCTL, madctl)?;
        self.write_reg(dsi_host, NT35510_CMD_CASET, &caset)?;
        self.write_reg(dsi_host, NT35510_CMD_RASET, &raset)?;
        self.write_cmd(dsi_host, NT35510_CMD_WRDISBV, 0x7F)?;
        self.write_cmd(dsi_host, NT35510_CMD_WRCTRLD, 0x2C)?;
        self.write_cmd(dsi_host, NT35510_CMD_WRCABC, 0x00)?;
        delay.delay_us(10_000);
        self.write_cmd(dsi_host, NT35510_CMD_DISPON, 0x00)?;
        delay.delay_us(10_000);
        self.write_cmd(dsi_host, NT35510_CMD_RAMWR, 0x00)?;

        self.initialized = true;
        Ok(())
    }

    fn read_id(&self, dsi_host: &mut impl DsiHostCtrlIo, cmd: u8) -> Result<u8, Error> {
        let mut id = [0u8; 1];
        dsi_host
            .read(DsiReadCommand::DcsShort { arg: cmd }, &mut id)
            .map_err(|_| Error::DsiRead)?;
        Ok(id[0])
    }

    fn write_cmd(
        &self,
        dsi_host: &mut impl DsiHostCtrlIo,
        cmd: u8,
        param: u8,
    ) -> Result<(), Error> {
        dsi_host
            .write(DsiWriteCommand::DcsShortP1 {
                arg: cmd,
                data: param,
            })
            .map_err(|_| Error::DsiWrite)
    }

    fn write_reg(
        &self,
        dsi_host: &mut impl DsiHostCtrlIo,
        reg: u8,
        data: &[u8],
    ) -> Result<(), Error> {
        if data.is_empty() {
            self.write_cmd(dsi_host, reg, 0)
        } else if data.len() == 1 {
            self.write_cmd(dsi_host, reg, data[0])
        } else {
            dsi_host
                .write(DsiWriteCommand::DcsLongWrite { arg: reg, data })
                .map_err(|_| Error::DsiWrite)
        }
    }
}
