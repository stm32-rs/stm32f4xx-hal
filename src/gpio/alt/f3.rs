use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod can {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PA11<9>,

            PB8<9>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD0<7>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PA12<9>,

            PB9<9>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD1<7>,
        ],
    }
    impl CanCommon for crate::pac::CAN {
        type Rx = Rx;
        type Tx = Tx;
    }
}

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA0<8>,

            PA6<8>,

            PA11<8>,

            PB8<8>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PF4<2>,
        ],
    }
}

pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA2<8>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f373"))]
            PA7<8>,

            PA12<8>,

            PB9<8>,
        ],
    }
}

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod comp3 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA8<8>,

            PC8<7>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f333"
))]
pub mod comp4 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB1<8>,
        ],
    }
}

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod comp5 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA9<8>,

            PC7<7>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f333"
))]
pub mod comp6 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA10<8>,

            PC6<7>,
        ],
    }
}

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod comp7 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PC2<3>,
        ],
    }
}

#[cfg(feature = "gpio-f303e")]
pub mod fmc {
    use super::*;

    pin! {
        <A0, PushPull> for [
            PH0<12>,
        ],

        <A1, PushPull> for [
            PH1<12>,
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

        <Cd, PushPull> for [
            PF9<12>,
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

#[cfg(feature = "gpio-f333")]
pub mod hrtim1 {
    use super::*;

    pin! {
        <Cha1, PushPull> for [ // High speed
            PA8<13>,
        ],

        <Cha2, PushPull> for [
            PA9<13>,
        ],

        <Chb1, PushPull> for [
            PA10<13>,
        ],

        <Chb2, PushPull> for [
            PA11<13>,
        ],

        <Chc1, PushPull> for [
            PB12<13>,
        ],

        <Chc2, PushPull> for [
            PB13<13>,
        ],

        <Chd1, PushPull> for [
            PB14<13>,
        ],

        <Chd2, PushPull> for [
            PB15<13>,
        ],

        <Che1, PushPull> for [
            PC8<3>,
        ],

        <Che2, PushPull> for [
            PC9<3>,
        ],

        <Eev1, PushPull> for [ // Low speed
            PC12<3>,
        ],

        <Eev10, PushPull> for [
            PC6<3>,
        ],

        <Eev2, PushPull> for [
            PC11<3>,
        ],

        <Eev3, PushPull> for [
            PB7<13>,
        ],

        <Eev4, PushPull> for [
            PB6<13>,
        ],

        <Eev5, PushPull> for [
            PB9<13>,
        ],

        <Eev6, PushPull> for [
            PB5<13>,
        ],

        <Eev7, PushPull> for [
            PB4<13>,
        ],

        <Eev8, PushPull> for [
            PB8<13>,
        ],

        <Eev9, PushPull> for [
            PB3<13>,
        ],

        <Flt1, PushPull> for [
            PA12<13>,
        ],

        <Flt2, PushPull> for [
            PA15<13>,
        ],

        <Flt3, PushPull> for [
            PB10<13>,
        ],

        <Flt4, PushPull> for [
            PB11<13>,
        ],

        <Flt5, PushPull> for [
            PC7<3>,
        ],

        <Scin, PushPull> for [
            PB2<13>,

            PB6<12>,
        ],
    }

    pin! {
        <Scout> default:PushPull for [ // High speed
            PB1<13>,

            PB3<12>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA15<4>,

            PB6<4>,

            PB8<4>,
        ],

        <Sda, OpenDrain> for [
            PA14<4>,

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

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f373"
))]
pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA9<4>,

            PF1<4>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PF6<4>,
        ],

        <Sda, OpenDrain> for [
            PA10<4>,

            PF0<4>,

            #[cfg(feature = "gpio-f373")]
            PF7<4>,
        ],

        <Smba, OpenDrain> for [
            PA8<4>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB12<4>,

            #[cfg(feature = "gpio-f373")]
            PF2<4>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<3>,
        ],

        <Sda, OpenDrain> for [
            PB5<8>,

            PC9<3>,
        ],

        <Smba, OpenDrain> for [
            PA9<2>,
        ],
    }
    use crate::pac::I2C3 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod i2s {
    use super::*;

    pin! {
        <Ckin, PushPull> for [
            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PA12<5>,

            PC9<5>,
        ],
    }
}

#[cfg(feature = "gpio-f373")]
pub mod i2s1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA5<5>,

            PA12<6>,

            PB3<5>,

            PC7<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA6<5>,

            PA13<6>,

            PB4<5>,

            PC8<5>,
        ],

        <Sd, PushPull> for [
            PA7<5>,

            PB0<5>,

            PB5<5>,

            PC9<5>,

            PF6<5>,
        ],

        <Ws, PushPull> for [
            PA4<5>,

            PA11<6>,

            PA15<5>,

            PC6<5>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f373"
))]
pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            #[cfg(feature = "gpio-f373")]
            PA8<5>,

            #[cfg(feature = "gpio-f373")]
            PB8<5>,

            #[cfg(feature = "gpio-f373")]
            PB10<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB13<5>,

            #[cfg(feature = "gpio-f373")]
            PD7<5>,

            #[cfg(feature = "gpio-f373")]
            PD8<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PF1<5>,
        ],

        <ExtSd, PushPull> for [
            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PA10<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB14<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PA8<5>,

            #[cfg(feature = "gpio-f373")]
            PA9<5>,

            #[cfg(feature = "gpio-f373")]
            PB14<5>,

            #[cfg(feature = "gpio-f373")]
            PC2<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PC6<6>,

            #[cfg(feature = "gpio-f373")]
            PD3<5>,
        ],

        <Sd, PushPull> for [
            #[cfg(feature = "gpio-f373")]
            PA10<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PA11<5>,

            PB15<5>,

            #[cfg(feature = "gpio-f373")]
            PC3<5>,

            #[cfg(feature = "gpio-f373")]
            PD4<5>,
        ],

        <Ws, PushPull> for [
            #[cfg(feature = "gpio-f373")]
            PA11<5>,

            #[cfg(feature = "gpio-f373")]
            PB9<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB12<5>,

            #[cfg(feature = "gpio-f373")]
            PD6<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PF0<5>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f373"
))]
pub mod i2s3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            #[cfg(feature = "gpio-f373")]
            PA1<6>,

            PB3<6>,

            PC10<6>,
        ],

        <ExtSd, PushPull> for [
            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB4<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PC11<6>,
        ],

        <Mck, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA2<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PA9<5>,

            #[cfg(feature = "gpio-f373")]
            PB4<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PC7<6>,

            #[cfg(feature = "gpio-f373")]
            PC11<6>,
        ],

        <Sd, PushPull> for [
            #[cfg(feature = "gpio-f373")]
            PA3<6>,

            PB5<6>,

            PC12<6>,
        ],

        <Ws, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],
    }
}

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<5>,

            PB9<6>,
        ],
    }
}

/*#[cfg(feature = "gpio-f333")]
pub mod opamp2 {
    use super::*;

    pin! {
        <Dig> for [
            PA6<13>,
        ],
    }
}*/

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f373"
))]
pub mod rtc {
    use super::*;

    pin! {
        <Refin, PushPull> for [
            PA1<0>,

            PB15<0>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f333",
    feature = "gpio-f373"
))]
pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<5>,

            #[cfg(feature = "gpio-f373")]
            PA13<6>,

            PB4<5>,

            #[cfg(feature = "gpio-f373")]
            PC8<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            #[cfg(feature = "gpio-f373")]
            PB0<5>,

            PB5<5>,

            #[cfg(feature = "gpio-f373")]
            PC9<5>,

            #[cfg(feature = "gpio-f373")]
            PF6<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            #[cfg(feature = "gpio-f373")]
            PA11<6>,

            PA15<5>,

            #[cfg(feature = "gpio-f373")]
            PC6<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<5>,

            #[cfg(feature = "gpio-f373")]
            PA12<6>,

            PB3<5>,

            #[cfg(feature = "gpio-f373")]
            PC7<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI1 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f373"
))]
pub mod spi2 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA9<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PA10<5>,

            PB14<5>,

            #[cfg(feature = "gpio-f373")]
            PC2<5>,

            #[cfg(feature = "gpio-f373")]
            PD3<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA10<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PA11<5>,

            PB15<5>,

            #[cfg(feature = "gpio-f373")]
            PC3<5>,

            #[cfg(feature = "gpio-f373")]
            PD4<5>,
        ],

        <Nss, PushPull> for [
            #[cfg(feature = "gpio-f373")]
            PA11<5>,

            #[cfg(feature = "gpio-f373")]
            PB9<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB12<5>,

            #[cfg(feature = "gpio-f373")]
            PD6<5>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PD15<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PF0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA8<5>,

            #[cfg(feature = "gpio-f373")]
            PB8<5>,

            #[cfg(feature = "gpio-f373")]
            PB10<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303", feature = "gpio-f303e"))]
            PB13<5>,

            #[cfg(feature = "gpio-f373")]
            PD7<5>,

            #[cfg(feature = "gpio-f373")]
            PD8<5>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e"))]
            PF1<5>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PF9<5>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PF10<5>,
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
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f373"
))]
pub mod spi3 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA2<6>,

            PB4<6>,

            PC11<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA3<6>,

            PB5<6>,

            PC12<6>,
        ],

        <Nss, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PA1<6>,

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

#[cfg(feature = "gpio-f303e")]
pub mod spi4 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PE5<5>,

            PE13<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PE6<5>,

            PE14<5>,
        ],

        <Nss, PushPull> for [
            PE3<5>,

            PE4<5>,

            PE11<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PE2<5>,

            PE12<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI4 {
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

        <Njtrst, PushPull> for [
            PB4<0>,
        ],

        <Traceck, PushPull> for [
            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE2<0>,
        ],

        <Traced0, PushPull> for [
            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE3<0>,
        ],

        <Traced1, PushPull> for [
            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE4<0>,
        ],

        <Traced2, PushPull> for [
            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE5<0>,
        ],

        <Traced3, PushPull> for [
            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE6<0>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f302",
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f333"
))]
pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PC0<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE9<2>,
        ],

        <Ch1N> default:PushPull for [
            PA7<6>,

            PA11<6>,

            PB13<6>,

            PC13<4>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE8<2>,
        ],

        <Ch2> default:PushPull for [
            PA9<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PC1<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE11<2>,
        ],

        <Ch2N> default:PushPull for [
            PA12<6>,

            PB0<6>,

            PB14<6>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE10<2>,
        ],

        <Ch3> default:PushPull for [
            PA10<6>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PC2<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE13<2>,
        ],

        <Ch3N> default:PushPull for [
            PB1<6>,

            PB15<4>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE12<2>,

            PF0<6>,
        ],

        <Ch4> default:PushPull for [
            PA11<11>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PC3<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE14<2>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<6>,

            PA14<6>,

            PA15<9>,

            PB8<12>,

            PB12<6>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE15<2>,
        ],

        <Bkin2, PushPull> for [
            PA11<12>,

            PC3<6>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE14<6>,
        ],

        <Etr, PushPull> for [
            PA12<11>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PC4<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE7<2>,
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
            PA0<1>,

            PA5<1>,

            PA15<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PD3<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<1>,

            PB3<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PD4<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<1>,

            PA9<10>,

            PB10<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PD7<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<1>,

            PA10<10>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB11<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PD6<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<1>,

            PA5<1>,

            PA15<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PD3<2>,
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

#[cfg(any(
    feature = "gpio-f303",
    feature = "gpio-f303e",
    feature = "gpio-f333",
    feature = "gpio-f373"
))]
pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<2>,

            PB4<2>,

            PC6<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE2<2>,
        ],

        <Ch2> default:PushPull for [
            PA4<2>,

            PA7<2>,

            #[cfg(feature = "gpio-f373")]
            PB0<10>,

            PB5<2>,

            PC7<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE3<2>,
        ],

        <Ch3> default:PushPull for [
            PB0<2>,

            #[cfg(feature = "gpio-f373")]
            PB6<10>,

            PC8<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE4<2>,
        ],

        <Ch4> default:PushPull for [
            PB1<2>,

            PB7<10>,

            PC9<2>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE5<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PB3<10>,

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

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
pub mod tim4 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA11<10>,

            PB6<2>,

            PD12<2>,
        ],

        <Ch2> default:PushPull for [
            PA12<10>,

            PB7<2>,

            PD13<2>,
        ],

        <Ch3> default:PushPull for [
            PA13<10>,

            PB8<2>,

            PD14<2>,
        ],

        <Ch4> default:PushPull for [
            PB9<2>,

            PD15<2>,

            PF6<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA8<10>,

            PB3<2>,

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

#[cfg(feature = "gpio-f373")]
pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PA8<2>,

            PC0<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PA11<2>,

            PC1<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PA12<2>,

            PC2<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PA13<2>,

            PC3<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<2>,

            PA8<2>,

            PC0<2>,
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

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod tim8 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA15<2>,

            PB6<5>,

            PC6<4>,
        ],

        <Ch1N> default:PushPull for [
            PA7<4>,

            PB3<4>,

            PC10<4>,
        ],

        <Ch2> default:PushPull for [
            PA14<5>,

            PB8<10>,

            PC7<4>,
        ],

        <Ch2N> default:PushPull for [
            PB0<4>,

            PB4<4>,

            PC11<4>,
        ],

        <Ch3> default:PushPull for [
            PB9<10>,

            PC8<4>,
        ],

        <Ch3N> default:PushPull for [
            PB1<4>,

            PB5<3>,

            PC12<4>,
        ],

        <Ch4> default:PushPull for [
            PC9<4>,

            PD1<4>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA0<9>,

            PA6<4>,

            PA10<11>,

            PB7<5>,

            PD2<4>,
        ],

        <Bkin2, PushPull> for [
            PB6<10>,

            PC9<6>,

            PD1<6>,
        ],

        <Etr, PushPull> for [
            PA0<10>,

            PB6<6>,
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
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(feature = "gpio-f373")]
pub mod tim12 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA4<10>,

            PA14<10>,

            PB14<9>,
        ],

        <Ch2> default:PushPull for [
            PA5<10>,

            PA15<10>,

            PB15<9>,
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

#[cfg(feature = "gpio-f373")]
pub mod tim13 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<9>,

            PA9<2>,

            PB3<9>,

            PC4<2>,
        ],
    }

    use crate::pac::TIM13 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

#[cfg(feature = "gpio-f373")]
pub mod tim14 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA5<9>,

            PA7<9>,

            PA10<9>,

            PF9<2>,
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
            PA2<9>,

            #[cfg(feature = "gpio-f373")]
            PB6<9>,

            PB14<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PF9<3>,
        ],

        <Ch1N> default:PushPull for [
            PA1<9>,

            #[cfg(feature = "gpio-f373")]
            PB4<9>,

            PB15<2>,
        ],

        <Ch2> default:PushPull for [
            PA3<9>,

            #[cfg(feature = "gpio-f373")]
            PB7<9>,

            PB15<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PF10<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA9<9>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PC5<2>,
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
            PA6<1>,

            PA12<1>,

            PB4<1>,

            PB8<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE0<4>,
        ],

        <Ch1N> default:PushPull for [
            PA13<1>,

            PB6<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PB5<1>,
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
            PA7<1>,

            PB5<10>,

            PB9<1>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
            PE1<4>,
        ],

        <Ch1N> default:PushPull for [
            PB7<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA10<1>,

            PB4<10>,
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

#[cfg(feature = "gpio-f373")]
pub mod tim19 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<11>,

            PB6<11>,

            PC10<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<11>,

            PB7<11>,

            PC11<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<11>,

            PB8<11>,

            PC12<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<11>,

            PB9<11>,

            PD0<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PB5<11>,

            PD1<2>,
        ],
    }

    use crate::pac::TIM19 as TIM;
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

#[cfg(feature = "gpio-f303e")]
pub mod tim20 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PE2<6>,

            PF12<2>,

            PH0<2>,
        ],

        <Ch1N> default:PushPull for [
            PE4<6>,

            PF4<3>,

            PG0<2>,
        ],

        <Ch2> default:PushPull for [
            PE3<6>,

            PF13<2>,

            PH1<2>,
        ],

        <Ch2N> default:PushPull for [
            PE5<6>,

            PF5<2>,

            PG1<2>,
        ],

        <Ch3> default:PushPull for [
            PF2<2>,

            PF14<2>,
        ],

        <Ch3N> default:PushPull for [
            PE6<6>,

            PG2<2>,
        ],

        <Ch4> default:PushPull for [
            PE1<6>,

            PF3<2>,

            PF15<2>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PF7<2>,

            PF9<2>,

            PG3<2>,
        ],

        <Bkin2, PushPull> for [
            PF8<2>,

            PF10<2>,

            PG4<2>,
        ],

        <Etr, PushPull> for [
            PE0<6>,

            PF11<2>,

            PG5<2>,
        ],
    }

    use crate::pac::TIM20 as TIM;
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

pub mod tsc {
    use super::*;

    pin! {
        <Sync, PushPull> for [ // Low speed
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303e",
                feature = "gpio-f333",
                feature = "gpio-f373"
            ))]
            PA15<3>,

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
            #[cfg(feature = "gpio-f373")]
            PC4<3>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PC5<3>,
        ],

        <G3Io2> default:PushPull for [
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB0<3>,

            #[cfg(feature = "gpio-f373")]
            PC5<3>,
        ],

        <G3Io3> default:PushPull for [
            #[cfg(feature = "gpio-f373")]
            PB0<3>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB1<3>,
        ],

        <G3Io4> default:PushPull for [
            #[cfg(feature = "gpio-f373")]
            PB1<3>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB2<3>,
        ],

        <G4Io1> default:PushPull for [
            PA9<3>,
        ],

        <G4Io2> default:PushPull for [
            PA10<3>,
        ],

        <G4Io3> default:PushPull for [
            PA13<3>,
        ],

        <G4Io4> default:PushPull for [
            PA14<3>,
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
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB11<3>,

            #[cfg(feature = "gpio-f373")]
            PB14<3>,
        ],

        <G6Io2> default:PushPull for [
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB12<3>,

            #[cfg(feature = "gpio-f373")]
            PB15<3>,
        ],

        <G6Io3> default:PushPull for [
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB13<3>,

            #[cfg(feature = "gpio-f373")]
            PD8<3>,
        ],

        <G6Io4> default:PushPull for [
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB14<3>,

            #[cfg(feature = "gpio-f373")]
            PD9<3>,
        ],
    }

    #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
    pin! {
        <G7Io1> default:PushPull for [
            PE2<3>,
        ],

        <G7Io2> default:PushPull for [
            PE3<3>,
        ],

        <G7Io3> default:PushPull for [
            PE4<3>,
        ],

        <G7Io4> default:PushPull for [
            PE5<3>,
        ],

        <G8Io1> default:PushPull for [
            PD12<3>,
        ],

        <G8Io2> default:PushPull for [
            PD13<3>,
        ],

        <G8Io3> default:PushPull for [
            PD14<3>,
        ],

        <G8Io4> default:PushPull for [
            PD15<3>,
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

        <De, PushPull> for [
            PA12<7>,
        ],

        <Rts, PushPull> for [
            PA12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA10<7>,

            PB7<7>,

            PC5<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE1<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<7>,

            PB6<7>,

            PC4<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE0<7>,
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

            PB5<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD7<7>,

            #[cfg(feature = "gpio-f373")]
            PF7<7>,
        ],

        <Cts, PushPull> for [
            PA0<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD3<7>,
        ],

        <De, PushPull> for [
            PA1<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD4<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD4<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<7>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PA15<7>,

            PB4<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD6<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<7>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PA14<7>,

            PB3<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
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
            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB12<7>,

            PC12<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD10<7>,
        ],

        <Cts, PushPull> for [
            PA13<7>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB13<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD11<7>,
        ],

        <De, PushPull> for [
            PB14<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD12<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PF6<7>,
        ],

        <Rts, PushPull> for [
            PB14<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD12<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PF6<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PB8<7>,

            #[cfg(feature = "gpio-f373")]
            PB9<7>,

            #[cfg(any(
                feature = "gpio-f302",
                feature = "gpio-f303",
                feature = "gpio-f303e",
                feature = "gpio-f333"
            ))]
            PB11<7>,

            PC11<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PD9<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
            PE15<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f373")]
            PB8<7>,

            #[cfg(any(feature = "gpio-f302", feature = "gpio-f303e", feature = "gpio-f333"))]
            PB9<7>,

            PB10<7>,

            PC10<7>,

            #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e", feature = "gpio-f373"))]
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

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod uart4 {
    use super::*;

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC11<5>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC10<5>,
        ],
    }
    use crate::pac::UART4 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
pub mod uart5 {
    use super::*;

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PD2<5>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC12<5>,
        ],
    }
    use crate::pac::UART5 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(any(feature = "gpio-f303", feature = "gpio-f373"))]
pub mod usb {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PA11<14>,
        ],

        <Dp, PushPull> for [
            PA12<14>,
        ],
    }
}
