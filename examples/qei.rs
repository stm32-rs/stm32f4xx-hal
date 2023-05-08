#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Demonstrates the use of a rotary encoder. This example was tested
// on a "black pill" USB C board:
// https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0
//
// The rotary encoder A and B pins are connected to pins A0 and A1,
// and they each have a 10K ohm pull-up resistor.

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::Direction as RotaryDirection;
use stm32f4xx_hal::{pac, prelude::*, qei::Qei};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("Failed to get stm32 peripherals");
    let cp = cortex_m::peripheral::Peripherals::take().expect("Failed to get cortex_m peripherals");

    // Set up the LED. This is pin C13 on the "black pill" USB C board here:
    // https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // Create a delay abstraction based on SysTick.
    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();

    // Connect a rotary encoder to pins A0 and A1.
    let rotary_encoder_pins = (gpioa.pa0, gpioa.pa1);
    let rotary_encoder_timer = dp.TIM2;
    let rotary_encoder = Qei::new(rotary_encoder_timer, rotary_encoder_pins);

    let mut current_count = rotary_encoder.count();

    loop {
        let new_count = rotary_encoder.count();

        if new_count != current_count {
            let _diff = new_count.wrapping_sub(current_count) as i16;

            // Light up the LED when turning clockwise, turn it off
            // when turning counter-clockwise.
            match rotary_encoder.direction() {
                RotaryDirection::Upcounting => led.set_low(),
                RotaryDirection::Downcounting => led.set_high(),
            }

            current_count = new_count;
        }

        delay.delay_ms(10_u32);
    }
}
