use super::*;
use crate::{
    pac::{self, DMA1, DMA2},
    timer,
};
use core::ops::Deref;

pub(crate) mod sealed {
    /// Converts value to bits for setting a register value.
    pub trait Bits<T> {
        /// Returns the bit value.
        fn bits(self) -> T;
    }
}
use sealed::Bits;

/// Marker trait for structs which can be safely accessed with shared reference
pub trait SafePeripheralRead {}

/// Trait for DMA stream interrupt handling.
pub trait StreamISR: crate::Sealed {
    /// Clear all interrupts flags for the DMA stream.
    fn clear_all_flags(&mut self);

    /// Clear transfer complete interrupt (tcif) for the DMA stream.
    fn clear_transfer_complete_flag(&mut self);

    /// Clear half transfer interrupt flag (htif) for the DMA stream.
    fn clear_half_transfer_flag(&mut self);

    /// Clear transfer error interrupt flag (teif) for the DMA stream.
    fn clear_transfer_error_flag(&mut self);

    /// Clear direct mode error interrupt flag (dmeif) for the DMA stream.
    fn clear_direct_mode_error_flag(&mut self);

    /// Clear fifo error interrupt flag (feif) for the DMA stream.
    fn clear_fifo_error_flag(&mut self);

    /// Get all interrupts flags a once.
    ///
    /// The tuple contain in order:
    ///  - transfer complete flag
    ///  - half transfer flag
    ///  - transfer error flag
    ///  - direct mode error flag
    ///  - fifo_error flag
    fn all_flags(&self) -> DmaFlags;

    /// Get transfer complete flag.
    fn transfer_complete_flag(&self) -> bool;

    /// Get half transfer flag.
    fn half_transfer_flag(&self) -> bool;

    /// Get transfer error flag
    fn transfer_error_flag(&self) -> bool;

    /// Get direct mode error flag
    fn direct_mode_error_flag(&self) -> bool;

    /// Get fifo error flag
    fn fifo_error_flag(&self) -> bool;
}

/// Trait for DMA streams types.
pub trait Stream: StreamISR + crate::Sealed {
    /// Number of the register stream.
    const NUMBER: usize;
    /// Set the peripheral address (par) of the DMA stream.
    fn set_peripheral_address(&mut self, value: u32);

    /// Set the memory address (m0ar) of the DMA stream.
    fn set_memory_address(&mut self, value: u32);

    /// Get the memory address (m0ar) of the DMA stream.
    fn memory_address(&self) -> u32;

    /// Set the second memory address (m1ar) of the DMA stream. Only relevant with double buffer
    /// mode.
    fn set_alternate_memory_address(&mut self, value: u32);

    /// Get the second memory address (m1ar) of the DMA stream. Only relevant with double buffer
    /// mode.
    fn alternate_memory_address(&self) -> u32;

    /// Set the number of transfers (ndt) for the DMA stream.
    fn set_number_of_transfers(&mut self, value: u16);

    /// Get the number of transfers (ndt) for the DMA stream.
    fn number_of_transfers(&self) -> u16;

    /// Enable the DMA stream.
    ///
    /// # Safety
    ///
    /// The user must ensure that all registers are properly configured.
    unsafe fn enable(&mut self);

    /// Returns the state of the DMA stream.
    fn is_enabled(&self) -> bool;

    /// Disable the DMA stream.
    ///
    /// Disabling may not immediate, you must check with [`is_enabled()`](Stream::is_enabled) to
    /// ensure the stream is correctly disabled. Note that the transfer complete interrupt flag is
    /// set when the stream is disabled. You need to delete transfer complete interrupt flag before
    /// re-enabling the stream. It's also advisable to clear all interrupt flag before re-enabling
    /// the stream.
    ///
    /// # Safety
    ///
    /// Disabling the stream before end of transfers may produce invalid data.
    unsafe fn disable(&mut self);

    /// Set the channel for the (chsel) the DMA stream.
    fn set_channel(&mut self, channel: DmaChannel);

    /// Set the priority (pl) the DMA stream.
    fn set_priority(&mut self, priority: config::Priority);

    /// Set the peripheral increment offset (pincos)
    fn set_peripheral_increment_offset(&mut self, value: PeripheralIncrementOffset);

    /// Set the memory size (msize) for the DMA stream.
    ///
    /// # Safety
    /// This must have the same alignment of the buffer used in the transfer.
    unsafe fn set_memory_size(&mut self, size: DmaDataSize);

    /// Set the peripheral memory size (psize) for the DMA stream.
    ///
    /// # Safety
    /// This must have the same alignment of the peripheral data used in the transfer.
    unsafe fn set_peripheral_size(&mut self, size: DmaDataSize);

    /// Enable/disable memory increment (minc) for the DMA stream.
    fn set_memory_increment(&mut self, increment: bool);

    /// Enable/disable peripheral increment (pinc) for the DMA stream.
    fn set_peripheral_increment(&mut self, increment: bool);

    /// Enable/disable circular mode (circ) for the DMA stream.
    fn set_circular_mode(&mut self, value: bool);

    /// Set the direction (dir) of the DMA stream.
    fn set_direction(&mut self, direction: DmaDirection);

    /// Set the flow controller (pfctrl).
    fn set_flow_controller(&mut self, value: DmaFlowController);

    /// Convenience method to configure several interrupts of the DMA stream.
    ///
    /// Note: fifo_error interrupt is not concerend because it's in a different register
    fn set_common_interrupts(&mut self, interrupts: DmaCommonInterrupts);

    /// Convenience method to get the value of several interrupts of the DMA stream.  The order of the
    /// returns are: `transfer_complete`, `half_transfer`, `transfer_error` and `direct_mode_error`
    ///
    /// Note: fifo_error interrupt is not returned because it's in a different register
    fn common_interrupts(&self) -> DmaCommonInterrupts;

    /// Enable/disable the transfer complete interrupt (tcie) of the DMA stream.
    fn set_transfer_complete_interrupt(&mut self, transfer_complete_interrupt: bool);

    /// Enable/disable the half transfer interrupt (htie) of the DMA stream.
    fn set_half_transfer_interrupt(&mut self, half_transfer_interrupt: bool);

    /// Enable/disable the transfer error interrupt (teie) of the DMA stream.
    fn set_transfer_error_interrupt(&mut self, transfer_error_interrupt: bool);

    /// Enable/disable the direct mode error interrupt (dmeie) of the DMA stream.
    fn set_direct_mode_error_interrupt(&mut self, direct_mode_error_interrupt: bool);

    /// Enable/disable the fifo error interrupt (feie) of the DMA stream.
    fn set_fifo_error_interrupt(&mut self, fifo_error_interrupt: bool);

    /// Enable/disable the double buffer (dbm) of the DMA stream.
    fn set_double_buffer(&mut self, double_buffer: bool);

    /// Set the fifo threshold (fcr.fth) of the DMA stream.
    fn set_fifo_threshold(&mut self, fifo_threshold: config::FifoThreshold);

    /// Enable/disable the fifo (dmdis) of the DMA stream.
    fn set_fifo_enable(&mut self, fifo_enable: bool);

    /// Set memory burst mode (mburst) of the DMA stream.
    fn set_memory_burst(&mut self, memory_burst: config::BurstMode);

    /// Set peripheral burst mode (pburst) of the DMA stream.
    fn set_peripheral_burst(&mut self, peripheral_burst: config::BurstMode);

    /// Get the current fifo level (fs) of the DMA stream.
    fn fifo_level(&self) -> FifoLevel;

    /// Get which buffer is currently in use by the DMA.
    fn current_buffer(&self) -> CurrentBuffer;
}

/// DMA direction.
pub trait Direction: Bits<u8> {
    /// Creates a new instance of the type.
    fn new() -> Self;

    /// Returns the `DmaDirection` of the type.
    fn direction() -> DmaDirection;
}

/// Get an address and memory size the DMA can use.
///
/// # Safety
///
/// Both the memory size and the address must be correct for the specific peripheral and for the
/// DMA.
pub unsafe trait PeriAddress {
    /// Memory size of the peripheral.
    type MemSize;

    /// Returns the address to be used by the DMA stream.
    fn address(&self) -> u32;
}

// Convenience macro for implementing addresses on peripherals
macro_rules! address {
    ($(($peripheral:ty, $register:ident, $size: ty)),+ $(,)*) => {
        $(
            unsafe impl PeriAddress for $peripheral {
                #[inline(always)]
                fn address(&self) -> u32 {
                    &self.$register as *const _ as u32
                }

                type MemSize = $size;
            }
        )+
    };
}
use address;

/// Type alias to a DMA RegisterBlock.
pub type DMARegisterBlock = pac::dma1::RegisterBlock;

/// Trait that represents an instance of a DMA peripheral.
pub trait Instance: Deref<Target = DMARegisterBlock> + crate::Sealed {
    /// Gives a pointer to the RegisterBlock.
    fn ptr() -> *const DMARegisterBlock;
}

impl Instance for DMA1 {
    #[inline(always)]
    fn ptr() -> *const DMARegisterBlock {
        DMA1::ptr()
    }
}

impl Instance for DMA2 {
    #[inline(always)]
    fn ptr() -> *const DMARegisterBlock {
        DMA2::ptr()
    }
}

/// A trait for marker tha represent Channel of a DMA stream.
pub trait Channel {
    const VALUE: DmaChannel;
}

/// Trait to mark a set of Stream, Channel and Direction for a Peripheral as correct together.
///
/// # Safety
///
/// Memory corruption might occur if this trait is implemented for an invalid combination.
pub unsafe trait DMASet<STREAM, const CHANNEL: u8, DIRECTION> {}

macro_rules! dma_map {
    ($(($Stream:ty:$C:literal, $Peripheral:ty, [$($Dir:ty)|+])),+ $(,)*) => {
        $(
            $(
               unsafe impl DMASet<$Stream, $C, $Dir> for $Peripheral {}
            )+
        )+
    };
}
use dma_map;

mod f4;
pub use f4::*;

#[cfg(feature = "dfsdm")]
pub struct FLT<T, const F: u8> {
    _per: PhantomData<T>,
}

#[cfg(feature = "dfsdm")]
impl<T, const F: u8> crate::Sealed for FLT<T, F> {}

#[cfg(feature = "sai")]
pub struct SAICH<T, const C: u8> {
    _per: PhantomData<T>,
}

#[cfg(feature = "sai")]
impl<T, const C: u8> crate::Sealed for SAICH<T, C> {}

dma_map!(
    (Stream0<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream1<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream2<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream3<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream4<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream5<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream6<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
    (Stream7<DMA2>:0, MemoryToMemory<u8>, [MemoryToMemory<u8> | MemoryToMemory<u16> | MemoryToMemory<u32>]),
);

#[cfg(feature = "spdifrx")]
address!((pac::SPDIFRX, dr, u32),);

#[cfg(feature = "aes")]
pub struct AES_IN(());
#[cfg(feature = "aes")]
pub struct AES_OUT(());

#[cfg(feature = "aes")]
unsafe impl PeriAddress for AES_IN {
    fn address(&self) -> u32 {
        unsafe { &(*pac::AES::ptr()).dinr as *const _ as u32 }
    }
    type MemSize = u32;
}
#[cfg(feature = "aes")]
unsafe impl PeriAddress for AES_OUT {
    fn address(&self) -> u32 {
        unsafe { &(*pac::AES::ptr()).doutr as *const _ as u32 }
    }
    type MemSize = u32;
}

#[cfg(feature = "cryp")]
pub struct CRYP_IN(());
#[cfg(feature = "cryp")]
pub struct CRYP_OUT(());

#[cfg(feature = "cryp")]
unsafe impl PeriAddress for CRYP_IN {
    fn address(&self) -> u32 {
        unsafe { &(*pac::CRYP::ptr()).din as *const _ as u32 }
    }
    type MemSize = u32;
}
#[cfg(feature = "cryp")]
unsafe impl PeriAddress for CRYP_OUT {
    fn address(&self) -> u32 {
        unsafe { &(*pac::CRYP::ptr()).dout as *const _ as u32 }
    }
    type MemSize = u32;
}
