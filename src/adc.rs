//! Analog to digital converter configuration.
//! According to CubeMx, all STM32F4 chips use the same ADC IP so this should be correct for all variants.

#![deny(missing_docs)]

/*
    Currently unused but this is the formula for using temperature calibration:
    Temperature in °C = (110-30) * (adc_sample - VtempCal30::get().read()) / (VtempCal110::get().read()-VtempCal30::get().read()) + 30
*/

use crate::dma::traits::{DMASet, PeriAddress, SafePeripheralRead};
use crate::dma::PeripheralToMemory;
use crate::rcc::{Enable, Reset};
use crate::{
    gpio::{self, Analog},
    pac,
    signature::VrefCal,
    signature::VDDA_CALIB,
};
use core::fmt;

/// Vref internal signal, used for calibration
pub struct Vref;

/// Vbat internal signal, used for monitoring the battery (if used)
pub struct Vbat;

/// Core temperature internal signal
pub struct Temperature;

macro_rules! adc_pins {
    ($($pin:ty => ($adc:ident, $chan:expr)),+ $(,)*) => {
        $(
            impl embedded_hal::adc::Channel<pac::$adc> for $pin {
                type ID = u8;
                fn channel() -> u8 { $chan }
            }
        )+
    };
}

/// Contains types related to ADC configuration
pub mod config {
    /// The place in the sequence a given channel should be captured
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum Sequence {
        /// 1
        One,
        /// 2
        Two,
        /// 3
        Three,
        /// 4
        Four,
        /// 5
        Five,
        /// 6
        Six,
        /// 7
        Seven,
        /// 8
        Eight,
        /// 9
        Nine,
        /// 10
        Ten,
        /// 11
        Eleven,
        /// 12
        Twelve,
        /// 13
        Thirteen,
        /// 14
        Fourteen,
        /// 15
        Fifteen,
        /// 16
        Sixteen,
    }

    impl From<Sequence> for u8 {
        fn from(s: Sequence) -> u8 {
            match s {
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

    /// The number of cycles to sample a given channel for
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum SampleTime {
        /// 3 cycles
        Cycles_3,
        /// 15 cycles
        Cycles_15,
        /// 28 cycles
        Cycles_28,
        /// 56 cycles
        Cycles_56,
        /// 84 cycles
        Cycles_84,
        /// 112 cycles
        Cycles_112,
        /// 144 cycles
        Cycles_144,
        /// 480 cycles
        Cycles_480,
    }

    impl From<u8> for SampleTime {
        fn from(f: u8) -> SampleTime {
            match f {
                0 => SampleTime::Cycles_3,
                1 => SampleTime::Cycles_15,
                2 => SampleTime::Cycles_28,
                3 => SampleTime::Cycles_56,
                4 => SampleTime::Cycles_84,
                5 => SampleTime::Cycles_112,
                6 => SampleTime::Cycles_144,
                7 => SampleTime::Cycles_480,
                _ => unimplemented!(),
            }
        }
    }

    impl From<SampleTime> for u8 {
        fn from(l: SampleTime) -> u8 {
            match l {
                SampleTime::Cycles_3 => 0,
                SampleTime::Cycles_15 => 1,
                SampleTime::Cycles_28 => 2,
                SampleTime::Cycles_56 => 3,
                SampleTime::Cycles_84 => 4,
                SampleTime::Cycles_112 => 5,
                SampleTime::Cycles_144 => 6,
                SampleTime::Cycles_480 => 7,
            }
        }
    }

    /// Clock config for the ADC
    /// Check the datasheet for the maximum speed the ADC supports
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Clock {
        /// PCLK2 (APB2) divided by 2
        Pclk2_div_2,
        /// PCLK2 (APB2) divided by 4
        Pclk2_div_4,
        /// PCLK2 (APB2) divided by 6
        Pclk2_div_6,
        /// PCLK2 (APB2) divided by 8
        Pclk2_div_8,
    }

    impl From<Clock> for u8 {
        fn from(c: Clock) -> u8 {
            match c {
                Clock::Pclk2_div_2 => 0,
                Clock::Pclk2_div_4 => 1,
                Clock::Pclk2_div_6 => 2,
                Clock::Pclk2_div_8 => 3,
            }
        }
    }

    /// Resolution to sample at
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Resolution {
        /// 12-bit
        Twelve,
        /// 10-bit
        Ten,
        /// 8-bit
        Eight,
        /// 6-bit
        Six,
    }
    impl From<Resolution> for u8 {
        fn from(r: Resolution) -> u8 {
            match r {
                Resolution::Twelve => 0,
                Resolution::Ten => 1,
                Resolution::Eight => 2,
                Resolution::Six => 3,
            }
        }
    }

    /// Possible external triggers the ADC can listen to
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum ExternalTrigger {
        /// TIM1 compare channel 1
        Tim_1_cc_1,
        /// TIM1 compare channel 2
        Tim_1_cc_2,
        /// TIM1 compare channel 3
        Tim_1_cc_3,
        /// TIM2 compare channel 2
        Tim_2_cc_2,
        /// TIM2 compare channel 3
        Tim_2_cc_3,
        /// TIM2 compare channel 4
        Tim_2_cc_4,
        /// TIM2 trigger out
        Tim_2_trgo,
        /// TIM3 compare channel 1
        Tim_3_cc_1,
        /// TIM3 trigger out
        Tim_3_trgo,
        /// TIM4 compare channel 4
        Tim_4_cc_4,
        /// TIM5 compare channel 1
        Tim_5_cc_1,
        /// TIM5 compare channel 2
        Tim_5_cc_2,
        /// TIM5 compare channel 3
        Tim_5_cc_3,
        /// External interrupt line 11
        Exti_11,
    }
    impl From<ExternalTrigger> for u8 {
        fn from(et: ExternalTrigger) -> u8 {
            match et {
                ExternalTrigger::Tim_1_cc_1 => 0b0000,
                ExternalTrigger::Tim_1_cc_2 => 0b0001,
                ExternalTrigger::Tim_1_cc_3 => 0b0010,
                ExternalTrigger::Tim_2_cc_2 => 0b0011,
                ExternalTrigger::Tim_2_cc_3 => 0b0100,
                ExternalTrigger::Tim_2_cc_4 => 0b0101,
                ExternalTrigger::Tim_2_trgo => 0b0110,
                ExternalTrigger::Tim_3_cc_1 => 0b0111,
                ExternalTrigger::Tim_3_trgo => 0b1000,
                ExternalTrigger::Tim_4_cc_4 => 0b1001,
                ExternalTrigger::Tim_5_cc_1 => 0b1010,
                ExternalTrigger::Tim_5_cc_2 => 0b1011,
                ExternalTrigger::Tim_5_cc_3 => 0b1100,
                ExternalTrigger::Exti_11 => 0b1111,
            }
        }
    }

    /// Possible trigger modes
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum TriggerMode {
        /// Don't listen to external trigger
        Disabled,
        /// Listen for rising edges of external trigger
        RisingEdge,
        /// Listen for falling edges of external trigger
        FallingEdge,
        /// Listen for both rising and falling edges of external trigger
        BothEdges,
    }
    impl From<TriggerMode> for u8 {
        fn from(tm: TriggerMode) -> u8 {
            match tm {
                TriggerMode::Disabled => 0,
                TriggerMode::RisingEdge => 1,
                TriggerMode::FallingEdge => 2,
                TriggerMode::BothEdges => 3,
            }
        }
    }

    /// Data register alignment
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Align {
        /// Right align output data
        Right,
        /// Left align output data
        Left,
    }
    impl From<Align> for bool {
        fn from(a: Align) -> bool {
            match a {
                Align::Right => false,
                Align::Left => true,
            }
        }
    }

    /// Scan enable/disable
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Scan {
        /// Scan mode disabled
        Disabled,
        /// Scan mode enabled
        Enabled,
    }
    impl From<Scan> for bool {
        fn from(s: Scan) -> bool {
            match s {
                Scan::Disabled => false,
                Scan::Enabled => true,
            }
        }
    }

    /// Continuous mode enable/disable
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Continuous {
        /// Single mode, continuous disabled
        Single,
        /// Continuous mode enabled
        Continuous,
    }
    impl From<Continuous> for bool {
        fn from(c: Continuous) -> bool {
            match c {
                Continuous::Single => false,
                Continuous::Continuous => true,
            }
        }
    }

    /// DMA mode
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Dma {
        /// No DMA, disabled
        Disabled,
        /// Single DMA, DMA will be disabled after each conversion sequence
        Single,
        /// Continuous DMA, DMA will remain enabled after conversion
        Continuous,
    }

    /// End-of-conversion interrupt enabled/disabled
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Eoc {
        /// End-of-conversion interrupt disabled
        Disabled,
        /// End-of-conversion interrupt enabled per conversion
        Conversion,
        /// End-of-conversion interrupt enabled per sequence
        Sequence,
    }

    /// Configuration for the adc.
    /// There are some additional parameters on the adc peripheral that can be
    /// added here when needed but this covers several basic usecases.
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct AdcConfig {
        pub(crate) clock: Clock,
        pub(crate) resolution: Resolution,
        pub(crate) align: Align,
        pub(crate) scan: Scan,
        pub(crate) external_trigger: (TriggerMode, ExternalTrigger),
        pub(crate) continuous: Continuous,
        pub(crate) dma: Dma,
        pub(crate) end_of_conversion_interrupt: Eoc,
        pub(crate) default_sample_time: SampleTime,
        pub(crate) vdda: Option<u32>,
    }

    impl AdcConfig {
        /// change the clock field
        pub fn clock(mut self, clock: Clock) -> Self {
            self.clock = clock;
            self
        }
        /// change the resolution field
        pub fn resolution(mut self, resolution: Resolution) -> Self {
            self.resolution = resolution;
            self
        }
        /// change the align field
        pub fn align(mut self, align: Align) -> Self {
            self.align = align;
            self
        }
        /// change the scan field
        pub fn scan(mut self, scan: Scan) -> Self {
            self.scan = scan;
            self
        }
        /// change the external_trigger field
        pub fn external_trigger(
            mut self,
            trigger_mode: TriggerMode,
            trigger: ExternalTrigger,
        ) -> Self {
            self.external_trigger = (trigger_mode, trigger);
            self
        }
        /// change the continuous field
        pub fn continuous(mut self, continuous: Continuous) -> Self {
            self.continuous = continuous;
            self
        }
        /// change the dma field
        pub fn dma(mut self, dma: Dma) -> Self {
            self.dma = dma;
            self
        }
        /// change the end_of_conversion_interrupt field
        pub fn end_of_conversion_interrupt(mut self, end_of_conversion_interrupt: Eoc) -> Self {
            self.end_of_conversion_interrupt = end_of_conversion_interrupt;
            self
        }
        /// change the default_sample_time field
        pub fn default_sample_time(mut self, default_sample_time: SampleTime) -> Self {
            self.default_sample_time = default_sample_time;
            self
        }

        /// Specify the reference voltage for the ADC.
        ///
        /// # Args
        /// * `vdda_mv` - The ADC reference voltage in millivolts.
        pub fn reference_voltage(mut self, vdda_mv: u32) -> Self {
            self.vdda = Some(vdda_mv);
            self
        }
    }

    impl Default for AdcConfig {
        fn default() -> Self {
            Self {
                clock: Clock::Pclk2_div_2,
                resolution: Resolution::Twelve,
                align: Align::Right,
                scan: Scan::Disabled,
                external_trigger: (TriggerMode::Disabled, ExternalTrigger::Tim_1_cc_1),
                continuous: Continuous::Single,
                dma: Dma::Disabled,
                end_of_conversion_interrupt: Eoc::Disabled,
                default_sample_time: SampleTime::Cycles_480,
                vdda: None,
            }
        }
    }
}

/// Analog to Digital Converter
/// # Status
/// Most options relating to regular conversions are implemented. One-shot and sequences of conversions
/// have been tested and work as expected.
///
/// GPIO to channel mapping should be correct for all supported F4 devices. The mappings were taken from
/// CubeMX. The mappings are feature gated per 4xx device but there are actually sub variants for some
/// devices and some pins may be missing on some variants. The implementation has been split up and commented
/// to show which pins are available on certain device variants but currently the library doesn't enforce this.
/// To fully support the right pins would require 10+ more features for the various variants.
/// ## Todo
/// * Injected conversions
/// * Analog watchdog config
/// * Discontinuous mode
/// # Examples
/// ## One-shot conversion
/// ```
/// use stm32f4xx_hal::{
///   gpio::gpioa,
///   adc::{
///     Adc,
///     config::AdcConfig,
///     config::SampleTime,
///   },
/// };
///
/// let mut adc = Adc::adc1(device.ADC1, true, AdcConfig::default());
/// let pa3 = gpioa.pa3.into_analog();
/// let sample = adc.convert(&pa3, SampleTime::Cycles_480);
/// let millivolts = adc.sample_to_millivolts(sample);
/// info!("pa3: {}mV", millivolts);
/// ```
///
/// ## Sequence conversion
/// ```
/// use stm32f4xx_hal::{
///   gpio::gpioa,
///   adc::{
///     Adc,
///     config::AdcConfig,
///     config::SampleTime,
///     config::Sequence,
///     config::Eoc,
///     config::Scan,
///     config::Clock,
///   },
/// };
///
/// let config = AdcConfig::default()
///     //We'll either need DMA or an interrupt per conversion to convert
///     //multiple values in a sequence
///     .end_of_conversion_interrupt(Eoc::Conversion)
///     //Scan mode is also required to convert a sequence
///     .scan(Scan::Enabled)
///     //And since we're looking for one interrupt per conversion the
///     //clock will need to be fairly slow to avoid overruns breaking
///     //the sequence. If you are running in debug mode and logging in
///     //the interrupt, good luck... try setting pclk2 really low.
///     //(Better yet use DMA)
///     .clock(Clock::Pclk2_div_8);
/// let mut adc = Adc::adc1(device.ADC1, true, config);
/// let pa0 = gpioa.pa0.into_analog();
/// let pa3 = gpioa.pa3.into_analog();
/// adc.configure_channel(&pa0, Sequence::One, SampleTime::Cycles_112);
/// adc.configure_channel(&pa3, Sequence::Two, SampleTime::Cycles_480);
/// adc.configure_channel(&pa0, Sequence::Three, SampleTime::Cycles_112);
/// adc.start_conversion();
/// ```
///
/// ## External trigger
///
/// A common mistake on STM forums is enabling continuous mode but that causes it to start
/// capturing on the first trigger and capture as fast as possible forever, regardless of
/// future triggers. Continuous mode is disabled by default but I thought it was worth
/// highlighting.
///
/// Getting the timer config right to make sure it's sending the event the ADC is listening
/// to can be a bit of a pain but the key fields are highlighted below. Try hooking a timer
/// channel up to an external pin with an LED or oscilloscope attached to check it's really
/// generating pulses if the ADC doesn't seem to be triggering.
/// ```
/// use stm32f4xx_hal::{
///   gpio::gpioa,
///   adc::{
///     Adc,
///     config::AdcConfig,
///     config::SampleTime,
///     config::Sequence,
///     config::Eoc,
///     config::Scan,
///     config::Clock,
///   },
/// };
///
///  let config = AdcConfig::default()
///      //Set the trigger you want
///      .external_trigger(TriggerMode::RisingEdge, ExternalTrigger::Tim_1_cc_1);
///  let mut adc = Adc::adc1(device.ADC1, true, config);
///  let pa0 = gpioa.pa0.into_analog();
///  adc.configure_channel(&pa0, Sequence::One, SampleTime::Cycles_112);
///  //Make sure it's enabled but don't start the conversion
///  adc.enable();
///
/// //Configure the timer
/// let mut tim = Timer::tim1(device.TIM1, 1.hz(), clocks);
/// unsafe {
///     let tim = &(*TIM1::ptr());
///
///     //Channel 1
///     //Disable the channel before configuring it
///     tim.ccer.modify(|_, w| w.cc1e().clear_bit());
///
///     tim.ccmr1_output().modify(|_, w| w
///       //Preload enable for channel
///       .oc1pe().set_bit()
///
///       //Set mode for channel, the default mode is "frozen" which won't work
///       .oc1m().pwm_mode1()
///     );
///
///     //Set the duty cycle, 0 won't work in pwm mode but might be ok in
///     //toggle mode or match mode
///     let max_duty = tim.arr.read().arr().bits() as u16;
///     tim.ccr1.modify(|_, w| w.ccr().bits(max_duty / 2));
///
///     //Enable the channel
///     tim.ccer.modify(|_, w| w.cc1e().set_bit());
///
///     //Enable the TIM main Output
///     tim.bdtr.modify(|_, w| w.moe().set_bit());
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Adc<ADC> {
    /// Current config of the ADC, kept up to date by the various set methods
    config: config::AdcConfig,
    /// The adc peripheral
    adc_reg: ADC,
    /// VDDA in millivolts calculated from the factory calibration and vrefint
    calibrated_vdda: u32,
    /// Exclusive limit for the sample value possible for the configured resolution.
    max_sample: u32,
}
impl<ADC> fmt::Debug for Adc<ADC> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Adc: {{ calibrated_vdda: {:?}, max_sample: {:?}, config: {:?}, ... }}",
            self.calibrated_vdda, self.max_sample, self.config
        )
    }
}

macro_rules! adc {
    // Note that only ADC1 supports measurement of VREF, VBAT, and the internal temperature sensor.
    (additionals: ADC1 => ($common_type:ident)) => {
        /// Calculates the system VDDA by sampling the internal VREF channel and comparing
        /// the result with the value stored at the factory.
        pub fn calibrate(&mut self) {
            self.enable();

            let vref_en = self.temperature_and_vref_enabled();
            if !vref_en {
                self.enable_temperature_and_vref();
            }

            let vref_cal = VrefCal::get().read();
            let vref_samp = self.read(&mut Vref).unwrap(); //This can't actually fail, it's just in a result to satisfy hal trait

            self.calibrated_vdda = (VDDA_CALIB * u32::from(vref_cal)) / u32::from(vref_samp);
            if !vref_en {
                self.disable_temperature_and_vref();
            }
        }

        /// Enables the vbat internal channel
        pub fn enable_vbat(&self) {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.vbate().set_bit());
            }
        }

        /// Enables the vbat internal channel
        pub fn disable_vbat(&self) {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.vbate().clear_bit());
            }
        }

        /// Enables the temp and vref internal channels.
        /// They can't work while vbat is also enabled so this method also disables vbat.
        pub fn enable_temperature_and_vref(&mut self) {
            //VBAT prevents TS and VREF from being sampled
            self.disable_vbat();
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.tsvrefe().set_bit());
            }
        }

        /// Disables the temp and vref internal channels
        pub fn disable_temperature_and_vref(&mut self) {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.tsvrefe().clear_bit());
            }
        }

        /// Returns if the temp and vref internal channels are enabled
        pub fn temperature_and_vref_enabled(&mut self) -> bool {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.read().tsvrefe().bit_is_set()
            }
        }
    };

    // Provide a stub implementation for ADCs that do not have a means of sampling VREF.
    (additionals: $adc_type:ident => ($common_type:ident)) => {
        fn calibrate(&mut self) {}
    };

    ($($adc_type:ident => ($constructor_fn_name:ident, $common_type:ident, $en_bit: expr)),+ $(,)*) => {
        $(
            impl SafePeripheralRead for Adc<pac::$adc_type> { }

            impl Adc<pac::$adc_type> {

                adc!(additionals: $adc_type => ($common_type));

                /// Enables the ADC clock, resets the peripheral (optionally), runs calibration and applies the supplied config
                /// # Arguments
                /// * `reset` - should a reset be performed. This is provided because on some devices multiple ADCs share the same common reset
                pub fn $constructor_fn_name(adc: pac::$adc_type, reset: bool, config: config::AdcConfig) -> Adc<pac::$adc_type> {
                    unsafe {
                        // All ADCs share the same reset interface.
                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        let rcc = &(*pac::RCC::ptr());

                        //Enable the clock
                        pac::$adc_type::enable(rcc);

                        if reset {
                            //Reset the peripheral(s)
                            pac::$adc_type::reset(rcc);
                        }
                    }

                    let mut s = Self {
                        config,
                        adc_reg: adc,
                        calibrated_vdda: VDDA_CALIB,
                        max_sample: 0,
                    };

                    //Probably unnecessary to disable the ADC in most cases but it shouldn't do any harm either
                    s.disable();
                    s.apply_config(config);

                    s.enable();
                    s.calibrate();

                    // If the user specified a VDDA, use that over the internally determined value.
                    if let Some(vdda) = s.config.vdda {
                        s.calibrated_vdda = vdda;
                    }

                    s
                }

                /// Applies all fields in AdcConfig
                pub fn apply_config(&mut self, config: config::AdcConfig) {
                    self.set_clock(config.clock);
                    self.set_resolution(config.resolution);
                    self.set_align(config.align);
                    self.set_scan(config.scan);
                    self.set_external_trigger(config.external_trigger);
                    self.set_continuous(config.continuous);
                    self.set_dma(config.dma);
                    self.set_end_of_conversion_interrupt(config.end_of_conversion_interrupt);
                    self.set_default_sample_time(config.default_sample_time);

                    if let Some(vdda) = config.vdda {
                        self.calibrated_vdda = vdda;
                    }
                }

                /// Returns if the adc is enabled
                pub fn is_enabled(&self) -> bool {
                    self.adc_reg.cr2.read().adon().bit_is_set()
                }

                /// Enables the adc
                pub fn enable(&mut self) {
                    self.adc_reg.cr2.modify(|_, w| w.adon().set_bit());
                }

                /// Disables the adc
                /// # Note
                /// The ADC in the f4 has few restrictions on what can be configured while the ADC
                /// is enabled. If any bugs are found where some settings aren't "sticking" try disabling
                /// the ADC before changing them. The reference manual for the chip I'm using only states
                /// that the sequence registers are locked when they are being converted.
                pub fn disable(&mut self) {
                    self.adc_reg.cr2.modify(|_, w| w.adon().clear_bit());
                }

                /// Starts conversion sequence. Waits for the hardware to indicate it's actually started.
                pub fn start_conversion(&mut self) {
                    self.enable();
                    self.clear_end_of_conversion_flag();
                    //Start conversion
                    self.adc_reg.cr2.modify(|_, w| w.swstart().set_bit());

                    while !self.adc_reg.sr.read().strt().bit_is_set() {}
                }

                /// Sets the clock for the adc
                pub fn set_clock(&mut self, clock: config::Clock) {
                    self.config.clock = clock;
                    unsafe {
                        let common = &(*pac::$common_type::ptr());
                        common.ccr.modify(|_, w| w.adcpre().bits(clock.into()));
                    }
                }

                /// Sets the sampling resolution
                pub fn set_resolution(&mut self, resolution: config::Resolution) {
                    self.max_sample = match resolution {
                        config::Resolution::Twelve => (1 << 12),
                        config::Resolution::Ten => (1 << 10),
                        config::Resolution::Eight => (1 << 8),
                        config::Resolution::Six => (1 << 6),
                    };
                    self.config.resolution = resolution;
                    self.adc_reg.cr1.modify(|_, w| w.res().bits(resolution.into()));
                }

                /// Sets the DR register alignment to left or right
                pub fn set_align(&mut self, align: config::Align) {
                    self.config.align = align;
                    self.adc_reg.cr2.modify(|_, w| w.align().bit(align.into()));
                }

                /// Enables and disables scan mode
                pub fn set_scan(&mut self, scan: config::Scan) {
                    self.config.scan = scan;
                    self.adc_reg.cr1.modify(|_, w| w.scan().bit(scan.into()));
                }

                /// Sets which external trigger to use and if it is disabled, rising, falling or both
                pub fn set_external_trigger(&mut self, (edge, extsel): (config::TriggerMode, config::ExternalTrigger)) {
                    self.config.external_trigger = (edge, extsel);
                    #[cfg(any(
                        feature = "stm32f401",
                        feature = "stm32f410",
                        feature = "stm32f411",
                    ))] // TODO: fix pac
                    self.adc_reg.cr2.modify(|_, w| unsafe { w
                        .extsel().bits(extsel.into())
                        .exten().bits(edge.into())
                    });
                    #[cfg(not(any(
                        feature = "stm32f401",
                        feature = "stm32f410",
                        feature = "stm32f411",
                    )))]
                    self.adc_reg.cr2.modify(|_, w| w
                        .extsel().bits(extsel.into())
                        .exten().bits(edge.into())
                    );
                }

                /// Enables and disables continuous mode
                pub fn set_continuous(&mut self, continuous: config::Continuous) {
                    self.config.continuous = continuous;
                    self.adc_reg.cr2.modify(|_, w| w.cont().bit(continuous.into()));
                }

                /// Sets DMA to disabled, single or continuous
                pub fn set_dma(&mut self, dma: config::Dma) {
                    self.config.dma = dma;
                    let (dds, en) = match dma {
                        config::Dma::Disabled => (false, false),
                        config::Dma::Single => (false, true),
                        config::Dma::Continuous => (true, true),
                    };
                    self.adc_reg.cr2.modify(|_, w| w
                        //DDS stands for "DMA disable selection"
                        //0 means do one DMA then stop
                        //1 means keep sending DMA requests as long as DMA=1
                        .dds().bit(dds)
                        .dma().bit(en)
                    );
                }

                /// Sets if the end-of-conversion behaviour.
                /// The end-of-conversion interrupt occur either per conversion or for the whole sequence.
                pub fn set_end_of_conversion_interrupt(&mut self, eoc: config::Eoc) {
                    self.config.end_of_conversion_interrupt = eoc;
                    let (en, eocs) = match eoc {
                        config::Eoc::Disabled => (false, false),
                        config::Eoc::Conversion => (true, true),
                        config::Eoc::Sequence => (true, false),
                    };
                    self.adc_reg.cr1.modify(|_, w| w.eocie().bit(en));
                    self.adc_reg.cr2.modify(|_, w| w.eocs().bit(eocs));
                }

                /// Resets the end-of-conversion flag
                pub fn clear_end_of_conversion_flag(&mut self) {
                    self.adc_reg.sr.modify(|_, w| w.eoc().clear_bit());
                }

                /// Sets the default sample time that is used for one-shot conversions.
                /// [configure_channel](#method.configure_channel) and [start_conversion](#method.start_conversion) can be \
                /// used for configurations where different sampling times are required per channel.
                pub fn set_default_sample_time(&mut self, sample_time: config::SampleTime) {
                    self.config.default_sample_time = sample_time;
                }

                /// Returns the current sequence length. Primarily useful for configuring DMA.
                pub fn sequence_length(&mut self) -> u8 {
                    self.adc_reg.sqr1.read().l().bits() + 1
                }

                /// Reset the sequence
                pub fn reset_sequence(&mut self) {
                    //The reset state is One conversion selected
                    self.adc_reg.sqr1.modify(|_, w| w.l().bits(config::Sequence::One.into()));
                }

                /// Returns the address of the ADC data register. Primarily useful for configuring DMA.
                pub fn data_register_address(&mut self) -> u32 {
                    &self.adc_reg.dr as *const _ as u32
                }

                /// Configure a channel for sampling.
                /// It will make sure the sequence is at least as long as the `sequence` provided.
                /// # Arguments
                /// * `channel` - channel to configure
                /// * `sequence` - where in the sequence to sample the channel. Also called rank in some STM docs/code
                /// * `sample_time` - how long to sample for. See datasheet and ref manual to work out how long you need\
                /// to sample for at a given ADC clock frequency
                pub fn configure_channel<CHANNEL>(&mut self, _channel: &CHANNEL, sequence: config::Sequence, sample_time: config::SampleTime)
                where
                    CHANNEL: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>
                {
                    //Check the sequence is long enough
                    self.adc_reg.sqr1.modify(|r, w| {
                        let prev: config::Sequence = r.l().bits().into();
                        if prev < sequence {
                            w.l().bits(sequence.into())
                        } else {
                            w
                        }
                    });

                    let channel = CHANNEL::channel();

                    //Set the channel in the right sequence field
                    match sequence {
                        config::Sequence::One      => self.adc_reg.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel) }),
                        config::Sequence::Two      => self.adc_reg.sqr3.modify(|_, w| unsafe {w.sq2().bits(channel) }),
                        config::Sequence::Three    => self.adc_reg.sqr3.modify(|_, w| unsafe {w.sq3().bits(channel) }),
                        config::Sequence::Four     => self.adc_reg.sqr3.modify(|_, w| unsafe {w.sq4().bits(channel) }),
                        config::Sequence::Five     => self.adc_reg.sqr3.modify(|_, w| unsafe {w.sq5().bits(channel) }),
                        config::Sequence::Six      => self.adc_reg.sqr3.modify(|_, w| unsafe {w.sq6().bits(channel) }),
                        config::Sequence::Seven    => self.adc_reg.sqr2.modify(|_, w| unsafe {w.sq7().bits(channel) }),
                        config::Sequence::Eight    => self.adc_reg.sqr2.modify(|_, w| unsafe {w.sq8().bits(channel) }),
                        config::Sequence::Nine     => self.adc_reg.sqr2.modify(|_, w| unsafe {w.sq9().bits(channel) }),
                        config::Sequence::Ten      => self.adc_reg.sqr2.modify(|_, w| unsafe {w.sq10().bits(channel) }),
                        config::Sequence::Eleven   => self.adc_reg.sqr2.modify(|_, w| unsafe {w.sq11().bits(channel) }),
                        config::Sequence::Twelve   => self.adc_reg.sqr2.modify(|_, w| unsafe {w.sq12().bits(channel) }),
                        config::Sequence::Thirteen => self.adc_reg.sqr1.modify(|_, w| unsafe {w.sq13().bits(channel) }),
                        config::Sequence::Fourteen => self.adc_reg.sqr1.modify(|_, w| unsafe {w.sq14().bits(channel) }),
                        config::Sequence::Fifteen  => self.adc_reg.sqr1.modify(|_, w| unsafe {w.sq15().bits(channel) }),
                        config::Sequence::Sixteen  => self.adc_reg.sqr1.modify(|_, w| unsafe {w.sq16().bits(channel) }),
                    }

                    fn replace_bits(mut v: u32, offset: u32, width: u32, value: u32) -> u32 {
                        let mask = !(((1 << width) -1) << (offset * width));
                        v &= mask;
                        v |= value << (offset * width);
                        v
                    }

                    //Set the sample time for the channel
                    let st = u8::from(sample_time);
                    let st = u32::from(st);
                    let ch = u32::from(channel);
                    match channel {
                        0..=9   => self.adc_reg.smpr2.modify(|r, w| unsafe { w.bits(replace_bits(r.bits(), ch, 3, st)) }),
                        10..=18 => self.adc_reg.smpr1.modify(|r, w| unsafe { w.bits(replace_bits(r.bits(), ch-10, 3, st)) }),
                        _ => unimplemented!(),
                    }
                }

                /// Returns the current sample stored in the ADC data register
                pub fn current_sample(&self) -> u16 {
                    self.adc_reg.dr.read().data().bits()
                }

                /// Converts a sample value to millivolts using calibrated VDDA and configured resolution.
                /// Due to the ADC characteristics VDDA will never be reached as described in #362 and
                /// [AN2834-How to get the best ADC accuracy in STM32 microcontrollers](https://www.st.com/resource/en/application_note/cd00211314-how-to-get-the-best-adc-accuracy-in-stm32-microcontrollers-stmicroelectronics.pdf) in section 3.1.2.
                pub fn sample_to_millivolts(&self, sample: u16) -> u16 {
                    ((u32::from(sample) * self.calibrated_vdda) / self.max_sample) as u16
                }

                /// Make a converter for samples to millivolts
                pub fn make_sample_to_millivolts(&self) -> impl Fn(u16)->u16 {
                    let calibrated_vdda= self.calibrated_vdda;
                    let max_sample=self.max_sample;
                    move |sample| {
                     ((u32::from(sample) * calibrated_vdda) / max_sample) as u16
                    }
                }

                /// Returns the VDDA in millivolts calculated from the factory calibration and vrefint. Can be used to get calibration data from ADC1 and use it to configure ADCs that don't support calibration.
                pub fn reference_voltage(&self) -> u32 {
                    self.calibrated_vdda
                }

                /// Block until the conversion is completed
                /// # Panics
                /// Will panic if there is no conversion started and the end-of-conversion bit is not set
                pub fn wait_for_conversion_sequence(&self) {
                    if !self.adc_reg.sr.read().strt().bit_is_set() && !self.adc_reg.sr.read().eoc().bit_is_set() {
                        panic!("Waiting for end-of-conversion but no conversion started");
                    }
                    while !self.adc_reg.sr.read().eoc().bit_is_set() {}
                    //Clear the conversion started flag
                    self.adc_reg.sr.modify(|_, w| w.strt().clear_bit());
                }

                /// Synchronously convert a single sample
                /// Note that it reconfigures the adc sequence and doesn't restore it
                pub fn convert<PIN>(&mut self, pin: &PIN, sample_time: config::SampleTime) -> u16
                where
                    PIN: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>
                {
                    self.adc_reg.cr2.modify(|_, w| w
                        .dma().clear_bit() //Disable dma
                        .cont().clear_bit() //Disable continuous mode
                        .exten().bits(config::TriggerMode::Disabled.into()) //Disable trigger
                        .eocs().clear_bit() //EOC is set at the end of the sequence
                    );
                    self.adc_reg.cr1.modify(|_, w| w
                        .scan().clear_bit() //Disable scan mode
                        .eocie().clear_bit() //Disable end of conversion interrupt
                    );

                    self.reset_sequence();
                    self.configure_channel(pin, config::Sequence::One, sample_time);
                    self.enable();
                    self.clear_end_of_conversion_flag();
                    self.start_conversion();

                    //Wait for the sequence to complete
                    self.wait_for_conversion_sequence();

                    let result = self.current_sample();

                    //Reset the config
                    self.apply_config(self.config);

                    result
                }
            }

            impl Adc<pac::$adc_type> {
                fn read<PIN>(&mut self, pin: &mut PIN) -> nb::Result<u16, ()>
                    where PIN: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>,
                {
                    let enabled = self.is_enabled();
                    if !enabled {
                        self.enable();
                    }

                    let sample = self.convert(pin, self.config.default_sample_time);

                    if !enabled {
                        self.disable();
                    }

                    Ok(sample)
                }
            }

            impl<PIN> embedded_hal::adc::OneShot<pac::$adc_type, u16, PIN> for Adc<pac::$adc_type>
            where
                PIN: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>,
            {
                type Error = ();

                fn read(&mut self, pin: &mut PIN) -> nb::Result<u16, Self::Error> {
                    self.read::<PIN>(pin)
                }
            }

            unsafe impl PeriAddress for Adc<pac::$adc_type> {
                #[inline(always)]
                fn address(&self) -> u32 {
                    &self.adc_reg.dr as *const _ as u32
                }

                type MemSize = u16;
            }
        )+
    };
}

unsafe impl<ADC, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory> for Adc<ADC> where
    ADC: DMASet<STREAM, CHANNEL, PeripheralToMemory>
{
}

adc!(ADC1 => (adc1, ADC_COMMON, 8));

#[cfg(feature = "adc2")]
adc!(ADC2 => (adc2, ADC_COMMON, 9));

#[cfg(feature = "adc3")]
adc!(ADC3 => (adc3, ADC_COMMON, 10));

#[cfg(feature = "stm32f401")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    Temperature => (ADC1, 16),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on C variant
#[cfg(feature = "stm32f401")]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(any(feature = "stm32f405", feature = "stm32f415"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    Temperature => (ADC1, 16),
    Temperature => (ADC2, 16),
    Temperature => (ADC3, 16),
    Vbat => (ADC1, 18),
    Vbat => (ADC2, 18),
    Vbat => (ADC3, 18),
    Vref => (ADC1, 17),
    Vref => (ADC2, 17),
    Vref => (ADC3, 17),
);

// Not available on O variant
#[cfg(any(feature = "stm32f405", feature = "stm32f415"))]
adc_pins!(
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(any(feature = "stm32f407", feature = "stm32f417"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    Temperature => (ADC1, 16),
    Temperature => (ADC2, 16),
    Temperature => (ADC3, 16),
    Vbat => (ADC1, 18),
    Vbat => (ADC2, 18),
    Vbat => (ADC3, 18),
    Vref => (ADC1, 17),
    Vref => (ADC2, 17),
    Vref => (ADC3, 17),
);

// Not available on V variant
#[cfg(any(feature = "stm32f407", feature = "stm32f417"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(feature = "stm32f410")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA5<Analog> => (ADC1, 5),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 16),
    Vref => (ADC1, 17),
);

// Not available on T variant
#[cfg(feature = "stm32f410")]
adc_pins!(
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
);

// Only available on R variant
#[cfg(feature = "stm32f410")]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(feature = "stm32f411")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    Temperature => (ADC1, 16),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on C variant
#[cfg(feature = "stm32f411")]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(feature = "stm32f412")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on C variant
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
adc_pins!(
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC5<Analog> => (ADC1, 15),
);

#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on V variant
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);

// Only available on I and Z variants
#[cfg(any(feature = "stm32f427", feature = "stm32f437"))]
adc_pins!(
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(any(feature = "stm32f429", feature = "stm32f439"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on V variant
#[cfg(any(feature = "stm32f429", feature = "stm32f439"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);

// Not available on V or A variants
#[cfg(any(feature = "stm32f429", feature = "stm32f439"))]
adc_pins!(
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(feature = "stm32f446")]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on M variant
#[cfg(feature = "stm32f446")]
adc_pins!(
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
    gpio::PC5<Analog> => (ADC3, 15),
);

// Only available on Z variant
#[cfg(feature = "stm32f446")]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA0<Analog> => (ADC2, 0),
    gpio::PA0<Analog> => (ADC3, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA1<Analog> => (ADC2, 1),
    gpio::PA1<Analog> => (ADC3, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA2<Analog> => (ADC2, 2),
    gpio::PA2<Analog> => (ADC3, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA3<Analog> => (ADC2, 3),
    gpio::PA3<Analog> => (ADC3, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA4<Analog> => (ADC2, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA5<Analog> => (ADC2, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA6<Analog> => (ADC2, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PA7<Analog> => (ADC2, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB0<Analog> => (ADC2, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB1<Analog> => (ADC2, 9),
    gpio::PC0<Analog> => (ADC1, 10),
    gpio::PC0<Analog> => (ADC2, 10),
    gpio::PC0<Analog> => (ADC3, 10),
    gpio::PC1<Analog> => (ADC1, 11),
    gpio::PC1<Analog> => (ADC2, 11),
    gpio::PC1<Analog> => (ADC3, 11),
    Temperature => (ADC1, 18),
    Vbat => (ADC1, 18),
    Vref => (ADC1, 17),
);

// Not available on A variant
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PC2<Analog> => (ADC1, 12),
    gpio::PC2<Analog> => (ADC2, 12),
    gpio::PC2<Analog> => (ADC3, 12),
    gpio::PC3<Analog> => (ADC1, 13),
    gpio::PC3<Analog> => (ADC2, 13),
    gpio::PC3<Analog> => (ADC3, 13),
);

// Not available on V or A variants
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PC4<Analog> => (ADC1, 14),
    gpio::PC4<Analog> => (ADC2, 14),
    gpio::PC5<Analog> => (ADC1, 15),
    gpio::PC5<Analog> => (ADC2, 15),
);

// Not available on V variant
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PF10<Analog> => (ADC3, 8),
    gpio::PF3<Analog> => (ADC3, 9),
    gpio::PF4<Analog> => (ADC3, 14),
    gpio::PF5<Analog> => (ADC3, 15),
);

// Only available on B/I/N variants
#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
adc_pins!(
    gpio::PF6<Analog> => (ADC3, 4),
    gpio::PF7<Analog> => (ADC3, 5),
    gpio::PF8<Analog> => (ADC3, 6),
    gpio::PF9<Analog> => (ADC3, 7),
);
