//! This example shows how to use the FMC controller on the STM32F7 to communicate with an
//! off-chip SDRAM memory device. The `stm32-fmc` crate does the majority of the work, and
//! after initialization the SDRAM is memory mapped to the STM32F7 address space.
//!
//! This example was tested on the STM32F746G Discovery Board. The board has an IS42S32400F-6BL
//! SDRAM chip, with only 16 of the 32 data wires connected to the microcontroller. This device
//! is not explictly supported in the `stm32-fmc` crate at time of writing, but the IS42S16400J
//! has very similar parameters and is used as a placeholder for now. Because of this, only half
//! of the available memory space is available (128 Mb = 16 MB, so 8 MB available).
//!
//! To use the example, launch a GDB server and then `cargo run`. After several seconds, the
//! message "Success!" should be printed in the GDB server terminal (via semihosting).

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use core::{mem, slice};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32_fmc::devices::is42s16400j_7;
use stm32f4xx_hal::{
    fmc::FmcExt,
    gpio::{alt::fmc as alt, Speed},
    pac,
    prelude::*,
};

/// Configure pins for the FMC controller
macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        (
            $(
                alt::$alt::from($pin.into_alternate()
                    .speed(Speed::VeryHigh)
                    .internal_pull_up(true)
                )
            ),*
        )
    };
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Get the delay provider.
    let clocks = dp.RCC.constrain().cfgr.sysclk(216.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    // IO
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();
    let gpiof = dp.GPIOF.split();
    let gpiog = dp.GPIOG.split();
    let gpioh = dp.GPIOH.split();

    // Initialise SDRAM
    let fmc_io = fmc_pins! {
        A0: gpiof.pf0,
        A1: gpiof.pf1,
        A2: gpiof.pf2,
        A3: gpiof.pf3,
        A4: gpiof.pf4,
        A5: gpiof.pf5,
        A6: gpiof.pf12,
        A7: gpiof.pf13,
        A8: gpiof.pf14,
        A9: gpiof.pf15,
        A10: gpiog.pg0,
        A11: gpiog.pg1,
        Ba0: gpiog.pg4,
        Ba1: gpiog.pg5,
        D0: gpiod.pd14,
        D1: gpiod.pd15,
        D2: gpiod.pd0,
        D3: gpiod.pd1,
        D4: gpioe.pe7,
        D5: gpioe.pe8,
        D6: gpioe.pe9,
        D7: gpioe.pe10,
        D8: gpioe.pe11,
        D9: gpioe.pe12,
        D10: gpioe.pe13,
        D11: gpioe.pe14,
        D12: gpioe.pe15,
        D13: gpiod.pd8,
        D14: gpiod.pd9,
        D15: gpiod.pd10,
        Nbl0: gpioe.pe0,
        Nbl1: gpioe.pe1,
        Sdcke0: gpioc.pc3,
        Sdclk: gpiog.pg8,
        Sdncas: gpiog.pg15,
        Sdne0: gpioh.ph3,
        Sdnras: gpiof.pf11,
        Sdnwe: gpioh.ph5,
    };

    // New SDRAM
    let mut sdram = dp.FMC.sdram(fmc_io, is42s16400j_7::Is42s16400j {}, &clocks);

    // Initialise controller and SDRAM
    let len_bytes = (16 * 1024 * 1024) / 2;
    let len_words = len_bytes / mem::size_of::<u32>();
    let ram = unsafe {
        let ram_ptr: *mut u32 = sdram.init(&mut delay);
        slice::from_raw_parts_mut(ram_ptr, len_words)
    };

    // Access all the words in SDRAM (takes several seconds)
    for addr in 0..len_words {
        let val: u32 = addr as u32;

        // Write
        ram[addr] = val;

        // Read
        let res: u32 = ram[addr];
        if res != val {
            panic!(
                "Error: Expected {} while reading address {:X} but got {}.",
                val, addr, res
            );
        }
    }

    hprintln!("Success!");

    loop {}
}
