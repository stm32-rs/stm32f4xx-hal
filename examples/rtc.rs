//! This example demonstrates how to use the RTC.
//! Note that the LSI can be quite inaccurate.
//! The tolerance is up to ±47% (Min 17 kHz, Typ 32 kHz, Max 47 kHz).

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::{pac, prelude::*, rtc::Rtc};
use time::{
    macros::{date, time},
    PrimitiveDateTime,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut p = pac::Peripherals::take().unwrap();
    let rcc = p.RCC.constrain();

    let mut rtc = Rtc::new(p.RTC, &mut p.PWR);
    let mut delay = p.TIM5.delay_us(&rcc.clocks);

    rtc.set_datetime(&PrimitiveDateTime::new(
        date!(2022 - 02 - 07),
        time!(23:59:50),
    ))
    .unwrap();
    // Alternatively:
    // rtc.set_date(&date!(2022 - 02 - 07)).unwrap();
    // rtc.set_time(&time!(23:59:50)).unwrap();
    // Or:
    // rtc.set_year(2022).unwrap();
    // rtc.set_month(02).unwrap();
    // rtc.set_day(07).unwrap();
    // rtc.set_hours(23).unwrap();
    // rtc.set_minutes(59).unwrap();
    // rtc.set_seconds(50).unwrap();
    loop {
        rprintln!("{}", rtc.get_datetime());
        delay.delay(500.millis());
    }
}
