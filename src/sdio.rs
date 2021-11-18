//! Sdio host

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::{gpioa::*, gpiob::*};
use crate::gpio::{gpioc::*, gpiod::*, Alternate, PushPull};
use crate::pac::{self, RCC, SDIO};
use crate::rcc::{Clocks, Enable, Reset};
#[allow(unused_imports)]
use crate::time::Hertz;
pub use sdio_host::{
    cmd, cmd::ResponseLen, CardCapacity, CardStatus, Cmd, CurrentState, SDStatus, CIC, CID, CSD,
    OCR, RCA, SCR,
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
    CLK: [PC12<Alternate<PushPull, 12>>]
    CMD: [PD2<Alternate<PushPull, 12>>]
    D0: [PC8<Alternate<PushPull, 12>>]
    D1: [PC9<Alternate<PushPull, 12>>]
    D2: [PC10<Alternate<PushPull, 12>>]
    D3: [PC11<Alternate<PushPull, 12>>]
}

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
pins! {
    CLK: [PB15<Alternate<PushPull, 12>>]
    CMD: [PA6<Alternate<PushPull, 12>>]
    D0: [PB4<Alternate<PushPull, 12>>, PB6<Alternate<PushPull, 12>>]
    D1: [PA8<Alternate<PushPull, 12>>]
    D2: [PA9<Alternate<PushPull, 12>>]
    D3: [PB5<Alternate<PushPull, 12>>]
}

#[cfg(feature = "stm32f411")]
pins! {
    CLK: [PB15<Alternate<PushPull, 12>>]
    CMD: [PA6<Alternate<PushPull, 12>>]
    D0: [PB4<Alternate<PushPull, 12>>, PB7<Alternate<PushPull, 12>>]
    D1: [PA8<Alternate<PushPull, 12>>]
    D2: [PA9<Alternate<PushPull, 12>>]
    D3: [PB5<Alternate<PushPull, 12>>]
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Buswidth {
    Buswidth1 = 0,
    Buswidth4 = 1,
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    clock: Hertz,
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
    pub fn new<PINS: Pins>(sdio: SDIO, _pins: PINS, clocks: &Clocks) -> Self {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &*RCC::ptr();
            // Enable and reset the sdio peripheral, it's the same bit position for both registers
            SDIO::enable(rcc);
            SDIO::reset(rcc);
        }

        // Configure clock
        sdio.clkcr.write(|w| {
            w.widbus()
                .bus_width1()
                .clken()
                .enabled()
                .clkdiv()
                .bits(ClockFreq::F400Khz as u8)
                .pwrsav()
                .disabled()
                .bypass()
                .disabled()
                .negedge()
                .rising()
                .hwfc_en()
                .enabled()
        });

        let mut host = Sdio {
            sdio,
            bw: PINS::BUSWIDTH,
            card: None,
            clock: clocks.sysclk(),
        };

        // Make sure card is powered off
        host.power_card(false);
        host
    }

    /// Initializes card (if present) and sets the bus at the specified frequency.
    pub fn init_card(&mut self, freq: ClockFreq) -> Result<(), Error> {
        // Enable power to card
        self.power_card(true);

        // Enable clock
        self.sdio.clkcr.modify(|_, w| w.clken().enabled());
        // Send card to idle state
        self.cmd(cmd::idle())?;

        // Check if cards supports CMD 8 (with pattern)
        self.cmd(cmd::send_if_cond(1, 0xAA))?;
        let cic = CIC::from(self.sdio.resp1.read().bits());

        // If card did't echo back the pattern, we do not have a v2 card
        if cic.pattern() != 0xAA {
            return Err(Error::UnsupportedCardVersion);
        }

        if cic.voltage_accepted() & 1 == 0 {
            return Err(Error::UnsupportedVoltage);
        }

        let ocr = loop {
            // Initialize card

            // 3.2-3.3V
            let voltage_window = 1 << 20;
            match self.app_cmd(cmd::sd_send_op_cond(true, false, true, voltage_window)) {
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
            CardCapacity::SDSC
        };

        // Get CID
        self.cmd(cmd::all_send_cid())?;
        let mut cid = [0; 4];
        cid[3] = self.sdio.resp1.read().bits();
        cid[2] = self.sdio.resp2.read().bits();
        cid[1] = self.sdio.resp3.read().bits();
        cid[0] = self.sdio.resp4.read().bits();
        let cid = CID::from(cid);

        // Get RCA
        self.cmd(cmd::send_relative_address())?;
        let rca = RCA::from(self.sdio.resp1.read().bits());
        let card_addr = rca.address();

        // Get CSD
        self.cmd(cmd::send_csd(card_addr))?;

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

        self.card.replace(card);

        // Wait before setting the bus witdth and frequency to avoid timeouts on SDSC cards
        while !self.card_ready()? {}

        self.set_bus(self.bw, freq)?;
        Ok(())
    }

    fn power_card(&mut self, on: bool) {
        use crate::pac::sdio::power::PWRCTRL_A;

        self.sdio.power.modify(|_, w| {
            w.pwrctrl().variant(if on {
                PWRCTRL_A::POWERON
            } else {
                PWRCTRL_A::POWEROFF
            })
        });

        // Wait for 2 ms after changing power settings
        cortex_m::asm::delay(2 * (self.clock.0 / 1000));
    }

    /// Get a reference to the initialized card
    pub fn card(&self) -> Result<&Card, Error> {
        self.card.as_ref().ok_or(Error::NoCard)
    }

    /// Read a block from the card
    pub fn read_block(&mut self, blockaddr: u32, block: &mut [u8; 512]) -> Result<(), Error> {
        let card = self.card()?;

        // Always read 1 block of 512 bytes
        // SDSC cards are byte addressed hence the blockaddress is in multiples of 512 bytes
        let blockaddr = match card.capacity {
            CardCapacity::SDSC => blockaddr * 512,
            _ => blockaddr,
        };
        self.cmd(cmd::set_block_length(512))?;
        self.start_datapath_transfer(512, 9, true);
        self.cmd(cmd::read_single_block(blockaddr))?;

        let mut i = 0;
        let mut sta;

        while {
            sta = self.sdio.sta.read();
            sta.rxact().bit_is_set()
        } {
            if sta.rxfifohf().bit() {
                for _ in 0..8 {
                    let bytes = self.sdio.fifo.read().bits().to_le_bytes();
                    block[i..i + 4].copy_from_slice(&bytes);
                    i += 4;
                }
            }

            if i == block.len() {
                break;
            }
        }

        status_to_error(sta)?;

        // Wait for card to be ready
        while !self.card_ready()? {}

        Ok(())
    }

    /// Write a block to card
    pub fn write_block(&mut self, blockaddr: u32, block: &[u8; 512]) -> Result<(), Error> {
        let card = self.card()?;

        // Always write 1 block of 512 bytes
        // SDSC cards are byte addressed hence the blockaddress is in multiples of 512 bytes
        let blockaddr = match card.capacity {
            CardCapacity::SDSC => blockaddr * 512,
            _ => blockaddr,
        };
        self.cmd(cmd::set_block_length(512))?;
        self.start_datapath_transfer(512, 9, false);
        self.cmd(cmd::write_single_block(blockaddr))?;

        let mut i = 0;
        let mut sta;

        while {
            sta = self.sdio.sta.read();
            sta.txact().bit_is_set()
        } {
            if sta.txfifohe().bit() {
                for _ in 0..8 {
                    let mut wb = [0u8; 4];
                    wb.copy_from_slice(&block[i..i + 4]);
                    let word = u32::from_le_bytes(wb);
                    self.sdio.fifo.write(|w| unsafe { w.bits(word) });
                    i += 4;
                }
            }

            if i == block.len() {
                break;
            }
        }

        status_to_error(sta)?;

        // Wait for card to finish writing data
        while !self.card_ready()? {}

        Ok(())
    }

    fn start_datapath_transfer(&self, length_bytes: u32, block_size: u8, card_to_controller: bool) {
        use crate::pac::sdio::dctrl::DTDIR_A;

        // Block Size up to 2^14 bytes
        assert!(block_size <= 14);

        // Command AND Data state machines must be idle
        while self.sdio.sta.read().cmdact().bit_is_set()
            || self.sdio.sta.read().rxact().bit_is_set()
            || self.sdio.sta.read().txact().bit_is_set()
        {}

        let dtdir = if card_to_controller {
            DTDIR_A::CARDTOCONTROLLER
        } else {
            DTDIR_A::CONTROLLERTOCARD
        };

        // Data timeout, in bus cycles
        self.sdio.dtimer.write(|w| w.datatime().bits(0xFFFF_FFFF));
        // Data length, in bytes
        self.sdio.dlen.write(|w| w.datalength().bits(length_bytes));
        // Transfer
        self.sdio.dctrl.write(|w| {
            w.dblocksize()
                .bits(block_size) // 2^n bytes block size
                .dtdir()
                .variant(dtdir)
                .dten()
                .enabled() // Enable transfer
        });
    }

    /// Read the state bits of the status
    fn read_status(&mut self) -> Result<CardStatus, Error> {
        let card = self.card()?;

        self.cmd(cmd::card_status(card.rca.address(), false))?;

        let r1 = self.sdio.resp1.read().bits();
        Ok(CardStatus::from(r1))
    }

    /// Check if card is done writing/reading and back in transfer state
    fn card_ready(&mut self) -> Result<bool, Error> {
        Ok(self.read_status()?.state() == CurrentState::Transfer)
    }

    /// Read the SDStatus struct
    pub fn read_sd_status(&mut self) -> Result<SDStatus, Error> {
        let _card = self.card()?;
        self.cmd(cmd::set_block_length(64))?;
        self.start_datapath_transfer(64, 6, true);
        self.app_cmd(cmd::sd_status())?;

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
    fn select_card(&self, rca: u16) -> Result<(), Error> {
        let r = self.cmd(cmd::select_card(rca));
        match (r, rca) {
            (Err(Error::Timeout), 0) => Ok(()),
            _ => r,
        }
    }

    /// Get the Card configuration for card at `address`
    fn get_scr(&self, rca: u16) -> Result<SCR, Error> {
        self.cmd(cmd::set_block_length(8))?;
        self.start_datapath_transfer(8, 3, true);
        self.cmd(cmd::app_cmd(rca))?;
        self.cmd(cmd::send_scr())?;

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
    fn set_bus(&self, width: Buswidth, freq: ClockFreq) -> Result<(), Error> {
        use crate::pac::sdio::clkcr::WIDBUS_A;

        let card_widebus = self.card()?.supports_widebus();

        let width = match width {
            Buswidth::Buswidth4 if card_widebus => WIDBUS_A::BUSWIDTH4,
            _ => WIDBUS_A::BUSWIDTH1,
        };

        self.app_cmd(cmd::set_bus_width(width == WIDBUS_A::BUSWIDTH4))?;

        self.sdio.clkcr.modify(|_, w| {
            w.clkdiv()
                .bits(freq as u8)
                .widbus()
                .variant(width)
                .clken()
                .enabled()
        });
        Ok(())
    }

    fn app_cmd<R: cmd::Resp>(&self, acmd: Cmd<R>) -> Result<(), Error> {
        let rca = self.card().map(|card| card.rca.address()).unwrap_or(0);
        self.cmd(cmd::app_cmd(rca))?;
        self.cmd(acmd)
    }

    /// Send command to card
    fn cmd<R: cmd::Resp>(&self, cmd: Cmd<R>) -> Result<(), Error> {
        use crate::pac::sdio::cmd::WAITRESP_A;

        // Command state machines must be idle
        while self.sdio.sta.read().cmdact().bit_is_set() {}

        // Clear the interrupts before we start
        clear_all_interrupts(&self.sdio.icr);

        // Command arg
        self.sdio.arg.write(|w| w.cmdarg().bits(cmd.arg));

        // Determine what kind of response the CPSM should wait for
        let waitresp = match cmd.response_len() {
            ResponseLen::Zero => WAITRESP_A::NORESPONSE,
            ResponseLen::R48 => WAITRESP_A::SHORTRESPONSE,
            ResponseLen::R136 => WAITRESP_A::LONGRESPONSE,
        };

        // Send the command
        self.sdio.cmd.write(|w| {
            w.waitresp()
                .variant(waitresp)
                .cmdindex()
                .bits(cmd.cmd)
                .waitint()
                .disabled()
                .cpsmen()
                .enabled()
        });

        let mut timeout: u32 = 0xFFFF_FFFF;

        let mut sta;
        if cmd.response_len() == ResponseLen::Zero {
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

fn status_to_error(sta: pac::sdio::sta::R) -> Result<(), Error> {
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

fn clear_all_interrupts(icr: &pac::sdio::ICR) {
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
}
