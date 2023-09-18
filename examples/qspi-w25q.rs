//! Example of using the QSPI peripheral with a W25Q flash chip (W25Q128JV).
//! Pins configured for QSPI Bank1 of STM32F412 board. Adjust as needed.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::qspi::{
    FlashSize, MemoryMapped, Qspi, QspiConfig, QspiError, QspiMemoryMappedConfig, QspiMode,
    QspiPins, QspiReadCommand, QspiWriteCommand,
};

pub struct W25Q<PINS: QspiPins> {
    qspi: Qspi<PINS>,
}

pub struct DeviceId(u8);

impl<PINS> W25Q<PINS>
where
    PINS: QspiPins,
{
    pub fn new(qspi: Qspi<PINS>) -> Result<Self, QspiError> {
        let mut chip = Self { qspi };
        chip.release_from_power_down()?;
        chip.quad_enable()?;
        Ok(chip)
    }

    pub fn release_from_power_down(&mut self) -> Result<DeviceId, QspiError> {
        let mut buf = [0u8; 1];

        self.qspi.indirect_read(
            QspiReadCommand::new(&mut buf, QspiMode::SingleChannel)
                .instruction(0xAB, QspiMode::SingleChannel)
                .address(0x0, QspiMode::SingleChannel),
        )?;

        Ok(DeviceId(buf[0]))
    }

    pub fn wait_on_busy(&mut self) -> Result<(), QspiError> {
        let mut buf = [0u8; 1];

        loop {
            self.qspi.indirect_read(
                QspiReadCommand::new(&mut buf, QspiMode::SingleChannel)
                    .instruction(0x05, QspiMode::SingleChannel),
            )?;

            if buf[0] & 0x01 == 0 {
                return Ok(());
            }
        }
    }

    pub fn erase_sector(&mut self, address: u32) -> Result<(), QspiError> {
        self.write_enable()?;
        self.qspi.indirect_write(
            QspiWriteCommand::default()
                .instruction(0x20, QspiMode::SingleChannel)
                .address(address, QspiMode::SingleChannel),
        )?;

        self.wait_on_busy()?;
        Ok(())
    }

    pub fn write_enable(&mut self) -> Result<(), QspiError> {
        self.qspi.indirect_write(
            QspiWriteCommand::default().instruction(0x06, QspiMode::SingleChannel),
        )?;
        self.wait_on_busy()?;

        Ok(())
    }

    pub fn quad_enable(&mut self) -> Result<(), QspiError> {
        // First check if quad is already enabled
        let mut buf = [0u8; 1];
        self.qspi.indirect_read(
            QspiReadCommand::new(&mut buf, QspiMode::SingleChannel)
                .instruction(0x35, QspiMode::SingleChannel),
        )?;

        if buf[0] & 0x02 == 0x02 {
            return Ok(());
        }

        // If not, first we need to make the register writable
        self.write_enable()?;

        // Then we can set the quad enable bit
        self.qspi.indirect_write(
            QspiWriteCommand::default()
                .instruction(0x31, QspiMode::SingleChannel)
                .address(0x0, QspiMode::SingleChannel)
                .data(&[buf[0] | 0x2], QspiMode::SingleChannel),
        )?;
        Ok(())
    }

    pub fn program_page(&mut self, address: u32, data: &[u8]) -> Result<(), QspiError> {
        self.write_enable()?;

        self.qspi.indirect_write(
            QspiWriteCommand::default()
                .instruction(0x32, QspiMode::SingleChannel)
                .address(address, QspiMode::SingleChannel)
                .data(data, QspiMode::QuadChannel),
        )?;

        self.wait_on_busy()?;
        Ok(())
    }

    pub fn read(&mut self, address: u32, data: &mut [u8]) -> Result<(), QspiError> {
        self.qspi.indirect_read(
            QspiReadCommand::new(data, QspiMode::QuadChannel)
                .instruction(0xEB, QspiMode::SingleChannel)
                .address(address, QspiMode::QuadChannel)
                // the M0-M7 bits will be send by alternate_bytes, and the value should be 0xFX
                .alternate_bytes(&[0xFF], QspiMode::QuadChannel)
                .dummy_cycles(4),
        )?;

        Ok(())
    }

    pub fn memory_mapped<'a>(&'a mut self) -> Result<MemoryMapped<'a, PINS>, QspiError> {
        self.qspi.memory_mapped(
            QspiMemoryMappedConfig::default()
                .instruction(0xEB, QspiMode::SingleChannel)
                .address_mode(QspiMode::QuadChannel)
                .data_mode(QspiMode::QuadChannel)
                // the M0-M7 bits will be send by alternate_bytes, and the value should be 0xFX
                .alternate_bytes(&[0xFF], QspiMode::QuadChannel)
                .dummy_cycles(4),
        )
    }
}

#[entry]
fn main() -> ! {
    if let Some(dp) = stm32f4xx_hal::pac::Peripherals::take() {
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpiod = dp.GPIOD.split();
        let gpioe = dp.GPIOE.split();

        let qspi = Qspi::bank1(
            dp.QUADSPI,
            (
                gpiob.pb6, gpiod.pd11, gpiod.pd12, gpioe.pe2, gpioa.pa1, gpiob.pb1,
            ),
            QspiConfig::default()
                .address_size(hal::qspi::AddressSize::Addr24Bit)
                .flash_size(FlashSize::from_megabytes(16))
                .clock_prescaler(0)
                .sample_shift(hal::qspi::SampleShift::HalfACycle),
        );

        let mut flash = W25Q::new(qspi).unwrap();
        flash.erase_sector(0).unwrap();
        flash.program_page(0, "Hello, world!".as_bytes()).unwrap();

        let mut buf = [0u8; 13];
        flash.read(0, &mut buf).unwrap();

        hprintln!("Read: {:?}", core::str::from_utf8(&buf));

        let mem_mapped = flash.memory_mapped().unwrap();
        hprintln!(
            "Mapped: {:?}",
            core::str::from_utf8(&mem_mapped.buffer()[0..13])
        );
    }

    loop {}
}
