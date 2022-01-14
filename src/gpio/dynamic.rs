use super::*;

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
    InputFloating,
    InputPullUp,
    InputPullDown,
    OutputPushPull,
    OutputOpenDrain,
}

#[derive(Debug, PartialEq)]
pub enum PinModeError {
    IncorrectMode,
}

impl Dynamic {
    pub fn is_input(&self) -> bool {
        use Dynamic::*;
        match self {
            InputFloating | InputPullUp | InputPullDown | OutputOpenDrain => true,
            OutputPushPull => false,
        }
    }
    pub fn is_output(&self) -> bool {
        use Dynamic::*;
        match self {
            InputFloating | InputPullUp | InputPullDown => false,
            OutputPushPull | OutputOpenDrain => true,
        }
    }
}

// For convertion simplify
struct Unknown;

impl<const P: char, const N: u8> DynamicPin<P, N> {
    pub const fn new(mode: Dynamic) -> Self {
        Self { mode }
    }

    #[inline]
    pub fn make_pull_up_input(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_pull_up_input();
        self.mode = Dynamic::InputPullUp;
    }
    #[inline]
    pub fn make_pull_down_input(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_pull_down_input();
        self.mode = Dynamic::InputPullDown;
    }
    #[inline]
    pub fn make_floating_input(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_floating_input();
        self.mode = Dynamic::InputFloating;
    }
    #[inline]
    pub fn make_push_pull_output(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_push_pull_output();
        self.mode = Dynamic::OutputPushPull;
    }
    #[inline]
    pub fn make_push_pull_output_in_state(&mut self, state: PinState) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_push_pull_output_in_state(state);
        self.mode = Dynamic::OutputPushPull;
    }
    #[inline]
    pub fn make_open_drain_output(&mut self) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_open_drain_output();
        self.mode = Dynamic::OutputOpenDrain;
    }
    #[inline]
    pub fn make_open_drain_output_in_state(&mut self, state: PinState) {
        // NOTE(unsafe), we have a mutable reference to the current pin
        Pin::<P, N, Unknown>::new().into_open_drain_output_in_state(state);
        self.mode = Dynamic::OutputOpenDrain;
    }

    pub fn set_high(&mut self) -> Result<(), PinModeError> {
        if self.mode.is_output() {
            Pin::<P, N, Unknown>::new()._set_state(PinState::High);
            Ok(())
        } else {
            Err(PinModeError::IncorrectMode)
        }
    }
    pub fn set_low(&mut self) -> Result<(), PinModeError> {
        if self.mode.is_output() {
            Pin::<P, N, Unknown>::new()._set_state(PinState::Low);
            Ok(())
        } else {
            Err(PinModeError::IncorrectMode)
        }
    }

    pub fn is_high(&self) -> Result<bool, PinModeError> {
        self.is_low().map(|b| !b)
    }
    pub fn is_low(&self) -> Result<bool, PinModeError> {
        if self.mode.is_input() {
            Ok(Pin::<P, N, Unknown>::new()._is_low())
        } else {
            Err(PinModeError::IncorrectMode)
        }
    }
}
