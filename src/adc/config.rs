//! Contains types related to ADC configuration

/// The place in the sequence a given channel should be captured
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Sequence {
    /// 1
    One = 0,
    /// 2
    Two = 1,
    /// 3
    Three = 2,
    /// 4
    Four = 3,
    /// 5
    Five = 4,
    /// 6
    Six = 5,
    /// 7
    Seven = 6,
    /// 8
    Eight = 7,
    /// 9
    Nine = 8,
    /// 10
    Ten = 9,
    /// 11
    Eleven = 10,
    /// 12
    Twelve = 11,
    /// 13
    Thirteen = 12,
    /// 14
    Fourteen = 13,
    /// 15
    Fifteen = 14,
    /// 16
    Sixteen = 15,
}

impl From<Sequence> for u8 {
    fn from(s: Sequence) -> u8 {
        s as _
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
#[repr(u8)]
pub enum SampleTime {
    /// 3 cycles
    Cycles_3 = 0,
    /// 15 cycles
    Cycles_15 = 1,
    /// 28 cycles
    Cycles_28 = 2,
    /// 56 cycles
    Cycles_56 = 3,
    /// 84 cycles
    Cycles_84 = 4,
    /// 112 cycles
    Cycles_112 = 5,
    /// 144 cycles
    Cycles_144 = 6,
    /// 480 cycles
    Cycles_480 = 7,
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
        l as _
    }
}

/// Clock config for the ADC
/// Check the datasheet for the maximum speed the ADC supports
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Clock {
    /// PCLK2 (APB2) divided by 2
    Pclk2_div_2 = 0,
    /// PCLK2 (APB2) divided by 4
    Pclk2_div_4 = 1,
    /// PCLK2 (APB2) divided by 6
    Pclk2_div_6 = 2,
    /// PCLK2 (APB2) divided by 8
    Pclk2_div_8 = 3,
}

impl From<Clock> for u8 {
    fn from(c: Clock) -> u8 {
        c as _
    }
}

/// Resolution to sample at
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Resolution {
    /// 12-bit
    Twelve = 0,
    /// 10-bit
    Ten = 1,
    /// 8-bit
    Eight = 2,
    /// 6-bit
    Six = 3,
}
impl From<Resolution> for u8 {
    fn from(r: Resolution) -> u8 {
        r as _
    }
}

/// Possible external triggers the ADC can listen to
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ExternalTrigger {
    /// TIM1 compare channel 1
    Tim_1_cc_1 = 0b0000,
    /// TIM1 compare channel 2
    Tim_1_cc_2 = 0b0001,
    /// TIM1 compare channel 3
    Tim_1_cc_3 = 0b0010,
    /// TIM2 compare channel 2
    Tim_2_cc_2 = 0b0011,
    /// TIM2 compare channel 3
    Tim_2_cc_3 = 0b0100,
    /// TIM2 compare channel 4
    Tim_2_cc_4 = 0b0101,
    /// TIM2 trigger out
    Tim_2_trgo = 0b0110,
    /// TIM3 compare channel 1
    Tim_3_cc_1 = 0b0111,
    /// TIM3 trigger out
    Tim_3_trgo = 0b1000,
    /// TIM4 compare channel 4
    Tim_4_cc_4 = 0b1001,
    /// TIM5 compare channel 1
    Tim_5_cc_1 = 0b1010,
    /// TIM5 compare channel 2
    Tim_5_cc_2 = 0b1011,
    /// TIM5 compare channel 3
    Tim_5_cc_3 = 0b1100,
    /// External interrupt line 11
    Exti_11 = 0b1111,
}
impl From<ExternalTrigger> for u8 {
    fn from(et: ExternalTrigger) -> u8 {
        et as _
    }
}

/// Possible trigger modes
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TriggerMode {
    /// Don't listen to external trigger
    Disabled = 0,
    /// Listen for rising edges of external trigger
    RisingEdge = 1,
    /// Listen for falling edges of external trigger
    FallingEdge = 2,
    /// Listen for both rising and falling edges of external trigger
    BothEdges = 3,
}
impl From<TriggerMode> for u8 {
    fn from(tm: TriggerMode) -> u8 {
        tm as _
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
    pub fn external_trigger(mut self, trigger_mode: TriggerMode, trigger: ExternalTrigger) -> Self {
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
