use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod debug {
    use super::*;

    pin! {
        <Swclk, PushPull> for [
            PA14<0>,
        ],

        <Swdio, PushPull> for [
            PA13<0>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA9<6>,

            PB6<6>,

            PB7<14>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB8<6>,
        ],

        <Sda, OpenDrain> for [
            PA10<6>,

            PB7<6>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<6>,

            PC14<14>,
        ],

        <Smba, OpenDrain> for [
            PA1<6>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB5<6>,

            PB6<7>,
        ],
    }
    use crate::pac::I2C1 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2s {
    use super::*;

    pin! {
        <Ckin, PushPull> for [
            PA12<5>,
        ],
    }
}

pub mod i2s1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA1<0>,

            PA5<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<0>,

            PB6<10>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA6<0>,

            PA11<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB4<0>,

            PB6<9>,
        ],

        <Sd, PushPull> for [
            PA2<0>,

            PA7<0>,

            PA12<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB5<0>,

            PB6<8>,
        ],

        <Ws, PushPull> for [
            PA4<0>,

            PA8<8>,

            PA14<8>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB0<0>,
        ],
    }
}

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<0>,

            PC14<8>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,

            PA9<0>,

            PF2<0>,
        ],

        <Mco2, PushPull> for [
            PA8<15>,

            PA10<3>,

            PA14<11>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<3>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB2<3>,
        ],

        <Osc32En, PushPull> for [
            PC15<0>,
        ],

        <OscEn, PushPull> for [
            PC15<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PF1<0>,
        ],
    }
}

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<0>,

            PA11<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB4<0>,

            PB6<9>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA2<0>,

            PA7<0>,

            PA12<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB5<0>,

            PB6<8>,
        ],

        <Nss, PushPull> for [
            PA4<0>,

            PA8<8>,

            PA14<8>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB0<0>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA1<0>,

            PA5<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<0>,

            PB6<10>,
        ],
    }
    impl SpiCommon for crate::pac::SPI1 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod spi2 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA9<4>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA10<0>,
        ],
    }
    impl SpiCommon for crate::pac::SPI2 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<5>,

            PA5<5>,

            PA8<2>,

            PA14<10>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<2>,
        ],

        <Ch1N> default:PushPull for [
            PA3<2>,

            PA7<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB13<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD2<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<5>,

            PA9<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<1>,

            PB6<11>,
        ],

        <Ch2N> default:PushPull for [
            PA4<2>,

            PA8<9>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB0<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB1<5>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB14<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD3<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<5>,

            PA10<2>,

            PB6<1>,
        ],

        <Ch3N> default:PushPull for [
            PA5<2>,

            PA8<10>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB1<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB15<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<5>,

            PA11<2>,

            PB7<1>,

            PF2<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB12<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PC13<2>,
        ],

        <Bkin2, PushPull> for [
            PA11<5>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB12<1>,

            PC14<2>,
        ],

        <Etr, PushPull> for [
            PA12<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PC13<1>,

            PC14<1>,

            PC15<2>,
        ],
    }

    use crate::pac::TIM1 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimNCPin<1> for TIM {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<2> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB4<1>,

            PB6<12>,

            PB7<11>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB8<3>,

            #[cfg(feature = "gpio-c0xx_453")]
            PC6<1>,
        ],

        <Ch2> default:PushPull for [
            PA7<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<3>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB5<1>,

            PB6<13>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<3>,

            #[cfg(feature = "gpio-c0xx_453")]
            PC7<1>,

            PC14<11>,
        ],

        <Ch3> default:PushPull for [
            PA8<11>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB0<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB5<3>,

            PB6<3>,

            PC15<3>,
        ],

        <Ch4> default:PushPull for [
            PA8<12>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB1<1>,

            PB7<3>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA2<3>,

            PA9<3>,

            PA13<3>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD2<1>,
        ],
    }

    use crate::pac::TIM3 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim14 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA4<4>,

            PA7<4>,

            PA8<13>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB1<0>,

            #[cfg(feature = "gpio-c0xx_453")]
            PF0<2>,
        ],
    }

    use crate::pac::TIM14 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim15 {
    use super::*;

    pin! {
        <Bk, PushPull> for [
            PA9<5>,
        ],
    }

    use crate::pac::TIM15 as TIM;
}

pub mod tim16 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PA6<5>,

            PB7<10>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB8<2>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD0<2>,
        ],

        <Ch1N> default:PushPull for [
            PA2<2>,

            PB6<2>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            #[cfg(feature = "gpio-c0xx_453")]
            PB5<2>,

            PB6<14>,
        ],
    }

    use crate::pac::TIM16 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod tim17 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA1<2>,

            PA7<5>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<2>,

            PC14<10>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD1<2>,
        ],

        <Ch1N> default:PushPull for [
            PA4<5>,

            PB7<2>,
        ],
    }

    pin! {
        <Bk, PushPull> for [
            PA10<5>,
        ],

        <Bkin, PushPull> for [
            PA10<5>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB4<5>,

            PB6<15>,
        ],
    }

    use crate::pac::TIM17 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA12<1>,

            PA14<12>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<4>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<4>,

            PB6<4>,
        ],

        <Cts, PushPull> for [
            PA11<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB4<4>,

            PB6<5>,
        ],

        <De, PushPull> for [
            PA12<1>,

            PA14<12>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<4>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<4>,

            PB6<4>,
        ],

        <Nss, PushPull> for [
            PA11<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB4<4>,

            PB6<5>,
        ],

        <Rts, PushPull> for [
            PA12<1>,

            PA14<12>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<4>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB3<4>,

            PB6<4>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA1<4>,

            PA8<14>,

            PA10<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB2<0>,

            PB7<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<4>,

            PA9<1>,

            PB6<0>,

            PC14<0>,
        ],
    }
    use crate::pac::USART1 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod usart2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA1<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<1>,

            PC14<9>,
        ],

        <Cts, PushPull> for [
            PA0<1>,

            PB7<9>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB8<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD3<0>,
        ],

        <De, PushPull> for [
            PA1<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<1>,

            PC14<9>,
        ],

        <Nss, PushPull> for [
            PA0<1>,

            PB7<9>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB8<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PD3<0>,
        ],

        <Rts, PushPull> for [
            PA1<1>,

            #[cfg(feature = "gpio-c0xx_453")]
            PB9<1>,

            PC14<9>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<1>,

            PA5<1>,

            PA13<4>,

            PA14<9>,

            #[cfg(feature = "gpio-c0xx_453")]
            PA15<1>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<1>,

            PA4<1>,

            PA8<1>,

            PA14<1>,
        ],
    }
    use crate::pac::USART2 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}
