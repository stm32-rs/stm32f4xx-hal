# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Simple docs in gpio. `into_<output>_in_state`, `with_<output>_in_state`
- Weaker constrains for examples.
- Deprecate `stm32` alias.
- Temporary change pin mode
- More badges in README
- `RccBus` & `GetBusFreq` traits. `AHBx`, `APBx` structures.
- Filler `NoPin` type 
- Add inherent impl of `PwmPin` methods on `PwmChannel`s.
- `Serial:tx` and `Serial::rx` that take only 1 pin
- Instead of `Alternate<AF1>` you can just use `Alternate<1>`.
- `PinState` and `get/set_state`.
- Inherent methods for infallible digital operations.
- Generic `into_alternate` and `into_alternate_open_drain`. Non-generic ones are deprecated
- `PinExt` trait. Make `ExtiPin` implementation generic
- `Enable`, `LPEnable` and `Reset` traits in `rcc`. Implemented for all used peripherals
- Features corresponding to peripherals
- Fixed typo in string representation in DMAError type
- Improved documentation of rng and prelude
- Added an example of integration with RTIC.
- Added internal pullup configuaration for the AlternateOD pin type
- Added USART support for sending and receiving 9-bit words [#299]
- Added support for I2S communication using SPI peripherals, and two examples [#265]
- Added support for some LCD controllers using the Flexible Static Memory
  Controller / Flexible Memory Controller [#297]
- Added `DelayMs` / `DelayUs` impls for TIM2/TIM5 [#309]
- Added an example for using the new FSMC interface with the provided
  `display-interface` driver and the `st7789` driver on a F413Discovery board [#302]
- Derive `Eq`, `PartialEq`, `Copy` and `Clone` for error types
- Added open-drain pin mode support for PWM output [#313]
- Added missing error flags for dma streams [#318]
- Added PWM input capability to all compatable timers [#271]
- [breaking-change] `gpio::Edge::{RISING, FALLING, RISING_FALLING}` are renamed to `Rising`, `Falling`, `RisingFalling`, respectively.
- Bidi mode support for SPI [#349]

[#265]: https://github.com/stm32-rs/stm32f4xx-hal/pull/265
[#271] https://github.com/stm32-rs/stm32f4xx-hal/pull/271
[#297]: https://github.com/stm32-rs/stm32f4xx-hal/pull/297
[#302]: https://github.com/stm32-rs/stm32f4xx-hal/pull/302
[#309]: https://github.com/stm32-rs/stm32f4xx-hal/pull/309
[#313]: https://github.com/stm32-rs/stm32f4xx-hal/pull/313
[#318]: https://github.com/stm32-rs/stm32f4xx-hal/pull/318
[#349]: https://github.com/stm32-rs/stm32f4xx-hal/pull/349

### Changed

- [breaking-change] `into_<output>` fns set pin in `Low` state by default
- Use manual impls for blocking spi instead of `Default`.
- Split `Stream` trait on `Stream` and `StreamISR`.
  Use const generics for `Stream` and `Channel`.
- [breaking-change] `Timer::new` now just initializes peripheral.
  Use `.start_count_down` to start count, `pwm` or `delay` on `Timer` struct.
- Add `Spi::new`, `I2s::new, `spi::Instance` and deprecate `Spi:spix`,
  deprecate `Serial::usartx`, remove deprecated `I2c::i2cx`
- Deprecate `free` in favour of `release`
- Clean features in `serial`, `spi`, `i2c`, `timer`
- Internal implementation of GPIO Pin API changed to use Const Generics
- Update the sdio driver to match the changes in the PAC
- Update README.md with current information
- Updated serial driver to use 32-bit reads and writes when accessing the USART data register [#299]
- Add possibility to use DMA with the ADC abstraction, add example for ADC with DMA [#258]
- Remove unsafe code from ADC DMA example
- [breaking-change] DMA: Memory to peripheral transfers now only require `StaticReadBuffer` [#257].
- Rename erased `Pin` to `EPin`, partially erased `PXx` to `PEPin`, `PX` to `Pin`.

[#299]: https://github.com/stm32-rs/stm32f4xx-hal/pull/299
[#258]: https://github.com/stm32-rs/stm32f4xx-hal/pull/258
[#257]: https://github.com/stm32-rs/stm32f4xx-hal/pull/257
### Fixed

- Corrected pin definitions for the Flexible Static Memory Controller / Flexible Memory Controller
  LCD interface [#312]
- Eliminated `channel_impl` macro warnings caused by unused ident

[#312]: https://github.com/stm32-rs/stm32f4xx-hal/pull/312

## [v0.9.0] - 2021-04-04

### Changed

- [breaking-change] Bump `rand_core` dependency to 0.6.
- [breaking-change] Bump main crate dependencies `cortex-m`, `bare-metal` and `nb`
- [breaking-change] Bump `stm32f4` version to 0.13.
- Removing error on I2C bus errors due to errata workaround.
- [breaking-change] Updated synopsys-usb-otg dependency to v0.2.0.
- Cleanups to the Sdio driver, some hw independent functionality moved to the new sdio-host library.
- [breaking-change] Sdio is disabled by default, enable with the `sdio` feature flag.
- Move SDIO card power handling to its own function.
- [breaking-change] Add a 2 ms delay after changing SDIO card power setting.
- [breaking-change] Changed sdio::{read, write}\_block buf argument to &[u8; 512].
- Voltage regulator overdrive is enabled where supported and required for selected HCLK.
- I2C driver updated to detect and clear all error condition flags.
- Allow for skipping an ongoing DMA transfer if not using double buffering.
- Change DMA traits to `embedded-dma`.
- Use bitbanding during clock enabling and peripheral reset to avoid data races.
- Add missing `Write` implementation for `Serial` and implemented better error handling.
- [breaking-change] ADC2 and ADC3 no longer allow access to VREF, VBAT, or the internal
  temperature measurement (ADC2 and ADC3 do not have an internal connection for these channels)
- Improved Serial baudrate calculation to be correct for higher baudrates or lower PCLKs
- Added `SysCfg` wrapper to enforce clock enable for `SYSCFG`
- [breaking-change] gpio::ExtiPin now uses `SysCfg` wrapper instead of `SYSCFG`
- Change `WriteBuffer + 'static` to `StaticWriteBuffer`in the DMA module.
- Fixed a race condition where SPI writes could get stuck in an error state forever (PR #269).
- Implement generics on the serial module.
- Implement generics on the i2c module, not including fast i2c.
- Updated SDIO_D0 pin to PB7 for stm32f411 [#277]
- Address ST erratum 2.1.13 (DM00037591) [#278]
- Implement generics on the qei module.
- Bump ssd1306 dev-dependency and cleanup examples

### Added

- Reexport PAC as `pac` for consistency with other crates, consider `stm32` virtually deprecated
- Added external interrupt (EXTI) support for output pins
- Added `check_interrupt` method for GPIO pins
- Basic support for DAC
- Add initial DMA support
- Allow specification of ADC reference voltage in ADC configuraton structure
- Added support for hardware-based CRC32 functionality
- Add `MonoTimer` and `Instant` structs for basic time measurement.
- Added support for I2S and SAI clocks
- Added support for canbus with the bxcan crate.[#273] The version range is `<=0.4, <0.6`. (Currently
  the latest version is `0.5.0`) [#286]
- Added a `freeze_unchecked` method [#231]
- Added support for the Real Time Clock (RTC)
- Added option to bypass the HSE oscillator and use a clock input [#263]
- Added support for CAN on additional models: STM32F412, STM32F413, STM32F415,
  STM32F417, STM32F423, STM32F427, STM32F429, STM32F437, STM32F439, STM32F469,
  and STM32F479 [#262]
- Added `gpio::gpiox::Pxi::downgrade2` method [#272]
- Added flash driver

[#231]: https://github.com/stm32-rs/stm32f4xx-hal/pull/231
[#262]: https://github.com/stm32-rs/stm32f4xx-hal/pull/262
[#263]: https://github.com/stm32-rs/stm32f4xx-hal/pull/263
[#278]: https://github.com/stm32-rs/stm32f4xx-hal/issues/278
[#273]: https://github.com/stm32-rs/stm32f4xx-hal/pull/273
[#286]: https://github.com/stm32-rs/stm32f4xx-hal/pull/286
[#272]: https://github.com/stm32-rs/stm32f4xx-hal/issues/272

### Fixed

- Stability fixes related to SD card write
- Fixed issue where timer generated a spurious interrupt after start
- Allow implementations for DMASet from outside the crate [#237]
- DMA: Make it possible to create the wrapper types for the timers [#237]
- DMA: Fix some `compiler_fences` [#237]
- DMA: Fix docs [#237]
- RCC for F412, F413, F423, F446: Add missing configuration of PLLI2SCFGR.PLLI2SN [#261]
- RCC for F411: Add missing configuration of PLLI2SCFGR.PLLI2SM [#264]
- CRC: Fixed CRC clock not being enabled [#283]

[#237]: https://github.com/stm32-rs/stm32f4xx-hal/pull/237
[#261]: https://github.com/stm32-rs/stm32f4xx-hal/pull/261
[#264]: https://github.com/stm32-rs/stm32f4xx-hal/pull/264
[#283]: https://github.com/stm32-rs/stm32f4xx-hal/pull/283

## [v0.8.3] - 2020-06-12

### Fixed

- Make sure that I2C writes are concluded with a STOP condition

## [v0.8.2] - 2020-05-29

### Added

- Added sdio driver
- Allow specifying the desired SDIO bus speed during initialization

## [v0.8.1] - 2020-05-10

### Added

- Implement `timer::Cancel` trait for `Timer<TIM>`.
- Added DWT cycle counter based delay and stopwatch, including an example.

## [v0.8.0] - 2020-04-30

### Changed

- [breaking-change] Updated stm32f4 dependency to v0.11.
- Wait 16 cycles after setting prescalers for some clock domains to follow manual.
- Fixed `TIM9` `pclk` and `ppre`.

### Added

- Implement `timer::Cancel` trait for `Timer<SYST>`.
- Added PWM support and example.

## [v0.7.0] - 2020-03-07

### Changed

- Added more type states for open drain AF modes so we can prevent (potential fatal) I2C misuse
- [breaking-change] Updated stm32f4 dependency to v0.10.0.

### Added

- Added examples in the examples folder.
- Added USB driver.
- PLL48Clk configuration.
- Added bit-banding implementation.
- Added support for RNG peripheral and rand_core, and an example that uses it.

## [v0.6.0] - 2019-10-19

### Changed

- [breaking-change] Updated embedded-hal dependency to v0.2.3 and switched
  to digtial::v2 traits.

### Added

- Implemented `Qei` trait
- Implemented `clear_interrupt()` method for TIM timers

## [v0.5.0] - 2019-04-27

### Changed

- [breaking-change] Updated stm32f4 dependency to v0.7.0.
- Replace macro by generic impl over spi1::RegisterBlock in SPI.

### Fixed

- Properly terminate I2C read with a NACK then a STOP.

## [v0.4.0] - 2019-04-12

### Added

- API to enable and disable SPI interrupts

- Hal ADC supporting one-shot and sequence conversion of regular channels.

- Implement IndependentWatchdog for the IWDG peripheral

- Implement reading the device electronic signature from flash

### Changed

- [breaking-change] Updated cortex-m dependency to v0.6.0.

## [v0.3.0] - 2019-01-14

### Added

- Support ToggleableOutputPin trait in GPIO to allow using toggle().

- Possibility to configure GPIO pins into analog input mode.

- Possibility to configure GPIO pins to generate external interrupts.

- Support NoTx and NoRx in Serial to allow setting up a Rx only or Tx only port.

- Support for stm32f405, stm32f410, stm32f413, stm32f415, stm32f417, stm32f423,
  stm32f427, stm32f437, stm32f439, stm32f446, stm32f469 and stm32f479.

- Support for I2C2 and I2C3 in addition to I2C1.

- Read and Write implementations for Serial.

### Changed

- More versatile RCC clocks configuration.

- Allow using any pair of Pins for I2C rather than only a few hardcoded ones.

- Allow using any pair of Pins for Serial rather than only a few hardcoded ones.

- [breaking-change] Updated stm32f4 dependency to v0.6.0.

### Fixed

- Serial baud rate divisor fractional overflow.

## [v0.2.8] - 2018-12-16

### Fixed

- Documentation generation.

## [v0.2.7] - 2018-12-16

### Changed

- Switched to Rust 2018 edition.

## [v0.2.6] - 2018-12-06

### Added

- Support for GPIOH PH0 and PH1 on stm32f401.

- Support for serial port Idle interrupt.

- Basic `memory.x` linker script and default linking options.

### Changed

- Changed repository URL.

### Fixed

- I2C clock setting.

- GPIO `set_open_drain` is now setting the `otyper` register correctly.

## [v0.2.5] - 2018-11-17

### Added

- Support for stm32f411.

- Spi trait implementation.

### Changed

- Updated stm32f4 dependency to v0.4.0.

- Simplified and improved RCC parameters selection.

## [v0.2.4] - 2018-11-05

### Added

- Support for Serial interrupt enable and disable.

### Changed

- Made the `rt` feature of stm32f4 optionnal.

### Fixed

- Avoid overwritting the cache bits in `flash.acr`.

## [v0.2.3] - 2018-11-04

### Added

- Support for stm32f412.

- Gpio InputPin implementation for output pins to allow reading current
  value of open drain output pins.

- Timer trait implementation.

### Changed

- No default features selected for the stm32f4 crate. User must specify its
  specific device.

## [v0.2.2] - 2018-10-27

### Added

- Gpio `set_speed`.

## [v0.2.1] - 2018-10-13

### Fixed

- Removed unnecessary feature gates.

## [v0.2.0] - 2018-10-11

### Added

- Support for stm32f401.

- [breaking-change]. Support for setting serial port word length, parity and
  stop bits.

### Changed

- Support longuer Delay without asserting.

## v0.1.0 - 2018-10-02

### Added

- Support for stm32f407 and stm32f429.

[Unreleased]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.9.0...HEAD
[v0.9.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.8.3...v0.9.0
[v0.8.3]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.8.2...v0.8.3
[v0.8.2]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.8.1...v0.8.2
[v0.8.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.8.0...v0.8.1
[v0.8.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.8...v0.3.0
[v0.2.8]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.7...v0.2.8
[v0.2.7]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.1.0...v0.2.0
