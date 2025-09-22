use super::*;

pub use PartiallyErasedPin as PEPin;

/// Partially erased pin
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
pub struct PartiallyErasedPin<const P: char, MODE> {
    pub(crate) i: u8,
    _mode: PhantomData<MODE>,
}

impl<const P: char, MODE> PartiallyErasedPin<P, MODE> {
    pub(crate) fn new(i: u8) -> Self {
        Self {
            i,
            _mode: PhantomData,
        }
    }

    /// Convert partially type erased pin to `Pin` with fixed type
    pub fn restore<const N: u8>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.i, N);
        Pin::new()
    }
}

impl<const P: char, MODE> fmt::Debug for PartiallyErasedPin<P, MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P{}({})<{}>",
            P,
            self.i,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<const P: char, MODE> defmt::Format for PartiallyErasedPin<P, MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P{}({})<{}>",
            P,
            self.i,
            crate::stripped_type_name::<MODE>()
        );
    }
}

impl<const P: char, MODE> PinExt for PartiallyErasedPin<P, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        self.i
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}

impl<const P: char, MODE> PartiallyErasedPin<P, Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { &*gpiox::<P>() }
            .bsrr()
            .write(|w| w.bs(self.i).set_bit());
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { &*gpiox::<P>() }
            .bsrr()
            .write(|w| w.br(self.i).set_bit());
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
        // NOTE(unsafe) atomic read with no side effects
        unsafe { &*gpiox::<P>() }
            .odr()
            .read()
            .odr(self.i)
            .bit_is_clear()
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

impl<const P: char, MODE> PartiallyErasedPin<P, MODE>
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
        // NOTE(unsafe) atomic read with no side effects
        unsafe { &*gpiox::<P>() }
            .idr()
            .read()
            .idr(self.i)
            .bit_is_clear()
    }
}

impl<const P: char, MODE> From<PartiallyErasedPin<P, MODE>> for AnyPin<MODE> {
    /// Partially erased pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: PartiallyErasedPin<P, MODE>) -> Self {
        AnyPin::new(P as u8 - b'A', p.i)
    }
}
