use super::*;

#[cfg(feature = "STM32L4A6_dma_v1_1")]
dma_map! {
    (Channel1<DMA1>:0, ADC1, [PeripheralToMemory]), //ADC1
    (Channel2<DMA1>:0, ADC2, [PeripheralToMemory]), //ADC2
    (Channel3<DMA1>:0, ADC3, [PeripheralToMemory]), //ADC3
    (Channel4<DMA1>:0, DFSDM1_FLT0, [PeripheralToMemory]), //DFSDM1_FLT0
    (Channel5<DMA1>:0, DFSDM1_FLT1, [PeripheralToMemory]), //DFSDM1_FLT1
    (Channel6<DMA1>:0, DFSDM1_FLT2, [PeripheralToMemory]), //DFSDM1_FLT2
    (Channel7<DMA1>:0, DFSDM1_FLT3, [PeripheralToMemory]), //DFSDM1_FLT3
    (Channel2<DMA1>:1, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Channel3<DMA1>:1, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Channel4<DMA1>:1, SPI2_RX, [PeripheralToMemory]), //SPI2_RX
    (Channel5<DMA1>:1, SPI2_TX, [MemoryToPeripheral]), //SPI2_TX
    (Channel6<DMA1>:1, SAI2_A, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_A
    (Channel7<DMA1>:1, SAI2_B, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B
    (Channel2<DMA1>:2, USART3_TX, [MemoryToPeripheral]), //USART3_TX
    (Channel3<DMA1>:2, USART3_RX, [PeripheralToMemory]), //USART3_RX
    (Channel4<DMA1>:2, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Channel5<DMA1>:2, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Channel6<DMA1>:2, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel7<DMA1>:2, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel2<DMA1>:3, I2C3_TX, [MemoryToPeripheral]), //I2C3_TX
    (Channel3<DMA1>:3, I2C3_RX, [PeripheralToMemory]), //I2C3_RX
    (Channel4<DMA1>:3, I2C2_TX, [MemoryToPeripheral]), //I2C2_TX
    (Channel5<DMA1>:3, I2C2_RX, [PeripheralToMemory]), //I2C2_RX
    (Channel6<DMA1>:3, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel7<DMA1>:3, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel1<DMA1>:4, TIM2_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    (Channel2<DMA1>:4, TIM2_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Channel3<DMA1>:4, TIM16_CH1/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM16_CH1/UP
    (Channel5<DMA1>:4, TIM2_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Channel6<DMA1>:4, TIM16_CH1/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM16_CH1/UP
    (Channel7<DMA1>:4, TIM2_CH2/CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2/CH4
    (Channel1<DMA1>:5, TIM17_CH1/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM17_CH1/UP
    (Channel2<DMA1>:5, TIM3_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH3
    (Channel3<DMA1>:5, TIM3_CH4/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH4/UP
    (Channel4<DMA1>:5, DAC_CH2:Conflict:TIM7_UP, [MemoryToPeripheral]), //DAC_CH2:Conflict:TIM7_UP
    (Channel4<DMA1>:5, TIM7_UP:Conflict:DAC_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP:Conflict:DAC_CH2
    (Channel5<DMA1>:5, QUADSPI, [MemoryToPeripheral | PeripheralToMemory]), //QUADSPI
    (Channel6<DMA1>:5, TIM3_CH1/TRIG, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH1/TRIG
    (Channel7<DMA1>:5, TIM17_CH1/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM17_CH1/UP
    (Channel1<DMA1>:6, TIM4_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH1
    (Channel3<DMA1>:6, DAC_CH1:Conflict:TIM6_UP, [MemoryToPeripheral]), //DAC_CH1:Conflict:TIM6_UP
    (Channel3<DMA1>:6, TIM6_UP:Conflict:DAC_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP:Conflict:DAC_CH1
    (Channel4<DMA1>:6, TIM4_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH2
    (Channel5<DMA1>:6, TIM4_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH3
    (Channel7<DMA1>:6, TIM4_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_UP
    (Channel2<DMA1>:7, TIM1_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Channel3<DMA1>:7, TIM1_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH2
    (Channel4<DMA1>:7, TIM1_CH4/TRIG/COM, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH4/TRIG/COM
    (Channel5<DMA1>:7, TIM15_CH1/UP/TRIG/COM, [MemoryToPeripheral | PeripheralToMemory]), //TIM15_CH1/UP/TRIG/COM
    (Channel6<DMA1>:7, TIM1_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_UP
    (Channel7<DMA1>:7, TIM1_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH3
    (Channel1<DMA2>:0, I2C4_RX, [PeripheralToMemory]), //I2C4_RX
    (Channel2<DMA2>:0, I2C4_TX, [MemoryToPeripheral]), //I2C4_TX
    (Channel3<DMA2>:0, ADC1, [PeripheralToMemory]), //ADC1
    (Channel4<DMA2>:0, ADC2, [PeripheralToMemory]), //ADC2
    (Channel5<DMA2>:0, ADC3, [PeripheralToMemory]), //ADC3
    (Channel6<DMA2>:0, DCMI:DMA_REQUEST_0, [PeripheralToMemory]), //DCMI:DMA_REQUEST_0
    (Channel1<DMA2>:1, SAI1_A, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A
    (Channel2<DMA2>:1, SAI1_B, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B
    (Channel3<DMA2>:1, SAI2_A, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_A
    (Channel4<DMA2>:1, SAI2_B, [MemoryToPeripheral | PeripheralToMemory]), //SAI2_B
    (Channel6<DMA2>:1, SAI1_A, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_A
    (Channel7<DMA2>:1, SAI1_B, [MemoryToPeripheral | PeripheralToMemory]), //SAI1_B
    (Channel1<DMA2>:2, UART5_TX, [MemoryToPeripheral]), //UART5_TX
    (Channel2<DMA2>:2, UART5_RX, [PeripheralToMemory]), //UART5_RX
    (Channel3<DMA2>:2, UART4_TX, [MemoryToPeripheral]), //UART4_TX
    (Channel5<DMA2>:2, UART4_RX, [PeripheralToMemory]), //UART4_RX
    (Channel6<DMA2>:2, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Channel7<DMA2>:2, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Channel1<DMA2>:3, SPI3_RX, [PeripheralToMemory]), //SPI3_RX
    (Channel2<DMA2>:3, SPI3_TX, [MemoryToPeripheral]), //SPI3_TX
    (Channel4<DMA2>:3, DAC_CH1, [MemoryToPeripheral]), //DAC_CH1
    (Channel4<DMA2>:3, TIM6_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP
    (Channel5<DMA2>:3, DAC_CH2, [MemoryToPeripheral]), //DAC_CH2
    (Channel5<DMA2>:3, TIM7_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP
    (Channel7<DMA2>:3, QUADSPI:DMA_REQUEST_3, [MemoryToPeripheral | PeripheralToMemory]), //QUADSPI:DMA_REQUEST_3
    (Channel1<DMA2>:4, SWPMI_RX, [PeripheralToMemory]), //SWPMI_RX
    (Channel2<DMA2>:4, SWPMI_TX, [MemoryToPeripheral]), //SWPMI_TX
    (Channel3<DMA2>:4, SPI1_RX:DMA_REQUEST_4, [PeripheralToMemory]), //SPI1_RX:DMA_REQUEST_4
    (Channel4<DMA2>:4, SPI1_TX:DMA_REQUEST_4, [MemoryToPeripheral]), //SPI1_TX:DMA_REQUEST_4
    (Channel5<DMA2>:4, DCMI, [PeripheralToMemory]), //DCMI
    (Channel6<DMA2>:4, LPUART_TX, [MemoryToPeripheral]), //LPUART_TX
    (Channel7<DMA2>:4, LPUART_RX, [PeripheralToMemory]), //LPUART_RX
    (Channel1<DMA2>:5, TIM5_CH4/TRIG/COM, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4/TRIG/COM
    (Channel2<DMA2>:5, TIM5_CH3/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH3/UP
    (Channel4<DMA2>:5, TIM5_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH2
    (Channel5<DMA2>:5, TIM5_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH1
    (Channel6<DMA2>:5, I2C1_RX:DMA_REQUEST_5, [PeripheralToMemory]), //I2C1_RX:DMA_REQUEST_5
    (Channel7<DMA2>:5, I2C1_TX:DMA_REQUEST_5, [MemoryToPeripheral]), //I2C1_TX:DMA_REQUEST_5
    (Channel1<DMA2>:6, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Channel2<DMA2>:6, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel3<DMA2>:6, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel5<DMA2>:6, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Channel7<DMA2>:6, HASH_IN, [MemoryToPeripheral]), //HASH_IN
    (Channel1<DMA2>:7, TIM8_CH3/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH3/UP
    (Channel2<DMA2>:7, TIM8_CH4/TRIG/COM, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH4/TRIG/COM
    (Channel4<DMA2>:7, SDMMC1:Conflict:SDMMC1_RX,SDMMC1_TX, [MemoryToPeripheral | PeripheralToMemory]), //SDMMC1:Conflict:SDMMC1_RX,SDMMC1_TX
    (Channel4<DMA2>:7, SDMMC1_RX:Conflict:SDMMC1, [PeripheralToMemory]), //SDMMC1_RX:Conflict:SDMMC1
    (Channel4<DMA2>:7, SDMMC1_TX:Conflict:SDMMC1, [MemoryToPeripheral]), //SDMMC1_TX:Conflict:SDMMC1
    (Channel5<DMA2>:7, SDMMC1, [MemoryToPeripheral | PeripheralToMemory]), //SDMMC1
    (Channel5<DMA2>:7, SDMMC1_RX, [PeripheralToMemory]), //SDMMC1_RX
    (Channel5<DMA2>:7, SDMMC1_TX, [MemoryToPeripheral]), //SDMMC1_TX
    (Channel6<DMA2>:7, TIM8_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1
    (Channel7<DMA2>:7, TIM8_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH2
}
