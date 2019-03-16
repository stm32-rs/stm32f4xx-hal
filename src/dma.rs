//! Direct Memory Access.
//!
//!
//! # Status
//! This implementation currently only provides convenience methods for configuration and clearing of interrupts. No attempt has been made
//! (yet) to provide a safe api for dealing with buffers or ongoing transfers.
//!
//! DmaStream::init is an attempt at providing compile time checking of peripheral-stream-channel-direction combinations.
//! DmaStream::init is implemented for valid combinations. [DmaStream](struct.DmaStream.html) shows a list of valid combinations.
//! You can also directly configure the DMA via the StreamX<DMA> structs but this won't give you any protection from invalid combinations.
//!
//! ## Implemented and tested
//! * ADC
//!     * Single conversion
//!     * Sequence conversions
//!     * Circular mode
//!     * Double buffer
//! ## Implemented but need further testing
//! * Features
//!     * SxCR.PFCTRL - peripheral flow control
//! * Peripherals
//!     * TIM - Timers support dozens of DMA modes. Might be a nice idea to implement the "arbitrary waveform generator" from AN4776 as a demo.
//!     * CRYP
//!     * HASH
//!     * UART
//!     * USART
//!     * SPI
//!     * I2C
//!     * DCMI
//!     * QUADSPI
//! ## Not implemented
//! * SDIO
//! * DAC
//! * AES
//! * SAI
//! * FMPI2C1
//! * DFSDM*
//! * SPDIFRX
//! * TIM6/7 - lots of devices missing DMAR register, not sure if this is an SVD issue or dma works differently on those devices.
//!
//! # Examples
//! ## ADC sequence
//! ```
//! static mut ADC_DMA_BUFFER1: [u16; 3] = [0; 3]
//! static mut ADC_DMA_BUFFER2: [u16; 3] = [0; 3]
//! fn main() {
//!     let adc_config = AdcConfig::default()
//!         .scan(Scan::Enabled)
//!         .dma(Dma::Continuous);
//!
//!     let dma_config = DmaConfig::default()
//!         .circular(true)
//!         .transfer_complete_interrupt(true)
//!         .double_buffer(true);
//!
//!     unsafe {
//!         let dma_stream = DmaStream::<DMA2, Stream0<DMA2>, Channel0, ADC1, PeripheralToMemory>::init(
//!             &mut device.DMA2,
//!             &device.ADC1,
//!             &ADC_DMA_BUFFER1 as &[u16],
//!             Some(&ADC_DMA_BUFFER2 as &[u16]),
//!             dma_config);
//!         //you can keep the returned dma_stream reference around for updating config/clearing interrupts/etc.
//!     }
//!
//!     let mut adc = Adc::adc1(device.ADC1, true, adc_config);
//!     //... configure sequence ...
//!     adc.start_conversion();
//! }
//!
//! #[interrupt]
//! fn DMA2_STREAM0() {
//!     //... get dma_stream and the dma peripheral from where we stashed them (resources if using rtfm, a static if not) ...
//!     let cb = dma_stream.current_buffer(dma);
//!     dma_stream.clear_interrupts(dma);
//!
//!     //If we didn't keep the DmaStream around, can use this instead
//!     unsafe {
//!         Stream0::<DMA2>::clear_interrupts_unsafe();
//!     }
//!
//!     info!("DMA: {:?} {:?} {:?}", cb, ADC_DMA_BUFFER1, ADC_DMA_BUFFER2);
//! }
//! ```

#![warn(missing_docs)]

use core::{
    marker::PhantomData,
    ops::Deref,
};
use crate::{
    stm32,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum DmaDirection {
    MemoryToMemory,
    PeripheralToMemory,
    MemoryToPeripheral,
}

/// Converts value to bits for setting a register value
pub trait Bits<T> {
    /// Returns the bit value
    fn bits(self) -> T;
}

/// DMA direction
pub trait Direction: Bits<u8> {}

/// DMA from a peripheral to a memory location
#[derive(Debug, Clone, Copy)]
pub struct PeripheralToMemory;
impl Bits<u8> for PeripheralToMemory { fn bits(self) -> u8 { 0 } }
impl Direction for PeripheralToMemory {}
impl PeripheralToMemory {
    fn new() -> Self {
        PeripheralToMemory
    }
    fn direction() -> DmaDirection {
        DmaDirection::PeripheralToMemory
    }
}

/// DMA from one memory location to another memory location
#[derive(Debug, Clone, Copy)]
pub struct MemoryToMemory;
impl Bits<u8> for MemoryToMemory { fn bits(self) -> u8 { 2 } }
impl Direction for MemoryToMemory {}
impl MemoryToMemory {
    fn new() -> Self {
        MemoryToMemory
    }
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToMemory
    }
}

/// DMA from a memory location to a peripheral
#[derive(Debug, Clone, Copy)]
pub struct MemoryToPeripheral;
impl Bits<u8> for MemoryToPeripheral { fn bits(self) -> u8 { 1 } }
impl Direction for MemoryToPeripheral {}
impl MemoryToPeripheral {
    fn new() -> Self {
        MemoryToPeripheral
    }
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToPeripheral
    }
}

/// DMA Stream
pub struct DmaStream<DMA, STREAM, CHANNEL, PERIPHERAL, DIRECTION> {
    _dma: PhantomData<DMA>,
    _stream: PhantomData<STREAM>,
    _channel: PhantomData<CHANNEL>,
    _peripheral: PhantomData<PERIPHERAL>,
    _direction: PhantomData<DIRECTION>,
}

/// Get an address the DMA can use
pub trait Address {
    /// Returns the address to be used by the DMA stream
    fn address(&self) -> u32;
}

impl Address for &u32 {
    fn address(&self) -> u32 {
        *self as *const _ as u32
    }
}

impl Address for &u16 {
    fn address(&self) -> u32 {
        *self as *const _ as u32
    }
}

impl Address for MemoryToMemory {
    fn address(&self) -> u32 {
        unimplemented!()
    }
}

impl Address for &[u16] {
    fn address(&self) -> u32 {
        self.as_ptr() as *const _ as u32
    }
}

/// Convenience macro for implementing addresses on peripherals
macro_rules! address {
    ($(($peripheral:ty, $register:ident)),+ $(,)*) => {
        $(
            impl Address for $peripheral {
                fn address(&self) -> u32 {
                    &self.$register as *const _ as u32
                }
            }
        )+
    };
}


macro_rules! tim_channels {
    ($(($name:ident, $register:ident)),+ $(,)*) => {
        $(
            /// Wrapper type that indicates which register of the contained timer to use for DMA
            pub struct $name<T> (T);
            impl<T> Deref for $name<T> {
                type Target = T;
                fn deref(&self) -> &T {
                    &self.0
                }
            }
        )+
    };
}

tim_channels!(
    (CCR1, ccr1),
    (CCR2, ccr2),
    (CCR3, ccr3),
    (CCR4, ccr4),
    (DMAR, dmar),
    (ARR, arr),
);

/// How full the DMA stream's fifo is
#[derive(Debug, Clone, Copy)]
pub enum FifoLevel {
    /// 0 < fifo_level < 1/4
    GtZeroLtQuarter,
    /// 1/4 <= fifo_level < 1/2
    GteQuarterLtHalf,
    /// 1/2 <= fifo_level < 3/4
    GteHalfLtThreeQuarter,
    /// 3/4 <= fifo_level < full
    GteThreeQuarterLtFull,
    /// Fifo is empty
    Empty,
    /// Fifo is full
    Full,
}


impl From<u8> for FifoLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => FifoLevel::GtZeroLtQuarter,
            1 => FifoLevel::GteQuarterLtHalf,
            2 => FifoLevel::GteHalfLtThreeQuarter,
            3 => FifoLevel::GteThreeQuarterLtFull,
            4 => FifoLevel::Empty,
            5 => FifoLevel::Full,
            _ => unimplemented!(),
        }
    }
}

/// Which DMA buffer is in use
#[derive(Debug, Clone, Copy)]
pub enum CurrentBuffer {
    /// The first buffer (m0ar) is in use
    FirstBuffer,
    /// The second buffer (m1ar) is in use
    DoubleBuffer,
}

/// Stream 0 on the DMA controller
///
/// Can be used directly:
/// ```
/// Stream0<DMA2>::clear_interrupts(dma);
/// ```
///
/// But it is recommended to use DmaStream::init which is only implemented for valid stream-channel-peripheral-direction
/// combinations. Using StreamX directly you can configure invalid peripheral addresses or direction combinations.
pub struct Stream0<DMA> { _dma: PhantomData<DMA> }
/// Stream 1 on the DMA controller. See Stream0 for more info.
pub struct Stream1<DMA> { _dma: PhantomData<DMA> }
/// Stream 2 on the DMA controller. See Stream0 for more info.
pub struct Stream2<DMA> { _dma: PhantomData<DMA> }
/// Stream 3 on the DMA controller. See Stream0 for more info.
pub struct Stream3<DMA> { _dma: PhantomData<DMA> }
/// Stream 4 on the DMA controller. See Stream0 for more info.
pub struct Stream4<DMA> { _dma: PhantomData<DMA> }
/// Stream 5 on the DMA controller. See Stream0 for more info.
pub struct Stream5<DMA> { _dma: PhantomData<DMA> }
/// Stream 6 on the DMA controller. See Stream0 for more info.
pub struct Stream6<DMA> { _dma: PhantomData<DMA> }
/// Stream 7 on the DMA controller. See Stream0 for more info.
pub struct Stream7<DMA> { _dma: PhantomData<DMA> }

/// Macro that creates a struct representing a stream on either DMA controller
/// The implementation does the heavy lifting of mapping to the right fields on the stream
macro_rules! dma_stream {
    ($(($name:ident, $DMA:ty, $par:ident, $cr:ident, $m0ar:ident, $m1ar:ident, $ndtr:ident,
        $ifcr:ident, $tcif:ident, $htif:ident, $teif:ident, $dmeif:ident, $feif:ident,
        $fcr:ident)),+ $(,)*) => {
        $(
            #[allow(dead_code)]
            impl $name<$DMA> {
                /// Clear all interrupts for the DMA stream
                pub fn clear_interrupts(dma: &mut $DMA) {
                    dma.$ifcr.write(|w| w
                        .$tcif().set_bit() //Clear transfer complete interrupt flag
                        .$htif().set_bit() //Clear half transfer interrupt flag
                        .$teif().set_bit() //Clear transfer error interrupt flag
                        .$dmeif().set_bit() //Clear direct mode error interrupt flag
                        .$feif().set_bit() //Clear fifo error interrupt flag
                    );
                }

                /// Clear all interrupts by unsafely getting access to the DMA peripheral.
                pub unsafe fn clear_interrupts_unsafe() {
                    let dma = &(*<$DMA>::ptr());
                    dma.$ifcr.write(|w| w
                        .$tcif().set_bit() //Clear transfer complete interrupt flag
                        .$htif().set_bit() //Clear half transfer interrupt flag
                        .$teif().set_bit() //Clear transfer error interrupt flag
                        .$dmeif().set_bit() //Clear direct mode error interrupt flag
                        .$feif().set_bit() //Clear fifo error interrupt flag
                    );
                }

                /// Clear transfer complete interrupt (tcif) for the DMA stream
                pub fn clear_transfer_complete_interrupt(dma: &mut $DMA) {
                    //Clear transfer complete interrupt flag
                    dma.$ifcr.write(|w| w.$tcif().set_bit());
                }

                /// Clear half transfer interrupt (htif) for the DMA stream
                pub fn clear_half_transfer_interrupt(dma: &mut $DMA) {
                    //Clear half transfer interrupt flag
                    dma.$ifcr.write(|w| w.$htif().set_bit());
                }

                /// Clear transfer error interrupt (teif) for the DMA stream
                pub fn clear_transfer_error_interrupt(dma: &mut $DMA) {
                    //Clear transfer error interrupt flag
                    dma.$ifcr.write(|w| w.$teif().set_bit());
                }

                /// Clear direct mode error interrupt (dmeif) for the DMA stream
                pub fn clear_direct_mode_error_interrupt(dma: &mut $DMA) {
                    //Clear direct mode error interrupt flag
                    dma.$ifcr.write(|w| w.$dmeif().set_bit());
                }

                /// Clear fifo error interrupt (feif) for the DMA stream
                pub fn clear_fifo_error_interrupt(dma: &mut $DMA) {
                    //Clear fifo error interrupt flag
                    dma.$ifcr.write(|w| w.$feif().set_bit());
                }

                /// Set the peripheral address (par) for the DMA stream
                pub fn set_peripheral_address(dma: &mut $DMA, value: u32) {
                    dma.$par.write(|w| w.pa().bits(value));
                }

                /// Set the memory address (m0ar) for the DMA stream
                pub fn set_memory_address(dma: &mut $DMA, value: u32) {
                    dma.$m0ar.write(|w| w.m0a().bits(value));
                }

                /// Set the double buffer address (m1ar) for the DMA stream
                pub fn set_memory_double_buffer_address(dma: &mut $DMA, value: u32) {
                    dma.$m1ar.write(|w| w.m1a().bits(value));
                }

                /// Set the number of transfers (ndt) for the DMA stream
                pub fn set_number_of_transfers(dma: &mut $DMA, value: u16) {
                    dma.$ndtr.write(|w| w.ndt().bits(value));
                }

                /// Enable the DMA stream
                pub fn enable(dma: &mut $DMA) {
                    dma.$cr.modify(|_, w| w.en().set_bit());
                }

                /// Is the DMA stream enabled?
                pub fn is_enabled(dma: &$DMA) -> bool {
                    dma.$cr.read().en().bit_is_set()
                }

                /// Disable the DMA stream
                pub fn disable(dma: &mut $DMA) {
                    dma.$cr.modify(|_, w| w.en().clear_bit());
                }

                /// Set the channel for the (chsel) the DMA stream
                pub fn set_channel<C>(dma: &mut $DMA, channel: C)
                where
                    C: Channel,
                {
                    //Some device crates have this field unsafe, others don't.
                    #[allow(unused_unsafe)]
                    dma.$cr.modify(|_, w| unsafe { w.chsel().bits(channel.bits()) });
                }

                /// Set the priority (pl) the DMA stream
                pub fn set_priority(dma: &mut $DMA, priority: config::Priority) {
                    dma.$cr.modify(|_, w| w.pl().bits(priority.bits()));
                }

                /// Set the memory size (msize) for the DMA stream
                pub fn set_memory_size(dma: &mut $DMA, size: config::TransferSize) {
                    dma.$cr.modify(|_, w| unsafe { w.msize().bits(size.bits()) });
                }

                /// Set the peripheral size (psize) for the DMA stream
                pub fn set_peripheral_size(dma: &mut $DMA, size: config::TransferSize) {
                    dma.$cr.modify(|_, w| unsafe { w.psize().bits(size.bits()) });
                }

                /// Enable/disable memory increment (minc) for the DMA stream
                pub fn set_memory_increment(dma: &mut $DMA, increment: bool) {
                    dma.$cr.modify(|_, w| w.minc().bit(increment));
                }

                /// Enable/disable peripheral increment (pinc) for the DMA stream
                pub fn set_peripheral_increment(dma: &mut $DMA, increment: bool) {
                    dma.$cr.modify(|_, w| w.pinc().bit(increment));
                }

                /// Enable/disable circular mode (circ) for the DMA stream
                pub fn set_circular(dma: &mut $DMA, circular: bool) {
                    dma.$cr.modify(|_, w| w.circ().bit(circular));
                }

                /// Set the direction (dir) of the DMA stream
                pub fn set_direction<D>(dma: &mut $DMA, direction: D)
                where
                    D: Direction,
                {
                    dma.$cr.modify(|_, w| unsafe { w.dir().bits(direction.bits()) });
                }

                /// Convenience method to configure the 4 common interrupts for the DMA stream
                pub fn set_interrupts(dma: &mut $DMA, transfer_complete: bool, half_transfer: bool, transfer_error: bool, direct_mode_error: bool) {
                    dma.$cr.modify(|_, w| w
                        .tcie().bit(transfer_complete)
                        .htie().bit(half_transfer)
                        .teie().bit(transfer_error)
                        .dmeie().bit(direct_mode_error)
                    );
                }

                /// Enable/disable the transfer complete interrupt (tcie) of the DMA stream
                pub fn set_transfer_complete_interrupt(dma: &mut $DMA, transfer_complete_interrupt: bool) {
                    dma.$cr.modify(|_, w| w.tcie().bit(transfer_complete_interrupt));
                }

                /// Enable/disable the half transfer interrupt (htie) of the DMA stream
                pub fn set_half_transfer_interrupt(dma: &mut $DMA, half_transfer_interrupt: bool) {
                    dma.$cr.modify(|_, w| w.htie().bit(half_transfer_interrupt));
                }

                /// Enable/disable the transfer error interrupt (teie) of the DMA stream
                pub fn set_transfer_error_interrupt(dma: &mut $DMA, transfer_error_interrupt: bool) {
                    dma.$cr.modify(|_, w| w.teie().bit(transfer_error_interrupt));
                }

                /// Enable/disable the direct mode error interrupt (dmeie) of the DMA stream
                pub fn set_direct_mode_error_interrupt(dma: &mut $DMA, direct_mode_error_interrupt: bool) {
                    dma.$cr.modify(|_, w| w.dmeie().bit(direct_mode_error_interrupt));
                }

                /// Enable/disable the fifo error interrupt (feie) of the DMA stream
                pub fn set_fifo_error_interrupt(dma: &mut $DMA, fifo_error_interrupt: bool) {
                    dma.$fcr.modify(|_, w| w.feie().bit(fifo_error_interrupt));
                }

                /// Enable/disable the double buffer (dbm) of the DMA stream
                pub fn set_double_buffer(dma: &mut $DMA, double_buffer: bool) {
                    dma.$cr.modify(|_, w| w.dbm().bit(double_buffer));
                }

                /// Set the fifo threshold (fcr.fth) of the DMA stream
                pub fn set_fifo_threshold(dma: &mut $DMA, fifo_threshold: config::FifoThreshold) {
                    dma.$fcr.modify(|_, w| w.fth().bits(fifo_threshold.bits()));
                }

                /// Enable/disable the fifo (dmdis) of the DMA stream
                pub fn set_fifo_enable(dma: &mut $DMA, fifo_enable: bool) {
                    //Register is actually direct mode disable rather than fifo enable
                    dma.$fcr.modify(|_, w| w.dmdis().bit(!fifo_enable));
                }

                /// Set memory burst mode (mburst) of the DMA stream
                pub fn set_memory_burst(dma: &mut $DMA, memory_burst: config::BurstMode) {
                    dma.$cr.modify(|_, w| w.mburst().bits(memory_burst.bits()));
                }

                /// Set peripheral burst mode (pburst) of the DMA stream
                pub fn set_peripheral_burst(dma: &mut $DMA, peripheral_burst: config::BurstMode) {
                    dma.$cr.modify(|_, w| w.pburst().bits(peripheral_burst.bits()));
                }

                /// Get the current fifo level (fs) of the DMA stream
                pub fn fifo_level(dma: &$DMA) -> FifoLevel {
                    dma.$fcr.read().fs().bits().into()
                }

                /// Set the flow controller (pfctrl) of the DMA stream
                pub fn set_flow_controller(dma: &mut $DMA, flow_controller: config::FlowController) {
                    dma.$cr.modify(|_, w| w.pfctrl().bit(flow_controller.bits()));
                }

                /// Get which buffer is currently in use
                pub fn current_buffer(dma: &$DMA) -> CurrentBuffer {
                    if dma.$cr.read().ct().bit_is_set() {
                        CurrentBuffer::DoubleBuffer
                    } else {
                        CurrentBuffer::FirstBuffer
                    }
                }
            }
        )+
    };
}

dma_stream!(
    (Stream0, stm32::DMA1, s0par, s0cr, s0m0ar, s0m1ar, s0ndtr, lifcr, ctcif0, chtif0, cteif0, cdmeif0, cfeif0, s0fcr),
    (Stream0, stm32::DMA2, s0par, s0cr, s0m0ar, s0m1ar, s0ndtr, lifcr, ctcif0, chtif0, cteif0, cdmeif0, cfeif0, s0fcr),
    (Stream1, stm32::DMA1, s1par, s1cr, s1m0ar, s1m1ar, s1ndtr, lifcr, ctcif1, chtif1, cteif1, cdmeif1, cfeif1, s1fcr),
    (Stream1, stm32::DMA2, s1par, s1cr, s1m0ar, s1m1ar, s1ndtr, lifcr, ctcif1, chtif1, cteif1, cdmeif1, cfeif1, s1fcr),
    (Stream2, stm32::DMA1, s2par, s2cr, s2m0ar, s2m1ar, s2ndtr, lifcr, ctcif2, chtif2, cteif2, cdmeif2, cfeif2, s2fcr),
    (Stream2, stm32::DMA2, s2par, s2cr, s2m0ar, s2m1ar, s2ndtr, lifcr, ctcif2, chtif2, cteif2, cdmeif2, cfeif2, s2fcr),
    (Stream3, stm32::DMA1, s3par, s3cr, s3m0ar, s3m1ar, s3ndtr, lifcr, ctcif3, chtif3, cteif3, cdmeif3, cfeif3, s3fcr),
    (Stream3, stm32::DMA2, s3par, s3cr, s3m0ar, s3m1ar, s3ndtr, lifcr, ctcif3, chtif3, cteif3, cdmeif3, cfeif3, s3fcr),
    (Stream4, stm32::DMA1, s4par, s4cr, s4m0ar, s4m1ar, s4ndtr, hifcr, ctcif4, chtif4, cteif4, cdmeif4, cfeif4, s4fcr),
    (Stream4, stm32::DMA2, s4par, s4cr, s4m0ar, s4m1ar, s4ndtr, hifcr, ctcif4, chtif4, cteif4, cdmeif4, cfeif4, s4fcr),
    (Stream5, stm32::DMA1, s5par, s5cr, s5m0ar, s5m1ar, s5ndtr, hifcr, ctcif5, chtif5, cteif5, cdmeif5, cfeif5, s5fcr),
    (Stream5, stm32::DMA2, s5par, s5cr, s5m0ar, s5m1ar, s5ndtr, hifcr, ctcif5, chtif5, cteif5, cdmeif5, cfeif5, s5fcr),
    (Stream6, stm32::DMA1, s6par, s6cr, s6m0ar, s6m1ar, s6ndtr, hifcr, ctcif6, chtif6, cteif6, cdmeif6, cfeif6, s6fcr),
    (Stream6, stm32::DMA2, s6par, s6cr, s6m0ar, s6m1ar, s6ndtr, hifcr, ctcif6, chtif6, cteif6, cdmeif6, cfeif6, s6fcr),
    (Stream7, stm32::DMA1, s7par, s7cr, s7m0ar, s7m1ar, s7ndtr, hifcr, ctcif7, chtif7, cteif7, cdmeif7, cfeif7, s7fcr),
    (Stream7, stm32::DMA2, s7par, s7cr, s7m0ar, s7m1ar, s7ndtr, hifcr, ctcif7, chtif7, cteif7, cdmeif7, cfeif7, s7fcr),
);

/// A channel that can be configured on a DMA stream
pub trait Channel: Bits<u8> {}

/// Macro that defines a channel and it's conversion to u8
macro_rules! dma_channel {
    ($(($name:ident, $value:expr)),+ $(,)*) => {
        $(
            /// A Channel that can be configured on a DMA stream
            #[derive(Debug, Clone, Copy)]
            pub struct $name;
            impl Bits<u8> for $name {
                fn bits(self) -> u8 { $value }
            }
            impl Channel for $name {}
            impl $name {
                fn new() -> Self {
                    $name
                }
            }
        )+
    };
}


dma_channel!(
    (Channel0, 0),
    (Channel1, 1),
    (Channel2, 2),
    (Channel3, 3),
    (Channel4, 4),
    (Channel5, 5),
    (Channel6, 6),
    (Channel7, 7),
);

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_channel!(
    (Channel8, 8),
    (Channel9, 9),
);

/// Things that implement this can have their RCC enabled
trait RccEnable {
    fn rcc_enable();
}

impl RccEnable for stm32::DMA1 {
    fn rcc_enable() {
        unsafe {
            let rcc = &(*stm32::RCC::ptr());
            rcc.ahb1enr.modify(|_, w| w.dma1en().set_bit());
        }
    }
}

impl RccEnable for stm32::DMA2 {
    fn rcc_enable() {
        unsafe {
            let rcc = &(*stm32::RCC::ptr());
            rcc.ahb1enr.modify(|_, w| w.dma2en().set_bit());
        }
    }
}

/// Contains types related to DMA configuration
pub mod config {
    use super::Bits;

    /// Priority of the DMA stream. Defaults to Medium. Deadlocks are resolved by
    /// the lower numbered stream gets priority.
    #[derive(Debug, Clone, Copy)]
    pub enum Priority {
        /// Low priority
        Low,
        /// Medium priority
        Medium,
        /// High priority
        High,
        /// Very high priority
        VeryHigh,
    }

    impl Bits<u8> for Priority {
        fn bits(self) -> u8 {
            match self {
                Priority::Low => 0,
                Priority::Medium => 1,
                Priority::High => 2,
                Priority::VeryHigh => 3,
            }
        }
    }

    /// Size of the transfer. Same type for memory and peripheral size but the values
    /// needn't be the same. Check the datasheet for how byte packing/unpacking works.
    #[derive(Debug, Clone, Copy)]
    pub enum TransferSize {
        /// 8-bit
        Byte,
        /// 16-bit
        HalfWord,
        /// 32-bit
        Word,
    }

    impl Bits<u8> for TransferSize {
        fn bits(self) -> u8 {
            match self {
                TransferSize::Byte => 0,
                TransferSize::HalfWord => 1,
                TransferSize::Word => 2,
            }
        }
    }

    /// The level to fill the fifo to before performing the transaction
    #[derive(Debug, Clone, Copy)]
    pub enum FifoThreshold {
        /// 1/4 full
        QuarterFull,
        /// 1/2 full
        HalfFull,
        /// 3/4 full
        ThreeQuarterFull,
        /// Full
        Full,
    }

    impl Bits<u8> for FifoThreshold {
        fn bits(self) -> u8 {
            match self {
                FifoThreshold::QuarterFull => 0,
                FifoThreshold::HalfFull => 1,
                FifoThreshold::ThreeQuarterFull => 2,
                FifoThreshold::Full => 3,
            }
        }
    }

    /// How burst transfers are done. Requires fifo enabled. Check datasheet for valid combinations.
    #[derive(Debug, Clone, Copy)]
    pub enum BurstMode {
        /// Single transfer, no burst
        NoBurst,
        /// Burst transfer of 4 beats
        Burst4,
        /// Burst transfer of 8 beats
        Burst8,
        /// Burst transfer of 16 beats
        Burst16,
    }

    impl Bits<u8> for BurstMode {
        fn bits(self) -> u8 {
            match self {
                BurstMode::NoBurst => 0,
                BurstMode::Burst4 => 1,
                BurstMode::Burst8 => 2,
                BurstMode::Burst16 => 3,
            }
        }
    }

    /// Is the DMA controller the flow controller or is the peripheral?
    #[derive(Debug, Clone, Copy)]
    pub enum FlowController {
        /// DMA controller is the flow controller
        Dma,
        /// Source or destination peripheral is the flow controller
        Peripheral,
    }

    impl Bits<bool> for FlowController {
        fn bits(self) -> bool {
            match self {
                FlowController::Dma => false,
                FlowController::Peripheral => true,
            }
        }
    }

    /// Contains the complete set of configuration for a DMA stream
    #[derive(Debug, Clone, Copy)]
    pub struct DmaConfig {
        pub(crate) memory_size: TransferSize,
        pub(crate) peripheral_size: TransferSize,
        pub(crate) number_of_transfers: u16,
        pub(crate) priority: Priority,
        pub(crate) memory_increment: bool,
        pub(crate) peripheral_increment: bool,
        pub(crate) circular: bool,
        pub(crate) transfer_complete_interrupt: bool,
        pub(crate) half_transfer_interrupt: bool,
        pub(crate) transfer_error_interrupt: bool,
        pub(crate) direct_mode_error_interrupt: bool,
        pub(crate) fifo_error_interrupt: bool,
        pub(crate) double_buffer: bool,
        pub(crate) fifo_threshold: FifoThreshold,
        pub(crate) fifo_enable: bool,
        pub(crate) memory_burst: BurstMode,
        pub(crate) peripheral_burst: BurstMode,
        pub(crate) flow_controller: FlowController,
    }

    impl Default for DmaConfig {
        fn default() -> Self {
            Self {
                memory_size: TransferSize::HalfWord,
                peripheral_size: TransferSize::HalfWord,
                number_of_transfers: 1,
                priority: Priority::Medium,
                memory_increment: false,
                peripheral_increment: false,
                circular: false,
                transfer_complete_interrupt: false,
                half_transfer_interrupt: false,
                transfer_error_interrupt: false,
                direct_mode_error_interrupt: false,
                fifo_error_interrupt: false,
                double_buffer: false,
                fifo_threshold: FifoThreshold::QuarterFull,
                fifo_enable: false,
                memory_burst: BurstMode::NoBurst,
                peripheral_burst: BurstMode::NoBurst,
                flow_controller: FlowController::Dma,
            }
        }
    }

    impl DmaConfig {
        /// Set the memory_size
        pub fn memory_size(mut self, memory_size: TransferSize) -> Self {
            self.memory_size = memory_size;
            self
        }
        /// Set the peripheral_size
        pub fn peripheral_size(mut self, peripheral_size: TransferSize) -> Self {
            self.peripheral_size = peripheral_size;
            self
        }
        /// Set the number_of_transfers
        pub fn number_of_transfers(mut self, number_of_transfers: u16) -> Self {
            self.number_of_transfers = number_of_transfers;
            self
        }
        /// Set the priority
        pub fn priority(mut self, priority: Priority) -> Self {
            self.priority = priority;
            self
        }
        /// Set the memory_increment
        pub fn memory_increment(mut self, memory_increment: bool) -> Self {
            self.memory_increment = memory_increment;
            self
        }
        /// Set the peripheral_increment
        pub fn peripheral_increment(mut self, peripheral_increment: bool) -> Self {
            self.peripheral_increment = peripheral_increment;
            self
        }
        /// Set the circular
        pub fn circular(mut self, circular: bool) -> Self {
            self.circular = circular;
            self
        }
        /// Set the transfer_complete_interrupt
        pub fn transfer_complete_interrupt(mut self, transfer_complete_interrupt: bool) -> Self {
            self.transfer_complete_interrupt = transfer_complete_interrupt;
            self
        }
        /// Set the half_transfer_interrupt
        pub fn half_transfer_interrupt(mut self, half_transfer_interrupt: bool) -> Self {
            self.half_transfer_interrupt = half_transfer_interrupt;
            self
        }
        /// Set the transfer_error_interrupt
        pub fn transfer_error_interrupt(mut self, transfer_error_interrupt: bool) -> Self {
            self.transfer_error_interrupt = transfer_error_interrupt;
            self
        }
        /// Set the direct_mode_error_interrupt
        pub fn direct_mode_error_interrupt(mut self, direct_mode_error_interrupt: bool) -> Self {
            self.direct_mode_error_interrupt = direct_mode_error_interrupt;
            self
        }
        /// Set the fifo_error_interrupt
        pub fn fifo_error_interrupt(mut self, fifo_error_interrupt: bool) -> Self {
            self.fifo_error_interrupt = fifo_error_interrupt;
            self
        }
        /// Set the double_buffer
        pub fn double_buffer(mut self, double_buffer: bool) -> Self {
            self.double_buffer = double_buffer;
            self
        }
        /// Set the fifo_threshold
        pub fn fifo_threshold(mut self, fifo_threshold: FifoThreshold) -> Self {
            self.fifo_threshold = fifo_threshold;
            self
        }
        /// Set the fifo_enable
        pub fn fifo_enable(mut self, fifo_enable: bool) -> Self {
            self.fifo_enable = fifo_enable;
            self
        }
        /// Set the memory_burst
        pub fn memory_burst(mut self, memory_burst: BurstMode) -> Self {
            self.memory_burst = memory_burst;
            self
        }
        /// Set the peripheral_burst
        pub fn peripheral_burst(mut self, peripheral_burst: BurstMode) -> Self {
            self.peripheral_burst = peripheral_burst;
            self
        }
        /// Set the flow_controller
        pub fn flow_controller(mut self, flow_controller: FlowController) -> Self {
            self.flow_controller = flow_controller;
            self
        }
    }
}

macro_rules! dma_map {
    ($(($DMA:ty, $stream:ident, $channel:ty, $peripheral:ty, $dir:ty)),+ $(,)*) => {
        $(
            impl DmaStream<$DMA, $stream<$DMA>, $channel, $peripheral, $dir> {
                /// Change the priority of the DMA stream
                pub fn set_priority(&self, dma: &mut $DMA, priority: config::Priority) {
                    <$stream<$DMA>>::set_priority(dma, priority);
                }

                /// Clear all interrupts on the DMA stream
                pub fn clear_interrupts(&self, dma: &mut $DMA) {
                    <$stream<$DMA>>::clear_interrupts(dma);
                }

                /// Disable the stream
                pub fn disable(&self, dma: &mut $DMA) {
                    <$stream<$DMA>>::disable(dma);
                }

                /// Enable the stream
                pub fn enable(&self, dma: &mut $DMA) {
                    <$stream<$DMA>>::enable(dma);
                }

                /// Is the stream enabled?
                pub fn is_enabled(&self, dma: &mut $DMA) -> bool{
                    <$stream<$DMA>>::is_enabled(dma)
                }

                /// Change the memory size of the DMA stream
                pub fn set_memory_size(&self, dma: &mut $DMA, size: config::TransferSize) {
                    <$stream<$DMA>>::set_memory_size(dma, size);
                }

                /// Change the peripheral size of the DMA stream
                pub fn set_peripheral_size(&self, dma: &mut $DMA, size: config::TransferSize) {
                    <$stream<$DMA>>::set_peripheral_size(dma, size);
                }

                /// Change the number of transfers of the DMA stream
                pub fn set_number_of_transfers(&self, dma: &mut $DMA, number_of_transfers: u16) {
                    <$stream<$DMA>>::set_number_of_transfers(dma, number_of_transfers);
                }

                /// Enable/disable memory increment of the DMA stream
                pub fn set_memory_increment(&self, dma: &mut $DMA, memory_increment: bool) {
                    <$stream<$DMA>>::set_memory_increment(dma, memory_increment);
                }

                /// Enable/disable peripheral increment of the DMA stream
                pub fn set_peripheral_increment(&self, dma: &mut $DMA, peripheral_increment: bool) {
                    <$stream<$DMA>>::set_peripheral_increment(dma, peripheral_increment);
                }

                /// Enable/disable circular mode of the DMA stream
                pub fn set_circular(&self, dma: &mut $DMA, circular: bool) {
                    <$stream<$DMA>>::set_circular(dma, circular);
                }

                /// Enable/disable the transfer complete interrupt of the DMA stream
                pub fn set_transfer_complete_interrupt(&self, dma: &mut $DMA, transfer_complete_interrupt: bool) {
                    <$stream<$DMA>>::set_transfer_complete_interrupt(dma, transfer_complete_interrupt);
                }

                /// Enable/disable the half transfer interrupt of the DMA stream
                pub fn set_half_transfer_interrupt(&self, dma: &mut $DMA, half_transfer_interrupt: bool) {
                    <$stream<$DMA>>::set_half_transfer_interrupt(dma, half_transfer_interrupt);
                }

                /// Enable/disable the transfer error interrupt of the DMA stream
                pub fn set_transfer_error_interrupt(&self, dma: &mut $DMA, transfer_error_interrupt: bool) {
                    <$stream<$DMA>>::set_transfer_error_interrupt(dma, transfer_error_interrupt);
                }

                /// Enable/disable the direct mode error interrupt of the DMA stream
                pub fn set_direct_mode_error_interrupt(&self, dma: &mut $DMA, direct_mode_error_interrupt: bool) {
                    <$stream<$DMA>>::set_direct_mode_error_interrupt(dma, direct_mode_error_interrupt);
                }

                /// Enable/disable the fifo error interrupt of the DMA stream
                pub fn set_fifo_error_interrupt(&self, dma: &mut $DMA, fifo_error_interrupt: bool) {
                    <$stream<$DMA>>::set_fifo_error_interrupt(dma, fifo_error_interrupt);
                }

                /// Enable/disable the double buffer of the DMA stream
                pub fn set_double_buffer(&self, dma: &mut $DMA, double_buffer: bool) {
                    <$stream<$DMA>>::set_double_buffer(dma, double_buffer);
                }

                /// Change the fifo threshold of the DMA stream
                pub fn set_fifo_threshold(&self, dma: &mut $DMA, fifo_threshold: config::FifoThreshold) {
                    <$stream<$DMA>>::set_fifo_threshold(dma, fifo_threshold);
                }

                /// Enable/disable the fifo of the DMA stream
                pub fn set_fifo_enable(&self, dma: &mut $DMA, fifo_enable: bool) {
                    <$stream<$DMA>>::set_fifo_enable(dma, fifo_enable);
                }

                /// Change memory burst mode of the DMA stream
                pub fn set_memory_burst(&self, dma: &mut $DMA, memory_burst: config::BurstMode) {
                    <$stream<$DMA>>::set_memory_burst(dma, memory_burst);
                }

                /// Change the peripheral burst mode of the DMA stream
                pub fn set_peripheral_burst(&self, dma: &mut $DMA, peripheral_burst: config::BurstMode) {
                    <$stream<$DMA>>::set_peripheral_burst(dma, peripheral_burst);
                }

                /// Get the current fifo level of the DMA stream
                pub fn fifo_level(&self, dma: &mut $DMA) -> FifoLevel {
                    <$stream<$DMA>>::fifo_level(dma)
                }

                /// Set the flow controller of the DMA stream
                pub fn set_flow_controller(&self, dma: &mut $DMA, flow_controller: config::FlowController) {
                    <$stream<$DMA>>::set_flow_controller(dma, flow_controller);
                }

                /// Get the current buffer the DMA stream is using
                pub fn current_buffer(&self, dma: &$DMA) -> CurrentBuffer {
                    <$stream<$DMA>>::current_buffer(dma)
                }

                /// Applies all fields in DmaConfig
                pub fn apply_config(&self, dma: &mut $DMA, config: config::DmaConfig) {
                    let was_enabled = self.is_enabled(dma);
                    if was_enabled {
                        self.disable(dma);
                    }

                    self.clear_interrupts(dma);
                    self.set_priority(dma, config.priority);
                    self.set_memory_size(dma, config.memory_size);
                    self.set_peripheral_size(dma, config.peripheral_size);
                    self.set_memory_increment(dma, config.memory_increment);
                    self.set_peripheral_increment(dma, config.peripheral_increment);
                    self.set_circular(dma, config.circular);
                    self.set_transfer_complete_interrupt(dma, config.transfer_complete_interrupt);
                    self.set_half_transfer_interrupt(dma, config.half_transfer_interrupt);
                    self.set_transfer_error_interrupt(dma, config.transfer_error_interrupt);
                    self.set_direct_mode_error_interrupt(dma, config.direct_mode_error_interrupt);
                    self.set_fifo_error_interrupt(dma, config.fifo_error_interrupt);
                    self.set_number_of_transfers(dma, config.number_of_transfers);
                    self.set_double_buffer(dma, config.double_buffer);
                    self.set_fifo_threshold(dma, config.fifo_threshold);
                    self.set_fifo_enable(dma, config.fifo_enable);
                    self.set_memory_burst(dma, config.memory_burst);
                    self.set_peripheral_burst(dma, config.peripheral_burst);
                    self.set_flow_controller(dma, config.flow_controller);

                    if was_enabled {
                        self.enable(dma);
                    }
                }

                /// Configures DMA stream to correct channel for peripheral, configures source and destination addresses and applies supplied config
                pub fn init<MA>(dma: &mut $DMA, peripheral: &$peripheral, memory: MA, double_buffer: Option<MA>, config: config::DmaConfig) -> Self
                where
                    MA: Address,
                {
                    //Enable RCC
                    <$DMA>::rcc_enable();

                    let s = Self {
                        _dma: PhantomData,
                        _stream: PhantomData,
                        _channel: PhantomData,
                        _peripheral: PhantomData,
                        _direction: PhantomData,
                    };

                    s.disable(dma);

                    //Set the channel
                    <$stream<$DMA>>::set_channel(dma, <$channel>::new());

                    //Set peripheral to memory mode
                    <$stream<$DMA>>::set_direction(dma, <$dir>::new());

                    //Set the memory address
                    <$stream<$DMA>>::set_memory_address(dma, memory.address());

                    let is_mem2mem = <$dir>::direction() == DmaDirection::MemoryToMemory;
                    if is_mem2mem {
                        //Fifo must be enabled for memory to memory
                        assert!(config.fifo_enable);
                    } else {
                        //Set the peripheral address
                        <$stream<$DMA>>::set_peripheral_address(dma, peripheral.address());
                    }

                    if let Some(db) = double_buffer {
                        if is_mem2mem {
                            //Double buffer is the source in mem2mem mode
                            <$stream<$DMA>>::set_peripheral_address(dma, db.address());
                        } else {
                            <$stream<$DMA>>::set_memory_double_buffer_address(dma, db.address());
                        }
                    } else {
                        // Double buffer mode must not be enabled if we haven't been given a second buffer
                        assert!(!config.double_buffer);
                    }

                    s.apply_config(dma, config);
                    s.enable(dma);

                    s
                }
            }
        )+
    };
}


#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream0, Channel2, CCR1<stm32::TIM4>, MemoryToPeripheral), //TIM4_CH1
    (stm32::DMA1, Stream0, Channel2, CCR1<stm32::TIM4>, PeripheralToMemory), //TIM4_CH1
    (stm32::DMA1, Stream2, Channel5, CCR4<stm32::TIM3>, MemoryToPeripheral), //TIM3_CH4
    (stm32::DMA1, Stream2, Channel5, CCR4<stm32::TIM3>, PeripheralToMemory), //TIM3_CH4
    (stm32::DMA1, Stream2, Channel5, DMAR<stm32::TIM3>, MemoryToPeripheral), //TIM3_UP
    (stm32::DMA1, Stream2, Channel5, DMAR<stm32::TIM3>, PeripheralToMemory), //TIM3_UP
    (stm32::DMA1, Stream3, Channel2, CCR2<stm32::TIM4>, MemoryToPeripheral), //TIM4_CH2
    (stm32::DMA1, Stream3, Channel2, CCR2<stm32::TIM4>, PeripheralToMemory), //TIM4_CH2
    (stm32::DMA1, Stream4, Channel5, CCR1<stm32::TIM3>, MemoryToPeripheral), //TIM3_CH1
    (stm32::DMA1, Stream4, Channel5, CCR1<stm32::TIM3>, PeripheralToMemory), //TIM3_CH1
    (stm32::DMA1, Stream4, Channel5, DMAR<stm32::TIM3>, MemoryToPeripheral), //TIM3_TRIG
    (stm32::DMA1, Stream4, Channel5, DMAR<stm32::TIM3>, PeripheralToMemory), //TIM3_TRIG
    (stm32::DMA1, Stream5, Channel3, CCR1<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH1
    (stm32::DMA1, Stream5, Channel3, CCR1<stm32::TIM2>, PeripheralToMemory), //TIM2_CH1
    (stm32::DMA1, Stream5, Channel5, CCR2<stm32::TIM3>, MemoryToPeripheral), //TIM3_CH2
    (stm32::DMA1, Stream5, Channel5, CCR2<stm32::TIM3>, PeripheralToMemory), //TIM3_CH2
    (stm32::DMA1, Stream6, Channel2, DMAR<stm32::TIM4>, MemoryToPeripheral), //TIM4_UP
    (stm32::DMA1, Stream6, Channel2, DMAR<stm32::TIM4>, PeripheralToMemory), //TIM4_UP
    (stm32::DMA1, Stream6, Channel3, CCR2<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH2
    (stm32::DMA1, Stream6, Channel3, CCR2<stm32::TIM2>, PeripheralToMemory), //TIM2_CH2
    (stm32::DMA1, Stream6, Channel3, CCR4<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH4
    (stm32::DMA1, Stream6, Channel3, CCR4<stm32::TIM2>, PeripheralToMemory), //TIM2_CH4
    (stm32::DMA1, Stream7, Channel2, CCR3<stm32::TIM4>, MemoryToPeripheral), //TIM4_CH3
    (stm32::DMA1, Stream7, Channel2, CCR3<stm32::TIM4>, PeripheralToMemory), //TIM4_CH3
    (stm32::DMA1, Stream7, Channel5, CCR3<stm32::TIM3>, MemoryToPeripheral), //TIM3_CH3
    (stm32::DMA1, Stream7, Channel5, CCR3<stm32::TIM3>, PeripheralToMemory), //TIM3_CH3

    (stm32::DMA1, Stream0, Channel0, stm32::SPI3, PeripheralToMemory), //SPI3_RX
    (stm32::DMA1, Stream2, Channel0, stm32::SPI3, PeripheralToMemory), //SPI3_RX
    (stm32::DMA1, Stream4, Channel3, stm32::I2C3, MemoryToPeripheral), //I2C3_TX
    (stm32::DMA1, Stream5, Channel0, stm32::SPI3, MemoryToPeripheral), //SPI3_TX
    (stm32::DMA1, Stream7, Channel0, stm32::SPI3, MemoryToPeripheral), //SPI3_TX
    //(stm32::DMA2, Stream3, Channel4, stm32::SDIO, MemoryToPeripheral), //SDIO
    //(stm32::DMA2, Stream3, Channel4, stm32::SDIO, PeripheralToMemory), //SDIO
    //(stm32::DMA2, Stream6, Channel4, stm32::SDIO, MemoryToPeripheral), //SDIO
    //(stm32::DMA2, Stream6, Channel4, stm32::SDIO, PeripheralToMemory), //SDIO
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (CCR1<stm32::TIM4>, ccr1),
    (CCR4<stm32::TIM3>, ccr4),
    (CCR1<stm32::TIM2>, ccr1),
    (CCR1<stm32::TIM3>, ccr1),
    (CCR2<stm32::TIM2>, ccr2),
    (CCR2<stm32::TIM3>, ccr2),
    (CCR2<stm32::TIM4>, ccr2),
    (CCR3<stm32::TIM3>, ccr3),
    (CCR3<stm32::TIM4>, ccr3),
    (CCR4<stm32::TIM2>, ccr4),
    (DMAR<stm32::TIM3>, dmar),
    (DMAR<stm32::TIM4>, dmar),
    (stm32::SPI3, dr),
    (stm32::I2C3, dr),
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream0, Channel6, CCR3<stm32::TIM5>, MemoryToPeripheral), //TIM5_CH3
    (stm32::DMA1, Stream0, Channel6, CCR3<stm32::TIM5>, PeripheralToMemory), //TIM5_CH3
    (stm32::DMA1, Stream0, Channel6, DMAR<stm32::TIM5>, MemoryToPeripheral), //TIM5_UP
    (stm32::DMA1, Stream0, Channel6, DMAR<stm32::TIM5>, PeripheralToMemory), //TIM5_UP
    (stm32::DMA1, Stream1, Channel6, CCR4<stm32::TIM5>, MemoryToPeripheral), //TIM5_CH4
    (stm32::DMA1, Stream1, Channel6, CCR4<stm32::TIM5>, PeripheralToMemory), //TIM5_CH4
    (stm32::DMA1, Stream1, Channel6, DMAR<stm32::TIM5>, MemoryToPeripheral), //TIM5_TRIG
    (stm32::DMA1, Stream1, Channel6, DMAR<stm32::TIM5>, PeripheralToMemory), //TIM5_TRIG
    (stm32::DMA1, Stream2, Channel6, CCR1<stm32::TIM5>, MemoryToPeripheral), //TIM5_CH1
    (stm32::DMA1, Stream2, Channel6, CCR1<stm32::TIM5>, PeripheralToMemory), //TIM5_CH1
    (stm32::DMA1, Stream3, Channel6, CCR4<stm32::TIM5>, MemoryToPeripheral), //TIM5_CH4
    (stm32::DMA1, Stream3, Channel6, CCR4<stm32::TIM5>, PeripheralToMemory), //TIM5_CH4
    (stm32::DMA1, Stream3, Channel6, DMAR<stm32::TIM5>, MemoryToPeripheral), //TIM5_TRIG
    (stm32::DMA1, Stream3, Channel6, DMAR<stm32::TIM5>, PeripheralToMemory), //TIM5_TRIG
    (stm32::DMA1, Stream4, Channel6, CCR2<stm32::TIM5>, MemoryToPeripheral), //TIM5_CH2
    (stm32::DMA1, Stream4, Channel6, CCR2<stm32::TIM5>, PeripheralToMemory), //TIM5_CH2
    (stm32::DMA1, Stream6, Channel6, DMAR<stm32::TIM5>, MemoryToPeripheral), //TIM5_UP
    (stm32::DMA1, Stream6, Channel6, DMAR<stm32::TIM5>, PeripheralToMemory), //TIM5_UP
    (stm32::DMA2, Stream0, Channel6, DMAR<stm32::TIM1>, MemoryToPeripheral), //TIM1_TRIG
    (stm32::DMA2, Stream0, Channel6, DMAR<stm32::TIM1>, PeripheralToMemory), //TIM1_TRIG
    (stm32::DMA2, Stream1, Channel6, CCR1<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH1
    (stm32::DMA2, Stream1, Channel6, CCR1<stm32::TIM1>, PeripheralToMemory), //TIM1_CH1
    (stm32::DMA2, Stream2, Channel6, CCR2<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH2
    (stm32::DMA2, Stream2, Channel6, CCR2<stm32::TIM1>, PeripheralToMemory), //TIM1_CH2
    (stm32::DMA2, Stream3, Channel6, CCR1<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH1
    (stm32::DMA2, Stream3, Channel6, CCR1<stm32::TIM1>, PeripheralToMemory), //TIM1_CH1
    (stm32::DMA2, Stream4, Channel6, CCR4<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH4
    (stm32::DMA2, Stream4, Channel6, CCR4<stm32::TIM1>, PeripheralToMemory), //TIM1_CH4
    (stm32::DMA2, Stream4, Channel6, DMAR<stm32::TIM1>, MemoryToPeripheral), //TIM1_TRIG/COM
    (stm32::DMA2, Stream4, Channel6, DMAR<stm32::TIM1>, PeripheralToMemory), //TIM1_TRIG/COM
    (stm32::DMA2, Stream5, Channel6, DMAR<stm32::TIM1>, MemoryToPeripheral), //TIM1_UP
    (stm32::DMA2, Stream5, Channel6, DMAR<stm32::TIM1>, PeripheralToMemory), //TIM1_UP
    (stm32::DMA2, Stream6, Channel0, CCR1<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH1
    (stm32::DMA2, Stream6, Channel0, CCR1<stm32::TIM1>, PeripheralToMemory), //TIM1_CH1
    (stm32::DMA2, Stream6, Channel0, CCR2<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH2
    (stm32::DMA2, Stream6, Channel0, CCR2<stm32::TIM1>, PeripheralToMemory), //TIM1_CH2
    (stm32::DMA2, Stream6, Channel0, CCR3<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH3
    (stm32::DMA2, Stream6, Channel0, CCR3<stm32::TIM1>, PeripheralToMemory), //TIM1_CH3
    (stm32::DMA2, Stream6, Channel6, CCR3<stm32::TIM1>, MemoryToPeripheral), //TIM1_CH3
    (stm32::DMA2, Stream6, Channel6, CCR3<stm32::TIM1>, PeripheralToMemory), //TIM1_CH3
    (stm32::DMA1, Stream0, Channel1, stm32::I2C1, PeripheralToMemory), //I2C1_RX
    (stm32::DMA1, Stream2, Channel7, stm32::I2C2, PeripheralToMemory), //I2C2_RX
    (stm32::DMA1, Stream3, Channel0, stm32::SPI2, PeripheralToMemory), //SPI2_RX
    (stm32::DMA1, Stream3, Channel7, stm32::I2C2, PeripheralToMemory), //I2C2_RX
    (stm32::DMA1, Stream4, Channel0, stm32::SPI2, MemoryToPeripheral), //SPI2_TX
    (stm32::DMA1, Stream5, Channel1, stm32::I2C1, PeripheralToMemory), //I2C1_RX
    (stm32::DMA1, Stream5, Channel4, stm32::USART2, PeripheralToMemory), //USART2_RX
    (stm32::DMA1, Stream6, Channel4, stm32::USART2, MemoryToPeripheral), //USART2_TX
    (stm32::DMA1, Stream7, Channel7, stm32::I2C2, MemoryToPeripheral), //I2C2_TX
    (stm32::DMA2, Stream0, Channel0, stm32::ADC1, PeripheralToMemory), //ADC1
    (stm32::DMA2, Stream0, Channel3, stm32::SPI1, PeripheralToMemory), //SPI1_RX
    (stm32::DMA2, Stream1, Channel5, stm32::USART6, PeripheralToMemory), //USART6_RX
    (stm32::DMA2, Stream2, Channel3, stm32::SPI1, PeripheralToMemory), //SPI1_RX
    (stm32::DMA2, Stream2, Channel4, stm32::USART1, PeripheralToMemory), //USART1_RX
    (stm32::DMA2, Stream2, Channel5, stm32::USART6, PeripheralToMemory), //USART6_RX
    (stm32::DMA2, Stream4, Channel0, stm32::ADC1, PeripheralToMemory), //ADC1
    (stm32::DMA2, Stream5, Channel4, stm32::USART1, PeripheralToMemory), //USART1_RX
    (stm32::DMA2, Stream6, Channel5, stm32::USART6, MemoryToPeripheral), //USART6_TX
    (stm32::DMA2, Stream7, Channel4, stm32::USART1, MemoryToPeripheral), //USART1_TX
    (stm32::DMA2, Stream7, Channel5, stm32::USART6, MemoryToPeripheral), //USART6_TX
    (stm32::DMA2, Stream0, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream1, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream2, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream3, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream4, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream5, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream6, Channel0, MemoryToMemory, MemoryToMemory),
    (stm32::DMA2, Stream7, Channel0, MemoryToMemory, MemoryToMemory),
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (CCR1<stm32::TIM1>, ccr1),
    (CCR2<stm32::TIM1>, ccr2),
    (CCR3<stm32::TIM1>, ccr3),
    (CCR4<stm32::TIM1>, ccr4),
    (DMAR<stm32::TIM1>, dmar),
    (CCR1<stm32::TIM5>, ccr1),
    (CCR2<stm32::TIM5>, ccr2),
    (CCR3<stm32::TIM5>, ccr3),
    (CCR4<stm32::TIM5>, ccr4),
    (DMAR<stm32::TIM5>, dmar),
    (stm32::ADC1, dr),
    (stm32::I2C1, dr),
    (stm32::I2C2, dr),
    (stm32::SPI1, dr),
    (stm32::SPI2, dr),
    (stm32::USART1, dr),
    (stm32::USART2, dr),
    (stm32::USART6, dr),
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
dma_map!(
    (stm32::DMA1, Stream1, Channel1, stm32::I2C3, PeripheralToMemory), //I2C3_RX
    (stm32::DMA1, Stream2, Channel3, stm32::I2C3, PeripheralToMemory), //I2C3_RX:DMA_CHANNEL_3
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
))]
dma_map!(
    (stm32::DMA1, Stream1, Channel3, CCR3<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH3
    (stm32::DMA1, Stream1, Channel3, CCR3<stm32::TIM2>, PeripheralToMemory), //TIM2_CH3
    (stm32::DMA1, Stream1, Channel3, DMAR<stm32::TIM2>, MemoryToPeripheral), //TIM2_UP
    (stm32::DMA1, Stream1, Channel3, DMAR<stm32::TIM2>, PeripheralToMemory), //TIM2_UP
    (stm32::DMA1, Stream7, Channel3, CCR4<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH4
    (stm32::DMA1, Stream7, Channel3, CCR4<stm32::TIM2>, PeripheralToMemory), //TIM2_CH4
    (stm32::DMA1, Stream7, Channel3, DMAR<stm32::TIM2>, MemoryToPeripheral), //TIM2_UP
    (stm32::DMA1, Stream7, Channel3, DMAR<stm32::TIM2>, PeripheralToMemory), //TIM2_UP
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
))]
address!(
    (CCR3<stm32::TIM2>, ccr3),
    (DMAR<stm32::TIM2>, dmar),
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (stm32::DMA1, Stream5, Channel6, stm32::I2C3, MemoryToPeripheral), //I2C3_TX:DMA_CHANNEL_6
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream6, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX
    (stm32::DMA1, Stream7, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX
    (stm32::DMA2, Stream3, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX
    (stm32::DMA2, Stream5, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA2, Stream0, Channel4, stm32::SPI4, PeripheralToMemory), //SPI4_RX
    (stm32::DMA2, Stream1, Channel4, stm32::SPI4, MemoryToPeripheral), //SPI4_TX
    (stm32::DMA2, Stream3, Channel5, stm32::SPI4, PeripheralToMemory), //SPI4_RX:DMA_CHANNEL_5
    (stm32::DMA2, Stream4, Channel5, stm32::SPI4, MemoryToPeripheral), //SPI4_TX:DMA_CHANNEL_5
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::SPI4, dr),
);

#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream0, Channel4, stm32::UART5, PeripheralToMemory), //UART5_RX
    (stm32::DMA1, Stream2, Channel4, stm32::UART4, PeripheralToMemory), //UART4_RX
    (stm32::DMA1, Stream4, Channel4, stm32::UART4, MemoryToPeripheral), //UART4_TX
    //(stm32::DMA1, Stream6, Channel7, stm32::DAC, MemoryToPeripheral), //DAC2
);

#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::UART4, dr),
    (stm32::UART5, dr),
    //(stm32::DAC, ??),
);

#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream1, Channel3, DMAR<stm32::TIM2>, MemoryToPeripheral), //TIM2_UP
    (stm32::DMA1, Stream1, Channel3, DMAR<stm32::TIM2>, PeripheralToMemory), //TIM2_UP
    (stm32::DMA1, Stream1, Channel3, CCR3<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH3
    (stm32::DMA1, Stream1, Channel3, CCR3<stm32::TIM2>, PeripheralToMemory), //TIM2_CH3
    //(stm32::DMA1, Stream2, Channel1, DMAR<stm32::TIM7>, MemoryToPeripheral), //TIM7_UP //dmar register appears to be missing
    //(stm32::DMA1, Stream2, Channel1, DMAR<stm32::TIM7>, PeripheralToMemory), //TIM7_UP //dmar register appears to be missing
    //(stm32::DMA1, Stream4, Channel1, DMAR<stm32::TIM7>, MemoryToPeripheral), //TIM7_UP //dmar register appears to be missing
    //(stm32::DMA1, Stream4, Channel1, DMAR<stm32::TIM7>, PeripheralToMemory), //TIM7_UP //dmar register appears to be missing
    (stm32::DMA1, Stream7, Channel3, DMAR<stm32::TIM2>, MemoryToPeripheral), //TIM2_UP
    (stm32::DMA1, Stream7, Channel3, DMAR<stm32::TIM2>, PeripheralToMemory), //TIM2_UP
    (stm32::DMA1, Stream7, Channel3, CCR4<stm32::TIM2>, MemoryToPeripheral), //TIM2_CH4
    (stm32::DMA1, Stream7, Channel3, CCR4<stm32::TIM2>, PeripheralToMemory), //TIM2_CH4
    (stm32::DMA2, Stream1, Channel7, DMAR<stm32::TIM8>, MemoryToPeripheral), //TIM8_UP
    (stm32::DMA2, Stream1, Channel7, DMAR<stm32::TIM8>, PeripheralToMemory), //TIM8_UP
    (stm32::DMA2, Stream2, Channel0, CCR1<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH1
    (stm32::DMA2, Stream2, Channel0, CCR1<stm32::TIM8>, PeripheralToMemory), //TIM8_CH1
    (stm32::DMA2, Stream2, Channel0, CCR2<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH2
    (stm32::DMA2, Stream2, Channel0, CCR2<stm32::TIM8>, PeripheralToMemory), //TIM8_CH2
    (stm32::DMA2, Stream2, Channel0, CCR3<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH3
    (stm32::DMA2, Stream2, Channel0, CCR3<stm32::TIM8>, PeripheralToMemory), //TIM8_CH3
    (stm32::DMA2, Stream2, Channel7, CCR1<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH1
    (stm32::DMA2, Stream2, Channel7, CCR1<stm32::TIM8>, PeripheralToMemory), //TIM8_CH1
    (stm32::DMA2, Stream3, Channel7, CCR2<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH2
    (stm32::DMA2, Stream3, Channel7, CCR2<stm32::TIM8>, PeripheralToMemory), //TIM8_CH2
    (stm32::DMA2, Stream4, Channel7, CCR3<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH3
    (stm32::DMA2, Stream4, Channel7, CCR3<stm32::TIM8>, PeripheralToMemory), //TIM8_CH3
    (stm32::DMA2, Stream7, Channel7, CCR4<stm32::TIM8>, MemoryToPeripheral), //TIM8_CH4
    (stm32::DMA2, Stream7, Channel7, CCR4<stm32::TIM8>, PeripheralToMemory), //TIM8_CH4
    (stm32::DMA2, Stream7, Channel7, DMAR<stm32::TIM8>, MemoryToPeripheral), //TIM8_COM/TRIG
    (stm32::DMA2, Stream7, Channel7, DMAR<stm32::TIM8>, PeripheralToMemory), //TIM8_COM/TRIG
    (stm32::DMA1, Stream1, Channel4, stm32::USART3, PeripheralToMemory), //USART3_RX
    (stm32::DMA1, Stream3, Channel4, stm32::USART3, MemoryToPeripheral), //USART3_TX
    (stm32::DMA1, Stream4, Channel7, stm32::USART3, MemoryToPeripheral), //USART3_TX:DMA_CHANNEL_7
);

#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (CCR1<stm32::TIM8>, ccr1),
    (CCR2<stm32::TIM8>, ccr2),
    (CCR3<stm32::TIM8>, ccr3),
    (CCR4<stm32::TIM8>, ccr4),
    (DMAR<stm32::TIM8>, dmar),
    (CCR3<stm32::TIM2>, ccr3),
    (DMAR<stm32::TIM2>, dmar),
    //(DMAR<stm32::TIM7>, dmar), //Missing?
    (stm32::USART3, dr),
);


/* DMAR register appears to be missing from TIM6 derived timers on these devices
   Not sure how _UP is supposed to work without DMAR or if this is just an SVD issue
#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream1, Channel7, DMAR<stm32::TIM6>, MemoryToPeripheral), //TIM6_UP
    (stm32::DMA1, Stream1, Channel7, DMAR<stm32::TIM6>, PeripheralToMemory), //TIM6_UP
);

#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (DMAR<stm32::TIM6>, dmar),
);
*/


#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream2, Channel3, stm32::I2C3, PeripheralToMemory), //I2C3_RX
    (stm32::DMA2, Stream5, Channel2, stm32::CRYP, PeripheralToMemory), //CRYP_OUT
    (stm32::DMA2, Stream6, Channel2, stm32::CRYP, MemoryToPeripheral), //CRYP_IN
    (stm32::DMA2, Stream7, Channel2, stm32::HASH, MemoryToPeripheral), //HASH_IN
);

#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::HASH, din),
    (stm32::CRYP, din),
);

/* Not sure how DAC works with DMA
#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream5, Channel7, stm32::DAC, MemoryToPeripheral), //DAC1
);
#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::DAC, ??),
);
*/


#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream7, Channel4, stm32::UART5, MemoryToPeripheral), //UART5_TX
    (stm32::DMA2, Stream0, Channel2, stm32::ADC3, PeripheralToMemory), //ADC3
    (stm32::DMA2, Stream1, Channel1, stm32::DCMI, PeripheralToMemory), //DCMI
    (stm32::DMA2, Stream1, Channel2, stm32::ADC3, PeripheralToMemory), //ADC3
    (stm32::DMA2, Stream2, Channel1, stm32::ADC2, PeripheralToMemory), //ADC2
    (stm32::DMA2, Stream3, Channel1, stm32::ADC2, PeripheralToMemory), //ADC2
    (stm32::DMA2, Stream7, Channel1, stm32::DCMI, PeripheralToMemory), //DCMI
);
#[cfg(any(
    feature = "stm32f417",
    feature = "stm32f415",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::ADC2, dr),
    (stm32::ADC3, dr),
    (stm32::DCMI, dr),
);

/* FMPI2C missing from peripheral crates
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (stm32::DMA1, Stream0, Channel7, stm32::FMPI2C1, PeripheralToMemory), //FMPI2C1_RX
    (stm32::DMA1, Stream1, Channel2, stm32::FMPI2C1, MemoryToPeripheral), //FMPI2C1_TX
    (stm32::DMA1, Stream3, Channel1, stm32::FMPI2C1, PeripheralToMemory), //FMPI2C1_RX:DMA_CHANNEL_1
    (stm32::DMA1, Stream7, Channel4, stm32::FMPI2C1, MemoryToPeripheral), //FMPI2C1_TX:DMA_CHANNEL_4
);

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
address!(
    (stm32::FMPI2C1, dr),
);
*/

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (stm32::DMA1, Stream1, Channel0, stm32::I2C1, MemoryToPeripheral), //I2C1_TX
    (stm32::DMA1, Stream6, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX:DMA_CHANNEL_1
    (stm32::DMA1, Stream7, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX:DMA_CHANNEL_1
    (stm32::DMA1, Stream7, Channel6, stm32::USART2, PeripheralToMemory), //USART2_RX:DMA_CHANNEL_6
    (stm32::DMA2, Stream2, Channel2, stm32::SPI1, MemoryToPeripheral), //SPI1_TX
    (stm32::DMA2, Stream3, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX:DMA_CHANNEL_3
    (stm32::DMA2, Stream5, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX:DMA_CHANNEL_3
    (stm32::DMA2, Stream5, Channel5, stm32::SPI5, MemoryToPeripheral), //SPI5_TX:DMA_CHANNEL_5
);

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA2, Stream3, Channel2, stm32::SPI5, PeripheralToMemory), //SPI5_RX
    (stm32::DMA2, Stream4, Channel2, stm32::SPI5, MemoryToPeripheral), //SPI5_TX
    (stm32::DMA2, Stream5, Channel7, stm32::SPI5, PeripheralToMemory), //SPI5_RX:DMA_CHANNEL_7
    (stm32::DMA2, Stream6, Channel7, stm32::SPI5, MemoryToPeripheral), //SPI5_TX:DMA_CHANNEL_7
);

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::SPI5, dr),
);

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (stm32::DMA2, Stream4, Channel4, stm32::SPI4, PeripheralToMemory), //SPI4_RX
);

/* DFSDM1 appears to be missing from SVD
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (stm32::DMA2, Stream0, Channel7, stm32::DFSDM1, PeripheralToMemory), //DFSDM1_FLT0
    (stm32::DMA2, Stream1, Channel3, stm32::DFSDM1, PeripheralToMemory), //DFSDM1_FLT1
    (stm32::DMA2, Stream4, Channel3, stm32::DFSDM1, PeripheralToMemory), //DFSDM1_FLT1
    (stm32::DMA2, Stream6, Channel3, stm32::DFSDM1, PeripheralToMemory), //DFSDM1_FLT0:DMA_CHANNEL_3
);
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
address!(
    (stm32::DFSDM1, dr),
);
*/

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA2, Stream7, Channel3, stm32::QUADSPI, MemoryToPeripheral), //QUADSPI
    (stm32::DMA2, Stream7, Channel3, stm32::QUADSPI, PeripheralToMemory), //QUADSPI
);

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::QUADSPI, dr),
);


#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA1, Stream0, Channel5, stm32::UART8, MemoryToPeripheral), //UART8_TX
    (stm32::DMA1, Stream1, Channel5, stm32::UART7, MemoryToPeripheral), //UART7_TX
    (stm32::DMA1, Stream3, Channel5, stm32::UART7, PeripheralToMemory), //UART7_RX
    (stm32::DMA1, Stream6, Channel5, stm32::UART8, PeripheralToMemory), //UART8_RX
);

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::UART7, dr),
    (stm32::UART8, dr),
);

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (stm32::DMA1, Stream7, Channel8, stm32::UART5, MemoryToPeripheral), //UART5_TX
    (stm32::DMA2, Stream0, Channel1, stm32::UART9, MemoryToPeripheral), //UART9_TX
    (stm32::DMA2, Stream0, Channel5, stm32::UART10, PeripheralToMemory), //UART10_RX
    (stm32::DMA2, Stream3, Channel9, stm32::UART10, PeripheralToMemory), //UART10_RX:DMA_CHANNEL_9
    (stm32::DMA2, Stream5, Channel9, stm32::UART10, MemoryToPeripheral), //UART10_TX
    (stm32::DMA2, Stream7, Channel0, stm32::UART9, PeripheralToMemory), //UART9_RX
    (stm32::DMA2, Stream7, Channel6, stm32::UART10, MemoryToPeripheral), //UART10_TX:DMA_CHANNEL_6
    //(stm32::DMA2, Stream0, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT0
    //(stm32::DMA2, Stream1, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT1
    //(stm32::DMA2, Stream2, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT2
    //(stm32::DMA2, Stream3, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT3
    //(stm32::DMA2, Stream4, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT0
    //(stm32::DMA2, Stream5, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT1
    //(stm32::DMA2, Stream6, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT2
    //(stm32::DMA2, Stream7, Channel8, stm32::DFSDM2, PeripheralToMemory), //DFSDM2_FLT3
    //(stm32::DMA2, Stream6, Channel2, IN<stm32::AES>, MemoryToPeripheral), //AES_IN
    //(stm32::DMA2, Stream5, Channel2, OUT<stm32::AES>, PeripheralToMemory), //AES_OUT
);

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
))]
address!(
    //(IN<stm32::AES>, dinr),
    //(OUT<stm32::AES>, doutr),
    (stm32::UART9, dr),
    (stm32::UART10, dr),
);

/* Not sure how SAI works
#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA2, Stream1, Channel0, stm32::SAI, MemoryToPeripheral), //SAI1_A
    (stm32::DMA2, Stream1, Channel0, stm32::SAI, PeripheralToMemory), //SAI1_A
    (stm32::DMA2, Stream3, Channel0, stm32::SAI, MemoryToPeripheral), //SAI1_A
    (stm32::DMA2, Stream3, Channel0, stm32::SAI, PeripheralToMemory), //SAI1_A
    (stm32::DMA2, Stream4, Channel1, stm32::SAI, MemoryToPeripheral), //SAI1_B
    (stm32::DMA2, Stream4, Channel1, stm32::SAI, PeripheralToMemory), //SAI1_B
    (stm32::DMA2, Stream5, Channel0, stm32::SAI, MemoryToPeripheral), //SAI1_B:DMA_CHANNEL_0
    (stm32::DMA2, Stream5, Channel0, stm32::SAI, PeripheralToMemory), //SAI1_B:DMA_CHANNEL_0
);

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::SAI, dr),
);
*/

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
dma_map!(
    (stm32::DMA2, Stream5, Channel1, stm32::SPI6, MemoryToPeripheral), //SPI6_TX
    (stm32::DMA2, Stream6, Channel1, stm32::SPI6, PeripheralToMemory), //SPI6_RX
);

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!(
    (stm32::SPI6, dr),
);

/*
#[cfg(any(
    feature = "stm32f446",
))]
dma_map!(
    (stm32::DMA1, Stream1, Channel0, stm32::SPDIFRX, PeripheralToMemory), //SPDIF_RX_DT
    (stm32::DMA1, Stream2, Channel2, stm32::FMPI2C1, PeripheralToMemory), //FMPI2C1_RX
    (stm32::DMA1, Stream5, Channel2, stm32::FMPI2C1, MemoryToPeripheral), //FMPI2C1_TX
    (stm32::DMA1, Stream6, Channel0, stm32::SPDIFRX, PeripheralToMemory), //SPDIF_RX_CS
    (stm32::DMA2, Stream4, Channel3, stm32::SAI2, MemoryToPeripheral), //SAI2_A
    (stm32::DMA2, Stream4, Channel3, stm32::SAI2, PeripheralToMemory), //SAI2_A
    (stm32::DMA2, Stream6, Channel3, stm32::SAI2, MemoryToPeripheral), //SAI2_B
    (stm32::DMA2, Stream6, Channel3, stm32::SAI2, PeripheralToMemory), //SAI2_B
    (stm32::DMA2, Stream7, Channel0, stm32::SAI2, MemoryToPeripheral), //SAI2_B:DMA_CHANNEL_0
    (stm32::DMA2, Stream7, Channel0, stm32::SAI2, PeripheralToMemory), //SAI2_B:DMA_CHANNEL_0
);
#[cfg(any(
    feature = "stm32f446",
))]
address!(
    (stm32::SPDIFRX, ??),
    (stm32::FMPI2C1, ??),
    (stm32::SAI2, ??),
);
*/