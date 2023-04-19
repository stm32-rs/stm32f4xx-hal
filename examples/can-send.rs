//! Simple CAN example.
//! Requires a transceiver connected to PB8, 9 (CAN1) or PB5 PB6 (CAN2).

#![no_main]
#![no_std]

use panic_halt as _;

use bxcan::filter::Mask32;
use bxcan::{Fifo, Frame, StandardId};
use cortex_m_rt::entry;
use nb::block;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    // To meet CAN clock accuracy requirements an external crystal or ceramic
    // resonator must be used. The blue pill has a 8MHz external crystal.
    // Other boards might have a crystal with another frequency or none at all.
    rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpiob = dp.GPIOB.split();
    let mut can1 = {
        let rx = gpiob.pb8.into_alternate::<9>();
        let tx = gpiob.pb9.into_alternate();

        // let can = Can::new(dp.CAN1, (tx, rx));
        // or
        let can = dp.CAN1.can((tx, rx));

        bxcan::Can::builder(can)
            // APB1 (PCLK1): 8MHz, Bit rate: 500kBit/s, Sample Point 87.5%
            // Value was calculated with http://www.bittiming.can-wiki.info/
            .set_bit_timing(0x001c_0000)
            .enable()
    };

    // Configure filters so that can frames can be received.
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

    let _can2 = {
        let tx = gpiob.pb13.into_alternate();
        let rx = gpiob.pb12.into_alternate();

        let can = dp.CAN2.can((tx, rx));

        let can2 = bxcan::Can::builder(can)
            // APB1 (PCLK1): 8MHz, Bit rate: 500kBit/s, Sample Point 87.5%
            // Value was calculated with http://www.bittiming.can-wiki.info/
            .set_bit_timing(0x001c_0000)
            .enable();

        // A total of 28 filters are shared between the two CAN instances.
        // Split them equally between CAN1 and CAN2.
        filters.set_split(14);
        let mut slave_filters = filters.slave_filters();
        slave_filters.enable_bank(14, Fifo::Fifo0, Mask32::accept_all());
        can2
    };

    // Drop filters to leave filter configuration mode.
    drop(filters);

    // Select the interface.
    let mut can = can1;
    //let mut can = can2;

    // Echo back received packages in sequence.
    // See the `can-rtfm` example for an echo implementation that adheres to
    // correct frame ordering based on the transfer id.
    let mut test: [u8; 8] = [0; 8];
    let mut count: u8 = 0;
    let id: u16 = 0x500;

    test[1] = 1;
    test[2] = 2;
    test[3] = 3;
    test[4] = 4;
    test[5] = 5;
    test[6] = 6;
    test[7] = 7;
    loop {
        test[0] = count;
        let test_frame = Frame::new_data(StandardId::new(id).unwrap(), test);
        block!(can.transmit(&test_frame)).unwrap();
        if count < 255 {
            count += 1;
        } else {
            count = 0;
        }
    }
}
