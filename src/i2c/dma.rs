use core::{marker::PhantomData, mem::transmute};

use super::{I2c, Instance};
use crate::dma::{
    config::DmaConfig,
    traits::{Channel, DMASet, PeriAddress, Stream},
    ChannelX, MemoryToPeripheral, PeripheralToMemory, Transfer,
};

use nb;

#[non_exhaustive]
pub enum Error {
    TransferError,
}

/// Callback type to notify user code of completion I2C transfers
pub type I2cCompleteCallback = fn(Result<(), Error>);

pub trait I2CMasterWriteDMA {
    /// This function is unsafe because user must ensure that `bytes` will live until `callback` called
    unsafe fn write_dma(
        &mut self,
        addr: u8,
        bytes: &[u8],
        callback: Option<I2cCompleteCallback>,
    ) -> nb::Result<(), super::Error>;
}

impl<I2C: Instance, PINS> I2c<I2C, PINS> {
    /// Creates I2C struct that can work in blocking mode and non-blocking mode using DMA
    pub fn use_dma<TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8>(
        self,
        tx_stream: TX_STREAM,
        rx_stream: RX_STREAM,
    ) -> I2CMasterDma<I2C, PINS, TX_STREAM, TX_CH, RX_STREAM, RX_CH>
    where
        TX_STREAM: Stream,
        RX_STREAM: Stream,
    {
        let tx = Tx { i2c: PhantomData };
        let rx = Rx { i2c: PhantomData };

        I2CMasterDma {
            hal_i2c: self,
            callback: None,

            tx: Some(tx),
            tx_stream: Some(tx_stream),
            tx_transfer: None,

            rx: Some(rx),
            rx_stream: Some(rx_stream),
            rx_transfer: None,
        }
    }
}

pub struct I2CMasterDma<I2C, PINS, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8>
where
    I2C: Instance,
    TX_STREAM: Stream,
    RX_STREAM: Stream,
{
    hal_i2c: I2c<I2C, PINS>,

    callback: Option<I2cCompleteCallback>,

    tx: Option<Tx<I2C>>,
    tx_stream: Option<TX_STREAM>,
    tx_transfer: Option<Transfer<TX_STREAM, TX_CH, Tx<I2C>, MemoryToPeripheral, &'static [u8]>>,

    rx: Option<Rx<I2C>>,
    rx_stream: Option<RX_STREAM>,
    rx_transfer: Option<Transfer<RX_STREAM, RX_CH, Rx<I2C>, PeripheralToMemory, &'static mut [u8]>>,
}

impl<I2C, PINS, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8>
    I2CMasterDma<I2C, PINS, TX_STREAM, TX_CH, RX_STREAM, RX_CH>
where
    I2C: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
{
    fn call_callback_once(&mut self, res: Result<(), Error>) {
        if let Some(c) = self.callback.take() {
            c(res);
        }
    }

    fn create_tx_transfer(&mut self, buf: &'static [u8]) {
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

    fn destroy_tx_transfer(&mut self) {
        assert!(self.tx_transfer.is_some());

        let (str, tx, ..) = self.tx_transfer.take().unwrap().release();
        self.tx = Some(tx);
        self.tx_stream = Some(str);
    }

    fn create_rx_transfer(&mut self, buf: &'static mut [u8]) {
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

    fn destroy_rx_transfer(&mut self) {
        assert!(self.rx_transfer.is_some());

        let (str, tx, ..) = self.rx_transfer.take().unwrap().release();
        self.rx = Some(tx);
        self.rx_stream = Some(str);
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

    fn send_start(&mut self) -> Result<(), super::Error> {
        let i2c = &self.hal_i2c.i2c;
        i2c.cr1.modify(|_, w| w.start().set_bit());

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
        while self.hal_i2c.i2c.cr1.read().stop().bit_is_set() {}
    }

    fn send_address(&mut self, addr: u8) -> Result<(), super::Error> {
        let i2c = &self.hal_i2c.i2c;
        // Set up current address, we're trying to talk to
        i2c.dr.write(|w| unsafe { w.bits(u32::from(addr) << 1) });

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

    /// Handles DMA interrupt.
    /// This method a client must call in DMAx_STREAMy interrupt
    pub fn handle_dma_interrupt(&mut self) {
        if let Some(tx_t) = &mut self.tx_transfer {
            if TX_STREAM::get_fifo_error_flag() {
                tx_t.clear_fifo_error_interrupt();

                return;
            }

            if TX_STREAM::get_transfer_error_flag() {
                tx_t.clear_transfer_error_interrupt();

                self.disable_dma_requests();
                self.call_callback_once(Err(Error::TransferError));
                self.destroy_tx_transfer();

                return;
            }

            if TX_STREAM::get_transfer_complete_flag() {
                tx_t.clear_transfer_complete_interrupt();

                self.disable_dma_requests();
                self.call_callback_once(Ok(()));
                self.destroy_tx_transfer();

                // Wait for BTF
                while self.hal_i2c.i2c.sr1.read().btf().bit_is_clear() {}

                // Generate stop and wait for it
                self.send_stop();

                return;
            }
        }
    }

    /// Reads in blocking mode but if i2c is busy returns `WouldBlock` and do nothing
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> nb::Result<(), super::Error> {
        self.busy_res()?;
        match self.hal_i2c.read(addr, buffer) {
            Ok(_) => Ok(()),
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
            Err(error) => Err(nb::Error::Other(error)),
        }
    }

    /// Write in blocking mode but if i2c is busy returns `WouldBlock` and do nothing
    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> nb::Result<(), super::Error> {
        self.busy_res()?;
        match self.hal_i2c.write(addr, bytes) {
            Ok(_) => Ok(()),
            Err(error) => Err(nb::Error::Other(error)),
        }
    }
}

impl<I2C, PINS, TX_STREAM, const TX_CH: u8, RX_STREAM, const RX_CH: u8> I2CMasterWriteDMA
    for I2CMasterDma<I2C, PINS, TX_STREAM, TX_CH, RX_STREAM, RX_CH>
where
    I2C: Instance,
    TX_STREAM: Stream,
    ChannelX<TX_CH>: Channel,
    Tx<I2C>: DMASet<TX_STREAM, TX_CH, MemoryToPeripheral>,

    RX_STREAM: Stream,
    ChannelX<RX_CH>: Channel,
    Rx<I2C>: DMASet<RX_STREAM, RX_CH, PeripheralToMemory>,
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
        self.create_tx_transfer(static_bytes);
        self.callback = callback;

        // Start
        if let Err(e) = self.send_start() {
            return Err(nb::Error::Other(e));
        }

        // Send address
        if let Err(e) = self.send_address(addr) {
            return Err(nb::Error::Other(e));
        }

        // Clear condition by reading SR2
        self.hal_i2c.i2c.sr2.read();

        // Start DMA processing
        self.tx_transfer.as_mut().unwrap().start(|_| {});

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
