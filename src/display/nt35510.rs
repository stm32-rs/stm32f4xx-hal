//! NT35510 DSI LCD controller driver.
//!
//! Supports the NT35510 panel found on STM32F469I-DISCO B08 revision boards.
//! Provides initialization for both RGB888 and RGB565 pixel modes, plus
//! runtime detection via DSI register reads.
// Based on work by Stepan Snigirev (diybitcoinhardware/f469-disco, MIT)

use crate::dsi::DsiHost;
use embedded_display_controller::dsi::{DsiHostCtrlIo, DsiReadCommand, DsiWriteCommand};
use embedded_hal_02::blocking::delay::DelayUs;

const CMD_SLPOUT: u8 = 0x11;
const CMD_DISPON: u8 = 0x29;
const CMD_RAMWR: u8 = 0x2C;
const CMD_CASET: u8 = 0x2A;
const CMD_RASET: u8 = 0x2B;
const CMD_MADCTL: u8 = 0x36;
const CMD_COLMOD: u8 = 0x3A;
const CMD_WRDISBV: u8 = 0x51;
const CMD_WRCTRLD: u8 = 0x53;
const CMD_WRCABC: u8 = 0x55;
const COLMOD_RGB888: u8 = 0x77;
const COLMOD_RGB565: u8 = 0x55;

const CMD_RDID1: u8 = 0xDA;
const CMD_RDID2: u8 = 0xDB;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    DsiRead,
    DsiWrite,
    ProbeMismatch(u8),
}

pub struct Nt35510 {
    initialized: bool,
}

impl Nt35510 {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Probe whether an NT35510 is connected by reading its ID registers.
    ///
    /// Returns `Ok(())` if the panel responds with expected NT35510 IDs.
    /// Returns `Err(Error::ProbeMismatch(id))` if a different panel responds.
    /// Returns `Err(Error::DsiRead)` if DSI reads fail entirely.
    pub fn probe<D: DelayUs<u32>>(
        &mut self,
        dsi_host: &mut DsiHost,
        _delay: &mut D,
    ) -> Result<(), Error> {
        let mut id2 = [0u8; 1];
        match dsi_host.read(DsiReadCommand::DcsShort { arg: CMD_RDID2 }, &mut id2) {
            Ok(_) if id2[0] == 0x80 => {
                #[cfg(feature = "defmt")]
                defmt::info!("NT35510 detected - RDID2=0x80");
                return Ok(());
            }
            Ok(_) => {
                #[cfg(feature = "defmt")]
                defmt::info!("RDID2 returned 0x{:02x}, not NT35510", id2[0]);
                return Err(Error::ProbeMismatch(id2[0]));
            }
            Err(_) => {}
        }

        let mut id1 = [0u8; 1];
        match dsi_host.read(DsiReadCommand::DcsShort { arg: CMD_RDID1 }, &mut id1) {
            Ok(_) if id1[0] == 0x00 => {
                #[cfg(feature = "defmt")]
                defmt::info!("NT35510 detected - RDID1=0x00");
                Ok(())
            }
            Ok(_) => {
                #[cfg(feature = "defmt")]
                defmt::info!("RDID1 returned 0x{:02x}, not NT35510", id1[0]);
                Err(Error::ProbeMismatch(id1[0]))
            }
            Err(_) => Err(Error::DsiRead),
        }
    }

    /// Initialize the panel in RGB888 (24-bit) mode.
    pub fn init<D: DelayUs<u32>>(
        &mut self,
        dsi_host: &mut DsiHost,
        delay: &mut D,
    ) -> Result<(), Error> {
        self.init_panel(dsi_host, delay, COLMOD_RGB888)
    }

    /// Initialize the panel in RGB565 (16-bit) mode.
    pub fn init_rgb565<D: DelayUs<u32>>(
        &mut self,
        dsi_host: &mut DsiHost,
        delay: &mut D,
    ) -> Result<(), Error> {
        self.init_panel(dsi_host, delay, COLMOD_RGB565)
    }

    fn init_panel<D: DelayUs<u32>>(
        &mut self,
        dsi_host: &mut DsiHost,
        delay: &mut D,
        colmod: u8,
    ) -> Result<(), Error> {
        if self.initialized {
            #[cfg(feature = "defmt")]
            defmt::warn!("NT35510 already initialized, skipping re-initialization");
            return Ok(());
        }

        // Page 1 power settings
        self.write_reg(dsi_host, 0xF0, &[0x55, 0xAA, 0x52, 0x08, 0x01])?;
        self.write_reg(dsi_host, 0xB0, &[0x03, 0x03, 0x03])?;
        self.write_reg(dsi_host, 0xB6, &[0x46, 0x46, 0x46])?;
        self.write_reg(dsi_host, 0xB1, &[0x03, 0x03, 0x03])?;
        self.write_reg(dsi_host, 0xB7, &[0x36, 0x36, 0x36])?;
        self.write_reg(dsi_host, 0xB2, &[0x00, 0x00, 0x02])?;
        self.write_reg(dsi_host, 0xB8, &[0x26, 0x26, 0x26])?;
        self.write_reg(dsi_host, 0xBF, &[0x01])?;
        self.write_reg(dsi_host, 0xB3, &[0x09, 0x09, 0x09])?;
        self.write_reg(dsi_host, 0xB9, &[0x36, 0x36, 0x36])?;
        self.write_reg(dsi_host, 0xB5, &[0x08, 0x08, 0x08])?;
        self.write_reg(dsi_host, 0xBA, &[0x26, 0x26, 0x26])?;
        self.write_reg(dsi_host, 0xBC, &[0x00, 0x80, 0x00])?;
        self.write_reg(dsi_host, 0xBD, &[0x00, 0x80, 0x00])?;
        self.write_reg(dsi_host, 0xBE, &[0x00, 0x50])?;

        // Page 0 display settings
        self.write_reg(dsi_host, 0xF0, &[0x55, 0xAA, 0x52, 0x08, 0x00])?;
        self.write_reg(dsi_host, 0xB1, &[0xFC, 0x00])?;
        self.write_reg(dsi_host, 0xB6, &[0x03, 0x03])?;
        self.write_reg(dsi_host, 0xB5, &[0x50, 0x50])?;
        self.write_reg(dsi_host, 0xB7, &[0x00, 0x00])?;
        self.write_reg(dsi_host, 0xB8, &[0x01, 0x02, 0x02, 0x02])?;
        self.write_reg(dsi_host, 0xBC, &[0x00, 0x00, 0x00])?;
        self.write_reg(dsi_host, 0xCC, &[0x03, 0x00, 0x00])?;
        self.write_reg(dsi_host, 0xBA, &[0x01, 0x01])?;

        delay.delay_us(200_000);
        self.write_cmd(dsi_host, CMD_SLPOUT, 0x00)?;
        delay.delay_us(120_000);
        self.write_cmd(dsi_host, CMD_COLMOD, colmod)?;
        self.write_cmd(dsi_host, CMD_MADCTL, 0x00)?;
        self.write_reg(dsi_host, CMD_CASET, &[0x00, 0x00, 0x01, 0xDF])?;
        self.write_reg(dsi_host, CMD_RASET, &[0x00, 0x00, 0x03, 0x1F])?;
        self.write_cmd(dsi_host, CMD_WRDISBV, 0x7F)?;
        self.write_cmd(dsi_host, CMD_WRCTRLD, 0x2C)?;
        self.write_cmd(dsi_host, CMD_WRCABC, 0x00)?;
        delay.delay_us(10_000);
        self.write_cmd(dsi_host, CMD_DISPON, 0x00)?;
        delay.delay_us(10_000);
        self.write_cmd(dsi_host, CMD_RAMWR, 0x00)?;

        self.initialized = true;
        Ok(())
    }

    fn write_cmd(&self, dsi_host: &mut DsiHost, cmd: u8, param: u8) -> Result<(), Error> {
        dsi_host
            .write(DsiWriteCommand::DcsShortP1 {
                arg: cmd,
                data: param,
            })
            .map_err(|_| Error::DsiWrite)
    }

    fn write_reg(&self, dsi_host: &mut DsiHost, reg: u8, data: &[u8]) -> Result<(), Error> {
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
