#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    dma::{config, MemoryToPeripheral, Stream6, StreamsTuple, Transfer},
    pac,
    prelude::*,
    timer::Event,
};

const ARRAY_SIZE: usize = 10;

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let mut rcc = dp.RCC.constrain();

        let gpioa = dp.GPIOA.split(&mut rcc);
        let steams = StreamsTuple::new(dp.DMA2, &mut rcc);
        let stream = steams.6;

        let (_, (ch1, ch2, ..)) = dp.TIM1.pwm_us(100.micros(), &mut rcc);
        let mut ch1 = ch1.with(gpioa.pa8);
        let mut _ch2 = ch2.with(gpioa.pa9);

        let max_duty = ch1.get_max_duty();
        ch1.enable();

        let buffer = cortex_m::singleton!(: [u16; ARRAY_SIZE] = [1; ARRAY_SIZE]).unwrap();
        let step = max_duty / (ARRAY_SIZE as u16);

        for (i, b) in buffer.iter_mut().enumerate() {
            *b = step * (i as u16);
        }

        let mut transfer = Transfer::init_memory_to_peripheral(
            stream,
            ch1,
            buffer,
            None,
            config::DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(true),
        );

        transfer.start(|_tx| {});
    }

    loop {
        cortex_m::asm::nop();
    }
}
