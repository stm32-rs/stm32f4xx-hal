use core::marker::PhantomData;

use crate::stm32::{DMA1, DMA2, RCC};

pub trait DmaExt: Sized {
    /// Enables the DMA peripheral and splits it into streams.
    fn split(
        self,
    ) -> (
        Stream0<Self>,
        Stream1<Self>,
        Stream2<Self>,
        Stream3<Self>,
        Stream4<Self>,
        Stream5<Self>,
        Stream6<Self>,
        Stream7<Self>,
    );
}

pub struct Stream0<DMA>(PhantomData<DMA>);
pub struct Stream1<DMA>(PhantomData<DMA>);
pub struct Stream2<DMA>(PhantomData<DMA>);
pub struct Stream3<DMA>(PhantomData<DMA>);
pub struct Stream4<DMA>(PhantomData<DMA>);
pub struct Stream5<DMA>(PhantomData<DMA>);
pub struct Stream6<DMA>(PhantomData<DMA>);
pub struct Stream7<DMA>(PhantomData<DMA>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InterruptBits {
    pub transfer_complete: bool,
    pub transfer_half: bool,
    pub transfer_error: bool,
    pub direct_mode_error: bool,
    pub fifo_error: bool,
}

impl InterruptBits {
    pub const ALL: Self = Self {
        transfer_complete: true,
        transfer_half: true,
        transfer_error: true,
        direct_mode_error: true,
        fifo_error: true,
    };
    pub const NONE: Self = Self {
        transfer_complete: false,
        transfer_half: false,
        transfer_error: false,
        direct_mode_error: false,
        fifo_error: false,
    };
}

macro_rules! impl_dma {
    ($dma:ident $dmaen:ident) => (
        impl DmaExt for $dma {
            #[inline]
            fn split(
                self,
            ) -> (
                Stream0<Self>,
                Stream1<Self>,
                Stream2<Self>,
                Stream3<Self>,
                Stream4<Self>,
                Stream5<Self>,
                Stream6<Self>,
                Stream7<Self>,
            ) {
                unsafe {
                    let rcc = &*RCC::ptr();
                    rcc.ahb1enr.modify(|_, w| w.$dmaen().set_bit());
                }
                (
                    Stream0(PhantomData),
                    Stream1(PhantomData),
                    Stream2(PhantomData),
                    Stream3(PhantomData),
                    Stream4(PhantomData),
                    Stream5(PhantomData),
                    Stream6(PhantomData),
                    Stream7(PhantomData),
                )
            }
        }

        impl_stream!($dma Stream0 0 lisr tcif0 htif0 teif0 dmeif0 feif0 lifcr ctcif0 chtif0 cteif0 cdmeif0 cfeif0);
        impl_stream!($dma Stream1 1 lisr tcif1 htif1 teif1 dmeif1 feif1 lifcr ctcif1 chtif1 cteif1 cdmeif1 cfeif1);
        impl_stream!($dma Stream2 2 lisr tcif2 htif2 teif2 dmeif2 feif2 lifcr ctcif2 chtif2 cteif2 cdmeif2 cfeif2);
        impl_stream!($dma Stream3 3 lisr tcif3 htif3 teif3 dmeif3 feif3 lifcr ctcif3 chtif3 cteif3 cdmeif3 cfeif3);
        impl_stream!($dma Stream4 4 hisr tcif4 htif4 teif4 dmeif4 feif4 hifcr ctcif4 chtif4 cteif4 cdmeif4 cfeif4);
        impl_stream!($dma Stream5 5 hisr tcif5 htif5 teif5 dmeif5 feif5 hifcr ctcif5 chtif5 cteif5 cdmeif5 cfeif5);
        impl_stream!($dma Stream6 6 hisr tcif6 htif6 teif6 dmeif6 feif6 hifcr ctcif6 chtif6 cteif6 cdmeif6 cfeif6);
        impl_stream!($dma Stream7 7 hisr tcif7 htif7 teif7 dmeif7 feif7 hifcr ctcif7 chtif7 cteif7 cdmeif7 cfeif7);
    );
}

macro_rules! impl_stream {
    (
        $dma:ident $stream:ident $index:tt
        $isr:ident $tcif:ident $htif:ident $teif:ident $dmeif:ident $feif:ident
        $ifcr:ident $ctcif:ident $chtif:ident $cteif:ident $cdmeif:ident $cfeif:ident
    ) => {
        impl $stream<$dma> {
            #[inline]
            pub fn st(&mut self) -> &ST {
                unsafe {
                    let dma = &*$dma::ptr();
                    &dma.st[$index]
                }
            }

            #[inline]
            pub fn interrupt_status(&mut self) -> InterruptBits {
                let dma = unsafe { &*$dma::ptr() };
                let r = dma.$isr.read();
                InterruptBits {
                    transfer_complete: r.$tcif().bit_is_set(),
                    transfer_half: r.$htif().bit_is_set(),
                    transfer_error: r.$teif().bit_is_set(),
                    direct_mode_error: r.$dmeif().bit_is_set(),
                    fifo_error: r.$feif().bit_is_set(),
                }
            }

            #[inline]
            pub fn clear_interrupt_bits(&mut self, bits: InterruptBits) {
                let dma = unsafe { &*$dma::ptr() };
                dma.$ifcr.write(|w| {
                    if bits.transfer_complete {
                        w.$ctcif().set_bit();
                    }
                    if bits.transfer_half {
                        w.$chtif().set_bit();
                    }
                    if bits.transfer_error {
                        w.$cteif().set_bit();
                    }
                    if bits.direct_mode_error {
                        w.$cdmeif().set_bit();
                    }
                    if bits.fifo_error {
                        w.$cfeif().set_bit();
                    }
                    w
                });
            }

            #[inline]
            pub fn clear_all_interrupt_bits(&mut self) {
                self.clear_interrupt_bits(InterruptBits::ALL);
            }
        }
    };
}

impl_dma!(DMA1 dma1en);
impl_dma!(DMA2 dma2en);

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f413",
    feature = "stm32f423",
))]
pub type ST = crate::stm32::dma1::ST;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
pub type ST = crate::stm32::dma2::ST;
