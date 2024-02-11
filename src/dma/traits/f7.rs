use super::*;

dma_map! {
    (Stream0<DMA1>:0, pac::SPI3, [PeripheralToMemory]), //SPI3_RX
    (Stream2<DMA1>:0, pac::SPI3, [PeripheralToMemory]), //SPI3_RX
    (Stream3<DMA1>:0, pac::SPI2, [PeripheralToMemory]), //SPI2_RX
    (Stream4<DMA1>:0, pac::SPI2, [MemoryToPeripheral]), //SPI2_TX
    (Stream5<DMA1>:0, pac::SPI3, [MemoryToPeripheral]), //SPI3_TX
    (Stream7<DMA1>:0, pac::SPI3, [MemoryToPeripheral]), //SPI3_TX

    (Stream0<DMA1>:1, pac::I2C1, [PeripheralToMemory]), //I2C1_RX
    (Stream1<DMA1>:1, pac::I2C3, [PeripheralToMemory]), //I2C3_RX
    (Stream2<DMA1>:1, timer::DMAR<pac::TIM7>, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP
    (Stream4<DMA1>:1, timer::DMAR<pac::TIM7>, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP
    (Stream5<DMA1>:1, pac::I2C1, [PeripheralToMemory]), //I2C1_RX
    (Stream6<DMA1>:1, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX
    (Stream7<DMA1>:1, pac::I2C1, [MemoryToPeripheral]), //I2C1_TX

    (Stream0<DMA1>:2, timer::CCR1<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH1
    (Stream3<DMA1>:2, timer::CCR2<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH2
    (Stream6<DMA1>:2, timer::DMAR<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_UP
    (Stream7<DMA1>:2, timer::CCR3<pac::TIM4>, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH3

    (Stream1<DMA1>:3, timer::CCR3<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    (Stream1<DMA1>:3, timer::DMAR<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Stream2<DMA1>:3, pac::I2C3, [PeripheralToMemory]), //I2C3_RX
    (Stream4<DMA1>:3, pac::I2C3, [MemoryToPeripheral]), //I2C3_TX
    (Stream5<DMA1>:3, timer::CCR1<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Stream6<DMA1>:3, timer::CCR2<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2
    (Stream6<DMA1>:3, timer::CCR4<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Stream7<DMA1>:3, timer::CCR4<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Stream7<DMA1>:3, timer::DMAR<pac::TIM2>, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP

    (Stream0<DMA1>:4, pac::UART5, [PeripheralToMemory]), //UART5_RX
    (Stream1<DMA1>:4, pac::USART3, [PeripheralToMemory]), //USART3_RX
    (Stream2<DMA1>:4, pac::UART4, [PeripheralToMemory]), //UART4_RX
    (Stream3<DMA1>:4, pac::USART3, [MemoryToPeripheral]), //USART3_TX
    (Stream4<DMA1>:4, pac::UART4, [MemoryToPeripheral]), //UART4_TX
    (Stream5<DMA1>:4, pac::USART2, [PeripheralToMemory]), //USART2_RX
    (Stream6<DMA1>:4, pac::USART2, [MemoryToPeripheral]), //USART2_TX
    (Stream7<DMA1>:4, pac::UART5, [MemoryToPeripheral]), //UART5_TX

    (Stream0<DMA1>:5, pac::UART8, [MemoryToPeripheral]), //UART8_TX
    (Stream1<DMA1>:5, pac::UART7, [MemoryToPeripheral]), //UART7_TX
    (Stream2<DMA1>:5, timer::CCR4<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH4
    (Stream2<DMA1>:5, timer::DMAR<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_UP
    (Stream3<DMA1>:5, pac::UART7, [PeripheralToMemory]), //UART7_RX
    (Stream4<DMA1>:5, timer::CCR1<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH1
    (Stream4<DMA1>:5, timer::DMAR<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_TRIG
    (Stream5<DMA1>:5, timer::CCR2<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH2
    (Stream6<DMA1>:5, pac::UART8, [PeripheralToMemory]), //UART8_RX
    (Stream7<DMA1>:5, timer::CCR3<pac::TIM3>, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH3

    (Stream0<DMA1>:6, timer::CCR3<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH3
    (Stream0<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_UP
    (Stream1<DMA1>:6, timer::CCR4<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4
    (Stream1<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_TRIG
    (Stream2<DMA1>:6, timer::CCR1<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH1
    (Stream3<DMA1>:6, timer::CCR4<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4
    (Stream3<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_TRIG
    (Stream4<DMA1>:6, timer::CCR2<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH2
    (Stream6<DMA1>:6, timer::DMAR<pac::TIM5>, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_UP

    (Stream1<DMA1>:7, timer::DMAR<pac::TIM6>, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP
    (Stream2<DMA1>:7, pac::I2C2, [PeripheralToMemory]),     //I2C2_RX
    (Stream3<DMA1>:7, pac::I2C2, [PeripheralToMemory]), //I2C2_RX
    (Stream4<DMA1>:7, pac::USART3, [MemoryToPeripheral]), //USART3_TX:DMA_CHANNEL_7
    (Stream5<DMA1>:7, pac::DAC, [MemoryToPeripheral]), //DAC1
    //(Stream6<DMA1>:7, pac::DAC2, [MemoryToPeripheral]), //DAC2
    (Stream7<DMA1>:7, pac::I2C2, [MemoryToPeripheral]), //I2C2_TX


    (Stream0<DMA2>:0, pac::ADC1, [PeripheralToMemory]), //ADC1
    (Stream1<DMA2>:0, SAICH<pac::SAI1, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A
    (Stream2<DMA2>:0, timer::CCR1<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1
    (Stream2<DMA2>:0, timer::CCR2<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH2
    (Stream2<DMA2>:0, timer::CCR3<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH3
    (Stream3<DMA2>:0, SAICH<pac::SAI1, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A
    (Stream4<DMA2>:0, pac::ADC1, [PeripheralToMemory]), //ADC1
    (Stream5<DMA2>:0, SAICH<pac::SAI1, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B:DMA_CHANNEL_0
    (Stream6<DMA2>:0, timer::CCR1<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream6<DMA2>:0, timer::CCR2<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH2
    (Stream6<DMA2>:0, timer::CCR3<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH3
    (Stream7<DMA2>:0, SAICH<pac::SAI2, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B:DMA_CHANNEL_0

    (Stream2<DMA2>:1, pac::ADC2, [PeripheralToMemory]), //ADC2
    (Stream3<DMA2>:1, pac::ADC2, [PeripheralToMemory]), //ADC2
    (Stream4<DMA2>:1, SAICH<pac::SAI1, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B

    (Stream0<DMA2>:2, pac::ADC3, [PeripheralToMemory]), //ADC3
    (Stream1<DMA2>:2, pac::ADC3, [PeripheralToMemory]), //ADC3
    (Stream3<DMA2>:2, pac::SPI5, [PeripheralToMemory]), //SPI5_RX
    (Stream4<DMA2>:2, pac::SPI5, [MemoryToPeripheral]), //SPI5_TX

    (Stream0<DMA2>:3, pac::SPI1, [PeripheralToMemory]), //SPI1_RX
    (Stream2<DMA2>:3, pac::SPI1, [PeripheralToMemory]), //SPI1_RX
    (Stream3<DMA2>:3, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX
    (Stream4<DMA2>:3, SAICH<pac::SAI2, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_A
    (Stream5<DMA2>:3, pac::SPI1, [MemoryToPeripheral]), //SPI1_TX
    (Stream6<DMA2>:3, SAICH<pac::SAI2, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B
    (Stream7<DMA2>:3, pac::QUADSPI, [MemoryToPeripheral | PeripheralToMemory]), //QUADSPI

    (Stream0<DMA2>:4, pac::SPI4, [PeripheralToMemory]), //SPI4_RX
    (Stream1<DMA2>:4, pac::SPI4, [MemoryToPeripheral]), //SPI4_TX
    (Stream2<DMA2>:4, pac::USART1, [PeripheralToMemory]), //USART1_RX
    (Stream3<DMA2>:4, pac::SDMMC1, [MemoryToPeripheral | PeripheralToMemory]), //SDMMC1
    (Stream5<DMA2>:4, pac::USART1, [PeripheralToMemory]), //USART1_RX
    (Stream6<DMA2>:4, pac::SDMMC1, [MemoryToPeripheral | PeripheralToMemory]), //SDMMC1
    (Stream7<DMA2>:4, pac::USART1, [MemoryToPeripheral]), //USART1_TX

    (Stream1<DMA2>:5, pac::USART6, [PeripheralToMemory]), //USART6_RX
    (Stream2<DMA2>:5, pac::USART6, [PeripheralToMemory]), //USART6_RX
    (Stream3<DMA2>:5, pac::SPI4, [PeripheralToMemory]), //SPI4_RX:DMA_CHANNEL_5
    (Stream4<DMA2>:5, pac::SPI4, [MemoryToPeripheral]), //SPI4_TX:DMA_CHANNEL_5
    (Stream6<DMA2>:5, pac::USART6, [MemoryToPeripheral]), //USART6_TX
    (Stream7<DMA2>:5, pac::USART6, [MemoryToPeripheral]), //USART6_TX

    (Stream0<DMA2>:6, timer::DMAR<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_TRIG
    (Stream1<DMA2>:6, timer::CCR1<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream2<DMA2>:6, timer::CCR2<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH2
    (Stream3<DMA2>:6, timer::CCR1<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream4<DMA2>:6, timer::CCR4<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH4
    (Stream4<DMA2>:6, timer::DMAR<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_TRIG/COM
    (Stream5<DMA2>:6, timer::DMAR<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_UP
    (Stream6<DMA2>:6, timer::CCR3<pac::TIM1>, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH3

    (Stream1<DMA2>:7, timer::DMAR<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_UP
    (Stream2<DMA2>:7, timer::CCR1<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1
    (Stream3<DMA2>:7, timer::CCR2<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH2
    (Stream4<DMA2>:7, timer::CCR3<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH3
    (Stream5<DMA2>:7, pac::SPI5, [PeripheralToMemory]), //SPI5_RX:DMA_CHANNEL_7
    (Stream6<DMA2>:7, pac::SPI5, [MemoryToPeripheral]), //SPI5_TX:DMA_CHANNEL_7
    (Stream7<DMA2>:7, timer::CCR4<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH4
    (Stream7<DMA2>:7, timer::DMAR<pac::TIM8>, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_COM/TRIG
}

#[cfg(feature = "spdifrx")]
dma_map!(
    (Stream1<DMA1>:0, pac::SPDIFRX, [PeripheralToMemory]), //SPDIF_RX_DT
    //(Stream6<DMA1>:0, SPDIFRX_CS, [PeripheralToMemory]), //SPDIF_RX_CS
);

#[cfg(feature = "i2c4")]
dma_map!(
    (Stream2<DMA1>:2, pac::I2C4, [PeripheralToMemory]), //I2C4_RX
    (Stream5<DMA1>:2, pac::I2C4, [PeripheralToMemory]), //I2C4_RX
);

#[cfg(feature = "sdmmc2")]
dma_map!(
    (Stream0<DMA2>:11, pac::SDMMC2, [MemoryToPeripheral | PeripheralToMemory]), //SDMMC2
    (Stream5<DMA2>:11, pac::SDMMC2, [MemoryToPeripheral | PeripheralToMemory]), //SDMMC2
);

#[cfg(feature = "dcmi")]
dma_map!(
    (Stream1<DMA2>:1, pac::DCMI, [PeripheralToMemory]),  //DCMI
    (Stream7<DMA2>:1, pac::DCMI, [PeripheralToMemory]),  //DCMI
);
#[cfg(feature = "dcmi")]
address!((pac::DCMI, dr, u32),);

#[cfg(feature = "spi6")]
dma_map!(
    (Stream5<DMA2>:1, pac::SPI6, [MemoryToPeripheral]), //SPI6_TX
    (Stream6<DMA2>:1, pac::SPI6, [PeripheralToMemory]), //SPI6_RX
);

#[cfg(feature = "spi6")]
address!((pac::SPI6, dr, u8),);

#[cfg(feature = "aes")]
dma_map!(
    (Stream6<DMA2>:2, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Stream5<DMA2>:2, AES_OUT, [PeripheralToMemory]), //AES_OUT
);

#[cfg(feature = "cryp")]
dma_map!(
    (Stream6<DMA2>:2, CRYP_IN, [MemoryToPeripheral]), //CRYP_IN
    (Stream5<DMA2>:2, CRYP_OUT, [PeripheralToMemory]), //CRYP_OUT
);

/*#[cfg(feature = "hash")]
dma_map!(
    (Stream7<DMA2>:2, HASH_IN, [MemoryToPeripheral]), //HASH_IN
);*/

#[cfg(feature = "gpio-f76x")]
dma_map!(
    (Stream0<DMA1>:8, pac::I2C3, [MemoryToPeripheral]), //I2C3_TX
    (Stream1<DMA1>:8, pac::I2C4, [PeripheralToMemory]), //I2C4_RX
    (Stream4<DMA1>:8, pac::I2C2, [MemoryToPeripheral]), //I2C2_TX
    (Stream6<DMA1>:8, pac::I2C4, [MemoryToPeripheral]), //I2C4_TX
    (Stream1<DMA1>:9, pac::SPI2, [PeripheralToMemory]), //SPI2_RX
    (Stream6<DMA1>:9, pac::SPI2, [MemoryToPeripheral]), //SPI2_TX


    (Stream0<DMA2>:8, FLT<pac::DFSDM1, 0>, [PeripheralToMemory]), //DFSDM1_FLT0
    (Stream1<DMA2>:8, FLT<pac::DFSDM1, 1>, [PeripheralToMemory]), //DFSDM1_FLT1
    (Stream2<DMA2>:8, FLT<pac::DFSDM1, 2>, [PeripheralToMemory]), //DFSDM1_FLT2
    (Stream3<DMA2>:8, FLT<pac::DFSDM1, 3>, [PeripheralToMemory]), //DFSDM1_FLT3
    (Stream4<DMA2>:8, FLT<pac::DFSDM1, 0>, [PeripheralToMemory]), //DFSDM1_FLT0
    (Stream5<DMA2>:8, FLT<pac::DFSDM1, 1>, [PeripheralToMemory]), //DFSDM1_FLT1
    (Stream6<DMA2>:8, FLT<pac::DFSDM1, 2>, [PeripheralToMemory]), //DFSDM1_FLT2
    (Stream7<DMA2>:8, FLT<pac::DFSDM1, 3>, [PeripheralToMemory]), //DFSDM1_FLT3

    //(Stream0<DMA2>:9, FLT<pac::JPEG, 0>, [MemoryToPeripheral]), //JPEG_IN
    //(Stream1<DMA2>:9, FLT<pac::JPEG, 0>, [PeripheralToMemory]), //JPEG_OUT
    //(Stream3<DMA2>:9, FLT<pac::JPEG, 0>, [MemoryToPeripheral]), //JPEG_IN
    //(Stream4<DMA2>:9, FLT<pac::JPEG, 0>, [PeripheralToMemory]), //JPEG_OUT

    (Stream0<DMA2>:10, SAICH<pac::SAI1, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B
    (Stream1<DMA2>:10, SAICH<pac::SAI2, 1>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B
    (Stream2<DMA2>:10, SAICH<pac::SAI2, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_A
    (Stream6<DMA2>:10, SAICH<pac::SAI1, 0>, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A

    (Stream2<DMA2>:11, pac::QUADSPI, [MemoryToPeripheral | PeripheralToMemory]), //QUADSPI

);
