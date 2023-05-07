use super::*;

#[cfg(feature = "STM32L021_dma_v1_1")]
dma_map! {
    (Channel1<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel2<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel4<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel2<DMA1>:1, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Channel3<DMA1>:1, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Channel4<DMA1>:1, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Channel5<DMA1>:1, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Channel2<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel3<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel4<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel5<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel2<DMA1>:5, LPUART1_TX, [MemoryToPeripheral]), //LPUART1_TX
    (Channel3<DMA1>:5, LPUART1_RX, [PeripheralToMemory]), //LPUART1_RX
    (Channel4<DMA1>:5, LPUART1_TX, [MemoryToPeripheral]), //LPUART1_TX
    (Channel5<DMA1>:5, LPUART1_RX, [PeripheralToMemory]), //LPUART1_RX
    (Channel2<DMA1>:6, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel3<DMA1>:6, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel4<DMA1>:6, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel5<DMA1>:6, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel1<DMA1>:8, TIM2_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    (Channel2<DMA1>:8, TIM2_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Channel3<DMA1>:8, TIM2_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2
    (Channel4<DMA1>:8, TIM2_CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Channel5<DMA1>:8, TIM2_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Channel1<DMA1>:11, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Channel2<DMA1>:11, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel3<DMA1>:11, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel5<DMA1>:11, AES_IN, [MemoryToPeripheral]), //AES_IN
}
#[cfg(feature = "STM32L063_dma_v1_1")]
dma_map! {
    (Channel1<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel2<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel2<DMA1>:1, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Channel3<DMA1>:1, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Channel4<DMA1>:2, SPI2_RX, [PeripheralToMemory]), //SPI2_RX
    (Channel5<DMA1>:2, SPI2_TX, [MemoryToPeripheral]), //SPI2_TX
    (Channel6<DMA1>:2, SPI2_RX, [PeripheralToMemory]), //SPI2_RX
    (Channel7<DMA1>:2, SPI2_TX, [MemoryToPeripheral]), //SPI2_TX
    (Channel2<DMA1>:3, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Channel3<DMA1>:3, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Channel4<DMA1>:3, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Channel5<DMA1>:3, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Channel4<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel5<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel6<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel7<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel2<DMA1>:5, LPUART1_TX, [MemoryToPeripheral]), //LPUART1_TX
    (Channel3<DMA1>:5, LPUART1_RX, [PeripheralToMemory]), //LPUART1_RX
    (Channel6<DMA1>:5, LPUART1_RX, [PeripheralToMemory]), //LPUART1_RX
    (Channel7<DMA1>:5, LPUART1_TX, [MemoryToPeripheral]), //LPUART1_TX
    (Channel2<DMA1>:6, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel3<DMA1>:6, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel6<DMA1>:6, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel7<DMA1>:6, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel4<DMA1>:7, I2C2_TX, [MemoryToPeripheral]), //I2C2_TX
    (Channel5<DMA1>:7, I2C2_RX, [PeripheralToMemory]), //I2C2_RX
    (Channel1<DMA1>:8, TIM2_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    (Channel2<DMA1>:8, TIM2_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Channel3<DMA1>:8, TIM2_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2
    (Channel4<DMA1>:8, TIM2_CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Channel5<DMA1>:8, TIM2_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Channel7<DMA1>:8, TIM2_CH2/CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2/CH4
    (Channel2<DMA1>:9, DAC_CH1, [MemoryToPeripheral]), //DAC_CH1
    (Channel2<DMA1>:9, TIM6_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP
    (Channel1<DMA1>:11, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Channel2<DMA1>:11, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel3<DMA1>:11, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel5<DMA1>:11, AES_IN, [MemoryToPeripheral]), //AES_IN
}
#[cfg(feature = "STM32L083_dma_v1_1")]
dma_map! {
    (Channel1<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel2<DMA1>:0, ADC, [PeripheralToMemory]), //ADC
    (Channel2<DMA1>:1, SPI1_RX, [PeripheralToMemory]), //SPI1_RX
    (Channel3<DMA1>:1, SPI1_TX, [MemoryToPeripheral]), //SPI1_TX
    (Channel4<DMA1>:2, SPI2_RX, [PeripheralToMemory]), //SPI2_RX
    (Channel5<DMA1>:2, SPI2_TX, [MemoryToPeripheral]), //SPI2_TX
    (Channel6<DMA1>:2, SPI2_RX, [PeripheralToMemory]), //SPI2_RX
    (Channel7<DMA1>:2, SPI2_TX, [MemoryToPeripheral]), //SPI2_TX
    (Channel2<DMA1>:3, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Channel3<DMA1>:3, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Channel4<DMA1>:3, USART1_TX, [MemoryToPeripheral]), //USART1_TX
    (Channel5<DMA1>:3, USART1_RX, [PeripheralToMemory]), //USART1_RX
    (Channel4<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel5<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel6<DMA1>:4, USART2_RX, [PeripheralToMemory]), //USART2_RX
    (Channel7<DMA1>:4, USART2_TX, [MemoryToPeripheral]), //USART2_TX
    (Channel2<DMA1>:5, LPUART1_TX, [MemoryToPeripheral]), //LPUART1_TX
    (Channel3<DMA1>:5, LPUART1_RX, [PeripheralToMemory]), //LPUART1_RX
    (Channel6<DMA1>:5, LPUART1_RX, [PeripheralToMemory]), //LPUART1_RX
    (Channel7<DMA1>:5, LPUART1_TX, [MemoryToPeripheral]), //LPUART1_TX
    (Channel2<DMA1>:6, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel3<DMA1>:6, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel6<DMA1>:6, I2C1_TX, [MemoryToPeripheral]), //I2C1_TX
    (Channel7<DMA1>:6, I2C1_RX, [PeripheralToMemory]), //I2C1_RX
    (Channel4<DMA1>:7, I2C2_TX, [MemoryToPeripheral]), //I2C2_TX
    (Channel5<DMA1>:7, I2C2_RX, [PeripheralToMemory]), //I2C2_RX
    (Channel1<DMA1>:8, TIM2_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH3
    (Channel2<DMA1>:8, TIM2_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_UP
    (Channel3<DMA1>:8, TIM2_CH2, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2
    (Channel4<DMA1>:8, TIM2_CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH4
    (Channel5<DMA1>:8, TIM2_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH1
    (Channel7<DMA1>:8, TIM2_CH2/CH4, [MemoryToPeripheral | PeripheralToMemory]), //TIM2_CH2/CH4
    (Channel2<DMA1>:9, DAC_CH1, [MemoryToPeripheral]), //DAC_CH1
    (Channel2<DMA1>:9, TIM6_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM6_UP
    (Channel2<DMA1>:10, TIM3_CH3, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH3
    (Channel3<DMA1>:10, TIM3_CH4/UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH4/UP
    (Channel5<DMA1>:10, TIM3_CH1, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_CH1
    (Channel6<DMA1>:10, TIM3_TRIG, [MemoryToPeripheral | PeripheralToMemory]), //TIM3_TRIG
    (Channel1<DMA1>:11, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Channel2<DMA1>:11, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel3<DMA1>:11, AES_OUT, [PeripheralToMemory]), //AES_OUT
    (Channel5<DMA1>:11, AES_IN, [MemoryToPeripheral]), //AES_IN
    (Channel2<DMA1>:12, USART4_RX, [PeripheralToMemory]), //USART4_RX
    (Channel3<DMA1>:12, USART4_TX, [MemoryToPeripheral]), //USART4_TX
    (Channel6<DMA1>:12, USART4_RX, [PeripheralToMemory]), //USART4_RX
    (Channel7<DMA1>:12, USART4_TX, [MemoryToPeripheral]), //USART4_TX
    (Channel2<DMA1>:13, USART5_RX, [PeripheralToMemory]), //USART5_RX
    (Channel3<DMA1>:13, USART5_TX, [MemoryToPeripheral]), //USART5_TX
    (Channel6<DMA1>:13, USART5_RX, [PeripheralToMemory]), //USART5_RX
    (Channel7<DMA1>:13, USART5_TX, [MemoryToPeripheral]), //USART5_TX
    (Channel2<DMA1>:14, I2C3_TX, [MemoryToPeripheral]), //I2C3_TX
    (Channel3<DMA1>:14, I2C3_RX, [PeripheralToMemory]), //I2C3_RX
    (Channel4<DMA1>:14, I2C3_TX, [MemoryToPeripheral]), //I2C3_TX
    (Channel5<DMA1>:14, I2C3_RX, [PeripheralToMemory]), //I2C3_RX
    (Channel4<DMA1>:15, DAC_CH2, [MemoryToPeripheral]), //DAC_CH2
    (Channel4<DMA1>:15, TIM7_UP, [MemoryToPeripheral | PeripheralToMemory]), //TIM7_UP
}
