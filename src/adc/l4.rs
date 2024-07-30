//! # Analog to Digital converter

use core::{
    convert::Infallible,
    marker::PhantomData,
    //ops::DerefMut,
    //sync::atomic::{self, Ordering},
};
#[cfg(feature = "dma")]
use stable_deref_trait::StableDeref;

#[cfg(feature = "dma")]
use crate::{
    dma::{dma1, Event as DMAEvent, RxDma, Transfer, TransferPayload, W},
    dmamux::{DmaInput, DmaMux},
};
use crate::{
    gpio::{self, Analog},
    hal::{
        adc::{Channel as EmbeddedHalChannel, OneShot},
        blocking::delay::DelayUs,
    },
    pac::{self, ADC1},
    rcc::{Enable, Reset, AHB2, CCIPR},
    signature::{VrefCal, VtempCalHigh, VtempCalLow, VDDA_CALIB_MV},
};

/// Internal voltage reference channel, used for calibration.
pub struct Vref {
    _0: (),
}

/// Internal battery monitoring channel.
pub struct Vbat {
    _0: (),
}

/// Internal temperature sensor channel.
pub struct Temperature {
    _0: (),
}

/// Wrapper for safely sharing [`ADC_COMMON`](pac::ADC_COMMON) between `Adc`s.
#[derive(Clone, Copy)]
pub struct AdcCommon {
    _0: PhantomData<pac::ADC_COMMON>,
    #[allow(unused)]
    csr: AdcCommonCsr,
    ccr: AdcCommonCcr,
    #[allow(unused)]
    cdr: AdcCommonCdr,
}

#[derive(Clone, Copy)]
struct AdcCommonCsr {
    _0: PhantomData<stm32l4::Reg<pac::adc_common::csr::CSR_SPEC>>,
}

#[derive(Clone, Copy)]
struct AdcCommonCcr {
    _0: PhantomData<stm32l4::Reg<pac::adc_common::ccr::CCR_SPEC>>,
}

#[derive(Clone, Copy)]
struct AdcCommonCdr {
    _0: PhantomData<stm32l4::Reg<pac::adc_common::cdr::CDR_SPEC>>,
}

impl AdcCommonCcr {
    #[inline]
    fn read(&self) -> pac::adc_common::ccr::R {
        let adc_common = unsafe { &*pac::ADC_COMMON::ptr() };
        adc_common.ccr.read()
    }

    #[inline]
    fn modify<F>(&mut self, f: F)
    where
        for<'w> F: FnOnce(
            &pac::adc_common::ccr::R,
            &'w mut pac::adc_common::ccr::W,
        ) -> &'w mut stm32l4::W<pac::adc_common::ccr::CCR_SPEC>,
    {
        cortex_m::interrupt::free(|_| {
            let adc_common = unsafe { &*pac::ADC_COMMON::ptr() };
            adc_common.ccr.modify(|r, w| f(r, w))
        })
    }
}

impl AdcCommon {
    /// Enable and reset [`ADC_COMMON`](pac::ADC_COMMON) peripheral.
    pub fn new(adc_common: pac::ADC_COMMON, ahb: &mut AHB2) -> Self {
        <pac::ADC_COMMON>::enable(ahb);
        <pac::ADC_COMMON>::reset(ahb);

        drop(adc_common);

        Self {
            _0: PhantomData,
            csr: AdcCommonCsr { _0: PhantomData },
            ccr: AdcCommonCcr { _0: PhantomData },
            cdr: AdcCommonCdr { _0: PhantomData },
        }
    }
}

/// Analog to Digital converter interface
pub struct Adc<ADC> {
    adc: ADC,
    adc_common: AdcCommon,
    resolution: Resolution,
    sample_time: SampleTime,
    calibrated_vdda: u32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DmaMode {
    Disabled = 0,
    Oneshot = 1,
    // FIXME: Figure out how to get circular DMA to function properly (requires circbuffer?)
    // Circular = 2,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Sequence {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Eleven = 10,
    Twelve = 11,
    Thirteen = 12,
    Fourteen = 13,
    Fifteen = 14,
    Sixteen = 15,
}

impl From<u8> for Sequence {
    fn from(bits: u8) -> Self {
        match bits {
            0 => Sequence::One,
            1 => Sequence::Two,
            2 => Sequence::Three,
            3 => Sequence::Four,
            4 => Sequence::Five,
            5 => Sequence::Six,
            6 => Sequence::Seven,
            7 => Sequence::Eight,
            8 => Sequence::Nine,
            9 => Sequence::Ten,
            10 => Sequence::Eleven,
            11 => Sequence::Twelve,
            12 => Sequence::Thirteen,
            13 => Sequence::Fourteen,
            14 => Sequence::Fifteen,
            15 => Sequence::Sixteen,
            _ => unimplemented!(),
        }
    }
}

impl Into<u8> for Sequence {
    fn into(self) -> u8 {
        match self {
            Sequence::One => 0,
            Sequence::Two => 1,
            Sequence::Three => 2,
            Sequence::Four => 3,
            Sequence::Five => 4,
            Sequence::Six => 5,
            Sequence::Seven => 6,
            Sequence::Eight => 7,
            Sequence::Nine => 8,
            Sequence::Ten => 9,
            Sequence::Eleven => 10,
            Sequence::Twelve => 11,
            Sequence::Thirteen => 12,
            Sequence::Fourteen => 13,
            Sequence::Fifteen => 14,
            Sequence::Sixteen => 15,
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Event {
    EndOfRegularSequence,
    EndOfRegularConversion,
}

impl<ADC> Adc<ADC> {
    /// Set the ADC resolution
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }

    /// Set the sample time
    pub fn set_sample_time(&mut self, sample_time: SampleTime) {
        self.sample_time = sample_time;
    }

    /// Get the max value for the current resolution
    pub fn get_max_value(&self) -> u16 {
        match self.resolution {
            Resolution::Bits12 => 4095,
            Resolution::Bits10 => 1023,
            Resolution::Bits8 => 255,
            Resolution::Bits6 => 63,
        }
    }

    /// Release the ADC peripheral
    ///
    /// Drops `Adc` and returns the `(pac::ADC, pad::ADC_COMMON)` that it was wrapping,
    /// giving the user full access to the peripheral.
    pub fn release(self) -> ADC {
        self.adc
    }

    /// Convert a measurement to millivolts
    pub fn to_millivolts(&self, sample: u16) -> u16 {
        ((u32::from(sample) * self.calibrated_vdda) / self.resolution.to_max_count()) as u16
    }

    /// Convert a raw sample from the `Temperature` to deg C
    pub fn to_degrees_centigrade(&self, sample: u16) -> f32 {
        let sample = (u32::from(sample) * self.calibrated_vdda) / VDDA_CALIB_MV;
        (VtempCalHigh::TEMP_DEGREES - VtempCalLow::TEMP_DEGREES) as f32
          // as signed because RM0351 doesn't specify against this being an
          // inverse relation (which would result in a negative differential)
          / (VtempCalHigh::get().read() as i32 - VtempCalLow::get().read() as i32) as f32
          // this can definitely be negative so must be done as a signed value
          * (sample as i32 - VtempCalLow::get().read() as i32) as f32
          // while it would make sense for this to be `VtempCalLow::TEMP_DEGREES` (which is 30*C),
          // the RM specifically uses 30*C so this will too
          + 30.0
    }
}

impl Adc<ADC1> {
    // DMA channels:
    //  ADC1: DMA2_3 with C2S 0000
    //  ADC2: DMA2_4 with C2S 0000
    //  ADC1: DMA1_1 with C1S 0000 (implemented)
    //  ADC2: DMA1_2 with C1S 0000
}

impl<C> OneShot<ADC1, u16, C> for Adc<ADC1>
where
    C: Channel<ADC1>,
{
    type Error = Infallible;

    fn read(&mut self, channel: &mut C) -> nb::Result<u16, Self::Error> {
        self.configure_sequence(channel, Sequence::One, self.sample_time);

        self.start_conversion();
        while !self.has_completed_sequence() {}

        // Read ADC value first time and discard it, as per errata sheet.
        // The errata state that if we do conversions slower than 1 kHz, the
        // first read ADC value can be corrupted, so we discard it and measure again.
        let _ = self.current_sample();

        self.start_conversion();
        while !self.has_completed_sequence() {}

        // Read ADC value.
        let val = self.current_sample();

        // Disable ADC.
        self.disable();

        Ok(val)
    }
}

#[cfg(feature = "dma")]
impl TransferPayload for RxDma<Adc<ADC1>, dma1::C1> {
    fn start(&mut self) {
        self.channel.start();
    }

    fn stop(&mut self) {
        self.channel.stop();
    }
}

#[cfg(feature = "dma")]
impl RxDma<Adc<ADC1>, dma1::C1> {
    pub fn split(mut self) -> (Adc<ADC1>, dma1::C1) {
        self.stop();
        (self.payload, self.channel)
    }
}

#[cfg(feature = "dma")]
impl<BUFFER, const N: usize> Transfer<W, BUFFER, RxDma<Adc<ADC1>, dma1::C1>>
where
    BUFFER: Sized + StableDeref<Target = [u16; N]> + DerefMut + 'static,
{
    pub fn from_adc_dma(
        dma: RxDma<Adc<ADC1>, dma1::C1>,
        buffer: BUFFER,
        dma_mode: DmaMode,
        transfer_complete_interrupt: bool,
    ) -> Self {
        let (adc, channel) = dma.split();
        Transfer::from_adc(adc, channel, buffer, dma_mode, transfer_complete_interrupt)
    }

    /// Initiate a new DMA transfer from an ADC.
    ///
    /// `dma_mode` indicates the desired mode for DMA.
    ///
    /// If `transfer_complete_interrupt` is true, the transfer
    /// complete interrupt (= `DMA1_CH1`) will be enabled
    pub fn from_adc(
        mut adc: Adc<ADC1>,
        mut channel: dma1::C1,
        buffer: BUFFER,
        dma_mode: DmaMode,
        transfer_complete_interrupt: bool,
    ) -> Self {
        assert!(dma_mode != DmaMode::Disabled);

        let (enable, circular) = match dma_mode {
            DmaMode::Disabled => (false, false),
            DmaMode::Oneshot => (true, false),
        };

        adc.adc
            .cfgr
            .modify(|_, w| w.dmaen().bit(enable).dmacfg().bit(circular));

        channel.set_peripheral_address(&adc.adc.dr as *const _ as u32, false);

        // SAFETY: since the length of BUFFER is known to be `N`, we are allowed
        // to perform N transfers into said buffer
        channel.set_memory_address(buffer.as_ptr() as u32, true);
        channel.set_transfer_length(N as u16);

        channel.set_request_line(DmaInput::Adc1).unwrap();

        channel.ccr().modify(|_, w| unsafe {
            w.mem2mem()
                .clear_bit()
                // 00: Low, 01: Medium, 10: High, 11: Very high
                .pl()
                .bits(0b01)
                // 00: 8-bits, 01: 16-bits, 10: 32-bits, 11: Reserved
                .msize()
                .bits(0b01)
                // 00: 8-bits, 01: 16-bits, 10: 32-bits, 11: Reserved
                .psize()
                .bits(0b01)
                // Peripheral -> Mem
                .dir()
                .clear_bit()
                .circ()
                .bit(circular)
        });

        if transfer_complete_interrupt {
            channel.listen(DMAEvent::TransferComplete);
        }

        atomic::compiler_fence(Ordering::Release);

        channel.start();
        adc.start_conversion();

        Transfer::w(
            buffer,
            RxDma {
                channel,
                payload: adc,
            },
        )
    }
}

/// ADC resolution setting
///
/// The default setting is 12 bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resolution {
    /// 12-bit resolution
    Bits12 = 0b00,

    /// 10-bit resolution
    Bits10 = 0b01,

    /// 8-bit resolution
    Bits8 = 0b10,

    /// 6-bit resolution
    Bits6 = 0b11,
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Bits12
    }
}

impl Resolution {
    fn to_max_count(&self) -> u32 {
        match self {
            Resolution::Bits12 => (1 << 12) - 1,
            Resolution::Bits10 => (1 << 10) - 1,
            Resolution::Bits8 => (1 << 8) - 1,
            Resolution::Bits6 => (1 << 6) - 1,
        }
    }
}

/// ADC sample time
///
/// The default setting is 2.5 ADC clock cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum SampleTime {
    /// 2.5 ADC clock cycles
    Cycles2_5 = 0b000,

    /// 6.5 ADC clock cycles
    Cycles6_5 = 0b001,

    /// 12.5 ADC clock cycles
    Cycles12_5 = 0b010,

    /// 24.5 ADC clock cycles
    Cycles24_5 = 0b011,

    /// 47.5 ADC clock cycles
    Cycles47_5 = 0b100,

    /// 92.5 ADC clock cycles
    Cycles92_5 = 0b101,

    /// 247.5 ADC clock cycles
    Cycles247_5 = 0b110,

    /// 640.5 ADC clock cycles
    Cycles640_5 = 0b111,
}

impl Default for SampleTime {
    fn default() -> Self {
        Self::Cycles2_5
    }
}

/// Implemented for all types that represent ADC channels
pub trait Channel<T>: EmbeddedHalChannel<T, ID = u8> {
    fn set_sample_time(&mut self, adc: &mut T, sample_time: SampleTime);
}

macro_rules! impl_embedded_hal_channel {
    ($pin:ty => ($adc_type:ident, $chan:expr)) => {
        impl EmbeddedHalChannel<pac::$adc_type> for $pin {
            type ID = u8;

            fn channel() -> Self::ID {
                $chan
            }
        }
    };
}

macro_rules! impl_channel {
    ($pin:ty => ($adc_type:ident, $smpr:ident, $smp:ident, $($min_sample_time:expr)?)) => {
        impl Channel<pac::$adc_type> for $pin {
            #[inline]
            fn set_sample_time(&mut self, adc: &mut pac::$adc_type, sample_time: SampleTime) {
                $(
                    // Ensure minimum sample time.
                    let sample_time = if sample_time < $min_sample_time {
                        $min_sample_time
                    } else {
                        sample_time
                    };
                )*

                adc.$smpr.modify(|_, w| {
                    // This is sound, as all `SampleTime` values are valid for this field.
                    unsafe { w.$smp().bits(sample_time as u8) }
                })
            }
        }
    };
}

macro_rules! adc_pins {
    ($($pin:ty => ($adc_type:ident, $chan:expr, $smpr:ident, $smp:ident $(, $min_sample_time:expr)?),)+ $(,)?) => {
        $(
            impl_embedded_hal_channel!($pin => ($adc_type, $chan));
            impl_channel!($pin =>($adc_type, $smpr, $smp, $($min_sample_time)*));
        )*
    };
}

macro_rules! adc {
    (@vref: $adc_type:ident => ($common_type:ident)) => {
        /// Calculates the system VDDA by sampling the internal VREF channel and comparing
        /// the result with the value stored at the factory. If the chip's VDDA is not stable, run
        /// this before each ADC conversion.
        ///
        /// Returns the calibrated VDDA voltage in millivolts.
        #[inline]
        pub fn calibrate(&mut self, delay: &mut impl DelayUs<u32>) -> u16 {
            let vref = self.enable_vref(delay);

            let vref_cal = VrefCal::get().read();

            // This can't actually fail, it's just in a result to satisfy hal trait
            let vref_samp = self.read(&mut Vref { _0: () }).unwrap();

            // Safety: DIV by 0 is possible if vref_samp is 0
            self.calibrated_vdda = (VDDA_CALIB_MV * u32::from(vref_cal)) / u32::from(vref_samp);

            // Disable VREF again if it was disabled before.
            if let Some(vref) = vref {
                self.disable_vref(vref);
            }

            self.calibrated_vdda as u16
        }

        /// Check if the internal voltage reference channel is enabled.
        #[inline]
        pub fn is_vref_enabled(&self) -> bool {
            self.adc_common.ccr.read().vrefen().bit_is_set()
        }

        /// Enable the internal voltage reference channel.
        ///
        /// Returns `Some(Vref)` if the channel was enabled or `None` if it was already enabled before.
        #[inline]
        pub fn enable_vref(&mut self, delay: &mut impl DelayUs<u32>) -> Option<Vref> {
            if self.is_vref_enabled() {
                return None
            }

            self.adc_common.ccr.modify(|_, w| w.vrefen().set_bit());

            // "Table 24. Embedded internal voltage reference" states that it takes a maximum of 12 us
            // to stabilize the internal voltage reference, we wait a little more.
            delay.delay_us(15);

            Some(Vref { _0: () })
        }

        /// Disable the internal voltage reference channel.
        #[inline]
        pub fn disable_vref(&mut self, vref: Vref) {
            drop(vref);

            self.adc_common.ccr.modify(|_, w| w.vrefen().clear_bit());
        }
    };

    // Provide a stub implementation for ADCs that do not have a means of sampling VREF.
    (@no_vref: $adc_type:ident => ($common_type:ident)) => {
        #[inline]
        fn calibrate(&mut self, _delay: &mut impl DelayUs<u32>) {}
    };

    (@vbat: $adc_type:ident => ($common_type:ident)) => {
        /// Check if the battery voltage monitoring channel is enabled.
        #[inline]
        pub fn is_vbat_enabled(&self) -> bool {
            self.adc_common.ccr.read().ch18sel().bit_is_set()
        }

        /// Enable the battery voltage monitoring channel.
        ///
        ///
        /// Returns `Some(Vbat)` if the channel was enabled or `None` if it was already enabled before.
        #[inline]
        pub fn enable_vbat(&mut self) -> Option<Vbat> {
            if self.is_vbat_enabled() {
                return None
            }

            self.adc_common.ccr.modify(|_, w| w.ch18sel().set_bit());
            Some(Vbat { _0: () })
        }

        /// Disable the battery voltage monitoring channel.
        #[inline]
        pub fn disable_vbat(&mut self, vbat: Vbat) {
            drop(vbat);

            self.adc_common.ccr.modify(|_, w| w.ch18sel().clear_bit());
        }
    };

    (@no_vbat: $adc_type:ident => ($common_type:ident)) => {

    };

    (@vts: $adc_type:ident => ($common_type:ident)) => {
        /// Check if the internal temperature sensor channel is enabled.
        pub fn is_temperature_enabled(&self) -> bool {
            self.adc_common.ccr.read().ch17sel().bit_is_set()
        }

        /// Enable the internal temperature sensor channel.
        ///
        /// Returns `Some(Temperature)` if the channel was enabled or `None` if it was already enabled before.
        pub fn enable_temperature(&mut self, delay: &mut impl DelayUs<u32>) -> Option<Temperature> {
            if self.is_temperature_enabled() {
                return None
            }

            self.adc_common.ccr.modify(|_, w| w.ch17sel().set_bit());

            // FIXME: This note from the reference manual is currently not possible
            // rm0351 section 18.4.32 pg580 (L47/L48/L49/L4A models)
            // Note:
            // The sensor has a startup time after waking from power-down mode before it can output VTS
            // at the correct level. The ADC also has a startup time after power-on, so to minimize the
            // delay, the ADEN and CH17SEL bits should be set at the same time.
            //
            // https://github.com/STMicroelectronics/STM32CubeL4/blob/master/Drivers/STM32L4xx_HAL_Driver/Inc/stm32l4xx_ll_adc.h#L1363
            // 120us is used in the ST HAL code
            delay.delay_us(150);

            Some(Temperature { _0: () })
        }

        /// Disable the internal temperature sensor channel.
        pub fn disable_temperature(&mut self, temperature: Temperature) {
            drop(temperature);

            self.adc_common.ccr.modify(|_, w| w.ch17sel().clear_bit())
        }
    };

    (@no_vts: $adc_type:ident => ($common_type:ident)) => {

    };

    // ADC1 supports VREF, VBAT and VTS.
    (@additionals: ADC1 => ($common_type:ident)) => {
        adc!(@vref: ADC1 => ($common_type));
        adc!(@vbat: ADC1 => ($common_type));
        adc!(@vts: ADC1 => ($common_type));
    };

    // ADC3 supports VBAT and VTS.
    (@additionals: ADC3 => ($common_type:ident)) => {
        adc!(@no_vref: ADC3 => ($common_type));
        adc!(@vbat: ADC3 => ($common_type));
        adc!(@vts: ADC3 => ($common_type));
    };

    (@additionals: $adc_type:ident => ($common_type:ident)) => {
        adc!(@no_vref: $adc_type => ($common_type));
        adc!(@no_vbat: $adc_type => ($common_type));
        adc!(@no_vts: $adc_type => ($common_type));
    };

    ($($adc_type:ident => ($constructor_fn_name:ident, $common_type:ident)),+ $(,)?) => {
        $(
            impl Adc<pac::$adc_type> {
                /// Enable the ADC clock and runs calibration.
                pub fn $constructor_fn_name(
                    adc: pac::$adc_type,
                    adc_common: AdcCommon,
                    ccipr: &mut CCIPR,
                    delay: &mut impl DelayUs<u32>,
                ) -> Self {
                    // Select system clock as ADC clock source
                    ccipr.ccipr().modify(|_, w| w.adcsel().sysclk());

                    // Initialize the ADC, according to the STM32L4xx Reference Manual,
                    // section 16.4.6.
                    adc.cr.write(|w| w.deeppwd().clear_bit()); // exit deep-power-down mode
                    adc.cr.modify(|_, w| w.advregen().set_bit()); // enable internal voltage regulator

                    // According to the STM32L4xx Reference Manual, section 16.4.6, we need
                    // to wait for T_ADCVREG_STUP after enabling the internal voltage
                    // regulator. For the STM32L433, this is 20 us. We choose 25 us to
                    // account for bad clocks.
                    delay.delay_us(25);

                    // Calibration procedure according to section 16.4.8.
                    adc.cr.modify(|_, w| {
                        w.adcal().set_bit(); // start calibration
                        w.adcaldif().clear_bit(); // single-ended mode

                        w
                    });

                    while adc.cr.read().adcal().bit_is_set() {}

                    // We need to wait 4 ADC clock after ADCAL goes low, 1 us is more than enough
                    delay.delay_us(1);

                    let mut s = Self {
                        adc,
                        adc_common,
                        resolution: Resolution::default(),
                        sample_time: SampleTime::default(),
                        calibrated_vdda: VDDA_CALIB_MV,
                    };

                    s.calibrate(delay);

                    s
                }

                adc!(@additionals: $adc_type => ($common_type));

                /// Check if the ADC is enabled.
                #[inline]
                pub fn is_enabled(&self) -> bool {
                    self.adc.cr.read().aden().bit_is_set()
                }

                /// Enable the ADC.
                #[inline]
                pub fn enable(&mut self) {
                    if !self.is_enabled() {
                        // Make sure bits are off
                        while self.adc.cr.read().addis().bit_is_set() {}

                        // Clear ADRDY by setting it (See Reference Manual section 1.16.1)
                        self.adc.isr.modify(|_, w| w.adrdy().set_bit());
                        self.adc.cr.modify(|_, w| w.aden().set_bit());
                        while self.adc.isr.read().adrdy().bit_is_clear() {}

                        // Configure ADC
                        self.adc.cfgr.modify(|_, w| {
                            // This is sound, as all `Resolution` values are valid for this
                            // field.
                            unsafe { w.res().bits(self.resolution as u8) }
                        });
                    }
                }

                /// Disable the ADC.
                #[inline]
                pub fn disable(&mut self) {
                    self.adc.cr.modify(|_, w| w.addis().set_bit());
                }

                /// Returns the current sample stored in the ADC data register.
                #[inline]
                pub fn current_sample(&self) -> u16 {
                    // Sound, as bits 31:16 are reserved, read-only and 0 in ADC_DR
                    // TODO: Switch to using `rdata` once https://github.com/stm32-rs/stm32-rs/pull/723 is released.
                    self.adc.dr.read().bits() as u16
                }

                /// Configure the channel for a specific step in the sequence.
                ///
                /// Automatically sets the sequence length to the farthes sequence
                /// index that has been used so far. Use [`Adc::reset_sequence`] to
                /// reset the sequence length.
                pub fn configure_sequence<C>(
                    &mut self,
                    channel: &mut C,
                    sequence: Sequence,
                    sample_time: SampleTime,
                ) where
                    C: Channel<pac::$adc_type>,
                {
                    let channel_bits = C::channel();
                    channel.set_sample_time(&mut self.adc, sample_time);

                    unsafe {
                        // This is sound as channel() always returns a valid channel number
                        match sequence {
                            Sequence::One => self.adc.sqr1.modify(|_, w| w.sq1().bits(channel_bits)),
                            Sequence::Two => self.adc.sqr1.modify(|_, w| w.sq2().bits(channel_bits)),
                            Sequence::Three => self.adc.sqr1.modify(|_, w| w.sq3().bits(channel_bits)),
                            Sequence::Four => self.adc.sqr1.modify(|_, w| w.sq4().bits(channel_bits)),
                            Sequence::Five => self.adc.sqr2.modify(|_, w| w.sq5().bits(channel_bits)),
                            Sequence::Six => self.adc.sqr2.modify(|_, w| w.sq6().bits(channel_bits)),
                            Sequence::Seven => self.adc.sqr2.modify(|_, w| w.sq7().bits(channel_bits)),
                            Sequence::Eight => self.adc.sqr2.modify(|_, w| w.sq8().bits(channel_bits)),
                            Sequence::Nine => self.adc.sqr2.modify(|_, w| w.sq9().bits(channel_bits)),
                            Sequence::Ten => self.adc.sqr3.modify(|_, w| w.sq10().bits(channel_bits)),
                            Sequence::Eleven => self.adc.sqr3.modify(|_, w| w.sq11().bits(channel_bits)),
                            Sequence::Twelve => self.adc.sqr3.modify(|_, w| w.sq12().bits(channel_bits)),
                            Sequence::Thirteen => self.adc.sqr3.modify(|_, w| w.sq13().bits(channel_bits)),
                            Sequence::Fourteen => self.adc.sqr3.modify(|_, w| w.sq14().bits(channel_bits)),
                            Sequence::Fifteen => self.adc.sqr4.modify(|_, w| w.sq15().bits(channel_bits)),
                            Sequence::Sixteen => self.adc.sqr4.modify(|_, w| w.sq16().bits(channel_bits)),
                        }
                    }

                    // This will only ever extend the sequence, not shrink it.
                    let current_seql = self.get_sequence_length();
                    let next_seql: u8 = sequence.into();
                    if next_seql >= current_seql {
                        // Note: sequence length of 0 = 1 conversion
                        self.set_sequence_length(sequence.into());
                    }
                }

                /// Get the configured sequence length (= `actual sequence length - 1`)
                #[inline]
                pub(crate) fn get_sequence_length(&self) -> u8 {
                    self.adc.sqr1.read().l().bits()
                }

                /// Private: length must be `actual sequence length - 1`, so not API-friendly.
                /// Use [`Adc::reset_sequence`] and [`Adc::configure_sequence`] instead
                #[inline]
                fn set_sequence_length(&mut self, length: u8) {
                    self.adc.sqr1.modify(|_, w| unsafe { w.l().bits(length) });
                }

                /// Reset the sequence length to 1
                ///
                /// Does *not* erase previously configured sequence settings, only
                /// changes the sequence length
                #[inline]
                pub fn reset_sequence(&mut self) {
                    self.adc.sqr1.modify(|_, w| unsafe { w.l().bits(0b0000) })
                }

                #[inline]
                pub fn has_completed_conversion(&self) -> bool {
                    self.adc.isr.read().eoc().bit_is_set()
                }

                #[inline]
                pub fn has_completed_sequence(&self) -> bool {
                    self.adc.isr.read().eos().bit_is_set()
                }

                #[inline]
                pub fn clear_end_flags(&mut self) {
                    // EOS and EOC are reset by setting them (See reference manual section 16.6.1)
                    self.adc
                        .isr
                        .modify(|_, w| w.eos().set_bit().eoc().set_bit());
                }

                #[inline]
                pub fn start_conversion(&mut self) {
                    self.enable();
                    self.clear_end_flags();
                    self.adc.cr.modify(|_, w| w.adstart().set_bit());
                }

                #[inline]
                pub fn is_converting(&self) -> bool {
                    self.adc.cr.read().adstart().bit_is_set()
                }

                #[inline]
                pub fn listen(&mut self, event: Event) {
                    self.adc.ier.modify(|_, w| match event {
                        Event::EndOfRegularSequence => w.eosie().set_bit(),
                        Event::EndOfRegularConversion => w.eocie().set_bit(),
                    });
                }

                #[inline]
                pub fn unlisten(&mut self, event: Event) {
                    self.adc.ier.modify(|_, w| match event {
                        Event::EndOfRegularSequence => w.eosie().clear_bit(),
                        Event::EndOfRegularConversion => w.eocie().clear_bit(),
                    });
                }
            }
        )*
    };
}

adc!(ADC1 => (adc1, ADC_COMMON));

adc_pins!(
    // “Table 25. Embedded internal voltage reference” in the STM32L496xx datasheet states that
    // the sample time needs to be at least 4 us. With 640.5 ADC cycles at 80 MHz, we have a
    // minimum of 8 us, leaving some headroom.
    Vref              => (ADC1, 0,  smpr1, smp0, SampleTime::Cycles640_5),
    gpio::PC0<Analog> => (ADC1, 1,  smpr1, smp1),
    gpio::PC1<Analog> => (ADC1, 2,  smpr1, smp2),
    gpio::PC2<Analog> => (ADC1, 3,  smpr1, smp3),
    gpio::PC3<Analog> => (ADC1, 4,  smpr1, smp4),
    gpio::PA0<Analog> => (ADC1, 5,  smpr1, smp5),
    gpio::PA1<Analog> => (ADC1, 6,  smpr1, smp6),
    gpio::PA2<Analog> => (ADC1, 7,  smpr1, smp7),
    gpio::PA3<Analog> => (ADC1, 8,  smpr1, smp8),
    gpio::PA4<Analog> => (ADC1, 9,  smpr1, smp9),
    gpio::PA5<Analog> => (ADC1, 10, smpr2, smp10),
    gpio::PA6<Analog> => (ADC1, 11, smpr2, smp11),
    gpio::PA7<Analog> => (ADC1, 12, smpr2, smp12),
    gpio::PC4<Analog> => (ADC1, 13, smpr2, smp13),
    gpio::PC5<Analog> => (ADC1, 14, smpr2, smp14),
    gpio::PB0<Analog> => (ADC1, 15, smpr2, smp15),
    gpio::PB1<Analog> => (ADC1, 16, smpr2, smp16),
    Temperature       => (ADC1, 17, smpr2, smp17),
    Vbat              => (ADC1, 18, smpr2, smp18),
);

#[cfg(not(any(
    feature = "stm32l433",
    feature = "stm32l443",
    // feature = "stm32l4p5",
    // feature = "stm32l4q5",
    // feature = "stm32l4r5",
    // feature = "stm32l4s5",
    // feature = "stm32l4r7",
    // feature = "stm32l4s7",
    feature = "stm32l4r9",
    feature = "stm32l4s9",
)))]
adc!(ADC2 => (adc2, ADC_COMMON));

#[cfg(not(any(
  feature = "stm32l433",
  feature = "stm32l443",
  // feature = "stm32l4p5",
  // feature = "stm32l4q5",
  // feature = "stm32l4r5",
  // feature = "stm32l4s5",
  // feature = "stm32l4r7",
  // feature = "stm32l4s7",
  feature = "stm32l4r9",
  feature = "stm32l4s9",
)))]
adc_pins!(
    gpio::PC0<Analog> => (ADC2, 1,  smpr1, smp1),
    gpio::PC1<Analog> => (ADC2, 2,  smpr1, smp2),
    gpio::PC2<Analog> => (ADC2, 3,  smpr1, smp3),
    gpio::PC3<Analog> => (ADC2, 4,  smpr1, smp4),
    gpio::PA0<Analog> => (ADC2, 5,  smpr1, smp5),
    gpio::PA1<Analog> => (ADC2, 6,  smpr1, smp6),
    gpio::PA2<Analog> => (ADC2, 7,  smpr1, smp7),
    gpio::PA3<Analog> => (ADC2, 8,  smpr1, smp8),
    gpio::PA4<Analog> => (ADC2, 9,  smpr1, smp9),
    gpio::PA5<Analog> => (ADC2, 10, smpr2, smp10),
    gpio::PA6<Analog> => (ADC2, 11, smpr2, smp11),
    gpio::PA7<Analog> => (ADC2, 12, smpr2, smp12),
    gpio::PC4<Analog> => (ADC2, 13, smpr2, smp13),
    gpio::PC5<Analog> => (ADC2, 14, smpr2, smp14),
    gpio::PB0<Analog> => (ADC2, 15, smpr2, smp15),
    gpio::PB1<Analog> => (ADC2, 16, smpr2, smp16),
    // DAC1           => (ADC2, 17, smpr2, smp17),
    // DAC2           => (ADC2, 18, smpr2, smp18),
);

#[cfg(any(
    feature = "stm32l476",
    feature = "stm32l486",
    feature = "stm32l496",
    feature = "stm32l4a6",
))]
adc!(ADC3 => (adc3, ADC_COMMON));

#[cfg(any(
    feature = "stm32l476",
    feature = "stm32l486",
    feature = "stm32l496",
    feature = "stm32l4a6",
))]
adc_pins!(
    gpio::PC0<Analog>  => (ADC3, 1,  smpr1, smp1),
    gpio::PC1<Analog>  => (ADC3, 2,  smpr1, smp2),
    gpio::PC2<Analog>  => (ADC3, 3,  smpr1, smp3),
    gpio::PC3<Analog>  => (ADC3, 4,  smpr1, smp4),
    gpio::PF3<Analog>  => (ADC3, 6,  smpr1, smp6),
    gpio::PF4<Analog>  => (ADC3, 7,  smpr1, smp7),
    gpio::PF5<Analog>  => (ADC3, 8,  smpr1, smp8),
    gpio::PF6<Analog>  => (ADC3, 9,  smpr1, smp9),
    gpio::PF7<Analog>  => (ADC3, 10, smpr2, smp10),
    gpio::PF8<Analog>  => (ADC3, 11, smpr2, smp11),
    gpio::PF9<Analog>  => (ADC3, 12, smpr2, smp12),
    gpio::PF10<Analog> => (ADC3, 13, smpr2, smp13),
    // DAC1            => (ADC2, 14, smpr2, smp14),
    // DAC2            => (ADC2, 15, smpr2, smp15),
    Temperature        => (ADC3, 17, smpr2, smp17),
    Vbat               => (ADC3, 18, smpr2, smp18),
);
