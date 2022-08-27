use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr;

use crate::dma::traits::PeriAddress;
use crate::gpio::{Const, NoPin, PinA, PushPull, SetAlternate};
use crate::pac;

/// Clock polarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    /// Clock signal low when idle
    IdleLow,
    /// Clock signal high when idle
    IdleHigh,
}

/// Clock phase
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Data in "captured" on the first clock transition
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition
    CaptureOnSecondTransition,
}

/// SPI mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mode {
    /// Clock polarity
    pub polarity: Polarity,
    /// Clock phase
    pub phase: Phase,
}

mod hal_02;
mod hal_1;

use crate::pac::{spi1, RCC};
use crate::rcc;

use crate::rcc::Clocks;
use fugit::HertzU32 as Hertz;

/// SPI error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
}

pub struct Sck;
impl crate::Sealed for Sck {}
pub struct Miso;
impl crate::Sealed for Miso {}
pub struct Mosi;
impl crate::Sealed for Mosi {}
pub struct Nss;
impl crate::Sealed for Nss {}

pub trait Pins<SPI> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}

impl<SPI, SCK, MISO, MOSI, const SCKA: u8, const MISOA: u8, const MOSIA: u8> Pins<SPI>
    for (SCK, MISO, MOSI)
where
    SCK: PinA<Sck, SPI, A = Const<SCKA>> + SetAlternate<SCKA, PushPull>,
    MISO: PinA<Miso, SPI, A = Const<MISOA>> + SetAlternate<MISOA, PushPull>,
    MOSI: PinA<Mosi, SPI, A = Const<MOSIA>> + SetAlternate<MOSIA, PushPull>,
{
    fn set_alt_mode(&mut self) {
        self.0.set_alt_mode();
        self.1.set_alt_mode();
        self.2.set_alt_mode();
    }
    fn restore_mode(&mut self) {
        self.0.restore_mode();
        self.1.restore_mode();
        self.2.restore_mode();
    }
}

/// A filler type for when the SCK pin is unnecessary
pub type NoSck = NoPin;
/// A filler type for when the Miso pin is unnecessary
pub type NoMiso = NoPin;
/// A filler type for when the Mosi pin is unnecessary
pub type NoMosi = NoPin;

/// Interrupt events
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Event {
    /// New data has been received
    Rxne,
    /// Data can be sent
    Txe,
    /// An error occurred
    Error,
}

/// Normal mode - RX and TX pins are independent
#[allow(non_upper_case_globals)]
pub const TransferModeNormal: bool = false;
/// BIDI mode - use TX pin as RX then spi receive data
#[allow(non_upper_case_globals)]
pub const TransferModeBidi: bool = true;

/// Spi in Master mode (type state)
pub struct Master;
/// Spi in Slave mode (type state)
pub struct Slave;

#[derive(Debug)]
pub struct Spi<SPI, PINS, const BIDI: bool = false, OPERATION = Master> {
    spi: SPI,
    pins: PINS,
    _operation: PhantomData<OPERATION>,
}

// Implemented by all SPI instances
pub trait Instance:
    crate::Sealed + Deref<Target = spi1::RegisterBlock> + rcc::Enable + rcc::Reset + rcc::BusClock
{
    #[doc(hidden)]
    fn ptr() -> *const spi1::RegisterBlock;
}

// Implemented by all SPI instances
macro_rules! spi {
    ($SPI:ty: $Spi:ident) => {
        pub type $Spi<PINS, const BIDI: bool = false, OPERATION = Master> =
            Spi<$SPI, PINS, BIDI, OPERATION>;

        impl Instance for $SPI {
            fn ptr() -> *const spi1::RegisterBlock {
                <$SPI>::ptr() as *const _
            }
        }
    };
}

spi! { pac::SPI1: Spi1 }
spi! { pac::SPI2: Spi2 }

#[cfg(feature = "spi3")]
spi! { pac::SPI3: Spi3 }

#[cfg(feature = "spi4")]
spi! { pac::SPI4: Spi4 }

#[cfg(feature = "spi5")]
spi! { pac::SPI5: Spi5 }

#[cfg(feature = "spi6")]
spi! { pac::SPI6: Spi6 }

pub trait SpiExt: Sized + Instance {
    fn spi<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), false, Master>
    where
        (SCK, MISO, MOSI): Pins<Self>;
    fn spi_bidi<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), true, Master>
    where
        (SCK, MISO, MOSI): Pins<Self>;
    fn spi_slave<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), false, Slave>
    where
        (SCK, MISO, MOSI): Pins<Self>;
    fn spi_bidi_slave<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), true, Slave>
    where
        (SCK, MISO, MOSI): Pins<Self>;
}

impl<SPI: Instance> SpiExt for SPI {
    fn spi<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), false, Master>
    where
        (SCK, MISO, MOSI): Pins<Self>,
    {
        Spi::new(self, pins, mode, freq, clocks)
    }
    fn spi_bidi<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), true, Master>
    where
        (SCK, MISO, MOSI): Pins<Self>,
    {
        Spi::new_bidi(self, pins, mode, freq, clocks)
    }
    fn spi_slave<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), false, Slave>
    where
        (SCK, MISO, MOSI): Pins<Self>,
    {
        Spi::new_slave(self, pins, mode, freq, clocks)
    }
    fn spi_bidi_slave<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, (SCK, MISO, MOSI), true, Slave>
    where
        (SCK, MISO, MOSI): Pins<Self>,
    {
        Spi::new_bidi_slave(self, pins, mode, freq, clocks)
    }
}

impl<SPI: Instance, PINS, const BIDI: bool, OPERATION> Spi<SPI, PINS, BIDI, OPERATION> {
    pub fn init(self) -> Self {
        self.spi.cr1.modify(|_, w| {
            // bidimode: 2-line or 1-line unidirectional
            w.bidimode().bit(BIDI);
            w.bidioe().bit(BIDI);
            // spe: enable the SPI bus
            w.spe().set_bit()
        });

        self
    }
}

impl<SPI: Instance, PINS, OPERATION> Spi<SPI, PINS, false, OPERATION> {
    pub fn to_bidi_transfer_mode(self) -> Spi<SPI, PINS, true, OPERATION> {
        let mut dev_w_new_t_mode = self.into_mode::<true>();
        dev_w_new_t_mode.enable(false);
        dev_w_new_t_mode.init()
    }
}

impl<SPI: Instance, PINS, OPERATION> Spi<SPI, PINS, true, OPERATION> {
    pub fn to_normal_transfer_mode(self) -> Spi<SPI, PINS, false, OPERATION> {
        let mut dev_w_new_t_mode = self.into_mode::<false>();
        dev_w_new_t_mode.enable(false);
        dev_w_new_t_mode.init()
    }
}

impl<SPI: Instance, PINS, const BIDI: bool> Spi<SPI, PINS, BIDI, Master> {
    pub fn to_slave_operation(self) -> Spi<SPI, PINS, BIDI, Slave> {
        let mut dev_w_new_operation = self.into_operation::<Slave>();
        dev_w_new_operation.enable(false);
        dev_w_new_operation.init()
    }
}

impl<SPI: Instance, PINS, const BIDI: bool> Spi<SPI, PINS, BIDI, Slave> {
    pub fn to_master_operation(self) -> Spi<SPI, PINS, BIDI, Master> {
        let mut dev_w_new_operation = self.into_operation::<Master>();
        dev_w_new_operation.enable(false);
        dev_w_new_operation.init()
    }
}

impl<SPI: Instance, SCK, MISO, MOSI> Spi<SPI, (SCK, MISO, MOSI), false, Master> {
    pub fn new(
        spi: SPI,
        mut pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self
    where
        (SCK, MISO, MOSI): Pins<SPI>,
    {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.set_alt_mode();

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), true)
            .init()
    }
}

impl<SPI: Instance, SCK, MISO, MOSI> Spi<SPI, (SCK, MISO, MOSI), true, Master> {
    pub fn new_bidi(
        spi: SPI,
        mut pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self
    where
        (SCK, MISO, MOSI): Pins<SPI>,
    {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.set_alt_mode();

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), true)
            .init()
    }
}

impl<SPI: Instance, SCK, MISO, MOSI> Spi<SPI, (SCK, MISO, MOSI), false, Slave> {
    pub fn new_slave(
        spi: SPI,
        mut pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self
    where
        (SCK, MISO, MOSI): Pins<SPI>,
    {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.set_alt_mode();

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), false)
            .init()
    }
}

impl<SPI: Instance, SCK, MISO, MOSI> Spi<SPI, (SCK, MISO, MOSI), true, Slave> {
    pub fn new_bidi_slave(
        spi: SPI,
        mut pins: (SCK, MISO, MOSI),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self
    where
        (SCK, MISO, MOSI): Pins<SPI>,
    {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.set_alt_mode();

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), false)
            .init()
    }
}

impl<SPI, SCK, MISO, MOSI, const BIDI: bool, OPERATION> Spi<SPI, (SCK, MISO, MOSI), BIDI, OPERATION>
where
    SPI: Instance,
    (SCK, MISO, MOSI): Pins<SPI>,
{
    pub fn release(mut self) -> (SPI, (SCK, MISO, MOSI)) {
        self.pins.restore_mode();

        (self.spi, (self.pins.0, self.pins.1, self.pins.2))
    }
}

impl<SPI: Instance, PINS, const BIDI: bool, OPERATION> Spi<SPI, PINS, BIDI, OPERATION> {
    fn _new(spi: SPI, pins: PINS) -> Self {
        Self {
            spi,
            pins,
            _operation: PhantomData,
        }
    }

    /// Convert the spi to another transfer mode.
    fn into_mode<const BIDI2: bool>(self) -> Spi<SPI, PINS, BIDI2, OPERATION> {
        Spi::_new(self.spi, self.pins)
    }

    /// Convert the spi to another operation mode.
    fn into_operation<OPERATION2>(self) -> Spi<SPI, PINS, BIDI, OPERATION2> {
        Spi::_new(self.spi, self.pins)
    }

    /// Enable/disable spi
    pub fn enable(&mut self, enable: bool) {
        self.spi.cr1.modify(|_, w| {
            // spe: enable the SPI bus
            w.spe().bit(enable)
        });
    }

    /// Pre initializing the SPI bus.
    fn pre_init(self, mode: Mode, freq: Hertz, clock: Hertz, is_master: bool) -> Self {
        // disable SS output
        self.spi.cr2.write(|w| w.ssoe().clear_bit());

        let br = match clock.raw() / freq.raw() {
            0 => unreachable!(),
            1..=2 => 0b000,
            3..=5 => 0b001,
            6..=11 => 0b010,
            12..=23 => 0b011,
            24..=47 => 0b100,
            48..=95 => 0b101,
            96..=191 => 0b110,
            _ => 0b111,
        };

        self.spi.cr1.write(|w| {
            w.cpha().bit(mode.phase == Phase::CaptureOnSecondTransition);
            w.cpol().bit(mode.polarity == Polarity::IdleHigh);
            // mstr: master configuration
            w.mstr().bit(is_master);
            w.br().bits(br);
            // lsbfirst: MSB first
            w.lsbfirst().clear_bit();
            // ssm: enable software slave management (NSS pin free for other uses)
            w.ssm().set_bit();
            // ssi: set nss high = master mode
            w.ssi().bit(is_master);
            w.rxonly().clear_bit();
            // dff: 8 bit frames
            w.dff().clear_bit()
        });

        self
    }

    /// Enable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => self.spi.cr2.modify(|_, w| w.rxneie().set_bit()),
            Event::Txe => self.spi.cr2.modify(|_, w| w.txeie().set_bit()),
            Event::Error => self.spi.cr2.modify(|_, w| w.errie().set_bit()),
        }
    }

    /// Disable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => self.spi.cr2.modify(|_, w| w.rxneie().clear_bit()),
            Event::Txe => self.spi.cr2.modify(|_, w| w.txeie().clear_bit()),
            Event::Error => self.spi.cr2.modify(|_, w| w.errie().clear_bit()),
        }
    }

    /// Return `true` if the TXE flag is set, i.e. new data to transmit
    /// can be written to the SPI.
    pub fn is_tx_empty(&self) -> bool {
        self.spi.sr.read().txe().bit_is_set()
    }

    /// Return `true` if the RXNE flag is set, i.e. new data has been received
    /// and can be read from the SPI.
    pub fn is_rx_not_empty(&self) -> bool {
        self.spi.sr.read().rxne().bit_is_set()
    }

    /// Return `true` if the MODF flag is set, i.e. the SPI has experienced a
    /// Master Mode Fault. (see chapter 28.3.10 of the STM32F4 Reference Manual)
    pub fn is_modf(&self) -> bool {
        self.spi.sr.read().modf().bit_is_set()
    }

    /// Returns true if the transfer is in progress
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.spi.sr.read().bsy().bit_is_set()
    }

    /// Return `true` if the OVR flag is set, i.e. new data has been received
    /// while the receive data register was already filled.
    pub fn is_overrun(&self) -> bool {
        self.spi.sr.read().ovr().bit_is_set()
    }

    pub fn use_dma(self) -> DmaBuilder<SPI> {
        DmaBuilder { spi: self.spi }
    }

    #[inline(always)]
    fn check_read(&mut self) -> nb::Result<u8, Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            Error::Overrun.into()
        } else if sr.modf().bit_is_set() {
            Error::ModeFault.into()
        } else if sr.crcerr().bit_is_set() {
            Error::Crc.into()
        } else if sr.rxne().bit_is_set() {
            return Ok(self.read_u8());
        } else {
            nb::Error::WouldBlock
        })
    }

    #[inline(always)]
    fn check_send(&mut self, byte: u8) -> nb::Result<(), Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            // Read from the DR to clear the OVR bit
            let _ = self.spi.dr.read();
            Error::Overrun.into()
        } else if sr.modf().bit_is_set() {
            // Write to CR1 to clear MODF
            self.spi.cr1.modify(|_r, w| w);
            Error::ModeFault.into()
        } else if sr.crcerr().bit_is_set() {
            // Clear the CRCERR bit
            self.spi.sr.modify(|_r, w| {
                w.crcerr().clear_bit();
                w
            });
            Error::Crc.into()
        } else if sr.txe().bit_is_set() {
            self.send_u8(byte);
            return Ok(());
        } else {
            nb::Error::WouldBlock
        })
    }

    #[inline(always)]
    fn read_u8(&mut self) -> u8 {
        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows reading a half-word)
        unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const u8) }
    }

    #[inline(always)]
    fn send_u8(&mut self, byte: u8) {
        // NOTE(write_volatile) see note above
        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }
    }
}

pub struct DmaBuilder<SPI> {
    spi: SPI,
}

pub struct Tx<SPI> {
    spi: PhantomData<SPI>,
}

pub struct Rx<SPI> {
    spi: PhantomData<SPI>,
}

impl<SPI: Instance> DmaBuilder<SPI> {
    pub fn tx(self) -> Tx<SPI> {
        self.new_tx()
    }

    pub fn rx(self) -> Rx<SPI> {
        self.new_rx()
    }

    pub fn txrx(self) -> (Tx<SPI>, Rx<SPI>) {
        (self.new_tx(), self.new_rx())
    }

    fn new_tx(&self) -> Tx<SPI> {
        self.spi.cr2.modify(|_, w| w.txdmaen().enabled());
        Tx { spi: PhantomData }
    }

    fn new_rx(self) -> Rx<SPI> {
        self.spi.cr2.modify(|_, w| w.rxdmaen().enabled());
        Rx { spi: PhantomData }
    }
}

unsafe impl<SPI: Instance> PeriAddress for Rx<SPI> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*SPI::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}

unsafe impl<SPI: Instance> PeriAddress for Tx<SPI> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*SPI::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}
