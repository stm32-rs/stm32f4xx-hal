#![no_main]
#![no_std]

use panic_halt as _;

use crate::hal::spi::{Mode, Phase, Polarity};
use crate::hal::{gpio::Pull, pac, prelude::*};
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    let gpioa = p.GPIOA.split();

    let sck = gpioa.pa5.internal_resistor(Pull::Up);
    let miso = gpioa.pa6.internal_resistor(Pull::Down);
    let mosi = gpioa.pa7.internal_resistor(Pull::Down);

    // clock speed is determined by the master
    let nss = gpioa.pa4.internal_resistor(Pull::Up);
    let mut spi = p
        .SPI1
        .spi_slave((Some(sck), Some(miso), Some(mosi), Some(nss)), MODE);
    // alternativelly you could use software `chip select`
    // let mut spi = SpiSlave::new(p.SPI1, (sck, miso, mosi, None), MODE);
    // spi.set_internal_nss(false);

    let mut data = [0x1];
    // this will block until the master starts the clock
    spi.transfer_in_place(&mut data).unwrap();

    // when you reach this breakpoint you'll be able to inspect the variable `data` which contains the
    // data sent by the master
    asm::bkpt();

    loop {
        continue;
    }
}
