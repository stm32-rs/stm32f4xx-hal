//! # Quadrature Encoder Interface
use crate::{
    hal::{self, Direction},
    pac::RCC,
    rcc,
    timer::General,
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
    type Count = TIM::Width;

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

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + General {
    fn setup_qei(&mut self);

    fn read_direction(&self) -> bool;
}

macro_rules! hal {
    ($($TIM:ty,)+) => {
        $(
            impl Instance for $TIM {
                fn setup_qei(&mut self) {
                    // Configure TxC1 and TxC2 as captures
                    #[cfg(not(feature = "stm32f410"))]
                    self.ccmr1_input().write(|w| w.cc1s().ti1().cc2s().ti2());
                    #[cfg(feature = "stm32f410")]
                    self.ccmr1_input()
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
                    #[cfg(not(feature = "stm32f410"))]
                    self.smcr.write(|w| w.sms().encoder_mode_3());
                    #[cfg(feature = "stm32f410")]
                    self.smcr.write(|w| unsafe { w.sms().bits(3) });
                    self.set_auto_reload(<$TIM as General>::Width::MAX as u32).unwrap();
                    self.cr1.write(|w| w.cen().set_bit());
                }

                fn read_direction(&self) -> bool {
                    self.cr1.read().dir().bit_is_clear()
                }
            }
        )+
    }
}

hal! {
    crate::pac::TIM1,
    crate::pac::TIM5,
}

#[cfg(feature = "tim2")]
hal! {
    crate::pac::TIM2,
    crate::pac::TIM3,
    crate::pac::TIM4,
}

#[cfg(feature = "tim8")]
hal! {
    crate::pac::TIM8,
}
