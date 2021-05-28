use super::*;
use crate::bb;

macro_rules! bus_enable {
    ($PER:ident => ($busenr:ident, $enbit:literal)) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                unsafe {
                    bb::set(&rcc.$busenr, $enbit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(&rcc.$busenr, $enbit);
                }
            }
        }
    };
}
macro_rules! bus_lpenable {
    ($PER:ident => ($buslpenr:ident, $lpenbit:literal)) => {
        impl LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn low_power_enable(rcc: &RccRB) {
                unsafe {
                    bb::set(&rcc.$buslpenr, $lpenbit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn low_power_disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(&rcc.$buslpenr, $lpenbit);
                }
            }
        }
    };
}
macro_rules! bus_reset {
    ($PER:ident => ($busrstr:ident, $resetbit:literal)) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                unsafe {
                    bb::set(&rcc.$busrstr, $resetbit);
                    bb::clear(&rcc.$busrstr, $resetbit);
                }
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busenr:ident, $buslpenr:ident, $busrstr:ident, $bit:literal),)+) => {
        $(
            impl private::Sealed for crate::pac::$PER {}
            bus_enable!($PER => ($busenr, $bit));
            bus_lpenable!($PER => ($buslpenr, $bit));
            bus_reset!($PER => ($busrstr, $bit));
        )+
    }
}

bus! {
    CRC => (ahb1enr, ahb1lpenr, ahb1rstr, 12),
    DMA1 => (ahb1enr, ahb1lpenr, ahb1rstr, 21),
    DMA2 => (ahb1enr, ahb1lpenr, ahb1rstr, 22),
}

bus! {
    GPIOA => (ahb1enr, ahb1lpenr, ahb1rstr, 0),
    GPIOB => (ahb1enr, ahb1lpenr, ahb1rstr, 1),
    GPIOC => (ahb1enr, ahb1lpenr, ahb1rstr, 2),
    GPIOH => (ahb1enr, ahb1lpenr, ahb1rstr, 7),
}

#[cfg(any(feature = "gpiod", feature = "gpioe"))]
bus! {
    GPIOD => (ahb1enr, ahb1lpenr, ahb1rstr, 3),
    GPIOE => (ahb1enr, ahb1lpenr, ahb1rstr, 4),
}
#[cfg(any(feature = "gpiof", feature = "gpiog"))]
bus! {
    GPIOF => (ahb1enr, ahb1lpenr, ahb1rstr, 5),
    GPIOG => (ahb1enr, ahb1lpenr, ahb1rstr, 6),
}

#[cfg(feature = "gpioi")]
bus! {
    GPIOI => (ahb1enr, ahb1lpenr, ahb1rstr, 8),
}

#[cfg(any(feature = "gpioj", feature = "gpiok"))]
bus! {
    GPIOJ => (ahb1enr, ahb1lpenr, ahb1rstr, 9),
    GPIOK => (ahb1enr, ahb1lpenr, ahb1rstr, 10),
}

#[cfg(feature = "rng")]
bus! {
    RNG => (ahb2enr, ahb2lpenr, ahb2rstr, 6),
}

#[cfg(feature = "otg-fs")]
bus! {
    OTG_FS_GLOBAL => (ahb2enr, ahb2lpenr, ahb2rstr, 7),
}

#[cfg(feature = "otg-hs")]
bus! {
    OTG_HS_GLOBAL => (ahb1enr, ahb1lpenr, ahb1rstr, 29),
}

#[cfg(feature = "fmc")]
bus! {
    FMC => (ahb3enr, ahb3lpenr, ahb3rstr, 0),
}

// TODO: fix absent ahb3lpenr
#[cfg(feature = "fsmc")]
impl private::Sealed for crate::pac::FSMC {}
#[cfg(feature = "fsmc")]
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
bus_enable!(FSMC => (ahb3enr, 0));
#[cfg(feature = "fsmc")]
#[cfg(not(any(feature = "stm32f427", feature = "stm32f437")))]
bus_enable!(FSMC => (ahb3enr, 0));
#[cfg(feature = "fsmc")]
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
bus_reset!(FSMC => (ahb3rstr, 0));
#[cfg(feature = "fsmc")]
#[cfg(not(any(feature = "stm32f427", feature = "stm32f437")))]
bus_reset!(FSMC => (ahb3rstr, 0));

bus! {
    PWR => (apb1enr, apb1lpenr, apb1rstr, 28),
}

bus! {
    SPI1 => (apb2enr, apb2lpenr, apb2rstr, 12),
    SPI2 => (apb1enr, apb1lpenr, apb1rstr, 14),
}
#[cfg(feature = "spi3")]
bus! {
    SPI3 => (apb1enr, apb1lpenr, apb1rstr, 15),
}

#[cfg(feature = "spi4")]
bus! {
    SPI4 => (apb2enr, apb2lpenr, apb2rstr, 13),
}

#[cfg(feature = "spi5")]
bus! {
    SPI5 => (apb2enr, apb2lpenr, apb2rstr, 20),
}

#[cfg(feature = "spi6")]
bus! {
    SPI6 => (apb2enr, apb2lpenr, apb2rstr, 21),
}

bus! {
    I2C1 => (apb1enr, apb1lpenr, apb1rstr, 21),
    I2C2 => (apb1enr, apb1lpenr, apb1rstr, 22),
}
#[cfg(feature = "i2c3")]
bus! {
    I2C3 => (apb1enr, apb1lpenr, apb1rstr, 23),
}
#[cfg(feature = "fmpi2c1")]
bus! {
    FMPI2C1 => (apb1enr, apb1lpenr, apb1rstr, 24),
}

bus! {
    USART1 => (apb2enr, apb2lpenr, apb2rstr, 4),
    USART2 => (apb1enr, apb1lpenr, apb1rstr, 17),
    USART6 => (apb2enr, apb2lpenr, apb2rstr, 5),
}
#[cfg(feature = "usart3")]
bus! {
    USART3 => (apb1enr, apb1lpenr, apb1rstr, 18),
}

#[cfg(any(feature = "uart4", feature = "uart5"))]
bus! {
    UART4 => (apb1enr, apb1lpenr, apb1rstr, 19),
    UART5 => (apb1enr, apb1lpenr, apb1rstr, 20),
}

#[cfg(any(feature = "uart7", feature = "uart8"))]
bus! {
    UART7 => (apb1enr, apb1lpenr, apb1rstr, 30),
    UART8 => (apb1enr, apb1lpenr, apb1rstr, 31),
}
#[cfg(any(feature = "uart9", feature = "uart10"))]
bus! {
    UART9 => (apb2enr, apb2lpenr, apb2rstr, 6),
    UART10 => (apb2enr, apb2lpenr, apb2rstr, 7),
}

#[cfg(any(feature = "can1", feature = "can2"))]
bus! {
    CAN1 => (apb1enr, apb1lpenr, apb1rstr, 25),
    CAN2 => (apb1enr, apb1lpenr, apb1rstr, 26),
}
#[cfg(feature = "dac")]
bus! {
    DAC => (apb1enr, apb1lpenr, apb1rstr, 29),
}

bus! {
    SYSCFG => (apb2enr, apb2lpenr, apb2rstr, 14),
}

bus! {
    ADC1 => (apb2enr, apb2lpenr, apb2rstr, 8),
}

#[cfg(feature = "adc2")]
impl private::Sealed for crate::pac::ADC2 {}
#[cfg(feature = "adc2")]
bus_enable!(ADC2 => (apb2enr, 9));
#[cfg(feature = "adc2")]
bus_lpenable!(ADC2 => (apb2lpenr, 9));
#[cfg(feature = "adc2")]
bus_reset!(ADC2 => (apb2rstr, 8));

#[cfg(feature = "adc3")]
impl private::Sealed for crate::pac::ADC3 {}
#[cfg(feature = "adc3")]
bus_enable!(ADC3 => (apb2enr, 10));
#[cfg(feature = "adc3")]
bus_lpenable!(ADC3 => (apb2lpenr, 10));
#[cfg(feature = "adc3")]
bus_reset!(ADC3 => (apb2rstr, 8));

#[cfg(feature = "sdio")]
bus! {
    SDIO => (apb2enr, apb2lpenr, apb2rstr, 11),
}

bus! {
    TIM1 => (apb2enr, apb2lpenr, apb2rstr, 0),
    TIM5 => (apb1enr, apb1lpenr, apb1rstr, 3),
    TIM9 => (apb2enr, apb2lpenr, apb2rstr, 16),
    TIM11 => (apb2enr, apb2lpenr, apb2rstr, 18),
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
    TIM2 => (apb1enr, apb1lpenr, apb1rstr, 0),
    TIM3 => (apb1enr, apb1lpenr, apb1rstr, 1),
    TIM4 => (apb1enr, apb1lpenr, apb1rstr, 2),
    TIM10 => (apb2enr, apb2lpenr, apb2rstr, 17),
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
    TIM6 => (apb1enr, apb1lpenr, apb1rstr, 4),
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
    TIM7 => (apb1enr, apb1lpenr, apb1rstr, 5),
    TIM8 => (apb2enr, apb2lpenr, apb2rstr, 1),
    TIM12 => (apb1enr, apb1lpenr, apb1rstr, 6),
    TIM13 => (apb1enr, apb1lpenr, apb1rstr, 7),
    TIM14 => (apb1enr, apb1lpenr, apb1rstr, 8),
}
