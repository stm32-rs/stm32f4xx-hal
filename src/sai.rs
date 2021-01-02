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
//! The following code configures the both sub-blocks of SAI for full-duplex communication using
//! I2S-encoded audio.
//!
//! ```
//! // Initialize clocks.
//! let rcc = ctx.device.RCC.constrain();
//! let clocks = rcc
//!     .cfgr
//!     .use_hse(8.mhz())
//!     .saia_clk(172.mhz())
//!     .saib_clk(172.mhz())
//!     .freeze();
//! // Test that the SAI clock is suitable for 48000KHz audio.
//! assert!(clocks.saia_clk().unwrap() == 172.mhz().into());
//! assert!(clocks.saib_clk().unwrap() == 172.mhz().into());
//!
//! let gpioe = ctx.device.GPIOE.split();
//! // SAIB is made synchronous to A.
//! let (saia, saib) = ctx.device.SAI.split_sync_b();
//! let protocol = Protocol {
//!     sync: Synchronization::I2S,
//!     word_size: 16,
//!     slot_size: 16,
//!     num_slots: 2,
//! };
//! let tx = saia.master_tx(
//!     (
//!         gpioe.pe2.into_alternate_af6(),
//!         gpioe.pe4.into_alternate_af6(),
//!         gpioe.pe5.into_alternate_af6(),
//!         gpioe.pe6.into_alternate_af6(),
//!     ),
//!     protocol,
//!     48000.hz(),
//!     clocks,
//! );
//! let rx = saib.slave_rx(gpioe.pe3.into_alternate_af6(), protocol);
//!
//! let mut duplex = Duplex::new(rx, tx);
//! duplex.start();
//! loop {
//!     duplex.try_send(0xaaaa, 0xf0f0).ok();
//!     let _input = duplex.try_read();
//! }
//! ```
//!
//! The following code configures the A sub-block of SAI as a master transmitter for PCM-encoded
//! audio.
//!
//! ```
//! // Initialize clocks.
//! let rcc = ctx.device.RCC.constrain();
//! let clocks = rcc
//!     .cfgr
//!     .use_hse(8.mhz())
//!     .saia_clk(172.mhz())
//!     .freeze();
//! // Test that the SAI clock is suitable for 48000KHz audio.
//! assert!(clocks.saia_clk().unwrap() == 172.mhz().into());
//!
//! let gpioe = ctx.device.GPIOE.split();
//! let (saia, _) = ctx.device.SAI.split();
//! let protocol = Protocol {
//!     sync: Synchronization::PCMShortFrame,
//!     word_size: 16,
//!     slot_size: 16,
//!     // Stereo audio, two slots per frame.
//!     num_slots: 2,
//! };
//! let tx = saia.master_tx(
//!     (
//!         gpioe.pe2.into_alternate_af6(),
//!         gpioe.pe4.into_alternate_af6(),
//!         gpioe.pe5.into_alternate_af6(),
//!         gpioe.pe6.into_alternate_af6(),
//!     ),
//!     protocol,
//!     48000.hz(),
//!     clocks,
//! );
//! tx.start();
//! loop {
//!     tx.try_send(0xaaaa, 0xf0f0).ok();
//! }
//! ```
//!
//! # Clock Selection
//!
//! TODO
//!

// TODO: Unify capitalization of template parameters.
// TODO: Synchronization of multiple SAIs.

use core::marker::PhantomData;
use core::ops::Deref;

use crate::gpio::gpiod::PD6;
use crate::gpio::gpioe::{PE2, PE3, PE4, PE5, PE6};
use crate::gpio::gpiof::{PF6, PF7, PF8, PF9};
use crate::gpio::{Alternate, AF6};
use crate::rcc::Clocks;
use crate::stm32::RCC;
#[cfg(not(feature = "stm32f446"))]
use crate::stm32::{sai, SAI};
#[cfg(feature = "stm32f446")]
use crate::stm32::{SAI1, SAI2};
use crate::time::Hertz;

/// SAI A sub-block.
pub struct SAIA<SAIX> {
    _sai: PhantomData<SAIX>,
}

/// SAI B sub-block.
pub struct SAIB<SAIX> {
    _sai: PhantomData<SAIX>,
}

#[cfg(not(feature = "stm32f446"))]
pub type SAI1A = SAIA<SAI>;
#[cfg(not(feature = "stm32f446"))]
pub type SAI1B = SAIB<SAI>;
#[cfg(feature = "stm32f446")]
pub type SAI1A = SAIA<SAI1>;
#[cfg(feature = "stm32f446")]
pub type SAI1B = SAIB<SAI1>;
#[cfg(feature = "stm32f446")]
pub type SAI2A = SAIA<SAI2>;
#[cfg(feature = "stm32f446")]
pub type SAI2B = SAIB<SAI2>;

/// Trait for master clock pins.
pub trait PinMck<Ch> {}
/// Trait for frame select pins.
pub trait PinFs<Ch> {}
/// Trait for bit clock pins.
pub trait PinSck<Ch> {}
/// Trait for data pins.
pub trait PinSd<Ch> {}

/// Pins required for an asynchronous SAI master channel.
pub trait MasterPins<Ch> {}

impl<Ch, MCK, FS, SCK, SD> MasterPins<Ch> for (MCK, FS, SCK, SD)
where
    MCK: PinMck<Ch>,
    FS: PinFs<Ch>,
    SCK: PinSck<Ch>,
    SD: PinSd<Ch>,
{
}

/// Pins required for an asynchronous SAI slave channel.
pub trait SlavePins<Ch> {}

impl<Ch, MCK, FS, SCK, SD> SlavePins<Ch> for (MCK, FS, SCK, SD)
where
    MCK: PinMck<Ch>,
    FS: PinFs<Ch>,
    SCK: PinSck<Ch>,
    SD: PinSd<Ch>,
{
}

/// A filler type for when the MCK pin is unnecessary
pub struct NoMck;

macro_rules! pins {
    ($($CH:ty: MCK: [$($MCK:ty),*] FS: [$($FS:ty),*] SCK: [$($SCK:ty),*] SD: [$($SD:ty),*])+) => {
        $(
            $(
                impl PinMck<$CH> for $MCK {}
            )*
            $(
                impl PinFs<$CH> for $FS {}
            )*
            $(
                impl PinSck<$CH> for $SCK{}
            )*
            $(
                impl PinSd<$CH> for $SD{}
            )*
        )+
    }
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
))]
pins! {
    SAI1A:
        MCK: [
            NoMck,
            PE2<Alternate<AF6>>
        ]
        FS: [
            PE4<Alternate<AF6>>
        ]
        SCK: [
            PE5<Alternate<AF6>>
        ]
        SD: [
            PD6<Alternate<AF6>>,
            PE6<Alternate<AF6>>
        ]
    SAI1B:
        MCK: [
            NoMck,
            PF7<Alternate<AF6>>
        ]
        FS: [
            PF9<Alternate<AF6>>
        ]
        SCK: [
            PF8<Alternate<AF6>>
        ]
        SD: [
            PE3<Alternate<AF6>>,
            PF6<Alternate<AF6>>
        ]
}

/// Serial audio protocol.
#[derive(Clone, Copy)]
pub struct Protocol {
    /// Synchronization scheme.
    pub sync: Synchronization,
    /// Number of bits filled with audio data.
    ///
    /// The only values allowed are 8, 10, 16, 20, 24, and 32.
    pub word_size: u8,
    /// Number of bits transmitted per word.
    ///
    /// If a master clock is generated, the slot size should be a power of two if an integer ratio
    /// between the master clock and the bit clock is required.
    ///
    /// If the value does not equal the word size, the only other values allowed are 16 and 32. In
    /// any case, the value has to be equal or larger than the word size. If the slot size does not
    /// match the word size, the data is padded according to the synchronization scheme.
    pub slot_size: u8,
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

/// Asynchronous SAI sub-block which has not yet been configured.
///
/// Asynchronous means that the sub-block has its own set of clock pins.
pub struct Asynchronous;

/// Asynchronous SAI sub-block which as been configured as a master.
pub struct AsyncMaster<Pins> {
    // TODO: Remove attribute when destroy function is implemented.
    #[allow(unused)]
    pins: Pins,
}

/// Asynchronous SAI sub-block which as been configured as a slave.
pub struct AsyncSlave<Pins> {
    // TODO: Remove attribute when destroy function is implemented.
    #[allow(unused)]
    pins: Pins,
}

/// Synchronous SAI sub-block.
///
/// Synchronous sub-blocks are always configured as slaves.
pub struct Synchronous;

/// Synchronous SAI sub-block which as been configured as a slave.
pub struct SyncSlave<SD> {
    // TODO: Remove attribute when destroy function is implemented.
    #[allow(unused)]
    sd: SD,
}

/// SAI sub-block which has neither been configured as a receiver nor as a transmitter.
pub struct NoDir;

/// SAI sub-block which has been configured as a receiver.
pub struct Receive;

/// SAI sub-block which has been configured as a transmitter.
pub struct Transmit;

impl Deref for SAIA<SAI> {
    type Target = sai::CH;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*SAI::ptr()).cha }
    }
}

impl Deref for SAIB<SAI> {
    type Target = sai::CH;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*SAI::ptr()).chb }
    }
}

pub trait Channel {
    fn set_master_tx(&self);
    fn set_master_rx(&self);
    fn set_slave_tx(&self);
    fn set_slave_rx(&self);

    fn is_slave(&self) -> bool;

    fn set_clock_gen(&self, sample_freq: Hertz, clocks: Clocks);
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

impl<Ch> Channel for Ch
where
    Ch: Deref<Target = sai::CH>,
{
    fn set_master_tx(&self) {
        self.cr1.modify(|_, w| w.mode().master_tx());
    }

    fn set_master_rx(&self) {
        self.cr1.modify(|_, w| w.mode().master_rx());
    }

    fn set_slave_tx(&self) {
        self.cr1.modify(|_, w| w.mode().slave_tx());
    }

    fn set_slave_rx(&self) {
        self.cr1.modify(|_, w| w.mode().slave_rx());
    }

    fn is_slave(&self) -> bool {
        let mode = self.cr1.read().mode();
        mode.is_slave_tx() || mode.is_slave_rx()
    }

    fn set_clock_gen(&self, sample_freq: Hertz, clocks: Clocks) {
        let mclk = sample_freq.0 * 256;
        // TODO: Use saib_clock for SAIB.
        let sai_clock = clocks.saia_clk().expect("no SAI clock available").0;
        if (sai_clock + (mclk >> 1)) / mclk == 1 {
            // TODO: Typo in stm32f4
            self.cr1.modify(|_, w| unsafe { w.mcjdiv().bits(0) });
        } else {
            let best_divider = (sai_clock + mclk) / (mclk << 1);
            assert!(best_divider < 16);
            self.cr1
                .modify(|_, w| unsafe { w.mcjdiv().bits(best_divider as u8) });
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
        if protocol.slot_size < protocol.word_size {
            panic!("slot size smaller than word size");
        }
        self.cr1.modify(|_, w| {
            match protocol.word_size {
                8 => w.ds().bit8(),
                10 => w.ds().bit10(),
                16 => w.ds().bit16(),
                20 => w.ds().bit20(),
                24 => w.ds().bit24(),
                32 => w.ds().bit32(),
                _ => panic!("invalid word size"),
            };
            if (pcm && tx) || (!pcm && !tx) {
                w.ckstr().rising_edge();
            } else {
                w.ckstr().falling_edge();
            }
            w
        });
        self.frcr.modify(|_, w| {
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
                    w.frl().bits((protocol.slot_size * protocol.num_slots) - 1);
                }
            } else {
                w.fspol().falling_edge();
                w.fsdef().set_bit();
                unsafe {
                    w.fsall().bits(protocol.slot_size - 1);
                    w.frl().bits((protocol.slot_size << 1) - 1);
                }
            }
            w
        });
        self.slotr.modify(|_, w| unsafe {
            if pcm {
                w.sloten().bits((1 << protocol.num_slots as u32) - 1);
                w.nbslot().bits(protocol.num_slots - 1);
            } else {
                w.sloten().bits(0x3);
                w.nbslot().bits(1);
            }
            if protocol.slot_size == protocol.word_size {
                w.slotsz().data_size();
            } else if protocol.slot_size == 16 {
                w.slotsz().bit16();
            } else if protocol.slot_size == 32 {
                w.slotsz().bit32();
            } else {
                panic!("invalid slot size");
            }
            match protocol.sync {
                Synchronization::LSBJustified => {
                    w.fboff().bits(protocol.slot_size - protocol.word_size)
                }
                _ => w.fboff().bits(0),
            };
            w
        });
    }

    fn start(&self) {
        self.clrfr.modify(|_, w| {
            w.clfsdet().set_bit();
            w.cafsdet().set_bit();
            w.ccnrdy().set_bit();
            w.cwckcfg().set_bit();
            w.cmutedet().set_bit();
            w.covrudr().set_bit();
            w
        });
        self.cr2.modify(|_, w| w.fflush().flush());
        self.cr1.modify(|_, w| w.saien().enabled());
    }

    fn stop(&self) {
        self.cr1.modify(|_, w| w.saien().disabled());
        while self.cr1.read().saien().bit_is_set() {}
    }

    fn fifo_full(&self) -> bool {
        // We usually write at least two words (stereo data).
        self.sr.read().flvl().is_full() || self.sr.read().flvl().is_quarter4()
    }
    fn fifo_empty(&self) -> bool {
        // We usually readat least two words (stereo data).
        self.sr.read().flvl().is_empty() || self.sr.read().flvl().is_quarter1()
    }

    fn write(&self, word: u32) {
        self.dr.write(|w| unsafe { w.data().bits(word) });
    }
    fn read(&self) -> u32 {
        self.dr.read().data().bits()
    }
}

/// Wrapper for a single channel of the SAI and its configuration.
pub struct SubBlock<Channel, Config, Direction> {
    channel: Channel,
    config: Config,
    direction: Direction,
}

/// Functions to configure the two sub-blocks of an SAI instance.
///
/// For the two sub-blocks of a single SAI instance, only specific combinations of modes are valid.
/// This trait has one method for each such combination.
pub trait SAIExt {
    /// Splits the SAI instance into two asynchronous sub-blocks.
    fn split(
        self,
    ) -> (
        SubBlock<SAIA<Self>, Asynchronous, NoDir>,
        SubBlock<SAIB<Self>, Asynchronous, NoDir>,
    )
    where
        Self: Sized;

    /// Splits the SAI instance so that the A block uses the synchronization signals of the B
    /// block.
    fn split_sync_a(
        self,
    ) -> (
        SubBlock<SAIA<Self>, Synchronous, NoDir>,
        SubBlock<SAIB<Self>, Asynchronous, NoDir>,
    )
    where
        Self: Sized;

    /// Splits the SAI instance so that the B block uses the synchronization signals of the A
    /// block.
    fn split_sync_b(
        self,
    ) -> (
        SubBlock<SAIA<Self>, Asynchronous, NoDir>,
        SubBlock<SAIB<Self>, Synchronous, NoDir>,
    )
    where
        Self: Sized;

    /*/// Un-splits the two sub-blocks and resets the SAI.
    fn uninit<ConfigA, ConfigB>(a: SubBlock<SAIA, ConfigA>, b: SubBlock<SAIB, ConfigB>) -> Self
    where
        Self: Sized;*/

    /// Enables and resets the SAI instance.
    fn reset(&mut self);
}

impl SAIExt for SAI {
    fn split(
        mut self,
    ) -> (
        SubBlock<SAI1A, Asynchronous, NoDir>,
        SubBlock<SAI1B, Asynchronous, NoDir>,
    )
    where
        Self: Sized,
    {
        self.reset();
        (
            SubBlock {
                channel: SAIA { _sai: PhantomData },
                config: Asynchronous,
                direction: NoDir,
            },
            SubBlock {
                channel: SAIB { _sai: PhantomData },
                config: Asynchronous,
                direction: NoDir,
            },
        )
    }

    fn split_sync_a(
        mut self,
    ) -> (
        SubBlock<SAI1A, Synchronous, NoDir>,
        SubBlock<SAI1B, Asynchronous, NoDir>,
    )
    where
        Self: Sized,
    {
        self.reset();
        (
            SubBlock {
                channel: SAIA { _sai: PhantomData },
                config: Synchronous,
                direction: NoDir,
            },
            SubBlock {
                channel: SAIB { _sai: PhantomData },
                config: Asynchronous,
                direction: NoDir,
            },
        )
    }

    fn split_sync_b(
        mut self,
    ) -> (
        SubBlock<SAI1A, Asynchronous, NoDir>,
        SubBlock<SAI1B, Synchronous, NoDir>,
    )
    where
        Self: Sized,
    {
        self.reset();
        (
            SubBlock {
                channel: SAIA { _sai: PhantomData },
                config: Asynchronous,
                direction: NoDir,
            },
            SubBlock {
                channel: SAIB { _sai: PhantomData },
                config: Synchronous,
                direction: NoDir,
            },
        )
    }

    /*fn uninit<ConfigA, ConfigB>(a: SubBlock<SAIA, ConfigA>, b: SubBlock<SAIB, ConfigB>) -> Self {
        // TODO
    }*/

    fn reset(&mut self) {
        let rcc = unsafe { &*RCC::ptr() };
        rcc.apb2enr.modify(|_, w| w.sai1en().set_bit());
        rcc.apb2rstr.modify(|_, w| w.sai1rst().set_bit());
        rcc.apb2rstr.modify(|_, w| w.sai1rst().clear_bit());
    }
}

impl<Ch> SubBlock<Ch, Asynchronous, NoDir>
where
    Ch: Channel,
{
    /// Configures the channel as a master and a receiver.
    pub fn master_rx<Pins, F>(
        self,
        pins: Pins,
        protocol: Protocol,
        sample_freq: F,
        clocks: Clocks,
    ) -> SubBlock<Ch, AsyncMaster<Pins>, Receive>
    where
        Pins: MasterPins<Ch>,
        F: Into<Hertz>,
    {
        self.channel.set_clock_gen(sample_freq.into(), clocks);
        self.channel.set_master_rx();
        self.channel.set_protocol(protocol, false);

        SubBlock {
            channel: self.channel,
            config: AsyncMaster { pins },
            direction: Receive,
        }
    }

    /// Configures the channel as a master and a receiver.
    pub fn master_tx<Pins, F>(
        self,
        pins: Pins,
        protocol: Protocol,
        sample_freq: F,
        clocks: Clocks,
    ) -> SubBlock<Ch, AsyncMaster<Pins>, Transmit>
    where
        Pins: MasterPins<Ch>,
        F: Into<Hertz>,
    {
        self.channel.set_clock_gen(sample_freq.into(), clocks);
        self.channel.set_master_tx();
        self.channel.set_protocol(protocol, true);

        SubBlock {
            channel: self.channel,
            config: AsyncMaster { pins },
            direction: Transmit,
        }
    }

    /// Configures the channel as a slave and a receiver.
    pub fn slave_rx<Pins>(
        self,
        pins: Pins,
        protocol: Protocol,
    ) -> SubBlock<Ch, AsyncSlave<Pins>, Receive>
    where
        Pins: SlavePins<Ch>,
    {
        self.channel.set_slave_rx();
        self.channel.set_protocol(protocol, false);

        SubBlock {
            channel: self.channel,
            config: AsyncSlave { pins },
            direction: Receive,
        }
    }

    /// Configures the channel as a master and a receiver.
    pub fn slave_tx<Pins>(
        self,
        pins: Pins,
        protocol: Protocol,
    ) -> SubBlock<Ch, AsyncSlave<Pins>, Transmit>
    where
        Pins: SlavePins<Ch>,
    {
        self.channel.set_slave_tx();
        self.channel.set_protocol(protocol, true);

        SubBlock {
            channel: self.channel,
            config: AsyncSlave { pins },
            direction: Transmit,
        }
    }
}

impl<Ch> SubBlock<Ch, Synchronous, NoDir>
where
    Ch: Channel,
{
    /// Configures the channel as a slave and a receiver.
    pub fn slave_rx<SD>(self, sd: SD, protocol: Protocol) -> SubBlock<Ch, SyncSlave<SD>, Receive>
    where
        SD: PinSd<Ch>,
    {
        self.channel.set_slave_rx();
        self.channel.set_protocol(protocol, false);

        SubBlock {
            channel: self.channel,
            config: SyncSlave { sd },
            direction: Receive,
        }
    }

    /// Configures the channel as a master and a receiver.
    pub fn slave_tx<SD>(self, sd: SD, protocol: Protocol) -> SubBlock<Ch, SyncSlave<SD>, Transmit>
    where
        SD: PinSd<Ch>,
    {
        self.channel.set_slave_tx();
        self.channel.set_protocol(protocol, true);

        SubBlock {
            channel: self.channel,
            config: SyncSlave { sd },
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
