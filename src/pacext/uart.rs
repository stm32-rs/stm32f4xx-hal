#![allow(unused)]

use crate::{sealed, Sealed};

use super::*;
#[cfg(feature = "uart4")]
use crate::pac::uart4;
use crate::pac::usart1;

pub trait UartRB: Sealed {
    fn cr1(&self) -> &usart1::CR1;
    fn dr(&self) -> &usart1::DR;
    fn brr(&self) -> &usart1::BRR;
    type SRrs: reg::SrR + reg::SrW;
    fn sr(&self) -> &Reg<Self::SRrs>;
    type CR2rs: reg::Cr2R + reg::Cr2W;
    fn cr2(&self) -> &Reg<Self::CR2rs>;
    type CR3rs: reg::Cr3R + reg::Cr3W;
    fn cr3(&self) -> &Reg<Self::CR3rs>;
    type GTPRrs: reg::GtprR + reg::GtprW;
    fn gtpr(&self) -> &Reg<Self::GTPRrs>;
}

wrap_r! {
    pub trait SrR {
        fn pe(&self) -> usart1::sr::PE_R;
        fn fe(&self) -> usart1::sr::FE_R;
        fn nf(&self) -> usart1::sr::NF_R;
        fn ore(&self) -> usart1::sr::ORE_R;
        fn idle(&self) -> usart1::sr::IDLE_R;
        fn rxne(&self) -> usart1::sr::RXNE_R;
        fn tc(&self) -> usart1::sr::TC_R;
        fn txe(&self) -> usart1::sr::TXE_R;
        fn lbd(&self) -> usart1::sr::LBD_R;
    }
}
wrap_w! {
    pub trait SrW {
        fn rxne(&mut self) -> usart1::sr::RXNE_W<'_, REG>;
        fn tc(&mut self) -> usart1::sr::TC_W<'_, REG>;
        fn lbd(&mut self) -> usart1::sr::LBD_W<'_, REG>;
    }
}

wrap_r! {
    pub trait Cr2R {
        fn add(&self) -> usart1::cr2::ADD_R;
        fn lbdl(&self) -> usart1::cr2::LBDL_R;
        fn lbdie(&self) -> usart1::cr2::LBDIE_R;
        fn linen(&self) -> usart1::cr2::LINEN_R;
    }
}
wrap_w! {
    pub trait Cr2W {
        fn add(&mut self) -> usart1::cr2::ADD_W<'_, REG>;
        fn lbdl(&mut self) -> usart1::cr2::LBDL_W<'_, REG>;
        fn lbdie(&mut self) -> usart1::cr2::LBDIE_W<'_, REG>;
        fn linen(&mut self) -> usart1::cr2::LINEN_W<'_, REG>;
    }
}

wrap_r! {
    pub trait Cr3R {
        fn eie(&self) -> usart1::cr3::EIE_R;
        fn iren(&self) -> usart1::cr3::IREN_R;
        fn irlp(&self) -> usart1::cr3::IRLP_R;
        fn hdsel(&self) -> usart1::cr3::HDSEL_R;
        fn dmar(&self) -> usart1::cr3::DMAR_R;
        fn dmat(&self) -> usart1::cr3::DMAT_R;
        fn onebit(&self) -> usart1::cr3::ONEBIT_R;
    }
}
wrap_w! {
    pub trait Cr3W {
        fn eie(&mut self) -> usart1::cr3::EIE_W<'_, REG>;
        fn iren(&mut self) -> usart1::cr3::IREN_W<'_, REG>;
        fn irlp(&mut self) -> usart1::cr3::IRLP_W<'_, REG>;
        fn hdsel(&mut self) -> usart1::cr3::HDSEL_W<'_, REG>;
        fn dmar(&mut self) -> usart1::cr3::DMAR_W<'_, REG>;
        fn dmat(&mut self) -> usart1::cr3::DMAT_W<'_, REG>;
        fn onebit(&mut self) -> usart1::cr3::ONEBIT_W<'_, REG>;
    }
}

wrap_r! {
    pub trait GtprR {
        fn psc(&self) -> usart1::gtpr::PSC_R;
    }
}
wrap_w! {
    pub trait GtprW {
        fn psc(&mut self) -> usart1::gtpr::PSC_W<'_, REG>;
    }
}

mod reg {
    use super::*;

    pub trait SrR: RegisterSpec<Ux = u16> + Readable + Sized {
        fn pe(r: &R<Self>) -> usart1::sr::PE_R;
        fn fe(r: &R<Self>) -> usart1::sr::FE_R;
        fn nf(r: &R<Self>) -> usart1::sr::NF_R;
        fn ore(r: &R<Self>) -> usart1::sr::ORE_R;
        fn idle(r: &R<Self>) -> usart1::sr::IDLE_R;
        fn rxne(r: &R<Self>) -> usart1::sr::RXNE_R;
        fn tc(r: &R<Self>) -> usart1::sr::TC_R;
        fn txe(r: &R<Self>) -> usart1::sr::TXE_R;
        fn lbd(r: &R<Self>) -> usart1::sr::LBD_R;
    }
    pub trait SrW: RegisterSpec<Ux = u16> + Writable + Resettable + Sized {
        fn rxne(w: &mut W<Self>) -> usart1::sr::RXNE_W<'_, Self>;
        fn tc(w: &mut W<Self>) -> usart1::sr::TC_W<'_, Self>;
        fn lbd(w: &mut W<Self>) -> usart1::sr::LBD_W<'_, Self>;
    }

    pub trait Cr2R: RegisterSpec<Ux = u16> + Readable + Sized {
        fn add(r: &R<Self>) -> usart1::cr2::ADD_R;
        fn lbdl(r: &R<Self>) -> usart1::cr2::LBDL_R;
        fn lbdie(r: &R<Self>) -> usart1::cr2::LBDIE_R;
        fn linen(r: &R<Self>) -> usart1::cr2::LINEN_R;
    }
    pub trait Cr2W: RegisterSpec<Ux = u16> + Writable + Resettable + Sized {
        fn add(w: &mut W<Self>) -> usart1::cr2::ADD_W<'_, Self>;
        fn lbdl(w: &mut W<Self>) -> usart1::cr2::LBDL_W<'_, Self>;
        fn lbdie(w: &mut W<Self>) -> usart1::cr2::LBDIE_W<'_, Self>;
        fn linen(w: &mut W<Self>) -> usart1::cr2::LINEN_W<'_, Self>;
    }

    pub trait Cr3R: RegisterSpec<Ux = u16> + Readable + Sized {
        fn eie(r: &R<Self>) -> usart1::cr3::EIE_R;
        fn iren(r: &R<Self>) -> usart1::cr3::IREN_R;
        fn irlp(r: &R<Self>) -> usart1::cr3::IRLP_R;
        fn hdsel(r: &R<Self>) -> usart1::cr3::HDSEL_R;
        fn dmar(r: &R<Self>) -> usart1::cr3::DMAR_R;
        fn dmat(r: &R<Self>) -> usart1::cr3::DMAT_R;
        fn onebit(r: &R<Self>) -> usart1::cr3::ONEBIT_R;
    }
    pub trait Cr3W: RegisterSpec<Ux = u16> + Writable + Resettable + Sized {
        fn eie(w: &mut W<Self>) -> usart1::cr3::EIE_W<'_, Self>;
        fn iren(w: &mut W<Self>) -> usart1::cr3::IREN_W<'_, Self>;
        fn irlp(w: &mut W<Self>) -> usart1::cr3::IRLP_W<'_, Self>;
        fn hdsel(w: &mut W<Self>) -> usart1::cr3::HDSEL_W<'_, Self>;
        fn dmar(w: &mut W<Self>) -> usart1::cr3::DMAR_W<'_, Self>;
        fn dmat(w: &mut W<Self>) -> usart1::cr3::DMAT_W<'_, Self>;
        fn onebit(w: &mut W<Self>) -> usart1::cr3::ONEBIT_W<'_, Self>;
    }

    pub trait GtprR: RegisterSpec<Ux = u16> + Readable + Sized {
        fn psc(r: &R<Self>) -> usart1::gtpr::PSC_R;
    }
    pub trait GtprW: RegisterSpec<Ux = u16> + Writable + Resettable + Sized {
        fn psc(w: &mut W<Self>) -> usart1::gtpr::PSC_W<Self>;
    }
}

macro_rules! impl_ext {
    ($(#[$attr:meta])* $uart:ident) => {
        impl Sealed for $uart::RegisterBlock {}
        impl UartRB for $uart::RegisterBlock {
            type SRrs = $uart::sr::SRrs;
            type CR2rs = $uart::cr2::CR2rs;
            type CR3rs = $uart::cr3::CR3rs;
            type GTPRrs = $uart::gtpr::GTPRrs;
            impl_reg! {
                cr1 -> &usart1::CR1;
                dr -> &usart1::DR;
                brr -> &usart1::BRR;
                sr -> &Reg<Self::SRrs>;
                cr2 -> &Reg<Self::CR2rs>;
                cr3 -> &Reg<Self::CR3rs>;
                gtpr -> &Reg<Self::GTPRrs>;
            }
        }

        impl reg::SrR for $uart::sr::SRrs {
            impl_read! {
                pe -> usart1::sr::PE_R;
                fe -> usart1::sr::FE_R;
                nf -> usart1::sr::NF_R;
                ore -> usart1::sr::ORE_R;
                idle -> usart1::sr::IDLE_R;
                rxne -> usart1::sr::RXNE_R;
                tc -> usart1::sr::TC_R;
                txe -> usart1::sr::TXE_R;
                lbd -> usart1::sr::LBD_R;
            }
        }
        impl reg::SrW for $uart::sr::SRrs {
            impl_write! {
                rxne -> usart1::sr::RXNE_W<'_, Self>;
                tc -> usart1::sr::TC_W<'_, Self>;
                lbd -> usart1::sr::LBD_W<'_, Self>;
            }
        }

        impl reg::Cr2R for $uart::cr2::CR2rs {
            impl_read! {
                add -> usart1::cr2::ADD_R;
                lbdl -> usart1::cr2::LBDL_R;
                lbdie -> usart1::cr2::LBDIE_R;
                linen -> usart1::cr2::LINEN_R;
            }
        }
        impl reg::Cr2W for $uart::cr2::CR2rs {
            impl_write! {
                add -> usart1::cr2::ADD_W<'_, Self>;
                lbdl -> usart1::cr2::LBDL_W<'_, Self>;
                lbdie -> usart1::cr2::LBDIE_W<'_, Self>;
                linen -> usart1::cr2::LINEN_W<'_, Self>;
            }
        }

        $(#[$attr])*
        impl reg::Cr3R for $uart::cr3::CR3rs {
            impl_read! {
                eie -> usart1::cr3::EIE_R;
                iren -> usart1::cr3::IREN_R;
                irlp -> usart1::cr3::IRLP_R;
                hdsel -> usart1::cr3::HDSEL_R;
                dmar -> usart1::cr3::DMAR_R;
                dmat -> usart1::cr3::DMAT_R;
                onebit -> usart1::cr3::ONEBIT_R;
            }
        }
        $(#[$attr])*
        impl reg::Cr3W for $uart::cr3::CR3rs {
            impl_write! {
                eie -> usart1::cr3::EIE_W<'_, Self>;
                iren -> usart1::cr3::IREN_W<'_, Self>;
                irlp -> usart1::cr3::IRLP_W<'_, Self>;
                hdsel -> usart1::cr3::HDSEL_W<'_, Self>;
                dmar -> usart1::cr3::DMAR_W<'_, Self>;
                dmat -> usart1::cr3::DMAT_W<'_, Self>;
                onebit -> usart1::cr3::ONEBIT_W<'_, Self>;
            }
        }

        impl reg::GtprR for $uart::gtpr::GTPRrs {
            impl_read! {
                psc -> usart1::gtpr::PSC_R;
            }
        }
        impl reg::GtprW for $uart::gtpr::GTPRrs {
            impl_write! {
                psc -> usart1::gtpr::PSC_W<'_, Self>;
            }
        }
    };
}

impl_ext!(usart1);
#[cfg(feature = "uart4")]
impl_ext!(
    #[cfg(not(feature = "gpio-f446"))]
    uart4
);
