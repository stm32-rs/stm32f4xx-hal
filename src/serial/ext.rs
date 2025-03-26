#[cfg(feature = "uart4")]
use crate::pac::uart4;
use crate::pac::usart1;
use stm32f4::{Readable, Reg, RegisterSpec, Resettable, Writable, R, W};

pub trait RBExt
where
    W<Self::SRrs>: SrW<Spec = Self::SRrs>,
    R<Self::SRrs>: SrR,
    W<Self::CR2rs>: Cr2W<Spec = Self::CR2rs>,
    R<Self::CR2rs>: Cr2R,
    W<Self::CR3rs>: Cr3W<Spec = Self::CR3rs>,
    R<Self::CR3rs>: Cr3R,
    W<Self::GTPRrs>: GtprW<Spec = Self::GTPRrs>,
    R<Self::GTPRrs>: GtprR,
{
    fn cr1(&self) -> &usart1::CR1;
    fn dr(&self) -> &usart1::DR;
    fn brr(&self) -> &usart1::BRR;
    type SRrs: RegisterSpec<Ux = u16> + Readable + Writable + Resettable;
    fn sr(&self) -> &Reg<Self::SRrs>;
    type CR2rs: RegisterSpec<Ux = u16> + Readable + Writable + Resettable;
    fn cr2(&self) -> &Reg<Self::CR2rs>;
    type CR3rs: RegisterSpec<Ux = u16> + Readable + Writable + Resettable;
    fn cr3(&self) -> &Reg<Self::CR3rs>;
    type GTPRrs: RegisterSpec<Ux = u16> + Readable + Writable + Resettable;
    fn gtpr(&self) -> &Reg<Self::GTPRrs>;
}

impl RBExt for usart1::RegisterBlock {
    fn cr1(&self) -> &usart1::CR1 {
        self.cr1()
    }
    fn dr(&self) -> &usart1::DR {
        self.dr()
    }
    fn brr(&self) -> &usart1::BRR {
        self.brr()
    }
    type SRrs = usart1::sr::SRrs;
    fn sr(&self) -> &Reg<Self::SRrs> {
        self.sr()
    }
    type CR2rs = usart1::cr2::CR2rs;
    fn cr2(&self) -> &Reg<Self::CR2rs> {
        self.cr2()
    }
    type CR3rs = usart1::cr3::CR3rs;
    fn cr3(&self) -> &Reg<Self::CR3rs> {
        self.cr3()
    }
    type GTPRrs = usart1::gtpr::GTPRrs;
    fn gtpr(&self) -> &Reg<Self::GTPRrs> {
        self.gtpr()
    }
}

#[cfg(feature = "uart4")]
impl RBExt for uart4::RegisterBlock {
    fn cr1(&self) -> &usart1::CR1 {
        self.cr1()
    }
    fn dr(&self) -> &usart1::DR {
        self.dr()
    }
    fn brr(&self) -> &usart1::BRR {
        self.brr()
    }
    type SRrs = uart4::sr::SRrs;
    fn sr(&self) -> &Reg<Self::SRrs> {
        self.sr()
    }
    type CR2rs = uart4::cr2::CR2rs;
    fn cr2(&self) -> &Reg<Self::CR2rs> {
        self.cr2()
    }
    type CR3rs = uart4::cr3::CR3rs;
    fn cr3(&self) -> &Reg<Self::CR3rs> {
        self.cr3()
    }
    type GTPRrs = uart4::gtpr::GTPRrs;
    fn gtpr(&self) -> &Reg<Self::GTPRrs> {
        self.gtpr()
    }
}

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

impl SrR for usart1::sr::R {
    fn pe(&self) -> usart1::sr::PE_R {
        self.pe()
    }
    fn fe(&self) -> usart1::sr::FE_R {
        self.fe()
    }
    fn nf(&self) -> usart1::sr::NF_R {
        self.nf()
    }
    fn ore(&self) -> usart1::sr::ORE_R {
        self.ore()
    }
    fn idle(&self) -> usart1::sr::IDLE_R {
        self.idle()
    }
    fn rxne(&self) -> usart1::sr::RXNE_R {
        self.rxne()
    }
    fn tc(&self) -> usart1::sr::TC_R {
        self.tc()
    }
    fn txe(&self) -> usart1::sr::TXE_R {
        self.txe()
    }
    fn lbd(&self) -> usart1::sr::LBD_R {
        self.lbd()
    }
}

#[cfg(feature = "uart4")]
impl SrR for uart4::sr::R {
    fn pe(&self) -> usart1::sr::PE_R {
        self.pe()
    }
    fn fe(&self) -> usart1::sr::FE_R {
        self.fe()
    }
    fn nf(&self) -> usart1::sr::NF_R {
        self.nf()
    }
    fn ore(&self) -> usart1::sr::ORE_R {
        self.ore()
    }
    fn idle(&self) -> usart1::sr::IDLE_R {
        self.idle()
    }
    fn rxne(&self) -> usart1::sr::RXNE_R {
        self.rxne()
    }
    fn tc(&self) -> usart1::sr::TC_R {
        self.tc()
    }
    fn txe(&self) -> usart1::sr::TXE_R {
        self.txe()
    }
    fn lbd(&self) -> usart1::sr::LBD_R {
        self.lbd()
    }
}

pub trait SrW {
    type Spec: Writable;
    fn rxne(&mut self) -> usart1::sr::RXNE_W<Self::Spec>;
    fn tc(&mut self) -> usart1::sr::TC_W<Self::Spec>;
    fn lbd(&mut self) -> usart1::sr::LBD_W<Self::Spec>;
}

impl SrW for usart1::sr::W {
    type Spec = usart1::sr::SRrs;
    fn rxne(&mut self) -> usart1::sr::RXNE_W<Self::Spec> {
        self.rxne()
    }
    fn tc(&mut self) -> usart1::sr::TC_W<Self::Spec> {
        self.tc()
    }
    fn lbd(&mut self) -> usart1::sr::LBD_W<Self::Spec> {
        self.lbd()
    }
}

#[cfg(feature = "uart4")]
impl SrW for uart4::sr::W {
    type Spec = uart4::sr::SRrs;
    fn rxne(&mut self) -> usart1::sr::RXNE_W<Self::Spec> {
        self.rxne()
    }
    fn tc(&mut self) -> usart1::sr::TC_W<Self::Spec> {
        self.tc()
    }
    fn lbd(&mut self) -> usart1::sr::LBD_W<Self::Spec> {
        self.lbd()
    }
}

pub trait Cr2R {
    fn add(&self) -> usart1::cr2::ADD_R;
    fn lbdl(&self) -> usart1::cr2::LBDL_R;
    fn lbdie(&self) -> usart1::cr2::LBDIE_R;
    fn linen(&self) -> usart1::cr2::LINEN_R;
}

impl Cr2R for usart1::cr2::R {
    fn add(&self) -> usart1::cr2::ADD_R {
        self.add()
    }
    fn lbdl(&self) -> usart1::cr2::LBDL_R {
        self.lbdl()
    }
    fn lbdie(&self) -> usart1::cr2::LBDIE_R {
        self.lbdie()
    }
    fn linen(&self) -> usart1::cr2::LINEN_R {
        self.linen()
    }
}

#[cfg(feature = "uart4")]
impl Cr2R for uart4::cr2::R {
    fn add(&self) -> usart1::cr2::ADD_R {
        self.add()
    }
    fn lbdl(&self) -> usart1::cr2::LBDL_R {
        self.lbdl()
    }
    fn lbdie(&self) -> usart1::cr2::LBDIE_R {
        self.lbdie()
    }
    fn linen(&self) -> usart1::cr2::LINEN_R {
        self.linen()
    }
}

pub trait Cr2W {
    type Spec: Writable;
    fn add(&mut self) -> usart1::cr2::ADD_W<Self::Spec>;
    fn lbdl(&mut self) -> usart1::cr2::LBDL_W<Self::Spec>;
    fn lbdie(&mut self) -> usart1::cr2::LBDIE_W<Self::Spec>;
    fn linen(&mut self) -> usart1::cr2::LINEN_W<Self::Spec>;
}

impl Cr2W for usart1::cr2::W {
    type Spec = usart1::cr2::CR2rs;
    fn add(&mut self) -> usart1::cr2::ADD_W<Self::Spec> {
        self.add()
    }
    fn lbdl(&mut self) -> usart1::cr2::LBDL_W<Self::Spec> {
        self.lbdl()
    }
    fn lbdie(&mut self) -> usart1::cr2::LBDIE_W<Self::Spec> {
        self.lbdie()
    }
    fn linen(&mut self) -> usart1::cr2::LINEN_W<Self::Spec> {
        self.linen()
    }
}

#[cfg(feature = "uart4")]
impl Cr2W for uart4::cr2::W {
    type Spec = uart4::cr2::CR2rs;
    fn add(&mut self) -> usart1::cr2::ADD_W<Self::Spec> {
        self.add()
    }
    fn lbdl(&mut self) -> usart1::cr2::LBDL_W<Self::Spec> {
        self.lbdl()
    }
    fn lbdie(&mut self) -> usart1::cr2::LBDIE_W<Self::Spec> {
        self.lbdie()
    }
    fn linen(&mut self) -> usart1::cr2::LINEN_W<Self::Spec> {
        self.linen()
    }
}

pub trait Cr3R {
    fn eie(&self) -> usart1::cr3::EIE_R;
    fn iren(&self) -> usart1::cr3::IREN_R;
    fn irlp(&self) -> usart1::cr3::IRLP_R;
    fn hdsel(&self) -> usart1::cr3::HDSEL_R;
    fn dmar(&self) -> usart1::cr3::DMAR_R;
    fn dmat(&self) -> usart1::cr3::DMAT_R;
    fn onebit(&self) -> usart1::cr3::ONEBIT_R;
}

impl Cr3R for usart1::cr3::R {
    fn eie(&self) -> usart1::cr3::EIE_R {
        self.eie()
    }
    fn iren(&self) -> usart1::cr3::IREN_R {
        self.iren()
    }
    fn irlp(&self) -> usart1::cr3::IRLP_R {
        self.irlp()
    }
    fn hdsel(&self) -> usart1::cr3::HDSEL_R {
        self.hdsel()
    }
    fn dmar(&self) -> usart1::cr3::DMAR_R {
        self.dmar()
    }
    fn dmat(&self) -> usart1::cr3::DMAT_R {
        self.dmat()
    }
    fn onebit(&self) -> usart1::cr3::ONEBIT_R {
        self.onebit()
    }
}

#[cfg(feature = "uart4")]
impl Cr3R for uart4::cr3::R {
    fn eie(&self) -> usart1::cr3::EIE_R {
        self.eie()
    }
    fn iren(&self) -> usart1::cr3::IREN_R {
        self.iren()
    }
    fn irlp(&self) -> usart1::cr3::IRLP_R {
        self.irlp()
    }
    fn hdsel(&self) -> usart1::cr3::HDSEL_R {
        self.hdsel()
    }
    fn dmar(&self) -> usart1::cr3::DMAR_R {
        self.dmar()
    }
    fn dmat(&self) -> usart1::cr3::DMAT_R {
        self.dmat()
    }
    fn onebit(&self) -> usart1::cr3::ONEBIT_R {
        self.onebit()
    }
}

pub trait Cr3W {
    type Spec: Writable;
    fn eie(&mut self) -> usart1::cr3::EIE_W<Self::Spec>;
    fn iren(&mut self) -> usart1::cr3::IREN_W<Self::Spec>;
    fn irlp(&mut self) -> usart1::cr3::IRLP_W<Self::Spec>;
    fn hdsel(&mut self) -> usart1::cr3::HDSEL_W<Self::Spec>;
    fn dmar(&mut self) -> usart1::cr3::DMAR_W<Self::Spec>;
    fn dmat(&mut self) -> usart1::cr3::DMAT_W<Self::Spec>;
    fn onebit(&mut self) -> usart1::cr3::ONEBIT_W<Self::Spec>;
}

impl Cr3W for usart1::cr3::W {
    type Spec = usart1::cr3::CR3rs;
    fn eie(&mut self) -> usart1::cr3::EIE_W<Self::Spec> {
        self.eie()
    }
    fn iren(&mut self) -> usart1::cr3::IREN_W<Self::Spec> {
        self.iren()
    }
    fn irlp(&mut self) -> usart1::cr3::IRLP_W<Self::Spec> {
        self.irlp()
    }
    fn hdsel(&mut self) -> usart1::cr3::HDSEL_W<Self::Spec> {
        self.hdsel()
    }
    fn dmar(&mut self) -> usart1::cr3::DMAR_W<Self::Spec> {
        self.dmar()
    }
    fn dmat(&mut self) -> usart1::cr3::DMAT_W<Self::Spec> {
        self.dmat()
    }
    fn onebit(&mut self) -> usart1::cr3::ONEBIT_W<Self::Spec> {
        self.onebit()
    }
}

#[cfg(feature = "uart4")]
impl Cr3W for uart4::cr3::W {
    type Spec = uart4::cr3::CR3rs;
    fn eie(&mut self) -> usart1::cr3::EIE_W<Self::Spec> {
        self.eie()
    }
    fn iren(&mut self) -> usart1::cr3::IREN_W<Self::Spec> {
        self.iren()
    }
    fn irlp(&mut self) -> usart1::cr3::IRLP_W<Self::Spec> {
        self.irlp()
    }
    fn hdsel(&mut self) -> usart1::cr3::HDSEL_W<Self::Spec> {
        self.hdsel()
    }
    fn dmar(&mut self) -> usart1::cr3::DMAR_W<Self::Spec> {
        self.dmar()
    }
    fn dmat(&mut self) -> usart1::cr3::DMAT_W<Self::Spec> {
        self.dmat()
    }
    fn onebit(&mut self) -> usart1::cr3::ONEBIT_W<Self::Spec> {
        self.onebit()
    }
}

pub trait GtprR {
    fn psc(&self) -> usart1::gtpr::PSC_R;
}

impl GtprR for usart1::gtpr::R {
    fn psc(&self) -> usart1::gtpr::PSC_R {
        self.psc()
    }
}

#[cfg(feature = "uart4")]
impl GtprR for uart4::gtpr::R {
    fn psc(&self) -> usart1::gtpr::PSC_R {
        self.psc()
    }
}

pub trait GtprW {
    type Spec: Writable;
    fn psc(&mut self) -> usart1::gtpr::PSC_W<Self::Spec>;
}

impl GtprW for usart1::gtpr::W {
    type Spec = usart1::gtpr::GTPRrs;
    fn psc(&mut self) -> usart1::gtpr::PSC_W<Self::Spec> {
        self.psc()
    }
}

#[cfg(feature = "uart4")]
impl GtprW for uart4::gtpr::W {
    type Spec = uart4::gtpr::GTPRrs;
    fn psc(&mut self) -> usart1::gtpr::PSC_W<Self::Spec> {
        self.psc()
    }
}
