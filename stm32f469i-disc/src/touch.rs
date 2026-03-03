//! FT6X06 touch controller setup for STM32F469I-DISCO board
//!
//! Provides convenient initialization for the FT6X06 capacitive touch controller
//! on the correct I2C bus and interrupt pin for this board.
//!
//! # Usage
//!
//! ```no_run
//! let mut rcc = dp.RCC.freeze(...);
//! let gpiob = dp.GPIOB.split(&mut rcc);
//! let gpioc = dp.GPIOC.split(&mut rcc);
//! let i2c = touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);
//! let ts_int = gpioc.pc1.into_pull_down_input();
//! let mut touch = touch::init_ft6x06(&i2c, ts_int);
//! // Or with calibration:
//! let mut touch = touch::init_touchscreen(&mut i2c, ts_int, &mut delay);
//! ```

use crate::hal::gpio::alt::i2c1;
use crate::hal::i2c::I2c;
use crate::hal::pac::I2C1;
use crate::hal::prelude::*;
use crate::hal::rcc::Rcc;
use ft6x06::Ft6X06;

/// FT6X06 I2C slave address
pub const FT6X06_I2C_ADDR: u8 = 0x38;

/// Initialize I2C1 and return for use with touch controller
///
/// I2C1 uses:
/// - PB8 (SCL)
/// - PB9 (SDA)
/// at 400 kHz
///
/// Pass the individual pins directly from GPIO split:
/// ```no_run
/// let gpiob = dp.GPIOB.split(&mut rcc);
/// let i2c = touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);
/// ```
pub fn init_i2c(
    i2c: I2C1,
    pb8: impl Into<i2c1::Scl>,
    pb9: impl Into<i2c1::Sda>,
    rcc: &mut Rcc,
) -> I2c<I2C1> {
    // PB8 = SCL, PB9 = SDA for I2C1 on STM32F469I-DISCO
    // I2c::new expects (SCL, SDA) order
    I2c::new(i2c, (pb8, pb9), 400.kHz(), rcc)
}

/// Initialize the FT6X06 touch controller
///
/// The interrupt pin `ts_int` should be created as:
/// ```no_run
/// let ts_int = gpioc.pc1.into_pull_down_input();
/// ```
///
/// Returns `Some(Ft6X06)` if the touch controller is detected on the I2C bus,
/// `None` if it fails to initialize.
pub fn init_ft6x06<T>(i2c: &I2c<I2C1>, ts_int: T) -> Option<Ft6X06<I2c<I2C1>, T>>
where
    T: embedded_hal_02::digital::v2::InputPin,
{
    match Ft6X06::new(i2c, FT6X06_I2C_ADDR, ts_int) {
        Ok(touch) => {
            #[cfg(feature = "defmt")]
            defmt::info!("FT6X06 touch controller initialized");
            Some(touch)
        }
        Err(_) => {
            #[cfg(feature = "defmt")]
            defmt::warn!("FT6X06 touch controller not detected");
            None
        }
    }
}

/// Initialize the FT6X06 touch controller with calibration
///
/// The interrupt pin `ts_int` should be created as:
/// ```no_run
/// let ts_int = gpioc.pc1.into_pull_down_input();
/// ```
///
/// This function runs the touchscreen calibration routine, which requires
/// user interaction (tapping crosshairs on screen).
///
/// Returns `Some(Ft6X06)` if the touch controller is detected and initialized,
/// `None` if it fails to initialize.
pub fn init_touchscreen<T, D>(
    i2c: &mut I2c<I2C1>,
    ts_int: T,
    delay: &mut D,
) -> Option<Ft6X06<I2c<I2C1>, T>>
where
    T: embedded_hal_02::digital::v2::InputPin,
    D: embedded_hal_02::blocking::delay::DelayMs<u32>,
{
    match Ft6X06::new(i2c, FT6X06_I2C_ADDR, ts_int) {
        Ok(mut touch) => {
            #[cfg(feature = "defmt")]
            defmt::info!("FT6X06 touch controller initialized, starting calibration");

            // Run touchscreen calibration (may require user interaction)
            match touch.ts_calibration(i2c, delay) {
                Ok(_) => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("Touchscreen calibration completed");
                }
                Err(e) => {
                    #[cfg(feature = "defmt")]
                    defmt::warn!("Touchscreen calibration error: {}", e);
                    #[cfg(not(feature = "defmt"))]
                    let _ = e;
                }
            }
            Some(touch)
        }
        Err(_) => {
            #[cfg(feature = "defmt")]
            defmt::warn!("FT6X06 touch controller not detected");
            None
        }
    }
}
