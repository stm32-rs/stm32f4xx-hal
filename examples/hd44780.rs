#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use stm32f4xx_hal::{pac, prelude::*};

// Connections:
// VSS: GND
// VDD: 5V
// V0:  10k poti between 5V and GND
// RS:  PB7
// RW:  GND
// E:   PB8
// D4-D7: PB6-PB3
// A:   5V
// K:   GND

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split();

    let clocks = rcc.cfgr.freeze();
    let mut delay = dp.TIM1.delay_us(&clocks);

    let rs = gpiob.pb7.into_push_pull_output();
    let en = gpiob.pb8.into_push_pull_output();
    let d4 = gpiob.pb6.into_push_pull_output();
    let d5 = gpiob.pb5.into_push_pull_output();
    let d6 = gpiob.pb4.into_push_pull_output();
    let d7 = gpiob.pb3.into_push_pull_output();

    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, &mut delay).unwrap();
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();
    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Visible,
            cursor_blink: CursorBlink::On,
        },
        &mut delay,
    )
    .unwrap();
    lcd.write_str("Hello, world!", &mut delay).unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
