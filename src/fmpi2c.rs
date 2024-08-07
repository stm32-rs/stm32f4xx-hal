use core::ops::Deref;

use crate::gpio;
use crate::i2c::{Error, NoAcknowledgeSource};
use crate::pac::fmpi2c1 as i2c1;
use crate::pac::{self, rcc, RCC};
use crate::rcc::{BusClock, Clocks, Enable, Reset};
use fugit::{HertzU32 as Hertz, RateExtU32};
use micromath::F32Ext;

// Old names
pub use I2c as FmpI2c;
pub use Mode as FmpMode;

mod hal_02;
mod hal_1;

type I2cSel = rcc::dckcfgr2::FMPI2C1SEL;

pub trait Instance:
    crate::Sealed
    + crate::Ptr<RB = i2c1::RegisterBlock>
    + Deref<Target = Self::RB>
    + Enable
    + Reset
    + BusClock
    + gpio::alt::I2cCommon
{
    fn set_clock_source(rcc: &rcc::RegisterBlock, source: I2cSel);
}

macro_rules! i2c {
    ($I2C:ty, $i2csel:ident, $I2Calias:ident) => {
        pub type $I2Calias = I2c<$I2C>;

        impl Instance for $I2C {
            fn set_clock_source(rcc: &rcc::RegisterBlock, source: I2cSel) {
                rcc.dckcfgr2().modify(|_, w| w.$i2csel().variant(source));
            }
        }

        impl crate::Ptr for $I2C {
            type RB = i2c1::RegisterBlock;
            #[inline(always)]
            fn ptr() -> *const Self::RB {
                Self::ptr()
            }
        }
    };
}

#[cfg(feature = "fmpi2c1")]
i2c!(pac::FMPI2C1, fmpi2c1sel, FMPI2c1);

/// I2C FastMode+ abstraction
pub struct I2c<I2C: Instance> {
    i2c: I2C,
    pins: (I2C::Scl, I2C::Sda),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Standard { frequency: Hertz },
    Fast { frequency: Hertz },
    FastPlus { frequency: Hertz },
    Custom { timing_r: I2cTiming },
}

impl Mode {
    pub fn standard(frequency: Hertz) -> Self {
        Self::Standard { frequency }
    }

    pub fn fast(frequency: Hertz) -> Self {
        Self::Fast { frequency }
    }

    pub fn fast_plus(frequency: Hertz) -> Self {
        Self::FastPlus { frequency }
    }

    /*pub fn get_frequency(&self) -> Hertz {
        match *self {
            Self::Standard { frequency } => frequency,
            Self::Fast { frequency } => frequency,
            Self::FastPlus { frequency } => frequency,
        }
    }*/
}

impl From<Hertz> for Mode {
    fn from(frequency: Hertz) -> Self {
        let k100: Hertz = 100.kHz();
        let k400: Hertz = 400.kHz();
        if frequency <= k100 {
            Self::Standard { frequency }
        } else if frequency <= k400 {
            Self::Fast { frequency }
        } else {
            Self::FastPlus { frequency }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockSource<'a> {
    Apb(&'a Clocks),
    Hsi,
}

impl<'a> From<&'a Clocks> for ClockSource<'a> {
    fn from(value: &'a Clocks) -> Self {
        Self::Apb(value)
    }
}

// hddat and vddat are removed because SDADEL is always going to be 0 in this implementation so
// condition is always met
struct I2cSpec {
    freq_max: Hertz,
    sudat_min: u32,
    _lscl_min: u32,
    _hscl_min: u32,
    trise_max: u32, // in ns
    _tfall_max: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I2cTiming {
    pub presc: u8,
    pub scldel: u8,
    pub sdadel: u8,
    pub sclh: u8,
    pub scll: u8,
}

// everything is in nano seconds
const I2C_STANDARD_MODE_SPEC: I2cSpec = I2cSpec {
    freq_max: Hertz::Hz(102400),
    sudat_min: 250,
    _lscl_min: 4700,
    _hscl_min: 4000,
    trise_max: 640,
    _tfall_max: 20,
};
const I2C_FAST_MODE_SPEC: I2cSpec = I2cSpec {
    freq_max: Hertz::Hz(409600),
    sudat_min: 100,
    _lscl_min: 1300,
    _hscl_min: 600,
    trise_max: 250,
    _tfall_max: 100,
};

const I2C_FAST_PLUS_MODE_SPEC: I2cSpec = I2cSpec {
    freq_max: Hertz::kHz(1024),
    sudat_min: 50,
    _lscl_min: 500,
    _hscl_min: 260,
    trise_max: 60,
    _tfall_max: 100,
};

fn calculate_timing(
    spec: I2cSpec,
    i2c_freq: Hertz,
    scl_freq: Hertz,
    an_filter: bool,
    dnf: u8,
) -> I2cTiming {
    let i2c_freq = i2c_freq.raw();
    // frequency limit check
    assert!(scl_freq <= spec.freq_max);
    let scl_freq = scl_freq.raw();
    // T_sync or delay introduced in SCL
    // generally it is 2-3 clock cycles
    // t_sync + dnf delay
    let t_dnf = (dnf) as f32 / i2c_freq as f32;
    // if analog filter is enabled then it offer about 50 - 70 ns delay
    let t_af: f32 = if an_filter {
        40.0 / 1_000_000_000f32
    } else {
        0.0
    };
    // t_sync = 2 to 3 * i2cclk
    let t_sync = 2.0 / (i2c_freq as f32);
    // fall or rise time
    let t_fall: f32 = 50f32 / 1_000_000_000f32;
    let t_rise: f32 = 60f32 / 1_000_000_000f32;
    let t_delay = t_fall + t_rise + 2.0 * (t_dnf + t_af + t_sync);
    // formula is like F_i2cclk/F/F_scl_clk = (scl_h+scl_l+2)*(Presc + 1)
    // consider scl_l+scl_h is 256 max. but in that case clock should always
    // be 50% duty cycle. lets consider scl_l+scl_h to be 128. so that it can
    // be changed later
    // (scl_l+scl_h+2)(presc +1 ) ==> as scl_width*presc ==F_i2cclk/F/F_scl_clk
    let product: f32 = (1.0 - t_delay * (scl_freq as f32)) * (i2c_freq / scl_freq) as f32;
    let scl_l: u8;
    let scl_h: u8;
    let mut presc: u8;
    // if ratio is > (scll+sclh)*presc. that frequancy is not possible to generate. so
    // minimum frequancy possible is generated
    if product > 8192 as f32 {
        // TODO: should we panic or use minimum allowed frequancy
        scl_l = 0x7fu8;
        scl_h = 0x7fu8;
        presc = 0xfu8;
    } else {
        // smaller the minimum devition less difference between expected vs
        // actual scl clock
        let mut min_deviation = 16f32;
        // TODO: use duty cycle and based on that use precstart
        let presc_start = (product / 512.0).ceil() as u8;
        presc = presc_start;
        for tmp_presc in presc_start..17 {
            let deviation = product % tmp_presc as f32;
            if min_deviation > deviation {
                min_deviation = deviation;
                presc = tmp_presc as u8;
            }
        }
        // now that we have optimal prescalar value. optimal scl_l and scl_h
        // needs to be calculated
        let scl_width = (product / presc as f32) as u16; // it will be always less than 256
        scl_h = (scl_width / 2 - 1) as u8;
        scl_l = (scl_width - scl_h as u16 - 1) as u8; // This is to get max precision
        presc -= 1;
    }
    let scldel: u8 = (((spec.trise_max + spec.sudat_min) as f32 / 1_000_000_000.0)
        / ((presc + 1) as f32 / i2c_freq as f32)
        - 1.0)
        .ceil() as u8;
    I2cTiming {
        presc,
        scldel,
        sdadel: 0,
        sclh: scl_h,
        scll: scl_l,
    }
}

pub trait I2cExt: Sized + Instance {
    fn i2c<'a>(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
        clocks: impl Into<ClockSource<'a>>,
    ) -> I2c<Self>;
}

impl<I2C: Instance> I2cExt for I2C {
    fn i2c<'a>(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
        clocks: impl Into<ClockSource<'a>>,
    ) -> I2c<Self> {
        I2c::new(self, pins, mode, clocks)
    }
}

impl<I2C: Instance> I2c<I2C> {
    pub fn new<'a>(
        i2c: I2C,
        pins: (impl Into<I2C::Scl>, impl Into<I2C::Sda>),
        mode: impl Into<Mode>,
        clocks: impl Into<ClockSource<'a>>,
    ) -> Self {
        unsafe {
            // Enable and reset clock.
            I2C::enable_unchecked();
            I2C::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(mode, clocks.into());
        i2c
    }

    pub fn release(self) -> (I2C, (I2C::Scl, I2C::Sda)) {
        (self.i2c, self.pins)
    }
}

impl<I2C: Instance> I2c<I2C> {
    fn i2c_init(&self, mode: impl Into<Mode>, clocks: ClockSource<'_>) {
        let mode = mode.into();

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1().modify(|_, w| w.pe().clear_bit());

        let cr1 = self.i2c.cr1().read();
        let an_filter: bool = cr1.anfoff().is_enabled();
        let dnf = cr1.dnf().bits();

        let i2c_timingr = match clocks {
            ClockSource::Apb(clocks) => {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                unsafe {
                    let rcc = &(*RCC::ptr());
                    I2C::set_clock_source(rcc, I2cSel::Apb);
                }
                let pclk = I2C::clock(clocks);
                match mode {
                    Mode::Standard { frequency } => {
                        calculate_timing(I2C_STANDARD_MODE_SPEC, pclk, frequency, an_filter, dnf)
                    }
                    Mode::Fast { frequency } => {
                        calculate_timing(I2C_FAST_MODE_SPEC, pclk, frequency, an_filter, dnf)
                    }
                    Mode::FastPlus { frequency } => {
                        calculate_timing(I2C_FAST_PLUS_MODE_SPEC, pclk, frequency, an_filter, dnf)
                    }
                    Mode::Custom { timing_r } => timing_r,
                }
            }
            ClockSource::Hsi => {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                unsafe {
                    let rcc = &(*RCC::ptr());
                    I2C::set_clock_source(rcc, I2cSel::Hsi);
                }

                use core::cmp;
                // We're using the HSI clock to keep things simple so this is going to be always 16 MHz
                const FREQ: u32 = 16_000_000;
                // Normal I2C speeds use a different scaling than fast mode below and fast mode+ even more
                // below
                match mode {
                    Mode::Standard { frequency } => {
                        let presc = 3;
                        let scll =
                            cmp::max((((FREQ >> presc) >> 1) / frequency.raw()) - 1, 255) as u8;
                        I2cTiming {
                            presc,
                            scldel: 4,
                            sdadel: 2,
                            sclh: scll - 4,
                            scll,
                        }
                    }
                    Mode::Fast { frequency } => {
                        let presc = 1;
                        let scll =
                            cmp::max((((FREQ >> presc) >> 1) / frequency.raw()) - 1, 255) as u8;
                        I2cTiming {
                            presc,
                            scldel: 3,
                            sdadel: 2,
                            sclh: scll - 6,
                            scll,
                        }
                    }
                    Mode::FastPlus { frequency } => {
                        let presc = 0;
                        let scll =
                            cmp::max((((FREQ >> presc) >> 1) / frequency.raw()) - 4, 255) as u8;
                        I2cTiming {
                            presc,
                            scldel: 2,
                            sdadel: 0,
                            sclh: scll - 2,
                            scll,
                        }
                    }
                    Mode::Custom { timing_r } => timing_r,
                }
            }
        };

        // Enable I2C signal generator, and configure I2C for configured speed
        self.i2c.timingr().write(|w| {
            w.presc().set(i2c_timingr.presc);
            w.scldel().set(i2c_timingr.scldel);
            w.sdadel().set(i2c_timingr.sdadel);
            w.sclh().set(i2c_timingr.sclh);
            w.scll().set(i2c_timingr.scll)
        });

        // Enable the I2C processing
        self.i2c.cr1().modify(|_, w| w.pe().set_bit());
    }

    #[inline(always)]
    fn check_and_clear_error_flags(&self, isr: &i2c1::isr::R) -> Result<(), Error> {
        // If we received a NACK, then this is an error
        if isr.nackf().bit_is_set() {
            self.i2c
                .icr()
                .write(|w| w.stopcf().set_bit().nackcf().set_bit());
            return Err(Error::NoAcknowledge(NoAcknowledgeSource::Unknown));
        }

        Ok(())
    }

    #[inline(always)]
    fn end_transaction(&self) -> Result<(), Error> {
        // Check and clear flags if they somehow ended up set
        self.check_and_clear_error_flags(&self.i2c.isr().read())
            .map_err(Error::nack_data)?;
        Ok(())
    }

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_addr)?;
            isr.txis().bit_is_clear()
        } {}

        // Push out a byte of data
        self.i2c
            .txdr()
            .write(|w| unsafe { w.bits(u32::from(byte)) });

        self.end_transaction()
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_data)?;
            isr.rxne().bit_is_clear()
        } {}

        let value = self.i2c.rxdr().read().bits() as u8;
        Ok(value)
    }

    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // Set up current address for reading
        self.i2c.cr2().modify(|_, w| {
            w.sadd().set(u16::from(addr) << 1);
            w.nbytes().set(buffer.len() as u8);
            w.rd_wrn().set_bit()
        });

        // Send a START condition
        self.i2c.cr2().modify(|_, w| w.start().set_bit());

        // Send the autoend after setting the start to get a restart
        self.i2c.cr2().modify(|_, w| w.autoend().set_bit());

        // Now read in all bytes
        for c in buffer.iter_mut() {
            *c = self.recv_byte()?;
        }

        self.end_transaction()
    }

    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        // Set up current slave address for writing and enable autoending
        self.i2c.cr2().modify(|_, w| {
            w.sadd().set(u16::from(addr) << 1);
            w.nbytes().set(bytes.len() as u8);
            w.rd_wrn().clear_bit();
            w.autoend().set_bit()
        });

        // Send a START condition
        self.i2c.cr2().modify(|_, w| w.start().set_bit());

        // Send out all individual bytes
        for c in bytes {
            self.send_byte(*c)?;
        }

        self.end_transaction()
    }

    pub fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        // Set up current slave address for writing and disable autoending
        self.i2c.cr2().modify(|_, w| {
            w.sadd().set(u16::from(addr) << 1);
            w.nbytes().set(bytes.len() as u8);
            w.rd_wrn().clear_bit();
            w.autoend().clear_bit()
        });

        // Send a START condition
        self.i2c.cr2().modify(|_, w| w.start().set_bit());

        // Wait until the transmit buffer is empty and there hasn't been any error condition
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_addr)?;
            isr.txis().bit_is_clear() && isr.tc().bit_is_clear()
        } {}

        // Send out all individual bytes
        for c in bytes {
            self.send_byte(*c)?;
        }

        // Wait until data was sent
        while {
            let isr = self.i2c.isr().read();
            self.check_and_clear_error_flags(&isr)
                .map_err(Error::nack_data)?;
            isr.tc().bit_is_clear()
        } {}

        // Set up current address for reading
        self.i2c.cr2().modify(|_, w| {
            w.sadd().set(u16::from(addr) << 1);
            w.nbytes().set(buffer.len() as u8);
            w.rd_wrn().set_bit()
        });

        // Send another START condition
        self.i2c.cr2().modify(|_, w| w.start().set_bit());

        // Send the autoend after setting the start to get a restart
        self.i2c.cr2().modify(|_, w| w.autoend().set_bit());

        // Now read in all bytes
        for c in buffer.iter_mut() {
            *c = self.recv_byte()?;
        }

        self.end_transaction()
    }
}
