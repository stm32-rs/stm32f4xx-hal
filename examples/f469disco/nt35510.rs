//! NT35510 DSI LCD controller support for STM32F469I-DISCO B08 boards.

use core::result::Result;
use embedded_display_controller::dsi::{DsiHostCtrlIo, DsiReadCommand, DsiWriteCommand};
use embedded_hal_02::blocking::delay::DelayUs;
use stm32f4xx_hal::dsi::DsiHost;

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

// Used only for runtime probing; unused when `nt35510-only` or `otm8009a-only` features are enabled
#[allow(dead_code)]
const CMD_RDID1: u8 = 0xDA;
#[allow(dead_code)]
const CMD_RDID2: u8 = 0xDB;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nt35510Error {
    // Used only in probe(); unused when `nt35510-only` or `otm8009a-only` features skip runtime detection
    #[allow(dead_code)]
    DsiRead,
    DsiWrite,
    // Used only in probe(); unused when `nt35510-only` or `otm8009a-only` features skip runtime detection
    #[allow(dead_code)]
    ProbeMismatch(u8),
}

pub struct Nt35510 {
    initialized: bool,
}

impl Nt35510 {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    // Used only for runtime detection; unused when `nt35510-only` or `otm8009a-only` features are enabled
    #[allow(dead_code)]
    pub fn probe<D: DelayUs<u32>>(
        &mut self,
        dsi_host: &mut DsiHost,
        _delay: &mut D, // Unused for this controller; present for API consistency
    ) -> Result<(), Nt35510Error> {
        let mut id2 = [0u8; 1];
        match dsi_host.read(DsiReadCommand::DcsShort { arg: CMD_RDID2 }, &mut id2) {
            Ok(_) if id2[0] == 0x80 => {
                defmt::info!("NT35510 detected - RDID2=0x80");
                return Ok(());
            }
            Ok(_) => {
                defmt::info!("RDID2 returned 0x{:02x}, not NT35510", id2[0]);
                return Err(Nt35510Error::ProbeMismatch(id2[0]));
            }
            Err(_) => {}
        }

        let mut id1 = [0u8; 1];
        match dsi_host.read(DsiReadCommand::DcsShort { arg: CMD_RDID1 }, &mut id1) {
            Ok(_) if id1[0] == 0x00 => {
                defmt::info!("NT35510 detected - RDID1=0x00");
                Ok(())
            }
            Ok(_) => {
                defmt::info!("RDID1 returned 0x{:02x}, not NT35510", id1[0]);
                Err(Nt35510Error::ProbeMismatch(id1[0]))
            }
            Err(_) => Err(Nt35510Error::DsiRead),
        }
    }

    pub fn init<D: DelayUs<u32>>(
        &mut self,
        dsi_host: &mut DsiHost,
        delay: &mut D,
    ) -> Result<(), Nt35510Error> {
        if self.initialized {
            defmt::warn!("NT35510 already initialized, skipping re-initialization");
            return Ok(());
        }

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
        self.write_cmd(dsi_host, CMD_COLMOD, COLMOD_RGB888)?;
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

    fn write_cmd(&self, dsi_host: &mut DsiHost, cmd: u8, param: u8) -> Result<(), Nt35510Error> {
        dsi_host
            .write(DsiWriteCommand::DcsShortP1 {
                arg: cmd,
                data: param,
            })
            .map_err(|_| Nt35510Error::DsiWrite)
    }

    fn write_reg(&self, dsi_host: &mut DsiHost, reg: u8, data: &[u8]) -> Result<(), Nt35510Error> {
        if data.is_empty() {
            self.write_cmd(dsi_host, reg, 0)
        } else if data.len() == 1 {
            self.write_cmd(dsi_host, reg, data[0])
        } else {
            dsi_host
                .write(DsiWriteCommand::DcsLongWrite { arg: reg, data })
                .map_err(|_| Nt35510Error::DsiWrite)
        }
    }
}
