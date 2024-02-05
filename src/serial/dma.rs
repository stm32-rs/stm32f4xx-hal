use core::{marker::PhantomData, mem::transmute, ops::Deref};

use super::{Instance, RegisterBlockImpl, Serial};
use crate::dma::{
    config::DmaConfig,
    traits::{Channel, DMASet, DmaFlagExt, PeriAddress, Stream, StreamISR},
    ChannelX, MemoryToPeripheral, PeripheralToMemory, Transfer,
};
use crate::ReadFlags;

use nb;

#[non_exhaustive]
pub enum Error {
    SerialError(super::Error),
    TransferError,
}

/// Tag for TX/RX channel that a corresponding channel should not be used in DMA mode
#[non_exhaustive]
pub struct NoDMA;

/// Callback type to notify user code of completion serial transfers
pub type SerialCompleteCallback = fn(Result<(), Error>);

pub trait SerialWriteDMA {
    /// Writes `bytes` to the serial interface in non-blocking mode
    ///
    /// # Arguments
    /// * `bytes` - byte slice that need to send
    /// * `callback` - callback that will be called on completion
    ///
    /// # Safety
    /// This function relies on supplied slice `bytes` until `callback` called. So the slice must live until that moment.
    ///
    /// # Warning
    /// `callback` may be called before function returns value. It happens on errors in preparation stages.
    unsafe fn write_dma(
        &mut self,
        bytes: &[u8],
        callback: Option<SerialCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

pub trait SerialReadDMA {
    /// Reads bytes from the serial interface in non-blocking mode and writes these bytes in `buf`
    ///
    /// # Arguments
    /// * `buf` - byte slice where received bytes will be written
    /// * `callback` - callback that will be called on completion
    ///
    /// # Safety
    /// This function relies on supplied slice `buf` until `callback` called. So the slice must live until that moment.
    ///
    /// # Warning
    /// `callback` may be called before function returns value. It happens on errors in preparation stages.
    unsafe fn read_dma(
        &mut self,
        buf: &mut [u8],
        callback: Option<SerialCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

pub trait SerialWriteReadDMA {
    /// Writes `bytes` to the serial interface in non-blocking mode and then generate ReStart and receive a bytes from the safe interface
    ///
    /// # Arguments
    /// * `bytes` - byte slice that need to send
    /// * `buf` - byte slice where received bytes will be written
    /// * `callback` - callback that will be called on completion
    ///
    /// # Safety
    /// This function relies on supplied slices `bytes` and `buf` until `callback` called. So slices must live until that moment.
    ///
    /// # Warning
    /// `callback` may be called before function returns value. It happens on errors in preparation stages.
    unsafe fn write_read_dma(
        &mut self,
        bytes: &[u8],
        buf: &mut [u8],
        callback: Option<SerialCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

/// Trait with handle interrupts functions
pub trait SerialHandleIT {
    fn handle_dma_interrupt(&mut self);
    fn handle_error_interrupt(&mut self);
}

impl<Serial_> Serial<Serial_>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,
{
    /// Converts blocking [Serial] to non-blocking [SerialDma] that use `tx_stream` and `rx_stream` to send/receive data
    pub fn use_dma<TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
        rx_stream: RX_STREAM,
    ) -> SerialDma<Serial_, TxDMA<Serial_, TX_STREAM, TX_CH>, RxDMA<Serial_, RX_STREAM, RX_CH>>
    where
        TX_STREAM: Stream,
        ChannelX<TX_CH>: Channel,
        Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

        RX_STREAM: Stream,
        ChannelX<RX_CH>: Channel,
        Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
    {
        let tx = TxDMA::new(tx_stream);
        let rx = RxDMA::new(rx_stream);

        SerialDma {
            hal_serial: self,
            callback: None,
            tx,
            rx,
        }
    }

    /// Converts blocking [Serial] to non-blocking [SerialDma] that use `tx_stream` to only send data
    pub fn use_dma_tx<TX_STREAM, const TX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
    ) -> SerialDma<Serial_, TxDMA<Serial_, TX_STREAM, TX_CH>, NoDMA>
    where
        TX_STREAM: Stream,
        ChannelX<TX_CH>: Channel,
        Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
    {
        let tx = TxDMA::new(tx_stream);
        let rx = NoDMA;

        SerialDma {
            hal_serial: self,
            callback: None,
            tx,
            rx,
        }
    }

    /// Converts blocking [Serial] to non-blocking [SerialDma] that use `rx_stream` to only receive data
    pub fn use_dma_rx<RX_STREAM, const RX_CH: u8>(
        self,
        rx_stream: RX_STREAM,
    ) -> SerialDma<Serial_, NoDMA, RxDMA<Serial_, RX_STREAM, RX_CH>>
    where
        RX_STREAM: Stream,
        ChannelX<RX_CH>: Channel,
        Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
    {
        let tx = NoDMA;
        let rx = RxDMA::new(rx_stream);

        SerialDma {
            hal_serial: self,
            callback: None,
            tx,
            rx,
        }
    }
}

/// Serial abstraction that can work in non-blocking mode by using DMA
///
/// The struct should be used for sending/receiving bytes to/from the serial interface in non-blocking mode.
/// A client must follow these requirements to use that feature:
/// * Enable interrupts DMAx_STREAMy used for transmit and another DMAq_STREAMp used for receive.
/// * In these interrupts call [`handle_dma_interrupt`](Self::handle_dma_interrupt); defined in trait SerialHandleIT.
/// * Enable interrupts USARTx or UARTx for handling errors and call [`handle_error_interrupt`](Self::handle_error_interrupt) in corresponding handler; defined in trait SerialHandleIT.
///
/// The struct can be also used to send/receive bytes in blocking mode with methods:
/// [`write`](Self::write()), [`read`](Self::read()), [`write_read`](Self::write_read()).
pub struct SerialDma<Serial_, TX_TRANSFER, RX_TRANSFER>
where
    Serial_: Instance,
{
    hal_serial: Serial<Serial_>,
    callback: Option<SerialCompleteCallback>,
    tx: TX_TRANSFER,
    rx: RX_TRANSFER,
}

/// trait for DMA transfer holder
pub trait DMATransfer<BUF> {
    /// Creates DMA Transfer using specified buffer
    fn create_transfer(&mut self, buf: BUF);
    /// Destroys created transfer
    /// # Panics
    ///   - If transfer had not created before
    fn destroy_transfer(&mut self);
    /// Checks if transfer created
    fn created(&self) -> bool;
}

// Mock implementations for NoDMA
// For Tx operations
impl DMATransfer<&'static [u8]> for NoDMA {
    fn create_transfer(&mut self, _: &'static [u8]) {
        unreachable!()
    }
    fn destroy_transfer(&mut self) {
        unreachable!()
    }
    fn created(&self) -> bool {
        false
    }
}
// ... and for Rx operations
impl DMATransfer<&'static mut [u8]> for NoDMA {
    fn create_transfer(&mut self, _: &'static mut [u8]) {
        unreachable!()
    }
    fn destroy_transfer(&mut self) {
        unreachable!()
    }
    fn created(&self) -> bool {
        false
    }
}

/// DMA Transfer holder for Tx operations
pub struct TxDMA<Serial_, TX_STREAM, const TX_CH: u8>
where
    Serial_: Instance,
    TX_STREAM: Stream,
{
    tx: Option<Tx<Serial_>>,
    tx_stream: Option<TX_STREAM>,
    tx_transfer: Option<Transfer<TX_STREAM, TX_CH, Tx<Serial_>, MemoryToPeripheral, &'static [u8]>>,
}

impl<Serial_, TX_STREAM, const TX_CH: u8> TxDMA<Serial_, TX_STREAM, TX_CH>
where
    Serial_: Instance,
    TX_STREAM: Stream,
{
    fn new(stream: TX_STREAM) -> Self {
        let tx = Tx {
            serial: PhantomData,
        };

        Self {
            tx: Some(tx),
            tx_stream: Some(stream),
            tx_transfer: None,
        }
    }
}

impl<Serial_, TX_STREAM, const TX_CH: u8> DMATransfer<&'static [u8]>
    for TxDMA<Serial_, TX_STREAM, TX_CH>
where
    Serial_: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
{
    fn create_transfer(&mut self, buf: &'static [u8]) {
        assert!(self.tx.is_some());
        assert!(self.tx_stream.is_some());

        let transfer = Transfer::init_memory_to_peripheral(
            self.tx_stream.take().unwrap(),
            self.tx.take().unwrap(),
            buf,
            None,
            DmaConfig::default()
                .memory_increment(true)
                .transfer_complete_interrupt(true)
                .transfer_error_interrupt(true),
        );

        self.tx_transfer = Some(transfer);
    }

    fn destroy_transfer(&mut self) {
        assert!(self.tx_transfer.is_some());

        let (str, tx, ..) = self.tx_transfer.take().unwrap().release();
        self.tx = Some(tx);
        self.tx_stream = Some(str);
    }

    fn created(&self) -> bool {
        self.tx_transfer.is_some()
    }
}

/// DMA Transfer holder for Rx operations
pub struct RxDMA<Serial_, RX_STREAM, const RX_CH: u8>
where
    Serial_: Instance,
    RX_STREAM: Stream,
{
    rx: Option<Rx<Serial_>>,
    rx_stream: Option<RX_STREAM>,
    rx_transfer:
        Option<Transfer<RX_STREAM, RX_CH, Rx<Serial_>, PeripheralToMemory, &'static mut [u8]>>,
}

impl<Serial_, RX_STREAM, const RX_CH: u8> RxDMA<Serial_, RX_STREAM, RX_CH>
where
    Serial_: Instance,
    RX_STREAM: Stream,
{
    fn new(stream: RX_STREAM) -> Self {
        let tx = Rx {
            serial: PhantomData,
        };

        Self {
            rx: Some(tx),
            rx_stream: Some(stream),
            rx_transfer: None,
        }
    }
}

impl<Serial_, RX_STREAM, const RX_CH: u8> DMATransfer<&'static mut [u8]>
    for RxDMA<Serial_, RX_STREAM, RX_CH>
where
    Serial_: Instance,
    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn create_transfer(&mut self, buf: &'static mut [u8]) {
        assert!(self.rx.is_some());
        assert!(self.rx_stream.is_some());

        let transfer = Transfer::init_peripheral_to_memory(
            self.rx_stream.take().unwrap(),
            self.rx.take().unwrap(),
            buf,
            None,
            DmaConfig::default()
                .memory_increment(true)
                .transfer_complete_interrupt(true)
                .transfer_error_interrupt(true),
        );

        self.rx_transfer = Some(transfer);
    }

    fn destroy_transfer(&mut self) {
        assert!(self.rx_transfer.is_some());

        let (str, tx, ..) = self.rx_transfer.take().unwrap().release();
        self.rx = Some(tx);
        self.rx_stream = Some(str);
    }

    fn created(&self) -> bool {
        self.rx_transfer.is_some()
    }
}

/// Common implementation
impl<Serial_, TX_TRANSFER, RX_TRANSFER> SerialDma<Serial_, TX_TRANSFER, RX_TRANSFER>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,
    TX_TRANSFER: DMATransfer<&'static [u8]>,
    RX_TRANSFER: DMATransfer<&'static mut [u8]>,
{
    fn call_callback_once(&mut self, res: Result<(), Error>) {
        if let Some(c) = self.callback.take() {
            c(res);
        }
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<(), super::Error> {
        self.hal_serial.tx.usart.deref().bwrite_all_u8(bytes)
    }

    pub fn read(&mut self, bytes: &mut [u8]) -> Result<(), super::Error> {
        self.hal_serial.tx.usart.deref().bread_all_u8(bytes)
    }

    fn enable_error_interrupt_generation(&mut self) {
        self.hal_serial.tx.usart.enable_error_interrupt_generation();
    }

    fn disable_error_interrupt_generation(&mut self) {
        self.hal_serial
            .tx
            .usart
            .disable_error_interrupt_generation();
    }

    fn finish_transfer_with_result(&mut self, result: Result<(), Error>) {
        self.disable_error_interrupt_generation();

        self.call_callback_once(result);

        if self.tx.created() {
            self.tx.destroy_transfer();
        }

        if self.rx.created() {
            self.rx.destroy_transfer();
        }
    }
}

impl<Serial_, TX_STREAM, const TX_CH: u8> SerialHandleIT
    for SerialDma<Serial_, TxDMA<Serial_, TX_STREAM, TX_CH>, NoDMA>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,

    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
{
    fn handle_dma_interrupt(&mut self) {
        if let Some(tx_t) = &mut self.tx.tx_transfer {
            let flags = tx_t.flags();

            if flags.is_fifo_error() {
                tx_t.clear_fifo_error();
            } else if flags.is_transfer_error() {
                tx_t.clear_transfer_error();

                self.finish_transfer_with_result(Err(Error::TransferError));
            } else if flags.is_transfer_complete() {
                tx_t.clear_transfer_complete();

                self.finish_transfer_with_result(Ok(()));
            }
        }
    }

    fn handle_error_interrupt(&mut self) {
        let res = self
            .hal_serial
            .tx
            .usart
            .deref()
            .check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::SerialError(e)));
        }
    }
}

impl<Serial_, RX_STREAM, const RX_CH: u8> SerialHandleIT
    for SerialDma<Serial_, NoDMA, RxDMA<Serial_, RX_STREAM, RX_CH>>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn handle_dma_interrupt(&mut self) {
        if let Some(rx_t) = &mut self.rx.rx_transfer {
            let flags = rx_t.flags();

            if flags.is_fifo_error() {
                rx_t.clear_fifo_error();
            } else if flags.is_transfer_error() {
                rx_t.clear_transfer_error();

                self.finish_transfer_with_result(Err(Error::TransferError));
            } else if flags.is_transfer_complete() {
                rx_t.clear_transfer_complete();

                self.finish_transfer_with_result(Ok(()));
            }
        }
    }

    fn handle_error_interrupt(&mut self) {
        let res = self
            .hal_serial
            .tx
            .usart
            .deref()
            .check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::SerialError(e)));
        }
    }
}

/// Only for both TX and RX DMA
impl<Serial_, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8> SerialHandleIT
    for SerialDma<Serial_, TxDMA<Serial_, TX_STREAM, TX_CH>, RxDMA<Serial_, RX_STREAM, RX_CH>>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,

    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn handle_dma_interrupt(&mut self) {
        // Handle Transmit
        if let Some(tx_t) = &mut self.tx.tx_transfer {
            let flags = tx_t.flags();

            if flags.is_fifo_error() {
                tx_t.clear_fifo_error();
            } else if flags.is_transfer_error() {
                tx_t.clear_transfer_error();

                self.finish_transfer_with_result(Err(Error::TransferError));
            } else if flags.is_transfer_complete() {
                tx_t.clear_transfer_complete();

                // If we have prepared Rx Transfer, there are write_read command, generate restart signal and do not disable DMA requests
                // Indicate that we have read after this transmit
                let have_read_after = self.rx.rx_transfer.is_some();

                self.tx.destroy_transfer();

                // If we have prepared Rx Transfer, there are write_read command, generate restart signal
                if have_read_after {
                    self.rx.rx_transfer.as_mut().unwrap().start(|_| {});
                } else {
                    self.finish_transfer_with_result(Ok(()));
                }
            }

            // If Transmit handled then receive should not be handled even if exists.
            // This return protects for handling Tx and Rx events in one interrupt.
            return;
        }

        if let Some(rx_t) = &mut self.rx.rx_transfer {
            let flags = rx_t.flags();

            if flags.is_fifo_error() {
                rx_t.clear_fifo_error();
            } else if flags.is_transfer_error() {
                rx_t.clear_transfer_error();

                self.finish_transfer_with_result(Err(Error::TransferError));
            } else if flags.is_transfer_complete() {
                rx_t.clear_transfer_complete();

                self.finish_transfer_with_result(Ok(()));
            }
        }
    }

    fn handle_error_interrupt(&mut self) {
        let res = self
            .hal_serial
            .tx
            .usart
            .deref()
            .check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::SerialError(e)));
        }
    }
}

// Write DMA implementations for TX only and TX/RX Serial DMA
impl<Serial_, TX_STREAM, const TX_CH: u8, RX_TRANSFER> SerialWriteDMA
    for SerialDma<Serial_, TxDMA<Serial_, TX_STREAM, TX_CH>, RX_TRANSFER>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,

    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_TRANSFER: DMATransfer<&'static mut [u8]>,
{
    unsafe fn write_dma(
        &mut self,
        bytes: &[u8],
        callback: Option<SerialCompleteCallback>,
    ) -> nb::Result<(), super::Error> {
        self.enable_error_interrupt_generation();
        let static_bytes: &'static [u8] = transmute(bytes);
        self.tx.create_transfer(static_bytes);
        self.callback = callback;

        // Start DMA processing
        self.tx.tx_transfer.as_mut().unwrap().start(|_| {});

        Ok(())
    }
}

// Read DMA implementations for RX only and TX/RX Serial DMA
impl<Serial_, TX_TRANSFER, RX_STREAM, const RX_CH: u8> SerialReadDMA
    for SerialDma<Serial_, TX_TRANSFER, RxDMA<Serial_, RX_STREAM, RX_CH>>
where
    Serial_: Instance,
    Serial_: Deref<Target = <Serial_ as Instance>::RegisterBlock>,
    <Serial_ as Instance>::RegisterBlock: RegisterBlockImpl,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,

    TX_TRANSFER: DMATransfer<&'static [u8]>,
{
    unsafe fn read_dma(
        &mut self,
        buf: &mut [u8],
        callback: Option<SerialCompleteCallback>,
    ) -> nb::Result<(), super::Error> {
        self.enable_error_interrupt_generation();
        let static_buf: &'static mut [u8] = transmute(buf);
        self.rx.create_transfer(static_buf);
        self.callback = callback;

        // Start DMA processing
        self.rx.rx_transfer.as_mut().unwrap().start(|_| {});

        Ok(())
    }
}

impl<Serial_, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8> SerialWriteReadDMA
    for SerialDma<Serial_, TxDMA<Serial_, TX_STREAM, TX_CH>, RxDMA<Serial_, RX_STREAM, RX_CH>>
where
    Serial_: Instance,

    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<Serial_>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<Serial_>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    unsafe fn write_read_dma(
        &mut self,
        bytes: &[u8],
        buf: &mut [u8],
        callback: Option<SerialCompleteCallback>,
    ) -> nb::Result<(), super::Error> {
        let static_bytes: &'static [u8] = transmute(bytes);
        self.tx.create_transfer(static_bytes);
        let static_buf: &'static mut [u8] = transmute(buf);
        self.rx.create_transfer(static_buf);
        self.callback = callback;

        // Start DMA processing
        self.tx.tx_transfer.as_mut().unwrap().start(|_| {});

        Ok(())
    }
}

pub struct Tx<Serial_> {
    serial: PhantomData<Serial_>,
}

pub struct Rx<Serial_> {
    serial: PhantomData<Serial_>,
}

unsafe impl<Serial_> PeriAddress for Rx<Serial_>
where
    Serial_: Instance,
{
    #[inline(always)]
    fn address(&self) -> u32 {
        <Serial_ as Instance>::peri_address()
    }

    type MemSize = u8;
}

unsafe impl<Serial_> PeriAddress for Tx<Serial_>
where
    Serial_: Instance,
{
    #[inline(always)]
    fn address(&self) -> u32 {
        <Serial_ as Instance>::peri_address()
    }

    type MemSize = u8;
}

unsafe impl<Serial_, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory>
    for Rx<Serial_>
where
    Serial_: DMASet<STREAM, CHANNEL, PeripheralToMemory>,
{
}

unsafe impl<Serial_, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral>
    for Tx<Serial_>
where
    Serial_: DMASet<STREAM, CHANNEL, MemoryToPeripheral>,
{
}
