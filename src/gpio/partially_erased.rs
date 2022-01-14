use super::*;

pub type PEPin<const P: char, MODE> = PartiallyErasedPin<P, MODE>;

/// Partially erased pin
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
pub struct PartiallyErasedPin<const P: char, MODE> {
    i: u8,
    _mode: PhantomData<MODE>,
}

impl<const P: char, MODE> PartiallyErasedPin<P, MODE> {
    pub(crate) fn new(i: u8) -> Self {
        Self {
            i,
            _mode: PhantomData,
        }
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
    #[inline(always)]
    pub fn set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
    }

    #[inline(always)]
    pub fn set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            (*Gpio::<P>::ptr())
                .bsrr
                .write(|w| w.bits(1 << (self.i + 16)))
        }
    }

    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).odr.read().bits() & (1 << self.i) == 0 }
    }

    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<const P: char> PartiallyErasedPin<P, Output<OpenDrain>> {
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    #[inline(always)]
    pub fn is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << self.i) == 0 }
    }
}

impl<const P: char, MODE> PartiallyErasedPin<P, Input<MODE>> {
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    #[inline(always)]
    pub fn is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << self.i) == 0 }
    }
}
