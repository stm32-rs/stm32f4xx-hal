//! This example shows how to use the FMC controller on the STM32F469I-DISC to communicate with an
//! off-chip SDRAM memory device.
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_probe as _;

use core::{mem, slice};
use stm32f4xx_hal::{fmc::FmcExt, gpio::alt::fmc as alt, pac, prelude::*};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};

use stm32_fmc::devices::is42s32400f_6;

/// Configure pins for the FMC controller
macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        (
            $(
                alt::$alt::from($pin.internal_pull_up(true))
            ),*
        )
    };
}

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
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let rcc = p.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();
        let mut delay = cp.SYST.delay(&clocks);

        let gpioc = p.GPIOC.split();
        let gpiod = p.GPIOD.split();
        let gpioe = p.GPIOE.split();
        let gpiof = p.GPIOF.split();
        let gpiog = p.GPIOG.split();
        let gpioh = p.GPIOH.split();
        let gpioi = p.GPIOI.split();

        #[rustfmt::skip]
        let pins = fmc_pins! {
            A0: gpiof.pf0, A1: gpiof.pf1, A2: gpiof.pf2, A3: gpiof.pf3,
            A4: gpiof.pf4, A5: gpiof.pf5, A6: gpiof.pf12, A7: gpiof.pf13,
            A8: gpiof.pf14, A9: gpiof.pf15, A10: gpiog.pg0, A11: gpiog.pg1,
            Ba0: gpiog.pg4, Ba1: gpiog.pg5,
            D0: gpiod.pd14, D1: gpiod.pd15, D2: gpiod.pd0, D3: gpiod.pd1,
            D4: gpioe.pe7, D5: gpioe.pe8, D6: gpioe.pe9, D7: gpioe.pe10,
            D8: gpioe.pe11, D9: gpioe.pe12, D10: gpioe.pe13, D11: gpioe.pe14,
            D12: gpioe.pe15, D13: gpiod.pd8, D14: gpiod.pd9, D15: gpiod.pd10,
            D16: gpioh.ph8, D17: gpioh.ph9, D18: gpioh.ph10, D19: gpioh.ph11,
            D20: gpioh.ph12, D21: gpioh.ph13, D22: gpioh.ph14, D23: gpioh.ph15,
            D24: gpioi.pi0, D25: gpioi.pi1, D26: gpioi.pi2, D27: gpioi.pi3,
            D28: gpioi.pi6, D29: gpioi.pi7, D30: gpioi.pi9, D31: gpioi.pi10,
            Nbl0: gpioe.pe0, Nbl1: gpioe.pe1, Nbl2: gpioi.pi4, Nbl3: gpioi.pi5,
            Sdcke0: gpioh.ph2, Sdclk: gpiog.pg8,
            Sdncas: gpiog.pg15, Sdne0: gpioh.ph3,
            Sdnras: gpiof.pf11, Sdnwe: gpioc.pc0,
        };

        rtt_init_print!();

        rprintln!("Initializing SDRAM...\r");

        let mut sdram = p.FMC.sdram(pins, is42s32400f_6::Is42s32400f6 {}, &clocks);
        let len_bytes = 16 * 1024 * 1024;
        let len_words = len_bytes / mem::size_of::<u32>();
        let ram_ptr: *mut u32 = sdram.init(&mut delay);
        let ram = unsafe { slice::from_raw_parts_mut(ram_ptr, len_words) };

        rprintln!("Testing SDRAM...\r");

        let seed: u32 = 0x8675309D;
        let mut pattern = XorShift32::new(seed);

        // write our pattern
        for (addr, res) in ram.iter_mut().enumerate().take(len_words) {
            let val = pattern.next();

            if (addr & 0x1ffff) == 0 {
                rprintln!("Write: {:X} <- {:X}\r", (ram_ptr as usize) + addr, val);
            }

            *res = val;
        }

        // read back pattern
        pattern = XorShift32::new(seed);
        for (addr, &res) in ram.iter().enumerate().take(len_words) {
            let val = pattern.next();

            if (addr & 0x1ffff) == 0 {
                rprintln!("Read:  {:X} -> {:X}\r", (ram_ptr as usize) + addr, val);
            }

            if res != val {
                rprintln!(
                    "Error: {:X} -> {:X} != {:X}\r",
                    (ram_ptr as usize) + addr,
                    val,
                    res
                );
                break;
            }
        }

        rprintln!("Done!\r");
    }
    loop {
        continue;
    }
}
