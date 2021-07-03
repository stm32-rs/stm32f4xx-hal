//! # Controller Area Network (CAN) Interface
//!

use crate::pac::{CAN1, CAN2};
use crate::rcc::{Enable, Reset};

mod sealed {
    pub trait Sealed {}
}

/// A pair of (TX, RX) pins configured for CAN communication
pub trait Pins: sealed::Sealed {
    /// The CAN peripheral that uses these pins
    type Instance;
}

/// Implements sealed::Sealed and Pins for a (TX, RX) pair of pins associated with a CAN peripheral
/// The alternate function number can be specified after each pin name. If not specified, both
/// default to AF9.
macro_rules! pins {
    ($($PER:ident => ($tx:ident<$txaf:literal>, $rx:ident<$rxaf:literal>),)+) => {
        $(
            impl crate::can::sealed::Sealed for ($tx<crate::gpio::Alternate<$txaf>>, $rx<crate::gpio::Alternate<$rxaf>>) {}
            impl crate::can::Pins for ($tx<crate::gpio::Alternate<$txaf>>, $rx<crate::gpio::Alternate<$rxaf>>) {
                type Instance = $PER;
            }
        )+
    };
    ($($PER:ident => ($tx:ident, $rx:ident),)+) => {
        pins! { $($PER => ($tx<9>, $rx<9>),)+ }
    }
}

mod common_pins {
    use crate::gpio::{
        gpioa::{PA11, PA12},
        gpiob::{PB12, PB13, PB5, PB6},
        gpiod::{PD0, PD1},
    };
    use crate::pac::{CAN1, CAN2};
    // All STM32F4 models with CAN support these pins
    pins! {
        CAN1 => (PA12<9>, PA11<9>),
        CAN1 => (PD1<9>, PD0<9>),
        CAN2 => (PB13<9>, PB12<9>),
        CAN2 => (PB6<9>, PB5<9>),
    }
}

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
mod pb9_pb8_af8 {
    use crate::gpio::gpiob::{PB8, PB9};
    use crate::pac::CAN1;
    pins! { CAN1 => (PB9<8>, PB8<8>), }
}

#[cfg(any(feature = "can1", feature = "can2",))]
mod pb9_pb8_af9 {
    use crate::gpio::gpiob::{PB8, PB9};
    use crate::pac::CAN1;
    pins! { CAN1 => (PB9<9>, PB8<9>), }
}

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
mod pg1_pg0 {
    use crate::gpio::gpiog::{PG0, PG1};
    use crate::pac::CAN1;
    pins! { CAN1 => (PG1<9>, PG0<9>), }
}

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
mod pg12_pg11 {
    use crate::gpio::gpiog::{PG11, PG12};
    use crate::pac::CAN2;
    pins! { CAN2 => (PG12<9>, PG11<9>), }
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
    use crate::gpio::{gpioh::PH13, gpioi::PI9};
    use crate::pac::CAN1;
    pins! { CAN1 => (PH13<9>, PI9<9>), }
}

/// Pins and definitions for models with a third CAN peripheral
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
mod can3 {
    use super::Can;
    use crate::gpio::{
        gpioa::{PA15, PA8},
        gpiob::{PB3, PB4},
    };
    use crate::pac::CAN3;
    pins! {
        CAN3 => (PA15<11>, PA8<11>),
        CAN3 => (PB4<11>, PB3<11>),
    }

    unsafe impl bxcan::Instance for Can<CAN3> {
        const REGISTERS: *mut bxcan::RegisterBlock = CAN3::ptr() as *mut _;
    }

    unsafe impl bxcan::FilterOwner for Can<CAN3> {
        const NUM_FILTER_BANKS: u8 = 14;
    }
}

/// Interface to the CAN peripheral.
pub struct Can<Instance> {
    _peripheral: Instance,
}

impl<Instance> Can<Instance>
where
    Instance: Enable + Reset,
{
    /// Creates a CAN interface.
    pub fn new<P>(can: Instance, _pins: P) -> Can<Instance>
    where
        P: Pins<Instance = Instance>,
    {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*crate::pac::RCC::ptr());
            Instance::enable(rcc);
            Instance::reset(rcc);
        }
        Can { _peripheral: can }
    }

    pub fn new_unchecked(can: Instance) -> Can<Instance> {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*crate::pac::RCC::ptr());
            Instance::enable(rcc);
            Instance::reset(rcc);
        }
        Can { _peripheral: can }
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
