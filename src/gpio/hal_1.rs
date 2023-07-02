use core::convert::Infallible;

use super::{
    dynamic::PinModeError, marker, DynamicPin, ErasedPin, Output, PartiallyErasedPin, Pin,
};

pub use embedded_hal_one::digital::PinState;
use embedded_hal_one::digital::{
    ErrorType, InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin,
};

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

impl<const P: char, const N: u8, MODE> InputPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
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

impl<MODE> InputPin for ErasedPin<MODE>
where
    MODE: marker::Readable,
{
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

impl<const P: char, MODE> InputPin for PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

// Implementations for `DynamicPin
impl<const P: char, const N: u8> ErrorType for DynamicPin<P, N> {
    type Error = PinModeError;
}

impl<const P: char, const N: u8> OutputPin for DynamicPin<P, N> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high()
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low()
    }
}

impl<const P: char, const N: u8> InputPin for DynamicPin<P, N> {
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_high()
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}
