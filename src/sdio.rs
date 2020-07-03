//! Sdio host

use crate::bb;
#[allow(unused_imports)]
use crate::gpio::{gpioa::*, gpiob::*, gpioc::*, gpiod::*, Alternate, AF12};
use crate::stm32::{RCC, SDIO};

pub use sdio_host::{CardCapacity, Cic, Cid, Csd, Ocr, Rca, Scr, SdStatus};

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
pub enum Error {
    Timeout,
    SoftwareTimeout,
    Crc,
    UnsupportedCardVersion,
    UnsupportedCardType,
    UnsupportedVoltage,
    DataCrcFail,
    RxOverFlow,
    TxUnderErr,
    NoCard,
    CsdParseError,
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

#[derive(Debug)]
/// Sd card
pub struct Card {
    pub capacity: CardCapacity,
    pub ocr: Ocr,
    pub rca: Rca, // Relative Card Address
    pub cid: Cid<[u32; 4]>,
    pub csd: Csd,
    pub scr: Scr<[u32; 2]>,
    pub status: SdStatus<[u32; 16]>,
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
        let cic = Cic(self.sdio.resp1.read().bits());

        // If card did't echo back the pattern, we do not have a v2 card
        if cic.checkpattern() != 0xAA {
            return Err(Error::UnsupportedCardVersion);
        }

        if cic.voltage_accepted() & 0b0001 == 0 {
            return Err(Error::UnsupportedVoltage);
        }

        let ocr = loop {
            let arg = CmdAppOper::VOLTAGE_WINDOW_SD as u32
                | CmdAppOper::HIGH_CAPACITY as u32
                | CmdAppOper::SD_SWITCH_1_8V_CAPACITY as u32;

            // Signal that next command is a app command
            self.cmd(Cmd::app_cmd(0))?;
            // Initialize card
            match self.cmd(Cmd::app_op_cmd(arg)) {
                Ok(_) => (),
                Err(Error::Crc) => (),
                Err(err) => return Err(err),
            }
            let ocr = Ocr(self.sdio.resp1.read().bits());
            if !ocr.powered() {
                // Still powering up
                continue;
            }

            break ocr;
        };

        // True for SDHC and SDXC False for SDSC
        let capacity = if ocr.high_capacity() {
            CardCapacity::SDHC
        } else {
            // Note: SDSC Not supported yet
            return Err(Error::UnsupportedCardType);
        };

        // Get CID
        self.cmd(Cmd::all_send_cid())?;
        let mut cid = [0; 4];
        cid[3] = self.sdio.resp1.read().bits();
        cid[2] = self.sdio.resp2.read().bits();
        cid[1] = self.sdio.resp3.read().bits();
        cid[0] = self.sdio.resp4.read().bits();

        // Get RCA
        self.cmd(Cmd::send_rel_addr())?;
        let mut rca = Rca(self.sdio.resp1.read().bits());
        // Zero out the status bits to let us use rca as argument to others commands
        // The same status bits are available as a R1 response to Card Status CMD13
        rca.set_status(0);

        // Get CSD
        self.cmd(Cmd::send_csd(rca.0))?;
        let mut csd = [0; 4];
        csd[3] = self.sdio.resp1.read().bits();
        csd[2] = self.sdio.resp2.read().bits();
        csd[1] = self.sdio.resp3.read().bits();
        csd[0] = self.sdio.resp4.read().bits();

        let csd = if let Some(csd) = Csd::parse(csd) {
            csd
        } else {
            return Err(Error::CsdParseError);
        };

        let mut card = Card {
            capacity,
            ocr,
            rca,
            cid: Cid(cid),
            csd,
            scr: Scr::default(),
            status: SdStatus::default(),
        };

        self.select_card(Some(&card))?;
        self.get_scr(&mut card)?;
        self.set_bus(self.bw, freq, &card)?;

        self.read_card_status(&mut card)?;
        self.card.replace(card);

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

    fn read_card_status(&mut self, card: &mut Card) -> Result<(), Error> {
        self.cmd(Cmd::set_blocklen(64))?;

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

        self.cmd(Cmd::app_cmd(card.rca.0))?;
        self.cmd(Cmd::send_card_status())?;

        let status = &mut card.status.0;
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
                    status[15 - idx] = self.sdio.fifo.read().bits().swap_bytes();
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

        Ok(())
    }

    fn select_card(&self, card: Option<&Card>) -> Result<(), Error> {
        let rca = card.map(|c| c.rca.0).unwrap_or(0);

        let r = self.cmd(Cmd::sel_desel_card(rca));
        match (r, rca) {
            (Err(Error::Timeout), 0) => Ok(()),
            _ => r,
        }
    }

    fn get_scr(&self, card: &mut Card) -> Result<(), Error> {
        self.cmd(Cmd::set_blocklen(8))?;

        self.sdio
            .dtimer
            .write(|w| unsafe { w.datatime().bits(0xFFFF_FFFF) });
        self.sdio.dlen.write(|w| unsafe { w.datalength().bits(8) });
        self.sdio
            .dctrl
            .write(|w| unsafe { w.dblocksize().bits(3).dtdir().set_bit().dten().set_bit() });

        self.cmd(Cmd::app_cmd(card.rca.0))?;
        self.cmd(Cmd::cmd51())?;

        let mut buf = [0; 2];
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
                buf[i] = self.sdio.fifo.read().bits();
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

        let scr = &mut card.scr.0;

        // The data received is byte swapped so swap it back
        scr[0] = buf[1].swap_bytes();
        scr[1] = buf[0].swap_bytes();

        Ok(())
    }

    /// Set bus width and clock frequency
    fn set_bus(&self, width: Buswidth, freq: ClockFreq, card: &Card) -> Result<(), Error> {
        let (width, acmd_arg) = match width {
            Buswidth::Buswidth4 if card.supports_widebus() => (width, 2),
            _ => (Buswidth::Buswidth1, 1),
        };

        self.cmd(Cmd::app_cmd(card.rca.0))?;
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
        self.csd.blocks()
    }

    /// Card supports wide bus
    fn supports_widebus(&self) -> bool {
        self.scr.bus_widths() & 0b0100 != 0
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
