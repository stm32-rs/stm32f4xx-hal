use super::*;

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $bit:literal, $($en:ident)?, $($lpen:ident)?, $($rst:ident)?),)+) => {
        $(
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            $(bus_enable!($PER => $bit, $en);)?
            $(bus_lpenable!($PER => $bit, $lpen);)?
            $(bus_reset!($PER => $bit, $rst);)?
        )+
    }
}
use bus;

macro_rules! bus_enable {
    ($PER:ident => $bit:literal, $en:ident) => {
        impl $crate::rcc::Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &mut RCC) {
                let reg = Self::Bus::enr(rcc);
                #[cfg(feature = "bb")]
                unsafe {
                    $crate::bb::set(reg, $bit);
                }
                #[cfg(not(feature = "bb"))]
                reg.modify(|_, w| w.$en().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &mut RCC) {
                let reg = Self::Bus::enr(rcc);
                #[cfg(feature = "bb")]
                unsafe {
                    $crate::bb::clear(reg, $bit);
                }
                #[cfg(not(feature = "bb"))]
                reg.modify(|_, w| w.$en().clear_bit());
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                let reg = Self::Bus::enr(unsafe { &*RCC::ptr() });
                #[cfg(feature = "bb")]
                {
                    (reg.read().bits() >> $bit) & 0x1 != 0
                }
                #[cfg(not(feature = "bb"))]
                {
                    reg.read().$en().bit_is_set()
                }
            }
        }
    };
}
use bus_enable;

macro_rules! bus_lpenable {
    ($PER:ident => $bit:literal, $lpen:ident) => {
        impl $crate::rcc::LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn enable_in_low_power(rcc: &mut RCC) {
                let reg = Self::Bus::lpenr(rcc);
                #[cfg(feature = "bb")]
                unsafe {
                    $crate::bb::set(reg, $bit);
                }
                #[cfg(not(feature = "bb"))]
                reg.modify(|_, w| w.$lpen().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable_in_low_power(rcc: &mut RCC) {
                let reg = Self::Bus::lpenr(rcc);
                #[cfg(feature = "bb")]
                unsafe {
                    $crate::bb::clear(reg, $bit);
                }
                #[cfg(not(feature = "bb"))]
                reg.modify(|_, w| w.$lpen().clear_bit());
            }
            #[inline(always)]
            fn is_enabled_in_low_power() -> bool {
                let rcc = RCC::ptr();
                let reg = Self::Bus::lpenr(unsafe { &*rcc });
                #[cfg(feature = "bb")]
                {
                    (reg.read().bits() >> $bit) & 0x1 != 0
                }
                #[cfg(not(feature = "bb"))]
                {
                    reg.read().$lpen().bit_is_set()
                }
            }
        }
    };
}
use bus_lpenable;

macro_rules! bus_reset {
    ($PER:ident => $bit:literal, $rst:ident) => {
        impl $crate::rcc::Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &mut RCC) {
                let reg = Self::Bus::rstr(rcc);
                #[cfg(feature = "bb")]
                unsafe {
                    $crate::bb::set(reg, $bit);
                    $crate::bb::clear(reg, $bit);
                }
                #[cfg(not(feature = "bb"))]
                {
                    let bits = reg.modify(|_, w| w.$rst().set_bit());
                    reg.write(|w| unsafe { w.bits(bits).$rst().clear_bit() });
                }
            }
        }
    };
}
use bus_reset;

bus! {
    GPIOA => (AHB1, 0, gpioaen, gpioalpen, gpioarst),
    GPIOB => (AHB1, 1, gpioben, gpioblpen, gpiobrst),
}
#[cfg(feature = "gpioc")]
bus! { GPIOC => (AHB1, 2, gpiocen, gpioclpen, gpiocrst),}
#[cfg(feature = "gpiod")]
bus! { GPIOD => (AHB1, 3, gpioden, gpiodlpen, gpiodrst),}
#[cfg(feature = "gpioe")]
bus! { GPIOE => (AHB1, 4, gpioeen, gpioelpen, gpioerst),}
#[cfg(feature = "gpiof")]
bus! { GPIOF => (AHB1, 5, gpiofen, gpioflpen, gpiofrst),}
#[cfg(feature = "gpiog")]
bus! { GPIOG => (AHB1, 6, gpiogen, gpioglpen, gpiogrst),}
#[cfg(feature = "gpioh")]
bus! { GPIOH => (AHB1, 7, gpiohen, gpiohlpen, gpiohrst),}
#[cfg(feature = "gpioi")]
bus! { GPIOI => (AHB1, 8, gpioien, gpioilpen, gpioirst),}
#[cfg(feature = "gpioj")]
bus! { GPIOJ => (AHB1, 9, gpiojen, gpiojlpen, gpiojrst),}
#[cfg(feature = "gpiok")]
bus! { GPIOK => (AHB1, 10, gpioken, gpioklpen, gpiokrst),}

bus! {
    CRC => (AHB1, 12, crcen, crclpen, crcrst),
    DMA1 => (AHB1, 21, dma1en, dma1lpen, dma1rst),
    DMA2 => (AHB1, 22, dma2en, dma2lpen, dma2rst),
}

#[cfg(feature = "dma2d")]
bus! { DMA2D => (AHB1, 23, dma2den, dma2dlpen, dma2drst),}

#[cfg(feature = "eth")]
bus! { ETHERNET_MAC => (AHB1, 25, ethmacen, ethmaclpen, ethmacrst),}

#[cfg(feature = "otg-hs")]
bus! { OTG_HS_GLOBAL => (AHB1, 29, otghsen, otghslpen, otghsrst),}

#[cfg(feature = "dcmi")]
bus! { DCMI => (AHB2, 0, dcmien, dcmilpen, dcmirst),}

#[cfg(feature = "jpeg")]
bus! { JPEG => (AHB2, 1, jpegen, jpeglpen,),}

#[cfg(feature = "f4")]
#[cfg(feature = "aes")]
bus! { AES => (AHB2, 4, crypen, , cryprst),}

#[cfg(not(feature = "f4"))]
#[cfg(feature = "aes")]
bus! { AES => (AHB2, 4, aesen, aeslpen, aesrst),}

#[cfg(feature = "cryp")]
bus! { CRYP => (AHB2, 4, crypen, cryplpen, cryprst),}

#[cfg(feature = "hash")]
bus! { HASH => (AHB2, 5, hashen, hashlpen,),}

#[cfg(feature = "rng")]
bus! { RNG => (AHB2, 6, rngen, rnglpen, rngrst),}

#[cfg(feature = "otg-fs")]
bus! { OTG_FS_GLOBAL => (AHB2, 7, otgfsen, otgfslpen, otgfsrst),}

#[cfg(feature = "fmc")]
bus! { FMC => (AHB3, 0, fmcen, fmclpen, fmcrst),}

#[cfg(feature = "fsmc")]
#[cfg(feature = "svd-f427")]
bus! { FSMC => (AHB3, 0, fmcen, fmclpen, fmcrst),}

#[cfg(feature = "fsmc")]
#[cfg(not(feature = "svd-f427"))]
bus! { FSMC => (AHB3, 0, fsmcen, fsmclpen, fsmcrst),}

#[cfg(feature = "quadspi")]
bus! { QUADSPI => (AHB3, 1, qspien, , qspirst),}

#[cfg(feature = "tim2")]
bus! { TIM2 => (APB1, 0, tim2en, tim2lpen, tim2rst),}
#[cfg(feature = "tim3")]
bus! { TIM3 => (APB1, 1, tim3en, tim3lpen, tim3rst),}
#[cfg(feature = "tim4")]
bus! { TIM4 => (APB1, 2, tim4en, tim4lpen, tim4rst),}
#[cfg(feature = "tim5")]
bus! { TIM5 => (APB1, 3, tim5en, tim5lpen, tim5rst),}
#[cfg(feature = "tim6")]
bus! { TIM6 => (APB1, 4, tim6en, tim6lpen, tim6rst),}
#[cfg(feature = "tim7")]
bus! { TIM7 => (APB1, 5, tim7en, tim7lpen, tim7rst),}
#[cfg(feature = "tim12")]
bus! { TIM12 => (APB1, 6, tim12en, tim12lpen, tim12rst),}
#[cfg(feature = "tim13")]
bus! { TIM13 => (APB1, 7, tim13en, tim13lpen, tim13rst),}
#[cfg(feature = "tim14")]
bus! { TIM14 => (APB1, 8, tim14en, tim14lpen, tim14rst),}

#[cfg(feature = "lptim1")]
#[cfg(not(feature = "svd-f413"))]
bus! { LPTIM1 => (APB1, 9, lptim1en, lptim1lpen, lptim1rst),}
#[cfg(feature = "lptim1")]
#[cfg(feature = "svd-f413")]
bus! { LPTIM => (APB1, 9, lptimer1en, lptimer1lpen, lptimer1rst),}

bus! { WWDG => (APB1, 11, wwdgen, wwdglpen, wwdgrst),}

#[cfg(feature = "spi2")]
bus! { SPI2 => (APB1, 14, spi2en, spi2lpen, spi2rst),}
#[cfg(feature = "spi3")]
bus! { SPI3 => (APB1, 15, spi3en, spi3lpen, spi3rst),}

#[cfg(feature = "spdifrx")]
bus! { SPDIFRX => (APB1, 16, spdifrxen, spdifrxlpen, spdifrxrst),}

#[cfg(feature = "usart2")]
bus! { USART2 => (APB1, 17, usart2en, usart2lpen, usart2rst),}
#[cfg(feature = "usart3")]
bus! { USART3 => (APB1, 18, usart3en, usart3lpen, usart3rst),}

#[cfg(feature = "uart4")]
bus! { UART4 => (APB1, 19, uart4en, uart4lpen, uart4rst),}
#[cfg(feature = "uart5")]
bus! { UART5 => (APB1, 20, uart5en, uart5lpen, uart5rst),}

bus! { I2C1 => (APB1, 21, i2c1en, i2c1lpen, i2c1rst),}
#[cfg(feature = "i2c2")]
bus! { I2C2 => (APB1, 22, i2c2en, i2c2lpen, i2c2rst),}
#[cfg(feature = "i2c3")]
bus! { I2C3 => (APB1, 23, i2c3en, i2c3lpen, i2c3rst),}
#[cfg(feature = "i2c4")]
bus! { I2C4 => (APB1, 24, i2c4en, i2c4lpen, i2c4rst),}
#[cfg(feature = "fmpi2c1")]
#[cfg(not(feature = "svd-f412"))]
bus! { FMPI2C1 => (APB1, 24, fmpi2c1en, fmpi2c1lpen, fmpi2c1rst),}
#[cfg(feature = "fmpi2c1")]
#[cfg(feature = "svd-f412")]
bus! { FMPI2C1 => (APB1, 24, i2c4en, i2c4lpen, i2c4rst),}

#[cfg(feature = "can1")]
bus! { CAN1 => (APB1, 25, can1en, can1lpen, can1rst),}
#[cfg(feature = "can2")]
bus! { CAN2 => (APB1, 26, can2en, can2lpen, can2rst),}

#[cfg(feature = "f4")]
#[cfg(feature = "can3")]
bus! { CAN3 => (APB1, 27, can3en, can3lpen, can3rst),}

#[cfg(feature = "f7")]
#[cfg(feature = "can3")]
bus! { CAN3 => (APB1, 13, can3en, can3lpen, can3rst),}

#[cfg(feature = "f4")]
#[cfg(feature = "cec")]
bus! { HDMP_CEC => (APB1, 27, cecen, ceclpen, cecrst),}

#[cfg(feature = "f7")]
#[cfg(feature = "cec")]
bus! { CEC => (APB1, 27, cecen, ceclpen, cecrst),}

bus! { PWR => (APB1, 28, pwren, pwrlpen, pwrrst),}

#[cfg(feature = "dac")]
bus! { DAC => (APB1, 29, dacen, daclpen, dacrst),}

#[cfg(feature = "uart7")]
bus! {UART7 => (APB1, 30, uart7en, uart7lpen, uart7rst),}
#[cfg(feature = "uart8")]
bus! {UART8 => (APB1, 31, uart8en, uart8lpen, uart8rst),}

#[cfg(feature = "tim1")]
bus! { TIM1 => (APB2, 0, tim1en, tim1lpen, tim1rst),}
#[cfg(feature = "tim8")]
bus! {TIM8 => (APB2, 1, tim8en, tim8lpen, tim8rst),}

#[cfg(feature = "usart1")]
bus! { USART1 => (APB2, 4, usart1en, usart1lpen, usart1rst),}
#[cfg(feature = "usart6")]
bus! { USART6 => (APB2, 5, usart6en, usart6lpen, usart6rst),}
#[cfg(feature = "uart9")]
bus! { UART9 => (APB2, 6, uart9en, uart9lpen, uart9rst),}
#[cfg(feature = "uart10")]
bus! { UART10 => (APB2, 7, uart10en, uart10lpen, uart10rst),}

#[cfg(feature = "sdmmc2")]
bus! { SDMMC2 => (APB2, 7, sdmmc2en, sdmmc2lpen, sdmmc2rst),}

bus! { ADC1 => (APB2, 8, adc1en, adc1lpen, adcrst),}

#[cfg(feature = "adc2")]
bus! { ADC2 => (APB2, 9, adc2en, adc2lpen, ),}
#[cfg(feature = "adc2")]
bus_reset!(ADC2 => 8, adcrst);

#[cfg(feature = "adc3")]
bus! { ADC3 => (APB2, 10, adc3en, adc3lpen, ),}
#[cfg(feature = "adc3")]
bus_reset!(ADC3 => 8, adcrst);

#[cfg(feature = "sdio")]
bus! { SDIO => (APB2, 11, sdioen, sdiolpen, sdiorst),}
#[cfg(feature = "sdmmc1")]
bus! { SDMMC1 => (APB2, 11, sdmmc1en, sdmmc1lpen, sdmmc1rst),}

#[cfg(feature = "spi1")]
bus! { SPI1 => (APB2, 12, spi1en, spi1lpen, spi1rst),}
#[cfg(feature = "spi4")]
bus! { SPI4 => (APB2, 13, spi4en, spi4lpen, spi4rst),}

#[cfg(feature = "sys")]
bus! { SYSCFG => (APB2, 14, syscfgen, syscfglpen, syscfgrst),}

#[cfg(feature = "tim9")]
bus! {TIM9 => (APB2, 16, tim9en, tim9lpen, tim9rst),}
#[cfg(feature = "tim10")]
bus! { TIM10 => (APB2, 17, tim10en, tim10lpen, tim10rst),}
#[cfg(feature = "tim11")]
bus! { TIM11 => (APB2, 18, tim11en, tim11lpen, tim11rst),}

#[cfg(feature = "spi5")]
bus! { SPI5 => (APB2, 20, spi5en, spi5lpen, spi5rst),}
#[cfg(feature = "spi6")]
bus! { SPI6 => (APB2, 21, spi6en, spi6lpen, spi6rst),}

#[cfg(any(
    feature = "gpio-f413",
    feature = "gpio-f469",
    feature = "stm32f429",
    feature = "stm32f439"
))]
#[cfg(feature = "sai1")]
bus! {
    SAI => (APB2, 22, sai1en, sai1lpen, sai1rst),
}

#[cfg(not(any(
    feature = "gpio-f413",
    feature = "gpio-f469",
    feature = "stm32f429",
    feature = "stm32f439"
)))]
#[cfg(feature = "sai1")]
bus! {
    SAI1 => (APB2, 22, sai1en, sai1lpen, sai1rst),
}

#[cfg(feature = "sai2")]
bus! {
SAI2 => (APB2, 23, sai2en, sai2lpen, sai2rst),}

#[cfg(feature = "gpio-f412")]
#[cfg(feature = "dfsdm1")]
bus! { DFSDM => (APB2, 24, dfsdmen, dfsdmlpen, dfsdmrst),}
#[cfg(feature = "gpio-f413")]
#[cfg(feature = "dfsdm1")]
bus! { DFSDM1 => (APB2, 24, dfsdmen, dfsdmlpen, dfsdmrst),}
#[cfg(feature = "f4")]
#[cfg(feature = "dfsdm2")]
bus! { DFSDM2 => (APB2, 25, dfsdmen, dfsdmlpen, dfsdmrst),}

#[cfg(feature = "ltdc")]
bus! { LTDC => (APB2, 26, ltdcen, ltdclpen, ltdcrst),}

#[cfg(feature = "dsihost")]
bus! { DSI => (APB2, 27, dsien, dsilpen, dsirst),}

#[cfg(feature = "f7")]
#[cfg(feature = "dfsdm1")]
bus! { DFSDM => (APB2, 29, dfsdm1en, dfsdm1lpen, dfsdm1rst),}

#[cfg(feature = "mdios")]
bus! { MDIOS => (APB2, 30, mdioen, mdiolpen, mdiorst),}
#[cfg(feature = "svd-f730")]
bus! { USBPHYC => (APB2, 31, usbphycen,, usbphycrst),}
