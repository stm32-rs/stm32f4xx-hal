#![no_std]
#![no_main]

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    self,
    dma::{StreamX, StreamsTuple},
    interrupt,
    pac::{self, DMA1, USART2},
    prelude::*,
    serial::{
        self,
        dma::{RxDMA, SerialDma, TxDMA},
    },
    uart::{config::StopBits, Config, Serial},
};

/// Global variable for USART2 DMA handle.
static USART2_DMA: Mutex<
    RefCell<
        Option<
            SerialDma<
                USART2,
                TxDMA<USART2, StreamX<DMA1, 6>, 4>,
                RxDMA<USART2, StreamX<DMA1, 5>, 4>,
            >,
        >,
    >,
> = Mutex::new(RefCell::new(None));

/// Boolean flag to wait for DMA finish.
static DONE: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        // Enable DMA1.
        let dma1 = StreamsTuple::new(dp.DMA1);

        // Enable GPIOA.
        let gpioa = dp.GPIOA.split();

        // Configure USART2.
        let rx_2 = gpioa.pa3.into_alternate();
        let tx_2 = gpioa.pa2.into_alternate();
        let usart2 = Serial::new(
            dp.USART2,
            (tx_2, rx_2),
            Config::default()
                .baudrate(115200.bps())
                .parity_none()
                .stopbits(StopBits::STOP1)
                .dma(serial::config::DmaConfig::TxRx),
            &clocks,
        )
        .unwrap();

        // Make USART2 use DMA.
        let usart2_dma = usart2.use_dma(dma1.6, dma1.5);

        // Put DMA handle to the global variable.
        cortex_m::interrupt::free(|cs| {
            USART2_DMA.borrow(cs).borrow_mut().replace(usart2_dma);
        });

        // Enable interrupt
        unsafe {
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART2);
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA1_STREAM5);
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA1_STREAM6);
        }

        /*** Test Below ***/

        let mut buf = [0u8; 6];

        // Read 6 bytes to the buffer using the DMA non-blocking mode.
        cortex_m::interrupt::free(|cs| unsafe {
            if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
                usart2_dma.read_dma(&mut buf, None).unwrap();
            }
        });

        // Wait until DMA finishes.
        while !DONE.load(Ordering::SeqCst) {}
        DONE.store(false, Ordering::SeqCst);

        // Write the 6 bytes back using the DMA non-blocking mode.
        cortex_m::interrupt::free(|cs| unsafe {
            if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
                usart2_dma.write_dma(&buf, None).unwrap();
            }
        });

        // Wait until DMA finishes.
        while !DONE.load(Ordering::SeqCst) {}

        // Read 6 bytes to the buffer using the blocking mode.
        cortex_m::interrupt::free(|cs| {
            if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
                usart2_dma.read(&mut buf).unwrap();
            }
        });

        // Write the 6 bytes back using the blocking mode.
        cortex_m::interrupt::free(|cs| {
            if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
                usart2_dma.write(&buf).unwrap();
            }
        });
    }

    loop {}
}

#[interrupt]
#[allow(non_snake_case)]
fn USART2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
            usart2_dma.handle_error_interrupt();
        }
    });
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA1_STREAM5() {
    cortex_m::interrupt::free(|cs| {
        if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
            usart2_dma.handle_dma_interrupt();
            DONE.store(true, Ordering::SeqCst);
        }
    });
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA1_STREAM6() {
    cortex_m::interrupt::free(|cs| {
        if let Some(usart2_dma) = USART2_DMA.borrow(cs).borrow_mut().as_mut() {
            usart2_dma.handle_dma_interrupt();
            DONE.store(true, Ordering::SeqCst);
        }
    });
}
