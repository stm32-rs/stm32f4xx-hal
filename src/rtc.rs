//! Interface to the real time clock. See STM32F303 reference manual, section 27.
//! For more details, see
//! [ST AN4759](https:/www.st.com%2Fresource%2Fen%2Fapplication_note%2Fdm00226326-using-the-hardware-realtime-clock-rtc-and-the-tamper-management-unit-tamp-with-stm32-microcontrollers-stmicroelectronics.pdf&usg=AOvVaw3PzvL2TfYtwS32fw-Uv37h)

use crate::bb;
use crate::pac::rtc::{dr, tr, DR, TR};
use crate::pac::{self, rcc::RegisterBlock, PWR, RCC, RTC};
use crate::rcc::Enable;
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::marker::PhantomData;
use time::{Date, PrimitiveDateTime, Time, Weekday};

/// Invalid input error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    InvalidInputData,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Event {
    AlarmA,
    AlarmB,
    Wakeup,
    Timestamp,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Alarm {
    AlarmA = 0,
    AlarmB = 1,
}

impl From<Alarm> for Event {
    fn from(a: Alarm) -> Self {
        match a {
            Alarm::AlarmA => Event::AlarmA,
            Alarm::AlarmB => Event::AlarmB,
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AlarmDay {
    Date(Date),
    Weekday(Weekday),
    EveryDay,
}

/// RTC clock source LSE oscillator clock (type state)
pub struct Lse;
/// RTC clock source LSI oscillator clock (type state)
pub struct Lsi;

/// Real Time Clock peripheral
pub struct Rtc<CS = Lse> {
    /// RTC Peripheral register
    pub regs: RTC,
    _clock_source: PhantomData<CS>,
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

impl Rtc<Lse> {
    /// Create and enable a new RTC with external crystal or ceramic resonator and default prescalers.
    pub fn new(regs: RTC, pwr: &mut PWR) -> Self {
        Self::with_config(regs, pwr, LSEClockMode::Oscillator, 255, 127)
    }

    /// Create and enable a new RTC, and configure its clock source and prescalers.
    ///
    /// From AN3371, Table 3, when using the LSE,
    /// set `prediv_s` to 255, and `prediv_a` to 127 to get a calendar clock of 1Hz.
    pub fn with_config(
        regs: RTC,
        pwr: &mut PWR,
        mode: LSEClockMode,
        prediv_s: u16,
        prediv_a: u8,
    ) -> Self {
        let mut result = Self {
            regs,
            _clock_source: PhantomData,
        };

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
            result.unlock(rcc, pwr);
            // If necessary, enable the LSE.
            if rcc.bdcr.read().lserdy().bit_is_clear() {
                result.enable_lse(rcc, mode);
            }
            // Set clock source to LSE.
            rcc.bdcr.modify(|_, w| w.rtcsel().lse());
            result.enable(rcc);
        }

        result.modify(true, |regs| {
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

    /// Enable the low frequency external oscillator. This is the only mode currently
    /// supported, to avoid exposing the `CR` and `CRS` registers.
    fn enable_lse(&mut self, rcc: &RegisterBlock, mode: LSEClockMode) {
        unsafe {
            // Force a reset of the backup domain.
            self.backup_reset(rcc);
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
        }
    }
}

impl Rtc<Lsi> {
    /// Create and enable a new RTC with internal crystal and default prescalers.
    pub fn new_lsi(regs: RTC, pwr: &mut PWR) -> Self {
        Self::lsi_with_config(regs, pwr, 249, 127)
    }

    /// Create and enable a new RTC, and configure its clock source and prescalers.
    ///
    /// From AN3371, Table 3, when using the LSI,
    /// set `prediv_s` to 249, and `prediv_a` to 127 to get a calendar clock of 1Hz.
    pub fn lsi_with_config(regs: RTC, pwr: &mut PWR, prediv_s: u16, prediv_a: u8) -> Self {
        let mut result = Self {
            regs,
            _clock_source: PhantomData,
        };

        // Steps:
        // Enable PWR and DBP
        // Enable LSI (if needed)
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
            result.unlock(rcc, pwr);
            // If necessary, enable the LSE.
            if rcc.csr.read().lsirdy().bit_is_clear() {
                result.enable_lsi(rcc);
            }
            // Set clock source to LSI.
            rcc.bdcr.modify(|_, w| w.rtcsel().lsi());
            result.enable(rcc);
        }

        result.modify(true, |regs| {
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

    fn enable_lsi(&mut self, rcc: &RegisterBlock) {
        // Force a reset of the backup domain.
        self.backup_reset(rcc);
        // Enable the LSI.
        rcc.csr.modify(|_, w| w.lsion().on());
        while rcc.csr.read().lsirdy().is_not_ready() {}
    }
}

impl<CS> Rtc<CS> {
    fn unlock(&mut self, rcc: &RegisterBlock, pwr: &mut PWR) {
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

    fn backup_reset(&mut self, rcc: &RegisterBlock) {
        unsafe {
            // Set BDCR - Bit 16 (BDRST)
            bb::set(&rcc.bdcr, 16);
            // Clear BDCR - Bit 16 (BDRST)
            bb::clear(&rcc.bdcr, 16);
        }
    }

    fn enable(&mut self, rcc: &RegisterBlock) {
        // Start the actual RTC.
        // Set BDCR - Bit 15 (RTCEN)
        unsafe {
            bb::set(&rcc.bdcr, 15);
        }
    }

    pub fn set_prescalers(&mut self, prediv_s: u16, prediv_a: u8) {
        self.modify(true, |regs| {
            // Set prescalers
            regs.prer.modify(|_, w| {
                w.prediv_s().bits(prediv_s);
                w.prediv_a().bits(prediv_a)
            })
        });
    }

    /// As described in Section 27.3.7 in RM0316,
    /// this function is used to disable write protection
    /// when modifying an RTC register
    fn modify<F>(&mut self, init_mode: bool, mut closure: F)
    where
        F: FnMut(&mut RTC),
    {
        // Disable write protection
        // This is safe, as we're only writin the correct and expected values.
        self.regs.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.regs.wpr.write(|w| unsafe { w.bits(0x53) });
        // Enter init mode
        if init_mode && self.regs.isr.read().initf().bit_is_clear() {
            self.regs.isr.modify(|_, w| w.init().set_bit());
            // wait till init state entered
            // ~2 RTCCLK cycles
            while self.regs.isr.read().initf().bit_is_clear() {}
        }
        // Invoke closure
        closure(&mut self.regs);
        // Exit init mode
        if init_mode {
            self.regs.isr.modify(|_, w| w.init().clear_bit());
        }
        // wait for last write to be done
        while !self.regs.isr.read().initf().bit_is_clear() {}

        // Re-enable write protection.
        // This is safe, as the field accepts the full range of 8-bit values.
        self.regs.wpr.write(|w| unsafe { w.bits(0xFF) });
    }

    /// Set the time using time::Time.
    pub fn set_time(&mut self, time: &Time) -> Result<(), Error> {
        let (ht, hu) = bcd2_encode(time.hour().into())?;
        let (mnt, mnu) = bcd2_encode(time.minute().into())?;
        let (st, su) = bcd2_encode(time.second().into())?;
        self.modify(true, |regs| {
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
        self.modify(true, |regs| {
            regs.tr.modify(|_, w| w.st().bits(st).su().bits(su))
        });

        Ok(())
    }

    /// Set the minutes [0-59].
    pub fn set_minutes(&mut self, minutes: u8) -> Result<(), Error> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        let (mnt, mnu) = bcd2_encode(minutes.into())?;
        self.modify(true, |regs| {
            regs.tr.modify(|_, w| w.mnt().bits(mnt).mnu().bits(mnu))
        });

        Ok(())
    }

    /// Set the hours [0-23].
    pub fn set_hours(&mut self, hours: u8) -> Result<(), Error> {
        if hours > 23 {
            return Err(Error::InvalidInputData);
        }
        let (ht, hu) = bcd2_encode(hours.into())?;

        self.modify(true, |regs| {
            regs.tr.modify(|_, w| w.ht().bits(ht).hu().bits(hu))
        });

        Ok(())
    }

    /// Set the day of week [1-7].
    pub fn set_weekday(&mut self, weekday: u8) -> Result<(), Error> {
        if !(1..=7).contains(&weekday) {
            return Err(Error::InvalidInputData);
        }
        self.modify(true, |regs| {
            regs.dr.modify(|_, w| unsafe { w.wdu().bits(weekday) })
        });

        Ok(())
    }

    /// Set the day of month [1-31].
    pub fn set_day(&mut self, day: u8) -> Result<(), Error> {
        if !(1..=31).contains(&day) {
            return Err(Error::InvalidInputData);
        }
        let (dt, du) = bcd2_encode(day as u32)?;
        self.modify(true, |regs| {
            regs.dr.modify(|_, w| w.dt().bits(dt).du().bits(du))
        });

        Ok(())
    }

    /// Set the month [1-12].
    pub fn set_month(&mut self, month: u8) -> Result<(), Error> {
        if !(1..=12).contains(&month) {
            return Err(Error::InvalidInputData);
        }
        let (mt, mu) = bcd2_encode(month as u32)?;
        self.modify(true, |regs| {
            regs.dr.modify(|_, w| w.mt().bit(mt > 0).mu().bits(mu))
        });

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
        self.modify(true, |regs| {
            regs.dr.modify(|_, w| w.yt().bits(yt).yu().bits(yu))
        });

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
        let wdu = date.weekday().number_from_monday();

        self.modify(true, |regs| {
            regs.dr.write(|w| {
                w.dt().bits(dt);
                w.du().bits(du);
                w.mt().bit(mt > 0);
                w.mu().bits(mu);
                w.yt().bits(yt);
                w.yu().bits(yu);
                unsafe { w.wdu().bits(wdu) }
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
        let wdu = date.weekday().number_from_monday();

        let (ht, hu) = bcd2_encode(date.hour().into())?;
        let (mnt, mnu) = bcd2_encode(date.minute().into())?;
        let (st, su) = bcd2_encode(date.second().into())?;

        self.modify(true, |regs| {
            regs.dr.write(|w| {
                w.dt().bits(dt);
                w.du().bits(du);
                w.mt().bit(mt > 0);
                w.mu().bits(mu);
                w.yt().bits(yt);
                w.yu().bits(yu);
                unsafe { w.wdu().bits(wdu) }
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
        let ss = self.regs.ssr.read().ss().bits();
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
        let prediv_s = self.regs.prer.read().prediv_s().bits();
        let nano = ss_to_nano(ss, prediv_s);

        PrimitiveDateTime::new(
            Date::from_calendar_date(year.into(), month.try_into().unwrap(), day).unwrap(),
            Time::from_hms_nano(hours, minutes, seconds, nano).unwrap(),
        )
    }

    /// Configures the wakeup timer to trigger periodically every `interval` seconds
    ///
    /// # Panics
    ///
    /// Panics if interval is greater than 2¹⁷-1.
    pub fn enable_wakeup(&mut self, interval: fugit::SecsDurationU32) {
        let interval = interval.ticks();
        self.modify(false, |regs| {
            regs.cr.modify(|_, w| w.wute().clear_bit());
            regs.isr.modify(|_, w| w.wutf().clear_bit());
            while regs.isr.read().wutwf().bit_is_clear() {}

            if interval > 1 << 16 {
                regs.cr.modify(|_, w| unsafe { w.wucksel().bits(0b110) });
                let interval = u16::try_from(interval - (1 << 16) - 1)
                    .expect("Interval was too large for wakeup timer");
                regs.wutr.write(|w| w.wut().bits(interval));
            } else {
                regs.cr.modify(|_, w| unsafe { w.wucksel().bits(0b100) });
                let interval =
                    u16::try_from(interval - 1).expect("Interval was too large for wakeup timer");
                regs.wutr.write(|w| w.wut().bits(interval));
            }

            regs.cr.modify(|_, w| w.wute().set_bit());
        });
    }

    /// Disables the wakeup timer
    pub fn disable_wakeup(&mut self) {
        self.modify(false, |regs| {
            regs.cr.modify(|_, w| w.wute().clear_bit());
            regs.isr.modify(|_, w| w.wutf().clear_bit());
        });
    }

    /// Configures the timestamp to be captured when the RTC switches to Vbat power
    pub fn enable_vbat_timestamp(&mut self) {
        self.modify(false, |regs| {
            regs.cr.modify(|_, w| w.tse().clear_bit());
            regs.isr.modify(|_, w| w.tsf().clear_bit());
            regs.cr.modify(|_, w| w.tse().set_bit());
        });
    }

    /// Disables the timestamp
    pub fn disable_timestamp(&mut self) {
        self.modify(false, |regs| {
            regs.cr.modify(|_, w| w.tse().clear_bit());
            regs.isr.modify(|_, w| w.tsf().clear_bit());
        });
    }

    /// Reads the stored value of the timestamp if present
    ///
    /// Clears the timestamp interrupt flags.
    pub fn read_timestamp(&self) -> PrimitiveDateTime {
        while self.regs.isr.read().rsf().bit_is_clear() {}

        // Timestamp doesn't include year, get it from the main calendar
        let ss = self.regs.tsssr.read().ss().bits();

        // TODO: remove unsafe after PAC update
        let tr = &self.regs.tstr;
        let tr = unsafe { (*(tr as *const _ as *const TR)).read() };
        let dr = &self.regs.tsdr;
        let dr = unsafe { (*(dr as *const _ as *const DR)).read() };
        let dry = self.regs.dr.read();
        let seconds = decode_seconds(&tr);
        let minutes = decode_minutes(&tr);
        let hours = decode_hours(&tr);
        let day = decode_day(&dr);
        let month = decode_month(&dr);
        let year = decode_year(&dry);
        let prediv_s = self.regs.prer.read().prediv_s().bits();
        let nano = ss_to_nano(ss, prediv_s);

        PrimitiveDateTime::new(
            Date::from_calendar_date(year.into(), month.try_into().unwrap(), day).unwrap(),
            Time::from_hms_nano(hours, minutes, seconds, nano).unwrap(),
        )
    }

    /// Sets the time at which an alarm will be triggered
    /// This also clears the alarm flag if it is set
    pub fn set_alarm(&mut self, alarm: Alarm, date: AlarmDay, time: Time) -> Result<(), Error> {
        let (daymask, wdsel, (dt, du)) = match date {
            AlarmDay::Date(date) => (false, false, bcd2_encode(date.day().into())?),
            AlarmDay::Weekday(weekday) => (false, true, (0, weekday.number_days_from_monday())),
            AlarmDay::EveryDay => (true, false, (0, 0)),
        };
        let (ht, hu) = bcd2_encode(time.hour().into())?;
        let (mnt, mnu) = bcd2_encode(time.minute().into())?;
        let (st, su) = bcd2_encode(time.second().into())?;

        self.modify(false, |rtc| {
            unsafe {
                bb::clear(&rtc.cr, 8 + (alarm as u8));
                bb::clear(&rtc.isr, 8 + (alarm as u8));
            }
            while rtc.isr.read().bits() & (1 << (alarm as u32)) == 0 {}
            let reg = &rtc.alrmr[alarm as usize];
            reg.modify(|_, w| {
                w.dt().bits(dt);
                w.du().bits(du);
                w.ht().bits(ht);
                w.hu().bits(hu);
                w.mnt().bits(mnt);
                w.mnu().bits(mnu);
                w.st().bits(st);
                w.su().bits(su);
                w.pm().clear_bit();
                w.wdsel().bit(wdsel);
                w.msk4().bit(daymask)
            });
            // subsecond alarm not implemented
            // would need a conversion method between `time.micros` and RTC ticks
            // write the SS value and mask to `rtc.alrmssr[alarm]`

            // enable alarm and reenable interrupt if it was enabled
            unsafe {
                bb::set(&rtc.cr, 8 + (alarm as u8));
            }
        });
        Ok(())
    }

    /// Start listening for `event`
    pub fn listen(&mut self, exti: &mut pac::EXTI, event: Event) {
        // Input Mapping:
        // EXTI 17 = RTC Alarms
        // EXTI 21 = RTC Tamper, RTC Timestamp
        // EXTI 22 = RTC Wakeup Timer
        self.modify(false, |regs| match event {
            Event::AlarmA => {
                exti.rtsr.modify(|_, w| w.tr17().enabled());
                exti.imr.modify(|_, w| w.mr17().set_bit());
                regs.cr.modify(|_, w| w.alraie().set_bit());
            }
            Event::AlarmB => {
                exti.rtsr.modify(|_, w| w.tr17().enabled());
                exti.imr.modify(|_, w| w.mr17().set_bit());
                regs.cr.modify(|_, w| w.alrbie().set_bit());
            }
            Event::Wakeup => {
                exti.rtsr.modify(|_, w| w.tr22().enabled());
                exti.imr.modify(|_, w| w.mr22().set_bit());
                regs.cr.modify(|_, w| w.wutie().set_bit());
            }
            Event::Timestamp => {
                exti.rtsr.modify(|_, w| w.tr21().enabled());
                exti.imr.modify(|_, w| w.mr21().set_bit());
                regs.cr.modify(|_, w| w.tsie().set_bit());
            }
        });
    }

    /// Stop listening for `event`
    pub fn unlisten(&mut self, exti: &mut pac::EXTI, event: Event) {
        // See the note in listen() about EXTI
        self.modify(false, |regs| match event {
            Event::AlarmA => {
                regs.cr.modify(|_, w| w.alraie().clear_bit());
                exti.imr.modify(|_, w| w.mr17().clear_bit());
                exti.rtsr.modify(|_, w| w.tr17().disabled());
            }
            Event::AlarmB => {
                regs.cr.modify(|_, w| w.alrbie().clear_bit());
                exti.imr.modify(|_, w| w.mr17().clear_bit());
                exti.rtsr.modify(|_, w| w.tr17().disabled());
            }
            Event::Wakeup => {
                regs.cr.modify(|_, w| w.wutie().clear_bit());
                exti.imr.modify(|_, w| w.mr22().clear_bit());
                exti.rtsr.modify(|_, w| w.tr22().disabled());
            }
            Event::Timestamp => {
                regs.cr.modify(|_, w| w.tsie().clear_bit());
                exti.imr.modify(|_, w| w.mr21().clear_bit());
                exti.rtsr.modify(|_, w| w.tr21().disabled());
            }
        });
    }

    /// Returns `true` if `event` is pending
    pub fn is_pending(&self, event: Event) -> bool {
        match event {
            Event::AlarmA => self.regs.isr.read().alraf().bit_is_set(),
            Event::AlarmB => self.regs.isr.read().alrbf().bit_is_set(),
            Event::Wakeup => self.regs.isr.read().wutf().bit_is_set(),
            Event::Timestamp => self.regs.isr.read().tsf().bit_is_set(),
        }
    }

    /// Clears the interrupt flag for `event`
    pub fn clear_interrupt(&mut self, event: Event) {
        match event {
            Event::AlarmA => {
                self.regs.isr.modify(|_, w| w.alraf().clear_bit());
                unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr17().set_bit()) };
            }
            Event::AlarmB => {
                self.regs.isr.modify(|_, w| w.alrbf().clear_bit());
                unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr17().set_bit()) };
            }
            Event::Wakeup => {
                self.regs.isr.modify(|_, w| w.wutf().clear_bit());
                unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr22().set_bit()) };
            }
            Event::Timestamp => {
                self.regs.isr.modify(|_, w| w.tsf().clear_bit());
                unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr21().set_bit()) };
            }
        }
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

const fn bcd2_decode(fst: u8, snd: u8) -> u8 {
    fst * 10 + snd
}

#[inline(always)]
fn decode_seconds(tr: &tr::R) -> u8 {
    bcd2_decode(tr.st().bits(), tr.su().bits())
}

#[inline(always)]
fn decode_minutes(tr: &tr::R) -> u8 {
    bcd2_decode(tr.mnt().bits(), tr.mnu().bits())
}

#[inline(always)]
fn decode_hours(tr: &tr::R) -> u8 {
    bcd2_decode(tr.ht().bits(), tr.hu().bits())
}

#[inline(always)]
fn decode_day(dr: &dr::R) -> u8 {
    bcd2_decode(dr.dt().bits(), dr.du().bits())
}

#[inline(always)]
fn decode_month(dr: &dr::R) -> u8 {
    let mt = u8::from(dr.mt().bit());
    bcd2_decode(mt, dr.mu().bits())
}

#[inline(always)]
fn decode_year(dr: &dr::R) -> u16 {
    let year = (bcd2_decode(dr.yt().bits(), dr.yu().bits()) as u32) + 1970; // 1970-01-01 is the epoch begin.
    year as u16
}

const fn ss_to_nano(ss: u16, prediv_s: u16) -> u32 {
    let ss = ss as u32;
    let prediv_s = prediv_s as u32;
    assert!(ss <= prediv_s);

    (((prediv_s - ss) * 100_000) / (prediv_s + 1)) * 10_000
}
