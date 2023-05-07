use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

#[cfg(any(
    feature = "gpio-l43x",
    feature = "gpio-l45x",
    feature = "gpio-l47x",
    feature = "gpio-l49x"
))]
pub mod can1 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PA11<9>,

            #[cfg(feature = "gpio-l45x")]
            PB5<3>,

            PB8<9>,

            #[cfg(feature = "gpio-l45x")]
            PB12<10>,

            PD0<9>,

            #[cfg(feature = "gpio-l49x")]
            PI9<9>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PA12<9>,

            #[cfg(feature = "gpio-l45x")]
            PB6<8>,

            PB9<9>,

            #[cfg(feature = "gpio-l45x")]
            PB13<10>,

            PD1<9>,

            #[cfg(feature = "gpio-l49x")]
            PH13<9>,
        ],
    }
    impl CanCommon for crate::pac::CAN1 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

#[cfg(feature = "gpio-l49x")]
pub mod can2 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PB5<3>,

            PB12<10>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PB6<8>,

            PB13<10>,
        ],
    }
    impl CanCommon for crate::pac::CAN2 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

pub mod comp1 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA0<12>,

            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA6<6>,

            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA11<6>,

            PB0<12>,

            PB10<12>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-l43x",
    feature = "gpio-l45x",
    feature = "gpio-l47x",
    feature = "gpio-l49x"
))]
pub mod comp2 {
    use super::*;

    pin! {
        <Out, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA2<12>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA7<12>,

            PB5<12>,

            PB11<12>,
        ],
    }
}

/*#[cfg(any(
    feature = "gpio-l41x",
    feature = "gpio-l43x",
    feature = "gpio-l45x",
    feature = "gpio-l49x"
))]
pub mod crs {
    use super::*;

    pin! {
        <Sync> for [
            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA10<10>,

            #[cfg(feature = "gpio-l49x")]
            PB3<10>,
        ],
    }
}*/

#[cfg(feature = "gpio-l49x")]
pub mod dcmi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<5>,

            PC6<10>,

            PH9<10>,
        ],

        <D1, PushPull> for [
            PA10<5>,

            PC7<10>,

            PH10<10>,
        ],

        <D10, PushPull> for [
            PB5<10>,

            PD6<4>,

            PI3<10>,
        ],

        <D11, PushPull> for [
            PD2<10>,

            PF10<10>,

            PH15<10>,
        ],

        <D12, PushPull> for [
            PB4<10>,

            PF11<10>,

            PI8<10>,
        ],

        <D13, PushPull> for [
            PG15<10>,

            PI0<10>,
        ],

        <D2, PushPull> for [
            PC8<10>,

            PE0<10>,

            PH11<10>,
        ],

        <D3, PushPull> for [
            PC9<4>,

            PE1<10>,

            PH12<10>,
        ],

        <D4, PushPull> for [
            PC11<10>,

            PE4<10>,

            PH14<10>,
        ],

        <D5, PushPull> for [
            PB6<10>,

            PD3<4>,

            PI4<10>,
        ],

        <D6, PushPull> for [
            PB8<10>,

            PE5<10>,

            PI6<10>,
        ],

        <D7, PushPull> for [
            PB9<10>,

            PE6<10>,

            PI7<10>,
        ],

        <D8, PushPull> for [
            PC10<10>,

            PH6<10>,

            PI1<10>,
        ],

        <D9, PushPull> for [
            PC12<10>,

            PH7<10>,

            PI2<10>,
        ],

        <Hsync, PushPull> for [
            PA4<10>,

            PD8<10>,

            PH8<10>,
        ],

        <Pixclk, PushPull> for [
            PA6<4>,

            PD9<10>,

            PH5<10>,
        ],

        <Vsync, PushPull> for [
            PB7<10>,

            PI5<10>,
        ],
    }
}

#[cfg(any(feature = "gpio-l45x", feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod dfsdm1 {
    use super::*;

    pin! {
        <Ckin0, PushPull> for [
            #[cfg(feature = "gpio-l45x")]
            PB0<6>,

            PB2<6>,

            PD4<6>,
        ],

        <Ckin1, PushPull> for [
            #[cfg(feature = "gpio-l45x")]
            PA8<6>,

            PB13<6>,

            PD7<6>,
        ],

        <Ckin2, PushPull> for [
            PB15<6>,

            PE8<6>,
        ],

        <Ckin3, PushPull> for [
            PC6<6>,

            PE5<6>,
        ],

        <Ckout, PushPull> for [
            #[cfg(feature = "gpio-l45x")]
            PA5<6>,

            PC2<6>,

            PE9<6>,
        ],

        <Datin0, PushPull> for [
            #[cfg(feature = "gpio-l45x")]
            PA7<6>,

            PB1<6>,

            PD3<6>,
        ],

        <Datin1, PushPull> for [
            #[cfg(feature = "gpio-l45x")]
            PA9<6>,

            PB12<6>,

            PD6<6>,
        ],

        <Datin2, PushPull> for [
            PB14<6>,

            PE7<6>,
        ],

        <Datin3, PushPull> for [
            PC7<6>,

            PE4<6>,
        ],
    }

    #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
    pin! {
        <Ckin4, PushPull> for [
            PC1<6>,

            PE11<6>,
        ],

        <Ckin5, PushPull> for [
            PB7<6>,

            PE13<6>,
        ],

        <Ckin6, PushPull> for [
            PB9<6>,

            PF14<6>,
        ],

        <Ckin7, PushPull> for [
            PB11<6>,

            PD1<6>,
        ],

        <Datin4, PushPull> for [
            PC0<6>,

            PE10<6>,
        ],

        <Datin5, PushPull> for [
            PB6<6>,

            PE12<6>,
        ],

        <Datin6, PushPull> for [
            PB8<6>,

            PF13<6>,
        ],

        <Datin7, PushPull> for [
            PB10<6>,

            PD0<6>,
        ],
    }

    use crate::pac::DFSDM;
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

    #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
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

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
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
            PE6<12>,
        ],

        <A23, PushPull> for [
            PE2<12>,
        ],

        <A24, PushPull> for [
            PG13<12>,
        ],

        <A25, PushPull> for [
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

        <Clk, PushPull> for [
            PD3<12>,
        ],

        <D0, PushPull> for [
            PD14<12>,
        ],

        <D1, PushPull> for [
            PD15<12>,
        ],

        <D10, PushPull> for [
            PE13<12>,
        ],

        <D11, PushPull> for [
            PE14<12>,
        ],

        <D12, PushPull> for [
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

        <D2, PushPull> for [
            PD0<12>,
        ],

        <D3, PushPull> for [
            PD1<12>,
        ],

        <D4, PushPull> for [
            PE7<12>,
        ],

        <D5, PushPull> for [
            PE8<12>,
        ],

        <D6, PushPull> for [
            PE9<12>,
        ],

        <D7, PushPull> for [
            PE10<12>,
        ],

        <D8, PushPull> for [
            PE11<12>,
        ],

        <D9, PushPull> for [
            PE12<12>,
        ],

        <Da0, PushPull> for [
            PD14<12>,
        ],

        <Da1, PushPull> for [
            PD15<12>,
        ],

        <Da10, PushPull> for [
            PE13<12>,
        ],

        <Da11, PushPull> for [
            PE14<12>,
        ],

        <Da12, PushPull> for [
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
            PE9<12>,
        ],

        <Da7, PushPull> for [
            PE10<12>,
        ],

        <Da8, PushPull> for [
            PE11<12>,
        ],

        <Da9, PushPull> for [
            PE12<12>,
        ],

        <Int, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PG7<12>,
        ],

        <Int3, PushPull> for [
            #[cfg(feature = "gpio-l47x")]
            PG7<12>,
        ],

        <Nbl0, PushPull> for [
            PE0<12>,
        ],

        <Nbl1, PushPull> for [
            PE1<12>,
        ],

        <Nce, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PG9<12>,
        ],

        <Nce3, PushPull> for [
            #[cfg(feature = "gpio-l47x")]
            PG9<12>,
        ],

        <Ne1, PushPull> for [
            PD7<12>,
        ],

        <Ne2, PushPull> for [
            PG9<12>,
        ],

        <Ne3, PushPull> for [
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
            PD6<12>,
        ],

        <Nwe, PushPull> for [
            PD5<12>,
        ],
    }
}

pub mod i2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA9<4>,

            PB6<4>,

            PB8<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG14<4>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
            PA10<4>,

            PB7<4>,

            PB9<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG13<4>,
        ],

        <Smba, OpenDrain> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA1<4>,

            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA14<4>,

            PB5<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG15<4>,
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

            PB13<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF1<4>,

            #[cfg(feature = "gpio-l49x")]
            PH4<4>,
        ],

        <Sda, OpenDrain> for [
            PB11<4>,

            PB14<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF0<4>,

            #[cfg(feature = "gpio-l49x")]
            PH5<4>,
        ],

        <Smba, OpenDrain> for [
            PB12<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF2<4>,

            #[cfg(feature = "gpio-l49x")]
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
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA7<4>,

            PC0<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG7<4>,

            #[cfg(feature = "gpio-l49x")]
            PH7<4>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PB4<4>,

            PC1<4>,

            #[cfg(feature = "gpio-l49x")]
            PC9<6>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG8<4>,

            #[cfg(feature = "gpio-l49x")]
            PH8<4>,
        ],

        <Smba, OpenDrain> for [
            PB2<4>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG6<4>,

            #[cfg(feature = "gpio-l49x")]
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

#[cfg(any(feature = "gpio-l45x", feature = "gpio-l49x"))]
pub mod i2c4 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PB6<5>,

            PB10<3>,

            PC0<2>,

            PD12<4>,

            #[cfg(feature = "gpio-l49x")]
            PF14<4>,
        ],

        <Sda, OpenDrain> for [
            PB7<5>,

            PB11<3>,

            PC1<2>,

            PD13<4>,

            #[cfg(feature = "gpio-l49x")]
            PF15<4>,
        ],

        <Smba, OpenDrain> for [
            PA14<5>,

            PD11<4>,

            #[cfg(feature = "gpio-l49x")]
            PF13<4>,
        ],
    }
    use crate::pac::I2C4 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod ir {
    use super::*;

    pin! {
        <Out> default: PushPull for [
            PA13<1>,

            PB9<1>,
        ],
    }
}

#[cfg(any(feature = "gpio-l43x", feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod lcd {
    use super::*;

    pin! {
        <Com0, PushPull> for [
            PA8<11>,
        ],

        <Com1, PushPull> for [
            PA9<11>,
        ],

        <Com2, PushPull> for [
            PA10<11>,
        ],

        <Com3, PushPull> for [
            PB9<11>,
        ],

        <Com4, PushPull> for [
            PC10<11>,
        ],

        <Com5, PushPull> for [
            PC11<11>,
        ],

        <Com6, PushPull> for [
            PC12<11>,
        ],

        <Com7, PushPull> for [
            PD2<11>,
        ],

        <Seg0, PushPull> for [
            PA1<11>,
        ],

        <Seg1, PushPull> for [
            PA2<11>,
        ],

        <Seg10, PushPull> for [
            PB10<11>,
        ],

        <Seg11, PushPull> for [
            PB11<11>,
        ],

        <Seg12, PushPull> for [
            PB12<11>,
        ],

        <Seg13, PushPull> for [
            PB13<11>,
        ],

        <Seg14, PushPull> for [
            PB14<11>,
        ],

        <Seg15, PushPull> for [
            PB15<11>,
        ],

        <Seg16, PushPull> for [
            PB8<11>,
        ],

        <Seg17, PushPull> for [
            PA15<11>,
        ],

        <Seg18, PushPull> for [
            PC0<11>,
        ],

        <Seg19, PushPull> for [
            PC1<11>,
        ],

        <Seg2, PushPull> for [
            PA3<11>,
        ],

        <Seg20, PushPull> for [
            PC2<11>,
        ],

        <Seg21, PushPull> for [
            PB7<11>,
        ],

        <Seg22, PushPull> for [
            PC4<11>,
        ],

        <Seg23, PushPull> for [
            PC5<11>,
        ],

        <Seg24, PushPull> for [
            PC6<11>,
        ],

        <Seg25, PushPull> for [
            PC7<11>,
        ],

        <Seg26, PushPull> for [
            PC8<11>,
        ],

        <Seg27, PushPull> for [
            PC9<11>,
        ],

        <Seg28, PushPull> for [
            PC10<11>,

            PD8<11>,
        ],

        <Seg29, PushPull> for [
            PC11<11>,

            PD9<11>,
        ],

        <Seg3, PushPull> for [
            PA6<11>,
        ],

        <Seg30, PushPull> for [
            PC12<11>,

            PD10<11>,
        ],

        <Seg31, PushPull> for [
            PD2<11>,

            PD11<11>,
        ],

        <Seg32, PushPull> for [
            PD12<11>,
        ],

        <Seg33, PushPull> for [
            PD13<11>,
        ],

        <Seg34, PushPull> for [
            PD14<11>,
        ],

        <Seg35, PushPull> for [
            PD15<11>,
        ],

        <Seg36, PushPull> for [
            PE0<11>,
        ],

        <Seg37, PushPull> for [
            PE1<11>,
        ],

        <Seg38, PushPull> for [
            PE2<11>,
        ],

        <Seg39, PushPull> for [
            PE3<11>,
        ],

        <Seg4, PushPull> for [
            PA7<11>,
        ],

        <Seg40, PushPull> for [
            PC10<11>,
        ],

        <Seg41, PushPull> for [
            PC11<11>,
        ],

        <Seg42, PushPull> for [
            PC12<11>,
        ],

        <Seg43, PushPull> for [
            PD2<11>,
        ],

        <Seg5, PushPull> for [
            PB0<11>,
        ],

        <Seg6, PushPull> for [
            PB1<11>,
        ],

        <Seg7, PushPull> for [
            PB3<11>,
        ],

        <Seg8, PushPull> for [
            PB4<11>,
        ],

        <Seg9, PushPull> for [
            PB5<11>,
        ],

        <Vlcd, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l49x"))]
            PB2<11>,

            PC3<11>,
        ],
    }
}

pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB6<1>,

            PC3<1>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG12<1>,
        ],

        <In1, PushPull> for [
            PB5<1>,

            PC0<1>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG10<1>,
        ],

        <In2, PushPull> for [
            PB7<1>,

            PC2<1>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG11<1>,
        ],
    }

    pin! {
        <Out> default:PushPull for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA14<1>,

            PB2<1>,

            PC1<1>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG15<1>,
        ],
    }
}

pub mod lptim2 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PA5<14>,

            PC3<14>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD11<14>,
        ],

        <In1, PushPull> for [
            PB1<14>,

            PC0<14>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD12<14>,
        ],
    }

    pin! {
        <Out> default:PushPull for [
            PA4<14>,

            PA8<14>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD13<14>,
        ],
    }
}

pub mod lpuart1 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA6<8>,

            PB13<8>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG5<8>,
        ],

        <De, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PB1<8>,

            PB12<8>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG6<8>,
        ],

        <Rts, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PB1<8>,

            PB12<8>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG6<8>,
        ],

        <Rx, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA3<8>,

            PB10<8>,

            PC0<8>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG8<8>,
        ],
    }

    pin! {
        <Tx> default:PushPull for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA2<8>,

            PB11<8>,

            PC1<8>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG7<8>,
        ],
    }
}

pub mod quadspi {
    use super::*;

    pin! {
        <Bk1Io0, PushPull> for [
            PB1<10>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE12<10>,

            #[cfg(feature = "gpio-l49x")]
            PF8<10>,
        ],

        <Bk1Io1, PushPull> for [
            PB0<10>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE13<10>,

            #[cfg(feature = "gpio-l49x")]
            PF9<10>,
        ],

        <Bk1Io2, PushPull> for [
            PA7<10>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE14<10>,

            #[cfg(feature = "gpio-l49x")]
            PF7<10>,
        ],

        <Bk1Io3, PushPull> for [
            PA6<10>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE15<10>,

            #[cfg(feature = "gpio-l49x")]
            PF6<10>,
        ],

        <Bk1Ncs, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA2<10>,

            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PB11<10>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PE11<10>,
        ],

        <Bk2Io0, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PC1<10>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PD4<10>,

            #[cfg(feature = "gpio-l49x")]
            PH2<3>,
        ],

        <Bk2Io1, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PC2<10>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PD5<10>,

            #[cfg(feature = "gpio-l49x")]
            PD6<5>,
        ],

        <Bk2Io2, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PC3<10>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PD6<10>,
        ],

        <Bk2Io3, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PC4<10>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PD7<10>,
        ],

        <Bk2Ncs, PushPull> for [
            #[cfg(feature = "gpio-l49x")]
            PC11<5>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PD3<10>,
        ],

        <Clk, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA3<10>,

            PB10<10>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE10<10>,

            #[cfg(feature = "gpio-l49x")]
            PF10<3>,
        ],

        <Ncs, PushPull> for [
            #[cfg(feature = "gpio-l47x")]
            PB11<10>,

            #[cfg(feature = "gpio-l47x")]
            PE11<10>,
        ],
    }
}

pub mod rcc {
    use super::*;

    pin! {
        <Mco, PushPull> for [
            PA8<0>,
        ],
    }
}

pub mod rtc {
    use super::*;

    pin! {
        <Out2, PushPull> for [
            #[cfg(feature = "gpio-l41x")]
            PB2<0>,
        ],

        <OutAlarm, PushPull> for [
            PB2<0>,
        ],

        <OutCalib, PushPull> for [
            PB2<0>,
        ],

        <Refin, PushPull> for [
            PB15<0>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-l43x",
    feature = "gpio-l45x",
    feature = "gpio-l47x",
    feature = "gpio-l49x"
))]
pub mod sai1 {
    use super::*;

    pin! {
        <Extclk, PushPull> for [
            PA0<13>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PB0<13>,
        ],

        <FsA, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PA9<13>,

            PB9<13>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x"))]
            PB12<13>,

            PE4<13>,
        ],

        <FsB, PushPull> for [
            PA4<13>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PA14<13>,

            PB6<13>,

            PE9<13>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF9<13>,
        ],

        <MclkA, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PA3<13>,

            PB8<13>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x"))]
            PB14<13>,

            PE2<13>,

            #[cfg(feature = "gpio-l49x")]
            PG7<13>,
        ],

        <MclkB, PushPull> for [
            PB4<13>,

            PE10<13>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF7<13>,
        ],

        <SckA, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PA8<13>,

            PB10<13>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x"))]
            PB13<13>,

            PE5<13>,
        ],

        <SckB, PushPull> for [
            PB3<13>,

            PE8<13>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF8<13>,
        ],

        <SdA, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PA10<13>,

            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x"))]
            PB15<13>,

            #[cfg(feature = "gpio-l49x")]
            PC1<13>,

            PC3<13>,

            PD6<13>,

            PE6<13>,
        ],

        <SdB, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PA13<13>,

            PB5<13>,

            PE3<13>,

            PE7<13>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF6<13>,
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

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod sai2 {
    use super::*;

    pin! {
        <Extclk, PushPull> for [
            PA2<13>,

            PC9<13>,
        ],

        <FsA, PushPull> for [
            PB12<13>,

            PD12<13>,

            PG10<13>,
        ],

        <FsB, PushPull> for [
            PA15<13>,

            PG3<13>,
        ],

        <MclkA, PushPull> for [
            PB14<13>,

            PC6<13>,

            PD9<13>,

            PG11<13>,
        ],

        <MclkB, PushPull> for [
            PC7<13>,

            PC11<13>,

            PG4<13>,
        ],

        <SckA, PushPull> for [
            PB13<13>,

            PD10<13>,

            PG9<13>,
        ],

        <SckB, PushPull> for [
            PC10<13>,

            PG2<13>,
        ],

        <SdA, PushPull> for [
            PB15<13>,

            PD11<13>,

            PG12<13>,
        ],

        <SdB, PushPull> for [
            PC12<13>,

            PG5<13>,
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

#[cfg(any(
    feature = "gpio-l43x",
    feature = "gpio-l45x",
    feature = "gpio-l47x",
    feature = "gpio-l49x"
))]
pub mod sdmmc1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC12<12>,
        ],

        <Cmd, PushPull> for [
            PD2<12>,
        ],

        <D0, PushPull> for [
            PC8<12>,
        ],

        <D1, PushPull> for [
            PC9<12>,
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

pub mod spi1 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PA6<5>,

            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA11<5>,

            PB4<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE14<5>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG3<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA12<5>,

            PB5<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE15<5>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG4<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,

            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PB0<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE12<5>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG5<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA1<5>,

            PA5<5>,

            PB3<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE13<5>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG2<5>,
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

            PC2<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD3<5>,

            #[cfg(feature = "gpio-l49x")]
            PI2<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB15<5>,

            #[cfg(feature = "gpio-l49x")]
            PC1<3>,

            PC3<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD4<5>,

            #[cfg(feature = "gpio-l49x")]
            PI3<5>,
        ],

        <Nss, PushPull> for [
            PB9<5>,

            PB12<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD0<5>,

            #[cfg(feature = "gpio-l49x")]
            PI0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-l49x")]
            PA9<3>,

            PB10<5>,

            PB13<5>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD1<5>,

            #[cfg(feature = "gpio-l49x")]
            PD3<3>,

            #[cfg(feature = "gpio-l49x")]
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

#[cfg(any(
    feature = "gpio-l43x",
    feature = "gpio-l45x",
    feature = "gpio-l47x",
    feature = "gpio-l49x"
))]
pub mod spi3 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB4<6>,

            PC11<6>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG10<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            PB5<6>,

            PC12<6>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG11<6>,
        ],

        <Nss, PushPull> for [
            PA4<6>,

            PA15<6>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG12<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB3<6>,

            PC10<6>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG9<6>,
        ],
    }
    impl SpiCommon for crate::pac::SPI3 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(any(feature = "gpio-l43x", feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod swpmi1 {
    use super::*;

    pin! {
        <Io, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l49x"))]
            PA8<12>,

            PB12<12>,
        ],

        <Rx, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l49x"))]
            PA14<12>,

            PB14<12>,
        ],

        <Suspend, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l49x"))]
            PA15<12>,

            PB15<12>,
        ],

        <Tx, PushPull> for [
            #[cfg(any(feature = "gpio-l43x", feature = "gpio-l49x"))]
            PA13<12>,

            PB13<12>,
        ],
    }
}

pub mod sys {
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

        <Jtrst, PushPull> for [
            PB4<0>,
        ],

        <Traceclk, PushPull> for [
            #[cfg(feature = "gpio-l41x")]
            PB7<0>,

            #[cfg(feature = "gpio-l41x")]
            PC0<0>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE2<0>,
        ],

        <Traced0, PushPull> for [
            #[cfg(feature = "gpio-l41x")]
            PB0<0>,

            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PC1<0>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE3<0>,
        ],

        <Traced1, PushPull> for [
            #[cfg(feature = "gpio-l41x")]
            PB1<0>,

            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PC10<0>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE4<0>,
        ],

        <Traced2, PushPull> for [
            #[cfg(feature = "gpio-l41x")]
            PB5<0>,

            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PD2<0>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE5<0>,
        ],

        <Traced3, PushPull> for [
            #[cfg(feature = "gpio-l41x")]
            PB6<0>,

            #[cfg(any(feature = "gpio-l41x", feature = "gpio-l45x", feature = "gpio-l49x"))]
            PC12<0>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE6<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA8<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE9<1>,
        ],

        <Ch1N> default:PushPull for [
            PA7<1>,

            PB13<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE8<1>,
        ],

        <Ch2> default:PushPull for [
            PA9<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE11<1>,
        ],

        <Ch2N> default:PushPull for [
            PB0<1>,

            PB14<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE10<1>,
        ],

        <Ch3> default:PushPull for [
            PA10<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE13<1>,
        ],

        <Ch3N> default:PushPull for [
            PB1<1>,

            PB15<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE12<1>,
        ],

        <Ch4> default:PushPull for [
            PA11<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE14<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<1>,

            PB12<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE15<1>,
        ],

        <Bkin2, PushPull> for [
            PA11<2>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE14<2>,
        ],

        <Bkin2Comp1, PushPull> for [
            PA11<12>,
        ],

        <Bkin2Comp2, PushPull> for [
            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE14<3>,
        ],

        <BkinComp1, PushPull> for [
            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE15<3>,
        ],

        <BkinComp2, PushPull> for [
            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PA6<12>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PB12<3>,
        ],

        <Etr, PushPull> for [
            PA12<1>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE7<1>,
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
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<2> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<3> for TIM {
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
            PA0<14>,

            PA5<2>,

            PA15<2>,
        ],
    }

    use crate::pac::TIM2 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(any(feature = "gpio-l45x", feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA6<2>,

            PB4<2>,

            PC6<2>,

            PE3<2>,
        ],

        <Ch2> default:PushPull for [
            PA7<2>,

            PB5<2>,

            PC7<2>,

            PE4<2>,
        ],

        <Ch3> default:PushPull for [
            PB0<2>,

            PC8<2>,

            PE5<2>,
        ],

        <Ch4> default:PushPull for [
            PB1<2>,

            PC9<2>,

            PE6<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PD2<2>,

            PE2<2>,
        ],
    }

    use crate::pac::TIM3 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
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
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA0<2>,

            PF6<2>,

            #[cfg(feature = "gpio-l49x")]
            PH10<2>,
        ],

        <Ch2> default:PushPull for [
            PA1<2>,

            PF7<2>,

            #[cfg(feature = "gpio-l49x")]
            PH11<2>,
        ],

        <Ch3> default:PushPull for [
            PA2<2>,

            PF8<2>,

            #[cfg(feature = "gpio-l49x")]
            PH12<2>,
        ],

        <Ch4> default:PushPull for [
            PA3<2>,

            PF9<2>,

            #[cfg(feature = "gpio-l49x")]
            PI0<2>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PF6<1>,
        ],
    }

    use crate::pac::TIM5 as TIM;
    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod tim8 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PC6<3>,

            #[cfg(feature = "gpio-l49x")]
            PI5<3>,
        ],

        <Ch1N> default:PushPull for [
            PA5<3>,

            PA7<3>,

            #[cfg(feature = "gpio-l49x")]
            PH13<3>,
        ],

        <Ch2> default:PushPull for [
            PC7<3>,

            #[cfg(feature = "gpio-l49x")]
            PI6<3>,
        ],

        <Ch2N> default:PushPull for [
            PB0<3>,

            PB14<3>,

            #[cfg(feature = "gpio-l49x")]
            PH14<3>,
        ],

        <Ch3> default:PushPull for [
            PC8<3>,

            #[cfg(feature = "gpio-l49x")]
            PI7<3>,
        ],

        <Ch3N> default:PushPull for [
            PB1<3>,

            PB15<3>,

            #[cfg(feature = "gpio-l49x")]
            PH15<3>,
        ],

        <Ch4> default:PushPull for [
            PC9<3>,

            #[cfg(feature = "gpio-l49x")]
            PI2<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<3>,

            PB7<3>,

            #[cfg(feature = "gpio-l49x")]
            PI4<3>,
        ],

        <Bkin2, PushPull> for [
            PB6<3>,

            PC9<1>,
        ],

        <Bkin2Comp1, PushPull> for [
            PC9<14>,
        ],

        <Bkin2Comp2, PushPull> for [
            PB6<12>,
        ],

        <BkinComp1, PushPull> for [
            PB7<13>,
        ],

        <BkinComp2, PushPull> for [
            PA6<13>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<3>,

            #[cfg(feature = "gpio-l49x")]
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
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimNCPin<2> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim15 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA2<14>,

            PB14<14>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF9<14>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG10<14>,
        ],

        <Ch1N> default:PushPull for [
            PA1<14>,

            PB13<14>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG9<14>,
        ],

        <Ch2> default:PushPull for [
            PA3<14>,

            PB15<14>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PF10<14>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG11<14>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA9<14>,

            PB12<14>,
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
            PA6<14>,

            PB8<14>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PE0<14>,
        ],

        <Ch1N> default:PushPull for [
            PB6<14>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PB5<14>,
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

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod tim17 {
    use super::*;

    pin! {
        <Ch1> default:PushPull for [
            PA7<14>,

            PB9<14>,

            PE1<14>,
        ],

        <Ch1N> default:PushPull for [
            PB7<14>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA10<14>,

            PB4<14>,
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

pub mod tsc {
    use super::*;

    pin! {
        <Sync, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PB10<9>,

            PD2<9>,
        ],
    }

    pin! {
        <G1Io1> default:PushPull for [
            PB12<9>,
        ],

        <G1Io2> default:PushPull for [
            PB13<9>,
        ],

        <G1Io3> default:PushPull for [
            PB14<9>,
        ],

        <G1Io4> default:PushPull for [
            PB15<9>,
        ],

        <G2Io1> default:PushPull for [
            PB4<9>,
        ],

        <G2Io2> default:PushPull for [
            PB5<9>,
        ],

        <G2Io3> default:PushPull for [
            PB6<9>,
        ],

        <G2Io4> default:PushPull for [
            PB7<9>,
        ],

        <G3Io1> default:PushPull for [
            PA15<9>,
        ],

        <G3Io2> default:PushPull for [
            PC10<9>,
        ],

        <G3Io3> default:PushPull for [
            PC11<9>,
        ],

        <G3Io4> default:PushPull for [
            PC12<9>,
        ],

        <G4Io1> default:PushPull for [
            PC6<9>,
        ],

        <G4Io2> default:PushPull for [
            PC7<9>,
        ],

        <G4Io3> default:PushPull for [
            PC8<9>,
        ],

        <G4Io4> default:PushPull for [
            PC9<9>,
        ],
    }

    #[cfg(any(
        feature = "gpio-l43x",
        feature = "gpio-l45x",
        feature = "gpio-l47x",
        feature = "gpio-l49x"
    ))]
    pin! {
        <G5Io1> default:PushPull for [
            PE10<9>,
        ],

        <G5Io2> default:PushPull for [
            PE11<9>,
        ],

        <G5Io3> default:PushPull for [
            PE12<9>,
        ],

        <G5Io4> default:PushPull for [
            PE13<9>,
        ],

        <G6Io1> default:PushPull for [
            PD10<9>,
        ],

        <G6Io2> default:PushPull for [
            PD11<9>,
        ],

        <G6Io3> default:PushPull for [
            PD12<9>,
        ],

        <G6Io4> default:PushPull for [
            PD13<9>,
        ],

        <G7Io1> default:PushPull for [
            PE2<9>,
        ],

        <G7Io2> default:PushPull for [
            PE3<9>,
        ],

        <G7Io3> default:PushPull for [
            PE4<9>,
        ],

        <G7Io4> default:PushPull for [
            PE5<9>,
        ],
    }

    #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
    pin! {
        <G8Io1> default:PushPull for [
            PF14<9>,
        ],

        <G8Io2> default:PushPull for [
            PF15<9>,
        ],

        <G8Io3> default:PushPull for [
            PG0<9>,
        ],

        <G8Io4> default:PushPull for [
            PG1<9>,
        ],
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<7>,

            PB5<7>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG13<7>,
        ],

        <Cts, PushPull> for [
            PA11<7>,

            PB4<7>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG11<7>,
        ],

        <De, PushPull> for [
            PA12<7>,

            PB3<7>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG12<7>,
        ],

        <Rts, PushPull> for [
            PA12<7>,

            PB3<7>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA10<7>,

            PB7<7>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG10<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA9<7>,

            PB6<7>,

            #[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
            PG9<7>,
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

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD7<7>,
        ],

        <Cts, PushPull> for [
            PA0<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD3<7>,
        ],

        <De, PushPull> for [
            PA1<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD4<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD4<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA3<7>,

            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA15<3>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD6<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA2<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
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
            PB0<7>,

            PB12<7>,

            PC12<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD10<7>,
        ],

        <Cts, PushPull> for [
            PA6<7>,

            PB13<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD11<7>,
        ],

        <De, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA15<7>,

            PB1<7>,

            PB14<7>,

            PD2<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD12<7>,
        ],

        <Rts, PushPull> for [
            #[cfg(any(
                feature = "gpio-l41x",
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l49x"
            ))]
            PA15<7>,

            PB1<7>,

            PB14<7>,

            PD2<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD12<7>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PB11<7>,

            PC5<7>,

            PC11<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
            PD9<7>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PB10<7>,

            PC4<7>,

            PC10<7>,

            #[cfg(any(
                feature = "gpio-l43x",
                feature = "gpio-l45x",
                feature = "gpio-l47x",
                feature = "gpio-l49x"
            ))]
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

#[cfg(any(feature = "gpio-l45x", feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod uart4 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB7<8>,
        ],

        <De, PushPull> for [
            PA15<8>,
        ],

        <Rts, PushPull> for [
            PA15<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PA1<8>,

            PC11<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
            PA0<8>,

            PC10<8>,
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

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
pub mod uart5 {
    use super::*;

    pin! {
        <Cts, PushPull> for [
            PB5<8>,
        ],

        <De, PushPull> for [
            PB4<8>,
        ],

        <Rts, PushPull> for [
            PB4<8>,
        ],
    }

    pin! {
        <Rx> default:PushPull for no:NoPin, [
            PD2<8>,
        ],

        <Tx> default:PushPull for no:NoPin, [
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

#[cfg(any(feature = "gpio-l41x", feature = "gpio-l43x", feature = "gpio-l45x"))]
pub mod usb {
    use super::*;

    pin! {
        <Dm, PushPull> for [
            PA11<10>,
        ],

        <Dp, PushPull> for [
            PA12<10>,
        ],

        <Noe, PushPull> for [
            PA13<10>,

            PC9<10>,
        ],
    }
}

#[cfg(any(feature = "gpio-l47x", feature = "gpio-l49x"))]
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

        <Noe, PushPull> for [
            PA13<10>,

            PC9<10>,
        ],

        <Sof, PushPull> for [
            PA8<10>,

            #[cfg(feature = "gpio-l49x")]
            PA14<10>,
        ],
    }
}
