//! QSPI interface
//!
//! This module implements the QuadSPI interface which allows high speed
//! communication with external flash memory.
//!
//! Limitations:
//!     - Interrupts are not supported.
//!     - Status polling mode is not supported.

// Based on work by Unizippro <madas@astrupa.dk> in stm32l4xx-hal.

pub use crate::gpio::alt::QuadSpiBank;
use crate::{
    gpio::{alt::quadspi as alt, PinSpeed, Speed},
    pac::QUADSPI,
    rcc::Enable,
};
pub use alt::{Bank1, Bank2};

pub trait QspiPins {
    const FSEL: bool = false;
    const DFM: bool = false;
    type Pins;
}

impl QspiPins for Bank1
where
    Bank1: QuadSpiBank,
{
    type Pins = (
        alt::Bk1Ncs,
        alt::Bk1Io0,
        alt::Bk1Io1,
        alt::Bk1Io2,
        alt::Bk1Io3,
        alt::Clk,
    );
}

impl QspiPins for Bank2
where
    Bank1: QuadSpiBank,
{
    const FSEL: bool = true;

    type Pins = (
        alt::Bk2Ncs,
        alt::Bk2Io0,
        alt::Bk2Io1,
        alt::Bk2Io2,
        alt::Bk2Io3,
        alt::Clk,
    );
}

pub struct DualFlash;

impl QspiPins for DualFlash {
    const DFM: bool = true;

    type Pins = (
        alt::Bk1Ncs,
        alt::Bk2Ncs,
        alt::Bk1Io0,
        alt::Bk1Io1,
        alt::Bk1Io2,
        alt::Bk1Io3,
        alt::Bk2Io0,
        alt::Bk2Io1,
        alt::Bk2Io2,
        alt::Bk2Io3,
        alt::Clk,
    );
}

pub struct Qspi<BANK: QspiPins> {
    qspi: QUADSPI,
    config: QspiConfig,
    pins: BANK::Pins,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QspiConfig {
    /// This field defines the scaler factor for generating CLK based on the AHB clock
    /// (value+1).
    clock_prescaler: u8,
    /// Number of bytes in Flash memory = 2^[FSIZE+1]
    flash_size: FlashSize,
    address_size: AddressSize,
    /// This bit indicates the level that CLK takes between commands Mode 0(low) / mode 3(high)
    clock_mode: ClockMode,
    /// FIFO threshold level (Activates FTF, QUADSPI_SR[2]) 0-15.
    fifo_threshold: u8,
    sample_shift: SampleShift,
    /// CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must
    /// remain high between commands issued to the Flash memory.
    chip_select_high_time: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FlashSize {
    // actual flash size in bytes is 2^[inner+1]
    inner: u8,
}

impl FlashSize {
    pub const fn new_raw(inner: u8) -> Self {
        Self { inner }
    }

    pub const fn from_megabytes(megabytes: u32) -> Self {
        let pow = 32 - megabytes.leading_zeros();
        Self::new_raw((pow + 18) as u8)
    }
}

impl Default for QspiConfig {
    fn default() -> QspiConfig {
        QspiConfig {
            clock_prescaler: 0,
            flash_size: FlashSize { inner: 0 },
            address_size: AddressSize::Addr24Bit,
            clock_mode: ClockMode::Mode0,
            fifo_threshold: 1,
            sample_shift: SampleShift::HalfACycle,
            chip_select_high_time: 1,
        }
    }
}

impl QspiConfig {
    pub fn clock_prescaler(mut self, clock_prescaler: u8) -> Self {
        self.clock_prescaler = clock_prescaler;
        self
    }

    pub fn flash_size(mut self, flash_size: FlashSize) -> Self {
        self.flash_size = flash_size;
        self
    }

    pub fn address_size(mut self, address_size: AddressSize) -> Self {
        self.address_size = address_size;
        self
    }

    pub fn clock_mode(mut self, clock_mode: ClockMode) -> Self {
        self.clock_mode = clock_mode;
        self
    }

    pub fn fifo_threshold(mut self, fifo_threshold: u8) -> Self {
        self.fifo_threshold = fifo_threshold;
        self
    }

    pub fn sample_shift(mut self, sample_shift: SampleShift) -> Self {
        self.sample_shift = sample_shift;
        self
    }

    pub fn chip_select_high_time(mut self, chip_select_high_time: u8) -> Self {
        self.chip_select_high_time = chip_select_high_time;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum QspiMode {
    SingleChannel = 0b01,
    DualChannel = 0b10,
    QuadChannel = 0b11,
}

impl Default for QspiMode {
    fn default() -> Self {
        QspiMode::SingleChannel
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum AddressSize {
    Addr8Bit = 0b00,
    Addr16Bit = 0b01,
    Addr24Bit = 0b10,
    Addr32Bit = 0b11,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SampleShift {
    None,
    HalfACycle,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClockMode {
    Mode0,
    Mode3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QspiError {
    Busy,
    Address,
    Unknown,
    IllegalArgument,
}

impl Qspi<Bank1> {
    /// Enable the QSPI peripheral with the given configuration.
    pub fn bank1(
        qspi: QUADSPI,
        pins: (
            impl Into<alt::Bk1Ncs>,
            impl Into<alt::Bk1Io0>,
            impl Into<alt::Bk1Io1>,
            impl Into<alt::Bk1Io2>,
            impl Into<alt::Bk1Io3>,
            impl Into<alt::Clk>,
        ),
        config: QspiConfig,
    ) -> Self {
        Self::new(qspi, pins, config)
    }
}

impl Qspi<Bank2> {
    /// Enable the QSPI peripheral with the given configuration.
    pub fn bank2(
        qspi: QUADSPI,
        pins: (
            impl Into<alt::Bk2Ncs>,
            impl Into<alt::Bk2Io0>,
            impl Into<alt::Bk2Io1>,
            impl Into<alt::Bk2Io2>,
            impl Into<alt::Bk2Io3>,
            impl Into<alt::Clk>,
        ),
        config: QspiConfig,
    ) -> Self {
        Self::new(qspi, pins, config)
    }
}

impl<BANK> Qspi<BANK>
where
    BANK: QuadSpiBank
        + QspiPins<
            Pins = (
                BANK::Ncs,
                BANK::Io0,
                BANK::Io1,
                BANK::Io2,
                BANK::Io3,
                alt::Clk,
            ),
        >,
{
    /// Enable the QSPI peripheral with the given configuration.
    pub fn new(
        qspi: QUADSPI,
        pins: (
            impl Into<BANK::Ncs>,
            impl Into<BANK::Io0>,
            impl Into<BANK::Io1>,
            impl Into<BANK::Io2>,
            impl Into<BANK::Io3>,
            impl Into<alt::Clk>,
        ),
        config: QspiConfig,
    ) -> Self {
        // Enable quad SPI in the clocks.
        unsafe {
            QUADSPI::enable_unchecked();
        }

        let pins = (
            pins.0.into().speed(Speed::VeryHigh),
            pins.1.into().speed(Speed::VeryHigh),
            pins.2.into().speed(Speed::VeryHigh),
            pins.3.into().speed(Speed::VeryHigh),
            pins.4.into().speed(Speed::VeryHigh),
            pins.5.into().speed(Speed::VeryHigh),
        );

        // Disable QUADSPI before configuring it.
        qspi.cr().modify(|_, w| w.en().clear_bit());

        // Clear all pending flags.
        qspi.fcr().write(|w| {
            w.ctof().set_bit();
            w.csmf().set_bit();
            w.ctcf().set_bit();
            w.ctef().set_bit()
        });

        let mut unit = Qspi { qspi, config, pins };
        unit.apply_config(config);
        unit
    }
}

impl Qspi<DualFlash> {
    /// Enable the QSPI peripheral with the given configuration.
    pub fn new(
        qspi: QUADSPI,
        pins: (
            impl Into<alt::Bk1Ncs>,
            impl Into<alt::Bk2Ncs>,
            impl Into<alt::Bk1Io0>,
            impl Into<alt::Bk1Io1>,
            impl Into<alt::Bk1Io2>,
            impl Into<alt::Bk1Io3>,
            impl Into<alt::Bk2Io0>,
            impl Into<alt::Bk2Io1>,
            impl Into<alt::Bk2Io2>,
            impl Into<alt::Bk2Io3>,
            impl Into<alt::Clk>,
        ),
        config: QspiConfig,
    ) -> Self {
        // Enable quad SPI in the clocks.
        unsafe {
            QUADSPI::enable_unchecked();
        }

        let pins = (
            pins.0.into().speed(Speed::VeryHigh),
            pins.1.into().speed(Speed::VeryHigh),
            pins.2.into().speed(Speed::VeryHigh),
            pins.3.into().speed(Speed::VeryHigh),
            pins.4.into().speed(Speed::VeryHigh),
            pins.5.into().speed(Speed::VeryHigh),
            pins.6.into().speed(Speed::VeryHigh),
            pins.7.into().speed(Speed::VeryHigh),
            pins.8.into().speed(Speed::VeryHigh),
            pins.9.into().speed(Speed::VeryHigh),
            pins.10.into().speed(Speed::VeryHigh),
        );

        // Disable QUADSPI before configuring it.
        qspi.cr().modify(|_, w| w.en().clear_bit());

        // Clear all pending flags.
        qspi.fcr().write(|w| {
            w.ctof().set_bit();
            w.csmf().set_bit();
            w.ctcf().set_bit();
            w.ctef().set_bit()
        });

        let mut unit = Qspi { qspi, config, pins };
        unit.apply_config(config);
        unit
    }
}

impl<BANK: QspiPins> Qspi<BANK> {
    pub fn is_busy(&self) -> bool {
        self.qspi.sr().read().busy().bit_is_set()
    }

    /// Aborts any ongoing transaction
    /// Note can cause problems if aborting writes to flash satus register
    pub fn abort_transmission(&mut self) {
        self.qspi.cr().modify(|_, w| w.abort().set_bit());
        while self.qspi.sr().read().busy().bit_is_set() {}
    }

    pub fn apply_config(&mut self, config: QspiConfig) {
        if self.qspi.sr().read().busy().bit_is_set() {
            self.abort_transmission();
        }

        self.qspi
            .cr()
            .modify(|_, w| unsafe { w.fthres().bits(config.fifo_threshold) });

        while self.qspi.sr().read().busy().bit_is_set() {}

        self.qspi.cr().modify(|_, w| unsafe {
            w.prescaler().bits(config.clock_prescaler);
            w.sshift()
                .bit(config.sample_shift == SampleShift::HalfACycle);
            w.fsel().bit(BANK::FSEL);
            w.dfm().bit(BANK::DFM)
        });
        while self.is_busy() {}

        // Modify DCR with flash size, CSHT and clock mode
        self.qspi.dcr().modify(|_, w| unsafe {
            w.fsize().bits(config.flash_size.inner);
            w.csht().bits(config.chip_select_high_time);
            w.ckmode().bit(config.clock_mode == ClockMode::Mode3)
        });
        while self.is_busy() {}

        // Enable QSPI
        self.qspi.cr().modify(|_, w| w.en().set_bit());
        while self.is_busy() {}

        self.config = config;
    }

    /// Perform an indirect read operation with the given command.
    pub fn indirect_read(&mut self, command: QspiReadCommand) -> Result<(), QspiError> {
        let buffer = command.data.0;
        if buffer.is_empty() {
            // Illegal to perform an indirect read with no buffer.
            return Err(QspiError::IllegalArgument);
        }

        if self.is_busy() {
            return Err(QspiError::Busy);
        }

        // If double data rate change shift
        if command.double_data_rate {
            self.qspi.cr().modify(|_, w| w.sshift().bit(false));
        }
        while self.is_busy() {}

        // Clear the transfer complete flag.
        self.qspi.fcr().modify(|_, w| w.ctcf().set_bit());

        let dmode: u8 = command.data.1 as u8;
        let mut instruction: u8 = 0;
        let mut imode: u8 = 0;
        let mut admode: u8 = 0;
        let mut adsize: u8 = 0;
        let mut abmode: u8 = 0;
        let mut absize: u8 = 0;

        // Write the length and format of data
        self.qspi
            .dlr()
            .write(|w| unsafe { w.dl().bits(buffer.len() as u32 - 1) });

        // Write instruction mode
        if let Some((inst, mode)) = command.instruction {
            imode = mode as u8;
            instruction = inst;
        }

        // Note Address mode
        if let Some((_, mode)) = command.address {
            admode = mode as u8;
            adsize = self.config.address_size as u8;
        }

        // Write alternate bytes
        if let Some((a_bytes, mode)) = command.alternate_bytes {
            abmode = mode as u8;
            absize = a_bytes.len() as u8 - 1;

            self.qspi.abr().write(|w| {
                let mut reg_byte: u32 = 0;
                for (i, element) in a_bytes.iter().rev().enumerate() {
                    reg_byte |= (*element as u32) << (i * 8);
                }
                unsafe { w.alternate().bits(reg_byte) }
            });
        }

        // Write CCR register with instruction etc.
        self.qspi.ccr().modify(|_, w| unsafe {
            w.fmode().bits(0b01 /* Indirect read */);
            w.admode().bits(admode);
            w.adsize().bits(adsize);
            w.abmode().bits(abmode);
            w.absize().bits(absize);
            w.ddrm().bit(command.double_data_rate);
            w.dcyc().bits(command.dummy_cycles);
            w.dmode().bits(dmode);
            w.imode().bits(imode);
            w.instruction().bits(instruction)
        });

        // Write address, triggers send
        if let Some((addr, _)) = command.address {
            self.qspi.ar().write(|w| unsafe { w.address().bits(addr) });

            // Transfer error
            if self.qspi.sr().read().tef().bit_is_set() {
                return Err(QspiError::Address);
            }
        }

        // Transfer error
        if self.qspi.sr().read().tef().bit_is_set() {
            return Err(QspiError::Unknown);
        }

        // Read data from the buffer
        let mut b = buffer.iter_mut();
        while self.qspi.sr().read().tcf().bit_is_clear() {
            if self.qspi.sr().read().ftf().bit_is_set() {
                if let Some(v) = b.next() {
                    unsafe {
                        *v = core::ptr::read_volatile(self.qspi.dr().as_ptr() as *const u8);
                    }
                } else {
                    // OVERFLOW
                    self.abort_transmission();
                    break;
                }
            }
        }
        // When transfer complete, empty fifo buffer
        while self.qspi.sr().read().flevel().bits() > 0 {
            if let Some(v) = b.next() {
                unsafe {
                    *v = core::ptr::read_volatile(self.qspi.dr().as_ptr() as *const u8);
                }
            } else {
                // OVERFLOW
                self.abort_transmission();
                break;
            }
        }
        // If double data rate set shift back to original and if busy abort.
        if command.double_data_rate {
            if self.is_busy() {
                self.abort_transmission();
            }
            self.qspi.cr().modify(|_, w| {
                w.sshift()
                    .bit(self.config.sample_shift == SampleShift::HalfACycle)
            });
        }
        while self.is_busy() {}
        self.qspi.fcr().write(|w| w.ctcf().set_bit());
        Ok(())
    }

    /// Perform an indirect write with the given command.
    pub fn indirect_write(&mut self, command: QspiWriteCommand) -> Result<(), QspiError> {
        if self.is_busy() {
            return Err(QspiError::Busy);
        }
        // Clear the transfer complete flag.
        self.qspi.fcr().modify(|_, w| w.ctcf().set_bit());

        let mut dmode: u8 = 0;
        let mut instruction: u8 = 0;
        let mut imode: u8 = 0;
        let mut admode: u8 = 0;
        let mut adsize: u8 = 0;
        let mut abmode: u8 = 0;
        let mut absize: u8 = 0;

        // Write the length and format of data
        if let Some((data, mode)) = command.data {
            self.qspi
                .dlr()
                .write(|w| unsafe { w.dl().bits(data.len() as u32 - 1) });
            dmode = mode as u8;
        }

        // Write instruction mode
        if let Some((inst, mode)) = command.instruction {
            imode = mode as u8;
            instruction = inst;
        }

        // Note Address mode
        if let Some((_, mode)) = command.address {
            admode = mode as u8;
            adsize = self.config.address_size as u8;
        }

        // Write alternate bytes
        if let Some((a_bytes, mode)) = command.alternate_bytes {
            abmode = mode as u8;

            absize = a_bytes.len() as u8 - 1;

            self.qspi.abr().write(|w| {
                let mut reg_byte: u32 = 0;
                for (i, element) in a_bytes.iter().rev().enumerate() {
                    reg_byte |= (*element as u32) << (i * 8);
                }
                unsafe { w.alternate().bits(reg_byte) }
            });
        }

        if command.double_data_rate {
            self.qspi.cr().modify(|_, w| w.sshift().bit(false));
        }

        // Write CCR register with instruction etc.
        self.qspi.ccr().modify(|_, w| unsafe {
            w.fmode().bits(0b00 /* Indirect write mode */);
            w.admode().bits(admode);
            w.adsize().bits(adsize);
            w.abmode().bits(abmode);
            w.absize().bits(absize);
            w.ddrm().bit(command.double_data_rate);
            w.dcyc().bits(command.dummy_cycles);
            w.dmode().bits(dmode);
            w.imode().bits(imode);
            w.instruction().bits(instruction)
        });

        // Write address, triggers send
        if let Some((addr, _)) = command.address {
            self.qspi.ar().write(|w| unsafe { w.address().bits(addr) });
        }

        // Transfer error
        if self.qspi.sr().read().tef().bit_is_set() {
            return Err(QspiError::Unknown);
        }

        // Write data to the FIFO
        if let Some((data, _)) = command.data {
            for byte in data {
                while self.qspi.sr().read().ftf().bit_is_clear() {}
                unsafe {
                    core::ptr::write_volatile(self.qspi.dr().as_ptr() as *mut u8, *byte);
                }
            }
        }

        while self.qspi.sr().read().tcf().bit_is_clear() {}

        self.qspi.fcr().write(|w| w.ctcf().set_bit());

        if command.double_data_rate {
            self.qspi.cr().modify(|_, w| {
                w.sshift()
                    .bit(self.config.sample_shift == SampleShift::HalfACycle)
            });
        }
        Ok(())
    }

    /// Put the QSPI peripheral into memory mapped mode. Returns a slice to the
    /// memory mapped region.
    /// Provide the command for the read operation for your flash chip.
    pub fn memory_mapped<'a>(
        &'a mut self,
        command: QspiMemoryMappedConfig,
    ) -> Result<MemoryMapped<'a, BANK>, QspiError> {
        if self.is_busy() {
            return Err(QspiError::Busy);
        }

        // If double data rate change shift
        if command.double_data_rate {
            self.qspi.cr().modify(|_, w| w.sshift().bit(false));
        }
        while self.is_busy() {}

        // Clear the transfer complete flag.
        self.qspi.fcr().modify(|_, w| w.ctcf().set_bit());

        let mut abmode: u8 = 0;
        let mut absize: u8 = 0;
        let mut imode: u8 = 0;
        let mut instruction: u8 = 0;

        if let Some((inst, mode)) = command.instruction {
            imode = mode as u8;
            instruction = inst;
        }

        // Write alternate bytes
        if let Some((a_bytes, mode)) = command.alternate_bytes {
            abmode = mode as u8;
            absize = a_bytes.len() as u8 - 1;

            self.qspi.abr().write(|w| {
                let mut reg_byte: u32 = 0;
                for (i, element) in a_bytes.iter().rev().enumerate() {
                    reg_byte |= (*element as u32) << (i * 8);
                }
                unsafe { w.alternate().bits(reg_byte) }
            });
        }

        self.qspi.ccr().modify(|_, w| unsafe {
            w.fmode().bits(0b11 /* Memory mapped mode */);
            w.admode().bits(command.address_mode as u8);
            w.adsize().bits(self.config.address_size as u8);
            w.abmode().bits(abmode);
            w.absize().bits(absize);
            w.ddrm().bit(command.double_data_rate);
            w.dcyc().bits(command.dummy_cycles);
            w.dmode().bits(command.data_mode as u8);
            w.imode().bits(imode);
            w.instruction().bits(instruction)
        });

        let buffer = unsafe {
            core::slice::from_raw_parts(
                0x9000_0000 as *const u8,
                2usize.pow(self.config.flash_size.inner as u32 + 1),
            )
        };

        Ok(MemoryMapped { qspi: self, buffer })
    }

    pub fn release(self) -> (QUADSPI, BANK::Pins) {
        (self.qspi, self.pins)
    }
}

/// This struct is used to configure a write command for the QSPI peripheral.
/// Specify None for any phase to skip it in the transaction.
/// Each phase requires a mode to be specified.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct QspiWriteCommand<'a> {
    instruction: Option<(u8, QspiMode)>,
    address: Option<(u32, QspiMode)>,
    alternate_bytes: Option<(&'a [u8], QspiMode)>,
    dummy_cycles: u8,
    data: Option<(&'a [u8], QspiMode)>,
    double_data_rate: bool,
}

impl<'a> QspiWriteCommand<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instruction(mut self, instruction: u8, instruction_mode: QspiMode) -> Self {
        self.instruction = Some((instruction, instruction_mode));
        self
    }

    pub fn address(mut self, address: u32, address_mode: QspiMode) -> Self {
        self.address = Some((address, address_mode));
        self
    }

    pub fn alternate_bytes(
        mut self,
        alternate_bytes: &'a [u8],
        alternate_bytes_mode: QspiMode,
    ) -> Self {
        self.alternate_bytes = Some((alternate_bytes, alternate_bytes_mode));
        self
    }

    pub fn dummy_cycles(mut self, dummy_cycles: u8) -> Self {
        self.dummy_cycles = dummy_cycles;
        self
    }

    pub fn data(mut self, data: &'a [u8], data_mode: QspiMode) -> Self {
        self.data = Some((data, data_mode));
        self
    }

    pub fn double_data_rate(mut self, double_data_rate: bool) -> Self {
        self.double_data_rate = double_data_rate;
        self
    }
}

/// QSPI Memory mapped mode configuration
/// The configuration is used by the QSPI peripheral to access the external memory.
/// Specify the read command for your external memory.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct QspiMemoryMappedConfig<'a> {
    instruction: Option<(u8, QspiMode)>,
    address_mode: QspiMode,
    alternate_bytes: Option<(&'a [u8], QspiMode)>,
    dummy_cycles: u8,
    data_mode: QspiMode,
    double_data_rate: bool,
}

impl<'a> QspiMemoryMappedConfig<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instruction(mut self, instruction: u8, instruction_mode: QspiMode) -> Self {
        self.instruction = Some((instruction, instruction_mode));
        self
    }

    pub fn address_mode(mut self, address_mode: QspiMode) -> Self {
        self.address_mode = address_mode;
        self
    }

    pub fn alternate_bytes(
        mut self,
        alternate_bytes: &'a [u8],
        alternate_bytes_mode: QspiMode,
    ) -> Self {
        self.alternate_bytes = Some((alternate_bytes, alternate_bytes_mode));
        self
    }

    pub fn dummy_cycles(mut self, dummy_cycles: u8) -> Self {
        self.dummy_cycles = dummy_cycles;
        self
    }

    pub fn data_mode(mut self, data_mode: QspiMode) -> Self {
        self.data_mode = data_mode;
        self
    }

    pub fn double_data_rate(mut self, double_data_rate: bool) -> Self {
        self.double_data_rate = double_data_rate;
        self
    }
}

/// This struct is used to configure a read command for the QSPI peripheral.
/// Specify None for any phase to skip it in the transaction.
/// Each phase requires a mode to be specified.
#[derive(Debug, PartialEq)]
pub struct QspiReadCommand<'a> {
    instruction: Option<(u8, QspiMode)>,
    address: Option<(u32, QspiMode)>,
    alternate_bytes: Option<(&'a [u8], QspiMode)>,
    dummy_cycles: u8,
    data: (&'a mut [u8], QspiMode),
    double_data_rate: bool,
}

impl<'a> QspiReadCommand<'a> {
    pub fn new(data: &'a mut [u8], data_mode: QspiMode) -> Self {
        Self {
            instruction: None,
            address: None,
            alternate_bytes: None,
            dummy_cycles: 0,
            data: (data, data_mode),
            double_data_rate: false,
        }
    }

    pub fn instruction(mut self, instruction: u8, instruction_mode: QspiMode) -> Self {
        self.instruction = Some((instruction, instruction_mode));
        self
    }

    pub fn address(mut self, address: u32, address_mode: QspiMode) -> Self {
        self.address = Some((address, address_mode));
        self
    }

    pub fn alternate_bytes(
        mut self,
        alternate_bytes: &'a [u8],
        alternate_bytes_mode: QspiMode,
    ) -> Self {
        self.alternate_bytes = Some((alternate_bytes, alternate_bytes_mode));
        self
    }

    pub fn dummy_cycles(mut self, dummy_cycles: u8) -> Self {
        self.dummy_cycles = dummy_cycles;
        self
    }

    pub fn data(mut self, data: &'a mut [u8], data_mode: QspiMode) -> Self {
        self.data = (data, data_mode);
        self
    }

    pub fn double_data_rate(mut self, double_data_rate: bool) -> Self {
        self.double_data_rate = double_data_rate;
        self
    }
}

pub struct MemoryMapped<'a, PINS: QspiPins + 'static> {
    qspi: &'a mut Qspi<PINS>,
    buffer: &'a [u8],
}

impl<'a, PINS: QspiPins> MemoryMapped<'a, PINS> {
    pub fn buffer(&self) -> &[u8] {
        self.buffer
    }
}

impl<'a, PINS: QspiPins> Drop for MemoryMapped<'a, PINS> {
    fn drop(&mut self) {
        self.qspi.abort_transmission();
    }
}
