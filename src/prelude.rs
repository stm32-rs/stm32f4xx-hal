pub use embedded_hal::digital::v2::InputPin as _embedded_hal_digital_v2_InputPin;
pub use embedded_hal::digital::v2::OutputPin as _embedded_hal_digital_v2_OutputPin;
pub use embedded_hal::digital::v2::StatefulOutputPin as _embedded_hal_digital_v2_StatefulOutputPin;
pub use embedded_hal::digital::v2::ToggleableOutputPin as _embedded_hal_digital_v2_ToggleableOutputPin;
pub use embedded_hal::prelude::*;

#[cfg(all(
    feature = "device-selected",
    not(any(feature = "stm32f411", feature = "stm32f412", feature = "stm32f401",))
))]
pub use crate::dac::DacExt as _stm32f4xx_hal_dac_DacExt;
pub use crate::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use crate::i2c::Pins as _stm32f4xx_hal_i2c_Pins;
pub use crate::rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
#[cfg(all(
    feature = "device-selected",
    not(any(
        feature = "stm32f401",
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f446",
    ))
))]
pub use crate::rng::RngExt as _stm32f4xx_hal_rng_RngExt;
pub use crate::syscfg::SysCfgExt as _stm32f4xx_hal_syscfg_SysCfgExt;
pub use crate::time::U32Ext as _stm32f4xx_hal_time_U32Ext;
