// auto-generated using codegen
// STM32CubeMX DB release: DB.6.0.50
pub use super::*;

pub use super::Input as DefaultMode;

#[cfg(feature = "gpio-f302")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 3, 7, 15]),
    PA1: (pa1, 1, [0, 1, 3, 7, 9, 15]),
    PA2: (pa2, 2, [1, 3, 7, 8, 9, 15]),
    PA3: (pa3, 3, [1, 3, 7, 9, 15]),
    PA4: (pa4, 4, [3, 6, 7, 15]),
    PA5: (pa5, 5, [1, 3, 15]),
    PA6: (pa6, 6, [1, 3, 6, 15]),
    PA7: (pa7, 7, [1, 3, 6, 15]),
    PA8: (pa8, 8, [0, 3, 4, 5, 6, 7, 15]),
    PA9: (pa9, 9, [2, 3, 4, 5, 6, 7, 9, 10, 15]),
    PA10: (pa10, 10, [1, 3, 4, 5, 6, 7, 8, 10, 15]),
    PA11: (pa11, 11, [5, 6, 7, 9, 11, 12, 15]),
    PA12: (pa12, 12, [1, 5, 6, 7, 8, 9, 11, 15]),
    PA13: (pa13, 13, [0, 1, 3, 5, 7, 15], super::Debugger),
    PA14: (pa14, 14, [0, 3, 4, 6, 7, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 3, 4, 6, 7, 9, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f302")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [3, 6, 15]),
    PB1: (pb1, 1, [3, 6, 8, 15]),
    PB2: (pb2, 2, [3, 15]),
    PB3: (pb3, 3, [0, 1, 3, 6, 7, 15], super::Debugger),
    PB4: (pb4, 4, [0, 1, 3, 6, 7, 10, 15], super::Debugger),
    PB5: (pb5, 5, [1, 4, 6, 7, 8, 10, 15]),
    PB6: (pb6, 6, [1, 3, 4, 7, 15]),
    PB7: (pb7, 7, [1, 3, 4, 7, 15]),
    PB8: (pb8, 8, [1, 3, 4, 7, 9, 12, 15]),
    PB9: (pb9, 9, [1, 4, 6, 7, 8, 9, 15]),
    PB10: (pb10, 10, [1, 3, 7, 15]),
    PB11: (pb11, 11, [1, 3, 7, 15]),
    PB12: (pb12, 12, [3, 4, 5, 6, 7, 15]),
    PB13: (pb13, 13, [3, 5, 6, 7, 15]),
    PB14: (pb14, 14, [1, 3, 5, 6, 7, 15]),
    PB15: (pb15, 15, [0, 1, 2, 4, 5, 15]),
]);

#[cfg(feature = "gpio-f302")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2]),
    PC1: (pc1, 1, [1, 2]),
    PC2: (pc2, 2, [1, 2]),
    PC3: (pc3, 3, [1, 2, 6]),
    PC4: (pc4, 4, [1, 2, 7]),
    PC5: (pc5, 5, [1, 2, 3, 7]),
    PC6: (pc6, 6, [1, 6, 7]),
    PC7: (pc7, 7, [1, 6]),
    PC8: (pc8, 8, [1]),
    PC9: (pc9, 9, [1, 3, 5]),
    PC10: (pc10, 10, [1, 6, 7]),
    PC11: (pc11, 11, [1, 6, 7]),
    PC12: (pc12, 12, [1, 6, 7]),
    PC13: (pc13, 13, [4]),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f302")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD2: (pd2, 2, [1]),
]);

#[cfg(feature = "gpio-f302")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 5, 6]),
    PF1: (pf1, 1, [4, 5]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 3, 7, 8, 9, 10, 15]),
    PA1: (pa1, 1, [0, 1, 3, 7, 9, 15]),
    PA2: (pa2, 2, [1, 3, 7, 8, 9, 15]),
    PA3: (pa3, 3, [1, 3, 7, 9, 15]),
    PA4: (pa4, 4, [2, 3, 5, 6, 7, 15]),
    PA5: (pa5, 5, [1, 3, 5, 15]),
    PA6: (pa6, 6, [1, 2, 3, 4, 5, 6, 8, 15]),
    PA7: (pa7, 7, [1, 2, 3, 4, 5, 6, 15]),
    PA8: (pa8, 8, [0, 3, 4, 5, 6, 7, 8, 10, 15]),
    PA9: (pa9, 9, [2, 3, 4, 5, 6, 7, 8, 9, 10, 15]),
    PA10: (pa10, 10, [1, 3, 4, 5, 6, 7, 8, 10, 11, 15]),
    PA11: (pa11, 11, [5, 6, 7, 8, 9, 10, 11, 12, 15]),
    PA12: (pa12, 12, [1, 5, 6, 7, 8, 9, 10, 11, 15]),
    PA13: (pa13, 13, [0, 1, 3, 5, 7, 10, 15], super::Debugger),
    PA14: (pa14, 14, [0, 3, 4, 5, 6, 7, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 9, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [2, 3, 4, 6, 15]),
    PB1: (pb1, 1, [2, 3, 4, 6, 8, 15]),
    PB2: (pb2, 2, [3, 15]),
    PB3: (pb3, 3, [0, 1, 2, 3, 4, 5, 6, 7, 10, 15], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 4, 5, 6, 7, 10, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 3, 4, 5, 6, 7, 8, 10, 15]),
    PB6: (pb6, 6, [1, 2, 3, 4, 5, 6, 7, 10, 15]),
    PB7: (pb7, 7, [1, 2, 3, 4, 5, 7, 10, 12, 15]),
    PB8: (pb8, 8, [1, 2, 3, 4, 7, 8, 9, 10, 12, 15]),
    PB9: (pb9, 9, [1, 2, 4, 6, 7, 8, 9, 10, 15]),
    PB10: (pb10, 10, [1, 3, 7, 15]),
    PB11: (pb11, 11, [1, 3, 7, 15]),
    PB12: (pb12, 12, [3, 4, 5, 6, 7, 15]),
    PB13: (pb13, 13, [3, 5, 6, 7, 15]),
    PB14: (pb14, 14, [1, 3, 5, 6, 7, 15]),
    PB15: (pb15, 15, [0, 1, 2, 4, 5, 15]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2]),
    PC1: (pc1, 1, [1, 2]),
    PC2: (pc2, 2, [1, 2, 3]),
    PC3: (pc3, 3, [1, 2, 6]),
    PC4: (pc4, 4, [1, 2, 7]),
    PC5: (pc5, 5, [1, 2, 3, 7]),
    PC6: (pc6, 6, [1, 2, 4, 6, 7]),
    PC7: (pc7, 7, [1, 2, 4, 6, 7]),
    PC8: (pc8, 8, [1, 2, 4, 7]),
    PC9: (pc9, 9, [1, 2, 3, 4, 5, 6]),
    PC10: (pc10, 10, [1, 4, 5, 6, 7]),
    PC11: (pc11, 11, [1, 4, 5, 6, 7]),
    PC12: (pc12, 12, [1, 4, 5, 6, 7]),
    PC13: (pc13, 13, [1, 4]),
    PC14: (pc14, 14, [1]),
    PC15: (pc15, 15, [1]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [1, 7, 12]),
    PD1: (pd1, 1, [1, 4, 6, 7, 12]),
    PD2: (pd2, 2, [1, 2, 4, 5]),
    PD3: (pd3, 3, [1, 2, 7, 12]),
    PD4: (pd4, 4, [1, 2, 7, 12]),
    PD5: (pd5, 5, [1, 7, 12]),
    PD6: (pd6, 6, [1, 2, 7, 12]),
    PD7: (pd7, 7, [1, 2, 7, 12]),
    PD8: (pd8, 8, [1, 7, 12]),
    PD9: (pd9, 9, [1, 7, 12]),
    PD10: (pd10, 10, [1, 7, 12]),
    PD11: (pd11, 11, [1, 7, 12]),
    PD12: (pd12, 12, [1, 2, 3, 7, 12]),
    PD13: (pd13, 13, [1, 2, 3, 12]),
    PD14: (pd14, 14, [1, 2, 3, 12]),
    PD15: (pd15, 15, [1, 2, 3, 6, 12]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [1, 2, 4, 6, 7, 12]),
    PE1: (pe1, 1, [1, 4, 6, 7, 12]),
    PE2: (pe2, 2, [0, 1, 2, 3, 5, 6, 12]),
    PE3: (pe3, 3, [0, 1, 2, 3, 5, 6, 12]),
    PE4: (pe4, 4, [0, 1, 2, 3, 5, 6, 12]),
    PE5: (pe5, 5, [0, 1, 2, 3, 5, 6, 12]),
    PE6: (pe6, 6, [0, 1, 5, 6, 12]),
    PE7: (pe7, 7, [1, 2, 12]),
    PE8: (pe8, 8, [1, 2, 12]),
    PE9: (pe9, 9, [1, 2, 12]),
    PE10: (pe10, 10, [1, 2, 12]),
    PE11: (pe11, 11, [1, 2, 5, 12]),
    PE12: (pe12, 12, [1, 2, 5, 12]),
    PE13: (pe13, 13, [1, 2, 5, 12]),
    PE14: (pe14, 14, [1, 2, 5, 6, 12]),
    PE15: (pe15, 15, [1, 2, 7, 12]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [1, 4, 5, 6]),
    PF1: (pf1, 1, [1, 4, 5]),
    PF2: (pf2, 2, [1, 2, 12]),
    PF3: (pf3, 3, [1, 2, 12]),
    PF4: (pf4, 4, [1, 2, 3, 12]),
    PF5: (pf5, 5, [1, 2, 12]),
    PF6: (pf6, 6, [1, 2, 4, 7, 12]),
    PF7: (pf7, 7, [1, 2, 12]),
    PF8: (pf8, 8, [1, 2, 12]),
    PF9: (pf9, 9, [1, 2, 3, 5, 12]),
    PF10: (pf10, 10, [1, 2, 3, 5, 12]),
    PF11: (pf11, 11, [1, 2]),
    PF12: (pf12, 12, [1, 2, 12]),
    PF13: (pf13, 13, [1, 2, 12]),
    PF14: (pf14, 14, [1, 2, 12]),
    PF15: (pf15, 15, [1, 2, 12]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [1, 2, 12]),
    PG1: (pg1, 1, [1, 2, 12]),
    PG2: (pg2, 2, [1, 2, 12]),
    PG3: (pg3, 3, [1, 2, 12]),
    PG4: (pg4, 4, [1, 2, 12]),
    PG5: (pg5, 5, [1, 2, 12]),
    PG6: (pg6, 6, [1, 12]),
    PG7: (pg7, 7, [1, 12]),
    PG8: (pg8, 8, [1]),
    PG9: (pg9, 9, [1, 12]),
    PG10: (pg10, 10, [1, 12]),
    PG11: (pg11, 11, [1, 12]),
    PG12: (pg12, 12, [1, 12]),
    PG13: (pg13, 13, [1, 12]),
    PG14: (pg14, 14, [1, 12]),
    PG15: (pg15, 15, [1]),
]);

#[cfg(feature = "gpio-f303e")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [1, 2, 12]),
    PH1: (ph1, 1, [1, 2, 12]),
    PH2: (ph2, 2, [1]),
]);

#[cfg(feature = "gpio-f303")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 3, 7, 8, 9, 10, 15]),
    PA1: (pa1, 1, [0, 1, 3, 7, 9, 15]),
    PA2: (pa2, 2, [1, 3, 7, 8, 9, 15]),
    PA3: (pa3, 3, [1, 3, 7, 9, 15]),
    PA4: (pa4, 4, [2, 3, 5, 6, 7, 15]),
    PA5: (pa5, 5, [1, 3, 5, 15]),
    PA6: (pa6, 6, [1, 2, 3, 4, 5, 6, 8, 15]),
    PA7: (pa7, 7, [1, 2, 3, 4, 5, 6, 8, 15]),
    PA8: (pa8, 8, [0, 4, 5, 6, 7, 8, 10, 15]),
    PA9: (pa9, 9, [3, 4, 5, 6, 7, 8, 9, 10, 15]),
    PA10: (pa10, 10, [1, 3, 4, 6, 7, 8, 10, 11, 15]),
    PA11: (pa11, 11, [6, 7, 8, 9, 10, 11, 12, 14, 15]),
    PA12: (pa12, 12, [1, 6, 7, 8, 9, 10, 11, 14, 15]),
    PA13: (pa13, 13, [0, 1, 3, 5, 7, 10, 15], super::Debugger),
    PA14: (pa14, 14, [0, 3, 4, 5, 6, 7, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 4, 5, 6, 7, 9, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f303")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [2, 3, 4, 6, 15]),
    PB1: (pb1, 1, [2, 3, 4, 6, 8, 15]),
    PB2: (pb2, 2, [3, 15]),
    PB3: (pb3, 3, [0, 1, 2, 3, 4, 5, 6, 7, 10, 15], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 4, 5, 6, 7, 10, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 3, 4, 5, 6, 7, 10, 15]),
    PB6: (pb6, 6, [1, 2, 3, 4, 5, 6, 7, 10, 15]),
    PB7: (pb7, 7, [1, 2, 3, 4, 5, 7, 10, 15]),
    PB8: (pb8, 8, [1, 2, 3, 4, 8, 9, 10, 12, 15]),
    PB9: (pb9, 9, [1, 2, 4, 6, 8, 9, 10, 15]),
    PB10: (pb10, 10, [1, 3, 7, 15]),
    PB11: (pb11, 11, [1, 3, 7, 15]),
    PB12: (pb12, 12, [3, 4, 5, 6, 7, 15]),
    PB13: (pb13, 13, [3, 5, 6, 7, 15]),
    PB14: (pb14, 14, [1, 3, 5, 6, 7, 15]),
    PB15: (pb15, 15, [0, 1, 2, 4, 5, 15]),
]);

#[cfg(feature = "gpio-f303")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1]),
    PC1: (pc1, 1, [1]),
    PC2: (pc2, 2, [1, 3]),
    PC3: (pc3, 3, [1, 6]),
    PC4: (pc4, 4, [1, 7]),
    PC5: (pc5, 5, [1, 3, 7]),
    PC6: (pc6, 6, [1, 2, 4, 6, 7]),
    PC7: (pc7, 7, [1, 2, 4, 6, 7]),
    PC8: (pc8, 8, [1, 2, 4, 7]),
    PC9: (pc9, 9, [1, 2, 4, 5, 6]),
    PC10: (pc10, 10, [1, 4, 5, 6, 7]),
    PC11: (pc11, 11, [1, 4, 5, 6, 7]),
    PC12: (pc12, 12, [1, 4, 5, 6, 7]),
    PC13: (pc13, 13, [4]),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f303")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [1, 7]),
    PD1: (pd1, 1, [1, 4, 6, 7]),
    PD2: (pd2, 2, [1, 2, 4, 5]),
    PD3: (pd3, 3, [1, 2, 7]),
    PD4: (pd4, 4, [1, 2, 7]),
    PD5: (pd5, 5, [1, 7]),
    PD6: (pd6, 6, [1, 2, 7]),
    PD7: (pd7, 7, [1, 2, 7]),
    PD8: (pd8, 8, [1, 7]),
    PD9: (pd9, 9, [1, 7]),
    PD10: (pd10, 10, [1, 7]),
    PD11: (pd11, 11, [1, 7]),
    PD12: (pd12, 12, [1, 2, 3, 7]),
    PD13: (pd13, 13, [1, 2, 3]),
    PD14: (pd14, 14, [1, 2, 3]),
    PD15: (pd15, 15, [1, 2, 3, 6]),
]);

#[cfg(feature = "gpio-f303")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [1, 2, 4, 7]),
    PE1: (pe1, 1, [1, 4, 7]),
    PE2: (pe2, 2, [0, 1, 2, 3]),
    PE3: (pe3, 3, [0, 1, 2, 3]),
    PE4: (pe4, 4, [0, 1, 2, 3]),
    PE5: (pe5, 5, [0, 1, 2, 3]),
    PE6: (pe6, 6, [0, 1]),
    PE7: (pe7, 7, [1, 2]),
    PE8: (pe8, 8, [1, 2]),
    PE9: (pe9, 9, [1, 2]),
    PE10: (pe10, 10, [1, 2]),
    PE11: (pe11, 11, [1, 2]),
    PE12: (pe12, 12, [1, 2]),
    PE13: (pe13, 13, [1, 2]),
    PE14: (pe14, 14, [1, 2, 6]),
    PE15: (pe15, 15, [1, 2, 7]),
]);

#[cfg(feature = "gpio-f303")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 6]),
    PF1: (pf1, 1, [4]),
    PF2: (pf2, 2, [1]),
    PF4: (pf4, 4, [1, 2]),
    PF6: (pf6, 6, [1, 2, 4, 7]),
    PF9: (pf9, 9, [1, 3, 5]),
    PF10: (pf10, 10, [1, 3, 5]),
]);

#[cfg(feature = "gpio-f333")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 3, 7, 15]),
    PA1: (pa1, 1, [1, 3, 7, 9, 15]),
    PA2: (pa2, 2, [1, 3, 7, 8, 9, 15]),
    PA3: (pa3, 3, [1, 3, 7, 9, 15]),
    PA4: (pa4, 4, [2, 3, 5, 7, 15]),
    PA5: (pa5, 5, [1, 3, 5, 15]),
    PA6: (pa6, 6, [1, 2, 3, 5, 6, 13, 15]),
    PA7: (pa7, 7, [1, 2, 3, 5, 6, 15]),
    PA8: (pa8, 8, [0, 6, 7, 13, 15]),
    PA9: (pa9, 9, [3, 6, 7, 9, 10, 13, 15]),
    PA10: (pa10, 10, [1, 3, 6, 7, 8, 10, 13, 15]),
    PA11: (pa11, 11, [6, 7, 9, 11, 12, 13, 15]),
    PA12: (pa12, 12, [1, 6, 7, 8, 9, 11, 13, 15]),
    PA13: (pa13, 13, [0, 1, 3, 5, 7, 15], super::Debugger),
    PA14: (pa14, 14, [0, 3, 4, 6, 7, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 3, 4, 5, 7, 9, 13, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f333")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [2, 3, 6, 15]),
    PB1: (pb1, 1, [2, 3, 6, 8, 13, 15]),
    PB2: (pb2, 2, [3, 13, 15]),
    PB3: (pb3, 3, [0, 1, 3, 5, 7, 10, 12, 13, 15], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 5, 7, 10, 13, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 4, 5, 7, 10, 13, 15]),
    PB6: (pb6, 6, [1, 3, 4, 7, 12, 13, 15]),
    PB7: (pb7, 7, [1, 3, 4, 7, 10, 13, 15]),
    PB8: (pb8, 8, [1, 3, 4, 7, 9, 12, 13, 15]),
    PB9: (pb9, 9, [1, 4, 6, 7, 8, 9, 13, 15]),
    PB10: (pb10, 10, [1, 3, 7, 13, 15]),
    PB11: (pb11, 11, [1, 3, 7, 13, 15]),
    PB12: (pb12, 12, [3, 6, 7, 13, 15]),
    PB13: (pb13, 13, [3, 6, 7, 13, 15]),
    PB14: (pb14, 14, [1, 3, 6, 7, 13, 15]),
    PB15: (pb15, 15, [1, 2, 4, 13, 15]),
]);

#[cfg(feature = "gpio-f333")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2]),
    PC1: (pc1, 1, [1, 2]),
    PC2: (pc2, 2, [1, 2]),
    PC3: (pc3, 3, [1, 2, 6]),
    PC4: (pc4, 4, [1, 2, 7]),
    PC5: (pc5, 5, [1, 2, 3, 7]),
    PC6: (pc6, 6, [1, 2, 3, 7]),
    PC7: (pc7, 7, [1, 2, 3]),
    PC8: (pc8, 8, [1, 2, 3]),
    PC9: (pc9, 9, [1, 2, 3]),
    PC10: (pc10, 10, [1, 7]),
    PC11: (pc11, 11, [1, 3, 7]),
    PC12: (pc12, 12, [1, 3, 7]),
    PC13: (pc13, 13, [4]),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f333")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD2: (pd2, 2, [1, 2]),
]);

#[cfg(feature = "gpio-f333")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [6]),
    PF1: (pf1, 1, []),
]);

#[cfg(feature = "gpio-f373")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 11, 15]),
    PA1: (pa1, 1, [0, 1, 2, 3, 6, 7, 9, 11, 15]),
    PA2: (pa2, 2, [1, 2, 3, 6, 7, 8, 9, 11, 15]),
    PA3: (pa3, 3, [1, 2, 3, 6, 7, 9, 11, 15]),
    PA4: (pa4, 4, [2, 3, 5, 6, 7, 10, 15]),
    PA5: (pa5, 5, [1, 3, 5, 7, 9, 10, 15]),
    PA6: (pa6, 6, [1, 2, 3, 5, 8, 9, 15]),
    PA7: (pa7, 7, [1, 2, 3, 5, 8, 9, 15]),
    PA8: (pa8, 8, [0, 2, 4, 5, 7, 10, 15]),
    PA9: (pa9, 9, [2, 3, 4, 5, 7, 9, 10, 15]),
    PA10: (pa10, 10, [1, 3, 4, 5, 7, 9, 10, 15]),
    PA11: (pa11, 11, [2, 5, 6, 7, 8, 9, 10, 14, 15]),
    PA12: (pa12, 12, [1, 2, 6, 7, 8, 9, 10, 14, 15]),
    PA13: (pa13, 13, [0, 1, 2, 3, 5, 6, 7, 10, 15], super::Debugger),
    PA14: (pa14, 14, [0, 3, 4, 10, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 3, 4, 5, 6, 10, 15], super::Debugger),
]);

#[cfg(feature = "gpio-f373")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [2, 3, 5, 10, 15]),
    PB1: (pb1, 1, [2, 3, 15]),
    PB2: (pb2, 2, [15]),
    PB3: (pb3, 3, [0, 1, 2, 3, 5, 6, 7, 9, 10, 15], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 5, 6, 7, 9, 10, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 4, 5, 6, 7, 10, 11, 15]),
    PB6: (pb6, 6, [1, 2, 3, 4, 7, 9, 10, 11, 15]),
    PB7: (pb7, 7, [1, 2, 3, 4, 7, 9, 10, 11, 15]),
    PB8: (pb8, 8, [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 15]),
    PB9: (pb9, 9, [1, 2, 4, 5, 6, 7, 8, 9, 11, 15]),
    PB10: (pb10, 10, [1, 3, 5, 6, 7, 15]),
    PB14: (pb14, 14, [1, 3, 5, 7, 9, 15]),
    PB15: (pb15, 15, [0, 1, 2, 3, 5, 9, 15]),
]);

#[cfg(feature = "gpio-f373")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2]),
    PC1: (pc1, 1, [1, 2]),
    PC2: (pc2, 2, [1, 2, 5]),
    PC3: (pc3, 3, [1, 2, 5]),
    PC4: (pc4, 4, [1, 2, 3, 7]),
    PC5: (pc5, 5, [1, 3, 7]),
    PC6: (pc6, 6, [1, 2, 5]),
    PC7: (pc7, 7, [1, 2, 5]),
    PC8: (pc8, 8, [1, 2, 5]),
    PC9: (pc9, 9, [1, 2, 5]),
    PC10: (pc10, 10, [1, 2, 6, 7]),
    PC11: (pc11, 11, [1, 2, 6, 7]),
    PC12: (pc12, 12, [1, 2, 6, 7]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f373")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [1, 2, 7]),
    PD1: (pd1, 1, [1, 2, 7]),
    PD2: (pd2, 2, [1, 2]),
    PD3: (pd3, 3, [1, 5, 7]),
    PD4: (pd4, 4, [1, 5, 7]),
    PD5: (pd5, 5, [1, 7]),
    PD6: (pd6, 6, [1, 5, 7]),
    PD7: (pd7, 7, [1, 5, 7]),
    PD8: (pd8, 8, [1, 3, 5, 7]),
    PD9: (pd9, 9, [1, 3, 7]),
    PD10: (pd10, 10, [1, 7]),
    PD11: (pd11, 11, [1, 7]),
    PD12: (pd12, 12, [1, 2, 3, 7]),
    PD13: (pd13, 13, [1, 2, 3]),
    PD14: (pd14, 14, [1, 2, 3]),
    PD15: (pd15, 15, [1, 2, 3]),
]);

#[cfg(feature = "gpio-f373")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [1, 2, 7]),
    PE1: (pe1, 1, [1, 7]),
    PE2: (pe2, 2, [0, 1, 3]),
    PE3: (pe3, 3, [0, 1, 3]),
    PE4: (pe4, 4, [0, 1, 3]),
    PE5: (pe5, 5, [0, 1, 3]),
    PE6: (pe6, 6, [0, 1]),
    PE7: (pe7, 7, [1]),
    PE8: (pe8, 8, [1]),
    PE9: (pe9, 9, [1]),
    PE10: (pe10, 10, [1]),
    PE11: (pe11, 11, [1]),
    PE12: (pe12, 12, [1]),
    PE13: (pe13, 13, [1]),
    PE14: (pe14, 14, [1]),
    PE15: (pe15, 15, [1, 7]),
]);

#[cfg(feature = "gpio-f373")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4]),
    PF1: (pf1, 1, [4]),
    PF2: (pf2, 2, [1, 4]),
    PF4: (pf4, 4, [1]),
    PF6: (pf6, 6, [1, 2, 4, 5, 7]),
    PF7: (pf7, 7, [1, 4, 7]),
    PF9: (pf9, 9, [1, 2]),
    PF10: (pf10, 10, [1]),
]);
