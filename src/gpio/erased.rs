use super::*;

pub use AnyPin as ErasedPin;

/// Fully erased pin
///
/// `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
pub struct AnyPin<MODE> {
    // Bits 0-3: Pin, Bits 4-7: Port
    pin_port: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> fmt::Debug for AnyPin<MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P({}{})<{}>",
            self.port_id(),
            self.pin_id(),
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<MODE> defmt::Format for AnyPin<MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P({}{})<{}>",
            self.port_id(),
            self.pin_id(),
            crate::stripped_type_name::<MODE>()
        );
    }
}

impl<MODE> PinExt for AnyPin<MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        self.pin_port & 0x0f
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        self.pin_port >> 4
    }
}

impl<MODE> AnyPin<MODE> {
    pub(crate) fn from_pin_port(pin_port: u8) -> Self {
        Self {
            pin_port,
            _mode: PhantomData,
        }
    }
    pub(crate) fn into_pin_port(self) -> u8 {
        self.pin_port
    }
    pub(crate) fn new(port: u8, pin: u8) -> Self {
        Self {
            pin_port: port << 4 | pin,
            _mode: PhantomData,
        }
    }

    /// Convert type erased pin to `Pin` with fixed type
    pub fn restore<const P: char, const N: u8>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.port_id(), P as u8 - b'A');
        assert_eq!(self.pin_id(), N);
        Pin::new()
    }

    #[inline]
    pub(crate) fn block(&self) -> *const crate::pac::gpioa::RegisterBlock {
        // This function uses pointer arithmetic instead of branching to be more efficient

        // The logic relies on the following assumptions:
        // - GPIOA register is available on all chips
        // - all gpio register blocks have the same layout
        // - consecutive gpio register blocks have the same offset between them, namely 0x0400
        // - ErasedPin::new was called with a valid port

        // FIXME could be calculated after const_raw_ptr_to_usize_cast stabilization #51910
        const GPIO_REGISTER_OFFSET: usize = 0x0400;

        let offset = GPIO_REGISTER_OFFSET * self.port_id() as usize;
        (crate::pac::GPIOA::ptr() as usize + offset) as *const crate::pac::gpioa::RegisterBlock
    }

    state_inner!();
}

impl<MODE> AnyPin<Output<MODE>> {
    state_output!();
}

impl<MODE> AnyPin<MODE>
where
    MODE: marker::Readable,
{
    state_input!();
}

impl<MODE> AnyPin<MODE>
where
    MODE: marker::OutputSpeed,
{
    speed!();
}

impl<MODE> AnyPin<MODE>
where
    MODE: marker::Active,
{
    internal_resistor!();
}
