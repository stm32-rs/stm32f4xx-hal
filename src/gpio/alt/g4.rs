use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA0<8>,

            PA6<8>,

            PA11<8>,

            PB8<8>,

            #[cfg(feature = "gpio-g47x")]
            PF4<2>,
        ],
    }
}

pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA2<8>,

            PA7<8>,

            PA12<8>,

            PB9<8>,
        ],
    }
}

pub mod comp3 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB7<8>,

            PB15<3>,

            PC2<3>,
        ],
    }
}

pub mod comp4 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PB1<8>,

            PB6<8>,

            PB14<8>,
        ],
    }
}

#[cfg(feature = "gpio-g47x")]
pub mod comp5 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA9<8>,

            PC7<7>,
        ],
    }
}

#[cfg(feature = "gpio-g47x")]
pub mod comp6 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA10<8>,

            PC6<7>,
        ],
    }
}

#[cfg(feature = "gpio-g47x")]
pub mod comp7 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA8<8>,

            PC8<7>,
        ],
    }
}

/*pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            PA10<3>,

            PB3<3>,
        ],
    }
}*/

pub mod fdcan1 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PA11<9>,

            PB8<9>,

            PD0<9>,
        ],

        <Tx, PushPull> for [
            PA12<9>,

            PB9<9>,

            PD1<9>,
        ],
    }
}

#[cfg(any(feature = "gpio-g47x", feature = "gpio-g49x"))]
pub mod fdcan2 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PB5<9>,

            PB12<9>,
        ],

        <Tx, PushPull> for [
            PB6<9>,

            PB13<9>,
        ],
    }
}

#[cfg(feature = "gpio-g47x")]
pub mod fdcan3 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PA8<11>,

            PB3<11>,
        ],

        <Tx, PushPull> for [
            PA15<11>,

            PB4<11>,
        ],
    }
}

#[cfg(feature = "gpio-g47x")]
pub mod fmc {
    use super::*;

    pin! {
        <A0, PushPull> for [
            PF10<12>,
        ],

        <A1, PushPull> for [
            PF7<12>,
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
            PF8<12>,
        ],

        <A25, PushPull> for [
            PF9<12>,
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
            PG6<12>,

            PG7<12>,
        ],

        <Nbl0, PushPull> for [
            PE0<12>,
        ],

        <Nbl1, PushPull> for [
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
            PG8<12>,
        ],

        <Ne4, PushPull> for [
            PF11<12>,
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

#[cfg(feature = "gpio-g47x")]
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

        <Chf1, PushPull> for [
            PC6<13>,
        ],

        <Chf2, PushPull> for [
            PC7<13>,
        ],

        <Eev1, PushPull> for [ // Low speed
            PC12<3>,
        ],

        <Eev10, PushPull> for [
            PC5<13>,

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
            PB0<13>,

            PC7<3>,
        ],

        <Flt6, PushPull> for [
            PC10<13>,
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
            PA13<4>,

            PA15<4>,

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

pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA9<4>,

            PC4<4>,

            #[cfg(feature = "gpio-g47x")]
            PF6<4>,
        ],

        <Sda, OpenDrain> for [
            PA8<4>,

            PF0<4>,
        ],

        <Smba, OpenDrain> for [
            PA10<4>,

            PB12<4>,

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

pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<2>,

            PC8<8>,

            #[cfg(feature = "gpio-g47x")]
            PF3<4>,

            #[cfg(feature = "gpio-g47x")]
            PG7<4>,
        ],

        <Sda, OpenDrain> for [
            PB5<8>,

            PC9<8>,

            PC11<8>,

            #[cfg(feature = "gpio-g47x")]
            PF4<4>,

            #[cfg(feature = "gpio-g47x")]
            PG8<4>,
        ],

        <Smba, OpenDrain> for [
            PA9<2>,

            PB2<4>,

            #[cfg(feature = "gpio-g47x")]
            PG6<4>,
        ],
    }
    use crate::pac::I2C3 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(feature = "gpio-g47x")]
pub mod i2c4 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA13<3>,

            PC6<8>,

            PF14<4>,

            PG3<4>,
        ],

        <Sda, OpenDrain> for [
            PB7<3>,

            PC7<8>,

            PF15<4>,

            PG4<4>,
        ],

        <Smba, OpenDrain> for [
            PA14<3>,

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

pub mod i2s {
    use super::*;

    pin! {
        <Ckin, PushPull> for [
            PA12<5>,

            PC9<5>,
        ],
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB13<5>,

            PF1<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA8<5>,

            PC6<6>,
        ],

        <Sd, PushPull> for [
            PA11<5>,

            PB15<5>,
        ],

        <Ws, PushPull> for [
            PB12<5>,

            PF0<5>,
        ],
    }
}

pub mod i2s3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB3<6>,

            PC10<6>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA9<5>,

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

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<5>,

            PB9<6>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB6<11>,

            PC3<1>,
        ],

        <In1, PushPull> for [
            PB5<11>,

            PC0<1>,
        ],

        <In2, PushPull> for [
            PB7<11>,

            PC2<1>,
        ],
    }

    pin! {
        <Out> default:PushPull for [
            PA14<1>,

            PB2<1>,

            PC1<1>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA6<12>,

            PB13<8>,

            #[cfg(feature = "gpio-g47x")]
            PG5<8>,
        ],

        <De, PushPull> for [
            PB1<12>,

            PB12<8>,

            #[cfg(feature = "gpio-g47x")]
            PG6<8>,
        ],

        <Rts, PushPull> for [
            PB1<12>,

            PB12<8>,

            #[cfg(feature = "gpio-g47x")]
            PG6<8>,
        ],

        <Rx, PushPull> for [
            PA3<12>,

            PB10<8>,

            PC0<8>,

            #[cfg(feature = "gpio-g47x")]
            PG8<8>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            PA2<12>,

            PB11<8>,

            PC1<8>,

            #[cfg(feature = "gpio-g47x")]
            PG7<8>,
        ],
    }
}

#[cfg(any(feature = "gpio-g47x", feature = "gpio-g49x"))]
pub mod quadspi1 {
    use super::*;

    pin! {
        <Bk1Io0, PushPull> for [
            PB1<10>,

            PE12<10>,

            #[cfg(feature = "gpio-g47x")]
            PF8<10>,
        ],

        <Bk1Io1, PushPull> for [
            PB0<10>,

            PE13<10>,

            PF9<10>,
        ],

        <Bk1Io2, PushPull> for [
            PA7<10>,

            PE14<10>,

            #[cfg(feature = "gpio-g47x")]
            PF7<10>,
        ],

        <Bk1Io3, PushPull> for [
            PA6<10>,

            PE15<10>,

            #[cfg(feature = "gpio-g47x")]
            PF6<10>,
        ],

        <Bk1Ncs, PushPull> for [
            PA2<10>,

            PB11<10>,

            PE11<10>,
        ],

        <Bk2Io0, PushPull> for [
            PC1<10>,

            PD4<10>,
        ],

        <Bk2Io1, PushPull> for [
            PB2<10>,

            PC2<10>,

            PD5<10>,
        ],

        <Bk2Io2, PushPull> for [
            PC3<10>,

            PD6<10>,
        ],

        <Bk2Io3, PushPull> for [
            PC4<10>,

            PD7<10>,
        ],

        <Bk2Ncs, PushPull> for [
            PD3<10>,
        ],

        <Clk, PushPull> for [
            PA3<10>,

            PB10<10>,

            PE10<10>,

            PF10<10>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,

            PG10<0>,
        ],
    }
}

pub mod rtc {
    use super::*;

    pin! {
        <Out2, PushPull> for [
            PB2<0>,
        ],

        <Refin, PushPull> for [
            PA1<0>,

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

            #[cfg(feature = "gpio-g47x")]
            PG7<3>,
        ],

        <Ck2, PushPull> for [
            PA8<12>,

            PE5<3>,
        ],

        <D1, PushPull> for [
            PA10<12>,

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
            PA9<14>,

            PB9<14>,

            PE4<13>,
        ],

        <FsB, PushPull> for [
            PA4<13>,

            PA14<13>,

            PB6<14>,

            PE9<13>,

            PF9<13>,
        ],

        <MclkA, PushPull> for [
            PA3<13>,

            PB8<14>,

            PE2<13>,

            #[cfg(feature = "gpio-g47x")]
            PG7<13>,
        ],

        <MclkB, PushPull> for [
            PB4<14>,

            PE10<13>,

            #[cfg(feature = "gpio-g47x")]
            PF7<13>,
        ],

        <SckA, PushPull> for [
            PA8<14>,

            PB10<14>,

            PE5<13>,
        ],

        <SckB, PushPull> for [
            PB3<14>,

            PE8<13>,

            #[cfg(feature = "gpio-g47x")]
            PF8<13>,
        ],

        <SdA, PushPull> for [
            PA10<14>,

            PC1<13>,

            PC3<13>,

            PD6<13>,

            PE6<13>,
        ],

        <SdB, PushPull> for [
            PA13<13>,

            PB5<12>,

            PE3<13>,

            PE7<13>,

            #[cfg(feature = "gpio-g47x")]
            PF6<3>,
        ],
    }
    use crate::pac::SAI;
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

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<5>,

            PB4<5>,

            #[cfg(feature = "gpio-g47x")]
            PG3<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PB5<5>,

            #[cfg(feature = "gpio-g47x")]
            PG4<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,

            #[cfg(feature = "gpio-g47x")]
            PG5<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<5>,

            PB3<5>,

            #[cfg(feature = "gpio-g47x")]
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
            PA10<5>,

            PB14<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA11<5>,

            PB15<5>,
        ],

        <Nss, PushPull> for [
            PB12<5>,

            PD15<6>,

            PF0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB13<5>,

            PF1<5>,

            PF9<5>,

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

            #[cfg(feature = "gpio-g47x")]
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

#[cfg(feature = "gpio-g47x")]
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

        <JtdoSwo, PushPull> for [
            PB3<0>,
        ],

        <JtmsSwdio, PushPull> for [
            PA13<0>,
        ],

        <Jtrst, PushPull> for [
            PB4<0>,
        ],

        <Sleep, PushPull> for [
            #[cfg(feature = "gpio-g47x")]
            PC3<0>,
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
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<6>,

            PC0<2>,

            PE9<2>,
        ],

        <Ch1N> default:PushPull for [
            PA7<6>,

            PA11<6>,

            PB13<6>,

            PC13<4>,

            PE8<2>,
        ],

        <Ch2> default:PushPull for [
            PA9<6>,

            PC1<2>,

            PE11<2>,
        ],

        <Ch2N> default:PushPull for [
            PA12<6>,

            PB0<6>,

            PB14<6>,

            PE10<2>,
        ],

        <Ch3> default:PushPull for [
            PA10<6>,

            PC2<2>,

            PE13<2>,
        ],

        <Ch3N> default:PushPull for [
            PB1<6>,

            PB9<12>,

            PB15<4>,

            PE12<2>,

            PF0<6>,
        ],

        <Ch4> default:PushPull for [
            PA11<11>,

            PC3<2>,

            PE14<2>,
        ],

        <Ch4N> default:PushPull for [
            PC5<6>,

            PE15<6>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<6>,

            PA14<6>,

            PA15<9>,

            PB8<12>,

            PB10<12>,

            PB12<6>,

            PC13<2>,

            PE15<2>,
        ],

        <Bkin2, PushPull> for [
            PA11<12>,

            PC3<6>,

            PE14<6>,
        ],

        <Etr, PushPull> for [
            PA12<11>,

            PC4<2>,

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

            PD3<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<1>,

            PB3<1>,

            PD4<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<1>,

            PA9<10>,

            PB10<1>,

            PD7<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<1>,

            PA10<10>,

            PB11<1>,

            PD6<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<14>,

            PA5<2>,

            PA15<14>,

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

pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<2>,

            PB4<2>,

            PC6<2>,

            PE2<2>,
        ],

        <Ch2> default:PushPull for [
            PA4<2>,

            PA7<2>,

            PB5<2>,

            PC7<2>,

            PE3<2>,
        ],

        <Ch3> default:PushPull for [
            PB0<2>,

            PC8<2>,

            PE4<2>,
        ],

        <Ch4> default:PushPull for [
            PB1<2>,

            PB7<10>,

            PC9<2>,

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

            #[cfg(feature = "gpio-g47x")]
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

#[cfg(feature = "gpio-g47x")]
pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PB2<2>,

            PF6<6>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PC12<1>,

            PF7<6>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PE8<1>,

            PF8<6>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PE9<1>,

            PF9<6>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PB12<2>,

            PD11<1>,

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

        <Ch4N> default:PushPull for [
            PC13<6>,

            PD0<6>,
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
            PA2<9>,

            PB14<1>,

            PF9<3>,
        ],

        <Ch1N> default:PushPull for [
            PA1<9>,

            PB15<2>,

            #[cfg(feature = "gpio-g47x")]
            PG9<14>,
        ],

        <Ch2> default:PushPull for [
            PA3<9>,

            PB15<1>,

            PF10<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA9<9>,

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

#[cfg(any(feature = "gpio-g47x", feature = "gpio-g49x"))]
pub mod tim20 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB2<3>,

            PE2<6>,

            #[cfg(feature = "gpio-g47x")]
            PF12<2>,
        ],

        <Ch1N> default:PushPull for [
            PE4<6>,

            #[cfg(feature = "gpio-g47x")]
            PF4<3>,

            #[cfg(feature = "gpio-g47x")]
            PG0<2>,
        ],

        <Ch2> default:PushPull for [
            PC2<6>,

            PE3<6>,

            #[cfg(feature = "gpio-g47x")]
            PF13<2>,
        ],

        <Ch2N> default:PushPull for [
            PE5<6>,

            #[cfg(feature = "gpio-g47x")]
            PF5<2>,

            #[cfg(feature = "gpio-g47x")]
            PG1<2>,
        ],

        <Ch3> default:PushPull for [
            PC8<6>,

            PF2<2>,

            #[cfg(feature = "gpio-g47x")]
            PF14<2>,
        ],

        <Ch3N> default:PushPull for [
            PE6<6>,

            #[cfg(feature = "gpio-g47x")]
            PG2<2>,
        ],

        <Ch4> default:PushPull for [
            PE1<6>,

            #[cfg(feature = "gpio-g47x")]
            PF3<2>,

            #[cfg(feature = "gpio-g47x")]
            PF15<2>,
        ],

        <Ch4N> default:PushPull for [
            PE0<3>,

            #[cfg(feature = "gpio-g47x")]
            PG3<6>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            #[cfg(feature = "gpio-g47x")]
            PF7<2>,

            PF9<2>,

            #[cfg(feature = "gpio-g47x")]
            PG3<2>,

            #[cfg(feature = "gpio-g47x")]
            PG6<2>,
        ],

        <Bkin2, PushPull> for [
            #[cfg(feature = "gpio-g47x")]
            PF8<2>,

            PF10<2>,

            #[cfg(feature = "gpio-g47x")]
            PG4<2>,
        ],

        <Etr, PushPull> for [
            #[cfg(feature = "gpio-g49x")]
            PA15<3>,

            PE0<6>,

            #[cfg(feature = "gpio-g47x")]
            PF11<2>,

            #[cfg(feature = "gpio-g47x")]
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

pub mod ucpd1 {
    use super::*;

    pin! {
        <Frstx1, PushPull> for [
            PA2<14>,

            PA5<14>,

            PA7<14>,

            PB0<14>,

            PC12<14>,
        ],

        <Frstx2, PushPull> for [
            PA2<14>,

            PA5<14>,

            PA7<14>,

            PB0<14>,

            PC12<14>,
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

        <Nss, PushPull> for [
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

            PC5<7>,

            PE1<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<7>,

            PB6<7>,

            PC4<7>,

            PE0<7>,

            #[cfg(feature = "gpio-g47x")]
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

            PB5<7>,

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

        <Nss, PushPull> for [
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

            PA15<7>,

            PB4<7>,

            PD6<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<7>,

            PA14<7>,

            PB3<7>,

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
            PA13<7>,

            PB13<7>,

            PD11<7>,
        ],

        <De, PushPull> for [
            PB14<7>,

            PD12<7>,

            #[cfg(feature = "gpio-g47x")]
            PF6<7>,
        ],

        <Nss, PushPull> for [
            PA13<7>,

            PB13<7>,

            PD11<7>,
        ],

        <Rts, PushPull> for [
            PB14<7>,

            PD12<7>,

            #[cfg(feature = "gpio-g47x")]
            PF6<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB8<7>,

            PB11<7>,

            PC11<7>,

            PD9<7>,

            PE15<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB9<7>,

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

pub mod uart4 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB7<14>,
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
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(any(feature = "gpio-g47x", feature = "gpio-g49x"))]
pub mod uart5 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB5<14>,
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
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}
