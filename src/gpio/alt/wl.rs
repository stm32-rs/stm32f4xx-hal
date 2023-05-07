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

            PB10<15>,

            PB11<15>,

            PB12<15>,

            PB13<15>,

            PB14<15>,

            PB15<15>,

            PC0<15>,

            PC1<15>,

            PC2<15>,

            PC3<15>,

            PC4<15>,

            PC5<15>,

            PC6<15>,

            PC13<15>,

            PC14<15>,

            PC15<15>,

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

            PB10<12>,
        ],
    }
}

pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PA2<12>,

            PA7<12>,

            PB5<12>,

            PB11<12>,
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

        <PwrLdordy, PushPull> for [
            PA2<13>,
        ],

        <PwrReglp1S, PushPull> for [
            PA0<13>,
        ],

        <PwrReglp2S, PushPull> for [
            PA1<13>,
        ],

        <RfBusy, PushPull> for [
            PA12<6>,
        ],

        <RfDtb1, PushPull> for [
            PB3<13>,
        ],

        <RfHse32Rdy, PushPull> for [
            PA10<13>,
        ],

        <RfLdordy, PushPull> for [
            PB4<13>,
        ],

        <RfNreset, PushPull> for [
            PA11<13>,
        ],

        <RfSmpsrdy, PushPull> for [
            PB2<13>,
        ],

        <SubghzspiMisoout, PushPull> for [
            PA6<13>,
        ],

        <SubghzspiMosiout, PushPull> for [
            PA7<13>,
        ],

        <SubghzspiNssout, PushPull> for [
            PA4<13>,
        ],

        <SubghzspiSckout, PushPull> for [
            PA5<13>,
        ],
    }
}

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

pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA12<4>,

            PB15<4>,
        ],

        <Sda, OpenDrain> for [
            PA11<4>,

            PA15<4>,
        ],

        <Smba, OpenDrain> for [
            PA6<4>,

            PA13<4>,
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

            PB10<4>,

            PB13<4>,

            PC0<4>,
        ],

        <Sda, OpenDrain> for [
            PB4<4>,

            PB11<4>,

            PB14<4>,

            PC1<4>,
        ],

        <Smba, OpenDrain> for [
            PA0<4>,

            PB2<4>,

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

pub mod i2s {
    use super::*;

    pin! {
        <Ckin, PushPull> for [
            PA0<5>,
        ],
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<5>,

            PA9<5>,

            PB10<5>,

            PB13<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA3<5>,

            PB14<3>,

            PC6<5>,
        ],

        <Sd, PushPull> for [
            PA10<5>,

            PB15<5>,

            PC1<3>,

            PC3<5>,
        ],

        <Ws, PushPull> for [
            PA9<3>,

            PB9<5>,

            PB12<5>,
        ],
    }
}

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<8>,

            PB9<8>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB6<1>,

            PC3<1>,
        ],

        <In1, PushPull> for [
            PB5<1>,

            PC0<1>,
        ],

        <In2, PushPull> for [
            PB7<1>,

            PC2<1>,
        ],

        <Out> default:PushPull for [
            PA4<1>,

            PA14<1>,

            PB2<1>,

            PC1<1>,
        ],
    }
}

pub mod lptim2 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PA5<14>,

            PC3<14>,
        ],

        <In1, PushPull> for [
            PB1<14>,

            PC0<14>,
        ],

        <Out> default:PushPull for [
            PA4<14>,

            PA8<14>,
        ],
    }
}

pub mod lptim3 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PA11<3>,
        ],

        <In1, PushPull> for [
            PA12<3>,
        ],

        <Out> default:PushPull for [
            PA1<3>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA6<8>,

            PB13<8>,
        ],

        <De, PushPull> for [
            PB1<8>,

            PB12<8>,
        ],

        <Rts, PushPull> for [
            PA1<8>,

            PB1<8>,

            PB12<8>,
        ],

        <Rx, PushPull> for [
            PA3<8>,

            PB10<8>,

            PC0<8>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            PA2<8>,

            PB11<8>,

            PC1<8>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Lsco, PushPull> for [
            PA2<0>,
        ],

        <Mco, PushPull> for [
            PA8<0>,
        ],
    }
}

pub mod rf {
    use super::*;

    pin! {
        <Irq0, PushPull> for [
            PB3<6>,
        ],

        <Irq1, PushPull> for [
            PB5<6>,
        ],

        <Irq2, PushPull> for [
            PB8<6>,
        ],
    }
}

pub mod rtc {
    use super::*;

    pin! {
        <Refin, PushPull> for [
            PA10<0>,
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
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PA12<5>,

            PB5<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,

            PB2<5>,
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

pub mod spi2 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA5<3>,

            PB14<5>,

            PC2<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA10<5>,

            PB15<5>,

            PC1<3>,

            PC3<5>,
        ],

        <Nss, PushPull> for [
            PA9<3>,

            PB9<5>,

            PB12<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA8<5>,

            PA9<5>,

            PB10<5>,

            PB13<5>,
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
        <Jtrst, PushPull> for [
            PB4<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<1>,
        ],

        <Ch1N> default:PushPull for [
            PA7<1>,

            PB13<1>,
        ],

        <Ch2> default:PushPull for [
            PA9<1>,
        ],

        <Ch2N> default:PushPull for [
            PB8<1>,

            PB14<1>,
        ],

        <Ch3> default:PushPull for [
            PA10<1>,
        ],

        <Ch3N> default:PushPull for [
            PB9<1>,

            PB15<1>,
        ],

        <Ch4> default:PushPull for [
            PA11<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<12>,

            PB7<3>,

            PB12<3>,
        ],

        <Bkin2, PushPull> for [
            PA11<12>,
        ],

        <Etr, PushPull> for [
            PA12<1>,
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

pub mod tim16 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<14>,

            PB8<14>,
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

pub mod usart2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA4<7>,
        ],

        <Cts, PushPull> for [
            PA0<7>,
        ],

        <De, PushPull> for [
            PA1<7>,
        ],

        <Nss, PushPull> for [
            PA0<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<7>,
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
