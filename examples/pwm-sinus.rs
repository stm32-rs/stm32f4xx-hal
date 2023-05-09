#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use core::f32::consts::FRAC_PI_2;
use cortex_m_rt::entry;
use fugit::ExtU32;
use micromath::F32Ext;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    timer::{Channel, Channel1, Channel2},
};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

        let gpioa = dp.GPIOA.split();
        let channels = (Channel1::new(gpioa.pa8), Channel2::new(gpioa.pa9));

        let mut pwm = dp.TIM1.pwm_us(channels, 100.micros(), &clocks);
        let mut counter = dp.TIM2.counter_us(&clocks);
        let max_duty = pwm.get_max_duty();

        const N: usize = 50;
        let mut sin_a = [0_u16; N + 1];
        // Fill sinus array
        let a = FRAC_PI_2 / (N as f32);
        for (i, b) in sin_a.iter_mut().enumerate() {
            let angle = a * (i as f32);
            *b = (angle.sin() * (max_duty as f32)) as u16;
        }

        counter.start(100.micros()).unwrap();
        pwm.enable(Channel::C1);
        pwm.enable(Channel::C2);
        let mut i = 0;
        loop {
            if i == 0 {
                pwm.set_duty(Channel::C2, 0);
            }
            if i == 2 * N {
                pwm.set_duty(Channel::C1, 0);
            }
            if i < N {
                pwm.set_duty(Channel::C1, sin_a[i]);
            } else if i < 2 * N {
                pwm.set_duty(Channel::C1, sin_a[2 * N - i]);
            } else if i < 3 * N {
                pwm.set_duty(Channel::C2, sin_a[i - 2 * N]);
            } else {
                pwm.set_duty(Channel::C2, sin_a[4 * N - i]);
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
