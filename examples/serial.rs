#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*, serial::Serial};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(25.mhz()).freeze();

    let mut delay = dp.TIM1.delay_ms(&clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa9.into_alternate();

    // configure serial
    let mut tx = Serial::tx(dp.USART1, tx_pin, 9600.bps(), &clocks).unwrap();

    let mut value: u8 = 0;

    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {:02}\r", value).unwrap();
        value = value.wrapping_add(1);
        delay.delay(2.secs()).unwrap();
    }
}
