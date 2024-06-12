#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32f4xx_hal as hal;

use hal::{
    pac,
    prelude::*,
    spi::{NoMiso, Spi},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = p.GPIOA.split();
    let gpioc = p.GPIOC.split();
    let gpiod = p.GPIOD.split();

    // Configure pin for button. This happens to be the pin for the USER button
    // on the NUCLEO-F746ZG board.
    let button = gpioc.pc13.into_floating_input();

    // Prepare pins for SPI
    let mut ncs = gpiod.pd14.into_push_pull_output();
    let sck = gpioa.pa5;
    let mosi = gpioa.pa7;

    // Set NCS pin to high (disabled) initially
    ncs.set_high();

    // Initialize SPI
    let mut spi = Spi::new(
        p.SPI1,
        (sck, NoMiso::new(), mosi),
        embedded_hal::spi::MODE_0,
        250.kHz(),
        &clocks,
    )
    .frame_size_16bit();

    // Use a button to control output via the Maxim Integrated MAX5214 DAC.
    loop {
        let data = if button.is_high() { 0xffff } else { 0x0000 };

        let word: u16 = (0b01 << 14) |   // write-through mode
            (data & 0x3fff); // data bits

        ncs.set_low();
        spi.write(&[word]).unwrap();
        ncs.set_high();
    }
}
