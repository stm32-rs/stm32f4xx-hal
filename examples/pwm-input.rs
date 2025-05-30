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
        let mut rcc = dp.RCC.constrain();

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpioc = dp.GPIOC.split(&mut rcc);

        // configure tim1 as a PWM output of known frequency.
        let (_, (ch1, ch2, ..)) = Timer::new(dp.TIM1, &mut rcc).pwm_hz(501.Hz());
        let mut ch1 = ch1.with(gpioa.pa8);
        let mut _ch2 = ch2.with(gpioa.pa9);
        let max_duty = ch1.get_max_duty();
        ch1.set_duty(max_duty / 2);
        ch1.enable();

        // Configure a pin into TIM8_CH1 mode, which will be used to observe an input PWM signal.
        let pwm_reader_ch1 = gpioc.pc6;

        // configure tim8 as a PWM input, using the best-guess frequency of the input signal.
        let monitor = Timer::new(dp.TIM8, &mut rcc).pwm_input(500.Hz(), pwm_reader_ch1);

        // NOTE: this value may only be accurately observed at the CC2 interrupt.
        let _duty = monitor.get_duty_cycle();
    }

    loop {
        cortex_m::asm::nop();
    }
}
