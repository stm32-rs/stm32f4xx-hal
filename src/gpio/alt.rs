use super::{marker, Alternate, NoPin, OpenDrain, Pin, PinMode, PushPull};
use crate::{gpio, i2c, i2s, pac, serial, spi};

pub(crate) struct Const<const A: u8>;

pub(crate) trait SetAlternate<const A: u8, Otype> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}
impl<Otype> SetAlternate<0, Otype> for NoPin {
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}
impl<const P: char, const N: u8, MODE: PinMode + marker::NotAlt, const A: u8>
    SetAlternate<A, PushPull> for Pin<P, N, MODE>
{
    fn set_alt_mode(&mut self) {
        self.mode::<Alternate<A, PushPull>>();
    }

    fn restore_mode(&mut self) {
        self.mode::<MODE>();
    }
}

impl<const P: char, const N: u8, MODE: PinMode + marker::NotAlt, const A: u8>
    SetAlternate<A, OpenDrain> for Pin<P, N, MODE>
{
    fn set_alt_mode(&mut self) {
        self.mode::<Alternate<A, OpenDrain>>();
    }

    fn restore_mode(&mut self) {
        self.mode::<MODE>();
    }
}

impl<const P: char, const N: u8, const A: u8> SetAlternate<A, PushPull>
    for Pin<P, N, Alternate<A, PushPull>>
{
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}

impl<const P: char, const N: u8, const A: u8> SetAlternate<A, OpenDrain>
    for Pin<P, N, Alternate<A, OpenDrain>>
{
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}

pub(crate) trait PinA<PIN, PER> {
    type A;
}

impl<PIN, PER> PinA<PIN, PER> for NoPin
where
    PIN: crate::Sealed,
    PER: crate::Sealed,
{
    type A = Const<0>;
}

macro_rules! pin {
    ( $($name:ident, <$Pin:ty, $I2C:ident> for [$(
            $(#[$attr:meta])* $PX:ident<$A:literal $(, $Otype:ident)?>,
        )*],)*) => {
            $(
                $(
                    $(#[$attr])*

                        impl<MODE> PinA<$Pin, pac::$I2C> for gpio::$PX<MODE> {
                            type A = Const<$A>;
                        }
                )*

                #[derive(Debug)]
                pub enum $name {
                    $(
                        $(#[$attr])*
                        $PX(gpio::$PX),
                    )*
                }
            )*
    };
}

// CAN pins

#[cfg(all(feature = "can", feature = "can1"))]
pub mod can1 {
    use super::*;
    use crate::can;

    pin! {
        Tx, <can::Tx, CAN1> for [
            PA12<9>,
            PD1<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PB9<8>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PB9<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PG1<9>,
        ],

        #[cfg(any(
            feature = "stm32f405",
            feature = "stm32f407",
            feature = "stm32f415",
            feature = "stm32f417",
            feature = "stm32f427",
            feature = "stm32f429",
            feature = "stm32f437",
            feature = "stm32f439",
            feature = "stm32f469",
            feature = "stm32f479"
        ))]
        PH13<9>,

        Rx, <can::Rx, CAN1> for [
            PA11<9>,
            PD0<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PB8<8>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PB8<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PG0<9>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI9<9>,
        ],
    }
}

#[cfg(all(feature = "can", feature = "can2"))]
pub mod can2 {
    use super::*;
    use crate::can;

    pin! {
        Tx, <can::Tx, CAN2> for [
            PB13<9>,
            PB6<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PG12<9>,
        ],

        Rx, <can::Rx, CAN2> for [
            PB12<9>,
            PB5<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PG11<9>,
        ],
    }
}

#[cfg(all(feature = "can", feature = "can3"))]
pub mod can3 {
    use super::*;
    use crate::can;

    pin! {
        Tx, <can::Tx, CAN3> for [PA15<11>, PB4<11>,],
        Rx, <can::Rx, CAN3> for [PA8<11>, PB3<11>,]
    }
}

// I2C pins

pub mod i2c1 {
    use super::*;

    pin! {
        Scl, <i2c::Scl, I2C1> for [PB6<4, OpenDrain>, PB8<4, OpenDrain>,],

        Sda, <i2c::Sda, I2C1> for [PB7<4, OpenDrain>, PB9<4, OpenDrain>,],
    }
}

pub mod i2c2 {
    use super::*;

    pin! {
        Sda, <i2c::Sda, I2C2> for [
            #[cfg(any(feature = "stm32f446"))]
            PB3<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB3<9, OpenDrain>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB9<9, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PB11<4, OpenDrain>,

            #[cfg(any(feature = "stm32f446"))]
            PC12<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF0<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH5<4, OpenDrain>,
        ],

        Scl, <i2c::Scl, I2C2> for [
            PB10<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF1<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH4<4, OpenDrain>,
        ],
    }
}

#[cfg(feature = "i2c3")]
pub mod i2c3 {
    use super::*;

    pin! {
        Scl, <i2c::Scl, I2C3> for [
            PA8<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH7<4, OpenDrain>,
        ],

        Sda, <i2c::Sda, I2C3> for [
            PC9<4, OpenDrain>,

            #[cfg(feature = "stm32f446")]
            PB4<4, OpenDrain>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB4<9, OpenDrain>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB8<9, OpenDrain>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH8<4, OpenDrain>,
        ],
    }
}

#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c1 {
    use super::*;
    pin! {
        Sda, <i2c::Sda, FMPI2C1> for [
            PB3<4, OpenDrain>,
            PB14<4, OpenDrain>,
            PC7<4, OpenDrain>,
            PD13<4, OpenDrain>,
            PD15<4, OpenDrain>,
            PF15<4, OpenDrain>,
        ],
        Scl, <i2c::Scl, FMPI2C1> for [
            PB10<9, OpenDrain>,
            PB15<4, OpenDrain>,
            PC6<4, OpenDrain>,
            PD12<4, OpenDrain>,
            PD14<4, OpenDrain>,
            PF14<4, OpenDrain>,
        ],
    }
}

// SPI pins

pub mod spi1 {
    use super::*;
    pin! {
        Sck, <spi::Sck,  SPI1> for [PA5<5>, PB3<5>,],

        Miso, <spi::Miso, SPI1> for [PA6<5>, PB4<5>,],

        Mosi, <spi::Mosi, SPI1> for [
            PA7<5>, PB5<5>,
        ],

        Nss, <spi::Nss,  SPI1> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PA4<5>,
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PA15<5>,
        ],
    }
}

pub mod spi2 {
    use super::*;
    pin! {
        Sck, <spi::Sck,  SPI2> for [
            PB10<5>,
            PB13<5>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD3<5>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PC7<5>,

            #[cfg(any(
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f469",
                feature = "stm32f479",
                feature = "stm32f446"
            ))]
            PA9<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI1<5>,
        ],

        Miso, <spi::Miso, SPI2> for [
            PB14<5>, PC2<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI2<5>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA12<5>,
        ],

        Mosi, <spi::Mosi, SPI2> for [
            PB15<5>,
            PC3<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI3<5>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA10<5>,

            #[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
            PC1<5>,

            #[cfg(feature = "stm32f446")]
            PC1<7>,
        ],

        Nss, <spi::Nss,  SPI2> for [
            PB9<5>,
            PB12<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI0<5>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA11<5>,

            #[cfg(feature = "stm32f446")]
            PB4<7>,

            #[cfg(feature = "stm32f446")]
            PD1<7>,
        ],
    }
}

#[cfg(feature = "spi3")]
pub mod spi3 {
    use super::*;
    pin! {
        Sck, <spi::Sck,  SPI3> for [
            PB3<6>,
            PC10<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB12<7>,
        ],

        Miso, <spi::Miso, SPI3> for [PB4<6>, PC11<6>,],

        Mosi, <spi::Mosi, SPI3> for [
            PB5<6>,
            PC12<6>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD6<5>,

            #[cfg(feature = "stm32f446")]
            PB0<7>,
            #[cfg(feature = "stm32f446")]
            PB2<7>,
            #[cfg(feature = "stm32f446")]
            PC1<5>,
            #[cfg(feature = "stm32f446")]
            PD0<6>,
        ],

        Nss, <spi::Nss, SPI3> for [PA4<6>, PA15<6>,],
    }
}

#[cfg(feature = "spi4")]
pub mod spi4 {
    use super::*;

    pin! {
        Sck, <spi::Sck,  SPI4> for [
            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PE2<5>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PE12<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB13<6>,

            #[cfg(feature = "stm32f446")]
            PG11<6>,
        ],

        Miso, <spi::Miso, SPI4> for [
            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PE5<5>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PE13<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA11<6>,

            #[cfg(feature = "stm32f446")]
            PG12<6>,

            #[cfg(feature = "stm32f446")]
            PD0<5>,
        ],

        Mosi, <spi::Mosi, SPI4> for [
            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PE6<5>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PE14<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA1<5>,

            #[cfg(feature = "stm32f446")]
            PG13<6>,
        ],
    }

    pin! {
        Nss, <spi::Nss,  SPI4> for [
            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB12<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE4<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE11<5>,
        ],
    }
}

#[cfg(feature = "spi5")]
pub mod spi5 {
    use super::*;

    pin! {
        Sck, <spi::Sck,  SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB0<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE2<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE12<6>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF7<5>,
            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH6<5>,
        ],

        Miso, <spi::Miso, SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA12<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE5<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE13<6>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF8<5>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH7<5>,
        ],

        Mosi, <spi::Mosi, SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA10<6>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB8<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE6<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE14<6>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF9<5>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF11<5>,
        ],

        Nss, <spi::Nss,  SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB1<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE4<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE11<6>,
        ],
    }
}

#[cfg(feature = "spi6")]
pub mod spi6 {
    use super::*;

    pin! {
        Sck, <spi::Sck,  SPI6> for [PG13<5>,],
        Miso, <spi::Miso, SPI6> for [PG12<5>,],
        Mosi, <spi::Mosi, SPI6> for [PG14<5>,],
    }
}

// SPI pins for I2S mode
pub mod i2s1 {
    use super::*;

    pin! {
        Ck, <i2s::Ck,  SPI1> for [
            PA5<5>,
            PB3<5>,
        ],
        Sd, <i2s::Sd, SPI1> for [
            PA7<5>,
            PB5<5>,
        ],

        Ws, <i2s::Ws,  SPI1> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PA4<5>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PA15<5>,
        ],

        Mck, <i2s::Mck,  SPI1> for [
            #[cfg(any(
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446",
            ))]
            PC4<5>,

            #[cfg(feature = "stm32f410")]
            PC7<6>,

            #[cfg(feature = "stm32f410")]
            PB10<6>,
        ],
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        Ck, <i2s::Ck,  SPI2> for [
            PB10<5>,
            PB13<5>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD3<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI1<5>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PC7<5>,

            #[cfg(any(
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PA9<5>,
        ],

        Sd, <i2s::Sd, SPI2> for [
            PB15<5>,
            PC3<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI3<5>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA10<5>,

            #[cfg(feature = "stm32f446")]
            PC1<7>,

            #[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
            PC1<5>,
        ],

        Ws, <i2s::Ws,  SPI2> for [
            PB9<5>,
            PB12<5>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PI0<5>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA11<5>,

            #[cfg(feature = "stm32f446")]
            PB4<7>,

            #[cfg(feature = "stm32f446")]
            PD1<7>,
        ],

        Mck, <i2s::Mck,  SPI2> for [
            PC6<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]

            PA3<5>,
            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA6<6>,
        ],
    }
}

#[cfg(feature = "spi3")]
pub mod i2s3 {
    use super::*;

    pin! {
        Ck, <i2s::Ck,  SPI3> for [
            PB3<6>,
            PC10<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB12<7>,
        ],

        Sd, <i2s::Sd, SPI3> for [
            PB5<6>,
            PC12<6>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD6<5>,

            #[cfg(feature = "stm32f446")]
            PB0<7>,
            #[cfg(feature = "stm32f446")]
            PB2<7>,
            #[cfg(feature = "stm32f446")]
            PC1<5>,
            #[cfg(feature = "stm32f446")]
            PD0<6>,
        ],

        Ws, <i2s::Ws,  SPI3> for [
            PA4<6>,
            PA15<6>,
        ],

        Mck, <i2s::Mck,  SPI3> for [
            PC7<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB10<6>,
        ],
    }
}

#[cfg(feature = "spi4")]
pub mod i2s4 {
    use super::*;

    pin! {
        Ck, <i2s::Ck, SPI4> for [
            PE2<5>,
            PE12<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB13<6>,

            #[cfg(feature = "stm32f446")]
            PG11<6>,
        ],

        Sd, <i2s::Sd, SPI4> for [
            PE6<5>,
            PE14<5>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA1<5>,

            #[cfg(feature = "stm32f446")]
            PG13<6>,
        ],

        Ws, <i2s::Ws, SPI4> for [
            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB12<6>,
            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE4<5>,
            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE11<5>,
        ],
    }
}

#[cfg(feature = "spi5")]
pub mod i2s5 {
    use super::*;

    pin! {
        Ck, <i2s::Ck, SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB0<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE2<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE12<6>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF7<5>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PH6<5>,
        ],

        Sd, <i2s::Sd, SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA10<6>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB8<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE6<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE14<6>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF9<5>,

            #[cfg(any(
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PF11<5>,
        ],

        Ws, <i2s::Ws, SPI5> for [
            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB1<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE4<6>,

            #[cfg(any(
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PE11<6>,
        ],
    }
}

#[cfg(feature = "spi6")]
pub mod i2s6 {
    use super::*;

    pin! {
        Ck, <i2s::Ck, SPI6> for [PG13<5>,],
        Sd, <i2s::Sd, SPI6> for [PG14<5>,],
    }
}

// Serial pins

pub mod usart1 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, USART1> for [
            PA9<7>,
            PB6<7>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA15<7>,
        ],

        Rx, <serial::RxPin, USART1> for [
            PA10<7>,
            PB7<7>,

            #[cfg(any(
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PB3<7>,
        ],
    }
}

pub mod usart2 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, USART2> for [
            PA2<7>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD5<7>,
        ],

        Rx, <serial::RxPin, USART2> for [
            PA3<7>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD6<7>,
        ],
    }
}

#[cfg(feature = "usart3")]
pub mod usart3 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, USART3> for [
            PB10<7>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PC10<7>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD8<7>,
        ],

        Rx, <serial::RxPin, USART3> for [
            PB11<7>,

            #[cfg(any(
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423",
                feature = "stm32f446"
            ))]
            PC5<7>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PC11<7>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f423",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PD9<7>,
        ],
    }
}

pub mod usart6 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, USART6> for [
            PC6<8>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA11<8>,

            #[cfg(feature = "gpiog")]
            PG14<8>,
        ],
        Rx, <serial::RxPin, USART6> for [
            PC7<8>,

            #[cfg(any(
                feature = "stm32f401",
                feature = "stm32f410",
                feature = "stm32f411",
                feature = "stm32f412",
                feature = "stm32f413",
                feature = "stm32f423"
            ))]
            PA12<8>,

            #[cfg(feature = "gpiog")]
            PG9<8>,
        ],
    }
}

#[cfg(feature = "uart4")]
pub mod uart4 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, UART4> for [
            PA0<8>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PC10<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA12<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PD1<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PD10<8>,
        ],

        Rx, <serial::RxPin, UART4> for [
            PA1<8>,

            #[cfg(any(
                feature = "stm32f405",
                feature = "stm32f407",
                feature = "stm32f415",
                feature = "stm32f417",
                feature = "stm32f427",
                feature = "stm32f429",
                feature = "stm32f437",
                feature = "stm32f439",
                feature = "stm32f446",
                feature = "stm32f469",
                feature = "stm32f479"
            ))]
            PC11<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA11<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PD0<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PC11<8>,
        ],
    }
}

#[cfg(feature = "uart5")]
pub mod uart5 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, UART5> for [
            PC12<8>,

            #[cfg(feature = "stm32f446")]
            PE8<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB6<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB9<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB13<11>,
        ],

        Rx, <serial::RxPin, UART5> for [
            PD2<8>,

            #[cfg(feature = "stm32f446")]
            PE7<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB5<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB8<11>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB12<11>,
        ],
    }
}

#[cfg(feature = "uart7")]
pub mod uart7 {
    use super::*;

    pin! {
        Tx,<serial::TxPin, UART7> for [
            #[cfg(feature = "gpioe")]
            PE8<8>,

            #[cfg(feature = "gpiof")]
            PF7<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA15<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB4<8>,
        ],

        Rx, <serial::RxPin, UART7> for [
            #[cfg(feature = "gpioe")]
            PE7<8>,

            #[cfg(feature = "gpiof")]
            PF6<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA8<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB3<8>,
        ],
    }
}

#[cfg(feature = "uart8")]
pub mod uart8 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, UART8> for [
            #[cfg(feature = "gpioe")]
            PE1<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PF9<8>,
        ],

        Rx, <serial::RxPin, UART8> for [
            #[cfg(feature = "gpioe")]
            PE0<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PF8<8>,
        ],
    }
}

#[cfg(feature = "uart9")]
pub mod uart9 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, UART9> for [PD15<11>, PG1<11>,],
        Rx, <serial::RxPin, UART9> for [PD14<11>, PG0<11>,],
    }
}

#[cfg(feature = "uart10")]
pub mod uart10 {
    use super::*;

    pin! {
        Tx, <serial::TxPin, UART10> for [PE3<11>, PG12<11>,],
        Rx, <serial::RxPin, UART10> for [PE2<11>, PG11<11>,],
    }
}
