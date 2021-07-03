use super::*;

impl<MODE, const P: char, const N: u8, const A: u8> From<PX<Input<MODE>, P, N>>
    for PX<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Input<MODE>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<PX<Output<MODE>, P, N>>
    for PX<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Output<MODE>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Analog, P, N>> for PX<Alternate<A>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, const B: u8> From<PX<AlternateOD<B>, P, N>>
    for PX<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: PX<AlternateOD<B>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<PX<Input<MODE>, P, N>>
    for PX<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Input<MODE>, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<PX<Output<MODE>, P, N>>
    for PX<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Output<MODE>, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Analog, P, N>> for PX<AlternateOD<A>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8, const B: u8> From<PX<Alternate<B>, P, N>>
    for PX<AlternateOD<A>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Alternate<B>, P, N>) -> Self {
        f.into_alternate_open_drain::<A>()
    }
}

impl<const P: char, const N: u8> From<PX<Input<Floating>, P, N>> for PX<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<Floating>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<PX<Input<PullUp>, P, N>> for PX<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<PullUp>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Output<MODE>, P, N>> for PX<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: PX<Output<MODE>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<PX<Analog, P, N>> for PX<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Alternate<A>, P, N>>
    for PX<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Alternate<A>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<AlternateOD<A>, P, N>>
    for PX<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: PX<AlternateOD<A>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<PX<Input<Floating>, P, N>> for PX<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<Floating>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<PX<Input<PullDown>, P, N>> for PX<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<PullDown>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Output<MODE>, P, N>> for PX<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: PX<Output<MODE>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<PX<Analog, P, N>> for PX<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Alternate<A>, P, N>>
    for PX<Input<PullUp>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Alternate<A>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<AlternateOD<A>, P, N>>
    for PX<Input<PullUp>, P, N>
{
    #[inline(always)]
    fn from(f: PX<AlternateOD<A>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<PX<Input<PullDown>, P, N>> for PX<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<PullDown>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<PX<Input<PullUp>, P, N>> for PX<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<PullUp>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Output<MODE>, P, N>> for PX<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: PX<Output<MODE>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<PX<Analog, P, N>> for PX<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Alternate<A>, P, N>>
    for PX<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Alternate<A>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<AlternateOD<A>, P, N>>
    for PX<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: PX<AlternateOD<A>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Input<MODE>, P, N>> for PX<Output<OpenDrain>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<MODE>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8> From<PX<Output<PushPull>, P, N>> for PX<Output<OpenDrain>, P, N> {
    #[inline(always)]
    fn from(f: PX<Output<PushPull>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8> From<PX<Analog, P, N>> for PX<Output<OpenDrain>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Alternate<A>, P, N>>
    for PX<Output<OpenDrain>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Alternate<A>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<AlternateOD<A>, P, N>>
    for PX<Output<OpenDrain>, P, N>
{
    #[inline(always)]
    fn from(f: PX<AlternateOD<A>, P, N>) -> Self {
        f.into_open_drain_output()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Input<MODE>, P, N>> for PX<Output<PushPull>, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<MODE>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8> From<PX<Output<OpenDrain>, P, N>> for PX<Output<PushPull>, P, N> {
    #[inline(always)]
    fn from(f: PX<Output<OpenDrain>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8> From<PX<Analog, P, N>> for PX<Output<PushPull>, P, N> {
    #[inline(always)]
    fn from(f: PX<Analog, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Alternate<A>, P, N>>
    for PX<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: PX<Alternate<A>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<AlternateOD<A>, P, N>>
    for PX<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: PX<AlternateOD<A>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Input<MODE>, P, N>> for PX<Analog, P, N> {
    #[inline(always)]
    fn from(f: PX<Input<MODE>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<MODE, const P: char, const N: u8> From<PX<Output<MODE>, P, N>> for PX<Analog, P, N> {
    #[inline(always)]
    fn from(f: PX<Output<MODE>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<Alternate<A>, P, N>> for PX<Analog, P, N> {
    #[inline(always)]
    fn from(f: PX<Alternate<A>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<const P: char, const N: u8, const A: u8> From<PX<AlternateOD<A>, P, N>> for PX<Analog, P, N> {
    #[inline(always)]
    fn from(f: PX<AlternateOD<A>, P, N>) -> Self {
        f.into_analog()
    }
}

impl<MODE, const P: char, const N: u8> PX<MODE, P, N> {
    /// Configures the pin to operate alternate mode
    pub fn into_alternate<const A: u8>(self) -> PX<Alternate<A>, P, N> {
        #[allow(path_statements, clippy::no_effect)]
        {
            Assert::<A, 16>::LESS;
        }
        _set_alternate_mode::<P, N, A>();
        PX::new()
    }

    /// Configures the pin to operate in alternate open drain mode
    #[allow(path_statements)]
    pub fn into_alternate_open_drain<const A: u8>(self) -> PX<AlternateOD<A>, P, N> {
        #[allow(path_statements, clippy::no_effect)]
        {
            Assert::<A, 16>::LESS;
        }
        _set_alternate_mode::<P, N, A>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF0 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0(self) -> PX<Alternate<0>, P, N> {
        _set_alternate_mode::<P, N, 0>();
        PX::new()
    }

    /// Configures the pin to operate in AF1 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1(self) -> PX<Alternate<1>, P, N> {
        _set_alternate_mode::<P, N, 1>();
        PX::new()
    }

    /// Configures the pin to operate in AF2 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2(self) -> PX<Alternate<2>, P, N> {
        _set_alternate_mode::<P, N, 2>();
        PX::new()
    }

    /// Configures the pin to operate in AF3 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3(self) -> PX<Alternate<3>, P, N> {
        _set_alternate_mode::<P, N, 3>();
        PX::new()
    }

    /// Configures the pin to operate in AF4 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4(self) -> PX<Alternate<4>, P, N> {
        _set_alternate_mode::<P, N, 4>();
        PX::new()
    }

    /// Configures the pin to operate in AF5 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5(self) -> PX<Alternate<5>, P, N> {
        _set_alternate_mode::<P, N, 5>();
        PX::new()
    }

    /// Configures the pin to operate in AF6 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6(self) -> PX<Alternate<6>, P, N> {
        _set_alternate_mode::<P, N, 6>();
        PX::new()
    }

    /// Configures the pin to operate in AF7 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7(self) -> PX<Alternate<7>, P, N> {
        _set_alternate_mode::<P, N, 7>();
        PX::new()
    }

    /// Configures the pin to operate in AF8 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8(self) -> PX<Alternate<8>, P, N> {
        _set_alternate_mode::<P, N, 8>();
        PX::new()
    }

    /// Configures the pin to operate in AF9 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9(self) -> PX<Alternate<9>, P, N> {
        _set_alternate_mode::<P, N, 9>();
        PX::new()
    }

    /// Configures the pin to operate in AF10 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10(self) -> PX<Alternate<10>, P, N> {
        _set_alternate_mode::<P, N, 10>();
        PX::new()
    }

    /// Configures the pin to operate in AF11 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11(self) -> PX<Alternate<11>, P, N> {
        _set_alternate_mode::<P, N, 11>();
        PX::new()
    }

    /// Configures the pin to operate in AF12 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12(self) -> PX<Alternate<12>, P, N> {
        _set_alternate_mode::<P, N, 12>();
        PX::new()
    }

    /// Configures the pin to operate in AF13 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af13(self) -> PX<Alternate<13>, P, N> {
        _set_alternate_mode::<P, N, 13>();
        PX::new()
    }

    /// Configures the pin to operate in AF14 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14(self) -> PX<Alternate<14>, P, N> {
        _set_alternate_mode::<P, N, 14>();
        PX::new()
    }

    /// Configures the pin to operate in AF15 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15(self) -> PX<Alternate<15>, P, N> {
        _set_alternate_mode::<P, N, 15>();
        PX::new()
    }

    /// Configures the pin to operate in AF0 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af0_open_drain(self) -> PX<AlternateOD<0>, P, N> {
        _set_alternate_mode::<P, N, 0>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF1 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af1_open_drain(self) -> PX<AlternateOD<1>, P, N> {
        _set_alternate_mode::<P, N, 1>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF2 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af2_open_drain(self) -> PX<AlternateOD<2>, P, N> {
        _set_alternate_mode::<P, N, 2>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF3 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af3_open_drain(self) -> PX<AlternateOD<3>, P, N> {
        _set_alternate_mode::<P, N, 3>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF4 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af4_open_drain(self) -> PX<AlternateOD<4>, P, N> {
        _set_alternate_mode::<P, N, 4>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF5 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af5_open_drain(self) -> PX<AlternateOD<5>, P, N> {
        _set_alternate_mode::<P, N, 5>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF6 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af6_open_drain(self) -> PX<AlternateOD<6>, P, N> {
        _set_alternate_mode::<P, N, 6>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF7 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af7_open_drain(self) -> PX<AlternateOD<7>, P, N> {
        _set_alternate_mode::<P, N, 7>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF8 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af8_open_drain(self) -> PX<AlternateOD<8>, P, N> {
        _set_alternate_mode::<P, N, 8>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF9 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af9_open_drain(self) -> PX<AlternateOD<9>, P, N> {
        _set_alternate_mode::<P, N, 9>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF10 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af10_open_drain(self) -> PX<AlternateOD<10>, P, N> {
        _set_alternate_mode::<P, N, 10>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF11 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af11_open_drain(self) -> PX<AlternateOD<11>, P, N> {
        _set_alternate_mode::<P, N, 11>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF12 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af12_open_drain(self) -> PX<AlternateOD<12>, P, N> {
        _set_alternate_mode::<P, N, 12>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF13 open drain modev
    pub fn into_alternate_af13_open_drain(self) -> PX<AlternateOD<13>, P, N> {
        _set_alternate_mode::<P, N, 13>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF14 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af14_open_drain(self) -> PX<AlternateOD<14>, P, N> {
        _set_alternate_mode::<P, N, 14>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate in AF15 open drain mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_af15_open_drain(self) -> PX<AlternateOD<15>, P, N> {
        _set_alternate_mode::<P, N, 15>();
        PX::new().set_open_drain()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(self) -> PX<Input<Floating>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(self) -> PX<Input<PullDown>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(self) -> PX<Input<PullUp>, P, N> {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)))
        };

        PX::new()
    }

    /// Configures the pin to operate as an open drain output pin
    pub fn into_open_drain_output(self) -> PX<Output<OpenDrain>, P, N> {
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

        PX::new()
    }

    /// Configures the pin to operate as an push pull output pin
    pub fn into_push_pull_output(self) -> PX<Output<PushPull>, P, N> {
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

        PX::new()
    }

    /// Configures the pin to operate as an analog input pin
    pub fn into_analog(self) -> PX<Analog, P, N> {
        let offset = 2 * { N };

        unsafe {
            (*Gpio::<P>::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
            (*Gpio::<P>::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b11 << offset)))
        };

        PX::new()
    }
}
