#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;
use stm32f4xx_hal::dma::{DmaFlag, PeripheralToMemory, Stream1};

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f4xx_hal::dma::config::DmaConfig;
use stm32f4xx_hal::pac::Interrupt;
use stm32f4xx_hal::pac::{interrupt, DMA1};
use stm32f4xx_hal::serial::{config::StopBits, Config, Rx, Serial};
use stm32f4xx_hal::{
    dma::{StreamsTuple, Transfer},
    pac,
    prelude::*,
    serial,
};

// uart buffer size
const UART_BUFFER_SIZE: usize = 128;

// Simple ring buffer
pub struct Buffer {
    buffer: [u8; UART_BUFFER_SIZE],
    write_idx: usize,
    read_idx: usize,
}

impl Buffer {
    pub(crate) const fn new() -> Buffer {
        Buffer {
            buffer: [0; UART_BUFFER_SIZE],
            write_idx: 0,
            read_idx: 0,
        }
    }

    pub fn push(&mut self, data: u8) {
        self.buffer[self.write_idx] = data;
        self.write_idx = (self.write_idx + 1) % UART_BUFFER_SIZE;
    }

    pub fn read(&mut self) -> Option<u8> {
        if self.write_idx != self.read_idx {
            let data = self.buffer[self.read_idx];
            self.read_idx = (self.read_idx + 1) % UART_BUFFER_SIZE;
            Some(data)
        } else {
            None
        }
    }
}

// dma type, needs to be adapted for uart and dma channel
type UartDma = Transfer<
    Stream1<DMA1>,
    4,
    Rx<pac::USART3>,
    PeripheralToMemory,
    &'static mut [u8; UART_BUFFER_SIZE],
>;

// shared dma reference
pub static G_TRANSFER: Mutex<RefCell<Option<UartDma>>> = Mutex::new(RefCell::new(None));

// shared uart3 reference
pub static G_UART3_BUFFER: Mutex<RefCell<Option<Buffer>>> = Mutex::new(RefCell::new(None));

// shared TX reference
pub static G_UART3_TX: Mutex<RefCell<Option<serial::Tx<pac::USART3>>>> =
    Mutex::new(RefCell::new(None));

// dma buffer
pub static mut RX_UART3_BUFFER: [u8; UART_BUFFER_SIZE] = [0; UART_BUFFER_SIZE];

// a wrapper function that reads out of the uart ring buffer
pub fn uart3_read_until(eol: u8) -> Option<[u8; UART_BUFFER_SIZE]> {
    let r = cortex_m::interrupt::free(|cs| {
        if let Some(buffer) = G_UART3_BUFFER.borrow(cs).borrow_mut().as_mut() {
            let mut buf = [0; UART_BUFFER_SIZE];
            let mut i = 0;
            while let Some(byte) = buffer.read() {
                if byte == eol {
                    break;
                }
                if i < UART_BUFFER_SIZE - 1 {
                    buf[i] = byte;
                } else {
                    break;
                }
                i += 1;
            }
            if buf[0] == 0 {
                return None;
            }
            Some(buf)
        } else {
            None
        }
    });
    r
}

// a wrapper function for uart write
pub fn uart3_write(data: &[u8]) -> Result<(), serial::Error> {
    cortex_m::interrupt::free(|cs| {
        let ret = if let Some(uart) = G_UART3_TX.borrow(cs).borrow_mut().as_mut() {
            let non_zero_len = data
                .iter()
                .rposition(|&x| x != 0)
                .map(|idx| idx + 1)
                .unwrap_or(0);
            // Create a custom slice with only non-zero elements
            uart.bwrite_all(&data[0..non_zero_len])?;
            uart.bflush()
        } else {
            Err(serial::Error::Other)
        };
        ret
    })
}

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let dma1 = StreamsTuple::new(dp.DMA1);

        let gpiod = dp.GPIOD.split();

        // configure UART, it is important to configure this to use DMA
        let rx_3 = gpiod.pd9.into_alternate();
        let tx_3 = gpiod.pd8.into_alternate();
        let uart3 = Serial::new(
            dp.USART3,
            (tx_3, rx_3),
            Config::default()
                .baudrate(9600.bps())
                .parity_none()
                .stopbits(StopBits::STOP1)
                .dma(serial::config::DmaConfig::Rx),
            &clocks,
        )
        .unwrap();

        // Note! It is better to use memory pools, such as heapless::pool::Pool. But it not work with embedded_dma yet.
        // See CHANGELOG of unreleased main branch and issue https://github.com/japaric/heapless/pull/362 for details.
        let rx_buffer1 =
            cortex_m::singleton!(: [u8; UART_BUFFER_SIZE] = [0; UART_BUFFER_SIZE]).unwrap();
        let _rx_buffer2 =
            cortex_m::singleton!(: [u8; UART_BUFFER_SIZE] = [0; UART_BUFFER_SIZE]).unwrap();

        let (tx, mut rx) = uart3.split();

        rx.listen_idle();

        cortex_m::interrupt::free(|cs| *G_UART3_TX.borrow(cs).borrow_mut() = Some(tx));

        cortex_m::interrupt::free(|cs| {
            *G_UART3_BUFFER.borrow(cs).borrow_mut() = Some(Buffer::new());
        });
        // Initialize and start DMA stream
        let mut rx_transfer = Transfer::init_peripheral_to_memory(
            dma1.1,
            rx,
            rx_buffer1,
            None,
            DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(true),
        );

        rx_transfer.start(|_rx| {});

        cortex_m::interrupt::free(|cs| *G_TRANSFER.borrow(cs).borrow_mut() = Some(rx_transfer));

        // Enable interrupt
        unsafe {
            cortex_m::peripheral::NVIC::unmask(Interrupt::USART3);
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA1_STREAM1);
        }
    }

    // to test this, connect the RX and TX pins together for a loopback
    // - so you can listen to what you just sent.

    uart3_write(b"hello world\r").expect("Failed to write to UART3");
    let _response = uart3_read_until(b'\r').expect("Failed to read from UART3");
    // ... do something with response

    loop {
        cortex_m::asm::nop();
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn USART3() {
    cortex_m::interrupt::free(|cs| {
        if let Some(transfer) = G_TRANSFER.borrow(cs).borrow_mut().as_mut() {
            if transfer.is_idle() {
                // Calc received bytes count
                let bytes_count = UART_BUFFER_SIZE - transfer.number_of_transfers() as usize;
                unsafe {
                    let mut buffer = [0; UART_BUFFER_SIZE];
                    match transfer.next_transfer(&mut RX_UART3_BUFFER) {
                        Ok((b, _)) => buffer = *b,
                        Err(_err) => {}
                    }
                    if let Some(ring_buffer) = G_UART3_BUFFER.borrow(cs).borrow_mut().as_mut() {
                        for i in 0..bytes_count {
                            ring_buffer.push(buffer[i]);
                        }
                    }
                }
            }
            transfer.clear_idle_interrupt();
        }
    });
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA1_STREAM1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(transfer) = G_TRANSFER.borrow(cs).borrow_mut().as_mut() {
            // Its important to clear fifo errors as the transfer is paused until it is cleared
            transfer.clear_flags(DmaFlag::FifoError | DmaFlag::TransferComplete);
        }
    });
}
