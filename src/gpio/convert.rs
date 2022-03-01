use super::*;

/// Const assert hack
struct Assert<const L: u8, const R: u8>;

impl<const L: u8, const R: u8> Assert<L, R> {
    pub const LESS: u8 = R - L - 1;
}

impl<const P: char, const N: u8, const A: u8, MODE> From<Pin<P, N, Input<MODE>>>
    for Pin<P, N, Alternate<A, PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Input<MODE>>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, MODE> From<Pin<P, N, Output<MODE>>>
    for Pin<P, N, Alternate<A, PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Output<MODE>>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Analog>>
    for Pin<P, N, Alternate<A, PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, const B: u8> From<Pin<P, N, Alternate<B, OpenDrain>>>
    for Pin<P, N, Alternate<A, PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<B, OpenDrain>>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, MODE> From<Pin<P, N, Input<MODE>>>
    for Pin<P, N, Alternate<A, OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Input<MODE>>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, MODE> From<Pin<P, N, Output<MODE>>>
    for Pin<P, N, Alternate<A, OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Output<MODE>>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Analog>>
    for Pin<P, N, Alternate<A, OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, const B: u8> From<Pin<P, N, Alternate<B, PushPull>>>
    for Pin<P, N, Alternate<A, OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<B, PushPull>>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Input<Floating>>> for Pin<P, N, Input<PullDown>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<Floating>>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Input<PullUp>>> for Pin<P, N, Input<PullDown>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<PullUp>>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Output<MODE>>>
    for Pin<P, N, Input<PullDown>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Output<MODE>>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Analog>> for Pin<P, N, Input<PullDown>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, PushPull>>>
    for Pin<P, N, Input<PullDown>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, PushPull>>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, OpenDrain>>>
    for Pin<P, N, Input<PullDown>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, OpenDrain>>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Input<Floating>>> for Pin<P, N, Input<PullUp>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<Floating>>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Input<PullDown>>> for Pin<P, N, Input<PullUp>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<PullDown>>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Output<MODE>>> for Pin<P, N, Input<PullUp>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Output<MODE>>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Analog>> for Pin<P, N, Input<PullUp>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, PushPull>>>
    for Pin<P, N, Input<PullUp>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, PushPull>>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, OpenDrain>>>
    for Pin<P, N, Input<PullUp>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, OpenDrain>>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Input<PullDown>>> for Pin<P, N, Input<Floating>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<PullDown>>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Input<PullUp>>> for Pin<P, N, Input<Floating>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<PullUp>>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Output<MODE>>>
    for Pin<P, N, Input<Floating>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Output<MODE>>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Analog>> for Pin<P, N, Input<Floating>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, PushPull>>>
    for Pin<P, N, Input<Floating>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, PushPull>>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, OpenDrain>>>
    for Pin<P, N, Input<Floating>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, OpenDrain>>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Input<MODE>>>
    for Pin<P, N, Output<OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Input<MODE>>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Output<PushPull>>>
    for Pin<P, N, Output<OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Output<PushPull>>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Analog>> for Pin<P, N, Output<OpenDrain>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, PushPull>>>
    for Pin<P, N, Output<OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, PushPull>>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, OpenDrain>>>
    for Pin<P, N, Output<OpenDrain>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, OpenDrain>>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Input<MODE>>>
    for Pin<P, N, Output<PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Input<MODE>>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Output<OpenDrain>>>
    for Pin<P, N, Output<PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Output<OpenDrain>>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Analog>> for Pin<P, N, Output<PushPull>> {
    #[inline(always)]
    fn from(f: Pin<P, N, Analog>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, PushPull>>>
    for Pin<P, N, Output<PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, PushPull>>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, OpenDrain>>>
    for Pin<P, N, Output<PushPull>>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, OpenDrain>>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Input<MODE>>> for Pin<P, N, Analog> {
    #[inline(always)]
    fn from(f: Pin<P, N, Input<MODE>>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, Output<MODE>>> for Pin<P, N, Analog> {
    #[inline(always)]
    fn from(f: Pin<P, N, Output<MODE>>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, PushPull>>>
    for Pin<P, N, Analog>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, PushPull>>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<P, N, Alternate<A, OpenDrain>>>
    for Pin<P, N, Analog>
{
    #[inline(always)]
    fn from(f: Pin<P, N, Alternate<A, OpenDrain>>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    pub(super) fn set_alternate<const A: u8>(&mut self) {
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
    }
    /// Configures the pin to operate alternate mode
    pub fn into_alternate<const A: u8>(mut self) -> Pin<P, N, Alternate<A, PushPull>> {
        self.set_alternate::<A>();
        Pin::new()
    }

    /// Configures the pin to operate in alternate open drain mode
    #[allow(path_statements)]
    pub fn into_alternate_open_drain<const A: u8>(self) -> Pin<P, N, Alternate<A, OpenDrain>> {
        self.into_alternate::<A>().set_open_drain()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(mut self) -> Pin<P, N, Input<Floating>> {
        self.mode::<Input<Floating>>();
        Pin::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(mut self) -> Pin<P, N, Input<PullDown>> {
        self.mode::<Input<PullDown>>();
        Pin::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(mut self) -> Pin<P, N, Input<PullUp>> {
        self.mode::<Input<PullUp>>();
        Pin::new()
    }

    /// Configures the pin to operate as an open drain output pin
    /// Initial state will be low.
    pub fn into_open_drain_output(mut self) -> Pin<P, N, Output<OpenDrain>> {
        self.mode::<Output<OpenDrain>>();
        Pin::new()
    }

    /// Configures the pin to operate as an open-drain output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_open_drain_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<P, N, Output<OpenDrain>> {
        self._set_state(initial_state);
        self.mode::<Output<OpenDrain>>();
        Pin::new()
    }

    /// Configures the pin to operate as an push pull output pin
    /// Initial state will be low.
    pub fn into_push_pull_output(mut self) -> Pin<P, N, Output<PushPull>> {
        self._set_low();
        self.mode::<Output<PushPull>>();
        Pin::new()
    }

    /// Configures the pin to operate as an push-pull output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_push_pull_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<P, N, Output<PushPull>> {
        self._set_state(initial_state);
        self.mode::<Output<PushPull>>();
        Pin::new()
    }

    /// Configures the pin to operate as an analog input pin
    pub fn into_analog(mut self) -> Pin<P, N, Analog> {
        self.mode::<Analog>();
        Pin::new()
    }

    /// Configures the pin as a pin that can change between input
    /// and output without changing the type. It starts out
    /// as a floating input
    pub fn into_dynamic(self) -> DynamicPin<P, N> {
        self.into_floating_input();
        DynamicPin::new(Dynamic::InputFloating)
    }

    /// Puts `self` into mode `M`.
    ///
    /// This violates the type state constraints from `MODE`, so callers must
    /// ensure they use this properly.
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
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

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: PinMode,
{
    fn with_mode<M, F, R>(&mut self, f: F) -> R
    where
        M: PinMode,
        F: FnOnce(&mut Pin<P, N, M>) -> R,
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
        f: impl FnOnce(&mut Pin<P, N, Input<Floating>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a pulled-down input.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_pull_down_input<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<P, N, Input<PullDown>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a pulled-up input.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_pull_up_input<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<P, N, Input<PullUp>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an analog pin.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_analog<R>(&mut self, f: impl FnOnce(&mut Pin<P, N, Analog>) -> R) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an open drain output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// The value of the pin after conversion is undefined. If you
    /// want to control it, use `with_open_drain_output_in_state`
    pub fn with_open_drain_output<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<P, N, Output<OpenDrain>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an open drain output .
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// Note that the new state is set slightly before conversion
    /// happens. This can cause a short output glitch if switching
    /// between output modes
    pub fn with_open_drain_output_in_state<R>(
        &mut self,
        state: PinState,
        f: impl FnOnce(&mut Pin<P, N, Output<OpenDrain>>) -> R,
    ) -> R {
        self._set_state(state);
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a push-pull output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// The value of the pin after conversion is undefined. If you
    /// want to control it, use `with_push_pull_output_in_state`
    pub fn with_push_pull_output<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<P, N, Output<PushPull>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a push-pull output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// Note that the new state is set slightly before conversion
    /// happens. This can cause a short output glitch if switching
    /// between output modes
    pub fn with_push_pull_output_in_state<R>(
        &mut self,
        state: PinState,
        f: impl FnOnce(&mut Pin<P, N, Output<PushPull>>) -> R,
    ) -> R {
        self._set_state(state);
        self.with_mode(f)
    }
}

struct ResetMode<'a, const P: char, const N: u8, ORIG: PinMode> {
    pin: &'a mut Pin<P, N, ORIG>,
}

impl<'a, const P: char, const N: u8, ORIG: PinMode> Drop for ResetMode<'a, P, N, ORIG> {
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
