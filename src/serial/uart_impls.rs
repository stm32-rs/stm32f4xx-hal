#[cfg(features = "dma")]
use crate::dma::{traits::DMASet, MemoryToPeripheral, PeripheralToMemory};

use super::*;

impl<UART: CommonPins> Rx<UART, u8> {
    pub(super) fn with_u16_data(self) -> Rx<UART, u16> {
        Rx::new(self.pin)
    }
}

impl<UART: CommonPins> Rx<UART, u16> {
    pub(super) fn with_u8_data(self) -> Rx<UART, u8> {
        Rx::new(self.pin)
    }
}

impl<UART: CommonPins> Tx<UART, u8> {
    pub(super) fn with_u16_data(self) -> Tx<UART, u16> {
        Tx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins> Tx<UART, u16> {
    pub(super) fn with_u8_data(self) -> Tx<UART, u8> {
        Tx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins, WORD> Rx<UART, WORD> {
    pub(super) fn new(pin: UART::Rx<PushPull>) -> Self {
        Self {
            _word: PhantomData,
            pin,
        }
    }
}

impl<UART: CommonPins, WORD> Tx<UART, WORD> {
    pub(super) fn new(usart: UART, pin: UART::Tx<PushPull>) -> Self {
        Self {
            _word: PhantomData,
            usart,
            pin,
        }
    }
}

impl<UART: Instance, WORD> AsRef<Tx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<UART, WORD> {
        &self.tx
    }
}

impl<UART: Instance, WORD> AsRef<Rx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<UART, WORD> {
        &self.rx
    }
}

impl<UART: Instance, WORD> AsMut<Tx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<UART, WORD> {
        &mut self.tx
    }
}

impl<UART: Instance, WORD> AsMut<Rx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<UART, WORD> {
        &mut self.rx
    }
}

impl<UART: Instance, WORD> Rx<UART, WORD> {
    pub fn join<TX>(self, tx: Tx<UART, WORD>) -> Serial<UART, WORD> {
        Serial { tx, rx: self }
    }

    /// Start listening for an rx not empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self) {
        unsafe { (*UART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) }
    }

    /// Stop listening for the rx not empty interrupt event
    pub fn unlisten(&mut self) {
        unsafe { (*UART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) }
    }

    /// Start listening for a line idle interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen_idle(&mut self) {
        unsafe { (*UART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) }
    }

    /// Stop listening for the line idle interrupt event
    pub fn unlisten_idle(&mut self) {
        unsafe { (*UART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) }
    }
}

impl<UART: Instance, WORD> RxISR for Rx<UART, WORD> {
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool {
        #[cfg(feature = "uart_v1")]
        unsafe {
            (*UART::ptr()).sr.read().idle().bit_is_set()
        }
        #[cfg(feature = "uart_v2")]
        unsafe {
            (*UART::ptr()).isr.read().idle().bit_is_set()
        }
    }

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool {
        #[cfg(feature = "uart_v1")]
        unsafe {
            (*UART::ptr()).sr.read().rxne().bit_is_set()
        }
        #[cfg(feature = "uart_v2")]
        unsafe {
            (*UART::ptr()).isr.read().rxne().bit_is_set()
        }
    }

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self) {
        #[cfg(feature = "uart_v1")]
        unsafe {
            let _ = (*UART::ptr()).sr.read();
            let _ = (*UART::ptr()).dr.read();
        }
        #[cfg(feature = "uart_v2")]
        unsafe {
            (*UART::ptr()).icr.write(|w| w.idlecf().set_bit())
        };
    }
}

impl<UART: Instance, WORD> Tx<UART, WORD> {
    pub fn join(self, rx: Rx<UART, WORD>) -> Serial<UART, WORD> {
        Serial { tx: self, rx }
    }

    /// Start listening for a tx empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self) {
        unsafe { (*UART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) }
    }

    /// Stop listening for the tx empty interrupt event
    pub fn unlisten(&mut self) {
        unsafe { (*UART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) }
    }
}

impl<UART: Instance, WORD> TxISR for Tx<UART, WORD> {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool {
        #[cfg(feature = "uart_v1")]
        unsafe {
            (*UART::ptr()).sr.read().txe().bit_is_set()
        }
        #[cfg(feature = "uart_v2")]
        unsafe {
            (*UART::ptr()).isr.read().txe().bit_is_set()
        }
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD> {
    /// Starts listening for an interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*UART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) },
            Event::Txe => unsafe { (*UART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) },
            Event::Idle => unsafe { (*UART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) },
        }
    }

    /// Stop listening for an interrupt event
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*UART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) },
            Event::Txe => unsafe { (*UART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) },
            Event::Idle => unsafe { (*UART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) },
        }
    }

    pub fn split(self) -> (Tx<UART, WORD>, Rx<UART, WORD>) {
        (self.tx, self.rx)
    }
}

impl<UART: Instance, WORD> RxISR for Serial<UART, WORD> {
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool {
        self.rx.is_idle()
    }

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool {
        self.rx.is_rx_not_empty()
    }

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self) {
        self.rx.clear_idle_interrupt();
    }
}

impl<UART: Instance, WORD> TxISR for Serial<UART, WORD> {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }
}

impl<UART: Instance> fmt::Write for Serial<UART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<UART: Instance> fmt::Write for Tx<UART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| block!(self.write(c)))
            .map_err(|_| fmt::Error)
    }
}

impl<UART: Instance> Rx<UART, u8> {
    pub(super) fn read(&mut self) -> nb::Result<u8, Error> {
        // Delegate to the Read<u16> implementation, then truncate to 8 bits
        unsafe {
            (*(self as *mut Self as *mut Rx<UART, u16>))
                .read()
                .map(|word16| word16 as u8)
        }
    }
}

impl<UART: Instance> Rx<UART, u16> {
    pub(super) fn read(&mut self) -> nb::Result<u16, Error> {
        // NOTE(unsafe) atomic read with no side effects
        #[cfg(feature = "uart_v1")]
        let sr = unsafe { (*UART::ptr()).sr.read() };
        #[cfg(feature = "uart_v2")]
        let sr = unsafe { (*UART::ptr()).isr.read() };

        // Any error requires the dr to be read to clear
        #[cfg(feature = "uart_v1")]
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*UART::ptr()).dr.read() };
        }
        #[cfg(feature = "uart_v2")]
        let icr = unsafe { &(*UART::ptr()).icr };
        Err(if sr.pe().bit_is_set() {
            #[cfg(feature = "uart_v2")]
            icr.write(|w| w.pecf().clear());
            Error::Parity.into()
        } else if sr.fe().bit_is_set() {
            #[cfg(feature = "uart_v2")]
            icr.write(|w| w.fecf().clear());
            Error::FrameFormat.into()
        } else if sr.nf().bit_is_set() {
            #[cfg(feature = "uart_v2")]
            icr.write(|w| w.ncf().clear());
            Error::Noise.into()
        } else if sr.ore().bit_is_set() {
            #[cfg(feature = "uart_v2")]
            icr.write(|w| w.orecf().clear());
            Error::Overrun.into()
        } else if sr.rxne().bit_is_set() {
            // NOTE(unsafe) atomic read from stateless register
            #[cfg(feature = "uart_v1")]
            return Ok(unsafe { &*UART::ptr() }.dr.read().dr().bits());
            #[cfg(feature = "uart_v2")]
            return Ok(unsafe { &*UART::ptr() }.rdr.read().rdr().bits());
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl<UART: Instance> Tx<UART, u8> {
    pub(super) fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        // Delegate to u16 version
        unsafe { (*(self as *mut Self as *mut Tx<UART, u16>)).write(u16::from(word)) }
    }

    pub(super) fn flush(&mut self) -> nb::Result<(), Error> {
        // Delegate to u16 version
        unsafe { (*(self as *mut Self as *mut Tx<UART, u16>)).flush() }
    }

    pub(super) fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for &b in bytes {
            nb::block!(self.write(b))?;
        }
        Ok(())
    }

    pub(super) fn bflush(&mut self) -> Result<(), Error> {
        nb::block!(self.flush())
    }
}

impl<UART: Instance> Tx<UART, u16> {
    pub(super) fn write(&mut self, word: u16) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        #[cfg(feature = "uart_v1")]
        let sr = unsafe { (*UART::ptr()).sr.read() };
        #[cfg(feature = "uart_v2")]
        let sr = unsafe { (*UART::ptr()).isr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            #[cfg(feature = "uart_v1")]
            unsafe { &*UART::ptr() }.dr.write(|w| w.dr().bits(word));
            #[cfg(feature = "uart_v2")]
            unsafe { &*UART::ptr() }.tdr.write(|w| w.tdr().bits(word));
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub(super) fn flush(&mut self) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        #[cfg(feature = "uart_v1")]
        let sr = unsafe { (*UART::ptr()).sr.read() };
        #[cfg(feature = "uart_v2")]
        let sr = unsafe { (*UART::ptr()).isr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub(super) fn bwrite_all(&mut self, buffer: &[u16]) -> Result<(), Error> {
        for &b in buffer {
            nb::block!(self.write(b))?;
        }
        Ok(())
    }

    pub(super) fn bflush(&mut self) -> Result<(), Error> {
        nb::block!(self.flush())
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

impl<UART: Instance> Serial<UART, u8> {
    /// Converts this Serial into a version that can read and write `u16` values instead of `u8`s
    ///
    /// This can be used with a word length of 9 bits.
    pub fn with_u16_data(self) -> Serial<UART, u16> {
        Serial {
            tx: self.tx.with_u16_data(),
            rx: self.rx.with_u16_data(),
        }
    }
}

impl<UART: Instance> Serial<UART, u16> {
    /// Converts this Serial into a version that can read and write `u8` values instead of `u16`s
    ///
    /// This can be used with a word length of 8 bits.
    pub fn with_u8_data(self) -> Serial<UART, u8> {
        Serial {
            tx: self.tx.with_u8_data(),
            rx: self.rx.with_u8_data(),
        }
    }
}

#[cfg(features = "dma")]
unsafe impl<UART: Instance> PeriAddress for Rx<UART, u8> {
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*UART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

#[cfg(features = "dma")]
unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory>
    for Rx<UART>
where
    UART: DMASet<STREAM, CHANNEL, PeripheralToMemory>,
{
}

#[cfg(features = "dma")]
unsafe impl<UART: Instance> PeriAddress for Tx<UART, u8> {
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*UART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

#[cfg(features = "dma")]
unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral>
    for Tx<UART>
where
    UART: DMASet<STREAM, CHANNEL, MemoryToPeripheral>,
{
}
