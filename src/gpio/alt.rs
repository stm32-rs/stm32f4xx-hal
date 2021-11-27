use super::*;
use crate::{i2c, i2s, pac, serial, spi};

pub struct Const<const A: u8>;

pub trait SetAlternate<Otype, const A: u8> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}
impl<Otype> SetAlternate<Otype, 0> for NoPin {
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}
impl<MODE: PinMode, const P: char, const N: u8, const A: u8> SetAlternate<PushPull, A>
    for Pin<MODE, P, N>
{
    fn set_alt_mode(&mut self) {
        self.set_alternate::<A>();
    }

    fn restore_mode(&mut self) {
        self.mode::<MODE>();
    }
}

impl<MODE: PinMode, const P: char, const N: u8, const A: u8> SetAlternate<OpenDrain, A>
    for Pin<MODE, P, N>
{
    fn set_alt_mode(&mut self) {
        self.set_alternate::<A>();
        unsafe {
            (*Gpio::<P>::ptr())
                .otyper
                .modify(|r, w| w.bits(r.bits() | (1 << N)))
        };
    }

    fn restore_mode(&mut self) {
        self.mode::<MODE>();
    }
}

impl<const P: char, const N: u8, const A: u8> SetAlternate<PushPull, A>
    for Pin<Alternate<PushPull, A>, P, N>
{
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}

impl<const P: char, const N: u8, const A: u8> SetAlternate<OpenDrain, A>
    for Pin<Alternate<OpenDrain, A>, P, N>
{
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}

pub trait PinA<PIN, PER> {
    type A;
}

impl<PIN, PER> PinA<PIN, PER> for NoPin
where
    PIN: crate::Sealed,
    PER: crate::Sealed,
{
    type A = Const<0>;
}

macro_rules! pin {
    ( $(<$Pin:ty, $I2C:ident> for [$($gpio:ident::$PX:ident<$A:literal>),*]),*) => {
        $(
            $(
                impl<MODE> PinA<$Pin, pac::$I2C> for $gpio::$PX<MODE> {
                    type A = Const<$A>;
                }
            )*
        )*
    };
}

// CAN pins

#[cfg(all(feature = "can", any(feature = "can1", feature = "can2")))]
mod can {
    use super::*;
    use crate::can;

    pin! {
        <can::Tx, CAN1> for [gpioa::PA12<9>, gpiod::PD1<9>],
        <can::Rx, CAN1> for [gpioa::PA11<9>, gpiod::PD0<9>],

        <can::Tx, CAN2> for [gpiob::PB13<9>, gpiob::PB6<9>],
        <can::Rx, CAN2> for [gpiob::PB12<9>, gpiob::PB5<9>]
    }

    #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
    pin! {
        <can::Tx, CAN1> for [gpiob::PB9<8>],
        <can::Rx, CAN1> for [gpiob::PB8<8>]
    }

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
    pin! {
        <can::Tx, CAN1> for [gpiob::PB9<9>],
        <can::Rx, CAN1> for [gpiob::PB8<9>]
    }

    #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
    pin! {
        <can::Tx, CAN1> for [gpiog::PG1<9>],
        <can::Rx, CAN1> for [gpiog::PG0<9>],

        <can::Tx, CAN2> for [gpiog::PG12<9>],
        <can::Rx, CAN2> for [gpiog::PG11<9>]
    }

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
    pin! {
        <can::Tx, CAN1> for [gpioh::PH13<9>],
        <can::Rx, CAN1> for [gpioi::PI9<9>]
    }

    #[cfg(feature = "can3")]
    pin! {
        <can::Tx, CAN3> for [gpioa::PA15<11>, gpiob::PB4<11>],
        <can::Rx, CAN3> for [gpioa::PA8<11>, gpiob::PB3<11>]
    }
}

// I2C pins

pin! {
    <i2c::Scl, I2C1> for [gpiob::PB6<4>, gpiob::PB8<4>],
    <i2c::Sda, I2C1> for [gpiob::PB7<4>, gpiob::PB9<4>]
}

#[cfg(any(feature = "stm32f446"))]
pin! { <i2c::Sda, I2C2> for [gpiob::PB3<4>] }

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C2> for [gpiob::PB3<9>] }

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C2> for [gpiob::PB9<9>] }

pin! { <i2c::Scl, I2C2> for [gpiob::PB10<4>] }

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
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
pin! { <i2c::Sda, I2C2> for [gpiob::PB11<4>] }

#[cfg(any(feature = "stm32f446"))]
pin! { <i2c::Sda, I2C2> for [gpioc::PC12<4>] }

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
pin! {
    <i2c::Scl, I2C2> for [gpiof::PF1<4>],
    <i2c::Sda, I2C2> for [gpiof::PF0<4>]
}

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
pin! {
    <i2c::Scl, I2C2> for [gpioh::PH4<4>],
    <i2c::Sda, I2C2> for [gpioh::PH5<4>]
}

#[cfg(feature = "i2c3")]
pin! {
    <i2c::Scl, I2C3> for [gpioa::PA8<4>],
    <i2c::Sda, I2C3> for [gpioc::PC9<4>]
}

#[cfg(feature = "stm32f446")]
pin! { <i2c::Sda, I2C3> for [gpiob::PB4<4>] }

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C3> for [gpiob::PB4<9>] }

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C3> for [gpiob::PB8<9>] }

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
pin! {
    <i2c::Scl, I2C3> for [gpioh::PH7<4>],
    <i2c::Sda, I2C3> for [gpioh::PH8<4>]
}

#[cfg(feature = "fmpi2c1")]
pin! {
    <i2c::Scl, FMPI2C1> for [gpioc::PC6<4>],
    <i2c::Sda, FMPI2C1> for [gpioc::PC7<4>],
    <i2c::Sda, FMPI2C1> for [gpiob::PB3<4>],
    <i2c::Scl, FMPI2C1> for [gpiob::PB10<9>],
    <i2c::Sda, FMPI2C1> for [gpiob::PB14<4>],
    <i2c::Scl, FMPI2C1> for [gpiob::PB15<4>],
    <i2c::Scl, FMPI2C1> for [gpiod::PD12<4>],
    <i2c::Scl, FMPI2C1> for [gpiob::PB13<4>],
    <i2c::Scl, FMPI2C1> for [gpiod::PD14<4>],
    <i2c::Scl, FMPI2C1> for [gpiod::PD15<4>],
    <i2c::Scl, FMPI2C1> for [gpiof::PF14<4>],
    <i2c::Scl, FMPI2C1> for [gpiof::PF15<4>]
}

// SPI pins

pin! {
    <spi::Sck,  SPI1> for [gpioa::PA5<5>, gpiob::PB3<5>],
    <spi::Miso, SPI1> for [gpioa::PA6<5>, gpiob::PB4<5>],
    <spi::Mosi, SPI1> for [gpioa::PA7<5>, gpiob::PB5<5>],

    <spi::Sck,  SPI2> for [gpiob::PB10<5>, gpiob::PB13<5>],
    <spi::Miso, SPI2> for [gpiob::PB14<5>, gpioc::PC2<5>],
    <spi::Mosi, SPI2> for [gpiob::PB15<5>, gpioc::PC3<5>],
    <spi::Nss,  SPI2> for [gpiob::PB9<5>, gpiob::PB12<5>],
    <i2s::Mck,  SPI2> for [gpioc::PC6<5>]
}

#[cfg(feature = "spi3")]
pin! {
    <spi::Sck,  SPI3> for [gpiob::PB3<6>, gpioc::PC10<6>],
    <spi::Miso, SPI3> for [gpiob::PB4<6>, gpioc::PC11<6>],
    <spi::Mosi, SPI3> for [gpiob::PB5<6>, gpioc::PC12<6>],
    <spi::Nss,  SPI3> for [gpioa::PA4<6>, gpioa::PA15<6>],
    <i2s::Mck,  SPI3> for [gpioc::PC7<6>]
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pin! {
    <spi::Sck,  SPI2> for [gpiod::PD3<5>],

    <spi::Mosi, SPI3> for [gpiod::PD6<5>],

    <spi::Sck,  SPI4> for [gpioe::PE2<5>, gpioe::PE12<5>],
    <spi::Miso, SPI4> for [gpioe::PE5<5>, gpioe::PE13<5>],
    <spi::Mosi, SPI4> for [gpioe::PE6<5>, gpioe::PE14<5>]
}

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
pin! {
    <spi::Sck,  SPI2> for [gpioi::PI1<5>],
    <spi::Miso, SPI2> for [gpioi::PI2<5>],
    <spi::Mosi, SPI2> for [gpioi::PI3<5>],
    <spi::Nss,  SPI2> for [gpioi::PI0<5>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
pin! {
    <spi::Sck, SPI2> for [gpioc::PC7<5>],
    <spi::Nss,  SPI1> for [gpioa::PA4<5>, gpioa::PA15<5>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <spi::Sck,  SPI5> for [gpiob::PB0<6>],
    <spi::Miso, SPI5> for [gpioa::PA12<6>],
    <spi::Mosi, SPI5> for [gpioa::PA10<6>, gpiob::PB8<6>],
    <spi::Nss,  SPI5> for [gpiob::PB1<6>]
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <i2s::Mck,  SPI2> for [gpioa::PA3<5>, gpioa::PA6<6>],

    <spi::Sck,  SPI3> for [gpiob::PB12<7>],
    <i2s::Mck,  SPI3> for [gpiob::PB10<6>],

    <spi::Sck,  SPI4> for [gpiob::PB13<6>],
    <spi::Miso, SPI4> for [gpioa::PA11<6>],
    <spi::Mosi, SPI4> for [gpioa::PA1<5>],
    <spi::Nss,  SPI4> for [gpiob::PB12<6>, gpioe::PE4<5>, gpioe::PE11<5>],

    <spi::Sck,  SPI5> for [gpioe::PE2<6>, gpioe::PE12<6>],
    <spi::Miso, SPI5> for [gpioe::PE5<6>, gpioe::PE13<6>],
    <spi::Mosi, SPI5> for [gpioe::PE6<6>, gpioe::PE14<6>],
    <spi::Nss,  SPI5> for [gpioe::PE4<6>, gpioe::PE11<6>]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin! {
    <spi::Sck,  SPI2> for [gpioa::PA9<5>],
    <spi::Miso, SPI2> for [gpioa::PA12<5>],
    <spi::Mosi, SPI2> for [gpioa::PA10<5>],
    <spi::Nss,  SPI2> for [gpioa::PA11<5>]
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pin! {
    <spi::Sck,  SPI5> for [gpiof::PF7<5>, gpioh::PH6<5>],
    <spi::Miso, SPI5> for [gpiof::PF8<5>, gpioh::PH7<5>],
    <spi::Mosi, SPI5> for [gpiof::PF9<5>, gpiof::PF11<5>],

    <spi::Sck,  SPI6> for [gpiog::PG13<5>],
    <spi::Miso, SPI6> for [gpiog::PG12<5>],
    <spi::Mosi, SPI6> for [gpiog::PG14<5>]
}

#[cfg(feature = "stm32f446")]
pin! {
    <spi::Sck,  SPI2> for [gpioa::PA9<5>],
    <spi::Mosi, SPI2> for [gpioc::PC1<7>],
    <spi::Nss,  SPI2> for [gpiob::PB4<7>, gpiod::PD1<7>],

    <spi::Mosi, SPI3> for [gpiob::PB0<7>, gpiob::PB2<7>, gpiod::PD0<6>],

    <spi::Sck,  SPI4> for [gpiog::PG11<6>],
    <spi::Miso, SPI4> for [gpiog::PG12<6>, gpiod::PD0<5>],
    <spi::Mosi, SPI4> for [gpiog::PG13<6>]
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
pin! {
    <spi::Sck,  SPI2> for [gpioa::PA9<5>],
    <spi::Mosi, SPI2> for [gpioc::PC1<5>]
}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
pin! { <i2s::Mck,  SPI1> for [gpioc::PC4<5>] }

#[cfg(feature = "stm32f410")]
pin! { <i2s::Mck,  SPI1> for [gpioc::PC7<6>, gpiob::PB10<6>] }

// Serial pins

pin! {
    <serial::TxPin, USART1> for [gpioa::PA9<7>, gpiob::PB6<7>],
    <serial::RxPin, USART1> for [gpioa::PA10<7>, gpiob::PB7<7>],

    <serial::TxPin, USART2> for [gpioa::PA2<7>],
    <serial::RxPin, USART2> for [gpioa::PA3<7>],

    <serial::TxPin, USART6> for [gpioc::PC6<8>],
    <serial::RxPin, USART6> for [gpioc::PC7<8>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <serial::TxPin, USART1> for [gpioa::PA15<7>],
    <serial::RxPin, USART1> for [gpiob::PB3<7>]
}

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
pin! {
    <serial::TxPin, USART2> for [gpiod::PD5<7>],
    <serial::RxPin, USART2> for [gpiod::PD6<7>]
}

#[cfg(feature = "usart3")]
pin! {
    <serial::TxPin, USART3> for [gpiob::PB10<7>],
    <serial::RxPin, USART3> for [gpiob::PB11<7>]
}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
pin! {
    <serial::RxPin, USART3> for [gpioc::PC5<7>]
}

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
pin! {
    <serial::TxPin, USART3> for [gpioc::PC10<7>],
    <serial::RxPin, USART3> for [gpioc::PC11<7>]
}

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
pin! {
    <serial::TxPin, USART3> for [gpiod::PD8<7>],
    <serial::RxPin, USART3> for [gpiod::PD9<7>]
}

#[cfg(feature = "uart4")]
pin! {
    <serial::TxPin, UART4> for [gpioa::PA0<8>],
    <serial::RxPin, UART4> for [gpioa::PA1<8>]
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pin! {
    <serial::TxPin, UART4> for [gpioc::PC10<8>],
    <serial::RxPin, UART4> for [gpioc::PC11<8>]
}
#[cfg(feature = "uart5")]
pin! {
    <serial::TxPin, UART5> for [gpioc::PC12<8>],
    <serial::RxPin, UART5> for [gpiod::PD2<8>]
}

#[cfg(any(feature = "stm32f446"))]
pin! {
    <serial::TxPin, UART5> for [gpioe::PE8<8>],
    <serial::RxPin, UART5> for [gpioe::PE7<8>]
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <serial::TxPin, USART6> for [gpioa::PA11<8>],
    <serial::RxPin, USART6> for [gpioa::PA12<8>]
}

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
pin! {
    <serial::TxPin, USART6> for [gpiog::PG14<8>],
    <serial::RxPin, USART6> for [gpiog::PG9<8>]
}

#[cfg(all(feature = "uart7", feature = "gpioe"))]
pin! {
    <serial::TxPin, UART7> for [gpioe::PE8<8>],
    <serial::RxPin, UART7> for [gpioe::PE7<8>]
}

#[cfg(all(feature = "uart7", feature = "gpiof"))]
pin! {
    <serial::TxPin, UART7> for [gpiof::PF7<8>],
    <serial::RxPin, UART7> for [gpiof::PF6<8>]
}

#[cfg(all(feature = "uart8", feature = "gpioe"))]
pin! {
    <serial::TxPin, UART8> for [gpioe::PE1<8>],
    <serial::RxPin, UART8> for [gpioe::PE0<8>]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin! {
    <serial::TxPin, UART4> for [gpioa::PA12<11>, gpiod::PD1<11>, gpiod::PD10<8>],
    <serial::RxPin, UART4> for [gpioa::PA11<11>, gpiod::PD0<11>, gpioc::PC11<8>],

    <serial::TxPin, UART5> for [gpiob::PB6<11>, gpiob::PB9<11>, gpiob::PB13<11>],
    <serial::RxPin, UART5> for [gpiob::PB5<11>, gpiob::PB8<11>, gpiob::PB12<11>],

    <serial::TxPin, UART7> for [gpioa::PA15<8>, gpiob::PB4<8>],
    <serial::RxPin, UART7> for [gpioa::PA8<8>, gpiob::PB3<8>],

    <serial::TxPin, UART8> for [gpiof::PF9<8>],
    <serial::RxPin, UART8> for [gpiof::PF8<8>],

    <serial::TxPin, UART9> for [gpiod::PD15<11>, gpiog::PG1<11>],
    <serial::RxPin, UART9> for [gpiod::PD14<11>, gpiog::PG0<11>],

    <serial::TxPin, UART10> for [gpioe::PE3<11>, gpiog::PG12<11>],
    <serial::RxPin, UART10> for [gpioe::PE2<11>, gpiog::PG11<11>]
}
