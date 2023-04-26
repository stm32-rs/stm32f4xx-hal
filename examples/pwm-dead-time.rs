#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use hal::{
    pac,
    prelude::*,
    timer::Channel,
    timer::{Channel1, Polarity},
};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock. We want to run at 84MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(25.MHz()).freeze();

        let gpioa = dp.GPIOA.split();

        let channels = Channel1::new(gpioa.pa8).with_complementary(gpioa.pa7);

        let mut pwm = dp.TIM1.pwm_hz(channels, 20.kHz(), &clocks);

        let max_duty: u16 = pwm.get_max_duty();

        pwm.set_polarity(Channel::C1, Polarity::ActiveHigh);
        pwm.set_complementary_polarity(Channel::C1, Polarity::ActiveHigh);

        pwm.set_duty(Channel::C1, max_duty / 2);

        pwm.set_dead_time(200);

        pwm.enable(Channel::C1);
        pwm.enable_complementary(Channel::C1);
    }

    loop {
        cortex_m::asm::nop();
    }
}
