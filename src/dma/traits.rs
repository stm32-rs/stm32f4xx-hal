use super::*;
use crate::{
    adc::Adc,
    pac::{self, DMA1, DMA2},
    serial::{Rx, Tx},
};
use core::ops::Deref;

pub(crate) mod sealed {
    /// Converts value to bits for setting a register value.
    pub trait Bits<T> {
        /// Returns the bit value.
        fn bits(self) -> T;
    }
    pub trait Sealed {}
}
use sealed::{Bits, Sealed};

/// Trait for DMA stream interrupt handling.
pub trait StreamISR: Sealed {
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
pub trait Stream: StreamISR + Sealed {
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
    fn set_channel<C: Channel>(&mut self, channel: C);

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

impl Sealed for DMA1 {}
impl Sealed for DMA2 {}

#[cfg(not(any(
    feature = "stm32f411",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f410"
)))]
/// Type alias to a DMA RegisterBlock.
pub type DMARegisterBlock = pac::dma2::RegisterBlock;

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f410"
))]
/// Type alias to a DMA RegisterBlock.
pub type DMARegisterBlock = pac::dma1::RegisterBlock;

/// Trait that represents an instance of a DMA peripheral.
pub trait Instance: Deref<Target = DMARegisterBlock> + Sealed {
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

macro_rules! tim_channels {
    ($($name:ident),+ $(,)*) => {
        $(
            /// Wrapper type that indicates which register of the contained timer to use for DMA.
            pub struct $name<T> (pub T);

            impl<T> Deref for $name<T> {
                type Target = T;

                #[inline(always)]
                fn deref(&self) -> &T {
                    &self.0
                }
            }
        )+
    };
}

/// A channel that can be configured on a DMA stream.
pub trait Channel: Bits<u8> {
    /// Returns a new instance of the type.
    fn new() -> Self;
}

/// Trait to mark a set of Stream, Channel and Direction for a Peripheral as correct together.
///
/// # Safety
///
/// Memory corruption might occur if this trait is implemented for an invalid combination.
pub unsafe trait DMASet<STREAM, CHANNEL, DIRECTION> {}

tim_channels!(CCR1, CCR2, CCR3, CCR4, DMAR, ARR);

macro_rules! dma_map {
    ($(($Stream:ty, $C:literal, $Peripheral:ty, $Dir:ty)),+ $(,)*) => {
        $(
            unsafe impl DMASet<$Stream, ChannelX<$C>, $Dir> for $Peripheral {}
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
    (Stream0<DMA1>, 2, CCR1<pac::TIM4>, MemoryToPeripheral), //TIM4_CH1
    (Stream0<DMA1>, 2, CCR1<pac::TIM4>, PeripheralToMemory), //TIM4_CH1
    (Stream2<DMA1>, 5, CCR4<pac::TIM3>, MemoryToPeripheral), //TIM3_CH4
    (Stream2<DMA1>, 5, CCR4<pac::TIM3>, PeripheralToMemory), //TIM3_CH4
    (Stream2<DMA1>, 5, DMAR<pac::TIM3>, MemoryToPeripheral), //TIM3_UP
    (Stream2<DMA1>, 5, DMAR<pac::TIM3>, PeripheralToMemory), //TIM3_UP
    (Stream3<DMA1>, 2, CCR2<pac::TIM4>, MemoryToPeripheral), //TIM4_CH2
    (Stream3<DMA1>, 2, CCR2<pac::TIM4>, PeripheralToMemory), //TIM4_CH2
    (Stream4<DMA1>, 5, CCR1<pac::TIM3>, MemoryToPeripheral), //TIM3_CH1
    (Stream4<DMA1>, 5, CCR1<pac::TIM3>, PeripheralToMemory), //TIM3_CH1
    (Stream4<DMA1>, 5, DMAR<pac::TIM3>, MemoryToPeripheral), //TIM3_TRIG
    (Stream4<DMA1>, 5, DMAR<pac::TIM3>, PeripheralToMemory), //TIM3_TRIG
    (Stream5<DMA1>, 3, CCR1<pac::TIM2>, MemoryToPeripheral), //TIM2_CH1
    (Stream5<DMA1>, 3, CCR1<pac::TIM2>, PeripheralToMemory), //TIM2_CH1
    (Stream5<DMA1>, 5, CCR2<pac::TIM3>, MemoryToPeripheral), //TIM3_CH2
    (Stream5<DMA1>, 5, CCR2<pac::TIM3>, PeripheralToMemory), //TIM3_CH2
    (Stream6<DMA1>, 2, DMAR<pac::TIM4>, MemoryToPeripheral), //TIM4_UP
    (Stream6<DMA1>, 2, DMAR<pac::TIM4>, PeripheralToMemory), //TIM4_UP
    (Stream6<DMA1>, 3, CCR2<pac::TIM2>, MemoryToPeripheral), //TIM2_CH2
    (Stream6<DMA1>, 3, CCR2<pac::TIM2>, PeripheralToMemory), //TIM2_CH2
    (Stream6<DMA1>, 3, CCR4<pac::TIM2>, MemoryToPeripheral), //TIM2_CH4
    (Stream6<DMA1>, 3, CCR4<pac::TIM2>, PeripheralToMemory), //TIM2_CH4
    (Stream7<DMA1>, 2, CCR3<pac::TIM4>, MemoryToPeripheral), //TIM4_CH3
    (Stream7<DMA1>, 2, CCR3<pac::TIM4>, PeripheralToMemory), //TIM4_CH3
    (Stream7<DMA1>, 5, CCR3<pac::TIM3>, MemoryToPeripheral), //TIM3_CH3
    (Stream7<DMA1>, 5, CCR3<pac::TIM3>, PeripheralToMemory), //TIM3_CH3
    (Stream0<DMA1>, 0, pac::SPI3, PeripheralToMemory),       //SPI3_RX
    (Stream2<DMA1>, 0, pac::SPI3, PeripheralToMemory),       //SPI3_RX
    (Stream4<DMA1>, 3, pac::I2C3, MemoryToPeripheral),       //I2C3_TX
    (Stream5<DMA1>, 0, pac::SPI3, MemoryToPeripheral),       //SPI3_TX
    (Stream7<DMA1>, 0, pac::SPI3, MemoryToPeripheral),       //SPI3_TX
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
    (CCR1<pac::TIM4>, ccr1, u16),
    (CCR4<pac::TIM3>, ccr4, u16),
    (CCR1<pac::TIM2>, ccr1, u16),
    (CCR1<pac::TIM3>, ccr1, u16),
    (CCR2<pac::TIM2>, ccr2, u16),
    (CCR2<pac::TIM3>, ccr2, u16),
    (CCR2<pac::TIM4>, ccr2, u16),
    (CCR3<pac::TIM3>, ccr3, u16),
    (CCR3<pac::TIM4>, ccr3, u16),
    (CCR4<pac::TIM2>, ccr4, u16),
    (DMAR<pac::TIM3>, dmar, u16),
    (DMAR<pac::TIM4>, dmar, u16),
    (pac::SPI3, dr, u8),
    (pac::I2C3, dr, u8),
);

#[cfg(not(any(feature = "stm32f410")))]
dma_map!(
    (Stream3<DMA2>, 4, pac::SDIO, MemoryToPeripheral), //SDIO
    (Stream3<DMA2>, 4, pac::SDIO, PeripheralToMemory), //SDIO
    (Stream6<DMA2>, 4, pac::SDIO, MemoryToPeripheral), //SDIO
    (Stream6<DMA2>, 4, pac::SDIO, PeripheralToMemory), //SDIO
);

#[cfg(not(any(feature = "stm32f410")))]
address!((pac::SDIO, fifo, u32),);

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
    (Stream0<DMA1>, 6, CCR3<pac::TIM5>, MemoryToPeripheral), //TIM5_CH3
    (Stream0<DMA1>, 6, CCR3<pac::TIM5>, PeripheralToMemory), //TIM5_CH3
    (Stream0<DMA1>, 6, DMAR<pac::TIM5>, MemoryToPeripheral), //TIM5_UP
    (Stream0<DMA1>, 6, DMAR<pac::TIM5>, PeripheralToMemory), //TIM5_UP
    (Stream1<DMA1>, 6, CCR4<pac::TIM5>, MemoryToPeripheral), //TIM5_CH4
    (Stream1<DMA1>, 6, CCR4<pac::TIM5>, PeripheralToMemory), //TIM5_CH4
    (Stream1<DMA1>, 6, DMAR<pac::TIM5>, MemoryToPeripheral), //TIM5_TRIG
    (Stream1<DMA1>, 6, DMAR<pac::TIM5>, PeripheralToMemory), //TIM5_TRIG
    (Stream2<DMA1>, 6, CCR1<pac::TIM5>, MemoryToPeripheral), //TIM5_CH1
    (Stream2<DMA1>, 6, CCR1<pac::TIM5>, PeripheralToMemory), //TIM5_CH1
    (Stream3<DMA1>, 6, CCR4<pac::TIM5>, MemoryToPeripheral), //TIM5_CH4
    (Stream3<DMA1>, 6, CCR4<pac::TIM5>, PeripheralToMemory), //TIM5_CH4
    (Stream3<DMA1>, 6, DMAR<pac::TIM5>, MemoryToPeripheral), //TIM5_TRIG
    (Stream3<DMA1>, 6, DMAR<pac::TIM5>, PeripheralToMemory), //TIM5_TRIG
    (Stream4<DMA1>, 6, CCR2<pac::TIM5>, MemoryToPeripheral), //TIM5_CH2
    (Stream4<DMA1>, 6, CCR2<pac::TIM5>, PeripheralToMemory), //TIM5_CH2
    (Stream6<DMA1>, 6, DMAR<pac::TIM5>, MemoryToPeripheral), //TIM5_UP
    (Stream6<DMA1>, 6, DMAR<pac::TIM5>, PeripheralToMemory), //TIM5_UP
    (Stream0<DMA2>, 6, DMAR<pac::TIM1>, MemoryToPeripheral), //TIM1_TRIG
    (Stream0<DMA2>, 6, DMAR<pac::TIM1>, PeripheralToMemory), //TIM1_TRIG
    (Stream1<DMA2>, 6, CCR1<pac::TIM1>, MemoryToPeripheral), //TIM1_CH1
    (Stream1<DMA2>, 6, CCR1<pac::TIM1>, PeripheralToMemory), //TIM1_CH1
    (Stream2<DMA2>, 6, CCR2<pac::TIM1>, MemoryToPeripheral), //TIM1_CH2
    (Stream2<DMA2>, 6, CCR2<pac::TIM1>, PeripheralToMemory), //TIM1_CH2
    (Stream3<DMA2>, 6, CCR1<pac::TIM1>, MemoryToPeripheral), //TIM1_CH1
    (Stream3<DMA2>, 6, CCR1<pac::TIM1>, PeripheralToMemory), //TIM1_CH1
    (Stream4<DMA2>, 6, CCR4<pac::TIM1>, MemoryToPeripheral), //TIM1_CH4
    (Stream4<DMA2>, 6, CCR4<pac::TIM1>, PeripheralToMemory), //TIM1_CH4
    (Stream4<DMA2>, 6, DMAR<pac::TIM1>, MemoryToPeripheral), //TIM1_TRIG/COM
    (Stream4<DMA2>, 6, DMAR<pac::TIM1>, PeripheralToMemory), //TIM1_TRIG/COM
    (Stream5<DMA2>, 6, DMAR<pac::TIM1>, MemoryToPeripheral), //TIM1_UP
    (Stream5<DMA2>, 6, DMAR<pac::TIM1>, PeripheralToMemory), //TIM1_UP
    (Stream6<DMA2>, 0, CCR1<pac::TIM1>, MemoryToPeripheral), //TIM1_CH1
    (Stream6<DMA2>, 0, CCR1<pac::TIM1>, PeripheralToMemory), //TIM1_CH1
    (Stream6<DMA2>, 0, CCR2<pac::TIM1>, MemoryToPeripheral), //TIM1_CH2
    (Stream6<DMA2>, 0, CCR2<pac::TIM1>, PeripheralToMemory), //TIM1_CH2
    (Stream6<DMA2>, 0, CCR3<pac::TIM1>, MemoryToPeripheral), //TIM1_CH3
    (Stream6<DMA2>, 0, CCR3<pac::TIM1>, PeripheralToMemory), //TIM1_CH3
    (Stream6<DMA2>, 6, CCR3<pac::TIM1>, MemoryToPeripheral), //TIM1_CH3
    (Stream6<DMA2>, 6, CCR3<pac::TIM1>, PeripheralToMemory), //TIM1_CH3
    (Stream0<DMA1>, 1, pac::I2C1, PeripheralToMemory),       //I2C1_RX
    (Stream2<DMA1>, 7, pac::I2C2, PeripheralToMemory),       //I2C2_RX
    (Stream3<DMA1>, 0, pac::SPI2, PeripheralToMemory),       //SPI2_RX
    (Stream3<DMA1>, 7, pac::I2C2, PeripheralToMemory),       //I2C2_RX
    (Stream4<DMA1>, 0, pac::SPI2, MemoryToPeripheral),       //SPI2_TX
    (Stream5<DMA1>, 1, pac::I2C1, PeripheralToMemory),       //I2C1_RX
    (Stream5<DMA1>, 4, pac::USART2, PeripheralToMemory),     //USART2_RX
    (Stream5<DMA1>, 4, Rx<pac::USART2>, PeripheralToMemory), //USART2_RX
    (Stream6<DMA1>, 4, pac::USART2, MemoryToPeripheral),     //USART2_TX
    (Stream6<DMA1>, 4, Tx<pac::USART2>, MemoryToPeripheral), //USART2_TX
    (Stream7<DMA1>, 7, pac::I2C2, MemoryToPeripheral),       //I2C2_TX
    (Stream0<DMA2>, 0, pac::ADC1, PeripheralToMemory),       //ADC1
    (Stream0<DMA2>, 0, Adc<pac::ADC1>, PeripheralToMemory),
    (Stream0<DMA2>, 3, pac::SPI1, PeripheralToMemory), //SPI1_RX
    (Stream1<DMA2>, 5, pac::USART6, PeripheralToMemory), //USART6_RX
    (Stream1<DMA2>, 5, Rx<pac::USART6>, PeripheralToMemory), //USART6_RX
    (Stream2<DMA2>, 3, pac::SPI1, PeripheralToMemory), //SPI1_RX
    (Stream2<DMA2>, 4, pac::USART1, PeripheralToMemory), //USART1_RX
    (Stream2<DMA2>, 4, Rx<pac::USART1>, PeripheralToMemory), //USART1_RX
    (Stream2<DMA2>, 5, pac::USART6, PeripheralToMemory), //USART6_RX
    (Stream2<DMA2>, 5, Rx<pac::USART6>, PeripheralToMemory), //USART6_RX
    (Stream4<DMA2>, 0, pac::ADC1, PeripheralToMemory), //ADC1
    (Stream5<DMA2>, 4, pac::USART1, PeripheralToMemory), //USART1_RX
    (Stream5<DMA2>, 4, Rx<pac::USART1>, PeripheralToMemory), //USART1_RX
    (Stream6<DMA2>, 5, pac::USART6, MemoryToPeripheral), //USART6_TX
    (Stream6<DMA2>, 5, Tx<pac::USART6>, MemoryToPeripheral), //USART6_TX
    (Stream7<DMA2>, 4, pac::USART1, MemoryToPeripheral), //USART1_TX
    (Stream7<DMA2>, 4, Tx<pac::USART1>, MemoryToPeripheral), //USART1_TX
    (Stream7<DMA2>, 5, pac::USART6, MemoryToPeripheral), //USART6_TX
    (Stream7<DMA2>, 5, Tx<pac::USART6>, MemoryToPeripheral), //USART6_TX
    (Stream0<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream1<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream2<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream3<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream4<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream5<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream6<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream7<DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>),
    (Stream0<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream1<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream2<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream3<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream4<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream5<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream6<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream7<DMA2>, 0, MemoryToMemory<u16>, MemoryToMemory<u16>),
    (Stream0<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream1<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream2<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream3<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream4<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream5<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream6<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
    (Stream7<DMA2>, 0, MemoryToMemory<u32>, MemoryToMemory<u32>),
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
    (CCR1<pac::TIM1>, ccr1, u16),
    (CCR2<pac::TIM1>, ccr2, u16),
    (CCR3<pac::TIM1>, ccr3, u16),
    (CCR4<pac::TIM1>, ccr4, u16),
    (DMAR<pac::TIM1>, dmar, u16),
    (CCR1<pac::TIM5>, ccr1, u16),
    (CCR2<pac::TIM5>, ccr2, u16),
    (CCR3<pac::TIM5>, ccr3, u16),
    (CCR4<pac::TIM5>, ccr4, u16),
    (DMAR<pac::TIM5>, dmar, u16),
    (pac::ADC1, dr, u16),
    (pac::I2C1, dr, u8),
    (pac::I2C2, dr, u8),
    (pac::SPI1, dr, u8),
    (pac::SPI2, dr, u8),
    (pac::USART1, dr, u8),
    (pac::USART2, dr, u8),
    (pac::USART6, dr, u8),
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
    (Stream1<DMA1>, 1, pac::I2C3, PeripheralToMemory), //I2C3_RX
    (Stream2<DMA1>, 3, pac::I2C3, PeripheralToMemory), //I2C3_RX:DMA_CHANNEL_3
);

#[cfg(any(feature = "stm32f401", feature = "stm32f411",))]
dma_map!(
    (Stream1<DMA1>, 3, CCR3<pac::TIM2>, MemoryToPeripheral), //TIM2_CH3
    (Stream1<DMA1>, 3, CCR3<pac::TIM2>, PeripheralToMemory), //TIM2_CH3
    (Stream1<DMA1>, 3, DMAR<pac::TIM2>, MemoryToPeripheral), //TIM2_UP
    (Stream1<DMA1>, 3, DMAR<pac::TIM2>, PeripheralToMemory), //TIM2_UP
    (Stream7<DMA1>, 3, CCR4<pac::TIM2>, MemoryToPeripheral), //TIM2_CH4
    (Stream7<DMA1>, 3, CCR4<pac::TIM2>, PeripheralToMemory), //TIM2_CH4
    (Stream7<DMA1>, 3, DMAR<pac::TIM2>, MemoryToPeripheral), //TIM2_UP
    (Stream7<DMA1>, 3, DMAR<pac::TIM2>, PeripheralToMemory), //TIM2_UP
);

#[cfg(any(feature = "stm32f401", feature = "stm32f411",))]
address!((CCR3<pac::TIM2>, ccr3, u16), (DMAR<pac::TIM2>, dmar, u16),);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!((Stream5<DMA1>, 6, pac::I2C3, MemoryToPeripheral),); //I2C3_TX:DMA_CHANNEL_6);

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
    (Stream6<DMA1>, 1, pac::I2C1, MemoryToPeripheral), //I2C1_TX
    (Stream7<DMA1>, 1, pac::I2C1, MemoryToPeripheral), //I2C1_TX
    (Stream3<DMA2>, 3, pac::SPI1, MemoryToPeripheral), //SPI1_TX
    (Stream5<DMA2>, 3, pac::SPI1, MemoryToPeripheral), //SPI1_TX
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
    (Stream0<DMA2>, 4, pac::SPI4, PeripheralToMemory), //SPI4_RX
    (Stream1<DMA2>, 4, pac::SPI4, MemoryToPeripheral), //SPI4_TX
    (Stream3<DMA2>, 5, pac::SPI4, PeripheralToMemory), //SPI4_RX:DMA_CHANNEL_5
    (Stream4<DMA2>, 5, pac::SPI4, MemoryToPeripheral), //SPI4_TX:DMA_CHANNEL_5
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
address!((pac::SPI4, dr, u8),);

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
    (Stream0<DMA1>, 4, pac::UART5, PeripheralToMemory), //UART5_RX
    (Stream2<DMA1>, 4, pac::UART4, PeripheralToMemory), //UART4_RX
    (Stream4<DMA1>, 4, pac::UART4, MemoryToPeripheral), //UART4_TX
    //(Stream6<DMA1>, 7, pac::DAC2, MemoryToPeripheral), //DAC2
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
    (pac::UART4, dr, u8),
    (pac::UART5, dr, u8),
    //(pac::DAC, ??),
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
    (Stream1<DMA1>, 3, DMAR<pac::TIM2>, MemoryToPeripheral), //TIM2_UP
    (Stream1<DMA1>, 3, DMAR<pac::TIM2>, PeripheralToMemory), //TIM2_UP
    (Stream1<DMA1>, 3, CCR3<pac::TIM2>, MemoryToPeripheral), //TIM2_CH3
    (Stream1<DMA1>, 3, CCR3<pac::TIM2>, PeripheralToMemory), //TIM2_CH3
    //(pac::DMA1, Stream2, 1, DMAR<pac::TIM7>, MemoryToPeripheral), //TIM7_UP //dmar register appears to be missing
    //(pac::DMA1, Stream2, 1, DMAR<pac::TIM7>, PeripheralToMemory), //TIM7_UP //dmar register appears to be missing
    //(pac::DMA1, Stream4, 1, DMAR<pac::TIM7>, MemoryToPeripheral), //TIM7_UP //dmar register appears to be missing
    //(pac::DMA1, Stream4, 1, DMAR<pac::TIM7>, PeripheralToMemory), //TIM7_UP //dmar register appears to be missing
    (Stream7<DMA1>, 3, DMAR<pac::TIM2>, MemoryToPeripheral), //TIM2_UP
    (Stream7<DMA1>, 3, DMAR<pac::TIM2>, PeripheralToMemory), //TIM2_UP
    (Stream7<DMA1>, 3, CCR4<pac::TIM2>, MemoryToPeripheral), //TIM2_CH4
    (Stream7<DMA1>, 3, CCR4<pac::TIM2>, PeripheralToMemory), //TIM2_CH4
    (Stream1<DMA2>, 7, DMAR<pac::TIM8>, MemoryToPeripheral), //TIM8_UP
    (Stream1<DMA2>, 7, DMAR<pac::TIM8>, PeripheralToMemory), //TIM8_UP
    (Stream2<DMA2>, 0, CCR1<pac::TIM8>, MemoryToPeripheral), //TIM8_CH1
    (Stream2<DMA2>, 0, CCR1<pac::TIM8>, PeripheralToMemory), //TIM8_CH1
    (Stream2<DMA2>, 0, CCR2<pac::TIM8>, MemoryToPeripheral), //TIM8_CH2
    (Stream2<DMA2>, 0, CCR2<pac::TIM8>, PeripheralToMemory), //TIM8_CH2
    (Stream2<DMA2>, 0, CCR3<pac::TIM8>, MemoryToPeripheral), //TIM8_CH3
    (Stream2<DMA2>, 0, CCR3<pac::TIM8>, PeripheralToMemory), //TIM8_CH3
    (Stream2<DMA2>, 7, CCR1<pac::TIM8>, MemoryToPeripheral), //TIM8_CH1
    (Stream2<DMA2>, 7, CCR1<pac::TIM8>, PeripheralToMemory), //TIM8_CH1
    (Stream3<DMA2>, 7, CCR2<pac::TIM8>, MemoryToPeripheral), //TIM8_CH2
    (Stream3<DMA2>, 7, CCR2<pac::TIM8>, PeripheralToMemory), //TIM8_CH2
    (Stream4<DMA2>, 7, CCR3<pac::TIM8>, MemoryToPeripheral), //TIM8_CH3
    (Stream4<DMA2>, 7, CCR3<pac::TIM8>, PeripheralToMemory), //TIM8_CH3
    (Stream7<DMA2>, 7, CCR4<pac::TIM8>, MemoryToPeripheral), //TIM8_CH4
    (Stream7<DMA2>, 7, CCR4<pac::TIM8>, PeripheralToMemory), //TIM8_CH4
    (Stream7<DMA2>, 7, DMAR<pac::TIM8>, MemoryToPeripheral), //TIM8_COM/TRIG
    (Stream7<DMA2>, 7, DMAR<pac::TIM8>, PeripheralToMemory), //TIM8_COM/TRIG
    (Stream1<DMA1>, 4, pac::USART3, PeripheralToMemory),     //USART3_RX
    (Stream3<DMA1>, 4, pac::USART3, MemoryToPeripheral),     //USART3_TX
    (Stream4<DMA1>, 7, pac::USART3, MemoryToPeripheral),     //USART3_TX:DMA_CHANNEL_7
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
    (CCR1<pac::TIM8>, ccr1, u16),
    (CCR2<pac::TIM8>, ccr2, u16),
    (CCR3<pac::TIM8>, ccr3, u16),
    (CCR4<pac::TIM8>, ccr4, u16),
    (DMAR<pac::TIM8>, dmar, u16),
    (CCR3<pac::TIM2>, ccr3, u16),
    (DMAR<pac::TIM2>, dmar, u16),
    //(DMAR<pac::TIM7>, dmar), //Missing?
    (pac::USART3, dr, u8),
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
    (pac::DMA1, Stream1, 7, DMAR<pac::TIM6>, MemoryToPeripheral), //TIM6_UP
    (pac::DMA1, Stream1, 7, DMAR<pac::TIM6>, PeripheralToMemory), //TIM6_UP
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
    (DMAR<pac::TIM6>, dmar),
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
    (Stream2<DMA1>, 3, pac::I2C3, PeripheralToMemory), //I2C3_RX
    (Stream5<DMA2>, 2, pac::CRYP, PeripheralToMemory), //CRYP_OUT
    (Stream6<DMA2>, 2, pac::CRYP, MemoryToPeripheral), //CRYP_IN
    (Stream7<DMA2>, 2, pac::HASH, MemoryToPeripheral), //HASH_IN
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
address!((pac::HASH, din, u32), (pac::CRYP, din, u32),);

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
    (pac::DMA1, Stream5, 7, pac::DAC, MemoryToPeripheral), //DAC1
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
    (pac::DAC, ??),
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
    (Stream7<DMA1>, 4, pac::UART5, MemoryToPeripheral), //UART5_TX
    (Stream0<DMA2>, 2, pac::ADC3, PeripheralToMemory),  //ADC3
    (Stream1<DMA2>, 1, pac::DCMI, PeripheralToMemory),  //DCMI
    (Stream1<DMA2>, 2, pac::ADC3, PeripheralToMemory),  //ADC3
    (Stream2<DMA2>, 1, pac::ADC2, PeripheralToMemory),  //ADC2
    (Stream3<DMA2>, 1, pac::ADC2, PeripheralToMemory),  //ADC2
    (Stream7<DMA2>, 1, pac::DCMI, PeripheralToMemory),  //DCMI
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
    (pac::ADC2, dr, u16),
    (pac::ADC3, dr, u16),
    (pac::DCMI, dr, u32),
);

/* FMPI2C missing from peripheral crates (?)
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!(
    (pac::DMA1, Stream0, 7, pac::FMPI2C1, PeripheralToMemory), //FMPI2C1_RX
    (pac::DMA1, Stream1, 2, pac::FMPI2C1, MemoryToPeripheral), //FMPI2C1_TX
    (pac::DMA1, Stream3, 1, pac::FMPI2C1, PeripheralToMemory), //FMPI2C1_RX:DMA_CHANNEL_1
    (pac::DMA1, Stream7, 4, pac::FMPI2C1, MemoryToPeripheral), //FMPI2C1_TX:DMA_CHANNEL_4
);

// TODO: Probably need to create other type for tx_dr and rx_dr
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
address!(
    (pac::FMPI2C1, dr),
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
    (Stream1<DMA1>, 0, pac::I2C1, MemoryToPeripheral), //I2C1_TX
    (Stream6<DMA1>, 1, pac::I2C1, MemoryToPeripheral), //I2C1_TX:DMA_CHANNEL_1
    (Stream7<DMA1>, 1, pac::I2C1, MemoryToPeripheral), //I2C1_TX:DMA_CHANNEL_1
    (Stream7<DMA1>, 6, pac::USART2, PeripheralToMemory), //USART2_RX:DMA_CHANNEL_6
    (Stream2<DMA2>, 2, pac::SPI1, MemoryToPeripheral), //SPI1_TX
    (Stream3<DMA2>, 3, pac::SPI1, MemoryToPeripheral), //SPI1_TX:DMA_CHANNEL_3
    (Stream5<DMA2>, 3, pac::SPI1, MemoryToPeripheral), //SPI1_TX:DMA_CHANNEL_3
    (Stream5<DMA2>, 5, pac::SPI5, MemoryToPeripheral), //SPI5_TX:DMA_CHANNEL_5
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
    (Stream3<DMA2>, 2, pac::SPI5, PeripheralToMemory), //SPI5_RX
    (Stream4<DMA2>, 2, pac::SPI5, MemoryToPeripheral), //SPI5_TX
    (Stream5<DMA2>, 7, pac::SPI5, PeripheralToMemory), //SPI5_RX:DMA_CHANNEL_7
    (Stream6<DMA2>, 7, pac::SPI5, MemoryToPeripheral), //SPI5_TX:DMA_CHANNEL_7
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
address!((pac::SPI5, dr, u8),);

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
dma_map!((Stream4<DMA2>, 4, pac::SPI4, PeripheralToMemory),); //SPI4_RX);

/* TODO: DFSDM support
#[cfg(feature = "stm32f412")]
dma_map!(
    (Stream0<pac::DMA2>, 7, pac::DFSDM, PeripheralToMemory), //DFSDM1_FLT0
    (Stream1<pac::DMA2>, 3, pac::DFSDM, PeripheralToMemory), //DFSDM1_FLT1
    (Stream4<pac::DMA2>, 3, pac::DFSDM, PeripheralToMemory), //DFSDM1_FLT1
    (Stream6<pac::DMA2>, 3, pac::DFSDM, PeripheralToMemory), //DFSDM1_FLT0:DMA_CHANNEL_3
);
#[cfg(feature = "stm32f412")]
address!((pac::DFSDM, dr),);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
dma_map!(
    (Stream0<pac::DMA2>, 7, pac::DFSDM1, PeripheralToMemory), //DFSDM1_FLT0
    (Stream1<pac::DMA2>, 3, pac::DFSDM1, PeripheralToMemory), //DFSDM1_FLT1
    (Stream4<pac::DMA2>, 3, pac::DFSDM1, PeripheralToMemory), //DFSDM1_FLT1
    (Stream6<pac::DMA2>, 3, pac::DFSDM1, PeripheralToMemory), //DFSDM1_FLT0:DMA_CHANNEL_3
    (Stream0<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT0
    (Stream1<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT1
    (Stream2<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT2
    (Stream3<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT3
    (Stream4<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT0
    (Stream5<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT1
    (Stream6<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT2
    (Stream7<pac::DMA2>, 8, pac::DFSDM2, PeripheralToMemory), //DFSDM2_FLT3
);
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
address!((pac::DFSDM1, dr), (pac::DFSDM2, dr),);
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
    (Stream7<DMA2>, 3, pac::QUADSPI, MemoryToPeripheral), //QUADSPI
    (Stream7<DMA2>, 3, pac::QUADSPI, PeripheralToMemory), //QUADSPI
);

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!((pac::QUADSPI, dr, u32),);

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
    (Stream0<DMA1>, 5, pac::UART8, MemoryToPeripheral), //UART8_TX
    (Stream1<DMA1>, 5, pac::UART7, MemoryToPeripheral), //UART7_TX
    (Stream3<DMA1>, 5, pac::UART7, PeripheralToMemory), //UART7_RX
    (Stream6<DMA1>, 5, pac::UART8, PeripheralToMemory), //UART8_RX
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
address!((pac::UART7, dr, u8), (pac::UART8, dr, u8),);

#[cfg(any(feature = "stm32f413", feature = "stm32f423",))]
dma_map!(
    (Stream7<DMA1>, 8, pac::UART5, MemoryToPeripheral), //UART5_TX
    (Stream0<DMA2>, 1, pac::UART9, MemoryToPeripheral), //UART9_TX
    (Stream0<DMA2>, 5, pac::UART10, PeripheralToMemory), //UART10_RX
    (Stream3<DMA2>, 9, pac::UART10, PeripheralToMemory), //UART10_RX:DMA_CHANNEL_9
    (Stream5<DMA2>, 9, pac::UART10, MemoryToPeripheral), //UART10_TX
    (Stream7<DMA2>, 0, pac::UART9, PeripheralToMemory), //UART9_RX
    (Stream7<DMA2>, 6, pac::UART10, MemoryToPeripheral), //UART10_TX:DMA_CHANNEL_6
    //(pac::DMA2, Stream6, 2, IN<pac::AES>, MemoryToPeripheral), //AES_IN
    //(pac::DMA2, Stream5, 2, OUT<pac::AES>, PeripheralToMemory), //AES_OUT
);

#[cfg(any(feature = "stm32f413", feature = "stm32f423",))]
address!(
    //(IN<pac::AES>, dinr),
    //(OUT<pac::AES>, doutr),
    (pac::UART9, dr, u8),
    (pac::UART10, dr, u8),
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
    (pac::DMA2, Stream1, 0, pac::SAI, MemoryToPeripheral), //SAI1_A
    (pac::DMA2, Stream1, 0, pac::SAI, PeripheralToMemory), //SAI1_A
    (pac::DMA2, Stream3, 0, pac::SAI, MemoryToPeripheral), //SAI1_A
    (pac::DMA2, Stream3, 0, pac::SAI, PeripheralToMemory), //SAI1_A
    (pac::DMA2, Stream4, 1, pac::SAI, MemoryToPeripheral), //SAI1_B
    (pac::DMA2, Stream4, 1, pac::SAI, PeripheralToMemory), //SAI1_B
    (pac::DMA2, Stream5, 0, pac::SAI, MemoryToPeripheral), //SAI1_B:DMA_CHANNEL_0
    (pac::DMA2, Stream5, 0, pac::SAI, PeripheralToMemory), //SAI1_B:DMA_CHANNEL_0
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
    (pac::SAI, dr),
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
    (Stream5<DMA2>, 1, pac::SPI6, MemoryToPeripheral), //SPI6_TX
    (Stream6<DMA2>, 1, pac::SPI6, PeripheralToMemory), //SPI6_RX
);

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f439",
    feature = "stm32f437",
    feature = "stm32f429",
    feature = "stm32f469",
    feature = "stm32f479",
))]
address!((pac::SPI6, dr, u8),);

/*
#[cfg(any(
    feature = "stm32f446",
))]
dma_map!(
    (pac::DMA1, Stream1, 0, pac::SPDIFRX, PeripheralToMemory), //SPDIF_RX_DT
    (pac::DMA1, Stream2, 2, pac::FMPI2C1, PeripheralToMemory), //FMPI2C1_RX
    (pac::DMA1, Stream5, 2, pac::FMPI2C1, MemoryToPeripheral), //FMPI2C1_TX
    (pac::DMA1, Stream6, 0, pac::SPDIFRX, PeripheralToMemory), //SPDIF_RX_CS
    (pac::DMA2, Stream4, 3, pac::SAI2, MemoryToPeripheral), //SAI2_A
    (pac::DMA2, Stream4, 3, pac::SAI2, PeripheralToMemory), //SAI2_A
    (pac::DMA2, Stream6, 3, pac::SAI2, MemoryToPeripheral), //SAI2_B
    (pac::DMA2, Stream6, 3, pac::SAI2, PeripheralToMemory), //SAI2_B
    (pac::DMA2, Stream7, 0, pac::SAI2, MemoryToPeripheral), //SAI2_B:DMA_CHANNEL_0
    (pac::DMA2, Stream7, 0, pac::SAI2, PeripheralToMemory), //SAI2_B:DMA_CHANNEL_0
);
#[cfg(any(
    feature = "stm32f446",
))]
address!(
    (pac::SPDIFRX, ??),
    (pac::FMPI2C1, ??),
    (pac::SAI2, ??),
);
*/
