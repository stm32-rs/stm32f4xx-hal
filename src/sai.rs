//! Serial Audio Interface.
//!
//! Each SAI instance consists of two sub-blocks (SAIA and SAIB), each of which can be configured
//! completely independently. Optionally, one block can be configured to be synchronous to the
//! other, in which no clock and word select pins are required.
//!
//! This implementation supports PCM audio with short and long frame synchronization signals as
//! well as I2S and its left-justified and right-justified variations.
//!
//! # Usage Examples
//!
//! See examples/sai-duplex.rs
//!
//! # Clock Selection
//!
//! TODO
//!

// TODO: Unify capitalization of template parameters.
// TODO: Synchronization of multiple SAIs.

use core::ops::Deref;

use crate::gpio::alt::SaiChannel;
use crate::pac::RCC;
#[cfg(feature = "sai2")]
use crate::pac::SAI2;
#[cfg(any(
    feature = "gpio-f413",
    feature = "gpio-f469",
    feature = "stm32f429",
    feature = "stm32f439"
))]
use crate::pac::{sai, SAI as SAI1};
#[cfg(any(feature = "stm32f427", feature = "stm32f437", feature = "stm32f446"))]
use crate::pac::{sai1 as sai, SAI1};
use crate::rcc;
use crate::time::Hertz;

pub use sai::ch::{cr1::DS as WordSize, slotr::SLOTSZ as SlotSize};

fn word_size(ws: WordSize) -> u8 {
    match ws {
        WordSize::Bit8 => 8,
        WordSize::Bit10 => 10,
        WordSize::Bit16 => 16,
        WordSize::Bit20 => 20,
        WordSize::Bit24 => 24,
        WordSize::Bit32 => 32,
    }
}

fn slot_size(sz: SlotSize, ws: u8) -> u8 {
    match sz {
        SlotSize::DataSize => ws,
        SlotSize::Bit16 => 16,
        SlotSize::Bit32 => 32,
    }
}

/// Serial audio protocol.
#[derive(Clone, Copy)]
pub struct Protocol {
    /// Synchronization scheme.
    pub sync: Synchronization,
    /// Number of bits filled with audio data.
    ///
    /// The only values allowed are 8, 10, 16, 20, 24, and 32.
    pub word_size: WordSize,
    /// Number of bits transmitted per word.
    ///
    /// If a master clock is generated, the slot size should be a power of two if an integer ratio
    /// between the master clock and the bit clock is required.
    ///
    /// If the value does not equal the word size, the only other values allowed are 16 and 32. In
    /// any case, the value has to be equal or larger than the word size. If the slot size does not
    /// match the word size, the data is padded according to the synchronization scheme.
    pub slot_size: SlotSize,
    /// Number of slots (i.e., audio channels) per frame.
    ///
    /// For everything but PCM audio, the value needs to be 2 (stereo).
    pub num_slots: u8,
}

/// Synchronization mode.
#[derive(Clone, Copy)]
pub enum Synchronization {
    /// I2S standard synchronization.
    ///
    /// The frame-select signal is low during data for the left channel and high for the right
    /// channel. The frame-select signal is activated one SCK cycle before the first bit for the
    /// corresponding channel is available.
    ///
    /// Data is followed by zeros if the configured word size does not match the frame size.
    I2S,
    /// MSB-justified synchronization.
    ///
    /// Like I2S, but the frame-select signal is activated when the first bit for the corresponding
    /// channel is available.
    MSBJustified,
    /// LSB-justified synchronization.
    ///
    /// Like I2S, but the frame-select signal is activated when the first bit for the corresponding
    /// channel is available, and the leading bits are set to zero if the word size does not match
    /// the frame size.
    LSBJustified,
    /// PCM data with short-frame synchronization.
    ///
    /// The frame-select signal is asserted for one bit before the start of the data.
    PCMShortFrame,
    /// PCM data with long-frame synchronization.
    ///
    /// The frame-select signal is asserted at the same time as the start of the data and is held
    /// high for 13 bits.
    PCMLongFrame,
}

/// SAI sub-block.
pub struct SAICH<SAI, const C: bool> {
    sai: SAI,
}

impl<SAI: Instance, const C: bool> SAICH<SAI, C> {
    fn ch(&self) -> &sai::CH {
        self.sai.ch(usize::from(C))
    }
}

/// SAI A sub-block.
pub type SAIA<SAI> = SAICH<SAI, false>;

/// SAI B sub-block.
pub type SAIB<SAI> = SAICH<SAI, true>;

impl<SAI> crate::Sealed for SAIA<SAI> {}

/// Asynchronous SAI sub-block which has not yet been configured.
///
/// Asynchronous means that the sub-block has its own set of clock pins.
pub struct Asynchronous;

/// Asynchronous SAI sub-block which as been configured as a master.
pub struct AsyncMaster<Ch: SaiChannel> {
    // TODO: Remove attribute when destroy function is implemented.
    #[allow(unused)]
    pins: (Option<Ch::Mclk>, Ch::Fs, Ch::Sck, Ch::Sd),
}

impl<Ch: SaiChannel> AsyncMaster<Ch> {
    fn new(
        pins: (
            Option<impl Into<Ch::Mclk>>,
            impl Into<Ch::Fs>,
            impl Into<Ch::Sck>,
            impl Into<Ch::Sd>,
        ),
    ) -> Self {
        Self {
            pins: (
                pins.0.map(Into::into),
                pins.1.into(),
                pins.2.into(),
                pins.3.into(),
            ),
        }
    }
}

/// Asynchronous SAI sub-block which as been configured as a slave.
pub struct AsyncSlave<Ch: SaiChannel> {
    // TODO: Remove attribute when destroy function is implemented.
    #[allow(unused)]
    pins: (Option<Ch::Mclk>, Ch::Fs, Ch::Sck, Ch::Sd),
}

impl<Ch: SaiChannel> AsyncSlave<Ch> {
    fn new(
        pins: (
            Option<impl Into<Ch::Mclk>>,
            impl Into<Ch::Fs>,
            impl Into<Ch::Sck>,
            impl Into<Ch::Sd>,
        ),
    ) -> Self {
        Self {
            pins: (
                pins.0.map(Into::into),
                pins.1.into(),
                pins.2.into(),
                pins.3.into(),
            ),
        }
    }
}

/// Synchronous SAI sub-block.
///
/// Synchronous sub-blocks are always configured as slaves.
pub struct Synchronous;

/// Synchronous SAI sub-block which as been configured as a slave.
pub struct SyncSlave<Ch: SaiChannel> {
    // TODO: Remove attribute when destroy function is implemented.
    #[allow(unused)]
    sd: Ch::Sd,
}

impl<Ch: SaiChannel> SyncSlave<Ch> {
    fn new(sd: impl Into<Ch::Sd>) -> Self {
        Self { sd: sd.into() }
    }
}

/// SAI sub-block which has neither been configured as a receiver nor as a transmitter.
pub struct NoDir;

/// SAI sub-block which has been configured as a receiver.
pub struct Receive;

/// SAI sub-block which has been configured as a transmitter.
pub struct Transmit;

pub trait Instance: rcc::Instance + crate::Ptr<RB = sai::RegisterBlock> {}

impl<SAI: Instance, const C: bool> SAICH<SAI, C> {
    fn new(sai: SAI) -> Self {
        Self { sai }
    }
    fn new_steal() -> Self {
        Self {
            sai: unsafe { SAI::steal() },
        }
    }
}

macro_rules! sai_impl {
    ($SAI:ty, $sai:ident, $SAIA:ident, $SAIB:ident) => {
        pub type $SAIA = SAIA<$SAI>;
        pub type $SAIB = SAIB<$SAI>;

        impl Instance for $SAI {}
    };
}

sai_impl!(SAI1, sai1, SAI1A, SAI1B);
#[cfg(feature = "sai2")]
sai_impl!(SAI2, sai2, SAI2A, SAI2B);

impl<SAI, const C: bool> Deref for SAICH<SAI, C>
where
    SAI: Instance,
{
    type Target = sai::CH;

    fn deref(&self) -> &Self::Target {
        self.ch()
    }
}

pub trait ChannelClocks {
    fn get_clk_frequency(clocks: &rcc::Clocks) -> Option<Hertz>;
}

pub trait Channel: ChannelClocks {
    fn set_master_tx(&self);
    fn set_master_rx(&self);
    fn set_slave_tx(&self);
    fn set_slave_rx(&self);

    fn is_slave(&self) -> bool;
    fn set_clock_gen(&self, sample_freq: Hertz, clocks: &rcc::Clocks);
    fn set_protocol(&self, protocol: Protocol, tx: bool);

    fn start(&self);
    fn stop(&self);

    fn fifo_full(&self) -> bool;
    fn fifo_empty(&self) -> bool;

    fn write(&self, word: u32);
    fn read(&self) -> u32;

    // TODO: DMA information?
    // TODO: Interrupt bits?
    // TODO: SAIA is on Channel 0, Stream 1 and 3.
    // TODO: SAIB is on Channel 0, Stream 5 and Channel 1, Stream 4.
}

#[cfg(not(feature = "sai2"))]
impl<SAI, const C: bool> ChannelClocks for SAICH<SAI, C> {
    fn get_clk_frequency(clocks: &rcc::Clocks) -> Option<Hertz> {
        if C {
            clocks.saib_clk()
        } else {
            clocks.saia_clk()
        }
    }
}

#[cfg(feature = "sai2")]
impl<const C: bool> ChannelClocks for SAICH<SAI1, C> {
    fn get_clk_frequency(clocks: &rcc::Clocks) -> Option<Hertz> {
        clocks.sai1_clk()
    }
}

#[cfg(feature = "sai2")]
impl<const C: bool> ChannelClocks for SAICH<SAI2, C> {
    fn get_clk_frequency(clocks: &rcc::Clocks) -> Option<Hertz> {
        clocks.sai2_clk()
    }
}

impl<Ch> Channel for Ch
where
    Ch: Deref<Target = sai::CH> + ChannelClocks,
{
    fn set_master_tx(&self) {
        self.cr1().modify(|_, w| w.mode().master_tx());
    }

    fn set_master_rx(&self) {
        self.cr1().modify(|_, w| w.mode().master_rx());
    }

    fn set_slave_tx(&self) {
        self.cr1().modify(|_, w| w.mode().slave_tx());
    }

    fn set_slave_rx(&self) {
        self.cr1().modify(|_, w| w.mode().slave_rx());
    }

    fn is_slave(&self) -> bool {
        let mode = self.cr1().read().mode();
        mode.is_slave_tx() || mode.is_slave_rx()
    }

    fn set_clock_gen(&self, sample_freq: Hertz, clocks: &rcc::Clocks) {
        let mclk = sample_freq.raw() * 256;
        let sai_clock = Self::get_clk_frequency(clocks)
            .expect("no SAI clock available")
            .raw();
        if (sai_clock + (mclk >> 1)) / mclk == 1 {
            self.cr1().modify(|_, w| unsafe { w.mckdiv().bits(0) });
        } else {
            let best_divider = (sai_clock + mclk) / (mclk << 1);
            assert!(best_divider < 16);
            self.cr1()
                .modify(|_, w| unsafe { w.mckdiv().bits(best_divider as u8) });
        }
    }

    fn set_protocol(&self, protocol: Protocol, tx: bool) {
        // According to the reference manual (I2S section), PCM has an inverted bit clock by
        // default. Other sources sometimes disagree.
        let pcm = match protocol.sync {
            Synchronization::I2S => false,
            Synchronization::MSBJustified => false,
            Synchronization::LSBJustified => false,
            Synchronization::PCMLongFrame => true,
            Synchronization::PCMShortFrame => true,
        };
        if !pcm && protocol.num_slots != 2 {
            panic!("only stereo I2S supported");
        }
        assert!(protocol.num_slots > 0);
        let word_size = word_size(protocol.word_size);
        let slot_size = slot_size(protocol.slot_size, word_size);
        assert!(slot_size >= word_size, "slot size smaller than word size");
        self.cr1().modify(|_, w| {
            w.ds().variant(protocol.word_size);
            if (pcm && tx) || (!pcm && !tx) {
                w.ckstr().rising_edge()
            } else {
                w.ckstr().falling_edge()
            }
        });
        self.frcr().modify(|_, w| {
            match protocol.sync {
                Synchronization::PCMShortFrame => w.fsoff().before_first(),
                _ => w.fsoff().on_first(),
            };
            if pcm {
                w.fspol().rising_edge();
                w.fsdef().clear_bit();
                unsafe {
                    // The long frame sync signal is fixed and has a length of 13 bits.
                    match protocol.sync {
                        Synchronization::PCMShortFrame => w.fsall().bits(0),
                        Synchronization::PCMLongFrame => w.fsall().bits(12),
                        _ => unreachable!(),
                    };
                    w.frl().bits((slot_size * protocol.num_slots) - 1);
                }
            } else {
                w.fspol().falling_edge();
                w.fsdef().set_bit();
                unsafe {
                    w.fsall().bits(slot_size - 1);
                    w.frl().bits((slot_size << 1) - 1);
                }
            }
            w
        });
        self.slotr().modify(|_, w| unsafe {
            if pcm {
                w.sloten().bits((1 << protocol.num_slots as u32) - 1);
                w.nbslot().bits(protocol.num_slots - 1);
            } else {
                w.sloten().bits(0x3);
                w.nbslot().bits(1);
            }
            w.slotsz().variant(protocol.slot_size);
            match protocol.sync {
                Synchronization::LSBJustified => w.fboff().bits(slot_size - word_size),
                _ => w.fboff().bits(0),
            };
            w
        });
    }

    fn start(&self) {
        self.clrfr().write(|w| {
            w.clfsdet().set_bit();
            w.cafsdet().set_bit();
            w.ccnrdy().set_bit();
            w.cwckcfg().set_bit();
            w.cmutedet().set_bit();
            w.covrudr().set_bit();
            w
        });
        self.cr2().modify(|_, w| w.fflush().flush());
        self.cr1().modify(|_, w| w.saien().enabled());
    }

    fn stop(&self) {
        self.cr1().modify(|_, w| w.saien().disabled());
        while self.cr1().read().saien().bit_is_set() {}
    }

    fn fifo_full(&self) -> bool {
        // We usually write at least two words (stereo data).
        let sr = self.sr().read();
        sr.flvl().is_full() || sr.flvl().is_quarter4()
    }
    fn fifo_empty(&self) -> bool {
        // We usually readat least two words (stereo data).(stereo data).
        let sr = self.sr().read();
        sr.flvl().is_empty() || sr.flvl().is_quarter1()
    }

    fn write(&self, word: u32) {
        self.dr().write(|w| unsafe { w.data().bits(word) });
    }
    fn read(&self) -> u32 {
        self.dr().read().data().bits()
    }
}

/// Wrapper for a single channel of the SAI and its configuration.
pub struct SubBlock<Channel, Config, Direction = NoDir> {
    channel: Channel,
    #[allow(unused)]
    config: Config,
    #[allow(unused)]
    direction: Direction,
}

/// Functions to configure the two sub-blocks of an SAI instance.
///
/// For the two sub-blocks of a single SAI instance, only specific combinations of modes are valid.
/// This trait has one method for each such combination.
pub trait SAIExt
where
    Self: Instance,
{
    /// Splits the SAI instance into two asynchronous sub-blocks.
    fn split(
        self,
        rcc: &mut RCC,
    ) -> (
        SubBlock<SAIA<Self>, Asynchronous>,
        SubBlock<SAIB<Self>, Asynchronous>,
    )
    where
        Self: Sized;

    /// Splits the SAI instance so that the A block uses the synchronization signals of the B
    /// block.
    fn split_sync_a(
        self,
        rcc: &mut RCC,
    ) -> (
        SubBlock<SAIA<Self>, Synchronous>,
        SubBlock<SAIB<Self>, Asynchronous>,
    )
    where
        Self: Sized;

    /// Splits the SAI instance so that the B block uses the synchronization signals of the A
    /// block.
    fn split_sync_b(
        self,
        rcc: &mut RCC,
    ) -> (
        SubBlock<SAIA<Self>, Asynchronous>,
        SubBlock<SAIB<Self>, Synchronous>,
    )
    where
        Self: Sized;

    /*/// Un-splits the two sub-blocks and resets the SAI.
    fn uninit<ConfigA, ConfigB>(a: SubBlock<SAIA, ConfigA>, b: SubBlock<SAIB, ConfigB>) -> Self
    where
        Self: Sized;*/
}

impl<SAI> SAIExt for SAI
where
    SAI: Instance,
{
    fn split(
        self,
        rcc: &mut RCC,
    ) -> (
        SubBlock<SAIA<Self>, Asynchronous>,
        SubBlock<SAIB<Self>, Asynchronous>,
    )
    where
        Self: Sized,
    {
        SAI::enable(rcc);
        SAI::reset(rcc);
        (
            SubBlock {
                channel: SAIA::new(self),
                config: Asynchronous,
                direction: NoDir,
            },
            SubBlock {
                channel: SAIB::new_steal(),
                config: Asynchronous,
                direction: NoDir,
            },
        )
    }

    fn split_sync_a(
        self,
        rcc: &mut RCC,
    ) -> (
        SubBlock<SAIA<Self>, Synchronous>,
        SubBlock<SAIB<Self>, Asynchronous>,
    )
    where
        Self: Sized,
    {
        SAI::enable(rcc);
        SAI::reset(rcc);
        (
            SubBlock {
                channel: SAIA::new(self),
                config: Synchronous,
                direction: NoDir,
            },
            SubBlock {
                channel: SAIB::new_steal(),
                config: Asynchronous,
                direction: NoDir,
            },
        )
    }

    fn split_sync_b(
        self,
        rcc: &mut RCC,
    ) -> (
        SubBlock<SAIA<Self>, Asynchronous>,
        SubBlock<SAIB<Self>, Synchronous>,
    )
    where
        Self: Sized,
    {
        SAI::enable(rcc);
        SAI::reset(rcc);
        (
            SubBlock {
                channel: SAIA::new(self),
                config: Asynchronous,
                direction: NoDir,
            },
            SubBlock {
                channel: SAIB::new_steal(),
                config: Synchronous,
                direction: NoDir,
            },
        )
    }

    /*fn uninit<ConfigA, ConfigB>(a: SubBlock<SAIA, ConfigA>, b: SubBlock<SAIB, ConfigB>) -> Self {
        // TODO
    }*/
}

impl<Ch> SubBlock<Ch, Asynchronous>
where
    Ch: Channel + SaiChannel,
{
    /// Configures the channel as a master and a receiver.
    pub fn master_rx(
        self,
        pins: (
            Option<impl Into<Ch::Mclk>>,
            impl Into<Ch::Fs>,
            impl Into<Ch::Sck>,
            impl Into<Ch::Sd>,
        ),
        protocol: Protocol,
        sample_freq: impl Into<Hertz>,
        clocks: &rcc::Clocks,
    ) -> SubBlock<Ch, AsyncMaster<Ch>, Receive> {
        self.channel.set_clock_gen(sample_freq.into(), clocks);
        self.channel.set_master_rx();
        self.channel.set_protocol(protocol, false);

        SubBlock {
            channel: self.channel,
            config: AsyncMaster::new(pins),
            direction: Receive,
        }
    }

    /// Configures the channel as a master and a transmitter.
    pub fn master_tx(
        self,
        pins: (
            Option<impl Into<Ch::Mclk>>,
            impl Into<Ch::Fs>,
            impl Into<Ch::Sck>,
            impl Into<Ch::Sd>,
        ),
        protocol: Protocol,
        sample_freq: impl Into<Hertz>,
        clocks: &rcc::Clocks,
    ) -> SubBlock<Ch, AsyncMaster<Ch>, Transmit> {
        self.channel.set_clock_gen(sample_freq.into(), clocks);
        self.channel.set_master_tx();
        self.channel.set_protocol(protocol, true);

        SubBlock {
            channel: self.channel,
            config: AsyncMaster::new(pins),
            direction: Transmit,
        }
    }

    /// Configures the channel as a slave and a receiver.
    pub fn slave_rx(
        self,
        pins: (
            Option<impl Into<Ch::Mclk>>,
            impl Into<Ch::Fs>,
            impl Into<Ch::Sck>,
            impl Into<Ch::Sd>,
        ),
        protocol: Protocol,
    ) -> SubBlock<Ch, AsyncSlave<Ch>, Receive> {
        self.channel.set_slave_rx();
        self.channel.set_protocol(protocol, false);

        SubBlock {
            channel: self.channel,
            config: AsyncSlave::new(pins),
            direction: Receive,
        }
    }

    /// Configures the channel as a master and a transmitter.
    pub fn slave_tx(
        self,
        pins: (
            Option<impl Into<Ch::Mclk>>,
            impl Into<Ch::Fs>,
            impl Into<Ch::Sck>,
            impl Into<Ch::Sd>,
        ),
        protocol: Protocol,
    ) -> SubBlock<Ch, AsyncSlave<Ch>, Transmit> {
        self.channel.set_slave_tx();
        self.channel.set_protocol(protocol, true);

        SubBlock {
            channel: self.channel,
            config: AsyncSlave::new(pins),
            direction: Transmit,
        }
    }
}

impl<Ch> SubBlock<Ch, Synchronous>
where
    Ch: Channel + SaiChannel,
{
    /// Configures the channel as a slave and a receiver.
    pub fn slave_rx(
        self,
        sd: impl Into<Ch::Sd>,
        protocol: Protocol,
    ) -> SubBlock<Ch, SyncSlave<Ch>, Receive> {
        self.channel.set_slave_rx();
        self.channel.set_protocol(protocol, false);

        SubBlock {
            channel: self.channel,
            config: SyncSlave::new(sd),
            direction: Receive,
        }
    }

    /// Configures the channel as a slave and a transmitter.
    pub fn slave_tx(
        self,
        sd: impl Into<Ch::Sd>,
        protocol: Protocol,
    ) -> SubBlock<Ch, SyncSlave<Ch>, Transmit> {
        self.channel.set_slave_tx();
        self.channel.set_protocol(protocol, true);

        SubBlock {
            channel: self.channel,
            config: SyncSlave::new(sd),
            direction: Transmit,
        }
    }
}

impl<Ch, Config> SubBlock<Ch, Config, Receive>
where
    Ch: Channel,
{
    pub fn start(&mut self) {
        self.channel.start();
    }

    pub fn stop(&mut self) {
        self.channel.stop();
    }

    // TODO: Mono functions, DMA.

    pub fn try_read(&mut self) -> nb::Result<(u32, u32), ()> {
        if !self.channel.fifo_empty() {
            // Note that fifo_empty() actually checks for at least two words of data.
            Ok((self.channel.read(), self.channel.read()))
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<Ch, Config> SubBlock<Ch, Config, Transmit>
where
    Ch: Channel,
{
    pub fn start(&mut self) {
        self.channel.start();
    }

    pub fn stop(&mut self) {
        self.channel.stop();
    }

    // TODO: Mono functions, DMA.

    pub fn try_send(&mut self, left: u32, right: u32) -> nb::Result<(), ()> {
        if !self.channel.fifo_full() {
            // Note that fifo_full) actually checks for at least two words of space.
            self.channel.write(left);
            self.channel.write(right);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// Wrapper around `Receive` and `Transmit` blocks to provide full-duplex I2S transfers.
pub struct Duplex<Ch1, Config1, Ch2, Config2> {
    rx: SubBlock<Ch1, Config1, Receive>,
    tx: SubBlock<Ch2, Config2, Transmit>,
}

impl<Ch1, Config1, Ch2, Config2> Duplex<Ch1, Config1, Ch2, Config2>
where
    Ch1: Channel,
    Ch2: Channel,
{
    /// Wraps the specified receiver/transmitter objects.
    pub fn new(rx: SubBlock<Ch1, Config1, Receive>, tx: SubBlock<Ch2, Config2, Transmit>) -> Self {
        Self { rx, tx }
    }

    pub fn start(&mut self) {
        // When the two channels are synchronized (internally or externally), we need to start the
        // slave first.
        if self.rx.channel.is_slave() {
            self.rx.start();
        }
        if self.tx.channel.is_slave() {
            self.tx.start();
        }
        if !self.rx.channel.is_slave() {
            self.rx.start();
        }
        if !self.tx.channel.is_slave() {
            self.tx.start();
        }
    }

    pub fn stop(&mut self) {
        self.rx.stop();
        self.tx.stop();
    }

    pub fn try_read(&mut self) -> nb::Result<(u32, u32), ()> {
        self.rx.try_read()
    }

    pub fn try_send(&mut self, left: u32, right: u32) -> nb::Result<(), ()> {
        self.tx.try_send(left, right)
    }

    // TODO: Implement embedded-hal I2S traits for Duplex.
}

unsafe impl<SAI: Instance, const C: bool> crate::dma::traits::PeriAddress for SAICH<SAI, C> {
    #[inline(always)]
    fn address(&self) -> u32 {
        self.ch().dr().as_ptr() as u32
    }

    type MemSize = u32;
}
