#![deny(unsafe_code)]
#![no_main]
#![no_std]

// This example is designed to be run on the STM32F429I-DISCOVERY board

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use core::fmt::Write;
use hal::fmc::{
    SDRAMAddressWidth, SDRAMCommand, SDRAMCommandTargetBank, SDRAMConfig, SDRAMDataWidth,
    SDRAMTiming, UseSDRAMBank2, CAS_A, MODE_AW, NB_A, NC_A, NR_A, RPIPE_A, SDCLK_A,
};
use hal::serial::{config::Config as SerialConfig, config::StopBits::STOP1, Serial};
use hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at full speed for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .freeze();

        // get our USART going  -- usart1 is connected to the stlink on the discovery board
        let gpioa = dp.GPIOA.split();
        let u1tx = gpioa.pa9.into_alternate_af7();
        let u1rx = gpioa.pa10.into_alternate_af7();
        let serial_config = SerialConfig::default()
            .baudrate(115200.bps())
            .parity_none()
            .wordlength_8()
            .stopbits(STOP1);
        let usart1 = Serial::usart1(dp.USART1, (u1tx, u1rx), serial_config, clocks).unwrap();
        let (mut uart_tx, _uart_rx) = usart1.split();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();
        let sdcke1 = gpiob.pb5.into_alternate_af12();
        let sdne1 = gpiob.pb6.into_alternate_af12();
        let sdnwe = gpioc.pc0.into_alternate_af12();
        let mut sdram = SDRAMConfig {
            address_width: SDRAMAddressWidth::AW12,
            data_width: SDRAMDataWidth::DW16 {
                byte_enable_16: true,
            },
            number_of_column_bits: NC_A::BITS8,
            number_of_row_bits: NR_A::BITS12,
            module_bank_count: NB_A::NB4,
            sdclk_period: SDCLK_A::DIV2,
            rpipe_delay: RPIPE_A::CLOCKS1,
            write_protection: false,
            read_burst: false,
            timing: SDRAMTiming {
                cas_latency: CAS_A::CLOCKS3,
                t_mrd: 2,
                t_xsr: 7,
                t_ras: 4,
                t_rc: 7,
                t_wr: 3,
                t_rp: 2,
                t_rcd: 2,
            },
        }
        .configure(UseSDRAMBank2(sdcke1, sdne1), sdnwe);

        sdram.send_command(SDRAMCommand {
            command_mode: MODE_AW::CLOCKCONFIGURATIONENABLE,
            target_bank: SDRAMCommandTargetBank::Bank2,
            auto_refresh_number: 1,
            mode_register_definition: 0,
        });
        delay.delay_ms(1u32);

        sdram.send_command(SDRAMCommand {
            command_mode: MODE_AW::PALL,
            target_bank: SDRAMCommandTargetBank::Bank2,
            auto_refresh_number: 1,
            mode_register_definition: 0,
        });

        sdram.send_command(SDRAMCommand {
            command_mode: MODE_AW::AUTOREFRESHCOMMAND,
            target_bank: SDRAMCommandTargetBank::Bank2,
            auto_refresh_number: 4,
            mode_register_definition: 0,
        });

        // magic numbers for the mode reg on the stm32f429i-disco from the BSP lol
        // equivalent to:
        // SDRAM_MODEREG_BURST_LENGTH_1          | (0x0000)
        // SDRAM_MODEREG_BURST_TYPE_SEQUENTIAL   | (0x0000)
        // SDRAM_MODEREG_CAS_LATENCY_3           | (0x0030)
        // SDRAM_MODEREG_OPERATING_MODE_STANDARD | (0x0000)
        // SDRAM_MODEREG_WRITEBURST_MODE_SINGLE;   (0x2000)
        let mode_register_definition = 0x0230;

        sdram.send_command(SDRAMCommand {
            command_mode: MODE_AW::LOADMODEREGISTER,
            target_bank: SDRAMCommandTargetBank::Bank2,
            auto_refresh_number: 1,
            mode_register_definition,
        });

        // some kind of magic number, also taken straight from the 429i-disco BSP lol
        sdram.program_refresh_rate(1386);

        // credit where its due... this is the only unsafe :)
        let ram = sdram.sliced::<usize>(0x0080_0000);

        let mut had_sanity_check_failure = false;
        for n in 0..ram.len() {
            // give us some reassurance that we're actually accessing the memory.
            if n == 1 {
                write!(
                    uart_tx,
                    "if you can read this. we didn't hard fault! :)\r\n",
                )
                .unwrap();
            }
            ram[n] = n;
            // commenting this feedback out to speed up the process.
            //write!(uart_tx, "wrote to pos {} // {:p}\r\n", n, &ram[n]).unwrap();
        }
        write!(uart_tx, "we finished writing! now to readback test!\r\n").unwrap();
        for n in 0..ram.len() {
            let val = ram[n];
            if val != n {
                had_sanity_check_failure = true;
                write!(
                    uart_tx,
                    "oh noes -- we had a mismatch at pos {} // {:p}. got {} instead",
                    n, &ram[n], val
                )
                .unwrap();
            }
        }
        if had_sanity_check_failure {
            write!(uart_tx, "oh noes -- rams are borked\r\n").unwrap();
        } else {
            write!(uart_tx, "flawless victory!\r\n").unwrap();
        }
    }

    loop {}
}
