#[cfg(feature = "f4")]
use crate::pacext::uart::{Cr3W, SrR, UartRB};

#[cfg(feature = "f7")]
use crate::pacext::uart::UartRB;

use super::{config, config::IrdaMode, CFlag, Error, Event, Flag};
use enumflags2::BitFlags;

pub trait RegisterBlockImpl: UartRB {
    const IRDA: bool;
    fn configure_irda(&self, irda: IrdaMode, pclk_freq: u32);
    fn set_stopbits(&self, bits: config::StopBits);

    fn read_u16(&self) -> nb::Result<u16, Error> {
        // NOTE(unsafe) atomic read with no side effects
        #[cfg(feature = "uart_v2")]
        let sr = self.sr().read();
        #[cfg(feature = "uart_v3")]
        let sr = self.isr().read();

        // Any error requires the dr to be read to clear
        #[cfg(feature = "uart_v2")]
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            self.dr().read();
        }

        #[cfg(feature = "uart_v3")]
        let icr = self.icr();
        Err(if sr.pe().bit_is_set() {
            #[cfg(feature = "uart_v3")]
            icr.write(|w| w.pecf().clear());
            Error::Parity.into()
        } else if sr.fe().bit_is_set() {
            #[cfg(feature = "uart_v3")]
            icr.write(|w| w.fecf().clear());
            Error::FrameFormat.into()
        } else if sr.nf().bit_is_set() {
            #[cfg(feature = "uart_v3")]
            icr.write(|w| w.ncf().clear());
            Error::Noise.into()
        } else if sr.ore().bit_is_set() {
            #[cfg(feature = "uart_v3")]
            icr.write(|w| w.orecf().clear());
            Error::Overrun.into()
        } else if sr.rxne().bit_is_set() {
            // NOTE(unsafe) atomic read from stateless register
            #[cfg(feature = "uart_v2")]
            return Ok(self.dr().read().dr().bits());
            #[cfg(feature = "uart_v3")]
            return Ok(self.rdr().read().rdr().bits());
        } else {
            nb::Error::WouldBlock
        })
    }

    fn write_u16(&self, word: u16) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        #[cfg(feature = "uart_v2")]
        let sr = self.sr().read();
        #[cfg(feature = "uart_v3")]
        let sr = self.isr().read();

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            #[cfg(feature = "uart_v2")]
            self.dr().write(|w| w.dr().set(word));
            #[cfg(feature = "uart_v3")]
            self.tdr().write(|w| w.tdr().set(word));
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    #[inline(always)]
    fn read_u8(&self) -> nb::Result<u8, Error> {
        // Delegate to u16 version, then truncate to 8 bits
        self.read_u16().map(|word16| word16 as u8)
    }

    #[inline(always)]
    fn write_u8(&self, word: u8) -> nb::Result<(), Error> {
        // Delegate to u16 version
        self.write_u16(u16::from(word))
    }

    fn flush(&self) -> nb::Result<(), Error> {
        // NOTE(unsafe) atomic read with no side effects
        #[cfg(feature = "uart_v2")]
        let sr = self.sr().read();
        #[cfg(feature = "uart_v3")]
        let sr = self.isr().read();

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

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

    #[inline(always)]
    fn bflush(&self) -> Result<(), Error> {
        nb::block!(self.flush())
    }

    // ISR
    #[inline(always)]
    fn flags(&self) -> BitFlags<Flag> {
        #[cfg(feature = "uart_v2")]
        let sr = self.sr().read();
        #[cfg(feature = "uart_v3")]
        let sr = self.isr().read();
        BitFlags::from_bits_truncate(sr.bits())
    }

    #[inline(always)]
    fn is_idle(&self) -> bool {
        self.flags().contains(Flag::Idle)
    }
    #[inline(always)]
    fn is_rx_not_empty(&self) -> bool {
        self.flags().contains(Flag::RxNotEmpty)
    }
    #[inline(always)]
    fn is_tx_empty(&self) -> bool {
        self.flags().contains(Flag::TxEmpty)
    }
    #[inline(always)]
    fn clear_flags(&self, flags: BitFlags<CFlag>) {
        #[cfg(feature = "uart_v2")]
        self.sr().write(|w| unsafe { w.bits(!flags.bits()) });
        #[cfg(feature = "uart_v3")]
        self.icr().write(|w| unsafe { w.bits(flags.bits()) });
    }
    fn clear_idle_interrupt(&self) {
        #[cfg(feature = "uart_v2")]
        {
            let _ = self.sr().read();
            let _ = self.dr().read();
        }
        #[cfg(feature = "uart_v3")]
        self.icr().write(|w| w.idlecf().clear_bit_by_one());
    }
    #[cfg(feature = "uart_v2")]
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
    #[cfg(feature = "uart_v3")]
    fn check_and_clear_error_flags(&self) -> Result<(), Error> {
        let sr = self.isr().read();
        let icr = self.icr();

        if sr.pe().bit_is_set() {
            icr.write(|w| w.pecf().clear());
            Err(Error::Parity)
        } else if sr.fe().bit_is_set() {
            icr.write(|w| w.fecf().clear());
            Err(Error::FrameFormat)
        } else if sr.nf().bit_is_set() {
            icr.write(|w| w.ncf().clear());
            Err(Error::Noise)
        } else if sr.ore().bit_is_set() {
            icr.write(|w| w.orecf().clear());
            Err(Error::Overrun)
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

    // Listen
    fn listen_event(&self, disable: Option<BitFlags<Event>>, enable: Option<BitFlags<Event>>) {
        self.cr1().modify(|r, w| unsafe {
            w.bits({
                let mut bits = r.bits();
                if let Some(d) = disable {
                    bits &= !(d.bits());
                }
                if let Some(e) = enable {
                    bits |= e.bits();
                }
                bits
            })
        });
    }

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

    // PeriAddress for transfer data
    #[inline(always)]
    fn tx_peri_address(&self) -> u32 {
        #[cfg(feature = "uart_v2")]
        {
            self.dr().as_ptr() as u32
        }
        #[cfg(feature = "uart_v3")]
        {
            self.tdr().as_ptr() as u32
        }
    }

    // PeriAddress for receive data
    #[inline(always)]
    fn rx_peri_address(&self) -> u32 {
        #[cfg(feature = "uart_v2")]
        {
            self.dr().as_ptr() as u32
        }
        #[cfg(feature = "uart_v3")]
        {
            self.rdr().as_ptr() as u32
        }
    }

    fn enable_dma(&self, dc: config::DmaConfig) {
        use config::DmaConfig;
        match dc {
            DmaConfig::Tx => {
                self.cr3().write(|w| w.dmat().enabled());
            }
            DmaConfig::Rx => {
                self.cr3().write(|w| w.dmar().enabled());
            }
            DmaConfig::TxRx => {
                self.cr3().write(|w| {
                    w.dmar().enabled();
                    w.dmat().enabled()
                });
            }
            DmaConfig::None => {}
        }
    }
}

impl RegisterBlockImpl for crate::pac::usart1::RegisterBlock {
    const IRDA: bool = true;
    fn set_stopbits(&self, bits: config::StopBits) {
        use crate::pac::usart1::cr2::STOP;
        use config::StopBits;

        self.cr2().write(|w| {
            w.stop().variant(match bits {
                StopBits::STOP0P5 => STOP::Stop0p5,
                StopBits::STOP1 => STOP::Stop1,
                StopBits::STOP1P5 => STOP::Stop1p5,
                StopBits::STOP2 => STOP::Stop2,
            })
        });
    }
    fn configure_irda(&self, irda: IrdaMode, pclk_freq: u32) {
        match irda {
            IrdaMode::Normal => unsafe {
                self.gtpr().reset();
                self.cr3().write(|w| w.iren().enabled());
                self.gtpr().write(|w| w.psc().bits(1u8));
            },
            IrdaMode::LowPower => unsafe {
                self.gtpr().reset();
                self.cr3().write(|w| w.iren().enabled().irlp().low_power());
                // FIXME
                self.gtpr()
                    .write(|w| w.psc().bits((1843200u32 / pclk_freq) as u8));
            },
            IrdaMode::None => {}
        }
    }
}

#[cfg(feature = "f4")]
#[cfg(feature = "uart4")]
impl RegisterBlockImpl for crate::pac::uart4::RegisterBlock {
    const IRDA: bool = false;
    fn set_stopbits(&self, bits: config::StopBits) {
        use crate::pac::uart4::cr2::STOP;
        use config::StopBits;

        // StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART
        // STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
        self.cr2().write(|w| {
            w.stop().variant(match bits {
                StopBits::STOP0P5 | StopBits::STOP1 => STOP::Stop1,
                StopBits::STOP1P5 | StopBits::STOP2 => STOP::Stop2,
            })
        });
    }
    fn configure_irda(&self, _irda: IrdaMode, _pclk_freq: u32) {}
}
