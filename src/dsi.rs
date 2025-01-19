//! Display Serial Interface
//!
//! Interface with MIPI D-PHY

use crate::ltdc::DisplayConfig;
use crate::rcc::{Clocks, Enable};
use crate::{pac::DSI, time::Hertz};
use core::cmp::{max, min};
use embedded_display_controller::dsi::{DsiHostCtrlIo, DsiReadCommand, DsiWriteCommand};

const DSI_TIMEOUT_MS: usize = 100;

pub struct DsiHost {
    dsi: DSI,
    channel: DsiChannel,
    cycles_1ms: u32,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsiChannel {
    Ch0 = 0b00,
    Ch1 = 0b01,
    Ch2 = 0b10,
    Ch3 = 0b11,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    RegTimeout,
    PllTimeout,
    BufferIsToBig,
    WriteTimeout,
    ReadTimeout,
    ReadError,
    FifoTimeout,
    WrongId,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsiMode {
    Video { mode: DsiVideoMode },
    AdaptedCommand { tear_effect: Option<TearEffectMode> },
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsiVideoMode {
    NonBurstWithSyncPulses = 0b00,
    NonBurstWithSyncEvents = 0b01,
    Burst = 0b10,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TearEffectMode {
    pub source: TearEffectSource,
    pub auto_refresh: bool,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TearEffectSource {
    DsiLink,
    ExternalPin,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsiInterrupts {
    None,
    All,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsiCmdModeTransmissionKind {
    AllInHighSpeed,
    AllInLowPower,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DsiPhyTimers {
    pub dataline_hs2lp: u8,
    pub dataline_lp2hs: u8,
    pub clock_hs2lp: u16,
    pub clock_lp2hs: u16,
    pub dataline_max_read_time: u16,
    pub stop_wait_time: u8,
}

pub struct DsiRefreshHandle {
    dsi: DSI,
    // refresh_request: *mut bool,
}
// unsafe impl Send for DsiRefreshHandle {}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneCount {
    SingleLane = 0b00,
    DoubleLane = 0b01,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DsiPllConfig {
    ndiv: u8,
    idf: u8,
    odf: u8,
    eckdiv: u8,
}

impl DsiPllConfig {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn manual(ndiv: u8, idf: u8, odf: u8, eckdiv: u8) -> Self {
        DsiPllConfig {
            ndiv,
            idf,
            odf,
            eckdiv,
        }
    }
}

#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorCoding {
    SixteenBitsConfig1 = 0b000,
    SixteenBitsConfig2 = 0b001,
    SixteenBitsConfig3 = 0b010,
    EighteenBitsConfig1 = 0b011,
    EighteenBitsConfig2 = 0b100,
    TwentyFourBits = 0b101,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DsiConfig {
    pub mode: DsiMode,
    pub lane_count: LaneCount,
    pub channel: DsiChannel,
    pub hse_freq: Hertz,
    pub ltdc_freq: Hertz,
    pub interrupts: DsiInterrupts,
    pub color_coding_host: ColorCoding,
    pub color_coding_wrapper: ColorCoding,
    pub lp_size: u8,
    pub vlp_size: u8,
}

impl DsiHost {
    pub fn init(
        pll_config: DsiPllConfig,
        display_config: DisplayConfig,
        dsi_config: DsiConfig,
        dsi: DSI,
        clocks: &Clocks,
    ) -> Result<DsiHost, Error> {
        unsafe {
            DSI::enable_unchecked();
        }

        // Bring DSI peripheral out of reset
        dsi.cr().modify(|_, w| w.en().set_bit());

        //RCC_D1CCIPR: DSI clock from PHY is selected as DSI byte lane clock (default after reset)
        let cycles_1ms = clocks.sysclk().raw() / 1_000;

        // Enable regulator
        dsi.wrpcr().modify(|_, w| w.regen().set_bit());
        // Wait for it to be ready
        block_with_timeout(
            || dsi.wisr().read().rrs().bit_is_clear(),
            DSI_TIMEOUT_MS,
            cycles_1ms,
            Error::RegTimeout,
        )?;

        // Set PLL division factors
        // Fin = 25MHz ->/idf = 5MHz ->*2 = 10MHz ->*ndiv = 1GHz ->/2 = 500MHz ->/odf = 500MHz ->/8 = 62.5MHz
        // let ndiv = 125;
        // let ndiv = 102;
        // let idf  = 5;
        // let odf = 0b00;
        dsi.wrpcr().modify(|_, w| unsafe {
            w.ndiv()
                .bits(pll_config.ndiv) // allowed: 10 ..= 125
                .idf()
                .bits(pll_config.idf) // div1: 0b000, 0b001, div2: 0b010, div3: 0b011 ..= div7
                .odf()
                .bits(pll_config.odf) // div1: 0b00, div2: 0b01, div4: 0b10, div8: 0b11
        });
        // Enable PLL
        dsi.wrpcr().modify(|_, w| w.pllen().set_bit());
        // Required to wait 400us before checking PLLLS flag
        cortex_m::asm::delay(cycles_1ms / 2);
        // Wait for the lock
        block_with_timeout(
            || dsi.wisr().read().pllls().bit_is_clear(),
            DSI_TIMEOUT_MS,
            cycles_1ms,
            Error::PllTimeout,
        )?;

        // Clock and digital section enable
        dsi.pctlr().modify(|_, w| w.cke().set_bit().den().set_bit());

        // Clock lane config
        dsi.clcr().modify(
            |_, w| {
                w.dpcc()
                    .set_bit() // 1: lanes are running in high speed mode
                    .acr()
                    .clear_bit()
            }, // Automatically stop lanes clock when "time allows"
        );

        // Configure the number of active data lanes
        dsi.pconfr()
            .modify(|_, w| unsafe { w.nl().bits(dsi_config.lane_count as u8) }); // 0b00 - 1 lanes, 0b01 - 2 lanes

        // Set TX escape clock division factor
        dsi.ccr()
            .modify(|_, w| unsafe { w.txeckdiv().bits(pll_config.eckdiv) });

        // Set the bit period in high speed mode
        // Calculate the bit period in high-speed mode in unit of 0.25 ns (UIX4)
        // The equation is : UIX4 = IntegerPart( (1000/F_PHY_Mhz) * 4 )
        // Where : F_PHY_Mhz = (NDIV * HSE_Mhz) / (IDF * ODF)
        let odf = match pll_config.odf {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            0b11 => 8,
            _ => unreachable!(),
        };
        let f_phy_hz = ((pll_config.ndiv as u32) * dsi_config.hse_freq.raw())
            / u32::from(pll_config.idf)
            / odf;
        let f_pix_khz = f_phy_hz / 1_000 / 8;
        let uix4 = 4_000_000_000 / f_phy_hz;
        dsi.wpcr0()
            .modify(|_, w| unsafe { w.uix4().bits(uix4 as u8) });

        match dsi_config.interrupts {
            DsiInterrupts::None => {
                // Disable all error interrupts for now and reset the error mask
                dsi.ier0().write(|w| unsafe { w.bits(0) });
                dsi.ier1().write(|w| unsafe { w.bits(0) });
            }
            DsiInterrupts::All => {
                // Enable all error interrupts
                dsi.ier0()
                    .write(|w| unsafe { w.bits(0b00000000_00011111_11111111_11111111) });
                dsi.ier1().write(|w| unsafe { w.bits(0b00011111_11111111) });

                // Enable wrapper interrupts
                dsi.wier().write(|w| w.teie().set_bit().erie().set_bit());
            }
        }

        match dsi_config.mode {
            DsiMode::Video { mode } => {
                // Select video mode
                dsi.mcr().modify(|_, w| w.cmdm().clear_bit()); // 0 - video mode, 1 - command mode
                dsi.wcfgr().modify(|_, w| {
                    w
                        // 0 - video mode, 1 - adapted command mode
                        .dsim()
                        .clear_bit()
                        // 0 - DSI Link, 1 - External pin
                        .tesrc()
                        .clear_bit()
                        // 0 - Rising edge, 1 - Falling edge
                        .tepol()
                        .clear_bit()
                        // Refresh mode in DBI mode, 0 - disabled, 1 - automatic refresh enabled
                        .ar()
                        .clear_bit()
                });

                // Video mode transmission type, p. 1346
                dsi.vmcr().modify(|_, w| unsafe {
                    w.vmt()
                        .bits(mode as u8) // 0b00 - non-burst with sync pulses, 0b01 - non-burst with sync event, 0b1x - burst mode
                        .lpvsae()
                        .set_bit() // Enable LP transition in vertical sync period
                        .lpvbpe()
                        .set_bit() // Enable LP transition in VBP period
                        .lpvfpe()
                        .set_bit() // Enable LP transition in VFP period
                        .lpvae()
                        .set_bit() // Enable LP transition in VACT period
                        .lphbpe()
                        .set_bit() // Enable LP transition in HBP period
                        .lphfpe()
                        .set_bit() // Enable LP transition in HFP period
                        .lpce()
                        .set_bit() // 1 = Command transmission in low power mode enabled
                        .fbtaae()
                        .clear_bit() // Disable the request for an acknowledge response at the end of a frame
                });

                // Packet size, 14 bits max
                // TODO: Might be incorrect for 16 or 18bit
                dsi.vpcr()
                    .modify(|_, w| unsafe { w.vpsize().bits(display_config.active_width) });

                // TODO: Unhardcode?
                // This register configures the number of chunks to be transmitted during a line period (a chunk
                // consists of a video packet and a null packet).
                // If set to 0 or 1, the video line is transmitted in a single packet.
                // If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise,
                // multiple chunks are used to transmit each video line.
                dsi.vccr().modify(|_, w| unsafe { w.numc().bits(1) });

                // Size of the null packet
                dsi.vnpcr().modify(|_, w| unsafe { w.npsize().bits(0) });

                // Horizontal sync active (HSA) in lane byte clock cycles
                let f_ltdc_khz = dsi_config.ltdc_freq.to_kHz();
                let hsa = ((display_config.h_sync as u32) * f_pix_khz / f_ltdc_khz) as u16;
                dsi.vhsacr().modify(|_, w| unsafe { w.hsa().bits(hsa) });

                // Horizontal back porch (HBP) in lane byte clock cycles
                let hbp = ((display_config.h_back_porch as u32) * f_pix_khz / f_ltdc_khz) as u16;
                dsi.vhbpcr().modify(|_, w| unsafe { w.hbp().bits(hbp) });

                // Total line time, HLINE = HSA + HBP + HACT + HFP
                let hline = display_config.h_sync
                    + display_config.h_back_porch
                    + display_config.active_width
                    + display_config.h_front_porch;
                let hline = ((hline as u32) * f_pix_khz / f_ltdc_khz) as u16;
                // let hsync = f_phy * 3 * hline as u32 / 8;
                dsi.vlcr().modify(|_, w| unsafe { w.hline().bits(hline) });

                // Vertical sync active (VSA)
                dsi.vvsacr()
                    .modify(|_, w| unsafe { w.vsa().bits(display_config.v_sync) });

                // Vertical back porch (VBP)
                dsi.vvbpcr()
                    .modify(|_, w| unsafe { w.vbp().bits(display_config.v_back_porch) });

                // Vertical front porch (VFP)
                dsi.vvfpcr()
                    .modify(|_, w| unsafe { w.vfp().bits(display_config.v_front_porch) });

                // Vertical active period
                dsi.vvacr()
                    .modify(|_, w| unsafe { w.va().bits(display_config.active_height) });
            }
            DsiMode::AdaptedCommand { tear_effect } => {
                // Select command mode
                dsi.mcr().modify(|_, w| w.cmdm().set_bit()); // 0 - video mode, 1 - command mode
                let (is_external_pin, auto_refresh) = match tear_effect {
                    Some(te) => (te.source == TearEffectSource::ExternalPin, te.auto_refresh),
                    None => (false, false),
                };
                dsi.wcfgr().modify(|_, w| {
                    w
                        // 0 - video mode, 1 - adapted command mode
                        .dsim()
                        .set_bit()
                        // 0 - DSI Link, 1 - External pin
                        .tesrc()
                        .bit(is_external_pin)
                        // 0 - Rising edge, 1 - Falling edge
                        .tepol()
                        .clear_bit()
                        // Refresh mode in DBI mode, 0 - disabled, 1 - automatic refresh enabled
                        .ar()
                        .bit(auto_refresh)
                        // VSync polarity, 0 - LTDC halted on falling edge, 1 - LTDC halted on rising edge
                        .vspol()
                        .clear_bit()
                });

                // Maximum allowed size for memory write command
                dsi.lccr()
                    .modify(|_, w| unsafe { w.cmdsize().bits(display_config.active_width) });

                // Tearing effect acknowledge request
                dsi.cmcr().modify(|_, w| w.teare().set_bit());
            }
        }

        // Select virtual channel for the LTDC interface traffic
        dsi.lvcidr()
            .modify(|_, w| unsafe { w.vcid().bits(dsi_config.channel as u8) });

        // Polarity
        dsi.lpcr()
            .modify(|_, w| w.dep().clear_bit().vsp().clear_bit().hsp().clear_bit());

        // Color coding for the host
        let lpe = matches!(
            dsi_config.color_coding_host,
            ColorCoding::EighteenBitsConfig1 | ColorCoding::EighteenBitsConfig2
        );
        dsi.lcolcr().modify(|_, w| unsafe {
            w.lpe()
                .bit(lpe) // loosely packed: 18bits
                .colc()
                .bits(dsi_config.color_coding_host as u8) // 0: 16bit_1, 1: 16bit_2, 2: 16bit_3, 3: 18bit_1, 4: 18bit_2, 5: 24bit
        });

        // Color coding for the wrapper
        dsi.wcfgr()
            .modify(|_, w| unsafe { w.colmux().bits(dsi_config.color_coding_wrapper as u8) });

        dsi.lpmcr().modify(|_, w| unsafe {
            w.lpsize()
                .bits(dsi_config.lp_size) // Low power largest packet size
                .vlpsize()
                .bits(dsi_config.vlp_size) // Low power VACT largest packet size
        });

        Ok(DsiHost {
            dsi,
            channel: dsi_config.channel,
            cycles_1ms,
        })
    }

    pub fn set_command_mode_transmission_kind(&mut self, kind: DsiCmdModeTransmissionKind) {
        let is_low_power = match kind {
            DsiCmdModeTransmissionKind::AllInHighSpeed => false,
            DsiCmdModeTransmissionKind::AllInLowPower => true,
        };
        self.dsi.cmcr().modify(|_, w| {
            w.gsw0tx()
                .bit(is_low_power)
                .gsw1tx()
                .bit(is_low_power)
                .gsw2tx()
                .bit(is_low_power)
                .gsr0tx()
                .bit(is_low_power)
                .gsr1tx()
                .bit(is_low_power)
                .gsr2tx()
                .bit(is_low_power)
                .glwtx()
                .bit(is_low_power)
                .dsw0tx()
                .bit(is_low_power)
                .dsw1tx()
                .bit(is_low_power)
                .dsr0tx()
                .bit(is_low_power)
                .dlwtx()
                .bit(is_low_power)
                .mrdps()
                .bit(is_low_power)
        });
        self.dsi.cmcr().modify(|_, w| w.are().clear_bit()); // FIXME: might be incorrect
    }

    pub fn configure_phy_timers(&mut self, phy_timers: DsiPhyTimers) {
        let max_time = max(phy_timers.clock_lp2hs, phy_timers.clock_hs2lp);
        self.dsi
            .cltcr()
            .modify(|_, w| unsafe { w.hs2lp_time().bits(max_time).lp2hs_time().bits(max_time) });
        self.dsi.dltcr().modify(|_, w| unsafe {
            w.mrd_time().bits(phy_timers.dataline_max_read_time);
            w.hs2lp_time().bits(phy_timers.dataline_hs2lp);
            w.lp2hs_time().bits(phy_timers.dataline_lp2hs)
        });
        self.dsi
            .pconfr()
            .modify(|_, w| unsafe { w.sw_time().bits(phy_timers.stop_wait_time) });
    }

    pub fn force_rx_low_power(&mut self, force: bool) {
        self.dsi.wpcr1().modify(|_, w| w.flprxlpm().bit(force));
    }

    fn long_write(&mut self, cmd: u8, buf: &[u8], ghcr_dt: u8) -> Result<(), Error> {
        // debug!("{}, long {dcs_cmd:02x}, {buf:02x?}", self.write_idx);
        // self.write_idx += 1;

        if buf.len() >= 65_535 {
            // TODO: is it correct length?
            return Err(Error::BufferIsToBig);
        }

        // Put dcs_command and up to 3 bytes of data to GPDR
        let mut fifoword = u32::from(cmd);
        for (i, byte) in buf.iter().take(3).enumerate() {
            fifoword |= (*byte as u32) << (8 + 8 * i);
        }
        self.dsi.gpdr().write(|w| unsafe { w.bits(fifoword) });
        //debug!("gpdr = {fifoword:08x}");

        // Put the rest of the data, assuming that GPDR is accumulated in the hardware in some buffer.
        if buf.len() > 3 {
            let mut iter = buf[3..].chunks_exact(4);
            for chunk in &mut iter {
                let fifoword: [u8; 4] = chunk.try_into().unwrap();
                let fifoword = u32::from_ne_bytes(fifoword); //.swap_bytes();
                self.dsi.gpdr().write(|w| unsafe { w.bits(fifoword) });
                //debug!("gpdr = {fifoword:08x}");
            }
            if !iter.remainder().is_empty() {
                let mut fifoword = 0u32;
                for (i, byte) in iter.remainder().iter().enumerate() {
                    fifoword |= (*byte as u32) << (8 * i);
                }
                self.dsi.gpdr().write(|w| unsafe { w.bits(fifoword) });
                //debug!("gpdr = {fifoword:08x}");
            }
        }

        let len = buf.len() + 1; // dcs_cmd + actual data
        self.ghcr_write(((len >> 8) & 0xff) as u8, (len & 0xff) as u8, ghcr_dt);

        Ok(())
    }

    fn ghcr_write(&mut self, msb: u8, lsb: u8, dt: u8) {
        self.dsi.ghcr().write(|w| unsafe {
            w // GHCR p. 1354
                .wcmsb()
                .bits(msb)
                .wclsb()
                .bits(lsb)
                .vcid()
                .bits(self.channel as u8)
                .dt()
                .bits(dt)
        });
    }

    pub fn start(&mut self) {
        self.dsi.cr().modify(|_, w| w.en().set_bit());
        self.dsi.wcr().modify(|_, w| w.dsien().set_bit());
    }

    pub fn refresh(&mut self) {
        self.dsi.wcr().modify(|_, w| w.ltdcen().set_bit());
    }

    pub fn refresh_handle(&self) -> DsiRefreshHandle {
        let dsi = unsafe { crate::pac::Peripherals::steal().DSI };
        DsiRefreshHandle { dsi }
    }

    pub fn enable_bus_turn_around(&mut self) {
        self.dsi.pcr().modify(|_, w| w.btae().set_bit()); // Enable bus turn around
    }

    pub fn enable_color_test(&mut self) {
        self.dsi
            .vmcr()
            .modify(|_, w| w.pge().set_bit().pgm().clear_bit());
    }

    pub fn enable_ber_test(&mut self) {
        self.dsi
            .vmcr()
            .modify(|_, w| w.pge().set_bit().pgm().set_bit());
    }
}

impl DsiRefreshHandle {
    pub fn refresh_now(&mut self) {
        self.dsi.wcr().modify(|_, w| w.ltdcen().set_bit());
    }

    // pub fn refresh_when_te_happens(&mut self) {
    //     cortex_m::interrupt::free(|_| unsafe {
    //         *self.refresh_request = true;
    //     })
    // }
}

impl DsiHostCtrlIo for DsiHost {
    type Error = Error;

    fn write(&mut self, kind: DsiWriteCommand) -> Result<(), Error> {
        // debug!("DSI write: {:x?}", kind);
        // wait for command fifo to be empty
        block_with_timeout(
            || self.dsi.gpsr().read().cmdfe().bit_is_clear(),
            DSI_TIMEOUT_MS,
            self.cycles_1ms,
            Error::FifoTimeout,
        )?;
        match kind {
            DsiWriteCommand::DcsShortP0 { .. } => todo!(),
            DsiWriteCommand::DcsShortP1 { arg, data } => {
                // debug!("{}, short_p1: reg: {reg:02x}, data: {data:02x}", self.write_idx);
                // self.write_idx += 1;
                self.ghcr_write(data, arg, kind.discriminant());
            }
            DsiWriteCommand::DcsLongWrite { arg, data } => {
                self.long_write(arg, data, kind.discriminant())?
            }
            DsiWriteCommand::GenericShortP0 => todo!(),
            DsiWriteCommand::GenericShortP1 => todo!(),
            DsiWriteCommand::GenericShortP2 => todo!(),
            DsiWriteCommand::GenericLongWrite { arg, data } => {
                self.long_write(arg, data, kind.discriminant())?
            }
            DsiWriteCommand::SetMaximumReturnPacketSize(len) => {
                self.ghcr_write(
                    ((len >> 8) & 0xff) as u8,
                    (len & 0xff) as u8,
                    kind.discriminant(),
                );
            }
        }
        Ok(())
    }

    fn read(&mut self, kind: DsiReadCommand, buf: &mut [u8]) -> Result<(), Error> {
        // println!("DSI read: {:x?}", kind);
        if buf.len() > 2 && buf.len() <= 65_535 {
            self.write(DsiWriteCommand::SetMaximumReturnPacketSize(buf.len() as u16))?;
        } else if buf.len() > 65_535 {
            return Err(Error::BufferIsToBig);
        }

        match kind {
            DsiReadCommand::DcsShort { arg } => {
                self.ghcr_write(0, arg, kind.discriminant());
            }
            DsiReadCommand::GenericShortP0 => {
                self.ghcr_write(0, 0, kind.discriminant());
            }
            DsiReadCommand::GenericShortP1 { arg0 } => {
                self.ghcr_write(0, arg0, kind.discriminant());
            }
            DsiReadCommand::GenericShortP2 { arg0, arg1 } => {
                self.ghcr_write(arg1, arg0, kind.discriminant());
            }
        }

        let mut idx = 0;
        let mut bytes_left = buf.len();
        block_with_timeout(
            || {
                if bytes_left > 0 {
                    if self.dsi.gpsr().read().prdfe().bit_is_clear() {
                        // GPSR: p. 1355
                        let fifoword = self.dsi.gpdr().read().bits();
                        //debug!("fifoword read: {fifoword:08x}");
                        for b in fifoword
                            // .swap_bytes()
                            .to_ne_bytes()
                            .iter()
                            .take(min(bytes_left, 4))
                        {
                            buf[idx] = *b;
                            bytes_left -= 1;
                            idx += 1;
                        }
                    }
                    // Software workaround to avoid HAL_TIMEOUT when a DSI read command is
                    // issued to the panel and the read data is not captured by the DSI Host
                    // which returns Packet Size Error.
                    // Need to ensure that the Read command has finished before checking PSE
                    if self.dsi.gpsr().read().rcb().bit_is_clear()
                        && self.dsi.isr1().read().pse().bit_is_set()
                    {
                        return false;
                    }
                    true
                } else {
                    false
                }
            },
            DSI_TIMEOUT_MS,
            self.cycles_1ms,
            Error::ReadTimeout,
        )
        .map_err(|_| Error::ReadTimeout)?;
        if bytes_left > 0 {
            return Err(Error::ReadError);
        }
        Ok(())
    }
}

fn block_with_timeout<F: FnMut() -> bool>(
    mut f: F,
    retries: usize,
    delay_cycles: u32,
    err: Error,
) -> Result<(), Error> {
    for _ in 0..retries {
        if f() {
            cortex_m::asm::delay(delay_cycles);
        } else {
            return Ok(());
        }
    }
    //debug!("{name} {}", self.tim.counter());
    Err(err)
}
