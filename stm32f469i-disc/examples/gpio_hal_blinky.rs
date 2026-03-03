//! Cycle through the LEDs on the board in order
#![no_main]
#![no_std]

use panic_probe as _;

use defmt_rtt as _;

use stm32f469i_disc as board;

use crate::board::{
    hal::{pac, prelude::*, rcc},
    led::Leds,
};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take())
    {
        let rcc = p.RCC.constrain();

        let mut rcc = rcc.freeze(rcc::Config::hse(8.MHz()).sysclk(180.MHz()));

        let clocks = rcc.clocks;

        let gpiod = p.GPIOD.split(&mut rcc);
        let gpiog = p.GPIOG.split(&mut rcc);
        let gpiok = p.GPIOK.split(&mut rcc);

        let mut delay = cp.SYST.delay(&clocks);
        let pause = 200_u32;

        let mut leds = Leds::new(gpiod, gpiog, gpiok);

        loop {
            for led in leds.iter_mut() {
                led.on();
                delay.delay_ms(pause);
            }

            for led in leds.iter_mut() {
                led.off();
                delay.delay_ms(pause);
            }
        }
    }

    loop {
        continue;
    }
}
