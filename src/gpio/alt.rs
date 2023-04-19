mod f4;
pub use f4::*;

macro_rules! extipin {
    ($( $(#[$attr:meta])* $PX:ident,)*) => {
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
    };
}
use extipin;

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
                    $PX(gpio::$PX<Alternate<$A, $Otype>>),
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
                    MODE: marker::NotAlt + PinMode
                {
                    fn from(p: gpio::$PX<MODE>) -> Self {
                        Self::$PX(p.into_mode())
                    }
                }

                $(#[$attr])*
                impl From<gpio::$PX<Alternate<$A, $Otype>>> for $name {
                    fn from(p: gpio::$PX<Alternate<$A, $Otype>>) -> Self {
                        Self::$PX(p)
                    }
                }

                $(#[$attr])*
                #[allow(irrefutable_let_patterns)]
                impl<MODE> TryFrom<$name> for gpio::$PX<MODE>
                where
                    MODE: PinMode,
                    Alternate<$A, $Otype>: PinMode,
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
                    $PX(gpio::$PX<Alternate<$A, Otype>>),
                )*
            }

            impl<Otype> crate::Sealed for $name<Otype> { }

            #[allow(unreachable_patterns)]
            impl<Otype> $name<Otype> {
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
            impl<Otype> ExtiPin for $name<Otype> {
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
                    MODE: marker::NotAlt + PinMode,
                    Alternate<$A, Otype>: PinMode,
                {
                    fn from(p: gpio::$PX<MODE>) -> Self {
                        Self::$PX(p.into_mode())
                    }
                }

                $(#[$attr])*
                impl<Otype> From<gpio::$PX<Alternate<$A, Otype>>> for $name<Otype> {
                    fn from(p: gpio::$PX<Alternate<$A, Otype>>) -> Self {
                        Self::$PX(p)
                    }
                }

                $(#[$attr])*
                #[allow(irrefutable_let_patterns)]
                impl<MODE, Otype> TryFrom<$name<Otype>> for gpio::$PX<MODE>
                where
                    MODE: PinMode,
                    Alternate<$A, Otype>: PinMode,
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
#[cfg(feature = "dfsdm1")]
pub trait DfsdmCommon {
    type Ckin0;
    type Ckin1;
    type Ckin2;
    type Ckin3;
    type Ckout;
    type Datin0;
    type Datin1;
    type Datin2;
    type Datin3;
}
#[cfg(feature = "dfsdm2")]
pub trait DfsdmAdvanced {
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
    type Ws;
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
    type Io0;
    type Io1;
    type Io2;
    type Io3;
    type Ncs;
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
