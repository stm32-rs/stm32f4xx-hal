//! General Purpose Input / Output

use core::marker::PhantomData;

use crate::stm32::{EXTI, SYSCFG};

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;
pub struct AF5;
pub struct AF6;
pub struct AF7;
pub struct AF8;
pub struct AF9;
pub struct AF10;
pub struct AF11;
pub struct AF12;
pub struct AF13;
pub struct AF14;
pub struct AF15;

pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// GPIO Pin speed selection
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

#[derive(Debug, PartialEq)]
pub enum Edge {
    RISING,
    FALLING,
    RISING_FALLING,
}

/// External Interrupt Pin
pub trait ExtiPin {
    fn trigger_on_edge(&mut self, exti: &mut EXTI, level: Edge);
    fn enable_interrupt(&mut self, exti: &mut EXTI, syscfg: &mut SYSCFG);
    fn disable_interrupt(&mut self, exti: &mut EXTI);
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopxenr:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $exticri:ident, $extigpionr:expr, $tri:ident, $mri:ident),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use embedded_hal::digital::{InputPin, OutputPin};
            use crate::stm32::$GPIOX;

            use crate::stm32::{RCC, EXTI, SYSCFG};
            use super::{
                Alternate, Floating, GpioExt, Input, OpenDrain, Output, Speed,
                PullDown, PullUp, PushPull, AF0, AF1, AF2, AF3, AF4, AF5, AF6, AF7, AF8, AF9, AF10,
                AF11, AF12, AF13, AF14, AF15, Edge, ExtiPin,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    // NOTE(unsafe) This executes only during initialisation
                    let rcc = unsafe { &(*RCC::ptr()) };
                    rcc.ahb1enr.modify(|_, w| w.$iopxenr().set_bit());

                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
                }

                fn set_low(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
                }
            }

            impl<MODE> InputPin for $PXx<Output<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << self.i) == 0 }
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << self.i) == 0 }
                }
            }

            fn _set_alternate_mode (index: usize, mode: u32)
            {
                let offset = 2 * index;
                let offset2 = 4 * index;
                unsafe {
                    if offset2 < 32 {
                        &(*$GPIOX::ptr()).afrl.modify(|r, w| {
                            w.bits((r.bits() & !(0b1111 << offset2)) | (mode << offset2))
                        });
                    } else
                    {
                        let offset2 = offset2 - 32;
                        &(*$GPIOX::ptr()).afrh.modify(|r, w| {
                            w.bits((r.bits() & !(0b1111 << offset2)) | (mode << offset2))
                        });
                    }
                    &(*$GPIOX::ptr()).moder.modify(|r, w| {
                        w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset))
                    });
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate in AF0 mode
                    pub fn into_alternate_af0(
                        self,
                    ) -> $PXi<Alternate<AF0>> {
                        _set_alternate_mode($i, 0);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF1 mode
                    pub fn into_alternate_af1(
                        self,
                    ) -> $PXi<Alternate<AF1>> {
                        _set_alternate_mode($i, 1);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF2 mode
                    pub fn into_alternate_af2(
                        self,
                    ) -> $PXi<Alternate<AF2>> {
                        _set_alternate_mode($i, 2);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF3 mode
                    pub fn into_alternate_af3(
                        self,
                    ) -> $PXi<Alternate<AF3>> {
                        _set_alternate_mode($i, 3);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF4 mode
                    pub fn into_alternate_af4(
                        self,
                    ) -> $PXi<Alternate<AF4>> {
                        _set_alternate_mode($i, 4);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF5 mode
                    pub fn into_alternate_af5(
                        self,
                    ) -> $PXi<Alternate<AF5>> {
                        _set_alternate_mode($i, 5);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF6 mode
                    pub fn into_alternate_af6(
                        self,
                    ) -> $PXi<Alternate<AF6>> {
                        _set_alternate_mode($i, 6);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF7 mode
                    pub fn into_alternate_af7(
                        self,
                    ) -> $PXi<Alternate<AF7>> {
                        _set_alternate_mode($i, 7);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF8 mode
                    pub fn into_alternate_af8(
                        self,
                    ) -> $PXi<Alternate<AF8>> {
                        _set_alternate_mode($i, 8);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF9 mode
                    pub fn into_alternate_af9(
                        self,
                    ) -> $PXi<Alternate<AF9>> {
                        _set_alternate_mode($i, 9);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF10 mode
                    pub fn into_alternate_af10(
                        self,
                    ) -> $PXi<Alternate<AF10>> {
                        _set_alternate_mode($i, 10);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF11 mode
                    pub fn into_alternate_af11(
                        self,
                    ) -> $PXi<Alternate<AF11>> {
                        _set_alternate_mode($i, 11);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF12 mode
                    pub fn into_alternate_af12(
                        self,
                    ) -> $PXi<Alternate<AF12>> {
                        _set_alternate_mode($i, 12);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF13 mode
                    pub fn into_alternate_af13(
                        self,
                    ) -> $PXi<Alternate<AF13>> {
                        _set_alternate_mode($i, 13);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF14 mode
                    pub fn into_alternate_af14(
                        self,
                    ) -> $PXi<Alternate<AF14>> {
                        _set_alternate_mode($i, 14);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF15 mode
                    pub fn into_alternate_af15(
                        self,
                    ) -> $PXi<Alternate<AF15>> {
                        _set_alternate_mode($i, 15);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                    ) -> $PXi<Input<Floating>> {
                        let offset = 2 * $i;
                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                         });
                            &(*$GPIOX::ptr()).moder.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                         })};
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                        ) -> $PXi<Input<PullDown>> {
                        let offset = 2 * $i;
                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset))
                         });
                            &(*$GPIOX::ptr()).moder.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                         })};
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                    ) -> $PXi<Input<PullUp>> {
                        let offset = 2 * $i;
                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                         });
                            &(*$GPIOX::ptr()).moder.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                         })};

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                        self,
                    ) -> $PXi<Output<OpenDrain>> {
                        let offset = 2 * $i;
                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                         });
                            &(*$GPIOX::ptr()).otyper.modify(|r, w| {
                                w.bits(r.bits() | (0b1 << $i))
                         });
                            &(*$GPIOX::ptr()).moder.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                         })};


                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                    ) -> $PXi<Output<PushPull>> {
                        let offset = 2 * $i;

                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                            });
                            &(*$GPIOX::ptr()).otyper.modify(|r, w| {
                                w.bits(r.bits() & !(0b1 << $i))
                            });
                            &(*$GPIOX::ptr()).moder.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                         })};


                        $PXi { _mode: PhantomData }
                    }

                    /// Set pin speed
                    pub fn set_speed(self, speed: Speed) -> Self {
                        let offset = 2 * $i;

                        unsafe {
                            &(*$GPIOX::ptr()).ospeedr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset))
                         })};

                        self
                    }
                }

                impl $PXi<Output<OpenDrain>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(&mut self, on: bool) {
                        let offset = 2 * $i;
                        let value = if on { 0b01 } else { 0b00 };
                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (value << offset))
                         })};
                    }
                }

                impl<MODE> $PXi<Alternate<MODE>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(self, on: bool) -> Self {
                        let offset = 2 * $i;
                        let value = if on { 0b01 } else { 0b00 };
                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w| {
                                w.bits((r.bits() & !(0b11 << offset)) | (value << offset))
                         })};

                        self
                    }
                }

                impl<MODE> $PXi<Alternate<MODE>> {
                    /// Turns pin alternate configuration pin into open drain
                    pub fn set_open_drain(self) -> Self {
                        let offset = $i;
                        unsafe {
                            &(*$GPIOX::ptr()).otyper.modify(|r, w| {
                                w.bits(r.bits() | (1 << offset))
                         })};

                        self
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << ($i + 16))) }
                    }
                }

                impl<MODE> InputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl<MODE> $PXi<Input<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Input<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl<MODE> ExtiPin for $PXi<Input<MODE>> {
                    /// generate interrupt on rising edge, falling edge or both
                    fn trigger_on_edge(&mut self, exti: &mut EXTI, edge: Edge) {
                        match edge {
                            Edge::RISING => exti.rtsr.write(|w| { w.$tri().set_bit() }),
                            Edge::FALLING => exti.ftsr.write(|w| { w.$tri().set_bit() }),
                            Edge::RISING_FALLING => {
                                exti.rtsr.write(|w| { w.$tri().set_bit() });
                                exti.ftsr.write(|w| { w.$tri().set_bit() });
                            }
                        }
                    }

                    /// Enable external interrupts from this pin.
                    /// Note: This configures EXTI line $i to trigger from this pin
                    fn enable_interrupt(&mut self, exti: &mut EXTI, syscfg: &mut SYSCFG) {
                        syscfg.$exticri.modify(|r, w| unsafe {
                            let mut exticr = r.bits();
                            exticr = (exticr & $extigpionr) | $extigpionr;
                            w.bits(exticr)
                        });
                        exti.imr.write(|w| { w.$mri().set_bit() });
                    }

                    /// Disable external interrupts from this pin
                    fn disable_interrupt(&mut self, exti: &mut EXTI) {
                        exti.imr.write(|w| { w.$mri().clear_bit() });
                    }
                }

            )+

                impl<TYPE> $PXx<TYPE> {
                    pub fn get_id (&self) -> u8
                    {
                        self.i
                    }
                }
        }
    }
}

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
gpio!(GPIOA, gpioa, gpioaen, PA, [
    PA0: (pa0, 0, Input<Floating>, exticr1, 0, tr0, mr0),
    PA1: (pa1, 1, Input<Floating>, exticr1, 0, tr1, mr1),
    PA2: (pa2, 2, Input<Floating>, exticr1, 0, tr2, mr2),
    PA3: (pa3, 3, Input<Floating>, exticr1, 0, tr3, mr3),
    PA4: (pa4, 4, Input<Floating>, exticr2, 0, tr4, mr4),
    PA5: (pa5, 5, Input<Floating>, exticr2, 0, tr5, mr5),
    PA6: (pa6, 6, Input<Floating>, exticr2, 0, tr6, mr6),
    PA7: (pa7, 7, Input<Floating>, exticr2, 0, tr7, mr7),
    PA8: (pa8, 8, Input<Floating>, exticr3, 0, tr8, mr8),
    PA9: (pa9, 9, Input<Floating>, exticr3, 0, tr9, mr9),
    PA10: (pa10, 10, Input<Floating>, exticr3, 0, tr10, mr10),
    PA11: (pa11, 11, Input<Floating>, exticr3, 0, tr11, mr11),
    PA12: (pa12, 12, Input<Floating>, exticr4, 0, tr12, mr12),
    PA13: (pa13, 13, Input<Floating>, exticr4, 0, tr13, mr13),
    PA14: (pa14, 14, Input<Floating>, exticr4, 0, tr14, mr14),
    PA15: (pa15, 15, Input<Floating>, exticr4, 0, tr15, mr15),
]);

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
gpio!(GPIOB, gpiob, gpioben, PB, [
    PB0: (pb0, 0, Input<Floating>, exticr1, 1, tr0, mr0),
    PB1: (pb1, 1, Input<Floating>, exticr1, 1, tr1, mr1),
    PB2: (pb2, 2, Input<Floating>, exticr1, 1, tr2, mr2),
    PB3: (pb3, 3, Input<Floating>, exticr1, 1, tr3, mr3),
    PB4: (pb4, 4, Input<Floating>, exticr2, 1, tr4, mr4),
    PB5: (pb5, 5, Input<Floating>, exticr2, 1, tr5, mr5),
    PB6: (pb6, 6, Input<Floating>, exticr2, 1, tr6, mr6),
    PB7: (pb7, 7, Input<Floating>, exticr2, 1, tr7, mr7),
    PB8: (pb8, 8, Input<Floating>, exticr3, 1, tr8, mr8),
    PB9: (pb9, 9, Input<Floating>, exticr3, 1, tr9, mr9),
    PB10: (pb10, 10, Input<Floating>, exticr3, 1, tr10, mr10),
    PB11: (pb11, 11, Input<Floating>, exticr3, 1, tr11, mr11),
    PB12: (pb12, 12, Input<Floating>, exticr4, 1, tr12, mr12),
    PB13: (pb13, 13, Input<Floating>, exticr4, 1, tr13, mr13),
    PB14: (pb14, 14, Input<Floating>, exticr4, 1, tr14, mr14),
    PB15: (pb15, 15, Input<Floating>, exticr4, 1, tr15, mr15),
]);

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
gpio!(GPIOC, gpioc, gpiocen, PC, [
    PC0: (pc0, 0, Input<Floating>, exticr1, 2, tr0, mr0),
    PC1: (pc1, 1, Input<Floating>, exticr1, 2, tr1, mr1),
    PC2: (pc2, 2, Input<Floating>, exticr1, 2, tr2, mr2),
    PC3: (pc3, 3, Input<Floating>, exticr1, 2, tr3, mr3),
    PC4: (pc4, 4, Input<Floating>, exticr2, 2, tr4, mr4),
    PC5: (pc5, 5, Input<Floating>, exticr2, 2, tr5, mr5),
    PC6: (pc6, 6, Input<Floating>, exticr2, 2, tr6, mr6),
    PC7: (pc7, 7, Input<Floating>, exticr2, 2, tr7, mr7),
    PC8: (pc8, 8, Input<Floating>, exticr3, 2, tr8, mr8),
    PC9: (pc9, 9, Input<Floating>, exticr3, 2, tr9, mr9),
    PC10: (pc10, 10, Input<Floating>, exticr3, 2, tr10, mr10),
    PC11: (pc11, 11, Input<Floating>, exticr3, 2, tr11, mr11),
    PC12: (pc12, 12, Input<Floating>, exticr4, 2, tr12, mr12),
    PC13: (pc13, 13, Input<Floating>, exticr4, 2, tr13, mr13),
    PC14: (pc14, 14, Input<Floating>, exticr4, 2, tr14, mr14),
    PC15: (pc15, 15, Input<Floating>, exticr4, 2, tr15, mr15),
]);

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
gpio!(GPIOD, gpiod, gpioden, PD, [
    PD0: (pd0, 0, Input<Floating>, exticr1, 3, tr0, mr0),
    PD1: (pd1, 1, Input<Floating>, exticr1, 3, tr1, mr1),
    PD2: (pd2, 2, Input<Floating>, exticr1, 3, tr2, mr2),
    PD3: (pd3, 3, Input<Floating>, exticr1, 3, tr3, mr3),
    PD4: (pd4, 4, Input<Floating>, exticr2, 3, tr4, mr4),
    PD5: (pd5, 5, Input<Floating>, exticr2, 3, tr5, mr5),
    PD6: (pd6, 6, Input<Floating>, exticr2, 3, tr6, mr6),
    PD7: (pd7, 7, Input<Floating>, exticr2, 3, tr7, mr7),
    PD8: (pd8, 8, Input<Floating>, exticr3, 3, tr8, mr8),
    PD9: (pd9, 9, Input<Floating>, exticr3, 3, tr9, mr9),
    PD10: (pd10, 10, Input<Floating>, exticr3, 3, tr10, mr10),
    PD11: (pd11, 11, Input<Floating>, exticr3, 3, tr11, mr11),
    PD12: (pd12, 12, Input<Floating>, exticr4, 3, tr12, mr12),
    PD13: (pd13, 13, Input<Floating>, exticr4, 3, tr13, mr13),
    PD14: (pd14, 14, Input<Floating>, exticr4, 3, tr14, mr14),
    PD15: (pd15, 15, Input<Floating>, exticr4, 3, tr15, mr15),
]);

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
gpio!(GPIOE, gpioe, gpioeen, PE, [
    PE0: (pe0, 0, Input<Floating>, exticr1, 4, tr0, mr0),
    PE1: (pe1, 1, Input<Floating>, exticr1, 4, tr1, mr1),
    PE2: (pe2, 2, Input<Floating>, exticr1, 4, tr2, mr2),
    PE3: (pe3, 3, Input<Floating>, exticr1, 4, tr3, mr3),
    PE4: (pe4, 4, Input<Floating>, exticr2, 4, tr4, mr4),
    PE5: (pe5, 5, Input<Floating>, exticr2, 4, tr5, mr5),
    PE6: (pe6, 6, Input<Floating>, exticr2, 4, tr6, mr6),
    PE7: (pe7, 7, Input<Floating>, exticr2, 4, tr7, mr7),
    PE8: (pe8, 8, Input<Floating>, exticr3, 4, tr8, mr8),
    PE9: (pe9, 9, Input<Floating>, exticr3, 4, tr9, mr9),
    PE10: (pe10, 10, Input<Floating>, exticr3, 4, tr10, mr10),
    PE11: (pe11, 11, Input<Floating>, exticr3, 4, tr11, mr11),
    PE12: (pe12, 12, Input<Floating>, exticr4, 4, tr12, mr12),
    PE13: (pe13, 13, Input<Floating>, exticr4, 4, tr13, mr13),
    PE14: (pe14, 14, Input<Floating>, exticr4, 4, tr14, mr14),
    PE15: (pe15, 15, Input<Floating>, exticr4, 4, tr15, mr15),
]);

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
gpio!(GPIOF, gpiof, gpiofen, PF, [
    PF0: (pf0, 0, Input<Floating>, exticr1, 5, tr0, mr0),
    PF1: (pf1, 1, Input<Floating>, exticr1, 5, tr1, mr1),
    PF2: (pf2, 2, Input<Floating>, exticr1, 5, tr2, mr2),
    PF3: (pf3, 3, Input<Floating>, exticr1, 5, tr3, mr3),
    PF4: (pf4, 4, Input<Floating>, exticr2, 5, tr4, mr4),
    PF5: (pf5, 5, Input<Floating>, exticr2, 5, tr5, mr5),
    PF6: (pf6, 6, Input<Floating>, exticr2, 5, tr6, mr6),
    PF7: (pf7, 7, Input<Floating>, exticr2, 5, tr7, mr7),
    PF8: (pf8, 8, Input<Floating>, exticr3, 5, tr8, mr8),
    PF9: (pf9, 9, Input<Floating>, exticr3, 5, tr9, mr9),
    PF10: (pf10, 10, Input<Floating>, exticr3, 5, tr10, mr10),
    PF11: (pf11, 11, Input<Floating>, exticr3, 5, tr11, mr11),
    PF12: (pf12, 12, Input<Floating>, exticr4, 5, tr12, mr12),
    PF13: (pf13, 13, Input<Floating>, exticr4, 5, tr13, mr13),
    PF14: (pf14, 14, Input<Floating>, exticr4, 5, tr14, mr14),
    PF15: (pf15, 15, Input<Floating>, exticr4, 5, tr15, mr15),
]);

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
gpio!(GPIOG, gpiog, gpiogen, PG, [
    PG0: (pg0, 0, Input<Floating>, exticr1, 6, tr0, mr0),
    PG1: (pg1, 1, Input<Floating>, exticr1, 6, tr1, mr1),
    PG2: (pg2, 2, Input<Floating>, exticr1, 6, tr2, mr2),
    PG3: (pg3, 3, Input<Floating>, exticr1, 6, tr3, mr3),
    PG4: (pg4, 4, Input<Floating>, exticr2, 6, tr4, mr4),
    PG5: (pg5, 5, Input<Floating>, exticr2, 6, tr5, mr5),
    PG6: (pg6, 6, Input<Floating>, exticr2, 6, tr6, mr6),
    PG7: (pg7, 7, Input<Floating>, exticr2, 6, tr7, mr7),
    PG8: (pg8, 8, Input<Floating>, exticr3, 6, tr8, mr8),
    PG9: (pg9, 9, Input<Floating>, exticr3, 6, tr9, mr9),
    PG10: (pg10, 10, Input<Floating>, exticr3, 6, tr10, mr10),
    PG11: (pg11, 11, Input<Floating>, exticr3, 6, tr11, mr11),
    PG12: (pg12, 12, Input<Floating>, exticr4, 6, tr12, mr12),
    PG13: (pg13, 13, Input<Floating>, exticr4, 6, tr13, mr13),
    PG14: (pg14, 14, Input<Floating>, exticr4, 6, tr14, mr14),
    PG15: (pg15, 15, Input<Floating>, exticr4, 6, tr15, mr15),
]);

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
gpio!(GPIOH, gpioh, gpiohen, PH, [
    PH0: (ph0, 0, Input<Floating>, exticr1, 7, tr0, mr0),
    PH1: (ph1, 1, Input<Floating>, exticr1, 7, tr1, mr1),
    PH2: (ph2, 2, Input<Floating>, exticr1, 7, tr2, mr2),
    PH3: (ph3, 3, Input<Floating>, exticr1, 7, tr3, mr3),
    PH4: (ph4, 4, Input<Floating>, exticr2, 7, tr4, mr4),
    PH5: (ph5, 5, Input<Floating>, exticr2, 7, tr5, mr5),
    PH6: (ph6, 6, Input<Floating>, exticr2, 7, tr6, mr6),
    PH7: (ph7, 7, Input<Floating>, exticr2, 7, tr7, mr7),
    PH8: (ph8, 8, Input<Floating>, exticr3, 7, tr8, mr8),
    PH9: (ph9, 9, Input<Floating>, exticr3, 7, tr9, mr9),
    PH10: (ph10, 10, Input<Floating>, exticr3, 7, tr10, mr10),
    PH11: (ph11, 11, Input<Floating>, exticr3, 7, tr11, mr11),
    PH12: (ph12, 12, Input<Floating>, exticr4, 7, tr12, mr12),
    PH13: (ph13, 13, Input<Floating>, exticr4, 7, tr13, mr13),
    PH14: (ph14, 14, Input<Floating>, exticr4, 7, tr14, mr14),
    PH15: (ph15, 15, Input<Floating>, exticr4, 7, tr15, mr15),
]);

#[cfg(any(feature = "stm32f401"))]
gpio!(GPIOH, gpioh, gpiohen, PH, [
    PH0: (ph0, 0, Input<Floating>, exticr1, 7, tr0, mr0),
    PH1: (ph1, 1, Input<Floating>, exticr1, 7, tr1, mr1),
]);

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
gpio!(GPIOI, gpioi, gpioien, PI, [
    PI0: (pi0, 0, Input<Floating>, exticr1, 8, tr0, mr0),
    PI1: (pi1, 1, Input<Floating>, exticr1, 8, tr1, mr1),
    PI2: (pi2, 2, Input<Floating>, exticr1, 8, tr2, mr2),
    PI3: (pi3, 3, Input<Floating>, exticr1, 8, tr3, mr3),
    PI4: (pi4, 4, Input<Floating>, exticr2, 8, tr4, mr4),
    PI5: (pi5, 5, Input<Floating>, exticr2, 8, tr5, mr5),
    PI6: (pi6, 6, Input<Floating>, exticr2, 8, tr6, mr6),
    PI7: (pi7, 7, Input<Floating>, exticr2, 8, tr7, mr7),
    PI8: (pi8, 8, Input<Floating>, exticr3, 8, tr8, mr8),
    PI9: (pi9, 9, Input<Floating>, exticr3, 8, tr9, mr9),
    PI10: (pi10, 10, Input<Floating>, exticr3, 8, tr10, mr10),
    PI11: (pi11, 11, Input<Floating>, exticr3, 8, tr11, mr11),
    PI12: (pi12, 12, Input<Floating>, exticr4, 8, tr12, mr12),
    PI13: (pi13, 13, Input<Floating>, exticr4, 8, tr13, mr13),
    PI14: (pi14, 14, Input<Floating>, exticr4, 8, tr14, mr14),
    PI15: (pi15, 15, Input<Floating>, exticr4, 8, tr15, mr15),
]);

#[cfg(any(
    feature = "stm32f469",
    feature = "stm32f479"
))]
gpio!(GPIOJ, gpioj, gpiojen, PJ, [
    PJ0: (pj0, 0, Input<Floating>, exticr1, 9, tr0, mr0),
    PJ1: (pj1, 1, Input<Floating>, exticr1, 9, tr1, mr1),
    PJ2: (pj2, 2, Input<Floating>, exticr1, 9, tr2, mr2),
    PJ3: (pj3, 3, Input<Floating>, exticr1, 9, tr3, mr3),
    PJ4: (pj4, 4, Input<Floating>, exticr2, 9, tr4, mr4),
    PJ5: (pj5, 5, Input<Floating>, exticr2, 9, tr5, mr5),
    PJ6: (pj6, 6, Input<Floating>, exticr2, 9, tr6, mr6),
    PJ7: (pj7, 7, Input<Floating>, exticr2, 9, tr7, mr7),
    PJ8: (pj8, 8, Input<Floating>, exticr3, 9, tr8, mr8),
    PJ9: (pj9, 9, Input<Floating>, exticr3, 9, tr9, mr9),
    PJ10: (pj10, 10, Input<Floating>, exticr3, 9, tr10, mr10),
    PJ11: (pj11, 11, Input<Floating>, exticr3, 9, tr11, mr11),
    PJ12: (pj12, 12, Input<Floating>, exticr4, 9, tr12, mr12),
    PJ13: (pj13, 13, Input<Floating>, exticr4, 9, tr13, mr13),
    PJ14: (pj14, 14, Input<Floating>, exticr4, 9, tr14, mr14),
    PJ15: (pj15, 15, Input<Floating>, exticr4, 9, tr15, mr15),
]);

#[cfg(any(
    feature = "stm32f469",
    feature = "stm32f479"
))]
gpio!(GPIOK, gpiok, gpioken, PK, [
    PK0: (pk0, 0, Input<Floating>, exticr1, 9, tr0, mr0),
    PK1: (pk1, 1, Input<Floating>, exticr1, 9, tr1, mr1),
    PK2: (pk2, 2, Input<Floating>, exticr1, 9, tr2, mr2),
    PK3: (pk3, 3, Input<Floating>, exticr1, 9, tr3, mr3),
    PK4: (pk4, 4, Input<Floating>, exticr2, 9, tr4, mr4),
    PK5: (pk5, 5, Input<Floating>, exticr2, 9, tr5, mr5),
    PK6: (pk6, 6, Input<Floating>, exticr2, 9, tr6, mr6),
    PK7: (pk7, 7, Input<Floating>, exticr2, 9, tr7, mr7),
]);
