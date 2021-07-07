#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, serial::config::Config, serial::Serial, stm32};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    let mut delay = hal::delay::Delay::new(cp.SYST, &clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa2.into_alternate();

    // configure serial
    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default().baudrate(9600.bps()),
        clocks,
    )
    .unwrap();

    let mut value: u8 = 0;

    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {:02}\r", value).unwrap();
        value += 1;
        delay.delay_ms(500_u32);
    }
}
