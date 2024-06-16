use core::{marker::PhantomData, mem::transmute};

use super::{Instance, RegisterBlockImpl, Rx, Serial, Tx};
use crate::dma::{
    config::DmaConfig,
    traits::{Channel, DMASet, DmaFlagExt, Stream, StreamISR},
    ChannelX, MemoryToPeripheral, PeripheralToMemory, Transfer, TransferState,
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

/// Trait with handle interrupts functions
pub trait SerialHandleIT {
    fn handle_dma_interrupt(&mut self);
    fn handle_error_interrupt(&mut self);
}

impl<UART: Instance> Serial<UART> {
    /// Converts blocking [Serial] to non-blocking [SerialDma] that use `tx_stream` and `rx_stream` to send/receive data
    pub fn use_dma<TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
        rx_stream: RX_STREAM,
    ) -> SerialDma<UART, TxDMA<UART, TX_STREAM, TX_CH>, RxDMA<UART, RX_STREAM, RX_CH>>
    where
        TX_STREAM: Stream,
        ChannelX<TX_CH>: Channel,
        Tx<UART>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

        RX_STREAM: Stream,
        ChannelX<RX_CH>: Channel,
        Rx<UART>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
    {
        let tx = TxDMA::new(self.tx, tx_stream);
        let rx = RxDMA::new(self.rx, rx_stream);

        SerialDma {
            _uart: PhantomData,
            callback: None,
            tx,
            rx,
        }
    }

    /// Converts blocking [Serial] to non-blocking [SerialDma] that use `tx_stream` to only send data
    pub fn use_dma_tx<TX_STREAM, const TX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
    ) -> SerialDma<UART, TxDMA<UART, TX_STREAM, TX_CH>, NoDMA>
    where
        TX_STREAM: Stream,
        ChannelX<TX_CH>: Channel,
        Tx<UART>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
    {
        let tx = TxDMA::new(self.tx, tx_stream);
        let rx = NoDMA;

        SerialDma {
            _uart: PhantomData,
            callback: None,
            tx,
            rx,
        }
    }

    /// Converts blocking [Serial] to non-blocking [SerialDma] that use `rx_stream` to only receive data
    pub fn use_dma_rx<RX_STREAM, const RX_CH: u8>(
        self,
        rx_stream: RX_STREAM,
    ) -> SerialDma<UART, NoDMA, RxDMA<UART, RX_STREAM, RX_CH>>
    where
        RX_STREAM: Stream,
        ChannelX<RX_CH>: Channel,
        Rx<UART>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
    {
        let tx = NoDMA;
        let rx = RxDMA::new(self.rx, rx_stream);

        SerialDma {
            _uart: PhantomData,
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
pub struct SerialDma<UART: Instance, TX_TRANSFER, RX_TRANSFER> {
    _uart: PhantomData<UART>,
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
pub struct TxDMA<UART, TX_STREAM, const TX_CH: u8>
where
    UART: Instance,
    TX_STREAM: Stream,
{
    state: TransferState<TX_STREAM, TX_CH, Tx<UART>, MemoryToPeripheral, &'static [u8]>,
}

impl<UART, TX_STREAM, const TX_CH: u8> TxDMA<UART, TX_STREAM, TX_CH>
where
    UART: Instance,
    TX_STREAM: Stream,
{
    fn new(tx: Tx<UART>, stream: TX_STREAM) -> Self {
        Self {
            state: TransferState::Stopped { periph: tx, stream },
        }
    }
}

impl<UART, TX_STREAM, const TX_CH: u8> DMATransfer<&'static [u8]> for TxDMA<UART, TX_STREAM, TX_CH>
where
    UART: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<UART>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
{
    fn create_transfer(&mut self, buf: &'static [u8]) {
        if let TransferState::Stopped { periph, stream } = core::mem::take(&mut self.state) {
            let transfer = Transfer::init_memory_to_peripheral(
                stream,
                periph,
                buf,
                None,
                DmaConfig::default()
                    .memory_increment(true)
                    .transfer_complete_interrupt(true)
                    .transfer_error_interrupt(true),
            );

            self.state = TransferState::Running { transfer };
        } else {
            panic!("Broken TxDMA")
        }
    }

    fn destroy_transfer(&mut self) {
        if let TransferState::Running { transfer } = core::mem::take(&mut self.state) {
            let (stream, tx, ..) = transfer.release();
            self.state = TransferState::Stopped { periph: tx, stream }
        } else {
            panic!("Broken TxDMA")
        }
    }

    fn created(&self) -> bool {
        self.state.is_running()
    }
}

/// DMA Transfer holder for Rx operations
pub struct RxDMA<UART, RX_STREAM, const RX_CH: u8>
where
    UART: Instance,
    RX_STREAM: Stream,
{
    state: TransferState<RX_STREAM, RX_CH, Rx<UART>, PeripheralToMemory, &'static mut [u8]>,
}

impl<UART, RX_STREAM, const RX_CH: u8> RxDMA<UART, RX_STREAM, RX_CH>
where
    UART: Instance,
    RX_STREAM: Stream,
{
    fn new(rx: Rx<UART>, stream: RX_STREAM) -> Self {
        Self {
            state: TransferState::Stopped { periph: rx, stream },
        }
    }
}

impl<UART, RX_STREAM, const RX_CH: u8> DMATransfer<&'static mut [u8]>
    for RxDMA<UART, RX_STREAM, RX_CH>
where
    UART: Instance,
    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<UART>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn create_transfer(&mut self, buf: &'static mut [u8]) {
        if let TransferState::Stopped { periph, stream } = core::mem::take(&mut self.state) {
            let transfer = Transfer::init_peripheral_to_memory(
                stream,
                periph,
                buf,
                None,
                DmaConfig::default()
                    .memory_increment(true)
                    .transfer_complete_interrupt(true)
                    .transfer_error_interrupt(true),
            );

            self.state = TransferState::Running { transfer };
        } else {
            panic!("Broken RxDMA")
        }
    }

    fn destroy_transfer(&mut self) {
        if let TransferState::Running { transfer } = core::mem::take(&mut self.state) {
            let (stream, rx, ..) = transfer.release();
            self.state = TransferState::Stopped { periph: rx, stream }
        } else {
            panic!("Broken RxDMA")
        }
    }

    fn created(&self) -> bool {
        self.state.is_running()
    }
}

pub trait Periph {
    type Uart: Instance;
    fn periph(&self) -> &Self::Uart;
}

impl<UART: Instance, TX_STREAM, const TX_CH: u8, RX_TRANSFER> Periph
    for SerialDma<UART, TxDMA<UART, TX_STREAM, TX_CH>, RX_TRANSFER>
where
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
{
    type Uart = UART;
    fn periph(&self) -> &Self::Uart {
        &self.tx.state.periph().usart
    }
}
impl<UART: Instance, RX_STREAM, const RX_CH: u8> Periph
    for SerialDma<UART, NoDMA, RxDMA<UART, RX_STREAM, RX_CH>>
where
    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
{
    type Uart = UART;
    fn periph(&self) -> &Self::Uart {
        &self.rx.state.periph().usart
    }
}

/// Common implementation
impl<UART: Instance, TX_TRANSFER, RX_TRANSFER> SerialDma<UART, TX_TRANSFER, RX_TRANSFER>
where
    Self: Periph,
    TX_TRANSFER: DMATransfer<&'static [u8]>,
    RX_TRANSFER: DMATransfer<&'static mut [u8]>,
{
    fn call_callback_once(&mut self, res: Result<(), Error>) {
        if let Some(c) = self.callback.take() {
            c(res);
        }
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<(), super::Error> {
        self.periph().bwrite_all_u8(bytes)
    }

    pub fn read(&mut self, bytes: &mut [u8]) -> Result<(), super::Error> {
        self.periph().bread_all_u8(bytes)
    }

    fn enable_error_interrupt_generation(&mut self) {
        self.periph().enable_error_interrupt_generation();
    }

    fn disable_error_interrupt_generation(&mut self) {
        self.periph().disable_error_interrupt_generation();
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
impl<UART: Instance, TX_STREAM, const TX_CH: u8> SerialHandleIT
    for SerialDma<UART, TxDMA<UART, TX_STREAM, TX_CH>, NoDMA>
where
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<UART>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
{
    fn handle_dma_interrupt(&mut self) {
        if let TransferState::Running { transfer: tx_t } = &mut self.tx.state {
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
        let res = self.tx.state.periph().usart.check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::SerialError(e)));
        }
    }
}

impl<UART: Instance, RX_STREAM, const RX_CH: u8> SerialHandleIT
    for SerialDma<UART, NoDMA, RxDMA<UART, RX_STREAM, RX_CH>>
where
    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<UART>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn handle_dma_interrupt(&mut self) {
        if let TransferState::Running { transfer: rx_t } = &mut self.rx.state {
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
        let res = self.rx.state.periph().usart.check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::SerialError(e)));
        }
    }
}

/// Only for both TX and RX DMA
impl<UART: Instance, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8> SerialHandleIT
    for SerialDma<UART, TxDMA<UART, TX_STREAM, TX_CH>, RxDMA<UART, RX_STREAM, RX_CH>>
where
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<UART>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<UART>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn handle_dma_interrupt(&mut self) {
        // Handle Transmit
        if let TransferState::Running { transfer: tx_t } = &mut self.tx.state {
            let flags = tx_t.flags();

            if flags.is_fifo_error() {
                tx_t.clear_fifo_error();
            } else if flags.is_transfer_error() {
                tx_t.clear_transfer_error();

                self.finish_transfer_with_result(Err(Error::TransferError));
            } else if flags.is_transfer_complete() {
                tx_t.clear_transfer_complete();

                self.tx.destroy_transfer();

                // If we have prepared Rx Transfer, there are write_read command, generate restart signal and do not disable DMA requests
                // Indicate that we have read after this transmit
                // If we have prepared Rx Transfer, there are write_read command, generate restart signal
                if let TransferState::Running { transfer } = &mut self.rx.state {
                    transfer.start(|_| {});
                } else {
                    self.finish_transfer_with_result(Ok(()));
                }
            }

            // If Transmit handled then receive should not be handled even if exists.
            // This return protects for handling Tx and Rx events in one interrupt.
            return;
        }

        if let TransferState::Running { transfer: rx_t } = &mut self.rx.state {
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
        let res = self.tx.state.periph().usart.check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::SerialError(e)));
        }
    }
}

// Write DMA implementations for TX only and TX/RX Serial DMA
impl<UART: Instance, TX_STREAM, const TX_CH: u8, RX_TRANSFER> SerialWriteDMA
    for SerialDma<UART, TxDMA<UART, TX_STREAM, TX_CH>, RX_TRANSFER>
where
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<UART>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

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
        if let TransferState::Running { transfer } = &mut self.tx.state {
            transfer.start(|_| {});
        }

        Ok(())
    }
}

// Read DMA implementations for RX only and TX/RX Serial DMA
impl<UART: Instance, TX_TRANSFER, RX_STREAM, const RX_CH: u8> SerialReadDMA
    for SerialDma<UART, TX_TRANSFER, RxDMA<UART, RX_STREAM, RX_CH>>
where
    Self: Periph,
    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<UART>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,

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
        if let TransferState::Running { transfer } = &mut self.rx.state {
            transfer.start(|_| {});
        }

        Ok(())
    }
}
