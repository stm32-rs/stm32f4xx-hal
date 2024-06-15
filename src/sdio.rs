//! Sdio host

use crate::gpio::alt::sdio as alt;
use crate::pac::{self, SDIO};
use crate::rcc::{Clocks, Enable, Reset};
#[allow(unused_imports)]
use fugit::HertzU32 as Hertz;
pub use sdio_host::{
    common_cmd::{self, ResponseLen},
    emmc::{CardCapacity, CardStatus, CurrentState, CID, CSD, EMMC, OCR, RCA},
    emmc_cmd,
    sd::{SDStatus, CIC, SCR, SD},
    sd_cmd, Cmd,
};

pub trait Pins {
    const BUSWIDTH: Buswidth;

    type SdPins;
    fn convert(self) -> Self::SdPins;
}

impl<CLK, CMD, D0, D1, D2, D3, D4, D5, D6, D7> Pins for (CLK, CMD, D0, D1, D2, D3, D4, D5, D6, D7)
where
    CLK: Into<alt::Ck>,
    CMD: Into<alt::Cmd>,
    D0: Into<alt::D0>,
    D1: Into<alt::D1>,
    D2: Into<alt::D2>,
    D3: Into<alt::D3>,
    D4: Into<alt::D4>,
    D5: Into<alt::D5>,
    D6: Into<alt::D6>,
    D7: Into<alt::D7>,
{
    const BUSWIDTH: Buswidth = Buswidth::Buswidth8;

    type SdPins = (
        alt::Ck,
        alt::Cmd,
        alt::D0,
        alt::D1,
        alt::D2,
        alt::D3,
        alt::D4,
        alt::D5,
        alt::D6,
        alt::D7,
    );
    fn convert(self) -> Self::SdPins {
        (
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        )
    }
}

impl<CLK, CMD, D0, D1, D2, D3> Pins for (CLK, CMD, D0, D1, D2, D3)
where
    CLK: Into<alt::Ck>,
    CMD: Into<alt::Cmd>,
    D0: Into<alt::D0>,
    D1: Into<alt::D1>,
    D2: Into<alt::D2>,
    D3: Into<alt::D3>,
{
    const BUSWIDTH: Buswidth = Buswidth::Buswidth4;

    type SdPins = (alt::Ck, alt::Cmd, alt::D0, alt::D1, alt::D2, alt::D3);
    fn convert(self) -> Self::SdPins {
        (
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        )
    }
}

impl<CLK, CMD, D0> Pins for (CLK, CMD, D0)
where
    CLK: Into<alt::Ck>,
    CMD: Into<alt::Cmd>,
    D0: Into<alt::D0>,
{
    const BUSWIDTH: Buswidth = Buswidth::Buswidth1;

    type SdPins = (alt::Ck, alt::Cmd, alt::D0);
    fn convert(self) -> Self::SdPins {
        (self.0.into(), self.1.into(), self.2.into())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Buswidth {
    Buswidth1 = 0,
    Buswidth4 = 1,
    Buswidth8 = 2,
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

#[derive(Debug, Copy, Clone)]
pub enum AddressMode {
    Byte,
    Block512,
}

/// A peripheral that uses the SDIO hardware, generic over the particular type of device.
pub struct Sdio<P: SdioPeripheral> {
    sdio: SDIO,
    bw: Buswidth,
    card: Option<P>,
    clock: Hertz,
}

/// Sd card peripheral
pub struct SdCard {
    pub capacity: CardCapacity,
    pub ocr: OCR<SD>,
    pub rca: RCA<SD>, // Relative Card Address
    pub cid: CID<SD>,
    pub csd: CSD<SD>,
    pub scr: SCR,
}

/// eMMC device peripheral
pub struct Emmc {
    pub ocr: OCR<EMMC>,
    pub rca: RCA<EMMC>, // Relative Card Address
    pub cid: CID<EMMC>,
    pub csd: CSD<EMMC>,
}

impl<P: SdioPeripheral> Sdio<P> {
    /// Create and enable the Sdio device
    pub fn new<PINS: Pins>(sdio: SDIO, pins: PINS, clocks: &Clocks) -> Self {
        unsafe {
            // Enable and reset the sdio peripheral, it's the same bit position for both registers
            SDIO::enable_unchecked();
            SDIO::reset_unchecked();
        }

        // Configure clock
        sdio.clkcr().write(|w| {
            w.widbus().bus_width1();
            w.clken().enabled();
            w.clkdiv().set(ClockFreq::F400Khz as u8);
            w.pwrsav().disabled();
            w.bypass().disabled();
            w.negedge().rising();
            // Do not use hardware flow control.
            // Using it causes clock glitches and CRC errors.
            // See chip errata SDIO section:
            // - F42x/F43x: https://www.st.com/resource/en/errata_sheet/es0206-stm32f427437-and-stm32f429439-line-limitations-stmicroelectronics.pdf
            // - F40x/F41x: https://www.st.com/resource/en/errata_sheet/es0182-stm32f405407xx-and-stm32f415417xx-device-limitations-stmicroelectronics.pdf
            w.hwfc_en().disabled()
        });

        let _pins = pins.convert();

        let mut host = Self {
            sdio,
            bw: PINS::BUSWIDTH,
            card: None,
            clock: clocks.sysclk(),
        };

        // Make sure card is powered off
        host.power_card(false);
        host
    }

    fn power_card(&mut self, on: bool) {
        use crate::pac::sdio::power::PWRCTRL;

        self.sdio.power().modify(|_, w| {
            w.pwrctrl().variant(if on {
                PWRCTRL::PowerOn
            } else {
                PWRCTRL::PowerOff
            })
        });

        // Wait for 2 ms after changing power settings
        cortex_m::asm::delay(2 * (self.clock.raw() / 1000));
    }

    /// Get a reference to the initialized card
    pub fn card(&self) -> Result<&P, Error> {
        self.card.as_ref().ok_or(Error::NoCard)
    }

    /// Read a block from the card
    pub fn read_block(&mut self, blockaddr: u32, block: &mut [u8; 512]) -> Result<(), Error> {
        let card = self.card()?;

        // Always read 1 block of 512 bytes
        // SDSC cards are byte addressed hence the blockaddress is in multiples of 512 bytes
        let blockaddr = match card.get_address_mode() {
            AddressMode::Byte => blockaddr * 512,
            AddressMode::Block512 => blockaddr,
        };
        self.cmd(common_cmd::set_block_length(512))?;
        self.start_datapath_transfer(512, 9, true);
        self.cmd(common_cmd::read_single_block(blockaddr))?;

        let mut i = 0;

        let status = loop {
            let sta = self.sdio.sta().read();

            if sta.rxact().bit_is_clear() {
                break sta;
            }

            if sta.rxfifohf().bit() {
                for _ in 0..8 {
                    let bytes = self.sdio.fifo().read().bits().to_le_bytes();
                    block[i..i + 4].copy_from_slice(&bytes);
                    i += 4;
                }
            }

            if i == block.len() {
                break sta;
            }
        };

        status_to_error(status)?;

        // Wait for card to be ready
        while !self.card_ready()? {}

        Ok(())
    }

    /// Write a block to card
    pub fn write_block(&mut self, blockaddr: u32, block: &[u8; 512]) -> Result<(), Error> {
        let card = self.card()?;

        // Always write 1 block of 512 bytes
        // SDSC cards are byte addressed hence the blockaddress is in multiples of 512 bytes
        let blockaddr = match card.get_address_mode() {
            AddressMode::Byte => blockaddr * 512,
            AddressMode::Block512 => blockaddr,
        };
        self.cmd(common_cmd::set_block_length(512))?;
        self.start_datapath_transfer(512, 9, false);
        self.cmd(common_cmd::write_single_block(blockaddr))?;

        let mut i = 0;

        let status = loop {
            let sta = self.sdio.sta().read();

            if sta.txact().bit_is_clear() {
                break sta;
            }

            if sta.txfifohe().bit() {
                for _ in 0..8 {
                    let mut wb = [0u8; 4];
                    wb.copy_from_slice(&block[i..i + 4]);
                    let word = u32::from_le_bytes(wb);
                    self.sdio.fifo().write(|w| w.set(word));
                    i += 4;
                }
            }

            if i == block.len() {
                break sta;
            }
        };

        status_to_error(status)?;

        // Wait for SDIO module to finish transmitting data
        loop {
            let sta = self.sdio.sta().read();
            if !sta.txact().bit_is_set() {
                break;
            }
        }

        // Wait for card to finish writing data
        while !self.card_ready()? {}

        Ok(())
    }

    fn start_datapath_transfer(&self, length_bytes: u32, block_size: u8, card_to_controller: bool) {
        use crate::pac::sdio::dctrl::DTDIR;

        // Block Size up to 2^14 bytes
        assert!(block_size <= 14);

        // Command AND Data state machines must be idle
        loop {
            let status = self.sdio.sta().read();

            if status.cmdact().bit_is_clear()
                && status.rxact().bit_is_clear()
                && status.txact().bit_is_clear()
            {
                break;
            }
        }

        let dtdir = if card_to_controller {
            DTDIR::CardToController
        } else {
            DTDIR::ControllerToCard
        };

        // Data timeout, in bus cycles
        self.sdio.dtimer().write(|w| w.datatime().set(0xFFFF_FFFF));
        // Data length, in bytes
        self.sdio.dlen().write(|w| w.datalength().set(length_bytes));
        // Transfer
        self.sdio.dctrl().write(|w| {
            unsafe {
                w.dblocksize().bits(block_size);
            } // 2^n bytes block size
            w.dtdir().variant(dtdir);
            w.dten().enabled() // Enable transfer
        });
    }

    /// Read the state bits of the status
    fn read_status(&mut self) -> Result<CardStatus<P>, Error> {
        let card = self.card()?;

        self.cmd(common_cmd::card_status(card.get_address(), false))?;

        let r1 = self.sdio.resp1().read().bits();
        Ok(CardStatus::from(r1))
    }

    /// Check if card is done writing/reading and back in transfer state
    fn card_ready(&mut self) -> Result<bool, Error> {
        Ok(self.read_status()?.state() == CurrentState::Transfer)
    }

    /// Select the card with `address`
    fn select_card(&self, rca: u16) -> Result<(), Error> {
        let r = self.cmd(common_cmd::select_card(rca));
        match (r, rca) {
            (Err(Error::Timeout), 0) => Ok(()),
            _ => r,
        }
    }

    fn app_cmd<R: common_cmd::Resp>(&self, acmd: Cmd<R>) -> Result<(), Error> {
        let rca = self.card().map(|card| card.get_address()).unwrap_or(0);
        self.cmd(common_cmd::app_cmd(rca))?;
        self.cmd(acmd)
    }

    /// Send command to card
    fn cmd<R: common_cmd::Resp>(&self, cmd: Cmd<R>) -> Result<(), Error> {
        use crate::pac::sdio::cmd::WAITRESP;

        // Command state machines must be idle
        while self.sdio.sta().read().cmdact().bit_is_set() {}

        // Clear the interrupts before we start
        clear_all_interrupts(self.sdio.icr());

        // Command arg
        self.sdio.arg().write(|w| w.cmdarg().set(cmd.arg));

        // Determine what kind of response the CPSM should wait for
        let waitresp = match cmd.response_len() {
            ResponseLen::Zero => WAITRESP::NoResponse,
            ResponseLen::R48 => WAITRESP::ShortResponse,
            ResponseLen::R136 => WAITRESP::LongResponse,
        };

        // Send the command
        self.sdio.cmd().write(|w| {
            w.waitresp().variant(waitresp);
            w.cmdindex().set(cmd.cmd);
            w.waitint().disabled();
            w.cpsmen().enabled()
        });

        let mut timeout: u32 = 0xFFFF_FFFF;

        let status = if cmd.response_len() == ResponseLen::Zero {
            // Wait for command sent or a timeout
            loop {
                let sta = self.sdio.sta().read();

                if sta.cmdact().bit_is_clear()
                    && (sta.ctimeout().bit_is_set() || sta.cmdsent().bit_is_set())
                {
                    break sta;
                }

                if timeout == 0 {
                    return Err(Error::SoftwareTimeout);
                }

                timeout -= 1;
            }
        } else {
            loop {
                let sta = self.sdio.sta().read();

                if sta.cmdact().bit_is_clear()
                    && (sta.ctimeout().bit()
                        || sta.cmdrend().bit_is_set()
                        || sta.ccrcfail().bit_is_set())
                {
                    break sta;
                }

                if timeout == 0 {
                    return Err(Error::SoftwareTimeout);
                }

                timeout -= 1;
            }
        };

        status_to_error(status)
    }
}

impl Sdio<SdCard> {
    /// Initializes card (if present) and sets the bus at the specified frequency.
    pub fn init(&mut self, freq: ClockFreq) -> Result<(), Error> {
        // Enable power to card
        self.power_card(true);

        // Enable clock
        self.sdio.clkcr().modify(|_, w| w.clken().enabled());
        // Send card to idle state
        self.cmd(common_cmd::idle())?;

        // Check if cards supports CMD 8 (with pattern)
        self.cmd(sd_cmd::send_if_cond(1, 0xAA))?;
        let cic = CIC::from(self.sdio.resp1().read().bits());

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
            let voltage_window = 1 << 5;
            match self.app_cmd(sd_cmd::sd_send_op_cond(true, false, true, voltage_window)) {
                Ok(_) => (),
                Err(Error::Crc) => (),
                Err(err) => return Err(err),
            }
            let ocr = OCR::from(self.sdio.resp1().read().bits());
            if ocr.is_busy() {
                // Still powering up
                continue;
            }
            break ocr;
        };

        // True for SDHC and SDXC False for SDSC
        let capacity = if ocr.high_capacity() {
            CardCapacity::HighCapacity
        } else {
            CardCapacity::StandardCapacity
        };

        // Get CID
        self.cmd(common_cmd::all_send_cid())?;
        let mut cid = [0; 4];
        cid[3] = self.sdio.resp1().read().bits();
        cid[2] = self.sdio.resp2().read().bits();
        cid[1] = self.sdio.resp3().read().bits();
        cid[0] = self.sdio.resp4().read().bits();
        let cid = CID::from(cid);

        // Get RCA
        self.cmd(sd_cmd::send_relative_address())?;
        let rca = RCA::from(self.sdio.resp1().read().bits());
        let card_addr = rca.address();

        // Get CSD
        self.cmd(common_cmd::send_csd(card_addr))?;

        let mut csd = [0; 4];
        csd[3] = self.sdio.resp1().read().bits();
        csd[2] = self.sdio.resp2().read().bits();
        csd[1] = self.sdio.resp3().read().bits();
        csd[0] = self.sdio.resp4().read().bits();
        let csd = CSD::from(csd);

        self.select_card(card_addr)?;
        let scr = self.get_scr(card_addr)?;

        let card = SdCard {
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

    /// Read the SDStatus struct
    pub fn read_sd_status(&mut self) -> Result<SDStatus, Error> {
        let _card = self.card()?;
        self.cmd(common_cmd::set_block_length(64))?;
        self.start_datapath_transfer(64, 6, true);
        self.app_cmd(sd_cmd::sd_status())?;

        let mut status = [0u32; 16];
        let mut idx = 0;

        let s = loop {
            let sta = self.sdio.sta().read();

            if sta.rxact().bit_is_clear() {
                break sta;
            }

            if sta.rxfifohf().bit() {
                for _ in 0..8 {
                    status[15 - idx] = self.sdio.fifo().read().bits().swap_bytes();
                    idx += 1;
                }
            }

            if idx == status.len() {
                break sta;
            }
        };

        status_to_error(s)?;
        Ok(SDStatus::from(status))
    }

    /// Get the Card configuration for card at `address`
    fn get_scr(&self, rca: u16) -> Result<SCR, Error> {
        self.cmd(common_cmd::set_block_length(8))?;
        self.start_datapath_transfer(8, 3, true);
        self.cmd(common_cmd::app_cmd(rca))?;
        self.cmd(sd_cmd::send_scr())?;

        let mut buf = [0; 2];
        let mut i = 0;

        let status = loop {
            let sta = self.sdio.sta().read();

            if sta.rxact().bit_is_clear() {
                break sta;
            }

            if sta.rxdavl().bit() {
                buf[1 - i] = self.sdio.fifo().read().bits().swap_bytes();
                i += 1;
            }

            if i == 2 {
                break sta;
            }
        };

        status_to_error(status)?;
        Ok(SCR::from(buf))
    }

    /// Set bus width and clock frequency
    fn set_bus(&self, width: Buswidth, freq: ClockFreq) -> Result<(), Error> {
        use crate::pac::sdio::clkcr::WIDBUS;

        let card_widebus = self.card()?.supports_widebus();

        let width = match width {
            Buswidth::Buswidth4 if card_widebus => WIDBUS::BusWidth4,
            // Buswidth8 is not supported for SD cards
            _ => WIDBUS::BusWidth1,
        };

        self.app_cmd(sd_cmd::set_bus_width(width == WIDBUS::BusWidth4))?;

        self.sdio.clkcr().modify(|_, w| {
            w.clkdiv().set(freq as u8);
            w.widbus().variant(width);
            w.clken().enabled()
        });
        Ok(())
    }
}

impl Sdio<Emmc> {
    /// Initializes eMMC device (if present) and sets the bus at the specified frequency. eMMC device must support 512 byte blocks.
    pub fn init(&mut self, freq: ClockFreq) -> Result<(), Error> {
        let card_addr: RCA<EMMC> = RCA::from(1u16);

        // Enable power to card
        self.power_card(true);

        // Enable clock
        self.sdio.clkcr().modify(|_, w| w.clken().enabled());
        // Send card to idle state
        self.cmd(common_cmd::idle())?;

        let ocr = loop {
            // Initialize card

            // 3.2-3.3V
            //let voltage_window = 1 << 20;
            match self.cmd(emmc_cmd::send_op_cond(0b01000000111111111000000000000000)) {
                Ok(_) => (),
                Err(Error::Crc) => (),
                Err(err) => return Err(err),
            };
            let ocr = OCR::<EMMC>::from(self.sdio.resp1().read().bits());
            if !ocr.is_busy() {
                break ocr;
            }
        };

        // Get CID
        self.cmd(common_cmd::all_send_cid())?;
        let mut cid = [0; 4];
        cid[3] = self.sdio.resp1().read().bits();
        cid[2] = self.sdio.resp2().read().bits();
        cid[1] = self.sdio.resp3().read().bits();
        cid[0] = self.sdio.resp4().read().bits();
        let cid = CID::<EMMC>::from(cid);

        self.cmd(emmc_cmd::assign_relative_address(card_addr.address()))?;

        self.cmd(common_cmd::send_csd(card_addr.address()))?;

        let mut csd = [0; 4];
        csd[3] = self.sdio.resp1().read().bits();
        csd[2] = self.sdio.resp2().read().bits();
        csd[1] = self.sdio.resp3().read().bits();
        csd[0] = self.sdio.resp4().read().bits();
        let csd = CSD::<EMMC>::from(csd);

        self.select_card(card_addr.address())?;

        let card = Emmc {
            ocr,
            rca: card_addr,
            cid,
            csd,
        };

        self.card.replace(card);

        // Wait before setting the bus width and frequency to avoid timeouts on SDSC cards
        while !self.card_ready()? {}

        self.set_bus(self.bw, freq)?;
        Ok(())
    }

    pub fn set_bus(&mut self, width: Buswidth, freq: ClockFreq) -> Result<(), Error> {
        use crate::pac::sdio::clkcr::WIDBUS;

        // Use access mode 0b11 to write a value of 0x02 to byte 183. Cmd Set is 0 (not used).
        self.cmd(emmc_cmd::modify_ext_csd(
            emmc_cmd::AccessMode::WriteByte,
            183,
            width as u8,
        ))?;

        let width = match width {
            Buswidth::Buswidth1 => WIDBUS::BusWidth1,
            Buswidth::Buswidth4 => WIDBUS::BusWidth4,
            Buswidth::Buswidth8 => WIDBUS::BusWidth8,
        };

        // CMD6 is R1b, so wait for the card to be ready again before proceeding.
        while !self.card_ready()? {}
        self.sdio.clkcr().modify(|_, w| {
            w.clkdiv().set(freq as u8);
            w.widbus().variant(width);
            w.clken().enabled()
        });
        Ok(())
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
        w.ccrcfailc().set_bit();
        w.ctimeoutc().set_bit();
        #[cfg(not(feature = "stm32f446"))]
        w.ceataendc().set_bit();
        w.cmdrendc().set_bit();
        w.cmdsentc().set_bit();
        w.dataendc().set_bit();
        w.dbckendc().set_bit();
        w.dcrcfailc().set_bit();
        w.dtimeoutc().set_bit();
        w.sdioitc().set_bit();
        #[cfg(not(feature = "stm32f446"))]
        w.stbiterrc().set_bit();
        w.rxoverrc().set_bit();
        w.txunderrc().set_bit()
    });
}

impl SdCard {
    /// Size in blocks
    pub fn block_count(&self) -> u64 {
        self.csd.block_count()
    }

    /// Card supports wide bus
    fn supports_widebus(&self) -> bool {
        self.scr.bus_width_four()
    }
}

impl SdioPeripheral for SdCard {
    fn get_address(&self) -> u16 {
        self.rca.address()
    }
    fn get_address_mode(&self) -> AddressMode {
        match self.capacity {
            CardCapacity::StandardCapacity => AddressMode::Byte,
            CardCapacity::HighCapacity => AddressMode::Block512,
            _ => AddressMode::Block512,
        }
    }
}

impl SdioPeripheral for Emmc {
    fn get_address(&self) -> u16 {
        self.rca.address()
    }
    fn get_address_mode(&self) -> AddressMode {
        AddressMode::Block512
    }
}

pub trait SdioPeripheral {
    fn get_address(&self) -> u16;
    fn get_address_mode(&self) -> AddressMode;
}
