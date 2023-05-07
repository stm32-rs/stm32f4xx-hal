use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

#[cfg(feature = "gpio-l162xe")]
pub mod comp1 {
    use super::*;

    analog! {
        <Inp> for [
            PA0<14>,

            PA1<14>,

            PA2<14>,

            PA3<14>,

            PA4<14>,

            PA5<14>,

            PA6<14>,

            PA7<14>,

            PB0<14>,

            PB1<14>,

            PB12<14>,

            PB13<14>,

            PB14<14>,

            PB15<14>,

            PC0<14>,

            PC1<14>,

            PC2<14>,

            PC3<14>,

            PC4<14>,

            PC5<14>,

            PE7<14>,

            PE8<14>,

            PE9<14>,

            PE10<14>,

            PF6<14>,

            PF7<14>,

            PF8<14>,

            PF9<14>,

            PF10<14>,
        ],
    }
}

#[cfg(feature = "gpio-l162xe")]
pub mod comp2 {
    use super::*;

    analog! {
        <Inm> for [
            PB3<14>,
        ],

        <Inp> for [
            PB4<14>,

            PB5<14>,

            PB6<14>,

            PB7<14>,
        ],
    }
}

#[cfg(feature = "gpio-l162xd")]
pub mod fmc {
    use super::*;

    pin! {
        <A0, PushPull> for [
            PF0<12>,
        ],

        <A1, PushPull> for [
            PF1<12>,
        ],

        <A10, PushPull> for [
            PG0<12>,
        ],

        <A11, PushPull> for [
            PG1<12>,
        ],

        <A12, PushPull> for [
            PG2<12>,
        ],

        <A13, PushPull> for [
            PG3<12>,
        ],

        <A14, PushPull> for [
            PG4<12>,
        ],

        <A15, PushPull> for [
            PG5<12>,
        ],

        <A16, PushPull> for [
            PD11<12>,
        ],

        <A17, PushPull> for [
            PD12<12>,
        ],

        <A18, PushPull> for [
            PD13<12>,
        ],

        <A19, PushPull> for [
            PE3<12>,
        ],

        <A2, PushPull> for [
            PF2<12>,
        ],

        <A20, PushPull> for [
            PE4<12>,
        ],

        <A21, PushPull> for [
            PE5<12>,
        ],

        <A22, PushPull> for [
            PH2<12>,
        ],

        <A23, PushPull> for [
            PE2<12>,
        ],

        <A24, PushPull> for [
            PG13<12>,
        ],

        <A25, PushPull> for [
            PG14<12>,
        ],

        <A3, PushPull> for [
            PF3<12>,
        ],

        <A4, PushPull> for [
            PF4<12>,
        ],

        <A5, PushPull> for [
            PF5<12>,
        ],

        <A6, PushPull> for [
            PF12<12>,
        ],

        <A7, PushPull> for [
            PF13<12>,
        ],

        <A8, PushPull> for [
            PF14<12>,
        ],

        <A9, PushPull> for [
            PF15<12>,
        ],

        <Clk, PushPull> for [
            PD3<12>,
        ],

        <D0, PushPull> for [
            PD14<12>,
        ],

        <D1, PushPull> for [
            PD15<12>,
        ],

        <D10, PushPull> for [
            PE13<12>,
        ],

        <D11, PushPull> for [
            PE14<12>,
        ],

        <D12, PushPull> for [
            PE15<12>,
        ],

        <D13, PushPull> for [
            PD8<12>,
        ],

        <D14, PushPull> for [
            PD9<12>,
        ],

        <D15, PushPull> for [
            PD10<12>,
        ],

        <D2, PushPull> for [
            PD0<12>,
        ],

        <D3, PushPull> for [
            PD1<12>,
        ],

        <D4, PushPull> for [
            PE7<12>,
        ],

        <D5, PushPull> for [
            PE8<12>,
        ],

        <D6, PushPull> for [
            PE9<12>,
        ],

        <D7, PushPull> for [
            PE10<12>,
        ],

        <D8, PushPull> for [
            PE11<12>,
        ],

        <D9, PushPull> for [
            PE12<12>,
        ],

        <Da0, PushPull> for [
            PD14<12>,
        ],

        <Da1, PushPull> for [
            PD15<12>,
        ],

        <Da10, PushPull> for [
            PE13<12>,
        ],

        <Da11, PushPull> for [
            PE14<12>,
        ],

        <Da12, PushPull> for [
            PE15<12>,
        ],

        <Da13, PushPull> for [
            PD8<12>,
        ],

        <Da14, PushPull> for [
            PD9<12>,
        ],

        <Da15, PushPull> for [
            PD10<12>,
        ],

        <Da2, PushPull> for [
            PD0<12>,
        ],

        <Da3, PushPull> for [
            PD1<12>,
        ],

        <Da4, PushPull> for [
            PE7<12>,
        ],

        <Da5, PushPull> for [
            PE8<12>,
        ],

        <Da6, PushPull> for [
            PE9<12>,
        ],

        <Da7, PushPull> for [
            PE10<12>,
        ],

        <Da8, PushPull> for [
            PE11<12>,
        ],

        <Da9, PushPull> for [
            PE12<12>,
        ],

        <Nbl0, PushPull> for [
            PE0<12>,
        ],

        <Nbl1, PushPull> for [
            PE1<12>,
        ],

        <Ne1, PushPull> for [
            PD7<12>,
        ],

        <Ne2, PushPull> for [
            PG9<12>,
        ],

        <Ne3, PushPull> for [
            PG10<12>,
        ],

        <Ne4, PushPull> for [
            PG12<12>,
        ],

        <Nl, PushPull> for [
            PB7<12>,
        ],

        <Noe, PushPull> for [
            PD4<12>,
        ],

        <Nwait, PushPull> for [
            PD6<12>,
        ],

        <Nwe, PushPull> for [
            PD5<12>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB6<4>,

            PB8<4>,
        ],

        <Sda, OpenDrain> for [
            PB7<4>,

            PB9<4>,
        ],

        <Smba, OpenDrain> for [
            PB5<4>,
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
            PB10<4>,
        ],

        <Sda, OpenDrain> for [
            PB11<4>,
        ],

        <Smba, OpenDrain> for [
            PB12<4>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(any(
    feature = "gpio-l152xc",
    feature = "gpio-l15xxa",
    feature = "gpio-l162xd",
    feature = "gpio-l162xe"
))]
pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB13<5>,

            PD1<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PC6<5>,
        ],

        <Sd, PushPull> for [
            PB15<5>,

            PD4<5>,
        ],

        <Ws, PushPull> for [
            PB12<5>,

            PD0<5>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-l152xc",
    feature = "gpio-l15xxa",
    feature = "gpio-l162xd",
    feature = "gpio-l162xe"
))]
pub mod i2s3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB3<6>,

            PC10<6>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PC7<6>,
        ],

        <Sd, PushPull> for [
            PB5<6>,

            PC12<6>,
        ],

        <Ws, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],
    }
}

pub mod lcd {
    use super::*;

    pin! {
        <Com0, PushPull> for [
            PA8<11>,
        ],

        <Com1, PushPull> for [
            PA9<11>,
        ],

        <Com2, PushPull> for [
            PA10<11>,
        ],

        <Com3, PushPull> for [
            PB9<11>,
        ],

        <Com4, PushPull> for [
            PC10<11>,
        ],

        <Com5, PushPull> for [
            PC11<11>,
        ],

        <Com6, PushPull> for [
            PC12<11>,
        ],

        <Com7, PushPull> for [
            PD2<11>,
        ],

        <Seg0, PushPull> for [
            PA1<11>,
        ],

        <Seg1, PushPull> for [
            PA2<11>,
        ],

        <Seg10, PushPull> for [
            PB10<11>,
        ],

        <Seg11, PushPull> for [
            PB11<11>,
        ],

        <Seg12, PushPull> for [
            PB12<11>,
        ],

        <Seg13, PushPull> for [
            PB13<11>,
        ],

        <Seg14, PushPull> for [
            PB14<11>,
        ],

        <Seg15, PushPull> for [
            PB15<11>,
        ],

        <Seg16, PushPull> for [
            PB8<11>,
        ],

        <Seg17, PushPull> for [
            PA15<11>,
        ],

        <Seg18, PushPull> for [
            PC0<11>,
        ],

        <Seg19, PushPull> for [
            PC1<11>,
        ],

        <Seg2, PushPull> for [
            PA3<11>,
        ],

        <Seg20, PushPull> for [
            PC2<11>,
        ],

        <Seg21, PushPull> for [
            PC3<11>,
        ],

        <Seg22, PushPull> for [
            PC4<11>,
        ],

        <Seg23, PushPull> for [
            PC5<11>,
        ],

        <Seg24, PushPull> for [
            PC6<11>,
        ],

        <Seg25, PushPull> for [
            PC7<11>,
        ],

        <Seg26, PushPull> for [
            PC8<11>,
        ],

        <Seg27, PushPull> for [
            PC9<11>,
        ],

        <Seg28, PushPull> for [
            PC10<11>,

            PD8<11>,
        ],

        <Seg29, PushPull> for [
            PC11<11>,

            PD9<11>,
        ],

        <Seg3, PushPull> for [
            PA6<11>,
        ],

        <Seg30, PushPull> for [
            PC12<11>,

            PD10<11>,
        ],

        <Seg31, PushPull> for [
            PD2<11>,

            PD11<11>,
        ],

        <Seg32, PushPull> for [
            PD12<11>,
        ],

        <Seg33, PushPull> for [
            PD13<11>,
        ],

        <Seg34, PushPull> for [
            PD14<11>,
        ],

        <Seg35, PushPull> for [
            PD15<11>,
        ],

        <Seg36, PushPull> for [
            PE0<11>,
        ],

        <Seg37, PushPull> for [
            PE1<11>,
        ],

        <Seg38, PushPull> for [
            PE2<11>,
        ],

        <Seg39, PushPull> for [
            PE3<11>,
        ],

        <Seg4, PushPull> for [
            PA7<11>,
        ],

        <Seg40, PushPull> for [
            PC10<11>,
        ],

        <Seg41, PushPull> for [
            PC11<11>,
        ],

        <Seg42, PushPull> for [
            PC12<11>,
        ],

        <Seg43, PushPull> for [
            PD2<11>,
        ],

        <Seg5, PushPull> for [
            PB0<11>,
        ],

        <Seg6, PushPull> for [
            PB1<11>,
        ],

        <Seg7, PushPull> for [
            PB3<11>,
        ],

        <Seg8, PushPull> for [
            PB4<11>,
        ],

        <Seg9, PushPull> for [
            PB5<11>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,
        ],

        <Osc32In, PushPull> for [
            PC14<0>,
        ],

        <Osc32Out, PushPull> for [
            PC15<0>,
        ],

        <OscIn, PushPull> for [
            PH0<0>,
        ],

        <OscOut, PushPull> for [
            PH1<0>,
        ],
    }
}

pub mod rtc {
    use super::*;

    pin! {
        <Refin, PushPull> for [
            PB15<0>,
        ],

        <Tamp2, PushPull> for [
            #[cfg(any(
                feature = "gpio-l152xc",
                feature = "gpio-l15xxa",
                feature = "gpio-l162xd",
                feature = "gpio-l162xe"
            ))]
            PA0<0>,
        ],

        <Tamp3, PushPull> for [
            #[cfg(any(
                feature = "gpio-l152xc",
                feature = "gpio-l15xxa",
                feature = "gpio-l162xd",
                feature = "gpio-l162xe"
            ))]
            PE6<0>,
        ],
    }
}

#[cfg(feature = "gpio-l162xd")]
pub mod sdio {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC12<12>,
        ],

        <Cmd, PushPull> for [
            PD2<12>,
        ],

        <D0, PushPull> for [
            PC8<12>,
        ],

        <D1, PushPull> for [
            PC9<12>,
        ],

        <D2, PushPull> for [
            PC10<12>,
        ],

        <D3, PushPull> for [
            PC11<12>,
        ],

        <D4, PushPull> for [
            PB8<12>,
        ],

        <D5, PushPull> for [
            PB9<12>,
        ],

        <D6, PushPull> for [
            PC6<12>,
        ],

        <D7, PushPull> for [
            PC7<12>,
        ],
    }
}

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<5>,

            PA11<5>,

            PB4<5>,

            PE14<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PA12<5>,

            PB5<5>,

            PE15<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,

            PE12<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<5>,

            PB3<5>,

            PE13<5>,
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
            PB14<5>,

            PD3<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<5>,

            PD4<5>,
        ],

        <Nss, PushPull> for [
            PB12<5>,

            PD0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB13<5>,

            PD1<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI2 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(any(
    feature = "gpio-l152xc",
    feature = "gpio-l15xxa",
    feature = "gpio-l162xd",
    feature = "gpio-l162xe"
))]
pub mod spi3 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB4<6>,

            PC11<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB5<6>,

            PC12<6>,
        ],

        <Nss, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB3<6>,

            PC10<6>,
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
        <JtckSwclk, PushPull> for [
            PA14<0>,
        ],

        <Jtdi, PushPull> for [
            PA15<0>,
        ],

        <JtdoTraceswo, PushPull> for [
            PB3<0>,
        ],

        <JtmsSwdio, PushPull> for [
            PA13<0>,
        ],

        <Jtrst, PushPull> for [
            PB4<0>,
        ],

        <PvdIn, PushPull> for [
            #[cfg(feature = "gpio-l162xe")]
            PB7<14>,
        ],

        <Traceck, PushPull> for [
            PE2<0>,
        ],

        <Traced0, PushPull> for [
            PE3<0>,
        ],

        <Traced1, PushPull> for [
            PE4<0>,
        ],

        <Traced2, PushPull> for [
            PE5<0>,
        ],

        <Traced3, PushPull> for [
            PE6<0>,
        ],

        <VRefOut, PushPull> for [
            #[cfg(feature = "gpio-l162xe")]
            PB0<14>,

            #[cfg(feature = "gpio-l162xe")]
            PB1<14>,
        ],

        <Wkup1, PushPull> for [
            PA0<0>,
        ],

        <Wkup3, PushPull> for [
            PE6<0>,
        ],
    }
}

pub mod tim2 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<1>,

            PA5<1>,

            PA15<1>,

            PE9<1>,
        ],

        <Ch2> default:PushPull for [
            PA1<1>,

            PB3<1>,

            PE10<1>,
        ],

        <Ch3> default:PushPull for [
            PA2<1>,

            PB10<1>,

            PE11<1>,
        ],

        <Ch4> default:PushPull for [
            PA3<1>,

            PB11<1>,

            PE12<1>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            #[cfg(feature = "gpio-l152x8")]
            PA0<1>,
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
            PA6<2>,

            PB4<2>,

            PC6<2>,

            PE3<2>,
        ],

        <Ch2> default:PushPull for [
            PA7<2>,

            PB5<2>,

            PC7<2>,

            PE4<2>,
        ],

        <Ch3> default:PushPull for [
            PB0<2>,

            PC8<2>,
        ],

        <Ch4> default:PushPull for [
            PB1<2>,

            PC9<2>,
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

pub mod tim4 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB6<2>,

            PD12<2>,
        ],

        <Ch2> default:PushPull for [
            PB7<2>,

            PD13<2>,
        ],

        <Ch3> default:PushPull for [
            PB8<2>,

            PD14<2>,
        ],

        <Ch4> default:PushPull for [
            PB9<2>,

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

#[cfg(any(
    feature = "gpio-l152xc",
    feature = "gpio-l15xxa",
    feature = "gpio-l162xd",
    feature = "gpio-l162xe"
))]
pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF6<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF7<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF8<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF9<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            #[cfg(any(feature = "gpio-l152xc", feature = "gpio-l15xxa"))]
            PE9<2>,

            #[cfg(feature = "gpio-l162xd")]
            PF6<2>,
        ],
    }

    use crate::pac::TIM5 as TIM;
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

pub mod tim9 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<3>,

            PB13<3>,

            PD0<3>,

            PE5<3>,
        ],

        <Ch2> default:PushPull for [
            PA3<3>,

            PB14<3>,

            PD7<3>,

            PE6<3>,
        ],
    }

    use crate::pac::TIM9 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
}

pub mod tim10 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<3>,

            PB8<3>,

            PB12<3>,

            PE0<3>,
        ],
    }

    use crate::pac::TIM10 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim11 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<3>,

            PB9<3>,

            PB15<3>,

            PE1<3>,
        ],
    }

    use crate::pac::TIM11 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

// TODO check
pub mod timx {
    use super::*;

    pin! {
        <Ic1, PushPull> for [
            PA0<14>,

            PA4<14>,

            PA8<14>,

            PA12<14>,

            PC0<14>,

            PC4<14>,

            PC8<14>,

            PC12<14>,

            PD0<14>,

            PD4<14>,

            PD8<14>,

            PD12<14>,

            PE0<14>,

            PE4<14>,

            PE8<14>,

            PE12<14>,
        ],

        <Ic2, PushPull> for [
            PA1<14>,

            PA5<14>,

            PA9<14>,

            PA13<14>,

            PC1<14>,

            PC5<14>,

            PC9<14>,

            PD1<14>,

            PD5<14>,

            PD9<14>,

            PD13<14>,

            PE1<14>,

            PE5<14>,

            PE9<14>,

            PE13<14>,
        ],

        <Ic3, PushPull> for [
            PA2<14>,

            PA6<14>,

            PA10<14>,

            PA14<14>,

            PC2<14>,

            PC6<14>,

            PC10<14>,

            PC14<14>,

            PD2<14>,

            PD6<14>,

            PD10<14>,

            PD14<14>,

            PE2<14>,

            PE6<14>,

            PE10<14>,

            PE14<14>,
        ],

        <Ic4, PushPull> for [
            PA3<14>,

            PA7<14>,

            PA11<14>,

            PA15<14>,

            PC3<14>,

            PC7<14>,

            PC11<14>,

            PC15<14>,

            PD3<14>,

            PD7<14>,

            PD11<14>,

            PD15<14>,

            PE3<14>,

            PE7<14>,

            PE11<14>,

            PE15<14>,
        ],
    }

    use crate::pac::TIMX as TIM;
}

pub mod ts {
    use super::*;

    analog! {
        <G10Io1> for [
            PC6<14>,
        ],

        <G10Io2> for [
            PC7<14>,
        ],

        <G10Io3> for [
            PC8<14>,
        ],

        <G10Io4> for [
            PC9<14>,
        ],

        <G11Io1> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF6<14>,
        ],

        <G11Io2> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF7<14>,
        ],

        <G11Io3> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF8<14>,
        ],

        <G11Io4> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF9<14>,
        ],

        <G11Io5> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF10<14>,
        ],

        <G1Io1> for [
            PA0<14>,
        ],

        <G1Io2> for [
            PA1<14>,
        ],

        <G1Io3> for [
            PA2<14>,
        ],

        <G1Io4> for [
            PA3<14>,
        ],

        <G2Io1> for [
            PA6<14>,
        ],

        <G2Io2> for [
            PA7<14>,
        ],

        <G2Io3> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF15<14>,
        ],

        <G2Io4> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PG0<14>,
        ],

        <G2Io5> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PG1<14>,
        ],

        <G3Io1> for [
            PB0<14>,
        ],

        <G3Io2> for [
            PB1<14>,
        ],

        <G3Io3> for [
            #[cfg(any(
                feature = "gpio-l152xc",
                feature = "gpio-l15xxa",
                feature = "gpio-l162xd",
                feature = "gpio-l162xe"
            ))]
            PB2<14>,
        ],

        <G3Io4> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF11<14>,
        ],

        <G3Io5> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF12<14>,
        ],

        <G4Io1> for [
            PA8<14>,
        ],

        <G4Io2> for [
            PA9<14>,
        ],

        <G4Io3> for [
            PA10<14>,
        ],

        <G5Io1> for [
            PA13<14>,
        ],

        <G5Io2> for [
            PA14<14>,
        ],

        <G5Io3> for [
            PA15<14>,
        ],

        <G6Io1> for [
            PB4<14>,
        ],

        <G6Io2> for [
            PB5<14>,
        ],

        <G6Io3> for [
            #[cfg(any(
                feature = "gpio-l152xc",
                feature = "gpio-l15xxa",
                feature = "gpio-l162xd",
                feature = "gpio-l162xe"
            ))]
            PB6<14>,
        ],

        <G6Io4> for [
            #[cfg(any(
                feature = "gpio-l152xc",
                feature = "gpio-l15xxa",
                feature = "gpio-l162xd",
                feature = "gpio-l162xe"
            ))]
            PB7<14>,
        ],

        <G7Io1> for [
            PB12<14>,
        ],

        <G7Io2> for [
            PB13<14>,
        ],

        <G7Io3> for [
            PB14<14>,
        ],

        <G7Io4> for [
            PB15<14>,
        ],

        <G7Io5> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PG2<14>,
        ],

        <G7Io6> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PG3<14>,
        ],

        <G7Io7> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PG4<14>,
        ],

        <G8Io1> for [
            PC0<14>,
        ],

        <G8Io2> for [
            PC1<14>,
        ],

        <G8Io3> for [
            PC2<14>,
        ],

        <G8Io4> for [
            PC3<14>,
        ],

        <G9Io1> for [
            PC4<14>,
        ],

        <G9Io2> for [
            PC5<14>,
        ],

        <G9Io3> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF13<14>,
        ],

        <G9Io4> for [
            #[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
            PF14<14>,
        ],
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<7>,
        ],

        <Cts, PushPull> for [
            PA11<7>,
        ],

        <Rts, PushPull> for [
            PA12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA10<7>,

            PB7<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<7>,

            PB6<7>,
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
            PA4<7>,

            PD7<7>,
        ],

        <Cts, PushPull> for [
            PA0<7>,

            PD3<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,

            PD4<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<7>,

            PD6<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<7>,

            PD5<7>,
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

pub mod usart3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB12<7>,

            PC12<7>,

            PD10<7>,
        ],

        <Cts, PushPull> for [
            PB13<7>,

            PD11<7>,
        ],

        <Rts, PushPull> for [
            PB14<7>,

            PD12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB11<7>,

            PC11<7>,

            PD9<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB10<7>,

            PC10<7>,

            PD8<7>,
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

#[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
pub mod uart4 {
    use super::*;

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC11<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC10<8>,
        ],
    }
    use crate::pac::UART4 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(any(feature = "gpio-l162xd", feature = "gpio-l162xe"))]
pub mod uart5 {
    use super::*;

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PD2<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC12<8>,
        ],
    }
    use crate::pac::UART5 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

pub mod usb {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PA11<10>,
        ],

        <Dp, PushPull> for [
            PA12<10>,
        ],
    }
}

#[cfg(feature = "gpio-l162xe")]
pub mod v_ref {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB0<0>,

            PB1<0>,
        ],
    }

    analog! {
        <PvdIn> for [
            PB7<0>,
        ],
    }
}
