//! General Purpose Input / Output
//!
//! The GPIO pins are organised into groups of 16 pins which can be accessed through the
//! `gpioa`, `gpiob`... modules. To get access to the pins, you first need to convert them into a
//! HAL designed struct from the `pac` struct using the [split](trait.GpioExt.html#tymethod.split) function.
//! ```rust
//! // Acquire the GPIOC peripheral
//! // NOTE: `dp` is the device peripherals from the `PAC` crate
//! let mut gpioa = dp.GPIOA.split();
//! ```
//!
//! This gives you a struct containing all the pins `px0..px15`.
//! By default pins are in floating input mode. You can change their modes.
//! For example, to set `pa5` high, you would call
//!
//! ```rust
//! let output = gpioa.pa5.into_push_pull_output();
//! output.set_high();
//! ```
//!
//! ## Modes
//!
//! Each GPIO pin can be set to various modes:
//!
//! - **Alternate**: Pin mode required when the pin is driven by other peripherals
//! - **Analog**: Analog input to be used with ADC.
//! - **Dynamic**: Pin mode is selected at runtime. See changing configurations for more details
//! - Input
//!     - **PullUp**: Input connected to high with a weak pull up resistor. Will be high when nothing
//!     is connected
//!     - **PullDown**: Input connected to high with a weak pull up resistor. Will be low when nothing
//!     is connected
//!     - **Floating**: Input not pulled to high or low. Will be undefined when nothing is connected
//! - Output
//!     - **PushPull**: Output which either drives the pin high or low
//!     - **OpenDrain**: Output which leaves the gate floating, or pulls it do ground in drain
//!     mode. Can be used as an input in the `open` configuration
//!
//! ## Changing modes
//! The simplest way to change the pin mode is to use the `into_<mode>` functions. These return a
//! new struct with the correct mode that you can use the input or output functions on.
//!
//! If you need a more temporary mode change, and can not use the `into_<mode>` functions for
//! ownership reasons, you can use the closure based `with_<mode>` functions to temporarily change the pin type, do
//! some output or input, and then have it change back once done.
//!
//! ### Dynamic Mode Change
//! The above mode change methods guarantee that you can only call input functions when the pin is
//! in input mode, and output when in output modes, but can lead to some issues. Therefore, there
//! is also a mode where the state is kept track of at runtime, allowing you to change the mode
//! often, and without problems with ownership, or references, at the cost of some performance and
//! the risk of runtime errors.
//!
//! To make a pin dynamic, use the `into_dynamic` function, and then use the `make_<mode>` functions to
//! change the mode

use core::marker::PhantomData;

pub mod alt;
pub(crate) use alt::{Const, PinA, SetAlternate};
mod convert;
pub use convert::PinMode;
mod partially_erased;
pub use partially_erased::{PEPin, PartiallyErasedPin};
mod erased;
pub use erased::{EPin, ErasedPin};
mod exti;
pub use exti::ExtiPin;
mod dynamic;
pub use dynamic::{Dynamic, DynamicPin};
mod hal_02;
mod hal_1;
pub mod outport;

pub use embedded_hal::digital::v2::PinState;

use core::fmt;

/// A filler pin type
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NoPin;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Id, port and mode for any pin
pub trait PinExt {
    /// Current pin mode
    type Mode;
    /// Pin number
    fn pin_id(&self) -> u8;
    /// Port number starting from 0
    fn port_id(&self) -> u8;
}

/// Some alternate mode (type state)
pub struct Alternate<const A: u8, Otype = PushPull>(PhantomData<Otype>);

/// Input mode (type state)
pub struct Input;

/// Pull setting for an input.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// Floating
    None = 0,
    /// Pulled up
    Up = 1,
    /// Pulled down
    Down = 2,
}

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Output mode (type state)
pub struct Output<MODE = PushPull> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// Analog mode (type state)
pub struct Analog;

/// JTAG/SWD mote (type state)
pub type Debugger = Alternate<0, PushPull>;

pub(crate) mod marker {
    /// Marker trait that show if `ExtiPin` can be implemented
    pub trait Interruptable {}
    /// Marker trait for readable pin modes
    pub trait Readable {}
    /// Marker trait for slew rate configurable pin modes
    pub trait OutputSpeed {}
    /// Marker trait for active pin modes
    pub trait Active {}
    /// Marker trait for all pin modes except alternate
    pub trait NotAlt {}
    /// Marker trait for pins with alternate function `A` mapping
    pub trait IntoAf<const A: u8> {}
}

impl<MODE> marker::Interruptable for Output<MODE> {}
impl marker::Interruptable for Input {}
impl marker::Readable for Input {}
impl marker::Readable for Output<OpenDrain> {}
impl marker::Active for Input {}
impl<Otype> marker::OutputSpeed for Output<Otype> {}
impl<const A: u8, Otype> marker::OutputSpeed for Alternate<A, Otype> {}
impl<Otype> marker::Active for Output<Otype> {}
impl<const A: u8, Otype> marker::Active for Alternate<A, Otype> {}
impl marker::NotAlt for Input {}
impl<Otype> marker::NotAlt for Output<Otype> {}
impl marker::NotAlt for Analog {}

/// GPIO Pin speed selection
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Speed {
    /// Low speed
    Low = 0,
    /// Medium speed
    Medium = 1,
    /// High speed
    High = 2,
    /// Very high speed
    VeryHigh = 3,
}

/// GPIO interrupt trigger edge selection
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Edge {
    /// Rising edge of voltage
    Rising,
    /// Falling edge of voltage
    Falling,
    /// Rising and falling edge of voltage
    RisingFalling,
}

macro_rules! af {
    ($($i:literal: $AFi:ident),+) => {
        $(
            #[doc = concat!("Alternate function ", $i, " (type state)" )]
            pub type $AFi<Otype = PushPull> = Alternate<$i, Otype>;
        )+
    };
}

af!(
    0: AF0,
    1: AF1,
    2: AF2,
    3: AF3,
    4: AF4,
    5: AF5,
    6: AF6,
    7: AF7,
    8: AF8,
    9: AF9,
    10: AF10,
    11: AF11,
    12: AF12,
    13: AF13,
    14: AF14,
    15: AF15
);

/// Generic pin type
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `15`.
pub struct Pin<const P: char, const N: u8, MODE = Input> {
    _mode: PhantomData<MODE>,
}
impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const P: char, const N: u8, MODE> fmt::Debug for Pin<P, N, MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P{}{}<{}>",
            P,
            N,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<const P: char, const N: u8, MODE> defmt::Format for Pin<P, N, MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P{}{}<{}>", P, N, crate::stripped_type_name::<MODE>());
    }
}

impl<const P: char, const N: u8, MODE> PinExt for Pin<P, N, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    /// Set pin speed
    pub fn set_speed(&mut self, speed: Speed) {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .ospeedr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset)));
        }
    }

    /// Set pin speed
    pub fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    /// Set the internal pull-up and pull-down resistor
    pub fn set_internal_resistor(&mut self, resistor: Pull) {
        let offset = 2 * { N };
        let value = resistor as u32;
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (value << offset)));
        }
    }

    /// Set the internal pull-up and pull-down resistor
    pub fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }

    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Up)
        } else {
            self.internal_resistor(Pull::None)
        }
    }

    /// Enables / disables the internal pull down
    pub fn internal_pull_down(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Down)
        } else {
            self.internal_resistor(Pull::None)
        }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Erases the pin number from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn erase_number(self) -> PartiallyErasedPin<P, MODE> {
        PartiallyErasedPin::new(N)
    }

    /// Erases the pin number and the port from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn erase(self) -> ErasedPin<MODE> {
        ErasedPin::new(P as u8 - b'A', N)
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, MODE>> for PartiallyErasedPin<P, MODE> {
    /// Pin-to-partially erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: Pin<P, N, MODE>) -> Self {
        p.erase_number()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, MODE>> for ErasedPin<MODE> {
    /// Pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: Pin<P, N, MODE>) -> Self {
        p.erase()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).bsrr.write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).bsrr.write(|w| w.bits(1 << (16 + N))) }
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).odr.read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << N) == 0 }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    /// Is the pin in drive high or low mode?
    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        self._is_low()
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $PEPin:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, [$($A:literal),*] $(, $MODE:ty)?),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use crate::pac::{$GPIOX, RCC};
            use crate::rcc::{Enable, Reset};

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi $(<$MODE>)?,
                )+
            }

            impl super::GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    unsafe {
                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        let rcc = &(*RCC::ptr());

                        // Enable clock.
                        $GPIOX::enable(rcc);
                        $GPIOX::reset(rcc);
                    }
                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            #[doc="Common type for "]
            #[doc=stringify!($GPIOX)]
            #[doc=" related pins"]
            pub type $PXn<MODE> = super::PartiallyErasedPin<$port_id, MODE>;

            $(
                #[doc=stringify!($PXi)]
                #[doc=" pin"]
                pub type $PXi<MODE = super::Input> = super::Pin<$port_id, $i, MODE>;

                $(
                    impl<MODE> super::marker::IntoAf<$A> for $PXi<MODE> { }
                )*
            )+

        }

        pub use $gpiox::{ $($PXi,)+ };
    }
}

#[cfg(feature = "gpio-f401")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 7]),
    PA1: (pa1, 1, [1, 2, 7]),
    PA2: (pa2, 2, [1, 2, 3, 7]),
    PA3: (pa3, 3, [1, 2, 3, 7]),
    PA4: (pa4, 4, [5, 6, 7]),
    PA5: (pa5, 5, [1, 5]),
    PA6: (pa6, 6, [1, 2, 5]),
    PA7: (pa7, 7, [1, 2, 5]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10]),
    PA9: (pa9, 9, [1, 4, 7]),
    PA10: (pa10, 10, [1, 7, 10]),
    PA11: (pa11, 11, [1, 7, 8, 10]),
    PA12: (pa12, 12, [1, 7, 8, 10]),
    PA13: (pa13, 13, [0], super::Debugger), // SWDIO, PullUp VeryHigh speed
    PA14: (pa14, 14, [0, 5], super::Debugger), // SWCLK, PullDown
    PA15: (pa15, 15, [0, 1, 5, 6], super::Debugger), // JTDI, PullUp
]);

#[cfg(feature = "gpio-f401")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2]),
    PB1: (pb1, 1, [1, 2]),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, [0, 1, 5, 6, 9], super::Debugger), // SWO, VeryHigh speed
    PB4: (pb4, 4, [0, 2, 5, 6, 7, 9], super::Debugger), // JTRST, PullUp
    PB5: (pb5, 5, [2, 4, 5, 6]),
    PB6: (pb6, 6, [2, 4, 7]),
    PB7: (pb7, 7, [2, 4, 7]),
    PB8: (pb8, 8, [2, 3, 4, 12]),
    PB9: (pb9, 9, [2, 3, 4, 5, 12]),
    PB10: (pb10, 10, [1, 4, 5]),
    PB11: (pb11, 11, [1, 4]),
    PB12: (pb12, 12, [1, 4, 5, 6]),
    PB13: (pb13, 13, [1, 5, 6]),
    PB14: (pb14, 14, [1, 5, 6]),
    PB15: (pb15, 15, [0, 1, 5, 6]),
]);

#[cfg(feature = "gpio-f401")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, []),
    PC1: (pc1, 1, []),
    PC2: (pc2, 2, [5, 6]),
    PC3: (pc3, 3, [5]),
    PC4: (pc4, 4, []),
    PC5: (pc5, 5, []),
    PC6: (pc6, 6, [2, 5, 8, 12]),
    PC7: (pc7, 7, [2, 6, 8, 12]),
    PC8: (pc8, 8, [2, 8, 12]),
    PC9: (pc9, 9, [0, 2, 4, 5, 12]),
    PC10: (pc10, 10, [5, 6, 12]),
    PC11: (pc11, 11, [5, 6, 12]),
    PC12: (pc12, 12, [5, 6, 12]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f401")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, []),
    PD1: (pd1, 1, []),
    PD2: (pd2, 2, [2, 12]),
    PD3: (pd3, 3, [5, 7]),
    PD4: (pd4, 4, [7]),
    PD5: (pd5, 5, [7]),
    PD6: (pd6, 6, [5, 7]),
    PD7: (pd7, 7, [7]),
    PD8: (pd8, 8, []),
    PD9: (pd9, 9, []),
    PD10: (pd10, 10, []),
    PD11: (pd11, 11, []),
    PD12: (pd12, 12, [2]),
    PD13: (pd13, 13, [2]),
    PD14: (pd14, 14, [2]),
    PD15: (pd15, 15, [2]),
]);

#[cfg(feature = "gpio-f401")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2]),
    PE1: (pe1, 1, []),
    PE2: (pe2, 2, [0, 5]),
    PE3: (pe3, 3, [0]),
    PE4: (pe4, 4, [0, 5]),
    PE5: (pe5, 5, [0, 3, 5]),
    PE6: (pe6, 6, [0, 3, 5]),
    PE7: (pe7, 7, [1]),
    PE8: (pe8, 8, [1]),
    PE9: (pe9, 9, [1]),
    PE10: (pe10, 10, [1]),
    PE11: (pe11, 11, [1, 5]),
    PE12: (pe12, 12, [1, 5]),
    PE13: (pe13, 13, [1, 5]),
    PE14: (pe14, 14, [1, 5]),
    PE15: (pe15, 15, [1]),
]);

#[cfg(feature = "gpio-f401")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
]);

#[cfg(feature = "gpio-f410")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [2, 7, 15]),
    PA1: (pa1, 1, [2, 7, 15]),
    PA2: (pa2, 2, [2, 3, 5, 7, 15]),
    PA3: (pa3, 3, [2, 3, 5, 7, 15]),
    PA4: (pa4, 4, [5, 7, 15]),
    PA5: (pa5, 5, [5, 15]),
    PA6: (pa6, 6, [1, 5, 6, 15]),
    PA7: (pa7, 7, [1, 5, 15]),
    PA8: (pa8, 8, [0, 1, 4, 7, 15]),
    PA9: (pa9, 9, [1, 7, 15]),
    PA10: (pa10, 10, [1, 6, 7, 15]),
    PA11: (pa11, 11, [1, 7, 8, 15]),
    PA12: (pa12, 12, [1, 6, 7, 8, 15]),
    PA13: (pa13, 13, [0, 15], super::Debugger),
    PA14: (pa14, 14, [0, 15], super::Debugger),
    PA15: (pa15, 15, [0, 5, 7, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f410")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 6, 15]),
    PB1: (pb1, 1, [1, 6, 15]),
    PB2: (pb2, 2, [1, 15]),
    PB3: (pb3, 3, [0, 4, 5, 7, 9, 15], super::Debugger),
    PB4: (pb4, 4, [0, 5, 15], super::Debugger),
    PB5: (pb5, 5, [1, 4, 5, 15]),
    PB6: (pb6, 6, [1, 4, 7, 15]),
    PB7: (pb7, 7, [1, 4, 7, 15]),
    PB8: (pb8, 8, [1, 4, 6, 15]),
    PB9: (pb9, 9, [3, 4, 5, 9, 15]),
    PB10: (pb10, 10, [4, 5, 6, 9, 15]),
    PB11: (pb11, 11, [0, 2, 4, 5, 15]),
    PB12: (pb12, 12, [1, 2, 4, 5, 15]),
    PB13: (pb13, 13, [1, 4, 5, 15]),
    PB14: (pb14, 14, [1, 4, 5, 15]),
    PB15: (pb15, 15, [0, 1, 4, 5, 15]),
]);

#[cfg(feature = "gpio-f410")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 15]),
    PC1: (pc1, 1, [1, 15]),
    PC2: (pc2, 2, [1, 5, 15]),
    PC3: (pc3, 3, [1, 5, 15]),
    PC4: (pc4, 4, [3, 15]),
    PC5: (pc5, 5, [3, 4, 15]),
    PC6: (pc6, 6, [0, 4, 5, 8, 15]),
    PC7: (pc7, 7, [4, 5, 6, 8, 15]),
    PC8: (pc8, 8, [8, 15]),
    PC9: (pc9, 9, [0, 4, 5, 15]),
    PC10: (pc10, 10, [0, 2, 15]),
    PC11: (pc11, 11, [0, 2, 15]),
    PC12: (pc12, 12, [0, 3, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-f410")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
]);

#[cfg(feature = "gpio-f411")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 7]),
    PA1: (pa1, 1, [1, 2, 5, 7]),
    PA2: (pa2, 2, [1, 2, 3, 5, 7]),
    PA3: (pa3, 3, [1, 2, 3, 5, 7]),
    PA4: (pa4, 4, [5, 6, 7]),
    PA5: (pa5, 5, [1, 5]),
    PA6: (pa6, 6, [1, 2, 5, 6, 12]),
    PA7: (pa7, 7, [1, 2, 5]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10, 12]),
    PA9: (pa9, 9, [1, 4, 7, 10, 12]),
    PA10: (pa10, 10, [1, 6, 7, 10]),
    PA11: (pa11, 11, [1, 6, 7, 8, 10]),
    PA12: (pa12, 12, [1, 6, 7, 8, 10]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6, 7], super::Debugger),
]);

#[cfg(feature = "gpio-f411")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 6]),
    PB1: (pb1, 1, [1, 2, 6]),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, [0, 1, 5, 6, 7, 9], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7, 9, 12], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 12]),
    PB6: (pb6, 6, [2, 4, 7]),
    PB7: (pb7, 7, [2, 4, 7, 12]),
    PB8: (pb8, 8, [2, 3, 4, 6, 9, 12]),
    PB9: (pb9, 9, [2, 3, 4, 5, 9, 12]),
    PB10: (pb10, 10, [1, 4, 5, 6, 12]),
    PB11: (pb11, 11, [1, 4, 5]),
    PB12: (pb12, 12, [1, 4, 5, 6, 7]),
    PB13: (pb13, 13, [1, 5, 6]),
    PB14: (pb14, 14, [1, 5, 6, 12]),
    PB15: (pb15, 15, [0, 1, 5, 12]),
]);

#[cfg(feature = "gpio-f411")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, []),
    PC1: (pc1, 1, []),
    PC2: (pc2, 2, [5, 6]),
    PC3: (pc3, 3, [5]),
    PC4: (pc4, 4, []),
    PC5: (pc5, 5, []),
    PC6: (pc6, 6, [2, 5, 8, 12]),
    PC7: (pc7, 7, [2, 5, 6, 8, 12]),
    PC8: (pc8, 8, [2, 8, 12]),
    PC9: (pc9, 9, [0, 2, 4, 5, 12]),
    PC10: (pc10, 10, [6, 12]),
    PC11: (pc11, 11, [5, 6, 12]),
    PC12: (pc12, 12, [6, 12]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f411")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, []),
    PD1: (pd1, 1, []),
    PD2: (pd2, 2, [2, 12]),
    PD3: (pd3, 3, [5, 7]),
    PD4: (pd4, 4, [7]),
    PD5: (pd5, 5, [7]),
    PD6: (pd6, 6, [5, 7]),
    PD7: (pd7, 7, [7]),
    PD8: (pd8, 8, []),
    PD9: (pd9, 9, []),
    PD10: (pd10, 10, []),
    PD11: (pd11, 11, []),
    PD12: (pd12, 12, [2]),
    PD13: (pd13, 13, [2]),
    PD14: (pd14, 14, [2]),
    PD15: (pd15, 15, [2]),
]);

#[cfg(feature = "gpio-f411")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2]),
    PE1: (pe1, 1, []),
    PE2: (pe2, 2, [0, 5, 6]),
    PE3: (pe3, 3, [0]),
    PE4: (pe4, 4, [0, 5, 6]),
    PE5: (pe5, 5, [0, 3, 5, 6]),
    PE6: (pe6, 6, [0, 3, 5, 6]),
    PE7: (pe7, 7, [1]),
    PE8: (pe8, 8, [1]),
    PE9: (pe9, 9, [1]),
    PE10: (pe10, 10, [1]),
    PE11: (pe11, 11, [1, 5, 6]),
    PE12: (pe12, 12, [1, 5, 6]),
    PE13: (pe13, 13, [1, 5, 6]),
    PE14: (pe14, 14, [1, 5, 6]),
    PE15: (pe15, 15, [1]),
]);

#[cfg(feature = "gpio-f411")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 15]),
    PA1: (pa1, 1, [1, 2, 5, 7, 9, 15]),
    PA2: (pa2, 2, [1, 2, 3, 5, 7, 12, 15]),
    PA3: (pa3, 3, [1, 2, 3, 5, 7, 12, 15]),
    PA4: (pa4, 4, [5, 6, 7, 8, 12, 15]),
    PA5: (pa5, 5, [1, 3, 5, 8, 12, 15]),
    PA6: (pa6, 6, [1, 2, 3, 5, 6, 9, 10, 12, 15]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 10, 15]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10, 12, 15]),
    PA9: (pa9, 9, [1, 4, 7, 10, 12, 15]),
    PA10: (pa10, 10, [1, 6, 7, 10, 15]),
    PA11: (pa11, 11, [1, 6, 7, 8, 9, 10, 15]),
    PA12: (pa12, 12, [1, 6, 7, 8, 9, 10, 15]),
    PA13: (pa13, 13, [0, 15], super::Debugger),
    PA14: (pa14, 14, [0, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6, 7, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 6, 15]),
    PB1: (pb1, 1, [1, 2, 3, 6, 8, 9, 15]),
    PB2: (pb2, 2, [6, 9, 15]),
    PB3: (pb3, 3, [0, 1, 4, 5, 6, 7, 9, 15], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7, 9, 12, 15], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 9, 12, 15]),
    PB6: (pb6, 6, [2, 4, 7, 9, 10, 12, 15]),
    PB7: (pb7, 7, [2, 4, 7, 12, 15]),
    PB8: (pb8, 8, [2, 3, 4, 6, 8, 9, 12, 15]),
    PB9: (pb9, 9, [2, 3, 4, 5, 8, 9, 12, 15]),
    PB10: (pb10, 10, [1, 4, 5, 6, 7, 9, 12, 15]),
    PB11: (pb11, 11, [1, 4, 5, 7, 15]),
    PB12: (pb12, 12, [1, 4, 5, 6, 7, 8, 9, 10, 12, 15]),
    PB13: (pb13, 13, [1, 4, 5, 6, 8, 9, 10, 15]),
    PB14: (pb14, 14, [1, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15]),
    PB15: (pb15, 15, [0, 1, 3, 4, 5, 8, 9, 12, 15]),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [15]),
    PC1: (pc1, 1, [15]),
    PC2: (pc2, 2, [5, 6, 8, 12, 15]),
    PC3: (pc3, 3, [5, 12, 15]),
    PC4: (pc4, 4, [5, 10, 12, 15]),
    PC5: (pc5, 5, [4, 7, 10, 12, 15]),
    PC6: (pc6, 6, [2, 3, 4, 5, 6, 8, 10, 12, 15]),
    PC7: (pc7, 7, [2, 3, 4, 5, 6, 8, 10, 12, 15]),
    PC8: (pc8, 8, [2, 3, 8, 9, 12, 15]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 9, 12, 15]),
    PC10: (pc10, 10, [6, 7, 9, 12, 15]),
    PC11: (pc11, 11, [5, 6, 7, 9, 10, 12, 15]),
    PC12: (pc12, 12, [6, 7, 10, 12, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [9, 12, 15]),
    PD1: (pd1, 1, [9, 12, 15]),
    PD2: (pd2, 2, [2, 10, 12, 15]),
    PD3: (pd3, 3, [0, 5, 6, 7, 9, 12, 15]),
    PD4: (pd4, 4, [6, 7, 12, 15]),
    PD5: (pd5, 5, [7, 12, 15]),
    PD6: (pd6, 6, [5, 6, 7, 12, 15]),
    PD7: (pd7, 7, [6, 7, 12, 15]),
    PD8: (pd8, 8, [7, 12, 15]),
    PD9: (pd9, 9, [7, 12, 15]),
    PD10: (pd10, 10, [7, 12, 15]),
    PD11: (pd11, 11, [4, 7, 9, 12, 15]),
    PD12: (pd12, 12, [2, 4, 7, 9, 12, 15]),
    PD13: (pd13, 13, [2, 4, 9, 12, 15]),
    PD14: (pd14, 14, [2, 4, 12, 15]),
    PD15: (pd15, 15, [2, 4, 12, 15]),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 12, 15]),
    PE1: (pe1, 1, [12, 15]),
    PE2: (pe2, 2, [0, 5, 6, 9, 12, 15]),
    PE3: (pe3, 3, [0, 12, 15]),
    PE4: (pe4, 4, [0, 5, 6, 8, 12, 15]),
    PE5: (pe5, 5, [0, 3, 5, 6, 8, 12, 15]),
    PE6: (pe6, 6, [0, 3, 5, 6, 12, 15]),
    PE7: (pe7, 7, [1, 6, 10, 12, 15]),
    PE8: (pe8, 8, [1, 6, 10, 12, 15]),
    PE9: (pe9, 9, [1, 6, 10, 12, 15]),
    PE10: (pe10, 10, [1, 10, 12, 15]),
    PE11: (pe11, 11, [1, 5, 6, 12, 15]),
    PE12: (pe12, 12, [1, 5, 6, 12, 15]),
    PE13: (pe13, 13, [1, 5, 6, 12, 15]),
    PE14: (pe14, 14, [1, 5, 6, 12, 15]),
    PE15: (pe15, 15, [1, 12, 15]),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12, 15]),
    PF1: (pf1, 1, [4, 12, 15]),
    PF2: (pf2, 2, [4, 12, 15]),
    PF3: (pf3, 3, [2, 12, 15]),
    PF4: (pf4, 4, [2, 12, 15]),
    PF5: (pf5, 5, [2, 12, 15]),
    PF6: (pf6, 6, [0, 3, 9, 15]),
    PF7: (pf7, 7, [0, 3, 9, 15]),
    PF8: (pf8, 8, [9, 10, 15]),
    PF9: (pf9, 9, [9, 10, 15]),
    PF10: (pf10, 10, [1, 2, 15]),
    PF11: (pf11, 11, [3, 15]),
    PF12: (pf12, 12, [3, 12, 15]),
    PF13: (pf13, 13, [4, 12, 15]),
    PF14: (pf14, 14, [4, 12, 15]),
    PF15: (pf15, 15, [4, 12, 15]),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [9, 12, 15]),
    PG1: (pg1, 1, [9, 12, 15]),
    PG2: (pg2, 2, [12, 15]),
    PG3: (pg3, 3, [12, 15]),
    PG4: (pg4, 4, [12, 15]),
    PG5: (pg5, 5, [12, 15]),
    PG6: (pg6, 6, [10, 15]),
    PG7: (pg7, 7, [8, 15]),
    PG8: (pg8, 8, [8, 15]),
    PG9: (pg9, 9, [8, 9, 12, 15]),
    PG10: (pg10, 10, [12, 15]),
    PG11: (pg11, 11, [9, 15]),
    PG12: (pg12, 12, [8, 9, 12, 15]),
    PG13: (pg13, 13, [0, 8, 12, 15]),
    PG14: (pg14, 14, [0, 8, 9, 12, 15]),
    PG15: (pg15, 15, [8, 15]),
]);

#[cfg(feature = "gpio-f412")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 15]),
    PA1: (pa1, 1, [1, 2, 5, 7, 8, 9, 15]),
    PA2: (pa2, 2, [1, 2, 3, 5, 7, 12, 15]),
    PA3: (pa3, 3, [1, 2, 3, 5, 7, 10, 12, 15]),
    PA4: (pa4, 4, [5, 6, 7, 8, 12, 15]),
    PA5: (pa5, 5, [1, 3, 5, 8, 12, 15]),
    PA6: (pa6, 6, [1, 2, 3, 5, 6, 7, 9, 10, 12, 15]),
    PA7: (pa7, 7, [1, 2, 3, 5, 7, 9, 10, 15]),
    PA8: (pa8, 8, [0, 1, 4, 6, 7, 8, 10, 11, 12, 15]),
    PA9: (pa9, 9, [1, 3, 4, 5, 7, 10, 12, 15]),
    PA10: (pa10, 10, [1, 3, 5, 6, 7, 10, 15]),
    PA11: (pa11, 11, [1, 3, 5, 6, 7, 8, 9, 10, 11, 15]),
    PA12: (pa12, 12, [1, 3, 5, 6, 7, 8, 9, 10, 11, 15]),
    PA13: (pa13, 13, [0, 15], super::Debugger),
    PA14: (pa14, 14, [0, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6, 7, 8, 10, 11, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 6, 15]),
    PB1: (pb1, 1, [1, 2, 3, 6, 8, 9, 15]),
    PB2: (pb2, 2, [1, 6, 9, 15]),
    PB3: (pb3, 3, [0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 15], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 8, 9, 10, 11, 12, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 4, 5, 6, 9, 10, 11, 12, 15]),
    PB6: (pb6, 6, [1, 2, 4, 6, 7, 9, 10, 11, 12, 15]),
    PB7: (pb7, 7, [1, 2, 4, 6, 7, 12, 15]),
    PB8: (pb8, 8, [1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 15]),
    PB9: (pb9, 9, [2, 3, 4, 5, 6, 8, 9, 11, 12, 15]),
    PB10: (pb10, 10, [1, 4, 5, 6, 7, 9, 10, 12, 15]),
    PB11: (pb11, 11, [1, 4, 5, 7, 15]),
    PB12: (pb12, 12, [1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15]),
    PB13: (pb13, 13, [1, 4, 5, 6, 8, 9, 10, 11, 15]),
    PB14: (pb14, 14, [1, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15]),
    PB15: (pb15, 15, [0, 1, 3, 4, 5, 8, 9, 12, 15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 3, 7, 15]),
    PC1: (pc1, 1, [1, 3, 7, 15]),
    PC2: (pc2, 2, [1, 3, 5, 6, 7, 8, 12, 15]),
    PC3: (pc3, 3, [1, 3, 5, 7, 12, 15]),
    PC4: (pc4, 4, [3, 5, 10, 12, 15]),
    PC5: (pc5, 5, [3, 4, 7, 10, 12, 15]),
    PC6: (pc6, 6, [2, 3, 4, 5, 6, 7, 8, 10, 12, 15]),
    PC7: (pc7, 7, [2, 3, 4, 5, 6, 7, 8, 10, 12, 15]),
    PC8: (pc8, 8, [2, 3, 7, 8, 9, 12, 15]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 7, 9, 12, 15]),
    PC10: (pc10, 10, [3, 6, 7, 9, 12, 15]),
    PC11: (pc11, 11, [3, 5, 6, 7, 8, 9, 10, 12, 15]),
    PC12: (pc12, 12, [6, 7, 8, 10, 12, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [3, 9, 11, 12, 15]),
    PD1: (pd1, 1, [3, 9, 11, 12, 15]),
    PD2: (pd2, 2, [2, 3, 8, 10, 12, 15]),
    PD3: (pd3, 3, [0, 5, 6, 7, 9, 12, 15]),
    PD4: (pd4, 4, [6, 7, 12, 15]),
    PD5: (pd5, 5, [3, 7, 12, 15]),
    PD6: (pd6, 6, [5, 6, 7, 12, 15]),
    PD7: (pd7, 7, [6, 7, 12, 15]),
    PD8: (pd8, 8, [7, 12, 15]),
    PD9: (pd9, 9, [7, 12, 15]),
    PD10: (pd10, 10, [7, 8, 12, 15]),
    PD11: (pd11, 11, [3, 4, 7, 9, 12, 15]),
    PD12: (pd12, 12, [2, 3, 4, 7, 9, 12, 15]),
    PD13: (pd13, 13, [2, 4, 9, 12, 15]),
    PD14: (pd14, 14, [2, 4, 10, 11, 12, 15]),
    PD15: (pd15, 15, [2, 4, 10, 11, 12, 15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 3, 8, 12, 15]),
    PE1: (pe1, 1, [3, 8, 12, 15]),
    PE2: (pe2, 2, [0, 5, 6, 7, 9, 11, 12, 15]),
    PE3: (pe3, 3, [0, 7, 11, 12, 15]),
    PE4: (pe4, 4, [0, 5, 6, 7, 8, 12, 15]),
    PE5: (pe5, 5, [0, 3, 5, 6, 7, 8, 12, 15]),
    PE6: (pe6, 6, [0, 3, 5, 6, 7, 12, 15]),
    PE7: (pe7, 7, [1, 6, 8, 10, 12, 15]),
    PE8: (pe8, 8, [1, 6, 8, 10, 12, 15]),
    PE9: (pe9, 9, [1, 6, 10, 12, 15]),
    PE10: (pe10, 10, [1, 3, 10, 12, 15]),
    PE11: (pe11, 11, [1, 3, 5, 6, 12, 15]),
    PE12: (pe12, 12, [1, 3, 5, 6, 12, 15]),
    PE13: (pe13, 13, [1, 3, 5, 6, 12, 15]),
    PE14: (pe14, 14, [1, 5, 6, 10, 12, 15]),
    PE15: (pe15, 15, [1, 10, 12, 15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12, 15]),
    PF1: (pf1, 1, [4, 12, 15]),
    PF2: (pf2, 2, [4, 12, 15]),
    PF3: (pf3, 3, [2, 12, 15]),
    PF4: (pf4, 4, [2, 12, 15]),
    PF5: (pf5, 5, [2, 12, 15]),
    PF6: (pf6, 6, [0, 3, 7, 8, 9, 15]),
    PF7: (pf7, 7, [0, 3, 7, 8, 9, 15]),
    PF8: (pf8, 8, [7, 8, 9, 10, 15]),
    PF9: (pf9, 9, [7, 8, 9, 10, 15]),
    PF10: (pf10, 10, [1, 2, 15]),
    PF11: (pf11, 11, [3, 15]),
    PF12: (pf12, 12, [3, 12, 15]),
    PF13: (pf13, 13, [4, 12, 15]),
    PF14: (pf14, 14, [4, 12, 15]),
    PF15: (pf15, 15, [4, 12, 15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [9, 11, 12, 15]),
    PG1: (pg1, 1, [9, 11, 12, 15]),
    PG2: (pg2, 2, [12, 15]),
    PG3: (pg3, 3, [12, 15]),
    PG4: (pg4, 4, [12, 15]),
    PG5: (pg5, 5, [12, 15]),
    PG6: (pg6, 6, [10, 15]),
    PG7: (pg7, 7, [8, 15]),
    PG8: (pg8, 8, [8, 15]),
    PG9: (pg9, 9, [8, 9, 12, 15]),
    PG10: (pg10, 10, [12, 15]),
    PG11: (pg11, 11, [9, 11, 15]),
    PG12: (pg12, 12, [8, 9, 11, 12, 15]),
    PG13: (pg13, 13, [0, 8, 12, 15]),
    PG14: (pg14, 14, [0, 8, 9, 12, 15]),
    PG15: (pg15, 15, [8, 15]),
]);

#[cfg(feature = "gpio-f413")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [0, 1, 2, 3, 7, 8, 11]),
    PA1: (pa1, 1, [1, 2, 7, 8, 11]),
    PA2: (pa2, 2, [1, 2, 3, 7, 11]),
    PA3: (pa3, 3, [1, 2, 3, 7, 10, 11]),
    PA4: (pa4, 4, [5, 6, 7, 12, 13]),
    PA5: (pa5, 5, [1, 3, 5, 10]),
    PA6: (pa6, 6, [1, 2, 3, 5, 9, 13]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 11]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10]),
    PA9: (pa9, 9, [1, 4, 7, 13]),
    PA10: (pa10, 10, [1, 7, 10, 13]),
    PA11: (pa11, 11, [1, 7, 9, 10]),
    PA12: (pa12, 12, [1, 7, 9, 10]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6], super::Debugger),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 10, 11]),
    PB1: (pb1, 1, [1, 2, 3, 10, 11]),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, [0, 1, 5, 6], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 9, 10, 11, 13]),
    PB6: (pb6, 6, [2, 4, 7, 9, 13]),
    PB7: (pb7, 7, [2, 4, 7, 12, 13]),
    PB8: (pb8, 8, [2, 3, 4, 9, 11, 12, 13]),
    PB9: (pb9, 9, [2, 3, 4, 5, 9, 12, 13]),
    PB10: (pb10, 10, [1, 4, 5, 7, 10, 11]),
    PB11: (pb11, 11, [1, 4, 7, 10, 11]),
    PB12: (pb12, 12, [1, 4, 5, 7, 9, 10, 11, 12]),
    PB13: (pb13, 13, [1, 5, 7, 9, 10, 11]),
    PB14: (pb14, 14, [1, 3, 5, 6, 7, 9, 12]),
    PB15: (pb15, 15, [0, 1, 3, 5, 9, 12]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [10]),
    PC1: (pc1, 1, [11]),
    PC2: (pc2, 2, [5, 6, 10, 11]),
    PC3: (pc3, 3, [5, 10, 11]),
    PC4: (pc4, 4, [11]),
    PC5: (pc5, 5, [11]),
    PC6: (pc6, 6, [2, 3, 5, 8, 12, 13]),
    PC7: (pc7, 7, [2, 3, 6, 8, 12, 13]),
    PC8: (pc8, 8, [2, 3, 8, 12, 13]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 12, 13]),
    PC10: (pc10, 10, [6, 7, 8, 12, 13]),
    PC11: (pc11, 11, [5, 6, 7, 8, 12, 13]),
    PC12: (pc12, 12, [6, 7, 8, 12, 13]),
    PC13: (pc13, 13, [0]),
    PC14: (pc14, 14, [0]),
    PC15: (pc15, 15, [0]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [9, 12]),
    PD1: (pd1, 1, [9, 12]),
    PD2: (pd2, 2, [2, 8, 12, 13]),
    PD3: (pd3, 3, [7, 12]),
    PD4: (pd4, 4, [7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [7, 12]),
    PD7: (pd7, 7, [7, 12]),
    PD8: (pd8, 8, [7, 12]),
    PD9: (pd9, 9, [7, 12]),
    PD10: (pd10, 10, [7, 12]),
    PD11: (pd11, 11, [7, 12]),
    PD12: (pd12, 12, [2, 7, 12]),
    PD13: (pd13, 13, [2, 12]),
    PD14: (pd14, 14, [2, 12]),
    PD15: (pd15, 15, [2, 12]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 12, 13]),
    PE1: (pe1, 1, [12, 13]),
    PE2: (pe2, 2, [0, 11, 12]),
    PE3: (pe3, 3, [0, 12]),
    PE4: (pe4, 4, [0, 12, 13]),
    PE5: (pe5, 5, [0, 3, 12, 13]),
    PE6: (pe6, 6, [0, 3, 12, 13]),
    PE7: (pe7, 7, [1, 12]),
    PE8: (pe8, 8, [1, 12]),
    PE9: (pe9, 9, [1, 12]),
    PE10: (pe10, 10, [1, 12]),
    PE11: (pe11, 11, [1, 12]),
    PE12: (pe12, 12, [1, 12]),
    PE13: (pe13, 13, [1, 12]),
    PE14: (pe14, 14, [1, 12]),
    PE15: (pe15, 15, [1, 12]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 12]),
    PF7: (pf7, 7, [3, 12]),
    PF8: (pf8, 8, [9, 12]),
    PF9: (pf9, 9, [9, 12]),
    PF10: (pf10, 10, [12]),
    PF11: (pf11, 11, [13]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [12]),
    PF14: (pf14, 14, [12]),
    PF15: (pf15, 15, [12]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, [12]),
    PG7: (pg7, 7, [8, 12]),
    PG8: (pg8, 8, [8, 11]),
    PG9: (pg9, 9, [8, 12]),
    PG10: (pg10, 10, [12]),
    PG11: (pg11, 11, [11, 12]),
    PG12: (pg12, 12, [8, 12]),
    PG13: (pg13, 13, [8, 11, 12]),
    PG14: (pg14, 14, [8, 11, 12]),
    PG15: (pg15, 15, [8, 13]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [0]),
    PH1: (ph1, 1, [0]),
    PH2: (ph2, 2, [11]),
    PH3: (ph3, 3, [11]),
    PH4: (ph4, 4, [4, 10]),
    PH5: (ph5, 5, [4]),
    PH6: (ph6, 6, [4, 9, 11]),
    PH7: (ph7, 7, [4, 11]),
    PH8: (ph8, 8, [4, 13]),
    PH9: (ph9, 9, [4, 9, 13]),
    PH10: (ph10, 10, [2, 13]),
    PH11: (ph11, 11, [2, 13]),
    PH12: (ph12, 12, [2, 13]),
    PH13: (ph13, 13, [3, 9]),
    PH14: (ph14, 14, [3, 13]),
    PH15: (ph15, 15, [3, 13]),
]);

#[cfg(feature = "gpio-f417")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 13]),
    PI1: (pi1, 1, [5, 13]),
    PI2: (pi2, 2, [3, 5, 6, 13]),
    PI3: (pi3, 3, [3, 5, 13]),
    PI4: (pi4, 4, [3, 13]),
    PI5: (pi5, 5, [3, 13]),
    PI6: (pi6, 6, [3, 13]),
    PI7: (pi7, 7, [3, 13]),
    PI8: (pi8, 8, []),
    PI9: (pi9, 9, [9]),
    PI10: (pi10, 10, [11]),
    PI11: (pi11, 11, [10]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 11]),
    PA1: (pa1, 1, [1, 2, 7, 8, 11]),
    PA2: (pa2, 2, [1, 2, 3, 7, 11]),
    PA3: (pa3, 3, [1, 2, 3, 7, 10, 11, 14]),
    PA4: (pa4, 4, [5, 6, 7, 12, 13, 14]),
    PA5: (pa5, 5, [1, 3, 5, 10]),
    PA6: (pa6, 6, [1, 2, 3, 5, 9, 13, 14]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 11]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10, 14]),
    PA9: (pa9, 9, [1, 4, 7, 13]),
    PA10: (pa10, 10, [1, 7, 10, 13]),
    PA11: (pa11, 11, [1, 7, 9, 10, 14]),
    PA12: (pa12, 12, [1, 7, 9, 10, 14]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6], super::Debugger),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 9, 10, 11]),
    PB1: (pb1, 1, [1, 2, 3, 9, 10, 11]),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, [0, 1, 5, 6], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 9, 10, 11, 12, 13]),
    PB6: (pb6, 6, [2, 4, 7, 9, 12, 13]),
    PB7: (pb7, 7, [2, 4, 7, 12, 13]),
    PB8: (pb8, 8, [2, 3, 4, 9, 11, 12, 13, 14]),
    PB9: (pb9, 9, [2, 3, 4, 5, 9, 12, 13, 14]),
    PB10: (pb10, 10, [1, 4, 5, 7, 10, 11, 14]),
    PB11: (pb11, 11, [1, 4, 7, 10, 11, 14]),
    PB12: (pb12, 12, [1, 4, 5, 7, 9, 10, 11, 12]),
    PB13: (pb13, 13, [1, 5, 7, 9, 10, 11]),
    PB14: (pb14, 14, [1, 3, 5, 6, 7, 9, 12]),
    PB15: (pb15, 15, [0, 1, 3, 5, 9, 12]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [10, 12]),
    PC1: (pc1, 1, [11]),
    PC2: (pc2, 2, [5, 6, 10, 11, 12]),
    PC3: (pc3, 3, [5, 10, 11, 12]),
    PC4: (pc4, 4, [11]),
    PC5: (pc5, 5, [11]),
    PC6: (pc6, 6, [2, 3, 5, 8, 12, 13, 14]),
    PC7: (pc7, 7, [2, 3, 6, 8, 12, 13, 14]),
    PC8: (pc8, 8, [2, 3, 8, 12, 13]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 12, 13]),
    PC10: (pc10, 10, [6, 7, 8, 12, 13, 14]),
    PC11: (pc11, 11, [5, 6, 7, 8, 12, 13]),
    PC12: (pc12, 12, [6, 7, 8, 12, 13]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [9, 12]),
    PD1: (pd1, 1, [9, 12]),
    PD2: (pd2, 2, [2, 8, 12, 13]),
    PD3: (pd3, 3, [5, 7, 12, 13, 14]),
    PD4: (pd4, 4, [7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [5, 6, 7, 12, 13, 14]),
    PD7: (pd7, 7, [7, 12]),
    PD8: (pd8, 8, [7, 12]),
    PD9: (pd9, 9, [7, 12]),
    PD10: (pd10, 10, [7, 12, 14]),
    PD11: (pd11, 11, [7, 12]),
    PD12: (pd12, 12, [2, 7, 12]),
    PD13: (pd13, 13, [2, 12]),
    PD14: (pd14, 14, [2, 12]),
    PD15: (pd15, 15, [2, 12]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 8, 12, 13]),
    PE1: (pe1, 1, [8, 12, 13]),
    PE2: (pe2, 2, [0, 5, 6, 11, 12]),
    PE3: (pe3, 3, [0, 6, 12]),
    PE4: (pe4, 4, [0, 5, 6, 12, 13, 14]),
    PE5: (pe5, 5, [0, 3, 5, 6, 12, 13, 14]),
    PE6: (pe6, 6, [0, 3, 5, 6, 12, 13, 14]),
    PE7: (pe7, 7, [1, 8, 12]),
    PE8: (pe8, 8, [1, 8, 12]),
    PE9: (pe9, 9, [1, 12]),
    PE10: (pe10, 10, [1, 12]),
    PE11: (pe11, 11, [1, 5, 12, 14]),
    PE12: (pe12, 12, [1, 5, 12, 14]),
    PE13: (pe13, 13, [1, 5, 12, 14]),
    PE14: (pe14, 14, [1, 5, 12, 14]),
    PE15: (pe15, 15, [1, 12, 14]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 5, 6, 8, 12]),
    PF7: (pf7, 7, [3, 5, 6, 8, 12]),
    PF8: (pf8, 8, [5, 6, 9, 12]),
    PF9: (pf9, 9, [5, 6, 9, 12]),
    PF10: (pf10, 10, [12, 13, 14]),
    PF11: (pf11, 11, [5, 12, 13]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [12]),
    PF14: (pf14, 14, [12]),
    PF15: (pf15, 15, [12]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, [12, 13, 14]),
    PG7: (pg7, 7, [8, 12, 13, 14]),
    PG8: (pg8, 8, [5, 8, 11, 12]),
    PG9: (pg9, 9, [8, 12, 13]),
    PG10: (pg10, 10, [9, 12, 13, 14]),
    PG11: (pg11, 11, [11, 12, 13, 14]),
    PG12: (pg12, 12, [5, 8, 9, 12, 14]),
    PG13: (pg13, 13, [5, 8, 11, 12]),
    PG14: (pg14, 14, [5, 8, 11, 12]),
    PG15: (pg15, 15, [8, 12, 13]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
    PH2: (ph2, 2, [11, 12, 14]),
    PH3: (ph3, 3, [11, 12, 14]),
    PH4: (ph4, 4, [4, 10]),
    PH5: (ph5, 5, [4, 5, 12]),
    PH6: (ph6, 6, [4, 5, 9, 11, 12, 13]),
    PH7: (ph7, 7, [4, 5, 11, 12, 13]),
    PH8: (ph8, 8, [4, 12, 13, 14]),
    PH9: (ph9, 9, [4, 9, 12, 13, 14]),
    PH10: (ph10, 10, [2, 12, 13, 14]),
    PH11: (ph11, 11, [2, 12, 13, 14]),
    PH12: (ph12, 12, [2, 12, 13, 14]),
    PH13: (ph13, 13, [3, 9, 12, 14]),
    PH14: (ph14, 14, [3, 12, 13, 14]),
    PH15: (ph15, 15, [3, 12, 13, 14]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 12, 13, 14]),
    PI1: (pi1, 1, [5, 12, 13, 14]),
    PI2: (pi2, 2, [3, 5, 6, 12, 13, 14]),
    PI3: (pi3, 3, [3, 5, 12, 13]),
    PI4: (pi4, 4, [3, 12, 13, 14]),
    PI5: (pi5, 5, [3, 12, 13, 14]),
    PI6: (pi6, 6, [3, 12, 13, 14]),
    PI7: (pi7, 7, [3, 12, 13, 14]),
    PI8: (pi8, 8, []),
    PI9: (pi9, 9, [9, 12, 14]),
    PI10: (pi10, 10, [11, 12, 14]),
    PI11: (pi11, 11, [10]),
    PI12: (pi12, 12, [14]),
    PI13: (pi13, 13, [14]),
    PI14: (pi14, 14, [14]),
    PI15: (pi15, 15, [14]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOJ, gpioj, PJ, 'J', PJn, [
    PJ0: (pj0, 0, [14]),
    PJ1: (pj1, 1, [14]),
    PJ2: (pj2, 2, [14]),
    PJ3: (pj3, 3, [14]),
    PJ4: (pj4, 4, [14]),
    PJ5: (pj5, 5, [14]),
    PJ6: (pj6, 6, [14]),
    PJ7: (pj7, 7, [14]),
    PJ8: (pj8, 8, [14]),
    PJ9: (pj9, 9, [14]),
    PJ10: (pj10, 10, [14]),
    PJ11: (pj11, 11, [14]),
    PJ12: (pj12, 12, [14]),
    PJ13: (pj13, 13, [14]),
    PJ14: (pj14, 14, [14]),
    PJ15: (pj15, 15, [14]),
]);

#[cfg(feature = "gpio-f427")]
gpio!(GPIOK, gpiok, PK, 'K', PKn, [
    PK0: (pk0, 0, [14]),
    PK1: (pk1, 1, [14]),
    PK2: (pk2, 2, [14]),
    PK3: (pk3, 3, [14]),
    PK4: (pk4, 4, [14]),
    PK5: (pk5, 5, [14]),
    PK6: (pk6, 6, [14]),
    PK7: (pk7, 7, [14]),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8]),
    PA1: (pa1, 1, [1, 2, 7, 8, 9, 10]),
    PA2: (pa2, 2, [1, 2, 3, 7, 8]),
    PA3: (pa3, 3, [1, 2, 3, 6, 7, 10]),
    PA4: (pa4, 4, [5, 6, 7, 12, 13]),
    PA5: (pa5, 5, [1, 3, 5, 10]),
    PA6: (pa6, 6, [1, 2, 3, 5, 6, 9, 13]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 12]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10]),
    PA9: (pa9, 9, [1, 4, 5, 6, 7, 13]),
    PA10: (pa10, 10, [1, 7, 10, 13]),
    PA11: (pa11, 11, [1, 7, 9, 10]),
    PA12: (pa12, 12, [1, 7, 8, 9, 10]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 4, 5, 6, 8], super::Debugger),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 7, 8, 10, 12]),
    PB1: (pb1, 1, [1, 2, 3, 10, 12]),
    PB2: (pb2, 2, [1, 6, 7, 9, 10, 12]),
    PB3: (pb3, 3, [0, 1, 4, 5, 6], super::Debugger),
    PB4: (pb4, 4, [0, 2, 4, 5, 6, 7], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 9, 10, 12, 13]),
    PB6: (pb6, 6, [2, 3, 4, 7, 9, 10, 12, 13]),
    PB7: (pb7, 7, [2, 4, 7, 8, 12, 13]),
    PB8: (pb8, 8, [1, 2, 3, 4, 9, 12, 13]),
    PB9: (pb9, 9, [1, 2, 3, 4, 5, 6, 9, 12, 13]),
    PB10: (pb10, 10, [1, 4, 5, 6, 7, 10]),
    PB11: (pb11, 11, [1, 4, 7, 8, 10]),
    PB12: (pb12, 12, [1, 4, 5, 6, 7, 9, 10, 12]),
    PB13: (pb13, 13, [1, 5, 7, 9, 10]),
    PB14: (pb14, 14, [1, 3, 5, 7, 9, 12]),
    PB15: (pb15, 15, [0, 1, 3, 5, 9, 12]),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [6, 10, 12]),
    PC1: (pc1, 1, [5, 6, 7]),
    PC2: (pc2, 2, [5, 10, 12]),
    PC3: (pc3, 3, [5, 10, 12]),
    PC4: (pc4, 4, [5, 8, 12]),
    PC5: (pc5, 5, [7, 8, 12]),
    PC6: (pc6, 6, [2, 3, 4, 5, 8, 12, 13]),
    PC7: (pc7, 7, [2, 3, 4, 5, 6, 7, 8, 12, 13]),
    PC8: (pc8, 8, [0, 2, 3, 7, 8, 12, 13]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 7, 9, 12, 13]),
    PC10: (pc10, 10, [6, 7, 8, 9, 12, 13]),
    PC11: (pc11, 11, [6, 7, 8, 9, 12, 13]),
    PC12: (pc12, 12, [4, 6, 7, 8, 12, 13]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [5, 6, 9, 12]),
    PD1: (pd1, 1, [7, 9, 12]),
    PD2: (pd2, 2, [2, 8, 12, 13]),
    PD3: (pd3, 3, [0, 5, 7, 9, 12, 13]),
    PD4: (pd4, 4, [7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [5, 6, 7, 12, 13]),
    PD7: (pd7, 7, [7, 8, 12]),
    PD8: (pd8, 8, [7, 8, 12]),
    PD9: (pd9, 9, [7, 12]),
    PD10: (pd10, 10, [7, 12]),
    PD11: (pd11, 11, [4, 7, 9, 10, 12]),
    PD12: (pd12, 12, [2, 4, 7, 9, 10, 12]),
    PD13: (pd13, 13, [2, 4, 9, 10, 12]),
    PD14: (pd14, 14, [2, 4, 8, 12]),
    PD15: (pd15, 15, [2, 4, 12]),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 10, 12, 13]),
    PE1: (pe1, 1, [12, 13]),
    PE2: (pe2, 2, [0, 5, 6, 9, 12]),
    PE3: (pe3, 3, [0, 6, 12]),
    PE4: (pe4, 4, [0, 5, 6, 12, 13]),
    PE5: (pe5, 5, [0, 3, 5, 6, 12, 13]),
    PE6: (pe6, 6, [0, 3, 5, 6, 12, 13]),
    PE7: (pe7, 7, [1, 8, 10, 12]),
    PE8: (pe8, 8, [1, 8, 10, 12]),
    PE9: (pe9, 9, [1, 10, 12]),
    PE10: (pe10, 10, [1, 10, 12]),
    PE11: (pe11, 11, [1, 5, 10, 12]),
    PE12: (pe12, 12, [1, 5, 10, 12]),
    PE13: (pe13, 13, [1, 5, 10, 12]),
    PE14: (pe14, 14, [1, 5, 10, 12]),
    PE15: (pe15, 15, [1, 12]),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 6, 9]),
    PF7: (pf7, 7, [3, 6, 9]),
    PF8: (pf8, 8, [6, 9, 10]),
    PF9: (pf9, 9, [6, 9, 10]),
    PF10: (pf10, 10, [13]),
    PF11: (pf11, 11, [10, 12, 13]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [4, 12]),
    PF14: (pf14, 14, [4, 12]),
    PF15: (pf15, 15, [4, 12]),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, [10, 13]),
    PG7: (pg7, 7, [8, 12, 13]),
    PG8: (pg8, 8, [7, 8, 12]),
    PG9: (pg9, 9, [7, 8, 9, 10, 12, 13]),
    PG10: (pg10, 10, [10, 12, 13]),
    PG11: (pg11, 11, [6, 7, 13]),
    PG12: (pg12, 12, [6, 7, 8, 12]),
    PG13: (pg13, 13, [0, 6, 8, 12]),
    PG14: (pg14, 14, [0, 6, 8, 9, 12]),
    PG15: (pg15, 15, [8, 12, 13]),
]);

#[cfg(feature = "gpio-f446")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 11]),
    PA1: (pa1, 1, [1, 2, 7, 8, 9, 11, 14]),
    PA2: (pa2, 2, [1, 2, 3, 7, 11, 14]),
    PA3: (pa3, 3, [1, 2, 3, 7, 9, 10, 11, 14]),
    PA4: (pa4, 4, [5, 6, 7, 12, 13, 14]),
    PA5: (pa5, 5, [1, 3, 5, 10, 14]),
    PA6: (pa6, 6, [1, 2, 3, 5, 9, 13, 14]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 10, 11, 12]),
    PA8: (pa8, 8, [0, 1, 4, 7, 10, 14]),
    PA9: (pa9, 9, [1, 4, 5, 7, 13]),
    PA10: (pa10, 10, [1, 7, 10, 13]),
    PA11: (pa11, 11, [1, 7, 9, 10, 14]),
    PA12: (pa12, 12, [1, 7, 9, 10, 14]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6], super::Debugger),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 9, 10, 11, 14]),
    PB1: (pb1, 1, [1, 2, 3, 9, 10, 11, 14]),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, [0, 1, 5, 6], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 9, 10, 11, 12, 13, 14]),
    PB6: (pb6, 6, [2, 4, 7, 9, 10, 12, 13]),
    PB7: (pb7, 7, [2, 4, 7, 12, 13]),
    PB8: (pb8, 8, [2, 3, 4, 9, 11, 12, 13, 14]),
    PB9: (pb9, 9, [2, 3, 4, 5, 9, 12, 13, 14]),
    PB10: (pb10, 10, [1, 4, 5, 7, 9, 10, 11, 14]),
    PB11: (pb11, 11, [1, 4, 7, 10, 11, 13, 14]),
    PB12: (pb12, 12, [1, 4, 5, 7, 9, 10, 11, 12]),
    PB13: (pb13, 13, [1, 5, 7, 9, 10, 11]),
    PB14: (pb14, 14, [1, 3, 5, 6, 7, 9, 12]),
    PB15: (pb15, 15, [0, 1, 3, 5, 9, 12]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [10, 12, 14]),
    PC1: (pc1, 1, [0, 5, 6, 11]),
    PC2: (pc2, 2, [5, 6, 10, 11, 12]),
    PC3: (pc3, 3, [5, 10, 11, 12]),
    PC4: (pc4, 4, [11, 12]),
    PC5: (pc5, 5, [11, 12]),
    PC6: (pc6, 6, [2, 3, 5, 8, 12, 13, 14]),
    PC7: (pc7, 7, [2, 3, 6, 8, 12, 13, 14]),
    PC8: (pc8, 8, [0, 2, 3, 8, 12, 13]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 9, 12, 13]),
    PC10: (pc10, 10, [6, 7, 8, 9, 12, 13, 14]),
    PC11: (pc11, 11, [5, 6, 7, 8, 9, 12, 13]),
    PC12: (pc12, 12, [0, 6, 7, 8, 12, 13]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [9, 12]),
    PD1: (pd1, 1, [9, 12]),
    PD2: (pd2, 2, [0, 2, 8, 12, 13]),
    PD3: (pd3, 3, [5, 7, 12, 13, 14]),
    PD4: (pd4, 4, [7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [5, 6, 7, 12, 13, 14]),
    PD7: (pd7, 7, [7, 12]),
    PD8: (pd8, 8, [7, 12]),
    PD9: (pd9, 9, [7, 12]),
    PD10: (pd10, 10, [7, 12, 14]),
    PD11: (pd11, 11, [7, 9, 12]),
    PD12: (pd12, 12, [2, 7, 9, 12]),
    PD13: (pd13, 13, [2, 9, 12]),
    PD14: (pd14, 14, [2, 12]),
    PD15: (pd15, 15, [2, 12]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 8, 12, 13]),
    PE1: (pe1, 1, [8, 12, 13]),
    PE2: (pe2, 2, [0, 5, 6, 9, 11, 12]),
    PE3: (pe3, 3, [0, 6, 12]),
    PE4: (pe4, 4, [0, 5, 6, 12, 13, 14]),
    PE5: (pe5, 5, [0, 3, 5, 6, 12, 13, 14]),
    PE6: (pe6, 6, [0, 3, 5, 6, 12, 13, 14]),
    PE7: (pe7, 7, [1, 8, 10, 12]),
    PE8: (pe8, 8, [1, 8, 10, 12]),
    PE9: (pe9, 9, [1, 10, 12]),
    PE10: (pe10, 10, [1, 10, 12]),
    PE11: (pe11, 11, [1, 5, 12, 14]),
    PE12: (pe12, 12, [1, 5, 12, 14]),
    PE13: (pe13, 13, [1, 5, 12, 14]),
    PE14: (pe14, 14, [1, 5, 12, 14]),
    PE15: (pe15, 15, [1, 12, 14]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 5, 6, 8, 9]),
    PF7: (pf7, 7, [3, 5, 6, 8, 9]),
    PF8: (pf8, 8, [5, 6, 9, 10]),
    PF9: (pf9, 9, [5, 6, 9, 10]),
    PF10: (pf10, 10, [9, 13, 14]),
    PF11: (pf11, 11, [5, 12, 13]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [12]),
    PF14: (pf14, 14, [12]),
    PF15: (pf15, 15, [12]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, [13, 14]),
    PG7: (pg7, 7, [6, 8, 12, 13, 14]),
    PG8: (pg8, 8, [5, 8, 11, 12, 14]),
    PG9: (pg9, 9, [8, 9, 12, 13]),
    PG10: (pg10, 10, [9, 12, 13, 14]),
    PG11: (pg11, 11, [11, 13, 14]),
    PG12: (pg12, 12, [5, 8, 9, 12, 14]),
    PG13: (pg13, 13, [0, 5, 8, 11, 12, 14]),
    PG14: (pg14, 14, [0, 5, 8, 9, 11, 12, 14]),
    PG15: (pg15, 15, [8, 12, 13]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
    PH2: (ph2, 2, [9, 11, 12, 14]),
    PH3: (ph3, 3, [9, 11, 12, 14]),
    PH4: (ph4, 4, [4, 9, 10, 14]),
    PH5: (ph5, 5, [4, 5, 12]),
    PH6: (ph6, 6, [4, 5, 9, 11, 12, 13]),
    PH7: (ph7, 7, [4, 5, 11, 12, 13]),
    PH8: (ph8, 8, [4, 12, 13, 14]),
    PH9: (ph9, 9, [4, 9, 12, 13, 14]),
    PH10: (ph10, 10, [2, 12, 13, 14]),
    PH11: (ph11, 11, [2, 12, 13, 14]),
    PH12: (ph12, 12, [2, 12, 13, 14]),
    PH13: (ph13, 13, [3, 9, 12, 14]),
    PH14: (ph14, 14, [3, 12, 13, 14]),
    PH15: (ph15, 15, [3, 12, 13, 14]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 12, 13, 14]),
    PI1: (pi1, 1, [5, 12, 13, 14]),
    PI2: (pi2, 2, [3, 5, 6, 12, 13, 14]),
    PI3: (pi3, 3, [3, 5, 12, 13]),
    PI4: (pi4, 4, [3, 12, 13, 14]),
    PI5: (pi5, 5, [3, 12, 13, 14]),
    PI6: (pi6, 6, [3, 12, 13, 14]),
    PI7: (pi7, 7, [3, 12, 13, 14]),
    PI8: (pi8, 8, []),
    PI9: (pi9, 9, [9, 12, 14]),
    PI10: (pi10, 10, [11, 12, 14]),
    PI11: (pi11, 11, [9, 10]),
    PI12: (pi12, 12, [14]),
    PI13: (pi13, 13, [14]),
    PI14: (pi14, 14, [14]),
    PI15: (pi15, 15, [9, 14]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOJ, gpioj, PJ, 'J', PJn, [
    PJ0: (pj0, 0, [9, 14]),
    PJ1: (pj1, 1, [14]),
    PJ2: (pj2, 2, [13, 14]),
    PJ3: (pj3, 3, [14]),
    PJ4: (pj4, 4, [14]),
    PJ5: (pj5, 5, [14]),
    PJ12: (pj12, 12, [9, 14]),
    PJ13: (pj13, 13, [9, 14]),
    PJ14: (pj14, 14, [14]),
    PJ15: (pj15, 15, [14]),
]);

#[cfg(feature = "gpio-f469")]
gpio!(GPIOK, gpiok, PK, 'K', PKn, [
    PK3: (pk3, 3, [14]),
    PK4: (pk4, 4, [14]),
    PK5: (pk5, 5, [14]),
    PK6: (pk6, 6, [14]),
    PK7: (pk7, 7, [14]),
]);

/*#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f446"
))]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);*/

struct Gpio<const P: char>;
impl<const P: char> Gpio<P> {
    const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
        match P {
            'A' => crate::pac::GPIOA::ptr(),
            'B' => crate::pac::GPIOB::ptr() as _,
            'C' => crate::pac::GPIOC::ptr() as _,
            #[cfg(feature = "gpiod")]
            'D' => crate::pac::GPIOD::ptr() as _,
            #[cfg(feature = "gpioe")]
            'E' => crate::pac::GPIOE::ptr() as _,
            #[cfg(feature = "gpiof")]
            'F' => crate::pac::GPIOF::ptr() as _,
            #[cfg(feature = "gpiog")]
            'G' => crate::pac::GPIOG::ptr() as _,
            'H' => crate::pac::GPIOH::ptr() as _,
            #[cfg(feature = "gpioi")]
            'I' => crate::pac::GPIOI::ptr() as _,
            #[cfg(feature = "gpioj")]
            'J' => crate::pac::GPIOJ::ptr() as _,
            #[cfg(feature = "gpiok")]
            'K' => crate::pac::GPIOK::ptr() as _,
            _ => panic!("Unknown GPIO port"),
        }
    }
}
