//! # Controller Area Network (CAN) Interface
//!

use crate::gpio::{Const, NoPin, PushPull, SetAlternate};
use crate::pac::{CAN1, CAN2};
use crate::rcc;

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset {}

// Implemented by all SPI instances
impl Instance for CAN1 {}
impl Instance for CAN2 {}

pub trait Pins<Can> {}
pub trait PinTx<Can> {
    type A;
}
pub trait PinRx<Can> {
    type A;
}

impl<Can, TX, RX> Pins<Can> for (TX, RX)
where
    TX: PinTx<Can>,
    RX: PinRx<Can>,
{
}

impl<Can> PinTx<Can> for NoPin
where
    Can: Instance,
{
    type A = Const<0>;
}

impl<Can> PinRx<Can> for NoPin
where
    Can: Instance,
{
    type A = Const<0>;
}

macro_rules! pin {
    ($trait:ident<$I2C:ident> for $gpio:ident::$PX:ident<$A:literal>) => {
        impl<MODE> $trait<$I2C> for $gpio::$PX<MODE> {
            type A = Const<$A>;
        }
    };
}

mod common_pins {
    use super::*;
    use crate::gpio::{gpioa, gpiob, gpiod};
    pin!(PinTx<CAN1> for gpioa::PA12<9>);
    pin!(PinRx<CAN1> for gpioa::PA11<9>);
    pin!(PinTx<CAN1> for gpiod::PD1<9>);
    pin!(PinRx<CAN1> for gpiod::PD0<9>);
    pin!(PinTx<CAN2> for gpiob::PB13<9>);
    pin!(PinRx<CAN2> for gpiob::PB12<9>);
    pin!(PinTx<CAN2> for gpiob::PB6<9>);
    pin!(PinRx<CAN2> for gpiob::PB5<9>);
}

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
mod pb9_pb8_af8 {
    use super::*;
    use crate::gpio::gpiob;
    pin!(PinTx<CAN1> for gpiob::PB9<8>);
    pin!(PinRx<CAN1> for gpiob::PB8<8>);
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
mod pb9_pb8_af9 {
    use super::*;
    use crate::gpio::gpiob;
    pin!(PinTx<CAN1> for gpiob::PB9<9>);
    pin!(PinRx<CAN1> for gpiob::PB8<9>);
}

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
mod pg1_pg0 {
    use super::*;
    use crate::gpio::gpiog;
    pin!(PinTx<CAN1> for gpiog::PG1<9>);
    pin!(PinRx<CAN1> for gpiog::PG0<9>);
    pin!(PinTx<CAN2> for gpiog::PG12<9>);
    pin!(PinRx<CAN2> for gpiog::PG11<9>);
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
mod ph13_pi9 {
    use super::*;
    use crate::gpio::{gpioh, gpioi};
    pin!(PinTx<CAN1> for gpioh::PH13<9>);
    pin!(PinRx<CAN1> for gpioi::PI9<9>);
}

/// Pins and definitions for models with a third CAN peripheral
#[cfg(feature = "can3")]
mod can3 {
    use super::*;
    use crate::gpio::{gpioa, gpiob};
    use crate::pac::CAN3;

    impl Instance for CAN3 {}
    pin!(PinTx<CAN3> for gpioa::PA15<11>);
    pin!(PinRx<CAN3> for gpioa::PA8<11>);
    pin!(PinTx<CAN3> for gpiob::PB4<11>);
    pin!(PinRx<CAN3> for gpiob::PB3<11>);

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
    TX: PinTx<CAN, A = Const<TXA>> + SetAlternate<PushPull, TXA>,
    RX: PinRx<CAN, A = Const<RXA>> + SetAlternate<PushPull, RXA>,
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
    TX: PinTx<CAN, A = Const<TXA>> + SetAlternate<PushPull, TXA>,
{
    pub fn tx(usart: CAN, tx_pin: TX) -> Self {
        Self::new(usart, (tx_pin, NoPin))
    }
}

impl<CAN, RX, const RXA: u8> Can<CAN, (NoPin, RX)>
where
    CAN: Instance,
    RX: PinRx<CAN, A = Const<RXA>> + SetAlternate<PushPull, RXA>,
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
