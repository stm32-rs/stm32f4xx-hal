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
    /// Clear all interrupts for the DMA stream.
    fn clear_interrupts(&mut self);

    /// Clear transfer complete interrupt (tcif) for the DMA stream.
    fn clear_transfer_complete_interrupt(&mut self);

    /// Clear half transfer interrupt (htif) for the DMA stream.
    fn clear_half_transfer_interrupt(&mut self);

    /// Clear transfer error interrupt (teif) for the DMA stream.
    fn clear_transfer_error_interrupt(&mut self);

    /// Clear direct mode error interrupt (dmeif) for the DMA stream.
    fn clear_direct_mode_error_interrupt(&mut self);

    /// Clear fifo error interrupt (feif) for the DMA stream.
    fn clear_fifo_error_interrupt(&mut self);

    /// Get transfer complete flag.
    fn get_transfer_complete_flag() -> bool;

    /// Get half transfer flag.
    fn get_half_transfer_flag() -> bool;

    /// Get transfer error flag
    fn get_transfer_error_flag() -> bool;

    /// Get fifo error flag
    fn get_fifo_error_flag() -> bool;

    /// Get direct mode error flag
    fn get_direct_mode_error_flag() -> bool;
}

/// Trait for DMA streams types.
pub trait Stream: StreamISR + crate::Sealed {
    /// Number of the register stream.
    const NUMBER: usize;
    /// Set the peripheral address (par) for the DMA stream.
    fn set_peripheral_address(&mut self, value: u32);

    /// Set the memory address (m0ar) for the DMA stream.
    fn set_memory_address(&mut self, value: u32);

    /// Get the memory address (m0ar) for the DMA stream.
    fn get_memory_address(&self) -> u32;

    /// Set the double buffer address (m1ar) for the DMA stream.
    fn set_memory_double_buffer_address(&mut self, value: u32);

    /// Get the double buffer address (m1ar) for the DMA stream.
    fn get_memory_double_buffer_address(&self) -> u32;

    /// Set the number of transfers (ndt) for the DMA stream.
    fn set_number_of_transfers(&mut self, value: u16);

    /// Get the number of transfers (ndt) for the DMA stream.
    fn get_number_of_transfers() -> u16;

    /// Enable the DMA stream.
    ///
    /// # Safety
    ///
    /// The user must ensure that all registers are properly configured.
    unsafe fn enable(&mut self);

    /// Returns the state of the DMA stream.
    fn is_enabled() -> bool;

    /// Disable the DMA stream.
    ///
    /// Disabling the stream during an on-going transfer needs to be performed in a certain way to
    /// prevent problems if the stream is to be re-enabled shortly after, because of that, this
    /// method will also clear all the stream's interrupt flags if the stream is active.
    fn disable(&mut self);

    /// Set the channel for the (chsel) the DMA stream.
    fn set_channel<const C: u8>(&mut self)
    where
        ChannelX<C>: Channel;

    /// Set the priority (pl) the DMA stream.
    fn set_priority(&mut self, priority: config::Priority);

    /// Set the memory size (msize) for the DMA stream.
    ///
    /// # Safety
    /// This must have the same alignment of the buffer used in the transfer.
    /// Valid values:
    ///     * 0 -> byte
    ///     * 1 -> half word
    ///     * 2 -> word
    unsafe fn set_memory_size(&mut self, size: u8);

    /// Set the peripheral memory size (psize) for the DMA stream.
    ///
    /// # Safety
    /// This must have the same alignment of the peripheral data used in the transfer.
    /// Valid values:
    ///     * 0 -> byte
    ///     * 1 -> half word
    ///     * 2 -> word
    unsafe fn set_peripheral_size(&mut self, size: u8);

    /// Enable/disable memory increment (minc) for the DMA stream.
    fn set_memory_increment(&mut self, increment: bool);

    /// Enable/disable peripheral increment (pinc) for the DMA stream.
    fn set_peripheral_increment(&mut self, increment: bool);

    /// Set the direction (dir) of the DMA stream.
    fn set_direction<D: Direction>(&mut self, direction: D);

    /// Convenience method to configure the 4 common interrupts for the DMA stream.
    fn set_interrupts_enable(
        &mut self,
        transfer_complete: bool,
        half_transfer: bool,
        transfer_error: bool,
        direct_mode_error: bool,
    );

    /// Convenience method to get the value of the 4 common interrupts for the DMA stream.
    /// The order of the returns are: `transfer_complete`, `half_transfer`, `transfer_error` and
    /// `direct_mode_error`.
    fn get_interrupts_enable() -> (bool, bool, bool, bool);

    /// Enable/disable the transfer complete interrupt (tcie) of the DMA stream.
    fn set_transfer_complete_interrupt_enable(&mut self, transfer_complete_interrupt: bool);

    /// Enable/disable the half transfer interrupt (htie) of the DMA stream.
    fn set_half_transfer_interrupt_enable(&mut self, half_transfer_interrupt: bool);

    /// Enable/disable the transfer error interrupt (teie) of the DMA stream.
    fn set_transfer_error_interrupt_enable(&mut self, transfer_error_interrupt: bool);

    /// Enable/disable the direct mode error interrupt (dmeie) of the DMA stream.
    fn set_direct_mode_error_interrupt_enable(&mut self, direct_mode_error_interrupt: bool);

    /// Enable/disable the fifo error interrupt (feie) of the DMA stream.
    fn set_fifo_error_interrupt_enable(&mut self, fifo_error_interrupt: bool);

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
    fn fifo_level() -> FifoLevel;

    /// Get which buffer is currently in use by the DMA.
    fn current_buffer() -> CurrentBuffer;
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

/// A channel that can be configured on a DMA stream.
pub trait Channel {}

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

/*
#[cfg(feature = "c0")]
mod c0;
#[cfg(feature = "c0")]
pub use c0::*;

#[cfg(feature = "f0")]
mod f0;
#[cfg(feature = "f0")]
pub use f0::*;

#[cfg(feature = "f2")]
mod f2;
#[cfg(feature = "f2")]
pub use f2::*;

#[cfg(feature = "f3")]
mod f3;
#[cfg(feature = "f3")]
pub use f3::*;
*/
#[cfg(feature = "f4")]
mod f4;
#[cfg(feature = "f4")]
pub use f4::*;

#[cfg(feature = "f7")]
mod f7;
#[cfg(feature = "f7")]
pub use f7::*;
/*
#[cfg(feature = "g0")]
mod g0;
#[cfg(feature = "g0")]
pub use g0::*;

#[cfg(feature = "g4")]
mod g4;
#[cfg(feature = "g4")]
pub use g4::*;

#[cfg(feature = "h7")]
mod h7;
#[cfg(feature = "h7")]
pub use h7::*;

#[cfg(feature = "l0")]
mod l0;
#[cfg(feature = "l0")]
pub use l0::*;

#[cfg(feature = "l1")]
mod l1;
#[cfg(feature = "l1")]
pub use l1::*;
*/
#[cfg(feature = "l4x")]
mod l4;
#[cfg(feature = "l4x")]
pub use l4::*;
/*
#[cfg(feature = "l4p")]
mod l4p;
#[cfg(feature = "l4p")]
pub use l4p::*;

#[cfg(feature = "l5")]
mod l5;
#[cfg(feature = "l5")]
pub use l5::*;

#[cfg(feature = "u5")]
mod u5;
#[cfg(feature = "u5")]
pub use u5::*;

#[cfg(feature = "wl")]
mod wl;
#[cfg(feature = "wl")]
pub use wl::*;

#[cfg(feature = "wb")]
mod wb;
#[cfg(feature = "wb")]
pub use wb::*;
*/

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
address!(
    (pac::SPDIFRX, dr, u32),
);

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
