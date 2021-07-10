use super::*;

/// Const assert hack
struct Assert<const L: u8, const R: u8>;

impl<const L: u8, const R: u8> Assert<L, R> {
    pub const LESS: u8 = R - L - 1;
}

fn _set_alternate_mode<const P: char, const N: u8, const A: u8>() {
    let offset = 2 * { N };
    let offset2 = 4 * { N };
    let mode = A as u32;
    unsafe {
        if offset2 < 32 {
            (*Gpio::<P>::ptr())
                .afrl
                .modify(|r, w| w.bits((r.bits() & !(0b1111 << offset2)) | (mode << offset2)));
        } else {
            let offset2 = offset2 - 32;
            (*Gpio::<P>::ptr())
                .afrh
                .modify(|r, w| w.bits((r.bits() & !(0b1111 << offset2)) | (mode << offset2)));
        }
        (*Gpio::<P>::ptr())
            .moder
            .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)));
    }
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
        _set_alternate_mode::<P, N, A>();
        Pin::new()
    }

    /// Configures the pin to operate in alternate open drain mode
    #[allow(path_statements)]
    pub fn into_alternate_open_drain<const A: u8>(self) -> Pin<AlternateOD<A>, P, N> {
        #[allow(path_statements, clippy::no_effect)]
        {
            Assert::<A, 16>::LESS;
        }
        _set_alternate_mode::<P, N, A>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF0 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0(self) -> Pin<Alternate<0>, P, N> {
        _set_alternate_mode::<P, N, 0>();
        Pin::new()
    }

    /// Configures the pin to operate in AF1 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1(self) -> Pin<Alternate<1>, P, N> {
        _set_alternate_mode::<P, N, 1>();
        Pin::new()
    }

    /// Configures the pin to operate in AF2 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2(self) -> Pin<Alternate<2>, P, N> {
        _set_alternate_mode::<P, N, 2>();
        Pin::new()
    }

    /// Configures the pin to operate in AF3 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3(self) -> Pin<Alternate<3>, P, N> {
        _set_alternate_mode::<P, N, 3>();
        Pin::new()
    }

    /// Configures the pin to operate in AF4 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4(self) -> Pin<Alternate<4>, P, N> {
        _set_alternate_mode::<P, N, 4>();
        Pin::new()
    }

    /// Configures the pin to operate in AF5 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5(self) -> Pin<Alternate<5>, P, N> {
        _set_alternate_mode::<P, N, 5>();
        Pin::new()
    }

    /// Configures the pin to operate in AF6 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6(self) -> Pin<Alternate<6>, P, N> {
        _set_alternate_mode::<P, N, 6>();
        Pin::new()
    }

    /// Configures the pin to operate in AF7 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7(self) -> Pin<Alternate<7>, P, N> {
        _set_alternate_mode::<P, N, 7>();
        Pin::new()
    }

    /// Configures the pin to operate in AF8 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8(self) -> Pin<Alternate<8>, P, N> {
        _set_alternate_mode::<P, N, 8>();
        Pin::new()
    }

    /// Configures the pin to operate in AF9 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9(self) -> Pin<Alternate<9>, P, N> {
        _set_alternate_mode::<P, N, 9>();
        Pin::new()
    }

    /// Configures the pin to operate in AF10 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10(self) -> Pin<Alternate<10>, P, N> {
        _set_alternate_mode::<P, N, 10>();
        Pin::new()
    }

    /// Configures the pin to operate in AF11 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11(self) -> Pin<Alternate<11>, P, N> {
        _set_alternate_mode::<P, N, 11>();
        Pin::new()
    }

    /// Configures the pin to operate in AF12 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12(self) -> Pin<Alternate<12>, P, N> {
        _set_alternate_mode::<P, N, 12>();
        Pin::new()
    }

    /// Configures the pin to operate in AF13 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af13(self) -> Pin<Alternate<13>, P, N> {
        _set_alternate_mode::<P, N, 13>();
        Pin::new()
    }

    /// Configures the pin to operate in AF14 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14(self) -> Pin<Alternate<14>, P, N> {
        _set_alternate_mode::<P, N, 14>();
        Pin::new()
    }

    /// Configures the pin to operate in AF15 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15(self) -> Pin<Alternate<15>, P, N> {
        _set_alternate_mode::<P, N, 15>();
        Pin::new()
    }

    /// Configures the pin to operate in AF0 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0_open_drain(self) -> Pin<AlternateOD<0>, P, N> {
        _set_alternate_mode::<P, N, 0>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF1 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1_open_drain(self) -> Pin<AlternateOD<1>, P, N> {
        _set_alternate_mode::<P, N, 1>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF2 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2_open_drain(self) -> Pin<AlternateOD<2>, P, N> {
        _set_alternate_mode::<P, N, 2>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF3 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3_open_drain(self) -> Pin<AlternateOD<3>, P, N> {
        _set_alternate_mode::<P, N, 3>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF4 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4_open_drain(self) -> Pin<AlternateOD<4>, P, N> {
        _set_alternate_mode::<P, N, 4>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF5 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5_open_drain(self) -> Pin<AlternateOD<5>, P, N> {
        _set_alternate_mode::<P, N, 5>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF6 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6_open_drain(self) -> Pin<AlternateOD<6>, P, N> {
        _set_alternate_mode::<P, N, 6>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF7 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7_open_drain(self) -> Pin<AlternateOD<7>, P, N> {
        _set_alternate_mode::<P, N, 7>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF8 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8_open_drain(self) -> Pin<AlternateOD<8>, P, N> {
        _set_alternate_mode::<P, N, 8>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF9 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9_open_drain(self) -> Pin<AlternateOD<9>, P, N> {
        _set_alternate_mode::<P, N, 9>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF10 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10_open_drain(self) -> Pin<AlternateOD<10>, P, N> {
        _set_alternate_mode::<P, N, 10>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF11 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11_open_drain(self) -> Pin<AlternateOD<11>, P, N> {
        _set_alternate_mode::<P, N, 11>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF12 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12_open_drain(self) -> Pin<AlternateOD<12>, P, N> {
        _set_alternate_mode::<P, N, 12>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF13 open drain modev
    pub fn into_alternate_af13_open_drain(self) -> Pin<AlternateOD<13>, P, N> {
        _set_alternate_mode::<P, N, 13>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF14 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14_open_drain(self) -> Pin<AlternateOD<14>, P, N> {
        _set_alternate_mode::<P, N, 14>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate in AF15 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15_open_drain(self) -> Pin<AlternateOD<15>, P, N> {
        _set_alternate_mode::<P, N, 15>();
        Pin::new().set_open_drain()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(self) -> Pin<Input<Floating>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        Pin::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(self) -> Pin<Input<PullDown>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        Pin::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(self) -> Pin<Input<PullUp>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        Pin::new()
    }

    /// Configures the pin to operate as an open drain output pin
    pub fn into_open_drain_output(self) -> Pin<Output<OpenDrain>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .otyper
                .modify(|r, w| w.bits(r.bits() | (0b1 << { N })));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)))
        };

        Pin::new()
    }

    /// Configures the pin to operate as an push pull output pin
    pub fn into_push_pull_output(self) -> Pin<Output<PushPull>, P, N> {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .otyper
                .modify(|r, w| w.bits(r.bits() & !(0b1 << { N })));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)))
        };

        Pin::new()
    }

    /// Configures the pin to operate as an analog input pin
    pub fn into_analog(self) -> Pin<Analog, P, N> {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b11 << offset)))
        };

        Pin::new()
    }
}
