//! Quad Serial Peripheral Interface (QSPI) bus

use crate::gpio::{
    gpioa::{PA6, PA7},
    gpiob::{PB0, PB1, PB10, PB11},
    gpioe::{PE10, PE11, PE12, PE13, PE14, PE15},
};

#[cfg(not(any(feature = "stm32l475")))]
use crate::gpio::{
    gpioa::{PA2, PA3},
    gpiod::{PD3, PD4, PD5, PD6, PD7},
};

#[cfg(any(
    feature = "stm32l476",
    feature = "stm32l486",
    feature = "stm32l496",
    feature = "stm32l4a6"
))]
use crate::gpio::{
    gpioc::{PC1, PC2, PC4, PC5},
    gpiof::{PF6, PF7, PF8, PF9},
};

use crate::gpio::{Alternate, PushPull, Speed};
use crate::rcc::{Enable, AHB3};
use crate::stm32::QUADSPI;
use core::ptr;

#[doc(hidden)]
mod private {
    pub trait Sealed {}
}

/// CLK pin. This trait is sealed and cannot be implemented.
pub trait ClkPin<QSPI>: private::Sealed {
    fn set_speed(self, speed: Speed) -> Self;
}
/// nCS pin. This trait is sealed and cannot be implemented.
pub trait NCSPin<QSPI>: private::Sealed {
    fn set_speed(self, speed: Speed) -> Self;
}
/// IO0 pin. This trait is sealed and cannot be implemented.
pub trait IO0Pin<QSPI>: private::Sealed {
    fn set_speed(self, speed: Speed) -> Self;
}
/// IO1 pin. This trait is sealed and cannot be implemented.
pub trait IO1Pin<QSPI>: private::Sealed {
    fn set_speed(self, speed: Speed) -> Self;
}
/// IO2 pin. This trait is sealed and cannot be implemented.
pub trait IO2Pin<QSPI>: private::Sealed {
    fn set_speed(self, speed: Speed) -> Self;
}
/// IO3 pin. This trait is sealed and cannot be implemented.
pub trait IO3Pin<QSPI>: private::Sealed {
    fn set_speed(self, speed: Speed) -> Self;
}

macro_rules! pins {
    ($qspi:ident, $af:literal, CLK: [$($clk:ident),*], nCS: [$($ncs:ident),*],
        IO0: [$($io0:ident),*], IO1: [$($io1:ident),*], IO2: [$($io2:ident),*],
        IO3: [$($io3:ident),*]) => {
        $(
            impl private::Sealed for $clk<Alternate<PushPull, $af>> {}
            impl ClkPin<$qspi> for $clk<Alternate<PushPull, $af>> {
                fn set_speed(self, speed: Speed) -> Self{
                    self.set_speed(speed)
                }
            }
        )*
        $(
            impl private::Sealed for $ncs<Alternate<PushPull, $af>> {}
            impl NCSPin<$qspi> for $ncs<Alternate<PushPull, $af>> {
                fn set_speed(self, speed: Speed) -> Self{
                    self.set_speed(speed)
                }
            }
        )*
        $(
            impl private::Sealed for $io0<Alternate<PushPull, $af>> {}
            impl IO0Pin<$qspi> for $io0<Alternate<PushPull, $af>> {
                fn set_speed(self, speed: Speed) -> Self{
                    self.set_speed(speed)
                }
            }
        )*
        $(
            impl private::Sealed for $io1<Alternate<PushPull, $af>> {}
            impl IO1Pin<$qspi> for $io1<Alternate<PushPull, $af>> {
                fn set_speed(self, speed: Speed) -> Self{
                    self.set_speed(speed)
                }
            }
        )*
        $(
            impl private::Sealed for $io2<Alternate<PushPull, $af>> {}
            impl IO2Pin<$qspi> for $io2<Alternate<PushPull, $af>> {
                fn set_speed(self, speed: Speed) -> Self{
                    self.set_speed(speed)
                }
            }
        )*
        $(
            impl private::Sealed for $io3<Alternate<PushPull, $af>> {}
            impl IO3Pin<$qspi> for $io3<Alternate<PushPull, $af>> {
                fn set_speed(self, speed: Speed) -> Self{
                    self.set_speed(speed)
                }
            }
        )*
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum QspiMode {
    SingleChannel = 0b01,
    DualChannel = 0b10,
    QuadChannel = 0b11,
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QspiConfig {
    /// This field defines the scaler factor for generating CLK based on the AHB clock
    /// (value+1).
    clock_prescaler: u8,
    /// Number of bytes in Flash memory = 2^[FSIZE+1]
    flash_size: u8,
    address_size: AddressSize,
    /// This bit indicates the level that CLK takes between commands Mode 0(low) / mode 3(high)
    clock_mode: ClockMode,
    /// FIFO threshold level (Activates FTF, QUADSPI_SR[2]) 0-15.
    fifo_threshold: u8,
    sample_shift: SampleShift,
    /// CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must
    /// remain high between commands issued to the Flash memory.
    chip_select_high_time: u8,
    qpi_mode: bool,
}

impl Default for QspiConfig {
    fn default() -> QspiConfig {
        QspiConfig {
            clock_prescaler: 0,
            flash_size: 22, // 8MB // 26 = 128MB
            address_size: AddressSize::Addr24Bit,
            clock_mode: ClockMode::Mode0,
            fifo_threshold: 1,
            sample_shift: SampleShift::HalfACycle,
            chip_select_high_time: 1,
            qpi_mode: false,
        }
    }
}

impl QspiConfig {
    pub fn clock_prescaler(mut self, clk_pre: u8) -> Self {
        self.clock_prescaler = clk_pre;
        self
    }

    pub fn flash_size(mut self, fl_size: u8) -> Self {
        self.flash_size = fl_size;
        self
    }

    pub fn address_size(mut self, add_size: AddressSize) -> Self {
        self.address_size = add_size;
        self
    }

    pub fn clock_mode(mut self, clk_mode: ClockMode) -> Self {
        self.clock_mode = clk_mode;
        self
    }

    pub fn fifo_threshold(mut self, fifo_thres: u8) -> Self {
        self.fifo_threshold = fifo_thres;
        self
    }

    pub fn sample_shift(mut self, shift: SampleShift) -> Self {
        self.sample_shift = shift;
        self
    }

    pub fn chip_select_high_time(mut self, csht: u8) -> Self {
        self.chip_select_high_time = csht;
        self
    }

    pub fn qpi_mode(mut self, qpi: bool) -> Self {
        self.qpi_mode = qpi;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QspiWriteCommand<'a> {
    pub instruction: Option<(u8, QspiMode)>,
    pub address: Option<(u32, QspiMode)>,
    pub alternative_bytes: Option<(&'a [u8], QspiMode)>,
    pub dummy_cycles: u8,
    pub data: Option<(&'a [u8], QspiMode)>,
    pub double_data_rate: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QspiReadCommand<'a> {
    pub instruction: Option<(u8, QspiMode)>,
    pub address: Option<(u32, QspiMode)>,
    pub alternative_bytes: Option<(&'a [u8], QspiMode)>,
    pub dummy_cycles: u8,
    pub data_mode: QspiMode,
    pub receive_length: u32,
    pub double_data_rate: bool,
}

impl<'a> QspiWriteCommand<'a> {
    pub fn address(self, addr: u32, mode: QspiMode) -> Self {
        QspiWriteCommand {
            address: Some((addr, mode)),
            ..self
        }
    }

    pub fn alternative_bytes(self, bytes: &'a [u8], mode: QspiMode) -> Self {
        QspiWriteCommand {
            alternative_bytes: Some((bytes, mode)),
            ..self
        }
    }

    pub fn dummy_cycles(self, n: u8) -> Self {
        QspiWriteCommand {
            dummy_cycles: n,
            ..self
        }
    }

    pub fn data(self, bytes: &'a [u8], mode: QspiMode) -> Self {
        QspiWriteCommand {
            data: Some((bytes, mode)),
            ..self
        }
    }
}

impl<'a> QspiReadCommand<'a> {
    pub fn address(self, addr: u32, mode: QspiMode) -> Self {
        QspiReadCommand {
            address: Some((addr, mode)),
            ..self
        }
    }

    pub fn alternative_bytes(self, bytes: &'a [u8], mode: QspiMode) -> Self {
        QspiReadCommand {
            alternative_bytes: Some((bytes, mode)),
            ..self
        }
    }

    pub fn dummy_cycles(self, n: u8) -> Self {
        QspiReadCommand {
            dummy_cycles: n,
            ..self
        }
    }

    pub fn receive_length(self, length: u32) -> Self {
        QspiReadCommand {
            receive_length: length,
            ..self
        }
    }
}

pub struct Qspi<PINS> {
    qspi: QUADSPI,
    _pins: PINS,
    config: QspiConfig,
}

impl<CLK, NCS, IO0, IO1, IO2, IO3> Qspi<(CLK, NCS, IO0, IO1, IO2, IO3)> {
    pub fn new(
        qspi: QUADSPI,
        pins: (CLK, NCS, IO0, IO1, IO2, IO3),
        ahb3: &mut AHB3,
        config: QspiConfig,
    ) -> Self
    where
        CLK: ClkPin<QUADSPI>,
        NCS: NCSPin<QUADSPI>,
        IO0: IO0Pin<QUADSPI>,
        IO1: IO1Pin<QUADSPI>,
        IO2: IO2Pin<QUADSPI>,
        IO3: IO3Pin<QUADSPI>,
    {
        // Enable quad SPI in the clocks.
        QUADSPI::enable(ahb3);

        // Disable QUADSPI before configuring it.
        qspi.cr.modify(|_, w| w.en().clear_bit());

        // Clear all pending flags.
        qspi.fcr.write(|w| {
            w.ctof()
                .set_bit()
                .csmf()
                .set_bit()
                .ctcf()
                .set_bit()
                .ctef()
                .set_bit()
        });

        // Set gpio speed
        let high_speed_pins = (
            pins.0.set_speed(Speed::VeryHigh),
            pins.1.set_speed(Speed::VeryHigh),
            pins.2.set_speed(Speed::VeryHigh),
            pins.3.set_speed(Speed::VeryHigh),
            pins.4.set_speed(Speed::VeryHigh),
            pins.5.set_speed(Speed::VeryHigh),
        );

        let mut unit = Qspi {
            qspi,
            _pins: high_speed_pins,
            config,
        };
        unit.apply_config(config);
        unit
    }

    pub fn is_busy(&self) -> bool {
        self.qspi.sr.read().busy().bit_is_set()
    }

    /// Aborts any ongoing transaction
    /// Note can cause problems if aborting writes to flash satus register
    pub fn abort_transmission(&self) {
        self.qspi.cr.modify(|_, w| w.abort().set_bit());
        while self.qspi.sr.read().busy().bit_is_set() {}
    }

    pub fn get_config(&self) -> QspiConfig {
        self.config
    }

    pub fn apply_config(&mut self, config: QspiConfig) {
        if self.qspi.sr.read().busy().bit_is_set() {
            self.abort_transmission();
        }

        self.qspi
            .cr
            .modify(|_, w| unsafe { w.fthres().bits(config.fifo_threshold as u8) });

        while self.qspi.sr.read().busy().bit_is_set() {}

        // Modify the prescaler and select flash bank 2 - flash bank 1 is currently unsupported.
        self.qspi.cr.modify(|_, w| unsafe {
            w.prescaler()
                .bits(config.clock_prescaler as u8)
                .sshift()
                .bit(config.sample_shift == SampleShift::HalfACycle)
        });
        while self.is_busy() {}

        // Modify DCR with flash size, CSHT and clock mode
        self.qspi.dcr.modify(|_, w| unsafe {
            w.fsize()
                .bits(config.flash_size as u8)
                .csht()
                .bits(config.chip_select_high_time as u8)
                .ckmode()
                .bit(config.clock_mode == ClockMode::Mode3)
        });
        while self.is_busy() {}

        // Enable QSPI
        self.qspi.cr.modify(|_, w| w.en().set_bit());
        while self.is_busy() {}

        self.config = config;
    }

    pub fn transfer(&self, command: QspiReadCommand, buffer: &mut [u8]) -> Result<(), QspiError> {
        if self.is_busy() {
            return Err(QspiError::Busy);
        }

        // If double data rate change shift
        if command.double_data_rate {
            self.qspi.cr.modify(|_, w| w.sshift().bit(false));
        }
        while self.is_busy() {}

        // Clear the transfer complete flag.
        self.qspi.fcr.modify(|_, w| w.ctcf().set_bit());

        let mut dmode: u8 = 0;
        let mut instruction: u8 = 0;
        let mut imode: u8 = 0;
        let mut admode: u8 = 0;
        let mut adsize: u8 = 0;
        let mut abmode: u8 = 0;
        let mut absize: u8 = 0;

        // Write the length and format of data
        if command.receive_length > 0 {
            self.qspi
                .dlr
                .write(|w| unsafe { w.dl().bits(command.receive_length as u32 - 1) });
            if self.config.qpi_mode {
                dmode = QspiMode::QuadChannel as u8;
            } else {
                dmode = command.data_mode as u8;
            }
        }

        // Write instruction mode
        if let Some((inst, mode)) = command.instruction {
            if self.config.qpi_mode {
                imode = QspiMode::QuadChannel as u8;
            } else {
                imode = mode as u8;
            }
            instruction = inst;
        }

        // Note Address mode
        if let Some((_, mode)) = command.address {
            if self.config.qpi_mode {
                admode = QspiMode::QuadChannel as u8;
            } else {
                admode = mode as u8;
            }
            adsize = self.config.address_size as u8;
        }

        // Write Alternative bytes
        if let Some((a_bytes, mode)) = command.alternative_bytes {
            if self.config.qpi_mode {
                abmode = QspiMode::QuadChannel as u8;
            } else {
                abmode = mode as u8;
            }

            absize = a_bytes.len() as u8 - 1;

            self.qspi.abr.write(|w| {
                let mut reg_byte: u32 = 0;
                for (i, element) in a_bytes.iter().rev().enumerate() {
                    reg_byte |= (*element as u32) << (i * 8);
                }
                unsafe { w.alternate().bits(reg_byte) }
            });
        }

        // Write CCR register with instruction etc.
        self.qspi.ccr.modify(|_, w| unsafe {
            w.fmode()
                .bits(0b01)
                .admode()
                .bits(admode)
                .adsize()
                .bits(adsize)
                .abmode()
                .bits(abmode)
                .absize()
                .bits(absize)
                .ddrm()
                .bit(command.double_data_rate)
                .dcyc()
                .bits(command.dummy_cycles)
                .dmode()
                .bits(dmode)
                .imode()
                .bits(imode)
                .instruction()
                .bits(instruction)
        });

        // Write address, triggers send
        if let Some((addr, _)) = command.address {
            self.qspi.ar.write(|w| unsafe { w.address().bits(addr) });

            // Transfer error
            if self.qspi.sr.read().tef().bit_is_set() {
                return Err(QspiError::Address);
            }
        }

        // Transfer error
        if self.qspi.sr.read().tef().bit_is_set() {
            return Err(QspiError::Unknown);
        }

        // Read data from the buffer
        let mut b = buffer.iter_mut();
        while self.qspi.sr.read().tcf().bit_is_clear() {
            if self.qspi.sr.read().ftf().bit_is_set() {
                if let Some(v) = b.next() {
                    unsafe {
                        *v = ptr::read_volatile(&self.qspi.dr as *const _ as *const u8);
                    }
                } else {
                    // OVERFLOW
                }
            }
        }
        // When transfer complete, empty fifo buffer
        while self.qspi.sr.read().flevel().bits() > 0 {
            if let Some(v) = b.next() {
                unsafe {
                    *v = ptr::read_volatile(&self.qspi.dr as *const _ as *const u8);
                }
            } else {
                // OVERFLOW
            }
        }
        // If double data rate set shift back to original and if busy abort.
        if command.double_data_rate {
            if self.is_busy() {
                self.abort_transmission();
            }
            self.qspi.cr.modify(|_, w| {
                w.sshift()
                    .bit(self.config.sample_shift == SampleShift::HalfACycle)
            });
        }
        while self.is_busy() {}
        self.qspi.fcr.write(|w| w.ctcf().set_bit());
        Ok(())
    }

    pub fn write(&self, command: QspiWriteCommand) -> Result<(), QspiError> {
        if self.is_busy() {
            return Err(QspiError::Busy);
        }
        // Clear the transfer complete flag.
        self.qspi.fcr.modify(|_, w| w.ctcf().set_bit());

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
                .dlr
                .write(|w| unsafe { w.dl().bits(data.len() as u32 - 1) });
            if self.config.qpi_mode {
                dmode = QspiMode::QuadChannel as u8;
            } else {
                dmode = mode as u8;
            }
        }

        // Write instruction mode
        if let Some((inst, mode)) = command.instruction {
            if self.config.qpi_mode {
                imode = QspiMode::QuadChannel as u8;
            } else {
                imode = mode as u8;
            }
            instruction = inst;
        }

        // Note Address mode
        if let Some((_, mode)) = command.address {
            if self.config.qpi_mode {
                admode = QspiMode::QuadChannel as u8;
            } else {
                admode = mode as u8;
            }
            adsize = self.config.address_size as u8;
        }

        // Write Alternative bytes
        if let Some((a_bytes, mode)) = command.alternative_bytes {
            if self.config.qpi_mode {
                abmode = QspiMode::QuadChannel as u8;
            } else {
                abmode = mode as u8;
            }

            absize = a_bytes.len() as u8 - 1;

            self.qspi.abr.write(|w| {
                let mut reg_byte: u32 = 0;
                for (i, element) in a_bytes.iter().rev().enumerate() {
                    reg_byte |= (*element as u32) << (i * 8);
                }
                unsafe { w.alternate().bits(reg_byte) }
            });
        }

        if command.double_data_rate {
            self.qspi.cr.modify(|_, w| w.sshift().bit(false));
        }

        // Write CCR register with instruction etc.
        self.qspi.ccr.modify(|_, w| unsafe {
            w.fmode()
                .bits(0b00)
                .admode()
                .bits(admode)
                .adsize()
                .bits(adsize)
                .abmode()
                .bits(abmode)
                .absize()
                .bits(absize)
                .ddrm()
                .bit(command.double_data_rate)
                .dcyc()
                .bits(command.dummy_cycles)
                .dmode()
                .bits(dmode)
                .imode()
                .bits(imode)
                .instruction()
                .bits(instruction)
        });

        // Write address, triggers send
        if let Some((addr, _)) = command.address {
            self.qspi.ar.write(|w| unsafe { w.address().bits(addr) });
        }

        // Transfer error
        if self.qspi.sr.read().tef().bit_is_set() {
            return Err(QspiError::Unknown);
        }

        // Write data to the FIFO
        if let Some((data, _)) = command.data {
            for byte in data {
                while self.qspi.sr.read().ftf().bit_is_clear() {}
                unsafe {
                    ptr::write_volatile(&self.qspi.dr as *const _ as *mut u8, *byte);
                }
            }
        }

        while self.qspi.sr.read().tcf().bit_is_clear() {}

        self.qspi.fcr.write(|w| w.ctcf().set_bit());

        if self.is_busy() {}

        if command.double_data_rate {
            self.qspi.cr.modify(|_, w| {
                w.sshift()
                    .bit(self.config.sample_shift == SampleShift::HalfACycle)
            });
        }
        Ok(())
    }
}

pins!(
    QUADSPI,
    10,
    CLK: [PE10, PB10],
    nCS: [PE11, PB11],
    IO0: [PE12, PB1],
    IO1: [PE13, PB0],
    IO2: [PE14, PA7],
    IO3: [PE15, PA6]
);

#[cfg(not(any(feature = "stm32l475")))]
pins!(
    QUADSPI,
    10,
    CLK: [PA3],
    nCS: [PA2, PD3],
    IO0: [PD4],
    IO1: [PD5],
    IO2: [PD6],
    IO3: [PD7]
);

#[cfg(any(
    feature = "stm32l476",
    feature = "stm32l486",
    feature = "stm32l496",
    feature = "stm32l4a6"
))]
pins!(
    QUADSPI,
    10,
    CLK: [],
    nCS: [],
    IO0: [PC1, PF8],
    IO1: [PC2, PF9],
    IO2: [PC4, PF7],
    IO3: [PC5, PF6]
);
