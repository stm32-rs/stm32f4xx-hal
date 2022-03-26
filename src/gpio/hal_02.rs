use core::convert::Infallible;

use super::{
    dynamic::PinModeError, DynamicPin, ErasedPin, Input, OpenDrain, Output, PartiallyErasedPin,
    Pin, PinState, PushPull,
};

use embedded_hal::digital::v2::{
    InputPin, IoPin, OutputPin, StatefulOutputPin, ToggleableOutputPin,
};

// Implementations for `Pin`

impl<const P: char, const N: u8, MODE> OutputPin for Pin<P, N, Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<const P: char, const N: u8, MODE> StatefulOutputPin for Pin<P, N, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<const P: char, const N: u8, MODE> ToggleableOutputPin for Pin<P, N, Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char, const N: u8> InputPin for Pin<P, N, Output<OpenDrain>> {
    type Error = Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl<const P: char, const N: u8> InputPin for Pin<P, N, Input> {
    type Error = Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl<const P: char, const N: u8> IoPin<Self, Self> for Pin<P, N, Output<OpenDrain>> {
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(state);
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Pin<P, N, Input>, Self> for Pin<P, N, Output<OpenDrain>> {
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<P, N, Input>, Self::Error> {
        Ok(self.into_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(state);
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<P, N, Output<OpenDrain>>> for Pin<P, N, Input> {
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<P, N, Output<OpenDrain>>, Self::Error> {
        Ok(self.into_open_drain_output_in_state(state))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<P, N, Input>, Self> for Pin<P, N, Output<PushPull>> {
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<P, N, Input>, Self::Error> {
        Ok(self.into_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(state);
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<P, N, Output<PushPull>>> for Pin<P, N, Input> {
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<P, N, Output<PushPull>>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(state))
    }
}

// Implementations for `ErasedPin`

impl<MODE> OutputPin for ErasedPin<Output<MODE>> {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<MODE> StatefulOutputPin for ErasedPin<Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<MODE> ToggleableOutputPin for ErasedPin<Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl InputPin for ErasedPin<Output<OpenDrain>> {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl InputPin for ErasedPin<Input> {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

// Implementations for `PartiallyErasedPin`

impl<const P: char, MODE> OutputPin for PartiallyErasedPin<P, Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<const P: char, MODE> StatefulOutputPin for PartiallyErasedPin<P, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<const P: char, MODE> ToggleableOutputPin for PartiallyErasedPin<P, Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char> InputPin for PartiallyErasedPin<P, Output<OpenDrain>> {
    type Error = Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl<const P: char> InputPin for PartiallyErasedPin<P, Input> {
    type Error = Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

// Implementations for `DynamicPin`

impl<const P: char, const N: u8> OutputPin for DynamicPin<P, N> {
    type Error = PinModeError;
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high()
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low()
    }
}

impl<const P: char, const N: u8> InputPin for DynamicPin<P, N> {
    type Error = PinModeError;
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_high()
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}
