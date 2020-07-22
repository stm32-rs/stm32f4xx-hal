//! Direct Memory Access.

use core::{
    marker::PhantomData,
    mem,
    ops::Not,
    ptr,
    sync::atomic::{compiler_fence, Ordering},
};

pub mod buffer;
pub mod traits;
use buffer::WriteBuffer;
use traits::{
    sealed::{Bits, Sealed},
    Channel, Direction, Instance, PeriAddress, RccEnable, Stream,
};

/// Errors
#[derive(Debug, PartialEq)]
pub enum DMAError<T> {
    /// DMA not ready to change buffers
    NotReady(T),
    /// Double Buffering enabled buf second buffer not provided
    NoSecondBuffer,
    /// FIFO must be enabled if using memory to memory transfers
    FifoDisabled,
    /// Double buffer must not be enabled if using memory to memory transfers
    DoubleBufferEnabled,
    /// The user provided a buffer that is not big enough while double buffering
    SmallBuffer(T),
}

/// Possible DMA's directions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DmaDirection {
    /// Memory to Memory transfer
    MemoryToMemory,
    /// Peripheral to Memory transfer
    PeripheralToMemory,
    /// Memory to Peripheral transfer
    MemoryToPeripheral,
}

/// DMA from a peripheral to a memory location
#[derive(Debug, Clone, Copy)]
pub struct PeripheralToMemory;
impl Bits<u8> for PeripheralToMemory {
    #[inline]
    fn bits(self) -> u8 {
        0
    }
}
impl Direction for PeripheralToMemory {
    fn new() -> Self {
        PeripheralToMemory
    }
    #[inline]
    fn direction() -> DmaDirection {
        DmaDirection::PeripheralToMemory
    }
}

/// DMA from one memory location to another memory location
#[derive(Debug, Clone, Copy)]
pub struct MemoryToMemory<T> {
    _data: PhantomData<T>,
}

impl<T> Bits<u8> for MemoryToMemory<T> {
    #[inline]
    fn bits(self) -> u8 {
        2
    }
}

impl<T> Direction for MemoryToMemory<T> {
    fn new() -> Self {
        Self { _data: PhantomData }
    }
    #[inline]
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToMemory
    }
}

/// DMA from a memory location to a peripheral
#[derive(Debug, Clone, Copy)]
pub struct MemoryToPeripheral;
impl Bits<u8> for MemoryToPeripheral {
    #[inline]
    fn bits(self) -> u8 {
        1
    }
}
impl Direction for MemoryToPeripheral {
    fn new() -> Self {
        MemoryToPeripheral
    }
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToPeripheral
    }
}

impl Sealed for MemoryToMemory<u8> {}
impl Sealed for MemoryToMemory<u16> {}
impl Sealed for MemoryToMemory<u32> {}

impl PeriAddress for MemoryToMemory<u8> {
    fn address(&self) -> u32 {
        unimplemented!()
    }
    type MemSize = u8;
}

impl PeriAddress for MemoryToMemory<u16> {
    fn address(&self) -> u32 {
        unimplemented!()
    }
    type MemSize = u16;
}

impl PeriAddress for MemoryToMemory<u32> {
    fn address(&self) -> u32 {
        unimplemented!()
    }
    type MemSize = u32;
}

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
    /// Invalid Value
    Invalid,
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
            _ => FifoLevel::Invalid,
        }
    }
}

/// Which DMA buffer is in use
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentBuffer {
    /// The first buffer (m0ar) is in use
    FirstBuffer,
    /// The second buffer (m1ar) is in use
    DoubleBuffer,
}

impl Not for CurrentBuffer {
    type Output = CurrentBuffer;

    fn not(self) -> Self::Output {
        if self == CurrentBuffer::FirstBuffer {
            CurrentBuffer::DoubleBuffer
        } else {
            CurrentBuffer::FirstBuffer
        }
    }
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

/// Alias for a tuple with all DMA streams.
pub struct StreamsTuple<T>(
    pub Stream0<T>,
    pub Stream1<T>,
    pub Stream2<T>,
    pub Stream3<T>,
    pub Stream4<T>,
    pub Stream5<T>,
    pub Stream6<T>,
    pub Stream7<T>,
);

impl<T: RccEnable> StreamsTuple<T> {
    pub fn new(regs: T) -> Self {
        regs.rcc_enable();
        Self(
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
        $feif:ident, $isr:ident, $tcisr:ident, $htisr:ident)),+ $(,)*) => {
        $(
            #[allow(dead_code)]
            impl<I: Instance> Stream for $name<I> {

                const NUMBER: usize = $number;

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
                fn get_transfer_complete_flag() -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$tcisr().bit_is_set()
                }

                #[inline]
                fn get_half_transfer_flag() -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$htisr().bit_is_set()
                }

                #[inline]
                fn set_peripheral_address(&mut self, value: u32) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].par.write(|w| w.pa().bits(value));
                }

                #[inline]
                fn set_memory_address(&mut self, value: u32) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].m0ar.write(|w| w.m0a().bits(value));
                }

                #[inline]
                fn set_memory_double_buffer_address(&mut self, value: u32) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].m1ar.write(|w| w.m1a().bits(value));
                }

                #[inline]
                fn set_number_of_transfers(&mut self, value: u16) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].ndtr.write(|w| w.ndt().bits(value));
                }

                #[inline]
                unsafe fn enable(&mut self) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = &*I::ptr();
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.en().set_bit());
                }

                #[inline]
                fn is_enabled() -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.read().en().bit_is_set()
                }

                #[inline]
                fn disable(&mut self) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.en().clear_bit());
                }

                #[inline]
                fn set_channel<C: Channel>(&mut self, channel: C) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    //Some device crates have this field unsafe, others don't.
                    #[allow(unused_unsafe)]
                    dma.st[Self::NUMBER].cr.modify(|_, w| unsafe { w.chsel().bits(channel.bits()) });
                }

                #[inline]
                fn set_priority(&mut self, priority: config::Priority) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.pl().bits(priority.bits()));
                }

                #[inline]
                unsafe fn set_memory_size(&mut self, size: u8) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = &*I::ptr();
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.msize().bits(size));
                }

                #[inline]
                unsafe fn set_peripheral_size(&mut self, size: u8) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = &*I::ptr();
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.psize().bits(size));
                }

                #[inline]
                fn set_memory_increment(&mut self, increment: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.minc().bit(increment));
                }

                #[inline]
                fn set_peripheral_increment(&mut self, increment: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.pinc().bit(increment));
                }

                #[inline]
                fn set_direction<D: Direction>(&mut self, direction: D) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| unsafe { w.dir().bits(direction.bits()) });
                }

                #[inline]
                fn set_interrupts_enable(
                    &mut self,
                    transfer_complete: bool,
                    half_transfer: bool,
                    transfer_error: bool,
                    direct_mode_error: bool,
                )
                {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w
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
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.tcie().bit(transfer_complete_interrupt));
                }

                #[inline]
                fn set_half_transfer_interrupt_enable(&mut self, half_transfer_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.htie().bit(half_transfer_interrupt));
                }

                #[inline]
                fn set_transfer_error_interrupt_enable(&mut self, transfer_error_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.teie().bit(transfer_error_interrupt));
                }

                #[inline]
                fn set_direct_mode_error_interrupt_enable(&mut self, direct_mode_error_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.dmeie().bit(direct_mode_error_interrupt));
                }

                #[inline]
                fn set_fifo_error_interrupt_enable(&mut self, fifo_error_interrupt: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].fcr.modify(|_, w| w.feie().bit(fifo_error_interrupt));
                }

                #[inline]
                fn set_double_buffer(&mut self, double_buffer: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.dbm().bit(double_buffer));
                }

                #[inline]
                fn set_fifo_threshold(&mut self, fifo_threshold: config::FifoThreshold) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].fcr.modify(|_, w| w.fth().bits(fifo_threshold.bits()));
                }

                #[inline]
                fn set_fifo_enable(&mut self, fifo_enable: bool) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    //Register is actually direct mode disable rather than fifo enable
                    dma.st[Self::NUMBER].fcr.modify(|_, w| w.dmdis().bit(fifo_enable));
                }

                #[inline]
                fn set_memory_burst(&mut self, memory_burst: config::BurstMode) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.mburst().bits(memory_burst.bits()));
                }

                #[inline]
                fn set_peripheral_burst(&mut self, peripheral_burst: config::BurstMode) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.pburst().bits(peripheral_burst.bits()));
                }

                #[inline]
                fn fifo_level() -> FifoLevel {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].fcr.read().fs().bits().into()
                }

                #[inline]
                fn set_flow_controller(&mut self, flow_controller: config::FlowController) {
                    //NOTE(unsafe) We only access the registers that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.st[Self::NUMBER].cr.modify(|_, w| w.pfctrl().bit(flow_controller.bits()));
                }

                fn current_buffer() -> CurrentBuffer {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    if dma.st[Self::NUMBER].cr.read().ct().bit_is_set() {
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
    (Stream0, 0, lifcr, ctcif0, chtif0, cteif0, cdmeif0, cfeif0, lisr, tcif0, htif0),
    (Stream1, 1, lifcr, ctcif1, chtif1, cteif1, cdmeif1, cfeif1, lisr, tcif1, htif1),
    (Stream2, 2, lifcr, ctcif2, chtif2, cteif2, cdmeif2, cfeif2, lisr, tcif2, htif2),
    (Stream3, 3, lifcr, ctcif3, chtif3, cteif3, cdmeif3, cfeif3, lisr, tcif3, htif3),
    (Stream4, 4, hifcr, ctcif4, chtif4, cteif4, cdmeif4, cfeif4, hisr, tcif4, htif4),
    (Stream5, 5, hifcr, ctcif5, chtif5, cteif5, cdmeif5, cfeif5, hisr, tcif5, htif5),
    (Stream6, 6, hifcr, ctcif6, chtif6, cteif6, cdmeif6, cfeif6, hisr, tcif6, htif6),
    (Stream7, 7, hifcr, ctcif7, chtif7, cteif7, cdmeif7, cfeif7, hisr, tcif7, htif7),
);

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
            impl Channel for $name {
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
        pub(crate) priority: Priority,
        pub(crate) memory_increment: bool,
        pub(crate) peripheral_increment: bool,
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
                priority: Priority::Medium,
                memory_increment: false,
                peripheral_increment: false,
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
pub struct Transfer<STREAM, CHANNEL, PERIPHERAL, DIRECTION, BUF>
where
    STREAM: Stream,
    PERIPHERAL: PeriAddress,
    BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize> + 'static,
{
    stream: STREAM,
    _channel: PhantomData<CHANNEL>,
    peripheral: PERIPHERAL,
    _direction: PhantomData<DIRECTION>,
    buf: Option<BUF>,
    double_buf: Option<BUF>,
    // Used when double buffering
    transfer_length: u16,
}

impl<STREAM, CHANNEL, PERIPHERAL, DIR, BUF> Transfer<STREAM, CHANNEL, PERIPHERAL, DIR, BUF>
where
    STREAM: Stream,
    CHANNEL: Channel,
    DIR: Direction,
    PERIPHERAL: PeriAddress,
    BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize> + 'static,
    (STREAM, CHANNEL, PERIPHERAL, DIR): Sealed,
{
    /// Applies all fields in DmaConfig
    fn apply_config(&mut self, config: config::DmaConfig) {
        let msize = mem::align_of::<<PERIPHERAL as PeriAddress>::MemSize>() / 4;

        self.stream.clear_interrupts();
        self.stream.set_priority(config.priority);
        // NOTE(unsafe) These values are correct because of the invariants of PeriAddress
        unsafe {
            self.stream.set_memory_size(msize as u8);
            self.stream.set_peripheral_size(msize as u8);
        }
        self.stream.set_memory_increment(config.memory_increment);
        self.stream
            .set_peripheral_increment(config.peripheral_increment);
        self.stream
            .set_transfer_complete_interrupt_enable(config.transfer_complete_interrupt);
        self.stream
            .set_half_transfer_interrupt_enable(config.half_transfer_interrupt);
        self.stream
            .set_transfer_error_interrupt_enable(config.transfer_error_interrupt);
        self.stream
            .set_direct_mode_error_interrupt_enable(config.direct_mode_error_interrupt);
        self.stream
            .set_fifo_error_interrupt_enable(config.fifo_error_interrupt);
        self.stream.set_double_buffer(config.double_buffer);
        self.stream.set_fifo_threshold(config.fifo_threshold);
        self.stream.set_fifo_enable(config.fifo_enable);
        self.stream.set_memory_burst(config.memory_burst);
        self.stream.set_peripheral_burst(config.peripheral_burst);
        self.stream.set_flow_controller(config.flow_controller);
    }

    /// Configures DMA stream to correct channel for peripheral, configures source and destination
    /// addresses and applies supplied configuration. In a memory to memory transfer the `double_buf`
    /// argument is source of the data.
    pub fn init(
        mut stream: STREAM,
        peripheral: PERIPHERAL,
        mut memory: BUF,
        mut double_buf: Option<BUF>,
        config: config::DmaConfig,
    ) -> Result<Self, DMAError<BUF>> {
        stream.disable();

        // Set the channel
        stream.set_channel(CHANNEL::new());

        // Set peripheral to memory mode
        stream.set_direction(DIR::new());

        let (buf_ptr, buf_len) = unsafe { memory.write_buffer() };
        // Set the memory address
        stream.set_memory_address(buf_ptr as u32);

        let is_mem2mem = DIR::direction() == DmaDirection::MemoryToMemory;
        if is_mem2mem {
            // Fifo must be enabled for memory to memory
            if !config.fifo_enable {
                return Err(DMAError::FifoDisabled);
            } else if config.double_buffer {
                return Err(DMAError::DoubleBufferEnabled);
            }
        } else {
            // Set the peripheral address
            stream.set_peripheral_address(peripheral.address());
        }

        let db_len = if let Some(ref mut db) = double_buf {
            let (db_ptr, db_len) = unsafe { db.write_buffer() };
            if is_mem2mem {
                // Double buffer is the source in mem2mem mode
                stream.set_peripheral_address(db_ptr as u32);
            } else {
                stream.set_memory_double_buffer_address(db_ptr as u32);
            }
            Some(db_len)
        } else {
            // Double buffer mode must not be enabled if we haven't been given a second buffer
            if config.double_buffer {
                return Err(DMAError::NoSecondBuffer);
            }
            None
        };

        let n_transfers = if let Some(db) = db_len {
            buf_len.min(db) as u16
        } else {
            buf_len as u16
        };
        stream.set_number_of_transfers(n_transfers);

        let mut transfer = Self {
            stream,
            _channel: PhantomData,
            peripheral,
            _direction: PhantomData,
            buf: Some(memory),
            double_buf,
            transfer_length: n_transfers,
        };
        transfer.apply_config(config);

        Ok(transfer)
    }

    /// Starts the transfer, the closure will be executed right after enabling the stream.
    pub fn start<F>(&mut self, f: F)
    where
        F: FnOnce(&mut PERIPHERAL),
    {
        // "Preceding reads and writes cannot be moved past subsequent writes"
        compiler_fence(Ordering::Release);

        unsafe {
            self.stream.enable();
        }
        f(&mut self.peripheral);
    }

    /// Pauses the dma stream, the closure will be executed right before disabling the stream.
    pub fn pause<F>(&mut self, f: F)
    where
        F: FnOnce(&mut PERIPHERAL),
    {
        f(&mut self.peripheral);
        self.stream.disable()
    }

    /// Changes the buffer and restarts or continues a double buffer transfer. This must be called
    /// immediately after a transfer complete event. Returns the old buffer together with its
    /// `CurrentBuffer`, if an error occurs, this methods will return the new buffer with the error.
    pub fn next_transfer(
        &mut self,
        mut new_buf: BUF,
    ) -> Result<(BUF, CurrentBuffer), DMAError<BUF>> {
        if !STREAM::get_transfer_complete_flag() {
            return Err(DMAError::NotReady(new_buf));
        }
        self.stream.clear_transfer_complete_interrupt();

        if self.double_buf.is_some() && DIR::direction() != DmaDirection::MemoryToMemory {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on `new` until
            // the end of the DMA transfer
            let (new_buf_ptr, new_buf_len) = unsafe { new_buf.write_buffer() };

            // We can't change the transfer length while double buffering
            if new_buf_len < usize::from(self.transfer_length) {
                return Err(DMAError::SmallBuffer(new_buf));
            }

            if STREAM::current_buffer() == CurrentBuffer::DoubleBuffer {
                self.stream.set_memory_address(new_buf_ptr as u32);
                let old_buf = self.buf.replace(new_buf);

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                // We always have a buffer, so unwrap can't fail
                return Ok((old_buf.unwrap(), CurrentBuffer::FirstBuffer));
            } else {
                self.stream
                    .set_memory_double_buffer_address(new_buf_ptr as u32);
                let old_buf = self.double_buf.replace(new_buf);

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                // double buffering, unwrap can never fail
                return Ok((old_buf.unwrap(), CurrentBuffer::DoubleBuffer));
            }
        }
        self.stream.disable();

        // "No re-ordering of reads and writes across this point is allowed"
        compiler_fence(Ordering::SeqCst);

        // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on `new` until
        // the end of the DMA transfer
        let (buf_ptr, buf_len) = unsafe { new_buf.write_buffer() };
        self.stream.set_memory_address(buf_ptr as u32);
        self.stream.set_number_of_transfers(buf_len as u16);
        let old_buf = self.buf.replace(new_buf);

        // "Preceding reads and writes cannot be moved past subsequent writes"
        compiler_fence(Ordering::Release);

        unsafe {
            self.stream.enable();
        }

        Ok((old_buf.unwrap(), CurrentBuffer::FirstBuffer))
    }

    /// Stops the stream and returns the underlying Resources
    pub fn free(mut self) -> (STREAM, PERIPHERAL, BUF, Option<BUF>) {
        self.stream.disable();
        compiler_fence(Ordering::SeqCst);
        self.stream.clear_interrupts();

        unsafe {
            let stream = ptr::read(&self.stream);
            let peripheral = ptr::read(&self.peripheral);
            let buf = ptr::read(&self.buf);
            let double_buf = ptr::read(&self.double_buf);
            mem::forget(self);
            (stream, peripheral, buf.unwrap(), double_buf)
        }
    }

    /// Clear all interrupts for the DMA stream
    #[inline]
    pub fn clear_interrupts(&mut self) {
        self.stream.clear_interrupts();
    }

    /// Clear transfer complete interrupt (tcif) for the DMA stream
    #[inline]
    pub fn clear_transfer_complete_interrupt(&mut self) {
        self.stream.clear_transfer_complete_interrupt();
    }

    /// Clear half transfer interrupt (htif) for the DMA stream
    #[inline]
    pub fn clear_half_transfer_interrupt(&mut self) {
        self.stream.clear_half_transfer_interrupt();
    }

    /// Clear transfer error interrupt (teif) for the DMA stream
    #[inline]
    pub fn clear_transfer_error_interrupt(&mut self) {
        self.stream.clear_transfer_error_interrupt();
    }

    /// Clear direct mode error interrupt (dmeif) for the DMA stream
    #[inline]
    pub fn clear_direct_mode_error_interrupt(&mut self) {
        self.stream.clear_direct_mode_error_interrupt();
    }

    /// Clear fifo error interrupt (feif) for the DMA stream
    #[inline]
    pub fn clear_fifo_error_interrupt(&mut self) {
        self.stream.clear_fifo_error_interrupt();
    }

    /// Changes the buffer and restarts or continues a double buffer transfer. This must be called
    /// immediately after a transfer complete event. The closure must return `(BUF, T)` where BUF
    /// is the new buffer to be used. The closure will not be called if the transfer is not
    /// completed.
    ///
    /// # Panics
    /// This method will panic when double buffering and one or both of the following conditions
    /// happen:
    ///
    /// * The new buffer's length is smaller than the one used in the `init` method.
    /// * The closure `f` takes too long to return and an buffer overrun occurs.
    ///
    /// # Safety
    /// Memory corruption might occur if the previous buffer, the one passed to the closure, gets
    /// dropped in the closure and an overrun occurs in double buffering mode.
    pub unsafe fn next_transfer_with<F, T>(&mut self, f: F) -> Result<T, DMAError<()>>
    where
        F: FnOnce(BUF, CurrentBuffer) -> (BUF, T),
    {
        if !STREAM::get_transfer_complete_flag() {
            return Err(DMAError::NotReady(()));
        }
        self.stream.clear_transfer_complete_interrupt();

        if self.double_buf.is_some() && DIR::direction() != DmaDirection::MemoryToMemory {
            let current_buffer = STREAM::current_buffer();
            // double buffering, unwrap can never fail
            let db = if current_buffer == CurrentBuffer::DoubleBuffer {
                self.buf.take().unwrap()
            } else {
                self.double_buf.take().unwrap()
            };
            let r = f(db, !current_buffer);
            let mut new_buf = r.0;
            let (new_buf_ptr, new_buf_len) = new_buf.write_buffer();

            // We can't change the transfer length while double buffering
            assert!(
                new_buf_len >= usize::from(self.transfer_length),
                "Second Buffer not big enough"
            );

            // If this is false, then RAM corruption might have occurred, there's nothing we can do
            // apart from panicking.
            // TODO: Is this the best solution ? The closure based approach seems necessary if we
            // want to support BBqueue.
            assert!(current_buffer == STREAM::current_buffer(), "Overrun");

            if current_buffer == CurrentBuffer::DoubleBuffer {
                self.stream.set_memory_address(new_buf_ptr as u32);
                self.buf.replace(new_buf);

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                return Ok(r.1);
            } else {
                self.stream
                    .set_memory_double_buffer_address(new_buf_ptr as u32);
                self.double_buf.replace(new_buf);

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                return Ok(r.1);
            }
        }
        self.stream.disable();

        // "No re-ordering of reads and writes across this point is allowed"
        compiler_fence(Ordering::SeqCst);

        // Can never fail, we never let the Transfer without a buffer
        let old_buf = self.buf.take().unwrap();
        let r = f(old_buf, CurrentBuffer::FirstBuffer);
        let mut new_buf = r.0;

        let (buf_ptr, buf_len) = new_buf.write_buffer();
        self.stream.set_memory_address(buf_ptr as u32);
        self.stream.set_number_of_transfers(buf_len as u16);
        self.buf.replace(new_buf);

        // "Preceding reads and writes cannot be moved past subsequent writes"
        compiler_fence(Ordering::Release);

        self.stream.enable();

        Ok(r.1)
    }
}

impl<STREAM, CHANNEL, PERIPHERAL, DIR, BUF> Drop for Transfer<STREAM, CHANNEL, PERIPHERAL, DIR, BUF>
where
    STREAM: Stream,
    PERIPHERAL: PeriAddress,
    BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize> + 'static,
{
    fn drop(&mut self) {
        self.stream.disable();
    }
}
