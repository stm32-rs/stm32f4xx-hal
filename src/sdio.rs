//! Sdio host

use crate::bb;
#[allow(unused_imports)]
use crate::gpio::{gpioa::*, gpiob::*, gpioc::*, gpiod::*, Alternate, AF12};
use crate::rcc::Clocks;
use crate::stm32::{self, RCC, SDIO};
pub use sdio_host::{
    CardCapacity, CardStatus, CurrentState, SDStatus, CIC, CID, CSD, OCR, RCA, SCR,
};

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
}

/// Sdio device
pub struct Sdio {
    sdio: SDIO,
    bw: Buswidth,
    card: Option<Card>,
    clocks: Clocks,
}

/// Sd card
pub struct Card {
    pub capacity: CardCapacity,
    pub ocr: OCR,
    pub rca: RCA, // Relative Card Address
    pub cid: CID,
    pub csd: CSD,
    pub scr: SCR,
}

impl Sdio {
    /// Create and enable the Sdio device
    pub fn new<PINS: Pins>(sdio: SDIO, _pins: PINS, clocks: Clocks) -> Self {
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

        let mut host = Sdio {
            sdio,
            bw: PINS::BUSWIDTH,
            card: None,
            clocks,
        };

        // Make sure card is powered off
        host.set_power(PowerCtrl::Off);
        host
    }

    /// Initializes card (if present) and sets the bus at the specified frequency.
    pub fn init_card(&mut self, freq: ClockFreq) -> Result<(), Error> {
        // Enable power to card
        self.set_power(PowerCtrl::On);

        // Enable clock
        self.sdio.clkcr.modify(|_, w| w.clken().set_bit());

        self.cmd(Cmd::idle())?;

        // Check if cards supports CMD 8 (with pattern)
        self.cmd(Cmd::hs_send_ext_csd(0x1AA))?;
        let cic = CIC::from(self.sdio.resp1.read().bits());

        // If card did't echo back the pattern, we do not have a v2 card
        if cic.pattern() != 0xAA {
            return Err(Error::UnsupportedCardVersion);
        }

        if cic.voltage_accepted() & 1 == 0 {
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
            let ocr = OCR::from(self.sdio.resp1.read().bits());
            if ocr.is_busy() {
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
        let cid = CID::from(cid);

        // Get RCA
        self.cmd(Cmd::send_rel_addr())?;
        let rca = RCA::from(self.sdio.resp1.read().bits());
        let card_addr = (rca.address() as u32) << 16;

        // Get CSD
        self.cmd(Cmd::send_csd(card_addr))?;

        let mut csd = [0; 4];
        csd[3] = self.sdio.resp1.read().bits();
        csd[2] = self.sdio.resp2.read().bits();
        csd[1] = self.sdio.resp3.read().bits();
        csd[0] = self.sdio.resp4.read().bits();
        let csd = CSD::from(csd);

        self.select_card(card_addr)?;
        let scr = self.get_scr(card_addr)?;

        let card = Card {
            capacity,
            ocr,
            rca,
            cid,
            csd,
            scr,
        };

        self.set_bus(self.bw, freq, &card)?;
        self.card.replace(card);
        Ok(())
    }

    fn set_power(&mut self, pwr: PowerCtrl) {
        self.sdio
            .power
            .modify(|_, w| unsafe { w.pwrctrl().bits(pwr as u8) });

        // Wait for 2 ms after changing power settings
        cortex_m::asm::delay(2 * (self.clocks.sysclk().0 / 1000));
    }

    /// Get a reference to the initialized card
    pub fn card(&self) -> Result<&Card, Error> {
        self.card.as_ref().ok_or(Error::NoCard)
    }

    /// Read block from card. buf must be at least 512 bytes
    pub fn read_block(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Error> {
        let _card = self.card()?;

        self.cmd(Cmd::set_blocklen(512))?;
        self.start_datapath_transfer(512, 9, true);
        self.cmd(Cmd::read_single_block(addr))?;

        let mut i = 0;
        let mut sta;

        while {
            sta = self.sdio.sta.read();
            sta.rxact().bit_is_set()
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

        status_to_error(sta)?;

        // Wait for card to be ready
        while !self.card_ready()? {}

        Ok(())
    }

    /// Write block to card. buf must be at least 512 bytes
    pub fn write_block(&mut self, addr: u32, buf: &[u8]) -> Result<(), Error> {
        let _card = self.card()?;

        self.cmd(Cmd::set_blocklen(512))?;
        self.start_datapath_transfer(512, 9, false);
        self.cmd(Cmd::write_single_block(addr))?;

        let mut i = 0;
        let mut sta;

        while {
            sta = self.sdio.sta.read();
            sta.txact().bit_is_set()
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

        status_to_error(sta)?;

        // Wait for card to finish writing data
        while !self.card_ready()? {}

        Ok(())
    }

    fn start_datapath_transfer(&self, length_bytes: u32, block_size: u8, dtdir: bool) {
        // Block Size up to 2^14 bytes
        assert!(block_size <= 14);

        // Command AND Data state machines must be idle
        while self.sdio.sta.read().cmdact().bit_is_set()
            || self.sdio.sta.read().rxact().bit_is_set()
            || self.sdio.sta.read().txact().bit_is_set()
        {}

        // Data timeout, in bus cycles
        self.sdio
            .dtimer
            .write(|w| unsafe { w.datatime().bits(0xFFFF_FFFF) });
        // Data length, in bytes
        self.sdio
            .dlen
            .write(|w| unsafe { w.datalength().bits(length_bytes) });
        // Transfer
        self.sdio.dctrl.write(|w| unsafe {
            w.dblocksize()
                .bits(block_size) // 2^n bytes block size
                .dtdir()
                .bit(dtdir)
                .dten()
                .set_bit() // Enable transfer
        });
    }

    /// Read the state bits of the status
    fn read_status(&mut self) -> Result<CardStatus, Error> {
        let card = self.card()?;

        self.cmd(Cmd::cmd13(card.address()))?;
        let r1 = self.sdio.resp1.read().bits();
        Ok(CardStatus::from(r1))
    }

    /// Check if card is done writing/reading and back in transfer state
    fn card_ready(&mut self) -> Result<bool, Error> {
        Ok(self.read_status()?.state() == CurrentState::Transfer)
    }

    /// Read the SDStatus struct
    pub fn read_sd_status(&mut self) -> Result<SDStatus, Error> {
        let card = self.card()?;
        self.cmd(Cmd::set_blocklen(64))?;
        self.start_datapath_transfer(64, 6, true);
        self.cmd(Cmd::app_cmd(card.address()))?;
        self.cmd(Cmd::acmd13())?;

        let mut status = [0u32; 16];
        let mut idx = 0;
        let mut sta;

        while {
            sta = self.sdio.sta.read();
            sta.rxact().bit_is_set()
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

        status_to_error(sta)?;
        Ok(SDStatus::from(status))
    }

    /// Select the card with `address`
    fn select_card(&self, address: u32) -> Result<(), Error> {
        let r = self.cmd(Cmd::sel_desel_card(address));
        match (r, address) {
            (Err(Error::Timeout), 0) => Ok(()),
            _ => r,
        }
    }

    /// Get the Card configuration for card at `address`
    fn get_scr(&self, address: u32) -> Result<SCR, Error> {
        self.cmd(Cmd::set_blocklen(8))?;
        self.start_datapath_transfer(8, 3, true);
        self.cmd(Cmd::app_cmd(address))?;
        self.cmd(Cmd::cmd51())?;

        let mut buf = [0; 2];
        let mut i = 0;
        let mut sta;

        while {
            sta = self.sdio.sta.read();
            sta.rxact().bit_is_set()
        } {
            if sta.rxdavl().bit() {
                buf[1 - i] = self.sdio.fifo.read().bits().swap_bytes();
                i += 1;
            }

            if i == 2 {
                break;
            }
        }

        status_to_error(sta)?;
        Ok(SCR::from(buf))
    }

    /// Set bus width and clock frequency
    fn set_bus(&self, width: Buswidth, freq: ClockFreq, card: &Card) -> Result<(), Error> {
        let (width, acmd_arg) = match width {
            Buswidth::Buswidth4 if card.supports_widebus() => (width, 2),
            _ => (Buswidth::Buswidth1, 1),
        };

        self.cmd(Cmd::app_cmd(card.address()))?;
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
        // Command state machines must be idle
        while self.sdio.sta.read().cmdact().bit_is_set() {}

        // Clear all interrupts
        clear_all_interrupts(&self.sdio.icr);

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

                (!(sta.ctimeout().bit() || sta.cmdsent().bit()) || sta.cmdact().bit_is_set())
                    && timeout > 0
            } {
                timeout -= 1;
            }
        } else {
            while {
                sta = self.sdio.sta.read();
                (!(sta.ctimeout().bit() || sta.cmdrend().bit() || sta.ccrcfail().bit())
                    || sta.cmdact().bit_is_set())
                    && timeout > 0
            } {
                timeout -= 1;
            }
        }

        if timeout == 0 {
            return Err(Error::SoftwareTimeout);
        }

        status_to_error(sta)
    }
}

fn status_to_error(sta: stm32::sdio::sta::R) -> Result<(), Error> {
    if sta.ctimeout().bit_is_set() {
        return Err(Error::Timeout);
    } else if sta.ccrcfail().bit() {
        return Err(Error::Crc);
    } else if sta.dcrcfail().bit() {
        return Err(Error::DataCrcFail);
    } else if sta.rxoverr().bit() {
        return Err(Error::RxOverFlow);
    } else if sta.dtimeout().bit() {
        return Err(Error::Timeout);
    } else if sta.txunderr().bit() {
        return Err(Error::TxUnderErr);
    }
    Ok(())
}

fn clear_all_interrupts(icr: &stm32::sdio::ICR) {
    icr.modify(|_, w| {
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
}

impl Card {
    /// Size in blocks
    pub fn block_count(&self) -> u32 {
        self.csd.block_count()
    }

    /// Card supports wide bus
    fn supports_widebus(&self) -> bool {
        self.scr.bus_width_four()
    }

    /// Helper for using the address as a rca argument
    fn address(&self) -> u32 {
        (self.rca.address() as u32) << 16
    }
}

struct Cmd {
    cmd: u8,
    arg: u32,
    resp: Response,
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

    const fn cmd13(rca: u32) -> Cmd {
        Cmd::new(13, rca, Response::Short)
    }

    const fn acmd13() -> Cmd {
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
