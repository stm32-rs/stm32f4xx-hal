#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f4xx_hal::{gpio::PinState, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over raw device and convert it into the corresponding HAL struct
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze();

    // Acquire the GPIOC peripheral
    let gpioc = dp.GPIOC.split();

    let mut pin = gpioc.pc13.into_dynamic();
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_us();
    timer.start(1.secs()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        pin.make_floating_input();
        block!(timer.wait()).unwrap();
        hprintln!("{}", pin.is_high().unwrap());

        pin.make_push_pull_output_in_state(PinState::High);
        block!(timer.wait()).unwrap();
        pin.set_low().unwrap();
        block!(timer.wait()).unwrap();
    }
}
