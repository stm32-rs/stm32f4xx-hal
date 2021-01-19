//! # Controller Area Network (CAN) Interface
//!

use crate::bb;
#[cfg(any(feature = "stm32f405", feature = "stm32f407"))]
use crate::gpio::{
    gpioa::{PA11, PA12},
    gpiob::{PB12, PB13, PB5, PB6, PB8, PB9},
    gpiod::{PD0, PD1},
    gpioh::PH13,
    gpioi::PI9,
    Alternate, AF9,
};

#[cfg(feature = "stm32f446")]
use crate::gpio::{
    gpioa::{PA11, PA12},
    gpiob::{PB12, PB13, PB5, PB6, PB8, PB9},
    gpiod::{PD0, PD1},
    Alternate, AF9,
};
use crate::pac::{CAN1, CAN2};
use crate::stm32::RCC;

mod sealed {
    pub trait Sealed {}
}

pub trait Pins: sealed::Sealed {
    type Instance;
}

/*
    order: tx, rx similar to serial
*/
macro_rules! pins {
    ($($PER:ident => ($tx:ident, $rx:ident),)+) => {
        $(
            impl sealed::Sealed for ($tx<Alternate<AF9>>, $rx<Alternate<AF9>>) {}
            impl Pins for ($tx<Alternate<AF9>>, $rx<Alternate<AF9>>) {
                type Instance = $PER;
            }
        )+
    }
}

/*
    See DS8626 Rev 9 Table 9.
*/
#[cfg(any(feature = "stm32f405", feature = "stm32f407"))]
pins! {
    CAN1 => (PA12, PA11),
    CAN1 => (PB9, PB8),
    CAN1 => (PD1, PD0),
    CAN1 => (PH13, PI9),
    CAN2 => (PB13, PB12),
    CAN2 => (PB6, PB5),
}

/*
    See DS10693 Rev 9 Table 11.
*/
#[cfg(feature = "stm32f446")]
pins! {
    CAN1 => (PA12, PA11),
    CAN1 => (PB9, PB8),
    CAN1 => (PD1, PD0),
    CAN2 => (PB13, PB12),
    CAN2 => (PB6, PB5),
}

/// Enable/disable peripheral
pub trait Enable: sealed::Sealed {
    fn enable();
}

macro_rules! bus {
    ($($PER:ident => ($peren:literal),)+) => {
        $(
            impl sealed::Sealed for crate::pac::$PER {}
            impl Enable for crate::pac::$PER {
                #[inline(always)]
                fn enable() {
                    unsafe {
                        let rcc = &(*RCC::ptr());
                        bb::set(&rcc.apb1enr, $peren)
                    };
                }
            }
        )+
    }
}

bus! {
    CAN1 => (25),
    CAN2 => (26),
}

/// Interface to the CAN peripheral.
pub struct Can<Instance> {
    _peripheral: Instance,
}

impl<Instance> Can<Instance>
where
    Instance: Enable,
{
    /// Creates a CAN interaface.
    pub fn new<P>(can: Instance, _pins: P) -> Can<Instance>
    where
        P: Pins<Instance = Instance>,
    {
        Instance::enable();
        Can { _peripheral: can }
    }

    pub fn new_unchecked(can: Instance) -> Can<Instance> {
        Instance::enable();
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
