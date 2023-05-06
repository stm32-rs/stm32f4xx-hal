#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [TIM2])]
mod app {

    use embedded_hal::spi::{Mode, Phase, Polarity};
    use hal::{
        dma::{
            config::DmaConfig, traits::StreamISR, MemoryToPeripheral, PeripheralToMemory, Stream0,
            Stream5, StreamsTuple, Transfer,
        },
        gpio::{gpioc::PC13, GpioExt, Output, PushPull},
        pac::{DMA1, SPI3},
        prelude::*,
        rcc::RccExt,
        spi::{RxCoupledSlave, SpiSlave, TxCoupledSlave},
    };
    use panic_semihosting as _;
    use systick_monotonic::*;

    use stm32f4xx_hal as hal;

    const ARRAY_SIZE: usize = 3;

    type TxTransfer = Transfer<
        Stream5<DMA1>,
        0,
        TxCoupledSlave<SPI3, false>,
        MemoryToPeripheral,
        &'static mut [u8; ARRAY_SIZE],
    >;

    type RxTransfer = Transfer<
        Stream0<DMA1>,
        0,
        RxCoupledSlave<SPI3, false>,
        PeripheralToMemory,
        &'static mut [u8; ARRAY_SIZE],
    >;

    #[shared]
    struct Shared {
        led: PC13<Output<PushPull>>,
        tx_transfer: TxTransfer,
        rx_transfer: RxTransfer,
    }

    #[local]
    struct Local {
        rx_buffer: Option<&'static mut [u8; ARRAY_SIZE]>,
        tx_buffer: Option<&'static mut [u8; ARRAY_SIZE]>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<1000>; // 1000 Hz / 1 ms granularity

    #[init()]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut core = cx.core;
        core.DWT.enable_cycle_counter();

        let device_peripherals: hal::pac::Peripherals = cx.device;

        let rcc = device_peripherals.RCC;
        let rcc = rcc.constrain();
        let _clocks = rcc.cfgr.sysclk(100.MHz()).pclk1(36.MHz()).freeze();

        let mono = Systick::new(core.SYST, 100_000_000);

        let gpioc = device_peripherals.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        let gpiob = device_peripherals.GPIOB;
        let spi = device_peripherals.SPI3;

        let gpiob = gpiob.split();

        let sck = gpiob.pb3.into_alternate();
        let miso = gpiob.pb4.into_alternate();
        let mosi = gpiob.pb5.into_alternate();

        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };

        let mut spi3 = SpiSlave::new(spi, (sck, miso, mosi, None), mode);
        spi3.set_internal_nss(false);

        let (tx, rx) = spi3.use_dma().txrx();

        let streams = StreamsTuple::new(device_peripherals.DMA1);
        let tx_stream = streams.5;
        let rx_stream = streams.0;

        let rx_buffer = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [0; ARRAY_SIZE]).unwrap();
        let tx_buffer = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [1,2,3]).unwrap();

        let mut rx_transfer = Transfer::init_peripheral_to_memory(
            rx_stream,
            rx,
            rx_buffer,
            None,
            DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(true),
        );

        let mut tx_transfer = Transfer::init_memory_to_peripheral(
            tx_stream,
            tx,
            tx_buffer,
            None,
            DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(true),
        );

        rx_transfer.start(|_rx| {});
        tx_transfer.start(|_tx| {});

        led.set_high();

        let rx_buffer2 = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [0; ARRAY_SIZE]).unwrap();
        let tx_buffer2 = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [4,5,6]).unwrap();

        (
            Shared {
                led,
                tx_transfer,
                rx_transfer,
            },
            Local {
                rx_buffer: Some(rx_buffer2),
                tx_buffer: Some(tx_buffer2),
            },
            init::Monotonics(mono),
        )
    }

    // The led lights up if the first byte we receive is a 1, it turns off otherwise
    #[task(binds = DMA1_STREAM0, shared = [rx_transfer, led], local = [rx_buffer])]
    fn on_receiving(cx: on_receiving::Context) {
        let on_receiving::Context { mut shared, local } = cx;
        if Stream0::<DMA1>::get_fifo_error_flag() {
            shared
                .rx_transfer
                .lock(|spi_dma| spi_dma.clear_fifo_error_interrupt());
        }
        if Stream0::<DMA1>::get_transfer_complete_flag() {
            shared
                .rx_transfer
                .lock(|spi_dma| spi_dma.clear_transfer_complete_interrupt());
            let filled_buffer = shared.rx_transfer.lock(|spi_dma| {
                let (result, _) = spi_dma
                    .next_transfer(local.rx_buffer.take().unwrap())
                    .unwrap();
                result
            });
            match filled_buffer[0] {
                1 => shared.led.lock(|led| led.set_low()),
                _ => shared.led.lock(|led| led.set_high()),
            }
            *local.rx_buffer = Some(filled_buffer);
        }
    }

    // We either send [1,2,3] or [4,5,6] depending on which buffer was loaded
    #[task(binds = DMA1_STREAM5, shared = [tx_transfer, led], local = [tx_buffer])]
    fn on_sending(cx: on_sending::Context) {
        let on_sending::Context { mut shared, local } = cx;
        if Stream5::<DMA1>::get_fifo_error_flag() {
            shared
                .tx_transfer
                .lock(|spi_dma| spi_dma.clear_fifo_error_interrupt());
        }
        if Stream5::<DMA1>::get_transfer_complete_flag() {
            shared
                .tx_transfer
                .lock(|spi_dma| spi_dma.clear_transfer_complete_interrupt());
            let filled_buffer = shared.tx_transfer.lock(|spi_dma| {
                let (result, _) = spi_dma
                    .next_transfer(local.tx_buffer.take().unwrap())
                    .unwrap();
                result
            });
            *local.tx_buffer = Some(filled_buffer);
        }
    }
}
