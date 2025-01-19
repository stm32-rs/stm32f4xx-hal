use super::*;

macro_rules! bus_enable {
    ($PER:ident => $field:ident) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                Self::Bus::enr(rcc).bb_set(|w| w.$field());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &RccRB) {
                Self::Bus::enr(rcc).bb_clear(|w| w.$field());
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                let rcc = pac::RCC::ptr();
                Self::Bus::enr(unsafe { &*rcc })
                    .read()
                    .$field()
                    .bit_is_set()
            }
        }
    };
}
macro_rules! bus_lpenable {
    ($PER:ident => $field:ident) => {
        impl LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn enable_in_low_power(rcc: &RccRB) {
                Self::Bus::lpenr(rcc).bb_set(|w| w.$field());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable_in_low_power(rcc: &RccRB) {
                Self::Bus::lpenr(rcc).bb_clear(|w| w.$field());
            }
            #[inline(always)]
            fn is_enabled_in_low_power() -> bool {
                let rcc = pac::RCC::ptr();
                Self::Bus::lpenr(unsafe { &*rcc })
                    .read()
                    .$field()
                    .bit_is_set()
            }
        }
    };
}
macro_rules! bus_reset {
    ($PER:ident => $field:ident) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                Self::Bus::rstr(rcc).bb_set(|w| w.$field());
                Self::Bus::rstr(rcc).bb_clear(|w| w.$field());
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $fielden:ident, $fieldlpen:ident, $fieldrst:ident),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $fielden);
            bus_lpenable!($PER => $fieldlpen);
            bus_reset!($PER => $fieldrst);
        )+
    }
}

#[cfg(feature = "quadspi")]
impl crate::Sealed for crate::pac::QUADSPI {}
#[cfg(feature = "quadspi")]
impl RccBus for crate::pac::QUADSPI {
    type Bus = AHB3;
}

#[cfg(feature = "quadspi")]
bus_enable! { QUADSPI => qspien } // 1
#[cfg(feature = "quadspi")]
bus_reset! { QUADSPI => qspirst } // 1

bus! {
    CRC => (AHB1, crcen, crclpen, crcrst), // 12
    DMA1 => (AHB1, dma1en, dma1lpen, dma1rst), // 21
    DMA2 => (AHB1, dma2en, dma2lpen, dma2rst), // 22
}

bus! {
    GPIOA => (AHB1, gpioaen, gpioalpen, gpioarst), // 0
    GPIOB => (AHB1, gpioben, gpioblpen, gpiobrst), // 1
    GPIOC => (AHB1, gpiocen, gpioclpen, gpiocrst), // 2
    GPIOH => (AHB1, gpiohen, gpiohlpen, gpiohrst), // 7
}

#[cfg(any(feature = "gpiod", feature = "gpioe"))]
bus! {
    GPIOD => (AHB1, gpioden, gpiodlpen, gpiodrst), // 3
    GPIOE => (AHB1, gpioeen, gpioelpen, gpioerst), // 4
}
#[cfg(any(feature = "gpiof", feature = "gpiog"))]
bus! {
    GPIOF => (AHB1, gpiofen, gpioflpen, gpiofrst), // 5
    GPIOG => (AHB1, gpiogen, gpioglpen, gpiogrst), // 6
}

#[cfg(feature = "gpioi")]
bus! {
    GPIOI => (AHB1, gpioien, gpioilpen, gpioirst), // 8
}

#[cfg(any(feature = "gpioj", feature = "gpiok"))]
bus! {
    GPIOJ => (AHB1, gpiojen, gpiojlpen, gpiojrst), // 9
    GPIOK => (AHB1, gpioken, gpioklpen, gpiokrst), // 10
}

#[cfg(feature = "rng")]
bus! {
    RNG => (AHB2, rngen, rnglpen, rngrst), // 6
}

#[cfg(feature = "otg-fs")]
bus! {
    OTG_FS_GLOBAL => (AHB2, otgfsen, otgfslpen, otgfsrst), // 7
}

#[cfg(feature = "otg-hs")]
bus! {
    OTG_HS_GLOBAL => (AHB1, otghsen, otghslpen, otghsrst), // 29
}

#[cfg(feature = "fmc")]
bus! {
    FMC => (AHB3, fmcen, fmclpen, fmcrst), // 0
}

// TODO: fix absent ahb3lpenr
#[cfg(feature = "fsmc")]
impl crate::Sealed for crate::pac::FSMC {}
#[cfg(feature = "fsmc")]
impl RccBus for crate::pac::FSMC {
    type Bus = AHB3;
}
#[cfg(feature = "fsmc")]
bus_enable!(FSMC => fsmcen); // 0
#[cfg(feature = "fsmc")]
bus_reset!(FSMC => fsmcrst); // 0

bus! {
    PWR => (APB1, pwren, pwrlpen, pwrrst), // 28
}

bus! {
    SPI1 => (APB2, spi1en, spi1lpen, spi1rst), // 12
    SPI2 => (APB1, spi2en, spi2lpen, spi2rst), // 14
}
#[cfg(feature = "spi3")]
bus! {
    SPI3 => (APB1, spi3en, spi3lpen, spi3rst), // 15
}

#[cfg(feature = "spi4")]
bus! {
    SPI4 => (APB2, spi4en, spi4lpen, spi4rst), // 13
}

#[cfg(feature = "spi5")]
bus! {
    SPI5 => (APB2, spi5en, spi5lpen, spi5rst), // 20
}

#[cfg(feature = "spi6")]
bus! {
    SPI6 => (APB2, spi6en, spi6lpen, spi6rst), // 21
}

bus! {
    I2C1 => (APB1, i2c1en, i2c1lpen, i2c1rst), // 21
    I2C2 => (APB1, i2c2en, i2c2lpen, i2c2rst), // 22
}
#[cfg(feature = "i2c3")]
bus! {
    I2C3 => (APB1, i2c3en, i2c3lpen, i2c3rst), // 23
}
#[cfg(feature = "fmpi2c1")]
bus! {
    FMPI2C1 => (APB1, fmpi2c1en, fmpi2c1lpen, fmpi2c1rst), // 24
}

bus! {
    USART1 => (APB2, usart1en, usart1lpen, usart1rst), // 4
    USART2 => (APB1, usart2en, usart2lpen, usart2rst), // 17
    USART6 => (APB2, usart6en, usart6lpen, usart6rst), // 5
}
#[cfg(feature = "usart3")]
bus! {
    USART3 => (APB1, usart3en, usart3lpen, usart3rst), // 18
}

#[cfg(any(feature = "uart4", feature = "uart5"))]
bus! {
    UART4 => (APB1, uart4en, uart4lpen, uart4rst), // 19
    UART5 => (APB1, uart5en, uart5lpen, uart5rst), // 20
}

#[cfg(any(feature = "uart7", feature = "uart8"))]
bus! {
    UART7 => (APB1, uart7en, uart7lpen, uart7rst), // 30
    UART8 => (APB1, uart8en, uart8lpen, uart8rst), // 31
}
#[cfg(any(feature = "uart9", feature = "uart10"))]
bus! {
    UART9 => (APB2, uart9en, uart9lpen, uart9rst), // 6
    UART10 => (APB2, uart10en, uart10lpen, uart10rst), // 7
}

#[cfg(any(feature = "can1", feature = "can2"))]
bus! {
    CAN1 => (APB1, can1en, can1lpen, can1rst), // 25
    CAN2 => (APB1, can2en, can2lpen, can2rst), // 26
}

#[cfg(feature = "can3")]
bus! {
    CAN3 => (APB1, can3en, can3lpen, can3rst), // 27
}

#[cfg(feature = "dac")]
bus! {
    DAC => (APB1, dacen, daclpen, dacrst), // 29
}

bus! {
    SYSCFG => (APB2, syscfgen, syscfglpen, syscfgrst), // 14
}

bus! {
    ADC1 => (APB2, adc1en, adc1lpen, adcrst), // 8
}

#[cfg(feature = "adc2")]
bus! {
    ADC2 => (APB2, adc2en, adc2lpen, adcrst), // 9/9/8
}

#[cfg(feature = "adc3")]
bus! {
    ADC3 => (APB2, adc3en, adc3lpen, adcrst), // 10/10/8
}

#[cfg(any(
    feature = "gpio-f413",
    feature = "gpio-f469",
    feature = "stm32f429",
    feature = "stm32f439"
))]
bus! {
    SAI => (APB2, sai1en, sai1lpen, sai1rst), // 22
}

#[cfg(any(feature = "stm32f427", feature = "stm32f437", feature = "stm32f446"))]
bus! {
    SAI1 => (APB2, sai1en, sai1lpen, sai1rst), // 22
}

#[cfg(feature = "sai2")]
bus! {
    SAI2 => (APB2, sai2en, sai2lpen, sai2rst), // 23
}

#[cfg(feature = "sdio")]
bus! {
    SDIO => (APB2, sdioen, sdiolpen, sdiorst), // 11
}

bus! {
    TIM1 => (APB2, tim1en, tim1lpen, tim1rst), // 0
    TIM5 => (APB1, tim5en, tim5lpen, tim5rst), // 3
    TIM9 => (APB2, tim9en, tim9lpen, tim9rst), // 16
    TIM11 => (APB2, tim11en, tim11lpen, tim11rst), // 18
}

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
bus! {
    TIM2 => (APB1, tim2en, tim2lpen, tim2rst), // 0
    TIM3 => (APB1, tim3en, tim3lpen, tim3rst), // 1
    TIM4 => (APB1, tim4en, tim4lpen, tim4rst), // 2
    TIM10 => (APB2, tim10en, tim10lpen, tim10rst), // 17
}

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
bus! {
    TIM6 => (APB1, tim6en, tim6lpen, tim6rst), // 4
}

#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469",
))]
bus! {
    TIM7 => (APB1, tim7en, tim7lpen, tim7rst), // 5
    TIM8 => (APB2, tim8en, tim8lpen, tim8rst), // 1
    TIM12 => (APB1, tim12en, tim12lpen, tim12rst), // 6
    TIM13 => (APB1, tim13en, tim13lpen, tim13rst), // 7
    TIM14 => (APB1, tim14en, tim14lpen, tim14rst), // 8
}

#[cfg(feature = "ltdc")]
bus! {
    LTDC => (APB2, ltdcen, ltdclpen, ltdcrst), // 26
}
#[cfg(feature = "dma2d")]
bus! {
    DMA2D => (AHB1, dma2den, dma2dlpen, dma2drst), // 23
}
#[cfg(feature = "dsihost")]
bus! {
    DSI => (APB2, dsien, dsilpen, dsirst), // 27
}
