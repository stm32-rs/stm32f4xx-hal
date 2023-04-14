//! # Controller Area Network (CAN) Interface
//!

use crate::gpio::{self, NoPin};
use crate::pac::{CAN1, CAN2};
use crate::rcc;

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset {
    type Tx;
    type Rx;
}

macro_rules! can {
    ($CAN:ty: $Can:ident, $can:ident) => {
        pub type $Can = Can<$CAN>;

        impl Instance for $CAN {
            type Tx = gpio::alt::$can::Tx;
            type Rx = gpio::alt::$can::Rx;
        }
    };
}

// Implemented by all SPI instances
can! { CAN1: Can1, can1 }
can! { CAN2: Can2, can2 }
#[cfg(feature = "can3")]
can! { crate::pac::CAN3: Can3, can3 }

/// Pins and definitions for models with a third CAN peripheral
#[cfg(feature = "can3")]
mod can3 {
    use super::*;
    use crate::pac::CAN3;

    unsafe impl bxcan::Instance for Can<CAN3> {
        const REGISTERS: *mut bxcan::RegisterBlock = CAN3::ptr() as *mut _;
    }

    unsafe impl bxcan::FilterOwner for Can<CAN3> {
        const NUM_FILTER_BANKS: u8 = 14;
    }
}

pub trait CanExt: Sized + Instance {
    fn can(self, pins: (impl Into<Self::Tx>, impl Into<Self::Rx>)) -> Can<Self>;

    fn tx(self, tx_pin: impl Into<Self::Tx>) -> Can<Self>
    where
        NoPin: Into<Self::Rx>;

    fn rx(self, rx_pin: impl Into<Self::Rx>) -> Can<Self>
    where
        NoPin: Into<Self::Tx>;
}

impl<CAN: Instance> CanExt for CAN {
    fn can(self, pins: (impl Into<Self::Tx>, impl Into<Self::Rx>)) -> Can<Self> {
        Can::new(self, pins)
    }

    fn tx(self, tx_pin: impl Into<Self::Tx>) -> Can<Self>
    where
        NoPin: Into<Self::Rx>,
    {
        Can::tx(self, tx_pin)
    }

    fn rx(self, rx_pin: impl Into<Self::Rx>) -> Can<Self>
    where
        NoPin: Into<Self::Tx>,
    {
        Can::rx(self, rx_pin)
    }
}

/// Interface to the CAN peripheral.
pub struct Can<CAN: Instance> {
    can: CAN,
    pins: (CAN::Tx, CAN::Rx),
}

impl<CAN: Instance> Can<CAN> {
    /// Creates a CAN interface.
    pub fn new(can: CAN, pins: (impl Into<CAN::Tx>, impl Into<CAN::Rx>)) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*crate::pac::RCC::ptr());
            CAN::enable(rcc);
            CAN::reset(rcc);
        }

        let pins = (pins.0.into(), pins.1.into());

        Can { can, pins }
    }

    pub fn release<TX, RX, E>(self) -> Result<(CAN, (TX, RX)), E>
    where
        TX: TryFrom<CAN::Tx, Error = E>,
        RX: TryFrom<CAN::Rx, Error = E>,
    {
        Ok((self.can, (self.pins.0.try_into()?, self.pins.1.try_into()?)))
    }
}

impl<CAN: Instance> Can<CAN> {
    pub fn tx(usart: CAN, tx_pin: impl Into<CAN::Tx>) -> Self
    where
        NoPin: Into<CAN::Rx>,
    {
        Self::new(usart, (tx_pin, NoPin))
    }
}

impl<CAN: Instance> Can<CAN> {
    pub fn rx(usart: CAN, rx_pin: impl Into<CAN::Rx>) -> Self
    where
        NoPin: Into<CAN::Tx>,
    {
        Self::new(usart, (NoPin, rx_pin))
    }
}

unsafe impl bxcan::Instance for Can<CAN1> {
    const REGISTERS: *mut bxcan::RegisterBlock = CAN1::ptr() as *mut _;
}

unsafe impl bxcan::Instance for Can<CAN2> {
    const REGISTERS: *mut bxcan::RegisterBlock = CAN2::ptr() as *mut _;
}

unsafe impl bxcan::FilterOwner for Can<CAN1> {
    const NUM_FILTER_BANKS: u8 = 28;
}

unsafe impl bxcan::MasterInstance for Can<CAN1> {}
