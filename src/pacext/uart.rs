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

pub trait Cf: UartRB<SRrs: reg::SrRCf + reg::SrWCf, CR3rs: reg::Cr3RCf + reg::Cr3WCf> {}

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
wrap_r! {
    pub trait SrRCf {
        fn cts(&self) -> usart1::sr::CTS_R;
    }
}
wrap_w! {
    pub trait SrW {
        fn rxne(&mut self) -> usart1::sr::RXNE_W<REG>;
        fn tc(&mut self) -> usart1::sr::TC_W<REG>;
        fn lbd(&mut self) -> usart1::sr::LBD_W<REG>;
    }
}
wrap_w! {
    pub trait SrWCf {
        fn cts(&mut self) -> usart1::sr::CTS_W<REG>;
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
        fn add(&mut self) -> usart1::cr2::ADD_W<REG>;
        fn lbdl(&mut self) -> usart1::cr2::LBDL_W<REG>;
        fn lbdie(&mut self) -> usart1::cr2::LBDIE_W<REG>;
        fn linen(&mut self) -> usart1::cr2::LINEN_W<REG>;
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
wrap_r! {
    pub trait Cr3RCf {
        fn rtse(&self) -> usart1::cr3::RTSE_R;
        fn ctse(&self) -> usart1::cr3::CTSE_R;
        fn ctsie(&self) -> usart1::cr3::CTSIE_R;
    }
}
wrap_w! {
    pub trait Cr3W {
        fn eie(&mut self) -> usart1::cr3::EIE_W<REG>;
        fn iren(&mut self) -> usart1::cr3::IREN_W<REG>;
        fn irlp(&mut self) -> usart1::cr3::IRLP_W<REG>;
        fn hdsel(&mut self) -> usart1::cr3::HDSEL_W<REG>;
        fn dmar(&mut self) -> usart1::cr3::DMAR_W<REG>;
        fn dmat(&mut self) -> usart1::cr3::DMAT_W<REG>;
        fn onebit(&mut self) -> usart1::cr3::ONEBIT_W<REG>;
    }
}
wrap_w! {
    pub trait Cr3WCf {
        fn rtse(&mut self) -> usart1::cr3::RTSE_W<REG>;
        fn ctse(&mut self) -> usart1::cr3::CTSE_W<REG>;
        fn ctsie(&mut self) -> usart1::cr3::CTSIE_W<REG>;
    }
}

wrap_r! {
    pub trait GtprR {
        fn psc(&self) -> usart1::gtpr::PSC_R;
    }
}
wrap_w! {
    pub trait GtprW {
        fn psc(&mut self) -> usart1::gtpr::PSC_W<REG>;
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
    pub trait SrRCf: SrR {
        fn cts(r: &R<Self>) -> usart1::sr::CTS_R;
    }
    pub trait SrW: RegisterSpec<Ux = u16> + Writable + Resettable + Sized {
        fn rxne(w: &mut W<Self>) -> usart1::sr::RXNE_W<Self>;
        fn tc(w: &mut W<Self>) -> usart1::sr::TC_W<Self>;
        fn lbd(w: &mut W<Self>) -> usart1::sr::LBD_W<Self>;
    }
    pub trait SrWCf: SrW {
        fn cts(w: &mut W<Self>) -> usart1::sr::CTS_W<Self>;
    }

    pub trait Cr2R: RegisterSpec<Ux = u16> + Readable + Sized {
        fn add(r: &R<Self>) -> usart1::cr2::ADD_R;
        fn lbdl(r: &R<Self>) -> usart1::cr2::LBDL_R;
        fn lbdie(r: &R<Self>) -> usart1::cr2::LBDIE_R;
        fn linen(r: &R<Self>) -> usart1::cr2::LINEN_R;
    }
    pub trait Cr2W: RegisterSpec<Ux = u16> + Writable + Resettable + Sized {
        fn add(w: &mut W<Self>) -> usart1::cr2::ADD_W<Self>;
        fn lbdl(w: &mut W<Self>) -> usart1::cr2::LBDL_W<Self>;
        fn lbdie(w: &mut W<Self>) -> usart1::cr2::LBDIE_W<Self>;
        fn linen(w: &mut W<Self>) -> usart1::cr2::LINEN_W<Self>;
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
        fn eie(w: &mut W<Self>) -> usart1::cr3::EIE_W<Self>;
        fn iren(w: &mut W<Self>) -> usart1::cr3::IREN_W<Self>;
        fn irlp(w: &mut W<Self>) -> usart1::cr3::IRLP_W<Self>;
        fn hdsel(w: &mut W<Self>) -> usart1::cr3::HDSEL_W<Self>;
        fn dmar(w: &mut W<Self>) -> usart1::cr3::DMAR_W<Self>;
        fn dmat(w: &mut W<Self>) -> usart1::cr3::DMAT_W<Self>;
        fn onebit(w: &mut W<Self>) -> usart1::cr3::ONEBIT_W<Self>;
    }

    pub trait Cr3RCf: Cr3R {
        fn rtse(r: &R<Self>) -> usart1::cr3::RTSE_R;
        fn ctse(r: &R<Self>) -> usart1::cr3::CTSE_R;
        fn ctsie(r: &R<Self>) -> usart1::cr3::CTSIE_R;
    }

    pub trait Cr3WCf: Cr3W {
        fn rtse(w: &mut W<Self>) -> usart1::cr3::RTSE_W<Self>;
        fn ctse(w: &mut W<Self>) -> usart1::cr3::CTSE_W<Self>;
        fn ctsie(w: &mut W<Self>) -> usart1::cr3::CTSIE_W<Self>;
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
                rxne -> usart1::sr::RXNE_W<Self>;
                tc -> usart1::sr::TC_W<Self>;
                lbd -> usart1::sr::LBD_W<Self>;
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
                add -> usart1::cr2::ADD_W<Self>;
                lbdl -> usart1::cr2::LBDL_W<Self>;
                lbdie -> usart1::cr2::LBDIE_W<Self>;
                linen -> usart1::cr2::LINEN_W<Self>;
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
                eie -> usart1::cr3::EIE_W<Self>;
                iren -> usart1::cr3::IREN_W<Self>;
                irlp -> usart1::cr3::IRLP_W<Self>;
                hdsel -> usart1::cr3::HDSEL_W<Self>;
                dmar -> usart1::cr3::DMAR_W<Self>;
                dmat -> usart1::cr3::DMAT_W<Self>;
                onebit -> usart1::cr3::ONEBIT_W<Self>;
            }
        }

        impl reg::GtprR for $uart::gtpr::GTPRrs {
            impl_read! {
                psc -> usart1::gtpr::PSC_R;
            }
        }
        impl reg::GtprW for $uart::gtpr::GTPRrs {
            impl_write! {
                psc -> usart1::gtpr::PSC_W<Self>;
            }
        }
    };
}
macro_rules! impl_cf {
    ($uart:ident) => {
        impl Cf for $uart::RegisterBlock {}
        impl reg::SrRCf for $uart::sr::SRrs {
            impl_read! {
                cts -> usart1::sr::CTS_R;
            }
        }
        impl reg::SrWCf for $uart::sr::SRrs {
            impl_write! {
                cts -> usart1::sr::CTS_W<Self>;
            }
        }
        impl reg::Cr3RCf for $uart::cr3::CR3rs {
            impl_read! {
                rtse -> usart1::cr3::RTSE_R;
                ctse -> usart1::cr3::CTSE_R;
                ctsie -> usart1::cr3::CTSIE_R;
            }
        }
        impl reg::Cr3WCf for $uart::cr3::CR3rs {
            impl_write! {
                rtse -> usart1::cr3::RTSE_W<Self>;
                ctse -> usart1::cr3::CTSE_W<Self>;
                ctsie -> usart1::cr3::CTSIE_W<Self>;
            }
        }
    };
}

impl_ext!(usart1);
impl_cf!(usart1);
#[cfg(feature = "uart4")]
impl_ext!(
    #[cfg(not(feature = "gpio-f446"))]
    uart4
);
