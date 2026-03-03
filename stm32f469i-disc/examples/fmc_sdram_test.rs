//! Test the on-board SDRAM
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use core::slice;

use stm32f469i_disc as board;

use crate::board::hal::gpio::alt::fmc as alt;
use crate::board::hal::{pac, prelude::*, rcc};
use crate::board::sdram::{sdram_pins, Sdram};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

/// A psuedo-random pattern generator
struct XorShift32 {
    seed: u32,
}

impl XorShift32 {
    fn new(seed: u32) -> Self {
        XorShift32 { seed }
    }

    fn next(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take())
    {
        let rcc = p.RCC.constrain();

        let mut rcc = rcc.freeze(rcc::Config::hse(8.MHz()).sysclk(180.MHz()));

        let clocks = rcc.clocks;

        let mut delay = cp.SYST.delay(&clocks);

        let gpioc = p.GPIOC.split(&mut rcc);
        let gpiod = p.GPIOD.split(&mut rcc);
        let gpioe = p.GPIOE.split(&mut rcc);
        let gpiof = p.GPIOF.split(&mut rcc);
        let gpiog = p.GPIOG.split(&mut rcc);
        let gpioh = p.GPIOH.split(&mut rcc);
        let gpioi = p.GPIOI.split(&mut rcc);

        defmt::info!("Initializing SDRAM...\r");
        let sdram = Sdram::new(
            p.FMC,
            sdram_pins! {gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi},
            &clocks,
            &mut delay,
        );
        let ram = unsafe { slice::from_raw_parts_mut(sdram.mem, sdram.words) };

        defmt::info!("Testing SDRAM...\r");

        let seed: u32 = 0x8675309D;
        let mut pattern = XorShift32::new(seed);

        // write our pattern
        for (addr, res) in ram.iter_mut().enumerate().take(sdram.words) {
            let val = pattern.next();

            if (addr & 0x1ffff) == 0 {
                defmt::info!(
                    "Write: {:X} <- {:X}\r",
                    (sdram.mem as usize) + addr,
                    val
                );
            }

            *res = val;
        }

        // read back pattern
        pattern = XorShift32::new(seed);
        for (addr, res) in ram.iter_mut().enumerate().take(sdram.words) {
            let val = pattern.next();

            if (addr & 0x1ffff) == 0 {
                defmt::info!(
                    "Read:  {:X} -> {:X}\r",
                    (sdram.mem as usize) + addr,
                    val
                );
            }

            if *res != val {
                defmt::info!(
                    "Error: {:X} -> {:X} != {:X}\r",
                    (sdram.mem as usize) + addr,
                    val,
                    *res
                );
                break;
            }
        }

        defmt::info!("Done!\r");
    }
    loop {
        continue;
    }
}
