#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32, systick::SysTickTime};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LEDs. On the STM32F429I-DISC[O1] they are connected to pin PG13/14.
        let gpiog = dp.GPIOG.split();
        let mut led1 = gpiog.pg13.into_push_pull_output();
        let mut led2 = gpiog.pg14.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut systime = cp.SYST.to_systemtime(1000.hz(), clocks);

        loop {
            // On for 1s, off for 1s.
            led1.set_high().unwrap();
            led2.set_low().unwrap();
            systime.delay_ms(1000_u32);
            led1.set_low().unwrap();
            led2.set_high().unwrap();
            systime.delay_ms(1000_u32);
            // Also you can get the current time
            let _t = systime.as_secs_f64(); // in seconds as f64
            let _t = systime.as_millis(); // in milliseconds as u64
            let _t = systime.as_micros(); // in microseconds as u64
            let _t = systime.as_nanos(); // in nanoseconds as u64
        }
    }

    loop {}
}
