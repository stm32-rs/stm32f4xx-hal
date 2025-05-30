//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, rcc::Config};

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));

        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split(&mut rcc);
        let mut led = gpioa.pa5.into_push_pull_output();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&rcc.clocks);

        loop {
            // On for 1s, off for 1s.
            led.toggle();
            delay.delay_ms(1000);
        }
    }

    loop {}
}
