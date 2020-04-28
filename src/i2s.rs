use core::ops::Deref;
use core::ptr;

// use embedded_hal::spi;
// pub use embedded_hal::spi::{Mode, Phase, Polarity};
// use nb;

#[cfg(any(
    feature = "stm32f401",
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
use crate::stm32::{spi1, RCC, SPI1, SPI2};

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
use crate::stm32::SPI3;

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
use crate::stm32::SPI4;

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI5;

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI6;

#[cfg(any(
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioa::PA9;
#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::{PA1, PA11};
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::{PA10, PA12};
#[cfg(any(
    feature = "stm32f401",
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
use crate::gpio::gpioa::{PA15, PA4, PA5, PA6, PA7};
// NOTE: Added PA4 and PA15 here.

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpiob::PB0;
#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpiob::PB12;
#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpiob::PB2;
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpiob::PB8;
#[cfg(any(
    feature = "stm32f401",
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
use crate::gpio::gpiob::{PB10, PB13, PB14, PB15, PB3, PB4, PB5};

#[cfg(any(feature = "stm32f446", feature = "stm32f469", feature = "stm32f479"))]
use crate::gpio::gpioc::PC1;
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpioc::PC7;
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
use crate::gpio::gpioc::{PC10, PC11, PC12};
#[cfg(any(
    feature = "stm32f401",
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
use crate::gpio::gpioc::{PC2, PC3};

#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpiod::PD0;
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
use crate::gpio::gpiod::{PD3, PD6};

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
use crate::gpio::gpioe::{PE12, PE13, PE14, PE2, PE5, PE6};

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiof::{PF11, PF7, PF8, PF9};

#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpiog::PG11;
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiog::PG14;
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiog::{PG12, PG13};

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioh::{PH6, PH7};

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
use crate::gpio::gpioi::{PI1, PI2, PI3};

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::AF7;
#[cfg(any(
    feature = "stm32f401",
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
use crate::gpio::{Alternate, AF5, AF6, AF7};

use crate::rcc::Clocks;
use crate::time::Hertz;

/// I2S error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)]
    _Extensible,
}

pub trait Pins<I2S> {}
pub trait PinCk<I2S> {}
pub trait PinWs<I2S> {}
pub trait PinSd<I2S> {}
pub trait PinSdExt<I2S> {}

impl<I2S, CK, WS, SD, SDEXT> Pins<I2S> for (CK, WS, SD, SDEXT)
where
    CK: PinCk<I2S>,
    WS: PinWs<I2S>,
    SD: PinSd<I2S>,
    SDEXT: PinSdExt<I2S>,
{
}

/// A filler type for when the CK pin is unnecessary
pub struct NoCk;
/// A filler type for when the Ws pin is unnecessary
pub struct NoWs;
/// A filler type for when the Sd pin is unnecessary
pub struct NoSd;
/// A filler type for when the SdExt pin is unnecessary
pub struct NoSdExt;

// NOTE: Manual pins for I2S3 during development.
// TODO: Should be created with macro.
impl PinCk<SPI3> for NoCk {}
impl PinCk<SPI3> for PB3<Alternate<AF6>> {}
impl PinCk<SPI3> for PC10<Alternate<AF6>> {}

impl PinWs<SPI3> for NoWs {}
impl PinWs<SPI3> for PA4<Alternate<AF6>> {}
impl PinWs<SPI3> for PA15<Alternate<AF6>> {}

impl PinSd<SPI3> for NoSd {}
impl PinSd<SPI3> for PB5<Alternate<AF6>> {}
impl PinSd<SPI3> for PC12<Alternate<AF6>> {}

impl PinSdExt<SPI3> for NoSdExt {}
impl PinSdExt<SPI3> for PB4<Alternate<AF7>> {}
impl PinSdExt<SPI3> for PC11<Alternate<AF5>> {}

// macro_rules! pins {
//     ($($SPIX:ty: CK: [$($CK:ty),*] WS: [$($WS:ty),*] SD: [$($SD:ty),*] SDEXT: [$($SDEXT:ty),*])+) => {
//         $(
//             $(
//                 impl PinCk<$SPIX> for $CK {}
//             )*
//             $(
//                 impl PinWs<$SPIX> for $WS {}
//             )*
//             $(
//                 impl PinSd<$SPIX> for $SD {}
//             )*
//             $(
//                 impl PinSdExt<$SPIX> for $SDEXT {}
//             )*
//         )+
//     }
// }

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f405",
//     feature = "stm32f407",
//     feature = "stm32f410",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f415",
//     feature = "stm32f417",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// pins! {
//     SPI1:
//         CK: [
//             NoCk,
//             PA5<Alternate<AF5>>,
//             PB3<Alternate<AF5>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PA7<Alternate<AF5>>,
//             PB5<Alternate<AF5>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PA6<Alternate<AF5>>,
//             PB4<Alternate<AF5>>
//         ]

//     SPI2:
//         CK: [
//             NoCk,
//             PB10<Alternate<AF5>>,
//             PB13<Alternate<AF5>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PB15<Alternate<AF5>>,
//             PC3<Alternate<AF5>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PB14<Alternate<AF5>>,
//             PC2<Alternate<AF5>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f405",
//     feature = "stm32f407",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f415",
//     feature = "stm32f417",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// pins! {
//     SPI3:
//         CK: [
//             NoCk,
//             PB3<Alternate<AF6>>,
//             PC10<Alternate<AF6>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PB5<Alternate<AF6>>,
//             PC12<Alternate<AF6>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PB4<Alternate<AF6>>,
//             PC11<Alternate<AF6>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// pins! {
//     SPI2:
//         CK: [PD3<Alternate<AF5>>]
//         WS: [] // TODO: Fill in.
//         SD: []
//         SDEXT: []
//     SPI3:
//         CK: []
//         WS: [] // TODO: Fill in.
//         SD: []
//         SDEXT: [PD6<Alternate<AF5>>]
//     SPI4:
//         CK: [
//             NoCk,
//             PE2<Alternate<AF5>>,
//             PE12<Alternate<AF5>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PE6<Alternate<AF5>>,
//             PE14<Alternate<AF5>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PE5<Alternate<AF5>>,
//             PE13<Alternate<AF5>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f405",
//     feature = "stm32f407",
//     feature = "stm32f415",
//     feature = "stm32f417",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// pins! {
//     SPI2:
//         CK: [PI1<Alternate<AF5>>]
//         WS: [] // TODO: Fill in.
//         SD: [PI3<Alternate<AF5>>]
//         SDEXT: [PI2<Alternate<AF5>>]
// }

// #[cfg(any(
//     feature = "stm32f410",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f423",
//     feature = "stm32f446"
// ))]
// pins! {
//     SPI2:
//         CK: [PC7<Alternate<AF5>>]
//         WS: [] // TODO: Fill in.
//         SD: []
//         SDEXT: []
// }

// #[cfg(any(
//     feature = "stm32f410",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f423"
// ))]
// pins! {
//     SPI5:
//         CK: [
//             NoCk,
//             PB0<Alternate<AF6>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PA10<Alternate<AF6>>,
//             PB8<Alternate<AF6>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PA12<Alternate<AF6>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f423"
// ))]
// pins! {
//     SPI3:
//         CK: [PB12<Alternate<AF7>>]
//         WS: [] // TODO: Fill in.
//         SD: []
//         SDEXT: []
//     SPI4:
//         CK: [PB13<Alternate<AF6>>]
//         WS: [] // TODO: Fill in.
//         SD: [PA1<Alternate<AF5>>]
//         SDEXT: [PA11<Alternate<AF6>>]
//     SPI5:
//         CK: [
//             PE2<Alternate<AF6>>,
//             PE12<Alternate<AF6>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             PE6<Alternate<AF6>>,
//             PE14<Alternate<AF6>>
//         ]
//         SDEXT: [
//             PE5<Alternate<AF6>>,
//             PE13<Alternate<AF6>>
//         ]
// }

// #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
// pins! {
//     SPI2:
//         CK: [PA9<Alternate<AF5>>]
//         WS: [] // TODO: Fill in.
//         SD: [PA10<Alternate<AF5>>]
//         SDEXT: [PA12<Alternate<AF5>>]
// }

// #[cfg(any(
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// pins! {
//     SPI5:
//         CK: [
//             NoCk,
//             PF7<Alternate<AF5>>,
//             PH6<Alternate<AF5>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PF9<Alternate<AF5>>,
//             PF11<Alternate<AF5>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PF8<Alternate<AF5>>,
//             PH7<Alternate<AF5>>
//         ]

//     SPI6:
//         CK: [
//             NoCk,
//             PG13<Alternate<AF5>>
//         ]
//         WS: [] // TODO: Fill in.
//         SD: [
//             NoSd,
//             PG14<Alternate<AF5>>
//         ]
//         SDEXT: [
//             NoSdExt,
//             PG12<Alternate<AF5>>
//         ]
// }

// #[cfg(any(feature = "stm32f446"))]
// pins! {
//     SPI2:
//         CK: [PA9<Alternate<AF5>>]
//         WS: [] // TODO: Fill in.
//         SD: [PC1<Alternate<AF7>>]
//         SDEXT: []

//     SPI3:
//         CK: []
//         WS: [] // TODO: Fill in.
//         SD: [
//             PB0<Alternate<AF7>>,
//             PB2<Alternate<AF7>>,
//             PD0<Alternate<AF6>>
//         ]
//         SDEXT: []

//     SPI4:
//         CK: [PG11<Alternate<AF6>>]
//         WS: [] // TODO: Fill in.
//         SD: [PG13<Alternate<AF6>>]
//         SDEXT: [
//             PG12<Alternate<AF6>>,
//             PD0<Alternate<AF5>>
//         ]
// }

// #[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
// pins! {
//     SPI2:
//         CK: [PA9<Alternate<AF5>>]
//         WS: [] // TODO: Fill in.
//         SD: [PC1<Alternate<AF5>>]
//         SDEXT: []
// }

// /// Interrupt events
// pub enum Event {
//     /// New data has been received
//     Rxne,
//     /// Data can be sent
//     Txe,
//     /// An error occurred
//     Error,
// }

#[derive(Debug)]
pub struct I2s<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f405",
//     feature = "stm32f407",
//     feature = "stm32f410",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f415",
//     feature = "stm32f417",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// impl<PINS> Spi<SPI1, PINS> {
//     pub fn spi1(spi: SPI1, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
//     where
//         PINS: Pins<SPI1>,
//     {
//         // NOTE(unsafe) This executes only during initialisation
//         let rcc = unsafe { &(*RCC::ptr()) };

//         // Enable clock for SPI
//         rcc.apb2enr.modify(|_, w| w.spi1en().set_bit());

//         Spi { spi, pins }.init(mode, freq, clocks.pclk2())
//     }
// }

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f405",
//     feature = "stm32f407",
//     feature = "stm32f410",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f415",
//     feature = "stm32f417",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// impl<PINS> Spi<SPI2, PINS> {
//     pub fn spi2(spi: SPI2, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
//     where
//         PINS: Pins<SPI2>,
//     {
//         // NOTE(unsafe) This executes only during initialisation
//         let rcc = unsafe { &(*RCC::ptr()) };

//         // Enable clock for SPI
//         rcc.apb1enr.modify(|_, w| w.spi2en().set_bit());

//         Spi { spi, pins }.init(mode, freq, clocks.pclk1())
//     }
// }

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f405",
//     feature = "stm32f407",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f415",
//     feature = "stm32f417",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// impl<PINS> Spi<SPI3, PINS> {
//     pub fn spi3(spi: SPI3, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
//     where
//         PINS: Pins<SPI3>,
//     {
//         // NOTE(unsafe) This executes only during initialisation
//         let rcc = unsafe { &(*RCC::ptr()) };

//         // Enable clock for SPI
//         rcc.apb1enr.modify(|_, w| w.spi3en().set_bit());

//         Spi { spi, pins }.init(mode, freq, clocks.pclk1())
//     }
// }

// #[cfg(any(
//     feature = "stm32f401",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f446",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// impl<PINS> Spi<SPI4, PINS> {
//     pub fn spi4(spi: SPI4, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
//     where
//         PINS: Pins<SPI4>,
//     {
//         // NOTE(unsafe) This executes only during initialisation
//         let rcc = unsafe { &(*RCC::ptr()) };

//         // Enable clock for SPI
//         rcc.apb2enr.modify(|_, w| w.spi4en().set_bit());

//         Spi { spi, pins }.init(mode, freq, clocks.pclk2())
//     }
// }

// #[cfg(any(
//     feature = "stm32f410",
//     feature = "stm32f411",
//     feature = "stm32f412",
//     feature = "stm32f413",
//     feature = "stm32f423",
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// impl<PINS> Spi<SPI5, PINS> {
//     pub fn spi5(spi: SPI5, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
//     where
//         PINS: Pins<SPI5>,
//     {
//         // NOTE(unsafe) This executes only during initialisation
//         let rcc = unsafe { &(*RCC::ptr()) };

//         // Enable clock for SPI
//         rcc.apb2enr.modify(|_, w| w.spi5en().set_bit());

//         Spi { spi, pins }.init(mode, freq, clocks.pclk2())
//     }
// }

// #[cfg(any(
//     feature = "stm32f427",
//     feature = "stm32f429",
//     feature = "stm32f437",
//     feature = "stm32f439",
//     feature = "stm32f469",
//     feature = "stm32f479"
// ))]
// impl<PINS> Spi<SPI6, PINS> {
//     pub fn spi6(spi: SPI6, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
//     where
//         PINS: Pins<SPI6>,
//     {
//         // NOTE(unsafe) This executes only during initialisation
//         let rcc = unsafe { &(*RCC::ptr()) };

//         // Enable clock for SPI
//         rcc.apb2enr.modify(|_, w| w.spi6en().set_bit());

//         Spi { spi, pins }.init(mode, freq, clocks.pclk2())
//     }
// }

impl<PINS> I2s<SPI3, PINS> {
    pub fn i2s3(spi: SPI3, pins: PINS, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI3>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb1enr.modify(|_, w| w.spi3en().set_bit());

        // TODO: Use Real clock value from  I2S PLL.
        I2s { spi, pins }.init(freq, freq)
    }
}

impl<SPI, PINS> I2s<SPI, PINS>
where
    SPI: Deref<Target = spi1::RegisterBlock>,
{
    pub fn init(self, freq: Hertz, clock: Hertz) -> Self {
        // disable SS output
        self.spi.cr2.write(|w| w.ssoe().clear_bit());

        // TODO: Calculate baud rate.
        // let br = match clock.0 / freq.0 {
        //     0 => unreachable!(),
        //     1..=2 => 0b000,
        //     3..=5 => 0b001,
        //     6..=11 => 0b010,
        //     12..=23 => 0b011,
        //     24..=47 => 0b100,
        //     48..=95 => 0b101,
        //     96..=191 => 0b110,
        //     _ => 0b111,
        // };
        let br: u8 = 0;

        // Configure clock polarity.
        // self.spi.i2scfgr.write(|w| w.ckpol().idle_high());

        // Configure the I2S precsaler and enable MCKL output.
        // NOTE: Hardcoded for 48KHz audio sampling rate with PLLI2S at 86MHz.
        // I2S uses DIV=3, ODD=true to achive a 12.285714 MHz MCKL.
        // FS = I2SxCLK / [(16*2)*(((2*I2SDIV)+ODD)*8)] when the channel frame is 16-bit wide.
        // FS = 86MHz (from PLLI2S above) / [(16*2)*(((2*3)+1)*8)] = 48KHz
        // NOTE: Unsafe because the division can be set incorrectly.
        self.spi
            .i2spr
            .write(|w| unsafe { w.i2sdiv().bits(3).odd().odd().mckoe().enabled() });

        // Configure I2S.
        // TODO: Configurable I2S standard and data length from user input.
        self.spi.i2scfgr.write(|w| {
            w
                // SPI/I2S Mode.
                .i2smod()
                .i2smode()
                // I2S standard.
                .i2sstd()
                .philips()
                // Data and channel length.
                .datlen()
                .sixteen_bit()
                .chlen()
                .sixteen_bit()
                // Clock steady state polarity.
                .ckpol()
                .idle_high()
                // Master TX mode and enable.
                .i2scfg()
                .master_tx()
                .i2se()
                .enabled()
        });

        self
    }

    // TODO: Configure interrupts for I2S.
    // /// Enable interrupts for the given `event`:
    // ///  - Received data ready to be read (RXNE)
    // ///  - Transmit data register empty (TXE)
    // ///  - Transfer error
    // pub fn listen(&mut self, event: Event) {
    //     match event {
    //         Event::Rxne => self.spi.cr2.modify(|_, w| w.rxneie().set_bit()),
    //         Event::Txe => self.spi.cr2.modify(|_, w| w.txeie().set_bit()),
    //         Event::Error => self.spi.cr2.modify(|_, w| w.errie().set_bit()),
    //     }
    // }

    // /// Disable interrupts for the given `event`:
    // ///  - Received data ready to be read (RXNE)
    // ///  - Transmit data register empty (TXE)
    // ///  - Transfer error
    // pub fn unlisten(&mut self, event: Event) {
    //     match event {
    //         Event::Rxne => self.spi.cr2.modify(|_, w| w.rxneie().clear_bit()),
    //         Event::Txe => self.spi.cr2.modify(|_, w| w.txeie().clear_bit()),
    //         Event::Error => self.spi.cr2.modify(|_, w| w.errie().clear_bit()),
    //     }
    // }

    /// Return `true` if the TXE flag is set, i.e. new data to transmit
    /// can be written to the SPI.
    pub fn is_txe(&self) -> bool {
        self.spi.sr.read().txe().bit_is_set()
    }

    /// Return the value of the CHSIDE flag, i.e. which channel to transmit next.
    pub fn ch_side(&self) -> bool {
        self.spi.sr.read().chside().bit_is_set()
    }

    // /// Return `true` if the RXNE flag is set, i.e. new data has been received
    // /// and can be read from the SPI.
    // pub fn is_rxne(&self) -> bool {
    //     self.spi.sr.read().rxne().bit_is_set()
    // }

    // /// Return `true` if the MODF flag is set, i.e. the SPI has experienced a
    // /// Master Mode Fault. (see chapter 28.3.10 of the STM32F4 Reference Manual)
    // pub fn is_modf(&self) -> bool {
    //     self.spi.sr.read().modf().bit_is_set()
    // }

    // /// Return `true` if the OVR flag is set, i.e. new data has been received
    // /// while the receive data register was already filled.
    // pub fn is_ovr(&self) -> bool {
    //     self.spi.sr.read().ovr().bit_is_set()
    // }

    pub fn free(self) -> (SPI, PINS) {
        (self.spi, self.pins)
    }

    pub fn send(&mut self, data: u16) -> Result<(), Error> {
        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u16, data) }
        while self.spi.sr.read().txe().bit_is_clear() {}
        Ok(())
    }
}

// impl<SPI, PINS> spi::FullDuplex<u8> for Spi<SPI, PINS>
// where
//     SPI: Deref<Target = spi1::RegisterBlock>,
// {
//     type Error = Error;

//     fn read(&mut self) -> nb::Result<u8, Error> {
//         let sr = self.spi.sr.read();

//         Err(if sr.ovr().bit_is_set() {
//             nb::Error::Other(Error::Overrun)
//         } else if sr.modf().bit_is_set() {
//             nb::Error::Other(Error::ModeFault)
//         } else if sr.crcerr().bit_is_set() {
//             nb::Error::Other(Error::Crc)
//         } else if sr.rxne().bit_is_set() {
//             // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
//             // reading a half-word)
//             return Ok(unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const u8) });
//         } else {
//             nb::Error::WouldBlock
//         })
//     }

//     fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
//         let sr = self.spi.sr.read();

//         Err(if sr.ovr().bit_is_set() {
//             nb::Error::Other(Error::Overrun)
//         } else if sr.modf().bit_is_set() {
//             nb::Error::Other(Error::ModeFault)
//         } else if sr.crcerr().bit_is_set() {
//             nb::Error::Other(Error::Crc)
//         } else if sr.txe().bit_is_set() {
//             // NOTE(write_volatile) see note above
//             unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }
//             return Ok(());
//         } else {
//             nb::Error::WouldBlock
//         })
//     }
// }

// impl<SPI, PINS> embedded_hal::blocking::spi::transfer::Default<u8> for Spi<SPI, PINS> where
//     SPI: Deref<Target = spi1::RegisterBlock>
// {
// }

// impl<SPI, PINS> embedded_hal::blocking::spi::write::Default<u8> for Spi<SPI, PINS> where
//     SPI: Deref<Target = spi1::RegisterBlock>
// {
// }
