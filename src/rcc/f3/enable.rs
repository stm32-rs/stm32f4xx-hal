use super::*;

macro_rules! bus_enable {
    ($PER:ident => $en:ident) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(bus: &mut Self::Bus) {
                bus.enr().modify(|_, w| w.$en().set_bit());
            }
            #[inline(always)]
            fn disable(bus: &mut Self::Bus) {
                bus.enr().modify(|_, w| w.$en().clear_bit());
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                Self::Bus::new().enr().read().$en().bit_is_set()
            }
            #[inline(always)]
            fn is_disabled() -> bool {
                Self::Bus::new().enr().read().$en().bit_is_clear()
            }
            #[inline(always)]
            unsafe fn enable_unchecked() {
                Self::enable(&mut Self::Bus::new());
            }
            #[inline(always)]
            unsafe fn disable_unchecked() {
                Self::disable(&mut Self::Bus::new());
            }
        }
    };
}
macro_rules! bus_reset {
    ($PER:ident => $rst:ident) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(bus: &mut Self::Bus) {
                bus.rstr().modify(|_, w| w.$rst().set_bit());
                bus.rstr().modify(|_, w| w.$rst().clear_bit());
            }
            #[inline(always)]
            unsafe fn reset_unchecked() {
                Self::reset(&mut Self::Bus::new());
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $($en:ident)?, $($rst:ident)?),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            $(bus_enable!($PER => $en);)?
            $(bus_reset!($PER => $rst);)?
        )+
    };
}

bus! {
    DMA1 => (AHB, dma1en,), // 0
    CRC => (AHB, crcen,), // 6
    GPIOA => (AHB, iopaen, ioparst), // 17
    GPIOB => (AHB, iopben, iopbrst), // 18
    GPIOC => (AHB, iopcen, iopcrst), // 19
    GPIOD => (AHB, iopden, iopdrst), // 20
    GPIOF => (AHB, iopfen, iopfrst), // 22
    TSC => (AHB, tscen, tscrst), // 24

    TIM2 => (APB1, tim2en, tim2rst), // 0
    TIM6 => (APB1, tim6en, tim6rst), // 4
    WWDG => (APB1, wwdgen, wwdgrst), // 11
    USART2 => (APB1, usart2en, usart2rst), // 17
    USART3 => (APB1, usart3en, usart3rst), // 18
    I2C1 => (APB1, i2c1en, i2c1rst), // 21
    PWR => (APB1, pwren, pwrrst), // 28

    SYSCFG => (APB2, syscfgen, syscfgrst), // 0
    USART1 => (APB2, usart1en, usart1rst), // 14
    TIM15 => (APB2, tim15en, tim15rst), // 16
    TIM16 => (APB2, tim16en, tim16rst), // 17
    TIM17 => (APB2, tim17en, tim17rst), // 18
}

#[cfg(any(
    feature = "svd-f301",
    feature = "svd-f302",
    feature = "svd-f303",
    feature = "svd-f373",
))]
bus! {
    SPI2 => (APB1, spi2en, spi2rst), // 14
    SPI3 => (APB1, spi3en, spi3rst), // 15
    I2C2 => (APB1, i2c2en, i2c2rst), // 22
}

#[cfg(any(feature = "svd-f302", feature = "svd-f303", feature = "svd-f373"))]
bus! {
    DMA2 => (AHB, dma2en,), // 1
    GPIOE => (AHB, iopeen, ioperst), // 21
    TIM4 => (APB1, tim4en, tim4rst), // 2
    USB => (APB1, usben, usbrst), // 23
}

#[cfg(any(
    feature = "svd-f302",
    feature = "svd-f303",
    feature = "svd-f373",
    feature = "svd-f3x4"
))]
bus! {
    TIM3 => (APB1, tim3en, tim3rst), // 1
    CAN => (APB1, canen, canrst), // 25
    SPI1 => (APB2, spi1en, spi1rst), // 12
}

#[cfg(feature = "svd-f301")]
bus! {
    ADC1_2 => (AHB, adc1en, adc1rst), // 28
}

#[cfg(any(feature = "svd-f302", feature = "svd-f303", feature = "svd-f3x4"))]
bus! {
    ADC1_2 => (AHB, adc12en, adc12rst), // 28
}

#[cfg(any(feature = "svd-f302", feature = "svd-f303"))]
bus! {
    FMC => (AHB, fmcen, fmcrst), // 5
    SPI4 => (APB2, spi4en, spi4rst), // 15
    GPIOH => (AHB, iophen, iophrst), // 16
    UART4 => (APB1, uart4en, uart4rst), // 19
    UART5 => (APB1, uart5en, uart5rst), // 20
    GPIOG => (AHB, iopgen, iopgrst), // 23
}

#[cfg(any(
    feature = "svd-f301",
    feature = "svd-f302",
    feature = "svd-f303",
    feature = "svd-f3x4"
))]
bus! {
    TIM1 => (APB2, tim1en, tim1rst), // 11
}

bus! {
    DAC1 => (APB1, dac1en, dac1rst), // 29
}

#[cfg(any(feature = "svd-f303", feature = "svd-f373", feature = "svd-f3x4"))]
bus! {
    TIM7 => (APB1, tim7en, tim7rst), // 5
    DAC2 => (APB1, dac2en,), // 26
}

#[cfg(any(feature = "svd-f301", feature = "svd-f302", feature = "svd-f303"))]
bus! {
    I2C3 => (APB1, i2c3en, i2c3rst), // 30
}

#[cfg(feature = "svd-f303")]
bus! {
    TIM8 => (APB2, tim8en, tim8rst), // 13
    TIM20 => (APB2, tim20en, tim20rst), // 20
    ADC3_4 => (AHB, adc34en, adc34rst), // 29
}

#[cfg(feature = "svd-f373")]
bus! {
    TIM5 => (APB1, tim5en, tim5rst), // 3
    TIM12 => (APB1, tim12en, tim12rst), // 6
    TIM13 => (APB1, tim13en, tim13rst), // 7
    TIM14 => (APB1, tim14en, tim14rst), // 8
    TIM18 => (APB1, tim18en, tim18rst), // 9
    CEC => (APB1, cecen, cecrst), // 30
    TIM19 => (APB2, tim19en, tim19rst), // 19
    DBGMCU => (APB2, dbgmcuen,), // 22
    SDADC1 => (APB2, sdadc1en, sdadc1rst), // 24
    SDADC2 => (APB2, sdadc2en, sdadc2rst), // 25
    SDADC3 => (APB2, sdadc3en, sdadc3rst), // 26
}

#[cfg(feature = "svd-f3x4")]
bus! {
    HRTIM_COMMON => (APB2, hrtim1en, hrtim1rst), // 29
}
