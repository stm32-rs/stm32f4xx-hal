#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_probe as _;
use stm32f4xx_hal::{self as hal, gpio::gpioc};

use cortex_m_rt::entry;
use embedded_hal::spi::MODE_0;
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use hal::{pac, prelude::*};
use rtt_target::{rprintln, rtt_init_print};
use w25q_spi::{models::W25Q64, W25Q};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Started");
    let dp = pac::Peripherals::take().expect("cannot take peripherals");

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
    let mut delay = dp.TIM1.delay_us(&clocks);

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high();

    let cs = gpioa.pa4.into_push_pull_output();
    let spi = dp
        .SPI1
        .spi((gpioa.pa5, gpioa.pa6, gpioa.pa7), MODE_0, 1.MHz(), &clocks);
    let ed = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    rprintln!("Init");
    // Create the flash driver instance
    let mut flash = W25Q::new(W25Q64, ed);

    rprintln!("Id");
    let id = flash.device_id().unwrap();
    rprintln!("Id = {:?}", id);

    //rprintln!("Erase");
    // Erase the chip
    //flash.erase_chip().unwrap();

    rprintln!("Write");
    test_write(&mut flash);
    rprintln!("Read");
    test_read(&mut flash);
    rprintln!("Finish");

    loop {
        delay.delay(200.millis());
        led.toggle();
    }
}

const TEST_DATA: [u8; 4] = [0x36, 0x04, 0x81, 0xFE];
const TEST_OFFSET: u32 = 0x1000;

fn test_write(flash: &mut impl NorFlash) {
    flash.write(TEST_OFFSET, &TEST_DATA).unwrap();
}

fn test_read(flash: &mut impl ReadNorFlash) {
    let mut buf: [u8; 4] = [0; 4];
    flash.read(TEST_OFFSET, &mut buf).unwrap();

    assert_eq!(buf, TEST_DATA);
}
