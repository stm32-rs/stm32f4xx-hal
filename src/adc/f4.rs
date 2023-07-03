use super::*;

macro_rules! adc_pins {
    ($($pin:ty => ($adc:ident, $chan:expr)),+ $(,)*) => {
        $(
            impl embedded_hal::adc::Channel<pac::$adc> for $pin {
                type ID = u8;
                fn channel() -> u8 { $chan }
            }
        )+
    };
}

#[cfg(feature = "stm32f401")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on C variant
#[cfg(feature = "stm32f401")]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(any(feature = "stm32f405", feature = "stm32f415"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    Temperature => (ADC1, 16),
    Temperature => (ADC2, 16),
    Temperature => (ADC3, 16),
    Vbat => (ADC1, 18),
    Vbat => (ADC2, 18),
    Vbat => (ADC3, 18),
    Vref => (ADC1, 17),
    Vref => (ADC2, 17),
    Vref => (ADC3, 17),
);

// Not available on O variant
#[cfg(any(feature = "stm32f405", feature = "stm32f415"))]
adc_pins!(
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(any(feature = "stm32f407", feature = "stm32f417"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    Temperature => (ADC1, 16),
    Temperature => (ADC2, 16),
    Temperature => (ADC3, 16),
    Vbat => (ADC1, 18),
    Vbat => (ADC2, 18),
    Vbat => (ADC3, 18),
    Vref => (ADC1, 17),
    Vref => (ADC2, 17),
    Vref => (ADC3, 17),
);

// Not available on V variant
#[cfg(any(feature = "stm32f407", feature = "stm32f417"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(feature = "stm32f410")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA5<Analog> => (ADC1, 5),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 16),
    Vref => (ADC1, 17),
);

// Not available on T variant
#[cfg(feature = "stm32f410")]
adc_pins!(
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
);

// Only available on R variant
#[cfg(feature = "stm32f410")]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(feature = "stm32f411")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on C variant
#[cfg(feature = "stm32f411")]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(feature = "stm32f412")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on C variant
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on V variant
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);

// Only available on I and Z variants
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
adc_pins!(
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(any(feature = "stm32f429", feature = "stm32f439"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on V variant
#[cfg(any(feature = "stm32f429", feature = "stm32f439"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);

// Not available on V or A variants
#[cfg(any(feature = "stm32f429", feature = "stm32f439"))]
adc_pins!(
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(feature = "stm32f446")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on M variant
#[cfg(feature = "stm32f446")]
adc_pins!(
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    gpio::PC5<Analog> => (ADC3, 15),
);

// Only available on Z variant
#[cfg(feature = "stm32f446")]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on A variant
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
);

// Not available on V or A variants
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
);

// Not available on V variant
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);

// Only available on B/I/N variants
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);
