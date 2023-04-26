use super::*;

/// Convert tuple or array of pins to output port
pub trait OutPort {
    type Target;
    fn outport(self) -> Self::Target;
}

macro_rules! out_port {
    ( $name:ident => $n:literal, ( $($i:tt),+ ), ( $($N:ident),+ )) => {
        pub struct $name<const P: char $(, const $N: u8)+> (
            $(pub Pin<P, $N, Output<PushPull>>,)+
        );

        impl<const P: char $(, const $N: u8)+> OutPort for ($(Pin<P, $N, Output<PushPull>>),+) {
            type Target = $name<P $(, $N)+>;
            fn outport(self) -> Self::Target {
                $name($(self.$i),+)
            }
        }

        /// Wrapper for tuple of `Pin`s
        impl<const P: char $(, const $N: u8)+> $name<P $(, $N)+> {
            const fn mask() -> u32 {
                0 $( | (1 << { $N }))+
            }
            const fn value_for_write_bsrr(val: u32) -> u32 {
                0 $( | (1 << (if val & (1 << $i) != 0 { $N } else { $N + 16 })))+
            }

            #[doc=concat!("Set/reset pins according to `", $n, "` lower bits")]
            #[inline(never)]
            pub fn write(&mut self, word: u32) {
                unsafe {
                    (*Gpio::<P>::ptr())
                        .bsrr
                        .write(|w| w.bits(Self::value_for_write_bsrr(word)))
                }
            }

            /// Set all pins to `PinState::High`
            pub fn all_high(&mut self) {
                unsafe {
                    (*Gpio::<P>::ptr())
                        .bsrr
                        .write(|w| w.bits(Self::mask()))
                }
            }

            /// Reset all pins to `PinState::Low`
            pub fn all_low(&mut self) {
                unsafe {
                    (*Gpio::<P>::ptr())
                        .bsrr
                        .write(|w| w.bits(Self::mask() << 16))
                }
            }
        }
    }
}

out_port!(OutPort2 => 2, (0, 1), (N0, N1));
out_port!(OutPort3 => 3, (0, 1, 2), (N0, N1, N2));
out_port!(OutPort4 => 4, (0, 1, 2, 3), (N0, N1, N2, N3));
out_port!(OutPort5 => 5, (0, 1, 2, 3, 4), (N0, N1, N2, N3, N4));
out_port!(OutPort6 => 6, (0, 1, 2, 3, 4, 5), (N0, N1, N2, N3, N4, N5));
out_port!(OutPort7 => 7, (0, 1, 2, 3, 4, 5, 6), (N0, N1, N2, N3, N4, N5, N6));
out_port!(OutPort8 => 8, (0, 1, 2, 3, 4, 5, 6, 7), (N0, N1, N2, N3, N4, N5, N6, N7));

/// Wrapper for array of `PartiallyErasedPin`s
pub struct OutPortArray<const P: char, const SIZE: usize>(pub [PEPin<P, Output<PushPull>>; SIZE]);

impl<const P: char, const SIZE: usize> OutPort for [PEPin<P, Output<PushPull>>; SIZE] {
    type Target = OutPortArray<P, SIZE>;
    fn outport(self) -> Self::Target {
        OutPortArray(self)
    }
}

impl<const P: char, const SIZE: usize> OutPortArray<P, SIZE> {
    fn mask(&self) -> u32 {
        let mut msk = 0;
        for pin in self.0.iter() {
            msk |= 1 << pin.i;
        }
        msk
    }
    fn value_for_write_bsrr(&self, val: u32) -> u32 {
        let mut msk = 0;
        for (idx, pin) in self.0.iter().enumerate() {
            let n = pin.i;
            msk |= 1 << (if val & (1 << idx) != 0 { n } else { n + 16 });
        }
        msk
    }

    /// Set/reset pins according to `SIZE` lower bits
    #[inline(never)]
    pub fn write(&mut self, word: u32) {
        unsafe {
            (*Gpio::<P>::ptr())
                .bsrr
                .write(|w| w.bits(self.value_for_write_bsrr(word)))
        }
    }

    /// Set all pins to `PinState::High`
    pub fn all_high(&mut self) {
        unsafe { (*Gpio::<P>::ptr()).bsrr.write(|w| w.bits(self.mask())) }
    }

    /// Reset all pins to `PinState::Low`
    pub fn all_low(&mut self) {
        unsafe {
            (*Gpio::<P>::ptr())
                .bsrr
                .write(|w| w.bits(self.mask() << 16))
        }
    }
}
