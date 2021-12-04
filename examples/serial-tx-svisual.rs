#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use svisual::{prelude::*, OnlyFront};

use cortex_m_rt::entry;
use stm32f4xx_hal::{delay::Delay, pac, prelude::*, serial::Serial};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the raw rcc device and convert it into the corresponding HAL struct
    let rcc = p.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze();

    // Prepare the GPIOA peripheral
    let gpioa = p.GPIOA.split();

    // USART3
    // Configure pa2 as a push_pull output, this will be the tx pin
    let tx = gpioa.pa2.into_alternate();

    // Set up the usart device. Taks ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let mut tx = Serial::tx(p.USART2, tx, 115_200.bps(), &clocks).unwrap();

    let mut delay = Delay::new(cp.SYST, &clocks);

    // Create new map with not more than 2 different signals and 10 values in package
    let mut sv_map = SVMap::<4, 10>::new();

    loop {
        for i in 0..20 {
            const MOD_NAME: SVName = SVName::new("TempMod");
            const NAME1: SVName = SVName::new("Int1");
            const NAME2: SVName = SVName::new("Float1");
            const NAME3: SVName = SVName::new("send");
            // Set value of first signal of integers
            sv_map.set(&NAME1, 15 + i).ok();
            // Set value of second signal of floats
            sv_map.set(&NAME2, 14. - (i as f32) / 2.).ok();
            if sv_map.is_last() {
                sv_map.set(&NAME3, OnlyFront(true)).ok();
            }
            // Use next value cell
            sv_map.next(|s| {
                // if package is full, send package with module name
                tx.send_package(&MOD_NAME, s).ok();
            });
            // Wait
            delay.delay_ms(100u16);
        }
    }
}
