//! # API for the Analog to Digital converter

#![allow(dead_code)]

use crate::rcc::{Clocks, Enable, Reset, APB2};

use crate::gpio::{self, Analog};

use crate::pac::{ADC1, ADC2, ADC3, ADC_COMMON};

use cortex_m::asm::delay;
use fugit::HertzU32 as Hertz;

use embedded_hal::adc::{Channel, OneShot};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// ADC sampling time
///
/// Options for the sampling time, each is T ADC clock cycles.
// 15.13.4 >> ADC sample time register
pub enum SampleTime {
    /// 3 cycles sampling time
    T_3,
    /// 15 cycles sampling time
    T_15,
    /// 28 cycles sampling time
    T_28,
    /// 56 cycles sampling time
    T_56,
    /// 84 cycles sampling time
    T_84,
    /// 112 cycles sampling time
    T_112,
    /// 144 cycles sampling time
    T_144,
    /// 480 cycles sampling time
    T_480,
}

impl Default for SampleTime {
    /// Get the default sample time (currently 56 cycles)
    fn default() -> Self {
        SampleTime::T_56
    }
}

impl From<SampleTime> for u8 {
    fn from(val: SampleTime) -> Self {
        use SampleTime::*;
        match val {
            T_3 => 0,
            T_15 => 1,
            T_28 => 2,
            T_56 => 3,
            T_84 => 4,
            T_112 => 5,
            T_144 => 6,
            T_480 => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// ADC data register alignment
pub enum Align {
    /// Right alignment of output data
    Right,
    /// Left alignment of output data
    Left,
}

impl Default for Align {
    /// Default: right alignment
    fn default() -> Self {
        Align::Right
    }
}

// 15.13.3 ADC control register >> Bit 11 ALIGN: Data alignment
impl From<Align> for bool {
    fn from(val: Align) -> Self {
        match val {
            Align::Right => false,
            Align::Left => true,
        }
    }
}

/////////////////////////////////

macro_rules! adc_pins {
    ($ADC:ident, $($pin:ty => $chan:literal),+ $(,)*) => {
        $(
            impl Channel<$ADC> for $pin {
                type ID = u8;

                fn channel() -> u8 { $chan }
            }
        )+
    };
}

// See "Datasheet - production data"
// Pinouts and pin description (page 66..)
adc_pins!(ADC1,
    gpio::PA0<Analog> => 0,
    gpio::PA1<Analog> => 1,
    gpio::PA2<Analog> => 2,
    gpio::PA3<Analog> => 3,
    gpio::PA4<Analog> => 4,
    gpio::PA5<Analog> => 5,
    gpio::PA6<Analog> => 6,
    gpio::PA7<Analog> => 7,
    gpio::PB0<Analog>  => 8,
    gpio::PB1<Analog>  => 9,
    gpio::PC0<Analog>  => 10,
    gpio::PC1<Analog>  => 11,
    gpio::PC2<Analog>  => 12,
    gpio::PC3<Analog>  => 13,
    gpio::PC4<Analog>  => 14,
    gpio::PC5<Analog>  => 15,
);

adc_pins!(ADC2,
    gpio::PA0<Analog>  => 0,
    gpio::PA1<Analog>  => 1,
    gpio::PA2<Analog>  => 2,
    gpio::PA3<Analog>  => 3,
    gpio::PA4<Analog>  => 4,
    gpio::PA5<Analog>  => 5,
    gpio::PA6<Analog>  => 6,
    gpio::PA7<Analog>  => 7,
    gpio::PB0<Analog>  => 8,
    gpio::PB1<Analog>  => 9,
    gpio::PC0<Analog>  => 10,
    gpio::PC1<Analog>  => 11,
    gpio::PC2<Analog>  => 12,
    gpio::PC3<Analog>  => 13,
    gpio::PC4<Analog>  => 14,
    gpio::PC5<Analog>  => 15,
);

adc_pins!(ADC3,
    gpio::PA0<Analog> => 0,
    gpio::PA1<Analog> => 1,
    gpio::PA2<Analog> => 2,
    gpio::PA3<Analog> => 3,
    gpio::PF6<Analog> => 4,
    gpio::PF7<Analog> => 5,
    gpio::PF8<Analog> => 6,
    gpio::PF9<Analog> => 7,
    gpio::PF10<Analog> => 8,
    gpio::PF3<Analog> => 9,
    gpio::PC0<Analog> => 10,
    gpio::PC1<Analog> => 11,
    gpio::PC2<Analog> => 12,
    gpio::PC3<Analog> => 13,
    gpio::PF4<Analog> => 14,
    gpio::PF5<Analog> => 15,
);

////////////////////////////////////

/// ADC configuration
pub struct Adc<ADC> {
    rb: ADC,
    sample_time: SampleTime,
    align: Align,
    sysclk: Hertz,
}

/// Stored ADC config can be restored using the `Adc::restore_cfg` method
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct StoredConfig(SampleTime, Align);

macro_rules! adc_hal {
    ( $ADC:ident, $adc:ident) => {
        impl Adc<$ADC> {
            /// Init a new Adc
            ///
            /// Sets all configurable parameters to one-shot defaults,
            pub fn $adc(
                adc: $ADC,
                apb2: &mut APB2,
                clocks: &Clocks,
                nb_resolution_bits: u8,
                reset: bool,
            ) -> Self {
                let mut s = Self {
                    rb: adc,
                    sample_time: SampleTime::default(),
                    align: Align::default(),
                    sysclk: clocks.sysclk(),
                };
                <$ADC>::enable(apb2);
                if reset {
                    s.power_down();
                    <$ADC>::reset(apb2);
                }

                s.setup_oneshot();
                s.resolution(nb_resolution_bits);
                s.power_up();

                s
            }

            /// Save current ADC config
            pub fn save_cfg(&mut self) -> StoredConfig {
                StoredConfig(self.sample_time, self.align)
            }

            /// Restore saved ADC config
            pub fn restore_cfg(&mut self, cfg: StoredConfig) {
                self.sample_time = cfg.0;
                self.align = cfg.1;
            }

            /// Reset the ADC config to default, return existing config
            pub fn default_cfg(&mut self) -> StoredConfig {
                let cfg = self.save_cfg();
                self.sample_time = SampleTime::default();
                self.align = Align::default();
                cfg
            }

            /// Set ADC sampling time
            ///
            /// Options can be found in [SampleTime](crate::adc::SampleTime).
            pub fn set_sample_time(&mut self, t_samp: SampleTime) {
                self.sample_time = t_samp;
            }

            /// Set the Adc result alignment
            ///
            /// Options can be found in [Align](crate::adc::Align).
            pub fn set_align(&mut self, align: Align) {
                self.align = align;
            }

            /// Returns the largest possible sample value for the current settings
            pub fn max_sample(&self) -> u16 {
                match self.align {
                    Align::Left => u16::max_value(),
                    Align::Right => (1 << 12) - 1,
                }
            }

            #[inline(always)]
            pub fn set_external_trigger(&mut self, trigger: crate::pac::adc1::cr2::EXTSEL_A) {
                self.rb.cr2.modify(|_, w| w.extsel().variant(trigger))
            }

            fn power_up(&mut self) {
                self.rb.cr2.modify(|_, w| w.adon().set_bit());

                // The reference manual says that a stabilization time is needed after power_up,
                // this time can be found in the datasheets.
                // for STM32F7xx : delay(216_000_000/800_000)= delay(270 cycles) = 1.25us
                delay(self.sysclk.raw() / 800_000);
            }

            // 15.3.1 ADC on-off control
            fn power_down(&mut self) {
                self.rb.cr2.modify(|_, w| w.adon().clear_bit());
            }

            // 15.3.5 Single conversion mode (page: 444)
            // CONT bit >> 0 (continuous)
            // see EXTEN and EXTSEL[3:0]: for triggers (page:471)
            // SWSTART: Start conversion of regular channels
            fn setup_oneshot(&mut self) {
                self.rb
                    .cr2
                    .modify(|_, w| w.cont().clear_bit().swstart().set_bit());

                // SCAN: Scan mode
                // DISCEN: Discontinuous mode on regular channels
                self.rb
                    .cr1
                    .modify(|_, w| w.scan().clear_bit().discen().set_bit());

                // ADC regular sequence register
                // The total number of conversions in the regular group must be written in the L[3:0] bits in the ADC_SQR1 register. (15.3.4 page:444)
                self.rb.sqr1.modify(|_, w| w.l().bits(0b0));
            }

            /// setup the ADC Resolution : Bits 25:24 RES[1:0]
            fn resolution(&mut self, resol_bits: u8) {
                match resol_bits {
                    12 => self.rb.cr1.modify(|_, w| w.res().bits(0b00)),
                    10 => self.rb.cr1.modify(|_, w| w.res().bits(0b01)),
                    8 => self.rb.cr1.modify(|_, w| w.res().bits(0b10)),
                    6 => self.rb.cr1.modify(|_, w| w.res().bits(0b11)),
                    _ => self.rb.cr1.modify(|_, w| w.res().bits(0b00)),
                }
            }

            // See : ADC sample time registers (page: 474)
            fn set_channel_sample_time(&mut self, chan: u8, sample_time: SampleTime) {
                self.rb.smpr2.modify(|r, w| unsafe {
                    w.bits((r.bits() & !0x07) | ((sample_time as u32) & 0x07))
                });
                match chan {
                    0..=9 => {
                        // 3 first bits (we keep other bits) : SMP0[2:0]

                        // SMPchan[2:0]
                        self.rb.smpr2.modify(|r, w| unsafe {
                            w.bits(
                                (r.bits() & !(0x07 << (chan * 3)))
                                    | (((sample_time as u32) & 0x07) << (chan * 3)),
                            )
                        })
                    }

                    //////////////  SMPR1
                    10..=18 => {
                        // 3 first bits (we keep other bits) : SMP10[2:0]

                        // SMPchan[2:0]
                        self.rb.smpr1.modify(|r, w| unsafe {
                            w.bits(
                                (r.bits() & !(0x07 << ((chan - 10) * 3)))
                                    | (((sample_time as u32) & 0x07) << ((chan - 10) * 3)),
                            )
                        })
                    }

                    _ => unreachable!(),
                }
            }

            ////////////////

            fn set_regular_sequence(&mut self, channels: &[u8]) {
                let len = channels.len();
                let bits = channels
                    .iter()
                    .take(6)
                    .enumerate()
                    .fold(0u32, |s, (i, c)| s | ((*c as u32) << (i * 5)));
                self.rb.sqr3.write(|w| unsafe { w.bits(bits) });
                if len > 6 {
                    let bits = channels
                        .iter()
                        .skip(6)
                        .take(6)
                        .enumerate()
                        .fold(0u32, |s, (i, c)| s | ((*c as u32) << (i * 5)));
                    self.rb.sqr2.write(|w| unsafe { w.bits(bits) });
                }
                if len > 12 {
                    let bits = channels
                        .iter()
                        .skip(12)
                        .take(4)
                        .enumerate()
                        .fold(0u32, |s, (i, c)| s | ((*c as u32) << (i * 5)));
                    self.rb.sqr1.write(|w| unsafe { w.bits(bits) });
                }
                self.rb.sqr1.modify(|_, w| w.l().bits((len - 1) as u8));
            }

            fn set_continuous_mode(&mut self, continuous: bool) {
                self.rb.cr2.modify(|_, w| w.cont().bit(continuous));
            }

            fn set_discontinuous_mode(&mut self, channels_count: Option<u8>) {
                self.rb.cr1.modify(|_, w| match channels_count {
                    Some(count) => w.discen().set_bit().discnum().bits(count),
                    None => w.discen().clear_bit(),
                });
            }

            /*
              Performs an ADC conversion

              NOTE: Conversions can be started by writing a 1 to the ADON
              bit in the `CR2` while it is already 1, and no other bits
              are being written in the same operation. This means that
              the EOC bit *might* be set already when entering this function
              which can cause a read of stale values

              The check for `cr2.swstart.bit_is_set` *should* fix it, but
              does not. Therefore, ensure you do not do any no-op modifications
              to `cr2` just before calling this function
            */
            fn convert(&mut self, chan: u8) -> u16 {
                // Dummy read in case something accidentally triggered
                // a conversion by writing to CR2 without changing any
                // of the bits
                self.rb.dr.read().data().bits();

                self.set_channel_sample_time(chan, self.sample_time);
                self.rb.sqr3.modify(|_, w| unsafe { w.sq1().bits(chan) });

                // ADC start conversion of regular sequence
                self.rb
                    .cr2
                    .modify(|_, w| w.swstart().set_bit().align().bit(self.align.into()));
                while self.rb.cr2.read().swstart().bit_is_set() {}
                // ADC wait for conversion results
                while self.rb.sr.read().eoc().bit_is_clear() {}

                let res = self.rb.dr.read().data().bits();
                res
            }

            /// Powers down the ADC, disables the ADC clock and releases the ADC Peripheral
            pub fn release(mut self, apb2: &mut APB2) -> $ADC {
                self.power_down();
                <$ADC>::disable(apb2);
                self.rb
            }
        }

        impl ChannelTimeSequence for Adc<$ADC> {
            #[inline(always)]
            fn set_channel_sample_time(&mut self, chan: u8, sample_time: SampleTime) {
                self.set_channel_sample_time(chan, sample_time);
            }
            #[inline(always)]
            fn set_regular_sequence(&mut self, channels: &[u8]) {
                self.set_regular_sequence(channels);
            }
            #[inline(always)]
            fn set_continuous_mode(&mut self, continuous: bool) {
                self.set_continuous_mode(continuous);
            }
            #[inline(always)]
            fn set_discontinuous_mode(&mut self, channels: Option<u8>) {
                self.set_discontinuous_mode(channels);
            }
        }

        impl<WORD, PIN> OneShot<$ADC, WORD, PIN> for Adc<$ADC>
        where
            WORD: From<u16>,
            PIN: Channel<$ADC, ID = u8>,
        {
            type Error = ();

            fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
                let res = self.convert(PIN::channel());
                Ok(res.into())
            }
        }
    };
}

impl Adc<ADC1> {
    /// Internal reference voltage Vrefint is connected to channel 17 on ADC1.
    /// According to section 6.3.27 "Reference voltage" from STM32F7xx (page:168/252)
    /// datasheets, typical value of this reference voltage is 1210 mV.
    ///
    /// This value is useful when ADC readings need to be converted into voltages.
    /// For instance, reading from any ADC channel can be converted into voltage (mV)
    /// using the following formula:
    ///     v_chan = adc.read(chan) * 1210 / adc.read_vref()
    pub fn read_vref(&mut self, adc_common: &ADC_COMMON) -> u16 {
        ////////////////
        let tsv_off = if adc_common.ccr.read().tsvrefe().bit_is_clear() {
            adc_common.ccr.modify(|_, w| w.vbate().clear_bit());
            adc_common.ccr.modify(|_, w| w.tsvrefe().set_bit());

            // The reference manual says that a stabilization time is needed after the powering the
            // sensor, this time can be found in the datasheets.
            delay(self.sysclk.raw() / 80_000);
            true
        } else {
            false
        };

        //ADC1_IN17
        let val = self.convert(17u8);

        if tsv_off {
            adc_common.ccr.modify(|_, w| w.tsvrefe().clear_bit());
        }

        val
    }

    pub fn bits_to_voltage(&mut self, adc_common: &ADC_COMMON, data: u16) -> u16 {
        let v_chan = (data as u32) * 1210 / (self.read_vref(adc_common) as u32);

        v_chan as u16
    }
}

// Implement adc_hal! for ADC1, ADC2 and ADC3
adc_hal!(ADC1, adc1);

adc_hal!(ADC2, adc2);

adc_hal!(ADC3, adc3);

pub trait ChannelTimeSequence {
    /// Set ADC sampling time for particular channel
    fn set_channel_sample_time(&mut self, chan: u8, sample_time: SampleTime);
    /// ADC Set a Regular Channel Conversion Sequence
    ///
    /// Define a sequence of channels to be converted as a regular group.
    fn set_regular_sequence(&mut self, channels: &[u8]);
    /// Set ADC continuous conversion
    ///
    /// When continuous conversion is enabled conversion does not stop at the last selected group channel but continues again from the first selected group channel.
    fn set_continuous_mode(&mut self, continuous: bool);
    /// Set ADC discontinuous mode
    ///
    /// It can be used to convert a short sequence of conversions (up to 8) which is a part of the regular sequence of conversions.
    fn set_discontinuous_mode(&mut self, channels_count: Option<u8>);
}

/// Set channel sequence and sample times for custom pins
///
/// Example:
/// ```rust, ignore
/// pub struct AdcPins(PA0<Analog>, PA2<Analog>);
/// impl SetChannels<AdcPins> for Adc<ADC1> {
///     fn set_samples(&mut self) {
///         self.set_channel_sample_time(0, adc::SampleTime::T_28);
///         self.set_channel_sample_time(2, adc::SampleTime::T_28);
///     }
///     fn set_sequence(&mut self) {
///         self.set_regular_sequence(&[0, 2, 0, 2]);
///         // Optionally we can set continuous scan mode
///         self.set_continuous_mode(true);
///         // Also we can use discontinuous conversion (3 channels per conversion)
///         self.set_discontinuous_mode(Some(3));
///     }
/// }
/// ```
pub trait SetChannels<PINS>: ChannelTimeSequence {
    fn set_samples(&mut self);
    fn set_sequence(&mut self);
}
