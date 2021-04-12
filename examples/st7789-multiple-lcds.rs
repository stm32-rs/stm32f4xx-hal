//!
//! Demonstrates use of the Flexible Static Memory Controller to interface with four ST7789 LCD
//! controllers
//!
//! Hardware required: Some board with 4 LCDs all connected to the microcontroller's flexible static
//! memory controller
//!
//! We don't know if hardware like that actually exists. Still, this example shows how to compile
//! code that uses multiple address pins and multiple chip select pins.
//!

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_semihosting as _;

use stm32f4xx_hal::fsmc_lcd::{
    ChipSelect1, ChipSelect2, ChipSelect3, ChipSelect4, FsmcLcd, LcdPins, Timing,
};
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let _cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();
    let gpiof = dp.GPIOF.split();
    let gpiog = dp.GPIOG.split();

    // Pins connected to the LCD on the 32F412GDISCOVERY board
    let lcd_pins = LcdPins {
        data: (
            gpiod.pd14.into_alternate_af12(),
            gpiod.pd15.into_alternate_af12(),
            gpiod.pd0.into_alternate_af12(),
            gpiod.pd1.into_alternate_af12(),
            gpioe.pe7.into_alternate_af12(),
            gpioe.pe8.into_alternate_af12(),
            gpioe.pe9.into_alternate_af12(),
            gpioe.pe10.into_alternate_af12(),
            gpioe.pe11.into_alternate_af12(),
            gpioe.pe12.into_alternate_af12(),
            gpioe.pe13.into_alternate_af12(),
            gpioe.pe14.into_alternate_af12(),
            gpioe.pe15.into_alternate_af12(),
            gpiod.pd8.into_alternate_af12(),
            gpiod.pd9.into_alternate_af12(),
            gpiod.pd10.into_alternate_af12(),
        ),
        // Four address pins, one for each LCD
        // All of them will have the same level
        address: (
            gpiof.pf0.into_alternate_af12(),
            gpioe.pe2.into_alternate_af12(),
            gpioe.pe3.into_alternate_af12(),
            gpiof.pf14.into_alternate_af12(),
        ),
        read_enable: gpiod.pd4.into_alternate_af12(),
        write_enable: gpiod.pd5.into_alternate_af12(),
        // Four chip select pins, one for each LCD, controlled independently
        chip_select: (
            ChipSelect1(gpiod.pd7.into_alternate_af12()),
            ChipSelect2(gpiog.pg9.into_alternate_af12()),
            ChipSelect3(gpiog.pg10.into_alternate_af12()),
            ChipSelect4(gpiog.pg12.into_alternate_af12()),
        ),
    };

    let read_timing = Timing::default();
    let write_timing = Timing::default();

    let (_fsmc, mut lcds) = FsmcLcd::new(dp.FSMC, lcd_pins, &read_timing, &write_timing);
    // lcds is a tuple of four `Lcd` objects. Each one can be accessed independently.
    // This is just a basic indication of some things that can be done.
    lcds.0.write_command(37);
    lcds.1.write_command(38);
    lcds.2.write_command(39);
    lcds.3.write_command(40);

    loop {}
}
