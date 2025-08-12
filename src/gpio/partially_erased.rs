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

impl<const P: char, MODE> PartiallyErasedPin<P, MODE> {
    #[inline(always)]
    pub(crate) fn block(&self) -> *const crate::pac::gpioa::RegisterBlock {
        gpiox::<P>()
    }
    state_inner!();
}

impl<const P: char, MODE> PartiallyErasedPin<P, Output<MODE>> {
    state_output!();
}

impl<const P: char, MODE> PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    state_input!();
}

impl<const P: char, MODE> PartiallyErasedPin<P, MODE>
where
    MODE: marker::OutputSpeed,
{
    speed!();
}

impl<const P: char, MODE> PartiallyErasedPin<P, MODE>
where
    MODE: marker::Active,
{
    internal_resistor!();
}

impl<const P: char, MODE> From<PartiallyErasedPin<P, MODE>> for AnyPin<MODE> {
    /// Partially erased pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: PartiallyErasedPin<P, MODE>) -> Self {
        AnyPin::new(P as u8 - b'A', p.i)
    }
}
