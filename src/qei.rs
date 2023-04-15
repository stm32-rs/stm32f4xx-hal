//! # Quadrature Encoder Interface
use crate::{
    pac::{self, RCC},
    rcc,
    timer::{ChannelPin as Ch, General},
};

pub trait QeiExt: Sized + Instance {
    fn qei(
        self,
        pins: (
            impl Into<<Self as Ch<0>>::Pin>,
            impl Into<<Self as Ch<1>>::Pin>,
        ),
    ) -> Qei<Self>;
}

impl<TIM: Instance> QeiExt for TIM {
    fn qei(
        self,
        pins: (
            impl Into<<Self as Ch<0>>::Pin>,
            impl Into<<Self as Ch<1>>::Pin>,
        ),
    ) -> Qei<Self> {
        Qei::new(self, pins)
    }
}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TIM: Instance> {
    tim: TIM,
    pins: (<TIM as Ch<0>>::Pin, <TIM as Ch<1>>::Pin),
}

impl<TIM: Instance> Qei<TIM> {
    /// Configures a TIM peripheral as a quadrature encoder interface input
    pub fn new(
        mut tim: TIM,
        pins: (
            impl Into<<TIM as Ch<0>>::Pin>,
            impl Into<<TIM as Ch<1>>::Pin>,
        ),
    ) -> Self {
        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
        let rcc = unsafe { &(*RCC::ptr()) };
        // Enable and reset clock.
        TIM::enable(rcc);
        TIM::reset(rcc);

        let pins = (pins.0.into(), pins.1.into());
        tim.setup_qei();

        Qei { tim, pins }
    }

    /// Releases the TIM peripheral and QEI pins
    pub fn release<PC1, PC2, E>(self) -> Result<(TIM, (PC1, PC2)), E>
    where
        PC1: TryFrom<<TIM as Ch<0>>::Pin, Error = E>,
        PC2: TryFrom<<TIM as Ch<1>>::Pin, Error = E>,
    {
        Ok((self.tim, (self.pins.0.try_into()?, self.pins.1.try_into()?)))
    }
}

impl<TIM: Instance> embedded_hal::Qei for Qei<TIM> {
    type Count = TIM::Width;

    fn count(&self) -> Self::Count {
        self.tim.read_count()
    }

    fn direction(&self) -> embedded_hal::Direction {
        if self.tim.read_direction() {
            embedded_hal::Direction::Upcounting
        } else {
            embedded_hal::Direction::Downcounting
        }
    }
}

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + General + Ch<0> + Ch<1> {
    fn setup_qei(&mut self);

    fn read_direction(&self) -> bool;
}

macro_rules! hal {
    ($($TIM:ty,)+) => {
        $(
            impl Instance for $TIM {
                fn setup_qei(&mut self) {
                    // Configure TxC1 and TxC2 as captures
                    #[cfg(not(feature = "gpio-f410"))]
                    self.ccmr1_input().write(|w| w.cc1s().ti1().cc2s().ti2());
                    #[cfg(feature = "gpio-f410")]
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
                    self.smcr.write(|w| w.sms().encoder_mode_3());
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
    pac::TIM1,
    pac::TIM5,
}

#[cfg(feature = "tim2")]
hal! {
    pac::TIM2,
    pac::TIM3,
    pac::TIM4,
}

#[cfg(feature = "tim8")]
hal! {
    pac::TIM8,
}
