//! QUADSPI driver for the STM32F7. Supports INDIRECT mode only, using DMA or polling I/O.

use as_slice::AsSlice;
use core::convert::TryInto;
use core::marker::PhantomData;
use core::ops::Deref;
use core::pin::Pin;

use crate::dma;
use crate::pac::{QUADSPI, RCC};
use crate::rcc::Enable;
use crate::state;

/// The QSPI driver interface.
pub struct Qspi {
    /// QSPI peripheral registers.
    qspi: QUADSPI,
    /// Address size for all transactions.
    adsize: u8,
}

/// QSPI transaction description. Note that "advanced" settings like DDRM, DHHC,
/// SIOO, and the use of alternate bytes are not supported at the moment.
#[derive(Clone)]
pub struct QspiTransaction {
    pub iwidth: u8,
    pub awidth: u8,
    pub dwidth: u8,
    pub instruction: u8,
    pub address: Option<u32>,
    pub dummy: u8,
    pub data_len: Option<usize>,
}

/// QSPI errors.
#[derive(Debug)]
pub enum Error {
    /// Bad input parameters.
    BadParam,
}

/// QSPI transactions contain configurable instruction, address, and data fields.
/// Use these constants for the `*width` fields in `QspiTransaction`.
pub struct QspiWidth;

#[allow(dead_code)]
impl QspiWidth {
    pub const NONE: u8 = 0b00;
    pub const SING: u8 = 0b01;
    pub const DUAL: u8 = 0b10;
    pub const QUAD: u8 = 0b11;
}

/// QSPI functional mode. Only `INDIRECT_READ` and `INDIRECT_WRITE` are
/// supported at the moment.
struct QspiMode;

#[allow(dead_code)]
impl QspiMode {
    pub const INDIRECT_WRITE: u8 = 0b00;
    pub const INDIRECT_READ: u8 = 0b01;
    pub const AUTO_POLLING: u8 = 0b10;
    pub const MEMORY_MAPPED: u8 = 0b11;
}

impl Qspi {
    /// Initialize and configure the QSPI flash driver.
    /// - `size` is log2(flash size in bytes), e.g. 16 MB = 24.
    /// - `adsize` is the number of bytes needed to specify the address (1, 2, 3, or 4).
    pub fn new(_rcc: &mut RCC, qspi: QUADSPI, size: u8, mut adsize: u8) -> Self {
        assert!((1..=4).contains(&adsize));
        adsize -= 1;

        // Enable QUADSPI in RCC
        unsafe { QUADSPI::enable_unchecked() };

        // Configure QSPI
        unsafe {
            // Single flash mode with a QSPI clock prescaler of 2 (216 / 2 = 108 MHz), FIFO
            // threshold only matters for DMA and is set to 4 to allow word sized DMA requests
            qspi.cr
                .write_with_zero(|w| w.prescaler().bits(1).fthres().bits(3).en().set_bit());

            // Set the device size
            qspi.dcr.write_with_zero(|w| w.fsize().bits(size - 1));
        }

        Qspi { qspi, adsize }
    }

    /// DMA read. Wrapper around the HAL DMA driver. Performs QSPI register programming, creates a
    /// DMA transfer from peripheral to memory, and starts the transfer. Caller can use the DMA
    /// `wait` API to block until the transfer is complete.
    pub fn read_all<B>(
        &mut self,
        data: Pin<B>,
        transaction: QspiTransaction,
        dma: &dma::Handle<<RxTx<QUADSPI> as dma::Target>::Instance, state::Enabled>,
        stream: <RxTx<QUADSPI> as dma::Target>::Stream,
    ) -> Result<dma::Transfer<RxTx<QUADSPI>, B, dma::Started>, Error>
    where
        B: Deref + 'static,
        B::Target: AsSlice<Element = u8>,
    {
        // Only use DMA with data, for command only use `polling_read`
        match transaction.data_len {
            Some(data_len) => {
                assert!(
                    (data_len as u32) % 4 == 0,
                    "DMA transfer must be word aligned."
                );

                // Setup the transaction registers
                self.setup_transaction(QspiMode::INDIRECT_READ, &transaction);

                // Setup DMA transfer
                let rx_token = RxTx(PhantomData);
                let rx_transfer = unsafe {
                    dma::Transfer::new(
                        dma,
                        stream,
                        data,
                        rx_token,
                        self.dr_address(),
                        dma::Direction::PeripheralToMemory,
                    )
                };

                let rx_transfer = rx_transfer.start(dma);

                // Set DMA bit since we are using it
                self.qspi.cr.modify(|_, w| w.dmaen().set_bit());

                Ok(rx_transfer)
            }
            None => Err(Error::BadParam),
        }
    }

    /// DMA write. Wrapper around the HAL DMA driver. Performs QSPI register programming, creates a
    /// DMA transfer from memory to peripheral, and starts the transfer. Caller can use the DMA
    /// `wait` API to block until the transfer is complete.
    pub fn write_all<B>(
        &mut self,
        data: Pin<B>,
        transaction: QspiTransaction,
        dma: &dma::Handle<<RxTx<QUADSPI> as dma::Target>::Instance, state::Enabled>,
        stream: <RxTx<QUADSPI> as dma::Target>::Stream,
    ) -> Result<dma::Transfer<RxTx<QUADSPI>, B, dma::Started>, Error>
    where
        B: Deref + 'static,
        B::Target: AsSlice<Element = u8>,
    {
        // Only use DMA with data, for command only use `polling_write`
        match transaction.data_len {
            Some(data_len) => {
                assert!(
                    (data_len as u32) % 4 == 0,
                    "DMA transfer must be word aligned."
                );

                // Setup the transaction registers
                self.setup_transaction(QspiMode::INDIRECT_WRITE, &transaction);

                // Setup DMA transfer
                let tx_token = RxTx(PhantomData);
                let tx_transfer = unsafe {
                    dma::Transfer::new(
                        dma,
                        stream,
                        data,
                        tx_token,
                        self.dr_address(),
                        dma::Direction::MemoryToPeripheral,
                    )
                };

                let tx_transfer = tx_transfer.start(dma);

                // Set DMA bit since we are using it
                self.qspi.cr.modify(|_, w| w.dmaen().set_bit());

                Ok(tx_transfer)
            }
            None => Err(Error::BadParam),
        }
    }

    /// Polling indirect read. Can also be used to perform transactions with no data.
    pub fn read(&mut self, buf: &mut [u8], transaction: QspiTransaction) -> Result<(), Error> {
        // Clear DMA bit since we are not using it
        self.qspi.cr.modify(|_, w| w.dmaen().clear_bit());

        // Setup the transaction registers
        self.setup_transaction(QspiMode::INDIRECT_READ, &transaction);

        // If the transaction has data, read it word-by-word from the data register
        if let Some(len) = transaction.data_len {
            let mut idx: usize = 0;
            while idx < len {
                // Check if there are bytes in the FIFO
                let num_bytes = self.qspi.sr.read().flevel().bits();
                if num_bytes > 0 {
                    // Read a word
                    let word = self.qspi.dr.read().data().bits();

                    // Unpack the word
                    let num_unpack = if num_bytes >= 4 { 4 } else { num_bytes };
                    for i in 0..num_unpack {
                        buf[idx] = ((word & (0xFF << (i * 8))) >> (i * 8)).try_into().unwrap();
                        idx += 1;
                    }
                }
            }
        }

        Ok(())
    }

    /// Polling indirect write.
    pub fn write(&mut self, buf: &[u8], transaction: QspiTransaction) -> Result<(), Error> {
        // Clear DMA bit since we are not using it
        self.qspi.cr.modify(|_, w| w.dmaen().clear_bit());

        // Setup the transaction registers
        self.setup_transaction(QspiMode::INDIRECT_WRITE, &transaction);

        // If the transaction has data, write it word-by-word to the data register
        if let Some(len) = transaction.data_len {
            let mut idx: usize = 0;
            while idx < len {
                // Check if the FIFO is empty
                let num_bytes = self.qspi.sr.read().flevel().bits();
                if num_bytes == 0 {
                    // Pack the word
                    let mut word: u32 = 0;
                    let num_pack = if (len - idx) >= 4 { 4 } else { len - idx };
                    for i in 0..num_pack {
                        word |= (buf[idx] as u32) << (i * 8);
                        idx += 1;
                    }

                    // Write a word
                    unsafe {
                        self.qspi.dr.write(|w| w.data().bits(word));
                    }
                }
            }
        }

        Ok(())
    }

    /// Map from QspiTransaction to QSPI registers.
    fn setup_transaction(&mut self, fmode: u8, transaction: &QspiTransaction) {
        unsafe {
            // Clear any prior status flags
            self.qspi.fcr.write(|w| w.bits(0x1B));

            // Update data length, if applicable
            if let Some(len) = transaction.data_len {
                self.qspi.dlr.write(|w| w.bits(len as u32 - 1));
            }

            // Update CCR register with metadata
            self.qspi.ccr.write_with_zero(|w| {
                w.fmode().bits(fmode);
                w.imode().bits(transaction.iwidth);
                w.admode().bits(transaction.awidth);
                w.dmode().bits(transaction.dwidth);
                w.adsize().bits(self.adsize);
                w.abmode().bits(QspiWidth::NONE);
                w.dcyc().bits(transaction.dummy);
                w.instruction().bits(transaction.instruction)
            });

            // Update address register, if applicable
            if let Some(addr) = transaction.address {
                self.qspi.ar.write(|w| w.bits(addr));
            }
        }
    }

    /// Get data register address.
    fn dr_address(&self) -> u32 {
        &self.qspi.dr as *const _ as _
    }
}

/// Token used for DMA transfers.
pub struct RxTx<I>(PhantomData<I>);
