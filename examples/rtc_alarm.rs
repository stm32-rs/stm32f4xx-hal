//! Sets an RTC alarm

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use crate::hal::prelude::*;
use crate::hal::rtc::{Alarm, Event, Rtc};
use cortex_m::interrupt::{free, Mutex};
use time::{
    macros::{date, time},
    PrimitiveDateTime,
};

use core::{cell::RefCell, ops::DerefMut};
use hal::interrupt;
use hal::pac;
use pac::NVIC;

static RTC: Mutex<RefCell<Option<Rtc>>> = Mutex::new(RefCell::new(None));

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut p = hal::pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut rtc = Rtc::new(p.RTC, &mut rcc, &mut p.PWR);

    let today = date!(2023 - 05 - 28);
    rtc.set_datetime(&PrimitiveDateTime::new(today, time!(21:57:32)))
        .unwrap();

    // Set alarm A for 1 minute
    rtc.set_alarm(Alarm::AlarmA, today, time!(21:58:32))
        .unwrap();
    rtc.enable_wakeup(8.secs().into());
    rtc.listen(&mut p.EXTI, Event::AlarmA);
    rtc.listen(&mut p.EXTI, Event::Wakeup);

    rprintln!("Hello, world!");

    unsafe {
        NVIC::unmask(pac::Interrupt::RTC_ALARM);
        NVIC::unmask(pac::Interrupt::RTC_WKUP);
    }

    free(|cs| {
        RTC.borrow(cs).replace(Some(rtc));
    });

    loop {
        continue;
    }
}

#[interrupt]
fn RTC_ALARM() {
    free(|cs| {
        let mut rtc_ref = RTC.borrow(cs).borrow_mut();
        if let Some(rtc) = rtc_ref.deref_mut() {
            if rtc.is_pending(Event::AlarmA) {
                rtc.clear_interrupt(Event::AlarmA);
                rprintln!("RTC Alaaaaarm!");
            }
        }
    });
}

#[interrupt]
fn RTC_WKUP() {
    free(|cs| {
        let mut rtc_ref = RTC.borrow(cs).borrow_mut();
        if let Some(rtc) = rtc_ref.deref_mut() {
            if rtc.is_pending(Event::Wakeup) {
                rtc.clear_interrupt(Event::Wakeup);
                rprintln!("RTC Wakeup!");
            }
        }
    });
}
