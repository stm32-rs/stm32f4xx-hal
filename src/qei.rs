//! # Quadrature Encoder Interface
use crate::{
    hal::{self, Direction},
    pac::RCC,
    rcc,
};

pub trait Pins<TIM> {}
use crate::timer::{CPin, C1, C2};

impl<TIM, PC1, PC2> Pins<TIM> for (PC1, PC2)
where
    PC1: CPin<C1, TIM>,
    PC2: CPin<C2, TIM>,
{
}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

impl<TIM: Instance, PC1, PC2> Qei<TIM, (PC1, PC2)>
where
    PC1: CPin<C1, TIM>,
    PC2: CPin<C2, TIM>,
{
    /// Configures a TIM peripheral as a quadrature encoder interface input
    pub fn new(mut tim: TIM, pins: (PC1, PC2)) -> Self {
        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
        let rcc = unsafe { &(*RCC::ptr()) };
        // Enable and reset clock.
        TIM::enable(rcc);
        TIM::reset(rcc);

        tim.setup_qei();

        Qei { tim, pins }
    }
}

impl<TIM: Instance, PINS> Qei<TIM, PINS> {
    /// Releases the TIM peripheral and QEI pins
    pub fn release(self) -> (TIM, PINS) {
        (self.tim, self.pins)
    }
}

impl<TIM: Instance, PINS> hal::Qei for Qei<TIM, PINS> {
    type Count = TIM::Count;

    fn count(&self) -> Self::Count {
        self.tim.read_count() as Self::Count
    }

    fn direction(&self) -> Direction {
        if self.tim.read_direction() {
            hal::Direction::Upcounting
        } else {
            hal::Direction::Downcounting
        }
    }
}

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset {
    type Count;

    fn setup_qei(&mut self);
    fn read_count(&self) -> Self::Count;
    fn read_direction(&self) -> bool;
}

macro_rules! hal {
    ($($TIM:ty: ($bits:ident),)+) => {
        $(
            impl Instance for $TIM {
                type Count = $bits;

                fn setup_qei(&mut self) {
                    // Configure TxC1 and TxC2 as captures
                    self.ccmr1_output()
                        .write(|w| unsafe { w.cc1s().bits(0b01).cc2s().bits(0b01) });
                    // enable and configure to capture on rising edge
                    self.ccer.write(|w| {
                        w.cc1e()
                            .set_bit()
                            .cc1p()
                            .clear_bit()
                            .cc2e()
                            .set_bit()
                            .cc2p()
                            .clear_bit()
                    });
                    // configure as quadrature encoder
                    // some chip variants declare `.bits()` as unsafe, some don't
                    #[allow(unused_unsafe)]
                    self.smcr.write(|w| unsafe { w.sms().bits(3) });
                    #[allow(unused_unsafe)]
                    self.arr.write(|w| unsafe { w.bits($bits::MAX as u32) });
                    self.cr1.write(|w| w.cen().set_bit());
                }

                fn read_count(&self) -> Self::Count {
                    self.cnt.read().bits() as Self::Count
                }

                fn read_direction(&self) -> bool {
                    self.cr1.read().dir().bit_is_clear()
                }
            }
        )+
    }
}

hal! {
    crate::pac::TIM1: (u16),
    crate::pac::TIM5: (u32),
}

#[cfg(feature = "tim2")]
hal! {
    crate::pac::TIM2: (u32),
    crate::pac::TIM3: (u16),
    crate::pac::TIM4: (u16),
}

#[cfg(feature = "tim8")]
hal! {
    crate::pac::TIM8: (u16),
}
