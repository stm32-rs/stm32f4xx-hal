//! Low power timers
use crate::rcc::{Clocks, Enable, RccBus, Reset, CCIPR};

use crate::pac::{LPTIM1, LPTIM2, RCC};

/// Clock sources available for timers
pub enum ClockSource {
    /// Use PCLK as clock source
    PCLK = 0b00,
    /// Use LSI as clock source
    LSI = 0b01,
    /// Use HSI16 as clock source
    HSI16 = 0b10,
    /// Use LSE as clock source
    LSE = 0b11,
}

/// The prescaler value to use for a timer
///
/// Allow missing docs because the type is self explanatory
#[allow(missing_docs)]
pub enum PreScaler {
    U1 = 0b000,
    U2 = 0b001,
    U4 = 0b010,
    U8 = 0b011,
    U16 = 0b100,
    U32 = 0b101,
    U64 = 0b110,
    U128 = 0b111,
}

/// Count modes that are available.
///
/// All ClockSources currently supported require the Internal count mode
#[derive(PartialEq)]
pub enum CountMode {
    /// Use an internal clock source (which also includes LSE)
    Internal,
    // External,
}

/// All currently supported interrupt events
pub enum Event {
    /// Occurs when the compare value is the same as the counter value
    CompareMatch,
    /// Occurs when the arr value is the same as the counter value.
    /// When this event occurs, the counter value is set to 0 (by hardware)
    AutoReloadMatch,
}

/// Configuration of a low power timer
pub struct LowPowerTimerConfig {
    clock_source: ClockSource,
    prescaler: PreScaler,
    count_mode: CountMode,
    compare_value: u16,
    arr_value: u16,
}

impl Default for LowPowerTimerConfig {
    fn default() -> Self {
        Self {
            clock_source: ClockSource::LSI,
            prescaler: PreScaler::U1,
            count_mode: CountMode::Internal,
            compare_value: 0x0,
            arr_value: 0xFFFF,
        }
    }
}

impl LowPowerTimerConfig {
    /// Select which clock source should be used
    pub fn clock_source(mut self, clock_source: ClockSource) -> Self {
        self.clock_source = clock_source;
        self
    }

    /// Select which prescaler value should be used
    pub fn prescaler(mut self, prescaler: PreScaler) -> Self {
        self.prescaler = prescaler;
        self
    }

    /// Select the count mode that should be used
    pub fn count_mode(mut self, count_mode: CountMode) -> Self {
        self.count_mode = count_mode;
        self
    }

    /// Set the value of the compare register
    pub fn compare_value(mut self, compare_value: u16) -> Self {
        self.compare_value = compare_value;
        self
    }

    /// Set the value of the auto reload register
    pub fn arr_value(mut self, arr_value: u16) -> Self {
        self.arr_value = arr_value;
        self
    }
}

/// A low power hardware timer
///
/// Supported things:
/// * Compare match
/// * Auto reload matches
pub struct LowPowerTimer<LPTIM> {
    lptim: LPTIM,
}

macro_rules! hal {
    ($timer_type: ident, $lptimX: ident, $timXsel: ident) => {
        impl LowPowerTimer<$timer_type> {
            #[inline(always)]
            fn enable(&mut self) {
                self.set_enable(true);
            }

            #[inline(always)]
            fn disable(&mut self) {
                self.set_enable(false);
            }

            #[inline(always)]
            fn set_enable(&mut self, enabled: bool) {
                self.lptim.cr.modify(|_, w| w.enable().bit(enabled));
            }

            #[inline(always)]
            fn start_continuous_mode(&mut self) {
                self.lptim.cr.modify(|_, w| w.cntstrt().set_bit());
            }

            /// Consume the LPTIM and produce a LowPowerTimer that encapsulates
            /// said LPTIM.
            ///
            /// `config` contains details about the desired configuration for the
            /// LowPowerTimer
            ///
            /// # Panics
            /// This function panics if the value of ARR is less than or equal to CMP,
            /// and if the clock source is HSI16, LSI, or LSE and that clock is not enabled.
            pub fn $lptimX(
                lptim: $timer_type,
                config: LowPowerTimerConfig,
                apb1rn: &mut <$timer_type as RccBus>::Bus,
                ccipr: &mut CCIPR,
                clocks: Clocks,
            ) -> Self {
                let LowPowerTimerConfig {
                    clock_source,
                    count_mode,
                    prescaler,
                    compare_value,
                    arr_value,
                } = config;

                // ARR value must be strictly greater than CMP value
                assert!(arr_value > compare_value);

                // The used clock source must actually be enabled
                // PCLK is always on if a `Clocks` eixsts.
                match clock_source {
                    ClockSource::LSE => assert!(clocks.lse()),
                    ClockSource::LSI => assert!(clocks.lsi()),
                    // Check if HSI16 is enabled
                    // This operation is sound, as it is an atomic memory access
                    // that does not modify the memory/read value
                    ClockSource::HSI16 => {
                        assert!(unsafe { (&*RCC::ptr()).cr.read().hsion().bit_is_set() })
                    }
                    _ => {}
                }

                <$timer_type>::enable(apb1rn);
                <$timer_type>::reset(apb1rn);

                // This operation is sound as `ClockSource as u8` only produces valid values
                ccipr
                    .ccipr()
                    .modify(|_, w| w.$timXsel().bits(clock_source as u8));

                // This operation is sound as `PreScaler as u8` (which is the "unsafe" part) only
                // produces valid values
                lptim.cfgr.modify(|_, w| unsafe {
                    w.enc()
                        .clear_bit()
                        .countmode()
                        .bit(count_mode != CountMode::Internal)
                        .presc()
                        .bits(prescaler as u8)
                        .cksel()
                        .clear_bit()
                });

                let mut instance = LowPowerTimer { lptim };

                instance.enable();

                instance.start_continuous_mode();
                instance.set_autoreload(arr_value);
                instance.set_compare_match(compare_value);
                instance
            }

            /// Enable interrupts for the specified event
            pub fn listen(&mut self, event: Event) {
                // LPTIM_IER may only be modified when LPTIM is disabled
                self.disable();
                self.lptim.ier.modify(|_, w| match event {
                    Event::CompareMatch => w.cmpmie().set_bit(),
                    Event::AutoReloadMatch => w.arrmie().set_bit(),
                });
                self.enable();
                self.start_continuous_mode();
            }

            /// Disable interrupts for the specified event
            pub fn unlisten(&mut self, event: Event) {
                // LPTIM_IER may only be modified when LPTIM is disabled
                self.disable();
                self.lptim.ier.modify(|_, w| match event {
                    Event::CompareMatch => w.cmpmie().clear_bit(),
                    Event::AutoReloadMatch => w.arrmie().clear_bit(),
                });
                self.enable();
                self.start_continuous_mode();
            }

            /// Check if the specified event has been triggered for this LowPowerTimer.
            ///
            /// If this function returns `true` for an `Event` that this LowPowerTimer is listening for,
            /// [`LowPowerTimer::clear_event_flag`] must be called for that event to prevent the
            /// interrupt from looping eternally. This is not done in a single function to
            /// avoid using a mutable reference for an operation that does not require it.
            pub fn is_event_triggered(&self, event: Event) -> bool {
                let reg_val = self.lptim.isr.read();
                match event {
                    Event::CompareMatch => reg_val.cmpm().bit_is_set(),
                    Event::AutoReloadMatch => reg_val.arrm().bit_is_set(),
                }
            }

            /// Clear the interrupt flag for the specified event
            pub fn clear_event_flag(&mut self, event: Event) {
                self.lptim.icr.write(|w| match event {
                    Event::CompareMatch => w.cmpmcf().set_bit(),
                    Event::AutoReloadMatch => w.arrmcf().set_bit(),
                });
            }

            /// Set the compare match field for this LowPowerTimer
            #[inline]
            pub fn set_compare_match(&mut self, value: u16) {
                // clear compare register update ok flag
                self.lptim.icr.write(|w| w.cmpokcf().set_bit());

                // This operation is sound as compare_value is a u16, and there are 16 writeable bits
                // Additionally, the LPTIM peripheral will always be in the enabled state when this code is called
                self.lptim.cmp.write(|w| unsafe { w.bits(value as u32) });

                // wait for compare register update ok interrupt to be signalled
                // (see RM0394 Rev 4, sec 30.4.10 for further explanation and
                // sec. 30.7.1, Bit 4 for register field description)
                while self.lptim.isr.read().cmpok().bit_is_clear() {}
            }

            /// Set auto reload register
            /// has to be used _after_ enabling of lptim
            #[inline(always)]
            pub fn set_autoreload(&mut self, arr_value: u16) {
                // clear autoreload register OK interrupt flag
                self.lptim.icr.write(|w| w.arrokcf().set_bit());

                // Write autoreload value
                // This operation is sound as arr_value is a u16, and there are 16 writeable bits
                self.lptim
                    .arr
                    .write(|w| unsafe { w.bits(arr_value as u32) });

                // wait for autoreload write ok interrupt to be signalled
                // (see RM0394 Rev 4, sec 30.4.10 for further explanation and
                // sec. 30.7.1, Bit 4 for register field description)
                while self.lptim.isr.read().arrok().bit_is_clear() {}
            }

            /// Get the current counter value for this LowPowerTimer
            #[inline]
            pub fn get_counter(&self) -> u16 {
                self.lptim.cnt.read().bits() as u16
            }

            /// Get the value of the ARR register for this
            /// LowPowerTimer
            #[inline]
            pub fn get_arr(&self) -> u16 {
                self.lptim.arr.read().bits() as u16
            }

            pub fn pause(&mut self) {
                self.disable();
            }

            pub fn resume(&mut self) {
                self.enable();
                self.start_continuous_mode();
            }
        }
    };
}

hal!(LPTIM1, lptim1, lptim1sel);
hal!(LPTIM2, lptim2, lptim2sel);
