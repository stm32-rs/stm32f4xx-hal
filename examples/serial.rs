#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, rcc::Config};

use crate::hal::{pac, prelude::*};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.freeze(Config::hse(25.MHz()));

    let mut delay = dp.TIM1.delay_ms(&rcc.clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa9;

    // configure serial
    // let mut tx = Serial::tx(dp.USART1, tx_pin, 9600.bps(), &rcc.clocks).unwrap();
    // or
    let mut tx = dp.USART1.tx(tx_pin, 9600.bps(), &rcc.clocks).unwrap();

    let mut value: u8 = 0;

    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {value:02}\r").unwrap();
        value = value.wrapping_add(1);
        delay.delay(2.secs());
    }
}
