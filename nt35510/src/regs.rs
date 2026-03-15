//! NT35510 register and command definitions.

/// Sleep Out command.
pub const NT35510_CMD_SLPOUT: u8 = 0x11;
/// Display On command.
pub const NT35510_CMD_DISPON: u8 = 0x29;
/// Memory Write command.
pub const NT35510_CMD_RAMWR: u8 = 0x2C;
/// Column Address Set command.
pub const NT35510_CMD_CASET: u8 = 0x2A;
/// Row Address Set command.
pub const NT35510_CMD_RASET: u8 = 0x2B;
/// Memory Data Access Control command.
pub const NT35510_CMD_MADCTL: u8 = 0x36;
/// Interface Pixel Format command.
pub const NT35510_CMD_COLMOD: u8 = 0x3A;
/// Write Display Brightness command.
pub const NT35510_CMD_WRDISBV: u8 = 0x51;
/// Write Control Display command.
pub const NT35510_CMD_WRCTRLD: u8 = 0x53;
/// Write CABC command.
pub const NT35510_CMD_WRCABC: u8 = 0x55;

/// Read ID1 command.
pub const NT35510_CMD_RDID1: u8 = 0xDA;
/// Read ID2 command.
pub const NT35510_CMD_RDID2: u8 = 0xDB;

/// NT35510 expected ID1 value.
pub const NT35510_ID1_EXPECTED: u8 = 0x00;
/// NT35510 expected ID2 value.
pub const NT35510_ID2_EXPECTED: u8 = 0x80;

/// RGB565 (16-bit) pixel format value for COLMOD.
pub const NT35510_COLMOD_RGB565: u8 = 0x55;
/// RGB888 (24-bit) pixel format value for COLMOD.
pub const NT35510_COLMOD_RGB888: u8 = 0x77;

/// Portrait MADCTL value. Tested on STM32F469I-DISCO.
pub const NT35510_MADCTL_PORTRAIT: u8 = 0x00;
/// Landscape MADCTL value (MX | MV). Untested.
pub const NT35510_MADCTL_LANDSCAPE: u8 = 0x60;

/// Enable command set extension and select command page.
pub const NT35510_CMD_SETEXTC: u8 = 0xF0;
/// Voltage setting register block B0.
pub const NT35510_CMD_B0: u8 = 0xB0;
/// Voltage setting register block B1.
pub const NT35510_CMD_B1: u8 = 0xB1;
/// Voltage setting register block B2.
pub const NT35510_CMD_B2: u8 = 0xB2;
/// Voltage setting register block B3.
pub const NT35510_CMD_B3: u8 = 0xB3;
/// Voltage setting register block B5.
pub const NT35510_CMD_B5: u8 = 0xB5;
/// Voltage setting register block B6.
pub const NT35510_CMD_B6: u8 = 0xB6;
/// Voltage setting register block B7.
pub const NT35510_CMD_B7: u8 = 0xB7;
/// Voltage setting register block B8.
pub const NT35510_CMD_B8: u8 = 0xB8;
/// Voltage setting register block B9.
pub const NT35510_CMD_B9: u8 = 0xB9;
/// Voltage setting register block BA.
pub const NT35510_CMD_BA: u8 = 0xBA;
/// Voltage setting register block BC.
pub const NT35510_CMD_BC: u8 = 0xBC;
/// Voltage setting register block BD.
pub const NT35510_CMD_BD: u8 = 0xBD;
/// Voltage setting register block BE.
pub const NT35510_CMD_BE: u8 = 0xBE;
/// Voltage setting register block BF.
pub const NT35510_CMD_BF: u8 = 0xBF;
/// Panel timing/control register.
pub const NT35510_CMD_CC: u8 = 0xCC;
