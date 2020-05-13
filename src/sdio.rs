//! Sdio host

use crate::bb;
#[allow(unused_imports)]
use crate::gpio::{gpioa::*, gpiob::*, gpioc::*, gpiod::*, Alternate, AF12};
use crate::stm32::{RCC, SDIO};

pub trait PinClk {}
pub trait PinCmd {}
pub trait PinD0 {}
pub trait PinD1 {}
pub trait PinD2 {}
pub trait PinD3 {}

pub trait Pins {
    const BUSWIDTH: Buswidth;
}

impl<CLK, CMD, D0, D1, D2, D3> Pins for (CLK, CMD, D0, D1, D2, D3)
where
    CLK: PinClk,
    CMD: PinCmd,
    D0: PinD0,
    D1: PinD1,
    D2: PinD2,
    D3: PinD3,
{
    const BUSWIDTH: Buswidth = Buswidth::Buswidth4;
}

impl<CLK, CMD, D0> Pins for (CLK, CMD, D0)
where
    CLK: PinClk,
    CMD: PinCmd,
    D0: PinD0,
{
    const BUSWIDTH: Buswidth = Buswidth::Buswidth1;
}

macro_rules! pins {
    ($(CLK: [$($CLK:ty),*] CMD: [$($CMD:ty),*] D0: [$($D0:ty),*] D1: [$($D1:ty),*] D2: [$($D2:ty),*] D3: [$($D3:ty),*])+) => {
        $(
            $(
                impl PinClk for $CLK {}
            )*
            $(
                impl PinCmd for $CMD {}
            )*
            $(
                impl PinD0 for $D0 {}
            )*
            $(
                impl PinD1 for $D1 {}
            )*
            $(
                impl PinD2 for $D2 {}
            )*
            $(
                impl PinD3 for $D3 {}
            )*
        )+
    }
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    CLK: [PC12<Alternate<AF12>>]
    CMD: [PD2<Alternate<AF12>>]
    D0: [PC8<Alternate<AF12>>]
    D1: [PC9<Alternate<AF12>>]
    D2: [PC10<Alternate<AF12>>]
    D3: [PC11<Alternate<AF12>>]
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
pins! {
    CLK: [PB15<Alternate<AF12>>]
    CMD: [PA6<Alternate<AF12>>]
    D0: [PB4<Alternate<AF12>>, PB6<Alternate<AF12>>]
    D1: [PA8<Alternate<AF12>>]
    D2: [PA9<Alternate<AF12>>]
    D3: [PB5<Alternate<AF12>>]
}

#[derive(Copy, Clone)]
pub enum Buswidth {
    Buswidth1 = 0,
    Buswidth4 = 1,
}

enum PowerCtrl {
    Off = 0b00,
    On = 0b11,
}

/// Clock frequency of a SDIO bus.
#[allow(dead_code)]
pub enum ClockFreq {
    F24Mhz = 0,
    F16Mhz = 1,
    F12Mhz = 2,
    F8Mhz = 8,
    F4Mhz = 10,
    F1Mhz = 46,
    F400Khz = 118,
}

#[repr(u32)]
#[allow(dead_code)]
enum CmdAppOper {
    VOLTAGE_WINDOW_SD = 0x8010_0000,
    HIGH_CAPACITY = 0x4000_0000,
    SDMMC_STD_CAPACITY = 0x0000_0000,
    SDMMC_CHECK_PATTERN = 0x0000_01AA,
    SD_SWITCH_1_8V_CAPACITY = 0x0100_0000,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Response {
    None = 0,
    Short = 1,
    Long = 3,
}

#[derive(Debug, Copy, Clone)]
pub enum CardVersion {
    V2 = 2,
}

#[derive(Debug, Copy, Clone)]
pub enum CardType {
    /// Standard Capacity (< 2Gb)
    SDSC,
    /// High capacity (< 32Gb)
    SDHC,
}

impl Default for CardVersion {
    fn default() -> Self {
        CardVersion::V2
    }
}

impl Default for CardType {
    fn default() -> Self {
        CardType::SDSC
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    Timeout,
    SoftwareTimeout,
    Crc,
    UnsupportedCardVersion,
    UnsupportedCardType,
    DataCrcFail,
    RxOverFlow,
    TxUnderErr,
    NoCard,
}

/// Sdio device
pub struct Sdio {
    sdio: SDIO,
    bw: Buswidth,
    card: Option<Card>,
}

struct Cmd {
    cmd: u8,
    arg: u32,
    resp: Response,
}

#[derive(Debug, Copy, Clone, Default)]
/// Card identification
pub struct Cid {
    pub manufacturerid: u8,
    pub oem_applicationid: u16,
    pub prodname1: u32,
    pub prodname2: u8,
    pub prodrev: u8,
    pub prodsn: u32,
    pub manufact_date: u16,
    pub cid_crc: u8,
}

#[derive(Debug, Copy, Clone, Default)]
/// Card specific data
pub struct Csd {
    pub sys_spec_version: u8,
    pub max_bus_clk_frec: u8,
    pub rd_block_en: u8,
    // V2
    pub device_size: u32,
}

#[derive(Debug, Default, Copy, Clone)]
/// Sd card status
pub struct Status {
    pub bus_width: u8,
    pub secure_mode: u8,
    pub card_type: u16,
    pub protected_area_size: u32,
    pub speed_class: u8,
    pub performance_move: u8,
    pub allocation_units: u8,
    pub erase_size: u16,
    pub erase_timeout: u8,
    pub erase_offset: u8,
}

#[derive(Debug, Default)]
/// Sd card
pub struct Card {
    pub version: CardVersion,
    pub ctype: CardType,
    pub ocr: u32,
    pub rca: u32, // Relative Card Address
    pub cid: [u32; 4],
    pub csd: [u32; 4],
    pub scr: [u32; 2],
    pub status: [u32; 16],
}

impl Sdio {
    /// Create and enable the Sdio device
    pub fn new<PINS: Pins>(sdio: SDIO, _pins: PINS) -> Self {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &*RCC::ptr();
            // Enable and reset the sdio peripheral, it's the same bit position for both registers
            bb::set(&rcc.apb2enr, 11);
            bb::set(&rcc.apb2rstr, 11);
            bb::clear(&rcc.apb2rstr, 11);
        }

        // Configure clock
        sdio.clkcr.write(|w| unsafe {
            w.widbus()
                .bits(Buswidth::Buswidth1 as u8)
                .clken()
                .set_bit()
                .clkdiv()
                .bits(ClockFreq::F400Khz as u8)
                .pwrsav()
                .clear_bit()
                .bypass()
                .clear_bit()
                .negedge()
                .clear_bit()
                .hwfc_en()
                .set_bit()
        });

        sdio.power
            .modify(|_, w| unsafe { w.pwrctrl().bits(PowerCtrl::Off as u8) });

        Sdio {
            sdio,
            bw: PINS::BUSWIDTH,
            card: None,
        }
    }

    /// Initializes card (if present) and sets the bus at the specified frequency.
    pub fn init_card(&mut self, freq: ClockFreq) -> Result<(), Error> {
        // Enable power to card
        self.sdio
            .power
            .modify(|_, w| unsafe { w.pwrctrl().bits(PowerCtrl::On as u8) });

        // Enable clock
        self.sdio.clkcr.modify(|_, w| w.clken().set_bit());

        self.cmd(Cmd::idle())?;

        // Check if cards supports CMD 8 (with pattern)
        self.cmd(Cmd::hs_send_ext_csd(0x1AA))?;
        let r1 = self.sdio.resp1.read().bits();

        let mut card = if r1 == 0x1AA {
            /* Card echoed back the pattern, we have a v2 card */
            Card::default()
        } else {
            return Err(Error::UnsupportedCardVersion);
        };

        let ocr = loop {
            // Signal that next command is a app command
            self.cmd(Cmd::app_cmd(0))?;

            let arg = CmdAppOper::VOLTAGE_WINDOW_SD as u32
                | CmdAppOper::HIGH_CAPACITY as u32
                | CmdAppOper::SD_SWITCH_1_8V_CAPACITY as u32;

            // Initialize card
            match self.cmd(Cmd::app_op_cmd(arg)) {
                Ok(_) => (),
                Err(Error::Crc) => (),
                Err(err) => return Err(err),
            }
            let ocr = self.sdio.resp1.read().bits();
            if ocr & 0x8000_0000 == 0 {
                // Still powering up
                continue;
            }

            break ocr;
        };

        if ocr & 0x4000_0000 != 0 {
            card.ctype = CardType::SDHC;
        } else {
            return Err(Error::UnsupportedCardType);
        }

        card.ocr = ocr;

        // Get CID
        self.cmd(Cmd::all_send_cid())?;
        card.cid[0] = self.sdio.resp1.read().bits();
        card.cid[1] = self.sdio.resp2.read().bits();
        card.cid[2] = self.sdio.resp3.read().bits();
        card.cid[3] = self.sdio.resp4.read().bits();

        // Get RCA
        self.cmd(Cmd::send_rel_addr())?;
        card.rca = self.sdio.resp1.read().bits() >> 16;

        // Get CSD
        self.cmd(Cmd::send_csd(card.rca << 16))?;
        card.csd[0] = self.sdio.resp1.read().bits();
        card.csd[1] = self.sdio.resp2.read().bits();
        card.csd[2] = self.sdio.resp3.read().bits();
        card.csd[3] = self.sdio.resp4.read().bits();

        self.select_card(Some(&card))?;

        self.get_scr(&mut card)?;

        self.set_bus(self.bw, freq, &card)?;

        self.card.replace(card);
        self.read_card_status()?;

        Ok(())
    }

    /// Get a reference to the initialized card
    pub fn card(&self) -> Result<&Card, Error> {
        self.card.as_ref().ok_or(Error::NoCard)
    }

    /// Read block from card. buf must be at least 512 bytes
    pub fn read_block(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Error> {
        let _card = self.card()?;

        self.cmd(Cmd::set_blocklen(512))?;

        // Setup read command
        self.sdio
            .dtimer
            .write(|w| unsafe { w.datatime().bits(0xFFFF_FFFF) });
        self.sdio
            .dlen
            .write(|w| unsafe { w.datalength().bits(512) });
        self.sdio.dctrl.write(|w| unsafe {
            w.dblocksize()
                .bits(9) //512
                .dtdir()
                .set_bit()
                .dten()
                .set_bit()
        });
        self.cmd(Cmd::read_single_block(addr))?;

        let mut i = 0;
        let mut sta;
        while {
            sta = self.sdio.sta.read();
            !(sta.rxoverr().bit()
                || sta.dcrcfail().bit()
                || sta.dtimeout().bit()
                || sta.dataend().bit()
                || sta.stbiterr().bit())
        } {
            if sta.rxfifohf().bit() {
                for _ in 0..8 {
                    let bytes = self.sdio.fifo.read().bits().to_le_bytes();
                    buf[i..i + 4].copy_from_slice(&bytes);
                    i += 4;
                }
            }

            if i == buf.len() {
                break;
            }
        }

        if sta.dcrcfail().bit() {
            return Err(Error::DataCrcFail);
        } else if sta.rxoverr().bit() {
            return Err(Error::RxOverFlow);
        } else if sta.dtimeout().bit() {
            return Err(Error::Timeout);
        }

        Ok(())
    }

    /// Write block to card. buf must be at least 512 bytes
    pub fn write_block(&mut self, addr: u32, buf: &[u8]) -> Result<(), Error> {
        let _card = self.card()?;

        self.cmd(Cmd::set_blocklen(512))?;

        // Setup write command
        self.sdio
            .dtimer
            .write(|w| unsafe { w.datatime().bits(0xFFFF_FFFF) });
        self.sdio
            .dlen
            .write(|w| unsafe { w.datalength().bits(512) });
        self.sdio.dctrl.write(|w| unsafe {
            w.dblocksize()
                .bits(9) //512
                .dtdir()
                .clear_bit()
                .dten()
                .set_bit()
        });
        self.cmd(Cmd::write_single_block(addr))?;

        let mut i = 0;
        let mut sta;
        while {
            sta = self.sdio.sta.read();
            !(sta.txunderr().bit()
                || sta.dcrcfail().bit()
                || sta.dtimeout().bit()
                || sta.dataend().bit()
                || sta.stbiterr().bit())
        } {
            if sta.txfifohe().bit() {
                for _ in 0..8 {
                    let mut wb = [0u8; 4];
                    wb.copy_from_slice(&buf[i..i + 4]);
                    let word = u32::from_le_bytes(wb);
                    self.sdio.fifo.write(|w| unsafe { w.bits(word) });
                    i += 4;
                }
            }

            if i == buf.len() {
                break;
            }
        }

        if sta.dcrcfail().bit() {
            return Err(Error::DataCrcFail);
        } else if sta.txunderr().bit() {
            return Err(Error::TxUnderErr);
        } else if sta.dtimeout().bit() {
            return Err(Error::Timeout);
        }

        Ok(())
    }

    fn read_card_status(&mut self) -> Result<(), Error> {
        let card = self.card()?;

        self.cmd(Cmd::set_blocklen(64))?;

        self.cmd(Cmd::app_cmd(card.rca << 16))?;

        // Prepare the transfer
        self.sdio
            .dtimer
            .write(|w| unsafe { w.datatime().bits(0xFFFF_FFFF) });
        self.sdio.dlen.write(|w| unsafe { w.datalength().bits(64) });
        self.sdio.dctrl.write(|w| unsafe {
            w.dblocksize()
                .bits(6) // 64
                .dtdir()
                .set_bit()
                .dten()
                .set_bit()
        });

        self.cmd(Cmd::send_card_status())?;

        let mut status = [0u32; 16];
        let mut idx = 0;
        let mut sta;
        while {
            sta = self.sdio.sta.read();
            !(sta.rxoverr().bit()
                || sta.dcrcfail().bit()
                || sta.dtimeout().bit()
                || sta.dbckend().bit())
        } {
            if sta.rxfifohf().bit() {
                for _ in 0..8 {
                    status[idx] = self.sdio.fifo.read().bits();
                    idx += 1;
                }
            }

            if idx == status.len() {
                break;
            }
        }

        if sta.dcrcfail().bit() {
            return Err(Error::DataCrcFail);
        } else if sta.rxoverr().bit() {
            return Err(Error::RxOverFlow);
        } else if sta.dtimeout().bit() {
            return Err(Error::Timeout);
        }

        let card = self.card.as_mut().ok_or(Error::NoCard)?;
        card.status.copy_from_slice(&status);

        Ok(())
    }

    fn select_card(&self, card: Option<&Card>) -> Result<(), Error> {
        let rca = card.map(|c| c.rca << 16).unwrap_or(0);

        let r = self.cmd(Cmd::sel_desel_card(rca));
        match (r, rca) {
            (Err(Error::Timeout), 0) => Ok(()),
            _ => r,
        }
    }

    fn get_scr(&self, card: &mut Card) -> Result<(), Error> {
        self.cmd(Cmd::set_blocklen(8))?;
        self.cmd(Cmd::app_cmd(card.rca << 16))?;

        self.sdio
            .dtimer
            .write(|w| unsafe { w.datatime().bits(0xFFFF_FFFF) });
        self.sdio.dlen.write(|w| unsafe { w.datalength().bits(8) });
        self.sdio
            .dctrl
            .write(|w| unsafe { w.dblocksize().bits(3).dtdir().set_bit().dten().set_bit() });
        self.cmd(Cmd::cmd51())?;

        let mut scr = [0; 2];
        let mut i = 0;
        let mut sta;
        while {
            sta = self.sdio.sta.read();

            !(sta.rxoverr().bit()
                || sta.dcrcfail().bit()
                || sta.dtimeout().bit()
                || sta.dbckend().bit())
        } {
            if sta.rxdavl().bit() {
                scr[i] = self.sdio.fifo.read().bits();
                i += 1;
            }

            if i == 2 {
                break;
            }
        }

        if sta.dcrcfail().bit() {
            return Err(Error::DataCrcFail);
        } else if sta.rxoverr().bit() {
            return Err(Error::RxOverFlow);
        } else if sta.dtimeout().bit() {
            return Err(Error::Timeout);
        }

        card.scr[0] = ((scr[1] & 0xff) << 24)
            | ((scr[1] & 0xff00) << 8)
            | ((scr[1] & 0x00ff_0000) >> 8)
            | ((scr[1] & 0xff00_0000) >> 24);

        card.scr[1] = ((scr[0] & 0xff) << 24)
            | ((scr[0] & 0xff00) << 8)
            | ((scr[0] & 0x00ff_0000) >> 8)
            | ((scr[0] & 0xff00_0000) >> 24);

        Ok(())
    }

    /// Set bus width and clock frequency
    fn set_bus(&self, width: Buswidth, freq: ClockFreq, card: &Card) -> Result<(), Error> {
        let (width, acmd_arg) = match width {
            Buswidth::Buswidth4 if card.supports_widebus() => (width, 2),
            _ => (Buswidth::Buswidth1, 1),
        };

        self.cmd(Cmd::app_cmd(card.rca << 16))?;
        self.cmd(Cmd::acmd6(acmd_arg))?;

        self.sdio.clkcr.modify(|_, w| unsafe {
            w.clkdiv()
                .bits(freq as u8)
                .widbus()
                .bits(width as u8)
                .clken()
                .set_bit()
        });
        Ok(())
    }

    /// Send command to card
    fn cmd(&self, cmd: Cmd) -> Result<(), Error> {
        // Clear interrupts
        self.sdio.icr.modify(|_, w| {
            w.ccrcfailc()
                .set_bit()
                .ctimeoutc()
                .set_bit()
                .ceataendc()
                .set_bit()
                .cmdrendc()
                .set_bit()
                .cmdsentc()
                .set_bit()
                .dataendc()
                .set_bit()
                .dbckendc()
                .set_bit()
                .dcrcfailc()
                .set_bit()
                .dtimeoutc()
                .set_bit()
                .sdioitc()
                .set_bit()
                .stbiterrc()
                .set_bit()
                .rxoverrc()
                .set_bit()
                .txunderrc()
                .set_bit()
        });

        // Command arg
        self.sdio.arg.write(|w| unsafe { w.cmdarg().bits(cmd.arg) });

        self.sdio.cmd.write(|w| unsafe {
            w.waitresp()
                .bits(cmd.resp as u8)
                .cmdindex()
                .bits(cmd.cmd)
                .waitint()
                .clear_bit()
                .cpsmen()
                .set_bit()
        });

        let mut timeout: u32 = 0xFFFF_FFFF;

        let mut sta;
        if cmd.resp == Response::None {
            // Wait for command sent or a timeout
            while {
                sta = self.sdio.sta.read();
                !(sta.ctimeout().bit() || sta.cmdsent().bit()) && timeout > 0
            } {
                timeout -= 1;
            }
        } else {
            while {
                sta = self.sdio.sta.read();
                !(sta.ctimeout().bit() || sta.cmdrend().bit() || sta.ccrcfail().bit())
                    && timeout > 0
            } {
                timeout -= 1;
            }
        }

        if sta.ctimeout().bit_is_set() {
            return Err(Error::Timeout);
        } else if timeout == 0 {
            return Err(Error::SoftwareTimeout);
        } else if sta.ccrcfail().bit() {
            return Err(Error::Crc);
        }

        Ok(())
    }
}

impl Card {
    /// Size in bytes
    pub fn size(&self) -> u64 {
        u64::from(self.block_count()) * 512
    }

    /// Size in blocks
    pub fn block_count(&self) -> u32 {
        let block_count = ((self.csd[1] & 0x0000_003F) << 16) | (self.csd[2] >> 16);
        (block_count + 1) * 1024
    }

    fn supports_widebus(&self) -> bool {
        self.scr[1] & 0x0004_0000 != 0
    }

    pub fn cid(&self) -> Cid {
        Cid {
            manufacturerid: (self.cid[0] >> 24) as u8,
            oem_applicationid: ((self.cid[0] & 0x00FF_FF00) >> 8) as u16,
            prodname1: (((self.cid[0] & 0x0000_00FF) << 24) | ((self.cid[1] & 0xFFFF_FF00) >> 8)),
            prodname2: self.cid[1] as u8,
            prodrev: (self.cid[2] >> 24) as u8,
            prodsn: (((self.cid[2] & 0x00FF_FFFF) << 8) | ((self.cid[3] & 0xFF00_0000) >> 24)),
            manufact_date: ((self.cid[3] & 0x000F_FF00) >> 8) as u16,
            cid_crc: ((self.cid[3] & 0x0000_00FE) >> 1) as u8,
        }
    }

    pub fn csd(&self) -> Csd {
        Csd {
            sys_spec_version: ((self.csd[0] & 0x3C00_0000) >> 26) as u8,
            max_bus_clk_frec: (self.csd[0] & 0x0000_00FF) as u8,
            rd_block_en: ((self.csd[1] & 0x000F_0000) >> 16) as u8,
            device_size: (((self.csd[1] & 0x0000_003F) << 16) | (self.csd[2] >> 16)),
        }
    }

    pub fn status(&self) -> Status {
        Status {
            bus_width: ((self.status[0] & 0xC0) >> 6) as u8,
            secure_mode: ((self.status[0] & 0x20) >> 5) as u8,
            card_type: (((self.status[0] & 0x00FF_0000) >> 8)
                | ((self.status[0] & 0xFF00_0000) >> 24)) as u16,
            protected_area_size: (((self.status[1] & 0xFF) << 24)
                | ((self.status[1] & 0x0000_FF00) << 8)
                | ((self.status[1] & 0x00FF_0000) >> 8)
                | ((self.status[1] & 0xFF00_0000) >> 24)),
            speed_class: (self.status[2] & 0xFF) as u8,
            performance_move: ((self.status[2] & 0xFF00) >> 8) as u8,
            allocation_units: ((self.status[2] & 0x00F0_0000) >> 20) as u8,
            erase_size: (((self.status[2] & 0xFF00_0000) >> 16) | (self.status[3] & 0xFF)) as u16,
            erase_timeout: ((self.status[3] & 0xFC00) >> 10) as u8,
            erase_offset: ((self.status[3] & 0x0300) >> 8) as u8,
        }
    }
}

impl Cmd {
    const fn new(cmd: u8, arg: u32, resp: Response) -> Cmd {
        Cmd { cmd, arg, resp }
    }

    const fn idle() -> Cmd {
        Cmd::new(0, 0, Response::None)
    }

    const fn all_send_cid() -> Cmd {
        Cmd::new(2, 0, Response::Long)
    }

    const fn send_rel_addr() -> Cmd {
        Cmd::new(3, 0, Response::Short)
    }

    const fn acmd6(arg: u32) -> Cmd {
        Cmd::new(6, arg, Response::Short)
    }

    const fn sel_desel_card(rca: u32) -> Cmd {
        Cmd::new(7, rca, Response::Short)
    }

    const fn hs_send_ext_csd(arg: u32) -> Cmd {
        Cmd::new(8, arg, Response::Short)
    }

    const fn send_csd(rca: u32) -> Cmd {
        Cmd::new(9, rca, Response::Long)
    }

    const fn send_card_status() -> Cmd {
        Cmd::new(13, 0, Response::Short)
    }

    const fn set_blocklen(blocklen: u32) -> Cmd {
        Cmd::new(16, blocklen, Response::Short)
    }

    const fn read_single_block(addr: u32) -> Cmd {
        Cmd::new(17, addr, Response::Short)
    }

    const fn write_single_block(addr: u32) -> Cmd {
        Cmd::new(24, addr, Response::Short)
    }

    const fn app_op_cmd(arg: u32) -> Cmd {
        Cmd::new(41, arg, Response::Short)
    }

    const fn cmd51() -> Cmd {
        Cmd::new(51, 0, Response::Short)
    }

    /// App Command. Indicates that next command will be a app command
    const fn app_cmd(rca: u32) -> Cmd {
        Cmd::new(55, rca, Response::Short)
    }
}
