//! Touch sense controller
//!
//! From STM32 (https://www.st.com/content/ccc/resource/technical/document/application_note/9d/be/03/8c/5d/8c/49/50/DM00088471.pdf/files/DM00088471.pdf/jcr:content/translations/en.DM00088471.pdf):
//!
//! The Cs capacitance is a key parameter for sensitivity. For touchkey sensors, the Cs value is
//! usually comprised between 8.7nF to 22nF. For linear and rotary touch sensors, the value is
//! usually comprised between 47nF and 100nF. These values are given as reference for an
//! electrode fitting a human finger tip size across a few millimeters dielectric panel.

use crate::gpio::gpiob::{PB4, PB5, PB6, PB7};
use crate::gpio::{Alternate, OpenDrain};
use crate::pac::TSC;
use crate::rcc::{Enable, Reset, AHB1};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    /// Max count error
    MaxCountError,
    /// End of acquisition
    EndOfAcquisition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    /// Max count error
    MaxCountError,
    /// Wrong GPIO for reading - returns the ioccr register
    InvalidPin(u32),
}

pub trait SamplePin<TSC> {
    const GROUP: u32;
    const OFFSET: u32;
}
impl SamplePin<TSC> for PB4<Alternate<9, OpenDrain>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 0;
}
impl SamplePin<TSC> for PB5<Alternate<9, OpenDrain>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 1;
}
impl SamplePin<TSC> for PB6<Alternate<9, OpenDrain>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 2;
}
impl SamplePin<TSC> for PB7<Alternate<9, OpenDrain>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 3;
}

pub trait ChannelPin<TSC> {
    const GROUP: u32;
    const OFFSET: u32;
}
impl ChannelPin<TSC> for PB4<Alternate<9>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 0;
}
impl ChannelPin<TSC> for PB5<Alternate<9>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 1;
}
impl ChannelPin<TSC> for PB6<Alternate<9>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 2;
}
impl ChannelPin<TSC> for PB7<Alternate<9>> {
    const GROUP: u32 = 2;
    const OFFSET: u32 = 3;
}

pub struct Tsc<SPIN> {
    sample_pin: SPIN,
    tsc: TSC,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub clock_prescale: Option<ClockPrescaler>,
    pub max_count_error: Option<MaxCountError>,
    pub charge_transfer_high: Option<ChargeDischargeTime>,
    pub charge_transfer_low: Option<ChargeDischargeTime>,
    /// Spread spectrum deviation - a value between 0 and 128
    pub spread_spectrum_deviation: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockPrescaler {
    Hclk = 0b000,
    HclkDiv2 = 0b001,
    HclkDiv4 = 0b010,
    HclkDiv8 = 0b011,
    HclkDiv16 = 0b100,
    HclkDiv32 = 0b101,
    HclkDiv64 = 0b110,
    HclkDiv128 = 0b111,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaxCountError {
    /// 000: 255
    U255 = 0b000,
    /// 001: 511
    U511 = 0b001,
    /// 010: 1023
    U1023 = 0b010,
    /// 011: 2047
    U2047 = 0b011,
    /// 100: 4095
    U4095 = 0b100,
    /// 101: 8191
    U8191 = 0b101,
    /// 110: 16383
    U16383 = 0b110,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// How many tsc cycles are spent charging / discharging
pub enum ChargeDischargeTime {
    C1 = 0b0000,
    C2 = 0b0001,
    C3 = 0b0010,
    C4 = 0b0011,
    C5 = 0b0100,
    C6 = 0b0101,
    C7 = 0b0110,
    C8 = 0b0111,
    C9 = 0b1000,
    C10 = 0b1001,
    C11 = 0b1010,
    C12 = 0b1011,
    C13 = 0b1100,
    C14 = 0b1101,
    C15 = 0b1110,
    C16 = 0b1111,
}

impl<SPIN> Tsc<SPIN> {
    pub fn tsc(tsc: TSC, sample_pin: SPIN, ahb: &mut AHB1, cfg: Option<Config>) -> Self
    where
        SPIN: SamplePin<TSC>,
    {
        /* Enable the peripheral clock */
        TSC::enable(ahb);
        TSC::reset(ahb);

        let config = cfg.unwrap_or(Config {
            clock_prescale: None,
            max_count_error: None,
            charge_transfer_high: None,
            charge_transfer_low: None,
            spread_spectrum_deviation: None,
        });

        tsc.cr.write(|w| unsafe {
            w.ctph().bits(
                config
                    .charge_transfer_high
                    .unwrap_or(ChargeDischargeTime::C2) as u8,
            );
            w.ctpl().bits(
                config
                    .charge_transfer_low
                    .unwrap_or(ChargeDischargeTime::C2) as u8,
            );
            w.pgpsc()
                .bits(config.clock_prescale.unwrap_or(ClockPrescaler::Hclk) as u8);
            w.mcv()
                .bits(config.max_count_error.unwrap_or(MaxCountError::U8191) as u8);
            w.sse().bit(config.spread_spectrum_deviation.is_some());
            w.ssd()
                .bits(config.spread_spectrum_deviation.unwrap_or(0u8));
            w.tsce().set_bit()
        });

        let bit_pos = SPIN::OFFSET + (4 * (SPIN::GROUP - 1));

        // Schmitt trigger hysteresis on sample IOs
        tsc.iohcr.write(|w| unsafe { w.bits(1 << bit_pos) });

        // Set the sampling pin
        tsc.ioscr.write(|w| unsafe { w.bits(1 << bit_pos) });

        // set the acquisitiuon groups based of the channel pins, stm32l432xx only has group 2
        tsc.iogcsr.write(|w| w.g2e().set_bit());

        // clear interrupt & flags
        tsc.icr.write(|w| w.eoaic().set_bit().mceic().set_bit());

        Tsc { tsc, sample_pin }
    }

    /// Starts a charge acquisition
    pub fn start<PIN>(&self, _input: &mut PIN)
    where
        PIN: ChannelPin<TSC>,
    {
        self.clear(Event::EndOfAcquisition);
        self.clear(Event::MaxCountError);

        // discharge the caps ready for a new reading
        self.tsc.cr.modify(|_, w| w.iodef().clear_bit());

        let bit_pos = PIN::OFFSET + (4 * (PIN::GROUP - 1));

        // Set the channel pin
        self.tsc.ioccr.write(|w| unsafe { w.bits(1 << bit_pos) });

        self.tsc.cr.modify(|_, w| w.start().set_bit());
    }

    /// Clear interrupt & flags
    pub fn clear(&self, event: Event) {
        match event {
            Event::EndOfAcquisition => {
                self.tsc.icr.write(|w| w.eoaic().set_bit());
            }
            Event::MaxCountError => {
                self.tsc.icr.write(|w| w.mceic().set_bit());
            }
        }
    }

    /// Blocks waiting for a acquisition to complete or for a Max Count Error
    pub fn acquire<PIN>(&self, input: &mut PIN) -> Result<u16, Error>
    where
        PIN: ChannelPin<TSC>,
    {
        // start the acq
        self.start(input);

        let result = loop {
            let isr = self.tsc.isr.read();
            if isr.eoaf().bit_is_set() {
                self.tsc.icr.write(|w| w.eoaic().set_bit());
                break Ok(self.read_unchecked());
            } else if isr.mcef().bit_is_set() {
                self.tsc.icr.write(|w| w.mceic().set_bit());
                break Err(Error::MaxCountError);
            }
        };
        self.tsc.ioccr.write(|w| unsafe { w.bits(0b0) }); // clear channel register
        result
    }

    /// Reads the tsc group 2 count register
    pub fn read<PIN>(&self, _input: &mut PIN) -> Result<u16, Error>
    where
        PIN: ChannelPin<TSC>,
    {
        let bit_pos = PIN::OFFSET + (4 * (PIN::GROUP - 1));
        // Read the current channel config
        let channel = self.tsc.ioccr.read().bits();
        // if they are equal we have the right pin
        if channel == (1 << bit_pos) {
            Ok(self.read_unchecked())
        } else {
            Err(Error::InvalidPin(channel))
        }
    }

    /// Reads the tsc group 2 count register
    /// WARNING, just returns the contents of the register! No validation of the correct pin
    pub fn read_unchecked(&self) -> u16 {
        self.tsc.iog2cr().read().cnt().bits()
    }

    /// Is the tsc performing an aquisition
    pub fn in_progress(&mut self) -> bool {
        self.tsc.cr.read().start().bit_is_set()
    }

    /// Enables an interrupt event
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::EndOfAcquisition => {
                self.tsc.ier.modify(|_, w| w.eoaie().set_bit());
            }
            Event::MaxCountError => {
                self.tsc.ier.modify(|_, w| w.mceie().set_bit());
            }
        }
    }

    /// Disables an interrupt event
    pub fn unlisten(&self, event: Event) {
        match event {
            Event::EndOfAcquisition => {
                self.tsc.ier.modify(|_, w| w.eoaie().clear_bit());
            }
            Event::MaxCountError => {
                self.tsc.ier.modify(|_, w| w.mceie().clear_bit());
            }
        }
    }

    /// Releases the TSC peripheral and associated pins
    pub fn free(self) -> (TSC, SPIN) {
        (self.tsc, self.sample_pin)
    }
}
