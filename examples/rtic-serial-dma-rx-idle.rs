// This example implement simple and safe method receiving data with unknown length by UART.
// The data received by using DMA, and IDLE event denote end of data packet.
// See https://github.com/MaJerle/stm32-usart-uart-dma-rx-tx for details.

// If you use big buffers, it is recommended to add memory pools (allocators) and use
// lock-free queues to send buffer without memcpy.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [TIM2])]
mod app {

    use hal::{
        dma::{config::DmaConfig, DmaFlag, PeripheralToMemory, Stream2, StreamsTuple, Transfer},
        pac::{DMA2, USART1},
        prelude::*,
        rcc::RccExt,
        serial,
    };
    use panic_semihosting as _;
    use systick_monotonic::*;

    use stm32f4xx_hal as hal;

    const BUFFER_SIZE: usize = 100;

    type RxTransfer = Transfer<
        Stream2<DMA2>,
        4,
        serial::Rx<USART1>,
        PeripheralToMemory,
        &'static mut [u8; BUFFER_SIZE],
    >;

    #[shared]
    struct Shared {
        #[lock_free]
        rx_transfer: RxTransfer,
    }

    #[local]
    struct Local {
        rx_buffer: Option<&'static mut [u8; BUFFER_SIZE]>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<1000>; // 1000 Hz / 1 ms granularity

    #[init(local = [
        rx_pool_memory: [u8; 400] = [0; 400],
    ])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let core = cx.core;
        let dp: hal::pac::Peripherals = cx.device;

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let mono = Systick::new(core.SYST, clocks.sysclk().to_Hz());

        let gpioa = dp.GPIOA.split();

        // Initialize UART with DMA events
        let rx_pin = gpioa.pa10;
        let mut rx = dp
            .USART1
            .rx(
                rx_pin,
                serial::Config::default()
                    .baudrate(9600.bps())
                    .dma(serial::config::DmaConfig::Rx),
                &clocks,
            )
            .unwrap();

        // Listen UART IDLE event, which will be call USART1 interrupt
        rx.listen_idle();

        let dma2 = StreamsTuple::new(dp.DMA2);

        // Note! It is better to use memory pools, such as heapless::pool::Pool. But it not work with embedded_dma yet.
        // See CHANGELOG of unreleased main branch and issue https://github.com/japaric/heapless/pull/362 for details.
        let rx_buffer1 = cortex_m::singleton!(: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap();
        let rx_buffer2 = cortex_m::singleton!(: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap();

        // Initialize and start DMA stream
        let mut rx_transfer = Transfer::init_peripheral_to_memory(
            dma2.2,
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

        (
            Shared { rx_transfer },
            Local {
                rx_buffer: Some(rx_buffer2),
            },
            init::Monotonics(mono),
        )
    }

    // Important! USART1 and DMA2_STREAM2 should the same interrupt priority!
    #[task(binds = USART1, priority=1, local = [rx_buffer],shared = [rx_transfer])]
    fn usart1(mut cx: usart1::Context) {
        let transfer = &mut cx.shared.rx_transfer;

        if transfer.is_idle() {
            // Calc received bytes count
            let bytes_count = BUFFER_SIZE - transfer.number_of_transfers() as usize;

            // Allocate new buffer
            let new_buffer = cx.local.rx_buffer.take().unwrap();

            // Replace buffer and restart DMA stream
            let (buffer, _) = transfer.next_transfer(new_buffer).unwrap();

            // Get slice for received bytes
            let _bytes = &buffer[..bytes_count];

            // Do something with received bytes
            // For example, parse it or send (buffer, bytes_count) to lock-free queue.

            // Free buffer
            *cx.local.rx_buffer = Some(buffer);
        }
    }

    #[task(binds = DMA2_STREAM2, priority=1,shared = [rx_transfer])]
    fn dma2_stream2(mut cx: dma2_stream2::Context) {
        let transfer = &mut cx.shared.rx_transfer;

        let flags = transfer.flags();
        transfer.clear_flags(DmaFlag::FifoError | DmaFlag::TransferComplete);
        if flags.is_transfer_complete() {
            // Buffer is full, but no IDLE received!
            // You can process this data or discard data (ignore transfer complete interrupt and wait IDLE).

            // Note! If you want process this data, it is recommended to use double buffering.
            // See Transfer::init_peripheral_to_memory for details.
        }
    }
}
