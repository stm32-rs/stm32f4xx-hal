//! Serial echo using USART RX interrupt
//!
//! This example demonstrates interrupt-driven serial reception with echo.
//! Characters received on USART2 are echoed back.
//!
//! Tested on STM32F446 Nucleo board where USART2 is connected to the
//! ST-LINK virtual COM port (PA2=TX, PA3=RX).
//!
//! ```bash
//! cargo run --features stm32f446 --release --example serial-echo-irq
//! ```
//!
//! Example connection with picocom:
//!
//! ```bash
//! picocom -b 115200 /dev/ttyACM0
//! ```
//!
//! Type into the picocom terminal to see characters echoed.
//! You will also see the characters logged in the terminal connected to the board.
//!

#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::{
    interrupt, pac,
    prelude::*,
    rcc::Config,
    serial::{self, Serial},
};

type SerialType = Serial<pac::USART2>;
static SERIAL: Mutex<RefCell<Option<SerialType>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));
    let gpioa = dp.GPIOA.split(&mut rcc);

    let tx = gpioa.pa2;
    let rx = gpioa.pa3;

    let mut serial: SerialType = Serial::new(
        dp.USART2,
        (tx, rx),
        serial::Config::default().baudrate(115200.bps()),
        &mut rcc,
    )
    .unwrap();

    serial.listen(serial::Event::RxNotEmpty);

    cortex_m::interrupt::free(|cs| {
        SERIAL.borrow(cs).replace(Some(serial));
    });

    // Enable USART2 interrupt
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::USART2);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[interrupt]
fn USART2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(serial) = SERIAL.borrow(cs).borrow_mut().as_mut() {
            if let Ok(byte) = serial.read() {
                rprintln!("Received: {} ('{}')", byte, byte as char);

                // Echo received byte
                let _ = serial.write(byte);
            }
        }
    });
}
