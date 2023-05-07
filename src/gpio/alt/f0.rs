use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

#[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
pub mod can {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PA11<4>,

            PB8<4>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD0<0>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PA12<4>,

            PB9<4>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD1<0>,
        ],
    }
    impl CanCommon for crate::pac::CAN {
        type Rx = Rx;
        type Tx = Tx;
    }
}

#[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
pub mod comp1 {
    use super::*;

    analog! {
        <Inm> for [
            #[cfg(feature = "gpio-f051")]
            PA0<7>,

            #[cfg(feature = "gpio-f051")]
            PA4<7>,

            #[cfg(feature = "gpio-f051")]
            PA5<7>,
        ],

        <Inp> for [
            #[cfg(feature = "gpio-f051")]
            PA1<7>,
        ],
    }

    pin! {
        <Out, PushPull> for [
            PA0<7>,

            PA6<7>,

            PA11<7>,
        ],
    }
}

#[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
pub mod comp2 {
    use super::*;

    analog! {
        <Inm> for [
            #[cfg(feature = "gpio-f051")]
            PA2<7>,

            #[cfg(feature = "gpio-f051")]
            PA4<7>,

            #[cfg(feature = "gpio-f051")]
            PA5<7>,
        ],

        <Inp> for [
            #[cfg(feature = "gpio-f051")]
            PA3<7>,
        ],
    }

    pin! {
        <Out, PushPull> for [
            PA2<7>,

            PA7<7>,

            PA12<7>,
        ],
    }
}

/*#[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            PA8<4>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD15<0>,

            PF0<0>,
        ],
    }
}*/

#[cfg(feature = "gpio-f031")]
pub mod hdmi_cec {
    use super::*;

    pin! {
        <Cec, OpenDrain> for [
            PA5<1>,

            PB8<2>,

            PB10<2>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(any(feature = "gpio-f031", feature = "gpio-f042", feature = "gpio-f091"))]
            PA9<4>,

            #[cfg(feature = "gpio-f042")]
            PA11<5>,

            PB6<1>,

            PB8<1>,

            #[cfg(any(feature = "gpio-f031", feature = "gpio-f042"))]
            PB10<1>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f091"))]
            PF1<1>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(any(feature = "gpio-f031", feature = "gpio-f042", feature = "gpio-f091"))]
            PA10<4>,

            #[cfg(feature = "gpio-f042")]
            PA12<5>,

            PB7<1>,

            PB9<1>,

            #[cfg(any(feature = "gpio-f031", feature = "gpio-f042"))]
            PB11<1>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f091"))]
            PF0<1>,
        ],

        <Smba, OpenDrain> for [
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

pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(feature = "gpio-f091")]
            PA11<5>,

            #[cfg(any(
                feature = "gpio-f031",
                feature = "gpio-f051",
                feature = "gpio-f052",
                feature = "gpio-f091"
            ))]
            PB10<1>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
            PB13<5>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(feature = "gpio-f091")]
            PA12<5>,

            #[cfg(any(
                feature = "gpio-f031",
                feature = "gpio-f051",
                feature = "gpio-f052",
                feature = "gpio-f091"
            ))]
            PB11<1>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
            PB14<5>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2s1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA5<0>,

            PB3<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE13<1>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA6<0>,

            PB4<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE14<1>,
        ],

        <Sd, PushPull> for [
            PA7<0>,

            PB5<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE15<1>,
        ],

        <Ws, PushPull> for [
            PA4<0>,

            PA15<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE12<1>,
        ],
    }
}

#[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB10<5>,

            PB13<0>,

            PD1<1>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PB14<0>,

            PC2<1>,

            PD3<1>,
        ],

        <Sd, PushPull> for [
            PB15<0>,

            PC3<1>,

            PD4<1>,
        ],

        <Ws, PushPull> for [
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

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f091"))]
            PA9<5>,
        ],
    }
}

#[cfg(any(feature = "gpio-f031", feature = "gpio-f051"))]
pub mod rtc {
    use super::*;

    pin! {
        <Refin, PushPull> for [
            PB15<0>,
        ],

        <Tamp2, PushPull> for [
            PA0<0>,
        ],
    }
}

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<0>,

            PB4<0>,

            #[cfg(feature = "gpio-f031")]
            PB14<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE14<1>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<0>,

            PB5<0>,

            #[cfg(feature = "gpio-f031")]
            PB15<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE15<1>,
        ],

        <Nss, PushPull> for [
            PA4<0>,

            PA15<0>,

            #[cfg(feature = "gpio-f031")]
            PB12<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE12<1>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<0>,

            PB3<0>,

            #[cfg(feature = "gpio-f031")]
            PB13<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE13<1>,
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
            PB14<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PC2<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD3<1>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PC3<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD4<1>,
        ],

        <Nss, PushPull> for [
            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
            PB9<5>,

            PB12<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD0<1>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
            PB10<5>,

            PB13<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
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
        <Can, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC15<7>,
        ],

        <Comp, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC14<7>,
        ],

        <Dac, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC14<7>,
        ],

        <FunctionalityOnNewPin(forecastedInStingray64KPinoutFile), PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PD8<4>,
        ],

        <I2C, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC0<7>,
        ],

        <IrOut, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC13<7>,
        ],

        <NewFunctionality(notForecastedInStingray64KPinoutFile), PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PD4<4>,
        ],

        <NewPin(notExistingOnStingray64K), PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PD3<4>,
        ],

        <Spi, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC1<7>,
        ],

        <Swclk, PushPull> for [
            PA14<0>,
        ],

        <Swdio, PushPull> for [
            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
            PA13<0>,
        ],

        <System, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC3<7>,
        ],

        <Tim1, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC6<7>,
        ],

        <Tim14, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC9<7>,
        ],

        <Tim15, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC10<7>,
        ],

        <Tim16, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC11<7>,
        ],

        <Tim17, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC12<7>,
        ],

        <Tim2, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC7<7>,
        ],

        <Tim3, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC8<7>,
        ],

        <Touch, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC5<7>,
        ],

        <Usart, PushPull> for [
            #[cfg(feature = "gpio-f052")]
            PC2<7>,
        ],

        <Wkup1, PushPull> for [
            #[cfg(any(feature = "gpio-f031", feature = "gpio-f051"))]
            PA0<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE9<0>,
        ],

        <Ch1N> default:PushPull for [
            PA7<2>,

            PB13<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE8<0>,
        ],

        <Ch2> default:PushPull for [
            PA9<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE11<0>,
        ],

        <Ch2N> default:PushPull for [
            PB0<2>,

            PB14<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE10<0>,
        ],

        <Ch3> default:PushPull for [
            PA10<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE13<0>,
        ],

        <Ch3N> default:PushPull for [
            PB1<2>,

            PB15<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE12<0>,
        ],

        <Ch4> default:PushPull for [
            PA11<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE14<0>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<2>,

            PB12<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE15<0>,
        ],

        <Etr, PushPull> for [
            PA12<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE7<0>,
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
    impl TimBkin for TIM {
        type Bkin = Bkin;
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
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PB3<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PB10<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PB11<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<2>,

            PA5<2>,

            PA15<2>,
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

            #[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
            PC6<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE3<0>,
        ],

        <Ch2> default:PushPull for [
            PA7<1>,

            PB5<1>,

            #[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
            PC7<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE4<0>,
        ],

        <Ch3> default:PushPull for [
            PB0<1>,

            #[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
            PC8<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE5<0>,
        ],

        <Ch4> default:PushPull for [
            PB1<1>,

            #[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
            PC9<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE6<0>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            #[cfg(any(feature = "gpio-f051", feature = "gpio-f052", feature = "gpio-f091"))]
            PD2<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE2<0>,
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

            PB1<0>,
        ],
    }

    use crate::pac::TIM14 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

#[cfg(any(
    feature = "gpio-f031",
    feature = "gpio-f051",
    feature = "gpio-f052",
    feature = "gpio-f091"
))]
pub mod tim15 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<0>,

            PB14<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PF9<0>,
        ],

        <Ch1N> default:PushPull for [
            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PA1<5>,

            PB15<3>,
        ],

        <Ch2> default:PushPull for [
            PA3<0>,

            PB15<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PF10<0>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA9<0>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PB12<5>,
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
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod tim16 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<5>,

            PB8<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE0<0>,
        ],

        <Ch1N> default:PushPull for [
            PB6<2>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
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
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod tim17 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<5>,

            PB9<2>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PE1<0>,
        ],

        <Ch1N> default:PushPull for [
            PB7<2>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA10<0>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
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
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod tsc {
    use super::*;

    pin! {
        <Sync, PushPull> for [ // Low speed
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
    }

    #[cfg(any(
        feature = "gpio-f031",
        feature = "gpio-f051",
        feature = "gpio-f052",
        feature = "gpio-f091"
    ))]
    pin! {
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
    }

    #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
    pin! {
        <G3Io1> default:PushPull for [
            PC5<0>,
        ],

        <G7Io1> default:PushPull for [
            PE2<1>,
        ],

        <G7Io2> default:PushPull for [
            PE3<1>,
        ],

        <G7Io3> default:PushPull for [
            PE4<1>,
        ],

        <G7Io4> default:PushPull for [
            PE5<1>,
        ],

        <G8Io1> default:PushPull for [
            PD12<1>,
        ],

        <G8Io2> default:PushPull for [
            PD13<1>,
        ],

        <G8Io3> default:PushPull for [
            PD14<1>,
        ],

        <G8Io4> default:PushPull for [
            PD15<1>,
        ],
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            #[cfg(feature = "gpio-f031")]
            PA4<1>,

            PA8<1>,
        ],

        <Cts, PushPull> for [
            #[cfg(feature = "gpio-f031")]
            PA0<1>,

            PA11<1>,
        ],

        <De, PushPull> for [
            #[cfg(feature = "gpio-f031")]
            PA1<1>,

            PA12<1>,
        ],

        <Rts, PushPull> for [
            #[cfg(feature = "gpio-f031")]
            PA1<1>,

            PA12<1>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f031")]
            PA3<1>,

            PA10<1>,

            #[cfg(feature = "gpio-f031")]
            PA15<1>,

            PB7<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f031")]
            PA2<1>,

            PA9<1>,

            #[cfg(feature = "gpio-f031")]
            PA14<1>,

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
            PA4<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD7<0>,
        ],

        <Cts, PushPull> for [
            PA0<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD3<0>,
        ],

        <De, PushPull> for [
            PA1<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD4<0>,
        ],

        <Rts, PushPull> for [
            PA1<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD4<0>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<1>,

            PA15<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
            PD6<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<1>,

            PA14<1>,

            #[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
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

#[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
pub mod usart3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB0<4>,

            PB12<4>,

            PC12<1>,

            PD10<0>,
        ],

        <Cts, PushPull> for [
            PA6<4>,

            PB13<4>,

            PD11<0>,
        ],

        <De, PushPull> for [
            PB1<4>,

            PB14<4>,

            PD2<1>,

            PD12<0>,
        ],

        <Rts, PushPull> for [
            PB1<4>,

            PB14<4>,

            PD2<1>,

            PD12<0>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB11<4>,

            PC5<1>,

            PC11<1>,

            PD9<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB10<4>,

            PC4<1>,

            PC10<1>,

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

#[cfg(any(feature = "gpio-f052", feature = "gpio-f091"))]
pub mod usart4 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC12<0>,
        ],

        <Cts, PushPull> for [
            PB7<4>,
        ],

        <De, PushPull> for [
            PA15<4>,
        ],

        <Rts, PushPull> for [
            PA15<4>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA1<4>,

            PC11<0>,

            #[cfg(feature = "gpio-f091")]
            PE9<1>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<4>,

            PC10<0>,

            #[cfg(feature = "gpio-f091")]
            PE8<1>,
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

#[cfg(feature = "gpio-f091")]
pub mod usart5 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB5<4>,

            PE7<1>,
        ],

        <De, PushPull> for [
            PB5<4>,

            PE7<1>,
        ],

        <Rts, PushPull> for [
            PB5<4>,

            PE7<1>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB4<4>,

            PD2<2>,

            PE11<1>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB3<4>,

            PC12<2>,

            PE10<1>,
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

#[cfg(feature = "gpio-f091")]
pub mod usart6 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PF3<2>,
        ],

        <Rts, PushPull> for [
            PF3<2>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA5<5>,

            PC1<2>,

            PF10<1>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA4<5>,

            PC0<2>,

            PF9<1>,
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
}

#[cfg(feature = "gpio-f091")]
pub mod usart7 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PD15<2>,

            PF2<2>,
        ],

        <Rts, PushPull> for [
            PD15<2>,

            PF2<2>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC1<1>,

            PC7<1>,

            PF3<1>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC0<1>,

            PC6<1>,

            PF2<1>,
        ],
    }
    use crate::pac::USART7 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
}

#[cfg(feature = "gpio-f091")]
pub mod usart8 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PD12<2>,
        ],

        <Rts, PushPull> for [
            PD12<2>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC3<2>,

            PC9<1>,

            PD14<0>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC2<2>,

            PC8<1>,

            PD13<0>,
        ],
    }
    use crate::pac::USART8 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
}

#[cfg(any(feature = "gpio-f042", feature = "gpio-f052", feature = "gpio-f091"))]
pub mod usb {
    use super::*;

    pin! {
        <Noe, PushPull> for [
            #[cfg(feature = "gpio-f042")]
            PA4<2>,

            #[cfg(any(feature = "gpio-f042", feature = "gpio-f052"))]
            PA13<2>,

            #[cfg(feature = "gpio-f042")]
            PA15<5>,
        ],

        <Oe, PushPull> for [
            #[cfg(feature = "gpio-f091")]
            PA13<2>,
        ],
    }
}
