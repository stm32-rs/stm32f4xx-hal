use crate::gpio::*;
use crate::pac::RCC;
use crate::rcc::*;

pub type LscoPin = gpioa::PA2<DefaultMode>;

pub struct Lsco {
    pin: gpioa::PA2<Alternate<0>>,
}

impl Lsco {
    pub fn enable(&self) {
        let rcc = unsafe { &(*RCC::ptr()) };
        rcc.bdcr.modify(|_, w| w.lscoen().set_bit());
    }

    pub fn disable(&self) {
        let rcc = unsafe { &(*RCC::ptr()) };
        rcc.bdcr.modify(|_, w| w.lscoen().clear_bit());
    }

    pub fn release(self) -> LscoPin {
        self.pin.into_floating_input()
    }
}

pub trait LSCOExt {
    fn lsco(self, src: LSCOSrc, rcc: &mut Rcc) -> Lsco;
}

impl LSCOExt for LscoPin {
    fn lsco(self, src: LSCOSrc, rcc: &mut Rcc) -> Lsco {
        let src_select_bit = match src {
            LSCOSrc::LSE => {
                rcc.enable_lse(false);
                true
            }
            LSCOSrc::LSI => {
                rcc.enable_lsi();
                false
            }
        };
        rcc.unlock_rtc();
        rcc.rb.bdcr.modify(|_, w| w.lscosel().bit(src_select_bit));
        Lsco {
            pin: self.into_alternate(),
        }
    }
}

pub struct Mco<PIN> {
    pin: PIN,
    src_bits: u8,
}

impl<PIN> Mco<PIN> {
    pub fn enable(&self) {
        let rcc = unsafe { &(*RCC::ptr()) };
        rcc.cfgr
            .modify(|_, w| unsafe { w.mcosel().bits(self.src_bits) });
    }

    pub fn disable(&self) {
        let rcc = unsafe { &(*RCC::ptr()) };
        rcc.cfgr.modify(|_, w| unsafe { w.mcosel().bits(0) });
    }

    pub fn release(self) -> PIN {
        self.pin //TODO reverse pin to input<floating>
    }
}

pub trait MCOExt<PIN> {
    fn mco(self, src: MCOSrc, psc: Prescaler, rcc: &mut Rcc) -> Mco<PIN>;
}

macro_rules! mco {
    ($($PIN:ident),+) => {
        $(
            impl MCOExt<$PIN<Alternate<0>>> for $PIN<DefaultMode> {
                fn mco(self, src: MCOSrc, psc: Prescaler, rcc: &mut Rcc) -> Mco<$PIN<Alternate<0>>> {
                    let psc_bits = match psc {
                        Prescaler::NotDivided => 0b000,
                        Prescaler::Div2 => 0b001,
                        Prescaler::Div4 => 0b010,
                        Prescaler::Div8 => 0b011,
                        Prescaler::Div16 => 0b100,
                        Prescaler::Div32 => 0b101,
                        Prescaler::Div64 => 0b110,
                        _ => 0b111,
                    };
                    rcc.rb.cfgr.modify(|r, w| unsafe {
                        w.bits((r.bits() & !(0b111 << 28)) | (psc_bits << 28))
                    });

                    let src_bits = match src {
                        MCOSrc::SysClk => 0b001,
                        MCOSrc::HSI => {
                            rcc.enable_hsi();
                            0b011
                        },
                        MCOSrc::HSE => {
                            rcc.enable_hse(false);
                            0b100
                        },
                        MCOSrc::PLL => 0b101,
                        MCOSrc::LSI => {
                            rcc.enable_lsi();
                            0b110
                        },
                        MCOSrc::LSE => {
                            rcc.enable_lse(false);
                            0b111
                        },
                    };
                    Mco { src_bits, pin: self.into_alternate() }
                }
            }
        )+
    };
}

use crate::gpio::gpioa::PA8;
use crate::gpio::gpiog::PG10;

mco!(PA8, PG10);
