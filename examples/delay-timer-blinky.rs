//! Demonstrate the use of a blocking `Delay` using TIM5 general-purpose timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Mini-F4 it's connected to pin PC13.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on general-pupose 32-bit timer TIM5
        let mut delay = dp.TIM5.delay_us(&clocks);

        loop {
            // On for 1s, off for 3s.
            led.set_high();
            // Use `embedded_hal::DelayMs` trait
            delay.delay_ms(1000_u32);
            led.set_low();
            // or use `fugit::ExtU32` trait
            delay.delay(3.secs());
        }
    }

    loop {}
}
