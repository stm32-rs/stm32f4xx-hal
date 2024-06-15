use core::fmt;

use enumflags2::BitFlags;
use nb::block;

use super::{
    config, CFlag, Error, Event, Flag, Rx, RxISR, RxListen, Serial, SerialExt, Tx, TxISR, TxListen,
};
use crate::dma::{
    traits::{DMASet, PeriAddress},
    MemoryToPeripheral, PeripheralToMemory,
};
use crate::gpio::{alt::SerialAsync as CommonPins, NoPin, PushPull};
use crate::rcc::{self, Clocks};

#[cfg(feature = "uart4")]
pub(crate) use crate::pac::uart4::RegisterBlock as RegisterBlockUart;
pub(crate) use crate::pac::usart1::RegisterBlock as RegisterBlockUsart;

#[cfg(feature = "uart4")]
impl crate::Sealed for RegisterBlockUart {}
impl crate::Sealed for RegisterBlockUsart {}

// Implemented by all USART/UART instances
pub trait Instance:
    crate::Sealed
    + rcc::Enable
    + rcc::Reset
    + rcc::BusClock
    + CommonPins
    + core::ops::Deref<Target = Self::RegisterBlock>
{
    type RegisterBlock: RegisterBlockImpl;

    #[doc(hidden)]
    fn ptr() -> *const Self::RegisterBlock;
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
    #[doc(hidden)]
    fn peri_address() -> u32;
    #[doc(hidden)]
    unsafe fn steal() -> Self;
}

pub trait RegisterBlockImpl: crate::Sealed {
    #[allow(clippy::new_ret_no_self)]
    fn new<UART: Instance<RegisterBlock = Self>, WORD>(
        uart: UART,
        pins: (impl Into<UART::Tx<PushPull>>, impl Into<UART::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<UART, WORD>, config::InvalidConfig>;

    fn read_u16(&self) -> nb::Result<u16, Error>;
    fn write_u16(&self, word: u16) -> nb::Result<(), Error>;

    fn read_u8(&self) -> nb::Result<u8, Error> {
        // Delegate to u16 version, then truncate to 8 bits
        self.read_u16().map(|word16| word16 as u8)
    }

    fn write_u8(&self, word: u8) -> nb::Result<(), Error> {
        // Delegate to u16 version
        self.write_u16(u16::from(word))
    }

    fn flush(&self) -> nb::Result<(), Error>;

    fn bread_all_u8(&self, buffer: &mut [u8]) -> Result<(), Error> {
        for b in buffer.iter_mut() {
            *b = nb::block!(self.read_u8())?;
        }
        Ok(())
    }

    fn bread_all_u16(&self, buffer: &mut [u16]) -> Result<(), Error> {
        for b in buffer.iter_mut() {
            *b = nb::block!(self.read_u16())?;
        }
        Ok(())
    }

    fn bwrite_all_u8(&self, buffer: &[u8]) -> Result<(), Error> {
        for &b in buffer {
            nb::block!(self.write_u8(b))?;
        }
        Ok(())
    }

    fn bwrite_all_u16(&self, buffer: &[u16]) -> Result<(), Error> {
        for &b in buffer {
            nb::block!(self.write_u16(b))?;
        }
        Ok(())
    }

    fn bflush(&self) -> Result<(), Error> {
        nb::block!(self.flush())
    }

    // ISR
    fn flags(&self) -> BitFlags<Flag>;

    fn is_idle(&self) -> bool {
        self.flags().contains(Flag::Idle)
    }
    fn is_rx_not_empty(&self) -> bool {
        self.flags().contains(Flag::RxNotEmpty)
    }
    fn is_tx_empty(&self) -> bool {
        self.flags().contains(Flag::TxEmpty)
    }
    fn clear_flags(&self, flags: BitFlags<CFlag>);
    fn clear_idle_interrupt(&self);
    fn check_and_clear_error_flags(&self) -> Result<(), Error>;
    fn enable_error_interrupt_generation(&self);
    fn disable_error_interrupt_generation(&self);

    // Listen
    fn listen_event(&self, disable: Option<BitFlags<Event>>, enable: Option<BitFlags<Event>>);

    #[inline(always)]
    fn listen_rxne(&self) {
        self.listen_event(None, Some(Event::RxNotEmpty.into()))
    }
    #[inline(always)]
    fn unlisten_rxne(&self) {
        self.listen_event(Some(Event::RxNotEmpty.into()), None)
    }
    #[inline(always)]
    fn listen_idle(&self) {
        self.listen_event(None, Some(Event::Idle.into()))
    }
    #[inline(always)]
    fn unlisten_idle(&self) {
        self.listen_event(Some(Event::Idle.into()), None)
    }
    #[inline(always)]
    fn listen_txe(&self) {
        self.listen_event(None, Some(Event::TxEmpty.into()))
    }
    #[inline(always)]
    fn unlisten_txe(&self) {
        self.listen_event(Some(Event::TxEmpty.into()), None)
    }

    // PeriAddress
    fn peri_address(&self) -> u32;
}

macro_rules! uartCommon {
    () => {
        fn read_u16(&self) -> nb::Result<u16, Error> {
            // NOTE(unsafe) atomic read with no side effects
            let sr = self.sr().read();

            // Any error requires the dr to be read to clear
            if sr.pe().bit_is_set()
                || sr.fe().bit_is_set()
                || sr.nf().bit_is_set()
                || sr.ore().bit_is_set()
            {
                self.dr().read();
            }

            Err(if sr.pe().bit_is_set() {
                Error::Parity.into()
            } else if sr.fe().bit_is_set() {
                Error::FrameFormat.into()
            } else if sr.nf().bit_is_set() {
                Error::Noise.into()
            } else if sr.ore().bit_is_set() {
                Error::Overrun.into()
            } else if sr.rxne().bit_is_set() {
                // NOTE(unsafe) atomic read from stateless register
                return Ok(self.dr().read().dr().bits());
            } else {
                nb::Error::WouldBlock
            })
        }

        fn write_u16(&self, word: u16) -> nb::Result<(), Error> {
            // NOTE(unsafe) atomic read with no side effects
            let sr = self.sr().read();

            if sr.txe().bit_is_set() {
                // NOTE(unsafe) atomic write to stateless register
                self.dr().write(|w| w.dr().set(word));
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }

        fn flush(&self) -> nb::Result<(), Error> {
            // NOTE(unsafe) atomic read with no side effects
            let sr = self.sr().read();

            if sr.tc().bit_is_set() {
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }

        fn flags(&self) -> BitFlags<Flag> {
            BitFlags::from_bits_truncate(self.sr().read().bits())
        }

        fn clear_flags(&self, flags: BitFlags<CFlag>) {
            self.sr()
                .write(|w| unsafe { w.bits(0xffff & !flags.bits()) });
        }

        fn clear_idle_interrupt(&self) {
            let _ = self.sr().read();
            let _ = self.dr().read();
        }

        fn check_and_clear_error_flags(&self) -> Result<(), Error> {
            let sr = self.sr().read();
            let _ = self.dr().read();

            if sr.ore().bit_is_set() {
                Err(Error::Overrun)
            } else if sr.nf().bit_is_set() {
                Err(Error::Noise)
            } else if sr.fe().bit_is_set() {
                Err(Error::FrameFormat)
            } else if sr.pe().bit_is_set() {
                Err(Error::Parity)
            } else {
                Ok(())
            }
        }

        fn enable_error_interrupt_generation(&self) {
            self.cr3().modify(|_, w| w.eie().enabled());
        }

        fn disable_error_interrupt_generation(&self) {
            self.cr3().modify(|_, w| w.eie().disabled());
        }

        fn listen_event(&self, disable: Option<BitFlags<Event>>, enable: Option<BitFlags<Event>>) {
            self.cr1().modify(|r, w| unsafe {
                w.bits({
                    let mut bits = r.bits();
                    if let Some(d) = disable {
                        bits &= !(d.bits() as u32);
                    }
                    if let Some(e) = enable {
                        bits |= e.bits() as u32;
                    }
                    bits
                })
            });
        }

        fn peri_address(&self) -> u32 {
            self.dr().as_ptr() as u32
        }
    };
}

impl RegisterBlockImpl for RegisterBlockUsart {
    fn new<UART: Instance<RegisterBlock = Self>, WORD>(
        uart: UART,
        pins: (impl Into<UART::Tx<PushPull>>, impl Into<UART::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<UART, WORD>, config::InvalidConfig>
where {
        use self::config::*;

        let config = config.into();
        unsafe {
            // Enable clock.
            UART::enable_unchecked();
            UART::reset_unchecked();
        }

        let pclk_freq = UART::clock(clocks).raw();
        let baud = config.baudrate.0;

        // The frequency to calculate USARTDIV is this:
        //
        // (Taken from STM32F411xC/E Reference Manual,
        // Section 19.3.4, Equation 1)
        //
        // 16 bit oversample: OVER8 = 0
        // 8 bit oversample:  OVER8 = 1
        //
        // USARTDIV =          (pclk)
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // BUT, the USARTDIV has 4 "fractional" bits, which effectively
        // means that we need to "correct" the equation as follows:
        //
        // USARTDIV =      (pclk) * 16
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // When OVER8 is enabled, we can only use the lowest three
        // fractional bits, so we'll need to shift those last four bits
        // right one bit
        //
        // In IrDA Smartcard, LIN, and IrDA modes, OVER8 is always disabled.
        //
        // (Taken from STM32F411xC/E Reference Manual,
        // Section 19.3.4, Equation 2)
        //
        // USARTDIV =   pclk
        //            ---------
        //            16 x baud
        //
        // With reference to the above, OVER8 == 0 when in Smartcard, LIN, and
        // IrDA modes, so the register value needed for USARTDIV is the same
        // as for 16 bit oversampling.

        // Calculate correct baudrate divisor on the fly
        let (over8, div) = if (pclk_freq / 16) >= baud || config.irda != IrdaMode::None {
            // We have the ability to oversample to 16 bits, take
            // advantage of it.
            //
            // We also add `baud / 2` to the `pclk_freq` to ensure
            // rounding of values to the closest scale, rather than the
            // floored behavior of normal integer division.
            let div = (pclk_freq + (baud / 2)) / baud;
            (false, div)
        } else if (pclk_freq / 8) >= baud {
            // We are close enough to pclk where we can only
            // oversample 8.
            //
            // See note above regarding `baud` and rounding.
            let div = ((pclk_freq * 2) + (baud / 2)) / baud;

            // Ensure the the fractional bits (only 3) are
            // right-aligned.
            let frac = div & 0xF;
            let div = (div & !0xF) | (frac >> 1);
            (true, div)
        } else {
            return Err(config::InvalidConfig);
        };

        uart.brr().write(|w| unsafe { w.bits(div) });

        // Reset other registers to disable advanced USART features
        uart.cr2().reset();
        uart.cr3().reset(); // IrDA configuration - see STM32F411xC/E (RM0383) sections:
                            // 19.3.12 "IrDA SIR ENDEC block"
                            // 19.6.7 "Guard time and prescaler register (USART_GTPR)"
        if config.irda != IrdaMode::None && config.stopbits != StopBits::STOP1 {
            return Err(config::InvalidConfig);
        }

        match config.irda {
            IrdaMode::Normal => unsafe {
                uart.gtpr().reset();
                uart.cr3().write(|w| w.iren().enabled());
                uart.gtpr().write(|w| w.psc().bits(1u8))
            },
            IrdaMode::LowPower => unsafe {
                uart.gtpr().reset();
                uart.cr3().write(|w| w.iren().enabled().irlp().low_power());
                // FIXME
                uart.gtpr()
                    .write(|w| w.psc().bits((1843200u32 / pclk_freq) as u8))
            },
            IrdaMode::None => {}
        }

        // Enable transmission and receiving
        // and configure frame

        uart.cr1().write(|w| {
            w.ue().set_bit();
            w.over8().bit(over8);
            w.te().set_bit();
            w.re().set_bit();
            w.m().bit(config.wordlength == WordLength::DataBits9);
            w.pce().bit(config.parity != Parity::ParityNone);
            w.ps().bit(config.parity == Parity::ParityOdd)
        });

        match config.dma {
            DmaConfig::Tx => uart.cr3().write(|w| w.dmat().enabled()),
            DmaConfig::Rx => uart.cr3().write(|w| w.dmar().enabled()),
            DmaConfig::TxRx => uart.cr3().write(|w| w.dmar().enabled().dmat().enabled()),
            DmaConfig::None => {}
        }

        let serial = Serial {
            tx: Tx::new(uart, pins.0.into()),
            rx: Rx::new(unsafe { UART::steal() }, pins.1.into()),
        };
        serial.tx.usart.set_stopbits(config.stopbits);
        Ok(serial)
    }

    uartCommon! {}
}

#[cfg(feature = "uart4")]
impl RegisterBlockImpl for RegisterBlockUart {
    fn new<UART: Instance<RegisterBlock = Self>, WORD>(
        uart: UART,
        pins: (impl Into<UART::Tx<PushPull>>, impl Into<UART::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<UART, WORD>, config::InvalidConfig>
where {
        use self::config::*;

        let config = config.into();
        unsafe {
            // Enable clock.
            UART::enable_unchecked();
            UART::reset_unchecked();
        }

        let pclk_freq = UART::clock(clocks).raw();
        let baud = config.baudrate.0;

        // The frequency to calculate USARTDIV is this:
        //
        // (Taken from STM32F411xC/E Reference Manual,
        // Section 19.3.4, Equation 1)
        //
        // 16 bit oversample: OVER8 = 0
        // 8 bit oversample:  OVER8 = 1
        //
        // USARTDIV =          (pclk)
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // BUT, the USARTDIV has 4 "fractional" bits, which effectively
        // means that we need to "correct" the equation as follows:
        //
        // USARTDIV =      (pclk) * 16
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // When OVER8 is enabled, we can only use the lowest three
        // fractional bits, so we'll need to shift those last four bits
        // right one bit

        // Calculate correct baudrate divisor on the fly
        let (over8, div) = if (pclk_freq / 16) >= baud {
            // We have the ability to oversample to 16 bits, take
            // advantage of it.
            //
            // We also add `baud / 2` to the `pclk_freq` to ensure
            // rounding of values to the closest scale, rather than the
            // floored behavior of normal integer division.
            let div = (pclk_freq + (baud / 2)) / baud;
            (false, div)
        } else if (pclk_freq / 8) >= baud {
            // We are close enough to pclk where we can only
            // oversample 8.
            //
            // See note above regarding `baud` and rounding.
            let div = ((pclk_freq * 2) + (baud / 2)) / baud;

            // Ensure the the fractional bits (only 3) are
            // right-aligned.
            let frac = div & 0xF;
            let div = (div & !0xF) | (frac >> 1);
            (true, div)
        } else {
            return Err(config::InvalidConfig);
        };

        uart.brr().write(|w| unsafe { w.bits(div) });

        // Reset other registers to disable advanced USART features
        uart.cr2().reset();
        uart.cr3().reset();

        // Enable transmission and receiving
        // and configure frame

        uart.cr1().write(|w| {
            w.ue().set_bit();
            w.over8().bit(over8);
            w.te().set_bit();
            w.re().set_bit();
            w.m().bit(config.wordlength == WordLength::DataBits9);
            w.pce().bit(config.parity != Parity::ParityNone);
            w.ps().bit(config.parity == Parity::ParityOdd)
        });

        match config.dma {
            DmaConfig::Tx => uart.cr3().write(|w| w.dmat().enabled()),
            DmaConfig::Rx => uart.cr3().write(|w| w.dmar().enabled()),
            DmaConfig::TxRx => uart.cr3().write(|w| w.dmar().enabled().dmat().enabled()),
            DmaConfig::None => {}
        }

        let serial = Serial {
            tx: Tx::new(uart, pins.0.into()),
            rx: Rx::new(unsafe { UART::steal() }, pins.1.into()),
        };
        serial.tx.usart.set_stopbits(config.stopbits);
        Ok(serial)
    }

    uartCommon! {}
}

impl<UART: Instance, WORD> RxISR for Serial<UART, WORD>
where
    Rx<UART, WORD>: RxISR,
{
    fn is_idle(&self) -> bool {
        self.rx.is_idle()
    }

    fn is_rx_not_empty(&self) -> bool {
        self.rx.is_rx_not_empty()
    }

    /// This clears `Idle`, `Overrun`, `Noise`, `FrameError` and `ParityError` flags
    fn clear_idle_interrupt(&self) {
        self.rx.clear_idle_interrupt();
    }
}

impl<UART: Instance, WORD> RxISR for Rx<UART, WORD> {
    fn is_idle(&self) -> bool {
        self.usart.is_idle()
    }

    fn is_rx_not_empty(&self) -> bool {
        self.usart.is_rx_not_empty()
    }

    /// This clears `Idle`, `Overrun`, `Noise`, `FrameError` and `ParityError` flags
    fn clear_idle_interrupt(&self) {
        self.usart.clear_idle_interrupt();
    }
}

impl<UART: Instance, WORD> TxISR for Serial<UART, WORD>
where
    Tx<UART, WORD>: TxISR,
{
    fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }
}

impl<UART: Instance, WORD> TxISR for Tx<UART, WORD> {
    fn is_tx_empty(&self) -> bool {
        self.usart.is_tx_empty()
    }
}

impl<UART: Instance, WORD> RxListen for Rx<UART, WORD> {
    fn listen(&mut self) {
        self.usart.listen_rxne()
    }

    fn unlisten(&mut self) {
        self.usart.unlisten_rxne()
    }

    fn listen_idle(&mut self) {
        self.usart.listen_idle()
    }

    fn unlisten_idle(&mut self) {
        self.usart.unlisten_idle()
    }
}

impl<UART: Instance, WORD> TxListen for Tx<UART, WORD> {
    fn listen(&mut self) {
        self.usart.listen_txe()
    }

    fn unlisten(&mut self) {
        self.usart.unlisten_txe()
    }
}

impl<UART: Instance, WORD> crate::ClearFlags for Serial<UART, WORD> {
    type Flag = CFlag;

    #[inline(always)]
    fn clear_flags(&mut self, flags: impl Into<BitFlags<Self::Flag>>) {
        self.tx.usart.clear_flags(flags.into())
    }
}

impl<UART: Instance, WORD> crate::ReadFlags for Serial<UART, WORD> {
    type Flag = Flag;

    #[inline(always)]
    fn flags(&self) -> BitFlags<Self::Flag> {
        self.tx.usart.flags()
    }
}

impl<UART: Instance, WORD> crate::Listen for Serial<UART, WORD> {
    type Event = Event;

    #[inline(always)]
    fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tx.usart.listen_event(None, Some(event.into()));
    }

    #[inline(always)]
    fn listen_only(&mut self, event: impl Into<BitFlags<Self::Event>>) {
        self.tx
            .usart
            .listen_event(Some(BitFlags::ALL), Some(event.into()));
    }

    #[inline(always)]
    fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tx.usart.listen_event(Some(event.into()), None);
    }
}

impl<UART: Instance> fmt::Write for Serial<UART>
where
    Tx<UART>: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<UART: Instance> fmt::Write for Tx<UART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| block!(self.usart.write_u8(c)))
            .map_err(|_| fmt::Error)
    }
}

impl<UART: Instance> SerialExt for UART {
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig> {
        Serial::new(self, pins, config, clocks)
    }
    fn tx<WORD>(
        self,
        tx_pin: impl Into<Self::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Rx<PushPull>>,
    {
        Serial::tx(self, tx_pin, config, clocks)
    }
    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Tx<PushPull>>,
    {
        Serial::rx(self, rx_pin, config, clocks)
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD> {
    pub fn tx(
        usart: UART,
        tx_pin: impl Into<UART::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<UART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<UART::Rx<PushPull>>,
    {
        Self::new(usart, (tx_pin, NoPin::new()), config, clocks).map(|s| s.split().0)
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD> {
    pub fn rx(
        usart: UART,
        rx_pin: impl Into<UART::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<UART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<UART::Tx<PushPull>>,
    {
        Self::new(usart, (NoPin::new(), rx_pin), config, clocks).map(|s| s.split().1)
    }
}

unsafe impl<UART: Instance> PeriAddress for Rx<UART, u8> {
    #[inline(always)]
    fn address(&self) -> u32 {
        self.usart.peri_address()
    }

    type MemSize = u8;
}

unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory>
    for Rx<UART>
where
    UART: DMASet<STREAM, CHANNEL, PeripheralToMemory>,
{
}

unsafe impl<UART: Instance> PeriAddress for Tx<UART, u8> {
    #[inline(always)]
    fn address(&self) -> u32 {
        self.usart.peri_address()
    }

    type MemSize = u8;
}

unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral>
    for Tx<UART>
where
    UART: DMASet<STREAM, CHANNEL, MemoryToPeripheral>,
{
}
