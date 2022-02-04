use crate::gpio::{self, Alternate};

// Output channels markers
pub trait CPin<TIM, const C: u8> {}
pub struct Ch<const C: u8>;
pub const C1: u8 = 0;
pub const C2: u8 = 1;
pub const C3: u8 = 2;
pub const C4: u8 = 3;

macro_rules! channel_impl {
    ( $( $TIM:ident, $C:ident, $PINX:ident, $AF:literal; )+ ) => {
        $(
            impl<Otype> CPin<crate::pac::$TIM, $C> for gpio::$PINX<Alternate<Otype, $AF>> { }
        )+
    };
}

// The approach to PWM channel implementation is to group parts with
// common pins, starting with groupings of the largest number of parts
// and moving to smaller and smaller groupings.  Last, we have individual
// parts to cover exceptions.

// All parts have these PWM pins.
channel_impl!(
    TIM1, C1, PA8, 1;
    TIM1, C2, PA9, 1;
    TIM1, C3, PA10, 1;
    TIM1, C4, PA11, 1;

    TIM5, C1, PA0, 2;
    TIM5, C2, PA1, 2;
    TIM5, C3, PA2, 2;
    TIM5, C4, PA3, 2;

    TIM9, C1, PA2, 3;
    TIM9, C2, PA3, 3;

    TIM11, C1, PB9, 3;
);

// All parts except F410.
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM1, C1, PE9, 1;
    TIM1, C2, PE11, 1;
    TIM1, C3, PE13, 1;
    TIM1, C4, PE14, 1;

    TIM2, C1, PA0, 1;
    TIM2, C2, PA1, 1;
    TIM2, C3, PA2, 1;
    TIM2, C4, PA3, 1;

    TIM2, C2, PB3, 1;
    TIM2, C3, PB10, 1;
    TIM2, C4, PB11, 1;

    TIM2, C1, PA5, 1;
    TIM2, C1, PA15, 1;

    TIM3, C1, PA6, 2;
    TIM3, C2, PA7, 2;
    TIM3, C3, PB0, 2;
    TIM3, C4, PB1, 2;

    TIM3, C1, PB4, 2;
    TIM3, C2, PB5, 2;

    TIM3, C1, PC6, 2;
    TIM3, C2, PC7, 2;
    TIM3, C3, PC8, 2;
    TIM3, C4, PC9, 2;

    TIM4, C1, PB6, 2;
    TIM4, C2, PB7, 2;
    TIM4, C3, PB8, 2;
    TIM4, C4, PB9, 2;

    TIM4, C1, PD12, 2;
    TIM4, C2, PD13, 2;
    TIM4, C3, PD14, 2;
    TIM4, C4, PD15, 2;

    TIM10, C1, PB8, 3;
);

// All parts except F401 and F410.
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM9, C1, PE5, 3;
    TIM9, C2, PE6, 3;
);

// All parts except F401, F410, and F411.
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM8, C1, PC6, 3;
    TIM8, C2, PC7, 3;
    TIM8, C3, PC8, 3;
    TIM8, C4, PC9, 3;

    TIM10, C1, PF6, 3;

    TIM11, C1, PF7, 3;

    TIM12, C1, PB14, 9;
    TIM12, C2, PB15, 9;

    TIM13, C1, PA6, 9;
    TIM13, C1, PF8, 9;  // Not a mistake: TIM13 has only one channel.

    TIM14, C1, PA7, 9;
    TIM14, C1, PF9, 9;  // Not a mistake: TIM14 has only one channel.
);

// STM's "advanced and foundation" lines except F446.
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
channel_impl!(
    TIM5, C1, PH10, 2;
    TIM5, C2, PH11, 2;
    TIM5, C3, PH12, 2;
    TIM5, C4, PI0, 2;

    TIM8, C1, PI5, 3;
    TIM8, C2, PI6, 3;
    TIM8, C3, PI7, 3;
    TIM8, C4, PI2, 3;

    TIM12, C1, PH6, 9;
    TIM12, C2, PH9, 9;
);

#[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
channel_impl!(
    TIM5, C1, PF3, 2;
    TIM5, C2, PF4, 2;
    TIM5, C3, PF5, 2;
    TIM5, C4, PF10, 2;
);

#[cfg(feature = "stm32f410")]
channel_impl!(
    TIM5, C1, PB12, 2;
    TIM5, C2, PC10, 2;
    TIM5, C3, PC11, 2;
    TIM5, C4, PB11, 2;

    TIM9, C1, PC4, 3;
    TIM9, C2, PC5, 3;

    TIM11, C1, PC13, 3;
);

#[cfg(feature = "stm32f446")]
channel_impl!(
    TIM2, C1, PB8, 1;
    TIM2, C2, PB9, 1;

    TIM2, C4, PB2, 1;
);
