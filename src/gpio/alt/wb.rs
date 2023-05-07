use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

/*pub mod cm4 {
    use super::*;

    pin! {
        <Eventout> for [
            PA0<15>,

            PA1<15>,

            PA2<15>,

            PA3<15>,

            PA4<15>,

            PA5<15>,

            PA6<15>,

            PA7<15>,

            PA8<15>,

            PA9<15>,

            PA10<15>,

            PA11<15>,

            PA12<15>,

            PA13<15>,

            PA14<15>,

            PA15<15>,

            PB0<15>,

            PB1<15>,

            PB2<15>,

            PB3<15>,

            PB4<15>,

            PB5<15>,

            PB6<15>,

            PB7<15>,

            PB8<15>,

            PB9<15>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB11<15>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB12<15>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB13<15>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB14<15>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB15<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC0<15>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC1<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC2<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC3<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC4<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC5<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC6<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC7<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC8<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC9<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC10<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC11<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC12<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC13<15>,

            PC14<15>,

            PC15<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD0<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD1<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD2<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD3<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD4<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD5<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD6<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD7<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD8<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD9<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD10<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD11<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD12<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD13<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD14<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD15<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE0<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE1<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE2<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE3<15>,

            PE4<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PH0<15>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PH1<15>,

            PH3<15>,
        ],
    }
}*/

pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA0<12>,

            PB0<12>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<12>,
        ],
    }
}

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA2<12>,

            PA7<12>,

            PB5<12>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB11<12>,
        ],
    }
}

/*#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            PA10<10>,
        ],
    }
}*/

/*#[cfg(feature = "gpio-wb3x")]
pub mod ext {
    use super::*;

    pin! {
        <PaTx> for [
            PB0<6>,
        ],
    }
}*/

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA9<4>,

            PB6<4>,

            PB8<4>,
        ],

        <Sda, OpenDrain> for [
            PA10<4>,

            PB7<4>,

            PB9<4>,
        ],

        <Smba, OpenDrain> for [
            PA1<4>,

            PA14<4>,

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

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA7<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB13<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC0<4>,
        ],

        <Sda, OpenDrain> for [
            PB4<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB11<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB14<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC1<4>,
        ],

        <Smba, OpenDrain> for [
            PB2<4>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB12<4>,
        ],
    }
    use crate::pac::I2C3 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<8>,

            PB9<8>,
        ],
    }
}

#[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
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
            PB7<11>,
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
            PD7<11>,
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
            PA4<11>,

            PA14<11>,
        ],

        <Seg6, PushPull> for [
            PB6<11>,
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

        <Vlcd, PushPull> for [
            PB2<11>,

            PC3<11>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB6<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC3<1>,
        ],

        <In1, PushPull> for [
            PB5<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC0<1>,
        ],

        <In2, PushPull> for [
            PB7<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC2<1>,
        ],

        <Out> default:PushPull for [
            PA14<1>,

            PB2<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC1<1>,
        ],
    }
}

pub mod lptim2 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PA5<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC3<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD11<14>,
        ],

        <In1, PushPull> for [
            PB1<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC0<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD12<14>,
        ],

        <Out> default:PushPull for [
            PA4<14>,

            PA8<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD13<14>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA6<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB13<8>,
        ],

        <De, PushPull> for [
            PB1<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB12<8>,
        ],

        <Rts, PushPull> for [
            PB1<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB12<8>,
        ],

        <Rx, PushPull> for [
            PA3<8>,

            PA12<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC0<8>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            PA2<8>,

            PB5<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB11<8>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC1<8>,
        ],
    }
}

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod quadspi {
    use super::*;

    pin! {
        <Bk1Io0, PushPull> for [
            PB9<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD4<10>,
        ],

        <Bk1Io1, PushPull> for [
            PB8<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD5<10>,
        ],

        <Bk1Io2, PushPull> for [
            PA7<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD6<10>,
        ],

        <Bk1Io3, PushPull> for [
            PA6<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD7<10>,
        ],

        <Bk1Ncs, PushPull> for [
            PA2<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB11<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD3<10>,
        ],

        <Clk, PushPull> for [
            PA3<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<10>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Lsco, PushPull> for [
            PA2<0>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC12<6>,

            PH3<0>,
        ],

        <Mco, PushPull> for [
            PA8<0>,

            #[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb3x", feature = "gpio-wb55x"))]
            PA15<6>,

            PB6<0>,
        ],
    }
}

#[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod rf {
    use super::*;

    pin! {
        <Dtb0, PushPull> for [
            #[cfg(feature = "gpio-wb5mx")]
            PC14<6>,
        ],

        <Dtb1, PushPull> for [
            #[cfg(feature = "gpio-wb5mx")]
            PC15<6>,
        ],

        <TxModExtPa, PushPull> for [
            PB0<6>,
        ],
    }
}

#[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod rtc {
    use super::*;

    pin! {
        <Out, PushPull> for [
            #[cfg(feature = "gpio-wb35x")]
            PB2<0>,
        ],

        <Refin, PushPull> for [
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB15<0>,
        ],
    }
}

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod sai1 {
    use super::*;

    pin! {
        <Ck1, PushPull> for [
            PA3<3>,

            PB8<3>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE2<3>,
        ],

        <Ck2, PushPull> for [
            PA8<3>,
        ],

        <D1, PushPull> for [
            PA10<3>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC3<3>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD6<3>,
        ],

        <D2, PushPull> for [
            PA9<3>,

            PB9<3>,
        ],

        <D3, PushPull> for [
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC5<3>,
        ],

        <Di1, PushPull> for [
            #[cfg(feature = "gpio-wb3x")]
            PA10<3>,
        ],

        <Di2, PushPull> for [
            #[cfg(feature = "gpio-wb3x")]
            PA9<3>,

            #[cfg(feature = "gpio-wb3x")]
            PB9<3>,
        ],

        <Extclk, PushPull> for [
            PA0<13>,

            PB2<13>,
        ],

        <FsA, PushPull> for [
            PA9<13>,

            PB9<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB12<13>,
        ],

        <FsB, PushPull> for [
            PA4<13>,

            PA14<13>,

            PB6<13>,
        ],

        <MclkA, PushPull> for [
            PA3<13>,

            PB8<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB14<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE2<13>,
        ],

        <MclkB, PushPull> for [
            PB4<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD5<13>,
        ],

        <SckA, PushPull> for [
            PA8<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB13<13>,
        ],

        <SckB, PushPull> for [
            PB3<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC9<13>,
        ],

        <SdA, PushPull> for [
            PA10<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB15<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC3<13>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD6<13>,
        ],

        <SdB, PushPull> for [
            PA5<13>,

            PA13<13>,

            PB5<13>,
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

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<5>,

            PA11<5>,

            PB4<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-wb35x")]
            PA5<4>,

            PA7<5>,

            PA12<5>,

            #[cfg(feature = "gpio-wb35x")]
            PA13<5>,

            PB5<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            #[cfg(feature = "gpio-wb35x")]
            PA14<5>,

            PA15<5>,

            PB2<5>,

            #[cfg(feature = "gpio-wb35x")]
            PB6<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA1<5>,

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

#[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod spi2 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB14<5>,

            PC2<5>,

            PD3<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<5>,

            PC1<3>,

            PC3<5>,

            PD4<5>,
        ],

        <Nss, PushPull> for [
            PB9<5>,

            PB12<5>,

            PD0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA9<5>,

            PB10<5>,

            PB13<5>,

            PD1<5>,

            PD3<3>,
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
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE2<0>,
        ],

        <Traced0, PushPull> for [
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD9<0>,
        ],

        <Traced1, PushPull> for [
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC10<0>,
        ],

        <Traced2, PushPull> for [
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD2<0>,
        ],

        <Traced3, PushPull> for [
            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC12<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<1>,

            #[cfg(feature = "gpio-wb35x")]
            PB14<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD14<1>,
        ],

        <Ch1N> default:PushPull for [
            PA7<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB13<1>,
        ],

        <Ch2> default:PushPull for [
            PA9<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD15<1>,
        ],

        <Ch2N> default:PushPull for [
            PB8<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB14<1>,
        ],

        <Ch3> default:PushPull for [
            PA10<1>,

            #[cfg(feature = "gpio-wb35x")]
            PB7<12>,
        ],

        <Ch3N> default:PushPull for [
            PB9<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB15<1>,
        ],

        <Ch4> default:PushPull for [
            PA11<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            #[cfg(feature = "gpio-wb35x")]
            PA6<1>,

            #[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PA6<12>,

            PB7<3>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB12<3>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC9<3>,
        ],

        <Bkin2, PushPull> for [
            #[cfg(feature = "gpio-wb35x")]
            PA11<2>,

            #[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PA11<12>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PD8<2>,
        ],

        <Etr, PushPull> for [
            PA12<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE0<1>,
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

            #[cfg(feature = "gpio-wb35x")]
            PB15<1>,
        ],

        <Ch2> default:PushPull for [
            PA1<1>,

            PB3<1>,

            #[cfg(feature = "gpio-wb35x")]
            PB12<1>,
        ],

        <Ch3> default:PushPull for [
            PA2<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PB10<1>,

            #[cfg(feature = "gpio-wb35x")]
            PB13<1>,
        ],

        <Ch4> default:PushPull for [
            PA3<1>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
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

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod tim16 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<14>,

            PB8<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
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

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod tim17 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<14>,

            PB9<14>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
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

/*#[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod trig {
    use super::*;

    pin! {
        <Inout> for [
            PD10<0>,
        ],
    }
}*/

#[cfg(any(feature = "gpio-wb35x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod tsc {
    use super::*;

    #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
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
            PA15<9>,
        ],

        <G3Io2> default:PushPull for [
            #[cfg(feature = "gpio-wb35x")]
            PB10<9>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC10<9>,
        ],

        <G3Io3> default:PushPull for [
            #[cfg(feature = "gpio-wb35x")]
            PC1<9>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC11<9>,
        ],

        <G7Io1> default:PushPull for [
            #[cfg(feature = "gpio-wb35x")]
            PA13<9>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE2<9>,
        ],

        <G7Io2> default:PushPull for [
            #[cfg(feature = "gpio-wb35x")]
            PA10<9>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE1<9>,
        ],

        <G7Io3> default:PushPull for [
            #[cfg(feature = "gpio-wb35x")]
            PB8<9>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PE0<9>,
        ],

        <G7Io4> default:PushPull for [
            PB9<9>,
        ],
    }

    #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
    pin! {
        <G1Io3> default:PushPull for [
            PB14<9>,
        ],

        <G1Io4> default:PushPull for [
            PB15<9>,
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
            PD4<9>,
        ],

        <G5Io2> default:PushPull for [
            PD5<9>,
        ],

        <G5Io3> default:PushPull for [
            PD6<9>,
        ],

        <G5Io4> default:PushPull for [
            PD7<9>,
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
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<7>,

            PB5<7>,
        ],

        <Cts, PushPull> for [
            PA11<7>,

            PB4<7>,
        ],

        <De, PushPull> for [
            PA12<7>,

            PB3<7>,
        ],

        <Nss, PushPull> for [
            PA11<7>,

            PB4<7>,
        ],

        <Rts, PushPull> for [
            PA12<7>,

            PB3<7>,
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

#[cfg(any(feature = "gpio-wb3x", feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
pub mod usb {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PA11<10>,
        ],

        <Dp, PushPull> for [
            PA12<10>,
        ],

        <Noe, PushPull> for [
            PA13<10>,

            #[cfg(any(feature = "gpio-wb55x", feature = "gpio-wb5mx"))]
            PC9<10>,
        ],
    }
}
