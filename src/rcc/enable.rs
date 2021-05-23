use super::*;

macro_rules! bus_enable {
    ($PER:ident => ($busenr:ident, $peren:ident)) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                rcc.$busenr.modify(|_, w| w.$peren().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &RccRB) {
                rcc.$busenr.modify(|_, w| w.$peren().clear_bit());
            }
        }
    };
}
macro_rules! bus_lpenable {
    ($PER:ident => ($buslpenr:ident, $perlpen:ident)) => {
        impl LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn low_power_enable(rcc: &RccRB) {
                rcc.$buslpenr.modify(|_, w| w.$perlpen().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn low_power_disable(rcc: &RccRB) {
                rcc.$buslpenr.modify(|_, w| w.$perlpen().clear_bit());
            }
        }
    };
}
macro_rules! bus_reset {
    ($PER:ident => ($busrstr:ident, $perrst:ident)) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                rcc.$busrstr.modify(|_, w| w.$perrst().set_bit());
                rcc.$busrstr.modify(|_, w| w.$perrst().clear_bit());
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busenr:ident, $peren:ident, $buslpenr:ident, $perlpen:ident, $busrstr:ident, $perrst:ident),)+) => {
        $(
            impl private::Sealed for crate::pac::$PER {}
            bus_enable!($PER => ($busenr, $peren));
            bus_lpenable!($PER => ($buslpenr, $perlpen));
            bus_reset!($PER => ($busrstr, $perrst));
        )+
    }
}

bus! {
    CRC => (ahb1enr, crcen, ahb1lpenr, crclpen, ahb1rstr, crcrst),
    DMA1 => (ahb1enr, dma1en, ahb1lpenr, dma1lpen, ahb1rstr, dma1rst),
    DMA2 => (ahb1enr, dma2en, ahb1lpenr, dma2lpen, ahb1rstr, dma2rst),
}

bus! {
    GPIOA => (ahb1enr, gpioaen, ahb1lpenr, gpioalpen, ahb1rstr, gpioarst),
    GPIOB => (ahb1enr, gpioben, ahb1lpenr, gpioblpen, ahb1rstr, gpiobrst),
    GPIOC => (ahb1enr, gpiocen, ahb1lpenr, gpioclpen, ahb1rstr, gpiocrst),
    GPIOH => (ahb1enr, gpiohen, ahb1lpenr, gpiohlpen, ahb1rstr, gpiohrst),
}

#[cfg(any(feature = "gpiod", feature = "gpioe"))]
bus! {
    GPIOD => (ahb1enr, gpioden, ahb1lpenr, gpiodlpen, ahb1rstr, gpiodrst),
    GPIOE => (ahb1enr, gpioeen, ahb1lpenr, gpioelpen, ahb1rstr, gpioerst),
}
#[cfg(any(feature = "gpiof", feature = "gpiog"))]
bus! {
    GPIOF => (ahb1enr, gpiofen, ahb1lpenr, gpioflpen, ahb1rstr, gpiofrst),
    GPIOG => (ahb1enr, gpiogen, ahb1lpenr, gpioglpen, ahb1rstr, gpiogrst),
}

#[cfg(feature = "gpioi")]
bus! {
    GPIOI => (ahb1enr, gpioien, ahb1lpenr, gpioilpen, ahb1rstr, gpioirst),
}

#[cfg(any(feature = "gpioj", feature = "gpiok"))]
bus! {
    GPIOJ => (ahb1enr, gpiojen, ahb1lpenr, gpiojlpen, ahb1rstr, gpiojrst),
    GPIOK => (ahb1enr, gpioken, ahb1lpenr, gpioklpen, ahb1rstr, gpiokrst),
}

#[cfg(feature = "rng")]
bus! {
    RNG => (ahb2enr, rngen, ahb2lpenr, rnglpen, ahb2rstr, rngrst),
}

#[cfg(feature = "otg-fs")]
bus! {
    OTG_FS_GLOBAL => (ahb2enr, otgfsen, ahb2lpenr, otgfslpen, ahb2rstr, otgfsrst),
}

#[cfg(feature = "otg-hs")]
bus! {
    OTG_HS_GLOBAL => (ahb1enr, otghsen, ahb1lpenr, otghslpen, ahb1rstr, otghsrst),
}

#[cfg(feature = "fmc")]
bus! {
    FMC => (ahb3enr, fmcen, ahb3lpenr, fmclpen, ahb3rstr, fmcrst),
}

// TODO: fix absent ahb3lpenr
#[cfg(feature = "fsmc")]
impl private::Sealed for crate::pac::FSMC {}
#[cfg(feature = "fsmc")]
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
bus_enable!(FSMC => (ahb3enr, fmcen));
#[cfg(feature = "fsmc")]
#[cfg(not(any(feature = "stm32f427", feature = "stm32f437")))]
bus_enable!(FSMC => (ahb3enr, fsmcen));
#[cfg(feature = "fsmc")]
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
bus_reset!(FSMC => (ahb3rstr, fmcrst));
#[cfg(feature = "fsmc")]
#[cfg(not(any(feature = "stm32f427", feature = "stm32f437")))]
bus_reset!(FSMC => (ahb3rstr, fsmcrst));

bus! {
    PWR => (apb1enr, pwren, apb1lpenr, pwrlpen, apb1rstr, pwrrst),
}

bus! {
    SPI1 => (apb2enr, spi1en, apb2lpenr, spi1lpen, apb2rstr, spi1rst),
    SPI2 => (apb1enr, spi2en, apb1lpenr, spi2lpen, apb1rstr, spi2rst),
}
#[cfg(feature = "spi3")]
bus! {
    SPI3 => (apb1enr, spi3en, apb1lpenr, spi3lpen, apb1rstr, spi3rst),
}

#[cfg(feature = "spi4")]
bus! {
    SPI4 => (apb2enr, spi4en, apb2lpenr, spi4lpen, apb2rstr, spi4rst),
}

#[cfg(feature = "spi5")]
bus! {
    SPI5 => (apb2enr, spi5en, apb2lpenr, spi5lpen, apb2rstr, spi5rst),
}

#[cfg(feature = "spi6")]
bus! {
    SPI6 => (apb2enr, spi6en, apb2lpenr, spi6lpen, apb2rstr, spi6rst),
}

bus! {
    I2C1 => (apb1enr, i2c1en, apb1lpenr, i2c1lpen, apb1rstr, i2c1rst),
    I2C2 => (apb1enr, i2c2en, apb1lpenr, i2c2lpen, apb1rstr, i2c2rst),
}
#[cfg(feature = "i2c3")]
bus! {
    I2C3 => (apb1enr, i2c3en, apb1lpenr, i2c3lpen, apb1rstr, i2c3rst),
}
#[cfg(feature = "fmpi2c1")]
bus! {
    FMPI2C1 => (apb1enr, fmpi2c1en, apb1lpenr, fmpi2c1lpen, apb1rstr, fmpi2c1rst),
}

// TODO: fix uart2rst, uart3rst
bus! {
    USART1 => (apb2enr, usart1en, apb2lpenr, usart1lpen, apb2rstr, usart1rst),
    USART2 => (apb1enr, usart2en, apb1lpenr, usart2lpen, apb1rstr, uart2rst),
    USART6 => (apb2enr, usart6en, apb2lpenr, usart6lpen, apb2rstr, usart6rst),
}
#[cfg(feature = "usart3")]
#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
bus! {
    USART3 => (apb1enr, usart3en, apb1lpenr, usart3lpen, apb1rstr, usart3rst),
}
#[cfg(feature = "usart3")]
#[cfg(not(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423")))]
bus! {
    USART3 => (apb1enr, usart3en, apb1lpenr, usart3lpen, apb1rstr, uart3rst),
}

#[cfg(feature = "uart4")]
impl private::Sealed for crate::pac::UART4 {}
#[cfg(feature = "uart4")]
bus_enable!(UART4 => (apb1enr, uart4en));
#[cfg(feature = "uart5")]
impl private::Sealed for crate::pac::UART5 {}
#[cfg(feature = "uart5")]
bus_enable!(UART5 => (apb1enr, uart5en));

#[cfg(any(feature = "uart7", feature = "uart8"))]
bus! {
    UART7 => (apb1enr, uart7en, apb1lpenr, uart7lpen, apb1rstr, uart7rst),
    UART8 => (apb1enr, uart8en, apb1lpenr, uart8lpen, apb1rstr, uart8rst),
}
#[cfg(any(feature = "uart9", feature = "uart10"))]
bus! {
    UART9 => (apb2enr, uart9en, apb2lpenr, uart9lpen, apb2rstr, uart9rst),
    UART10 => (apb2enr, uart10en, apb2lpenr, uart10lpen, apb2rstr, uart10rst),
}

#[cfg(any(feature = "can1", feature = "can2"))]
bus! {
    CAN1 => (apb1enr, can1en, apb1lpenr, can1lpen, apb1rstr, can1rst),
    CAN2 => (apb1enr, can2en, apb1lpenr, can2lpen, apb1rstr, can2rst),
}
#[cfg(feature = "dac")]
bus! {
    DAC => (apb1enr, dacen, apb1lpenr, daclpen, apb1rstr, dacrst),
}

bus! {
    SYSCFG => (apb2enr, syscfgen, apb2lpenr, syscfglpen, apb2rstr, syscfgrst),
}

bus! {
    ADC1 => (apb2enr, adc1en, apb2lpenr, adc1lpen, apb2rstr, adcrst),
}

#[cfg(any(feature = "adc2", feature = "adc3"))]
bus! {
    ADC2 => (apb2enr, adc2en, apb2lpenr, adc2lpen, apb2rstr, adcrst),
    ADC3 => (apb2enr, adc3en, apb2lpenr, adc3lpen, apb2rstr, adcrst),
}

#[cfg(feature = "sdio")]
bus! {
    SDIO => (apb2enr, sdioen, apb2lpenr, sdiolpen, apb2rstr, sdiorst),
}

bus! {
    TIM1 => (apb2enr, tim1en, apb2lpenr, tim1lpen, apb2rstr, tim1rst),
    TIM5 => (apb1enr, tim5en, apb1lpenr, tim5lpen, apb1rstr, tim5rst),
    TIM9 => (apb2enr, tim9en, apb2lpenr, tim9lpen, apb2rstr, tim9rst),
    TIM11 => (apb2enr, tim11en, apb2lpenr, tim11lpen, apb2rstr, tim11rst),
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
bus! {
    TIM2 => (apb1enr, tim2en, apb1lpenr, tim2lpen, apb1rstr, tim2rst),
    TIM3 => (apb1enr, tim3en, apb1lpenr, tim3lpen, apb1rstr, tim3rst),
    TIM4 => (apb1enr, tim4en, apb1lpenr, tim4lpen, apb1rstr, tim4rst),
    TIM10 => (apb2enr, tim10en, apb2lpenr, tim10lpen, apb2rstr, tim10rst),
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
bus! {
    TIM6 => (apb1enr, tim6en, apb1lpenr, tim6lpen, apb1rstr, tim6rst),
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
bus! {
    TIM7 => (apb1enr, tim7en, apb1lpenr, tim7lpen, apb1rstr, tim7rst),
    TIM8 => (apb2enr, tim8en, apb2lpenr, tim8lpen, apb2rstr, tim8rst),
    TIM12 => (apb1enr, tim12en, apb1lpenr, tim12lpen, apb1rstr, tim12rst),
    TIM13 => (apb1enr, tim13en, apb1lpenr, tim13lpen, apb1rstr, tim13rst),
    TIM14 => (apb1enr, tim14en, apb1lpenr, tim14lpen, apb1rstr, tim14rst),
}
