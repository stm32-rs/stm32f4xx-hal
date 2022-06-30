use super::{marker, Alternate, NoPin, OpenDrain, Pin, PinMode, PushPull};
use crate::{gpio, i2c, i2s, pac, serial, spi};

pub struct Const<const A: u8>;

pub trait SetAlternate<const A: u8, Otype> {
    fn set_alt_mode(&mut self);
    fn restore_mode(&mut self);
}
impl<Otype> SetAlternate<0, Otype> for NoPin {
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}
impl<const P: char, const N: u8, MODE: PinMode + marker::NotAlt, const A: u8>
    SetAlternate<A, PushPull> for Pin<P, N, MODE>
{
    fn set_alt_mode(&mut self) {
        self.mode::<Alternate<A, PushPull>>();
    }

    fn restore_mode(&mut self) {
        self.mode::<MODE>();
    }
}

impl<const P: char, const N: u8, MODE: PinMode + marker::NotAlt, const A: u8>
    SetAlternate<A, OpenDrain> for Pin<P, N, MODE>
{
    fn set_alt_mode(&mut self) {
        self.mode::<Alternate<A, OpenDrain>>();
    }

    fn restore_mode(&mut self) {
        self.mode::<MODE>();
    }
}

impl<const P: char, const N: u8, const A: u8> SetAlternate<A, PushPull>
    for Pin<P, N, Alternate<A, PushPull>>
{
    fn set_alt_mode(&mut self) {}
    fn restore_mode(&mut self) {}
}

impl<const P: char, const N: u8, const A: u8> SetAlternate<A, OpenDrain>
    for Pin<P, N, Alternate<A, OpenDrain>>
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
    ( $(<$Pin:ty, $I2C:ident> for [$($PX:ident<$A:literal>),*]),*) => {
        $(
            $(
                impl<MODE> PinA<$Pin, pac::$I2C> for gpio::$PX<MODE> {
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
        <can::Tx, CAN1> for [PA12<9>, PD1<9>],
        <can::Rx, CAN1> for [PA11<9>, PD0<9>],

        <can::Tx, CAN2> for [PB13<9>, PB6<9>],
        <can::Rx, CAN2> for [PB12<9>, PB5<9>]
    }

    #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
    pin! {
        <can::Tx, CAN1> for [PB9<8>],
        <can::Rx, CAN1> for [PB8<8>]
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
        <can::Tx, CAN1> for [PB9<9>],
        <can::Rx, CAN1> for [PB8<9>]
    }

    #[cfg(any(feature = "stm32f412", feature = "stm32f413", feature = "stm32f423"))]
    pin! {
        <can::Tx, CAN1> for [PG1<9>],
        <can::Rx, CAN1> for [PG0<9>],

        <can::Tx, CAN2> for [PG12<9>],
        <can::Rx, CAN2> for [PG11<9>]
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
        <can::Tx, CAN1> for [PH13<9>],
        <can::Rx, CAN1> for [PI9<9>]
    }

    #[cfg(feature = "can3")]
    pin! {
        <can::Tx, CAN3> for [PA15<11>, PB4<11>],
        <can::Rx, CAN3> for [PA8<11>, PB3<11>]
    }
}

// I2C pins

pin! {
    <i2c::Scl, I2C1> for [PB6<4>, PB8<4>],
    <i2c::Sda, I2C1> for [PB7<4>, PB9<4>]
}

#[cfg(any(feature = "stm32f446"))]
pin! { <i2c::Sda, I2C2> for [PB3<4>] }

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C2> for [PB3<9>] }

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C2> for [PB9<9>] }

pin! { <i2c::Scl, I2C2> for [PB10<4>] }

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
pin! { <i2c::Sda, I2C2> for [PB11<4>] }

#[cfg(any(feature = "stm32f446"))]
pin! { <i2c::Sda, I2C2> for [PC12<4>] }

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
    <i2c::Scl, I2C2> for [PF1<4>],
    <i2c::Sda, I2C2> for [PF0<4>]
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
    <i2c::Scl, I2C2> for [PH4<4>],
    <i2c::Sda, I2C2> for [PH5<4>]
}

#[cfg(feature = "i2c3")]
pin! {
    <i2c::Scl, I2C3> for [PA8<4>],
    <i2c::Sda, I2C3> for [PC9<4>]
}

#[cfg(feature = "stm32f446")]
pin! { <i2c::Sda, I2C3> for [PB4<4>] }

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C3> for [PB4<9>] }

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! { <i2c::Sda, I2C3> for [PB8<9>] }

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
    <i2c::Scl, I2C3> for [PH7<4>],
    <i2c::Sda, I2C3> for [PH8<4>]
}

#[cfg(feature = "fmpi2c1")]
pin! {
    <i2c::Scl, FMPI2C1> for [PC6<4>],
    <i2c::Sda, FMPI2C1> for [PC7<4>],
    <i2c::Sda, FMPI2C1> for [PB3<4>],
    <i2c::Scl, FMPI2C1> for [PB10<9>],
    <i2c::Sda, FMPI2C1> for [PB14<4>],
    <i2c::Scl, FMPI2C1> for [PB15<4>],
    <i2c::Scl, FMPI2C1> for [PD12<4>],
    <i2c::Scl, FMPI2C1> for [PB13<4>],
    <i2c::Scl, FMPI2C1> for [PD14<4>],
    <i2c::Scl, FMPI2C1> for [PD15<4>],
    <i2c::Scl, FMPI2C1> for [PF14<4>],
    <i2c::Scl, FMPI2C1> for [PF15<4>]
}

// SPI pins

pin! {
    <spi::Sck,  SPI1> for [PA5<5>, PB3<5>],
    <spi::Miso, SPI1> for [PA6<5>, PB4<5>],
    <spi::Mosi, SPI1> for [PA7<5>, PB5<5>],

    <spi::Sck,  SPI2> for [PB10<5>, PB13<5>],
    <spi::Miso, SPI2> for [PB14<5>, PC2<5>],
    <spi::Mosi, SPI2> for [PB15<5>, PC3<5>],
    <spi::Nss,  SPI2> for [PB9<5>, PB12<5>]
}

#[cfg(feature = "spi3")]
pin! {
    <spi::Sck,  SPI3> for [PB3<6>, PC10<6>],
    <spi::Miso, SPI3> for [PB4<6>, PC11<6>],
    <spi::Mosi, SPI3> for [PB5<6>, PC12<6>],
    <spi::Nss,  SPI3> for [PA4<6>, PA15<6>]
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
    <spi::Sck,  SPI2> for [PD3<5>],

    <spi::Mosi, SPI3> for [PD6<5>],

    <spi::Sck,  SPI4> for [PE2<5>, PE12<5>],
    <spi::Miso, SPI4> for [PE5<5>, PE13<5>],
    <spi::Mosi, SPI4> for [PE6<5>, PE14<5>]
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
    <spi::Sck,  SPI2> for [PI1<5>],
    <spi::Miso, SPI2> for [PI2<5>],
    <spi::Mosi, SPI2> for [PI3<5>],
    <spi::Nss,  SPI2> for [PI0<5>]
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
    <spi::Sck, SPI2> for [PC7<5>],
    <spi::Nss,  SPI1> for [PA4<5>, PA15<5>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <spi::Sck,  SPI5> for [PB0<6>],
    <spi::Miso, SPI5> for [PA12<6>],
    <spi::Mosi, SPI5> for [PA10<6>, PB8<6>],
    <spi::Nss,  SPI5> for [PB1<6>]
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {

    <spi::Sck,  SPI3> for [PB12<7>],

    <spi::Sck,  SPI4> for [PB13<6>],
    <spi::Miso, SPI4> for [PA11<6>],
    <spi::Mosi, SPI4> for [PA1<5>],
    <spi::Nss,  SPI4> for [PB12<6>, PE4<5>, PE11<5>],

    <spi::Sck,  SPI5> for [PE2<6>, PE12<6>],
    <spi::Miso, SPI5> for [PE5<6>, PE13<6>],
    <spi::Mosi, SPI5> for [PE6<6>, PE14<6>],
    <spi::Nss,  SPI5> for [PE4<6>, PE11<6>]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin! {
    <spi::Sck,  SPI2> for [PA9<5>],
    <spi::Miso, SPI2> for [PA12<5>],
    <spi::Mosi, SPI2> for [PA10<5>],
    <spi::Nss,  SPI2> for [PA11<5>]
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
    <spi::Sck,  SPI5> for [PF7<5>, PH6<5>],
    <spi::Miso, SPI5> for [PF8<5>, PH7<5>],
    <spi::Mosi, SPI5> for [PF9<5>, PF11<5>],

    <spi::Sck,  SPI6> for [PG13<5>],
    <spi::Miso, SPI6> for [PG12<5>],
    <spi::Mosi, SPI6> for [PG14<5>]
}

#[cfg(feature = "stm32f446")]
pin! {
    <spi::Sck,  SPI2> for [PA9<5>],
    <spi::Mosi, SPI2> for [PC1<7>],
    <spi::Nss,  SPI2> for [PB4<7>, PD1<7>],

    <spi::Mosi, SPI3> for [PB0<7>, PB2<7>, PC1<5>, PD0<6>],

    <spi::Sck,  SPI4> for [PG11<6>],
    <spi::Miso, SPI4> for [PG12<6>, PD0<5>],
    <spi::Mosi, SPI4> for [PG13<6>]
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
pin! {
    <spi::Sck,  SPI2> for [PA9<5>],
    <spi::Mosi, SPI2> for [PC1<5>]
}

// SPI pins for I2S mode
pin! {
    <i2s::Ck,  SPI1> for [PA5<5>, PB3<5>],
    <i2s::Sd, SPI1> for [PA7<5>, PB5<5>],

    <i2s::Ck,  SPI2> for [PB10<5>, PB13<5>],
    <i2s::Sd, SPI2> for [PB15<5>, PC3<5>],
    <i2s::Ws,  SPI2> for [PB9<5>, PB12<5>],
    <i2s::Mck,  SPI2> for [PC6<5>]
}

#[cfg(feature = "spi3")]
pin! {
    <i2s::Ck,  SPI3> for [PB3<6>, PC10<6>],
    <i2s::Sd, SPI3> for [PB5<6>, PC12<6>],
    <i2s::Ws,  SPI3> for [PA4<6>, PA15<6>],
    <i2s::Mck,  SPI3> for [PC7<6>]
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
    <i2s::Ck,  SPI2> for [PD3<5>],

    <i2s::Sd, SPI3> for [PD6<5>],

    <i2s::Ck,  SPI4> for [PE2<5>, PE12<5>],
    <i2s::Sd, SPI4> for [PE6<5>, PE14<5>]
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
    <i2s::Ck,  SPI2> for [PI1<5>],
    <i2s::Sd, SPI2> for [PI3<5>],
    <i2s::Ws,  SPI2> for [PI0<5>]
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
    <i2s::Ck, SPI2> for [PC7<5>],
    <i2s::Ws,  SPI1> for [PA4<5>, PA15<5>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <i2s::Ck,  SPI5> for [PB0<6>],
    <i2s::Sd, SPI5> for [PA10<6>, PB8<6>],
    <i2s::Ws,  SPI5> for [PB1<6>]
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <i2s::Mck,  SPI2> for [PA3<5>, PA6<6>],

    <i2s::Ck,  SPI3> for [PB12<7>],
    <i2s::Mck,  SPI3> for [PB10<6>],

    <i2s::Ck,  SPI4> for [PB13<6>],
    <i2s::Sd, SPI4> for [PA1<5>],
    <i2s::Ws,  SPI4> for [PB12<6>, PE4<5>, PE11<5>],

    <i2s::Ck,  SPI5> for [PE2<6>, PE12<6>],
    <i2s::Sd, SPI5> for [PE6<6>, PE14<6>],
    <i2s::Ws,  SPI5> for [PE4<6>, PE11<6>]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin! {
    <i2s::Ck,  SPI2> for [PA9<5>],
    <i2s::Sd, SPI2> for [PA10<5>],
    <i2s::Ws,  SPI2> for [PA11<5>]
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
    <i2s::Ck,  SPI5> for [PF7<5>, PH6<5>],
    <i2s::Sd, SPI5> for [PF9<5>, PF11<5>],

    <i2s::Ck,  SPI6> for [PG13<5>],
    <i2s::Sd, SPI6> for [PG14<5>]
}

#[cfg(feature = "stm32f446")]
pin! {
    <i2s::Ck,  SPI2> for [PA9<5>],
    <i2s::Sd, SPI2> for [PC1<7>],
    <i2s::Ws,  SPI2> for [PB4<7>, PD1<7>],

    <i2s::Sd, SPI3> for [PB0<7>, PB2<7>, PC1<5>, PD0<6>],

    <i2s::Ck,  SPI4> for [PG11<6>],
    <i2s::Sd, SPI4> for [PG13<6>]
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
pin! {
    <i2s::Ck,  SPI2> for [PA9<5>],
    <i2s::Sd, SPI2> for [PC1<5>]
}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
pin! { <i2s::Mck,  SPI1> for [PC4<5>] }

#[cfg(feature = "stm32f410")]
pin! { <i2s::Mck,  SPI1> for [PC7<6>, PB10<6>] }

// Serial pins

pin! {
    <serial::TxPin, USART1> for [PA9<7>, PB6<7>],
    <serial::RxPin, USART1> for [PA10<7>, PB7<7>],

    <serial::TxPin, USART2> for [PA2<7>],
    <serial::RxPin, USART2> for [PA3<7>],

    <serial::TxPin, USART6> for [PC6<8>],
    <serial::RxPin, USART6> for [PC7<8>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pin! {
    <serial::TxPin, USART1> for [PA15<7>],
    <serial::RxPin, USART1> for [PB3<7>]
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
    <serial::TxPin, USART2> for [PD5<7>],
    <serial::RxPin, USART2> for [PD6<7>]
}

#[cfg(feature = "usart3")]
pin! {
    <serial::TxPin, USART3> for [PB10<7>],
    <serial::RxPin, USART3> for [PB11<7>]
}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
pin! {
    <serial::RxPin, USART3> for [PC5<7>]
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
    <serial::TxPin, USART3> for [PC10<7>],
    <serial::RxPin, USART3> for [PC11<7>]
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
    <serial::TxPin, USART3> for [PD8<7>],
    <serial::RxPin, USART3> for [PD9<7>]
}

#[cfg(feature = "uart4")]
pin! {
    <serial::TxPin, UART4> for [PA0<8>],
    <serial::RxPin, UART4> for [PA1<8>]
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
    <serial::TxPin, UART4> for [PC10<8>],
    <serial::RxPin, UART4> for [PC11<8>]
}
#[cfg(feature = "uart5")]
pin! {
    <serial::TxPin, UART5> for [PC12<8>],
    <serial::RxPin, UART5> for [PD2<8>]
}

#[cfg(any(feature = "stm32f446"))]
pin! {
    <serial::TxPin, UART5> for [PE8<8>],
    <serial::RxPin, UART5> for [PE7<8>]
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
    <serial::TxPin, USART6> for [PA11<8>],
    <serial::RxPin, USART6> for [PA12<8>]
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
    <serial::TxPin, USART6> for [PG14<8>],
    <serial::RxPin, USART6> for [PG9<8>]
}

#[cfg(all(feature = "uart7", feature = "gpioe"))]
pin! {
    <serial::TxPin, UART7> for [PE8<8>],
    <serial::RxPin, UART7> for [PE7<8>]
}

#[cfg(all(feature = "uart7", feature = "gpiof"))]
pin! {
    <serial::TxPin, UART7> for [PF7<8>],
    <serial::RxPin, UART7> for [PF6<8>]
}

#[cfg(all(feature = "uart8", feature = "gpioe"))]
pin! {
    <serial::TxPin, UART8> for [PE1<8>],
    <serial::RxPin, UART8> for [PE0<8>]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pin! {
    <serial::TxPin, UART4> for [PA12<11>, PD1<11>, PD10<8>],
    <serial::RxPin, UART4> for [PA11<11>, PD0<11>, PC11<8>],

    <serial::TxPin, UART5> for [PB6<11>, PB9<11>, PB13<11>],
    <serial::RxPin, UART5> for [PB5<11>, PB8<11>, PB12<11>],

    <serial::TxPin, UART7> for [PA15<8>, PB4<8>],
    <serial::RxPin, UART7> for [PA8<8>, PB3<8>],

    <serial::TxPin, UART8> for [PF9<8>],
    <serial::RxPin, UART8> for [PF8<8>],

    <serial::TxPin, UART9> for [PD15<11>, PG1<11>],
    <serial::RxPin, UART9> for [PD14<11>, PG0<11>],

    <serial::TxPin, UART10> for [PE3<11>, PG12<11>],
    <serial::RxPin, UART10> for [PE2<11>, PG11<11>]
}
