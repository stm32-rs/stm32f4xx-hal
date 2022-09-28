use super::*;

impl<USART: Instance, WORD> Rx<USART, WORD> {
    /// Start listening for an rx not empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) }
    }

    /// Stop listening for the rx not empty interrupt event
    pub fn unlisten(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) }
    }

    /// Start listening for a line idle interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen_idle(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) }
    }

    /// Stop listening for the line idle interrupt event
    pub fn unlisten_idle(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) }
    }

    /// Return true if the line idle status is set
    pub fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().idle().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    /// Clear idle line interrupt flag
    pub fn clear_idle_interrupt(&self) {
        unsafe {
            let _ = (*USART::ptr()).sr.read();
            let _ = (*USART::ptr()).dr.read();
        }
    }
}

impl<USART: Instance, WORD> Tx<USART, WORD> {
    /// Start listening for a tx empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) }
    }

    /// Stop listening for the tx empty interrupt event
    pub fn unlisten(&mut self) {
        unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) }
    }

    /// Return true if the tx register is empty (and can accept data)
    pub fn is_tx_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }
}

impl<USART: Instance, PINS, WORD> Serial<USART, PINS, WORD> {
    /// Starts listening for an interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) },
            Event::Txe => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) },
            Event::Idle => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) },
        }
    }

    /// Stop listening for an interrupt event
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) },
            Event::Txe => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) },
            Event::Idle => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) },
        }
    }

    /// Return true if the line idle status is set
    pub fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().idle().bit_is_set() }
    }

    /// Return true if the tx register is empty (and can accept data)
    pub fn is_tx_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    /// Clear idle line interrupt flag
    pub fn clear_idle_interrupt(&self) {
        unsafe {
            let _ = (*USART::ptr()).sr.read();
            let _ = (*USART::ptr()).dr.read();
        }
    }
}

impl<USART: Instance, PINS> fmt::Write for Serial<USART, PINS> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<USART: Instance> fmt::Write for Tx<USART> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| block!(self.write(c)))
            .map_err(|_| fmt::Error)
    }
}

impl<USART: Instance> Rx<USART, u8> {
    pub(super) fn read(&mut self) -> nb::Result<u8, Error> {
        // Delegate to the Read<u16> implementation, then truncate to 8 bits
        Rx::<USART, u16>::new().read().map(|word16| word16 as u8)
    }
}

impl<USART: Instance> Rx<USART, u16> {
    pub(super) fn read(&mut self) -> nb::Result<u16, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        // Any error requires the dr to be read to clear
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*USART::ptr()).dr.read() };
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
            return Ok(unsafe { &*USART::ptr() }.dr.read().dr().bits());
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl<USART: Instance> Tx<USART, u8> {
    pub(super) fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        // Delegate to u16 version
        Tx::<USART, u16>::new().write(u16::from(word))
    }

    pub(super) fn flush(&mut self) -> nb::Result<(), Error> {
        // Delegate to u16 version
        Tx::<USART, u16>::new().flush()
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

impl<USART: Instance> Tx<USART, u16> {
    pub(super) fn write(&mut self, word: u16) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            unsafe { &*USART::ptr() }.dr.write(|w| w.dr().bits(word));
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub(super) fn flush(&mut self) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

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
