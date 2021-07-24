use super::*;

/// Const assert hack
struct Assert<const L: u8, const R: u8>;

impl<const L: u8, const R: u8> Assert<L, R> {
    pub const LESS: u8 = R - L - 1;
}

impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Input<MODE>, P, N>>
    for Pin<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Analog, P, N>> for Pin<Alternate<A>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, const B: u8> From<Pin<AlternateOD<B>, P, N>>
    for Pin<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<B>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Input<MODE>, P, N>>
    for Pin<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Analog, P, N>>
    for Pin<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, const B: u8> From<Pin<Alternate<B>, P, N>>
    for Pin<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<B>, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<Floating>, P, N>> for Pin<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<Floating>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullUp>, P, N>> for Pin<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullUp>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
    for Pin<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<Floating>, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<Floating>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullDown>, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullDown>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Input<PullUp>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
    for Pin<Input<PullUp>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullDown>, P, N>> for Pin<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullDown>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullUp>, P, N>> for Pin<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullUp>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Input<MODE>, P, N>>
    for Pin<Output<OpenDrain>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8> From<Pin<Output<PushPull>, P, N>>
    for Pin<Output<OpenDrain>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<PushPull>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Output<OpenDrain>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Output<OpenDrain>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
    for Pin<Output<OpenDrain>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Input<MODE>, P, N>>
    for Pin<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8> From<Pin<Output<OpenDrain>, P, N>>
    for Pin<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<OpenDrain>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Output<PushPull>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
    for Pin<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Input<MODE>, P, N>> for Pin<Analog, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>> for Pin<Analog, P, N> {
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>> for Pin<Analog, P, N> {
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
    for Pin<Analog, P, N>
{
    #[inline(always)]
    fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<MODE, const P: char, const N: u8> Pin<MODE, P, N> {
    /// Configures the pin to operate alternate mode
    pub fn into_alternate<const A: u8>(self) -> Pin<Alternate<A>, P, N> {
        #[allow(path_statements, clippy::no_effect)]
        {
            Assert::<A, 16>::LESS;
        }
        let offset = 2 * { N };
        unsafe {
            if N < 8 {
                let offset2 = 4 * { N };
                (*Gpio::<P>::ptr()).afrl.modify(|r, w| {
                    w.bits((r.bits() & !(0b1111 << offset2)) | ((A as u32) << offset2))
                });
            } else {
                let offset2 = 4 * { N - 8 };
                (*Gpio::<P>::ptr()).afrh.modify(|r, w| {
                    w.bits((r.bits() & !(0b1111 << offset2)) | ((A as u32) << offset2))
                });
            }
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)));
        }
        Pin::new()
    }

    /// Configures the pin to operate in alternate open drain mode
    #[allow(path_statements)]
    pub fn into_alternate_open_drain<const A: u8>(self) -> Pin<AlternateOD<A>, P, N> {
        self.into_alternate::<A>().set_open_drain()
    }

    /// Configures the pin to operate in AF0 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0(self) -> Pin<Alternate<0>, P, N> {
        self.into_alternate::<0>()
    }

    /// Configures the pin to operate in AF1 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1(self) -> Pin<Alternate<1>, P, N> {
        self.into_alternate::<1>()
    }

    /// Configures the pin to operate in AF2 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2(self) -> Pin<Alternate<2>, P, N> {
        self.into_alternate::<2>()
    }

    /// Configures the pin to operate in AF3 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3(self) -> Pin<Alternate<3>, P, N> {
        self.into_alternate::<3>()
    }

    /// Configures the pin to operate in AF4 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4(self) -> Pin<Alternate<4>, P, N> {
        self.into_alternate::<4>()
    }

    /// Configures the pin to operate in AF5 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5(self) -> Pin<Alternate<5>, P, N> {
        self.into_alternate::<5>()
    }

    /// Configures the pin to operate in AF6 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6(self) -> Pin<Alternate<6>, P, N> {
        self.into_alternate::<6>()
    }

    /// Configures the pin to operate in AF7 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7(self) -> Pin<Alternate<7>, P, N> {
        self.into_alternate::<7>()
    }

    /// Configures the pin to operate in AF8 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8(self) -> Pin<Alternate<8>, P, N> {
        self.into_alternate::<8>()
    }

    /// Configures the pin to operate in AF9 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9(self) -> Pin<Alternate<9>, P, N> {
        self.into_alternate::<9>()
    }

    /// Configures the pin to operate in AF10 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10(self) -> Pin<Alternate<10>, P, N> {
        self.into_alternate::<10>()
    }

    /// Configures the pin to operate in AF11 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11(self) -> Pin<Alternate<11>, P, N> {
        self.into_alternate::<11>()
    }

    /// Configures the pin to operate in AF12 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12(self) -> Pin<Alternate<12>, P, N> {
        self.into_alternate::<12>()
    }

    /// Configures the pin to operate in AF13 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af13(self) -> Pin<Alternate<13>, P, N> {
        self.into_alternate::<13>()
    }

    /// Configures the pin to operate in AF14 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14(self) -> Pin<Alternate<14>, P, N> {
        self.into_alternate::<14>()
    }

    /// Configures the pin to operate in AF15 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15(self) -> Pin<Alternate<15>, P, N> {
        self.into_alternate::<15>()
    }

    /// Configures the pin to operate in AF0 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0_open_drain(self) -> Pin<AlternateOD<0>, P, N> {
        self.into_alternate_open_drain::<0>()
    }

    /// Configures the pin to operate in AF1 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1_open_drain(self) -> Pin<AlternateOD<1>, P, N> {
        self.into_alternate_open_drain::<1>()
    }

    /// Configures the pin to operate in AF2 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2_open_drain(self) -> Pin<AlternateOD<2>, P, N> {
        self.into_alternate_open_drain::<2>()
    }

    /// Configures the pin to operate in AF3 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3_open_drain(self) -> Pin<AlternateOD<3>, P, N> {
        self.into_alternate_open_drain::<3>()
    }

    /// Configures the pin to operate in AF4 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4_open_drain(self) -> Pin<AlternateOD<4>, P, N> {
        self.into_alternate_open_drain::<4>()
    }

    /// Configures the pin to operate in AF5 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5_open_drain(self) -> Pin<AlternateOD<5>, P, N> {
        self.into_alternate_open_drain::<5>()
    }

    /// Configures the pin to operate in AF6 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6_open_drain(self) -> Pin<AlternateOD<6>, P, N> {
        self.into_alternate_open_drain::<6>()
    }

    /// Configures the pin to operate in AF7 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7_open_drain(self) -> Pin<AlternateOD<7>, P, N> {
        self.into_alternate_open_drain::<7>()
    }

    /// Configures the pin to operate in AF8 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8_open_drain(self) -> Pin<AlternateOD<8>, P, N> {
        self.into_alternate_open_drain::<8>()
    }

    /// Configures the pin to operate in AF9 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9_open_drain(self) -> Pin<AlternateOD<9>, P, N> {
        self.into_alternate_open_drain::<9>()
    }

    /// Configures the pin to operate in AF10 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10_open_drain(self) -> Pin<AlternateOD<10>, P, N> {
        self.into_alternate_open_drain::<10>()
    }

    /// Configures the pin to operate in AF11 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11_open_drain(self) -> Pin<AlternateOD<11>, P, N> {
        self.into_alternate_open_drain::<11>()
    }

    /// Configures the pin to operate in AF12 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12_open_drain(self) -> Pin<AlternateOD<12>, P, N> {
        self.into_alternate_open_drain::<12>()
    }

    /// Configures the pin to operate in AF13 open drain modev
    pub fn into_alternate_af13_open_drain(self) -> Pin<AlternateOD<13>, P, N> {
        self.into_alternate_open_drain::<13>()
    }

    /// Configures the pin to operate in AF14 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14_open_drain(self) -> Pin<AlternateOD<14>, P, N> {
        self.into_alternate_open_drain::<14>()
    }

    /// Configures the pin to operate in AF15 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15_open_drain(self) -> Pin<AlternateOD<15>, P, N> {
        self.into_alternate_open_drain::<15>()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(mut self) -> Pin<Input<Floating>, P, N> {
        self.mode::<Input<Floating>>();
        Pin::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(mut self) -> Pin<Input<PullDown>, P, N> {
        self.mode::<Input<PullDown>>();
        Pin::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(mut self) -> Pin<Input<PullUp>, P, N> {
        self.mode::<Input<PullUp>>();
        Pin::new()
    }

    /// Configures the pin to operate as an open drain output pin
    pub fn into_open_drain_output(mut self) -> Pin<Output<OpenDrain>, P, N> {
        self.mode::<Output<OpenDrain>>();
        Pin::new()
    }

    /// Configures the pin to operate as an push pull output pin
    pub fn into_push_pull_output(mut self) -> Pin<Output<PushPull>, P, N> {
        self.mode::<Output<PushPull>>();
        Pin::new()
    }

    /// Configures the pin to operate as an analog input pin
    pub fn into_analog(mut self) -> Pin<Analog, P, N> {
        self.mode::<Analog>();
        Pin::new()
    }

    /// Puts `self` into mode `M`.
    ///
    /// This violates the type state constraints from `MODE`, so callers must
    /// ensure they use this properly.
    #[inline(always)]
    fn mode<M: PinMode>(&mut self) {
        let offset = 2 * N;
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (M::PUPDR << offset)));

            if let Some(otyper) = M::OTYPER {
                (*Gpio::<P>::ptr())
                    .otyper
                    .modify(|r, w| w.bits(r.bits() & !(0b1 << N) | (otyper << N)));
            }

            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (M::MODER << offset)));
        }
    }
}

impl<MODE, const P: char, const N: u8> Pin<MODE, P, N>
where
    MODE: PinMode,
{
    fn with_mode<M, F, R>(&mut self, f: F) -> R
    where
        M: PinMode,
        F: FnOnce(&mut Pin<M, P, N>) -> R,
    {
        self.mode::<M>();

        // This will reset the pin back to the original mode when dropped.
        // (so either when `with_mode` returns or when `f` unwinds)
        let _resetti = ResetMode { pin: self };

        let mut witness = Pin::new();

        f(&mut witness)
    }

    /// Temporarily configures this pin as a floating input.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_floating_input<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<Input<Floating>, P, N>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a pulled-down input.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_pull_down_input<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<Input<PullDown>, P, N>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a pulled-up input.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_pull_up_input<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<Input<PullUp>, P, N>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an analog pin.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_analog<R>(&mut self, f: impl FnOnce(&mut Pin<Analog, P, N>) -> R) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an open drain output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_open_drain_output<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<Output<OpenDrain>, P, N>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a push-pull output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_push_pull_output<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<Output<PushPull>, P, N>) -> R,
    ) -> R {
        self.with_mode(f)
    }
}

struct ResetMode<'a, ORIG: PinMode, const P: char, const N: u8> {
    pin: &'a mut Pin<ORIG, P, N>,
}

impl<'a, ORIG: PinMode, const P: char, const N: u8> Drop for ResetMode<'a, ORIG, P, N> {
    fn drop(&mut self) {
        self.pin.mode::<ORIG>();
    }
}

/// Marker trait for valid pin modes (type state).
///
/// It can not be implemented by outside types.
pub trait PinMode: crate::Sealed {
    // These constants are used to implement the pin configuration code.
    // They are not part of public API.

    #[doc(hidden)]
    const PUPDR: u32;
    #[doc(hidden)]
    const MODER: u32;
    #[doc(hidden)]
    const OTYPER: Option<u32> = None;
}

impl crate::Sealed for Input<Floating> {}
impl PinMode for Input<Floating> {
    const PUPDR: u32 = 0b00;
    const MODER: u32 = 0b00;
}

impl crate::Sealed for Input<PullDown> {}
impl PinMode for Input<PullDown> {
    const PUPDR: u32 = 0b10;
    const MODER: u32 = 0b00;
}

impl crate::Sealed for Input<PullUp> {}
impl PinMode for Input<PullUp> {
    const PUPDR: u32 = 0b01;
    const MODER: u32 = 0b00;
}

impl crate::Sealed for Analog {}
impl PinMode for Analog {
    const PUPDR: u32 = 0b00;
    const MODER: u32 = 0b11;
}

impl crate::Sealed for Output<OpenDrain> {}
impl PinMode for Output<OpenDrain> {
    const PUPDR: u32 = 0b00;
    const MODER: u32 = 0b01;
    const OTYPER: Option<u32> = Some(0b1);
}

impl crate::Sealed for Output<PushPull> {}
impl PinMode for Output<PushPull> {
    const PUPDR: u32 = 0b00;
    const MODER: u32 = 0b01;
    const OTYPER: Option<u32> = Some(0b0);
}
