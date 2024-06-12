use super::*;
use crate::bb;

macro_rules! bus_enable {
    ($PER:ident => $bit:literal) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::enr(rcc), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::enr(rcc), $bit);
                }
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                let rcc = pac::RCC::ptr();
                (Self::Bus::enr(unsafe { &*rcc }).read().bits() >> $bit) & 0x1 != 0
            }
        }
    };
}
macro_rules! bus_lpenable {
    ($PER:ident => $bit:literal) => {
        impl LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn enable_in_low_power(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::lpenr(rcc), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable_in_low_power(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::lpenr(rcc), $bit);
                }
            }
            #[inline(always)]
            fn is_enabled_in_low_power() -> bool {
                let rcc = pac::RCC::ptr();
                (Self::Bus::lpenr(unsafe { &*rcc }).read().bits() >> $bit) & 0x1 != 0
            }
        }
    };
}
macro_rules! bus_reset {
    ($PER:ident => $bit:literal) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::rstr(rcc), $bit);
                    bb::clear(Self::Bus::rstr(rcc), $bit);
                }
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $bit:literal),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $bit);
            bus_lpenable!($PER => $bit);
            bus_reset!($PER => $bit);
        )+
    }
}

#[cfg(pac_quadspi)]
impl crate::Sealed for crate::pac::QUADSPI {}
#[cfg(pac_quadspi)]
impl RccBus for crate::pac::QUADSPI {
    type Bus = AHB3;
}

#[cfg(pac_quadspi)]
bus_enable! { QUADSPI => 1 }
#[cfg(pac_quadspi)]
bus_reset! { QUADSPI => 1 }

bus! {
    CRC => (AHB1, 12),
    DMA1 => (AHB1, 21),
    DMA2 => (AHB1, 22),
}

bus! {
    GPIOA => (AHB1, 0),
    GPIOB => (AHB1, 1),
    GPIOC => (AHB1, 2),
    GPIOH => (AHB1, 7),
}

#[cfg(any(pac_gpiod, pac_gpioe))]
bus! {
    GPIOD => (AHB1, 3),
    GPIOE => (AHB1, 4),
}
#[cfg(any(pac_gpiof, pac_gpiog))]
bus! {
    GPIOF => (AHB1, 5),
    GPIOG => (AHB1, 6),
}

#[cfg(pac_gpioi)]
bus! {
    GPIOI => (AHB1, 8),
}

#[cfg(any(pac_gpioj, pac_gpiok))]
bus! {
    GPIOJ => (AHB1, 9),
    GPIOK => (AHB1, 10),
}

#[cfg(pac_rng)]
bus! {
    RNG => (AHB2, 6),
}

#[cfg(pac_otg_fs)]
bus! {
    OTG_FS_GLOBAL => (AHB2, 7),
}

#[cfg(pac_otg_hs)]
bus! {
    OTG_HS_GLOBAL => (AHB1, 29),
}

#[cfg(pac_fmc)]
bus! {
    FMC => (AHB3, 0),
}

// TODO: fix absent ahb3lpenr
#[cfg(pac_fsmc)]
impl crate::Sealed for crate::pac::FSMC {}
#[cfg(pac_fsmc)]
impl RccBus for crate::pac::FSMC {
    type Bus = AHB3;
}
#[cfg(pac_fsmc)]
bus_enable!(FSMC => 0);
#[cfg(pac_fsmc)]
bus_reset!(FSMC => 0);

bus! {
    PWR => (APB1, 28),
}

bus! {
    SPI1 => (APB2, 12),
    SPI2 => (APB1, 14),
}
#[cfg(pac_spi3)]
bus! {
    SPI3 => (APB1, 15),
}

#[cfg(pac_spi4)]
bus! {
    SPI4 => (APB2, 13),
}

#[cfg(pac_spi5)]
bus! {
    SPI5 => (APB2, 20),
}

#[cfg(pac_spi6)]
bus! {
    SPI6 => (APB2, 21),
}

bus! {
    I2C1 => (APB1, 21),
    I2C2 => (APB1, 22),
}
#[cfg(pac_i2c3)]
bus! {
    I2C3 => (APB1, 23),
}
#[cfg(pac_fmpi2c1)]
bus! {
    FMPI2C1 => (APB1, 24),
}

bus! {
    USART1 => (APB2, 4),
    USART2 => (APB1, 17),
    USART6 => (APB2, 5),
}
#[cfg(pac_usart3)]
bus! {
    USART3 => (APB1, 18),
}

#[cfg(any(pac_uart4, pac_uart5))]
bus! {
    UART4 => (APB1, 19),
    UART5 => (APB1, 20),
}

#[cfg(any(pac_uart7, pac_uart8))]
bus! {
    UART7 => (APB1, 30),
    UART8 => (APB1, 31),
}
#[cfg(any(pac_uart9, pac_uart10))]
bus! {
    UART9 => (APB2, 6),
    UART10 => (APB2, 7),
}

#[cfg(any(pac_can1, pac_can2))]
bus! {
    CAN1 => (APB1, 25),
    CAN2 => (APB1, 26),
}

#[cfg(pac_can3)]
bus! {
    CAN3 => (APB1, 27),
}

#[cfg(pac_dac)]
bus! {
    DAC => (APB1, 29),
}

bus! {
    SYSCFG => (APB2, 14),
}

bus! {
    ADC1 => (APB2, 8),
}

#[cfg(pac_adc2)]
impl crate::Sealed for crate::pac::ADC2 {}
#[cfg(pac_adc2)]
impl RccBus for crate::pac::ADC2 {
    type Bus = APB2;
}
#[cfg(pac_adc2)]
bus_enable!(ADC2 => 9);
#[cfg(pac_adc2)]
bus_lpenable!(ADC2 => 9);
#[cfg(pac_adc2)]
bus_reset!(ADC2 => 8);

#[cfg(pac_adc3)]
impl crate::Sealed for crate::pac::ADC3 {}
#[cfg(pac_adc3)]
impl RccBus for crate::pac::ADC3 {
    type Bus = APB2;
}
#[cfg(pac_adc3)]
bus_enable!(ADC3 => 10);
#[cfg(pac_adc3)]
bus_lpenable!(ADC3 => 10);
#[cfg(pac_adc3)]
bus_reset!(ADC3 => 8);

#[cfg(feature = "stm32f413")]
bus! {
    SAI => (APB2, 22),
}

#[cfg(any(feature = "stm32f427", feature = "stm32f437", feature = "stm32f446"))]
bus! {
    SAI1 => (APB2, 22),
}

#[cfg(pac_sai2)]
bus! {
    SAI2 => (APB2, 23),
}

#[cfg(pac_sdio)]
bus! {
    SDIO => (APB2, 11),
}

bus! {
    TIM1 => (APB2, 0),
    TIM5 => (APB1, 3),
    TIM9 => (APB2, 16),
    TIM11 => (APB2, 18),
}

#[cfg(any(
    gpio_f401, gpio_f417, gpio_f411, gpio_f412, gpio_f413, gpio_f427, gpio_f446, gpio_f469,
))]
bus! {
    TIM2 => (APB1, 0),
    TIM3 => (APB1, 1),
    TIM4 => (APB1, 2),
    TIM10 => (APB2, 17),
}

#[cfg(any(
    gpio_f410, gpio_f412, gpio_f413, gpio_f417, gpio_f427, gpio_f446, gpio_f469,
))]
bus! {
    TIM6 => (APB1, 4),
}

#[cfg(any(gpio_f412, gpio_f413, gpio_f417, gpio_f427, gpio_f446, gpio_f469))]
bus! {
    TIM7 => (APB1, 5),
    TIM8 => (APB2, 1),
    TIM12 => (APB1, 6),
    TIM13 => (APB1, 7),
    TIM14 => (APB1, 8),
}

#[cfg(pac_ltdc)]
bus! {
    LTDC => (APB2, 26),
}
#[cfg(pac_dma2d)]
bus! {
    DMA2D => (AHB1, 23),
}
