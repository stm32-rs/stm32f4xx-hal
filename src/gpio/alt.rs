use super::{Alternate, NoPin, OpenDrain, PinMode};
use crate::gpio::{self, Edge, ExtiPin};
use crate::pac::EXTI;
use crate::syscfg::SysCfg;

macro_rules! pin {
    ( $($(#[$docs:meta])* <$name:ident> for $(no: $NoPin:ty,)? [$(
            $(#[$attr:meta])* $PX:ident<$A:literal $(, $Otype:ident)?>,
        )*],)*) => {
        $(
            #[derive(Debug)]
            $(#[$docs])*
            pub enum $name {
                $(
                    None($NoPin),
                )?

                $(
                    $(#[$attr])*
                    $PX(gpio::$PX<Alternate<$A $(, $Otype)?>>),
                )*
            }

            impl crate::Sealed for $name { }

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
                        Self::$PX(p)
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
        <Tx> for no:NoPin, [
            PA12<9>,
            PD1<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB9<8>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PB9<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG1<9>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH13<9>,
        ],

        <Rx> for no:NoPin, [
            PA11<9>,
            PD0<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB8<8>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PB8<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG0<9>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI9<9>,
        ],
    }
}

#[cfg(all(feature = "can", feature = "can2"))]
pub mod can2 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            PB13<9>,
            PB6<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG12<9>,
        ],

        <Rx> for no:NoPin, [
            PB12<9>,
            PB5<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG11<9>,
        ],
    }
}

#[cfg(all(feature = "can", feature = "can3"))]
pub mod can3 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [PA15<11>, PB4<11>,],
        <Rx> for no:NoPin, [PA8<11>, PB3<11>,],
    }
}

// I2C pins

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl> for [PB6<4, OpenDrain>, PB8<4, OpenDrain>,],

        <Sda> for [PB7<4, OpenDrain>, PB9<4, OpenDrain>,],
    }
}

pub mod i2c2 {
    use super::*;

    pin! {
        <Sda> for [
            #[cfg(feature = "gpio-f446")]
            PB3<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB3<9, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB9<9, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PB11<4, OpenDrain>,

            #[cfg(feature = "gpio-f446")]
            PC12<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PF0<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH5<4, OpenDrain>,
        ],

        <Scl> for [
            PB10<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PF1<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH4<4, OpenDrain>,
        ],
    }
}

#[cfg(feature = "i2c3")]
pub mod i2c3 {
    use super::*;

    pin! {
        <Scl> for [
            PA8<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH7<4, OpenDrain>,
        ],

        <Sda> for [
            PC9<4, OpenDrain>,

            #[cfg(feature = "gpio-f446")]
            PB4<4, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB4<9, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB8<9, OpenDrain>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH8<4, OpenDrain>,
        ],
    }
}

#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c1 {
    use super::*;
    pin! {
        <Sda> for [
            PB3<4, OpenDrain>,
            PB14<4, OpenDrain>,
            PC7<4, OpenDrain>,
            PD13<4, OpenDrain>,
            PD15<4, OpenDrain>,
            PF15<4, OpenDrain>,
        ],
        <Scl> for [
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
        <Sck> for no:NoPin, [PA5<5>, PB3<5>,],

        <Miso> for no:NoPin, [PA6<5>, PB4<5>,],

        <Mosi> for no:NoPin, [
            PA7<5>, PB5<5>,
        ],

        <Nss> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PA4<5>,
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PA15<5>,
        ],
    }
}

pub mod spi2 {
    use super::*;
    pin! {
        <Sck> for no:NoPin, [
            PB10<5>,
            PB13<5>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD3<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PC7<5>,

            #[cfg(any(
                feature = "gpio-f413",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PA9<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI1<5>,
        ],

        <Miso> for no:NoPin, [
            PB14<5>, PC2<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI2<5>,

            #[cfg(feature = "gpio-f413")]
            PA12<5>,
        ],

        <Mosi> for no:NoPin, [
            PB15<5>,
            PC3<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI3<5>,

            #[cfg(feature = "gpio-f413")]
            PA10<5>,

            #[cfg(feature = "gpio-f469")]
            PC1<5>,

            #[cfg(feature = "gpio-f446")]
            PC1<7>,
        ],

        <Nss> for [
            PB9<5>,
            PB12<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI0<5>,

            #[cfg(feature = "gpio-f413")]
            PA11<5>,

            #[cfg(feature = "gpio-f446")]
            PB4<7>,

            #[cfg(feature = "gpio-f446")]
            PD1<7>,
        ],
    }
}

#[cfg(feature = "spi3")]
pub mod spi3 {
    use super::*;
    pin! {
        <Sck> for no:NoPin, [
            PB3<6>,
            PC10<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB12<7>,
        ],

        <Miso> for no:NoPin, [PB4<6>, PC11<6>,],

        <Mosi> for no:NoPin, [
            PB5<6>,
            PC12<6>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD6<5>,

            #[cfg(feature = "gpio-f446")]
            PB0<7>,
            #[cfg(feature = "gpio-f446")]
            PB2<7>,
            #[cfg(feature = "gpio-f446")]
            PC1<5>,
            #[cfg(feature = "gpio-f446")]
            PD0<6>,
        ],

        <Nss> for [PA4<6>, PA15<6>,],
    }
}

#[cfg(feature = "spi4")]
pub mod spi4 {
    use super::*;

    pin! {
        <Sck> for no:NoPin, [
            PE2<5>,
            PE12<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB13<6>,

            #[cfg(feature = "gpio-f446")]
            PG11<6>,
        ],

        <Miso> for no:NoPin, [
            PE5<5>,
            PE13<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA11<6>,

            #[cfg(feature = "gpio-f446")]
            PG12<6>,

            #[cfg(feature = "gpio-f446")]
            PD0<5>,
        ],

        <Mosi> for no:NoPin, [
            PE6<5>,
            PE14<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA1<5>,

            #[cfg(feature = "gpio-f446")]
            PG13<6>,
        ],

        <Nss> for [
            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB12<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE4<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE11<5>,
        ],
    }
}

#[cfg(feature = "spi5")]
pub mod spi5 {
    use super::*;

    pin! {
        <Sck> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB0<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE2<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE12<6>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF7<5>,
            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH6<5>,
        ],

        <Miso> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA12<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE5<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE13<6>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF8<5>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH7<5>,
        ],

        <Mosi> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA10<6>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB8<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE6<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE14<6>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF9<5>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF11<5>,
        ],

        <Nss> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB1<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE4<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE11<6>,
        ],
    }
}

#[cfg(feature = "spi6")]
pub mod spi6 {
    use super::*;

    pin! {
        <Sck> for no:NoPin, [PG13<5>,],
        <Miso> for no:NoPin, [PG12<5>,],
        <Mosi> for no:NoPin, [PG14<5>,],
        <Nss> for [],
    }
}

// SPI pins for I2S mode
pub mod i2s1 {
    use super::*;

    pin! {
        <Ck> for [
            PA5<5>,
            PB3<5>,
        ],
        <Sd> for [
            PA7<5>,
            PB5<5>,
        ],

        <Ws> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PA4<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PA15<5>,
        ],

        <Mck> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446",
            ))]
            PC4<5>,

            #[cfg(feature = "gpio-f410")]
            PC7<6>,

            #[cfg(feature = "gpio-f410")]
            PB10<6>,
        ],
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        <Ck> for [
            PB10<5>,
            PB13<5>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD3<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI1<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PC7<5>,

            #[cfg(any(
                feature = "gpio-f413",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PA9<5>,
        ],

        <Sd> for [
            PB15<5>,
            PC3<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI3<5>,

            #[cfg(feature = "gpio-f413")]
            PA10<5>,

            #[cfg(feature = "gpio-f446")]
            PC1<7>,

            #[cfg(feature = "gpio-f469")]
            PC1<5>,
        ],

        <Ws> for [
            PB9<5>,
            PB12<5>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PI0<5>,

            #[cfg(feature = "gpio-f413")]
            PA11<5>,

            #[cfg(feature = "gpio-f446")]
            PB4<7>,

            #[cfg(feature = "gpio-f446")]
            PD1<7>,
        ],

        <Mck> for no:NoPin, [
            PC6<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA3<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA6<6>,
        ],
    }
}

#[cfg(feature = "spi3")]
pub mod i2s3 {
    use super::*;

    pin! {
        <Ck> for [
            PB3<6>,
            PC10<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB12<7>,
        ],

        <Sd> for [
            PB5<6>,
            PC12<6>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD6<5>,

            #[cfg(feature = "gpio-f446")]
            PB0<7>,
            #[cfg(feature = "gpio-f446")]
            PB2<7>,
            #[cfg(feature = "gpio-f446")]
            PC1<5>,
            #[cfg(feature = "gpio-f446")]
            PD0<6>,
        ],

        <Ws> for [
            PA4<6>,
            PA15<6>,
        ],

        <Mck> for no:NoPin, [
            PC7<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB10<6>,
        ],
    }
}

#[cfg(feature = "spi4")]
pub mod i2s4 {
    use super::*;

    pin! {
        <Ck> for [
            PE2<5>,
            PE12<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB13<6>,

            #[cfg(feature = "gpio-f446")]
            PG11<6>,
        ],

        <Sd> for [
            PE6<5>,
            PE14<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA1<5>,

            #[cfg(feature = "gpio-f446")]
            PG13<6>,
        ],

        <Mck> for no:NoPin, [ ],

        <Ws> for [
            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB12<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE4<5>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE11<5>,
        ],
    }
}

#[cfg(feature = "spi5")]
pub mod i2s5 {
    use super::*;

    pin! {
        <Ck> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB0<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE2<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE12<6>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF7<5>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PH6<5>,
        ],

        <Sd> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA10<6>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB8<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE6<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE14<6>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF9<5>,

            #[cfg(any(
                feature = "gpio-f427",
                feature = "gpio-f469",
            ))]
            PF11<5>,
        ],

        <Mck> for no:NoPin, [ ],

        <Ws> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB1<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE4<6>,

            #[cfg(any(
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PE11<6>,
        ],
    }
}

#[cfg(feature = "spi6")]
pub mod i2s6 {
    use super::*;

    pin! {
        <Ck> for [PG13<5>,],
        <Sd> for [PG14<5>,],
    }
}

// Serial pins

pub mod usart1 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            PA9<7>,
            PB6<7>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA15<7>,
        ],

        <Rx> for no:NoPin, [
            PA10<7>,
            PB7<7>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PB3<7>,
        ],
    }
}

pub mod usart2 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            PA2<7>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f417",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD5<7>,
        ],

        <Rx> for no:NoPin, [
            PA3<7>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f417",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD6<7>,
        ],
    }
}

#[cfg(feature = "usart3")]
pub mod usart3 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            PB10<7>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PC10<7>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD8<7>,
        ],

        <Rx> for no:NoPin, [
            PB11<7>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PC5<7>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PC11<7>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PD9<7>,
        ],
    }
}

pub mod usart6 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            PC6<8>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
            ))]
            PA11<8>,

            #[cfg(feature = "gpiog")]
            PG14<8>,
        ],
        <Rx> for no:NoPin, [
            PC7<8>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
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
        <Tx> for no:NoPin, [
            PA0<8>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PC10<8>,

            #[cfg(feature = "gpio-f413")]
            PA12<11>,

            #[cfg(feature = "gpio-f413")]
            PD1<11>,

            #[cfg(feature = "gpio-f413")]
            PD10<8>,
        ],

        <Rx> for no:NoPin, [
            PA1<8>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PC11<8>,

            #[cfg(feature = "gpio-f413")]
            PA11<11>,

            #[cfg(feature = "gpio-f413")]
            PD0<11>,

            #[cfg(feature = "gpio-f413")]
            PC11<8>,
        ],
    }
}

#[cfg(feature = "uart5")]
pub mod uart5 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            PC12<8>,

            #[cfg(feature = "gpio-f446")]
            PE8<8>,

            #[cfg(feature = "gpio-f413")]
            PB6<11>,

            #[cfg(feature = "gpio-f413")]
            PB9<11>,

            #[cfg(feature = "gpio-f413")]
            PB13<11>,
        ],

        <Rx> for no:NoPin, [
            PD2<8>,

            #[cfg(feature = "gpio-f446")]
            PE7<8>,

            #[cfg(feature = "gpio-f413")]
            PB5<11>,

            #[cfg(feature = "gpio-f413")]
            PB8<11>,

            #[cfg(feature = "gpio-f413")]
            PB12<11>,
        ],
    }
}

#[cfg(feature = "uart7")]
pub mod uart7 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PE8<8>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PF7<8>,

            #[cfg(feature = "gpio-f413")]
            PA15<8>,

            #[cfg(feature = "gpio-f413")]
            PB4<8>,
        ],

        <Rx> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PE7<8>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PF6<8>,

            #[cfg(feature = "gpio-f413")]
            PA8<8>,

            #[cfg(feature = "gpio-f413")]
            PB3<8>,
        ],
    }
}

#[cfg(feature = "uart8")]
pub mod uart8 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PE1<8>,

            #[cfg(feature = "gpio-f413")]
            PF9<8>,
        ],

        <Rx> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469",
            ))]
            PE0<8>,

            #[cfg(feature = "gpio-f413")]
            PF8<8>,
        ],
    }
}

#[cfg(feature = "uart9")]
pub mod uart9 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [PD15<11>, PG1<11>,],
        <Rx> for no:NoPin, [PD14<11>, PG0<11>,],
    }
}

#[cfg(feature = "uart10")]
pub mod uart10 {
    use super::*;

    pin! {
        <Tx> for no:NoPin, [PE3<11>, PG12<11>,],
        <Rx> for no:NoPin, [PE2<11>, PG11<11>,],
    }
}

#[cfg(feature = "sdio")]
pub mod sdio {
    use super::*;

    pin! {
        <Clk> for [
            PC12<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PB15<12>,
        ],
        <Cmd> for [
            PD2<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PA6<12>,
        ],
        <D0> for [
            PC8<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PB4<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB6<12>,

            #[cfg(feature = "gpio-f411")]
            PB7<12>,
        ],
        <D1> for [
            PC9<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PA8<12>,
        ],
        <D2> for [
            PC10<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PA9<12>,
        ],
        <D3> for [
            PC11<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PB5<12>,
        ],
        <D4> for [
            PB8<12>,
        ],
        <D5> for [
            PB9<12>,
        ],
        <D6> for [
            PC6<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PB14<12>,
        ],
        <D7> for [
            PC7<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f411"))]
            PB10<12>,
        ],
    }
}

/// Pins available on all STM32F4 models that have an FSMC/FMC
#[cfg(any(feature = "fmc", feature = "fsmc"))]
pub mod fsmc {
    use super::*;

    // All FSMC/FMC pins use 12
    pin! {
        /// A pin that can be used for data bus 0
        <D0> for [
            PD14<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB14<10>,
        ],

        /// A pin that can be used for data bus 1
        <D1> for [
            PD15<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC6<10>,
        ],

        /// A pin that can be used for data bus 2
        <D2> for [
            PD0<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC11<10>,
        ],

        /// A pin that can be used for data bus 3
        <D3> for [
            PD1<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC12<10>,
        ],

        /// A pin that can be used for data bus 4
        <D4> for [
            PE7<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA2<12>,
        ],

        /// A pin that can be used for data bus 5
        <D5> for [
            PE8<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA3<12>,
        ],

        /// A pin that can be used for data bus 6
        <D6> for [
            PE9<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA4<12>,
        ],

        /// A pin that can be used for data bus 7
        <D7> for [
            PE10<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA5<12>,
        ],

        /// A pin that can be used for data bus 8
        <D8> for [
            PE11<12>,
        ],

        /// A pin that can be used for data bus 9
        <D9> for [
            PE12<12>,
        ],

        /// A pin that can be used for data bus 10
        <D10> for [
            PE13<12>,
        ],

        /// A pin that can be used for data bus 11
        <D11> for [
            PE14<12>,
        ],

        /// A pin that can be used for data bus 12
        <D12> for [
            PE15<12>,
        ],

        /// A pin that can be used for data bus 13
        <D13> for [
            PD8<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB12<12>,
        ],

        /// A pin that can be used for data bus 14
        <D14> for [
            PD9<12>,
        ],

        /// A pin that can be used for data bus 15
        <D15> for [
            PD10<12>,
        ],

        /// A pin that can be used for the output enable (read enable, NOE) signal
        <ReadEnable> for [
            PD4<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC5<12>,
        ],

        /// A pin that can be used for the write enable (NOE) signal
        <WriteEnable> for [
            PD5<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC2<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PD2<10>,
        ],

        /// A pin that can be used as one bit of the memory address
        ///
        /// This is used to switch between data and command mode.
        <Address> for [
            PD11<12>,
            PD12<12>,
            PD13<12>,
            PE2<12>,
            PE3<12>,
            PE4<12>,
            PE5<12>,
            PE6<12>,
            PF0<12>,
            PF1<12>,
            PF2<12>,
            PF3<12>,
            PF4<12>,
            PF5<12>,
            PF12<12>,
            PF13<12>,
            PF14<12>,
            PF15<12>,
            PG0<12>,
            PG1<12>,
            PG2<12>,
            PG3<12>,
            PG4<12>,
            PG5<12>,
            PG13<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC3<12>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 1
        <ChipSelect1> for [
            PD7<12>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 2
        <ChipSelect2> for [
            PG9<12>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 3
        <ChipSelect3> for [
            PG10<12>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 4
        <ChipSelect4> for [
            PG12<12>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC4<12>,
        ],
    }

    // PG14<Alternate<12> can be used as address 25 (A25), but that pin is not available here.
    // Because external addresses are in units of 16 bits, external address line 25 can never
    // be high. The internal memory address would overflow into the next sub-bank.
}

#[cfg(feature = "otg-fs")]
pub mod otg_fs {
    use super::*;

    pin! {
        <Dm> for [
            PA11<10>,
        ],
        <Dp> for [
            PA12<10>,
        ],
    }
}

#[cfg(feature = "otg-hs")]
pub mod otg_hs {
    use super::*;

    pin! {
        <Dm> for [
            PB14<12>,
        ],
        <Dp> for [
            PB15<12>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <C1> for [
            PA8<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE9<1>,
        ],
        <C2> for [
            PA9<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE11<1>,
        ],
        <C3> for [
            PA10<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE13<1>,
        ],
        <C4> for [
            PA11<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE14<1>,
        ],
        <NC1> for [
            PA7<1>,
            PB13<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE8<1>,
        ],
        <NC2> for [
            PB0<1>,
            PB14<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE10<1>,
        ],
        <NC3> for [
            PB1<1>,
            PB15<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE12<1>,
        ],
        <Etr> for [
            PA12<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE7<1>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF10<1>,
        ],
        <BkIn> for [
            PA6<1>,
            PB12<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE15<1>,
        ],
    }

    use crate::pac::TIM1;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM1 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM1 {
        type Pin = C2;
    }
    impl ChannelPin<2> for TIM1 {
        type Pin = C3;
    }
    impl ChannelPin<3> for TIM1 {
        type Pin = C4;
    }
    impl ChannelPin<0, true> for TIM1 {
        type Pin = NC1;
    }
    impl ChannelPin<1, true> for TIM1 {
        type Pin = NC2;
    }
    impl ChannelPin<2, true> for TIM1 {
        type Pin = NC3;
    }
}

#[cfg(not(feature = "gpio-f410"))]
pub mod tim2 {
    use super::*;

    pin! {
        <C1> for [
            PA0<1>,
            PA5<1>,
            PA15<1>,

            #[cfg(feature = "gpio-f446")]
            PB8<1>,
        ],
        <C2> for [
            PA1<1>,
            PB3<1>,

            #[cfg(feature = "gpio-f446")]
            PB9<1>,
        ],
        <C3> for [
            PA2<1>,
            PB10<1>,
        ],
        <C4> for [
            PA3<1>,
            PB11<1>,

            #[cfg(feature = "gpio-f446")]
            PB2<1>,
        ],
        <Etr> for [
            PA0<1>,
            PA5<1>,
            PA15<1>,

            #[cfg(feature = "gpio-f446")]
            PB8<1>,
        ],
    }

    use crate::pac::TIM2;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM2 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM2 {
        type Pin = C2;
    }
    impl ChannelPin<2> for TIM2 {
        type Pin = C3;
    }
    impl ChannelPin<3> for TIM2 {
        type Pin = C4;
    }
}

#[cfg(not(feature = "gpio-f410"))]
pub mod tim3 {
    use super::*;

    pin! {
        <C1> for [
            PA6<2>,
            PB4<2>,
            PC6<2>,
        ],
        <C2> for [
            PA7<2>,
            PB5<2>,
            PC7<2>,
        ],
        <C3> for [
            PB0<2>,
            PC8<2>,
        ],
        <C4> for [
            PB1<2>,
            PC9<2>,
        ],
        <Etr> for [
            PD2<2>,
        ],
    }

    use crate::pac::TIM3;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM3 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM3 {
        type Pin = C2;
    }
    impl ChannelPin<2> for TIM3 {
        type Pin = C3;
    }
    impl ChannelPin<3> for TIM3 {
        type Pin = C4;
    }
}

#[cfg(not(feature = "gpio-f410"))]
pub mod tim4 {
    use super::*;

    pin! {
        <C1> for [
            PB6<2>,
            PD12<2>,
        ],
        <C2> for [
            PB7<2>,
            PD13<2>,
        ],
        <C3> for [
            PB8<2>,
            PD14<2>,
        ],
        <C4> for [
            PB9<2>,
            PD15<2>,
        ],
        <Etr> for [
            PE0<2>,
        ],
    }

    use crate::pac::TIM4;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM4 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM4 {
        type Pin = C2;
    }
    impl ChannelPin<2> for TIM4 {
        type Pin = C3;
    }
    impl ChannelPin<3> for TIM4 {
        type Pin = C4;
    }
}

pub mod tim5 {
    use super::*;

    pin! {
        <C1> for [
            PA0<2>,

            #[cfg(feature = "gpio-f410")]
            PB12<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF3<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH10<2>,
        ],
        <C2> for [
            PA1<2>,

            #[cfg(feature = "gpio-f410")]
            PC10<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF4<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH11<2>,
        ],
        <C3> for [
            PA2<2>,

            #[cfg(feature = "gpio-f410")]
            PC11<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF5<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH12<2>,
        ],
        <C4> for [
            PA3<2>,

            #[cfg(feature = "gpio-f410")]
            PB11<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF10<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI0<2>,
        ],
    }

    use crate::pac::TIM5;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM5 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM5 {
        type Pin = C2;
    }
    impl ChannelPin<2> for TIM5 {
        type Pin = C3;
    }
    impl ChannelPin<3> for TIM5 {
        type Pin = C4;
    }
}

#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
pub mod tim8 {
    use super::*;

    pin! {
        <C1> for [
            PC6<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI5<3>,
        ],
        <C2> for [
            PC7<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI6<3>,
        ],
        <C3> for [
            PC8<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI7<3>,
        ],
        <C4> for [
            PC9<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI2<3>,
        ],
        <NC1> for [
            PA5<3>,
            PA7<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH13<3>,
        ],
        <NC2> for [
            PB0<3>,
            PB14<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH14<3>,
        ],
        <NC3> for [
            PB1<3>,
            PB15<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH15<3>,
        ],
        <Etr> for [
            PA0<3>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF11<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI3<3>,
        ],
        <BkIn> for [
            PA6<3>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF12<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI4<3>,
        ],
    }

    use crate::pac::TIM8;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM8 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM8 {
        type Pin = C2;
    }
    impl ChannelPin<2> for TIM8 {
        type Pin = C3;
    }
    impl ChannelPin<3> for TIM8 {
        type Pin = C4;
    }
    impl ChannelPin<0, true> for TIM8 {
        type Pin = NC1;
    }
    impl ChannelPin<1, true> for TIM8 {
        type Pin = NC2;
    }
    impl ChannelPin<2, true> for TIM8 {
        type Pin = NC3;
    }
}

pub mod tim9 {
    use super::*;

    pin! {
        <C1> for [
            PA2<3>,

            #[cfg(not(feature = "gpio-f410"))]
            PE5<3>,

            #[cfg(feature = "gpio-f410")]
            PC4<3>,
        ],
        <C2> for [
            PA3<3>,

            #[cfg(not(feature = "gpio-f410"))]
            PE6<3>,

            #[cfg(feature = "gpio-f410")]
            PC5<3>,
        ],
    }

    use crate::pac::TIM9;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM9 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM9 {
        type Pin = C2;
    }
}

#[cfg(not(feature = "gpio-f410"))]
pub mod tim10 {
    use super::*;

    pin! {
        <C1> for [
            PB8<3>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF6<3>,
        ],
    }

    use crate::pac::TIM10;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM10 {
        type Pin = C1;
    }
}

pub mod tim11 {
    use super::*;

    pin! {
        <C1> for [
            PB9<3>,

            #[cfg(feature = "gpio-f410")]
            PC12<3>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF7<3>,
        ],
    }

    use crate::pac::TIM11;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM11 {
        type Pin = C1;
    }
}

#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
pub mod tim12 {
    use super::*;

    pin! {
        <C1> for [
            PB14<9>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH6<9>,
        ],
        <C2> for [
            PB15<9>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH9<9>,
        ],
    }

    use crate::pac::TIM12;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM12 {
        type Pin = C1;
    }
    impl ChannelPin<1> for TIM12 {
        type Pin = C2;
    }
}

#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
pub mod tim13 {
    use super::*;

    pin! {
        <C1> for [
            PA6<9>,
            PF8<9>,
        ],
    }

    use crate::pac::TIM13;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM13 {
        type Pin = C1;
    }
}

#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
pub mod tim14 {
    use super::*;

    pin! {
        <C1> for [
            PA7<9>,
            PF9<9>,
        ],
    }

    use crate::pac::TIM14;
    use crate::timer::ChannelPin;

    impl ChannelPin<0> for TIM14 {
        type Pin = C1;
    }
}
