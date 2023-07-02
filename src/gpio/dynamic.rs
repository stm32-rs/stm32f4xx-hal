use super::*;
use embedded_hal_one::digital::ErrorKind;

/// Pin type with dynamic mode
///
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `15`.
pub struct DynamicPin<const P: char, const N: u8> {
    /// Current pin mode
    pub(crate) mode: Dynamic,
}

/// Tracks the current pin state for dynamic pins
pub enum Dynamic {
    /// Floating input mode
    InputFloating,
    /// Pull-up input mode
    InputPullUp,
    /// Pull-down input mode
    InputPullDown,
    /// Push-pull output mode
    OutputPushPull,
    /// Open-drain output mode
    OutputOpenDrain,
}

/// Error for [DynamicPin]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinModeError {
    /// For operations unsupported in current mode
    IncorrectMode,
}

impl embedded_hal_one::digital::Error for PinModeError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl Dynamic {
    /// Is pin in readable mode
    pub fn is_input(&self) -> bool {
        use Dynamic::*;
        match self {
            InputFloating | InputPullUp | InputPullDown | OutputOpenDrain => true,
            OutputPushPull => false,
        }
    }

    /// Is pin in writable mode
    pub fn is_output(&self) -> bool {
        use Dynamic::*;
        match self {
            InputFloating | InputPullUp | InputPullDown => false,
            OutputPushPull | OutputOpenDrain => true,
        }
    }
}

// For conversion simplify
struct Unknown;

impl crate::Sealed for Unknown {}
impl PinMode for Unknown {}

impl<const P: char, const N: u8> DynamicPin<P, N> {
    pub(super) const fn new(mode: Dynamic) -> Self {
        Self { mode }
    }

    /// Switch pin into pull-up input
    #[inline]
    pub fn make_pull_up_input(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_pull_up_input();
        self.mode = Dynamic::InputPullUp;
    }
    /// Switch pin into pull-down input
    #[inline]
    pub fn make_pull_down_input(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_pull_down_input();
        self.mode = Dynamic::InputPullDown;
    }
    /// Switch pin into floating input
    #[inline]
    pub fn make_floating_input(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_floating_input();
        self.mode = Dynamic::InputFloating;
    }
    /// Switch pin into push-pull output
    #[inline]
    pub fn make_push_pull_output(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_push_pull_output();
        self.mode = Dynamic::OutputPushPull;
    }
    /// Switch pin into push-pull output with required voltage state
    #[inline]
    pub fn make_push_pull_output_in_state(&mut self, state: PinState) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_push_pull_output_in_state(state);
        self.mode = Dynamic::OutputPushPull;
    }
    /// Switch pin into open-drain output
    #[inline]
    pub fn make_open_drain_output(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_open_drain_output();
        self.mode = Dynamic::OutputOpenDrain;
    }
    /// Switch pin into open-drain output with required voltage state
    #[inline]
    pub fn make_open_drain_output_in_state(&mut self, state: PinState) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_open_drain_output_in_state(state);
        self.mode = Dynamic::OutputOpenDrain;
    }

    /// Drives the pin high
    pub fn set_high(&mut self) -> Result<(), PinModeError> {
        if self.mode.is_output() {
            Pin::<P, N, Unknown>::new()._set_state(PinState::High);
            Ok(())
        } else {
            Err(PinModeError::IncorrectMode)
        }
    }

    /// Drives the pin low
    pub fn set_low(&mut self) -> Result<(), PinModeError> {
        if self.mode.is_output() {
            Pin::<P, N, Unknown>::new()._set_state(PinState::Low);
            Ok(())
        } else {
            Err(PinModeError::IncorrectMode)
        }
    }

    /// Is the input pin high?
    pub fn is_high(&self) -> Result<bool, PinModeError> {
        self.is_low().map(|b| !b)
    }

    /// Is the input pin low?
    pub fn is_low(&self) -> Result<bool, PinModeError> {
        if self.mode.is_input() {
            Ok(Pin::<P, N, Unknown>::new()._is_low())
        } else {
            Err(PinModeError::IncorrectMode)
        }
    }
}
