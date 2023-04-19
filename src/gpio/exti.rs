use super::{marker, Edge, Pin, PinExt};
use crate::{
    pac::{Interrupt, EXTI},
    syscfg::SysCfg,
};

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// NVIC interrupt number of interrupt from this pin
    ///
    /// Used to unmask / enable the interrupt with [`cortex_m::peripheral::NVIC::unmask()`].
    /// This is also useful for all other [`cortex_m::peripheral::NVIC`] functions.
    pub const fn interrupt(&self) -> Interrupt {
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
                exti.rtsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                exti.ftsr
                    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
            }
            Edge::Falling => {
                exti.ftsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                exti.rtsr
                    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
            }
            Edge::RisingFalling => {
                exti.rtsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
                exti.ftsr
                    .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
            }
        }
    }

    #[inline(always)]
    fn enable_interrupt(&mut self, exti: &mut EXTI) {
        exti.imr
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.pin_id())) });
    }

    #[inline(always)]
    fn disable_interrupt(&mut self, exti: &mut EXTI) {
        exti.imr
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin_id())) });
    }

    #[inline(always)]
    fn clear_interrupt_pending_bit(&mut self) {
        unsafe { (*EXTI::ptr()).pr.write(|w| w.bits(1 << self.pin_id())) };
    }

    #[inline(always)]
    fn check_interrupt(&self) -> bool {
        unsafe { ((*EXTI::ptr()).pr.read().bits() & (1 << self.pin_id())) != 0 }
    }
}
