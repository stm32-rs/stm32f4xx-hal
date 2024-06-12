use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod can1 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PA11<9>,

            PB8<9>,

            PD0<9>,

            PI9<9>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PA12<9>,

            PB9<9>,

            PD1<9>,

            PH13<9>,
        ],
    }
    impl CanCommon for crate::pac::CAN1 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

pub mod can2 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PB5<9>,

            PB12<9>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PB6<9>,

            PB13<9>,
        ],
    }
    impl CanCommon for crate::pac::CAN2 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

pub mod dcmi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<13>,

            PC6<13>,

            PH9<13>,
        ],

        <D1, PushPull> for [
            PA10<13>,

            PC7<13>,

            PH10<13>,
        ],

        <D10, PushPull> for [
            PB5<13>,

            PI3<13>,
        ],

        <D11, PushPull> for [
            PD2<13>,

            PH15<13>,
        ],

        <D12, PushPull> for [
            PF11<13>,
        ],

        <D13, PushPull> for [
            PG15<13>,

            PI0<13>,
        ],

        <D2, PushPull> for [
            PC8<13>,

            PE0<13>,

            PH11<13>,
        ],

        <D3, PushPull> for [
            PC9<13>,

            PE1<13>,

            PH12<13>,
        ],

        <D4, PushPull> for [
            PC11<13>,

            PE4<13>,

            PH14<13>,
        ],

        <D5, PushPull> for [
            PB6<13>,

            PI4<13>,
        ],

        <D6, PushPull> for [
            PB8<13>,

            PE5<13>,

            PI6<13>,
        ],

        <D7, PushPull> for [
            PB9<13>,

            PE6<13>,

            PI7<13>,
        ],

        <D8, PushPull> for [
            PC10<13>,

            PI1<13>,
        ],

        <D9, PushPull> for [
            PC12<13>,

            PI2<13>,
        ],

        <Hsync, PushPull> for [
            PA4<13>,

            PH8<13>,
        ],

        <Pixclk, PushPull> for [
            PA6<13>,
        ],

        <Vsync, PushPull> for [
            PB7<13>,

            PI5<13>,
        ],
    }
}

pub mod eth {
    use super::*;

    pin! {
        <Col, PushPull> for [
            PA3<11>,

            PH3<11>,
        ],

        <Crs, PushPull> for [
            PA0<11>,

            PH2<11>,
        ],

        <CrsDv, PushPull> for [
            PA7<11>,
        ],

        <Mdc, PushPull> for [
            PC1<11>,
        ],

        <Mdio, PushPull> for [
            PA2<11>,
        ],

        <PpsOut, PushPull> for [
            PB5<11>,

            PG8<11>,
        ],

        <RefClk, PushPull> for [
            PA1<11>,
        ],

        <RxClk, PushPull> for [
            PA1<11>,
        ],

        <RxDv, PushPull> for [
            PA7<11>,
        ],

        <RxEr, PushPull> for [
            PB10<11>,

            PI10<11>,
        ],

        <Rxd0, PushPull> for [
            PC4<11>,
        ],

        <Rxd1, PushPull> for [
            PC5<11>,
        ],

        <Rxd2, PushPull> for [
            PB0<11>,

            PH6<11>,
        ],

        <Rxd3, PushPull> for [
            PB1<11>,

            PH7<11>,
        ],

        <TxClk, PushPull> for [
            PC3<11>,
        ],

        <TxEn, PushPull> for [
            PB11<11>,

            PG11<11>,
        ],

        <Txd0, PushPull> for [
            PB12<11>,

            PG13<11>,
        ],

        <Txd1, PushPull> for [
            PB13<11>,

            PG14<11>,
        ],

        <Txd2, PushPull> for [
            PC2<11>,
        ],

        <Txd3, PushPull> for [
            PB8<11>,

            PE2<11>,
        ],
    }
}

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
            PE6<12>,
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

        <Ale, PushPull> for [
            PD12<12>,
        ],

        <Cd, PushPull> for [
            PF9<12>,
        ],

        <Cle, PushPull> for [
            PD11<12>,
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

        <Int2, PushPull> for [
            PG6<12>,
        ],

        <Int3, PushPull> for [
            PG7<12>,
        ],

        <Intr, PushPull> for [
            PF10<12>,
        ],

        <Nbl0, PushPull> for [
            PE0<12>,
        ],

        <Nbl1, PushPull> for [
            PE1<12>,
        ],

        <Nce2, PushPull> for [
            PD7<12>,
        ],

        <Nce3, PushPull> for [
            PG9<12>,
        ],

        <Nce41, PushPull> for [
            PG10<12>,
        ],

        <Nce42, PushPull> for [
            PG11<12>,
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

        <Niord, PushPull> for [
            PF6<12>,
        ],

        <Niowr, PushPull> for [
            PF8<12>,
        ],

        <Nl, PushPull> for [
            PB7<12>,
        ],

        <Noe, PushPull> for [
            PD4<12>,
        ],

        <Nreg, PushPull> for [
            PF7<12>,
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

            PF1<4>,

            PH4<4>,
        ],

        <Sda, OpenDrain> for [
            PB11<4>,

            PF0<4>,

            PH5<4>,
        ],

        <Smba, OpenDrain> for [
            PB12<4>,

            PF2<4>,

            PH6<4>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<4>,

            PH7<4>,
        ],

        <Sda, OpenDrain> for [
            PC9<4>,

            PH8<4>,
        ],

        <Smba, OpenDrain> for [
            PA9<4>,

            PH9<4>,
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
            PC9<5>,
        ],
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        <Mck, PushPull> for no:NoPin, [
            PC6<5>,
        ],

        <Sck, PushPull> for [
            PB10<5>,

            PB13<5>,

            PI1<5>,
        ],

        <Sd, PushPull> for [
            PB15<5>,

            PC3<5>,

            PI3<5>,
        ],

        <Ws, PushPull> for [
            PB9<5>,

            PB12<5>,

            PI0<5>,
        ],
    }
}

pub mod i2s3 {
    use super::*;

    pin! {
        <Mck, PushPull> for no:NoPin, [
            PC7<6>,
        ],

        <Sck, PushPull> for [
            PB3<6>,

            PC10<6>,
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

pub mod rcc {
    use super::*;

    pin! {
        <Mco1, PushPull> for [
            PA8<0>,
        ],

        <Mco2, PushPull> for [
            PC9<0>,
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
        <Af1, PushPull> for [
            PC13<0>,
        ],

        <Refin, PushPull> for [
            PB15<0>,
        ],
    }
}

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

            PB4<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PB5<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<5>,

            PB3<5>,
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

            PC2<5>,

            PI2<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<5>,

            PC3<5>,

            PI3<5>,
        ],

        <Nss, PushPull> for [
            PB9<5>,

            PB12<5>,

            PI0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB10<5>,

            PB13<5>,

            PI1<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI2 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

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

        <JtdoSwo, PushPull> for [
            PB3<0>,
        ],

        <JtmsSwdio, PushPull> for [
            PA13<0>,
        ],

        <Jtrst, PushPull> for [
            PB4<0>,
        ],

        <Traceclk, PushPull> for [
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

        <Wkup, PushPull> for [
            PA0<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<1>,

            PE9<1>,
        ],

        <Ch1N> default:PushPull for [
            PA7<1>,

            PB13<1>,

            PE8<1>,
        ],

        <Ch2> default:PushPull for [
            PA9<1>,

            PE11<1>,
        ],

        <Ch2N> default:PushPull for [
            PB0<1>,

            PB14<1>,

            PE10<1>,
        ],

        <Ch3> default:PushPull for [
            PA10<1>,

            PE13<1>,
        ],

        <Ch3N> default:PushPull for [
            PB1<1>,

            PB15<1>,

            PE12<1>,
        ],

        <Ch4> default:PushPull for [
            PA11<1>,

            PE14<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<1>,

            PB12<1>,

            PE15<1>,
        ],

        <Etr, PushPull> for [
            PA12<1>,

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

pub mod tim2 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<1>,

            PA5<1>,

            PA15<1>,
        ],

        <Ch2> default:PushPull for [
            PA1<1>,

            PB3<1>,
        ],

        <Ch3> default:PushPull for [
            PA2<1>,

            PB10<1>,
        ],

        <Ch4> default:PushPull for [
            PA3<1>,

            PB11<1>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<1>,

            PA5<1>,

            PA15<1>,
        ],
    }

    use crate::pac::TIM2 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
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
        ],

        <Ch2> default:PushPull for [
            PA7<2>,

            PB5<2>,

            PC7<2>,
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
        ],
    }

    use crate::pac::TIM3 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
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
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PH10<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PH11<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PH12<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PI0<2>,
        ],
    }

    use crate::pac::TIM5 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
}

pub mod tim8 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PC6<3>,

            PI5<3>,
        ],

        <Ch1N> default:PushPull for [
            PA5<3>,

            PA7<3>,

            PH13<3>,
        ],

        <Ch2> default:PushPull for [
            PC7<3>,

            PI6<3>,
        ],

        <Ch2N> default:PushPull for [
            PB0<3>,

            PB14<3>,

            PH14<3>,
        ],

        <Ch3> default:PushPull for [
            PC8<3>,

            PI7<3>,
        ],

        <Ch3N> default:PushPull for [
            PB1<3>,

            PB15<3>,

            PH15<3>,
        ],

        <Ch4> default:PushPull for [
            PC9<3>,

            PI2<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<3>,

            PI4<3>,
        ],

        <Etr, PushPull> for [
            PA0<3>,

            PI3<3>,
        ],
    }

    use crate::pac::TIM8 as TIM;
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

pub mod tim9 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<3>,

            PE5<3>,
        ],

        <Ch2> default:PushPull for [
            PA3<3>,

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
            PB8<3>,

            PF6<3>,
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
            PB9<3>,

            PF7<3>,
        ],
    }

    use crate::pac::TIM11 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim12 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB14<9>,

            PH6<9>,
        ],

        <Ch2> default:PushPull for [
            PB15<9>,

            PH9<9>,
        ],
    }

    use crate::pac::TIM12 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
}

pub mod tim13 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<9>,

            PF8<9>,
        ],
    }

    use crate::pac::TIM13 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim14 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<9>,

            PF9<9>,
        ],
    }

    use crate::pac::TIM14 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod uart4 {
    use super::*;

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA1<8>,

            PC11<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<8>,

            PC10<8>,
        ],
    }
    use crate::pac::UART4 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

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

pub mod usart6 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC8<8>,

            PG7<8>,
        ],

        <Cts, PushPull> for [
            PG13<8>,

            PG15<8>,
        ],

        <Rts, PushPull> for [
            PG8<8>,

            PG12<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC7<8>,

            PG9<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC6<8>,

            PG14<8>,
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

pub mod otg_fs {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PA11<10>,
        ],

        <Dp, PushPull> for [
            PA12<10>,
        ],

        <Id, PushPull> for [
            PA10<10>,
        ],

        <Sof, PushPull> for [
            PA8<10>,
        ],
    }
}

pub mod otg_hs {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PB14<12>,
        ],

        <Dp, PushPull> for [
            PB15<12>,
        ],

        <Id, PushPull> for [
            PB12<12>,
        ],

        <Sof, PushPull> for [
            PA4<12>,
        ],

        <UlpiCk, PushPull> for [
            PA5<10>,
        ],

        <UlpiD0, PushPull> for [
            PA3<10>,
        ],

        <UlpiD1, PushPull> for [
            PB0<10>,
        ],

        <UlpiD2, PushPull> for [
            PB1<10>,
        ],

        <UlpiD3, PushPull> for [
            PB10<10>,
        ],

        <UlpiD4, PushPull> for [
            PB11<10>,
        ],

        <UlpiD5, PushPull> for [
            PB12<10>,
        ],

        <UlpiD6, PushPull> for [
            PB13<10>,
        ],

        <UlpiD7, PushPull> for [
            PB5<10>,
        ],

        <UlpiDir, PushPull> for [
            PC2<10>,

            PI11<10>,
        ],

        <UlpiNxt, PushPull> for [
            PC3<10>,

            PH4<10>,
        ],

        <UlpiStp, PushPull> for [
            PC0<10>,
        ],

        <Vbus, PushPull> for [
            PB13<12>,
        ],
    }
}
