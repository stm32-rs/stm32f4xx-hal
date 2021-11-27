//! # Controller Area Network (CAN) Interface
//!

use crate::gpio::{Const, NoPin, PinA, PushPull, SetAlternate};
use crate::pac::{CAN1, CAN2};
use crate::rcc;

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset {}

// Implemented by all SPI instances
impl Instance for CAN1 {}
impl Instance for CAN2 {}

pub struct Tx;
impl crate::Sealed for Tx {}
pub struct Rx;
impl crate::Sealed for Rx {}

pub trait Pins<Can> {}

impl<Can, TX, RX> Pins<Can> for (TX, RX)
where
    TX: PinA<Tx, Can>,
    RX: PinA<Rx, Can>,
{
}

/// Pins and definitions for models with a third CAN peripheral
#[cfg(feature = "can3")]
mod can3 {
    use super::*;
    use crate::pac::CAN3;

    impl Instance for CAN3 {}

    unsafe impl<PINS> bxcan::Instance for Can<CAN3, PINS> {
        const REGISTERS: *mut bxcan::RegisterBlock = CAN3::ptr() as *mut _;
    }

    unsafe impl<PINS> bxcan::FilterOwner for Can<CAN3, PINS> {
        const NUM_FILTER_BANKS: u8 = 14;
    }
}

/// Interface to the CAN peripheral.
pub struct Can<CAN, PINS> {
    can: CAN,
    pins: PINS,
}

impl<CAN, TX, RX, const TXA: u8, const RXA: u8> Can<CAN, (TX, RX)>
where
    CAN: Instance,
    TX: PinA<Tx, CAN, A = Const<TXA>> + SetAlternate<PushPull, TXA>,
    RX: PinA<Rx, CAN, A = Const<RXA>> + SetAlternate<PushPull, RXA>,
{
    /// Creates a CAN interface.
    pub fn new(can: CAN, mut pins: (TX, RX)) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*crate::pac::RCC::ptr());
            CAN::enable(rcc);
            CAN::reset(rcc);
        }

        pins.0.set_alt_mode();
        pins.1.set_alt_mode();

        Can { can, pins }
    }

    pub fn release(mut self) -> (CAN, (TX, RX)) {
        self.pins.0.restore_mode();
        self.pins.1.restore_mode();

        (self.can, self.pins)
    }
}

impl<CAN, TX, const TXA: u8> Can<CAN, (TX, NoPin)>
where
    CAN: Instance,
    TX: PinA<Tx, CAN, A = Const<TXA>> + SetAlternate<PushPull, TXA>,
{
    pub fn tx(usart: CAN, tx_pin: TX) -> Self {
        Self::new(usart, (tx_pin, NoPin))
    }
}

impl<CAN, RX, const RXA: u8> Can<CAN, (NoPin, RX)>
where
    CAN: Instance,
    RX: PinA<Rx, CAN, A = Const<RXA>> + SetAlternate<PushPull, RXA>,
{
    pub fn rx(usart: CAN, rx_pin: RX) -> Self {
        Self::new(usart, (NoPin, rx_pin))
    }
}

unsafe impl<PINS> bxcan::Instance for Can<CAN1, PINS> {
    const REGISTERS: *mut bxcan::RegisterBlock = CAN1::ptr() as *mut _;
}

unsafe impl<PINS> bxcan::Instance for Can<CAN2, PINS> {
    const REGISTERS: *mut bxcan::RegisterBlock = CAN2::ptr() as *mut _;
}

unsafe impl<PINS> bxcan::FilterOwner for Can<CAN1, PINS> {
    const NUM_FILTER_BANKS: u8 = 28;
}

unsafe impl<PINS> bxcan::MasterInstance for Can<CAN1, PINS> {}
