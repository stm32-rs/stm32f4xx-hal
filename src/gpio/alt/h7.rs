use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

/*#[cfg(feature = "gpio-h747")]
pub mod comp {
    use super::*;

    pin! {
        <Tim1Bkin> for [
            PE15<13>,
        ],
    }
}*/

pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PC5<13>,

            PE12<13>,
        ],
    }
}

pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            PE8<13>,

            PE13<13>,
        ],
    }
}

/*pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            PB3<10>,
        ],
    }
}*/

pub mod dcmi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<13>,

            PC6<13>,

            PH9<13>,
        ],

        <D1, PushPull> for [
            PA10<13>,

            PC7<13>,

            PH10<13>,
        ],

        <D10, PushPull> for [
            PB5<13>,

            PD6<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI3<13>,
        ],

        <D11, PushPull> for [
            PD2<13>,

            PF10<13>,

            PH15<13>,
        ],

        <D12, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PD12<13>,

            PF11<13>,

            PG6<13>,
        ],

        <D13, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PD13<13>,

            PG7<13>,

            PG15<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI0<13>,
        ],

        <D2, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PB13<13>,

            PC8<13>,

            PE0<13>,

            PG10<13>,

            PH11<13>,
        ],

        <D3, PushPull> for [
            PC9<13>,

            PE1<13>,

            PG11<13>,

            PH12<13>,
        ],

        <D4, PushPull> for [
            PC11<13>,

            PE4<13>,

            PH14<13>,
        ],

        <D5, PushPull> for [
            PB6<13>,

            PD3<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI4<13>,
        ],

        <D6, PushPull> for [
            PB8<13>,

            PE5<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI6<13>,
        ],

        <D7, PushPull> for [
            PB9<13>,

            PE6<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI7<13>,
        ],

        <D8, PushPull> for [
            PC10<13>,

            PH6<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<13>,
        ],

        <D9, PushPull> for [
            PC12<13>,

            PH7<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI2<13>,
        ],

        <Hsync, PushPull> for [
            PA4<13>,

            PH8<13>,
        ],

        <Pixclk, PushPull> for [
            PA6<13>,
        ],

        <Vsync, PushPull> for [
            PB7<13>,

            PG9<13>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI5<13>,
        ],
    }
}

pub mod debug {
    use super::*;

    pin! {
        <JtckSwclk, PushPull> for [
            PA14<0>,
        ],

        <Jtdi, PushPull> for [
            PA15<0>,
        ],

        <JtdoSwo, PushPull> for [
            PB3<0>,
        ],

        <JtmsSwdio, PushPull> for [
            PA13<0>,
        ],

        <Traceclk, PushPull> for [
            PE2<0>,
        ],

        <Traced0, PushPull> for [
            PC1<0>,

            PE3<0>,

            PG13<0>,
        ],

        <Traced1, PushPull> for [
            PC8<0>,

            PE4<0>,

            PG14<0>,
        ],

        <Traced2, PushPull> for [
            PD2<0>,

            PE5<0>,
        ],

        <Traced3, PushPull> for [
            PC12<0>,

            PE6<0>,
        ],

        <Trgio, PushPull> for [
            PC7<0>,
        ],
    }

    #[cfg(feature = "gpio-h72")]
    pin! {
        <Jtrst, PushPull> for [
            PB4<0>,
        ],
    }

    #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
    pin! {
        <Trgin, PushPull> for [
            PJ7<0>,
        ],

        <Trgout, PushPull> for [
            PJ12<0>,
        ],
    }
}

pub mod dfsdm1 {
    use super::*;

    pin! {
        <Ckin0, PushPull> for [
            PC0<3>,
        ],

        <Ckin1, PushPull> for [
            PB2<4>,

            PB13<6>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC2<3>,

            PD7<6>,
        ],

        <Ckin2, PushPull> for [
            PB15<6>,

            PC4<3>,

            PE8<3>,
        ],

        <Ckin3, PushPull> for [
            PC6<4>,

            PD8<3>,

            PE5<3>,
        ],

        <Ckin4, PushPull> for [
            PC1<4>,

            PD6<3>,

            PE11<3>,
        ],

        <Ckin5, PushPull> for [
            PB7<11>,

            PC10<3>,

            PE13<3>,
        ],

        <Ckin6, PushPull> for [
            PD0<3>,

            PF14<3>,
        ],

        <Ckin7, PushPull> for [
            PB8<3>,

            PB11<6>,
        ],

        <Ckout, PushPull> for [
            PB0<6>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC2<6>,

            PD3<3>,

            PD10<3>,

            PE9<3>,
        ],

        <Datin0, PushPull> for [
            PC1<3>,
        ],

        <Datin1, PushPull> for [
            PB1<6>,

            PB12<6>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC3<3>,

            PD6<4>,
        ],

        <Datin2, PushPull> for [
            PB14<6>,

            PC5<3>,

            PE7<3>,
        ],

        <Datin3, PushPull> for [
            PC7<4>,

            PD9<3>,

            PE4<3>,
        ],

        <Datin4, PushPull> for [
            PC0<6>,

            PD7<3>,

            PE10<3>,
        ],

        <Datin5, PushPull> for [
            PB6<11>,

            PC11<3>,

            PE12<3>,
        ],

        <Datin6, PushPull> for [
            PD1<3>,

            PF13<3>,
        ],

        <Datin7, PushPull> for [
            PB9<3>,

            PB10<6>,
        ],
    }

    use crate::pac::DFSDM1 as DFSDM;
    impl DfsdmBasic for DFSDM {
        type Ckin0 = Ckin0;
        type Ckin1 = Ckin1;
        type Ckout = Ckout;
        type Datin0 = Datin0;
        type Datin1 = Datin1;
    }
    impl DfsdmGeneral for DFSDM {
        type Ckin2 = Ckin2;
        type Ckin3 = Ckin3;
        type Datin2 = Datin2;
        type Datin3 = Datin3;
    }

    impl DfsdmAdvanced for DFSDM {
        type Ckin4 = Ckin4;
        type Ckin5 = Ckin5;
        type Ckin6 = Ckin6;
        type Ckin7 = Ckin7;
        type Datin4 = Datin4;
        type Datin5 = Datin5;
        type Datin6 = Datin6;
        type Datin7 = Datin7;
    }
}

#[cfg(feature = "gpio-h7a2")]
pub mod dfsdm2 {
    use super::*;

    pin! {
        <Ckin0, PushPull> for [
            PC10<4>,
        ],

        <Ckin1, PushPull> for [
            PA2<6>,

            PB13<4>,
        ],

        <Ckout, PushPull> for [
            PB0<4>,

            PC12<4>,

            PD10<4>,
        ],

        <Datin0, PushPull> for [
            PC11<4>,
        ],

        <Datin1, PushPull> for [
            PA7<4>,

            PB12<11>,
        ],
    }

    use crate::pac::DFSDM2 as DFSDM;
    impl DfsdmBasic for DFSDM {
        type Ckin0 = Ckin0;
        type Ckin1 = Ckin1;
        type Ckout = Ckout;
        type Datin0 = Datin0;
        type Datin1 = Datin1;
    }
}

#[cfg(feature = "gpio-h747")]
pub mod dsihost {
    use super::*;

    pin! {
        <Te, PushPull> for [
            PA15<13>,

            PB11<13>,

            PJ2<13>,
        ],
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h747"))]
pub mod eth {
    use super::*;

    pin! {
        <Col, PushPull> for [
            PA3<11>,

            PH3<11>,
        ],

        <Crs, PushPull> for [
            PA0<11>,

            PH2<11>,
        ],

        <CrsDv, PushPull> for [
            PA7<11>,
        ],

        <Mdc, PushPull> for [
            PC1<11>,
        ],

        <Mdio, PushPull> for [
            PA2<11>,
        ],

        <PpsOut, PushPull> for [
            PB5<11>,

            PG8<11>,
        ],

        <RefClk, PushPull> for [
            PA1<11>,
        ],

        <RxClk, PushPull> for [
            PA1<11>,
        ],

        <RxDv, PushPull> for [
            PA7<11>,
        ],

        <RxEr, PushPull> for [
            PB10<11>,

            #[cfg(feature = "gpio-h747")]
            PI10<11>,
        ],

        <Rxd0, PushPull> for [
            PC4<11>,
        ],

        <Rxd1, PushPull> for [
            PC5<11>,
        ],

        <Rxd2, PushPull> for [
            PB0<11>,

            PH6<11>,
        ],

        <Rxd3, PushPull> for [
            PB1<11>,

            PH7<11>,
        ],

        <TxEn, PushPull> for [
            PB11<11>,

            PG11<11>,
        ],

        <Txd0, PushPull> for [
            PB12<11>,

            PG13<11>,
        ],

        <Txd1, PushPull> for [
            PB13<11>,

            PG12<11>,

            PG14<11>,
        ],

        <Txd3, PushPull> for [
            PB8<11>,

            PE2<11>,
        ],
    }

    #[cfg(feature = "gpio-h747")]
    pin! {
        <TxClk, PushPull> for [
            PC3<11>,
        ],

        <Txd2, PushPull> for [
            PC2<11>,
        ],
    }

    #[cfg(feature = "gpio-h72")]
    pin! {
        <TxEr, PushPull> for [
            PA9<11>,

            PB2<11>,
        ],
    }
}

pub mod fdcan1 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PA11<9>,

            PB8<9>,

            PD0<9>,

            PH14<9>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI9<9>,
        ],

        <Tx, PushPull> for [
            PA12<9>,

            PB9<9>,

            PD1<9>,

            PH13<9>,
        ],
    }
}

pub mod fdcan2 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PB5<9>,

            PB12<9>,
        ],

        <Tx, PushPull> for [
            PB6<9>,

            PB13<9>,
        ],
    }
}

#[cfg(feature = "gpio-h72")]
pub mod fdcan3 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PD12<5>,

            PF6<2>,

            PG10<2>,
        ],

        <Tx, PushPull> for [
            PD13<5>,

            PF7<2>,

            PG9<2>,
        ],
    }
}

pub mod fmc {
    use super::*;

    pin! {
        <A0, PushPull> for [
            PF0<12>,
        ],

        <A1, PushPull> for [
            PF1<12>,
        ],

        <A10, PushPull> for [
            PG0<12>,
        ],

        <A11, PushPull> for [
            PG1<12>,
        ],

        <A12, PushPull> for [
            PG2<12>,
        ],

        <A13, PushPull> for [
            PG3<12>,
        ],

        <A14, PushPull> for [
            PG4<12>,
        ],

        <A15, PushPull> for [
            PG5<12>,
        ],

        <A16, PushPull> for [
            PD11<12>,
        ],

        <A17, PushPull> for [
            PD12<12>,
        ],

        <A18, PushPull> for [
            PD13<12>,
        ],

        <A19, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA0<12>,

            PE3<12>,
        ],

        <A2, PushPull> for [
            PF2<12>,
        ],

        <A20, PushPull> for [
            PE4<12>,
        ],

        <A21, PushPull> for [
            PE5<12>,
        ],

        <A22, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC4<1>,

            PE6<12>,
        ],

        <A23, PushPull> for [
            PE2<12>,
        ],

        <A24, PushPull> for [
            PG13<12>,
        ],

        <A25, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC0<9>,

            PG14<12>,
        ],

        <A3, PushPull> for [
            PF3<12>,
        ],

        <A4, PushPull> for [
            PF4<12>,
        ],

        <A5, PushPull> for [
            PF5<12>,
        ],

        <A6, PushPull> for [
            PF12<12>,
        ],

        <A7, PushPull> for [
            PF13<12>,
        ],

        <A8, PushPull> for [
            PF14<12>,
        ],

        <A9, PushPull> for [
            PF15<12>,
        ],

        <Ba0, PushPull> for [
            PG4<12>,
        ],

        <Ba1, PushPull> for [
            PG5<12>,
        ],

        <Clk, PushPull> for [
            PD3<12>,
        ],

        <D0, PushPull> for [
            PD14<12>,
        ],

        <D1, PushPull> for [
            PD15<12>,
        ],

        <D2, PushPull> for [
            PD0<12>,
        ],

        <D3, PushPull> for [
            PD1<12>,
        ],

        <D10, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PB14<12>,

            PE13<12>,
        ],

        <D11, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PB15<12>,

            PE14<12>,
        ],

        <D12, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC0<1>,

            PE15<12>,
        ],

        <D13, PushPull> for [
            PD8<12>,
        ],

        <D14, PushPull> for [
            PD9<12>,
        ],

        <D15, PushPull> for [
            PD10<12>,
        ],

        <D16, PushPull> for [
            PH8<12>,
        ],

        <D17, PushPull> for [
            PH9<12>,
        ],

        <D18, PushPull> for [
            PH10<12>,
        ],

        <D19, PushPull> for [
            PH11<12>,
        ],

        <D20, PushPull> for [
            PH12<12>,
        ],

        <D21, PushPull> for [
            PH13<12>,
        ],

        <D22, PushPull> for [
            PH14<12>,
        ],

        <D23, PushPull> for [
            PH15<12>,
        ],

        <D4, PushPull> for [
            PE7<12>,
        ],

        <D5, PushPull> for [
            PE8<12>,
        ],

        <D6, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC12<1>,

            PE9<12>,
        ],

        <D7, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PD2<1>,

            PE10<12>,
        ],

        <D8, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA4<12>,

            PE11<12>,
        ],

        <D9, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA5<12>,

            PE12<12>,
        ],

        <Da0, PushPull> for [
            PD14<12>,
        ],

        <Da1, PushPull> for [
            PD15<12>,
        ],

        <Da10, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PB14<12>,

            PE13<12>,
        ],

        <Da11, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PB15<12>,

            PE14<12>,
        ],

        <Da12, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC0<1>,

            PE15<12>,
        ],

        <Da13, PushPull> for [
            PD8<12>,
        ],

        <Da14, PushPull> for [
            PD9<12>,
        ],

        <Da15, PushPull> for [
            PD10<12>,
        ],

        <Da2, PushPull> for [
            PD0<12>,
        ],

        <Da3, PushPull> for [
            PD1<12>,
        ],

        <Da4, PushPull> for [
            PE7<12>,
        ],

        <Da5, PushPull> for [
            PE8<12>,
        ],

        <Da6, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC12<1>,

            PE9<12>,
        ],

        <Da7, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PD2<1>,

            PE10<12>,
        ],

        <Da8, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA4<12>,

            PE11<12>,
        ],

        <Da9, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA5<12>,

            PE12<12>,
        ],

        <Int, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC8<10>,

            PG7<12>,
        ],

        <Nbl0, PushPull> for [
            PE0<12>,
        ],

        <Nbl1, PushPull> for [
            PE1<12>,
        ],

        <Nce, PushPull> for [
            PC8<9>,

            PG9<12>,
        ],

        <Ne1, PushPull> for [
            PC7<9>,

            PD7<12>,
        ],

        <Ne2, PushPull> for [
            PC8<9>,

            PG9<12>,
        ],

        <Ne3, PushPull> for [
            PG6<12>,

            PG10<12>,
        ],

        <Ne4, PushPull> for [
            PG12<12>,
        ],

        <Nl, PushPull> for [
            PB7<12>,
        ],

        <Noe, PushPull> for [
            PD4<12>,
        ],

        <Nwait, PushPull> for [
            PC6<9>,

            PD6<12>,
        ],

        <Nwe, PushPull> for [
            PD5<12>,
        ],

        <Sdcke0, PushPull> for [
            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC3<12>,

            PC5<12>,

            PH2<12>,
        ],

        <Sdcke1, PushPull> for [
            PB5<12>,

            PH7<12>,
        ],

        <Sdclk, PushPull> for [
            PG8<12>,
        ],

        <Sdncas, PushPull> for [
            PG15<12>,
        ],

        <Sdne0, PushPull> for [
            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC2<12>,

            PC4<12>,

            PH3<12>,
        ],

        <Sdne1, PushPull> for [
            PB6<12>,

            PH6<12>,
        ],

        <Sdnras, PushPull> for [
            PF11<12>,
        ],

        <Sdnwe, PushPull> for [
            PA7<12>,

            PC0<12>,

            PH5<12>,
        ],
    }

    #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
    pin! {
        <Ale, PushPull> for [
            PD12<12>,
        ],

        <Cle, PushPull> for [
            PD11<12>,
        ],

        <D24, PushPull> for [
            PI0<12>,
        ],

        <D25, PushPull> for [
            PI1<12>,
        ],

        <D26, PushPull> for [
            PI2<12>,
        ],

        <D27, PushPull> for [
            PI3<12>,
        ],

        <D28, PushPull> for [
            PI6<12>,
        ],

        <D29, PushPull> for [
            PI7<12>,
        ],

        <D30, PushPull> for [
            PI9<12>,
        ],

        <D31, PushPull> for [
            PI10<12>,
        ],

        <Nbl2, PushPull> for [
            PI4<12>,
        ],

        <Nbl3, PushPull> for [
            PI5<12>,
        ],
    }
}

#[cfg(feature = "gpio-h747")]
pub mod hrtim {
    use super::*;

    pin! {
        <Cha1, PushPull> for [ // High speed
            PC6<1>,
        ],

        <Cha2, PushPull> for [
            PC7<1>,
        ],

        <Chb1, PushPull> for [
            PC8<1>,
        ],

        <Chb2, PushPull> for [
            PA8<2>,
        ],

        <Chc1, PushPull> for [
            PA9<2>,
        ],

        <Chc2, PushPull> for [
            PA10<2>,
        ],

        <Chd1, PushPull> for [
            PA11<2>,
        ],

        <Chd2, PushPull> for [
            PA12<2>,
        ],

        <Che1, PushPull> for [
            PG6<2>,
        ],

        <Che2, PushPull> for [
            PG7<2>,
        ],

        <Eev1, PushPull> for [ // Low speed
            PC10<2>,
        ],

        <Eev10, PushPull> for [
            PG13<2>,
        ],

        <Eev2, PushPull> for [
            PC12<2>,
        ],

        <Eev3, PushPull> for [
            PD5<2>,
        ],

        <Eev4, PushPull> for [
            PG11<2>,
        ],

        <Eev5, PushPull> for [
            PG12<2>,
        ],

        <Eev6, PushPull> for [
            PB4<3>,
        ],

        <Eev7, PushPull> for [
            PB5<3>,
        ],

        <Eev8, PushPull> for [
            PB6<3>,
        ],

        <Eev9, PushPull> for [
            PB7<3>,
        ],

        <Flt1, PushPull> for [
            PA15<2>,
        ],

        <Flt2, PushPull> for [
            PC11<2>,
        ],

        <Flt3, PushPull> for [
            PD4<2>,
        ],

        <Flt4, PushPull> for [
            PB3<2>,
        ],

        <Flt5, PushPull> for [
            PG10<2>,
        ],

        <Scin, PushPull> for [
            PB11<2>,

            PE0<3>,
        ],
    }

    pin! {
        <Scout> default:PushPull for [ // High speed
            PB10<2>,

            PE1<3>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB6<4>,

            PB8<4>,
        ],

        <Sda, OpenDrain> for [
            PB7<4>,

            PB9<4>,
        ],

        <Smba, OpenDrain> for [
            PB5<4>,
        ],
    }
    use crate::pac::I2C1 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2c2 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB10<4>,

            PF1<4>,

            PH4<4>,
        ],

        <Sda, OpenDrain> for [
            PB11<4>,

            PF0<4>,

            PH5<4>,
        ],

        <Smba, OpenDrain> for [
            PB12<4>,

            PF2<4>,

            PH6<4>,
        ],
    }
    use crate::pac::I2C2 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<4>,

            PH7<4>,
        ],

        <Sda, OpenDrain> for [
            PC9<4>,

            PH8<4>,
        ],

        <Smba, OpenDrain> for [
            PA9<4>,

            PH9<4>,
        ],
    }
    use crate::pac::I2C3 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2c4 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB6<6>,

            PB8<6>,

            PD12<4>,

            PF14<4>,

            PH11<4>,
        ],

        <Sda, OpenDrain> for [
            PB7<6>,

            PB9<6>,

            PD13<4>,

            PF15<4>,

            PH12<4>,
        ],

        <Smba, OpenDrain> for [
            PB5<6>,

            PB9<11>,

            PD11<4>,

            PF13<4>,

            PH10<4>,
        ],
    }
    use crate::pac::I2C4 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

#[cfg(feature = "gpio-h72")]
pub mod i2c5 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<6>,

            PC11<4>,

            PF1<6>,
        ],

        <Sda, OpenDrain> for [
            PC9<6>,

            PC10<4>,

            PF0<6>,
        ],

        <Smba, OpenDrain> for [
            PA9<6>,

            PC12<4>,

            PF2<6>,
        ],
    }
    use crate::pac::I2C5 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2s {
    use super::*;

    pin! {
        <Ckin, PushPull> for [
            PC9<5>,
        ],
    }
}

pub mod i2s1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA5<5>,

            PB3<5>,

            PG11<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PC4<5>,
        ],

        <Sdi, PushPull> for [
            PA6<5>,

            PB4<5>,

            PG9<5>,
        ],

        <Sdo, PushPull> for [
            PA7<5>,

            PB5<5>,

            PD7<5>,
        ],

        <Ws, PushPull> for [
            PA4<5>,

            PA15<5>,

            PG10<5>,
        ],
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA9<5>,

            PA12<5>,

            PB10<5>,

            PB13<5>,

            PD3<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PC6<5>,
        ],

        <Sdi, PushPull> for [
            PB14<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC2<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI2<5>,
        ],

        <Sdo, PushPull> for [
            PB15<5>,

            PC1<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC3<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI3<5>,
        ],

        <Ws, PushPull> for [
            PA11<5>,

            PB4<7>,

            PB9<5>,

            PB12<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI0<5>,
        ],
    }
}

pub mod i2s3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB3<6>,

            PC10<6>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PC7<6>,
        ],

        <Sdi, PushPull> for [
            PB4<6>,

            PC11<6>,
        ],

        <Sdo, PushPull> for [
            PB2<7>,

            PB5<7>,

            PC12<6>,

            PD6<5>,
        ],

        <Ws, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
pub mod i2s6 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA5<8>,

            PB3<8>,

            PC12<5>,

            PG13<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            PA3<5>,
        ],

        <Sdi, PushPull> for [
            PA6<8>,

            PB4<8>,

            PG12<5>,
        ],

        <Sdo, PushPull> for [
            PA7<8>,

            PB5<8>,

            PG14<5>,
        ],

        <Ws, PushPull> for [
            PA0<5>,

            PA4<8>,

            PA15<7>,

            PG8<5>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PE0<1>,

            PG14<1>,
        ],

        <In1, PushPull> for [
            PD12<1>,

            PG12<1>,
        ],

        <In2, PushPull> for [
            PE1<1>,

            PG11<1>,

            PH2<1>,
        ],

        <Out> default:PushPull for [
            PD13<1>,

            PG13<1>,
        ],
    }
}

pub mod lptim2 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB11<3>,

            PE0<4>,
        ],

        <In1, PushPull> for [
            PB10<3>,

            PD12<3>,
        ],

        <In2, PushPull> for [
            PD11<3>,
        ],

        <Out> default:PushPull for [
            PB13<3>,
        ],
    }
}

pub mod lptim3 {
    use super::*;

    pin! {
        <Out> default:PushPull for [
            PA1<3>,
        ],
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h747"))]
pub mod lptim4 {
    use super::*;

    pin! {
        <Out> default:PushPull for [
            PA2<3>,
        ],
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h747"))]
pub mod lptim5 {
    use super::*;

    pin! {
        <Out> default:PushPull for [
            PA3<3>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PA11<3>,
        ],

        <De, PushPull> for [
            PA12<3>,
        ],

        <Rts, PushPull> for [
            PA12<3>,
        ],

        <Rx, PushPull> for [
            PA10<3>,

            PB7<8>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            PA9<3>,

            PB6<8>,
        ],
    }
}

pub mod ltdc {
    use super::*;

    pin! {
        <B0, PushPull> for [
            PE4<14>,

            PG14<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ12<14>,
        ],

        <B1, PushPull> for [
            PA10<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC10<10>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PD0<14>,

            PG12<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ13<14>,
        ],

        <B2, PushPull> for [
            PA3<9>,

            PC9<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PD2<14>,

            PD6<14>,

            PG10<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ14<14>,
        ],

        <B3, PushPull> for [
            PA8<13>,

            PD10<14>,

            PG11<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ15<14>,
        ],

        <B4, PushPull> for [
            PA10<12>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC11<14>,

            PE12<14>,

            PG12<9>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI4<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ13<9>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PK3<14>,
        ],

        <B5, PushPull> for [
            PA3<14>,

            #[cfg(feature = "gpio-h72")]
            PB5<3>,

            #[cfg(feature = "gpio-h7a2")]
            PB5<11>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI5<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PK4<14>,
        ],

        <B6, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PA15<14>,

            PB8<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI6<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PK5<14>,
        ],

        <B7, PushPull> for [
            PB9<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PD2<9>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI7<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PK6<14>,
        ],

        <Clk, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PB14<14>,

            PE14<14>,

            PG7<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI14<14>,
        ],

        <De, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC5<14>,

            PE13<14>,

            PF10<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PK7<14>,
        ],

        <G0, PushPull> for [
            PB1<14>,

            PE5<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ7<14>,
        ],

        <G1, PushPull> for [
            PB0<14>,

            PE6<14>,

            PJ8<14>,
        ],

        <G2, PushPull> for [
            PA6<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC0<11>,

            PH13<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI15<9>,

            PJ9<14>,
        ],

        <G3, PushPull> for [
            PC9<10>,

            PE11<14>,

            PG10<9>,

            PH14<14>,

            PJ10<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ12<9>,
        ],

        <G4, PushPull> for [
            PB10<14>,

            PH4<14>,

            PH15<14>,

            PJ11<14>,
        ],

        <G5, PushPull> for [
            PB11<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC1<14>,

            PH4<9>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI0<14>,

            PK0<14>,
        ],

        <G6, PushPull> for [
            PC7<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI11<9>,

            PK1<14>,
        ],

        <G7, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PB15<14>,

            PD3<14>,

            PG8<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI2<14>,

            PK2<14>,
        ],

        <Hsync, PushPull> for [
            PC6<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI10<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI12<14>,
        ],

        <R0, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PE0<14>,

            PG13<14>,

            PH2<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI15<14>,
        ],

        <R1, PushPull> for [
            PA2<14>,

            PH3<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ0<14>,
        ],

        <R2, PushPull> for [
            PA1<14>,

            PC10<14>,

            PH8<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ1<14>,
        ],

        <R3, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PA15<9>,

            PB0<9>,

            PH9<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ2<14>,
        ],

        <R4, PushPull> for [
            PA5<14>,

            PA11<14>,

            PH10<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ3<14>,
        ],

        <R5, PushPull> for [
            PA9<14>,

            PA12<14>,

            PC0<14>,

            PH11<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ4<14>,
        ],

        <R6, PushPull> for [
            PA8<14>,

            PB1<9>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC12<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PE1<14>,

            PH12<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ5<14>,
        ],

        <R7, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC4<14>,

            PE15<14>,

            PG6<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ0<9>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ6<14>,
        ],

        <Vsync, PushPull> for [
            PA4<14>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PA7<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI9<14>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI13<14>,
        ],
    }
}

pub mod mdios {
    use super::*;

    pin! {
        <Mdc, PushPull> for [
            PA6<11>,

            PC1<12>,
        ],

        <Mdio, PushPull> for [
            PA2<12>,

            PA10<11>,
        ],
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
pub mod octospim {
    use super::*;

    pin! {
        <P1Clk, PushPull> for [ // High speed
            #[cfg(feature = "gpio-h7a2")]
            PA3<3>,

            #[cfg(feature = "gpio-h72")]
            PA3<12>,

            PB2<9>,

            PF10<9>,
        ],

        <P1Dqs, PushPull> for [
            #[cfg(feature = "gpio-h7a2")]
            PA1<11>,

            #[cfg(feature = "gpio-h72")]
            PA1<12>,

            PB2<10>,

            PC5<10>,
        ],

        <P1Io0, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA2<6>,

            #[cfg(feature = "gpio-h72")]
            PB1<4>,

            #[cfg(feature = "gpio-h7a2")]
            PB1<11>,

            #[cfg(feature = "gpio-h72")]
            PB12<12>,

            #[cfg(feature = "gpio-h7a2")]
            PC3<9>,

            PC9<9>,

            PD11<9>,

            PF8<10>,
        ],

        <P1Io1, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PB0<4>,

            #[cfg(feature = "gpio-h7a2")]
            PB0<11>,

            PC10<9>,

            PD12<9>,

            PF9<10>,
        ],

        <P1Io2, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA3<6>,

            PA7<10>,

            #[cfg(feature = "gpio-h72")]
            PB13<4>,

            #[cfg(feature = "gpio-h7a2")]
            PC2<9>,

            PE2<9>,

            PF7<10>,
        ],

        <P1Io3, PushPull> for [
            PA1<9>,

            PA6<6>,

            PD13<9>,

            PF6<10>,
        ],

        <P1Io4, PushPull> for [
            PC1<10>,

            PD4<10>,

            PE7<10>,

            PH2<9>,
        ],

        <P1Io5, PushPull> for [
            #[cfg(feature = "gpio-h7a2")]
            PC2<11>,

            PD5<10>,

            PE8<10>,

            PH3<9>,
        ],

        <P1Io6, PushPull> for [
            #[cfg(feature = "gpio-h7a2")]
            PC3<11>,

            PD6<10>,

            PE9<10>,

            PG9<9>,
        ],

        <P1Io7, PushPull> for [
            PD7<10>,

            PE10<10>,

            PG14<9>,
        ],

        <P1Nclk, PushPull> for [
            PB12<3>,

            PF11<9>,
        ],

        <P1Ncs, PushPull> for [
            PB6<10>,

            PB10<9>,

            PC11<9>,

            PE11<11>,

            PG6<10>,
        ],

        <P2Clk, PushPull> for [
            PF4<9>,

            #[cfg(feature = "gpio-h7a2")]
            PI13<3>,
        ],

        <P2Dqs, PushPull> for [
            PF12<9>,

            PG7<9>,

            PG15<9>,

            #[cfg(feature = "gpio-h7a2")]
            PK6<3>,
        ],

        <P2Io0, PushPull> for [
            PF0<9>,

            #[cfg(feature = "gpio-h7a2")]
            PI9<3>,
        ],

        <P2Io1, PushPull> for [
            PF1<9>,

            #[cfg(feature = "gpio-h7a2")]
            PI10<3>,
        ],

        <P2Io2, PushPull> for [
            PF2<9>,

            #[cfg(feature = "gpio-h7a2")]
            PI11<3>,
        ],

        <P2Io3, PushPull> for [
            PF3<9>,

            #[cfg(feature = "gpio-h7a2")]
            PI12<3>,
        ],

        <P2Io4, PushPull> for [
            PG0<9>,

            #[cfg(feature = "gpio-h7a2")]
            PJ1<3>,
        ],

        <P2Io5, PushPull> for [
            PG1<9>,

            #[cfg(feature = "gpio-h7a2")]
            PJ2<3>,
        ],

        <P2Io6, PushPull> for [
            PG10<3>,

            #[cfg(feature = "gpio-h7a2")]
            PK3<3>,
        ],

        <P2Io7, PushPull> for [
            PG11<9>,

            #[cfg(feature = "gpio-h7a2")]
            PK4<3>,
        ],

        <P2Nclk, PushPull> for [
            PF5<9>,

            #[cfg(feature = "gpio-h7a2")]
            PI14<3>,
        ],

        <P2Ncs, PushPull> for [
            PG12<3>,

            #[cfg(feature = "gpio-h7a2")]
            PK5<3>,
        ],
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
pub mod pssi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<13>,

            PC6<13>,

            PH9<13>,
        ],

        <D1, PushPull> for [
            PA10<13>,

            PC7<13>,

            PH10<13>,
        ],

        <D10, PushPull> for [
            PB5<13>,

            PD6<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI3<13>,
        ],

        <D11, PushPull> for [
            PD2<13>,

            PF10<13>,

            PH15<13>,
        ],

        <D12, PushPull> for [
            PD12<13>,

            PF11<13>,

            PG6<13>,
        ],

        <D13, PushPull> for [
            PD13<13>,

            PG7<13>,

            PG15<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI0<13>,
        ],

        <D14, PushPull> for [
            PA5<13>,

            PH4<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI10<13>,
        ],

        <D15, PushPull> for [
            PC5<4>,

            PF10<4>,

            #[cfg(feature = "gpio-h7a2")]
            PI11<13>,
        ],

        <D2, PushPull> for [
            PB13<13>,

            PC8<13>,

            PE0<13>,

            PG10<13>,

            PH11<13>,
        ],

        <D3, PushPull> for [
            PC9<13>,

            PE1<13>,

            PG11<13>,

            PH12<13>,
        ],

        <D4, PushPull> for [
            PC11<13>,

            PE4<13>,

            PH14<13>,
        ],

        <D5, PushPull> for [
            PB6<13>,

            PD3<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI4<13>,
        ],

        <D6, PushPull> for [
            PB8<13>,

            PE5<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI6<13>,
        ],

        <D7, PushPull> for [
            PB9<13>,

            PE6<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI7<13>,
        ],

        <D8, PushPull> for [
            PC10<13>,

            PH6<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI1<13>,
        ],

        <D9, PushPull> for [
            PC12<13>,

            PH7<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI2<13>,
        ],

        <De, PushPull> for [
            PA4<13>,

            PH8<13>,
        ],

        <Pdck, PushPull> for [
            PA6<13>,
        ],

        <Rdy, PushPull> for [
            PB7<13>,

            PG9<13>,

            #[cfg(feature = "gpio-h7a2")]
            PI5<13>,
        ],
    }
}

#[cfg(feature = "gpio-h7a2")]
pub mod pwr {
    use super::*;

    pin! {
        <Csleep, PushPull> for [
            PC3<0>,
        ],

        <Cstop, PushPull> for [
            PC2<0>,
        ],

        <Ndstop2, PushPull> for [
            PA5<0>,
        ],
    }
}

#[cfg(feature = "gpio-h747")]
pub mod quadspi {
    use super::*;

    pin! {
        <Bk1Io0, PushPull> for [
            PC9<9>,

            PD11<9>,

            PF8<10>,
        ],

        <Bk1Io1, PushPull> for [
            PC10<9>,

            PD12<9>,

            PF9<10>,
        ],

        <Bk1Io2, PushPull> for [
            PE2<9>,

            PF7<9>,
        ],

        <Bk1Io3, PushPull> for [
            PA1<9>,

            PD13<9>,

            PF6<9>,
        ],

        <Bk1Ncs, PushPull> for [
            PB6<10>,

            PB10<9>,

            PG6<10>,
        ],

        <Bk2Io0, PushPull> for [
            PE7<10>,

            PH2<9>,
        ],

        <Bk2Io1, PushPull> for [
            PE8<10>,

            PH3<9>,
        ],

        <Bk2Io2, PushPull> for [
            PE9<10>,

            PG9<9>,
        ],

        <Bk2Io3, PushPull> for [
            PE10<10>,

            PG14<9>,
        ],

        <Bk2Ncs, PushPull> for [
            PC11<9>,
        ],

        <Clk, PushPull> for [
            PB2<9>,

            PF10<9>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco1, PushPull> for [
            PA8<0>,
        ],

        <Mco2, PushPull> for [
            PC9<0>,
        ],
    }
}

pub mod rtc {
    use super::*;

    #[cfg(any(feature = "gpio-h72", feature = "gpio-h747"))]
    pin! {
        <OutCalib, PushPull> for [
            PB2<0>,
        ],
    }

    pin! {
        <OutAlarm, PushPull> for [
            PB2<0>,
        ],

        <Refin, PushPull> for [
            PB15<0>,
        ],
    }
}

pub mod sai1 {
    use super::*;

    pin! {
        <Ck1, PushPull> for [
            PE2<2>,
        ],

        <Ck2, PushPull> for [
            PE5<2>,
        ],

        <D1, PushPull> for [
            PB2<2>,

            PC1<2>,

            PD6<2>,

            PE6<2>,
        ],

        <D2, PushPull> for [
            PE4<2>,
        ],

        <D3, PushPull> for [
            PC5<2>,

            PF10<2>,
        ],

        <FsA, PushPull> for [
            PE4<6>,
        ],

        <FsB, PushPull> for [
            PF9<6>,
        ],

        <MclkA, PushPull> for [
            PE2<6>,

            PG7<6>,
        ],

        <MclkB, PushPull> for [
            PF7<6>,
        ],

        <SckA, PushPull> for [
            PE5<6>,
        ],

        <SckB, PushPull> for [
            PF8<6>,
        ],

        <SdA, PushPull> for [
            PB2<6>,

            PC1<6>,

            PD6<6>,

            PE6<6>,
        ],

        <SdB, PushPull> for [
            PE3<6>,

            PF6<6>,
        ],
    }
    use crate::pac::SAI1 as SAI;
    pub struct ChannelA;
    pub struct ChannelB;
    impl SaiChannels for SAI {
        type A = ChannelA;
        type B = ChannelB;
    }
    impl SaiChannel for ChannelA {
        type Fs = FsA;
        type Mclk = MclkA;
        type Sck = SckA;
        type Sd = SdA;
    }
    impl SaiChannel for ChannelB {
        type Fs = FsB;
        type Mclk = MclkB;
        type Sck = SckB;
        type Sd = SdB;
    }
}

#[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
pub mod sai2 {
    use super::*;

    pin! {
        <FsA, PushPull> for [
            PD12<10>,

            PI7<10>,
        ],

        <FsB, PushPull> for [
            PA12<8>,

            PC0<8>,

            PE13<10>,

            PG9<10>,
        ],

        <MclkA, PushPull> for [
            PE0<10>,

            PI4<10>,
        ],

        <MclkB, PushPull> for [
            PA1<10>,

            PE6<10>,

            PE14<10>,

            PH3<10>,
        ],

        <SckA, PushPull> for [
            PD13<10>,

            PI5<10>,
        ],

        <SckB, PushPull> for [
            PA2<8>,

            PE12<10>,

            PH2<10>,
        ],

        <SdA, PushPull> for [
            PD11<10>,

            PI6<10>,
        ],

        <SdB, PushPull> for [
            PA0<10>,

            PE11<10>,

            PF11<10>,

            PG10<10>,
        ],
    }
    use crate::pac::SAI2 as SAI;
    pub struct ChannelA;
    pub struct ChannelB;
    impl SaiChannels for SAI {
        type A = ChannelA;
        type B = ChannelB;
    }
    impl SaiChannel for ChannelA {
        type Fs = FsA;
        type Mclk = MclkA;
        type Sck = SckA;
        type Sd = SdA;
    }
    impl SaiChannel for ChannelB {
        type Fs = FsB;
        type Mclk = MclkB;
        type Sck = SckB;
        type Sd = SdB;
    }
}

#[cfg(feature = "gpio-h747")]
pub mod sai3 {
    use super::*;

    pin! {
        <FsA, PushPull> for [
            PD4<6>,
        ],

        <FsB, PushPull> for [
            PD10<6>,
        ],

        <MclkA, PushPull> for [
            PD15<6>,
        ],

        <MclkB, PushPull> for [
            PD14<6>,
        ],

        <SckA, PushPull> for [
            PD0<6>,
        ],

        <SckB, PushPull> for [
            PD8<6>,
        ],

        <SdA, PushPull> for [
            PD1<6>,
        ],

        <SdB, PushPull> for [
            PD9<6>,
        ],
    }
    use crate::pac::SAI3 as SAI;
    pub struct ChannelA;
    pub struct ChannelB;
    impl SaiChannels for SAI {
        type A = ChannelA;
        type B = ChannelB;
    }
    impl SaiChannel for ChannelA {
        type Fs = FsA;
        type Mclk = MclkA;
        type Sck = SckA;
        type Sd = SdA;
    }
    impl SaiChannel for ChannelB {
        type Fs = FsB;
        type Mclk = MclkB;
        type Sck = SckB;
        type Sd = SdB;
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h747"))]
pub mod sai4 {
    use super::*;

    pin! {
        <Ck1, PushPull> for [
            PE2<10>,
        ],

        <Ck2, PushPull> for [
            PE5<10>,
        ],

        <D1, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PB2<1>,

            #[cfg(feature = "gpio-h747")]
            PB2<10>,

            #[cfg(feature = "gpio-h72")]
            PC1<1>,

            #[cfg(feature = "gpio-h747")]
            PC1<10>,

            #[cfg(feature = "gpio-h72")]
            PD6<1>,

            #[cfg(feature = "gpio-h747")]
            PD6<10>,

            PE6<9>,
        ],

        <D2, PushPull> for [
            PE4<10>,
        ],

        <D3, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC5<1>,

            #[cfg(feature = "gpio-h747")]
            PC5<10>,

            PF10<10>,
        ],

        <FsA, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PD12<10>,

            PE4<8>,
        ],

        <FsB, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA12<8>,

            #[cfg(feature = "gpio-h72")]
            PC0<8>,

            #[cfg(feature = "gpio-h72")]
            PE13<10>,

            PF9<8>,

            #[cfg(feature = "gpio-h72")]
            PG9<10>,
        ],

        <MclkA, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PE0<10>,

            PE2<8>,
        ],

        <MclkB, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA1<10>,

            #[cfg(feature = "gpio-h72")]
            PE6<10>,

            #[cfg(feature = "gpio-h72")]
            PE14<10>,

            PF7<8>,

            #[cfg(feature = "gpio-h72")]
            PH3<10>,
        ],

        <SckA, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PD13<10>,

            PE5<8>,
        ],

        <SckB, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA2<8>,

            #[cfg(feature = "gpio-h72")]
            PE12<10>,

            PF8<8>,

            #[cfg(feature = "gpio-h72")]
            PH2<10>,
        ],

        <SdA, PushPull> for [
            PB2<8>,

            PC1<8>,

            PD6<8>,

            #[cfg(feature = "gpio-h72")]
            PD11<10>,

            PE6<8>,
        ],

        <SdB, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA0<10>,

            PE3<8>,

            #[cfg(feature = "gpio-h72")]
            PE11<10>,

            PF6<8>,

            #[cfg(feature = "gpio-h72")]
            PF11<10>,

            #[cfg(feature = "gpio-h72")]
            PG10<10>,
        ],
    }
    use crate::pac::SAI4 as SAI;
    pub struct ChannelA;
    pub struct ChannelB;
    impl SaiChannels for SAI {
        type A = ChannelA;
        type B = ChannelB;
    }
    impl SaiChannel for ChannelA {
        type Fs = FsA;
        type Mclk = MclkA;
        type Sck = SckA;
        type Sd = SdA;
    }
    impl SaiChannel for ChannelB {
        type Fs = FsB;
        type Mclk = MclkB;
        type Sck = SckB;
        type Sd = SdB;
    }
}

pub mod sdmmc1 {
    use super::*;

    pin! {
        <Cdir, PushPull> for [
            PB9<7>,
        ],

        <Ck, PushPull> for [
            PC12<12>,
        ],

        <Ckin, PushPull> for [
            PB8<7>,
        ],

        <Cmd, PushPull> for [
            PD2<12>,
        ],

        <D0, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PB13<12>,

            PC8<12>,
        ],

        <D0Dir, PushPull> for [
            PC6<8>,
        ],

        <D1, PushPull> for [
            PC9<12>,
        ],

        <D123Dir, PushPull> for [
            PC7<8>,
        ],

        <D2, PushPull> for [
            PC10<12>,
        ],

        <D3, PushPull> for [
            PC11<12>,
        ],

        <D4, PushPull> for [
            PB8<12>,
        ],

        <D5, PushPull> for [
            PB9<12>,
        ],

        <D6, PushPull> for [
            PC6<12>,
        ],

        <D7, PushPull> for [
            PC7<12>,
        ],
    }
}

pub mod sdmmc2 {
    use super::*;

    #[cfg(feature = "gpio-h72")]
    pin! {
        <Ckin, PushPull> for [
            PC4<10>,
        ],
    }

    pin! {
        <Ck, PushPull> for [
            PC1<9>,

            PD6<11>,
        ],

        <Cmd, PushPull> for [
            PA0<9>,

            PD7<11>,
        ],

        <D0, PushPull> for [
            PB14<9>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PG9<11>,
        ],

        <D1, PushPull> for [
            PB15<9>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PG10<11>,
        ],

        <D2, PushPull> for [
            PB3<9>,

            PG11<10>,
        ],

        <D3, PushPull> for [
            PB4<9>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PG12<10>,
        ],

        <D4, PushPull> for [
            PB8<10>,
        ],

        <D5, PushPull> for [
            PB9<10>,
        ],

        <D6, PushPull> for [
            PC6<10>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PG13<10>,
        ],

        <D7, PushPull> for [
            PC7<10>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PG14<10>,
        ],
    }
}

#[cfg(feature = "gpio-h7a2")]
pub mod spdifrx {
    use super::*;

    pin! {
        <In1, PushPull> for [
            PD7<9>,

            PG11<8>,
        ],

        <In2, PushPull> for [
            PD8<9>,

            PG12<8>,
        ],

        <In3, PushPull> for [
            PC4<9>,

            PG8<8>,
        ],

        <In4, PushPull> for [
            PC5<9>,

            PG9<8>,
        ],
    }

    use crate::pac::SPDIFRX;
    impl SPdifIn<0> for SPDIFRX {
        type In = In0;
    }
    impl SPdifIn<1> for SPDIFRX {
        type In = In1;
    }
    impl SPdifIn<2> for SPDIFRX {
        type In = In2;
    }
    impl SPdifIn<3> for SPDIFRX {
        type In = In3;
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h747"))]
pub mod spdifrx1 {
    use super::*;

    #[cfg(feature = "gpio-h747")]
    pin! {
        <In0, PushPull> for [
            PD7<9>,

            PG11<8>,
        ],
    }

    pin! {
        <In1, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PD7<9>,

            #[cfg(feature = "gpio-h747")]
            PD8<9>,

            #[cfg(feature = "gpio-h72")]
            PG11<8>,

            #[cfg(feature = "gpio-h747")]
            PG12<8>,
        ],

        <In2, PushPull> for [
            #[cfg(feature = "gpio-h747")]
            PC4<9>,

            #[cfg(feature = "gpio-h72")]
            PD8<9>,

            #[cfg(feature = "gpio-h747")]
            PG8<8>,

            #[cfg(feature = "gpio-h72")]
            PG12<8>,
        ],

        <In3, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PC4<9>,

            #[cfg(feature = "gpio-h747")]
            PC5<9>,

            #[cfg(feature = "gpio-h72")]
            PG8<8>,

            #[cfg(feature = "gpio-h747")]
            PG9<8>,
        ],
    }

    #[cfg(feature = "gpio-h72")]
    pin! {
        <In4, PushPull> for [
            PC5<9>,

            PG9<8>,
        ],
    }

    use crate::pac::SPDIFRX;
    impl SPdifIn<0> for SPDIFRX {
        type In = In0;
    }
    impl SPdifIn<1> for SPDIFRX {
        type In = In1;
    }
    impl SPdifIn<2> for SPDIFRX {
        type In = In2;
    }
    impl SPdifIn<3> for SPDIFRX {
        type In = In3;
    }
}

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<5>,

            PB4<5>,

            PG9<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PB5<5>,

            PD7<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,

            PG10<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<5>,

            PB3<5>,

            PG11<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI1 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod spi2 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB14<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC2<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI2<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<5>,

            PC1<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC3<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI3<5>,
        ],

        <Nss, PushPull> for [
            PA11<5>,

            PB4<7>,

            PB9<5>,

            PB12<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA9<5>,

            PA12<5>,

            PB10<5>,

            PB13<5>,

            PD3<5>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI2 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod spi3 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB4<6>,

            PC11<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB2<7>,

            PB5<7>,

            PC12<6>,

            PD6<5>,
        ],

        <Nss, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB3<6>,

            PC10<6>,
        ],
    }
    impl SpiCommon for crate::pac::SPI3 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod spi4 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PE5<5>,

            PE13<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PE6<5>,

            PE14<5>,
        ],

        <Nss, PushPull> for [
            PE4<5>,

            PE11<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PE2<5>,

            PE12<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI4 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod spi5 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PF8<5>,

            PH7<5>,

            PJ11<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PF9<5>,

            PF11<5>,

            PJ10<5>,
        ],

        <Nss, PushPull> for [
            PF6<5>,

            PH5<5>,

            PK1<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PF7<5>,

            PH6<5>,

            PK0<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI5 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod spi6 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<8>,

            PB4<8>,

            PG12<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<8>,

            PB5<8>,

            PG14<5>,
        ],

        <Nss, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PA0<5>,

            PA4<8>,

            PA15<7>,

            PG8<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<8>,

            PB3<8>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC12<5>,

            PG13<5>,
        ],
    }
    impl SpiCommon for crate::pac::SPI6 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

pub mod swpmi1 {
    use super::*;

    pin! {
        <Rx, PushPull> for [
            PC8<11>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC10<11>,
        ],

        <Suspend, PushPull> for [
            PC9<11>,
        ],

        <Tx, PushPull> for [
            PC7<11>,
        ],
    }
}

#[cfg(feature = "gpio-h747")]
pub mod sys {
    use super::*;

    pin! {
        <Jtrst, PushPull> for [
            PB4<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<1>,

            PE9<1>,

            PK1<1>,
        ],

        <Ch1N> default:PushPull for [
            PA7<1>,

            PB13<1>,

            PE8<1>,

            PK0<1>,
        ],

        <Ch2> default:PushPull for [
            PA9<1>,

            PE11<1>,

            PJ11<1>,
        ],

        <Ch2N> default:PushPull for [
            PB0<1>,

            PB14<1>,

            PE10<1>,

            PJ10<1>,
        ],

        <Ch3> default:PushPull for [
            PA10<1>,

            PE13<1>,

            PJ9<1>,
        ],

        <Ch3N> default:PushPull for [
            PB1<1>,

            PB15<1>,

            PE12<1>,

            PJ8<1>,
        ],

        <Ch4> default:PushPull for [
            PA11<1>,

            PE14<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<1>,

            PB12<1>,

            PE15<1>,

            PK2<1>,
        ],

        <Bkin2, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PA12<12>,

            PE6<1>,

            PG4<1>,
        ],

        <Bkin2Comp1, PushPull> for [
            PE6<11>,

            PG4<11>,
        ],

        <Bkin2Comp2, PushPull> for [
            PE6<11>,

            PG4<11>,
        ],

        <BkinComp1, PushPull> for [
            PA6<12>,

            PB12<13>,

            PE15<13>,

            PK2<11>,
        ],

        <BkinComp2, PushPull> for [
            PA6<12>,

            PB12<13>,

            PE15<13>,

            PK2<11>,
        ],

        <Etr, PushPull> for [
            PA12<1>,

            PE7<1>,

            PG5<1>,
        ],
    }

    use crate::pac::TIM1 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimNCPin<1> for TIM {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<3> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim2 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<1>,

            PA5<1>,

            PA15<1>,
        ],

        <Ch2> default:PushPull for [
            PA1<1>,

            PB3<1>,
        ],

        <Ch3> default:PushPull for [
            PA2<1>,

            PB10<1>,
        ],

        <Ch4> default:PushPull for [
            PA3<1>,

            PB11<1>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<1>,

            PA5<1>,

            PA15<1>,
        ],
    }

    use crate::pac::TIM2 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<2>,

            PB4<2>,

            PC6<2>,
        ],

        <Ch2> default:PushPull for [
            PA7<2>,

            PB5<2>,

            PC7<2>,
        ],

        <Ch3> default:PushPull for [
            PB0<2>,

            PC8<2>,
        ],

        <Ch4> default:PushPull for [
            PB1<2>,

            PC9<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PD2<2>,
        ],
    }

    use crate::pac::TIM3 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim4 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB6<2>,

            PD12<2>,
        ],

        <Ch2> default:PushPull for [
            PB7<2>,

            PD13<2>,
        ],

        <Ch3> default:PushPull for [
            PB8<2>,

            PD14<2>,
        ],

        <Ch4> default:PushPull for [
            PB9<2>,

            PD15<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PE0<2>,
        ],
    }

    use crate::pac::TIM4 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PH10<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PH11<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PH12<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI0<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA4<2>,

            PH8<2>,
        ],
    }

    use crate::pac::TIM5 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim8 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PC6<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI5<3>,

            PJ8<3>,
        ],

        <Ch1N> default:PushPull for [
            PA5<3>,

            PA7<3>,

            PH13<3>,

            PJ9<3>,
        ],

        <Ch2> default:PushPull for [
            PC7<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI6<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ6<3>,

            PJ10<3>,
        ],

        <Ch2N> default:PushPull for [
            PB0<3>,

            PB14<3>,

            PH14<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PJ7<3>,

            PJ11<3>,
        ],

        <Ch3> default:PushPull for [
            PC8<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI7<3>,

            PK0<3>,
        ],

        <Ch3N> default:PushPull for [
            PB1<3>,

            PB15<3>,

            PH15<3>,

            PK1<3>,
        ],

        <Ch4> default:PushPull for [
            PC9<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI2<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<3>,

            PG2<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI4<3>,

            PK2<3>,
        ],

        <Bkin2, PushPull> for [
            PA8<3>,

            PG3<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<3>,
        ],

        <Bkin2Comp1, PushPull> for [
            PA8<12>,

            PG3<11>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<11>,
        ],

        <Bkin2Comp2, PushPull> for [
            PA8<12>,

            PG3<11>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI1<11>,
        ],

        <BkinComp1, PushPull> for [
            PA6<10>,

            PG2<11>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI4<11>,

            PK2<10>,
        ],

        <BkinComp2, PushPull> for [
            PA6<10>,

            PG2<11>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI4<11>,

            PK2<10>,
        ],

        <Etr, PushPull> for [
            PA0<3>,

            PG8<3>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI3<3>,
        ],
    }

    use crate::pac::TIM8 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimNCPin<1> for TIM {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<3> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim12 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB14<2>,

            PH6<2>,
        ],

        <Ch2> default:PushPull for [
            PB15<2>,

            PH9<2>,
        ],
    }

    use crate::pac::TIM12 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
}

pub mod tim13 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<9>,

            PF8<9>,
        ],
    }

    use crate::pac::TIM13 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim14 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<9>,

            PF9<9>,
        ],
    }

    use crate::pac::TIM14 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim15 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<4>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PC12<2>,

            PE5<4>,
        ],

        <Ch1N> default:PushPull for [
            PA1<4>,

            PE4<4>,
        ],

        <Ch2> default:PushPull for [
            PA3<4>,

            PE6<4>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA0<4>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PD2<4>,

            PE3<4>,
        ],

    }

    use crate::pac::TIM15 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod tim16 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB8<1>,

            PF6<1>,
        ],

        <Ch1N> default:PushPull for [
            PB6<1>,

            PF8<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PB4<1>,

            PF10<1>,
        ],

    }

    use crate::pac::TIM16 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

pub mod tim17 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PB9<1>,

            PF7<1>,
        ],

        <Ch1N> default:PushPull for [
            PB7<1>,

            PF9<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PB5<1>,

            PG6<1>,
        ],

    }

    use crate::pac::TIM17 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
}

#[cfg(feature = "gpio-h72")]
pub mod tim23 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PF0<13>,

            PF6<13>,

            PG12<13>,
        ],

        <Ch2> default:PushPull for [
            PF1<13>,

            PF7<13>,

            PG13<13>,
        ],

        <Ch3> default:PushPull for [
            PF2<13>,

            PF8<13>,

            PG14<13>,
        ],

        <Ch4> default:PushPull for [
            PF3<13>,

            PF9<13>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PB2<13>,

            PG3<13>,
        ],
    }

    use crate::pac::TIM23 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(feature = "gpio-h72")]
pub mod tim24 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PF11<14>,
        ],

        <Ch2> default:PushPull for [
            PF12<14>,
        ],

        <Ch3> default:PushPull for [
            PF13<14>,
        ],

        <Ch4> default:PushPull for [
            PF14<14>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PB3<14>,

            PG2<14>,
        ],
    }

    use crate::pac::TIM24 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<4> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod uart4 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB0<8>,

            PB15<8>,
        ],

        <De, PushPull> for [
            PA15<8>,

            PB14<8>,
        ],

        <Rts, PushPull> for [
            PA15<8>,

            PB14<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA1<8>,

            PA11<6>,

            PB8<8>,

            PC11<8>,

            PD0<8>,

            PH14<8>,

            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PI9<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<8>,

            PA12<6>,

            PB9<8>,

            PC10<8>,

            PD1<8>,

            PH13<8>,
        ],
    }
    use crate::pac::UART4 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod uart5 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PC9<8>,
        ],

        <De, PushPull> for [
            PC8<8>,
        ],

        <Rts, PushPull> for [
            PC8<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB5<14>,

            PB12<14>,

            PD2<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB6<14>,

            PB13<14>,

            PC12<8>,
        ],
    }
    use crate::pac::UART5 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod uart7 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PE10<7>,

            PF9<7>,
        ],

        <De, PushPull> for [
            PE9<7>,

            PF8<7>,
        ],

        <Rts, PushPull> for [
            PE9<7>,

            PF8<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA8<11>,

            PB3<11>,

            PE7<7>,

            PF6<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA15<11>,

            PB4<11>,

            PE8<7>,

            PF7<7>,
        ],
    }
    use crate::pac::UART7 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod uart8 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PD14<8>,
        ],

        <De, PushPull> for [
            PD15<8>,
        ],

        <Rts, PushPull> for [
            PD15<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PE0<8>,

            PJ9<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PE1<8>,

            PJ8<8>,
        ],
    }
    use crate::pac::UART8 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
pub mod uart9 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PD0<11>,

            #[cfg(feature = "gpio-h7a2")]
            PJ4<11>,
        ],

        <De, PushPull> for [
            PD13<11>,

            #[cfg(feature = "gpio-h7a2")]
            PJ3<11>,
        ],

        <Rts, PushPull> for [
            PD13<11>,

            #[cfg(feature = "gpio-h7a2")]
            PJ3<11>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PD14<11>,

            PG0<11>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PD15<11>,

            PG1<11>,
        ],
    }
    use crate::pac::UART9 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<7>,
        ],

        <Cts, PushPull> for [
            PA11<7>,
        ],

        <De, PushPull> for [
            PA12<7>,
        ],

        <Nss, PushPull> for [
            PA11<7>,
        ],

        <Rts, PushPull> for [
            PA12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA10<7>,

            PB7<7>,

            PB15<4>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<7>,

            PB6<7>,

            PB14<4>,
        ],
    }
    use crate::pac::USART1 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod usart2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA4<7>,

            PD7<7>,
        ],

        <Cts, PushPull> for [
            PA0<7>,

            PD3<7>,
        ],

        <De, PushPull> for [
            PA1<7>,

            PD4<7>,
        ],

        <Nss, PushPull> for [
            PA0<7>,

            PD3<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,

            PD4<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<7>,

            PD6<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<7>,

            PD5<7>,
        ],
    }
    use crate::pac::USART2 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod usart3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB12<7>,

            PC12<7>,

            PD10<7>,
        ],

        <Cts, PushPull> for [
            PB13<7>,

            PD11<7>,
        ],

        <De, PushPull> for [
            PB14<7>,

            PD12<7>,
        ],

        <Nss, PushPull> for [
            PB13<7>,

            PD11<7>,
        ],

        <Rts, PushPull> for [
            PB14<7>,

            PD12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB11<7>,

            PC11<7>,

            PD9<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB10<7>,

            PC10<7>,

            PD8<7>,
        ],
    }
    use crate::pac::USART3 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod usart6 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC8<7>,

            PG7<7>,
        ],

        <Cts, PushPull> for [
            PG13<7>,

            PG15<7>,
        ],

        <De, PushPull> for [
            PG8<7>,

            PG12<7>,
        ],

        <Nss, PushPull> for [
            PG13<7>,

            PG15<7>,
        ],

        <Rts, PushPull> for [
            PG8<7>,

            PG12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PC7<7>,

            PG9<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PC6<7>,

            PG14<7>,
        ],
    }
    use crate::pac::USART6 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
pub mod usart10 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PE15<11>,

            PG15<11>,
        ],

        <Cts, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PG13<4>,

            #[cfg(feature = "gpio-h7a2")]
            PG13<11>,
        ],

        <De, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PG14<4>,

            #[cfg(feature = "gpio-h7a2")]
            PG14<11>,
        ],

        <Nss, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PG13<4>,

            #[cfg(feature = "gpio-h7a2")]
            PG13<11>,
        ],

        <Rts, PushPull> for [
            #[cfg(feature = "gpio-h72")]
            PG14<4>,

            #[cfg(feature = "gpio-h7a2")]
            PG14<11>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            #[cfg(feature = "gpio-h72")]
            PE2<4>,

            #[cfg(feature = "gpio-h7a2")]
            PE2<11>,

            #[cfg(feature = "gpio-h72")]
            PG11<4>,

            #[cfg(feature = "gpio-h7a2")]
            PG11<11>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PE3<11>,

            #[cfg(feature = "gpio-h72")]
            PG12<4>,

            #[cfg(feature = "gpio-h7a2")]
            PG12<11>,
        ],
    }
    use crate::pac::USART10 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "gpio-h747")]
pub mod otg_fs {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PA11<10>,
        ],

        <Dp, PushPull> for [
            PA12<10>,
        ],

        <Id, PushPull> for [
            PA10<10>,
        ],

        <Sof, PushPull> for [
            PA8<10>,
        ],
    }
}

pub mod otg_hs {
    use super::*;

    #[cfg(feature = "gpio-h747")]
    pin! {
        <Dm, PushPull> for [
            PB14<12>,
        ],

        <Dp, PushPull> for [
            PB15<12>,
        ],
    }

    pin! {
        <Id, PushPull> for [
            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PA10<10>,

            #[cfg(feature = "gpio-h747")]
            PB12<12>,
        ],

        <Sof, PushPull> for [
            #[cfg(feature = "gpio-h747")]
            PA4<12>,

            #[cfg(any(feature = "gpio-h72", feature = "gpio-h7a2"))]
            PA8<10>,
        ],

        <UlpiCk, PushPull> for [
            PA5<10>,
        ],

        <UlpiD0, PushPull> for [
            PA3<10>,
        ],

        <UlpiD1, PushPull> for [
            PB0<10>,
        ],

        <UlpiD2, PushPull> for [
            PB1<10>,
        ],

        <UlpiD3, PushPull> for [
            PB10<10>,
        ],

        <UlpiD4, PushPull> for [
            PB11<10>,
        ],

        <UlpiD5, PushPull> for [
            PB12<10>,
        ],

        <UlpiD6, PushPull> for [
            PB13<10>,
        ],

        <UlpiD7, PushPull> for [
            PB5<10>,
        ],

        <UlpiNxt, PushPull> for [
            #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
            PC3<10>,

            PH4<10>,
        ],

        <UlpiStp, PushPull> for [
            PC0<10>,
        ],
    }

    #[cfg(any(feature = "gpio-h747", feature = "gpio-h7a2"))]
    pin! {
        <UlpiDir, PushPull> for [
            PC2<10>,

            PI11<10>,
        ],
    }
}
