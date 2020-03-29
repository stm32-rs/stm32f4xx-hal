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

use crate::{
    bb,
    stm32::{self, DMA1, DMA2, RCC},
};
use core::{
    marker::{PhantomData, Sized},
    ops::Deref,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum DmaDirection {
    MemoryToMemory,
    PeripheralToMemory,
    MemoryToPeripheral,
}

pub(crate) mod sealed {
    /// Converts value to bits for setting a register value
    pub trait Bits<T> {
        /// Returns the bit value
        fn bits(self) -> T;
    }
    pub trait Sealed {}
}
use sealed::{Bits, Sealed};

/// Trait for DMA streams types
pub trait Stream: Sealed {
    /// Number of the register stream
    fn number() -> usize;

    /// Clear all interrupts for the DMA stream
    fn clear_interrupts(&mut self);

    /// Clear transfer complete interrupt (tcif) for the DMA stream
    fn clear_transfer_complete_interrupt(&mut self);

    /// Clear half transfer interrupt (htif) for the DMA stream
    fn clear_half_transfer_interrupt(&mut self);

    /// Clear transfer error interrupt (teif) for the DMA stream
    fn clear_transfer_error_interrupt(&mut self);

    /// Clear direct mode error interrupt (dmeif) for the DMA stream
    fn clear_direct_mode_error_interrupt(&mut self);

    /// Clear fifo error interrupt (feif) for the DMA stream
    fn clear_fifo_error_interrupt(&mut self);

    /// Set the peripheral address (par) for the DMA stream
    fn set_peripheral_address(&mut self, value: u32);

    /// Set the memory address (m0ar) for the DMA stream
    fn set_memory_address(&mut self, value: u32);

    /// Set the double buffer address (m1ar) for the DMA stream
    fn set_memory_double_buffer_address(&mut self, value: u32);

    /// Set the number of transfers (ndt) for the DMA stream
    fn set_number_of_transfers(&mut self, value: u16);

    /// Enable the DMA stream
    ///
    /// # Safety
    ///
    /// The user must ensure that all registers are properly configured
    unsafe fn enable(&mut self);

    /// Returns the state of the DMA stream
    fn is_enabled() -> bool;

    /// Disable the DMA stream
    fn disable(&mut self);

    /// Set the channel for the (chsel) the DMA stream
    fn set_channel<C: Channel>(&mut self, channel: C);

    /// Set the priority (pl) the DMA stream
    fn set_priority(&mut self, priority: config::Priority);

    /// Set the memory size (msize) for the DMA stream
    fn set_memory_size(&mut self, size: config::TransferSize);

    /// Set the peripheral memory size (psize) for the DMA stream
    fn set_peripheral_size(&mut self, size: config::TransferSize);

    /// Enable/disable memory increment (minc) for the DMA stream
    fn set_memory_increment(&mut self, increment: bool);

    /// Enable/disable peripheral increment (pinc) for the DMA stream
    fn set_peripheral_increment(&mut self, increment: bool);

    /// Enable/disable circular mode (circ) for the DMA stream
    fn set_circular(&mut self, circular: bool);

    /// Set the direction (dir) of the DMA stream
    fn set_direction<D: Direction>(&mut self, direction: D);

    /// Convenience method to configure the 4 common interrupts for the DMA stream
    fn set_interrupts_enable(
        &mut self,
        transfer_complete: bool,
        half_transfer: bool,
        transfer_error: bool,
        direct_mode_error: bool,
    );

    /// Enable/disable the transfer complete interrupt (tcie) of the DMA stream
    fn set_transfer_complete_interrupt_enable(&mut self, transfer_complete_interrupt: bool);

    /// Enable/disable the half transfer interrupt (htie) of the DMA stream
    fn set_half_transfer_interrupt_enable(&mut self, half_transfer_interrupt: bool);

    /// Enable/disable the transfer error interrupt (teie) of the DMA stream
    fn set_transfer_error_interrupt_enable(&mut self, transfer_error_interrupt: bool);

    /// Enable/disable the direct mode error interrupt (dmeie) of the DMA stream
    fn set_direct_mode_error_interrupt_enable(&mut self, direct_mode_error_interrupt: bool);

    /// Enable/disable the fifo error interrupt (feie) of the DMA stream
    fn set_fifo_error_interrupt_enable(&mut self, fifo_error_interrupt: bool);

    /// Enable/disable the double buffer (dbm) of the DMA stream
    fn set_double_buffer(&mut self, double_buffer: bool);

    /// Set the fifo threshold (fcr.fth) of the DMA stream
    fn set_fifo_threshold(&mut self, fifo_threshold: config::FifoThreshold);

    /// Enable/disable the fifo (dmdis) of the DMA stream
    fn set_fifo_enable(&mut self, fifo_enable: bool);

    /// Set memory burst mode (mburst) of the DMA stream
    fn set_memory_burst(&mut self, memory_burst: config::BurstMode);

    /// Set peripheral burst mode (pburst) of the DMA stream
    fn set_peripheral_burst(&mut self, peripheral_burst: config::BurstMode);

    /// Get the current fifo level (fs) of the DMA stream
    fn fifo_level() -> FifoLevel;

    /// Set the flow controller (pfctrl) of the DMA stream
    fn set_flow_controller(&mut self, flow_controller: config::FlowController);

    /// Get which buffer is currently in use
    fn current_buffer() -> CurrentBuffer;
}

/// DMA direction
pub trait Direction: Bits<u8> {}

/// DMA from a peripheral to a memory location
#[derive(Debug, Clone, Copy)]
pub struct PeripheralToMemory;
impl Bits<u8> for PeripheralToMemory {
    fn bits(self) -> u8 {
        0
    }
}
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
impl Bits<u8> for MemoryToMemory {
    fn bits(self) -> u8 {
        2
    }
}
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
impl Bits<u8> for MemoryToPeripheral {
    fn bits(self) -> u8 {
        1
    }
}
impl Direction for MemoryToPeripheral {}
impl MemoryToPeripheral {
    fn new() -> Self {
        MemoryToPeripheral
    }
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToPeripheral
    }
}

/// Get an address the DMA can use
pub trait Address {
    /// Returns the address to be used by the DMA stream
    fn address(&self) -> u32;
}

impl Address for &u32 {
    #[inline]
    fn address(&self) -> u32 {
        *self as *const _ as u32
    }
}

impl Address for &u16 {
    #[inline]
    fn address(&self) -> u32 {
        *self as *const _ as u32
    }
}

// TODO: remove ?
impl Address for MemoryToMemory {
    fn address(&self) -> u32 {
        unimplemented!()
    }
}

impl Address for &[u16] {
    #[inline]
    fn address(&self) -> u32 {
        self.as_ptr() as *const _ as u32
    }
}

/// Convenience macro for implementing addresses on peripherals
macro_rules! address {
    ($(($peripheral:ty, $register:ident)),+ $(,)*) => {
        $(
            impl Address for $peripheral {
                #[inline]
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
                #[inline]
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

// TODO: remove ?
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
pub struct Stream0<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 1 on the DMA controller. See Stream0 for more info.
pub struct Stream1<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 2 on the DMA controller. See Stream0 for more info.
pub struct Stream2<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 3 on the DMA controller. See Stream0 for more info.
pub struct Stream3<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 4 on the DMA controller. See Stream0 for more info.
pub struct Stream4<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 5 on the DMA controller. See Stream0 for more info.
pub struct Stream5<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 6 on the DMA controller. See Stream0 for more info.
pub struct Stream6<DMA> {
    _dma: PhantomData<DMA>,
}
/// Stream 7 on the DMA controller. See Stream0 for more info.
pub struct Stream7<DMA> {
    _dma: PhantomData<DMA>,
}

impl<DMA> Sealed for Stream0<DMA> {}
impl<DMA> Sealed for Stream1<DMA> {}
impl<DMA> Sealed for Stream2<DMA> {}
impl<DMA> Sealed for Stream3<DMA> {}
impl<DMA> Sealed for Stream4<DMA> {}
impl<DMA> Sealed for Stream5<DMA> {}
impl<DMA> Sealed for Stream6<DMA> {}
impl<DMA> Sealed for Stream7<DMA> {}

impl Sealed for DMA1 {}
impl Sealed for DMA2 {}

#[cfg(not(any(feature = "stm32f411", feature = "stm32f413", feature = "stm32f423")))]
/// Type alias to a DMA RegisterBlock
pub type DMARegisterBlock = stm32::dma2::RegisterBlock;

#[cfg(any(feature = "stm32f411", feature = "stm32f413", feature = "stm32f423"))]
/// Type alias to a DMA RegisterBlock
pub type DMARegisterBlock = stm32::dma1::RegisterBlock;

/// Trait that represents an instance of a DMA peripheral
pub trait Instace: Deref<Target = DMARegisterBlock> + Sealed {
    /// Gives a pointer to the RegisterBlock
    fn ptr() -> *const DMARegisterBlock;
}

impl Instace for DMA1 {
    fn ptr() -> *const DMARegisterBlock {
        DMA1::ptr()
    }
}

impl Instace for DMA2 {
    fn ptr() -> *const DMARegisterBlock {
        DMA1::ptr()
    }
}

/// Things that implement this can have their RCC enabled
trait RccEnable {
    fn rcc_enable(&self);
}

impl RccEnable for stm32::DMA1 {
    fn rcc_enable(&self) {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable and reset the timer peripheral
            bb::set(&rcc.ahb1enr, 21);
            bb::set(&rcc.ahb1rstr, 21);
            bb::clear(&rcc.ahb1rstr, 21);
        }
    }
}

impl RccEnable for stm32::DMA2 {
    fn rcc_enable(&self) {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*RCC::ptr());
            // Enable and reset the timer peripheral
            bb::set(&rcc.ahb1enr, 22);
            bb::set(&rcc.ahb1rstr, 22);
            bb::clear(&rcc.ahb1rstr, 22);
        }
    }
}

/// Extension trait to split the DMA peripheral
pub trait DMAExt: Sized {
    /// Splits the DMA into channels
    fn split(
        self,
    ) -> (
        Stream0<Self>,
        Stream1<Self>,
        Stream2<Self>,
        Stream3<Self>,
        Stream4<Self>,
        Stream5<Self>,
        Stream6<Self>,
        Stream7<Self>,
    );
}

impl DMAExt for DMA1 {
    fn split(
        self,
    ) -> (
        Stream0<Self>,
        Stream1<Self>,
        Stream2<Self>,
        Stream3<Self>,
        Stream4<Self>,
        Stream5<Self>,
        Stream6<Self>,
        Stream7<Self>,
    ) {
        self.rcc_enable();
        (
            Stream0 { _dma: PhantomData },
            Stream1 { _dma: PhantomData },
            Stream2 { _dma: PhantomData },
            Stream3 { _dma: PhantomData },
            Stream4 { _dma: PhantomData },
            Stream5 { _dma: PhantomData },
            Stream6 { _dma: PhantomData },
            Stream7 { _dma: PhantomData },
        )
    }
}

impl DMAExt for DMA2 {
    fn split(
        self,
    ) -> (
        Stream0<Self>,
        Stream1<Self>,
        Stream2<Self>,
        Stream3<Self>,
        Stream4<Self>,
        Stream5<Self>,
        Stream6<Self>,
        Stream7<Self>,
    ) {
        self.rcc_enable();
        (
            Stream0 { _dma: PhantomData },
            Stream1 { _dma: PhantomData },
            Stream2 { _dma: PhantomData },
            Stream3 { _dma: PhantomData },
            Stream4 { _dma: PhantomData },
            Stream5 { _dma: PhantomData },
            Stream6 { _dma: PhantomData },
            Stream7 { _dma: PhantomData },
        )
    }
}

/// Macro that creates a struct representing a stream on either DMA controller
/// The implementation does the heavy lifting of mapping to the right fields on the stream
macro_rules! dma_stream {
    ($(($name:ident, $number:expr ,$ifcr:ident, $tcif:ident, $htif:ident, $teif:ident, $dmeif:ident,
        $feif:ident)),+ $(,)*) => {
        $(
            #[allow(dead_code)]
            impl<I: Instace> Stream for $name<I> {

                #[inline]
                fn number() -> usize {
                    $number
                }

                #[inline]
                fn clear_interrupts(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w
                        .$tcif().set_bit() //Clear transfer complete interrupt flag
                        .$htif().set_bit() //Clear half transfer interrupt flag
                        .$teif().set_bit() //Clear transfer error interrupt flag
                        .$dmeif().set_bit() //Clear direct mode error interrupt flag
                        .$feif().set_bit() //Clear fifo error interrupt flag
                    );
                }

                #[inline]
                fn clear_transfer_complete_interrupt(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$tcif().set_bit());
                }

                #[inline]
                fn clear_half_transfer_interrupt(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$htif().set_bit());
                }

                #[inline]
                fn clear_transfer_error_interrupt(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$teif().set_bit());
                }

                #[inline]
                fn clear_direct_mode_error_interrupt(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$dmeif().set_bit());
                }

                #[inline]
                fn clear_fifo_error_interrupt(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$feif().set_bit());
                }

                #[inline]
                fn set_peripheral_address(&mut self, value: u32) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].par.write(|w| w.pa().bits(value));
                }

                #[inline]
                fn set_memory_address(&mut self, value: u32) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].m0ar.write(|w| w.m0a().bits(value));
                }

                #[inline]
                fn set_memory_double_buffer_address(&mut self, value: u32) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].m1ar.write(|w| w.m1a().bits(value));
                }

                #[inline]
                fn set_number_of_transfers(&mut self, value: u16) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].ndtr.write(|w| w.ndt().bits(value));
                }

                #[inline]
                unsafe fn enable(&mut self) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = &*I::ptr();
                    dma.st[Self::number()].cr.modify(|_, w| w.en().set_bit());
                }

                #[inline]
                fn is_enabled() -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.read().en().bit_is_set()
                }

                #[inline]
                fn disable(&mut self) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.en().clear_bit());
                }

                #[inline]
                fn set_channel<C: Channel>(&mut self, channel: C) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    //Some device crates have this field unsafe, others don't.
                    #[allow(unused_unsafe)]
                    dma.st[Self::number()].cr.modify(|_, w| unsafe { w.chsel().bits(channel.bits()) });
                }

                #[inline]
                fn set_priority(&mut self, priority: config::Priority) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.pl().bits(priority.bits()));
                }

                #[inline]
                fn set_memory_size(&mut self, size: config::TransferSize) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| unsafe { w.msize().bits(size.bits()) });
                }

                #[inline]
                fn set_peripheral_size(&mut self, size: config::TransferSize) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| unsafe { w.psize().bits(size.bits()) });
                }

                #[inline]
                fn set_memory_increment(&mut self, increment: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.minc().bit(increment));
                }

                #[inline]
                fn set_peripheral_increment(&mut self, increment: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.pinc().bit(increment));
                }

                #[inline]
                fn set_circular(&mut self, circular: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.circ().bit(circular));
                }

                #[inline]
                fn set_direction<D: Direction>(&mut self, direction: D) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| unsafe { w.dir().bits(direction.bits()) });
                }

                #[inline]
                fn set_interrupts_enable(&mut self, transfer_complete: bool, half_transfer: bool, transfer_error: bool, direct_mode_error: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w
                        .tcie().bit(transfer_complete)
                        .htie().bit(half_transfer)
                        .teie().bit(transfer_error)
                        .dmeie().bit(direct_mode_error)
                    );
                }

                #[inline]
                fn set_transfer_complete_interrupt_enable(&mut self, transfer_complete_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.tcie().bit(transfer_complete_interrupt));
                }

                #[inline]
                fn set_half_transfer_interrupt_enable(&mut self, half_transfer_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.htie().bit(half_transfer_interrupt));
                }

                #[inline]
                fn set_transfer_error_interrupt_enable(&mut self, transfer_error_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.teie().bit(transfer_error_interrupt));
                }

                #[inline]
                fn set_direct_mode_error_interrupt_enable(&mut self, direct_mode_error_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.dmeie().bit(direct_mode_error_interrupt));
                }

                #[inline]
                fn set_fifo_error_interrupt_enable(&mut self, fifo_error_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].fcr.modify(|_, w| w.feie().bit(fifo_error_interrupt));
                }

                #[inline]
                fn set_double_buffer(&mut self, double_buffer: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.dbm().bit(double_buffer));
                }

                #[inline]
                fn set_fifo_threshold(&mut self, fifo_threshold: config::FifoThreshold) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].fcr.modify(|_, w| w.fth().bits(fifo_threshold.bits()));
                }

                #[inline]
                fn set_fifo_enable(&mut self, fifo_enable: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    //Register is actually direct mode disable rather than fifo enable
                    dma.st[Self::number()].fcr.modify(|_, w| w.dmdis().bit(fifo_enable));
                }

                #[inline]
                fn set_memory_burst(&mut self, memory_burst: config::BurstMode) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.mburst().bits(memory_burst.bits()));
                }

                #[inline]
                fn set_peripheral_burst(&mut self, peripheral_burst: config::BurstMode) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.pburst().bits(peripheral_burst.bits()));
                }

                #[inline]
                fn fifo_level() -> FifoLevel {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].fcr.read().fs().bits().into()
                }

                #[inline]
                fn set_flow_controller(&mut self, flow_controller: config::FlowController) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::number()].cr.modify(|_, w| w.pfctrl().bit(flow_controller.bits()));
                }

                fn current_buffer() -> CurrentBuffer {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    if dma.st[Self::number()].cr.read().ct().bit_is_set() {
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
    (Stream0, 0, lifcr, ctcif0, chtif0, cteif0, cdmeif0, cfeif0),
    (Stream1, 1, lifcr, ctcif1, chtif1, cteif1, cdmeif1, cfeif1),
    (Stream2, 2, lifcr, ctcif2, chtif2, cteif2, cdmeif2, cfeif2),
    (Stream3, 3, lifcr, ctcif3, chtif3, cteif3, cdmeif3, cfeif3),
    (Stream4, 4, hifcr, ctcif4, chtif4, cteif4, cdmeif4, cfeif4),
    (Stream5, 5, hifcr, ctcif5, chtif5, cteif5, cdmeif5, cfeif5),
    (Stream6, 6, hifcr, ctcif6, chtif6, cteif6, cdmeif6, cfeif6),
    (Stream7, 7, hifcr, ctcif7, chtif7, cteif7, cdmeif7, cfeif7),
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

#[cfg(any(feature = "stm32f413", feature = "stm32f423",))]
dma_channel!((Channel8, 8), (Channel9, 9),);

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
        #[inline]
        pub fn memory_size(mut self, memory_size: TransferSize) -> Self {
            self.memory_size = memory_size;
            self
        }
        /// Set the peripheral_size
        #[inline]
        pub fn peripheral_size(mut self, peripheral_size: TransferSize) -> Self {
            self.peripheral_size = peripheral_size;
            self
        }
        /// Set the number_of_transfers
        #[inline]
        pub fn number_of_transfers(mut self, number_of_transfers: u16) -> Self {
            self.number_of_transfers = number_of_transfers;
            self
        }
        /// Set the priority
        #[inline]
        pub fn priority(mut self, priority: Priority) -> Self {
            self.priority = priority;
            self
        }
        /// Set the memory_increment
        #[inline]
        pub fn memory_increment(mut self, memory_increment: bool) -> Self {
            self.memory_increment = memory_increment;
            self
        }
        /// Set the peripheral_increment
        #[inline]
        pub fn peripheral_increment(mut self, peripheral_increment: bool) -> Self {
            self.peripheral_increment = peripheral_increment;
            self
        }
        /// Set the circular
        #[inline]
        pub fn circular(mut self, circular: bool) -> Self {
            self.circular = circular;
            self
        }
        /// Set the transfer_complete_interrupt
        #[inline]
        pub fn transfer_complete_interrupt(mut self, transfer_complete_interrupt: bool) -> Self {
            self.transfer_complete_interrupt = transfer_complete_interrupt;
            self
        }
        /// Set the half_transfer_interrupt
        #[inline]
        pub fn half_transfer_interrupt(mut self, half_transfer_interrupt: bool) -> Self {
            self.half_transfer_interrupt = half_transfer_interrupt;
            self
        }
        /// Set the transfer_error_interrupt
        #[inline]
        pub fn transfer_error_interrupt(mut self, transfer_error_interrupt: bool) -> Self {
            self.transfer_error_interrupt = transfer_error_interrupt;
            self
        }
        /// Set the direct_mode_error_interrupt
        #[inline]
        pub fn direct_mode_error_interrupt(mut self, direct_mode_error_interrupt: bool) -> Self {
            self.direct_mode_error_interrupt = direct_mode_error_interrupt;
            self
        }
        /// Set the fifo_error_interrupt
        #[inline]
        pub fn fifo_error_interrupt(mut self, fifo_error_interrupt: bool) -> Self {
            self.fifo_error_interrupt = fifo_error_interrupt;
            self
        }
        /// Set the double_buffer
        #[inline]
        pub fn double_buffer(mut self, double_buffer: bool) -> Self {
            self.double_buffer = double_buffer;
            self
        }
        /// Set the fifo_threshold
        #[inline]
        pub fn fifo_threshold(mut self, fifo_threshold: FifoThreshold) -> Self {
            self.fifo_threshold = fifo_threshold;
            self
        }
        /// Set the fifo_enable
        #[inline]
        pub fn fifo_enable(mut self, fifo_enable: bool) -> Self {
            self.fifo_enable = fifo_enable;
            self
        }
        /// Set the memory_burst
        #[inline]
        pub fn memory_burst(mut self, memory_burst: BurstMode) -> Self {
            self.memory_burst = memory_burst;
            self
        }
        /// Set the peripheral_burst
        #[inline]
        pub fn peripheral_burst(mut self, peripheral_burst: BurstMode) -> Self {
            self.peripheral_burst = peripheral_burst;
            self
        }
        /// Set the flow_controller
        #[inline]
        pub fn flow_controller(mut self, flow_controller: FlowController) -> Self {
            self.flow_controller = flow_controller;
            self
        }
    }
}

/// DMA Stream
pub struct DmaStream<STREAM: Stream, CHANNEL, PERIPHERAL, DIRECTION> {
    stream: STREAM,
    _channel: PhantomData<CHANNEL>,
    _peripheral: PhantomData<PERIPHERAL>,
    _direction: PhantomData<DIRECTION>,
}

macro_rules! dma_map {
    ($(($Stream:ty, $channel:ty, $peripheral:ty, $dir:ty)),+ $(,)*) => {
        $(
            impl DmaStream<$Stream, $channel, $peripheral, $dir> {

                /// Applies all fields in DmaConfig
                pub fn apply_config(&mut self, config: config::DmaConfig) {
                    let was_enabled = <$Stream>::is_enabled();
                    if was_enabled {
                        self.stream.disable();
                    }

                    self.stream.clear_interrupts();
                    self.stream.set_priority(config.priority);
                    self.stream.set_memory_size(config.memory_size);
                    self.stream.set_peripheral_size(config.peripheral_size);
                    self.stream.set_memory_increment(config.memory_increment);
                    self.stream.set_peripheral_increment(config.peripheral_increment);
                    self.stream.set_circular(config.circular);
                    self.stream.set_transfer_complete_interrupt_enable(config.transfer_complete_interrupt);
                    self.stream.set_half_transfer_interrupt_enable(config.half_transfer_interrupt);
                    self.stream.set_transfer_error_interrupt_enable(config.transfer_error_interrupt);
                    self.stream.set_direct_mode_error_interrupt_enable(config.direct_mode_error_interrupt);
                    self.stream.set_fifo_error_interrupt_enable(config.fifo_error_interrupt);
                    self.stream.set_number_of_transfers(config.number_of_transfers);
                    self.stream.set_double_buffer(config.double_buffer);
                    self.stream.set_fifo_threshold(config.fifo_threshold);
                    self.stream.set_fifo_enable(config.fifo_enable);
                    self.stream.set_memory_burst(config.memory_burst);
                    self.stream.set_peripheral_burst(config.peripheral_burst);
                    self.stream.set_flow_controller(config.flow_controller);

                    if was_enabled {
                        unsafe { self.stream.enable(); }
                    }
                }

                /// Configures DMA stream to correct channel for peripheral, configures source and destination addresses and applies supplied config
                ///
                /// # Safety
                ///
                /// Buffer must be valid for DMA
                pub unsafe fn init<MA>(stream: $Stream , peripheral: &$peripheral, memory: MA, double_buffer: Option<MA>, config: config::DmaConfig) -> Self
                where
                    MA: Address,
                {
                    let mut transfer = Self {
                        stream,
                        _channel: PhantomData,
                        _peripheral: PhantomData,
                        _direction: PhantomData,
                    };

                    transfer.stream.disable();

                    //Set the channel
                    transfer.stream.set_channel(<$channel>::new());

                    //Set peripheral to memory mode
                    transfer.stream.set_direction(<$dir>::new());

                    //Set the memory address
                    transfer.stream.set_memory_address(memory.address());

                    let is_mem2mem = <$dir>::direction() == DmaDirection::MemoryToMemory;
                    if is_mem2mem {
                        //Fifo must be enabled for memory to memory
                        assert!(config.fifo_enable);
                    } else {
                        //Set the peripheral address
                        transfer.stream.set_peripheral_address(peripheral.address());
                    }

                    if let Some(db) = double_buffer {
                        if is_mem2mem {
                            //Double buffer is the source in mem2mem mode
                            transfer.stream.set_peripheral_address(db.address());
                        } else {
                            transfer.stream.set_memory_double_buffer_address(db.address());
                        }
                    } else {
                        // Double buffer mode must not be enabled if we haven't been given a second buffer
                        assert!(!config.double_buffer);
                    }

                    transfer.apply_config(config);
                    transfer.stream.enable();

                    transfer
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
    (
        Stream0<DMA1>,
        Channel2,
        CCR1<stm32::TIM4>,
        MemoryToPeripheral
    ), //TIM4_CH1
    (
        Stream0<DMA1>,
        Channel2,
        CCR1<stm32::TIM4>,
        PeripheralToMemory
    ), //TIM4_CH1
    (
        Stream2<DMA1>,
        Channel5,
        CCR4<stm32::TIM3>,
        MemoryToPeripheral
    ), //TIM3_CH4
    (
        Stream2<DMA1>,
        Channel5,
        CCR4<stm32::TIM3>,
        PeripheralToMemory
    ), //TIM3_CH4
    (
        Stream2<DMA1>,
        Channel5,
        DMAR<stm32::TIM3>,
        MemoryToPeripheral
    ), //TIM3_UP
    (
        Stream2<DMA1>,
        Channel5,
        DMAR<stm32::TIM3>,
        PeripheralToMemory
    ), //TIM3_UP
    (
        Stream3<DMA1>,
        Channel2,
        CCR2<stm32::TIM4>,
        MemoryToPeripheral
    ), //TIM4_CH2
    (
        Stream3<DMA1>,
        Channel2,
        CCR2<stm32::TIM4>,
        PeripheralToMemory
    ), //TIM4_CH2
    (
        Stream4<DMA1>,
        Channel5,
        CCR1<stm32::TIM3>,
        MemoryToPeripheral
    ), //TIM3_CH1
    (
        Stream4<DMA1>,
        Channel5,
        CCR1<stm32::TIM3>,
        PeripheralToMemory
    ), //TIM3_CH1
    (
        Stream4<DMA1>,
        Channel5,
        DMAR<stm32::TIM3>,
        MemoryToPeripheral
    ), //TIM3_TRIG
    (
        Stream4<DMA1>,
        Channel5,
        DMAR<stm32::TIM3>,
        PeripheralToMemory
    ), //TIM3_TRIG
    (
        Stream5<DMA1>,
        Channel3,
        CCR1<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH1
    (
        Stream5<DMA1>,
        Channel3,
        CCR1<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH1
    (
        Stream5<DMA1>,
        Channel5,
        CCR2<stm32::TIM3>,
        MemoryToPeripheral
    ), //TIM3_CH2
    (
        Stream5<DMA1>,
        Channel5,
        CCR2<stm32::TIM3>,
        PeripheralToMemory
    ), //TIM3_CH2
    (
        Stream6<DMA1>,
        Channel2,
        DMAR<stm32::TIM4>,
        MemoryToPeripheral
    ), //TIM4_UP
    (
        Stream6<DMA1>,
        Channel2,
        DMAR<stm32::TIM4>,
        PeripheralToMemory
    ), //TIM4_UP
    (
        Stream6<DMA1>,
        Channel3,
        CCR2<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH2
    (
        Stream6<DMA1>,
        Channel3,
        CCR2<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH2
    (
        Stream6<DMA1>,
        Channel3,
        CCR4<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH4
    (
        Stream6<DMA1>,
        Channel3,
        CCR4<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH4
    (
        Stream7<DMA1>,
        Channel2,
        CCR3<stm32::TIM4>,
        MemoryToPeripheral
    ), //TIM4_CH3
    (
        Stream7<DMA1>,
        Channel2,
        CCR3<stm32::TIM4>,
        PeripheralToMemory
    ), //TIM4_CH3
    (
        Stream7<DMA1>,
        Channel5,
        CCR3<stm32::TIM3>,
        MemoryToPeripheral
    ), //TIM3_CH3
    (
        Stream7<DMA1>,
        Channel5,
        CCR3<stm32::TIM3>,
        PeripheralToMemory
    ), //TIM3_CH3
    (Stream0<DMA1>, Channel0, stm32::SPI3, PeripheralToMemory), //SPI3_RX
    (Stream2<DMA1>, Channel0, stm32::SPI3, PeripheralToMemory), //SPI3_RX
    (Stream4<DMA1>, Channel3, stm32::I2C3, MemoryToPeripheral), //I2C3_TX
    (Stream5<DMA1>, Channel0, stm32::SPI3, MemoryToPeripheral), //SPI3_TX
    (Stream7<DMA1>, Channel0, stm32::SPI3, MemoryToPeripheral), //SPI3_TX
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
    (
        Stream0<DMA1>,
        Channel6,
        CCR3<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_CH3
    (
        Stream0<DMA1>,
        Channel6,
        CCR3<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_CH3
    (
        Stream0<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_UP
    (
        Stream0<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_UP
    (
        Stream1<DMA1>,
        Channel6,
        CCR4<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_CH4
    (
        Stream1<DMA1>,
        Channel6,
        CCR4<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_CH4
    (
        Stream1<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_TRIG
    (
        Stream1<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_TRIG
    (
        Stream2<DMA1>,
        Channel6,
        CCR1<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_CH1
    (
        Stream2<DMA1>,
        Channel6,
        CCR1<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_CH1
    (
        Stream3<DMA1>,
        Channel6,
        CCR4<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_CH4
    (
        Stream3<DMA1>,
        Channel6,
        CCR4<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_CH4
    (
        Stream3<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_TRIG
    (
        Stream3<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_TRIG
    (
        Stream4<DMA1>,
        Channel6,
        CCR2<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_CH2
    (
        Stream4<DMA1>,
        Channel6,
        CCR2<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_CH2
    (
        Stream6<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        MemoryToPeripheral
    ), //TIM5_UP
    (
        Stream6<DMA1>,
        Channel6,
        DMAR<stm32::TIM5>,
        PeripheralToMemory
    ), //TIM5_UP
    (
        Stream0<DMA2>,
        Channel6,
        DMAR<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_TRIG
    (
        Stream0<DMA2>,
        Channel6,
        DMAR<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_TRIG
    (
        Stream1<DMA2>,
        Channel6,
        CCR1<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH1
    (
        Stream1<DMA2>,
        Channel6,
        CCR1<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH1
    (
        Stream2<DMA2>,
        Channel6,
        CCR2<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH2
    (
        Stream2<DMA2>,
        Channel6,
        CCR2<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH2
    (
        Stream3<DMA2>,
        Channel6,
        CCR1<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH1
    (
        Stream3<DMA2>,
        Channel6,
        CCR1<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH1
    (
        Stream4<DMA2>,
        Channel6,
        CCR4<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH4
    (
        Stream4<DMA2>,
        Channel6,
        CCR4<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH4
    (
        Stream4<DMA2>,
        Channel6,
        DMAR<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_TRIG/COM
    (
        Stream4<DMA2>,
        Channel6,
        DMAR<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_TRIG/COM
    (
        Stream5<DMA2>,
        Channel6,
        DMAR<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_UP
    (
        Stream5<DMA2>,
        Channel6,
        DMAR<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_UP
    (
        Stream6<DMA2>,
        Channel0,
        CCR1<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH1
    (
        Stream6<DMA2>,
        Channel0,
        CCR1<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH1
    (
        Stream6<DMA2>,
        Channel0,
        CCR2<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH2
    (
        Stream6<DMA2>,
        Channel0,
        CCR2<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH2
    (
        Stream6<DMA2>,
        Channel0,
        CCR3<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH3
    (
        Stream6<DMA2>,
        Channel0,
        CCR3<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH3
    (
        Stream6<DMA2>,
        Channel6,
        CCR3<stm32::TIM1>,
        MemoryToPeripheral
    ), //TIM1_CH3
    (
        Stream6<DMA2>,
        Channel6,
        CCR3<stm32::TIM1>,
        PeripheralToMemory
    ), //TIM1_CH3
    (Stream0<DMA1>, Channel1, stm32::I2C1, PeripheralToMemory), //I2C1_RX
    (Stream2<DMA1>, Channel7, stm32::I2C2, PeripheralToMemory), //I2C2_RX
    (Stream3<DMA1>, Channel0, stm32::SPI2, PeripheralToMemory), //SPI2_RX
    (Stream3<DMA1>, Channel7, stm32::I2C2, PeripheralToMemory), //I2C2_RX
    (Stream4<DMA1>, Channel0, stm32::SPI2, MemoryToPeripheral), //SPI2_TX
    (Stream5<DMA1>, Channel1, stm32::I2C1, PeripheralToMemory), //I2C1_RX
    (Stream5<DMA1>, Channel4, stm32::USART2, PeripheralToMemory), //USART2_RX
    (Stream6<DMA1>, Channel4, stm32::USART2, MemoryToPeripheral), //USART2_TX
    (Stream7<DMA1>, Channel7, stm32::I2C2, MemoryToPeripheral), //I2C2_TX
    (Stream0<DMA2>, Channel0, stm32::ADC1, PeripheralToMemory), //ADC1
    (Stream0<DMA2>, Channel3, stm32::SPI1, PeripheralToMemory), //SPI1_RX
    (Stream1<DMA2>, Channel5, stm32::USART6, PeripheralToMemory), //USART6_RX
    (Stream2<DMA2>, Channel3, stm32::SPI1, PeripheralToMemory), //SPI1_RX
    (Stream2<DMA2>, Channel4, stm32::USART1, PeripheralToMemory), //USART1_RX
    (Stream2<DMA2>, Channel5, stm32::USART6, PeripheralToMemory), //USART6_RX
    (Stream4<DMA2>, Channel0, stm32::ADC1, PeripheralToMemory), //ADC1
    (Stream5<DMA2>, Channel4, stm32::USART1, PeripheralToMemory), //USART1_RX
    (Stream6<DMA2>, Channel5, stm32::USART6, MemoryToPeripheral), //USART6_TX
    (Stream7<DMA2>, Channel4, stm32::USART1, MemoryToPeripheral), //USART1_TX
    (Stream7<DMA2>, Channel5, stm32::USART6, MemoryToPeripheral), //USART6_TX
    (Stream0<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream1<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream2<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream3<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream4<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream5<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream6<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
    (Stream7<DMA2>, Channel0, MemoryToMemory, MemoryToMemory),
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
    (Stream1<DMA1>, Channel1, stm32::I2C3, PeripheralToMemory), //I2C3_RX
    (Stream2<DMA1>, Channel3, stm32::I2C3, PeripheralToMemory), //I2C3_RX:DMA_CHANNEL_3
);

#[cfg(any(feature = "stm32f401", feature = "stm32f411",))]
dma_map!(
    (
        Stream1<DMA1>,
        Channel3,
        CCR3<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH3
    (
        Stream1<DMA1>,
        Channel3,
        CCR3<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH3
    (
        Stream1<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_UP
    (
        Stream1<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_UP
    (
        Stream7<DMA1>,
        Channel3,
        CCR4<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH4
    (
        Stream7<DMA1>,
        Channel3,
        CCR4<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH4
    (
        Stream7<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_UP
    (
        Stream7<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_UP
);

#[cfg(any(feature = "stm32f401", feature = "stm32f411",))]
address!((CCR3<stm32::TIM2>, ccr3), (DMAR<stm32::TIM2>, dmar),);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!((Stream5<DMA1>, Channel6, stm32::I2C3, MemoryToPeripheral),); //I2C3_TX:DMA_CHANNEL_6);

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
    (Stream6<DMA1>, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX
    (Stream7<DMA1>, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX
    (Stream3<DMA2>, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX
    (Stream5<DMA2>, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX
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
    (Stream0<DMA2>, Channel4, stm32::SPI4, PeripheralToMemory), //SPI4_RX
    (Stream1<DMA2>, Channel4, stm32::SPI4, MemoryToPeripheral), //SPI4_TX
    (Stream3<DMA2>, Channel5, stm32::SPI4, PeripheralToMemory), //SPI4_RX:DMA_CHANNEL_5
    (Stream4<DMA2>, Channel5, stm32::SPI4, MemoryToPeripheral), //SPI4_TX:DMA_CHANNEL_5
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
address!((stm32::SPI4, dr),);

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
    (Stream0<DMA1>, Channel4, stm32::UART5, PeripheralToMemory), //UART5_RX
    (Stream2<DMA1>, Channel4, stm32::UART4, PeripheralToMemory), //UART4_RX
    (Stream4<DMA1>, Channel4, stm32::UART4, MemoryToPeripheral), //UART4_TX
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
    (
        Stream1<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_UP
    (
        Stream1<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_UP
    (
        Stream1<DMA1>,
        Channel3,
        CCR3<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH3
    (
        Stream1<DMA1>,
        Channel3,
        CCR3<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH3
    //(stm32::DMA1, Stream2, Channel1, DMAR<stm32::TIM7>, MemoryToPeripheral), //TIM7_UP //dmar register appears to be missing
    //(stm32::DMA1, Stream2, Channel1, DMAR<stm32::TIM7>, PeripheralToMemory), //TIM7_UP //dmar register appears to be missing
    //(stm32::DMA1, Stream4, Channel1, DMAR<stm32::TIM7>, MemoryToPeripheral), //TIM7_UP //dmar register appears to be missing
    //(stm32::DMA1, Stream4, Channel1, DMAR<stm32::TIM7>, PeripheralToMemory), //TIM7_UP //dmar register appears to be missing
    (
        Stream7<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_UP
    (
        Stream7<DMA1>,
        Channel3,
        DMAR<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_UP
    (
        Stream7<DMA1>,
        Channel3,
        CCR4<stm32::TIM2>,
        MemoryToPeripheral
    ), //TIM2_CH4
    (
        Stream7<DMA1>,
        Channel3,
        CCR4<stm32::TIM2>,
        PeripheralToMemory
    ), //TIM2_CH4
    (
        Stream1<DMA2>,
        Channel7,
        DMAR<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_UP
    (
        Stream1<DMA2>,
        Channel7,
        DMAR<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_UP
    (
        Stream2<DMA2>,
        Channel0,
        CCR1<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH1
    (
        Stream2<DMA2>,
        Channel0,
        CCR1<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH1
    (
        Stream2<DMA2>,
        Channel0,
        CCR2<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH2
    (
        Stream2<DMA2>,
        Channel0,
        CCR2<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH2
    (
        Stream2<DMA2>,
        Channel0,
        CCR3<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH3
    (
        Stream2<DMA2>,
        Channel0,
        CCR3<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH3
    (
        Stream2<DMA2>,
        Channel7,
        CCR1<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH1
    (
        Stream2<DMA2>,
        Channel7,
        CCR1<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH1
    (
        Stream3<DMA2>,
        Channel7,
        CCR2<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH2
    (
        Stream3<DMA2>,
        Channel7,
        CCR2<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH2
    (
        Stream4<DMA2>,
        Channel7,
        CCR3<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH3
    (
        Stream4<DMA2>,
        Channel7,
        CCR3<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH3
    (
        Stream7<DMA2>,
        Channel7,
        CCR4<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_CH4
    (
        Stream7<DMA2>,
        Channel7,
        CCR4<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_CH4
    (
        Stream7<DMA2>,
        Channel7,
        DMAR<stm32::TIM8>,
        MemoryToPeripheral
    ), //TIM8_COM/TRIG
    (
        Stream7<DMA2>,
        Channel7,
        DMAR<stm32::TIM8>,
        PeripheralToMemory
    ), //TIM8_COM/TRIG
    (Stream1<DMA1>, Channel4, stm32::USART3, PeripheralToMemory), //USART3_RX
    (Stream3<DMA1>, Channel4, stm32::USART3, MemoryToPeripheral), //USART3_TX
    (Stream4<DMA1>, Channel7, stm32::USART3, MemoryToPeripheral), //USART3_TX:DMA_CHANNEL_7
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

/*
DMAR register appears to be missing from TIM6 derived timers on these devices
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
    (Stream2<DMA1>, Channel3, stm32::I2C3, PeripheralToMemory), //I2C3_RX
    (Stream5<DMA2>, Channel2, stm32::CRYP, PeripheralToMemory), //CRYP_OUT
    (Stream6<DMA2>, Channel2, stm32::CRYP, MemoryToPeripheral), //CRYP_IN
    (Stream7<DMA2>, Channel2, stm32::HASH, MemoryToPeripheral), //HASH_IN
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
address!((stm32::HASH, din), (stm32::CRYP, din),);

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
    (Stream7<DMA1>, Channel4, stm32::UART5, MemoryToPeripheral), //UART5_TX
    (Stream0<DMA2>, Channel2, stm32::ADC3, PeripheralToMemory),  //ADC3
    (Stream1<DMA2>, Channel1, stm32::DCMI, PeripheralToMemory),  //DCMI
    (Stream1<DMA2>, Channel2, stm32::ADC3, PeripheralToMemory),  //ADC3
    (Stream2<DMA2>, Channel1, stm32::ADC2, PeripheralToMemory),  //ADC2
    (Stream3<DMA2>, Channel1, stm32::ADC2, PeripheralToMemory),  //ADC2
    (Stream7<DMA2>, Channel1, stm32::DCMI, PeripheralToMemory),  //DCMI
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
address!((stm32::ADC2, dr), (stm32::ADC3, dr), (stm32::DCMI, dr),);

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
    (Stream1<DMA1>, Channel0, stm32::I2C1, MemoryToPeripheral), //I2C1_TX
    (Stream6<DMA1>, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX:DMA_CHANNEL_1
    (Stream7<DMA1>, Channel1, stm32::I2C1, MemoryToPeripheral), //I2C1_TX:DMA_CHANNEL_1
    (Stream7<DMA1>, Channel6, stm32::USART2, PeripheralToMemory), //USART2_RX:DMA_CHANNEL_6
    (Stream2<DMA2>, Channel2, stm32::SPI1, MemoryToPeripheral), //SPI1_TX
    (Stream3<DMA2>, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX:DMA_CHANNEL_3
    (Stream5<DMA2>, Channel3, stm32::SPI1, MemoryToPeripheral), //SPI1_TX:DMA_CHANNEL_3
    (Stream5<DMA2>, Channel5, stm32::SPI5, MemoryToPeripheral), //SPI5_TX:DMA_CHANNEL_5
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
    (Stream3<DMA2>, Channel2, stm32::SPI5, PeripheralToMemory), //SPI5_RX
    (Stream4<DMA2>, Channel2, stm32::SPI5, MemoryToPeripheral), //SPI5_TX
    (Stream5<DMA2>, Channel7, stm32::SPI5, PeripheralToMemory), //SPI5_RX:DMA_CHANNEL_7
    (Stream6<DMA2>, Channel7, stm32::SPI5, MemoryToPeripheral), //SPI5_TX:DMA_CHANNEL_7
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
address!((stm32::SPI5, dr),);

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!((Stream4<DMA2>, Channel4, stm32::SPI4, PeripheralToMemory),); //SPI4_RX);

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
    (Stream7<DMA2>, Channel3, stm32::QUADSPI, MemoryToPeripheral), //QUADSPI
    (Stream7<DMA2>, Channel3, stm32::QUADSPI, PeripheralToMemory), //QUADSPI
);

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!((stm32::QUADSPI, dr),);

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
    (Stream0<DMA1>, Channel5, stm32::UART8, MemoryToPeripheral), //UART8_TX
    (Stream1<DMA1>, Channel5, stm32::UART7, MemoryToPeripheral), //UART7_TX
    (Stream3<DMA1>, Channel5, stm32::UART7, PeripheralToMemory), //UART7_RX
    (Stream6<DMA1>, Channel5, stm32::UART8, PeripheralToMemory), //UART8_RX
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
address!((stm32::UART7, dr), (stm32::UART8, dr),);

#[cfg(any(feature = "stm32f413", feature = "stm32f423",))]
dma_map!(
    (Stream7<DMA1>, Channel8, stm32::UART5, MemoryToPeripheral), //UART5_TX
    (Stream0<DMA2>, Channel1, stm32::UART9, MemoryToPeripheral), //UART9_TX
    (Stream0<DMA2>, Channel5, stm32::UART10, PeripheralToMemory), //UART10_RX
    (Stream3<DMA2>, Channel9, stm32::UART10, PeripheralToMemory), //UART10_RX:DMA_CHANNEL_9
    (Stream5<DMA2>, Channel9, stm32::UART10, MemoryToPeripheral), //UART10_TX
    (Stream7<DMA2>, Channel0, stm32::UART9, PeripheralToMemory), //UART9_RX
    (Stream7<DMA2>, Channel6, stm32::UART10, MemoryToPeripheral), //UART10_TX:DMA_CHANNEL_6
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

#[cfg(any(feature = "stm32f413", feature = "stm32f423",))]
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
    (Stream5<DMA2>, Channel1, stm32::SPI6, MemoryToPeripheral), //SPI6_TX
    (Stream6<DMA2>, Channel1, stm32::SPI6, PeripheralToMemory), //SPI6_RX
);

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!((stm32::SPI6, dr),);

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
