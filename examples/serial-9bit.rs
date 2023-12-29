//!
//! This example demonstrates 9-bit serial (USART) communication. It uses LEDs to display
//! some bits of the received signals.
//!
//! # Hardware required
//!
//! Use a 32F411EDISCOVERY evaluation board.
//! Use a wire to connect pins PA2 and PA3 (this loopback connection makes the microcontroller
//! receive everything it transmits).
//!
//! You can also easily adapt this example to any other STM32F4 evaluation board that has four LEDs.
//!
//! # Expected behavior
//!
//! The microcontroller sends increasing 9-bit numbers over the USART, and receives them.
//! The on-board LEDs display some bits of the received numbers:
//!
//! * Green LED LD4 (PD12) corresponds to bit 5
//! * Orange LED LD3 (PD13) corresponds to bit 6
//! * Red LED LD5 (PD14) corresponds to bit 7
//! * Blue LED LD6 (PD15) corresponds to bit 8
//!
//! Because the microcontroller sends a newly incremented number about every 10 milliseconds,
//! the green LED should toggle about every 320 milliseconds. The other LEDs, including the
//! blue LED (bit 8) should toggle appropriately, indicating that the microcontroller is sending
//! and receiving all 9 bits.
//!

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{block, pac, prelude::*, serial::config::Config};

use core::ops::Range;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiod = dp.GPIOD.split();

    let mut led_bit5 = gpiod.pd12.into_push_pull_output();
    let mut led_bit6 = gpiod.pd13.into_push_pull_output();
    let mut led_bit7 = gpiod.pd14.into_push_pull_output();
    let mut led_bit8 = gpiod.pd15.into_push_pull_output();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    // configure serial
    let serial = dp
        .USART2
        .serial(
            (tx_pin, rx_pin),
            Config::default().baudrate(9600.bps()).wordlength_9(),
            &clocks,
        )
        .unwrap()
        // Make this Serial object use u16s instead of u8s
        .with_u16_data();

    let (mut tx, mut rx) = serial.split();

    let nine_bit_integers: Range<u16> = 0x0..0x200;

    loop {
        for value in nine_bit_integers.clone() {
            block!(tx.write(value)).unwrap();
            // Receive what we just sent
            let received: u16 = block!(rx.read()).unwrap();

            // Update LEDs to display what was received
            if ((received >> 5) & 1) == 1 {
                led_bit5.set_high();
            } else {
                led_bit5.set_low();
            }
            if ((received >> 6) & 1) == 1 {
                led_bit6.set_high();
            } else {
                led_bit6.set_low();
            }
            if ((received >> 7) & 1) == 1 {
                led_bit7.set_high();
            } else {
                led_bit7.set_low();
            }
            if ((received >> 8) & 1) == 1 {
                led_bit8.set_high();
            } else {
                led_bit8.set_low();
            }

            delay.delay_ms(10);
        }
    }
}
