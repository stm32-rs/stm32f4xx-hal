#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*, timer::Timer};

use hal::spi::{Mode, Phase, Polarity};

use display_interface_spi_04::SPIInterface;
use ist7920::Ist7920;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low();

    let sck = gpiob.pb3.into_alternate();
    let mosi = gpiob.pb5;

    let dc = gpiob.pb4.into_push_pull_output();
    let mut res = gpiob.pb10.into_push_pull_output();
    let cs = gpiob.pb13.into_push_pull_output();

    let mut delay = Timer::syst(cp.SYST, &rcc.clocks).delay();

    let mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    // Change spi transfer mode to Bidi for more efficient operations.
    // let spi = Spi::new(dp.SPI1, (Some(sck), Some(miso), Some(mosi)), mode, 8.MHz(), &mut rcc).to_bidi_transfer_mode();
    // or
    let spi = dp
        .SPI1
        .spi_bidi((Some(sck), Some(mosi)), mode, 8.MHz(), &mut rcc);

    let iface = SPIInterface::new(spi, dc, cs);

    let mut display = Ist7920::new(iface);

    display.reset(&mut res, &mut delay).ok();

    display.init(&mut delay).ok();

    let mut select_figure = 0;
    loop {
        delay.delay_ms(500);
        let (begin, end) = match select_figure {
            0 => {
                select_figure = 1;
                ((48, 48), (48 + 31, 48 + 31))
            }

            1 => {
                select_figure = 2;
                ((32, 32), (32 + 63, 32 + 63))
            }
            _ => {
                select_figure = 0;
                ((24, 24), (24 + 79, 24 + 79))
            }
        };
        display.clear().ok();
        display.set_draw_area(begin, end).ok();
        for _ in 0..((end.1 - begin.1 + 1) / 16) {
            for _ in 0..((end.0 - begin.0 + 1) / 16) {
                display
                    .draw(&[
                        0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xff, 0xff, 0xff, 0xff,
                        0xff, 0xff, 0xff, 0xff,
                    ])
                    .ok();
            }
            for _ in 0..((end.0 - begin.0 + 1) / 16) {
                display
                    .draw(&[
                        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])
                    .ok();
            }
        }
        led.toggle();
    }
}
