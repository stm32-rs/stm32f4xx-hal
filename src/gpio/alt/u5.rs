use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

/*pub mod adf1 {
    use super::*;

    pin! {
        <Cck0> for [
            PB3<3>,

            PE9<3>,

            #[cfg(feature = "gpio-u59x")]
            PF3<3>,
        ],

        <Cck1> for [
            PC10<3>,
        ],

        <Sdi0> for [
            PB4<3>,

            PC11<3>,

            PE10<3>,

            #[cfg(feature = "gpio-u59x")]
            PF4<3>,
        ],
    }
}*/

pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB0<12>,

            PB10<12>,
        ],
    }
}

pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB5<12>,

            PB11<12>,
        ],
    }
}

/*pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            PA10<0>,

            PB3<10>,
        ],
    }
}*/

pub mod dcmi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<5>,

            PC6<10>,

            PH9<10>,
        ],

        <D1, PushPull> for [
            PA10<5>,

            PC7<10>,

            PH10<10>,
        ],

        <D10, PushPull> for [
            PB5<10>,

            PD6<4>,

            PI3<10>,
        ],

        <D11, PushPull> for [
            PD2<10>,

            PF10<10>,

            PH15<10>,
        ],

        <D12, PushPull> for [
            PB4<10>,

            PF6<4>,

            PF11<10>,
        ],

        <D13, PushPull> for [
            PG15<10>,

            PI0<10>,
        ],

        <D2, PushPull> for [
            PC8<10>,

            PC11<4>,

            PE0<10>,

            PH11<10>,
        ],

        <D3, PushPull> for [
            PC9<4>,

            PE1<10>,

            PH12<10>,
        ],

        <D4, PushPull> for [
            PC11<10>,

            PE4<10>,

            PH14<10>,
        ],

        <D5, PushPull> for [
            PB6<10>,

            PD3<4>,

            PI4<10>,
        ],

        <D6, PushPull> for [
            PB8<10>,

            PE5<10>,

            PI6<10>,
        ],

        <D7, PushPull> for [
            PB9<10>,

            PE6<10>,

            PI7<10>,
        ],

        <D8, PushPull> for [
            PC10<10>,

            PH6<10>,

            PI1<10>,
        ],

        <D9, PushPull> for [
            PC12<10>,

            PH7<10>,

            PI2<10>,
        ],

        <Hsync, PushPull> for [
            PA4<10>,

            PD8<10>,

            PH8<10>,
        ],

        <Pixclk, PushPull> for [
            PA6<4>,

            PD9<10>,

            PH5<10>,
        ],

        <Vsync, PushPull> for [
            PB7<10>,

            PI5<10>,
        ],
    }
}

pub mod debug {
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
            PA8<12>,

            PE2<0>,
        ],

        <Traced0, PushPull> for [
            PC1<0>,

            PC9<0>,

            PE3<0>,
        ],

        <Traced1, PushPull> for [
            PC10<0>,

            PE4<0>,
        ],

        <Traced2, PushPull> for [
            PD2<0>,

            PE5<0>,
        ],

        <Traced3, PushPull> for [
            PC12<0>,

            PE6<0>,
        ],
    }
}

#[cfg(feature = "gpio-u59x")]
pub mod dsihost {
    use super::*;

    pin! {
        <Te, PushPull> for [
            PF10<11>,

            PF11<11>,

            PG5<11>,
        ],
    }
}

pub mod fdcan1 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PA11<9>,

            PB8<9>,

            PD0<9>,

            PF7<9>,

            PH14<9>,
        ],

        <Tx, PushPull> for [
            PA12<9>,

            PB9<9>,

            PD1<9>,

            PF8<9>,

            PH13<9>,
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

        <Int, PushPull> for [
            PG7<12>,
        ],

        <Nbl0, PushPull> for [
            PE0<12>,
        ],

        <Nbl1, PushPull> for [
            PB15<11>,

            PE1<12>,
        ],

        <Nce, PushPull> for [
            PD7<12>,

            PG9<12>,
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

#[cfg(feature = "gpio-u59x")]
pub mod hspi1 {
    use super::*;

    pin! {
        <Clk, PushPull> for [
            PI3<8>,
        ],

        <Dqs0, PushPull> for [
            PI2<8>,
        ],

        <Dqs1, PushPull> for [
            PI8<8>,
        ],

        <Io0, PushPull> for [
            PH10<8>,
        ],

        <Io1, PushPull> for [
            PH11<8>,
        ],

        <Io10, PushPull> for [
            PI11<8>,
        ],

        <Io11, PushPull> for [
            PI12<8>,
        ],

        <Io12, PushPull> for [
            PI13<8>,
        ],

        <Io13, PushPull> for [
            PI14<8>,
        ],

        <Io14, PushPull> for [
            PI15<8>,
        ],

        <Io15, PushPull> for [
            PJ0<8>,
        ],

        <Io2, PushPull> for [
            PH12<8>,
        ],

        <Io3, PushPull> for [
            PH13<8>,
        ],

        <Io4, PushPull> for [
            PH14<8>,
        ],

        <Io5, PushPull> for [
            PH15<8>,
        ],

        <Io6, PushPull> for [
            PI0<8>,
        ],

        <Io7, PushPull> for [
            PI1<8>,
        ],

        <Io8, PushPull> for [
            PI9<8>,
        ],

        <Io9, PushPull> for [
            PI10<8>,
        ],

        <Nclk, PushPull> for [
            PI4<8>,
        ],

        <Ncs, PushPull> for [
            PH9<8>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB6<4>,

            PB8<4>,

            PG14<4>,
        ],

        <Sda, OpenDrain> for [
            PB3<4>,

            PB7<4>,

            PB9<4>,

            PG13<4>,
        ],

        <Smba, OpenDrain> for [
            PA1<4>,

            PA14<4>,

            PB5<4>,

            PG15<4>,
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

            PB13<4>,

            PF1<4>,

            PH4<4>,
        ],

        <Sda, OpenDrain> for [
            PB11<4>,

            PB14<4>,

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
            PA7<4>,

            PC0<4>,

            PG7<4>,

            PH7<4>,
        ],

        <Sda, OpenDrain> for [
            PB4<4>,

            PC1<4>,

            PG8<4>,

            PH8<4>,
        ],

        <Smba, OpenDrain> for [
            PB2<4>,

            PG6<4>,

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

pub mod i2c4 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB6<5>,

            PB10<3>,

            PD12<4>,

            PF14<4>,
        ],

        <Sda, OpenDrain> for [
            PB7<5>,

            PB11<3>,

            PD13<4>,

            PF15<4>,
        ],

        <Smba, OpenDrain> for [
            PA14<5>,

            PD11<4>,

            PF13<4>,
        ],
    }
    use crate::pac::I2C4 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(feature = "gpio-u59x")]
pub mod i2c5 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PD1<4>,

            PH5<2>,

            PJ2<4>,
        ],

        <Sda, OpenDrain> for [
            PD0<4>,

            PH4<2>,

            PJ1<4>,
        ],

        <Smba, OpenDrain> for [
            PD2<4>,

            PD10<4>,

            PH6<2>,

            PJ0<4>,
        ],
    }
    use crate::pac::I2C5 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(feature = "gpio-u59x")]
pub mod i2c6 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PD1<2>,

            PF1<2>,

            PJ10<2>,
        ],

        <Sda, OpenDrain> for [
            PD0<2>,

            PF0<2>,

            PJ9<2>,
        ],

        <Smba, OpenDrain> for [
            PB12<2>,

            PC4<2>,

            PD3<2>,

            PJ8<2>,
        ],
    }
    use crate::pac::I2C6 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<1>,

            PB9<1>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Ch1, PushPull> for [
            PA14<1>,

            PB2<1>,

            PB3<2>,

            PC1<1>,

            PG15<1>,
        ],

        <Ch2, PushPull> for [
            PA1<0>,

            PB4<1>,

            PG14<1>,
        ],

        <Etr, PushPull> for [
            PB6<1>,

            PC3<1>,

            PG12<1>,
        ],

        <In1, PushPull> for [
            PB5<1>,

            PC0<1>,

            PG10<1>,
        ],

        <In2, PushPull> for [
            PB7<1>,

            PC2<1>,

            PG11<1>,
        ],
    }
}

pub mod lptim2 {
    use super::*;

    pin! {
        <Ch1, PushPull> for [
            PA4<14>,

            PA8<14>,

            PD13<14>,
        ],

        <Ch2, PushPull> for [
            PA7<13>,

            PC7<14>,

            PD10<2>,
        ],

        <Etr, PushPull> for [
            PA5<14>,

            PC3<14>,

            PD11<14>,
        ],

        <In1, PushPull> for [
            PB1<14>,

            PC0<14>,

            PD12<14>,
        ],

        <In2, PushPull> for [
            PA10<2>,

            PB15<2>,

            PD9<2>,
        ],
    }
}

pub mod lptim3 {
    use super::*;

    pin! {
        <Ch1, PushPull> for [
            PB0<4>,

            PB10<2>,

            PC3<2>,

            PC8<14>,

            PD14<14>,

            PF5<2>,
        ],

        <Ch2, PushPull> for [
            PB1<4>,

            PC9<14>,

            PD15<14>,

            PF2<2>,
        ],

        <Etr, PushPull> for [
            PB14<2>,

            PC10<2>,

            PD10<14>,

            PF4<2>,
        ],

        <In1, PushPull> for [
            PB13<2>,

            PC11<2>,

            PD9<14>,

            PF3<2>,
        ],
    }
}

pub mod lptim4 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PD2<13>,

            PF12<13>,
        ],

        <In1, PushPull> for [
            PD13<13>,

            PF11<13>,
        ],

        <Out> default:PushPull for [
            PD7<13>,

            PF13<13>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA6<8>,

            PB13<8>,

            PG5<8>,
        ],

        <De, PushPull> for [
            PB1<8>,

            PB12<8>,

            PG6<8>,
        ],

        <Rts, PushPull> for [
            PB1<8>,

            PB12<8>,

            PG6<8>,
        ],

        <Rx, PushPull> for [
            PA3<8>,

            PB10<8>,

            PC0<8>,

            PG8<8>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            PA2<8>,

            PB11<8>,

            PC1<8>,

            PG7<8>,
        ],
    }
}

#[cfg(feature = "gpio-u59x")]
pub mod ltdc {
    use super::*;

    pin! {
        <B0, PushPull> for [
            PE4<8>,

            PF12<8>,
        ],

        <B1, PushPull> for [
            PB2<8>,

            PB8<7>,

            PF13<8>,
        ],

        <B2, PushPull> for [
            PD14<8>,
        ],

        <B3, PushPull> for [
            PD15<8>,
        ],

        <B4, PushPull> for [
            PD0<8>,
        ],

        <B5, PushPull> for [
            PD1<8>,
        ],

        <B6, PushPull> for [
            PE7<8>,
        ],

        <B7, PushPull> for [
            PE8<8>,
        ],

        <Clk, PushPull> for [
            PD3<8>,
        ],

        <De, PushPull> for [
            PD6<8>,

            PF11<8>,
        ],

        <G0, PushPull> for [
            PE5<8>,

            PF14<8>,
        ],

        <G1, PushPull> for [
            PE6<8>,

            PF15<8>,
        ],

        <G2, PushPull> for [
            PE9<8>,
        ],

        <G3, PushPull> for [
            PE10<8>,
        ],

        <G4, PushPull> for [
            PE11<8>,
        ],

        <G5, PushPull> for [
            PE12<8>,
        ],

        <G6, PushPull> for [
            PE13<8>,
        ],

        <G7, PushPull> for [
            PE14<8>,
        ],

        <Hsync, PushPull> for [
            PE0<8>,
        ],

        <R0, PushPull> for [
            PC6<7>,

            PE2<8>,

            PG13<8>,
        ],

        <R1, PushPull> for [
            PC7<7>,

            PE3<8>,

            PG6<7>,

            PG14<8>,
        ],

        <R2, PushPull> for [
            PE15<8>,
        ],

        <R3, PushPull> for [
            PD8<8>,
        ],

        <R4, PushPull> for [
            PD9<8>,
        ],

        <R5, PushPull> for [
            PD10<8>,
        ],

        <R6, PushPull> for [
            PD11<8>,
        ],

        <R7, PushPull> for [
            PD12<8>,
        ],

        <Vsync, PushPull> for [
            PD13<8>,

            PE1<8>,
        ],
    }
}

pub mod mdf1 {
    use super::*;

    pin! {
        <Cck0, PushPull> for [
            PB8<5>,

            PE9<6>,

            #[cfg(feature = "gpio-u59x")]
            PF3<6>,

            PG7<6>,
        ],

        <Cck1, PushPull> for [
            PC2<6>,

            PF10<6>,
        ],

        <Cki0, PushPull> for [
            PB2<6>,

            PD4<6>,

            #[cfg(feature = "gpio-u59x")]
            PF5<6>,
        ],

        <Cki1, PushPull> for [
            PB13<6>,

            PD7<6>,
        ],

        <Cki2, PushPull> for [
            PB15<6>,

            PE8<6>,
        ],

        <Cki3, PushPull> for [
            PC6<6>,

            PE5<6>,
        ],

        <Cki4, PushPull> for [
            PC1<6>,

            PE11<6>,
        ],

        <Cki5, PushPull> for [
            PB7<6>,

            PE13<6>,
        ],

        <Sdi0, PushPull> for [
            PB1<6>,

            PD3<6>,

            #[cfg(feature = "gpio-u59x")]
            PF4<6>,
        ],

        <Sdi1, PushPull> for [
            PB12<6>,

            PD6<6>,
        ],

        <Sdi2, PushPull> for [
            PB14<6>,

            PE7<6>,
        ],

        <Sdi3, PushPull> for [
            PC7<6>,

            PE4<6>,
        ],

        <Sdi4, PushPull> for [
            PC0<6>,

            PE10<6>,
        ],

        <Sdi5, PushPull> for [
            PB6<6>,

            PE12<6>,
        ],
    }
}

pub mod octospim {
    use super::*;

    pin! {
        <P1Clk, PushPull> for [ // High speed
            PA3<10>,

            PB10<10>,

            PE10<10>,

            PF10<3>,
        ],

        <P1Dqs, PushPull> for [
            PA1<10>,

            PB2<10>,

            PE3<3>,

            PG6<3>,
        ],

        <P1Io0, PushPull> for [
            PB1<10>,

            PE12<10>,

            PF8<10>,
        ],

        <P1Io1, PushPull> for [
            PB0<10>,

            PE13<10>,

            PF9<10>,
        ],

        <P1Io2, PushPull> for [
            PA7<10>,

            PE14<10>,

            PF7<10>,
        ],

        <P1Io3, PushPull> for [
            PA6<10>,

            PE15<10>,

            PF6<10>,
        ],

        <P1Io4, PushPull> for [
            PC1<10>,

            PD4<10>,

            PH2<3>,
        ],

        <P1Io5, PushPull> for [
            PC2<10>,

            PD5<10>,

            PG11<3>,

            PI0<3>,
        ],

        <P1Io6, PushPull> for [
            PC3<10>,

            PD6<10>,
        ],

        <P1Io7, PushPull> for [
            PC0<3>,

            PC4<10>,

            PD7<10>,
        ],

        <P1Nclk, PushPull> for [
            PB5<3>,

            PB12<10>,

            PE9<10>,

            PF11<3>,
        ],

        <P1Ncs, PushPull> for [
            PA2<10>,

            PA4<3>,

            PB11<10>,

            PC11<5>,

            PE11<10>,
        ],

        <P2Clk, PushPull> for [
            PF4<5>,

            PH6<5>,

            PI6<5>,
        ],

        <P2Dqs, PushPull> for [
            PF12<5>,

            PG7<5>,

            PG15<5>,

            PH4<5>,
        ],

        <P2Io0, PushPull> for [
            PF0<5>,

            PI3<6>,
        ],

        <P2Io1, PushPull> for [
            PF1<5>,

            PI2<6>,
        ],

        <P2Io2, PushPull> for [
            PF2<5>,

            PI1<6>,
        ],

        <P2Io3, PushPull> for [
            PF3<5>,

            PH8<5>,
        ],

        <P2Io4, PushPull> for [
            PG0<5>,

            PH9<5>,
        ],

        <P2Io5, PushPull> for [
            PG1<5>,

            PH10<5>,
        ],

        <P2Io6, PushPull> for [
            PG9<5>,

            PH11<5>,

            PH15<5>,
        ],

        <P2Io7, PushPull> for [
            PG10<5>,

            PH12<5>,
        ],

        <P2Nclk, PushPull> for [
            PF5<5>,

            PH7<5>,

            PI7<5>,
        ],

        <P2Ncs, PushPull> for [
            PA0<10>,

            PA12<6>,

            PD3<10>,

            PF6<5>,

            PG12<5>,

            PI5<5>,
        ],
    }
}

pub mod pssi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<5>,

            PC6<10>,

            PH9<10>,
        ],

        <D1, PushPull> for [
            PA10<5>,

            PC7<10>,

            PH10<10>,
        ],

        <D10, PushPull> for [
            PB5<10>,

            PD6<4>,

            PI3<10>,
        ],

        <D11, PushPull> for [
            PD2<10>,

            PF10<10>,

            PH15<10>,
        ],

        <D12, PushPull> for [
            PB4<10>,

            PF6<4>,

            PF11<10>,
        ],

        <D13, PushPull> for [
            PG15<10>,

            PI0<10>,
        ],

        <D14, PushPull> for [
            PA5<4>,

            PF8<4>,

            PH4<10>,
        ],

        <D15, PushPull> for [
            PC5<4>,

            PF9<4>,

            PF10<4>,
        ],

        <D2, PushPull> for [
            PC8<10>,

            PC11<4>,

            PE0<10>,

            PH11<10>,
        ],

        <D3, PushPull> for [
            PC9<4>,

            PE1<10>,

            PH12<10>,
        ],

        <D4, PushPull> for [
            PC11<10>,

            PE4<10>,

            PH14<10>,
        ],

        <D5, PushPull> for [
            PB6<10>,

            PD3<4>,

            PI4<10>,
        ],

        <D6, PushPull> for [
            PB8<10>,

            PE5<10>,

            PI6<10>,
        ],

        <D7, PushPull> for [
            PB9<10>,

            PE6<10>,

            PI7<10>,
        ],

        <D8, PushPull> for [
            PC10<10>,

            PH6<10>,

            PI1<10>,
        ],

        <D9, PushPull> for [
            PC12<10>,

            PH7<10>,

            PI2<10>,
        ],

        <De, PushPull> for [
            PA4<10>,

            PD8<10>,

            PH8<10>,
        ],

        <Pdck, PushPull> for [
            PA6<4>,

            PD9<10>,

            PH5<10>,
        ],

        <Rdy, PushPull> for [
            PB7<10>,

            PI5<10>,
        ],
    }
}

pub mod pwr {
    use super::*;

    pin! {
        <Cdstop, PushPull> for [
            PA6<0>,

            PC7<0>,
        ],

        <Csleep, PushPull> for [
            PA5<0>,

            PC6<0>,
        ],

        <Srdstop, PushPull> for [
            PA7<0>,

            PC8<0>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,
        ],
    }
}

pub mod rtc {
    use super::*;

    pin! {
        <Refin, PushPull> for [
            PB15<0>,
        ],
    }
}

pub mod sai1 {
    use super::*;

    pin! {
        <Ck1, PushPull> for [
            PA3<3>,

            PB8<3>,

            PE2<3>,

            PG7<3>,
        ],

        <Ck2, PushPull> for [
            PA8<3>,

            PE5<3>,
        ],

        <D1, PushPull> for [
            PA10<3>,

            PC3<3>,

            PD6<3>,

            PE6<3>,
        ],

        <D2, PushPull> for [
            PB9<3>,

            PE4<3>,
        ],

        <D3, PushPull> for [
            PC5<3>,

            PF10<13>,
        ],

        <FsA, PushPull> for [
            PA9<13>,

            PB9<13>,

            PE4<13>,
        ],

        <FsB, PushPull> for [
            PA4<13>,

            PA14<13>,

            PB6<13>,

            PE9<13>,

            PF9<13>,
        ],

        <MclkA, PushPull> for [
            PA3<13>,

            PB8<13>,

            PE2<13>,

            PG7<13>,
        ],

        <MclkB, PushPull> for [
            PB4<13>,

            PE10<13>,

            PF7<13>,
        ],

        <SckA, PushPull> for [
            PA8<13>,

            PB10<13>,

            PE5<13>,
        ],

        <SckB, PushPull> for [
            PB3<13>,

            PE8<13>,

            PF8<13>,
        ],

        <SdA, PushPull> for [
            PA10<13>,

            PC1<13>,

            PC3<13>,

            PD6<13>,

            PE6<13>,
        ],

        <SdB, PushPull> for [
            PA13<13>,

            PB5<13>,

            PE3<13>,

            PE7<13>,

            PF6<13>,
        ],
    }
    use crate::pac::SAI1 as SAI;
    pub struct ChannelA;
    pub struct ChannelB;
    impl SaiChannels for SAI {
        type A = ChannelA;
        type B = ChannelB;
    }
    impl SaiChannel for ChannelA {
        type Fs = FsA;
        type Mclk = MclkA;
        type Sck = SckA;
        type Sd = SdA;
    }
    impl SaiChannel for ChannelB {
        type Fs = FsB;
        type Mclk = MclkB;
        type Sck = SckB;
        type Sd = SdB;
    }
}

pub mod sai2 {
    use super::*;

    pin! {
        <FsA, PushPull> for [
            PB12<13>,

            PC0<13>,

            PD12<13>,

            PG10<13>,
        ],

        <FsB, PushPull> for [
            PA15<13>,

            PG3<13>,
        ],

        <MclkA, PushPull> for [
            PB14<13>,

            PC6<13>,

            PD9<13>,

            PG11<13>,
        ],

        <MclkB, PushPull> for [
            PC7<13>,

            PC11<13>,

            PG4<13>,
        ],

        <SckA, PushPull> for [
            PB13<13>,

            PD10<13>,

            PG9<13>,
        ],

        <SckB, PushPull> for [
            PC10<13>,

            PG2<13>,
        ],

        <SdA, PushPull> for [
            PB15<13>,

            PD11<13>,

            PG12<13>,
        ],

        <SdB, PushPull> for [
            PC12<13>,

            PG5<13>,
        ],
    }
    use crate::pac::SAI2 as SAI;
    pub struct ChannelA;
    pub struct ChannelB;
    impl SaiChannels for SAI {
        type A = ChannelA;
        type B = ChannelB;
    }
    impl SaiChannel for ChannelA {
        type Fs = FsA;
        type Mclk = MclkA;
        type Sck = SckA;
        type Sd = SdA;
    }
    impl SaiChannel for ChannelB {
        type Fs = FsB;
        type Mclk = MclkB;
        type Sck = SckB;
        type Sd = SdB;
    }
}

pub mod sdmmc1 {
    use super::*;

    pin! {
        <Cdir, PushPull> for [
            PB9<8>,
        ],

        <Ck, PushPull> for [
            PC12<12>,
        ],

        <Ckin, PushPull> for [
            PB8<8>,
        ],

        <Cmd, PushPull> for [
            PD2<12>,
        ],

        <D0, PushPull> for [
            PC8<12>,
        ],

        <D0Dir, PushPull> for [
            PC6<8>,
        ],

        <D1, PushPull> for [
            PC9<12>,
        ],

        <D123Dir, PushPull> for [
            PC7<8>,
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

            PC0<12>,
        ],

        <D6, PushPull> for [
            PC6<12>,
        ],

        <D7, PushPull> for [
            PC7<12>,
        ],
    }
}

pub mod sdmmc2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC1<12>,

            PD6<11>,
        ],

        <Cmd, PushPull> for [
            PA0<12>,

            PD7<11>,
        ],

        <D0, PushPull> for [
            PB14<12>,
        ],

        <D1, PushPull> for [
            PB15<12>,
        ],

        <D2, PushPull> for [
            PB3<12>,
        ],

        <D3, PushPull> for [
            PB4<12>,
        ],

        <D4, PushPull> for [
            PB8<11>,
        ],

        <D5, PushPull> for [
            PB9<11>,
        ],

        <D6, PushPull> for [
            PC6<11>,
        ],

        <D7, PushPull> for [
            PC7<11>,
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

            PG3<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PA12<5>,

            PB5<5>,

            PE15<5>,

            PG4<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,

            PB0<5>,

            PE12<5>,

            PG5<5>,
        ],

        <Rdy, PushPull> for [
            PA2<5>,

            PA8<5>,

            PB2<5>,

            PE11<5>,

            PG6<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA1<5>,

            PA5<5>,

            PB3<5>,

            PE13<5>,

            PG2<5>,
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

            PD3<5>,

            PI2<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<5>,

            PC1<3>,

            PC3<5>,

            PD4<5>,

            PI3<5>,
        ],

        <Nss, PushPull> for [
            PB9<5>,

            PB12<5>,

            PD0<5>,

            PI0<5>,
        ],

        <Rdy, PushPull> for [
            PB11<5>,

            PC0<5>,

            PD5<5>,

            PI4<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA9<3>,

            PB10<5>,

            PB13<5>,

            PD1<5>,

            PD3<3>,

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

            PG10<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB5<6>,

            PC12<6>,

            PD6<5>,

            PG11<6>,
        ],

        <Nss, PushPull> for [
            PA4<6>,

            PA15<6>,

            PG12<6>,
        ],

        <Rdy, PushPull> for [
            PA0<6>,

            PB8<6>,

            PG13<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB3<6>,

            PC10<6>,

            PG9<6>,
        ],
    }
    impl SpiCommon for crate::pac::SPI3 {
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

        <Ch4N> default:PushPull for [
            PC5<1>,

            PE15<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<1>,

            PB12<1>,

            PE15<1>,
        ],

        <Bkin2, PushPull> for [
            PA11<2>,

            PE14<2>,
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
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<3> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimNCPin<4> for TIM {
        type ChN<Otype> = Ch4N<Otype>;
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
            PA0<14>,

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

pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PF6<2>,

            PH10<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PF7<2>,

            PH11<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PF8<2>,

            PH12<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PF9<2>,

            PI0<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PF6<1>,
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

        <Ch4N> default:PushPull for [
            PB2<3>,

            PD0<3>,

            PH12<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<3>,

            PB7<3>,

            PI4<3>,
        ],

        <Bkin2, PushPull> for [
            PB6<3>,

            PC9<1>,
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
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<3> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimNCPin<4> for TIM {
        type ChN<Otype> = Ch4N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim15 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<14>,

            PB14<14>,

            PF9<14>,

            PG10<14>,
        ],

        <Ch1N> default:PushPull for [
            PA1<14>,

            PB13<14>,

            PG9<14>,
        ],

        <Ch2> default:PushPull for [
            PA3<14>,

            PB15<14>,

            PF10<14>,

            PG11<14>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA9<14>,

            PB12<14>,
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
            PA6<14>,

            PB8<14>,

            PE0<14>,
        ],

        <Ch1N> default:PushPull for [
            PB6<14>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PB5<14>,
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
            PA7<14>,

            PB9<14>,

            PE1<14>,
        ],

        <Ch1N> default:PushPull for [
            PB7<14>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA10<14>,

            PB4<14>,
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
        <Sync, PushPull> for [
            PB10<9>,

            PD2<9>,
        ],
    }

    pin! {
        <G1Io1> default:PushPull for [
            PB12<9>,
        ],

        <G1Io2> default:PushPull for [
            PB13<9>,
        ],

        <G1Io3> default:PushPull for [
            PB14<9>,
        ],

        <G1Io4> default:PushPull for [
            PC3<9>,
        ],

        <G2Io1> default:PushPull for [
            PB4<9>,
        ],

        <G2Io2> default:PushPull for [
            PB5<9>,
        ],

        <G2Io3> default:PushPull for [
            PB6<9>,
        ],

        <G2Io4> default:PushPull for [
            PB7<9>,
        ],

        <G3Io1> default:PushPull for [
            PC2<9>,
        ],

        <G3Io2> default:PushPull for [
            PC10<9>,
        ],

        <G3Io3> default:PushPull for [
            PC11<9>,
        ],

        <G3Io4> default:PushPull for [
            PC12<9>,
        ],

        <G4Io1> default:PushPull for [
            PC6<9>,
        ],

        <G4Io2> default:PushPull for [
            PC7<9>,
        ],

        <G4Io3> default:PushPull for [
            PC8<9>,
        ],

        <G4Io4> default:PushPull for [
            PC9<9>,
        ],

        <G5Io1> default:PushPull for [
            PE10<9>,
        ],

        <G5Io2> default:PushPull for [
            PE11<9>,
        ],

        <G5Io3> default:PushPull for [
            PE12<9>,
        ],

        <G5Io4> default:PushPull for [
            PE13<9>,
        ],

        <G6Io1> default:PushPull for [
            PD10<9>,
        ],

        <G6Io2> default:PushPull for [
            PD11<9>,
        ],

        <G6Io3> default:PushPull for [
            PD12<9>,
        ],

        <G6Io4> default:PushPull for [
            PD13<9>,
        ],

        <G7Io1> default:PushPull for [
            PE2<9>,
        ],

        <G7Io2> default:PushPull for [
            PE3<9>,
        ],

        <G7Io3> default:PushPull for [
            PE4<9>,
        ],

        <G7Io4> default:PushPull for [
            PE5<9>,
        ],

        <G8Io1> default:PushPull for [
            PF14<9>,
        ],

        <G8Io2> default:PushPull for [
            PF15<9>,
        ],

        <G8Io3> default:PushPull for [
            PG0<9>,
        ],

        <G8Io4> default:PushPull for [
            PG1<9>,
        ],
    }
}

pub mod ucpd1 {
    use super::*;

    pin! {
        <Frstx1, PushPull> for [
            PA2<11>,

            PB2<11>,

            PG6<11>,
        ],

        <Frstx2, PushPull> for [
            PC11<11>,

            PF13<11>,

            PG7<11>,
        ],
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<7>,

            PB5<7>,

            PG13<7>,
        ],

        <Cts, PushPull> for [
            PA11<7>,

            PB4<7>,

            PG11<7>,
        ],

        <De, PushPull> for [
            PA12<7>,

            PB3<7>,

            PG12<7>,
        ],

        <Rts, PushPull> for [
            PA12<7>,

            PB3<7>,

            PG12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA10<7>,

            PB7<7>,

            PG10<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<7>,

            PB6<7>,

            PG9<7>,
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

        <De, PushPull> for [
            PA1<7>,

            PD4<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,

            PD4<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<7>,

            PA15<3>,

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
            PB0<7>,

            PB12<7>,

            PC12<7>,

            PD10<7>,
        ],

        <Cts, PushPull> for [
            PA6<7>,

            PB13<7>,

            PD11<7>,
        ],

        <De, PushPull> for [
            PA15<7>,

            PB1<7>,

            PB14<7>,

            PD2<7>,

            PD12<7>,
        ],

        <Rts, PushPull> for [
            PA15<7>,

            PB1<7>,

            PB14<7>,

            PD2<7>,

            PD12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA5<7>,

            PB11<7>,

            PC5<7>,

            PC11<7>,

            PD9<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA7<7>,

            PB10<7>,

            PC4<7>,

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

pub mod uart4 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB7<8>,
        ],

        <De, PushPull> for [
            PA15<8>,
        ],

        <Rts, PushPull> for [
            PA15<8>,
        ],
    }

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
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod uart5 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB5<8>,
        ],

        <De, PushPull> for [
            PB4<8>,
        ],

        <Rts, PushPull> for [
            PB4<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PD2<8>,

            #[cfg(feature = "gpio-u59x")]
            PF4<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC12<8>,

            #[cfg(feature = "gpio-u59x")]
            PF3<8>,
        ],
    }
    use crate::pac::UART5 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "gpio-u59x")]
pub mod usart6 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC1<7>,

            PD14<7>,

            PE2<7>,

            PF2<7>,

            PJ6<7>,
        ],

        <Cts, PushPull> for [
            PC0<7>,

            PD13<7>,

            PE3<7>,

            PF3<7>,

            PJ7<7>,
        ],

        <De, PushPull> for [
            PD15<7>,

            PE4<7>,

            PF4<7>,

            PJ5<7>,
        ],

        <Rts, PushPull> for [
            PD15<7>,

            PE4<7>,

            PF4<7>,

            PJ5<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC2<7>,

            PC8<7>,

            PE0<7>,

            PF1<7>,

            PJ4<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC3<7>,

            PC9<7>,

            PE1<7>,

            PF0<7>,

            PJ3<7>,
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

#[cfg(feature = "gpio-u5x")]
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

        <Noe, PushPull> for [
            PA13<10>,

            PC9<10>,
        ],

        <Sof, PushPull> for [
            PA8<10>,

            PA14<10>,
        ],
    }
}

#[cfg(feature = "gpio-u59x")]
pub mod otg_hs {
    use super::*;

    pin! {
        <Id, PushPull> for [
            PA10<10>,
        ],

        <Sof, PushPull> for [
            PA8<10>,

            PA14<10>,
        ],
    }
}
