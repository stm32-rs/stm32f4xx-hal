use std::{collections::HashSet, env};

#[derive(Clone, Copy, Debug)]
enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match (self.next(), self.next()) {
            (Some(res), None) => Ok(res),
            (None, _) => Err(GetOneError::None),
            _ => Err(GetOneError::Multiple),
        }
    }
}
const FEATURES: phf::Map<&str, &[&str]> = phf::phf_map! {
    "stm32f401" => &["gpio_f401"],
    "stm32f405" => &["gpio_f417"],
    "stm32f407" => &["gpio_f417"],
    "stm32f415" => &["gpio_f417", "pac_cryp"],
    "stm32f417" => &["gpio_f417", "pac_cryp"],
    "stm32f410" => &["gpio_f410"],
    "stm32f411" => &["gpio_f411"],
    "stm32f412" => &["gpio_f412"],
    "stm32f413" => &["gpio_f413"],
    "stm32f423" => &["gpio_f413", "pac_aes"],
    "stm32f427" => &["gpio_f427", "pac_fsmc"],
    "stm32f429" => &["gpio_f427", "pac_fmc"],
    "stm32f437" => &["gpio_f427", "pac_fsmc", "pac_cryp"],
    "stm32f439" => &["gpio_f427", "pac_fmc", "pac_cryp"],
    "stm32f446" => &["gpio_f446"],
    "stm32f469" => &["gpio_f469"],
    "stm32f479" => &["gpio_f469", "pac_cryp"],

    "gpio_f401" => &[
        "pac_gpiod", "pac_gpioe",
        "pac_i2c3",
        "pac_otg_fs",
        "pac_sdio",
        "pac_spi3", "pac_spi4",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim9", "pac_tim10", "pac_tim11",
    ],
    "gpio_f410" => &[
        "pac_dac",
        "pac_fmpi2c1",
        "pac_lptim1",
        "pac_spi5",
        "pac_tim1", "pac_tim5", "pac_tim6", "pac_tim9", "pac_tim11",
    ],
    "gpio_f411" => &[
        "pac_gpiod", "pac_gpioe", // "pac_gpioi",
        "pac_i2c3",
        "pac_otg_fs",
        "pac_sdio",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim9", "pac_tim10", "pac_tim11",
        "pac_spi3", "pac_spi4", "pac_spi5",
    ],
    "gpio_f412" => &[
        "pac_gpiod", "pac_gpioe", "pac_gpiof", "pac_gpiog",
        "pac_can1", "pac_can2",
        "pac_dfsdm1",
        "pac_fmpi2c1",
        "pac_fsmc",
        "pac_i2c3",
        "pac_quadspi",
        "pac_otg_fs",
        "pac_rng",
        "pac_sdio",
        "pac_spi3", "pac_spi4", "pac_spi5",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim6", "pac_tim7", "pac_tim8", "pac_tim9", "pac_tim10", "pac_tim11", "pac_tim12", "pac_tim13", "pac_tim14",
        "pac_usart3",
    ],
    "gpio_f413" => &[
        "pac_gpiod", "pac_gpioe", "pac_gpiof", "pac_gpiog",
        "pac_can1", "pac_can2", "pac_can3",
        "pac_dac",
        "pac_dfsdm1",
        "pac_dfsdm2",
        "pac_fsmc",
        "pac_fmpi2c1",
        "pac_i2c3",
        "pac_lptim1",
        "pac_quadspi",
        "pac_otg_fs",
        "pac_rng",
        "pac_sai1",
        "pac_sdio",
        "pac_spi3", "pac_spi4", "pac_spi5",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim6", "pac_tim7", "pac_tim8", "pac_tim9", "pac_tim10", "pac_tim11", "pac_tim12", "pac_tim13", "pac_tim14",
        "pac_usart3", "pac_uart4", "pac_uart5", "pac_uart7", "pac_uart8", "pac_uart9", "pac_uart10",
    ],
    "gpio_f417" => &[
        "pac_gpiod", "pac_gpioe", "pac_gpiof", "pac_gpiog", "pac_gpioi",
        "pac_adc2", "pac_adc3",
        "pac_can1", "pac_can2",
        "pac_dac",
        "pac_dcmi",
        "pac_eth",
        "pac_fsmc",
        "pac_i2c3",
        "pac_otg_fs", "pac_otg_hs",
        "pac_rng",
        "pac_sdio",
        "pac_spi3",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim6", "pac_tim7", "pac_tim8", "pac_tim9", "pac_tim10", "pac_tim11", "pac_tim12", "pac_tim13", "pac_tim14",
        "pac_usart3", "pac_uart4", "pac_uart5",
    ],
    "gpio_f427" => &[
        "pac_gpiod", "pac_gpioe", "pac_gpiof", "pac_gpiog", "pac_gpioi", "pac_gpioj", "pac_gpiok",
        "pac_adc2", "pac_adc3",
        "pac_can1", "pac_can2",
        "pac_dac",
        "pac_dcmi",
        "pac_dma2d",
        "pac_eth",
        "pac_i2c3",
        "pac_ltdc",
        "pac_otg_fs", "pac_otg_hs",
        "pac_rng",
        "pac_sai1",
        "pac_sdio",
        "pac_spi3", "pac_spi4", "pac_spi5", "pac_spi6",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim6", "pac_tim7", "pac_tim8", "pac_tim9", "pac_tim10", "pac_tim11", "pac_tim12", "pac_tim13", "pac_tim14",
        "pac_usart3", "pac_uart4", "pac_uart5", "pac_uart7", "pac_uart8",
    ],
    "gpio_f446" => &[
        "pac_gpiod", "pac_gpioe", "pac_gpiof", "pac_gpiog",
        "pac_adc2", "pac_adc3",
        "pac_can1", "pac_can2",
        "pac_dac",
        "pac_dcmi",
        "pac_fmpi2c1",
        "pac_fmc",
        "pac_i2c3",
        "pac_quadspi",
        "pac_otg_fs", "pac_otg_hs",
        "pac_sai1",
        "pac_sai2",
        "pac_sdio",
        "pac_spi3", "pac_spi4",
        "pac_spdifrx",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim6", "pac_tim7", "pac_tim8", "pac_tim9", "pac_tim10", "pac_tim11", "pac_tim12", "pac_tim13", "pac_tim14",
        "pac_usart3", "pac_uart4", "pac_uart5",
    ],
    "gpio_f469" => &[
        "pac_gpiod", "pac_gpioe", "pac_gpiof", "pac_gpiog", "pac_gpioi", "pac_gpioj", "pac_gpiok",
        "pac_adc2", "pac_adc3",
        "pac_can1", "pac_can2",
        "pac_dac",
        "pac_dcmi",
        "pac_dma2d",
        "pac_dsihost",
        "pac_eth",
        "pac_fmc",
        "pac_i2c3",
        "pac_ltdc",
        "pac_quadspi",
        "pac_otg_fs", "pac_otg_hs",
        "pac_rng",
        "pac_sai1",
        "pac_sdio",
        "pac_spi3", "pac_spi4", "pac_spi5", "pac_spi6",
        "pac_tim1", "pac_tim2", "pac_tim3", "pac_tim4", "pac_tim5", "pac_tim6", "pac_tim7", "pac_tim8", "pac_tim9", "pac_tim10", "pac_tim11", "pac_tim12", "pac_tim13", "pac_tim14",
        "pac_usart3", "pac_uart4", "pac_uart5", "pac_uart7", "pac_uart8",
    ],

    "pac_sai1" => &["any_sai"],
    "pac_sai2" => &["any_sai"],
    "pac_dfsdm1" => &["any_dfsdm"],
    "pac_dfsdm2" => &["any_dfsdm"],
};

fn enable_feature<'a>(fname: &'a str, enabled: &mut HashSet<&'a str>) {
    if !enabled.contains(fname) {
        if let Some(v) = FEATURES.get(fname) {
            for &f in v.iter() {
                println!("cargo:rustc-cfg={}", f);
                enabled.insert(fname);
                enable_feature(f, enabled);
            }
        }
    }
}

fn main() {
    let mut enabled = HashSet::new();
    let chip_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_STM32F4"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No stm32xx Cargo feature enabled"),
        Err(GetOneError::Multiple) => panic!("Multiple stm32xx Cargo features enabled"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase();

    enable_feature(&chip_name, &mut enabled);
}
