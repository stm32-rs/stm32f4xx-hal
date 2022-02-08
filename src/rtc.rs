//! Interface to the real time clock. See STM32F303 reference manual, section 27.
//! For more details, see
//! [ST AN4759](https:/www.st.com%2Fresource%2Fen%2Fapplication_note%2Fdm00226326-using-the-hardware-realtime-clock-rtc-and-the-tamper-management-unit-tamp-with-stm32-microcontrollers-stmicroelectronics.pdf&usg=AOvVaw3PzvL2TfYtwS32fw-Uv37h)

use crate::bb;
use crate::pac::rtc::{dr, tr};
use crate::pac::{rcc::RegisterBlock, PWR, RCC, RTC};
use crate::rcc::Enable;
use core::convert::TryInto;
use core::fmt;
use time::{Date, PrimitiveDateTime, Time};

/// Invalid input error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    InvalidInputData,
}

pub const LSE_BITS: u8 = 0b01;

/// Real Time Clock peripheral
pub struct Rtc {
    /// RTC Peripheral register
    pub regs: RTC,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Rtc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rtc");
    }
}

impl fmt::Debug for Rtc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Rtc")
    }
}

/// LSE clock mode.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEClockMode {
    /// Enable LSE oscillator to use external crystal or ceramic resonator.
    Oscillator,
    /// Bypass LSE oscillator to use external clock source.
    /// Use this if an external oscillator is used which is not connected to `OSC32_IN` such as a MEMS resonator.
    Bypass,
}

impl Rtc {
    /// Create and enable a new RTC, and configure its clock source and prescalers.
    /// From AN4759, Table 7, when using the LSE (The only clock source this module
    /// supports currently), set `prediv_s` to 255, and `prediv_a` to 127 to get a
    /// calendar clock of 1Hz.
    /// The `bypass` argument is `true` if you're using an external oscillator that
    /// doesn't connect to `OSC32_IN`, such as a MEMS resonator.
    pub fn new(regs: RTC, prediv_s: u16, prediv_a: u8, mode: LSEClockMode, pwr: &mut PWR) -> Self {
        let mut result = Self { regs };

        // Steps:
        // Enable PWR and DBP
        // Enable LSE (if needed)
        // Enable RTC Clock
        // Disable Write Protect
        // Enter Init
        // Configure 24 hour format
        // Set prescalers
        // Exit Init
        // Enable write protect

        unsafe {
            let rcc = &(*RCC::ptr());
            // As per the sample code, unlock comes first. (Enable PWR and DBP)
            unlock(rcc, pwr);
            // If necessary, enable the LSE.
            if rcc.bdcr.read().lserdy().bit_is_clear() {
                enable_lse(rcc, mode);
            }
            enable(rcc);
        }

        result.modify(|regs| {
            // Set 24 Hour
            regs.cr.modify(|_, w| w.fmt().clear_bit());
            // Set prescalers
            regs.prer.modify(|_, w| {
                w.prediv_s().bits(prediv_s);
                w.prediv_a().bits(prediv_a)
            })
        });

        result
    }

    /// As described in Section 27.3.7 in RM0316,
    /// this function is used to disable write protection
    /// when modifying an RTC register
    fn modify<F>(&mut self, mut closure: F)
    where
        F: FnMut(&mut RTC),
    {
        // Disable write protection
        self.regs.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.regs.wpr.write(|w| unsafe { w.bits(0x53) });
        // Enter init mode
        let isr = self.regs.isr.read();
        if isr.initf().bit_is_clear() {
            self.regs.isr.modify(|_, w| w.init().set_bit());
            while self.regs.isr.read().initf().bit_is_clear() {}
        }
        // Invoke closure
        closure(&mut self.regs);
        // Exit init mode
        self.regs.isr.modify(|_, w| w.init().clear_bit());
        // wait for last write to be done
        while !self.regs.isr.read().initf().bit_is_clear() {}

        // Enable write protection
        self.regs.wpr.write(|w| unsafe { w.bits(0xFF) });
    }

    /// Set the time using time::Time.
    pub fn set_time(&mut self, time: &Time) -> Result<(), Error> {
        let (ht, hu) = bcd2_encode(time.hour().into())?;
        let (mnt, mnu) = bcd2_encode(time.minute().into())?;
        let (st, su) = bcd2_encode(time.second().into())?;
        self.modify(|regs| {
            regs.tr.write(|w| {
                w.ht().bits(ht);
                w.hu().bits(hu);
                w.mnt().bits(mnt);
                w.mnu().bits(mnu);
                w.st().bits(st);
                w.su().bits(su);
                w.pm().clear_bit()
            })
        });

        Ok(())
    }

    /// Set the seconds [0-59].
    pub fn set_seconds(&mut self, seconds: u8) -> Result<(), Error> {
        if seconds > 59 {
            return Err(Error::InvalidInputData);
        }
        let (st, su) = bcd2_encode(seconds.into())?;
        self.modify(|regs| regs.tr.modify(|_, w| w.st().bits(st).su().bits(su)));

        Ok(())
    }

    /// Set the minutes [0-59].
    pub fn set_minutes(&mut self, minutes: u8) -> Result<(), Error> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        let (mnt, mnu) = bcd2_encode(minutes.into())?;
        self.modify(|regs| regs.tr.modify(|_, w| w.mnt().bits(mnt).mnu().bits(mnu)));

        Ok(())
    }

    /// Set the hours [0-23].
    pub fn set_hours(&mut self, hours: u8) -> Result<(), Error> {
        if hours > 23 {
            return Err(Error::InvalidInputData);
        }
        let (ht, hu) = bcd2_encode(hours.into())?;

        self.modify(|regs| regs.tr.modify(|_, w| w.ht().bits(ht).hu().bits(hu)));

        Ok(())
    }

    /// Set the day of week [1-7].
    pub fn set_weekday(&mut self, weekday: u8) -> Result<(), Error> {
        if !(1..=7).contains(&weekday) {
            return Err(Error::InvalidInputData);
        }
        self.modify(|regs| regs.dr.modify(|_, w| unsafe { w.wdu().bits(weekday) }));

        Ok(())
    }

    /// Set the day of month [1-31].
    pub fn set_day(&mut self, day: u8) -> Result<(), Error> {
        if !(1..=31).contains(&day) {
            return Err(Error::InvalidInputData);
        }
        let (dt, du) = bcd2_encode(day as u32)?;
        self.modify(|regs| regs.dr.modify(|_, w| w.dt().bits(dt).du().bits(du)));

        Ok(())
    }

    /// Set the month [1-12].
    pub fn set_month(&mut self, month: u8) -> Result<(), Error> {
        if !(1..=12).contains(&month) {
            return Err(Error::InvalidInputData);
        }
        let (mt, mu) = bcd2_encode(month as u32)?;
        self.modify(|regs| regs.dr.modify(|_, w| w.mt().bit(mt > 0).mu().bits(mu)));

        Ok(())
    }

    /// Set the year [1970-2069].
    ///
    /// The year cannot be less than 1970, since the Unix epoch is assumed (1970-01-01 00:00:00).
    /// Also, the year cannot be greater than 2069 since the RTC range is 0 - 99.
    pub fn set_year(&mut self, year: u16) -> Result<(), Error> {
        if !(1970..=2069).contains(&year) {
            return Err(Error::InvalidInputData);
        }
        let (yt, yu) = bcd2_encode(year as u32 - 1970)?;
        self.modify(|regs| regs.dr.modify(|_, w| w.yt().bits(yt).yu().bits(yu)));

        Ok(())
    }

    /// Set the date.
    ///
    /// The year cannot be less than 1970, since the Unix epoch is assumed (1970-01-01 00:00:00).
    /// Also, the year cannot be greater than 2069 since the RTC range is 0 - 99.
    pub fn set_date(&mut self, date: &Date) -> Result<(), Error> {
        if !(1970..=2069).contains(&date.year()) {
            return Err(Error::InvalidInputData);
        }

        let (yt, yu) = bcd2_encode((date.year() - 1970) as u32)?;
        let (mt, mu) = bcd2_encode(u8::from(date.month()).into())?;
        let (dt, du) = bcd2_encode(date.day().into())?;

        self.modify(|regs| {
            regs.dr.write(|w| {
                w.dt().bits(dt);
                w.du().bits(du);
                w.mt().bit(mt > 0);
                w.mu().bits(mu);
                w.yt().bits(yt);
                w.yu().bits(yu)
            })
        });

        Ok(())
    }

    /// Set the date and time.
    ///
    /// The year cannot be less than 1970, since the Unix epoch is assumed (1970-01-01 00:00:00).
    /// Also, the year cannot be greater than 2069 since the RTC range is 0 - 99.
    pub fn set_datetime(&mut self, date: &PrimitiveDateTime) -> Result<(), Error> {
        if !(1970..=2069).contains(&date.year()) {
            return Err(Error::InvalidInputData);
        }

        let (yt, yu) = bcd2_encode((date.year() - 1970) as u32)?;
        let (mt, mu) = bcd2_encode(u8::from(date.month()).into())?;
        let (dt, du) = bcd2_encode(date.day().into())?;

        let (ht, hu) = bcd2_encode(date.hour().into())?;
        let (mnt, mnu) = bcd2_encode(date.minute().into())?;
        let (st, su) = bcd2_encode(date.second().into())?;

        self.modify(|regs| {
            regs.dr.write(|w| {
                w.dt().bits(dt);
                w.du().bits(du);
                w.mt().bit(mt > 0);
                w.mu().bits(mu);
                w.yt().bits(yt);
                w.yu().bits(yu)
            });
            regs.tr.write(|w| {
                w.ht().bits(ht);
                w.hu().bits(hu);
                w.mnt().bits(mnt);
                w.mnu().bits(mnu);
                w.st().bits(st);
                w.su().bits(su);
                w.pm().clear_bit()
            })
        });

        Ok(())
    }

    pub fn get_datetime(&mut self) -> PrimitiveDateTime {
        // Wait for Registers synchronization flag,  to ensure consistency between the RTC_SSR, RTC_TR and RTC_DR shadow registers.
        while self.regs.isr.read().rsf().bit_is_clear() {}

        // Reading either RTC_SSR or RTC_TR locks the values in the higher-order calendar shadow registers until RTC_DR is read.
        // So it is important to always read SSR, TR and then DR or TR and then DR.
        let tr = self.regs.tr.read();
        let dr = self.regs.dr.read();
        // In case the software makes read accesses to the calendar in a time interval smaller
        // than 2 RTCCLK periods: RSF must be cleared by software after the first calendar read.
        self.regs.isr.modify(|_, w| w.rsf().clear_bit());

        let seconds = decode_seconds(&tr);
        let minutes = decode_minutes(&tr);
        let hours = decode_hours(&tr);
        let day = decode_day(&dr);
        let month = decode_month(&dr);
        let year = decode_year(&dr);

        PrimitiveDateTime::new(
            Date::from_calendar_date(year.into(), month.try_into().unwrap(), day).unwrap(),
            Time::from_hms(hours, minutes, seconds).unwrap(),
        )
    }
}

// Two 32-bit registers (RTC_TR and RTC_DR) contain the seconds, minutes, hours (12- or 24-hour format), day (day
// of week), date (day of month), month, and year, expressed in binary coded decimal format
// (BCD). The sub-seconds value is also available in binary format.
//
// The following helper functions encode into BCD format from integer and
// decode to an integer from a BCD value respectively.
fn bcd2_encode(word: u32) -> Result<(u8, u8), Error> {
    let l = match (word / 10).try_into() {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::InvalidInputData);
        }
    };
    let r = match (word % 10).try_into() {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::InvalidInputData);
        }
    };

    Ok((l, r))
}

fn bcd2_decode(fst: u8, snd: u8) -> u32 {
    (fst * 10 + snd).into()
}

/// Enable the low frequency external oscillator. This is the only mode currently
/// supported, to avoid exposing the `CR` and `CRS` registers.
fn enable_lse(rcc: &RegisterBlock, mode: LSEClockMode) {
    unsafe {
        // Force a reset of the backup domain.
        // Set BDCR - Bit 16 (BDRST)
        bb::set(&rcc.bdcr, 16);
        // Clear BDCR - Bit 16 (BDRST)
        bb::clear(&rcc.bdcr, 16);
        // Enable the LSE.
        // Set BDCR - Bit 0 (LSEON)
        bb::set(&rcc.bdcr, 0);
        match mode {
            // Set BDCR - Bit 2 (LSEBYP)
            LSEClockMode::Bypass => bb::set(&rcc.bdcr, 2),
            // Clear BDCR - Bit 2 (LSEBYP)
            LSEClockMode::Oscillator => bb::clear(&rcc.bdcr, 2),
        }
        while rcc.bdcr.read().lserdy().bit_is_clear() {}
        // Set clock source to LSE.
        // Set BDCR - Bit 8 (RTCSEL to value for LSE)
        bb::set(&rcc.bdcr, 8);
    }
}

fn unlock(rcc: &RegisterBlock, pwr: &mut PWR) {
    // Enable the backup interface
    // Set APB1 - Bit 28 (PWREN)
    PWR::enable(rcc);

    pwr.cr.modify(|_, w| {
        w
            // Enable access to the backup registers
            .dbp()
            .set_bit()
    });
}

fn enable(rcc: &RegisterBlock) {
    // Start the actual RTC.
    // Set BDCR - Bit 15 (RTCEN)
    unsafe {
        bb::set(&rcc.bdcr, 15);
    }
}

#[inline(always)]
fn decode_seconds(tr: &tr::R) -> u8 {
    bcd2_decode(tr.st().bits(), tr.su().bits()) as u8
}

#[inline(always)]
fn decode_minutes(tr: &tr::R) -> u8 {
    bcd2_decode(tr.mnt().bits(), tr.mnu().bits()) as u8
}

#[inline(always)]
fn decode_hours(tr: &tr::R) -> u8 {
    bcd2_decode(tr.ht().bits(), tr.hu().bits()) as u8
}

#[inline(always)]
fn decode_day(dr: &dr::R) -> u8 {
    bcd2_decode(dr.dt().bits(), dr.du().bits()) as u8
}

#[inline(always)]
fn decode_month(dr: &dr::R) -> u8 {
    let mt: u8 = if dr.mt().bit() { 1 } else { 0 };
    bcd2_decode(mt, dr.mu().bits()) as u8
}

#[inline(always)]
fn decode_year(dr: &dr::R) -> u16 {
    let year = bcd2_decode(dr.yt().bits(), dr.yu().bits()) + 1970; // 1970-01-01 is the epoch begin.
    year as u16
}
