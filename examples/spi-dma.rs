#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use embedded_hal::spi::{Mode, Phase, Polarity};
use stm32f4xx_hal::pac::interrupt;
use stm32f4xx_hal::{
    dma::{config, traits::StreamISR, MemoryToPeripheral, Stream4, StreamsTuple, Transfer},
    pac,
    prelude::*,
    spi::*,
};

const ARRAY_SIZE: usize = 100;

type SpiDma = Transfer<
    Stream4<pac::DMA1>,
    0,
    Tx<pac::SPI2>,
    MemoryToPeripheral,
    &'static mut [u8; ARRAY_SIZE],
>;

static G_TRANSFER: Mutex<RefCell<Option<SpiDma>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let steams = StreamsTuple::new(dp.DMA1);
        let stream = steams.4;

        let gpiob = dp.GPIOB.split();
        let pb15 = gpiob.pb15.into_alternate().internal_pull_up(true);
        let pb13 = gpiob.pb13.into_alternate();

        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };

        let spi2 = Spi::new(dp.SPI2, (pb13, NoMiso {}, pb15), mode, 3.MHz(), &clocks);

        let buffer = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [1; ARRAY_SIZE]).unwrap();

        for (i, b) in buffer.iter_mut().enumerate() {
            *b = i as u8;
        }

        let tx = spi2.use_dma().tx();

        let mut transfer = Transfer::init_memory_to_peripheral(
            stream,
            tx,
            buffer,
            None,
            config::DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(true),
        );

        transfer.start(|_tx| {});

        // Hand off transfer to interrupt handler
        cortex_m::interrupt::free(|cs| *G_TRANSFER.borrow(cs).borrow_mut() = Some(transfer));
        // Enable interrupt
        unsafe {
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA2_STREAM4);
        }
    }

    loop {
        cortex_m::asm::nop();
    }
}

#[interrupt]
fn DMA2_STREAM4() {
    static mut TRANSFER: Option<SpiDma> = None;

    let transfer = TRANSFER.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TRANSFER.borrow(cs).replace(None).unwrap())
    });

    // Its important to clear fifo errors as the transfer is paused until it is cleared
    if Stream4::<pac::DMA1>::get_fifo_error_flag() {
        transfer.clear_fifo_error_interrupt();
    }
    if Stream4::<pac::DMA1>::get_transfer_complete_flag() {
        transfer.clear_transfer_complete_interrupt();
        unsafe {
            static mut BUFFER: [u8; ARRAY_SIZE] = [0; ARRAY_SIZE];
            for (i, b) in BUFFER.iter_mut().enumerate() {
                *b = (i + 1) as u8;
            }
            transfer.next_transfer(&mut BUFFER).unwrap();
        }
    }
}
