use super::*;

#[cfg(feature = "tim1")]
dma_map!(
    (Stream0<DMA2>:6, timer::DMAR<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_TRIG
    (Stream1<DMA2>:6, timer::CCR1<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream2<DMA2>:6, timer::CCR2<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH2
    (Stream3<DMA2>:6, timer::CCR1<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream4<DMA2>:6, timer::CCR4<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH4
    (Stream4<DMA2>:6, timer::DMAR<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_TRIG/COM
    (Stream5<DMA2>:6, timer::DMAR<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_UP
    (Stream6<DMA2>:0, timer::CCR1<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream6<DMA2>:0, timer::CCR2<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH2
    (Stream6<DMA2>:0, timer::CCR3<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH3
    (Stream6<DMA2>:6, timer::CCR3<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH3
);

#[cfg(feature = "tim5")]
dma_map!(
    (Stream0<DMA1>:6, timer::CCR3<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH3
    (Stream0<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_UP
    (Stream1<DMA1>:6, timer::CCR4<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4
    (Stream1<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_TRIG
    (Stream2<DMA1>:6, timer::CCR1<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH1
    (Stream3<DMA1>:6, timer::CCR4<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4
    (Stream3<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_TRIG
    (Stream4<DMA1>:6, timer::CCR2<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH2
    (Stream6<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_UP
);

dma_map!(
    (Stream0<DMA1>:1, pac::I2C1, [PeripheralToMemory]), //I2C1_RX
    (Stream2<DMA1>:7, pac::I2C2, [PeripheralToMemory]), //I2C2_RX
    (Stream3<DMA1>:0, pac::SPI2, [PeripheralToMemory]), //SPI2_RX
    (Stream3<DMA1>:7, pac::I2C2, [PeripheralToMemory]), //I2C2_RX
    (Stream4<DMA1>:0, pac::SPI2, [MemoryToPeripheral]), // SPI2_TX
    (Stream5<DMA1>:1, pac::I2C1, [PeripheralToMemory]), //I2C1_RX
    (Stream5<DMA1>:4, pac::USART2, [PeripheralToMemory]), //USART2_RX
    (Stream6<DMA1>:4, pac::USART2, [MemoryToPeripheral]), //USART2_TX
    (Stream7<DMA1>:7, pac::I2C2, [MemoryToPeripheral]), //I2C2_TX
    (Stream0<DMA2>:0, pac::ADC1, [PeripheralToMemory]), //ADC1
    (Stream0<DMA2>:3, pac::SPI1, [PeripheralToMemory]), //SPI1_RX
    (Stream1<DMA2>:5, pac::USART6, [PeripheralToMemory]), //USART6_RX
    (Stream2<DMA2>:3, pac::SPI1, [PeripheralToMemory]), //SPI1_RX
    (Stream2<DMA2>:4, pac::USART1, [PeripheralToMemory]), //USART1_RX
    (Stream2<DMA2>:5, pac::USART6, [PeripheralToMemory]), //USART6_RX
    (Stream4<DMA2>:0, pac::ADC1, [PeripheralToMemory]), //ADC1
    (Stream5<DMA2>:4, pac::USART1, [PeripheralToMemory]), //USART1_RX
    (Stream6<DMA2>:5, pac::USART6, [MemoryToPeripheral]), //USART6_TX
    (Stream7<DMA2>:4, pac::USART1, [MemoryToPeripheral]), //USART1_TX
    (Stream7<DMA2>:5, pac::USART6, [MemoryToPeripheral]), //USART6_TX
);

address!(
    (pac::ADC1, dr, u16),
    (pac::I2C1, dr, u8),
    (pac::I2C2, dr, u8),
    (pac::SPI1, dr, u8),
    (pac::SPI2, dr, u8),
    (pac::USART1, dr, u8),
    (pac::USART2, dr, u8),
    (pac::USART6, dr, u8),
);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f417",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream0<DMA1>:2, timer::CCR1<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH1
    (Stream2<DMA1>:5, timer::CCR4<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH4
    (Stream2<DMA1>:5, timer::DMAR<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_UP
    (Stream3<DMA1>:2, timer::CCR2<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH2
    (Stream4<DMA1>:5, timer::CCR1<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH1
    (Stream4<DMA1>:5, timer::DMAR<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_TRIG
    (Stream5<DMA1>:3, timer::CCR1<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Stream5<DMA1>:5, timer::CCR2<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH2
    (Stream6<DMA1>:2, timer::DMAR<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_UP
    (Stream6<DMA1>:3, timer::CCR2<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2
    (Stream6<DMA1>:3, timer::CCR4<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Stream7<DMA1>:2, timer::CCR3<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH3
    (Stream7<DMA1>:5, timer::CCR3<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH3
    (Stream0<DMA1>:0, pac::SPI3, [PeripheralToMemory]), //SPI3_RX
    (Stream2<DMA1>:0, pac::SPI3, [PeripheralToMemory]), //SPI3_RX
    (Stream4<DMA1>:3, pac::I2C3, [MemoryToPeripheral]), //I2C3_TX
    (Stream5<DMA1>:0, pac::SPI3, [MemoryToPeripheral]), //SPI3_TX
    (Stream7<DMA1>:0, pac::SPI3, [MemoryToPeripheral]), //SPI3_TX
);

#[cfg(feature = "i2c3")]
address!((pac::I2C3, dr, u8),);
#[cfg(feature = "spi3")]
address!((pac::SPI3, dr, u8),);

#[cfg(feature = "sdio")]
dma_map!(
    (Stream3<DMA2>:4, pac::SDIO, [MemoryToPeripheral | PeripheralToMemory]), //SDIO
    (Stream6<DMA2>:4, pac::SDIO, [MemoryToPeripheral | PeripheralToMemory]), //SDIO
);

#[cfg(feature = "sdio")]
address!((pac::SDIO, fifo, u32),);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f446",
))]
dma_map!(
    (Stream1<DMA1>:1, pac::I2C3, [PeripheralToMemory]), //I2C3_RX
    (Stream2<DMA1>:3, pac::I2C3, [PeripheralToMemory]), //I2C3_RX:DMA_CHANNEL_3
);

#[cfg(any(feature = "gpio-f401", feature = "gpio-f411",))]
dma_map!(
    (Stream1<DMA1>:3, timer::CCR3<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    (Stream1<DMA1>:3, timer::DMAR<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Stream7<DMA1>:3, timer::CCR4<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Stream7<DMA1>:3, timer::DMAR<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
))]
dma_map!(
    (Stream5<DMA1>:6, pac::I2C3, [MemoryToPeripheral]), //I2C3_TX:DMA_CHANNEL_6);
);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream6<DMA1>:1, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX
    (Stream7<DMA1>:1, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX
    (Stream3<DMA2>:3, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX
    (Stream5<DMA2>:3, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX
);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream0<DMA2>:4, pac::SPI4, [PeripheralToMemory]), //SPI4_RX
    (Stream1<DMA2>:4, pac::SPI4, [MemoryToPeripheral]), //SPI4_TX
    (Stream3<DMA2>:5, pac::SPI4, [PeripheralToMemory]), //SPI4_RX:DMA_CHANNEL_5
    (Stream4<DMA2>:5, pac::SPI4, [MemoryToPeripheral]), //SPI4_TX:DMA_CHANNEL_5
);

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
address!((pac::SPI4, dr, u8),);

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream0<DMA1>:4, pac::UART5, [PeripheralToMemory]), //UART5_RX
    (Stream2<DMA1>:4, pac::UART4, [PeripheralToMemory]), //UART4_RX
    (Stream4<DMA1>:4, pac::UART4, [MemoryToPeripheral]), //UART4_TX
    //(Stream6<DMA1>:7, pac::DAC2, [MemoryToPeripheral]), //DAC2
);

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
address!(
    (pac::UART4, dr, u8),
    (pac::UART5, dr, u8),
    //(pac::DAC, ??),
);

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream1<DMA1>:3, timer::DMAR<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Stream1<DMA1>:3, timer::CCR3<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    //(Stream2<DMA1>:1, timer::DMAR<pac::TIM7>, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP //dmar register appears to be missing
    //(Stream4<DMA1>:1, timer::DMAR<pac::TIM7>, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP //dmar register appears to be missing
    (Stream7<DMA1>:3, timer::DMAR<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Stream7<DMA1>:3, timer::CCR4<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Stream1<DMA2>:7, timer::DMAR<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_UP
    (Stream2<DMA2>:0, timer::CCR1<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1
    (Stream2<DMA2>:0, timer::CCR2<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH2
    (Stream2<DMA2>:0, timer::CCR3<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH3
    (Stream2<DMA2>:7, timer::CCR1<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1
    (Stream3<DMA2>:7, timer::CCR2<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH2
    (Stream4<DMA2>:7, timer::CCR3<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH3
    (Stream7<DMA2>:7, timer::CCR4<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH4
    (Stream7<DMA2>:7, timer::DMAR<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_COM/TRIG
    (Stream1<DMA1>:4, pac::USART3, [PeripheralToMemory]), //USART3_RX
    (Stream3<DMA1>:4, pac::USART3, [MemoryToPeripheral]), //USART3_TX
    (Stream4<DMA1>:7, pac::USART3, [MemoryToPeripheral]), //USART3_TX:DMA_CHANNEL_7
);

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
address!((pac::USART3, dr, u8),);

/*
DMAR register appears to be missing from TIM6 derived timers on these devices
   Not sure how _UP is supposed to work without DMAR or if this is just an SVD issue
#[cfg(feature = "tim6")]
dma_map!(
    (Stream1<DMA1>:7, timer::DMAR<pac::TIM6>, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP
);
*/

#[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469",))]
dma_map!(
    (Stream2<DMA1>:3, pac::I2C3, [PeripheralToMemory]), //I2C3_RX
    (Stream5<DMA2>:2, pac::CRYP, [PeripheralToMemory]), //CRYP_OUT
    (Stream6<DMA2>:2, pac::CRYP, [MemoryToPeripheral]), //CRYP_IN
    (Stream7<DMA2>:2, pac::HASH, [MemoryToPeripheral]), //HASH_IN
);

#[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469",))]
address!((pac::HASH, din, u32), (pac::CRYP, din, u32),);

/* Not sure how DAC works with DMA
#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f410",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream5<DMA1>:7, pac::DAC, [MemoryToPeripheral]), //DAC1
);
#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f410",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
address!(
    (pac::DAC, ??),
);
*/

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream7<DMA1>:4, pac::UART5, [MemoryToPeripheral]), //UART5_TX
    (Stream0<DMA2>:2, pac::ADC3, [PeripheralToMemory]), //ADC3
    (Stream1<DMA2>:2, pac::ADC3, [PeripheralToMemory]), //ADC3
    (Stream2<DMA2>:1, pac::ADC2, [PeripheralToMemory]), //ADC2
    (Stream3<DMA2>:1, pac::ADC2, [PeripheralToMemory]), //ADC2
);
#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
address!(
    (pac::ADC2, dr, u16),
    (pac::ADC3, dr, u16),
);

#[cfg(feature = "dcmi")]
dma_map!(
    (Stream1<DMA2>:1, pac::DCMI, [PeripheralToMemory]),  //DCMI
    (Stream7<DMA2>:1, pac::DCMI, [PeripheralToMemory]),  //DCMI
);
#[cfg(feature = "dcmi")]
address!(
    (pac::DCMI, dr, u32),
);

/* FMPI2C missing from peripheral crates (?)
#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f412",
    feature = "gpio-f413",
))]
dma_map!(
    (Stream0<DMA1>:7, pac::FMPI2C1, [PeripheralToMemory]), //FMPI2C1_RX
    (Stream1<DMA1>:2, pac::FMPI2C1, [MemoryToPeripheral]), //FMPI2C1_TX
    (Stream3<DMA1>:1, pac::FMPI2C1, [PeripheralToMemory]), //FMPI2C1_RX:DMA_CHANNEL_1
    (Stream7<DMA1>:4, pac::FMPI2C1, [MemoryToPeripheral]), //FMPI2C1_TX:DMA_CHANNEL_4
);

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f412",
    feature = "gpio-f413",
))]
address!(
    (pac::FMPI2C1, dr),
);
*/

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
))]
dma_map!(
    (Stream1<DMA1>:0, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX
    (Stream6<DMA1>:1, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX:DMA_CHANNEL_1
    (Stream7<DMA1>:1, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX:DMA_CHANNEL_1
    (Stream7<DMA1>:6, pac::USART2, [PeripheralToMemory]), //USART2_RX:DMA_CHANNEL_6
    (Stream2<DMA2>:2, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX
    (Stream3<DMA2>:3, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX:DMA_CHANNEL_3
    (Stream5<DMA2>:3, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX:DMA_CHANNEL_3
    (Stream5<DMA2>:5, pac::SPI5, [MemoryToPeripheral]), //SPI5_TX:DMA_CHANNEL_5
);

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
dma_map!(
    (Stream3<DMA2>:2, pac::SPI5, [PeripheralToMemory]), //SPI5_RX
    (Stream4<DMA2>:2, pac::SPI5, [MemoryToPeripheral]), //SPI5_TX
    (Stream5<DMA2>:7, pac::SPI5, [PeripheralToMemory]), //SPI5_RX:DMA_CHANNEL_7
    (Stream6<DMA2>:7, pac::SPI5, [MemoryToPeripheral]), //SPI5_TX:DMA_CHANNEL_7
);

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f427",
    feature = "gpio-f469",
))]
address!((pac::SPI5, dr, u8),);

#[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413",))]
dma_map!(
    (Stream4<DMA2>:4, pac::SPI4, [PeripheralToMemory]), //SPI4_RX
);
/*
#[cfg(feature = "dfsdm1")]
mod dfsdm1 {
    use super::*;

    #[cfg(feature = "gpio-f412")]
    use pac::DFSDM as DFSDM1;
    #[cfg(feature = "gpio-f413")]
    use pac::DFSDM1;

    dma_map!(
        (Stream0<DMA2>:7, FLT<DFSDM1, 0>, [PeripheralToMemory]), //DFSDM1_FLT0
        (Stream1<DMA2>:3, FLT<DFSDM1, 1>, [PeripheralToMemory]), //DFSDM1_FLT1
        (Stream4<DMA2>:3, FLT<DFSDM1, 1>, [PeripheralToMemory]), //DFSDM1_FLT1
        (Stream6<DMA2>:3, FLT<DFSDM1, 0>, [PeripheralToMemory]), //DFSDM1_FLT0:DMA_CHANNEL_3
    );

    unsafe impl<const F: u8> PeriAddress for FLT<DFSDM1, F> {
        #[inline(always)]
        fn address(&self) -> u32 {
            unsafe { &(*DFSDM1::ptr()).flt[F as usize].rdatar as *const _ as u32 }
        }

        type MemSize = u32;
    }
}

#[cfg(feature = "dfsdm2")]
dma_map!(
    (Stream0<DMA2>:8, FLT<pac::DFSDM2, 0>, [PeripheralToMemory]), //DFSDM2_FLT0
    (Stream1<DMA2>:8, FLT<pac::DFSDM2, 1>, [PeripheralToMemory]), //DFSDM2_FLT1
    (Stream2<DMA2>:8, FLT<pac::DFSDM2, 2>, [PeripheralToMemory]), //DFSDM2_FLT2
    (Stream3<DMA2>:8, FLT<pac::DFSDM2, 3>, [PeripheralToMemory]), //DFSDM2_FLT3
    (Stream4<DMA2>:8, FLT<pac::DFSDM2, 0>, [PeripheralToMemory]), //DFSDM2_FLT0
    (Stream5<DMA2>:8, FLT<pac::DFSDM2, 1>, [PeripheralToMemory]), //DFSDM2_FLT1
    (Stream6<DMA2>:8, FLT<pac::DFSDM2, 2>, [PeripheralToMemory]), //DFSDM2_FLT2
    (Stream7<DMA2>:8, FLT<pac::DFSDM2, 3>, [PeripheralToMemory]), //DFSDM2_FLT3
);
#[cfg(feature = "dfsdm2")]
unsafe impl<const F: u8> PeriAddress for FLT<pac::DFSDM2, F> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*DFSDM2::ptr()).flt[F as usize].rdatar as *const _ as u32 }
    }

    type MemSize = u32;
}
*/

#[cfg(feature = "quadspi")]
dma_map!(
    (Stream7<DMA2>:3, pac::QUADSPI, [MemoryToPeripheral | PeripheralToMemory]), //QUADSPI
);

#[cfg(feature = "quadspi")]
address!((pac::QUADSPI, dr, u32),);

#[cfg(any(feature = "gpio-f413", feature = "gpio-f427", feature = "gpio-f469",))]
dma_map!(
    (Stream0<DMA1>:5, pac::UART8, [MemoryToPeripheral]), //UART8_TX
    (Stream1<DMA1>:5, pac::UART7, [MemoryToPeripheral]), //UART7_TX
    (Stream3<DMA1>:5, pac::UART7, [PeripheralToMemory]), //UART7_RX
    (Stream6<DMA1>:5, pac::UART8, [PeripheralToMemory]), //UART8_RX
);

#[cfg(any(feature = "gpio-f413", feature = "gpio-f427", feature = "gpio-f469",))]
address!((pac::UART7, dr, u8), (pac::UART8, dr, u8),);

#[cfg(feature = "gpio-f413")]
dma_map!(
    (Stream7<DMA1>:8, pac::UART5, [MemoryToPeripheral]), //UART5_TX
    (Stream0<DMA2>:1, pac::UART9, [MemoryToPeripheral]), //UART9_TX
    (Stream0<DMA2>:5, pac::UART10, [PeripheralToMemory]), //UART10_RX
    (Stream3<DMA2>:9, pac::UART10, [PeripheralToMemory]), //UART10_RX:DMA_CHANNEL_9
    (Stream5<DMA2>:9, pac::UART10, [MemoryToPeripheral]), //UART10_TX
    (Stream7<DMA2>:0, pac::UART9, [PeripheralToMemory]), //UART9_RX
    (Stream7<DMA2>:6, pac::UART10, [MemoryToPeripheral]), //UART10_TX:DMA_CHANNEL_6
);
#[cfg(feature = "gpio-f413")]
address!(
    (pac::UART9, dr, u8),
    (pac::UART10, dr, u8),
);

#[cfg(feature = "aes")]
dma_map!(
    (Stream6<DMA2>:2, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Stream5<DMA2>:2, AES_OUT, [PeripheralToMemory]), //AES_OUT
);

#[cfg(feature = "sai1")]
mod sai1 {
    use super::*;
    #[cfg(not(any(feature = "gpio-f446", feature="gpio-f417", feature="svd-f427")))]
    use pac::SAI as SAI1;
    #[cfg(any(feature = "gpio-f446", feature="gpio-f417", feature="svd-f427"))]
    use pac::SAI1;

    dma_map!(
        (Stream1<DMA2>:0, SAICH<SAI1, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A
        (Stream3<DMA2>:0, SAICH<SAI1, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A
        (Stream4<DMA2>:1, SAICH<SAI1, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B
        (Stream5<DMA2>:0, SAICH<SAI1, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B:DMA_CHANNEL_0
    );

    #[cfg(feature = "sai1")]
    unsafe impl<const C: u8> PeriAddress for SAICH<SAI1, C> {
        #[inline(always)]
        fn address(&self) -> u32 {
            unsafe { &(*SAI1::ptr()).ch[C as usize].dr as *const _ as u32 }
        }

        type MemSize = u32;
    }
}
#[cfg(feature = "sai2")]
dma_map!(
    (Stream4<DMA2>:3, SAICH<pac::SAI2, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_A
    (Stream6<DMA2>:3, SAICH<pac::SAI2, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B
    (Stream7<DMA2>:0, SAICH<pac::SAI2, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B:DMA_CHANNEL_0
);

#[cfg(feature = "sai2")]
unsafe impl<const C: u8> PeriAddress for SAICH<pac::SAI2, C> {
    #[inline(always)]
    fn address(&self) -> u32 {
        unsafe { &(*pac::SAI2::ptr()).ch[C as usize].dr as *const _ as u32 }
    }

    type MemSize = u32;
}

#[cfg(feature = "spi6")]
dma_map!(
    (Stream5<DMA2>:1, pac::SPI6, [MemoryToPeripheral]), //SPI6_TX
    (Stream6<DMA2>:1, pac::SPI6, [PeripheralToMemory]), //SPI6_RX
);

#[cfg(feature = "spi6")]
address!((pac::SPI6, dr, u8),);

#[cfg(feature = "spdifrx")]
dma_map!(
    (Stream1<DMA1>:0, pac::SPDIFRX, [PeripheralToMemory]), //SPDIF_RX_DT
    //(Stream6<DMA1>:0, SPDIFRX_CS, [PeripheralToMemory]), //SPDIF_RX_CS
);

/*
#[cfg(any(
    feature = "gpio-f446",
))]
dma_map!(
    (Stream2<DMA1>:2, pac::FMPI2C1, [PeripheralToMemory]), //FMPI2C1_RX
    (Stream5<DMA1>:2, pac::FMPI2C1, [MemoryToPeripheral]), //FMPI2C1_TX
    (Stream6<DMA1>:0, pac::SPDIFRX, [PeripheralToMemory]), //SPDIF_RX_CS
);

#[cfg(any(
    feature = "gpio-f446",
))]
address!(
    (pac::FMPI2C1, ??),
);
*/

