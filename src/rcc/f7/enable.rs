use super::*;

macro_rules! bus_enable {
    ($PER:ident => $en:ident) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(bus: &mut Self::Bus) {
                bus.enr().modify(|_, w| w.$en().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
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

macro_rules! bus_lpenable {
    ($PER:ident => $lpen:ident) => {
        impl LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn low_power_enable(bus: &mut Self::Bus) {
                bus.lpenr().modify(|_, w| w.$lpen().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn low_power_disable(bus: &mut Self::Bus) {
                bus.lpenr().modify(|_, w| w.$lpen().clear_bit());
            }
            #[inline(always)]
            fn is_low_power_enabled() -> bool {
                Self::Bus::new().lpenr().read().$lpen().bit_is_set()
            }
            #[inline(always)]
            unsafe fn low_power_enable_unchecked() {
                Self::enable(&mut Self::Bus::new());
            }
            #[inline(always)]
            unsafe fn low_power_disable_unchecked() {
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
    ($($PER:ident => ($busX:ty, $($en:ident)?, $($lpen:ident)?, $($rst:ident)?),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            $(bus_enable!($PER => $en);)?
            $(bus_lpenable!($PER => $lpen);)?
            $(bus_reset!($PER => $rst);)?
        )+
    };
}

// Peripherals respective buses
// TODO: check which processor has which peripheral and add them
bus! {
    GPIOA => (AHB1, gpioaen, gpioalpen, gpioarst), // 0
    GPIOB => (AHB1, gpioben, gpioblpen, gpiobrst), // 1
    GPIOC => (AHB1, gpiocen, gpioclpen, gpiocrst), // 2
    GPIOD => (AHB1, gpioden, gpiodlpen, gpiodrst), // 3
    GPIOE => (AHB1, gpioeen, gpioelpen, gpioerst), // 4
    GPIOF => (AHB1, gpiofen, gpioflpen, gpiofrst), // 5
    GPIOG => (AHB1, gpiogen, gpioglpen, gpiogrst), // 6
    GPIOH => (AHB1, gpiohen, gpiohlpen, gpiohrst), // 7
    GPIOI => (AHB1, gpioien, gpioilpen, gpioirst), // 8
    CRC => (AHB1, crcen, crclpen, crcrst), // 12
    DMA1 => (AHB1, dma1en, dma1lpen, dma1rst), // 21
    DMA2 => (AHB1, dma2en, dma2lpen, dma2rst), // 22
    OTG_HS_GLOBAL => (AHB1, otghsen, otghslpen, otghsrst), // 29

    RNG => (AHB2, rngen, rnglpen, rngrst), // 6
    OTG_FS_GLOBAL => (AHB2, otgfsen, otgfslpen, otgfsrst), // 7

    FMC => (AHB3, fmcen, fmclpen, fmcrst), // 0
    QUADSPI => (AHB3, qspien, qspilpen, qspirst), // 1

    TIM2 => (APB1, tim2en, tim2lpen, tim2rst), // 0
    TIM3 => (APB1, tim3en, tim3lpen, tim3rst), // 1
    TIM4 => (APB1, tim4en, tim4lpen, tim4rst), // 2
    TIM5 => (APB1, tim5en, tim5lpen, tim5rst), // 3
    TIM6 => (APB1, tim6en, tim6lpen, tim6rst), // 4
    TIM7 => (APB1, tim7en, tim7lpen, tim7rst), // 5
    TIM12 => (APB1, tim12en, tim12lpen, tim12rst), // 6
    TIM13 => (APB1, tim13en, tim13lpen, tim13rst), // 7
    TIM14 => (APB1, tim14en, tim14lpen, tim14rst), // 8
    LPTIM1 => (APB1, lptim1en, lptim1lpen, lptim1rst), // 9
    WWDG => (APB1, wwdgen, wwdglpen, wwdgrst), // 11
    SPI2 => (APB1, spi2en, spi2lpen, spi2rst), // 14
    SPI3 => (APB1, spi3en, spi3lpen, spi3rst), // 15
    USART2 => (APB1, usart2en, usart2lpen, usart2rst), // 17
    USART3 => (APB1, usart3en, usart3lpen, usart3rst), // 18
    UART4 => (APB1, uart4en, uart4lpen, uart4rst), // 19
    UART5 => (APB1, uart5en, uart5lpen, uart5rst), // 20
    I2C1 => (APB1, i2c1en, i2c1lpen, i2c1rst), // 21
    I2C2 => (APB1, i2c2en, i2c2lpen, i2c2rst), // 22
    I2C3 => (APB1, i2c3en, i2c3lpen, i2c3rst), // 23
    CAN1 => (APB1, can1en, can1lpen, can1rst), // 25
    PWR => (APB1, pwren, pwrlpen, pwrrst), // 28
    DAC => (APB1, dacen, daclpen, dacrst), // 29
    UART7 => (APB1, uart7en, uart7lpen, uart7rst), // 30
    UART8 => (APB1, uart8en, uart8lpen, uart8rst), // 31

    TIM1 => (APB2, tim1en, tim1lpen, tim1rst), // 0
    TIM8 => (APB2, tim8en, tim8lpen, tim8rst), // 1
    USART1 => (APB2, usart1en, usart1lpen, usart1rst), // 4
    USART6 => (APB2, usart6en, usart6lpen, usart6rst), // 5
    ADC1 => (APB2, adc1en, adc1lpen, adcrst), // 8
    ADC2 => (APB2, adc2en, adc2lpen, adcrst), // 9
    ADC3 => (APB2, adc3en, adc3lpen, adcrst), // 10
    SDMMC1 => (APB2, sdmmc1en, sdmmc1lpen, sdmmc1rst), // 11
    SPI1 => (APB2, spi1en, spi1lpen, spi1rst), // 12
    SPI4 => (APB2, spi4en, spi4lpen, spi4rst), // 13
    SYSCFG => (APB2, syscfgen, syscfglpen, syscfgrst), // 14
    TIM9 => (APB2, tim9en, tim9lpen, tim9rst), // 16
    TIM10 => (APB2, tim10en, tim10lpen, tim10rst), // 17
    TIM11 => (APB2, tim11en, tim11lpen, tim11rst), // 18
    SPI5 => (APB2, spi5en, spi5lpen, spi5rst), // 20
    SAI1 => (APB2, sai1en, sai1lpen, sai1rst), // 22
    SAI2 => (APB2, sai2en, sai2lpen, sai2rst), // 23
}

#[cfg(any(feature = "svd-f730", feature = "svd-f7x2", feature = "svd-f7x3",))]
bus! {
    AES => (AHB2, aesen, aeslpen, aesrst), // 4

    SDMMC2 => (APB2, sdmmc2en, sdmmc2lpen, sdmmc2rst), // 7
    USBPHYC => (APB2, usbphycen,, usbphycrst), // 31
}

#[cfg(any(
    feature = "svd-f745",
    feature = "svd-f750",
    feature = "svd-f7x6",
    feature = "svd-f765",
    feature = "svd-f7x7",
    feature = "svd-f7x9",
))]
bus! {
    GPIOJ => (AHB1, gpiojen, gpiojlpen, gpiojrst), // 9
    GPIOK => (AHB1, gpioken, gpioklpen, gpiokrst), // 10
    DMA2D => (AHB1, dma2den, dma2dlpen, dma2drst), // 23
    ETHERNET_MAC => (AHB1, ethmacen, ethmaclpen, ethmacrst), // 25

    DCMI => (AHB2, dcmien, dcmilpen, dcmirst), // 0
    CRYP => (AHB2, crypen, cryplpen, cryprst), // 4
    HASH => (AHB2, hashen, hashlpen,), // 5

    SPDIFRX => (APB1, spdifrxen, spdifrxlpen, spdifrxrst), // 16
    I2C4 => (APB1, i2c4en, i2c4lpen, i2c4rst), // 24
    CAN2 => (APB1, can2en, can2lpen, can2rst), // 26
    CEC => (APB1, cecen, ceclpen, cecrst), // 27

    SPI6 => (APB2, spi6en, spi6lpen, spi6rst), // 21
    LTDC => (APB2, ltdcen, ltdclpen, ltdcrst), // 26
}

#[cfg(any(feature = "svd-f765", feature = "svd-f7x7", feature = "svd-f7x9"))]
bus! {
    JPEG => (AHB2, jpegen, jpeglpen,), // 1

    CAN3 => (APB1, can3en, can3lpen, can3rst), // 13

    DSI => (APB2, dsien, dsilpen, dsirst), // 27
    MDIOS => (APB2, mdioen, mdiolpen, mdiorst), // 30
}

#[cfg(any(feature = "svd-f7x9", feature = "svd-f7x9"))]
bus! {
    DFSDM => (APB2, dfsdm1en, dfsdm1lpen, dfsdm1rst), // 29
}

#[cfg(feature = "svd-f765")]
bus! {
    DFSDM1 => (APB2, dfsdm1en, dfsdm1lpen, dfsdm1rst), // 29
}
