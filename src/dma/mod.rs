//! Direct Memory Access.
//!
//! [Transfer::init](struct.Transfer.html#method.init) is only implemented for valid combinations of
//! peripheral-stream-channel-direction, providing compile time checking.
//!
//! This module implements Memory To Memory, Peripheral To Memory and Memory to Peripheral
//! transfers, double buffering is supported only for Peripheral To Memory and Memory to Peripheral
//! transfers.

use core::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    mem,
    ops::Not,
    ptr,
    sync::atomic::{compiler_fence, Ordering},
};
use embedded_dma::{ReadBuffer, WriteBuffer};

use crate::{pac, rcc};

pub mod traits;
use crate::serial::RxISR;
use traits::{
    sealed::Bits, Channel, DMASet, Direction, Instance, PeriAddress, SafePeripheralRead, Stream,
    StreamISR,
};

/// Errors.
#[derive(PartialEq, Eq)]
pub enum DMAError<T> {
    /// DMA not ready to change buffers.
    NotReady(T),
    /// The user provided a buffer that is not big enough while double buffering.
    SmallBuffer(T),
    /// Overrun during a double buffering or circular transfer.
    Overrun(T),
}

// Manually implement `Debug`, so we can have debug information even with a buffer `T` that doesn't
// implement `Debug`. `T` is always a buffer type chosen by the user, because of that the debug
// information can be helpful even without knowing the inner type
impl<T> Debug for DMAError<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DMAError::NotReady(_) => f.debug_tuple("NotReady").finish(),
            DMAError::SmallBuffer(_) => f.debug_tuple("SmallBuffer").finish(),
            DMAError::Overrun(_) => f.debug_tuple("Overrun").finish(),
        }
    }
}

// most of STM32F4 have 8 DmaChannel
#[cfg(not(feature = "gpio-f413"))]
/// Possible Channel of a DMA Stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaChannel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
}

//  STM32F413 and STM32F423 have 16 DmaChannel
#[cfg(feature = "gpio-f413")]
/// Possible Channel of a DMA Stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaChannel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
    Channel8 = 8,
    Channel9 = 9,
    Channel10 = 10,
    Channel11 = 11,
    Channel12 = 12,
    Channel13 = 13,
    Channel14 = 14,
    Channel15 = 15,
}

impl Bits<u8> for DmaChannel {
    fn bits(self) -> u8 {
        self as u8
    }
}

/// Peripheral increment offset size (pincos)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralIncrementOffset {
    /// The offset size is linked to the peripheral data size (psize)
    PeripheralDataSize = 0,
    /// The offset size is fixed to 4 (32 bits alignement)
    FixedSize = 1,
}

impl Bits<bool> for PeripheralIncrementOffset {
    fn bits(self) -> bool {
        match self {
            PeripheralIncrementOffset::PeripheralDataSize => false,
            PeripheralIncrementOffset::FixedSize => true,
        }
    }
}

/// Size of data transfered during a dma stream request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDataSize {
    Byte = 0,
    HalfWord = 1,
    Word = 2,
}

impl Bits<u8> for DmaDataSize {
    fn bits(self) -> u8 {
        self as u8
    }
}

/// Possible DMA's directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    /// Memory to Memory transfer.
    MemoryToMemory = 2,
    /// Peripheral to Memory transfer.
    PeripheralToMemory = 0,
    /// Memory to Peripheral transfer.
    MemoryToPeripheral = 1,
}

impl Bits<u8> for DmaDirection {
    fn bits(self) -> u8 {
        self as u8
    }
}

/// Dma flow controller selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaFlowController {
    Dma = 0,
    Peripheral = 1,
}

impl Bits<bool> for DmaFlowController {
    fn bits(self) -> bool {
        match self {
            DmaFlowController::Dma => false,
            DmaFlowController::Peripheral => true,
        }
    }
}

/// DMA from a peripheral to a memory location.
#[derive(Debug, Clone, Copy)]
pub struct PeripheralToMemory;

impl Bits<u8> for PeripheralToMemory {
    #[inline(always)]
    fn bits(self) -> u8 {
        0
    }
}

impl Direction for PeripheralToMemory {
    fn new() -> Self {
        PeripheralToMemory
    }
    #[inline(always)]
    fn direction() -> DmaDirection {
        DmaDirection::PeripheralToMemory
    }
}

/// DMA from one memory location to another memory location.
#[derive(Debug, Clone, Copy)]
pub struct MemoryToMemory<T> {
    _data: PhantomData<T>,
}

impl<T> Bits<u8> for MemoryToMemory<T> {
    #[inline(always)]
    fn bits(self) -> u8 {
        2
    }
}

impl<T> Direction for MemoryToMemory<T> {
    fn new() -> Self {
        Self { _data: PhantomData }
    }
    #[inline(always)]
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToMemory
    }
}

/// DMA from a memory location to a peripheral.
#[derive(Debug, Clone, Copy)]
pub struct MemoryToPeripheral;

impl Bits<u8> for MemoryToPeripheral {
    #[inline(always)]
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

unsafe impl PeriAddress for MemoryToMemory<u8> {
    fn address(&self) -> u32 {
        unimplemented!()
    }
    type MemSize = u8;
}

unsafe impl PeriAddress for MemoryToMemory<u16> {
    fn address(&self) -> u32 {
        unimplemented!()
    }
    type MemSize = u16;
}

unsafe impl PeriAddress for MemoryToMemory<u32> {
    fn address(&self) -> u32 {
        unimplemented!()
    }
    type MemSize = u32;
}

/// How full the DMA stream's fifo is.
#[derive(Debug, Clone, Copy)]
pub enum FifoLevel {
    /// 0 < fifo_level < 1/4.
    GtZeroLtQuarter,
    /// 1/4 <= fifo_level < 1/2.
    GteQuarterLtHalf,
    /// 1/2 <= fifo_level < 3/4.
    GteHalfLtThreeQuarter,
    /// 3/4 <= fifo_level < full.
    GteThreeQuarterLtFull,
    /// Fifo is empty.
    Empty,
    /// Fifo is full.
    Full,
    /// Invalid value.
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

/// Which DMA buffer is in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentBuffer {
    /// The first buffer (m0ar) is in use.
    FirstBuffer,
    /// The second buffer (m1ar) is in use.
    SecondBuffer,
}

impl Not for CurrentBuffer {
    type Output = CurrentBuffer;

    fn not(self) -> Self::Output {
        if self == CurrentBuffer::FirstBuffer {
            CurrentBuffer::SecondBuffer
        } else {
            CurrentBuffer::FirstBuffer
        }
    }
}

/// Structure representing setup of common interrupts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaCommonInterrupts {
    pub transfer_complete: bool,
    pub half_transfer: bool,
    pub transfer_error: bool,
    pub direct_mode_error: bool,
}

impl DmaCommonInterrupts {
    /// Return a new DmaCommonInterrupts with all fields set to true.
    const fn all() -> Self {
        Self {
            transfer_complete: true,
            half_transfer: true,
            transfer_error: true,
            direct_mode_error: true,
        }
    }
}

/// Structure returned by Stream or Transfer all_flags() method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaFlags {
    pub transfer_complete: bool,
    pub half_transfer: bool,
    pub transfer_error: bool,
    pub direct_mode_error: bool,
    pub fifo_error: bool,
}

impl DmaFlags {
    /// Return a new DmaFlags with all fields set to true.
    const fn all() -> Self {
        Self {
            transfer_complete: true,
            half_transfer: true,
            transfer_error: true,
            direct_mode_error: true,
            fifo_error: true,
        }
    }
}

/// Stream on the DMA controller.
pub struct StreamX<DMA, const S: u8> {
    _dma: PhantomData<DMA>,
}

impl<DMA, const S: u8> StreamX<DMA, S> {
    fn new() -> Self {
        Self { _dma: PhantomData }
    }
}

impl<DMA: Instance, const S: u8> StreamX<DMA, S> {
    #[cfg(not(any(feature = "gpio-f411", feature = "gpio-f413", feature = "gpio-f410")))]
    #[inline(always)]
    unsafe fn st() -> &'static pac::dma2::ST {
        &(*DMA::ptr()).st[S as usize]
    }
    #[cfg(any(feature = "gpio-f411", feature = "gpio-f413", feature = "gpio-f410"))]
    #[inline(always)]
    unsafe fn st() -> &'static pac::dma1::ST {
        &(*DMA::ptr()).st[S as usize]
    }
}

/// Stream 0 on the DMA controller.
pub type Stream0<DMA> = StreamX<DMA, 0>;
/// Stream 1 on the DMA controller.
pub type Stream1<DMA> = StreamX<DMA, 1>;
/// Stream 2 on the DMA controller.
pub type Stream2<DMA> = StreamX<DMA, 2>;
/// Stream 3 on the DMA controller.
pub type Stream3<DMA> = StreamX<DMA, 3>;
/// Stream 4 on the DMA controller.
pub type Stream4<DMA> = StreamX<DMA, 4>;
/// Stream 5 on the DMA controller.
pub type Stream5<DMA> = StreamX<DMA, 5>;
/// Stream 6 on the DMA controller.
pub type Stream6<DMA> = StreamX<DMA, 6>;
/// Stream 7 on the DMA controller.
pub type Stream7<DMA> = StreamX<DMA, 7>;

impl<DMA> crate::Sealed for StreamX<DMA, 0> {}
impl<DMA> crate::Sealed for StreamX<DMA, 1> {}
impl<DMA> crate::Sealed for StreamX<DMA, 2> {}
impl<DMA> crate::Sealed for StreamX<DMA, 3> {}
impl<DMA> crate::Sealed for StreamX<DMA, 4> {}
impl<DMA> crate::Sealed for StreamX<DMA, 5> {}
impl<DMA> crate::Sealed for StreamX<DMA, 6> {}
impl<DMA> crate::Sealed for StreamX<DMA, 7> {}

/// Alias for a tuple with all DMA streams.
pub struct StreamsTuple<DMA>(
    pub StreamX<DMA, 0>,
    pub StreamX<DMA, 1>,
    pub StreamX<DMA, 2>,
    pub StreamX<DMA, 3>,
    pub StreamX<DMA, 4>,
    pub StreamX<DMA, 5>,
    pub StreamX<DMA, 6>,
    pub StreamX<DMA, 7>,
);

impl<DMA: rcc::Enable + rcc::Reset> StreamsTuple<DMA> {
    /// Splits the DMA peripheral into streams.
    pub fn new(_regs: DMA) -> Self {
        unsafe {
            DMA::enable_unchecked();
            DMA::reset_unchecked();
        }
        Self(
            StreamX::new(),
            StreamX::new(),
            StreamX::new(),
            StreamX::new(),
            StreamX::new(),
            StreamX::new(),
            StreamX::new(),
            StreamX::new(),
        )
    }
}

impl<I: Instance, const S: u8> Stream for StreamX<I, S>
where
    Self: crate::Sealed + StreamISR,
{
    const NUMBER: usize = S as usize;

    #[inline(always)]
    fn set_peripheral_address(&mut self, value: u32) {
        unsafe { Self::st() }
            .par
            .write(|w| unsafe { w.pa().bits(value) });
    }

    #[inline(always)]
    fn set_memory_address(&mut self, value: u32) {
        unsafe { Self::st() }
            .m0ar
            .write(|w| unsafe { w.m0a().bits(value) });
    }

    #[inline(always)]
    fn memory_address(&self) -> u32 {
        unsafe { Self::st() }.m0ar.read().m0a().bits()
    }

    #[inline(always)]
    fn set_alternate_memory_address(&mut self, value: u32) {
        unsafe { Self::st() }
            .m1ar
            .write(|w| unsafe { w.m1a().bits(value) });
    }

    #[inline(always)]
    fn alternate_memory_address(&self) -> u32 {
        unsafe { Self::st() }.m1ar.read().m1a().bits()
    }

    #[inline(always)]
    fn set_number_of_transfers(&mut self, value: u16) {
        unsafe { Self::st() }.ndtr.write(|w| w.ndt().bits(value));
    }

    #[inline(always)]
    fn number_of_transfers(&self) -> u16 {
        unsafe { Self::st() }.ndtr.read().ndt().bits()
    }

    #[inline(always)]
    unsafe fn enable(&mut self) {
        Self::st().cr.modify(|_, w| w.en().set_bit());
    }

    #[inline(always)]
    fn is_enabled(&self) -> bool {
        unsafe { Self::st() }.cr.read().en().bit_is_set()
    }

    #[inline(always)]
    unsafe fn disable(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.en().clear_bit());
    }

    #[inline(always)]
    fn set_channel(&mut self, channel: DmaChannel) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.chsel().bits(channel.bits()));
    }

    #[inline(always)]
    fn set_priority(&mut self, priority: config::Priority) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.pl().bits(priority.bits()));
    }

    #[inline(always)]
    fn set_peripheral_increment_offset(&mut self, value: PeripheralIncrementOffset) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.pincos().bit(value.bits()));
    }

    #[inline(always)]
    unsafe fn set_memory_size(&mut self, size: DmaDataSize) {
        Self::st().cr.modify(|_, w| w.msize().bits(size.bits()));
    }

    #[inline(always)]
    unsafe fn set_peripheral_size(&mut self, size: DmaDataSize) {
        Self::st().cr.modify(|_, w| w.psize().bits(size.bits()));
    }

    #[inline(always)]
    fn set_memory_increment(&mut self, increment: bool) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.minc().bit(increment));
    }

    #[inline(always)]
    fn set_peripheral_increment(&mut self, increment: bool) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.pinc().bit(increment));
    }

    #[inline(always)]
    fn set_circular_mode(&mut self, value: bool) {
        unsafe { Self::st() }.cr.modify(|_, w| w.circ().bit(value));
    }

    #[inline(always)]
    fn set_direction(&mut self, direction: DmaDirection) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| unsafe { w.dir().bits(direction.bits()) });
    }

    #[inline(always)]
    fn set_flow_controller(&mut self, value: DmaFlowController) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.pfctrl().bit(value.bits()));
    }

    #[inline(always)]
    fn listen(&mut self, interrupts: DmaCommonInterrupts) {
        let mask = (interrupts.transfer_complete as u32) << 4
            | (interrupts.half_transfer as u32) << 3
            | (interrupts.transfer_error as u32) << 2
            | (interrupts.direct_mode_error as u32) << 1;
        unsafe { Self::st() }
            .cr
            .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    }

    #[inline(always)]
    fn unlisten(&mut self, interrupts: DmaCommonInterrupts) {
        let mask = (interrupts.transfer_complete as u32) << 4
            | (interrupts.half_transfer as u32) << 3
            | (interrupts.transfer_error as u32) << 2
            | (interrupts.direct_mode_error as u32) << 1;
        unsafe { Self::st() }
            .cr
            .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
    }

    #[inline(always)]
    fn common_interrupts(&self) -> DmaCommonInterrupts {
        let cr = unsafe { Self::st() }.cr.read();
        DmaCommonInterrupts {
            transfer_complete: cr.tcie().bit_is_set(),
            half_transfer: cr.htie().bit_is_set(),
            transfer_error: cr.teie().bit_is_set(),
            direct_mode_error: cr.dmeie().bit_is_set(),
        }
    }

    #[inline(always)]
    fn listen_transfer_complete(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.tcie().bit(true));
    }

    #[inline(always)]
    fn unlisten_transfer_complete(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.tcie().bit(false));
    }

    #[inline(always)]
    fn listen_half_transfer(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.htie().bit(true));
    }

    #[inline(always)]
    fn unlisten_half_transfer(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.htie().bit(false));
    }

    #[inline(always)]
    fn listen_transfer_error(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.teie().bit(true));
    }

    #[inline(always)]
    fn unlisten_transfer_error(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.teie().bit(false));
    }

    #[inline(always)]
    fn listen_direct_mode_error(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.dmeie().bit(true));
    }

    #[inline(always)]
    fn unlisten_direct_mode_error(&mut self) {
        unsafe { Self::st() }.cr.modify(|_, w| w.dmeie().bit(false));
    }

    #[inline(always)]
    fn listen_fifo_error(&mut self) {
        unsafe { Self::st() }.fcr.modify(|_, w| w.feie().bit(true));
    }

    #[inline(always)]
    fn unlisten_fifo_error(&mut self) {
        unsafe { Self::st() }.fcr.modify(|_, w| w.feie().bit(false));
    }

    #[inline(always)]
    fn set_double_buffer(&mut self, double_buffer: bool) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.dbm().bit(double_buffer));
    }

    #[inline(always)]
    fn set_fifo_threshold(&mut self, fifo_threshold: config::FifoThreshold) {
        unsafe { Self::st() }
            .fcr
            .modify(|_, w| w.fth().bits(fifo_threshold.bits()));
    }

    #[inline(always)]
    fn set_fifo_enable(&mut self, fifo_enable: bool) {
        //Register is actually direct mode disable rather than fifo enable
        unsafe { Self::st() }
            .fcr
            .modify(|_, w| w.dmdis().bit(fifo_enable));
    }

    #[inline(always)]
    fn set_memory_burst(&mut self, memory_burst: config::BurstMode) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.mburst().bits(memory_burst.bits()));
    }

    #[inline(always)]
    fn set_peripheral_burst(&mut self, peripheral_burst: config::BurstMode) {
        unsafe { Self::st() }
            .cr
            .modify(|_, w| w.pburst().bits(peripheral_burst.bits()));
    }

    #[inline(always)]
    fn fifo_level(&self) -> FifoLevel {
        unsafe { Self::st() }.fcr.read().fs().bits().into()
    }

    fn current_buffer(&self) -> CurrentBuffer {
        if unsafe { Self::st() }.cr.read().ct().bit_is_set() {
            CurrentBuffer::SecondBuffer
        } else {
            CurrentBuffer::FirstBuffer
        }
    }
}

// Macro that creates a struct representing a stream on either DMA controller
// The implementation does the heavy lifting of mapping to the right fields on the stream
macro_rules! dma_stream {
    ($(($number:expr ,$ifcr:ident, $tcif:ident, $htif:ident, $teif:ident, $dmeif:ident,
        $feif:ident, $isr:ident, $tcisr:ident, $htisr:ident, $teisr:ident, $feisr:ident, $dmeisr:ident)),+
        $(,)*) => {
        $(
            impl<I: Instance> StreamISR for StreamX<I, $number> where Self: crate::Sealed {

                #[inline(always)]
                fn clear_flags(&mut self, flag: DmaFlags) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w
                        .$tcif().bit(flag.transfer_complete) //Clear transfer complete interrupt flag
                        .$htif().bit(flag.half_transfer) //Clear half transfer interrupt flag
                        .$teif().bit(flag.transfer_error) //Clear transfer error interrupt flag
                        .$dmeif().bit(flag.direct_mode_error) //Clear direct mode error interrupt flag
                        .$feif().bit(flag.fifo_error) //Clear fifo error interrupt flag
                    );
                }

                #[inline(always)]
                fn clear_transfer_complete(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$tcif().set_bit());
                }

                #[inline(always)]
                fn clear_half_transfer(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$htif().set_bit());
                }

                #[inline(always)]
                fn clear_transfer_error(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$teif().set_bit());
                }

                #[inline(always)]
                fn clear_direct_mode_error(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$dmeif().set_bit());
                }

                #[inline(always)]
                fn clear_fifo_error(&mut self) {
                    //NOTE(unsafe) Atomic write with no side-effects and we only access the bits
                    // that belongs to the StreamX
                    let dma = unsafe { &*I::ptr() };
                    dma.$ifcr.write(|w| w.$feif().set_bit());
                }

                #[inline(always)]
                fn all_flags(&self) -> DmaFlags
                {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    DmaFlags{
                        transfer_complete: dma.$isr.read().$tcisr().bit_is_set(),
                        half_transfer: dma.$isr.read().$htisr().bit_is_set(),
                        transfer_error: dma.$isr.read().$teisr().bit_is_set(),
                        direct_mode_error: dma.$isr.read().$dmeisr().bit_is_set(),
                        fifo_error: dma.$isr.read().$feisr().bit_is_set(),
                    }

                }

                #[inline(always)]
                fn is_transfer_complete(&self) -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$tcisr().bit_is_set()
                }

                #[inline(always)]
                fn is_half_transfer(&self) -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$htisr().bit_is_set()
                }

                #[inline(always)]
                fn is_transfer_error(&self) -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$teisr().bit_is_set()
                }

                #[inline(always)]
                fn is_direct_mode_error(&self) -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$dmeisr().bit_is_set()
                }

                #[inline(always)]
                fn is_fifo_error(&self) -> bool {
                    //NOTE(unsafe) Atomic read with no side effects
                    let dma = unsafe { &*I::ptr() };
                    dma.$isr.read().$feisr().bit_is_set()
                }

            }

        )+
    };
}

dma_stream!(
    (0, lifcr, ctcif0, chtif0, cteif0, cdmeif0, cfeif0, lisr, tcif0, htif0, teif0, feif0, dmeif0),
    (1, lifcr, ctcif1, chtif1, cteif1, cdmeif1, cfeif1, lisr, tcif1, htif1, teif1, feif1, dmeif1),
    (2, lifcr, ctcif2, chtif2, cteif2, cdmeif2, cfeif2, lisr, tcif2, htif2, teif2, feif2, dmeif2),
    (3, lifcr, ctcif3, chtif3, cteif3, cdmeif3, cfeif3, lisr, tcif3, htif3, teif3, feif3, dmeif3),
    (4, hifcr, ctcif4, chtif4, cteif4, cdmeif4, cfeif4, hisr, tcif4, htif4, teif4, feif4, dmeif4),
    (5, hifcr, ctcif5, chtif5, cteif5, cdmeif5, cfeif5, hisr, tcif5, htif5, teif5, feif5, dmeif5),
    (6, hifcr, ctcif6, chtif6, cteif6, cdmeif6, cfeif6, hisr, tcif6, htif6, teif6, feif6, dmeif6),
    (7, hifcr, ctcif7, chtif7, cteif7, cdmeif7, cfeif7, hisr, tcif7, htif7, teif7, feif7, dmeif7),
);

/// A Channel that can be configured on a DMA stream.
#[derive(Debug, Clone, Copy)]
pub struct ChannelX<const C: u8>;

macro_rules! dma_channel {
    ($(($name:ident, $num:literal)),+ $(,)*) => {
        $(
            impl Channel for ChannelX<$num> {
                const VALUE: DmaChannel = DmaChannel::$name ;
            }
            pub type $name = ChannelX<$num>;
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

#[cfg(feature = "gpio-f413")]
dma_channel!((Channel8, 8), (Channel9, 9),);

/// Contains types related to DMA configuration.
pub mod config {
    use super::Bits;

    /// Priority of the DMA stream, defaults to `Medium`. If two requests have the same software
    /// priority level, the stream with the lower number takes priority over the stream with the
    /// higher number. For example, Stream 2 takes priority over Stream 4.
    #[derive(Debug, Clone, Copy)]
    pub enum Priority {
        /// Low priority.
        Low,
        /// Medium priority.
        Medium,
        /// High priority.
        High,
        /// Very high priority.
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

    /// The level to fill the fifo to before performing the transaction.
    #[derive(Debug, Clone, Copy)]
    pub enum FifoThreshold {
        /// 1/4 full.
        QuarterFull,
        /// 1/2 full.
        HalfFull,
        /// 3/4 full.
        ThreeQuarterFull,
        /// Full.
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

    /// How burst transfers are done, requires fifo enabled. Check datasheet for valid combinations.
    #[derive(Debug, Clone, Copy)]
    pub enum BurstMode {
        /// Single transfer, no burst.
        NoBurst,
        /// Burst transfer of 4 beats.
        Burst4,
        /// Burst transfer of 8 beats.
        Burst8,
        /// Burst transfer of 16 beats.
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

    /// Contains the complete set of configuration for a DMA stream.
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
            }
        }
    }

    impl DmaConfig {
        /// Set the priority.
        #[inline(always)]
        pub fn priority(mut self, priority: Priority) -> Self {
            self.priority = priority;
            self
        }

        /// Set the memory_increment.
        #[inline(always)]
        pub fn memory_increment(mut self, memory_increment: bool) -> Self {
            self.memory_increment = memory_increment;
            self
        }
        /// Set the peripheral_increment.
        #[inline(always)]
        pub fn peripheral_increment(mut self, peripheral_increment: bool) -> Self {
            self.peripheral_increment = peripheral_increment;
            self
        }
        /// Set the transfer_complete_interrupt.
        #[inline(always)]
        pub fn transfer_complete_interrupt(mut self, transfer_complete_interrupt: bool) -> Self {
            self.transfer_complete_interrupt = transfer_complete_interrupt;
            self
        }
        /// Set the half_transfer_interrupt.
        #[inline(always)]
        pub fn half_transfer_interrupt(mut self, half_transfer_interrupt: bool) -> Self {
            self.half_transfer_interrupt = half_transfer_interrupt;
            self
        }
        /// Set the transfer_error_interrupt.
        #[inline(always)]
        pub fn transfer_error_interrupt(mut self, transfer_error_interrupt: bool) -> Self {
            self.transfer_error_interrupt = transfer_error_interrupt;
            self
        }
        /// Set the direct_mode_error_interrupt.
        #[inline(always)]
        pub fn direct_mode_error_interrupt(mut self, direct_mode_error_interrupt: bool) -> Self {
            self.direct_mode_error_interrupt = direct_mode_error_interrupt;
            self
        }
        /// Set the fifo_error_interrupt.
        #[inline(always)]
        pub fn fifo_error_interrupt(mut self, fifo_error_interrupt: bool) -> Self {
            self.fifo_error_interrupt = fifo_error_interrupt;
            self
        }
        /// Set the double_buffer.
        #[inline(always)]
        pub fn double_buffer(mut self, double_buffer: bool) -> Self {
            self.double_buffer = double_buffer;
            self
        }
        /// Set the fifo_threshold.
        #[inline(always)]
        pub fn fifo_threshold(mut self, fifo_threshold: FifoThreshold) -> Self {
            self.fifo_threshold = fifo_threshold;
            self
        }
        /// Set the fifo_enable.
        #[inline(always)]
        pub fn fifo_enable(mut self, fifo_enable: bool) -> Self {
            self.fifo_enable = fifo_enable;
            self
        }
        /// Set the memory_burst.
        #[inline(always)]
        pub fn memory_burst(mut self, memory_burst: BurstMode) -> Self {
            self.memory_burst = memory_burst;
            self
        }
        /// Set the peripheral_burst.
        #[inline(always)]
        pub fn peripheral_burst(mut self, peripheral_burst: BurstMode) -> Self {
            self.peripheral_burst = peripheral_burst;
            self
        }
    }
}

/// DMA Transfer.
pub struct Transfer<STREAM, const CHANNEL: u8, PERIPHERAL, DIRECTION, BUF>
where
    STREAM: Stream,
    PERIPHERAL: PeriAddress,
{
    stream: STREAM,
    peripheral: PERIPHERAL,
    _direction: PhantomData<DIRECTION>,
    buf: Option<BUF>,
    double_buf: Option<BUF>,
    // Used when double buffering
    transfer_length: u16,
}

// utility function to disable gracefully the stream. It disable stream, wait until stream is
// disabled and prevent during process
fn stream_disable<T: Stream>(stream: &mut T) {
    if stream.is_enabled() {
        // Aborting an on-going transfer might cause interrupts to fire, disable
        let interrupts = stream.common_interrupts();
        stream.unlisten(DmaCommonInterrupts::all());
        unsafe { stream.disable() };
        while stream.is_enabled() {}

        stream.clear_flags(DmaFlags::all());
        stream.listen(interrupts);
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, BUF>
    Transfer<STREAM, CHANNEL, PERIPHERAL, MemoryToPeripheral, BUF>
where
    STREAM: Stream,
    ChannelX<CHANNEL>: Channel,
    PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, MemoryToPeripheral>,
    BUF: ReadBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize>,
{
    /// Configures the DMA stream to the correct channel for the peripheral, configures source and
    /// destination and applies supplied configuration. If double buffering is enabled, the
    /// number of transfers will be the minimum length of `memory` and `double_buf`.
    ///
    /// # Panics
    ///
    /// * When double buffering is enabled but the `double_buf` argument is `None`.
    pub fn init_memory_to_peripheral(
        mut stream: STREAM,
        peripheral: PERIPHERAL,
        buf: BUF,
        double_buf: Option<BUF>,
        config: config::DmaConfig,
    ) -> Self {
        let first_buf = {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on it until the
            // end of the DMA transfer
            let (buf_ptr, buf_len) = unsafe { buf.read_buffer() };
            (buf_ptr as u32, buf_len as u16)
        };

        let db = double_buf.as_ref().map(|db| {
            let (db_ptr, db_len) = unsafe { db.read_buffer() };
            (db_ptr as u32, db_len as u16)
        });
        let n_transfers = Self::init_common(&mut stream, &peripheral, config, first_buf, db);

        Self {
            stream,
            peripheral,
            _direction: PhantomData,
            buf: Some(buf),
            double_buf,
            transfer_length: n_transfers,
        }
    }

    /// Changes the buffer and restarts or continues a double buffer transfer. This must be called
    /// immediately after a transfer complete event if using double buffering, otherwise you might
    /// lose data. Returns the old buffer together with its `CurrentBuffer`. If an error occurs,
    /// this method will return the new buffer with the error.
    ///
    /// This method will clear the transfer complete flag on entry, it will also clear it again if
    /// an overrun occurs during its execution. Moreover, if an overrun occurs, the stream will be
    /// disabled and the transfer error flag will be set. This method can be called before the end
    /// of an ongoing transfer only if not using double buffering, in that case, the current
    /// transfer will be canceled and a new one will be started. A `NotReady` error will be returned
    /// if this method is called before the end of a transfer while double buffering.
    pub fn next_transfer(&mut self, new_buf: BUF) -> Result<(BUF, CurrentBuffer), DMAError<BUF>> {
        let ptr_and_len = {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on it until the
            // end of the DMA transfer
            let (buf_ptr, buf_len) = unsafe { new_buf.read_buffer() };
            (buf_ptr as u32, buf_len as u16)
        };
        self.next_transfer_common(new_buf, ptr_and_len, self.double_buf.is_some())
    }

    /// Changes the buffer and restarts or continues a double buffer transfer. This must be called
    /// immediately after a transfer complete event if using double buffering, otherwise you might
    /// lose data. The closure must return `(BUF, T)` where `BUF` is the new buffer to be used. This
    /// method can be called before the end of an ongoing transfer only if not using double
    /// buffering, in that case, the current transfer will be canceled and a new one will be
    /// started. A `NotReady` error will be returned if this method is called before the end of a
    /// transfer while double buffering and the closure won't be executed.
    ///
    /// # Panics
    /// This method will panic when double buffering if one or both of the following conditions
    /// happen:
    ///
    /// * The new buffer's length is smaller than the one used in the `init` method.
    /// * The closure `f` takes too long to return and a buffer overrun occurs.
    ///
    /// # Safety
    ///
    /// Memory corruption might occur in the previous buffer, the one passed to the closure, if an
    /// overrun occurs in double buffering mode.
    pub unsafe fn next_transfer_with<F, T>(&mut self, f: F) -> Result<T, DMAError<()>>
    where
        F: FnOnce(BUF, CurrentBuffer) -> (BUF, T),
    {
        if self.double_buf.is_some() {
            if !self.stream.is_transfer_complete() {
                return Err(DMAError::NotReady(()));
            }
            self.stream.clear_transfer_complete();

            let current_buffer = self.stream.current_buffer();
            // double buffering, unwrap can never fail
            let db = if current_buffer == CurrentBuffer::SecondBuffer {
                self.buf.take().unwrap()
            } else {
                self.double_buf.take().unwrap()
            };
            let r = f(db, !current_buffer);
            let new_buf = r.0;
            let ptr_and_len = {
                let (new_buf_ptr, new_buf_len) = new_buf.read_buffer();
                (new_buf_ptr as u32, new_buf_len as u16)
            };
            self.next_transfer_with_common(new_buf, ptr_and_len, true, current_buffer);
            return Ok(r.1);
        }
        stream_disable(&mut self.stream);
        self.stream.clear_transfer_complete();

        // "No re-ordering of reads and writes across this point is allowed"
        compiler_fence(Ordering::SeqCst);

        // Can never fail, we never let the Transfer without a buffer
        let old_buf = self.buf.take().unwrap();
        let r = f(old_buf, CurrentBuffer::FirstBuffer);
        let new_buf = r.0;

        let ptr_and_len = {
            let (new_buf_ptr, new_buf_len) = new_buf.read_buffer();
            (new_buf_ptr as u32, new_buf_len as u16)
        };
        self.next_transfer_with_common(new_buf, ptr_and_len, false, CurrentBuffer::FirstBuffer);
        Ok(r.1)
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, BUF> RxISR
    for Transfer<STREAM, CHANNEL, PERIPHERAL, PeripheralToMemory, BUF>
where
    STREAM: Stream,
    PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, PeripheralToMemory> + RxISR,
{
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool {
        self.peripheral.is_idle()
    }

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool {
        self.peripheral.is_rx_not_empty()
    }

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self) {
        self.peripheral.clear_idle_interrupt();
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, BUF>
    Transfer<STREAM, CHANNEL, PERIPHERAL, PeripheralToMemory, BUF>
where
    STREAM: Stream,
    ChannelX<CHANNEL>: Channel,
    PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, PeripheralToMemory> + SafePeripheralRead,
    BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize>,
{
    /// Access the owned peripheral for reading
    pub fn peripheral(&self) -> &PERIPHERAL {
        &self.peripheral
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, BUF>
    Transfer<STREAM, CHANNEL, PERIPHERAL, PeripheralToMemory, BUF>
where
    STREAM: Stream,
    ChannelX<CHANNEL>: Channel,
    PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, PeripheralToMemory>,
    BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize>,
{
    /// Configures the DMA stream to the correct channel for the peripheral, configures source and
    /// destination and applies supplied configuration. If double buffering is enabled, the
    /// number of transfers will be the minimum length of `memory` and `double_buf`.
    ///
    /// # Panics
    ///
    /// * When double buffering is enabled but the `double_buf` argument is `None`.
    pub fn init_peripheral_to_memory(
        mut stream: STREAM,
        peripheral: PERIPHERAL,
        mut buf: BUF,
        mut double_buf: Option<BUF>,
        config: config::DmaConfig,
    ) -> Self {
        let first_buf = {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on it until the
            // end of the DMA transfer
            let (buf_ptr, buf_len) = unsafe { buf.write_buffer() };
            (buf_ptr as u32, buf_len as u16)
        };

        let db = double_buf.as_mut().map(|db| {
            let (db_ptr, db_len) = unsafe { db.write_buffer() };
            (db_ptr as u32, db_len as u16)
        });
        let n_transfers = Self::init_common(&mut stream, &peripheral, config, first_buf, db);

        Self {
            stream,
            peripheral,
            _direction: PhantomData,
            buf: Some(buf),
            double_buf,
            transfer_length: n_transfers,
        }
    }

    /// Changes the buffer and restarts or continues a double buffer transfer. This must be called
    /// immediately after a transfer complete event if using double buffering, otherwise you might
    /// lose data. Returns the old buffer together with its `CurrentBuffer`. If an error occurs,
    /// this method will return the new buffer with the error.
    ///
    /// This method will clear the transfer complete flag on entry, it will also clear it again if
    /// an overrun occurs during its execution. Moreover, if an overrun occurs, the stream will be
    /// disabled and the transfer error flag will be set. This method can be called before the end
    /// of an ongoing transfer only if not using double buffering, in that case, the current
    /// transfer will be canceled and a new one will be started. A `NotReady` error will be returned
    /// if this method is called before the end of a transfer while double buffering.
    pub fn next_transfer(
        &mut self,
        mut new_buf: BUF,
    ) -> Result<(BUF, CurrentBuffer), DMAError<BUF>> {
        let ptr_and_len = {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on it until the
            // end of the DMA transfer
            let (buf_ptr, buf_len) = unsafe { new_buf.write_buffer() };
            (buf_ptr as u32, buf_len as u16)
        };
        self.next_transfer_common(new_buf, ptr_and_len, self.double_buf.is_some())
    }

    /// Changes the buffer and restarts or continues a double buffer transfer. This must be called
    /// immediately after a transfer complete event if using double buffering, otherwise you might
    /// lose data. The closure must return `(BUF, T)` where `BUF` is the new buffer to be used. This
    /// method can be called before the end of an ongoing transfer only if not using double
    /// buffering, in that case, the current transfer will be canceled and a new one will be
    /// started. A `NotReady` error will be returned if this method is called before the end of a
    /// transfer while double buffering and the closure won't be executed.
    ///
    /// # Panics
    /// This method will panic when double buffering if one or both of the following conditions
    /// happen:
    ///
    /// * The new buffer's length is smaller than the one used in the `init` method.
    /// * The closure `f` takes too long to return and a buffer overrun occurs.
    ///
    /// # Safety
    ///
    /// Memory corruption might occur in the previous buffer, the one passed to the closure, if an
    /// overrun occurs in double buffering mode.
    pub unsafe fn next_transfer_with<F, T>(&mut self, f: F) -> Result<T, DMAError<()>>
    where
        F: FnOnce(BUF, CurrentBuffer) -> (BUF, T),
    {
        if self.double_buf.is_some() {
            if !self.stream.is_transfer_complete() {
                return Err(DMAError::NotReady(()));
            }
            self.stream.clear_transfer_complete();

            let current_buffer = self.stream.current_buffer();
            // double buffering, unwrap can never fail
            let db = if current_buffer == CurrentBuffer::SecondBuffer {
                self.buf.take().unwrap()
            } else {
                self.double_buf.take().unwrap()
            };
            let r = f(db, !current_buffer);
            let mut new_buf = r.0;
            let ptr_and_len = {
                let (new_buf_ptr, new_buf_len) = new_buf.write_buffer();
                (new_buf_ptr as u32, new_buf_len as u16)
            };
            self.next_transfer_with_common(new_buf, ptr_and_len, true, current_buffer);
            return Ok(r.1);
        }
        stream_disable(&mut self.stream);
        self.stream.clear_transfer_complete();

        // "No re-ordering of reads and writes across this point is allowed"
        compiler_fence(Ordering::SeqCst);

        // Can never fail, we never let the Transfer without a buffer
        let old_buf = self.buf.take().unwrap();
        let r = f(old_buf, CurrentBuffer::FirstBuffer);
        let mut new_buf = r.0;

        let ptr_and_len = {
            let (new_buf_ptr, new_buf_len) = new_buf.write_buffer();
            (new_buf_ptr as u32, new_buf_len as u16)
        };
        self.next_transfer_with_common(new_buf, ptr_and_len, false, CurrentBuffer::FirstBuffer);
        Ok(r.1)
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, BUF, S>
    Transfer<STREAM, CHANNEL, PERIPHERAL, MemoryToMemory<S>, BUF>
where
    STREAM: Stream,
    ChannelX<CHANNEL>: Channel,
    PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, MemoryToMemory<S>>,
    MemoryToMemory<S>: PeriAddress,
    BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize>,
{
    /// Configures the DMA stream to the correct channel for the peripheral, configures source and
    /// destination and applies supplied configuration. In a memory to memory transfer,
    /// the `double_buf` argument is the source of the data. If double buffering is enabled, the
    /// number of transfers will be the minimum length of `memory` and `double_buf`.
    ///
    /// # Panics
    ///
    /// * When the FIFO is disabled or double buffering is enabled in `DmaConfig` while initializing
    /// a memory to memory transfer.
    pub fn init_memory_to_memory(
        mut stream: STREAM,
        peripheral: PERIPHERAL,
        mut buf: BUF,
        mut double_buf: BUF,
        config: config::DmaConfig,
    ) -> Self {
        let first_buf = {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on it until the
            // end of the DMA transfer
            let (buf_ptr, buf_len) = unsafe { buf.write_buffer() };
            (buf_ptr as u32, buf_len as u16)
        };

        let db = {
            let (db_ptr, db_len) = unsafe { double_buf.write_buffer() };
            (db_ptr as u32, db_len as u16)
        };
        let n_transfers = Self::init_common(&mut stream, &peripheral, config, first_buf, Some(db));

        Self {
            stream,
            peripheral,
            _direction: PhantomData,
            buf: Some(buf),
            double_buf: Some(double_buf),
            transfer_length: n_transfers,
        }
    }

    /// Changes the buffer and restarts.Returns the old buffer together with its `CurrentBuffer`. If
    /// an error occurs, this method will return the new buffer with the error.
    ///
    /// This method will clear the transfer complete flag on entry. This method can be called before
    /// the end of an ongoing transfer, in that case, the current transfer will be canceled and a
    /// new one will be started.
    pub fn next_transfer(
        &mut self,
        mut new_buf: BUF,
    ) -> Result<(BUF, CurrentBuffer), DMAError<BUF>> {
        let ptr_and_len = {
            // NOTE(unsafe) We now own this buffer and we won't call any &mut methods on it until the
            // end of the DMA transfer
            let (buf_ptr, buf_len) = unsafe { new_buf.write_buffer() };
            (buf_ptr as u32, buf_len as u16)
        };
        self.next_transfer_common(new_buf, ptr_and_len, false)
    }

    /// Changes the buffer and restarts a transfer. This must be called
    /// The closure must return `(BUF, T)` where `BUF` is the new buffer to be used. This
    /// method can be called before the end of an ongoing transfer,
    /// in that case, the current transfer will be canceled and a new one will be
    /// started.
    pub fn next_transfer_with<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(BUF, CurrentBuffer) -> (BUF, T),
    {
        stream_disable(&mut self.stream);
        self.stream.clear_transfer_complete();

        // "No re-ordering of reads and writes across this point is allowed"
        compiler_fence(Ordering::SeqCst);

        // Can never fail, we never let the Transfer without a buffer
        let old_buf = self.buf.take().unwrap();
        let r = f(old_buf, CurrentBuffer::FirstBuffer);
        let mut new_buf = r.0;

        let ptr_and_len = {
            let (new_buf_ptr, new_buf_len) = unsafe { new_buf.write_buffer() };
            (new_buf_ptr as u32, new_buf_len as u16)
        };
        // NOTE(unsafe) We aren't double buffering, so this is fine
        unsafe {
            self.next_transfer_with_common(new_buf, ptr_and_len, false, CurrentBuffer::FirstBuffer);
        }
        r.1
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, DIR, BUF>
    Transfer<STREAM, CHANNEL, PERIPHERAL, DIR, BUF>
where
    STREAM: Stream,
    ChannelX<CHANNEL>: Channel,
    DIR: Direction,
    PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, DIR>,
{
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
        stream_disable(&mut self.stream)
    }

    /// Stops the stream and returns the underlying resources.
    pub fn release(mut self) -> (STREAM, PERIPHERAL, BUF, Option<BUF>) {
        stream_disable(&mut self.stream);
        compiler_fence(Ordering::SeqCst);
        self.stream.clear_flags(DmaFlags::all());

        unsafe {
            let stream = ptr::read(&self.stream);
            let peripheral = ptr::read(&self.peripheral);
            let buf = ptr::read(&self.buf);
            let double_buf = ptr::read(&self.double_buf);
            mem::forget(self);
            (stream, peripheral, buf.unwrap(), double_buf)
        }
    }

    /// Get the number of remaining transfers (ndt) of the underlying DMA stream.
    pub fn number_of_transfers(&self) -> u16 {
        self.stream.number_of_transfers()
    }

    /// Clear all interrupts flags for the DMA stream.
    #[inline(always)]
    pub fn clear_all_flags(&mut self) {
        self.stream.clear_flags(DmaFlags::all());
    }

    /// Clear transfer complete interrupt (tcif) for the DMA stream.
    #[inline(always)]
    pub fn clear_transfer_complete(&mut self) {
        self.stream.clear_transfer_complete();
    }

    /// Clear half transfer interrupt (htif) for the DMA stream.
    #[inline(always)]
    pub fn clear_half_transfer(&mut self) {
        self.stream.clear_half_transfer();
    }

    /// Clear transfer error interrupt (teif) for the DMA stream.
    #[inline(always)]
    pub fn clear_transfer_error(&mut self) {
        self.stream.clear_transfer_error();
    }

    /// Clear direct mode error interrupt (dmeif) for the DMA stream.
    #[inline(always)]
    pub fn clear_direct_mode_error(&mut self) {
        self.stream.clear_direct_mode_error();
    }

    /// Clear fifo error interrupt (feif) for the DMA stream.
    #[inline(always)]
    pub fn clear_fifo_error(&mut self) {
        self.stream.clear_fifo_error();
    }

    /// Get all interrupts flags a once.
    ///
    /// The tuple contain in order:
    ///  - transfer complete flag
    ///  - half transfer flag
    ///  - transfer error flag
    ///  - direct mode error flag
    ///  - fifo_error flag
    pub fn all_flags(&self) -> DmaFlags {
        self.stream.all_flags()
    }

    /// Get transfer complete flag.
    pub fn is_transfer_complete(&self) -> bool {
        self.stream.is_transfer_complete()
    }

    /// Get half transfer flag.
    pub fn is_half_transfer(&self) -> bool {
        self.stream.is_half_transfer()
    }

    /// Get transfer error flag
    pub fn is_transfer_error(&self) -> bool {
        self.stream.is_transfer_error()
    }

    /// Get direct mode error flag
    pub fn is_direct_mode_error(&self) -> bool {
        self.stream.is_direct_mode_error()
    }

    /// Get fifo error flag
    pub fn is_fifo_error(&self) -> bool {
        self.stream.is_fifo_error()
    }

    /// Get the underlying stream of the transfer.
    ///
    /// # Safety
    ///
    /// This implementation relies on several configurations points in order to be sound, this
    /// method can void that. The use of this method is completely discouraged, only use it if you
    /// know the internals of this API in its entirety.
    pub unsafe fn stream(&mut self) -> &mut STREAM {
        &mut self.stream
    }

    /// Wait for the transfer to complete.
    #[inline(always)]
    pub fn wait(&self) {
        while !self.stream.is_transfer_complete() {}
    }

    /// Applies all fields in DmaConfig.
    fn apply_config(stream: &mut STREAM, config: config::DmaConfig) {
        let msize = match mem::size_of::<<PERIPHERAL as PeriAddress>::MemSize>() {
            1 => DmaDataSize::Byte,
            2 => DmaDataSize::HalfWord,
            4 => DmaDataSize::Word,
            //this case can only happen on wrong implemention of PeriAddress::MemSize
            _ => DmaDataSize::Word,
        };

        stream.clear_flags(DmaFlags::all());
        stream.set_priority(config.priority);
        // NOTE(unsafe) These values are correct because of the invariants of PeriAddress
        unsafe {
            stream.set_memory_size(msize);
            stream.set_peripheral_size(msize);
        }
        stream.set_memory_increment(config.memory_increment);
        stream.set_peripheral_increment(config.peripheral_increment);

        let interrupts = DmaCommonInterrupts {
            transfer_complete: config.transfer_complete_interrupt,
            half_transfer: config.half_transfer_interrupt,
            transfer_error: config.transfer_error_interrupt,
            direct_mode_error: config.direct_mode_error_interrupt,
        };
        stream.unlisten(DmaCommonInterrupts::all());
        stream.listen(interrupts);
        if config.fifo_error_interrupt {
            stream.listen_fifo_error();
        } else {
            stream.unlisten_fifo_error();
        }
        stream.set_double_buffer(config.double_buffer);
        stream.set_fifo_threshold(config.fifo_threshold);
        stream.set_fifo_enable(config.fifo_enable);
        stream.set_memory_burst(config.memory_burst);
        stream.set_peripheral_burst(config.peripheral_burst);
    }

    fn init_common(
        stream: &mut STREAM,
        peripheral: &PERIPHERAL,
        config: config::DmaConfig,
        // pointer and len
        buf: (u32, u16),
        db: Option<(u32, u16)>,
    ) -> u16 {
        stream_disable(stream);

        // Set the channel
        stream.set_channel(ChannelX::<CHANNEL>::VALUE);

        // Set peripheral to memory mode
        stream.set_direction(DIR::direction());
        let (buf_ptr, buf_len) = buf;

        // Set the memory address
        stream.set_memory_address(buf_ptr);

        let is_mem2mem = DIR::direction() == DmaDirection::MemoryToMemory;
        if is_mem2mem {
            // Fifo must be enabled for memory to memory
            if !config.fifo_enable {
                panic!("Fifo disabled.");
            } else if config.double_buffer {
                panic!("Double buffering enabled.");
            }
        } else {
            // Set the peripheral address
            stream.set_peripheral_address(peripheral.address());
        }

        let db_len = if let Some((db_ptr, db_len)) = db {
            if is_mem2mem {
                // Double buffer is the source in mem2mem mode
                stream.set_peripheral_address(db_ptr);
            } else {
                stream.set_alternate_memory_address(db_ptr);
            }
            Some(db_len)
        } else {
            // Double buffer mode must not be enabled if we haven't been given a second buffer
            if config.double_buffer {
                panic!("No second buffer.");
            }
            None
        };

        let n_transfers = if let Some(db) = db_len {
            buf_len.min(db)
        } else {
            buf_len
        };
        stream.set_number_of_transfers(n_transfers);

        Self::apply_config(stream, config);
        n_transfers
    }

    #[allow(clippy::branches_sharing_code)]
    fn next_transfer_common(
        &mut self,
        new_buf: BUF,
        ptr_and_len: (u32, u16),
        double_buffering: bool,
    ) -> Result<(BUF, CurrentBuffer), DMAError<BUF>> {
        if double_buffering {
            if !self.stream.is_transfer_complete() {
                return Err(DMAError::NotReady(new_buf));
            }
            self.stream.clear_transfer_complete();
            let (new_buf_ptr, new_buf_len) = ptr_and_len;

            // We can't change the transfer length while double buffering
            if new_buf_len < self.transfer_length {
                return Err(DMAError::SmallBuffer(new_buf));
            }

            if self.stream.current_buffer() == CurrentBuffer::SecondBuffer {
                // "Preceding reads and writes cannot be moved past subsequent writes"
                compiler_fence(Ordering::Release);
                self.stream.set_memory_address(new_buf_ptr);

                // Check if an overrun occurred, the buffer address won't be updated in that case
                if self.stream.memory_address() != new_buf_ptr {
                    self.stream.clear_transfer_complete();
                    return Err(DMAError::Overrun(new_buf));
                }

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                let old_buf = self.buf.replace(new_buf);

                // We always have a buffer, so unwrap can't fail
                return Ok((old_buf.unwrap(), CurrentBuffer::FirstBuffer));
            } else {
                // "Preceding reads and writes cannot be moved past subsequent writes"
                compiler_fence(Ordering::Release);
                self.stream.set_alternate_memory_address(new_buf_ptr);

                // Check if an overrun occurred, the buffer address won't be updated in that case
                if self.stream.alternate_memory_address() != new_buf_ptr {
                    self.stream.clear_transfer_complete();
                    return Err(DMAError::Overrun(new_buf));
                }

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                let old_buf = self.double_buf.replace(new_buf);

                // double buffering, unwrap can never fail
                return Ok((old_buf.unwrap(), CurrentBuffer::SecondBuffer));
            }
        }
        stream_disable(&mut self.stream);
        self.stream.clear_transfer_complete();

        // "No re-ordering of reads and writes across this point is allowed"
        compiler_fence(Ordering::SeqCst);

        let (buf_ptr, buf_len) = ptr_and_len;
        self.stream.set_memory_address(buf_ptr);
        self.stream.set_number_of_transfers(buf_len);
        let old_buf = self.buf.replace(new_buf);

        unsafe {
            self.stream.enable();
        }

        Ok((old_buf.unwrap(), CurrentBuffer::FirstBuffer))
    }

    /// # Safety
    ///
    /// Memory corruption might occur in the previous buffer, the one passed to the closure, if an
    /// overrun occurs in double buffering mode.
    #[allow(clippy::branches_sharing_code)]
    unsafe fn next_transfer_with_common(
        &mut self,
        new_buf: BUF,
        ptr_and_len: (u32, u16),
        double_buffering: bool,
        current_buffer: CurrentBuffer,
    ) {
        if double_buffering {
            let (new_buf_ptr, new_buf_len) = ptr_and_len;

            // We can't change the transfer length while double buffering
            assert!(
                new_buf_len >= self.transfer_length,
                "Second Buffer not big enough"
            );

            // We don't know how long the closure took to complete, we might have changed the
            // current buffer twice (or any even number of times) and got back to the same buffer
            // we had in the beginning of the method, check for that
            if self.stream.is_transfer_complete() {
                // If this is true, then RAM corruption might have occurred, there's nothing we
                // can do apart from panicking.
                // TODO: Is this the best solution ? The closure based approach seems necessary
                // if we want to support BBqueue.
                panic!("Overrun");
            }

            if current_buffer == CurrentBuffer::SecondBuffer {
                // "Preceding reads and writes cannot be moved past subsequent writes"
                compiler_fence(Ordering::Release);
                self.stream.set_memory_address(new_buf_ptr);

                // Check again if an overrun occurred, the buffer address won't be updated in that
                // case
                if self.stream.memory_address() != new_buf_ptr {
                    panic!("Overrun");
                }

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                self.buf.replace(new_buf);
            } else {
                // "Preceding reads and writes cannot be moved past subsequent writes"
                compiler_fence(Ordering::Release);
                self.stream.set_alternate_memory_address(new_buf_ptr);

                if self.stream.alternate_memory_address() != new_buf_ptr {
                    panic!("Overrun");
                }

                // "Subsequent reads and writes cannot be moved ahead of preceding reads"
                compiler_fence(Ordering::Acquire);

                self.double_buf.replace(new_buf);
            }
            return;
        }
        let (buf_ptr, buf_len) = ptr_and_len;
        self.stream.set_memory_address(buf_ptr);
        self.stream.set_number_of_transfers(buf_len);
        self.buf.replace(new_buf);

        self.stream.enable();
    }
}

impl<STREAM, const CHANNEL: u8, PERIPHERAL, DIR, BUF> Drop
    for Transfer<STREAM, CHANNEL, PERIPHERAL, DIR, BUF>
where
    STREAM: Stream,
    PERIPHERAL: PeriAddress,
{
    fn drop(&mut self) {
        stream_disable(&mut self.stream);
        compiler_fence(Ordering::SeqCst);
    }
}
