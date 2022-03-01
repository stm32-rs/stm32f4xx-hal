use core::convert::Infallible;

use super::{
    ErasedPin, Floating, Input, OpenDrain, Output, PartiallyErasedPin, Pin, PullDown, PullUp,
    PushPull,
};

pub use embedded_hal_one::digital::PinState;
use embedded_hal_one::digital::{
    blocking::{InputPin, IoPin, OutputPin, StatefulOutputPin, ToggleableOutputPin},
    ErrorType,
};

fn into_state(state: PinState) -> super::PinState {
    match state {
        PinState::Low => super::PinState::Low,
        PinState::High => super::PinState::High,
    }
}

// Implementations for `Pin`
impl<const P: char, const N: u8, MODE> ErrorType for Pin<P, N, MODE> {
    type Error = Infallible;
}

impl<const P: char, const N: u8, MODE> OutputPin for Pin<P, N, Output<MODE>> {
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
    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char, const N: u8> InputPin for Pin<P, N, Output<OpenDrain>> {
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl<const P: char, const N: u8, MODE> InputPin for Pin<P, N, Input<MODE>> {
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
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Pin<P, N, Input<Floating>>, Self>
    for Pin<P, N, Output<OpenDrain>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<P, N, Input<Floating>>, Self::Error> {
        Ok(self.into_floating_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<P, N, Output<OpenDrain>>>
    for Pin<P, N, Input<Floating>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<P, N, Output<OpenDrain>>, Self::Error> {
        Ok(self.into_open_drain_output_in_state(into_state(state)))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<P, N, Input<Floating>>, Self>
    for Pin<P, N, Output<PushPull>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<P, N, Input<Floating>>, Self::Error> {
        Ok(self.into_floating_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<P, N, Output<PushPull>>>
    for Pin<P, N, Input<Floating>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<P, N, Output<PushPull>>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(into_state(state)))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<P, N, Input<PullUp>>, Self>
    for Pin<P, N, Output<PushPull>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<P, N, Input<PullUp>>, Self::Error> {
        Ok(self.into_pull_up_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<P, N, Output<PushPull>>>
    for Pin<P, N, Input<PullUp>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<P, N, Output<PushPull>>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(into_state(state)))
    }
}

impl<const P: char, const N: u8> IoPin<Pin<P, N, Input<PullDown>>, Self>
    for Pin<P, N, Output<PushPull>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<P, N, Input<PullDown>>, Self::Error> {
        Ok(self.into_pull_down_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(into_state(state));
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<P, N, Output<PushPull>>>
    for Pin<P, N, Input<PullDown>>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<P, N, Output<PushPull>>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(into_state(state)))
    }
}

// Implementations for `ErasedPin`
impl<MODE> ErrorType for ErasedPin<MODE> {
    type Error = core::convert::Infallible;
}

impl<MODE> OutputPin for ErasedPin<Output<MODE>> {
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
    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl InputPin for ErasedPin<Output<OpenDrain>> {
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
impl<const P: char, MODE> ErrorType for PartiallyErasedPin<P, MODE> {
    type Error = Infallible;
}

impl<const P: char, MODE> OutputPin for PartiallyErasedPin<P, Output<MODE>> {
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
    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char> InputPin for PartiallyErasedPin<P, Output<OpenDrain>> {
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl<const P: char, MODE> InputPin for PartiallyErasedPin<P, Input<MODE>> {
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}
