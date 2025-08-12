use core::ops::{Deref, DerefMut};

use super::*;
use embedded_hal::digital::ErrorKind;

pub type DynamicPin<const P: char, const N: u8> = DynPin<Pin<P, N, Unknown>>;
pub type DynamicAnyPin = DynPin<AnyPin<Unknown>>;

/// Pin type with dynamic mode
pub struct DynPin<PIN> {
    pin: PIN,
    /// Current pin mode
    pub(crate) mode: Dynamic,
}

/// Tracks the current pin state for dynamic pins
pub enum Dynamic {
    /// Input mode
    Input,
    /// Push-pull output mode
    OutputPushPull,
    /// Open-drain output mode
    OutputOpenDrain,
}

pub trait DynamicMode {
    const MODE: Dynamic;
}

impl DynamicMode for Input {
    const MODE: Dynamic = Dynamic::Input;
}

impl DynamicMode for Output {
    const MODE: Dynamic = Dynamic::OutputPushPull;
}

impl DynamicMode for Output<OpenDrain> {
    const MODE: Dynamic = Dynamic::OutputOpenDrain;
}

impl<PIN> Deref for DynPin<PIN> {
    type Target = PIN;
    fn deref(&self) -> &Self::Target {
        &self.pin
    }
}

impl<PIN> DerefMut for DynPin<PIN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pin
    }
}

/// Error for [DynamicPin]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinModeError {
    /// For operations unsupported in current mode
    IncorrectMode,
}

impl embedded_hal::digital::Error for PinModeError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl Dynamic {
    /// Is pin in readable mode
    pub fn is_input(&self) -> bool {
        use Dynamic::*;
        match self {
            OutputPushPull => false,
            Input | OutputOpenDrain => true,
        }
    }

    /// Is pin in writable mode
    pub fn is_output(&self) -> bool {
        use Dynamic::*;
        match self {
            Input => false,
            OutputPushPull | OutputOpenDrain => true,
        }
    }
}

// For conversion simplify
#[non_exhaustive]
pub struct Unknown;

impl crate::Sealed for Unknown {}
impl PinMode for Unknown {}
impl marker::Interruptible for Unknown {}
impl marker::OutputSpeed for Unknown {}
impl marker::Active for Unknown {}

impl<const P: char, const N: u8, MODE: DynamicMode> Pin<P, N, MODE> {
    /// Configures the pin as a pin that can change between input
    /// and output without changing the type
    pub fn into_dynamic(self) -> DynPin<Pin<P, N, Unknown>> {
        DynPin::new(Pin::new(), MODE::MODE)
    }
}

impl<const P: char, MODE: DynamicMode> PartiallyErasedPin<P, MODE> {
    /// Configures the pin as a pin that can change between input
    /// and output without changing the type
    pub fn into_dynamic(self) -> DynPin<PartiallyErasedPin<P, Unknown>> {
        DynPin::new(PartiallyErasedPin::new(self.i), MODE::MODE)
    }
}

impl<MODE: DynamicMode> AnyPin<MODE> {
    /// Configures the pin as a pin that can change between input
    /// and output without changing the type
    pub fn into_dynamic(self) -> DynPin<AnyPin<Unknown>> {
        DynPin::new(AnyPin::from_pin_port(self.into_pin_port()), MODE::MODE)
    }
}

impl<PIN> DynPin<PIN> {
    pub(super) const fn new(pin: PIN, mode: Dynamic) -> Self {
        Self { pin, mode }
    }
}

macro_rules! impldyn {
    () => {
        pub fn set_mode<MODE: PinMode + DynamicMode>(&mut self) {
            self.pin.mode::<MODE>();
            self.mode = MODE::MODE
        }

        /// Switch pin into input
        #[inline]
        pub fn make_input(&mut self) {
            self.set_mode::<Input>();
        }

        /// Switch pin into pull-up input
        #[inline]
        pub fn make_pull_up_input(&mut self) {
            self.set_mode::<Input>();
            self.set_internal_resistor(Pull::Up);
        }
        /// Switch pin into pull-down input
        #[inline]
        pub fn make_pull_down_input(&mut self) {
            self.set_mode::<Input>();
            self.set_internal_resistor(Pull::Down);
        }
        /// Switch pin into floating input
        #[inline]
        pub fn make_floating_input(&mut self) {
            self.set_mode::<Input>();
            self.set_internal_resistor(Pull::None);
        }
        /// Switch pin into push-pull output
        #[inline]
        pub fn make_push_pull_output(&mut self) {
            self.set_mode::<Output>();
        }
        /// Switch pin into push-pull output with required voltage state
        #[inline]
        pub fn make_push_pull_output_in_state(&mut self, state: PinState) {
            self.pin._set_state(state);
            self.set_mode::<Output>();
        }
        /// Switch pin into open-drain output
        #[inline]
        pub fn make_open_drain_output(&mut self) {
            self.set_mode::<Output<OpenDrain>>();
        }
        /// Switch pin into open-drain output with required voltage state
        #[inline]
        pub fn make_open_drain_output_in_state(&mut self, state: PinState) {
            self.pin._set_state(state);
            self.set_mode::<Output<OpenDrain>>();
        }

        pub fn set_state(&mut self, state: PinState) -> Result<(), PinModeError> {
            if self.mode.is_output() {
                self.pin._set_state(state);
                Ok(())
            } else {
                Err(PinModeError::IncorrectMode)
            }
        }

        /// Drives the pin high
        #[inline(always)]
        pub fn set_high(&mut self) -> Result<(), PinModeError> {
            self.set_state(PinState::High)
        }

        /// Drives the pin low
        #[inline(always)]
        pub fn set_low(&mut self) -> Result<(), PinModeError> {
            self.set_state(PinState::Low)
        }

        /// Is the input pin high?
        #[inline(always)]
        pub fn is_high(&self) -> Result<bool, PinModeError> {
            self.is_low().map(|b| !b)
        }

        /// Is the input pin low?
        pub fn is_low(&self) -> Result<bool, PinModeError> {
            if self.mode.is_input() {
                Ok(self.pin._is_low())
            } else {
                Err(PinModeError::IncorrectMode)
            }
        }
    };
}

impl<const P: char, const N: u8> DynamicPin<P, N> {
    impldyn!();
}
impl<const P: char> DynPin<PartiallyErasedPin<P, Unknown>> {
    impldyn!();
}
impl DynamicAnyPin {
    impldyn!();
}
