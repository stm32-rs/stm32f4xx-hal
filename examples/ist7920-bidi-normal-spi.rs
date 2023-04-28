#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*, timer::Timer};

use hal::spi::{Mode, NoMiso, Phase, Polarity};

use display_interface_spi::SPIInterface;
use ist7920::Ist7920;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low();

    let sck = gpiob.pb3.into_alternate();
    let miso = NoMiso;
    let mosi = gpiob.pb5.into_alternate();

    let dc = gpiob.pb4.into_push_pull_output();
    let mut res = gpiob.pb10.into_push_pull_output();
    let cs = gpiob.pb13.into_push_pull_output();

    let mut delay = Timer::syst(cp.SYST, &clocks).delay();

    let mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    // Change spi transfer mode to Bidi for more efficient operations.
    // let spi = Spi::new(dp.SPI1, (sck, miso, mosi), mode, 8.MHz(), &clocks).to_bidi_transfer_mode();
    // or
    let spi = dp.SPI1.spi_bidi((sck, miso, mosi), mode, 8.MHz(), &clocks);

    let iface = SPIInterface::new(spi, dc, cs);

    let mut display = Ist7920::new(iface);

    display.reset(&mut res, &mut delay).ok();

    display.init(&mut delay).ok();

    let mut select_figure = 0;
    loop {
        delay.delay_ms(500_u16);
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
