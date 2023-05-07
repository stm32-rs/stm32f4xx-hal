use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA0<7>,

            PA6<7>,

            #[cfg(feature = "gpio-l021")]
            PA9<7>,

            #[cfg(feature = "gpio-l021")]
            PA10<7>,

            PA11<7>,

            #[cfg(feature = "gpio-l021")]
            PA13<7>,
        ],
    }
}

pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA2<7>,

            #[cfg(feature = "gpio-l021")]
            PA4<7>,

            PA7<7>,

            PA12<7>,

            #[cfg(feature = "gpio-l021")]
            PA14<7>,
        ],
    }
}

/*#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            PA8<2>,

            #[cfg(feature = "gpio-l071")]
            PD15<0>,

            PH0<0>,
        ],
    }
}*/

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(feature = "gpio-l021")]
            PA4<3>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA9<1>,

            #[cfg(feature = "gpio-l071")]
            PA9<6>,

            PB6<1>,

            PB8<4>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA10<1>,

            #[cfg(feature = "gpio-l071")]
            PA10<6>,

            #[cfg(feature = "gpio-l021")]
            PA13<3>,

            PB7<1>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PB9<4>,
        ],

        <Smba, OpenDrain> for [
            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA1<3>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA14<3>,

            PB5<3>,
        ],
    }
    use crate::pac::I2C1 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB10<6>,

            PB13<5>,
        ],

        <Sda, OpenDrain> for [
            PB11<6>,

            PB14<5>,
        ],

        <Smba, OpenDrain> for [
            PB12<5>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(feature = "gpio-l071")]
pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<7>,

            PC0<7>,
        ],

        <Sda, OpenDrain> for [
            PB4<7>,

            PC1<7>,

            PC9<7>,
        ],

        <Smba, OpenDrain> for [
            PA9<7>,

            PB2<7>,
        ],
    }
    use crate::pac::I2C3 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB13<0>,

            #[cfg(feature = "gpio-l071")]
            PD1<1>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PB14<0>,

            PC2<2>,

            #[cfg(feature = "gpio-l071")]
            PD3<2>,
        ],

        <Sd, PushPull> for [
            PB15<0>,

            PC3<2>,

            #[cfg(feature = "gpio-l071")]
            PD4<1>,
        ],

        <Ws, PushPull> for [
            PB9<5>,

            PB12<0>,

            #[cfg(feature = "gpio-l071")]
            PD0<1>,
        ],
    }
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod lcd {
    use super::*;

    pin! {
        <Com0, PushPull> for [
            PA8<1>,
        ],

        <Com1, PushPull> for [
            PA9<1>,
        ],

        <Com2, PushPull> for [
            PA10<1>,
        ],

        <Com3, PushPull> for [
            PB9<1>,
        ],

        <Com4, PushPull> for [
            PC10<1>,
        ],

        <Com5, PushPull> for [
            PC11<1>,
        ],

        <Com6, PushPull> for [
            PC12<1>,
        ],

        <Com7, PushPull> for [
            PD2<1>,
        ],

        <Seg0, PushPull> for [
            PA1<1>,
        ],

        <Seg1, PushPull> for [
            PA2<1>,
        ],

        <Seg2, PushPull> for [
            PA3<1>,
        ],

        <Seg3, PushPull> for [
            PA6<1>,
        ],

        <Seg4, PushPull> for [
            PA7<1>,
        ],

        <Seg5, PushPull> for [
            PB0<1>,
        ],

        <Seg6, PushPull> for [
            PB1<1>,
        ],

        <Seg7, PushPull> for [
            PB3<1>,
        ],

        <Seg8, PushPull> for [
            PB4<1>,
        ],

        <Seg9, PushPull> for [
            PB5<1>,
        ],

        <Seg10, PushPull> for [
            PB10<1>,
        ],

        <Seg11, PushPull> for [
            PB11<1>,
        ],

        <Seg12, PushPull> for [
            PB12<1>,
        ],

        <Seg13, PushPull> for [
            PB13<1>,
        ],

        <Seg14, PushPull> for [
            PB14<1>,
        ],

        <Seg15, PushPull> for [
            PB15<1>,
        ],

        <Seg16, PushPull> for [
            PB8<1>,
        ],

        <Seg17, PushPull> for [
            PA15<1>,
        ],

        <Seg18, PushPull> for [
            PC0<1>,
        ],

        <Seg19, PushPull> for [
            PC1<1>,
        ],

        <Seg20, PushPull> for [
            PC2<1>,
        ],

        <Seg21, PushPull> for [
            PC3<1>,
        ],

        <Seg22, PushPull> for [
            PC4<1>,
        ],

        <Seg23, PushPull> for [
            PC5<1>,
        ],

        <Seg24, PushPull> for [
            PC6<1>,
        ],

        <Seg25, PushPull> for [
            PC7<1>,
        ],

        <Seg26, PushPull> for [
            PC8<1>,
        ],

        <Seg27, PushPull> for [
            PC9<1>,
        ],

        <Seg28, PushPull> for [
            PC10<1>,

            #[cfg(feature = "gpio-l071")]
            PD8<1>,
        ],

        <Seg29, PushPull> for [
            PC11<1>,

            #[cfg(feature = "gpio-l071")]
            PD9<1>,
        ],

        <Seg30, PushPull> for [
            PC12<1>,

            #[cfg(feature = "gpio-l071")]
            PD10<1>,
        ],

        <Seg31, PushPull> for [
            PD2<1>,

            #[cfg(feature = "gpio-l071")]
            PD11<1>,
        ],

        <Seg40, PushPull> for [
            #[cfg(feature = "gpio-l051")]
            PC10<1>,

            #[cfg(feature = "gpio-l071")]
            PE10<1>,
        ],

        <Seg41, PushPull> for [
            #[cfg(feature = "gpio-l051")]
            PC11<1>,

            #[cfg(feature = "gpio-l071")]
            PE13<1>,
        ],

        <Seg42, PushPull> for [
            #[cfg(feature = "gpio-l051")]
            PC12<1>,

            #[cfg(feature = "gpio-l071")]
            PE14<1>,
        ],

        <Seg43, PushPull> for [
            #[cfg(feature = "gpio-l051")]
            PD2<1>,

            #[cfg(feature = "gpio-l071")]
            PE15<1>,
        ],
    }

    #[cfg(feature = "gpio-l071")]
    pin! {
        <Seg32, PushPull> for [
            PD12<1>,
        ],

        <Seg33, PushPull> for [
            PD13<1>,
        ],

        <Seg34, PushPull> for [
            PD14<1>,
        ],

        <Seg35, PushPull> for [
            PD15<1>,
        ],

        <Seg36, PushPull> for [
            PE0<1>,
        ],

        <Seg37, PushPull> for [
            PE1<1>,
        ],

        <Seg38, PushPull> for [
            PE2<1>,
        ],

        <Seg39, PushPull> for [
            PE3<1>,
        ],

        <Seg44, PushPull> for [
            PD3<1>,
        ],

        <Seg45, PushPull> for [
            PE7<1>,
        ],

        <Seg46, PushPull> for [
            PE8<1>,
        ],

        <Seg47, PushPull> for [
            PE9<1>,
        ],

        <Seg48, PushPull> for [
            PC10<1>,
        ],

        <Seg49, PushPull> for [
            PC11<1>,
        ],

        <Seg50, PushPull> for [
            PC12<1>,
        ],

        <Seg51, PushPull> for [
            PD2<1>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            #[cfg(feature = "gpio-l021")]
            PA4<2>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA6<1>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA13<1>,

            PB6<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC3<0>,
        ],

        <In1, PushPull> for [
            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA0<1>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA4<1>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA8<2>,

            #[cfg(feature = "gpio-l021")]
            PB1<2>,

            PB5<2>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PC0<0>,
        ],

        <In2, PushPull> for [
            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA1<1>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA5<1>,

            PB7<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC2<0>,
        ],

        <Out> default:PushPull for [
            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA7<1>,

            #[cfg(feature = "gpio-l021")]
            PA9<2>,

            #[cfg(feature = "gpio-l021")]
            PA11<1>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA14<1>,

            PB2<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC1<0>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA6<4>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB13<4>,

            #[cfg(feature = "gpio-l031")]
            PB13<6>,

            #[cfg(feature = "gpio-l071")]
            PD11<0>,
        ],

        <De, PushPull> for [
            PB1<4>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB12<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB14<4>,

            #[cfg(feature = "gpio-l031")]
            PB14<6>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PD2<0>,

            #[cfg(feature = "gpio-l071")]
            PD12<0>,
        ],

        <Rts, PushPull> for [
            PB1<4>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB12<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB14<4>,

            #[cfg(feature = "gpio-l031")]
            PB14<6>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PD2<0>,

            #[cfg(feature = "gpio-l071")]
            PD12<0>,
        ],

        <Rx, PushPull> for [
            #[cfg(feature = "gpio-l021")]
            PA0<6>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031", feature = "gpio-l071"))]
            PA3<6>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031", feature = "gpio-l071"))]
            PA13<6>,

            #[cfg(feature = "gpio-l021")]
            PB7<6>,

            #[cfg(feature = "gpio-l071")]
            PB10<7>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB11<4>,

            #[cfg(feature = "gpio-l031")]
            PB11<6>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l071"))]
            PC0<6>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC5<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC11<0>,

            #[cfg(feature = "gpio-l071")]
            PD9<0>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            #[cfg(feature = "gpio-l021")]
            PA1<6>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031", feature = "gpio-l071"))]
            PA2<6>,

            #[cfg(feature = "gpio-l021")]
            PA4<6>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031", feature = "gpio-l071"))]
            PA14<6>,

            #[cfg(feature = "gpio-l021")]
            PB6<6>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB10<4>,

            #[cfg(feature = "gpio-l031")]
            PB10<6>,

            #[cfg(feature = "gpio-l071")]
            PB11<7>,

            #[cfg(feature = "gpio-l071")]
            PC1<6>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC4<2>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC10<0>,

            #[cfg(feature = "gpio-l071")]
            PD8<0>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,

            PA9<0>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l071"))]
            PB13<2>,
        ],
    }
}

pub mod rtc {
    use super::*;

    pin! {
        <OutAlarm, PushPull> for [
            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PB14<2>,
        ],

        <OutCalib, PushPull> for [
            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PB14<2>,
        ],

        <Refin, PushPull> for [
            #[cfg(feature = "gpio-l021")]
            PA10<2>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PB15<2>,
        ],
    }
}

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<0>,

            PA11<0>,

            #[cfg(feature = "gpio-l021")]
            PA14<5>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB0<1>,

            PB4<0>,

            #[cfg(feature = "gpio-l031")]
            PB14<0>,

            #[cfg(feature = "gpio-l071")]
            PE14<2>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<0>,

            PA12<0>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB1<1>,

            PB5<0>,

            #[cfg(feature = "gpio-l031")]
            PB15<0>,

            #[cfg(feature = "gpio-l071")]
            PE15<2>,
        ],

        <Nss, PushPull> for [
            PA4<0>,

            PA15<0>,

            #[cfg(feature = "gpio-l021")]
            PB8<5>,

            #[cfg(feature = "gpio-l031")]
            PB12<0>,

            #[cfg(feature = "gpio-l071")]
            PE12<2>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<0>,

            #[cfg(feature = "gpio-l021")]
            PA13<5>,

            PB3<0>,

            #[cfg(feature = "gpio-l031")]
            PB13<0>,

            #[cfg(feature = "gpio-l071")]
            PE13<2>,
        ],
    }
    impl SpiCommon for crate::pac::SPI1 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod spi2 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB14<0>,

            PC2<2>,

            #[cfg(feature = "gpio-l071")]
            PD3<2>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<0>,

            PC3<2>,

            #[cfg(feature = "gpio-l071")]
            PD4<1>,
        ],

        <Nss, PushPull> for [
            PB9<5>,

            PB12<0>,

            #[cfg(feature = "gpio-l071")]
            PD0<1>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB10<5>,

            PB13<0>,

            #[cfg(feature = "gpio-l071")]
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

pub mod tim2 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PA5<5>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA8<5>,

            PA15<5>,

            #[cfg(feature = "gpio-l071")]
            PE9<0>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            #[cfg(feature = "gpio-l021")]
            PB0<2>,

            PB3<2>,

            #[cfg(feature = "gpio-l071")]
            PE10<0>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            #[cfg(feature = "gpio-l021")]
            PA10<5>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB0<5>,

            #[cfg(feature = "gpio-l021")]
            PB6<5>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PB10<2>,

            #[cfg(feature = "gpio-l071")]
            PE11<0>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB1<5>,

            #[cfg(feature = "gpio-l021")]
            PB7<5>,

            #[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
            PB11<2>,

            #[cfg(feature = "gpio-l071")]
            PE12<0>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<5>,

            #[cfg(feature = "gpio-l021")]
            PA4<5>,

            PA5<2>,

            PA15<2>,

            #[cfg(feature = "gpio-l071")]
            PE9<2>,
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

#[cfg(feature = "gpio-l071")]
pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<2>,

            PB4<2>,

            PC6<2>,

            PE3<2>,
        ],

        <Ch2> default:PushPull for [
            PA7<2>,

            PB5<4>,

            PC7<2>,

            PE4<2>,
        ],

        <Ch3> default:PushPull for [
            PB0<2>,

            PC8<2>,

            PE5<2>,
        ],

        <Ch4> default:PushPull for [
            PB1<2>,

            PC9<2>,

            PE6<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PD2<2>,

            PE2<2>,
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

pub mod tim21 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<0>,

            #[cfg(feature = "gpio-l021")]
            PA10<0>,

            #[cfg(feature = "gpio-l021")]
            PB5<5>,

            #[cfg(feature = "gpio-l031")]
            PB6<5>,

            #[cfg(feature = "gpio-l031")]
            PB13<5>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB13<6>,

            #[cfg(feature = "gpio-l071")]
            PD0<0>,

            #[cfg(feature = "gpio-l071")]
            PE5<0>,
        ],

        <Ch2> default:PushPull for [
            PA3<0>,

            #[cfg(feature = "gpio-l021")]
            PA9<5>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA11<5>,

            #[cfg(feature = "gpio-l031")]
            PB14<5>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PB14<6>,

            #[cfg(feature = "gpio-l071")]
            PD7<1>,

            #[cfg(feature = "gpio-l071")]
            PE6<0>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA1<5>,

            #[cfg(feature = "gpio-l021")]
            PA7<5>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC9<0>,
        ],
    }

    use crate::pac::TIM21 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(any(feature = "gpio-l031", feature = "gpio-l051", feature = "gpio-l071"))]
pub mod tim22 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<5>,

            #[cfg(feature = "gpio-l031")]
            PA9<5>,

            PB4<4>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC6<0>,

            #[cfg(feature = "gpio-l071")]
            PE3<0>,
        ],

        <Ch2> default:PushPull for [
            PA7<5>,

            #[cfg(feature = "gpio-l031")]
            PA10<5>,

            PB5<4>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC7<0>,

            #[cfg(feature = "gpio-l071")]
            PE4<0>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA4<5>,

            #[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
            PC8<0>,
        ],
    }

    use crate::pac::TIM22 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod tsc {
    use super::*;

    pin! {
        <Sync, PushPull> for [
            PB8<3>,

            PB10<3>,
        ],
    }

    pin! {
        <G1Io1> default:PushPull for [
            PA0<3>,
        ],

        <G1Io2> default:PushPull for [
            PA1<3>,
        ],

        <G1Io3> default:PushPull for [
            PA2<3>,
        ],

        <G1Io4> default:PushPull for [
            PA3<3>,
        ],

        <G2Io1> default:PushPull for [
            PA4<3>,
        ],

        <G2Io2> default:PushPull for [
            PA5<3>,
        ],

        <G2Io3> default:PushPull for [
            PA6<3>,
        ],

        <G2Io4> default:PushPull for [
            PA7<3>,
        ],

        <G3Io1> default:PushPull for [
            PC5<3>,
        ],

        <G3Io2> default:PushPull for [
            PB0<3>,
        ],

        <G3Io3> default:PushPull for [
            PB1<3>,
        ],

        <G3Io4> default:PushPull for [
            PB2<3>,
        ],

        <G4Io1> default:PushPull for [
            PA9<3>,
        ],

        <G4Io2> default:PushPull for [
            PA10<3>,
        ],

        <G4Io3> default:PushPull for [
            PA11<3>,
        ],

        <G4Io4> default:PushPull for [
            PA12<3>,
        ],

        <G5Io1> default:PushPull for [
            PB3<3>,
        ],

        <G5Io2> default:PushPull for [
            PB4<3>,
        ],

        <G5Io3> default:PushPull for [
            PB6<3>,
        ],

        <G5Io4> default:PushPull for [
            PB7<3>,
        ],

        <G6Io1> default:PushPull for [
            PB11<3>,
        ],

        <G6Io2> default:PushPull for [
            PB12<3>,
        ],

        <G6Io3> default:PushPull for [
            PB13<3>,
        ],

        <G6Io4> default:PushPull for [
            PB14<3>,
        ],

        <G7Io1> default:PushPull for [
            PC0<3>,
        ],

        <G7Io2> default:PushPull for [
            PC1<3>,
        ],

        <G7Io3> default:PushPull for [
            PC2<3>,
        ],

        <G7Io4> default:PushPull for [
            PC3<3>,
        ],

        <G8Io1> default:PushPull for [
            PC6<3>,
        ],

        <G8Io2> default:PushPull for [
            PC7<3>,
        ],

        <G8Io3> default:PushPull for [
            PC8<3>,
        ],

        <G8Io4> default:PushPull for [
            PC9<3>,
        ],
    }
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<4>,

            #[cfg(feature = "gpio-l071")]
            PB5<5>,
        ],

        <Cts, PushPull> for [
            PA11<4>,

            #[cfg(feature = "gpio-l071")]
            PB4<5>,
        ],

        <De, PushPull> for [
            PA12<4>,

            #[cfg(feature = "gpio-l071")]
            PB3<5>,
        ],

        <Rts, PushPull> for [
            PA12<4>,

            #[cfg(feature = "gpio-l071")]
            PB3<5>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA10<4>,

            PB7<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<4>,

            PB6<0>,
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
            PA4<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA8<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB1<0>,

            #[cfg(feature = "gpio-l071")]
            PD7<0>,
        ],

        <Cts, PushPull> for [
            PA0<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA7<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA11<4>,

            #[cfg(feature = "gpio-l071")]
            PD3<0>,
        ],

        <De, PushPull> for [
            PA1<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA12<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB0<4>,

            #[cfg(feature = "gpio-l071")]
            PD4<0>,
        ],

        <Rts, PushPull> for [
            PA1<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA12<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB0<4>,

            #[cfg(feature = "gpio-l071")]
            PD4<0>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            #[cfg(feature = "gpio-l021")]
            PA0<0>,

            PA3<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA10<4>,

            PA15<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB7<0>,

            #[cfg(feature = "gpio-l071")]
            PD6<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PA9<4>,

            PA14<4>,

            #[cfg(any(feature = "gpio-l021", feature = "gpio-l031"))]
            PB6<0>,

            #[cfg(feature = "gpio-l021")]
            PB8<0>,

            #[cfg(feature = "gpio-l071")]
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

#[cfg(feature = "gpio-l071")]
pub mod usart4 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC12<6>,
        ],

        <Cts, PushPull> for [
            PB7<6>,
        ],

        <De, PushPull> for [
            PA15<6>,
        ],

        <Rts, PushPull> for [
            PA15<6>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA1<6>,

            PC11<6>,

            PE9<6>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<6>,

            PC10<6>,

            PE8<6>,
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

#[cfg(feature = "gpio-l071")]
pub mod usart5 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB5<6>,

            PE7<6>,
        ],

        <De, PushPull> for [
            PB5<6>,

            PE7<6>,
        ],

        <Rts, PushPull> for [
            PB5<6>,

            PE7<6>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB4<6>,

            PD2<6>,

            PE11<6>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB3<6>,

            PC12<2>,

            PE10<6>,
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
}

#[cfg(any(feature = "gpio-l051", feature = "gpio-l071"))]
pub mod usb {
    use super::*;

    pin! {
        <Noe, PushPull> for [
            PA13<2>,

            PC9<2>,
        ],
    }
}
