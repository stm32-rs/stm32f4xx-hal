#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use hal::{pac, prelude::*, timer::Polarity};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock. We want to run at 84MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(25.MHz()).freeze();

        let gpioa = dp.GPIOA.split();

        let (mut pwm_mngr, (pwm_c1, ..)) = dp.TIM1.pwm_hz(20.kHz(), &clocks);

        let mut pwm_c1 = pwm_c1.with(gpioa.pa8).with_complementary(gpioa.pa7);

        let max_duty: u16 = pwm_c1.get_max_duty();

        pwm_c1.set_polarity(Polarity::ActiveHigh);
        pwm_c1.set_complementary_polarity(Polarity::ActiveHigh);

        pwm_c1.set_duty(max_duty / 2);

        pwm_mngr.set_dead_time(200);

        pwm_c1.enable();
        pwm_c1.enable_complementary();
    }

    loop {
        cortex_m::asm::nop();
    }
}
