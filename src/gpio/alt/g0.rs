use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

#[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA0<7>,

            PA6<7>,

            PA11<7>,

            PB0<7>,

            PB10<7>,
        ],
    }
}

#[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA2<7>,

            PA7<7>,

            PA12<7>,

            PB5<7>,

            PB11<7>,
        ],
    }
}

#[cfg(feature = "gpio-g0bx")]
pub mod comp3 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB1<7>,

            PC0<7>,

            PC2<7>,
        ],
    }
}

/*#[cfg(feature = "gpio-g0bx")]
pub mod crs1 {
    use super::*;

    pin! {
        <Sync> for [
            PA8<4>,

            PD15<0>,

            PF0<0>,
        ],
    }
}*/

#[cfg(feature = "gpio-g0bx")]
pub mod fdcan1 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PA11<3>,

            PB8<3>,

            PC4<3>,

            PD0<3>,

            PD12<3>,
        ],

        <Tx, PushPull> for [
            PA12<3>,

            PB9<3>,

            PC5<3>,

            PD1<3>,

            PD13<3>,
        ],
    }
}

#[cfg(feature = "gpio-g0bx")]
pub mod fdcan2 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PB0<3>,

            PB5<3>,

            PB12<3>,

            PC2<3>,

            PD14<3>,
        ],

        <Tx, PushPull> for [
            PB1<3>,

            PB6<3>,

            PB13<3>,

            PC3<3>,

            PD15<3>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA9<6>,

            PB6<6>,

            PB8<6>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA10<6>,

            PB7<6>,

            PB9<6>,
        ],

        <Smba, OpenDrain> for [
            PA1<6>,

            PB5<6>,
        ],
    }
    use crate::pac::I2C1 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(feature = "gpio-g0bx")]
            PA7<8>,

            #[cfg(feature = "gpio-g0bx")]
            PA9<8>,

            PA11<6>,

            #[cfg(feature = "gpio-g0bx")]
            PB3<8>,

            PB10<6>,

            PB13<6>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(feature = "gpio-g0bx")]
            PA6<8>,

            #[cfg(feature = "gpio-g0bx")]
            PA10<8>,

            PA12<6>,

            #[cfg(feature = "gpio-g0bx")]
            PB4<8>,

            PB11<6>,

            PB14<6>,
        ],

        <Smba, OpenDrain> for [
            #[cfg(feature = "gpio-g0bx")]
            PA8<8>,

            #[cfg(feature = "gpio-g0bx")]
            PA15<8>,

            #[cfg(feature = "gpio-g0bx")]
            PB12<8>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(feature = "gpio-g0bx")]
pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA7<9>,

            PB3<6>,

            PC0<6>,
        ],

        <Sda, OpenDrain> for [
            PA6<9>,

            PB4<6>,

            PC1<6>,
        ],
    }
    use crate::pac::I2C3 as I2C;
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

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC9<0>,
        ],
    }
}

pub mod i2s1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA1<0>,

            PA5<0>,

            PB3<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD8<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE13<0>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA6<0>,

            PA11<0>,

            PB4<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD5<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE14<0>,
        ],

        <Sd, PushPull> for [
            PA2<0>,

            PA7<0>,

            PA12<0>,

            PB5<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD6<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE15<0>,
        ],

        <Ws, PushPull> for [
            PA4<0>,

            PA15<0>,

            PB0<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD9<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE12<0>,
        ],
    }
}

#[cfg(feature = "gpio-g0bx")]
pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA0<0>,

            PB8<1>,

            PB10<5>,

            PB13<0>,

            PD1<1>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA3<0>,

            PA9<4>,

            PB2<1>,

            PB6<4>,

            PB14<0>,

            PC2<1>,

            PD3<1>,
        ],

        <Sd, PushPull> for [
            PA4<1>,

            PA10<0>,

            PB7<1>,

            PB11<0>,

            PB15<0>,

            PC3<1>,

            PD4<1>,
        ],

        <Ws, PushPull> for [
            PA8<1>,

            PB9<5>,

            PB12<0>,

            PD0<1>,
        ],
    }
}

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<1>,

            PB9<0>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB6<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC3<0>,
        ],

        <In1, PushPull> for [
            PB5<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC0<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC12<0>,
        ],

        <In2, PushPull> for [
            PB7<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC2<0>,
        ],

        <Out> default:PushPull for [
            PA0<5>,

            PB0<5>,

            PB2<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC1<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD8<2>,
        ],
    }
}

pub mod lptim2 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PA5<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC3<2>,

            #[cfg(feature = "gpio-g0bx")]
            PD11<1>,
        ],

        <In1, PushPull> for [
            PB1<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC0<2>,

            #[cfg(feature = "gpio-g0bx")]
            PD12<1>,
        ],

        <Out> default:PushPull for [
            PA4<5>,

            PA8<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD6<2>,

            #[cfg(feature = "gpio-g0bx")]
            PD13<1>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA6<6>,

            PB13<1>,

            #[cfg(feature = "gpio-g0bx")]
            PF7<1>,
        ],

        <De, PushPull> for [
            PB1<6>,

            PB12<1>,

            #[cfg(feature = "gpio-g0bx")]
            PF6<1>,
        ],

        <Rts, PushPull> for [
            PB1<6>,

            PB12<1>,

            #[cfg(feature = "gpio-g0bx")]
            PF6<1>,
        ],

        <Rx, PushPull> for [
            PA3<6>,

            PB10<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC0<1>,

            #[cfg(feature = "gpio-g0bx")]
            PF5<1>,
        ],
    }

    pin! {
        <Tx> default::PushPull for [
            PA2<6>,

            PB11<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC1<1>,

            #[cfg(feature = "gpio-g0bx")]
            PF4<1>,
        ],
    }
}

#[cfg(feature = "gpio-g0bx")]
pub mod lpuart2 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB0<10>,

            PC8<3>,

            PD14<1>,
        ],

        <De, PushPull> for [
            PB1<10>,

            PC9<3>,

            PD15<1>,

            PF2<3>,
        ],

        <Rts, PushPull> for [
            PB1<10>,

            PC9<3>,

            PD15<1>,

            PF2<3>,
        ],

        <Rx, PushPull> for [
            PA13<10>,

            PB7<10>,

            PC1<3>,

            PC7<3>,

            PF3<1>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            PA14<10>,

            PB6<10>,

            PC0<3>,

            PC6<3>,

            PF2<1>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,

            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA9<0>,

            #[cfg(feature = "gpio-g0bx")]
            PD10<0>,

            PF2<0>,
        ],

        <Mco2, PushPull> for [
            #[cfg(feature = "gpio-g0bx")]
            PA10<3>,

            #[cfg(feature = "gpio-g0bx")]
            PA15<3>,

            #[cfg(feature = "gpio-g0bx")]
            PB2<3>,

            #[cfg(feature = "gpio-g0bx")]
            PD7<3>,
        ],

        <Osc32En, PushPull> for [
            PC15<0>,
        ],

        <OscEn, PushPull> for [
            PC15<1>,

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

            PB4<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD5<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE14<0>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA2<0>,

            PA7<0>,

            PA12<0>,

            PB5<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD6<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE15<0>,
        ],

        <Nss, PushPull> for [
            PA4<0>,

            PA15<0>,

            PB0<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD9<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE12<0>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA1<0>,

            PA5<0>,

            PB3<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD8<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE13<0>,
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
            PA3<0>,

            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA9<4>,

            PB2<1>,

            PB6<4>,

            PB14<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC2<1>,

            PD3<1>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA4<1>,

            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA10<0>,

            PB7<1>,

            PB11<0>,

            PB15<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC3<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD4<1>,
        ],

        <Nss, PushPull> for [
            PA8<1>,

            PB9<5>,

            PB12<0>,

            PD0<1>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA0<0>,

            PB8<1>,

            PB10<5>,

            PB13<0>,

            PD1<1>,
        ],
    }
    impl SpiCommon for crate::pac::SPI2 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(feature = "gpio-g0bx")]
pub mod spi3 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB4<9>,

            PC11<4>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB5<9>,

            PC12<4>,
        ],

        <Nss, PushPull> for [
            PA4<9>,

            PA15<9>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB3<9>,

            PC10<4>,
        ],
    }
    impl SpiCommon for crate::pac::SPI3 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod sys {
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

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC8<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE9<1>,
        ],

        <Ch1N> default:PushPull for [
            PA7<2>,

            PB13<2>,

            PD2<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE8<1>,
        ],

        <Ch2> default:PushPull for [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA9<2>,

            PB3<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC9<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE11<1>,
        ],

        <Ch2N> default:PushPull for [
            PB0<2>,

            PB14<2>,

            PD3<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE10<1>,
        ],

        <Ch3> default:PushPull for [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA10<2>,

            PB6<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC10<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE13<1>,
        ],

        <Ch3N> default:PushPull for [
            PB1<2>,

            PB15<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD4<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE12<1>,
        ],

        <Ch4> default:PushPull for [
            PA11<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC11<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE14<1>,
        ],
    }

    pin! {
        <Bk, PushPull> for [
            PA6<2>,

            PB12<2>,

            PC13<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD5<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE15<1>,
        ],

        <Bk2, PushPull> for [
            PA11<5>,

            PC14<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD9<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE14<2>,
        ],

        <Etr, PushPull> for [
            PA12<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE7<1>,
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
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<3> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim2 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PA5<2>,

            PA15<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC4<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PB3<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC5<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PB10<2>,

            PC6<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PB11<2>,

            PC7<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<2>,

            PA5<2>,

            PA15<2>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC4<2>,
        ],
    }

    use crate::pac::TIM2 as TIM;
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

pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<1>,

            PB4<1>,

            PC6<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE3<1>,
        ],

        <Ch2> default:PushPull for [
            PA7<1>,

            PB5<1>,

            PC7<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE4<1>,
        ],

        <Ch3> default:PushPull for [
            PB0<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC8<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE5<1>,
        ],

        <Ch4> default:PushPull for [
            PB1<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC9<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE6<1>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PD2<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE2<1>,
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

#[cfg(feature = "gpio-g0bx")]
pub mod tim4 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB6<9>,

            PD12<2>,
        ],

        <Ch2> default:PushPull for [
            PB7<9>,

            PD13<2>,
        ],

        <Ch3> default:PushPull for [
            PB8<9>,

            PD14<2>,
        ],

        <Ch4> default:PushPull for [
            PB9<9>,

            PD15<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PE0<2>,
        ],
    }

    use crate::pac::TIM4 as TIM;
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

            PB1<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC12<2>,

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
        <Ch1> default:PushPull for [
            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA2<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PB14<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC1<2>,

            #[cfg(feature = "gpio-g0bx")]
            PF12<0>,
        ],

        <Ch1N> default:PushPull for [
            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA1<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PB13<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PB15<4>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PF1<2>,
        ],

        <Ch2> default:PushPull for [
            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA3<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PB15<5>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC2<2>,

            #[cfg(feature = "gpio-g0bx")]
            PF13<0>,
        ],
    }

    pin! {
        <Bk, PushPull> for [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA9<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PB8<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PB12<5>,

            #[cfg(any(feature = "gpio-g05x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC15<2>,
        ],
    }

    use crate::pac::TIM15 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
}

pub mod tim16 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<5>,

            PB8<2>,

            PD0<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE0<0>,
        ],

        <Ch1N> default:PushPull for [
            PB6<2>,
        ],
    }

    pin! {
        <Bk, PushPull> for [
            PB5<2>,
        ],

    }

    use crate::pac::TIM16 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
}

pub mod tim17 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<5>,

            PB9<2>,

            PD1<2>,

            #[cfg(feature = "gpio-g0bx")]
            PE1<0>,
        ],

        <Ch1N> default:PushPull for [
            PB7<2>,
        ],
    }

    pin! {
        <Bk, PushPull> for [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA10<5>,

            PB4<5>,
        ],

    }

    use crate::pac::TIM17 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
}

#[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
pub mod ucpd1 {
    use super::*;

    #[cfg(feature = "gpio-g0bx")]
    pin! {
        <Frstx1, PushPull> for [
            PA2<4>,

            PA5<6>,

            PA7<6>,

            PB0<6>,

            PB14<1>,

            PC6<0>,

            PC12<1>,
        ],

        <Frstx2, PushPull> for [
            PA2<4>,

            PA5<6>,

            PA7<6>,

            PB0<6>,

            PB14<1>,

            PC6<0>,

            PC12<1>,
        ],
    }

    #[cfg(feature = "gpio-g07x")]
    pin! {
        <Frstx, PushPull> for [
            PA2<4>,

            PA5<6>,

            PA7<6>,

            PB0<6>,

            PB14<1>,

            PC6<0>,

            PC12<1>,
        ],

        <Txdata, PushPull> for [
            PA5<3>,

            PA6<3>,

            PC6<3>,

            PC8<3>,

            PC9<3>,

            PD0<3>,

            PD1<3>,

            PD8<3>,
        ],

        <Txgnd, PushPull> for [
            PA1<3>,

            PA9<3>,

            PB2<3>,

            PB6<3>,

            PB8<3>,

            PB10<3>,

            PB11<3>,

            PC0<3>,

            PC1<3>,
        ],
    }
}

#[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
pub mod ucpd2 {
    use super::*;

    #[cfg(feature = "gpio-g0bx")]
    pin! {
        <Frstx1, PushPull> for [
            PA0<6>,

            PA3<4>,

            PA4<6>,

            PB9<1>,

            PB12<6>,

            PC7<0>,

            PC8<0>,
        ],

        <Frstx2, PushPull> for [
            PA0<6>,

            PA3<4>,

            PA4<6>,

            PB9<1>,

            PB12<6>,

            PC7<0>,

            PC8<0>,
        ],
    }

    #[cfg(feature = "gpio-g07x")]
    pin! {
        <Frstx, PushPull> for [
            PA0<6>,

            PA3<4>,

            PA4<6>,

            PB9<1>,

            PB12<6>,

            PC7<0>,

            PC8<0>,
        ],

        <Txdata, PushPull> for [
            PA7<3>,

            PA8<3>,

            PC7<3>,

            PC10<3>,

            PC11<3>,

            PD2<3>,

            PD3<3>,

            PD9<3>,
        ],

        <Txgnd, PushPull> for [
            PA3<3>,

            PA10<3>,

            PB4<3>,

            PB9<3>,

            PB13<3>,

            PB14<3>,

            PC2<3>,

            PC3<3>,

            PC5<3>,
        ],
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA12<1>,

            PB3<4>,
        ],

        <Cts, PushPull> for [
            PA11<1>,

            PB4<4>,
        ],

        <De, PushPull> for [
            PA12<1>,

            PB3<4>,
        ],

        <Nss, PushPull> for [
            PA11<1>,

            PB4<4>,
        ],

        <Rts, PushPull> for [
            PA12<1>,

            PB3<4>,
        ],

        <Rx> default:PushPull for no:NoPin, [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA10<1>,

            PB7<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC5<1>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            #[cfg(any(feature = "gpio-g03x", feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PA9<1>,

            PB6<0>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PC4<1>,
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

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD4<0>,
        ],

        <Cts, PushPull> for [
            PA0<1>,

            PD3<0>,
        ],

        <De, PushPull> for [
            PA1<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD4<0>,
        ],

        <Nss, PushPull> for [
            PA0<1>,

            PD3<0>,
        ],

        <Rts, PushPull> for [
            PA1<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD4<0>,
        ],

        <Rx> default:PushPull for no:NoPin, [
            PA3<1>,

            PA15<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD6<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<1>,

            PA14<1>,

            #[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
            PD5<0>,
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

#[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
pub mod usart3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA15<5>,

            PB1<4>,

            PB14<4>,

            PD2<0>,

            #[cfg(feature = "gpio-g0bx")]
            PD12<0>,
        ],

        <Cts, PushPull> for [
            PA6<4>,

            PB13<4>,

            #[cfg(feature = "gpio-g0bx")]
            PD11<0>,
        ],

        <De, PushPull> for [
            PA15<5>,

            PB1<4>,

            PB14<4>,

            PD2<0>,

            #[cfg(feature = "gpio-g0bx")]
            PD12<0>,
        ],

        <Nss, PushPull> for [
            PA6<4>,

            PB13<4>,

            #[cfg(feature = "gpio-g0bx")]
            PD11<0>,
        ],

        <Rts, PushPull> for [
            PA15<5>,

            PB1<4>,

            PB14<4>,

            PD2<0>,

            #[cfg(feature = "gpio-g0bx")]
            PD12<0>,
        ],

        <Rx> default:PushPull for no:NoPin, [
            PB0<4>,

            PB9<4>,

            PB11<4>,

            PC5<0>,

            PC11<0>,

            PD9<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA5<4>,

            PB2<4>,

            PB8<4>,

            PB10<4>,

            PC4<0>,

            PC10<0>,

            PD8<0>,
        ],
    }
    use crate::pac::USART3 as USART;
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

#[cfg(any(feature = "gpio-g07x", feature = "gpio-g0bx"))]
pub mod usart4 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA15<4>,
        ],

        <Cts, PushPull> for [
            PB7<4>,
        ],

        <De, PushPull> for [
            PA15<4>,
        ],

        <Nss, PushPull> for [
            PB7<4>,
        ],

        <Rts, PushPull> for [
            PA15<4>,
        ],

        <Rx> default:PushPull for no:NoPin, [
            PA1<4>,

            PC11<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE9<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<4>,

            PC10<1>,

            #[cfg(feature = "gpio-g0bx")]
            PE8<0>,
        ],
    }
    use crate::pac::USART4 as USART;
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

#[cfg(feature = "gpio-g0bx")]
pub mod usart5 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB5<8>,

            PD4<3>,

            PE7<3>,
        ],

        <Cts, PushPull> for [
            PB6<8>,

            PD5<3>,

            PF7<3>,
        ],

        <De, PushPull> for [
            PB5<8>,

            PD4<3>,

            PE7<3>,
        ],

        <Nss, PushPull> for [
            PB6<8>,

            PD5<3>,

            PF7<3>,
        ],

        <Rts, PushPull> for [
            PB5<8>,

            PD4<3>,

            PE7<3>,
        ],

        <Rx> default:PushPull for no:NoPin, [
            PB1<8>,

            PB4<3>,

            PD2<3>,

            PE11<3>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB0<8>,

            PB3<3>,

            PC12<3>,

            PD3<3>,

            PE10<3>,
        ],
    }
    use crate::pac::USART5 as USART;
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

#[cfg(feature = "gpio-g0bx")]
pub mod usart6 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA7<3>,

            PB14<8>,

            PF3<3>,

            PF11<3>,
        ],

        <Cts, PushPull> for [
            PA6<3>,

            PB15<8>,

            PF12<3>,
        ],

        <De, PushPull> for [
            PA7<3>,

            PB14<8>,

            PF3<3>,

            PF11<3>,
        ],

        <Nss, PushPull> for [
            PA6<3>,

            PB15<8>,

            PF12<3>,
        ],

        <Rts, PushPull> for [
            PA7<3>,

            PB14<8>,

            PF3<3>,

            PF11<3>,
        ],

        <Rx> default:PushPull for no:NoPin, [
            PA5<3>,

            PB9<8>,

            PC1<4>,

            PF10<3>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA4<3>,

            PB8<8>,

            PC0<4>,

            PF9<3>,
        ],
    }
    use crate::pac::USART6 as USART;
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

#[cfg(feature = "gpio-g0bx")]
pub mod usb {
    use super::*;

    pin! {
        <Noe, PushPull> for [
            PA4<2>,

            PA13<2>,

            PA15<6>,

            PC9<6>,
        ],
    }
}
