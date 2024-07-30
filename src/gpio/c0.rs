// auto-generated using codegen
// STM32CubeMX DB release: DB.6.0.50
pub use super::*;

pub use super::Analog as DefaultMode;

#[cfg(feature = "gpio-c0xx_453")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 4, 5]),
    PA1: (pa1, 1, [0, 1, 2, 4, 5, 6, 7]),
    PA2: (pa2, 2, [0, 1, 2, 3, 5]),
    PA3: (pa3, 3, [1, 2, 5, 7]),
    PA4: (pa4, 4, [0, 1, 2, 4, 5, 7]),
    PA5: (pa5, 5, [0, 1, 2, 5, 7]),
    PA6: (pa6, 6, [0, 1, 2, 5]),
    PA7: (pa7, 7, [0, 1, 2, 4, 5]),
    PA8: (pa8, 8, [0, 1, 2, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PA9: (pa9, 9, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA10: (pa10, 10, [0, 1, 2, 3, 5, 6, 7]),
    PA11: (pa11, 11, [0, 1, 2, 5]),
    PA12: (pa12, 12, [0, 1, 2, 5]),
    PA13: (pa13, 13, [0, 1, 3, 4, 7], super::Debugger),
    PA14: (pa14, 14, [0, 1, 7, 8, 9, 10, 11, 12], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 4, 7], super::Debugger),
]);

#[cfg(feature = "gpio-c0xx_453")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2]),
    PB1: (pb1, 1, [0, 1, 2, 5, 7]),
    PB2: (pb2, 2, [0, 3, 7]),
    PB3: (pb3, 3, [0, 1, 3, 4, 7], super::Debugger),
    PB4: (pb4, 4, [0, 1, 4, 5, 7], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3, 6]),
    PB6: (pb6, 6, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PB7: (pb7, 7, [0, 1, 2, 3, 6, 7, 9, 10, 11, 14]),
    PB8: (pb8, 8, [1, 2, 3, 6, 7]),
    PB9: (pb9, 9, [0, 1, 2, 3, 6, 7]),
    PB10: (pb10, 10, []),
    PB11: (pb11, 11, []),
    PB12: (pb12, 12, [1, 2, 7]),
    PB13: (pb13, 13, [2, 7]),
    PB14: (pb14, 14, [2, 7]),
    PB15: (pb15, 15, [2, 7]),
]);

#[cfg(feature = "gpio-c0xx_453")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC6: (pc6, 6, [1]),
    PC7: (pc7, 7, [1]),
    PC13: (pc13, 13, [1, 2]),
    PC14: (pc14, 14, [0, 1, 2, 8, 9, 10, 11, 14, 15]),
    PC15: (pc15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "gpio-c0xx_453")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [0, 2]),
    PD1: (pd1, 1, [0, 2]),
    PD2: (pd2, 2, [1, 2]),
    PD3: (pd3, 3, [0, 2]),
]);

#[cfg(feature = "gpio-c0xx_453")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [2]),
    PF1: (pf1, 1, [0]),
    PF2: (pf2, 2, [0, 1]),
    PF3: (pf3, 3, []),
]);
/*
#[cfg(feature = "gpio-c0xx_453")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
 */

#[cfg(feature = "gpio-c0xx")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 4, 5]),
    PA1: (pa1, 1, [0, 1, 2, 4, 5, 6, 7]),
    PA2: (pa2, 2, [0, 1, 2, 3, 5]),
    PA3: (pa3, 3, [1, 2, 5, 7]),
    PA4: (pa4, 4, [0, 1, 2, 4, 5, 7]),
    PA5: (pa5, 5, [0, 1, 2, 5, 7]),
    PA6: (pa6, 6, [0, 1, 2, 5]),
    PA7: (pa7, 7, [0, 1, 2, 4, 5]),
    PA8: (pa8, 8, [0, 1, 2, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PA9: (pa9, 9, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA10: (pa10, 10, [0, 1, 2, 3, 5, 6, 7]),
    PA11: (pa11, 11, [0, 1, 2, 5]),
    PA12: (pa12, 12, [0, 1, 2, 5]),
    PA13: (pa13, 13, [0, 1, 3, 4, 7], super::Debugger),
    PA14: (pa14, 14, [0, 1, 7, 8, 9, 10, 11, 12], super::Debugger),
]);

#[cfg(feature = "gpio-c0xx")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB6: (pb6, 6, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PB7: (pb7, 7, [0, 1, 2, 3, 6, 7, 9, 10, 11, 14]),
]);

#[cfg(feature = "gpio-c0xx")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, [0, 1, 2, 8, 9, 10, 11, 14, 15]),
    PC15: (pc15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "gpio-c0xx")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF2: (pf2, 2, [0, 1]),
]);
/*
#[cfg(feature = "gpio-c0xx")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/
