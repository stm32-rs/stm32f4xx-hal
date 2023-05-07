use super::*;

dma_map! {
    (Stream0<DMA1>:0, SPI3_RX, [PeripheralToMemory]), //SPI3_RX
    (Stream2<DMA1>:0, SPI3_RX, [PeripheralToMemory]), //SPI3_RX
    (Stream3<DMA1>:0, SPI2_RX, [PeripheralToMemory]), //SPI2_RX
    (Stream4<DMA1>:0, SPI2_TX, [MemoryToPeripheral]), //SPI2_TX
    (Stream5<DMA1>:0, SPI3_TX, [MemoryToPeripheral]), //SPI3_TX
    (Stream7<DMA1>:0, SPI3_TX, [MemoryToPeripheral]), //SPI3_TX
    (Stream0<DMA1>:1, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Stream2<DMA1>:1, TIM7_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP
    (Stream4<DMA1>:1, TIM7_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP
    (Stream5<DMA1>:1, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Stream6<DMA1>:1, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Stream7<DMA1>:1, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Stream0<DMA1>:2, TIM4_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH1
    (Stream3<DMA1>:2, TIM4_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH2
    (Stream6<DMA1>:2, TIM4_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_UP
    (Stream7<DMA1>:2, TIM4_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM4_CH3
    (Stream1<DMA1>:3, TIM2_UP/CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP/CH3
    (Stream2<DMA1>:3, I2C3_RX, [PeripheralToMemory]), //I2C3_RX
    (Stream4<DMA1>:3, I2C3_TX, [MemoryToPeripheral]), //I2C3_TX
    (Stream5<DMA1>:3, TIM2_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Stream6<DMA1>:3, TIM2_CH2/CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2/CH4
    (Stream7<DMA1>:3, TIM2_UP/CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP/CH4
    (Stream0<DMA1>:4, UART5_RX, [PeripheralToMemory]), //UART5_RX
    (Stream1<DMA1>:4, USART3_RX, [PeripheralToMemory]), //USART3_RX
    (Stream2<DMA1>:4, UART4_RX, [PeripheralToMemory]), //UART4_RX
    (Stream3<DMA1>:4, USART3_TX, [MemoryToPeripheral]), //USART3_TX
    (Stream4<DMA1>:4, UART4_TX, [MemoryToPeripheral]), //UART4_TX
    (Stream5<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Stream6<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Stream7<DMA1>:4, UART5_TX, [MemoryToPeripheral]), //UART5_TX
    (Stream2<DMA1>:5, TIM3_CH4/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH4/UP
    (Stream4<DMA1>:5, TIM3_CH1/TRIG, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH1/TRIG
    (Stream5<DMA1>:5, TIM3_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH2
    (Stream7<DMA1>:5, TIM3_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH3
    (Stream0<DMA1>:6, TIM5_CH3/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH3/UP
    (Stream1<DMA1>:6, TIM5_CH4/TRIG, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4/TRIG
    (Stream2<DMA1>:6, TIM5_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH1
    (Stream3<DMA1>:6, TIM5_CH4/TRIG, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH4/TRIG
    (Stream4<DMA1>:6, TIM5_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_CH2
    (Stream6<DMA1>:6, TIM5_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM5_UP
    (Stream1<DMA1>:7, TIM6_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP
    (Stream2<DMA1>:7, I2C2_RX, [PeripheralToMemory]), //I2C2_RX
    (Stream3<DMA1>:7, I2C2_RX, [PeripheralToMemory]), //I2C2_RX
    (Stream4<DMA1>:7, USART3_TX:DMA_CHANNEL_7, [MemoryToPeripheral]), //USART3_TX:DMA_CHANNEL_7
    (Stream5<DMA1>:7, DAC1, [MemoryToPeripheral]), //DAC1
    (Stream6<DMA1>:7, DAC2, [MemoryToPeripheral]), //DAC2
    (Stream7<DMA1>:7, I2C2_TX, [MemoryToPeripheral]), //I2C2_TX
    (Stream0<DMA2>:0, ADC1, [PeripheralToMemory]), //ADC1
    (Stream2<DMA2>:0, TIM8_CH1/CH2/CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1/CH2/CH3
    (Stream4<DMA2>:0, ADC1, [PeripheralToMemory]), //ADC1
    (Stream6<DMA2>:0, TIM1_CH1/CH2/CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1/CH2/CH3
    (Stream1<DMA2>:1, DCMI, [PeripheralToMemory]), //DCMI
    (Stream2<DMA2>:1, ADC2, [PeripheralToMemory]), //ADC2
    (Stream3<DMA2>:1, ADC2, [PeripheralToMemory]), //ADC2
    (Stream7<DMA2>:1, DCMI, [PeripheralToMemory]), //DCMI
    (Stream0<DMA2>:2, ADC3, [PeripheralToMemory]), //ADC3
    (Stream1<DMA2>:2, ADC3, [PeripheralToMemory]), //ADC3
    (Stream5<DMA2>:2, CRYP_OUT, [PeripheralToMemory]), //CRYP_OUT
    (Stream6<DMA2>:2, CRYP_IN, [MemoryToPeripheral]), //CRYP_IN
    (Stream7<DMA2>:2, HASH_IN, [MemoryToPeripheral]), //HASH_IN
    (Stream0<DMA2>:3, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Stream2<DMA2>:3, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Stream3<DMA2>:3, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Stream5<DMA2>:3, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Stream2<DMA2>:4, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Stream3<DMA2>:4, SDIO:Conflict:SDIO_RX,SDIO_TX, [MemoryToPeripheral | PeripheralToMemory]), //SDIO:Conflict:SDIO_RX,SDIO_TX
    (Stream3<DMA2>:4, SDIO_RX:Conflict:SDIO, [PeripheralToMemory]), //SDIO_RX:Conflict:SDIO
    (Stream3<DMA2>:4, SDIO_TX:Conflict:SDIO, [MemoryToPeripheral]), //SDIO_TX:Conflict:SDIO
    (Stream5<DMA2>:4, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Stream6<DMA2>:4, SDIO, [MemoryToPeripheral | PeripheralToMemory]), //SDIO
    (Stream6<DMA2>:4, SDIO_RX, [PeripheralToMemory]), //SDIO_RX
    (Stream6<DMA2>:4, SDIO_TX, [MemoryToPeripheral]), //SDIO_TX
    (Stream7<DMA2>:4, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Stream1<DMA2>:5, USART6_RX, [PeripheralToMemory]), //USART6_RX
    (Stream2<DMA2>:5, USART6_RX, [PeripheralToMemory]), //USART6_RX
    (Stream6<DMA2>:5, USART6_TX, [MemoryToPeripheral]), //USART6_TX
    (Stream7<DMA2>:5, USART6_TX, [MemoryToPeripheral]), //USART6_TX
    (Stream0<DMA2>:6, TIM1_TRIG, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_TRIG
    (Stream1<DMA2>:6, TIM1_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream2<DMA2>:6, TIM1_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH2
    (Stream3<DMA2>:6, TIM1_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH1
    (Stream4<DMA2>:6, TIM1_CH4/TRIG/COM, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH4/TRIG/COM
    (Stream5<DMA2>:6, TIM1_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_UP
    (Stream6<DMA2>:6, TIM1_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM1_CH3
    (Stream1<DMA2>:7, TIM8_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_UP
    (Stream2<DMA2>:7, TIM8_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH1
    (Stream3<DMA2>:7, TIM8_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH2
    (Stream4<DMA2>:7, TIM8_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH3
    (Stream7<DMA2>:7, TIM8_CH4/TRIG/COM, [MemoryToPeripheral | PeripheralToMemory]), //TIM8_CH4/TRIG/COM
}
