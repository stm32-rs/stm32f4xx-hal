//! # API for the Digital to Analog converter
//!
//! Currently only supports writing to the DR of the DAC,
//! just a basic one-shot conversion.
#![deny(unused_imports)]

use crate::{
    gpio::{Analog, PA4, PA5},
    pac::{DAC, RCC},
    rcc::{Enable, Reset},
};

pub struct C1;
pub struct C2;

pub trait DacOut<V> {
    fn set_value(&mut self, val: V);
    fn get_value(&mut self) -> V;
}

pub trait DacPin {
    fn enable(&mut self);
}

pub trait Pins<DAC> {
    type Output;
    #[doc(hidden)]
    fn init() -> Self::Output;
}

impl Pins<DAC> for PA4<Analog> {
    type Output = C1;
    fn init() -> Self::Output {
        C1
    }
}

impl Pins<DAC> for PA5<Analog> {
    type Output = C2;
    fn init() -> Self::Output {
        C2
    }
}

impl Pins<DAC> for (PA4<Analog>, PA5<Analog>) {
    type Output = (C1, C2);
    fn init() -> Self::Output {
        (C1, C2)
    }
}

pub fn dac<PINS>(_dac: DAC, _pins: PINS) -> PINS::Output
where
    PINS: Pins<DAC>,
{
    unsafe {
        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
        let rcc = &(*RCC::ptr());

        // Enable and reset clock.
        DAC::enable(rcc);
        DAC::reset(rcc);

        PINS::init()
    }
}

macro_rules! dac {
    ($CX:ident, $en:ident, $cen:ident, $cal_flag:ident, $trim:ident, $mode:ident, $dhrx:ident, $dac_dor:ident, $daccxdhr:ident) => {
        impl DacPin for $CX {
            fn enable(&mut self) {
                let dac = unsafe { &(*DAC::ptr()) };
                dac.cr.modify(|_, w| w.$en().set_bit());
            }
        }

        impl DacOut<u16> for $CX {
            fn set_value(&mut self, val: u16) {
                let dac = unsafe { &(*DAC::ptr()) };
                dac.$dhrx.write(|w| unsafe { w.bits(val as u32) });
            }

            fn get_value(&mut self) -> u16 {
                let dac = unsafe { &(*DAC::ptr()) };
                dac.$dac_dor.read().bits() as u16
            }
        }
    };
}

pub trait DacExt {
    fn constrain<PINS>(self, pins: PINS) -> PINS::Output
    where
        PINS: Pins<DAC>;
}

impl DacExt for DAC {
    fn constrain<PINS>(self, pins: PINS) -> PINS::Output
    where
        PINS: Pins<DAC>,
    {
        dac(self, pins)
    }
}

dac!(C1, en1, cen1, cal_flag1, otrim1, mode1, dhr12r1, dor1, dacc1dhr);
dac!(C2, en2, cen2, cal_flag2, otrim2, mode2, dhr12r2, dor2, dacc2dhr);
