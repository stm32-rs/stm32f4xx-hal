// auto-generated using codegen
// STM32CubeMX DB release: DB.6.0.50
pub use super::*;

pub use super::Analog as DefaultMode;

#[cfg(feature = "gpio-l021")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [0, 1, 2, 4, 5, 6, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 4, 5, 6]),
    PA2: (pa2, 2, [0, 2, 4, 6, 7]),
    PA3: (pa3, 3, [0, 2, 4, 6]),
    PA4: (pa4, 4, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA5: (pa5, 5, [0, 1, 2, 5]),
    PA6: (pa6, 6, [0, 1, 4, 6, 7]),
    PA7: (pa7, 7, [0, 1, 4, 5, 6, 7]),
    PA8: (pa8, 8, [0, 2, 3, 4, 5]),
    PA9: (pa9, 9, [0, 1, 2, 4, 5, 7]),
    PA10: (pa10, 10, [0, 1, 2, 4, 5, 7]),
    PA11: (pa11, 11, [0, 1, 2, 4, 5, 7]),
    PA12: (pa12, 12, [0, 2, 4, 7]),
    PA13: (pa13, 13, [0, 1, 3, 5, 6, 7], super::Debugger),
    PA14: (pa14, 14, [0, 1, 3, 4, 5, 6, 7], super::Debugger),
    PA15: (pa15, 15, [0, 2, 3, 4, 5], super::Debugger),
]);

#[cfg(feature = "gpio-l021")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 4, 5]),
    PB1: (pb1, 1, [0, 1, 2, 4, 5]),
    PB2: (pb2, 2, [2]),
    PB3: (pb3, 3, [0, 2, 4], super::Debugger),
    PB4: (pb4, 4, [0, 2], super::Debugger),
    PB5: (pb5, 5, [0, 2, 3, 5]),
    PB6: (pb6, 6, [0, 1, 2, 5, 6]),
    PB7: (pb7, 7, [0, 1, 2, 5, 6]),
    PB8: (pb8, 8, [0, 2, 4, 5]),
    PB9: (pb9, 9, []),
]);

#[cfg(feature = "gpio-l021")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);
/*
#[cfg(feature = "gpio-l021")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l031")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 4, 5, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 4, 5]),
    PA2: (pa2, 2, [0, 2, 4, 6, 7]),
    PA3: (pa3, 3, [0, 2, 4, 6]),
    PA4: (pa4, 4, [0, 1, 4, 5]),
    PA5: (pa5, 5, [0, 1, 2, 5]),
    PA6: (pa6, 6, [0, 1, 4, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 4, 5, 6, 7]),
    PA8: (pa8, 8, [0, 2, 3, 4, 5]),
    PA9: (pa9, 9, [0, 1, 4, 5]),
    PA10: (pa10, 10, [1, 4, 5]),
    PA11: (pa11, 11, [0, 2, 4, 5, 7]),
    PA12: (pa12, 12, [0, 2, 4, 7]),
    PA13: (pa13, 13, [0, 1, 6], super::Debugger),
    PA14: (pa14, 14, [0, 1, 3, 4, 6], super::Debugger),
    PA15: (pa15, 15, [0, 2, 3, 4, 5], super::Debugger),
]);

#[cfg(feature = "gpio-l031")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 4, 5]),
    PB1: (pb1, 1, [0, 1, 4, 5]),
    PB2: (pb2, 2, [2]),
    PB3: (pb3, 3, [0, 2, 4], super::Debugger),
    PB4: (pb4, 4, [0, 2, 4], super::Debugger),
    PB5: (pb5, 5, [0, 2, 3, 4]),
    PB6: (pb6, 6, [0, 1, 2, 5]),
    PB7: (pb7, 7, [0, 1, 2]),
    PB8: (pb8, 8, [4]),
    PB9: (pb9, 9, [2, 4]),
    PB10: (pb10, 10, [2, 6]),
    PB11: (pb11, 11, [0, 2, 6]),
    PB12: (pb12, 12, [0, 6]),
    PB13: (pb13, 13, [0, 2, 5, 6]),
    PB14: (pb14, 14, [0, 2, 5, 6]),
    PB15: (pb15, 15, [0, 2]),
]);

#[cfg(feature = "gpio-l031")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [0, 2, 6]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-l031")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
]);
/*
#[cfg(feature = "gpio-l031")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l051")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [2, 3, 4, 5, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 4, 5]),
    PA2: (pa2, 2, [0, 1, 2, 3, 4, 7]),
    PA3: (pa3, 3, [0, 1, 2, 3, 4]),
    PA4: (pa4, 4, [0, 3, 4, 5]),
    PA5: (pa5, 5, [0, 2, 3, 5]),
    PA6: (pa6, 6, [0, 1, 3, 4, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 3, 5, 6, 7]),
    PA8: (pa8, 8, [0, 1, 2, 3, 4]),
    PA9: (pa9, 9, [0, 1, 3, 4]),
    PA10: (pa10, 10, [1, 3, 4]),
    PA11: (pa11, 11, [0, 2, 3, 4, 7]),
    PA12: (pa12, 12, [0, 2, 3, 4, 7]),
    PA13: (pa13, 13, [0, 2], super::Debugger),
    PA14: (pa14, 14, [0, 4], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 4, 5], super::Debugger),
]);

#[cfg(feature = "gpio-l051")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 3]),
    PB1: (pb1, 1, [1, 3, 4]),
    PB2: (pb2, 2, [2, 3]),
    PB3: (pb3, 3, [0, 1, 2, 3, 4], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 4], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3, 4]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3]),
    PB8: (pb8, 8, [1, 3, 4]),
    PB9: (pb9, 9, [1, 2, 4, 5]),
    PB10: (pb10, 10, [1, 2, 3, 4, 5, 6]),
    PB11: (pb11, 11, [0, 1, 2, 3, 4, 6]),
    PB12: (pb12, 12, [0, 1, 2, 3, 5, 6]),
    PB13: (pb13, 13, [0, 1, 3, 4, 5, 6]),
    PB14: (pb14, 14, [0, 1, 2, 3, 4, 5, 6]),
    PB15: (pb15, 15, [0, 1, 2]),
]);

#[cfg(feature = "gpio-l051")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [0, 1, 2, 3]),
    PC1: (pc1, 1, [0, 1, 2, 3]),
    PC2: (pc2, 2, [0, 1, 2, 3]),
    PC3: (pc3, 3, [0, 1, 2, 3]),
    PC4: (pc4, 4, [0, 1, 2]),
    PC5: (pc5, 5, [1, 2, 3]),
    PC6: (pc6, 6, [0, 1, 3]),
    PC7: (pc7, 7, [0, 1, 3]),
    PC8: (pc8, 8, [0, 1, 3]),
    PC9: (pc9, 9, [0, 1, 2, 3]),
    PC10: (pc10, 10, [0, 1]),
    PC11: (pc11, 11, [0, 1]),
    PC12: (pc12, 12, [1]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-l051")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD2: (pd2, 2, [0, 1]),
]);

#[cfg(feature = "gpio-l051")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [0]),
    PH1: (ph1, 1, []),
]);
/*
#[cfg(feature = "gpio-l051")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l071")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [2, 3, 4, 5, 6, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 4, 5, 6]),
    PA2: (pa2, 2, [0, 1, 2, 3, 4, 6, 7]),
    PA3: (pa3, 3, [0, 1, 2, 3, 4, 6]),
    PA4: (pa4, 4, [0, 3, 4, 5]),
    PA5: (pa5, 5, [0, 2, 3, 5]),
    PA6: (pa6, 6, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 2, 3, 5, 6, 7]),
    PA8: (pa8, 8, [0, 1, 2, 3, 4, 7]),
    PA9: (pa9, 9, [0, 1, 3, 4, 6, 7]),
    PA10: (pa10, 10, [1, 3, 4, 6]),
    PA11: (pa11, 11, [0, 2, 3, 4, 7]),
    PA12: (pa12, 12, [0, 2, 3, 4, 7]),
    PA13: (pa13, 13, [0, 2, 6], super::Debugger),
    PA14: (pa14, 14, [0, 4, 6], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 4, 5, 6], super::Debugger),
]);

#[cfg(feature = "gpio-l071")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3]),
    PB1: (pb1, 1, [1, 2, 3, 4]),
    PB2: (pb2, 2, [2, 3, 7]),
    PB3: (pb3, 3, [0, 1, 2, 3, 4, 5, 6], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 4, 5, 6, 7], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3, 4, 5, 6]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3, 6]),
    PB8: (pb8, 8, [1, 3, 4]),
    PB9: (pb9, 9, [1, 2, 4, 5]),
    PB10: (pb10, 10, [1, 2, 3, 4, 5, 6, 7]),
    PB11: (pb11, 11, [0, 1, 2, 3, 4, 6, 7]),
    PB12: (pb12, 12, [0, 1, 2, 3, 5, 6]),
    PB13: (pb13, 13, [0, 1, 2, 3, 4, 5, 6]),
    PB14: (pb14, 14, [0, 1, 2, 3, 4, 5, 6]),
    PB15: (pb15, 15, [0, 1, 2]),
]);

#[cfg(feature = "gpio-l071")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [0, 1, 2, 3, 6, 7]),
    PC1: (pc1, 1, [0, 1, 2, 3, 6, 7]),
    PC2: (pc2, 2, [0, 1, 2, 3]),
    PC3: (pc3, 3, [0, 1, 2, 3]),
    PC4: (pc4, 4, [0, 1, 2]),
    PC5: (pc5, 5, [1, 2, 3]),
    PC6: (pc6, 6, [0, 1, 2, 3]),
    PC7: (pc7, 7, [0, 1, 2, 3]),
    PC8: (pc8, 8, [0, 1, 2, 3]),
    PC9: (pc9, 9, [0, 1, 2, 3, 7]),
    PC10: (pc10, 10, [0, 1, 6]),
    PC11: (pc11, 11, [0, 1, 6]),
    PC12: (pc12, 12, [1, 2, 6]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-l071")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [0, 1]),
    PD1: (pd1, 1, [1]),
    PD2: (pd2, 2, [0, 1, 2, 6]),
    PD3: (pd3, 3, [0, 1, 2]),
    PD4: (pd4, 4, [0, 1]),
    PD5: (pd5, 5, [0]),
    PD6: (pd6, 6, [0]),
    PD7: (pd7, 7, [0, 1]),
    PD8: (pd8, 8, [0, 1]),
    PD9: (pd9, 9, [0, 1]),
    PD10: (pd10, 10, [1]),
    PD11: (pd11, 11, [0, 1]),
    PD12: (pd12, 12, [0, 1]),
    PD13: (pd13, 13, [1]),
    PD14: (pd14, 14, [1]),
    PD15: (pd15, 15, [0, 1]),
]);

#[cfg(feature = "gpio-l071")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [1, 2]),
    PE1: (pe1, 1, [1, 2]),
    PE2: (pe2, 2, [1, 2]),
    PE3: (pe3, 3, [0, 1, 2]),
    PE4: (pe4, 4, [0, 2]),
    PE5: (pe5, 5, [0, 2]),
    PE6: (pe6, 6, [0, 2]),
    PE7: (pe7, 7, [1, 6]),
    PE8: (pe8, 8, [1, 6]),
    PE9: (pe9, 9, [0, 1, 2, 6]),
    PE10: (pe10, 10, [0, 1, 6]),
    PE11: (pe11, 11, [0, 6]),
    PE12: (pe12, 12, [0, 2]),
    PE13: (pe13, 13, [1, 2]),
    PE14: (pe14, 14, [1, 2]),
    PE15: (pe15, 15, [1, 2]),
]);

#[cfg(feature = "gpio-l071")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [0]),
    PH1: (ph1, 1, []),
    PH9: (ph9, 9, []),
    PH10: (ph10, 10, []),
]);
/*
#[cfg(feature = "gpio-l071")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/
