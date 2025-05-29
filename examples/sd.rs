#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use panic_semihosting as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::Config,
    sdio::{ClockFreq, SdCard, Sdio},
};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.freeze(
        Config::hse(12.MHz())
            .require_pll48clk()
            .sysclk(168.MHz())
            .hclk(168.MHz())
            .pclk1(42.MHz())
            .pclk2(84.MHz()),
    );

    assert!(clocks.is_pll48clk_valid());

    let mut delay = core.SYST.delay(&rcc.clocks);

    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let d0 = gpioc.pc8.internal_pull_up(true);
    let d1 = gpioc.pc9.internal_pull_up(true);
    let d2 = gpioc.pc10.internal_pull_up(true);
    let d3 = gpioc.pc11.internal_pull_up(true);
    let clk = gpioc.pc12;
    let cmd = gpiod.pd2.internal_pull_up(true);
    let mut sdio: Sdio<SdCard> = Sdio::new(device.SDIO, (clk, cmd, d0, d1, d2, d3), &clocks);

    hprintln!("Waiting for card...");

    // Wait for card to be ready
    loop {
        match sdio.init(ClockFreq::F24Mhz) {
            Ok(_) => break,
            Err(_err) => (),
        }

        delay.delay_ms(1000);
    }

    let nblocks = sdio.card().map(|c| c.block_count()).unwrap_or(0);
    hprintln!("Card detected: nbr of blocks: {:?}", nblocks);

    // Read a block from the card and print the data
    let mut block = [0u8; 512];

    match sdio.read_block(0, &mut block) {
        Ok(()) => (),
        Err(err) => {
            hprintln!("Failed to read block: {:?}", err);
        }
    }

    for b in block.iter() {
        hprint!("{:X} ", b);
    }

    loop {
        continue;
    }
}
