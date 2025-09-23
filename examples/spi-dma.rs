#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;
use stm32f4xx_hal::dma::DmaFlag;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use embedded_hal_02::spi::{Mode, Phase, Polarity};
use static_cell::ConstStaticCell;
use stm32f4xx_hal::pac::interrupt;
use stm32f4xx_hal::{
    dma::{config, MemoryToPeripheral, Stream4, StreamsTuple, Transfer},
    gpio::Speed,
    pac::{self, SPI2},
    prelude::*,
    spi::*,
};

const ARRAY_SIZE: usize = 100;

type SpiDma =
    Transfer<Stream4<pac::DMA1>, 0, Tx<SPI2>, MemoryToPeripheral, &'static mut [u8; ARRAY_SIZE]>;

static G_TRANSFER: Mutex<RefCell<Option<SpiDma>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let mut rcc = dp.RCC.constrain();

        let steams = StreamsTuple::new(dp.DMA1, &mut rcc);
        let stream = steams.4;

        let gpiob = dp.GPIOB.split(&mut rcc);

        // Note. We set GPIO speed as VeryHigh to it corresponds to SPI frequency 3MHz.
        // Otherwise it may lead to the 'wrong last bit in every received byte' problem.
        let pb15 = gpiob
            .pb15
            .into_alternate()
            .speed(Speed::VeryHigh)
            .internal_pull_up(true);
        let pb13 = gpiob.pb13.into_alternate().speed(Speed::VeryHigh);

        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };

        let spi2 = Spi::new(
            dp.SPI2,
            (Some(pb13), SPI2::NoMiso, Some(pb15)),
            mode,
            3.MHz(),
            &mut rcc,
        );

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
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA1_STREAM4);
        }
    }

    loop {
        cortex_m::asm::nop();
    }
}

#[interrupt]
fn DMA1_STREAM4() {
    static mut TRANSFER: Option<SpiDma> = None;

    let transfer = TRANSFER.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TRANSFER.borrow(cs).replace(None).unwrap())
    });

    let flags = transfer.flags();
    // Its important to clear fifo errors as the transfer is paused until it is cleared
    transfer.clear_flags(DmaFlag::FifoError | DmaFlag::TransferComplete);
    if flags.is_transfer_complete() {
        static BUFFER: ConstStaticCell<[u8; ARRAY_SIZE]> = ConstStaticCell::new([0; ARRAY_SIZE]);
        let buffer = BUFFER.take();
        for (i, b) in buffer.iter_mut().enumerate() {
            *b = (i + 1) as u8;
        }
        transfer.next_transfer(buffer).unwrap();
    }
}
