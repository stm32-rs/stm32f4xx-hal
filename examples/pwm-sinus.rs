#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use core::f32::consts::FRAC_PI_2;
use cortex_m_rt::entry;
use micromath::F32Ext;
use stm32f4xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.freeze(Config::hse(25.MHz()));

        let gpioa = dp.GPIOA.split();

        let (_, (pwm_c1, pwm_c2, ..)) = dp.TIM1.pwm_us(100.micros(), &rcc.clocks);
        let mut pwm_c1 = pwm_c1.with(gpioa.pa8);
        let mut pwm_c2 = pwm_c2.with(gpioa.pa9);

        let mut counter = dp.TIM2.counter_us(&rcc.clocks);
        let max_duty = pwm_c1.get_max_duty();

        const N: usize = 50;
        let mut sin_a = [0_u16; N + 1];
        // Fill sinus array
        let a = FRAC_PI_2 / (N as f32);
        for (i, b) in sin_a.iter_mut().enumerate() {
            let angle = a * (i as f32);
            *b = (angle.sin() * (max_duty as f32)) as u16;
        }

        counter.start(100.micros()).unwrap();
        pwm_c1.enable();
        pwm_c2.enable();
        let mut i = 0;
        loop {
            if i == 0 {
                pwm_c2.set_duty(0);
            }
            if i == 2 * N {
                pwm_c1.set_duty(0);
            }
            if i < N {
                pwm_c1.set_duty(sin_a[i]);
            } else if i < 2 * N {
                pwm_c1.set_duty(sin_a[2 * N - i]);
            } else if i < 3 * N {
                pwm_c2.set_duty(sin_a[i - 2 * N]);
            } else {
                pwm_c2.set_duty(sin_a[4 * N - i]);
            }
            nb::block!(counter.wait()).unwrap();
            i += 1;
            if i == 4 * N {
                i -= 4 * N;
            }
        }
    }

    loop {
        cortex_m::asm::nop();
    }
}
