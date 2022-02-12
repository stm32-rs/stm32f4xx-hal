//! Delays

mod hal_02;
mod hal_1;

use crate::{
    pac,
    rcc::Clocks,
    timer::{General, Timer},
};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::time::Hertz;

/// Timer as a delay provider (SysTick by default)
pub struct Delay<T = SYST> {
    tim: T,
    clk: Hertz,
}

impl<T> Delay<T> {
    /// Releases the timer resource
    pub fn release(self) -> T {
        self.tim
    }
}

impl Delay<SYST> {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::External);
        Self {
            tim,
            clk: clocks.hclk(),
        }
    }
}

mod sealed {
    pub trait Wait {
        fn wait(&mut self, prescaler: u16, auto_reload_register: u32);
    }
}
use sealed::Wait;

macro_rules! hal {
    ($($TIM:ty: ($tim:ident),)+) => {
        $(
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

            impl Wait for Delay<$TIM> {
                fn wait(&mut self, prescaler: u16, auto_reload_register: u32) {
                    // Write Prescaler (PSC)
                    self.tim.set_prescaler(prescaler - 1);

                    // Write Auto-Reload Register (ARR)
                    // Note: Make it impossible to set the ARR value to 0, since this
                    // would cause an infinite loop.
                    self.tim.set_auto_reload(auto_reload_register - 1).unwrap();

                    // Trigger update event (UEV) in the event generation register (EGR)
                    // in order to immediately apply the config
                    self.tim.trigger_update();

                    // Configure the counter in one-pulse mode (counter stops counting at
                    // the next updateevent, clearing the CEN bit) and enable the counter.
                    self.tim.cr1.write(|w| w.opm().set_bit().cen().set_bit());

                    // Wait for CEN bit to clear
                    while self.tim.is_counter_enabled() { /* wait */ }
                }
            }
        )+
    }
}

hal! {
    pac::TIM5: (tim5),
}

#[cfg(feature = "tim2")]
hal! {
    pac::TIM2: (tim2),
}
