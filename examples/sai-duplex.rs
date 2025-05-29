#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use stm32f4xx_hal::{self as hal, rcc::CFGR};

use crate::hal::{
    pac,
    prelude::*,
    sai::{Duplex, Protocol, SlotSize, Synchronization, WordSize},
};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // The following code configures the both sub-blocks of SAI for full-duplex communication using
    // I2S-encoded audio.

    // Initialize clocks.
    let rcc = p
        .RCC
        .freeze(CFGR::hse(8.MHz()).saia_clk(172.MHz()).saib_clk(172.MHz()));
    // Test that the SAI clock is suitable for 48000KHz audio.
    assert!(rcc.clocks.saia_clk() == Some(172.MHz()));
    assert!(rcc.clocks.saib_clk() == Some(172.MHz()));

    let gpioe = p.GPIOE.split();
    // SAIB is made synchronous to A.
    let (saia, saib) = p.SAI.split_sync_b();
    let protocol = Protocol {
        sync: Synchronization::I2S,
        word_size: WordSize::Bit16,
        slot_size: SlotSize::DataSize,
        num_slots: 2,
    };
    let tx = saia.master_tx(
        (Some(gpioe.pe2), gpioe.pe4, gpioe.pe5, gpioe.pe6),
        protocol,
        48.kHz(),
        &rcc.clocks,
    );
    let rx = saib.slave_rx(gpioe.pe3, protocol);

    let mut duplex = Duplex::new(rx, tx);
    duplex.start();
    loop {
        duplex.try_send(0xaaaa, 0xf0f0).ok();
        let _input = duplex.try_read();
    }

    /*
    // The following code configures the A sub-block of SAI as a master transmitter for PCM-encoded audio.

    // Initialize clocks.
    let rcc = p.RCC.freeze(CFGR::hse(8.MHz()).saia_clk(172.MHz()));
    // Test that the SAI clock is suitable for 48000KHz audio.
    assert!(clocks.saia_clk() == Some(172.MHz()));

    let gpioe = p.GPIOE.split();
    let (saia, _) = p.SAI.split();
    let protocol = Protocol {
        sync: Synchronization::PCMShortFrame,
        word_size: WordSize::Bit16,
        slot_size: SlotSize::DataSize,
        // Stereo audio, two slots per frame.
        num_slots: 2,
    };
    let mut tx = saia.master_tx(
        (gpioe.pe2, gpioe.pe4, gpioe.pe5, gpioe.pe6),
        protocol,
        48.kHz(),
        &clocks,
    );
    tx.start();
    loop {
        tx.try_send(0xaaaa, 0xf0f0).ok();
    }
    */
}
