use super::*;

// See "Datasheet - production data"
// Pinouts and pin description (page 66..)
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog>  => (ADC1, 8),
    gpio::PB1<Analog>  => (ADC1, 9),
    gpio::PC0<Analog>  => (ADC1, 10),
    gpio::PC1<Analog>  => (ADC1, 11),
    gpio::PC2<Analog>  => (ADC1, 12),
    gpio::PC3<Analog>  => (ADC1, 13),
    gpio::PC4<Analog>  => (ADC1, 14),
    gpio::PC5<Analog>  => (ADC1, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

adc_pins!(
    gpio::PA0<Analog>  => (ADC2, 0),
    gpio::PA1<Analog>  => (ADC2, 1),
    gpio::PA2<Analog>  => (ADC2, 2),
    gpio::PA3<Analog>  => (ADC2, 3),
    gpio::PA4<Analog>  => (ADC2, 4),
    gpio::PA5<Analog>  => (ADC2, 5),
    gpio::PA6<Analog>  => (ADC2, 6),
    gpio::PA7<Analog>  => (ADC2, 7),
    gpio::PB0<Analog>  => (ADC2, 8),
    gpio::PB1<Analog>  => (ADC2, 9),
    gpio::PC0<Analog>  => (ADC2, 10),
    gpio::PC1<Analog>  => (ADC2, 11),
    gpio::PC2<Analog>  => (ADC2, 12),
    gpio::PC3<Analog>  => (ADC2, 13),
    gpio::PC4<Analog>  => (ADC2, 14),
    gpio::PC5<Analog>  => (ADC2, 15),
);

adc_pins!(
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);
