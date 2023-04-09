use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr;

use crate::dma::traits::PeriAddress;
use crate::gpio::{self, NoPin};
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

pub trait Ms {
    const MSTR: bool;
}

impl Ms for Slave {
    const MSTR: bool = false;
}

impl Ms for Master {
    const MSTR: bool = true;
}

pub trait FrameSize: Copy + Default {
    const DFF: bool;
}

impl FrameSize for u8 {
    const DFF: bool = false;
}

impl FrameSize for u16 {
    const DFF: bool = true;
}

/// The bit format to send the data in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitFormat {
    /// Least significant bit first
    LsbFirst,
    /// Most significant bit first
    MsbFirst,
}

#[derive(Debug)]
pub struct Spi<SPI: Instance, const BIDI: bool = false, W = u8, OPERATION = Master> {
    spi: SPI,
    pins: (SPI::Sck, SPI::Miso, SPI::Mosi),
    _operation: PhantomData<(W, OPERATION)>,
}

// Implemented by all SPI instances
pub trait Instance:
    crate::Sealed + Deref<Target = spi1::RegisterBlock> + rcc::Enable + rcc::Reset + rcc::BusClock
{
    type Sck;
    type Miso;
    type Mosi;
    type Nss;

    #[doc(hidden)]
    fn ptr() -> *const spi1::RegisterBlock;
}

// Implemented by all SPI instances
macro_rules! spi {
    ($SPI:ty: $Spi:ident, $spi:ident) => {
        pub type $Spi<const BIDI: bool = false, W = u8, OPERATION = Master> =
            Spi<$SPI, BIDI, W, OPERATION>;

        impl Instance for $SPI {
            type Sck = gpio::alt::$spi::Sck;
            type Miso = gpio::alt::$spi::Miso;
            type Mosi = gpio::alt::$spi::Mosi;
            type Nss = gpio::alt::$spi::Nss;

            fn ptr() -> *const spi1::RegisterBlock {
                <$SPI>::ptr() as *const _
            }
        }
    };
}

spi! { pac::SPI1: Spi1, spi1 }
spi! { pac::SPI2: Spi2, spi2 }

#[cfg(feature = "spi3")]
spi! { pac::SPI3: Spi3, spi3 }

#[cfg(feature = "spi4")]
spi! { pac::SPI4: Spi4, spi4 }

#[cfg(feature = "spi5")]
spi! { pac::SPI5: Spi5, spi5 }

#[cfg(feature = "spi6")]
spi! { pac::SPI6: Spi6, spi6 }

pub trait SpiExt: Sized + Instance {
    fn spi(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, false, u8, Master>;

    fn spi_bidi(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, true, u8, Master>;

    fn spi_slave(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, false, u8, Slave>;

    fn spi_bidi_slave(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, true, u8, Slave>;
}

impl<SPI: Instance> SpiExt for SPI {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Master Normal mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    fn spi(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, false, u8, Master> {
        Spi::new(self, pins, mode, freq, clocks)
    }
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Master BIDI mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    fn spi_bidi(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, true, u8, Master> {
        Spi::new_bidi(self, pins, mode, freq, clocks)
    }
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Slave Normal mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    fn spi_slave(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, false, u8, Slave> {
        Spi::new_slave(self, pins, mode, freq, clocks)
    }
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Slave BIDI mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    fn spi_bidi_slave(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Spi<Self, true, u8, Slave> {
        Spi::new_bidi_slave(self, pins, mode, freq, clocks)
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize, OPERATION: Ms> Spi<SPI, BIDI, W, OPERATION> {
    pub fn init(self) -> Self {
        self.spi.cr1.modify(|_, w| {
            // bidimode: 2-line or 1-line unidirectional
            w.bidimode().bit(BIDI);
            w.bidioe().bit(BIDI);
            // master/slave mode
            w.mstr().bit(OPERATION::MSTR);
            // data frame size
            w.dff().bit(W::DFF);
            // spe: enable the SPI bus
            w.spe().set_bit()
        });

        self
    }
}

impl<SPI: Instance, W: FrameSize, OPERATION: Ms> Spi<SPI, false, W, OPERATION> {
    pub fn to_bidi_transfer_mode(self) -> Spi<SPI, true, W, OPERATION> {
        self.into_mode()
    }
}

impl<SPI: Instance, W: FrameSize, OPERATION: Ms> Spi<SPI, true, W, OPERATION> {
    pub fn to_normal_transfer_mode(self) -> Spi<SPI, false, W, OPERATION> {
        self.into_mode()
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize> Spi<SPI, BIDI, W, Master> {
    pub fn to_slave_operation(self) -> Spi<SPI, BIDI, W, Slave> {
        self.into_mode()
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize> Spi<SPI, BIDI, W, Slave> {
    pub fn to_master_operation(self) -> Spi<SPI, BIDI, W, Master> {
        self.into_mode()
    }
}

impl<SPI, const BIDI: bool, OPERATION: Ms> Spi<SPI, BIDI, u8, OPERATION>
where
    SPI: Instance,
{
    /// Converts from 8bit dataframe to 16bit.
    pub fn frame_size_16bit(self) -> Spi<SPI, BIDI, u16, OPERATION> {
        self.into_mode()
    }
}

impl<SPI, const BIDI: bool, OPERATION: Ms> Spi<SPI, BIDI, u16, OPERATION>
where
    SPI: Instance,
{
    /// Converts from 16bit dataframe to 8bit.
    pub fn frame_size_8bit(self) -> Spi<SPI, BIDI, u8, OPERATION> {
        self.into_mode()
    }
}

impl<SPI: Instance> Spi<SPI, false, u8, Master> {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Master Normal mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    pub fn new(
        spi: SPI,
        pins: (
            impl Into<SPI::Sck>,
            impl Into<SPI::Miso>,
            impl Into<SPI::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into());

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), true)
            .init()
    }
}

impl<SPI: Instance> Spi<SPI, true, u8, Master> {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Master BIDI mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    pub fn new_bidi(
        spi: SPI,
        pins: (
            impl Into<SPI::Sck>,
            impl Into<SPI::Miso>,
            impl Into<SPI::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into());

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), true)
            .init()
    }
}

impl<SPI: Instance> Spi<SPI, false, u8, Slave> {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Slave Normal mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    pub fn new_slave(
        spi: SPI,
        pins: (
            impl Into<SPI::Sck>,
            impl Into<SPI::Miso>,
            impl Into<SPI::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into());

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), false)
            .init()
    }
}

impl<SPI: Instance> Spi<SPI, true, u8, Slave> {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Slave BIDI mode.
    ///
    /// # Note
    /// Depending on `freq` you may need to set GPIO speed for `pins` (the `Speed::Low` is default for GPIO) before create `Spi` instance.
    /// Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    pub fn new_bidi_slave(
        spi: SPI,
        pins: (
            impl Into<SPI::Sck>,
            impl Into<SPI::Miso>,
            impl Into<SPI::Mosi>,
        ),
        mode: impl Into<Mode>,
        freq: Hertz,
        clocks: &Clocks,
    ) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into());

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks), false)
            .init()
    }
}

impl<SPI: Instance, const BIDI: bool, W, OPERATION> Spi<SPI, BIDI, W, OPERATION> {
    pub fn release<SCK, MISO, MOSI, E>(self) -> Result<(SPI, (SCK, MISO, MOSI)), E>
    where
        SCK: TryFrom<SPI::Sck, Error = E>,
        MISO: TryFrom<SPI::Miso, Error = E>,
        MOSI: TryFrom<SPI::Mosi, Error = E>,
    {
        Ok((
            self.spi,
            (
                self.pins.0.try_into()?,
                self.pins.1.try_into()?,
                self.pins.2.try_into()?,
            ),
        ))
    }
}

impl<SPI: Instance, const BIDI: bool, W, OPERATION> Spi<SPI, BIDI, W, OPERATION> {
    fn _new(spi: SPI, pins: (SPI::Sck, SPI::Miso, SPI::Mosi)) -> Self {
        Self {
            spi,
            pins,
            _operation: PhantomData,
        }
    }

    /// Convert the spi to another mode.
    fn into_mode<const BIDI2: bool, W2: FrameSize, OPERATION2: Ms>(
        self,
    ) -> Spi<SPI, BIDI2, W2, OPERATION2> {
        let mut spi = Spi::_new(self.spi, self.pins);
        spi.enable(false);
        spi.init()
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

    /// Select which frame format is used for data transfers
    pub fn bit_format(&mut self, format: BitFormat) {
        match format {
            BitFormat::LsbFirst => self.spi.cr1.modify(|_, w| w.lsbfirst().set_bit()),
            BitFormat::MsbFirst => self.spi.cr1.modify(|_, w| w.lsbfirst().clear_bit()),
        }
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
    #[inline]
    pub fn is_tx_empty(&self) -> bool {
        self.spi.sr.read().txe().bit_is_set()
    }
    #[inline]
    #[deprecated(since = "0.14.0", note = "please use `is_tx_empty` instead")]
    pub fn is_txe(&self) -> bool {
        self.is_tx_empty()
    }

    /// Return `true` if the RXNE flag is set, i.e. new data has been received
    /// and can be read from the SPI.
    #[inline]
    pub fn is_rx_not_empty(&self) -> bool {
        self.spi.sr.read().rxne().bit_is_set()
    }

    #[inline]
    #[deprecated(since = "0.14.0", note = "please use `is_rx_not_empty` instead")]
    pub fn is_rxne(&self) -> bool {
        self.is_rx_not_empty()
    }

    /// Return `true` if the MODF flag is set, i.e. the SPI has experienced a
    /// Master Mode Fault. (see chapter 28.3.10 of the STM32F4 Reference Manual)
    #[inline]
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
    #[inline]
    pub fn is_overrun(&self) -> bool {
        self.spi.sr.read().ovr().bit_is_set()
    }

    /// Set the slave select bit programmatically.
    #[inline]
    pub fn set_internal_nss(&mut self, value: bool) {
        self.spi.cr1.modify(|_, w| w.ssi().bit(value));
    }
}

trait ReadWriteReg<W> {
    fn read_data_reg(&mut self) -> W;
    fn write_data_reg(&mut self, data: W);
}

impl<SPI, const BIDI: bool, W, OPERATION> ReadWriteReg<W> for Spi<SPI, BIDI, W, OPERATION>
where
    SPI: Instance,
    W: FrameSize,
{
    fn read_data_reg(&mut self) -> W {
        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
        // reading a half-word)
        unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const W) }
    }

    fn write_data_reg(&mut self, data: W) {
        // NOTE(write_volatile) see note above
        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut W, data) }
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize, OPERATION> Spi<SPI, BIDI, W, OPERATION> {
    #[inline(always)]
    fn check_read(&mut self) -> nb::Result<W, Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            Error::Overrun.into()
        } else if sr.modf().bit_is_set() {
            Error::ModeFault.into()
        } else if sr.crcerr().bit_is_set() {
            Error::Crc.into()
        } else if sr.rxne().bit_is_set() {
            return Ok(self.read_data_reg());
        } else {
            nb::Error::WouldBlock
        })
    }

    #[inline(always)]
    fn check_send(&mut self, byte: W) -> nb::Result<(), Error> {
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
            self.write_data_reg(byte);
            return Ok(());
        } else {
            nb::Error::WouldBlock
        })
    }
}

// Spi DMA

impl<SPI: Instance, const BIDI: bool, OPERATION> Spi<SPI, BIDI, u8, OPERATION> {
    pub fn use_dma(self) -> DmaBuilder<SPI> {
        DmaBuilder { spi: self.spi }
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
