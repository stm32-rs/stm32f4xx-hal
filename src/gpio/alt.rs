#[cfg(feature = "c0")]
mod c0;
#[cfg(feature = "c0")]
pub use c0::*;

#[cfg(feature = "f0")]
mod f0;
#[cfg(feature = "f0")]
pub use f0::*;

#[cfg(feature = "f2")]
mod f2;
#[cfg(feature = "f2")]
pub use f2::*;

#[cfg(feature = "f3")]
mod f3;
#[cfg(feature = "f3")]
pub use f3::*;

#[cfg(feature = "f4")]
mod f4;
#[cfg(feature = "f4")]
pub use f4::*;

#[cfg(feature = "f7")]
mod f7;
#[cfg(feature = "f7")]
pub use f7::*;

#[cfg(feature = "g0")]
mod g0;
#[cfg(feature = "g0")]
pub use g0::*;

#[cfg(feature = "g4")]
mod g4;
#[cfg(feature = "g4")]
pub use g4::*;

#[cfg(feature = "h7")]
mod h7;
#[cfg(feature = "h7")]
pub use h7::*;

#[cfg(feature = "l0")]
mod l0;
#[cfg(feature = "l0")]
pub use l0::*;

#[cfg(feature = "l1")]
mod l1;
#[cfg(feature = "l1")]
pub use l1::*;

#[cfg(feature = "l4x")]
mod l4;
#[cfg(feature = "l4x")]
pub use l4::*;

#[cfg(feature = "l4p")]
mod l4p;
#[cfg(feature = "l4p")]
pub use l4p::*;

#[cfg(feature = "l5")]
mod l5;
#[cfg(feature = "l5")]
pub use l5::*;

#[cfg(feature = "u5")]
mod u5;
#[cfg(feature = "u5")]
pub use u5::*;

#[cfg(feature = "wl")]
mod wl;
#[cfg(feature = "wl")]
pub use wl::*;

#[cfg(feature = "wb")]
mod wb;
#[cfg(feature = "wb")]
pub use wb::*;

macro_rules! extipin {
    ($( $(#[$attr:meta])* $PX:ident,)*) => {
        fn make_interrupt_source(&mut self, _syscfg: &mut $crate::syscfg::SysCfg) {
            match self {
                $(
                    $(#[$attr])*
                    Self::$PX(p) => p.make_interrupt_source(_syscfg),
                )*
                _ => {},
            }

        }

        fn trigger_on_edge(&mut self, _exti: &mut $crate::pac::EXTI, _level: $crate::gpio::Edge) {
            match self {
                $(
                    $(#[$attr])*
                    Self::$PX(p) => p.trigger_on_edge(_exti, _level),
                )*
                _ => {},
            }
        }

        fn enable_interrupt(&mut self, _exti: &mut $crate::pac::EXTI) {
            match self {
                $(
                    $(#[$attr])*
                    Self::$PX(p) => p.enable_interrupt(_exti),
                )*
                _ => {},
            }
        }
        fn disable_interrupt(&mut self, _exti: &mut $crate::pac::EXTI) {
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
    };
}
use extipin;

#[allow(unused)]
macro_rules! analog {
    ( $($(#[$docs:meta])* <$name:ident> for $(no: $NoPin:ident,)? [$(
        $(#[$attr:meta])* $PX:ident<$A:literal>,
    )*],)*) => {
        $(
            #[derive(Debug)]
            $(#[$docs])*
            pub enum $name {
                $(
                    None($NoPin<$Otype>),
                )?

                $(
                    $(#[$attr])*
                    $PX(gpio::$PX<Analog>),
                )*
            }

            impl crate::Sealed for $name { }

            $(
                impl From<$NoPin<$Otype>> for $name {
                    fn from(p: $NoPin<$Otype>) -> Self {
                        Self::None(p)
                    }
                }
            )?

            $(
                $(#[$attr])*
                impl<MODE: PinMode> From<gpio::$PX<MODE>> for $name
                {
                    fn from(p: gpio::$PX<MODE>) -> Self {
                        Self::$PX(p.into_mode())
                    }
                }

                $(#[$attr])*
                #[allow(irrefutable_let_patterns)]
                impl<MODE> TryFrom<$name> for gpio::$PX<MODE>
                where
                    MODE: PinMode,
                {
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
#[allow(unused)]
use analog;

macro_rules! pin {
    ( $($(#[$docs:meta])* <$name:ident, $Otype:ident> for $(no: $NoPin:ident,)? [$(
        $(#[$attr:meta])* $PX:ident<$A:literal>,
    )*],)*) => {
        $(
            #[derive(Debug)]
            $(#[$docs])*
            pub enum $name {
                $(
                    None($NoPin<$Otype>),
                )?

                $(
                    $(#[$attr])*
                    $PX(gpio::$PX<$crate::gpio::Alternate<$A, $Otype>>),
                )*
            }

            impl crate::Sealed for $name { }

            #[allow(unreachable_patterns)]
            impl $crate::gpio::ReadPin for $name {
                fn is_low(&self) -> bool {
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
            impl $crate::gpio::PinSpeed for $name {
                fn set_speed(&mut self, _speed: $crate::gpio::Speed) {
                    match self {
                        $(
                            $(#[$attr])*
                            Self::$PX(p) => p.set_speed(_speed),
                        )*
                        _ => {}
                    }
                }
            }

            #[allow(unreachable_patterns)]
            impl $crate::gpio::PinPull for $name {
                fn set_internal_resistor(&mut self, _pull: $crate::gpio::Pull) {
                    match self {
                        $(
                            $(#[$attr])*
                            Self::$PX(p) => p.set_internal_resistor(_pull),
                        )*
                        _ => {}
                    }
                }
            }

            #[allow(unreachable_patterns)]
            impl $crate::gpio::ExtiPin for $name {
                extipin! { $( $(#[$attr])* $PX, )* }
            }

            $(
                impl From<$NoPin<$Otype>> for $name {
                    fn from(p: $NoPin<$Otype>) -> Self {
                        Self::None(p)
                    }
                }
            )?

            $(
                $(#[$attr])*
                impl<MODE> From<gpio::$PX<MODE>> for $name
                where
                    MODE: $crate::gpio::marker::NotAlt + $crate::gpio::PinMode
                {
                    fn from(p: gpio::$PX<MODE>) -> Self {
                        Self::$PX(p.into_mode())
                    }
                }

                $(#[$attr])*
                impl From<gpio::$PX<$crate::gpio::Alternate<$A, $Otype>>> for $name {
                    fn from(p: gpio::$PX<$crate::gpio::Alternate<$A, $Otype>>) -> Self {
                        Self::$PX(p)
                    }
                }

                $(#[$attr])*
                #[allow(irrefutable_let_patterns)]
                impl<MODE> TryFrom<$name> for gpio::$PX<MODE>
                where
                    MODE: $crate::gpio::PinMode,
                    $crate::gpio::Alternate<$A, $Otype>: $crate::gpio::PinMode,
                {
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

    ( $($(#[$docs:meta])* <$name:ident> default:$DefaultOtype:ident for $(no: $NoPin:ident,)? [$(
            $(#[$attr:meta])* $PX:ident<$A:literal>,
    )*],)*) => {
        $(
            #[derive(Debug)]
            $(#[$docs])*
            pub enum $name<Otype = $DefaultOtype> {
                $(
                    None($NoPin<Otype>),
                )?

                $(
                    $(#[$attr])*
                    $PX(gpio::$PX<$crate::gpio::Alternate<$A, Otype>>),
                )*
            }

            impl<Otype> crate::Sealed for $name<Otype> { }

            #[allow(unreachable_patterns)]
            impl<Otype> $crate::gpio::ReadPin for $name<Otype> {
                fn is_low(&self) -> bool {
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
            impl<Otype> $crate::gpio::PinSpeed for $name<Otype> {
                fn set_speed(&mut self, _speed: $crate::gpio::Speed) {
                    match self {
                        $(
                            $(#[$attr])*
                            Self::$PX(p) => p.set_speed(_speed),
                        )*
                        _ => {}
                    }
                }
            }

            #[allow(unreachable_patterns)]
            impl<Otype> $crate::gpio::PinPull for $name<Otype> {
                fn set_internal_resistor(&mut self, _pull: $crate::gpio::Pull) {
                    match self {
                        $(
                            $(#[$attr])*
                            Self::$PX(p) => p.set_internal_resistor(_pull),
                        )*
                        _ => {}
                    }
                }
            }

            #[allow(unreachable_patterns)]
            impl<Otype> $crate::gpio::ExtiPin for $name<Otype> {
                extipin! { $( $(#[$attr])* $PX, )* }
            }

            $(
                impl<Otype> From<$NoPin<Otype>> for $name<Otype> {
                    fn from(p: $NoPin<Otype>) -> Self {
                        Self::None(p)
                    }
                }
            )?

            $(
                $(#[$attr])*
                impl<MODE, Otype> From<gpio::$PX<MODE>> for $name<Otype>
                where
                    MODE: $crate::gpio::marker::NotAlt + $crate::gpio::PinMode,
                    $crate::gpio::Alternate<$A, Otype>: $crate::gpio::PinMode,
                {
                    fn from(p: gpio::$PX<MODE>) -> Self {
                        Self::$PX(p.into_mode())
                    }
                }

                $(#[$attr])*
                impl<Otype> From<gpio::$PX<$crate::gpio::Alternate<$A, Otype>>> for $name<Otype> {
                    fn from(p: gpio::$PX<$crate::gpio::Alternate<$A, Otype>>) -> Self {
                        Self::$PX(p)
                    }
                }

                $(#[$attr])*
                #[allow(irrefutable_let_patterns)]
                impl<MODE, Otype> TryFrom<$name<Otype>> for gpio::$PX<MODE>
                where
                    MODE: $crate::gpio::PinMode,
                    $crate::gpio::Alternate<$A, Otype>: $crate::gpio::PinMode,
                {
                    type Error = ();

                    fn try_from(a: $name<Otype>) -> Result<Self, Self::Error> {
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
use pin;

// CAN pins
#[cfg(feature = "can1")]
pub trait CanCommon {
    type Rx;
    type Tx;
}

// DFSDM pins
#[cfg(feature = "dfsdm")]
pub trait DfsdmBasic {
    type Ckin0;
    type Ckin1;
    type Ckout;
    type Datin0;
    type Datin1;
}
#[cfg(feature = "dfsdm")]
pub trait DfsdmGeneral: DfsdmBasic {
    type Ckin2;
    type Ckin3;
    type Datin2;
    type Datin3;
}
#[cfg(feature = "dfsdm")]
pub trait DfsdmAdvanced: DfsdmGeneral {
    type Ckin4;
    type Ckin5;
    type Ckin6;
    type Ckin7;
    type Datin4;
    type Datin5;
    type Datin6;
    type Datin7;
}

// Serial pins
pub trait SerialAsync {
    /// Receive
    type Rx<Otype>;
    /// Transmit
    type Tx<Otype>;
}
/// Synchronous mode
pub trait SerialSync {
    type Ck;
}
/// Hardware flow control (RS232)
pub trait SerialRs232 {
    /// Receive
    type Cts;
    /// Transmit
    type Rts;
}

// I2C pins
pub trait I2cCommon {
    type Scl;
    type Sda;
    type Smba;
}

// I2S pins
pub trait I2sCommon {
    type Ck;
    type Sd;
    type Ws: crate::gpio::ReadPin + crate::gpio::ExtiPin;
}
pub trait I2sMaster {
    type Mck;
}
pub trait I2sExtPin {
    type ExtSd;
}

// QuadSPI pins

#[cfg(feature = "quadspi")]
pub trait QuadSpiBanks {
    type Bank1;
    type Bank2;
}
#[cfg(feature = "quadspi")]
pub trait QuadSpiBank {
    type Io0: crate::gpio::PinSpeed;
    type Io1: crate::gpio::PinSpeed;
    type Io2: crate::gpio::PinSpeed;
    type Io3: crate::gpio::PinSpeed;
    type Ncs: crate::gpio::PinSpeed;
}

// SAI pins

#[cfg(feature = "sai1")]
pub trait SaiChannels {
    type A;
    type B;
}
#[cfg(feature = "sai1")]
pub trait SaiChannel {
    type Fs;
    type Mclk;
    type Sck;
    type Sd;
}

// SPDIFRX pins

#[cfg(feature = "spdifrx")]
pub trait SPdifIn<const C: u8> {
    type In;
}

// SPI pins
pub trait SpiCommon {
    type Miso;
    type Mosi;
    type Nss;
    type Sck;
}

// Timer pins

/// Input capture / Output compare channel `C`
pub trait TimCPin<const C: u8> {
    type Ch<Otype>;
}

/// Complementary output channel `C`
pub trait TimNCPin<const C: u8> {
    type ChN<Otype>;
}

/// Break input
pub trait TimBkin {
    type Bkin;
}

/// External trigger timer input
pub trait TimEtr {
    type Etr;
}
