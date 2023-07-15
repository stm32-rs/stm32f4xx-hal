use core::{marker::PhantomData, mem::transmute};

use super::{I2c, Instance};
use crate::dma::{
    config::DmaConfig,
    traits::{Channel, DMASet, DmaFlagExt, PeriAddress, Stream, StreamISR},
    ChannelX, MemoryToPeripheral, PeripheralToMemory, Transfer,
};
use crate::ReadFlags;

use nb;

#[non_exhaustive]
pub enum Error {
    I2CError(super::Error),
    TransferError,
}

/// Tag for TX/RX channel that a corresponding channel should not be used in DMA mode
#[non_exhaustive]
pub struct NoDMA;

/// Callback type to notify user code of completion I2C transfers
pub type I2cCompleteCallback = fn(Result<(), Error>);

pub trait I2CMasterWriteDMA {
    /// Writes `bytes` to slave with address `addr` in non-blocking mode
    ///
    /// # Arguments
    /// * `addr` - slave address
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
        addr: u8,
        bytes: &[u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

pub trait I2CMasterReadDMA {
    /// Reads bytes from slave device with address `addr` in non-blocking mode and writes these bytes in `buf`
    ///
    /// # Arguments
    /// * `addr` - slave address
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
        addr: u8,
        buf: &mut [u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

pub trait I2CMasterWriteReadDMA {
    /// Writes `bytes` to slave with address `addr` in non-blocking mode and then generate ReStart and receive a bytes from a same device
    ///
    /// # Arguments
    /// * `addr` - slave address
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
        addr: u8,
        bytes: &[u8],
        buf: &mut [u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

/// Trait with handle interrupts functions
pub trait I2CMasterHandleIT {
    fn handle_dma_interrupt(&mut self);
    fn handle_error_interrupt(&mut self);
}

impl<I2C: Instance> I2c<I2C> {
    /// Converts blocking [I2c] to non-blocking [I2CMasterDma] that use `tx_stream` and `rx_stream` to send/receive data
    pub fn use_dma<TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
        rx_stream: RX_STREAM,
    ) -> I2CMasterDma<I2C, TxDMA<I2C, TX_STREAM, TX_CH>, RxDMA<I2C, RX_STREAM, RX_CH>>
    where
        TX_STREAM: Stream,
        ChannelX<TX_CH>: Channel,
        Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

        RX_STREAM: Stream,
        ChannelX<RX_CH>: Channel,
        Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
    {
        let tx = TxDMA::new(tx_stream);
        let rx = RxDMA::new(rx_stream);

        I2CMasterDma {
            hal_i2c: self,
            callback: None,

            address: 0,
            rx_len: 0,

            tx,
            rx,
        }
    }

    /// Converts blocking [I2c] to non-blocking [I2CMasterDma] that use `tx_stream` to only send data
    pub fn use_dma_tx<TX_STREAM, const TX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
    ) -> I2CMasterDma<I2C, TxDMA<I2C, TX_STREAM, TX_CH>, NoDMA>
    where
        TX_STREAM: Stream,
        ChannelX<TX_CH>: Channel,
        Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
    {
        let tx = TxDMA::new(tx_stream);
        let rx = NoDMA;

        I2CMasterDma {
            hal_i2c: self,
            callback: None,

            address: 0,
            rx_len: 0,

            tx,
            rx,
        }
    }

    /// Converts blocking [I2c] to non-blocking [I2CMasterDma] that use `rx_stream` to only receive data
    pub fn use_dma_rx<RX_STREAM, const RX_CH: u8>(
        self,
        rx_stream: RX_STREAM,
    ) -> I2CMasterDma<I2C, NoDMA, RxDMA<I2C, RX_STREAM, RX_CH>>
    where
        RX_STREAM: Stream,
        ChannelX<RX_CH>: Channel,
        Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
    {
        let tx = NoDMA;
        let rx = RxDMA::new(rx_stream);

        I2CMasterDma {
            hal_i2c: self,
            callback: None,

            address: 0,
            rx_len: 0,

            tx,
            rx,
        }
    }
}

/// I2c abstraction that can work in non-blocking mode by using DMA
///
/// The struct should be used for sending/receiving bytes to/from slave device in non-blocking mode.
/// A client must follow these requirements to use that feature:
/// * Enable interrupts DMAx_STREAMy used for transmit and another DMAq_STREAMp used for receive.
/// * In these interrupts call [`handle_dma_interrupt`](Self::handle_dma_interrupt); defined in trait I2CMasterHandleIT
/// * Enable interrupts I2Cx_ER for handling errors and call [`handle_error_interrupt`](Self::handle_error_interrupt) in corresponding handler; defined in trait I2CMasterHandleIT
///
/// The struct can be also used to send/receive bytes in blocking mode with methods:
/// [`write`](Self::write()), [`read`](Self::read()), [`write_read`](Self::write_read()).
///
pub struct I2CMasterDma<I2C, TX_TRANSFER, RX_TRANSFER>
where
    I2C: Instance,
{
    hal_i2c: I2c<I2C>,

    callback: Option<I2cCompleteCallback>,

    /// Last address used in `write_read_dma` method
    address: u8,
    /// Len of `buf` in `write_read_dma` method
    rx_len: usize,

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
pub struct TxDMA<I2C, TX_STREAM, const TX_CH: u8>
where
    I2C: Instance,
    TX_STREAM: Stream,
{
    tx: Option<Tx<I2C>>,
    tx_stream: Option<TX_STREAM>,
    tx_transfer: Option<Transfer<TX_STREAM, TX_CH, Tx<I2C>, MemoryToPeripheral, &'static [u8]>>,
}

impl<I2C, TX_STREAM, const TX_CH: u8> TxDMA<I2C, TX_STREAM, TX_CH>
where
    I2C: Instance,
    TX_STREAM: Stream,
{
    fn new(stream: TX_STREAM) -> Self {
        let tx = Tx { i2c: PhantomData };

        Self {
            tx: Some(tx),
            tx_stream: Some(stream),
            tx_transfer: None,
        }
    }
}

impl<I2C, TX_STREAM, const TX_CH: u8> DMATransfer<&'static [u8]> for TxDMA<I2C, TX_STREAM, TX_CH>
where
    I2C: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
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
pub struct RxDMA<I2C, RX_STREAM, const RX_CH: u8>
where
    I2C: Instance,
    RX_STREAM: Stream,
{
    rx: Option<Rx<I2C>>,
    rx_stream: Option<RX_STREAM>,
    rx_transfer: Option<Transfer<RX_STREAM, RX_CH, Rx<I2C>, PeripheralToMemory, &'static mut [u8]>>,
}

impl<I2C, RX_STREAM, const RX_CH: u8> RxDMA<I2C, RX_STREAM, RX_CH>
where
    I2C: Instance,
    RX_STREAM: Stream,
{
    fn new(stream: RX_STREAM) -> Self {
        let tx = Rx { i2c: PhantomData };

        Self {
            rx: Some(tx),
            rx_stream: Some(stream),
            rx_transfer: None,
        }
    }
}

impl<I2C, RX_STREAM, const RX_CH: u8> DMATransfer<&'static mut [u8]>
    for RxDMA<I2C, RX_STREAM, RX_CH>
where
    I2C: Instance,
    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
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
impl<I2C, TX_TRANSFER, RX_TRANSFER> I2CMasterDma<I2C, TX_TRANSFER, RX_TRANSFER>
where
    I2C: Instance,
    TX_TRANSFER: DMATransfer<&'static [u8]>,
    RX_TRANSFER: DMATransfer<&'static mut [u8]>,
{
    fn call_callback_once(&mut self, res: Result<(), Error>) {
        if let Some(c) = self.callback.take() {
            c(res);
        }
    }

    /// Checks if there is communication in progress
    #[inline(always)]
    pub fn busy(&self) -> bool {
        self.hal_i2c.i2c.sr2.read().busy().bit_is_set()
    }

    /// Like `busy` but returns `WouldBlock` if busy
    fn busy_res(&self) -> nb::Result<(), super::Error> {
        if self.busy() {
            return nb::Result::Err(nb::Error::WouldBlock);
        }
        Ok(())
    }

    #[inline(always)]
    fn enable_dma_requests(&mut self) {
        self.hal_i2c.i2c.cr2.modify(|_, w| w.dmaen().enabled());
    }

    #[inline(always)]
    fn disable_dma_requests(&mut self) {
        self.hal_i2c.i2c.cr2.modify(|_, w| w.dmaen().disabled());
    }

    #[inline(always)]
    fn enable_error_interrupt_generation(&mut self) {
        self.hal_i2c.i2c.cr2.modify(|_, w| w.iterren().enabled());
    }

    #[inline(always)]
    fn disable_error_interrupt_generation(&mut self) {
        self.hal_i2c.i2c.cr2.modify(|_, w| w.iterren().disabled());
    }

    fn send_start(&mut self, read: bool) -> Result<(), super::Error> {
        let i2c = &self.hal_i2c.i2c;

        // Make sure the ack and start bit is set together in a single
        // read-modify-write operation to avoid race condition.
        // See PR: https://github.com/stm32-rs/stm32f4xx-hal/pull/662
        if read {
            i2c.cr1.modify(|_, w| w.ack().set_bit().start().set_bit());
        } else {
            i2c.cr1.modify(|_, w| w.start().set_bit());
        }

        // Wait until START condition was generated
        while self
            .hal_i2c
            .check_and_clear_error_flags()?
            .sb()
            .bit_is_clear()
        {}

        // Also wait until signalled we're master and everything is waiting for us
        loop {
            self.hal_i2c.check_and_clear_error_flags()?;

            let sr2 = i2c.sr2.read();
            if !(sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()) {
                break;
            }
        }

        Ok(())
    }

    fn send_stop(&mut self) {
        self.hal_i2c.i2c.cr1.modify(|_, w| w.stop().set_bit());
    }

    fn send_address(&mut self, addr: u8, read: bool) -> Result<(), super::Error> {
        let i2c = &self.hal_i2c.i2c;

        let mut to_send_addr = u32::from(addr) << 1;
        if read {
            to_send_addr += 1;
        }

        // Set up current address, we're trying to talk to
        i2c.dr.write(|w| unsafe { w.bits(to_send_addr) });

        // Wait until address was sent
        loop {
            // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
            let sr1 = self
                .hal_i2c
                .check_and_clear_error_flags()
                .map_err(super::Error::nack_addr)?;

            // Wait for the address to be acknowledged
            if sr1.addr().bit_is_set() {
                break;
            }
        }

        Ok(())
    }

    fn prepare_write(&mut self, addr: u8) -> Result<(), super::Error> {
        // Start
        self.send_start(false)?;

        // Send address
        self.send_address(addr, false)?;

        // Clear condition by reading SR2. This will clear ADDR flag
        self.hal_i2c.i2c.sr2.read();

        // Enable error interrupts
        self.enable_error_interrupt_generation();

        Ok(())
    }

    /// Generates start and send address for read commands
    fn prepare_read(&mut self, addr: u8, buf_len: usize) -> Result<(), super::Error> {
        // Start
        self.send_start(true)?;

        // Send address
        self.send_address(addr, true)?;

        // On small sized array we need to set ACK=0 before ADDR cleared
        if buf_len <= 1 {
            self.hal_i2c.i2c.cr1.modify(|_, w| w.ack().clear_bit());
        }

        // Clear condition by reading SR2. This will clear ADDR flag
        self.hal_i2c.i2c.sr2.read();

        // Enable error interrupts
        self.enable_error_interrupt_generation();

        Ok(())
    }

    /// Reads in blocking mode but if i2c is busy returns `WouldBlock` and do nothing
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> nb::Result<(), super::Error> {
        self.busy_res()?;
        match self.hal_i2c.read(addr, buffer) {
            Ok(_) => Ok(()),
            Err(super::Error::NoAcknowledge(source)) => {
                self.send_stop();
                Err(nb::Error::Other(super::Error::NoAcknowledge(source)))
            }
            Err(error) => Err(nb::Error::Other(error)),
        }
    }

    /// Write and then read in blocking mode but if i2c is busy returns `WouldBlock` and do nothing
    pub fn write_read(
        &mut self,
        addr: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> nb::Result<(), super::Error> {
        self.busy_res()?;
        match self.hal_i2c.write_read(addr, bytes, buffer) {
            Ok(_) => Ok(()),
            Err(super::Error::NoAcknowledge(source)) => {
                self.send_stop();
                Err(nb::Error::Other(super::Error::NoAcknowledge(source)))
            }
            Err(error) => Err(nb::Error::Other(error)),
        }
    }

    /// Write in blocking mode but if i2c is busy returns `WouldBlock` and do nothing
    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> nb::Result<(), super::Error> {
        self.busy_res()?;
        match self.hal_i2c.write(addr, bytes) {
            Ok(_) => Ok(()),
            Err(super::Error::NoAcknowledge(source)) => {
                self.send_stop();
                Err(nb::Error::Other(super::Error::NoAcknowledge(source)))
            }
            Err(error) => Err(nb::Error::Other(error)),
        }
    }

    fn finish_transfer_with_result(&mut self, result: Result<(), Error>) {
        self.disable_dma_requests();
        self.disable_error_interrupt_generation();

        if let Err(Error::I2CError(super::Error::NoAcknowledge(_))) = &result {
            self.send_stop();
        }

        self.call_callback_once(result);

        if self.tx.created() {
            self.tx.destroy_transfer();
        }

        if self.rx.created() {
            self.rx.destroy_transfer();
        }
    }
}

impl<I2C, TX_STREAM, const TX_CH: u8> I2CMasterHandleIT
    for I2CMasterDma<I2C, TxDMA<I2C, TX_STREAM, TX_CH>, NoDMA>
where
    I2C: Instance,

    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,
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

                // Wait for BTF
                while self.hal_i2c.i2c.sr1.read().btf().bit_is_clear() {}

                // Generate stop and wait for it
                self.send_stop();
            }
        }
    }

    fn handle_error_interrupt(&mut self) {
        let res = self.hal_i2c.check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::I2CError(e)));
        }
    }
}

impl<I2C, RX_STREAM, const RX_CH: u8> I2CMasterHandleIT
    for I2CMasterDma<I2C, NoDMA, RxDMA<I2C, RX_STREAM, RX_CH>>
where
    I2C: Instance,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
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

                // Clear ACK
                self.hal_i2c.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                // Generate stop and wait for it
                self.send_stop();
            }
        }
    }

    fn handle_error_interrupt(&mut self) {
        let res = self.hal_i2c.check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::I2CError(e)));
        }
    }
}

/// Only for both TX and RX DMA I2c
impl<I2C, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8> I2CMasterHandleIT
    for I2CMasterDma<I2C, TxDMA<I2C, TX_STREAM, TX_CH>, RxDMA<I2C, RX_STREAM, RX_CH>>
where
    I2C: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
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
                if !have_read_after {
                    self.finish_transfer_with_result(Ok(()));
                }

                // Wait for BTF
                while self.hal_i2c.i2c.sr1.read().btf().bit_is_clear() {}

                // If we have prepared Rx Transfer, there are write_read command, generate restart signal
                if have_read_after {
                    // Prepare for reading
                    if let Err(e) = self.prepare_read(self.address, self.rx_len) {
                        self.finish_transfer_with_result(Err(Error::I2CError(e)))
                    }

                    self.rx.rx_transfer.as_mut().unwrap().start(|_| {});
                } else {
                    // Generate stop and wait for it
                    self.send_stop();
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

                // Clear ACK
                self.hal_i2c.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                // Generate stop and wait for it
                self.send_stop();
            }
        }
    }

    fn handle_error_interrupt(&mut self) {
        let res = self.hal_i2c.check_and_clear_error_flags();
        if let Err(e) = res {
            self.finish_transfer_with_result(Err(Error::I2CError(e)));
        }
    }
}

// Write DMA implementations for TX only and TX/RX I2C DMA
impl<I2C, TX_STREAM, const TX_CH: u8, RX_TRANSFER> I2CMasterWriteDMA
    for I2CMasterDma<I2C, TxDMA<I2C, TX_STREAM, TX_CH>, RX_TRANSFER>
where
    I2C: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_TRANSFER: DMATransfer<&'static mut [u8]>,
{
    unsafe fn write_dma(
        &mut self,
        addr: u8,
        bytes: &[u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error> {
        self.busy_res()?;

        // Prepare transfer
        self.enable_dma_requests();
        let static_bytes: &'static [u8] = transmute(bytes);
        self.tx.create_transfer(static_bytes);
        self.callback = callback;

        if let Err(e) = self.prepare_write(addr) {
            // Reset struct on errors
            self.finish_transfer_with_result(Err(Error::I2CError(e)));
            return Err(nb::Error::Other(e));
        }

        // Start DMA processing
        self.tx.tx_transfer.as_mut().unwrap().start(|_| {});

        Ok(())
    }
}

// Write DMA implementations for RX only and TX/RX I2C DMA
impl<I2C, TX_TRANSFER, RX_STREAM, const RX_CH: u8> I2CMasterReadDMA
    for I2CMasterDma<I2C, TX_TRANSFER, RxDMA<I2C, RX_STREAM, RX_CH>>
where
    I2C: Instance,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,

    TX_TRANSFER: DMATransfer<&'static [u8]>,
{
    unsafe fn read_dma(
        &mut self,
        addr: u8,
        buf: &mut [u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error> {
        self.busy_res()?;

        //  If size is small we need to set ACK=0 before cleaning ADDR(reading SR2)
        let buf_len = buf.len();

        self.enable_dma_requests();
        let static_buf: &'static mut [u8] = transmute(buf);
        self.rx.create_transfer(static_buf);
        self.callback = callback;

        if let Err(e) = self.prepare_read(addr, buf_len) {
            // Reset struct on errors
            self.finish_transfer_with_result(Err(Error::I2CError(e)));
            return Err(nb::Error::Other(e));
        }

        // Start DMA processing
        self.rx.rx_transfer.as_mut().unwrap().start(|_| {});

        Ok(())
    }
}

impl<I2C, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8> I2CMasterWriteReadDMA
    for I2CMasterDma<I2C, TxDMA<I2C, TX_STREAM, TX_CH>, RxDMA<I2C, RX_STREAM, RX_CH>>
where
    I2C: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    unsafe fn write_read_dma(
        &mut self,
        addr: u8,
        bytes: &[u8],
        buf: &mut [u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error> {
        self.busy_res()?;

        self.address = addr;
        self.rx_len = buf.len();

        self.enable_dma_requests();
        let static_bytes: &'static [u8] = transmute(bytes);
        self.tx.create_transfer(static_bytes);
        let static_buf: &'static mut [u8] = transmute(buf);
        self.rx.create_transfer(static_buf);
        self.callback = callback;

        if let Err(e) = self.prepare_write(addr) {
            // Reset struct on errors
            self.finish_transfer_with_result(Err(Error::I2CError(e)));
            return Err(nb::Error::Other(e));
        }

        // Start DMA processing
        self.tx.tx_transfer.as_mut().unwrap().start(|_| {});

        Ok(())
    }
}

pub struct Tx<I2C> {
    i2c: PhantomData<I2C>,
}

pub struct Rx<I2C> {
    i2c: PhantomData<I2C>,
}

unsafe impl<I2C: Instance> PeriAddress for Rx<I2C> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*I2C::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}

unsafe impl<I2C: Instance> PeriAddress for Tx<I2C> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*I2C::ptr()).dr as *const _ as u32 }
    }

    type MemSize = u8;
}

unsafe impl<I2C, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory> for Rx<I2C> where
    I2C: DMASet<STREAM, CHANNEL, PeripheralToMemory>
{
}

unsafe impl<I2C, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral> for Tx<I2C> where
    I2C: DMASet<STREAM, CHANNEL, MemoryToPeripheral>
{
}
