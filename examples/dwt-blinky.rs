#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use crate::hal::{
    dwt::{ClockDuration, DwtExt},
    prelude::*,
    stm32,
};
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;

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

        // Create a delay abstraction based on DWT cycle counter
        let dwt = cp.DWT.constrain(cp.DCB, clocks);
        let mut delay = dwt.delay();

        // Create a stopwatch for maximum 9 laps
        // Note: it starts immediately
        let mut lap_times = [0u32; 10];
        let mut sw = dwt.stopwatch(&mut lap_times);
        loop {
            // On for 1s, off for 1s.
            led1.set_high().unwrap();
            led2.set_low().unwrap();
            delay.delay_ms(1000_u32);
            sw.lap();
            led1.set_low().unwrap();
            led2.set_high().unwrap();
            delay.delay_ms(900_u32);
            // Also you can measure with almost clock precision
            let cd: ClockDuration = dwt.measure(|| delay.delay_ms(100_u32));
            let _t: u32 = cd.as_ticks(); // Should return 48MHz * 0.1s as u32
            let _t: f32 = cd.as_secs_f32(); // Should return ~0.1s as a f32
            let _t: f64 = cd.as_secs_f64(); // Should return ~0.1s as a f64
            let _t: u64 = cd.as_nanos(); // Should return 100000000ns as a u64
            sw.lap();

            // Get all the lap times
            {
                let mut lap = 1;
                while let Some(lap_time) = sw.lap_time(lap) {
                    let _t = lap_time.as_secs_f64();
                    lap += 1;
                }
            }

            // Reset stopwatch
            sw.reset();
        }
    }

    loop {}
}
