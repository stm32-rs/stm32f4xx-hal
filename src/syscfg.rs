use crate::stm32;

pub struct Syscfg {
    pub(crate) raw: stm32::SYSCFG,
}

impl Syscfg {
    pub fn new(syscfg: stm32::SYSCFG) -> Self {
        let rcc = unsafe {&*stm32::RCC::ptr()};

        // Reset SYSCFG peripheral
        rcc.apb2rstr.modify(|_, w| w.syscfgrst().set_bit());
        rcc.apb2rstr.modify(|_, w| w.syscfgrst().clear_bit());

        //// Enable SYSCFG peripheral
        rcc.apb2enr.write(|w| w.syscfgen().enabled());

        Syscfg { raw: syscfg }
    }
}