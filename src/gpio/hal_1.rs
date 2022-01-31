use core::convert::Infallible;

use super::{
    ErasedPin, Floating, Input, OpenDrain, Output, PartiallyErasedPin, Pin, PullDown, PullUp,
    PushPull,
};

use embedded_hal_one::digital::blocking::{
    InputPin, IoPin, OutputPin, StatefulOutputPin, ToggleableOutputPin,
};
pub use embedded_hal_one::digital::PinState;

fn into_state(state: PinState) -> super::PinState {
    match state {
        PinState::Low => super::PinState::Low,
        PinState::High => super::PinState::High,
    }
}

// Implementations for `Pin`

impl<MODE, const P: char, const N: u8> OutputPin for Pin<Output<MODE>, P, N> {
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

impl<MODE, const P: char, const N: u8> StatefulOutputPin for Pin<Output<MODE>, P, N> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<MODE, const P: char, const N: u8> ToggleableOutputPin for Pin<Output<MODE>, P, N> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char, const N: u8> InputPin for Pin<Output<OpenDrain>, P, N> {
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

impl<MODE, const P: char, const N: u8> InputPin for Pin<Input<MODE>, P, N> {
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

impl<const P: char, const N: u8> IoPin<Self, Self> for Pin<Output<OpenDrain>, P, N> {
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Pin<Input<Floating>, P, N>, Self>
    for Pin<Output<OpenDrain>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<Input<Floating>, P, N>, Self::Error> {
        Ok(self.into_floating_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<Output<OpenDrain>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<Output<OpenDrain>, P, N>, Self::Error> {
        Ok(self.into_open_drain_output_in_state(into_state(state)))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<Input<Floating>, P, N>, Self>
    for Pin<Output<PushPull>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<Input<Floating>, P, N>, Self::Error> {
        Ok(self.into_floating_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<Output<PushPull>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<Output<PushPull>, P, N>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(into_state(state)))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<Input<PullUp>, P, N>, Self>
    for Pin<Output<PushPull>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<Input<PullUp>, P, N>, Self::Error> {
        Ok(self.into_pull_up_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<Output<PushPull>, P, N>>
    for Pin<Input<PullUp>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<Output<PushPull>, P, N>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(into_state(state)))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<Input<PullDown>, P, N>, Self>
    for Pin<Output<PushPull>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<Input<PullDown>, P, N>, Self::Error> {
        Ok(self.into_pull_down_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<Output<PushPull>, P, N>>
    for Pin<Input<PullDown>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<Output<PushPull>, P, N>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(into_state(state)))
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

impl<MODE> InputPin for ErasedPin<Input<MODE>> {
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

impl<MODE, const P: char> OutputPin for PartiallyErasedPin<Output<MODE>, P> {
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

impl<MODE, const P: char> StatefulOutputPin for PartiallyErasedPin<Output<MODE>, P> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<MODE, const P: char> ToggleableOutputPin for PartiallyErasedPin<Output<MODE>, P> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char> InputPin for PartiallyErasedPin<Output<OpenDrain>, P> {
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

impl<MODE, const P: char> InputPin for PartiallyErasedPin<Input<MODE>, P> {
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
