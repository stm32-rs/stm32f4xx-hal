#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::array::from_fn;

use panic_halt as _;
use stm32f4xx_hal::{self as hal, rcc::Config};

use cortex_m_rt::entry;
use hal::{
    pac::{self, SPI1},
    prelude::*,
};
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_spi as ws2812;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("cannot take peripherals");

    // Configure APB bus clock to 48 MHz, cause ws2812b requires 3 Mbps SPI
    let mut rcc = dp.RCC.freeze(Config::hse(25.MHz()).sysclk(48.MHz()));

    let mut delay = dp.TIM1.delay_us(&mut rcc);
    let gpioa = dp.GPIOA.split(&mut rcc);

    let spi = dp.SPI1.spi(
        (Some(gpioa.pa5), SPI1::NoMiso, Some(gpioa.pa7)),
        ws2812::MODE,
        3000.kHz(),
        &mut rcc,
    );

    const NUM_LEDS: usize = 8;
    let mut buffer = [0; NUM_LEDS * 12 + 20];
    let mut ws = ws2812::prerendered::Ws2812::new(spi, buffer.as_mut_slice());

    // Wait before start write for syncronization
    delay.delay(200.micros());

    let mut time = 0;

    let ukr: [RGB8; NUM_LEDS] = from_fn(|i| {
        if i < NUM_LEDS / 2 {
            RGB8::new(255, 255, 0)
        } else {
            RGB8::new(0, 50, 255)
        }
    });
    let bel: [RGB8; NUM_LEDS] = from_fn(|i| match i {
        3..5 => RGB8::new(255, 0, 0),
        _ => RGB8::new(255, 255, 255),
    });

    loop {
        match time {
            0..2 => {
                for j in 0..(256 * 5) {
                    let data = (0..NUM_LEDS).map(|i| {
                        wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8)
                    });
                    ws.write(brightness(data, 32)).unwrap();
                    delay.delay(10.millis());
                }
            }
            2..4 => {
                ws.write(brightness(ukr.into_iter(), 32)).unwrap();
                delay.delay(10.millis() * (256 * 5));
            }
            _ => {
                ws.write(brightness(bel.into_iter(), 32)).unwrap();
                delay.delay(10.millis() * (256 * 5));
            }
        }
        if time < 2 {
        } else {
        }
        time += 1;
        if time > 5 {
            time = 0;
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
