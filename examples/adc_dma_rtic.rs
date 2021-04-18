#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

use rtic::cyccnt::U32Ext;

use stm32f4xx_hal::{
    adc::{
        config::{AdcConfig, Dma, SampleTime, Scan, Sequence},
        Adc, Temperature,
    },
    dma::{config::DmaConfig, Channel0, PeripheralToMemory, Stream0, StreamsTuple, Transfer},
    prelude::*,
    signature::{VtempCal110, VtempCal30},
    stm32,
    stm32::{ADC1, DMA2},
};

const POLLING_PERIOD: u32 = 168_000_000 / 2;

type DMATransfer =
    Transfer<Stream0<DMA2>, Channel0, Adc<ADC1>, PeripheralToMemory, &'static mut [u16; 2]>;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        transfer: DMATransfer,
        buffer: Option<&'static mut [u16; 2]>,
    }

    #[init(schedule=[polling])]
    fn init(cx: init::Context) -> init::LateResources {
        let device: stm32::Peripherals = cx.device;

        let rcc = device.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .require_pll48clk()
            .sysclk(84.mhz())
            .hclk(84.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .freeze();

        let gpiob = device.GPIOB.split();
        let voltage = gpiob.pb1.into_analog();

        let dma = StreamsTuple::new(device.DMA2);

        let config = DmaConfig::default()
            .transfer_complete_interrupt(true)
            .memory_increment(true)
            .double_buffer(true);

        let adc_config = AdcConfig::default()
            .dma(Dma::Continuous)
            .scan(Scan::Enabled);

        let mut adc = Adc::adc1(device.ADC1, true, adc_config);
        adc.configure_channel(&Temperature, Sequence::One, SampleTime::Cycles_480);
        adc.configure_channel(&voltage, Sequence::Two, SampleTime::Cycles_480);
        adc.enable_temperature_and_vref();

        let first_buffer = cortex_m::singleton!(: [u16; 2] = [0; 2]).unwrap();
        let second_buffer = Some(cortex_m::singleton!(: [u16; 2] = [0; 2]).unwrap());
        let transfer = Transfer::init(dma.0, adc, first_buffer, None, config);

        let now = cx.start;
        cx.schedule.polling(now + POLLING_PERIOD.cycles()).unwrap();

        init::LateResources { transfer, buffer: second_buffer }
    }

    #[task(resources = [transfer], schedule = [polling])]
    fn polling(cx: polling::Context) {
        let transfer: &mut DMATransfer = cx.resources.transfer;
        transfer.start(|adc| {
            adc.start_conversion();
        });

        cx.schedule
            .polling(cx.scheduled + POLLING_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = DMA2_STREAM0, resources = [transfer, buffer])]
    fn dma(cx: dma::Context) {
        let transfer: &mut DMATransfer = cx.resources.transfer;

        let (buffer, _) = transfer.next_transfer(cx.resources.buffer.take().unwrap()).unwrap();
        let raw_temp = buffer[0];
        let raw_volt = buffer[1];

        *cx.resources.buffer = Some(buffer);

        let cal30 = VtempCal30::get().read() as f32;
        let cal110 = VtempCal110::get().read() as f32;

        let temperature = (110.0 - 30.0) * ((raw_temp as f32) - cal30) / (cal110 - cal30) + 30.0;
        let voltage = (raw_volt as f32) / ((2_i32.pow(12) - 1) as f32) * 3.3;

        hprintln!("temperature: {}, voltage: {}", temperature, voltage).unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
