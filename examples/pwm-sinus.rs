#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use core::f32::consts::FRAC_PI_2;
use cortex_m_rt::entry;
use micromath::F32Ext;
use stm32f4xx_hal::{pac, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
}
impl Quadrant {
    pub fn next_cw(&mut self) {
        *self = match self {
            Self::Q1 => Self::Q2,
            Self::Q2 => Self::Q3,
            Self::Q3 => Self::Q4,
            Self::Q4 => Self::Q1,
        };
    }
}

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.mhz()).freeze();

        let gpioa = dp.GPIOA.split();
        let channels = (gpioa.pa8.into_alternate(), gpioa.pa9.into_alternate());
        let mut quad = Quadrant::Q1;

        let pwm = dp.TIM1.pwm_us(&clocks, channels, 100.micros());
        let mut counter = dp.TIM2.counter_us(&clocks);
        let (mut ch1, mut ch2) = pwm;
        let max_duty = ch1.get_max_duty();

        const N: usize = 50;
        let mut sin_a = [0_u16; N + 1];
        // Fill sinus array
        let a = FRAC_PI_2 / (N as f32);
        for (i, b) in sin_a.iter_mut().enumerate() {
            let angle = a * (i as f32);
            *b = (angle.sin() * (max_duty as f32)) as u16;
        }

        counter.start(100.micros()).unwrap();
        ch1.enable();
        ch2.enable();
        let mut clos = |quad, duty| {
            match quad {
                Quadrant::Q1 | Quadrant::Q2 => {
                    ch1.set_duty(duty);
                    ch2.set_duty(0);
                }
                Quadrant::Q3 | Quadrant::Q4 => {
                    ch1.set_duty(0);
                    ch2.set_duty(duty);
                }
            }
            nb::block!(counter.wait()).unwrap();
        };
        loop {
            match quad {
                Quadrant::Q1 | Quadrant::Q3 => {
                    for &duty in sin_a.iter().take(N) {
                        clos(quad, duty);
                    }
                }
                Quadrant::Q2 | Quadrant::Q4 => {
                    for &duty in sin_a.iter().rev().take(N) {
                        clos(quad, duty);
                    }
                }
            }

            quad.next_cw();
        }
    }

    loop {
        cortex_m::asm::nop();
    }
}
