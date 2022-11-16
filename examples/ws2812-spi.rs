#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use stm32f4xx_hal as hal;

use cortex_m_rt::entry;
use hal::{gpio::NoPin, pac, prelude::*};
use smart_leds::{brightness, hsv::RGB8, SmartLedsWrite};
use ws2812_spi as ws2812;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("cannot take peripherals");

    // Configure APB bus clock to 56MHz, cause ws2812b requires 3.5Mbps SPI
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(56.MHz()).freeze();

    let mut delay = dp.TIM1.delay_us(&clocks);
    let gpioa = dp.GPIOA.split();

    let spi = dp.SPI1.spi(
        (gpioa.pa5, NoPin, gpioa.pa7),
        ws2812::MODE,
        3500.kHz(),
        &clocks,
    );

    let mut ws = ws2812::Ws2812::new(spi);

    const NUM_LEDS: usize = 8;
    let mut data = [RGB8::default(); NUM_LEDS];

    // Wait before start write for syncronization
    delay.delay(200.micros());

    loop {
        for j in 0..(256 * 5) {
            for (i, b) in data.iter_mut().enumerate() {
                *b = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            ws.write(brightness(data.iter().cloned(), 32)).unwrap();
            delay.delay(10.millis());
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
