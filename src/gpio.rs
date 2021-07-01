//! General Purpose Input / Output

use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};

use crate::pac::EXTI;
use crate::syscfg::SysCfg;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

pub trait PinExt {
    type Mode;
    /// Return pin number
    fn pin_id(&self) -> u8;
    /// Return port number
    fn port_id(&self) -> u8;
}

pub struct AF<const A: u8>;
pub type AF0 = AF<0>;
pub type AF1 = AF<1>;
pub type AF2 = AF<2>;
pub type AF3 = AF<3>;
pub type AF4 = AF<4>;
pub type AF5 = AF<5>;
pub type AF6 = AF<6>;
pub type AF7 = AF<7>;
pub type AF8 = AF<8>;
pub type AF9 = AF<9>;
pub type AF10 = AF<10>;
pub type AF11 = AF<11>;
pub type AF12 = AF<12>;
pub type AF13 = AF<13>;
pub type AF14 = AF<14>;
pub type AF15 = AF<15>;

/// Some alternate mode (type state)
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Some alternate mode in open drain configuration (type state)
pub struct AlternateOD<MODE> {
    _mode: PhantomData<MODE>,
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// Analog mode (type state)
pub struct Analog;

/// GPIO Pin speed selection
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

#[derive(Debug, PartialEq)]
pub enum Edge {
    RISING,
    FALLING,
    RISING_FALLING,
}

mod sealed {
    /// Marker trait that show if `ExtiPin` can be implemented
    pub trait Interruptable {}
}

use sealed::Interruptable;
impl<MODE> Interruptable for Output<MODE> {}
impl<MODE> Interruptable for Input<MODE> {}

/// External Interrupt Pin
pub trait ExtiPin {
    fn make_interrupt_source(&mut self, syscfg: &mut SysCfg);
    fn trigger_on_edge(&mut self, exti: &mut EXTI, level: Edge);
    fn enable_interrupt(&mut self, exti: &mut EXTI);
    fn disable_interrupt(&mut self, exti: &mut EXTI);
    fn clear_interrupt_pending_bit(&mut self);
    fn check_interrupt(&self) -> bool;
}

impl<PIN> ExtiPin for PIN
where
    PIN: PinExt,
    PIN::Mode: Interruptable,
{
    /// Make corresponding EXTI line sensitive to this pin
    #[inline(always)]
    fn make_interrupt_source(&mut self, syscfg: &mut SysCfg) {
        let i = self.pin_id();
        let port = self.port_id() as u32;
        let offset = 4 * (i % 4);
        match i {
            0..=3 => {
                syscfg.exticr1.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            4..=7 => {
                syscfg.exticr2.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            8..=11 => {
                syscfg.exticr3.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            12..=15 => {
                syscfg.exticr4.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            _ => unreachable!(),
        }
    }

    /// Generate interrupt on rising edge, falling edge or both
    #[inline(always)]
    fn trigger_on_edge(&mut self, exti: &mut EXTI, edge: Edge) {
        let i = self.pin_id();
        match edge {
            Edge::RISING => {
                exti.rtsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                exti.ftsr
                    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
            }
            Edge::FALLING => {
                exti.ftsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                exti.rtsr
                    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
            }
            Edge::RISING_FALLING => {
                exti.rtsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                exti.ftsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
            }
        }
    }

    /// Enable external interrupts from this pin.
    #[inline(always)]
    fn enable_interrupt(&mut self, exti: &mut EXTI) {
        exti.imr
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.pin_id())) });
    }

    /// Disable external interrupts from this pin
    #[inline(always)]
    fn disable_interrupt(&mut self, exti: &mut EXTI) {
        exti.imr
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin_id())) });
    }

    /// Clear the interrupt pending bit for this pin
    #[inline(always)]
    fn clear_interrupt_pending_bit(&mut self) {
        unsafe { (*EXTI::ptr()).pr.write(|w| w.bits(1 << self.pin_id())) };
    }

    /// Reads the interrupt pending bit for this pin
    #[inline(always)]
    fn check_interrupt(&self) -> bool {
        unsafe { ((*EXTI::ptr()).pr.read().bits() & (1 << self.pin_id())) != 0 }
    }
}

/// Partially erased pin
pub struct PXx<MODE, const P: u8> {
    i: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE, const P: u8> PinExt for PXx<MODE, P> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        self.i
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P
    }
}

impl<MODE, const P: u8> OutputPin for PXx<Output<MODE>, P> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).bsrr.write(|w| w.bits(1 << self.i)) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            (*Gpio::<P>::ptr())
                .bsrr
                .write(|w| w.bits(1 << (self.i + 16)))
        };
        Ok(())
    }
}

impl<MODE, const P: u8> StatefulOutputPin for PXx<Output<MODE>, P> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_set_low().map(|v| !v)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(unsafe { (*Gpio::<P>::ptr()).odr.read().bits() & (1 << self.i) == 0 })
    }
}

impl<MODE, const P: u8> toggleable::Default for PXx<Output<MODE>, P> {}

impl<const P: u8> InputPin for PXx<Output<OpenDrain>, P> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << self.i) == 0 })
    }
}

impl<MODE, const P: u8> InputPin for PXx<Input<MODE>, P> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << self.i) == 0 })
    }
}

fn _set_alternate_mode<const P: u8, const N: u8, const A: u8>() {
    let offset = 2 * { N };
    let offset2 = 4 * { N };
    let mode = A as u32;
    unsafe {
        if offset2 < 32 {
            (*Gpio::<P>::ptr())
                .afrl
                .modify(|r, w| w.bits((r.bits() & !(0b1111 << offset2)) | (mode << offset2)));
        } else {
            let offset2 = offset2 - 32;
            (*Gpio::<P>::ptr())
                .afrh
                .modify(|r, w| w.bits((r.bits() & !(0b1111 << offset2)) | (mode << offset2)));
        }
        (*Gpio::<P>::ptr())
            .moder
            .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)));
    }
}

/// Pin
pub struct PX<MODE, const P: u8, const N: u8> {
    _mode: PhantomData<MODE>,
}
impl<MODE, const P: u8, const N: u8> PX<MODE, P, N> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<MODE, const P: u8, const N: u8> PinExt for PX<MODE, P, N> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P
    }
}

impl<MODE, const P: u8, const N: u8> PX<MODE, P, N> {
    /// Configures the pin to operate alternate mode
    pub fn into_alternate<const A: u8>(self) -> PX<Alternate<AF<A>>, P, N> {
        less_than_16::<A>();
        _set_alternate_mode::<P, N, A>();
        PX::new()
    }

    /// Configures the pin to operate in alternate open drain mode
    pub fn into_alternate_open_drain<const A: u8>(self) -> PX<AlternateOD<AF<A>>, P, N> {
        less_than_16::<A>();
        _set_alternate_mode::<P, N, A>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF0 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0(self) -> PX<Alternate<AF<0>>, P, N> {
        _set_alternate_mode::<P, N, 0>();
        PX::new()
    }

    /// Configures the pin to operate in AF1 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1(self) -> PX<Alternate<AF<1>>, P, N> {
        _set_alternate_mode::<P, N, 1>();
        PX::new()
    }

    /// Configures the pin to operate in AF2 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2(self) -> PX<Alternate<AF<2>>, P, N> {
        _set_alternate_mode::<P, N, 2>();
        PX::new()
    }

    /// Configures the pin to operate in AF3 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3(self) -> PX<Alternate<AF<3>>, P, N> {
        _set_alternate_mode::<P, N, 3>();
        PX::new()
    }

    /// Configures the pin to operate in AF4 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4(self) -> PX<Alternate<AF<4>>, P, N> {
        _set_alternate_mode::<P, N, 4>();
        PX::new()
    }

    /// Configures the pin to operate in AF5 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5(self) -> PX<Alternate<AF<5>>, P, N> {
        _set_alternate_mode::<P, N, 5>();
        PX::new()
    }

    /// Configures the pin to operate in AF6 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6(self) -> PX<Alternate<AF<6>>, P, N> {
        _set_alternate_mode::<P, N, 6>();
        PX::new()
    }

    /// Configures the pin to operate in AF7 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7(self) -> PX<Alternate<AF<7>>, P, N> {
        _set_alternate_mode::<P, N, 7>();
        PX::new()
    }

    /// Configures the pin to operate in AF8 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8(self) -> PX<Alternate<AF<8>>, P, N> {
        _set_alternate_mode::<P, N, 8>();
        PX::new()
    }

    /// Configures the pin to operate in AF9 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9(self) -> PX<Alternate<AF<9>>, P, N> {
        _set_alternate_mode::<P, N, 9>();
        PX::new()
    }

    /// Configures the pin to operate in AF10 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10(self) -> PX<Alternate<AF<10>>, P, N> {
        _set_alternate_mode::<P, N, 10>();
        PX::new()
    }

    /// Configures the pin to operate in AF11 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11(self) -> PX<Alternate<AF<11>>, P, N> {
        _set_alternate_mode::<P, N, 11>();
        PX::new()
    }

    /// Configures the pin to operate in AF12 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12(self) -> PX<Alternate<AF<12>>, P, N> {
        _set_alternate_mode::<P, N, 12>();
        PX::new()
    }

    /// Configures the pin to operate in AF13 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af13(self) -> PX<Alternate<AF<13>>, P, N> {
        _set_alternate_mode::<P, N, 13>();
        PX::new()
    }

    /// Configures the pin to operate in AF14 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14(self) -> PX<Alternate<AF<14>>, P, N> {
        _set_alternate_mode::<P, N, 14>();
        PX::new()
    }

    /// Configures the pin to operate in AF15 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15(self) -> PX<Alternate<AF<15>>, P, N> {
        _set_alternate_mode::<P, N, 15>();
        PX::new()
    }

    /// Configures the pin to operate in AF0 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0_open_drain(self) -> PX<AlternateOD<AF<0>>, P, N> {
        _set_alternate_mode::<P, N, 0>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF1 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1_open_drain(self) -> PX<AlternateOD<AF<1>>, P, N> {
        _set_alternate_mode::<P, N, 1>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF2 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2_open_drain(self) -> PX<AlternateOD<AF<2>>, P, N> {
        _set_alternate_mode::<P, N, 2>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF3 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3_open_drain(self) -> PX<AlternateOD<AF<3>>, P, N> {
        _set_alternate_mode::<P, N, 3>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF4 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4_open_drain(self) -> PX<AlternateOD<AF<4>>, P, N> {
        _set_alternate_mode::<P, N, 4>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF5 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5_open_drain(self) -> PX<AlternateOD<AF<5>>, P, N> {
        _set_alternate_mode::<P, N, 5>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF6 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6_open_drain(self) -> PX<AlternateOD<AF<6>>, P, N> {
        _set_alternate_mode::<P, N, 6>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF7 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7_open_drain(self) -> PX<AlternateOD<AF<7>>, P, N> {
        _set_alternate_mode::<P, N, 7>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF8 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8_open_drain(self) -> PX<AlternateOD<AF<8>>, P, N> {
        _set_alternate_mode::<P, N, 8>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF9 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9_open_drain(self) -> PX<AlternateOD<AF<9>>, P, N> {
        _set_alternate_mode::<P, N, 9>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF10 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10_open_drain(self) -> PX<AlternateOD<AF<10>>, P, N> {
        _set_alternate_mode::<P, N, 10>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF11 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11_open_drain(self) -> PX<AlternateOD<AF<11>>, P, N> {
        _set_alternate_mode::<P, N, 11>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF12 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12_open_drain(self) -> PX<AlternateOD<AF<12>>, P, N> {
        _set_alternate_mode::<P, N, 12>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF13 open drain modev
    pub fn into_alternate_af13_open_drain(self) -> PX<AlternateOD<AF<13>>, P, N> {
        _set_alternate_mode::<P, N, 13>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF14 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14_open_drain(self) -> PX<AlternateOD<AF<14>>, P, N> {
        _set_alternate_mode::<P, N, 14>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF15 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15_open_drain(self) -> PX<AlternateOD<AF<15>>, P, N> {
        _set_alternate_mode::<P, N, 15>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(self) -> PX<Input<Floating>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(self) -> PX<Input<PullDown>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(self) -> PX<Input<PullUp>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as an open drain output pin
    pub fn into_open_drain_output(self) -> PX<Output<OpenDrain>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .otyper
                .modify(|r, w| w.bits(r.bits() | (0b1 << { N })));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as an push pull output pin
    pub fn into_push_pull_output(self) -> PX<Output<PushPull>, P, N> {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .otyper
                .modify(|r, w| w.bits(r.bits() & !(0b1 << { N })));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as an analog input pin
    pub fn into_analog(self) -> PX<Analog, P, N> {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b11 << offset)))
        };

        PX::new()
    }
}

impl<MODE, const P: u8, const N: u8> PX<Output<MODE>, P, N> {
    /// Set pin speed
    pub fn set_speed(self, speed: Speed) -> Self {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .ospeedr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset)))
        };

        self
    }
}

impl<const P: u8, const N: u8> PX<Output<OpenDrain>, P, N> {
    /// Enables / disables the internal pull up
    pub fn internal_pull_up(&mut self, on: bool) {
        let offset = 2 * { N };
        let value = if on { 0b01 } else { 0b00 };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (value << offset)))
        };
    }
}

impl<MODE, const P: u8, const N: u8> PX<Alternate<MODE>, P, N> {
    /// Set pin speed
    pub fn set_speed(self, speed: Speed) -> Self {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .ospeedr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset)))
        };

        self
    }

    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        let offset = 2 * { N };
        let value = if on { 0b01 } else { 0b00 };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (value << offset)))
        };

        self
    }
}

impl<MODE, const P: u8, const N: u8> PX<Alternate<MODE>, P, N> {
    /// Turns pin alternate configuration pin into open drain
    pub fn set_open_drain(self) -> PX<AlternateOD<MODE>, P, N> {
        let offset = { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .otyper
                .modify(|r, w| w.bits(r.bits() | (1 << offset)))
        };

        PX::new()
    }
}

impl<MODE, const P: u8, const N: u8> PX<MODE, P, N> {
    /// Erases the pin number from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn downgrade(self) -> PXx<MODE, P> {
        PXx {
            i: { N },
            _mode: self._mode,
        }
    }

    /// Erases the pin number and the port from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn downgrade2(self) -> Pin<MODE> {
        Pin::new(P, N)
    }
}

impl<MODE, const P: u8, const N: u8> OutputPin for PX<Output<MODE>, P, N> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).bsrr.write(|w| w.bits(1 << { N })) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            (*Gpio::<P>::ptr())
                .bsrr
                .write(|w| w.bits(1 << ({ N } + 16)))
        };
        Ok(())
    }
}

impl<MODE, const P: u8, const N: u8> StatefulOutputPin for PX<Output<MODE>, P, N> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_set_low().map(|v| !v)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(unsafe { (*Gpio::<P>::ptr()).odr.read().bits() & (1 << { N }) == 0 })
    }
}

impl<MODE, const P: u8, const N: u8> toggleable::Default for PX<Output<MODE>, P, N> {}

impl<const P: u8, const N: u8> InputPin for PX<Output<OpenDrain>, P, N> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << { N }) == 0 })
    }
}

impl<MODE, const P: u8, const N: u8> InputPin for PX<Input<MODE>, P, N> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(unsafe { (*Gpio::<P>::ptr()).idr.read().bits() & (1 << { N }) == 0 })
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $PXx:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use crate::pac::{$GPIOX, RCC};
            use crate::rcc::Enable;
            use super::{
                Floating, Input,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl super::GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    unsafe {
                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        let rcc = &(*RCC::ptr());

                        // Enable clock.
                        $GPIOX::enable(rcc);
                    }
                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            pub type $PXn<MODE> = super::PXx<MODE, $port_id>;

            $(
                pub type $PXi<MODE> = super::PX<MODE, $port_id, $i>;
            )+

        }
    }
}

gpio!(GPIOA, gpioa, PA, 0, PAn, [
    PA0: (pa0, 0, Input<Floating>),
    PA1: (pa1, 1, Input<Floating>),
    PA2: (pa2, 2, Input<Floating>),
    PA3: (pa3, 3, Input<Floating>),
    PA4: (pa4, 4, Input<Floating>),
    PA5: (pa5, 5, Input<Floating>),
    PA6: (pa6, 6, Input<Floating>),
    PA7: (pa7, 7, Input<Floating>),
    PA8: (pa8, 8, Input<Floating>),
    PA9: (pa9, 9, Input<Floating>),
    PA10: (pa10, 10, Input<Floating>),
    PA11: (pa11, 11, Input<Floating>),
    PA12: (pa12, 12, Input<Floating>),
    PA13: (pa13, 13, Input<Floating>),
    PA14: (pa14, 14, Input<Floating>),
    PA15: (pa15, 15, Input<Floating>),
]);

gpio!(GPIOB, gpiob, PB, 1, PBn, [
    PB0: (pb0, 0, Input<Floating>),
    PB1: (pb1, 1, Input<Floating>),
    PB2: (pb2, 2, Input<Floating>),
    PB3: (pb3, 3, Input<Floating>),
    PB4: (pb4, 4, Input<Floating>),
    PB5: (pb5, 5, Input<Floating>),
    PB6: (pb6, 6, Input<Floating>),
    PB7: (pb7, 7, Input<Floating>),
    PB8: (pb8, 8, Input<Floating>),
    PB9: (pb9, 9, Input<Floating>),
    PB10: (pb10, 10, Input<Floating>),
    PB11: (pb11, 11, Input<Floating>),
    PB12: (pb12, 12, Input<Floating>),
    PB13: (pb13, 13, Input<Floating>),
    PB14: (pb14, 14, Input<Floating>),
    PB15: (pb15, 15, Input<Floating>),
]);

gpio!(GPIOC, gpioc, PC, 2, PCn, [
    PC0: (pc0, 0, Input<Floating>),
    PC1: (pc1, 1, Input<Floating>),
    PC2: (pc2, 2, Input<Floating>),
    PC3: (pc3, 3, Input<Floating>),
    PC4: (pc4, 4, Input<Floating>),
    PC5: (pc5, 5, Input<Floating>),
    PC6: (pc6, 6, Input<Floating>),
    PC7: (pc7, 7, Input<Floating>),
    PC8: (pc8, 8, Input<Floating>),
    PC9: (pc9, 9, Input<Floating>),
    PC10: (pc10, 10, Input<Floating>),
    PC11: (pc11, 11, Input<Floating>),
    PC12: (pc12, 12, Input<Floating>),
    PC13: (pc13, 13, Input<Floating>),
    PC14: (pc14, 14, Input<Floating>),
    PC15: (pc15, 15, Input<Floating>),
]);

#[cfg(feature = "gpiod")]
gpio!(GPIOD, gpiod, PD, 3, PDn, [
    PD0: (pd0, 0, Input<Floating>),
    PD1: (pd1, 1, Input<Floating>),
    PD2: (pd2, 2, Input<Floating>),
    PD3: (pd3, 3, Input<Floating>),
    PD4: (pd4, 4, Input<Floating>),
    PD5: (pd5, 5, Input<Floating>),
    PD6: (pd6, 6, Input<Floating>),
    PD7: (pd7, 7, Input<Floating>),
    PD8: (pd8, 8, Input<Floating>),
    PD9: (pd9, 9, Input<Floating>),
    PD10: (pd10, 10, Input<Floating>),
    PD11: (pd11, 11, Input<Floating>),
    PD12: (pd12, 12, Input<Floating>),
    PD13: (pd13, 13, Input<Floating>),
    PD14: (pd14, 14, Input<Floating>),
    PD15: (pd15, 15, Input<Floating>),
]);

#[cfg(feature = "gpioe")]
gpio!(GPIOE, gpioe, PE, 4, PEn, [
    PE0: (pe0, 0, Input<Floating>),
    PE1: (pe1, 1, Input<Floating>),
    PE2: (pe2, 2, Input<Floating>),
    PE3: (pe3, 3, Input<Floating>),
    PE4: (pe4, 4, Input<Floating>),
    PE5: (pe5, 5, Input<Floating>),
    PE6: (pe6, 6, Input<Floating>),
    PE7: (pe7, 7, Input<Floating>),
    PE8: (pe8, 8, Input<Floating>),
    PE9: (pe9, 9, Input<Floating>),
    PE10: (pe10, 10, Input<Floating>),
    PE11: (pe11, 11, Input<Floating>),
    PE12: (pe12, 12, Input<Floating>),
    PE13: (pe13, 13, Input<Floating>),
    PE14: (pe14, 14, Input<Floating>),
    PE15: (pe15, 15, Input<Floating>),
]);

#[cfg(feature = "gpiof")]
gpio!(GPIOF, gpiof, PF, 5, PFn, [
    PF0: (pf0, 0, Input<Floating>),
    PF1: (pf1, 1, Input<Floating>),
    PF2: (pf2, 2, Input<Floating>),
    PF3: (pf3, 3, Input<Floating>),
    PF4: (pf4, 4, Input<Floating>),
    PF5: (pf5, 5, Input<Floating>),
    PF6: (pf6, 6, Input<Floating>),
    PF7: (pf7, 7, Input<Floating>),
    PF8: (pf8, 8, Input<Floating>),
    PF9: (pf9, 9, Input<Floating>),
    PF10: (pf10, 10, Input<Floating>),
    PF11: (pf11, 11, Input<Floating>),
    PF12: (pf12, 12, Input<Floating>),
    PF13: (pf13, 13, Input<Floating>),
    PF14: (pf14, 14, Input<Floating>),
    PF15: (pf15, 15, Input<Floating>),
]);

#[cfg(feature = "gpiog")]
gpio!(GPIOG, gpiog, PG, 6, PGn, [
    PG0: (pg0, 0, Input<Floating>),
    PG1: (pg1, 1, Input<Floating>),
    PG2: (pg2, 2, Input<Floating>),
    PG3: (pg3, 3, Input<Floating>),
    PG4: (pg4, 4, Input<Floating>),
    PG5: (pg5, 5, Input<Floating>),
    PG6: (pg6, 6, Input<Floating>),
    PG7: (pg7, 7, Input<Floating>),
    PG8: (pg8, 8, Input<Floating>),
    PG9: (pg9, 9, Input<Floating>),
    PG10: (pg10, 10, Input<Floating>),
    PG11: (pg11, 11, Input<Floating>),
    PG12: (pg12, 12, Input<Floating>),
    PG13: (pg13, 13, Input<Floating>),
    PG14: (pg14, 14, Input<Floating>),
    PG15: (pg15, 15, Input<Floating>),
]);

#[cfg(not(feature = "stm32f401"))]
gpio!(GPIOH, gpioh, PH, 7, PHn, [
    PH0: (ph0, 0, Input<Floating>),
    PH1: (ph1, 1, Input<Floating>),
    PH2: (ph2, 2, Input<Floating>),
    PH3: (ph3, 3, Input<Floating>),
    PH4: (ph4, 4, Input<Floating>),
    PH5: (ph5, 5, Input<Floating>),
    PH6: (ph6, 6, Input<Floating>),
    PH7: (ph7, 7, Input<Floating>),
    PH8: (ph8, 8, Input<Floating>),
    PH9: (ph9, 9, Input<Floating>),
    PH10: (ph10, 10, Input<Floating>),
    PH11: (ph11, 11, Input<Floating>),
    PH12: (ph12, 12, Input<Floating>),
    PH13: (ph13, 13, Input<Floating>),
    PH14: (ph14, 14, Input<Floating>),
    PH15: (ph15, 15, Input<Floating>),
]);

#[cfg(feature = "stm32f401")]
gpio!(GPIOH, gpioh, PH, 7, PHn, [
    PH0: (ph0, 0, Input<Floating>),
    PH1: (ph1, 1, Input<Floating>),
]);

#[cfg(feature = "gpioi")]
gpio!(GPIOI, gpioi, PI, 8, PIn, [
    PI0: (pi0, 0, Input<Floating>),
    PI1: (pi1, 1, Input<Floating>),
    PI2: (pi2, 2, Input<Floating>),
    PI3: (pi3, 3, Input<Floating>),
    PI4: (pi4, 4, Input<Floating>),
    PI5: (pi5, 5, Input<Floating>),
    PI6: (pi6, 6, Input<Floating>),
    PI7: (pi7, 7, Input<Floating>),
    PI8: (pi8, 8, Input<Floating>),
    PI9: (pi9, 9, Input<Floating>),
    PI10: (pi10, 10, Input<Floating>),
    PI11: (pi11, 11, Input<Floating>),
    PI12: (pi12, 12, Input<Floating>),
    PI13: (pi13, 13, Input<Floating>),
    PI14: (pi14, 14, Input<Floating>),
    PI15: (pi15, 15, Input<Floating>),
]);

#[cfg(feature = "gpioj")]
gpio!(GPIOJ, gpioj, PJ, 9, PJn, [
    PJ0: (pj0, 0, Input<Floating>),
    PJ1: (pj1, 1, Input<Floating>),
    PJ2: (pj2, 2, Input<Floating>),
    PJ3: (pj3, 3, Input<Floating>),
    PJ4: (pj4, 4, Input<Floating>),
    PJ5: (pj5, 5, Input<Floating>),
    PJ6: (pj6, 6, Input<Floating>),
    PJ7: (pj7, 7, Input<Floating>),
    PJ8: (pj8, 8, Input<Floating>),
    PJ9: (pj9, 9, Input<Floating>),
    PJ10: (pj10, 10, Input<Floating>),
    PJ11: (pj11, 11, Input<Floating>),
    PJ12: (pj12, 12, Input<Floating>),
    PJ13: (pj13, 13, Input<Floating>),
    PJ14: (pj14, 14, Input<Floating>),
    PJ15: (pj15, 15, Input<Floating>),
]);

#[cfg(feature = "gpiok")]
gpio!(GPIOK, gpiok, PK, 10, PKn, [
    PK0: (pk0, 0, Input<Floating>),
    PK1: (pk1, 1, Input<Floating>),
    PK2: (pk2, 2, Input<Floating>),
    PK3: (pk3, 3, Input<Floating>),
    PK4: (pk4, 4, Input<Floating>),
    PK5: (pk5, 5, Input<Floating>),
    PK6: (pk6, 6, Input<Floating>),
    PK7: (pk7, 7, Input<Floating>),
]);

/// Fully erased pin
pub struct Pin<MODE> {
    // Bits 0-3: Pin, Bits 4-7: Port
    pin_port: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> PinExt for Pin<MODE> {
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

impl<MODE> Pin<MODE> {
    fn new(port: u8, pin: u8) -> Self {
        Self {
            pin_port: port << 4 | pin,
            _mode: PhantomData,
        }
    }

    #[inline]
    fn block(&self) -> &crate::pac::gpioa::RegisterBlock {
        // This function uses pointer arithmetic instead of branching to be more efficient

        // The logic relies on the following assumptions:
        // - GPIOA register is available on all chips
        // - all gpio register blocks have the same layout
        // - consecutive gpio register blocks have the same offset between them, namely 0x0400
        // - Pin::new was called with a valid port

        // FIXME could be calculated after const_raw_ptr_to_usize_cast stabilization #51910
        const GPIO_REGISTER_OFFSET: usize = 0x0400;

        let offset = GPIO_REGISTER_OFFSET * self.port_id() as usize;
        let block_ptr =
            (crate::pac::GPIOA::ptr() as usize + offset) as *const crate::pac::gpioa::RegisterBlock;

        unsafe { &*block_ptr }
    }
}

impl<MODE> OutputPin for Pin<Output<MODE>> {
    type Error = core::convert::Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            self.block()
                .bsrr
                .write(|w| w.bits(1 << (self.pin_id() + 16)))
        };
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { self.block().bsrr.write(|w| w.bits(1 << self.pin_id())) };
        Ok(())
    }
}

impl<MODE> StatefulOutputPin for Pin<Output<MODE>> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_set_low().map(|v| !v)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        Ok(self.block().odr.read().bits() & (1 << self.pin_id()) == 0)
    }
}

impl<MODE> toggleable::Default for Pin<Output<MODE>> {}

impl InputPin for Pin<Output<OpenDrain>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.block().idr.read().bits() & (1 << self.pin_id()) == 0)
    }
}

impl<MODE> InputPin for Pin<Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.block().idr.read().bits() & (1 << self.pin_id()) == 0)
    }
}

struct Gpio<const P: u8>;
impl<const P: u8> Gpio<P> {
    const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
        match P {
            0 => crate::pac::GPIOA::ptr(),
            1 => crate::pac::GPIOB::ptr() as _,
            2 => crate::pac::GPIOC::ptr() as _,
            #[cfg(feature = "gpiod")]
            3 => crate::pac::GPIOD::ptr() as _,
            #[cfg(feature = "gpioe")]
            4 => crate::pac::GPIOE::ptr() as _,
            #[cfg(feature = "gpiof")]
            5 => crate::pac::GPIOF::ptr() as _,
            #[cfg(feature = "gpiog")]
            6 => crate::pac::GPIOG::ptr() as _,
            7 => crate::pac::GPIOH::ptr() as _,
            #[cfg(feature = "gpioi")]
            8 => crate::pac::GPIOI::ptr() as _,
            #[cfg(feature = "gpioj")]
            9 => crate::pac::GPIOJ::ptr() as _,
            #[cfg(feature = "gpiok")]
            10 => crate::pac::GPIOK::ptr() as _,
            _ => 0 as _,
        }
    }
}

#[allow(path_statements)]
const fn less_than_16<const A: u8>() {
    Assert::<A, 16>::LESS;
}

/// Const assert hack
struct Assert<const L: u8, const R: u8>;

impl<const L: u8, const R: u8> Assert<L, R> {
    pub const LESS: u8 = R - L - 1;
}
