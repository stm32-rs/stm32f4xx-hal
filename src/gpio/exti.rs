use super::{marker, Edge, PinExt};
use crate::{pac::EXTI, syscfg::SysCfg};

#[cfg(feature = "f4")]
impl<const P: char, const N: u8, MODE> super::Pin<P, N, MODE> {
    /// NVIC interrupt number of interrupt from this pin
    ///
    /// Used to unmask / enable the interrupt with [`cortex_m::peripheral::NVIC::unmask()`].
    /// This is also useful for all other [`cortex_m::peripheral::NVIC`] functions.
    pub const fn interrupt(&self) -> crate::pac::Interrupt {
        use crate::pac::Interrupt;
        match N {
            0 => Interrupt::EXTI0,
            1 => Interrupt::EXTI1,
            2 => Interrupt::EXTI2,
            3 => Interrupt::EXTI3,
            4 => Interrupt::EXTI4,
            5..=9 => Interrupt::EXTI9_5,
            10..=15 => Interrupt::EXTI15_10,
            _ => panic!("Unsupported pin number"),
        }
    }
}

/// External Interrupt Pin
pub trait ExtiPin {
    /// Make corresponding EXTI line sensitive to this pin
    fn make_interrupt_source(&mut self, syscfg: &mut SysCfg);

    /// Generate interrupt on rising edge, falling edge or both
    fn trigger_on_edge(&mut self, exti: &mut EXTI, level: Edge);

    /// Enable external interrupts from this pin.
    fn enable_interrupt(&mut self, exti: &mut EXTI);

    /// Disable external interrupts from this pin
    fn disable_interrupt(&mut self, exti: &mut EXTI);

    /// Clear the interrupt pending bit for this pin
    fn clear_interrupt_pending_bit(&mut self);

    /// Reads the interrupt pending bit for this pin
    fn check_interrupt(&self) -> bool;
}

impl<PIN> ExtiPin for PIN
where
    PIN: PinExt,
    PIN::Mode: marker::Interruptible,
{
    #[inline(always)]
    fn make_interrupt_source(&mut self, syscfg: &mut SysCfg) {
        let i = self.pin_id();
        let port = self.port_id() as u32;
        let offset = 4 * (i % 4);
        match i {
            0..=3 => {
                syscfg.exticr1.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            4..=7 => {
                syscfg.exticr2.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            8..=11 => {
                syscfg.exticr3.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            12..=15 => {
                syscfg.exticr4.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0xf << offset)) | (port << offset))
                });
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn trigger_on_edge(&mut self, exti: &mut EXTI, edge: Edge) {
        let i = self.pin_id();
        match edge {
            Edge::Rising => {
                #[cfg(any(feature = "f4", feature = "f7"))]
                let (rtsr, ftsr) = (&exti.rtsr, &exti.ftsr);
                #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
                let (rtsr, ftsr) = (&exti.rtsr1, &exti.ftsr1);
                rtsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                ftsr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
            }
            Edge::Falling => {
                #[cfg(any(feature = "f4", feature = "f7"))]
                let (rtsr, ftsr) = (&exti.rtsr, &exti.ftsr);
                #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
                let (rtsr, ftsr) = (&exti.rtsr1, &exti.ftsr1);
                ftsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                rtsr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
            }
            Edge::RisingFalling => {
                #[cfg(any(feature = "f4", feature = "f7"))]
                let (rtsr, ftsr) = (&exti.rtsr, &exti.ftsr);
                #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
                let (rtsr, ftsr) = (&exti.rtsr1, &exti.ftsr1);
                rtsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                ftsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
            }
        }
    }

    #[inline(always)]
    fn enable_interrupt(&mut self, exti: &mut EXTI) {
        #[cfg(any(feature = "f4", feature = "f7"))]
        let imr = &exti.imr;
        #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
        let imr = &exti.imr1;
        imr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.pin_id())) });
    }

    #[inline(always)]
    fn disable_interrupt(&mut self, exti: &mut EXTI) {
        #[cfg(any(feature = "f4", feature = "f7"))]
        let imr = &exti.imr;
        #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
        let imr = &exti.imr1;
        imr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin_id())) });
    }

    #[inline(always)]
    fn clear_interrupt_pending_bit(&mut self) {
        #[cfg(any(feature = "f4", feature = "f7"))]
        let pr = unsafe { &(*EXTI::ptr()).pr };
        #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
        let pr = unsafe { &(*EXTI::ptr()).pr1 };
        pr.write(|w| unsafe { w.bits(1 << self.pin_id()) });
    }

    #[inline(always)]
    fn check_interrupt(&self) -> bool {
        #[cfg(any(feature = "f4", feature = "f7"))]
        let pr = unsafe { &(*EXTI::ptr()).pr };
        #[cfg(any(feature = "f3", feature = "g4", feature = "l4"))]
        let pr = unsafe { &(*EXTI::ptr()).pr1 };
        (pr.read().bits() & (1 << self.pin_id())) != 0
    }
}
