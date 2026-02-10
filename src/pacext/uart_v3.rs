use crate::Sealed;

use super::*;
use crate::pac::usart1;

pub trait UartRB: Sealed {
    fn cr1(&self) -> &usart1::CR1;
    fn rdr(&self) -> &usart1::RDR;
    fn tdr(&self) -> &usart1::TDR;
    fn brr(&self) -> &usart1::BRR;
    fn icr(&self) -> &usart1::ICR;
    fn isr(&self) -> &usart1::ISR;
    fn cr2(&self) -> &usart1::CR2;
    fn cr3(&self) -> &usart1::CR3;
    fn gtpr(&self) -> &usart1::GTPR;
}

macro_rules! impl_ext {
    ($(#[$attr:meta])* $uart:ident) => {
        impl Sealed for $uart::RegisterBlock {}
        impl UartRB for $uart::RegisterBlock {
            impl_reg! {
                cr1 -> &usart1::CR1;
                rdr -> &usart1::RDR;
                tdr -> &usart1::TDR;
                brr -> &usart1::BRR;
                icr -> &usart1::ICR;
                isr -> &usart1::ISR;
                cr2 -> &usart1::CR2;
                cr3 -> &usart1::CR3;
                gtpr -> &usart1::GTPR;
            }
        }
    };
}

impl_ext!(usart1);
