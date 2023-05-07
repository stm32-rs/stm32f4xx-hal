//! # Controller Area Network (CAN) Interface
//!

use crate::gpio::{self, NoPin};
use crate::pac;
use crate::rcc;

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + gpio::alt::CanCommon {}

macro_rules! can {
    ($CAN:ty: $Can:ident) => {
        pub type $Can = Can<$CAN>;

        impl Instance for $CAN {}
    };
}

// Implemented by all SPI instances
can! { pac::CAN1: Can1 }
#[cfg(feature = "can2")]
can! { pac::CAN2: Can2 }
#[cfg(feature = "can3")]
can! { pac::CAN3: Can3 }

/// Pins and definitions for models with a third CAN peripheral
#[cfg(feature = "can3")]
mod can3 {
    use super::*;

    unsafe impl bxcan::Instance for Can<pac::CAN3> {
        const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN3::ptr() as *mut _;
    }

    unsafe impl bxcan::FilterOwner for Can<pac::CAN3> {
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
            CAN::enable_unchecked();
            CAN::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into());

        Can { can, pins }
    }

    pub fn release(self) -> (CAN, (CAN::Tx, CAN::Rx)) {
        (self.can, self.pins)
    }
}

impl<CAN: Instance> Can<CAN> {
    pub fn tx(usart: CAN, tx_pin: impl Into<CAN::Tx>) -> Self
    where
        NoPin: Into<CAN::Rx>,
    {
        Self::new(usart, (tx_pin, NoPin::new()))
    }
}

impl<CAN: Instance> Can<CAN> {
    pub fn rx(usart: CAN, rx_pin: impl Into<CAN::Rx>) -> Self
    where
        NoPin: Into<CAN::Tx>,
    {
        Self::new(usart, (NoPin::new(), rx_pin))
    }
}

unsafe impl bxcan::Instance for Can<pac::CAN1> {
    const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN1::ptr() as *mut _;
}

#[cfg(feature = "can2")]
unsafe impl bxcan::Instance for Can<pac::CAN2> {
    const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN2::ptr() as *mut _;
}

unsafe impl bxcan::FilterOwner for Can<pac::CAN1> {
    #[cfg(any(feature = "f4", feature = "f7"))]
    const NUM_FILTER_BANKS: u8 = 28;
    #[cfg(feature = "l4")]
    const NUM_FILTER_BANKS: u8 = 14;
}

unsafe impl bxcan::MasterInstance for Can<pac::CAN1> {}
