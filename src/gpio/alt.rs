use super::{Alternate, NoPin, OpenDrain, PinMode};
use crate::gpio::{self, Edge, ExtiPin};
use crate::pac::EXTI;
use crate::syscfg::SysCfg;

macro_rules! pin {
    ( $(<$name:ident, $I2C:ident> for $(no: $NoPin:ty,)? [$(
            $(#[$attr:meta])* $PX:ident<$A:literal $(, $Otype:ident)?>,
        )*],)*) => {
            $(
                #[derive(Debug)]
                pub enum $name {
                    $(
                        None($NoPin),
                    )?

                    $(
                        $(#[$attr])*
                        $PX(gpio::$PX<Alternate<$A $(, $Otype)?>>),
                    )*
                }

                #[allow(unreachable_patterns)]
                impl $name {
                    pub fn is_high(&self) -> bool {
                        !self.is_low()
                    }
                    pub fn is_low(&self) -> bool {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.is_low(),
                            )*
                            _ => false,
                        }
                    }
                }
                #[allow(unreachable_patterns)]
                impl ExtiPin for $name {
                    fn make_interrupt_source(&mut self, _syscfg: &mut SysCfg) {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.make_interrupt_source(_syscfg),
                            )*
                            _ => {},
                        }

                    }

                    fn trigger_on_edge(&mut self, _exti: &mut EXTI, _level: Edge) {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.trigger_on_edge(_exti, _level),
                            )*
                            _ => {},
                        }
                    }

                    fn enable_interrupt(&mut self, _exti: &mut EXTI) {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.enable_interrupt(_exti),
                            )*
                            _ => {},
                        }
                    }
                    fn disable_interrupt(&mut self, _exti: &mut EXTI) {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.disable_interrupt(_exti),
                            )*
                            _ => {},
                        }
                    }
                    fn clear_interrupt_pending_bit(&mut self) {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.clear_interrupt_pending_bit(),
                            )*
                            _ => {},
                        }
                    }
                    fn check_interrupt(&self) -> bool {
                        match self {
                            $(
                                $(#[$attr])*
                                Self::$PX(p) => p.check_interrupt(),
                            )*
                            _ => false,
                        }
                    }
                }

                $(
                    impl From<$NoPin> for $name {
                        fn from(p: $NoPin) -> Self {
                            Self::None(p)
                        }
                    }

                    #[allow(irrefutable_let_patterns)]
                    impl TryFrom<$name> for $NoPin {
                        type Error = ();

                        fn try_from(a: $name) -> Result<Self, Self::Error> {
                            if let $name::None(p) = a {
                                Ok(p)
                            } else {
                                Err(())
                            }
                        }
                    }
                )?

                $(
                    $(#[$attr])*
                    impl From<gpio::$PX> for $name {
                        fn from(p: gpio::$PX) -> Self {
                            Self::$PX(p.into_mode())
                        }
                    }

                    $(#[$attr])*
                    impl From<gpio::$PX<Alternate<$A $(, $Otype)?>>> for $name {
                        fn from(p: gpio::$PX<Alternate<$A $(, $Otype)?>>) -> Self {
                            Self::$PX(p.into_mode())
                        }
                    }

                    $(#[$attr])*
                    #[allow(irrefutable_let_patterns)]
                    impl<MODE: PinMode> TryFrom<$name> for gpio::$PX<MODE> {
                        type Error = ();

                        fn try_from(a: $name) -> Result<Self, Self::Error> {
                            if let $name::$PX(p) = a {
                                Ok(p.into_mode())
                            } else {
                                Err(())
                            }
                        }
                    }
                )*
            )*
    };
}

// CAN pins

#[cfg(all(feature = "can", feature = "can1"))]
pub mod can1 {
    use super::*;

    pin! {
        <Tx, CAN1> for no:NoPin, [
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
        ],

        <Rx, CAN1> for no:NoPin, [
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

    pin! {
        <Tx, CAN2> for no:NoPin, [
            PB13<9>,
            PB6<9>,

            #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
            PG12<9>,
        ],

        <Rx, CAN2> for no:NoPin, [
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

    pin! {
        <Tx, CAN3> for no:NoPin, [PA15<11>, PB4<11>,],
        <Rx, CAN3> for no:NoPin, [PA8<11>, PB3<11>,],
    }
}

// I2C pins

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, I2C1> for [PB6<4, OpenDrain>, PB8<4, OpenDrain>,],

        <Sda, I2C1> for [PB7<4, OpenDrain>, PB9<4, OpenDrain>,],
    }
}

pub mod i2c2 {
    use super::*;

    pin! {
        <Sda, I2C2> for [
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

        <Scl, I2C2> for [
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
        <Scl, I2C3> for [
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

        <Sda, I2C3> for [
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
        <Sda, FMPI2C1> for [
            PB3<4, OpenDrain>,
            PB14<4, OpenDrain>,
            PC7<4, OpenDrain>,
            PD13<4, OpenDrain>,
            PD15<4, OpenDrain>,
            PF15<4, OpenDrain>,
        ],
        <Scl, FMPI2C1> for [
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
        <Sck,  SPI1> for no:NoPin, [PA5<5>, PB3<5>,],

        <Miso, SPI1> for no:NoPin, [PA6<5>, PB4<5>,],

        <Mosi, SPI1> for no:NoPin, [
            PA7<5>, PB5<5>,
        ],

        <Nss,  SPI1> for [
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
        <Sck,  SPI2> for no:NoPin, [
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

        <Miso, SPI2> for no:NoPin, [
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

        <Mosi, SPI2> for no:NoPin, [
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

        <Nss,  SPI2> for [
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
        <Sck,  SPI3> for no:NoPin, [
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

        <Miso, SPI3> for no:NoPin, [PB4<6>, PC11<6>,],

        <Mosi, SPI3> for no:NoPin, [
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

        <Nss, SPI3> for [PA4<6>, PA15<6>,],
    }
}

#[cfg(feature = "spi4")]
pub mod spi4 {
    use super::*;

    pin! {
        <Sck,  SPI4> for no:NoPin, [
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

        <Miso, SPI4> for no:NoPin, [
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

        <Mosi, SPI4> for no:NoPin, [
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

        <Nss,  SPI4> for [
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
        <Sck,  SPI5> for no:NoPin, [
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

        <Miso, SPI5> for no:NoPin, [
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

        <Mosi, SPI5> for no:NoPin, [
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

        <Nss, SPI5> for [
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
        <Sck,  SPI6> for no:NoPin, [PG13<5>,],
        <Miso, SPI6> for no:NoPin, [PG12<5>,],
        <Mosi, SPI6> for no:NoPin, [PG14<5>,],
        <Nss, SPI6> for [],
    }
}

// SPI pins for I2S mode
pub mod i2s1 {
    use super::*;

    pin! {
        <Ck,  SPI1> for [
            PA5<5>,
            PB3<5>,
        ],
        <Sd, SPI1> for [
            PA7<5>,
            PB5<5>,
        ],

        <Ws, SPI1> for [
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

        <Mck, SPI1> for no:NoPin, [
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
        <Ck, SPI2> for [
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

        <Sd, SPI2> for [
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

        <Ws, SPI2> for [
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

        <Mck, SPI2> for no:NoPin, [
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
        <Ck, SPI3> for [
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

        <Sd, SPI3> for [
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

        <Ws,  SPI3> for [
            PA4<6>,
            PA15<6>,
        ],

        <Mck,  SPI3> for no:NoPin, [
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
        <Ck, SPI4> for [
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

        <Sd, SPI4> for [
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

        <Mck, SPI4> for no:NoPin, [ ],

        <Ws, SPI4> for [
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
        <Ck, SPI5> for [
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

        <Sd, SPI5> for [
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

        <Mck, SPI4> for no:NoPin, [ ],

        <Ws, SPI5> for [
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
        <Ck, SPI6> for [PG13<5>,],
        <Sd, SPI6> for [PG14<5>,],
    }
}

// Serial pins

pub mod usart1 {
    use super::*;

    pin! {
        <Tx, USART1> for no:NoPin, [
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

        <Rx, USART1> for no:NoPin, [
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
        <Tx, USART2> for no:NoPin, [
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

        <Rx, USART2> for no:NoPin, [
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
        <Tx, USART3> for no:NoPin, [
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

        <Rx, USART3> for no:NoPin, [
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
        <Tx, USART6> for no:NoPin, [
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
        <Rx, USART6> for no:NoPin, [
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
        <Tx, UART4> for no:NoPin, [
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

        <Rx, UART4> for no:NoPin, [
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
        <Tx, UART5> for no:NoPin, [
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

        <Rx, UART5> for no:NoPin, [
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
        <Tx, UART7> for no:NoPin, [
            #[cfg(feature = "gpioe")]
            PE8<8>,

            #[cfg(feature = "gpiof")]
            PF7<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PA15<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PB4<8>,
        ],

        <Rx, UART7> for no:NoPin, [
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
        <Tx, UART8> for no:NoPin, [
            #[cfg(feature = "gpioe")]
            PE1<8>,

            #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
            PF9<8>,
        ],

        <Rx, UART8> for no:NoPin, [
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
        <Tx, UART9> for no:NoPin, [PD15<11>, PG1<11>,],
        <Rx, UART9> for no:NoPin, [PD14<11>, PG0<11>,],
    }
}

#[cfg(feature = "uart10")]
pub mod uart10 {
    use super::*;

    pin! {
        <Tx, UART10> for no:NoPin, [PE3<11>, PG12<11>,],
        <Rx, UART10> for no:NoPin, [PE2<11>, PG11<11>,],
    }
}
