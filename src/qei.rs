//! # Quadrature Encoder Interface
use crate::{
    pac::RCC,
    rcc::{Enable, Reset},
};

#[cfg(not(feature = "ehal1"))]
use crate::hal as qei;
#[cfg(feature = "ehal1")]
use crate::hal::blocking::qei;

pub trait Pins<TIM> {}
use crate::timer::PinC1;
use crate::timer::PinC2;

impl<TIM, PC1, PC2> Pins<TIM> for (PC1, PC2)
where
    PC1: PinC1<TIM>,
    PC2: PinC2<TIM>,
{
}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

impl<TIM: Instance, PINS> Qei<TIM, PINS> {
    /// Configures a TIM peripheral as a quadrature encoder interface input
    pub fn new(tim: TIM, pins: PINS) -> Self
    where
        PINS: Pins<TIM>,
    {
        TIM::setup_clocks();

        tim.setup_qei();

        Qei { tim, pins }
    }

    /// Releases the TIM peripheral and QEI pins
    pub fn release(self) -> (TIM, PINS) {
        (self.tim, self.pins)
    }
}

#[cfg(not(feature = "ehal1"))]
impl<TIM: Instance, PINS> qei::Qei for Qei<TIM, PINS> {
    type Count = TIM::Count;

    fn count(&self) -> Self::Count {
        self.tim.read_count() as Self::Count
    }

    fn direction(&self) -> qei::Direction {
        if self.tim.read_direction() {
            qei::Direction::Upcounting
        } else {
            qei::Direction::Downcounting
        }
    }
}
#[cfg(feature = "ehal1")]
impl<TIM: Instance, PINS> qei::Qei for Qei<TIM, PINS> {
    type Error = core::convert::Infallible;
    type Count = TIM::Count;

    fn count(&self) -> Result<Self::Count, Self::Error> {
        Ok(self.tim.read_count() as Self::Count)
    }

    fn direction(&self) -> Result<qei::Direction, Self::Error> {
        Ok(if self.tim.read_direction() {
            qei::Direction::Upcounting
        } else {
            qei::Direction::Downcounting
        })
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait Instance: sealed::Sealed {
    type Count;

    fn setup_clocks();
    fn setup_qei(&self);
    fn read_count(&self) -> Self::Count;
    fn read_direction(&self) -> bool;
}

macro_rules! hal {
    ($($TIM:ty: ($tim:ident, $bits:ident),)+) => {
        $(
            impl sealed::Sealed for $TIM {}
            impl Instance for $TIM {
                type Count = $bits;

                fn setup_clocks() {
                    unsafe {
                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        let rcc = &(*RCC::ptr());
                        // Enable and reset clock.
                        <$TIM>::enable(rcc);
                        <$TIM>::reset(rcc);
                    }
                }

                fn setup_qei(&self) {
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
                    self.arr.write(|w| unsafe { w.bits(core::u32::MAX) });
                    self.cr1.write(|w| w.cen().set_bit());
                }

                fn read_count(&self) -> Self::Count {
                    self.cnt.read().bits() as Self::Count
                }

                fn read_direction(&self) -> bool {
                    self.cr1.read().dir().bit_is_clear()
                }
            }

            impl<PINS> Qei<$TIM, PINS> {
                /// Configures a TIM peripheral as a quadrature encoder interface input
                #[deprecated(
                    since = "0.9.0",
                    note = "Please use new instead"
                )]
                pub fn $tim(tim: $TIM, pins: PINS) -> Self
                where
                    PINS: Pins<$TIM>,
                {
                    Self::new(tim, pins)
                }
            }
        )+
    }
}

hal! {
    crate::pac::TIM1: (tim1, u16),
    crate::pac::TIM5: (tim5, u32),
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
hal! {
    crate::pac::TIM2: (tim2, u32),
    crate::pac::TIM3: (tim3, u16),
    crate::pac::TIM4: (tim4, u16),
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
hal! {
    crate::pac::TIM8: (tim8, u16),
}
