#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        let channels = (gpioa.pa8.into_alternate(), gpioa.pa9.into_alternate());
        // configure tim1 as a PWM output of known frequency.
        let pwm = Timer::new(dp.TIM1, &clocks).pwm(channels, 501.Hz());
        let (mut ch1, _ch2) = pwm.split();
        let max_duty = ch1.get_max_duty();
        ch1.set_duty(max_duty / 2);
        ch1.enable();

        // Configure a pin into TIM8_CH1 mode, which will be used to observe an input PWM signal.
        let pwm_reader_ch1 = gpioc.pc6.into_alternate();

        // configure tim8 as a PWM input, using the best-guess frequency of the input signal.
        let monitor = Timer::new(dp.TIM8, &clocks).pwm_input(500.Hz(), pwm_reader_ch1);

        // NOTE: this value may only be accurately observed at the CC2 interrupt.
        let _duty = monitor.get_duty_cycle();
    }

    loop {
        cortex_m::asm::nop();
    }
}
