#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let gpioa = dp.GPIOA.split();

        let (_, (ch1, ch2, ..)) = dp.TIM1.pwm_us(100.micros(), &clocks);
        let mut ch1 = ch1.with(gpioa.pa8);
        let mut _ch2 = ch2.with(gpioa.pa9);

        let max_duty = ch1.get_max_duty();
        ch1.set_duty(max_duty / 2);
        ch1.enable();
    }

    loop {
        cortex_m::asm::nop();
    }
}
