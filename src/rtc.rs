//! Interface to the real time clock. See STM32F303 reference manual, section 27.
//! For more details, see
//! [ST AN4759](https:/www.st.com%2Fresource%2Fen%2Fapplication_note%2Fdm00226326-using-the-hardware-realtime-clock-rtc-and-the-tamper-management-unit-tamp-with-stm32-microcontrollers-stmicroelectronics.pdf&usg=AOvVaw3PzvL2TfYtwS32fw-Uv37h)

use crate::pac::{PWR, RTC};
use crate::rcc::{APB1, BDCR};
use core::convert::TryInto;
use rtcc::{Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike};

/// Invalid input error
#[derive(Debug)]
pub enum Error {
    InvalidInputData,
}

pub const LSE_BITS: u8 = 0b01;

pub struct Rtc {
    pub regs: RTC,
}

impl Rtc {
    /// Create and enable a new RTC, and configure its clock source and prescalers.
    /// From AN4759, Table 7, when using the LSE (The only clock source this module
    /// supports currently), set `prediv_s` to 255, and `prediv_a` to 127 to get a
    /// calendar clock of 1Hz.
    /// The `bypass` argument is `true` if you're using an external oscillator that
    /// doesn't connect to `OSC32_IN`, such as a MEMS resonator.
    pub fn new(
        regs: RTC,
        prediv_s: u16,
        prediv_a: u8,
        bypass: bool,
        apb1: &mut APB1,
        bdcr: &mut BDCR,
        pwr: &mut PWR,
    ) -> Self {
        let mut result = Self { regs };

        enable_lse(bdcr, bypass);
        unlock(apb1, pwr);
        enable(bdcr);
        result.set_24h_fmt();

        result.regs.prer.modify(|_, w| unsafe {
            w.prediv_s().bits(prediv_s);
            w.prediv_a().bits(prediv_a)
        });

        result
    }

    /// Sets calendar clock to 24 hr format
    pub fn set_24h_fmt(&mut self) {
        self.regs.cr.modify(|_, w| w.fmt().set_bit());
    }
    /// Sets calendar clock to 12 hr format
    pub fn set_12h_fmt(&mut self) {
        self.regs.cr.modify(|_, w| w.fmt().clear_bit());
    }

    /// Reads current hour format selection
    pub fn is_24h_fmt(&self) -> bool {
        self.regs.cr.read().fmt().bit()
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
    }
}

impl Rtcc for Rtc {
    type Error = Error;

    /// set time using NaiveTime (ISO 8601 time without timezone)
    /// Hour format is 24h
    fn set_time(&mut self, time: &NaiveTime) -> Result<(), Self::Error> {
        self.set_24h_fmt();
        let (ht, hu) = bcd2_encode(time.hour())?;
        let (mnt, mnu) = bcd2_encode(time.minute())?;
        let (st, su) = bcd2_encode(time.second())?;
        self.regs.tr.write(|w| unsafe {
            w.ht().bits(ht);
            w.hu().bits(hu);
            w.mnt().bits(mnt);
            w.mnu().bits(mnu);
            w.st().bits(st);
            w.su().bits(su);
            w.pm().clear_bit()
        });

        Ok(())
    }

    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error> {
        if seconds > 59 {
            return Err(Error::InvalidInputData);
        }
        let (st, su) = bcd2_encode(seconds as u32)?;
        self.modify(|regs| {
            regs.tr
                .modify(|_, w| unsafe { w.st().bits(st).su().bits(su) })
        });

        Ok(())
    }

    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        let (mnt, mnu) = bcd2_encode(minutes as u32)?;
        self.modify(|regs| {
            regs.tr
                .modify(|_, w| unsafe { w.mnt().bits(mnt).mnu().bits(mnu) })
        });

        Ok(())
    }

    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error> {
        let (ht, hu) = hours_to_register(hours)?;
        match hours {
            Hours::H24(_h) => self.set_24h_fmt(),
            Hours::AM(_h) | Hours::PM(_h) => self.set_12h_fmt(),
        }

        self.regs
            .tr
            .modify(|_, w| unsafe { w.ht().bits(ht).hu().bits(hu) });

        Ok(())
    }

    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error> {
        if (weekday < 1) || (weekday > 7) {
            return Err(Error::InvalidInputData);
        }
        self.modify(|regs| regs.dr.modify(|_, w| unsafe { w.wdu().bits(weekday) }));

        Ok(())
    }

    fn set_day(&mut self, day: u8) -> Result<(), Self::Error> {
        if (day < 1) || (day > 31) {
            return Err(Error::InvalidInputData);
        }
        let (dt, du) = bcd2_encode(day as u32)?;
        self.modify(|regs| {
            regs.dr
                .modify(|_, w| unsafe { w.dt().bits(dt).du().bits(du) })
        });

        Ok(())
    }

    fn set_month(&mut self, month: u8) -> Result<(), Self::Error> {
        if (month < 1) || (month > 12) {
            return Err(Error::InvalidInputData);
        }
        let (mt, mu) = bcd2_encode(month as u32)?;
        self.modify(|regs| {
            regs.dr
                .modify(|_, w| unsafe { w.mt().bit(mt > 0).mu().bits(mu) })
        });

        Ok(())
    }

    fn set_year(&mut self, year: u16) -> Result<(), Self::Error> {
        if (year < 1970) || (year > 2038) {
            return Err(Error::InvalidInputData);
        }
        let (yt, yu) = bcd2_encode(year as u32)?;
        self.modify(|regs| {
            regs.dr
                .modify(|_, w| unsafe { w.yt().bits(yt).yu().bits(yu) })
        });

        Ok(())
    }

    /// Set the date using NaiveDate (ISO 8601 calendar date without timezone).
    /// WeekDay is set using the `set_weekday` method
    fn set_date(&mut self, date: &NaiveDate) -> Result<(), Self::Error> {
        if date.year() < 1970 {
            return Err(Error::InvalidInputData);
        }

        let (yt, yu) = bcd2_encode((date.year() - 1970) as u32)?;
        let (mt, mu) = bcd2_encode(date.month())?;
        let (dt, du) = bcd2_encode(date.day())?;

        self.regs.dr.write(|w| unsafe {
            w.dt().bits(dt);
            w.du().bits(du);
            w.mt().bit(mt > 0);
            w.mu().bits(mu);
            w.yt().bits(yt);
            w.yu().bits(yu)
        });

        Ok(())
    }

    fn set_datetime(&mut self, date: &NaiveDateTime) -> Result<(), Self::Error> {
        if date.year() < 1970 {
            return Err(Error::InvalidInputData);
        }

        self.set_24h_fmt();
        let (yt, yu) = bcd2_encode((date.year() - 1970) as u32)?;
        let (mt, mu) = bcd2_encode(date.month())?;
        let (dt, du) = bcd2_encode(date.day())?;

        let (ht, hu) = bcd2_encode(date.hour())?;
        let (mnt, mnu) = bcd2_encode(date.minute())?;
        let (st, su) = bcd2_encode(date.second())?;

        self.regs.dr.write(|w| unsafe {
            w.dt().bits(dt);
            w.du().bits(du);
            w.mt().bit(mt > 0);
            w.mu().bits(mu);
            w.yt().bits(yt);
            w.yu().bits(yu)
        });

        self.regs.tr.write(|w| unsafe {
            w.ht().bits(ht);
            w.hu().bits(hu);
            w.mnt().bits(mnt);
            w.mnu().bits(mnu);
            w.st().bits(st);
            w.su().bits(su);
            w.pm().clear_bit()
        });

        Ok(())
    }

    fn get_seconds(&mut self) -> Result<u8, Self::Error> {
        let tr = self.regs.tr.read();
        let seconds = bcd2_decode(tr.st().bits(), tr.su().bits());
        Ok(seconds as u8)
    }

    fn get_minutes(&mut self) -> Result<u8, Self::Error> {
        let tr = self.regs.tr.read();
        let minutes = bcd2_decode(tr.mnt().bits(), tr.mnu().bits());
        Ok(minutes as u8)
    }

    fn get_hours(&mut self) -> Result<Hours, Self::Error> {
        let tr = self.regs.tr.read();
        let hours = bcd2_decode(tr.ht().bits(), tr.hu().bits());
        if self.is_24h_fmt() {
            return Ok(Hours::H24(hours as u8));
        }
        if !tr.pm().bit() {
            return Ok(Hours::AM(hours as u8));
        }
        Ok(Hours::PM(hours as u8))
    }

    fn get_time(&mut self) -> Result<NaiveTime, Self::Error> {
        self.set_24h_fmt();
        let seconds = self.get_seconds().unwrap();
        let minutes = self.get_minutes().unwrap();
        let hours = hours_to_u8(self.get_hours()?)?;

        Ok(NaiveTime::from_hms(
            hours.into(),
            minutes.into(),
            seconds.into(),
        ))
    }

    fn get_weekday(&mut self) -> Result<u8, Self::Error> {
        let dr = self.regs.dr.read();
        let weekday = bcd2_decode(dr.wdu().bits(), 0x00);
        Ok(weekday as u8)
    }

    fn get_day(&mut self) -> Result<u8, Self::Error> {
        let dr = self.regs.dr.read();
        let day = bcd2_decode(dr.dt().bits(), dr.du().bits());
        Ok(day as u8)
    }

    fn get_month(&mut self) -> Result<u8, Self::Error> {
        let dr = self.regs.dr.read();
        let mt: u8 = if dr.mt().bit() { 1 } else { 0 };
        let month = bcd2_decode(mt, dr.mu().bits());
        Ok(month as u8)
    }

    fn get_year(&mut self) -> Result<u16, Self::Error> {
        let dr = self.regs.dr.read();
        let year = bcd2_decode(dr.yt().bits(), dr.yu().bits());
        Ok(year as u16)
    }

    fn get_date(&mut self) -> Result<NaiveDate, Self::Error> {
        let day = self.get_day().unwrap();
        let month = self.get_month().unwrap();
        let year = self.get_year().unwrap();

        Ok(NaiveDate::from_ymd(year.into(), month.into(), day.into()))
    }

    fn get_datetime(&mut self) -> Result<NaiveDateTime, Self::Error> {
        self.set_24h_fmt();

        let day = self.get_day().unwrap();
        let month = self.get_month().unwrap();
        let year = self.get_year().unwrap();

        let seconds = self.get_seconds().unwrap();
        let minutes = self.get_minutes().unwrap();
        let hours = hours_to_u8(self.get_hours()?)?;

        Ok(
            NaiveDate::from_ymd(year.into(), month.into(), day.into()).and_hms(
                hours.into(),
                minutes.into(),
                seconds.into(),
            ),
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

fn hours_to_register(hours: Hours) -> Result<(u8, u8), Error> {
    match hours {
        Hours::H24(h) => Ok(bcd2_encode(h as u32))?,
        Hours::AM(h) => Ok(bcd2_encode((h - 1) as u32))?,
        Hours::PM(h) => Ok(bcd2_encode((h + 11) as u32))?,
    }
}

fn hours_to_u8(hours: Hours) -> Result<u8, Error> {
    if let Hours::H24(h) = hours {
        Ok(h)
    } else {
        Err(Error::InvalidInputData)
    }
}

/// Enable the low frequency external oscillator. This is the only mode currently
/// supported, to avoid exposing the `CR` and `CRS` registers.
fn enable_lse(bdcr: &mut BDCR, bypass: bool) {
    bdcr.bdcr()
        .modify(|_, w| w.lseon().set_bit().lsebyp().bit(bypass));
    while bdcr.bdcr().read().lserdy().bit_is_clear() {}
}

fn unlock(apb1: &mut APB1, pwr: &mut PWR) {
    apb1.enr().modify(|_, w| {
        w
            // Enable the backup interface by setting PWREN
            .pwren()
            .set_bit()
    });
    pwr.cr.modify(|_, w| {
        w
            // Enable access to the backup registers
            .dbp()
            .set_bit()
    });

    while pwr.cr.read().dbp().bit_is_clear() {}
}

fn enable(bdcr: &mut BDCR) {
    bdcr.bdcr().modify(|_, w| w.bdrst().enabled());
    bdcr.bdcr().modify(|_, w| {
        w.rtcsel().lse();
        w.rtcen().enabled();
        w.bdrst().disabled()
    });
}
