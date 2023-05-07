use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr;

#[cfg(feature = "dma")]
use crate::dma::{
    traits::{DMASet, PeriAddress},
    MemoryToPeripheral, PeripheralToMemory,
};
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

use crate::pac::spi1;
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
pub struct Inner<SPI: Instance> {
    spi: SPI,
}

/// Spi in Master mode
#[derive(Debug)]
pub struct Spi<SPI: Instance, const BIDI: bool = false, W = u8> {
    inner: Inner<SPI>,
    pins: (SPI::Sck, SPI::Miso, SPI::Mosi),
    _operation: PhantomData<W>,
}

impl<SPI: Instance, const BIDI: bool, W> Deref for Spi<SPI, BIDI, W> {
    type Target = Inner<SPI>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<SPI: Instance, const BIDI: bool, W> DerefMut for Spi<SPI, BIDI, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Spi in Slave mode
#[derive(Debug)]
pub struct SpiSlave<SPI: Instance, const BIDI: bool = false, W = u8> {
    inner: Inner<SPI>,
    pins: (SPI::Sck, SPI::Miso, SPI::Mosi, Option<SPI::Nss>),
    _operation: PhantomData<W>,
}

impl<SPI: Instance, const BIDI: bool, W> Deref for SpiSlave<SPI, BIDI, W> {
    type Target = Inner<SPI>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<SPI: Instance, const BIDI: bool, W> DerefMut for SpiSlave<SPI, BIDI, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implemented by all SPI instances
pub trait Instance:
    crate::Sealed
    + Deref<Target = spi1::RegisterBlock>
    + rcc::Enable
    + rcc::Reset
    + rcc::BusClock
    + gpio::alt::SpiCommon
{
    #[doc(hidden)]
    fn ptr() -> *const spi1::RegisterBlock;
}

// Implemented by all SPI instances
macro_rules! spi {
    ($SPI:ty: $Spi:ident, $SpiSlave:ident) => {
        pub type $Spi<const BIDI: bool = false, W = u8> = Spi<$SPI, BIDI, W>;
        pub type $SpiSlave<const BIDI: bool = false, W = u8> = Spi<$SPI, BIDI, W>;

        impl Instance for $SPI {
            fn ptr() -> *const spi1::RegisterBlock {
                <$SPI>::ptr() as *const _
            }
        }
    };
}

#[cfg(feature = "spi1")]
spi! { pac::SPI1: Spi1, SpiSlave1 }
#[cfg(feature = "spi2")]
#[cfg(not(any(feature = "svd-f750", feature = "svd-f7x6")))]
spi! { pac::SPI2: Spi2, SpiSlave2 }
#[cfg(feature = "spi3")]
#[cfg(not(any(feature = "svd-f745", feature = "svd-f765")))]
spi! { pac::SPI3: Spi3, SpiSlave3 }
#[cfg(feature = "spi4")]
spi! { pac::SPI4: Spi4, SpiSlave4 }
#[cfg(feature = "spi5")]
spi! { pac::SPI5: Spi5, SpiSlave5 }
#[cfg(feature = "spi6")]
#[cfg(not(any(feature = "svd-f745", feature = "svd-f765")))]
spi! { pac::SPI6: Spi6, SpiSlave6 }

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
    ) -> Spi<Self, false, u8>;

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
    ) -> Spi<Self, true, u8>;

    fn spi_slave(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
            Option<Self::Nss>,
        ),
        mode: impl Into<Mode>,
    ) -> SpiSlave<Self, false, u8>;

    fn spi_bidi_slave(
        self,
        pins: (
            impl Into<Self::Sck>,
            impl Into<Self::Miso>,
            impl Into<Self::Mosi>,
            Option<Self::Nss>,
        ),
        mode: impl Into<Mode>,
    ) -> SpiSlave<Self, true, u8>;
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
    ) -> Spi<Self, false, u8> {
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
    ) -> Spi<Self, true, u8> {
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
            Option<Self::Nss>,
        ),
        mode: impl Into<Mode>,
    ) -> SpiSlave<Self, false, u8> {
        SpiSlave::new(self, pins, mode)
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
            Option<Self::Nss>,
        ),
        mode: impl Into<Mode>,
    ) -> SpiSlave<Self, true, u8> {
        SpiSlave::new_bidi(self, pins, mode)
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize> Spi<SPI, BIDI, W> {
    pub fn init(self) -> Self {
        self.spi.cr1.modify(|_, w| {
            // bidimode: 2-line or 1-line unidirectional
            w.bidimode().bit(BIDI);
            w.bidioe().bit(BIDI);
            // data frame size
            #[cfg(any(feature = "f4", feature = "l4p"))]
            w.dff().bit(W::DFF);
            #[cfg(any(feature = "f7", feature = "l4x"))]
            w.crcl().bit(W::DFF);
            // spe: enable the SPI bus
            w.spe().set_bit()
        });

        self
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize> SpiSlave<SPI, BIDI, W> {
    pub fn init(self) -> Self {
        self.spi.cr1.modify(|_, w| {
            // bidimode: 2-line or 1-line unidirectional
            w.bidimode().bit(BIDI);
            w.bidioe().bit(BIDI);
            // data frame size
            #[cfg(any(feature = "f4", feature = "l4p"))]
            w.dff().bit(W::DFF);
            #[cfg(any(feature = "f7", feature = "l4x"))]
            w.crcl().bit(W::DFF);
            // spe: enable the SPI bus
            w.spe().set_bit()
        });

        self
    }
}

impl<SPI: Instance, W: FrameSize> Spi<SPI, false, W> {
    pub fn to_bidi_transfer_mode(self) -> Spi<SPI, true, W> {
        self.into_mode()
    }
}

impl<SPI: Instance, W: FrameSize> Spi<SPI, true, W> {
    pub fn to_normal_transfer_mode(self) -> Spi<SPI, false, W> {
        self.into_mode()
    }
}

impl<SPI: Instance, W: FrameSize> SpiSlave<SPI, false, W> {
    pub fn to_bidi_transfer_mode(self) -> SpiSlave<SPI, true, W> {
        self.into_mode()
    }
}

impl<SPI: Instance, W: FrameSize> SpiSlave<SPI, true, W> {
    pub fn to_normal_transfer_mode(self) -> SpiSlave<SPI, false, W> {
        self.into_mode()
    }
}

impl<SPI, const BIDI: bool> Spi<SPI, BIDI, u8>
where
    SPI: Instance,
{
    /// Converts from 8bit dataframe to 16bit.
    pub fn frame_size_16bit(self) -> Spi<SPI, BIDI, u16> {
        self.into_mode()
    }
}

impl<SPI, const BIDI: bool> Spi<SPI, BIDI, u16>
where
    SPI: Instance,
{
    /// Converts from 16bit dataframe to 8bit.
    pub fn frame_size_8bit(self) -> Spi<SPI, BIDI, u8> {
        self.into_mode()
    }
}

impl<SPI, const BIDI: bool> SpiSlave<SPI, BIDI, u8>
where
    SPI: Instance,
{
    /// Converts from 8bit dataframe to 16bit.
    pub fn frame_size_16bit(self) -> SpiSlave<SPI, BIDI, u16> {
        self.into_mode()
    }
}

impl<SPI, const BIDI: bool> SpiSlave<SPI, BIDI, u16>
where
    SPI: Instance,
{
    /// Converts from 16bit dataframe to 8bit.
    pub fn frame_size_8bit(self) -> SpiSlave<SPI, BIDI, u8> {
        self.into_mode()
    }
}

impl<SPI: Instance> Spi<SPI, false, u8> {
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
            SPI::enable_unchecked();
            SPI::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into());

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks))
            .init()
    }
}

impl<SPI: Instance> Spi<SPI, true, u8> {
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
            SPI::enable_unchecked();
            SPI::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into());

        Self::_new(spi, pins)
            .pre_init(mode.into(), freq, SPI::clock(clocks))
            .init()
    }
}

impl<SPI: Instance> SpiSlave<SPI, false, u8> {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Slave Normal mode.
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
            Option<SPI::Nss>,
        ),
        mode: impl Into<Mode>,
    ) -> Self {
        unsafe {
            SPI::enable_unchecked();
            SPI::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into(), pins.3);

        Self::_new(spi, pins).pre_init(mode.into()).init()
    }
}

impl<SPI: Instance> SpiSlave<SPI, true, u8> {
    /// Enables the SPI clock, resets the peripheral, sets `Alternate` mode for `pins` and initialize the peripheral as SPI Slave BIDI mode.
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
            Option<SPI::Nss>,
        ),
        mode: impl Into<Mode>,
    ) -> Self {
        unsafe {
            SPI::enable_unchecked();
            SPI::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into(), pins.2.into(), pins.3);

        Self::_new(spi, pins).pre_init(mode.into()).init()
    }
}

impl<SPI: Instance, const BIDI: bool, W> Spi<SPI, BIDI, W> {
    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (SPI, (SPI::Sck, SPI::Miso, SPI::Mosi)) {
        (self.inner.spi, self.pins)
    }
}

impl<SPI: Instance, const BIDI: bool, W> SpiSlave<SPI, BIDI, W> {
    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (SPI, (SPI::Sck, SPI::Miso, SPI::Mosi, Option<SPI::Nss>)) {
        (self.inner.spi, self.pins)
    }
}

impl<SPI: Instance, const BIDI: bool, W> Spi<SPI, BIDI, W> {
    fn _new(spi: SPI, pins: (SPI::Sck, SPI::Miso, SPI::Mosi)) -> Self {
        Self {
            inner: Inner::new(spi),
            pins,
            _operation: PhantomData,
        }
    }

    /// Convert the spi to another mode.
    fn into_mode<const BIDI2: bool, W2: FrameSize>(self) -> Spi<SPI, BIDI2, W2> {
        let mut spi = Spi::_new(self.inner.spi, self.pins);
        spi.enable(false);
        spi.init()
    }
}

impl<SPI: Instance, const BIDI: bool, W> SpiSlave<SPI, BIDI, W> {
    fn _new(spi: SPI, pins: (SPI::Sck, SPI::Miso, SPI::Mosi, Option<SPI::Nss>)) -> Self {
        Self {
            inner: Inner::new(spi),
            pins,
            _operation: PhantomData,
        }
    }

    /// Convert the spi to another mode.
    fn into_mode<const BIDI2: bool, W2: FrameSize>(self) -> SpiSlave<SPI, BIDI2, W2> {
        let mut spi = SpiSlave::_new(self.inner.spi, self.pins);
        spi.enable(false);
        spi.init()
    }
}

impl<SPI: Instance, const BIDI: bool, W> Spi<SPI, BIDI, W> {
    /// Pre initializing the SPI bus.
    fn pre_init(self, mode: Mode, freq: Hertz, clock: Hertz) -> Self {
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
            w.mstr().set_bit();
            #[allow(unused_unsafe)]
            unsafe {
                w.br().bits(br);
            }
            // lsbfirst: MSB first
            w.lsbfirst().clear_bit();
            // ssm: enable software slave management (NSS pin free for other uses)
            w.ssm().set_bit();
            // ssi: set nss high
            w.ssi().set_bit();
            w.rxonly().clear_bit();
            // dff: 8 bit frames
            #[cfg(any(feature = "f4", feature = "l4p"))]
            w.dff().clear_bit();
            #[cfg(any(feature = "f7", feature = "l4x"))]
            w.crcl().clear_bit();
            w
        });

        self
    }
}

impl<SPI: Instance, const BIDI: bool, W> SpiSlave<SPI, BIDI, W> {
    /// Pre initializing the SPI bus.
    fn pre_init(self, mode: Mode) -> Self {
        self.spi.cr1.write(|w| {
            w.cpha().bit(mode.phase == Phase::CaptureOnSecondTransition);
            w.cpol().bit(mode.polarity == Polarity::IdleHigh);
            // mstr: slave configuration
            w.mstr().clear_bit();
            #[allow(unused_unsafe)]
            unsafe {
                w.br().bits(0);
            }
            // lsbfirst: MSB first
            w.lsbfirst().clear_bit();
            // ssm: enable software slave management (NSS pin free for other uses)
            w.ssm().bit(self.pins.3.is_none());
            // ssi: set nss high = master mode
            w.ssi().set_bit();
            w.rxonly().clear_bit();
            // dff: 8 bit frames
            #[cfg(any(feature = "f4", feature = "l4p"))]
            w.dff().clear_bit();
            #[cfg(any(feature = "f7", feature = "l4x"))]
            w.crcl().clear_bit();
            w
        });

        self
    }

    /// Set the slave select bit programmatically.
    #[inline]
    pub fn set_internal_nss(&mut self, value: bool) {
        self.spi.cr1.modify(|_, w| w.ssi().bit(value));
    }
}

impl<SPI: Instance> Inner<SPI> {
    fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Enable/disable spi
    pub fn enable(&mut self, enable: bool) {
        self.spi.cr1.modify(|_, w| {
            // spe: enable the SPI bus
            w.spe().bit(enable)
        });
    }

    /// Select which frame format is used for data transfers
    pub fn bit_format(&mut self, format: BitFormat) {
        self.spi
            .cr1
            .modify(|_, w| w.lsbfirst().bit(format == BitFormat::LsbFirst));
    }

    /// Enable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn listen(&mut self, event: Event) {
        self.spi.cr2.modify(|_, w| match event {
            Event::Rxne => w.rxneie().set_bit(),
            Event::Txe => w.txeie().set_bit(),
            Event::Error => w.errie().set_bit(),
        })
    }

    /// Disable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn unlisten(&mut self, event: Event) {
        self.spi.cr2.modify(|_, w| match event {
            Event::Rxne => w.rxneie().clear_bit(),
            Event::Txe => w.txeie().clear_bit(),
            Event::Error => w.errie().clear_bit(),
        })
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

    fn read_data_reg<W: FrameSize>(&mut self) -> W {
        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
        // reading a half-word)
        unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const W) }
    }

    fn write_data_reg<W: FrameSize>(&mut self, data: W) {
        // NOTE(write_volatile) see note above
        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut W, data) }
    }

    #[inline(always)]
    fn check_read<W: FrameSize>(&mut self) -> nb::Result<W, Error> {
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
    fn check_send<W: FrameSize>(&mut self, byte: W) -> nb::Result<(), Error> {
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
            self.spi.sr.modify(|_r, w| w.crcerr().clear_bit());
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

impl<SPI: Instance, const BIDI: bool> Spi<SPI, BIDI, u8> {
    pub fn use_dma(self) -> DmaBuilder<SPI> {
        DmaBuilder {
            spi: self.inner.spi,
        }
    }
}

impl<SPI: Instance, const BIDI: bool> SpiSlave<SPI, BIDI, u8> {
    pub fn use_dma(self) -> DmaBuilder<SPI> {
        DmaBuilder {
            spi: self.inner.spi,
        }
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
        self.spi.cr2.modify(|_, w| w.txdmaen().set_bit());
        Tx { spi: PhantomData }
    }

    pub fn rx(self) -> Rx<SPI> {
        self.spi.cr2.modify(|_, w| w.rxdmaen().set_bit());
        Rx { spi: PhantomData }
    }

    pub fn txrx(self) -> (Tx<SPI>, Rx<SPI>) {
        self.spi.cr2.modify(|_, w| {
            w.txdmaen().set_bit();
            w.rxdmaen().set_bit()
        });
        (Tx { spi: PhantomData }, Rx { spi: PhantomData })
    }
}

#[cfg(feature = "dma")]
unsafe impl<SPI: Instance> PeriAddress for Rx<SPI> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*SPI::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}

#[cfg(feature = "dma")]
unsafe impl<SPI, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory> for Rx<SPI> where
    SPI: DMASet<STREAM, CHANNEL, PeripheralToMemory>
{
}

#[cfg(feature = "dma")]
unsafe impl<SPI: Instance> PeriAddress for Tx<SPI> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*SPI::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}

#[cfg(feature = "dma")]
unsafe impl<SPI, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral> for Tx<SPI> where
    SPI: DMASet<STREAM, CHANNEL, MemoryToPeripheral>
{
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize> Spi<SPI, BIDI, W> {
    pub fn read_nonblocking(&mut self) -> nb::Result<W, Error> {
        if BIDI {
            self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
        }
        self.check_read()
    }

    pub fn write_nonblocking(&mut self, byte: W) -> nb::Result<(), Error> {
        if BIDI {
            self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
        }
        self.check_send(byte)
    }

    pub fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Error> {
        for word in words {
            nb::block!(self.write_nonblocking(*word))?;
            *word = nb::block!(self.read_nonblocking())?;
        }

        Ok(())
    }

    pub fn transfer(&mut self, buff: &mut [W], data: &[W]) -> Result<(), Error> {
        assert_eq!(data.len(), buff.len());

        for (d, b) in data.iter().cloned().zip(buff.iter_mut()) {
            nb::block!(self.write_nonblocking(d))?;
            *b = nb::block!(self.read_nonblocking())?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn write(&mut self, words: &[W]) -> Result<(), Error> {
        for word in words {
            nb::block!(self.write_nonblocking(*word))?;
            if !BIDI {
                nb::block!(self.read_nonblocking())?;
            }
        }

        Ok(())
    }

    pub fn read(&mut self, words: &mut [W]) -> Result<(), Error> {
        for word in words {
            nb::block!(self.write_nonblocking(W::default()))?;
            *word = nb::block!(self.read_nonblocking())?;
        }

        Ok(())
    }
}

impl<SPI: Instance, const BIDI: bool, W: FrameSize> SpiSlave<SPI, BIDI, W> {
    pub fn read_nonblocking(&mut self) -> nb::Result<W, Error> {
        if BIDI {
            self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
        }
        self.check_read()
    }

    pub fn write_nonblocking(&mut self, byte: W) -> nb::Result<(), Error> {
        if BIDI {
            self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
        }
        self.check_send(byte)
    }

    pub fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Error> {
        for word in words {
            nb::block!(self.write_nonblocking(*word))?;
            *word = nb::block!(self.read_nonblocking())?;
        }

        Ok(())
    }

    pub fn transfer(&mut self, buff: &mut [W], data: &[W]) -> Result<(), Error> {
        assert_eq!(data.len(), buff.len());

        for (d, b) in data.iter().cloned().zip(buff.iter_mut()) {
            nb::block!(self.write_nonblocking(d))?;
            *b = nb::block!(self.read_nonblocking())?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn write(&mut self, words: &[W]) -> Result<(), Error> {
        for word in words {
            nb::block!(self.write_nonblocking(*word))?;
            if !BIDI {
                nb::block!(self.read_nonblocking())?;
            }
        }

        Ok(())
    }

    pub fn read(&mut self, words: &mut [W]) -> Result<(), Error> {
        for word in words {
            nb::block!(self.write_nonblocking(W::default()))?;
            *word = nb::block!(self.read_nonblocking())?;
        }

        Ok(())
    }
}
