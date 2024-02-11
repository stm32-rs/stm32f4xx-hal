use super::*;

macro_rules! adc_pins {
    ($ADC:ty, $($pin:ty => $chan:literal),+ $(,)*) => {
        $(
            impl embedded_hal_02::adc::Channel<$ADC> for $pin {
                type ID = u8;

                fn channel() -> u8 { $chan }
            }
        )+
    };
}

// See "Datasheet - production data"
// Pinouts and pin description (page 66..)
adc_pins!(pac::ADC1,
    gpio::PA0<Analog> => 0,
    gpio::PA1<Analog> => 1,
    gpio::PA2<Analog> => 2,
    gpio::PA3<Analog> => 3,
    gpio::PA4<Analog> => 4,
    gpio::PA5<Analog> => 5,
    gpio::PA6<Analog> => 6,
    gpio::PA7<Analog> => 7,
    gpio::PB0<Analog>  => 8,
    gpio::PB1<Analog>  => 9,
    gpio::PC0<Analog>  => 10,
    gpio::PC1<Analog>  => 11,
    gpio::PC2<Analog>  => 12,
    gpio::PC3<Analog>  => 13,
    gpio::PC4<Analog>  => 14,
    gpio::PC5<Analog>  => 15,
    Temperature => 18,
    Vbat => 18,
    Vref => 17,
);

adc_pins!(pac::ADC2,
    gpio::PA0<Analog>  => 0,
    gpio::PA1<Analog>  => 1,
    gpio::PA2<Analog>  => 2,
    gpio::PA3<Analog>  => 3,
    gpio::PA4<Analog>  => 4,
    gpio::PA5<Analog>  => 5,
    gpio::PA6<Analog>  => 6,
    gpio::PA7<Analog>  => 7,
    gpio::PB0<Analog>  => 8,
    gpio::PB1<Analog>  => 9,
    gpio::PC0<Analog>  => 10,
    gpio::PC1<Analog>  => 11,
    gpio::PC2<Analog>  => 12,
    gpio::PC3<Analog>  => 13,
    gpio::PC4<Analog>  => 14,
    gpio::PC5<Analog>  => 15,
);

adc_pins!(pac::ADC3,
    gpio::PA0<Analog> => 0,
    gpio::PA1<Analog> => 1,
    gpio::PA2<Analog> => 2,
    gpio::PA3<Analog> => 3,
    gpio::PF6<Analog> => 4,
    gpio::PF7<Analog> => 5,
    gpio::PF8<Analog> => 6,
    gpio::PF9<Analog> => 7,
    gpio::PF10<Analog> => 8,
    gpio::PF3<Analog> => 9,
    gpio::PC0<Analog> => 10,
    gpio::PC1<Analog> => 11,
    gpio::PC2<Analog> => 12,
    gpio::PC3<Analog> => 13,
    gpio::PF4<Analog> => 14,
    gpio::PF5<Analog> => 15,
);
