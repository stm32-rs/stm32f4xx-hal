use core::convert::Infallible;

use super::{dynamic::PinModeError, marker, AnyPin, DynamicPin, Output, PartiallyErasedPin, Pin};

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

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
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_high(self))
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_low(self))
    }
}

impl<const P: char, const N: u8, MODE> InputPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_high(self))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_low(self))
    }
}

// Implementations for `ErasedPin`
impl<MODE> ErrorType for AnyPin<MODE> {
    type Error = core::convert::Infallible;
}

impl<MODE> OutputPin for AnyPin<Output<MODE>> {
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

impl<MODE> StatefulOutputPin for AnyPin<Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_high(self))
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_low(self))
    }
}

impl<MODE> InputPin for AnyPin<MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_high(self))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_low(self))
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
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_high(self))
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_low(self))
    }
}

impl<const P: char, MODE> InputPin for PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_high(self))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_low(self))
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
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Self::is_high(self)
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Self::is_low(self)
    }
}
