//! Delay implementation based on general-purpose 32 bit timers.
//!
//! TIM2 and TIM5 are a general purpose 32-bit auto-reload up/downcounter with
//! a 16-bit prescaler.

use core::cmp::max;

use cast::{u16, u32};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use crate::{pac, rcc::Clocks, timer::Timer};

use super::Delay;

macro_rules! hal {
    ($($TIM:ty: ($tim:ident, $waitfn:ident),)+) => {
        $(
            fn $waitfn(tim: &mut $TIM, prescaler: u16, auto_reload_register: u32) {
                // Write Prescaler (PSC)
                tim.psc.write(|w| w.psc().bits(prescaler));

                // Write Auto-Reload Register (ARR)
                // Note: Make it impossible to set the ARR value to 0, since this
                // would cause an infinite loop.
                tim.arr.write(|w| unsafe { w.bits(max(1, auto_reload_register)) });

                // Trigger update event (UEV) in the event generation register (EGR)
                // in order to immediately apply the config
                tim.cr1.modify(|_, w| w.urs().set_bit());
                tim.egr.write(|w| w.ug().set_bit());
                tim.cr1.modify(|_, w| w.urs().clear_bit());

                // Configure the counter in one-pulse mode (counter stops counting at
                // the next updateevent, clearing the CEN bit) and enable the counter.
                tim.cr1.write(|w| w.opm().set_bit().cen().set_bit());

                // Wait for CEN bit to clear
                while tim.cr1.read().cen().is_enabled() { /* wait */ }
            }

            impl Timer<$TIM> {
                pub fn delay(self) -> Delay<$TIM> {
                    let Self { tim, clk } = self;

                    // Enable one-pulse mode (counter stops counting at the next update
                    // event, clearing the CEN bit)
                    tim.cr1.modify(|_, w| w.opm().enabled());

                    Delay { tim, clk }
                }
            }

            impl Delay<$TIM> {
                /// Configures the timer as a delay provider
                pub fn $tim(tim: $TIM, clocks: &Clocks) -> Self {
                    Timer::new(tim, clocks).delay()
                }
            }

            impl DelayUs<u32> for Delay<$TIM> {
                /// Sleep for up to 2^32-1 microseconds (~71 minutes).
                fn delay_us(&mut self, us: u32) {
                    // Set up prescaler so that a tick takes exactly 1 µs.
                    //
                    // For example, if the clock is set to 48 MHz, with a prescaler of 48
                    // we'll get ticks that are 1 µs long. This means that we can write the
                    // delay value directly to the auto-reload register (ARR).
                    let psc = u16(self.clk.0 / 1_000_000)
                        .expect("Prescaler does not fit in u16");
                    let arr = us;
                    $waitfn(&mut self.tim, psc, arr);
                }
            }

            impl DelayUs<u16> for Delay<$TIM> {
                /// Sleep for up to 2^16-1 microseconds (~65 milliseconds).
                fn delay_us(&mut self, us: u16) {
                    // See DelayUs<u32> for explanations.
                    let psc = u16(self.clk.0 / 1_000_000)
                        .expect("Prescaler does not fit in u16");
                    let arr = u32(us);
                    $waitfn(&mut self.tim, psc, arr);
                }
            }

            impl DelayMs<u32> for Delay<$TIM> {
                /// Sleep for up to (2^32)/2-1 milliseconds (~24 days).
                /// If the `ms` value is larger than 2147483647, the code will panic.
                fn delay_ms(&mut self, ms: u32) {
                    // See next section for explanation why the usable range is reduced.
                    assert!(ms <= 2_147_483_647); // (2^32)/2-1

                    // Set up prescaler so that a tick takes exactly 0.5 ms.
                    //
                    // For example, if the clock is set to 48 MHz, with a prescaler of 24'000
                    // we'll get ticks that are 0.5 ms long. This means that we can write the
                    // delay value multipled by two to the auto-reload register (ARR).
                    //
                    // Note that we cannot simply use a prescaler value where the tick corresponds
                    // to 1 ms, because then a clock of 100 MHz would correspond to a prescaler
                    // value of 100'000, which doesn't fit in the 16-bit PSC register.
                    //
                    // Unfortunately this means that only one half of the full 32-bit range
                    // can be used, but 24 days should be plenty of usable delay time.
                    let psc = u16(self.clk.0 / 1000 / 2)
                        .expect("Prescaler does not fit in u16");

                    // Since PSC = 0.5 ms, double the value for the ARR
                    let arr = ms << 1;

                    $waitfn(&mut self.tim, psc, arr);
                }
            }

            impl DelayMs<u16> for Delay<$TIM> {
                /// Sleep for up to (2^16)-1 milliseconds (~65 seconds).
                fn delay_ms(&mut self, ms: u16) {
                    // See DelayMs<u32> for explanations. Since the value range is only 16 bit,
                    // we don't need an assert here.
                    let psc = u16(self.clk.0 / 1000 / 2)
                        .expect("Prescaler does not fit in u16");
                    let arr = u32(ms) << 1;
                    $waitfn(&mut self.tim, psc, arr);
                }
            }
        )+
    }
}

hal! {
    pac::TIM5: (tim5, wait_tim5),
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
hal! {
    pac::TIM2: (tim2, wait_tim2),
}
