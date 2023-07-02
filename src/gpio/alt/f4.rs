use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

#[cfg(feature = "can1")]
pub mod can1 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PA11<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB8<8>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PB8<9>,

            PD0<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG0<9>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI9<9>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PA12<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB9<8>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PB9<9>,

            PD1<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG1<9>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH13<9>,
        ],
    }

    impl CanCommon for crate::pac::CAN1 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

#[cfg(feature = "can2")]
pub mod can2 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PB5<9>,

            PB12<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG11<9>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PB6<9>,

            PB13<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PG12<9>,
        ],
    }

    impl CanCommon for crate::pac::CAN2 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

#[cfg(feature = "can3")]
pub mod can3 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            PA8<11>,

            PB3<11>,
        ],

        <Tx, PushPull> for no:NoPin, [
            PA15<11>,

            PB4<11>,
        ],
    }

    impl CanCommon for crate::pac::CAN3 {
        type Rx = Rx;
        type Tx = Tx;
    }
}

#[cfg(any(
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
pub mod dcmi {
    use super::*;

    pin! {
        <D0, PushPull> for [
            PA9<13>,

            PC6<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH9<13>,
        ],

        <D1, PushPull> for [
            PA10<13>,

            PC7<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH10<13>,
        ],

        <D10, PushPull> for [
            PB5<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PD6<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI3<13>,
        ],

        <D11, PushPull> for [
            PD2<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PF10<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH15<13>,
        ],

        <D12, PushPull> for [
            PF11<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PG6<13>,
        ],

        <D13, PushPull> for [
            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PG7<13>,

            PG15<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI0<13>,
        ],

        <D2, PushPull> for [
            PC8<13>,

            PE0<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PG10<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH11<13>,
        ],

        <D3, PushPull> for [
            PC9<13>,

            PE1<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PG11<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH12<13>,
        ],

        <D4, PushPull> for [
            PC11<13>,

            PE4<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH14<13>,
        ],

        <D5, PushPull> for [
            PB6<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PD3<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI4<13>,
        ],

        <D6, PushPull> for [
            PB8<13>,

            PE5<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI6<13>,
        ],

        <D7, PushPull> for [
            PB9<13>,

            PE6<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI7<13>,
        ],

        <D8, PushPull> for [
            PC10<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH6<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI1<13>,
        ],

        <D9, PushPull> for [
            PC12<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH7<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI2<13>,
        ],

        <Hsync, PushPull> for [
            PA4<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH8<13>,
        ],

        <Pixclk, PushPull> for [
            PA6<13>,
        ],

        <Vsync, PushPull> for [
            PB7<13>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PG9<13>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI5<13>,
        ],
    }
}

#[cfg(feature = "dfsdm1")]
pub mod dfsdm1 {
    use super::*;

    pin! {
        <Ckin0, PushPull> for [
            PB2<6>,

            PD4<6>,
        ],

        <Ckin1, PushPull> for [
            PA5<8>,

            PB13<10>,

            PD7<6>,
        ],

        <Ckin2, PushPull> for [
            PB15<8>,

            PE8<6>,
        ],

        <Ckin3, PushPull> for [
            PC6<6>,

            PE5<8>,
        ],

        <Ckout, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PA8<6>,

            PC2<8>,

            PE9<6>,
        ],

        <Datin0, PushPull> for [
            PB1<8>,

            PD3<6>,
        ],

        <Datin1, PushPull> for [
            PA4<8>,

            PB12<10>,

            PD6<6>,
        ],

        <Datin2, PushPull> for [
            PB14<8>,

            PE7<6>,
        ],

        <Datin3, PushPull> for [
            PC7<10>,

            PE4<8>,
        ],
    }

    #[cfg(feature = "gpio-f412")]
    use crate::pac::DFSDM;
    #[cfg(feature = "gpio-f413")]
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
}

#[cfg(feature = "dfsdm2")]
pub mod dfsdm2 {
    use super::*;

    pin! {
        <Ckin0, PushPull> for [
            PD14<10>,

            PE11<3>,
        ],

        <Ckin1, PushPull> for [
            PA6<7>,

            PB8<7>,

            PE15<10>,
        ],

        <Ckin2, PushPull> for [
            PC4<3>,

            PD12<3>,
        ],

        <Ckin3, PushPull> for [
            PA9<3>,

            PC8<7>,
        ],

        <Ckin4, PushPull> for [
            PC0<3>,

            PE0<3>,
        ],

        <Ckin5, PushPull> for [
            PA11<3>,

            PC10<3>,
        ],

        <Ckin6, PushPull> for [
            PC7<7>,

            PD0<3>,
        ],

        <Ckin7, PushPull> for [
            PB6<6>,

            PC3<3>,

            PE13<3>,
        ],

        <Ckout, PushPull> for [
            PB10<10>,

            PD2<3>,

            PD5<3>,
        ],

        <Datin0, PushPull> for [
            PD15<10>,

            PE10<3>,
        ],

        <Datin1, PushPull> for [
            PA7<7>,

            PB9<6>,

            PE14<10>,
        ],

        <Datin2, PushPull> for [
            PC5<3>,

            PD11<3>,
        ],

        <Datin3, PushPull> for [
            PA10<3>,

            PC9<7>,
        ],

        <Datin4, PushPull> for [
            PC1<3>,

            PE1<3>,
        ],

        <Datin5, PushPull> for [
            PA12<3>,

            PC11<3>,
        ],

        <Datin6, PushPull> for [
            PC6<7>,

            PD1<3>,
        ],

        <Datin7, PushPull> for [
            PB7<6>,

            PC2<3>,

            PE12<3>,
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

#[cfg(feature = "dsihost")]
pub mod dsihost {
    use super::*;

    pin! {
        <Te, PushPull> for [
            PB11<13>,

            PJ2<13>,
        ],
    }
}

#[cfg(feature = "eth")]
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

        <TxClk, PushPull> for [
            PC3<11>,
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

            PG14<11>,
        ],

        <Txd2, PushPull> for [
            PC2<11>,
        ],

        <Txd3, PushPull> for [
            PB8<11>,

            PE2<11>,
        ],
    }
}

/// Pins available on all STM32F4 models that have an FSMC/FMC
#[cfg(any(feature = "fmc", feature = "fsmc"))]
pub mod fsmc {
    use super::*;

    pub use Ne1 as ChipSelect1;
    pub use Ne2 as ChipSelect2;
    pub use Ne3 as ChipSelect3;
    pub use Ne4 as ChipSelect4;
    pub use Noe as ReadEnable;
    pub use Nwe as WriteEnable;

    // TODO: replace this with `Ax`
    pin! {
        /// A pin that can be used as one bit of the memory address
        ///
        /// This is used to switch between data and command mode.
        <Address, PushPull> for [
            PD11<12, Speed::VeryHigh>,
            PD12<12, Speed::VeryHigh>,
            PD13<12, Speed::VeryHigh>,
            PE2<12, Speed::VeryHigh>,
            PE3<12, Speed::VeryHigh>,
            PE4<12, Speed::VeryHigh>,
            PE5<12, Speed::VeryHigh>,
            PE6<12, Speed::VeryHigh>,
            PF0<12, Speed::VeryHigh>,
            PF1<12, Speed::VeryHigh>,
            PF2<12, Speed::VeryHigh>,
            PF3<12, Speed::VeryHigh>,
            PF4<12, Speed::VeryHigh>,
            PF5<12, Speed::VeryHigh>,
            PF12<12, Speed::VeryHigh>,
            PF13<12, Speed::VeryHigh>,
            PF14<12, Speed::VeryHigh>,
            PF15<12, Speed::VeryHigh>,
            PG0<12, Speed::VeryHigh>,
            PG1<12, Speed::VeryHigh>,
            PG2<12, Speed::VeryHigh>,
            PG3<12, Speed::VeryHigh>,
            PG4<12, Speed::VeryHigh>,
            PG5<12, Speed::VeryHigh>,
            PG13<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC3<12, Speed::VeryHigh>,
        ],
    }

    pin! {
        <A0, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC3<12, Speed::VeryHigh>,

            PF0<12, Speed::VeryHigh>,
        ],

        <A1, PushPull> for [
            PF1<12, Speed::VeryHigh>,
        ],

        <A2, PushPull> for [
            PF2<12, Speed::VeryHigh>,
        ],

        <A3, PushPull> for [
            PF3<12, Speed::VeryHigh>,
        ],

        <A4, PushPull> for [
            PF4<12, Speed::VeryHigh>,
        ],

        <A5, PushPull> for [
            PF5<12, Speed::VeryHigh>,
        ],

        <A6, PushPull> for [
            PF12<12, Speed::VeryHigh>,
        ],

        <A7, PushPull> for [
            PF13<12, Speed::VeryHigh>,
        ],

        <A8, PushPull> for [
            PF14<12, Speed::VeryHigh>,
        ],

        <A9, PushPull> for [
            PF15<12, Speed::VeryHigh>,
        ],

        <A10, PushPull> for [
            PG0<12, Speed::VeryHigh>,
        ],

        <A11, PushPull> for [
            PG1<12, Speed::VeryHigh>,
        ],

        <A12, PushPull> for [
            PG2<12, Speed::VeryHigh>,
        ],

        <A13, PushPull> for [
            PG3<12, Speed::VeryHigh>,
        ],

        <A14, PushPull> for [
            PG4<12, Speed::VeryHigh>,
        ],

        <A15, PushPull> for [
            PG5<12, Speed::VeryHigh>,
        ],

        <A16, PushPull> for [
            PD11<12, Speed::VeryHigh>,
        ],

        <A17, PushPull> for [
            PD12<12, Speed::VeryHigh>,
        ],

        <A18, PushPull> for [
            PD13<12, Speed::VeryHigh>,
        ],

        <A19, PushPull> for [
            PE3<12, Speed::VeryHigh>,
        ],

        <A20, PushPull> for [
            PE4<12, Speed::VeryHigh>,
        ],

        <A21, PushPull> for [
            PE5<12, Speed::VeryHigh>,
        ],

        <A22, PushPull> for [
            PE6<12, Speed::VeryHigh>,
        ],

        <A23, PushPull> for [
            PE2<12, Speed::VeryHigh>,
        ],

        <A24, PushPull> for [
            PG13<12, Speed::VeryHigh>,
        ],

        <A25, PushPull> for [
            PG14<12, Speed::VeryHigh>,
        ],

        <Clk, PushPull> for [
            PD3<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 0
        <D0, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB14<10, Speed::VeryHigh>,

            PD14<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 1
        <D1, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC6<10, Speed::VeryHigh>,

            PD15<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 2
        <D2, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC11<10, Speed::VeryHigh>,

            PD0<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 3
        <D3, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC12<10, Speed::VeryHigh>,

            PD1<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 4
        <D4, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA2<12, Speed::VeryHigh>,

            PE7<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 5
        <D5, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA3<12, Speed::VeryHigh>,

            PE8<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 6
        <D6, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA4<12, Speed::VeryHigh>,

            PE9<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 7
        <D7, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA5<12, Speed::VeryHigh>,

            PE10<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 8
        <D8, PushPull> for [
            PE11<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 9
        <D9, PushPull> for [
            PE12<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 10
        <D10, PushPull> for [
            PE13<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 11
        <D11, PushPull> for [
            PE14<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 12
        <D12, PushPull> for [
            PE15<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 13
        <D13, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB12<12, Speed::VeryHigh>,

            PD8<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 14
        <D14, PushPull> for [
            PD9<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for data bus 15
        <D15, PushPull> for [
            PD10<12, Speed::VeryHigh>,
        ],

        <Da0, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB14<10, Speed::VeryHigh>,

            PD14<12, Speed::VeryHigh>,
        ],

        <Da1, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC6<10, Speed::VeryHigh>,

            PD15<12, Speed::VeryHigh>,
        ],

        <Da2, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC11<10, Speed::VeryHigh>,

            PD0<12, Speed::VeryHigh>,
        ],

        <Da3, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC12<10, Speed::VeryHigh>,

            PD1<12, Speed::VeryHigh>,
        ],

        <Da4, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA2<12, Speed::VeryHigh>,

            PE7<12, Speed::VeryHigh>,
        ],

        <Da5, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA3<12, Speed::VeryHigh>,

            PE8<12, Speed::VeryHigh>,
        ],

        <Da6, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA4<12, Speed::VeryHigh>,

            PE9<12, Speed::VeryHigh>,
        ],

        <Da7, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA5<12, Speed::VeryHigh>,

            PE10<12, Speed::VeryHigh>,
        ],

        <Da8, PushPull> for [
            PE11<12, Speed::VeryHigh>,
        ],

        <Da9, PushPull> for [
            PE12<12, Speed::VeryHigh>,
        ],

        <Da10, PushPull> for [
            PE13<12, Speed::VeryHigh>,
        ],

        <Da11, PushPull> for [
            PE14<12, Speed::VeryHigh>,
        ],

        <Da12, PushPull> for [
            PE15<12, Speed::VeryHigh>,
        ],

        <Da13, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB12<12, Speed::VeryHigh>,

            PD8<12, Speed::VeryHigh>,
        ],

        <Da14, PushPull> for [
            PD9<12, Speed::VeryHigh>,
        ],

        <Da15, PushPull> for [
            PD10<12, Speed::VeryHigh>,
        ],

        <Nbl0, PushPull> for [
            PE0<12, Speed::VeryHigh>,
        ],

        <Nbl1, PushPull> for [
            PE1<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 1
        <Ne1, PushPull> for [
            PD7<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 2
        <Ne2, PushPull> for [
            PG9<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 3
        <Ne3, PushPull> for [
            PG10<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used to enable a memory device on sub-bank 4
        <Ne4, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC4<12, Speed::VeryHigh>,

            PG12<12, Speed::VeryHigh>,
        ],

        <Nl, PushPull> for [
            PB7<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for the output enable (read enable, NOE) signal
        <Noe, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC5<12, Speed::VeryHigh>,

            PD4<12, Speed::VeryHigh>,
        ],

        <Nwait, PushPull> for [
            PD6<12, Speed::VeryHigh>,
        ],

        /// A pin that can be used for the write enable (NOE) signal
        <Nwe, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC2<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PD2<10, Speed::VeryHigh>,

            PD5<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
    pin! {
        <Ba0, PushPull> for [
            PG4<12, Speed::VeryHigh>,
        ],

        <Ba1, PushPull> for [
            PG5<12, Speed::VeryHigh>,
        ],

        <Sdcke0, PushPull> for [
            PC3<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f446", feature = "gpio-f469"))]
            PC5<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH2<12, Speed::VeryHigh>,
        ],

        <Sdcke1, PushPull> for [
            PB5<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH7<12, Speed::VeryHigh>,
        ],

        <Sdclk, PushPull> for [
            PG8<12, Speed::VeryHigh>,
        ],

        <Sdncas, PushPull> for [
            PG15<12, Speed::VeryHigh>,
        ],

        <Sdne0, PushPull> for [
            PC2<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f446", feature = "gpio-f469"))]
            PC4<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH3<12, Speed::VeryHigh>,
        ],

        <Sdne1, PushPull> for [
            PB6<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH6<12, Speed::VeryHigh>,
        ],

        <Sdnras, PushPull> for [
            PF11<12, Speed::VeryHigh>,
        ],

        <Sdnwe, PushPull> for [
            #[cfg(any(feature = "gpio-f446", feature = "gpio-f469"))]
            PA7<12, Speed::VeryHigh>,

            PC0<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH5<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(any(feature = "gpio-f417", feature = "gpio-f427"))]
    pin! {
        <Cd, PushPull> for [
            PF9<12, Speed::VeryHigh>,
        ],

        <Int2, PushPull> for [
            PG6<12, Speed::VeryHigh>,
        ],

        <Intr, PushPull> for [
            PF10<12, Speed::VeryHigh>,
        ],

        <Nce2, PushPull> for [
            PD7<12, Speed::VeryHigh>,
        ],

        <Nce41, PushPull> for [
            PG10<12, Speed::VeryHigh>,
        ],

        <Nce42, PushPull> for [
            PG11<12, Speed::VeryHigh>,
        ],

        <Nreg, PushPull> for [
            PF7<12, Speed::VeryHigh>,
        ],

        <Niord, PushPull> for [
            PF6<12, Speed::VeryHigh>,
        ],

        <Niowr, PushPull> for [
            PF8<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
    pin! {
        <Ale, PushPull> for [
            PD12<12, Speed::VeryHigh>,
        ],

        <Cle, PushPull> for [
            PD11<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(feature = "gpio-f469")]
    pin! {
        <Int, PushPull> for [
            PG7<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f446"))]
    pin! {
        <Int3, PushPull> for [
            PG7<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(any(
        feature = "gpio-f417",
        feature = "gpio-f427",
        feature = "gpio-f446",
        feature = "gpio-f469"
    ))]
    pin! {
        <Nce3, PushPull> for [
            PG9<12, Speed::VeryHigh>,
        ],
    }

    #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
    pin! {
        <D16, PushPull> for [
            PH8<12, Speed::VeryHigh>,
        ],

        <D17, PushPull> for [
            PH9<12, Speed::VeryHigh>,
        ],

        <D18, PushPull> for [
            PH10<12, Speed::VeryHigh>,
        ],

        <D19, PushPull> for [
            PH11<12, Speed::VeryHigh>,
        ],

        <D20, PushPull> for [
            PH12<12, Speed::VeryHigh>,
        ],

        <D21, PushPull> for [
            PH13<12, Speed::VeryHigh>,
        ],

        <D22, PushPull> for [
            PH14<12, Speed::VeryHigh>,
        ],

        <D23, PushPull> for [
            PH15<12, Speed::VeryHigh>,
        ],

        <D24, PushPull> for [
            PI0<12, Speed::VeryHigh>,
        ],

        <D25, PushPull> for [
            PI1<12, Speed::VeryHigh>,
        ],

        <D26, PushPull> for [
            PI2<12, Speed::VeryHigh>,
        ],

        <D27, PushPull> for [
            PI3<12, Speed::VeryHigh>,
        ],

        <D28, PushPull> for [
            PI6<12, Speed::VeryHigh>,
        ],

        <D29, PushPull> for [
            PI7<12, Speed::VeryHigh>,
        ],

        <D30, PushPull> for [
            PI9<12, Speed::VeryHigh>,
        ],

        <D31, PushPull> for [
            PI10<12, Speed::VeryHigh>,
        ],

        <Nbl2, PushPull> for [
            PI4<12, Speed::VeryHigh>,
        ],

        <Nbl3, PushPull> for [
            PI5<12, Speed::VeryHigh>,
        ],
    }
}

#[cfg(feature = "fmpi2c1")]
pub mod fmpi2c1 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            #[cfg(feature = "gpio-f410")]
            PA8<4>,

            #[cfg(any(feature = "gpio-f410", feature = "gpio-f412", feature = "gpio-f413"))]
            PB10<9>,

            #[cfg(any(feature = "gpio-f410", feature = "gpio-f412", feature = "gpio-f413"))]
            PB15<4>,

            PC6<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD12<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD14<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PF14<4>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(any(feature = "gpio-f410", feature = "gpio-f412", feature = "gpio-f413"))]
            PB3<4>,

            #[cfg(any(feature = "gpio-f410", feature = "gpio-f412", feature = "gpio-f413"))]
            PB14<4>,

            PC7<4>,

            #[cfg(feature = "gpio-f410")]
            PC9<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD13<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD15<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PF15<4>,
        ],

        <Smba, OpenDrain> for [
            #[cfg(any(feature = "gpio-f410", feature = "gpio-f412", feature = "gpio-f413"))]
            PB13<4>,

            #[cfg(any(feature = "gpio-f410", feature = "gpio-f412", feature = "gpio-f413"))]
            PC5<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD11<4>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PF13<4>,
        ],
    }

    use crate::pac::FMPI2C1 as I2C;
    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
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

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF1<4>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH4<4>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(feature = "gpio-f446")]
            PB3<4>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB3<9>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB9<9>,

            PB11<4>,

            #[cfg(feature = "gpio-f446")]
            PC12<4>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF0<4>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH5<4>,
        ],

        <Smba, OpenDrain> for [
            PB12<4>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF2<4>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
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

#[cfg(feature = "i2c3")]
pub mod i2c3 {
    use super::*;

    pin! {
        <Scl, OpenDrain> for [
            PA8<4>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH7<4>,
        ],

        <Sda, OpenDrain> for [
            #[cfg(feature = "gpio-f446")]
            PB4<4>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB4<9>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB8<9>,

            PC9<4>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH8<4>,
        ],

        <Smba, OpenDrain> for [
            PA9<4>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
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

pub mod i2s {
    use super::*;

    pin! {
        <Ckin, PushPull> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA2<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB11<5>,

            PC9<5>,
        ],
    }
}

#[cfg(any(
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f446"
))]
pub mod i2s1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA5<5>,

            PB3<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f410")]
            PB10<6>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PC4<5>,

            #[cfg(feature = "gpio-f410")]
            PC7<6>,
        ],

        <Sd, PushPull> for [
            PA7<5>,

            PB5<5>,
        ],

        <Ws, PushPull> for [
            PA4<5>,

            PA15<5>,
        ],
    }

    use crate::pac::SPI1 as SPI;

    impl I2sCommon for SPI {
        type Ck = Ck;
        type Sd = Sd;
        type Ws = Ws;
    }
    impl I2sMaster for SPI {
        type Mck = Mck;
    }
}

pub mod i2s2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            #[cfg(any(feature = "gpio-f413", feature = "gpio-f446", feature = "gpio-f469"))]
            PA9<5>,

            PB10<5>,

            PB13<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PC7<5>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PD3<5>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI1<5>,
        ],

        <Mck, PushPull> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA3<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PA6<6>,

            PC6<5>,
        ],

        <Sd, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PA10<5>,

            PB15<5>,

            #[cfg(feature = "gpio-f469")]
            PC1<5>,

            #[cfg(feature = "gpio-f446")]
            PC1<7>,

            PC3<5>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI3<5>,
        ],

        <Ws, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PA11<5>,

            #[cfg(feature = "gpio-f446")]
            PB4<7>,

            PB9<5>,

            PB12<5>,

            #[cfg(feature = "gpio-f446")]
            PD1<7>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI0<5>,
        ],
    }

    #[cfg(not(any(feature = "gpio-f410", feature = "gpio-f446")))]
    pin! {
        <ExtSd, PushPull> for [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469"
            ))]
            PB14<6>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469"
            ))]
            PC2<6>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI2<6>,
        ],
    }

    use crate::pac::SPI2 as SPI;

    impl I2sCommon for SPI {
        type Ck = Ck;
        type Sd = Sd;
        type Ws = Ws;
    }
    impl I2sMaster for SPI {
        type Mck = Mck;
    }
    #[cfg(not(any(feature = "gpio-f410", feature = "gpio-f446")))]
    impl I2sExtPin for SPI {
        type ExtSd = ExtSd;
    }
}

#[cfg(feature = "gpio-f401")]
pub mod i2s2ext {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB13<6>,
        ],

        <Miso, PushPull> for [
            PB15<6>,
        ],

        <Ws, PushPull> for [
            PB12<6>,
        ],
    }
}

#[cfg(feature = "spi3")]
pub mod i2s3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB3<6>,

            #[cfg(feature = "gpio-f413")]
            PB12<5>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412"))]
            PB12<7>,

            PC10<6>,
        ],

        <Mck, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB10<6>,

            PC7<6>,
        ],

        <Sd, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PB0<7>,

            #[cfg(feature = "gpio-f446")]
            PB2<7>,

            PB5<6>,

            #[cfg(feature = "gpio-f446")]
            PC1<5>,

            PC12<6>,

            #[cfg(feature = "gpio-f446")]
            PD0<6>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PD6<5>,
        ],

        <Ws, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],
    }

    #[cfg(not(feature = "gpio-f446"))]
    pin! {
        <ExtSd, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PB4<5>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469"
            ))]
            PB4<7>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f469"
            ))]
            PC11<5>,
        ],
    }

    use crate::pac::SPI3 as SPI;

    impl I2sCommon for SPI {
        type Ck = Ck;
        type Sd = Sd;
        type Ws = Ws;
    }
    impl I2sMaster for SPI {
        type Mck = Mck;
    }
    #[cfg(not(feature = "gpio-f446"))]
    impl I2sExtPin for SPI {
        type ExtSd = ExtSd;
    }
}

#[cfg(feature = "gpio-f401")]
pub mod i2s3ext {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PC10<5>,
        ],

        <Miso, PushPull> for [
            PC12<5>,
        ],

        <Ws, PushPull> for [
            PA14<5>,
        ],
    }
}

#[cfg(feature = "spi4")]
pub mod i2s4 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB13<6>,

            PE2<5>,

            PE12<5>,
        ],

        <Sd, PushPull> for [
            PA1<5>,

            PE6<5>,

            PE14<5>,
        ],

        <Mck, PushPull> for no:NoPin, [ ],

        <Ws, PushPull> for [
            PB12<6>,

            PE4<5>,

            PE11<5>,
        ],
    }

    use crate::pac::SPI4 as SPI;

    impl I2sCommon for SPI {
        type Ck = Ck;
        type Sd = Sd;
        type Ws = Ws;
    }
    impl I2sMaster for SPI {
        type Mck = Mck;
    }
}

#[cfg(feature = "spi5")]
pub mod i2s5 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB0<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE2<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE12<6>,
        ],

        <Sd, PushPull> for [
            PA10<6>,

            PB8<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE6<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE14<6>,
        ],

        <Mck, PushPull> for no:NoPin, [ ],

        <Ws, PushPull> for [
            PB1<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE4<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE11<6>,
        ],
    }

    use crate::pac::SPI5 as SPI;

    impl I2sCommon for SPI {
        type Ck = Ck;
        type Sd = Sd;
        type Ws = Ws;
    }
    impl I2sMaster for SPI {
        type Mck = Mck;
    }
}

#[cfg(feature = "lptim1")]
pub mod lptim1 {
    use super::*;

    pin! {
        <Etr, PushPull> for [
            PB6<1>,

            PC3<1>,
        ],

        <In1, PushPull> for [
            PB5<1>,

            PC0<1>,
        ],

        <In2, PushPull> for [
            PB7<1>,

            PC2<1>,
        ],
    }

    pin! {
        <Out> default: PushPull for [
            PB2<1>,

            PB8<1>,

            PC1<1>,
        ],
    }
}

#[cfg(feature = "ltdc")]
pub mod ltdc {
    use super::*;

    pin! {
        <B0, PushPull> for [
            PE4<14>,

            #[cfg(feature = "gpio-f469")]
            PG14<14>,

            PJ12<14>,
        ],

        <B1, PushPull> for [
            PG12<14>,

            PJ13<14>,
        ],

        <B2, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PA3<9>,

            PD6<14>,

            PG10<14>,

            PJ14<14>,
        ],

        <B3, PushPull> for [
            PD10<14>,

            PG11<14>,

            PJ15<14>,
        ],

        <B4, PushPull> for [
            PE12<14>,

            PG12<9>,

            PI4<14>,

            PK3<14>,
        ],

        <B5, PushPull> for [
            PA3<14>,

            PI5<14>,

            PK4<14>,
        ],

        <B6, PushPull> for [
            PB8<14>,

            PI6<14>,

            PK5<14>,
        ],

        <B7, PushPull> for [
            PB9<14>,

            PI7<14>,

            PK6<14>,
        ],

        <Clk, PushPull> for [
            PE14<14>,

            PG7<14>,

            PI14<14>,
        ],

        <De, PushPull> for [
            PE13<14>,

            PF10<14>,

            PK7<14>,
        ],

        <G0, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PB1<14>,

            PE5<14>,

            #[cfg(feature = "gpio-f427")]
            PJ7<14>,
        ],

        <G1, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PB0<14>,

            PE6<14>,

            #[cfg(feature = "gpio-f427")]
            PJ8<14>,
        ],

        <G2, PushPull> for [
            PA6<14>,

            PH13<14>,

            #[cfg(feature = "gpio-f469")]
            PI15<9>,

            #[cfg(feature = "gpio-f427")]
            PJ9<14>,
        ],

        <G3, PushPull> for [
            PE11<14>,

            PG10<9>,

            PH14<14>,

            #[cfg(feature = "gpio-f427")]
            PJ10<14>,

            #[cfg(feature = "gpio-f469")]
            PJ12<9>,
        ],

        <G4, PushPull> for [
            PB10<14>,

            #[cfg(feature = "gpio-f469")]
            PH4<14>,

            PH15<14>,

            #[cfg(feature = "gpio-f427")]
            PJ11<14>,

            #[cfg(feature = "gpio-f469")]
            PJ13<9>,
        ],

        <G5, PushPull> for [
            PB11<14>,

            #[cfg(feature = "gpio-f469")]
            PH4<9>,

            PI0<14>,

            #[cfg(feature = "gpio-f427")]
            PK0<14>,
        ],

        <G6, PushPull> for [
            PC7<14>,

            PI1<14>,

            #[cfg(feature = "gpio-f469")]
            PI11<9>,

            #[cfg(feature = "gpio-f427")]
            PK1<14>,
        ],

        <G7, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PB5<14>,

            PD3<14>,

            #[cfg(feature = "gpio-f469")]
            PG8<14>,

            PI2<14>,

            #[cfg(feature = "gpio-f427")]
            PK2<14>,
        ],

        <Hsync, PushPull> for [
            PC6<14>,

            PI10<14>,

            PI12<14>,
        ],

        <R0, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PG13<14>,

            PH2<14>,

            PI15<14>,
        ],

        <R1, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PA2<14>,

            PH3<14>,

            PJ0<14>,
        ],

        <R2, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PA1<14>,

            PC10<14>,

            PH8<14>,

            PJ1<14>,
        ],

        <R3, PushPull> for [
            PB0<9>,

            PH9<14>,

            PJ2<14>,
        ],

        <R4, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PA5<14>,

            PA11<14>,

            PH10<14>,

            PJ3<14>,
        ],

        <R5, PushPull> for [
            PA12<14>,

            #[cfg(feature = "gpio-f469")]
            PC0<14>,

            PH11<14>,

            PJ4<14>,
        ],

        <R6, PushPull> for [
            PA8<14>,

            PB1<9>,

            PH12<14>,

            PJ5<14>,
        ],

        <R7, PushPull> for [
            PE15<14>,

            PG6<14>,

            #[cfg(feature = "gpio-f469")]
            PJ0<9>,

            #[cfg(feature = "gpio-f427")]
            PJ6<14>,
        ],

        <Vsync, PushPull> for [
            PA4<14>,

            PI9<14>,

            PI13<14>,
        ],
    }
}

#[cfg(feature = "quadspi")]
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
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC8<9>,

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

            #[cfg(feature = "gpio-f469")]
            PB10<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PG6<10>,
        ],

        <Bk2Io0, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA6<10>,

            PE7<10>,

            #[cfg(feature = "gpio-f469")]
            PH2<9>,
        ],

        <Bk2Io1, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PA7<10>,

            PE8<10>,

            #[cfg(feature = "gpio-f469")]
            PH3<9>,
        ],

        <Bk2Io2, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC4<10>,

            PE9<10>,

            PG9<9>,
        ],

        <Bk2Io3, PushPull> for [
            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PC5<10>,

            PE10<10>,

            PG14<9>,
        ],

        <Bk2Ncs, PushPull> for [
            PC11<9>,
        ],

        <Clk, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PA7<10>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB1<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PB2<9>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD3<9>,

            #[cfg(feature = "gpio-f469")]
            PF10<9>,
        ],
    }

    pub struct Bank1;
    pub struct Bank2;

    impl QuadSpiBanks for crate::pac::QUADSPI {
        type Bank1 = Bank1;
        type Bank2 = Bank2;
    }
    impl QuadSpiBank for Bank1 {
        type Io0 = Bk1Io0;
        type Io1 = Bk1Io1;
        type Io2 = Bk1Io2;
        type Io3 = Bk1Io3;
        type Ncs = Bk1Ncs;
    }
    impl QuadSpiBank for Bank2 {
        type Io0 = Bk2Io0;
        type Io1 = Bk2Io1;
        type Io2 = Bk2Io2;
        type Io3 = Bk2Io3;
        type Ncs = Bk2Ncs;
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

    #[cfg(feature = "gpio-f417")]
    pin! {
        <Osc32In, PushPull> for [
            PC14<0>,
        ],

        <Osc32Out, PushPull> for [
            PC15<0>,
        ],

        <OscIn, PushPull> for [
            PH0<0>,
        ],

        <OscOut, PushPull> for [
            PH1<0>,
        ],
    }
}

pub mod rtc {
    use super::*;

    #[cfg(feature = "gpio-f417")]
    pin! {
        <Af1, PushPull> for [
            PC13<0>,
        ],
    }

    pin! {
        <Refin, PushPull> for [
            PB15<0>,
        ],
    }
}

#[cfg(feature = "sai1")]
pub mod sai1 {
    use super::*;

    pin! {
        <FsA, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PA3<6>,

            #[cfg(feature = "gpio-f413")]
            PB5<10>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PE4<6>,

            #[cfg(feature = "gpio-f413")]
            PE6<7>,
        ],

        <FsB, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PB9<6>,

            #[cfg(feature = "gpio-f413")]
            PC3<7>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PF9<6>,

            #[cfg(feature = "gpio-f413")]
            PF9<7>,
        ],

        <MclkA, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PA15<10>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PE2<6>,

            #[cfg(feature = "gpio-f413")]
            PE2<7>,

            #[cfg(feature = "gpio-f469")]
            PG7<6>,
        ],

        <MclkB, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PC0<6>,

            #[cfg(feature = "gpio-f413")]
            PC0<7>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PF7<6>,

            #[cfg(feature = "gpio-f413")]
            PF7<7>,
        ],

        <SckA, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PB4<10>,

            #[cfg(feature = "gpio-f446")]
            PB10<6>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PE5<6>,

            #[cfg(feature = "gpio-f413")]
            PE5<7>,
        ],

        <SckB, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PB12<6>,

            #[cfg(feature = "gpio-f413")]
            PC2<7>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PF8<6>,

            #[cfg(feature = "gpio-f413")]
            PF8<7>,
        ],

        <SdA, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PB2<6>,

            #[cfg(feature = "gpio-f413")]
            PB3<10>,

            #[cfg(any(feature = "gpio-f446", feature = "gpio-f469"))]
            PC1<6>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PD6<6>,

            #[cfg(feature = "gpio-f413")]
            PE4<7>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PE6<6>,
        ],

        <SdB, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PA3<10>,

            #[cfg(feature = "gpio-f446")]
            PA9<6>,

            #[cfg(feature = "gpio-f413")]
            PC1<7>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PE3<6>,

            #[cfg(feature = "gpio-f413")]
            PE3<7>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469"))]
            PF6<6>,

            #[cfg(feature = "gpio-f413")]
            PF6<7>,
        ],
    }

    #[cfg(any(
        feature = "gpio-f413",
        feature = "gpio-f469",
        feature = "stm32f429",
        feature = "stm32f439"
    ))]
    use crate::pac::SAI;
    #[cfg(any(feature = "stm32f427", feature = "stm32f437", feature = "gpio-f446"))]
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

#[cfg(feature = "sai2")]
pub mod sai2 {
    use super::*;

    pin! {
        <FsA, PushPull> for [
            PD12<10>,
        ],

        <FsB, PushPull> for [
            PA12<8>,

            PE13<10>,

            PG9<10>,
        ],

        <MclkA, PushPull> for [
            PE0<10>,
        ],

        <MclkB, PushPull> for [
            PA1<10>,

            PE14<10>,
        ],

        <SckA, PushPull> for [
            PD13<10>,

            PD14<8>,
        ],

        <SckB, PushPull> for [
            PA2<8>,

            PE12<10>,
        ],

        <SdA, PushPull> for [
            PB11<8>,

            PD11<10>,
        ],

        <SdB, PushPull> for [
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

#[cfg(feature = "sdio")]
pub mod sdio {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PB2<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB15<12, Speed::VeryHigh>,

            PC12<12, Speed::VeryHigh>,
        ],

        <Cmd, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PA6<12, Speed::VeryHigh>,

            PD2<12, Speed::VeryHigh>,
        ],

        <D0, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB4<12, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB6<12, Speed::VeryHigh>,

            #[cfg(feature = "gpio-f411")]
            PB7<12, Speed::VeryHigh>,

            PC8<12, Speed::VeryHigh>,
        ],

        <D1, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PA8<12, Speed::VeryHigh>,

            #[cfg(feature = "gpio-f446")]
            PB0<12, Speed::VeryHigh>,

            PC9<12, Speed::VeryHigh>,
        ],

        <D2, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PA9<12, Speed::VeryHigh>,

            #[cfg(feature = "gpio-f446")]
            PB1<12, Speed::VeryHigh>,

            PC10<12, Speed::VeryHigh>,
        ],

        <D3, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB5<12, Speed::VeryHigh>,

            PC11<12, Speed::VeryHigh>,
        ],

        <D4, PushPull> for [
            PB8<12, Speed::VeryHigh>,
        ],

        <D5, PushPull> for [
            PB9<12, Speed::VeryHigh>,
        ],

        <D6, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB14<12, Speed::VeryHigh>,

            PC6<12>,
        ],

        <D7, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB10<12, Speed::VeryHigh>,

            PC7<12, Speed::VeryHigh>,
        ],
    }
}

#[cfg(feature = "spdifrx")]
pub mod spdifrx {
    use super::*;

    pin! {
        <In0, PushPull> for [
            PB7<8>,

            PD7<8>,

            PG11<7>,
        ],

        <In1, PushPull> for [
            PC7<7>,

            PD8<8>,

            PG12<7>,
        ],

        <In2, PushPull> for [
            PC4<8>,

            PG8<7>,
        ],

        <In3, PushPull> for [
            PC5<8>,

            PG9<7>,
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
        ],

        <Mosi, PushPull> for no:NoPin, [
            PA7<5>,

            PB5<5>,
        ],

        <Nss, PushPull> for [
            PA4<5>,

            PA15<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PA5<5>,

            PB3<5>,
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
            #[cfg(feature = "gpio-f413")]
            PA12<5>,

            PB14<5>,

            PC2<5>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI2<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f413")]
            PA10<5>,

            PB15<5>,

            #[cfg(feature = "gpio-f469")]
            PC1<5>,

            #[cfg(feature = "gpio-f446")]
            PC1<7>,

            PC3<5>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI3<5>,
        ],

        <Nss, PushPull> for [
            #[cfg(feature = "gpio-f413")]
            PA11<5>,

            #[cfg(feature = "gpio-f446")]
            PB4<7>,

            PB9<5>,

            PB12<5>,

            #[cfg(feature = "gpio-f446")]
            PD1<7>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI0<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f413", feature = "gpio-f446", feature = "gpio-f469"))]
            PA9<5>,

            PB10<5>,

            PB13<5>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f446"
            ))]
            PC7<5>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PD3<5>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
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

#[cfg(feature = "spi3")]
pub mod spi3 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            PB4<6>,

            PC11<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(feature = "gpio-f446")]
            PB0<7>,

            #[cfg(feature = "gpio-f446")]
            PB2<7>,

            PB5<6>,

            #[cfg(feature = "gpio-f446")]
            PC1<5>,

            PC12<6>,

            #[cfg(feature = "gpio-f446")]
            PD0<6>,

            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PD6<5>,
        ],

        <Nss, PushPull> for [
            PA4<6>,

            PA15<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            PB3<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB12<7>,

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

#[cfg(feature = "spi4")]
pub mod spi4 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PA11<6>,

            #[cfg(feature = "gpio-f446")]
            PD0<5>,

            PE5<5>,

            PE13<5>,

            #[cfg(feature = "gpio-f446")]
            PG12<6>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PA1<5>,

            PE6<5>,

            PE14<5>,

            #[cfg(feature = "gpio-f446")]
            PG13<6>,
        ],

        <Nss, PushPull> for [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB12<6>,

            PE4<5>,

            PE11<5>,

            #[cfg(feature = "gpio-f446")]
            PG14<6>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PB13<6>,

            PE2<5>,

            PE12<5>,

            #[cfg(feature = "gpio-f446")]
            PG11<6>,
        ],
    }

    impl SpiCommon for crate::pac::SPI4 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(feature = "spi5")]
pub mod spi5 {
    use super::*;

    pin! {
        <Miso, PushPull> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA12<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE5<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE13<6>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PF8<5>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH7<5>,
        ],

        <Mosi, PushPull> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA10<6>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB8<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE6<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE14<6>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PF9<5>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PF11<5>,
        ],

        <Nss, PushPull> for [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB1<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE4<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE11<6>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PF6<5>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH5<5>,
        ],

        <Sck, PushPull> for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB0<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE2<6>,

            #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
            PE12<6>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PF7<5>,

            #[cfg(any(feature = "gpio-f427", feature = "gpio-f469"))]
            PH6<5>,
        ],
    }

    impl SpiCommon for crate::pac::SPI5 {
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
        type Sck = Sck;
    }
}

#[cfg(feature = "spi6")]
pub mod spi6 {
    use super::*;

    pin! {
        <Miso, PushPull>  for no:NoPin, [
            PG12<5>,
        ],

        <Mosi, PushPull>  for no:NoPin, [
            PG14<5>,
        ],

        <Nss, PushPull> for [
            PG8<5>,
        ],

        <Sck, PushPull>  for no:NoPin, [
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
            #[cfg(feature = "gpio-f410")]
            PC6<0>,

            #[cfg(not(feature = "gpio-f410"))]
            PE2<0>,
        ],

        <Traced0, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PC1<0>,

            #[cfg(feature = "gpio-f446")]
            PC8<0>,

            #[cfg(feature = "gpio-f410")]
            PC10<0>,

            #[cfg(not(feature = "gpio-f410"))]
            PE3<0>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF6<0>,

            #[cfg(feature = "gpio-f469")]
            PG13<0>,
        ],

        <Traced1, PushPull> for [
            #[cfg(feature = "gpio-f469")]
            PC8<0>,

            #[cfg(feature = "gpio-f410")]
            PC11<0>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PD3<0>,

            #[cfg(not(feature = "gpio-f410"))]
            PE4<0>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF7<0>,

            #[cfg(feature = "gpio-f469")]
            PG14<0>,
        ],

        <Traced2, PushPull> for [
            #[cfg(feature = "gpio-f410")]
            PC12<0>,

            #[cfg(feature = "gpio-f469")]
            PD2<0>,

            #[cfg(not(feature = "gpio-f410"))]
            PE5<0>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PG13<0>,
        ],

        <Traced3, PushPull> for [
            #[cfg(feature = "gpio-f410")]
            PB11<0>,

            #[cfg(feature = "gpio-f469")]
            PC12<0>,

            #[cfg(not(feature = "gpio-f410"))]
            PE6<0>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PG14<0>,
        ],
    }

    #[cfg(feature = "gpio-f417")]
    pin! {
        <Wkup, PushPull> for [
            PA0<0>,
        ],
    }
}

pub mod tim1 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA8<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE9<1>,
        ],

        <Ch1N> default: PushPull for [
            PA7<1>,

            PB13<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE8<1>,
        ],

        <Ch2> default: PushPull for [
            PA9<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE11<1>,
        ],

        <Ch2N> default: PushPull for [
            PB0<1>,

            PB14<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE10<1>,
        ],

        <Ch3> default: PushPull for [
            PA10<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE13<1>,
        ],

        <Ch3N> default: PushPull for [
            PB1<1>,

            PB15<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE12<1>,
        ],

        <Ch4> default: PushPull for [
            PA11<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE14<1>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<1>,

            PB12<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE15<1>,
        ],

        <Etr, PushPull> for [
            PA12<1>,

            #[cfg(not(feature = "gpio-f410"))]
            PE7<1>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF10<1>,
        ],
    }

    use crate::pac::TIM1 as TIM;

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
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimNCPin<1> for TIM {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TimNCPin<2> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

#[cfg(feature = "tim2")]
pub mod tim2 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA0<1>,

            PA5<1>,

            PA15<1>,

            #[cfg(feature = "gpio-f446")]
            PB8<1>,
        ],

        <Ch2> default: PushPull for [
            PA1<1>,

            PB3<1>,

            #[cfg(feature = "gpio-f446")]
            PB9<1>,
        ],

        <Ch3> default: PushPull for [
            PA2<1>,

            PB10<1>,
        ],

        <Ch4> default: PushPull for [
            PA3<1>,

            #[cfg(feature = "gpio-f446")]
            PB2<1>,

            PB11<1>,
        ],
    }

    pin! {
        <Etr, PushPull> for [
            PA0<1>,

            PA5<1>,

            PA15<1>,

            #[cfg(feature = "gpio-f446")]
            PB8<1>,
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

#[cfg(feature = "tim2")]
pub mod tim3 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA6<2>,

            PB4<2>,

            PC6<2>,
        ],

        <Ch2> default: PushPull for [
            PA7<2>,

            PB5<2>,

            PC7<2>,
        ],

        <Ch3> default: PushPull for [
            PB0<2>,

            PC8<2>,
        ],

        <Ch4> default: PushPull for [
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

#[cfg(feature = "tim2")]
pub mod tim4 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PB6<2>,

            PD12<2>,
        ],

        <Ch2> default: PushPull for [
            PB7<2>,

            PD13<2>,
        ],

        <Ch3> default: PushPull for [
            PB8<2>,

            PD14<2>,
        ],

        <Ch4> default: PushPull for [
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

pub mod tim5 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA0<2>,

            #[cfg(feature = "gpio-f410")]
            PB12<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF3<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH10<2>,
        ],

        <Ch2> default: PushPull for [
            PA1<2>,

            #[cfg(feature = "gpio-f410")]
            PC10<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF4<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH11<2>,
        ],

        <Ch3> default: PushPull for [
            PA2<2>,

            #[cfg(feature = "gpio-f410")]
            PC11<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF5<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH12<2>,
        ],

        <Ch4> default: PushPull for [
            PA3<2>,

            #[cfg(feature = "gpio-f410")]
            PB11<2>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF10<2>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI0<2>,
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
}

#[cfg(feature = "tim8")]
pub mod tim8 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PC6<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI5<3>,
        ],

        <Ch1N> default: PushPull for [
            PA5<3>,

            PA7<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH13<3>,
        ],

        <Ch2> default: PushPull for [
            PC7<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI6<3>,
        ],

        <Ch2N> default: PushPull for [
            PB0<3>,

            PB14<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH14<3>,
        ],

        <Ch3> default: PushPull for [
            PC8<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI7<3>,
        ],

        <Ch3N> default: PushPull for [
            PB1<3>,

            PB15<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH15<3>,
        ],

        <Ch4> default: PushPull for [
            PC9<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI2<3>,
        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<3>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF12<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI4<3>,
        ],

        <Etr, PushPull> for [
            PA0<3>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PF11<3>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI3<3>,
        ],
    }

    use crate::pac::TIM8 as TIM;

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
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimNCPin<1> for TIM {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TimNCPin<2> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}

pub mod tim9 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA2<3>,

            #[cfg(feature = "gpio-f410")]
            PC4<3>,

            #[cfg(not(feature = "gpio-f410"))]
            PE5<3>,
        ],

        <Ch2> default: PushPull for [
            PA3<3>,

            #[cfg(feature = "gpio-f410")]
            PC5<3>,

            #[cfg(not(feature = "gpio-f410"))]
            PE6<3>,
        ],
    }

    use crate::pac::TIM9 as TIM;

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
}

#[cfg(feature = "tim2")]
pub mod tim10 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PB8<3>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF6<3>,
        ],
    }

    use crate::pac::TIM10 as TIM;

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

pub mod tim11 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PB9<3>,

            #[cfg(feature = "gpio-f410")]
            PC12<3>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PF7<3>,
        ],
    }

    use crate::pac::TIM11 as TIM;

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

#[cfg(feature = "tim8")]
pub mod tim12 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PB14<9>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH6<9>,
        ],

        <Ch2> default: PushPull for [
            PB15<9>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH9<9>,
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

#[cfg(feature = "tim8")]
pub mod tim13 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA6<9>,

            PF8<9>,
        ],
    }

    use crate::pac::TIM13 as TIM;

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
}

#[cfg(feature = "tim8")]
pub mod tim14 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA7<9>,

            PF9<9>,
        ],
    }

    use crate::pac::TIM14 as TIM;

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
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

        <Rts, PushPull> for [
            PA12<7>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PA10<7>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PB3<7>,

            PB7<7>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA9<7>,

            #[cfg(any(
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA15<7>,

            PB6<7>,
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

            #[cfg(not(feature = "gpio-f410"))]
            PD7<7>,
        ],

        <Cts, PushPull> for [
            PA0<7>,

            #[cfg(not(feature = "gpio-f410"))]
            PD3<7>,
        ],

        <Rts, PushPull> for [
            PA1<7>,

            #[cfg(not(feature = "gpio-f410"))]
            PD4<7>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PA3<7>,

            #[cfg(not(feature = "gpio-f410"))]
            PD6<7>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA2<7>,

            #[cfg(not(feature = "gpio-f410"))]
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

#[cfg(feature = "usart3")]
pub mod usart3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PB12<7>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB12<8>,

            PC12<7>,

            PD10<7>,
        ],

        <Cts, PushPull> for [
            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PB13<7>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
            PB13<8>,

            PD11<7>,
        ],

        <Rts, PushPull> for [
            PB14<7>,

            PD12<7>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PB11<7>,

            #[cfg(any(feature = "gpio-f412", feature = "gpio-f413", feature = "gpio-f446"))]
            PC5<7>,

            PC11<7>,

            PD9<7>,
        ],

        <Tx> default: PushPull for no:NoPin, [
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
            PC8<8>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PG7<8>,
        ],
    }

    #[cfg(any(
        feature = "gpio-f412",
        feature = "gpio-f413",
        feature = "gpio-f417",
        feature = "gpio-f427",
        feature = "gpio-f446",
        feature = "gpio-f469"
    ))]
    pin! {
        <Cts, PushPull> for [
            PG13<8>,

            PG15<8>,
        ],

        <Rts, PushPull> for [
            PG8<8>,

            PG12<8>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA12<8>,

            PC7<8>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PG9<8>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            #[cfg(any(
                feature = "gpio-f401",
                feature = "gpio-f410",
                feature = "gpio-f411",
                feature = "gpio-f412",
                feature = "gpio-f413"
            ))]
            PA11<8>,

            PC6<8>,

            #[cfg(any(
                feature = "gpio-f412",
                feature = "gpio-f413",
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PG14<8>,
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
    #[cfg(any(
        feature = "gpio-f412",
        feature = "gpio-f413",
        feature = "gpio-f417",
        feature = "gpio-f427",
        feature = "gpio-f446",
        feature = "gpio-f469"
    ))]
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "uart4")]
pub mod uart4 {
    use super::*;

    #[cfg(feature = "gpio-f446")]
    pin! {
        <Cts, PushPull> for [
            PB0<8>,
        ],

        <Rts, PushPull> for [
            PA15<8>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PA1<8>,

            #[cfg(feature = "gpio-f413")]
            PA11<11>,

            PC11<8>,

            #[cfg(feature = "gpio-f413")]
            PD0<11>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA0<8>,

            #[cfg(feature = "gpio-f413")]
            PA12<11>,

            #[cfg(any(
                feature = "gpio-f417",
                feature = "gpio-f427",
                feature = "gpio-f446",
                feature = "gpio-f469"
            ))]
            PC10<8>,

            #[cfg(feature = "gpio-f413")]
            PD1<11>,

            #[cfg(feature = "gpio-f413")]
            PD10<8>,
        ],
    }

    use crate::pac::UART4 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    #[cfg(feature = "gpio-f446")]
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "uart5")]
pub mod uart5 {
    use super::*;

    #[cfg(feature = "gpio-f446")]
    pin! {
        <Cts, PushPull> for [
            PC9<7>,
        ],

        <Rts, PushPull> for [
            PC8<7>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f413")]
            PB5<11>,

            #[cfg(feature = "gpio-f413")]
            PB8<11>,

            #[cfg(feature = "gpio-f413")]
            PB12<11>,

            PD2<8>,

            #[cfg(feature = "gpio-f446")]
            PE7<8>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f413")]
            PB6<11>,

            #[cfg(feature = "gpio-f413")]
            PB9<11>,

            #[cfg(feature = "gpio-f413")]
            PB13<11>,

            PC12<8>,

            #[cfg(feature = "gpio-f446")]
            PE8<8>,
        ],
    }

    use crate::pac::UART5 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    #[cfg(feature = "gpio-f446")]
    impl SerialRs232 for UART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "uart7")]
pub mod uart7 {
    use super::*;

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f413")]
            PA8<8>,

            #[cfg(feature = "gpio-f413")]
            PB3<8>,

            PE7<8>,

            PF6<8>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            #[cfg(feature = "gpio-f413")]
            PA15<8>,

            #[cfg(feature = "gpio-f413")]
            PB4<8>,

            PE8<8>,

            PF7<8>,
        ],
    }

    use crate::pac::UART7 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(feature = "uart8")]
pub mod uart8 {
    use super::*;

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PE0<8>,

            #[cfg(feature = "gpio-f413")]
            PF8<8>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PE1<8>,

            #[cfg(feature = "gpio-f413")]
            PF9<8>,
        ],
    }

    use crate::pac::UART8 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(feature = "uart9")]
pub mod uart9 {
    use super::*;

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PD14<11>,

            PG0<11>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PD15<11>,

            PG1<11>,
        ],
    }

    use crate::pac::UART9 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(feature = "uart10")]
pub mod uart10 {
    use super::*;

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PE2<11>,

            PG11<11>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PE3<11>,

            PG12<11>,
        ],
    }

    use crate::pac::UART10 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(feature = "otg-fs")]
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

    #[cfg(any(feature = "gpio-f411", feature = "gpio-f412", feature = "gpio-f413"))]
    pin! {
        <Vbus, PushPull> for [
            PA9<10>,
        ],
    }
}

#[cfg(feature = "otg-hs")]
pub mod otg_hs {
    use super::*;
    pin! {
        <Dm, PushPull> for [
            PB14<12>,
        ],

        <Dp, PushPull> for [
            PB15<12>,
        ],

        <Id, PushPull> for [
            PB12<12>,
        ],

        <Sof, PushPull> for [
            PA4<12>,
        ],

        <UlpiCk, PushPull> for [
            PA5<10, Speed::VeryHigh>,
        ],

        <UlpiD0, PushPull> for [
            PA3<10, Speed::VeryHigh>,
        ],

        <UlpiD1, PushPull> for [
            PB0<10, Speed::VeryHigh>,
        ],

        <UlpiD2, PushPull> for [
            PB1<10, Speed::VeryHigh>,
        ],

        <UlpiD3, PushPull> for [
            PB10<10, Speed::VeryHigh>,
        ],

        <UlpiD4, PushPull> for [
            #[cfg(feature = "gpio-f446")]
            PB2<10, Speed::VeryHigh>,

            PB11<10, Speed::VeryHigh>,
        ],

        <UlpiD5, PushPull> for [
            PB12<10, Speed::VeryHigh>,
        ],

        <UlpiD6, PushPull> for [
            PB13<10, Speed::VeryHigh>,
        ],

        <UlpiD7, PushPull> for [
            PB5<10, Speed::VeryHigh>,
        ],

        <UlpiDir, PushPull> for [
            PC2<10, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PI11<10, Speed::VeryHigh>,
        ],

        <UlpiNxt, PushPull> for [
            PC3<10, Speed::VeryHigh>,

            #[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
            PH4<10, Speed::VeryHigh>,
        ],

        <UlpiStp, PushPull> for [
            PC0<10, Speed::VeryHigh>,
        ],
    }
}
