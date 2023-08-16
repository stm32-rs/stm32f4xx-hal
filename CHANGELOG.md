# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- complete and rework Dma Stream API [#666]
- add `.set_count()` for QEI, add `.write_count()` for TIM [#677]
- add "Fast start" section in README [#678]
 - Optimized version of blocking SPI write [#523]
 - SPI bidi takes 2 pins [#526]

[#523]: https://github.com/stm32-rs/stm32f4xx-hal/pull/523
[#526]: https://github.com/stm32-rs/stm32f4xx-hal/pull/526
[#666]: https://github.com/stm32-rs/stm32f4xx-hal/pull/666
[#677]: https://github.com/stm32-rs/stm32f4xx-hal/pull/677
[#678]: https://github.com/stm32-rs/stm32f4xx-hal/pull/678

## [v0.17.1] - 2023-07-24

### Changed

- implement `embedded_hal::blocking::i2c::Transactional` for `I2c` [#671]

### Fixed

- reset timer interrupt in `Counter::start` [#670]

[#670]: https://github.com/stm32-rs/stm32f4xx-hal/pull/670
[#671]: https://github.com/stm32-rs/stm32f4xx-hal/pull/671

## [v0.17.0] - 2023-07-11

### Changed

 - `rcc::Enable`, `rcc::LPEnable` traits, timclk in `Clocks` instead of prescalers [#665]
 - move gpio, dma impls, adc pins in subdir, remove unused `From` impls [#658] [#664]
 - Bump `embedded-hal` to `1.0.0-alpha.11`. See [their changelog][embedded-hal-1.0.0-alpha.11] for further details. Note that this included breaking changes to the previous alpha APIs. [#663],[#668]
 - Fix race condition in sending start condition in I2C. [#662]

[#658]: https://github.com/stm32-rs/stm32f4xx-hal/pull/658
[#662]: https://github.com/stm32-rs/stm32f4xx-hal/pull/662
[#663]: https://github.com/stm32-rs/stm32f4xx-hal/pull/663
[#664]: https://github.com/stm32-rs/stm32f4xx-hal/pull/664
[#665]: https://github.com/stm32-rs/stm32f4xx-hal/pull/665
[#668]: https://github.com/stm32-rs/stm32f4xx-hal/pull/668
[embedded-hal-1.0.0-alpha.11]: https://github.com/rust-embedded/embedded-hal/blob/v1.0.0-alpha.11/embedded-hal/CHANGELOG.md

## [v0.16.2] - 2023-06-27

### Changed

 - enable `defmt` feature for VSCode, `set_alarm` takes `Into<AlarmDay>` [#660]
 - Optimize watchdog setup calculation [#657]

### Fixed

 - Compilation with `defmt` feature enabled

[#657]: https://github.com/stm32-rs/stm32f4xx-hal/pull/657
[#660]: https://github.com/stm32-rs/stm32f4xx-hal/pull/660

## [v0.16.1] - 2023-06-24

 - bors bot replaced with GH merge queue [#652]
 - Integrate new version of stm32_i2s (v0.5) to enable full-duplex operation [#637]
 - Add a rtic example to show how to do full-duplex i2s [#637]

### Changed

 - Set `Speed::VeryHigh` default for FMC, SDIO & OTG_HS_ULPI pins, clean into_alternate in examples [#632]
 - Join `Serial`, `Rx`, `Tx` for `USART` and `UART` again. Make inner traits with different implementation for USART and UART. [#636]

### Added

 - `into_mode` for `ErasedPin` and `PartiallyErasedPin` [#647]
 - Extended 64-bit monotonic timer [#640]
 - Basic blocking QSPI interface [#645]
 - Rtc: add subsecond reading, add interrupts [#446]

### Fixed

 - Fix #604 pwm output [#655]
 - map `$SpiSlave` into `SpiSlave` struct in `spi!` macro [#635]

[#446]: https://github.com/stm32-rs/stm32f4xx-hal/pull/446
[#632]: https://github.com/stm32-rs/stm32f4xx-hal/pull/632
[#635]: https://github.com/stm32-rs/stm32f4xx-hal/pull/635
[#636]: https://github.com/stm32-rs/stm32f4xx-hal/pull/636
[#637]: https://github.com/stm32-rs/stm32f4xx-hal/pull/637
[#640]: https://github.com/stm32-rs/stm32f4xx-hal/pull/640
[#645]: https://github.com/stm32-rs/stm32f4xx-hal/pull/645
[#647]: https://github.com/stm32-rs/stm32f4xx-hal/pull/647
[#652]: https://github.com/stm32-rs/stm32f4xx-hal/pull/652
[#655]: https://github.com/stm32-rs/stm32f4xx-hal/pull/655

## [v0.16.0] - 2023-05-07

### Changed

 - Use `enum`s for alternate peripheral pins (generic over otype) [#594] [#596] [#600] [#610] [#617]
 - Add `ReadPin`, `PinSpeed` & `PinPull` traits [#623]
 - Split USART and UART implementations [#608]
 - Split SPI master and slave implementations [#609]
 - Simplify `gpio::Outport` [#611]
 - Add autoimplementations of `DMASet` [#614]
 - `ws2812::prerendered` in example [#615]
 - Integrate new version of stm32_i2s (v0.4) [#626]

### Added

 - Improve SPI::new* docs [#587]
 - Add advanced timer dead time insertion example [#585]
 - Added missing U(S)ART DMA traits for HAL serial types [#593]
 - I2c dma can now use single DMA channel for TX or RX only [#598]
 - Improve SPI::new* docs [#587]
 - Implement `serial::RxISR` for `dma::Transfer<..., PERIPHERAL, ...>` where `PERIPHERAL: serial::RxISR`, add `rtic-serial-dma-rx-idle` example [#588]
 - Add `lapce` editor settings [#601]
 - rcc `enable_unchecked`, timer features [#618]

### Fixed

 - Cleanups [#595]
 - Fix comlementary for independent channels [#599] [#603]
 - Fix mstr bit for SPI Master/Slave [#625]

[#585]: https://github.com/stm32-rs/stm32f4xx-hal/pull/585
[#587]: https://github.com/stm32-rs/stm32f4xx-hal/pull/587
[#588]: https://github.com/stm32-rs/stm32f4xx-hal/pull/588
[#593]: https://github.com/stm32-rs/stm32f4xx-hal/pull/593
[#594]: https://github.com/stm32-rs/stm32f4xx-hal/pull/594
[#595]: https://github.com/stm32-rs/stm32f4xx-hal/pull/595
[#598]: https://github.com/stm32-rs/stm32f4xx-hal/pull/598
[#599]: https://github.com/stm32-rs/stm32f4xx-hal/pull/599
[#601]: https://github.com/stm32-rs/stm32f4xx-hal/pull/601
[#603]: https://github.com/stm32-rs/stm32f4xx-hal/pull/603
[#610]: https://github.com/stm32-rs/stm32f4xx-hal/pull/610
[#608]: https://github.com/stm32-rs/stm32f4xx-hal/pull/608
[#609]: https://github.com/stm32-rs/stm32f4xx-hal/pull/609
[#611]: https://github.com/stm32-rs/stm32f4xx-hal/pull/611
[#614]: https://github.com/stm32-rs/stm32f4xx-hal/pull/614
[#615]: https://github.com/stm32-rs/stm32f4xx-hal/pull/615
[#617]: https://github.com/stm32-rs/stm32f4xx-hal/pull/617
[#618]: https://github.com/stm32-rs/stm32f4xx-hal/pull/618
[#623]: https://github.com/stm32-rs/stm32f4xx-hal/pull/623
[#625]: https://github.com/stm32-rs/stm32f4xx-hal/pull/625
[#626]: https://github.com/stm32-rs/stm32f4xx-hal/pull/626

## [v0.15.0] - 2023-03-13

### Changed

 - Bump `nb` to 1.1
 - Bump `synopsys-usb-otg` to 0.3.2 (bug fix) [#575]
 - Update readme, clippy fixes
 - Added possibility to pass complementary pins to `Pwm` and change pwm channel polarity [#571],
   set dead time and idle state for advanced timers [#578] [#581]

### Added

 - Docs in `rtic-adc-dma` example [#532]
 - `OutPortX` (X = 2..8) and `OutPortArray` structures which can handle several pins at once [#426]
 - `restore` for `ErasedPin` and `PartiallyErasedPin` [#563]
 - Added a public method to set SSI bit in SPI. [#543]

### Fixed

 - `spi-dma` example pins speed
 - Fix alternate function pin definitions for FMPI2C1 [#572]
 - Fix SDIO hardware flow control errata [#577]

[#426]: https://github.com/stm32-rs/stm32f4xx-hal/pull/426
[#532]: https://github.com/stm32-rs/stm32f4xx-hal/pull/532
[#543]: https://github.com/stm32-rs/stm32f4xx-hal/pull/543
[#563]: https://github.com/stm32-rs/stm32f4xx-hal/pull/563
[#571]: https://github.com/stm32-rs/stm32f4xx-hal/pull/571
[#572]: https://github.com/stm32-rs/stm32f4xx-hal/pull/572
[#575]: https://github.com/stm32-rs/stm32f4xx-hal/pull/575
[#577]: https://github.com/stm32-rs/stm32f4xx-hal/pull/577
[#578]: https://github.com/stm32-rs/stm32f4xx-hal/pull/578
[#581]: https://github.com/stm32-rs/stm32f4xx-hal/pull/581
[#594]: https://github.com/stm32-rs/stm32f4xx-hal/pull/594
[#595]: https://github.com/stm32-rs/stm32f4xx-hal/pull/595
[#596]: https://github.com/stm32-rs/stm32f4xx-hal/pull/596
[#599]: https://github.com/stm32-rs/stm32f4xx-hal/pull/599
[#601]: https://github.com/stm32-rs/stm32f4xx-hal/pull/601
[#603]: https://github.com/stm32-rs/stm32f4xx-hal/pull/603
[#600]: https://github.com/stm32-rs/stm32f4xx-hal/pull/600


## [v0.14.0] - 2022-12-12

### Changed

 - Add missing timer pins [#536]
 - Revised temperature sensor input pins for all MCUs [#529]
 - Support `u16` read/write for SPI
 - Use `bool` for BIDI mode type
 - `PwmHz::get_period`: fix computation of return value, prevent division by zero
 - apply #[inline] attribute to bitbanding functions [#517]
 - update `stm32f4` to 0.15.1 [#481]
 - use `stm32_i2s_v12x` version 0.3, reexport it, and implements requirement for it [#490]
 - i2s module don't reuse marker from spi module and define its own [#490]
 - `i2s-audio-out` example updated and now use pcm5102 dac module instead one from discovery board [#490]
 - extend visibility of gpio/marker to crate since i2s module require it [#490]
 - Bump `synopsys-usb-otg` to `0.3.0` [#508]
 - Bump `embedded-hal` to `1.0.0-alpha.8` [#510]
 - Update `bxcan`, `rtic` and other dependencies [#519]
 - Bump `synopsys-usb-otg` to `0.3.1` [#535]
 - Renamed and updated rtic-button example (was rtic) [#551]
 - Rename adc_dma_rtic to rtic-adc-dma and move it to defmt [#552]
 - Rename spi_slave_dma_rtic to rtic-spi-slave-dma and unbroke it [#552]
 - Rename i2s-rtic-audio-in-out to rtic-i2s-audio-in-out [#552]

### Removed
 - `i2s-audio-out-dma.rs` example, too difficult to fix.

### Fixed
 - use register.modify instead of register.write to start PWM [#501]
 - add missing generic param for Spi::release implementation.
 - build rtic-usb-cdc-echo example [#554]
 - reset timer cnt register when changing pwm period [#555]
 - Trait typo preventing ADC2 being used with DMA2 [#557]

### Added

 - Serial Tx, Rx containing pins [#514] [#515] [#540]
 - example of using ft6x06 touchscreen driver for stm32f412 and stm32f413 [#527]
 - Serial Tx, Rx containing pins [#514] [#515]
 - Implementation of From trait for Pin-to-PartiallyErasedPin [#507]
 - Implementation of From trait for Pin-to-ErasedPin [#507]
 - Implementation of From trait for PartiallyErasedPin-to-ErasedPin [#507]
 - `SysMonoTimerExt` helper trait, `Pwm::(get/set)_duty_time` [#497]
 - example of using i2s in out with rtic and interrupt.
 - example of using USB CDC with interrupts.
 - Added non-blocking I2C based on DMA [#534]
 - Added Transactional I2C API [#542]
 - Added rtic-usart-shell example [#551]
 - Added rtic-usart-shell-ssd1306 example [#551]
 - Added rtic-usb-cdc-echo example [#553]
 - Add possibility to clear a Serial `Rx` idle interrupt from a DMA `Transfer` [#556]

- Serial Tx, Rx containing pins [#514] [#515] [#540]
- Implementation of From trait for Pin-to-PartiallyErasedPin [#507]
- Implementation of From trait for Pin-to-ErasedPin [#507]
- Implementation of From trait for PartiallyErasedPin-to-ErasedPin [#507]
- `SysMonoTimerExt` helper trait, `Pwm::(get/set)_duty_time` [#497]
- example of using i2s in out with rtic and interrupt.
- example of using USB CDC with interrupts.
- Added non-blocking I2C based on DMA [#534]
- Added Transactional I2C API [#542]
- Added wait method for DMA Transfer.

[#481]: https://github.com/stm32-rs/stm32f4xx-hal/pull/481
[#489]: https://github.com/stm32-rs/stm32f4xx-hal/pull/489
[#490]: https://github.com/stm32-rs/stm32f4xx-hal/pull/490
[#497]: https://github.com/stm32-rs/stm32f4xx-hal/pull/497
[#501]: https://github.com/stm32-rs/stm32f4xx-hal/issues/501
[#507]: https://github.com/stm32-rs/stm32f4xx-hal/pull/507
[#508]: https://github.com/stm32-rs/stm32f4xx-hal/pull/508
[#510]: https://github.com/stm32-rs/stm32f4xx-hal/pull/510
[#514]: https://github.com/stm32-rs/stm32f4xx-hal/pull/514
[#515]: https://github.com/stm32-rs/stm32f4xx-hal/pull/515
[#517]: https://github.com/stm32-rs/stm32f4xx-hal/pull/517
[#519]: https://github.com/stm32-rs/stm32f4xx-hal/pull/519
[#527]: https://github.com/stm32-rs/stm32f4xx-hal/pull/527
[#529]: https://github.com/stm32-rs/stm32f4xx-hal/pull/529
[#536]: https://github.com/stm32-rs/stm32f4xx-hal/pull/536
[#534]: https://github.com/stm32-rs/stm32f4xx-hal/pull/529
[#535]: https://github.com/stm32-rs/stm32f4xx-hal/pull/535
[#540]: https://github.com/stm32-rs/stm32f4xx-hal/pull/540
[#542]: https://github.com/stm32-rs/stm32f4xx-hal/pull/542
[#551]: https://github.com/stm32-rs/stm32f4xx-hal/pull/551
[#552]: https://github.com/stm32-rs/stm32f4xx-hal/pull/552
[#553]: https://github.com/stm32-rs/stm32f4xx-hal/pull/553
[#554]: https://github.com/stm32-rs/stm32f4xx-hal/pull/554
[#555]: https://github.com/stm32-rs/stm32f4xx-hal/pull/555
[#556]: https://github.com/stm32-rs/stm32f4xx-hal/pull/556
[#557]: https://github.com/stm32-rs/stm32f4xx-hal/pull/557

## [v0.13.2] - 2022-05-16

### Fixed

- `Pin::with_mode` [#489]

### Changed

- `Spi` can be operated as `Slave` [#487]

[#487]: https://github.com/stm32-rs/stm32f4xx-hal/pull/487
[#489]: https://github.com/stm32-rs/stm32f4xx-hal/pull/489

## [v0.13.1] - 2022-04-20

### Fixed

- Fix `embedded_hal 1.0-alpha.7` version, public `PinMode`, update deps [#485]
- Remove the defmt feature/dependency name workaround [#479]

[#479]: https://github.com/stm32-rs/stm32f4xx-hal/pull/479
[#485]: https://github.com/stm32-rs/stm32f4xx-hal/pull/485

## [v0.13.0] - 2022-04-04

### Changed

- `DmaSet` & `Transfer` take channel after stream generic, then other [#477]
- Depracate "rt" feature as enabled by-default in `pac` [#476]
- Add `Pin::interrupt()` helper method [#476]
- Add restriction for setting pins in alternate mode (`IntoAF`), add docs [#474]
- Explicit order for PINS, more smart aliases for peripherals [#472]
- Add `AFn` type aliases for `Alternate<n>` [#471]
- CI updates + cache [#468]
- Add missing `embedded-hal 1.0` for `DynamicPin` [#470]
- Remove pull resistor from `Input` mode, use `Pull` enum instead, add universal `into_mode` pin converter [#467]
- Move pin mode at the end of generics, add defaults for modes,
  bump MSRV to 1.59 [#418]
- Move hd44780-driver to dev-dependencies [#465]

### Fixed
- Fixed RCC example [#473]
- Enable the defmt feature on fugit when the defmt feature on the
  crate is enabled [#465]

### Added

- Support eMMC peripherals using SDIO module [#458]
- `defmt::Format` derive on enums behind `defmt` feature [#460]
- SPI transactional impl [#464]

[#418]: https://github.com/stm32-rs/stm32f4xx-hal/pull/418
[#458]: https://github.com/stm32-rs/stm32f4xx-hal/pull/458
[#460]: https://github.com/stm32-rs/stm32f4xx-hal/pull/460
[#464]: https://github.com/stm32-rs/stm32f4xx-hal/pull/464
[#465]: https://github.com/stm32-rs/stm32f4xx-hal/pull/465
[#467]: https://github.com/stm32-rs/stm32f4xx-hal/pull/467
[#468]: https://github.com/stm32-rs/stm32f4xx-hal/pull/468
[#470]: https://github.com/stm32-rs/stm32f4xx-hal/pull/470
[#471]: https://github.com/stm32-rs/stm32f4xx-hal/pull/471
[#472]: https://github.com/stm32-rs/stm32f4xx-hal/pull/472
[#473]: https://github.com/stm32-rs/stm32f4xx-hal/pull/473
[#474]: https://github.com/stm32-rs/stm32f4xx-hal/pull/474
[#476]: https://github.com/stm32-rs/stm32f4xx-hal/pull/476
[#477]: https://github.com/stm32-rs/stm32f4xx-hal/pull/477

## [v0.12.0] - 2022-02-23

### Changed

- Make `Monotonic` implementation generic for `u32` timers, fix `PwmExt` trait [#454]
- Rename `Delay<SYST>` to SysDelay, remove old `Delay<TIM2>`
- Extend timers to 32bit on `Delay`
- Move `MonoTimer` from `timer` to dwt mode [#448]
- Unify serial trait impls for embedded-hal 0.2 & 1.0 [#447]
- Add possibility to select Timer master mode
- Add inherent impl of `embedded_hal::Pwm` methods on `Pwm`s [#439]
- Use `embedded-dma` v0.2 [#440]
- Add LSI support for `Rtc` [#438]
- Use `time` for `Rtc` instead of `rtcc`, add `rtc` example [#436]
- Move `i2c` `embedded-hal` trait impls to `I2c` methods [#431]
- Reexport pins in `gpio` module
- Pwm channels now constants [#432]
- Use fugit rate types instead of custom [#430]
- Add channel events, make Event use bitflags (simplify interrupt handling) [#425]
- reexport `digital::v2::PinState` again [#428]
- Timer impls with time based on `fugit::Duration` same as `Hertz` moved to `timer` module,
  added appropriate `Ext` traits implemented on peripherals directly,
  added `Pwm` and `fugit-timer` impls [#423] [#449]

### Fixed

- Incorrect values of autoreload in `CountDown::start`, `Pwm::new`, `Delay` and prescaler in `Delay` [#422]

### Added

- Missing `DelayMs<u8>` / `DelayUs<u8>` impls for fugit::Delay
- Support of embedded-hal 1.0.0-alpha.7 [#443]
- `hd44780` example [#441]
- Aliases for peripheral wrappers [#434]
- `WithPwm` trait implemented for timers with channels (internals) [#425]
- `Pwm` struct with `split` method and implementation of embedded-hal::Pwm (similar to f1xx-hal) [#425]
- VSCode setting file
- Add CAN1 PB8/PB9 and SPI3 MOSI PC1 pin mappings for F446 [#421]
- Add embedded-storage traits for flash [#429]
- `Debug` & `defmt::Format` impls for some structures and enums [#385]

[#385]: https://github.com/stm32-rs/stm32f4xx-hal/pull/385
[#421]: https://github.com/stm32-rs/stm32f4xx-hal/pull/421
[#422]: https://github.com/stm32-rs/stm32f4xx-hal/pull/422
[#423]: https://github.com/stm32-rs/stm32f4xx-hal/pull/423
[#425]: https://github.com/stm32-rs/stm32f4xx-hal/pull/425
[#428]: https://github.com/stm32-rs/stm32f4xx-hal/pull/428
[#429]: https://github.com/stm32-rs/stm32f4xx-hal/pull/429
[#431]: https://github.com/stm32-rs/stm32f4xx-hal/pull/431
[#432]: https://github.com/stm32-rs/stm32f4xx-hal/pull/432
[#434]: https://github.com/stm32-rs/stm32f4xx-hal/pull/434
[#436]: https://github.com/stm32-rs/stm32f4xx-hal/pull/436
[#438]: https://github.com/stm32-rs/stm32f4xx-hal/pull/438
[#439]: https://github.com/stm32-rs/stm32f4xx-hal/pull/439
[#440]: https://github.com/stm32-rs/stm32f4xx-hal/pull/440
[#443]: https://github.com/stm32-rs/stm32f4xx-hal/pull/443
[#441]: https://github.com/stm32-rs/stm32f4xx-hal/pull/441
[#430]: https://github.com/stm32-rs/stm32f4xx-hal/pull/430
[#447]: https://github.com/stm32-rs/stm32f4xx-hal/pull/447
[#448]: https://github.com/stm32-rs/stm32f4xx-hal/pull/448
[#449]: https://github.com/stm32-rs/stm32f4xx-hal/pull/449
[#454]: https://github.com/stm32-rs/stm32f4xx-hal/pull/454

### Changed

- Fix Width of TIM5 F410 [#409]
- up `cortex-m` to 0.7.4, use `cycle_count` instead of `get_cycle_count` on DWT [#415]

[#409]: https://github.com/stm32-rs/stm32f4xx-hal/pull/409
[#415]: https://github.com/stm32-rs/stm32f4xx-hal/pull/415

## [v0.11.1] - 2021-12-30

- Remove optional `atat` dependency [#408]

[#408]: https://github.com/stm32-rs/stm32f4xx-hal/pull/408

## [v0.11.0] - 2021-12-29

### Added

- Added `Counter` with `CountDown<Time=fugit::TimerDuration>` and `atat::Clock` implementations [#381]
- `Into<serial::Config>` for `Bps` [#387]
- Added the missing DMA implementations for USART3 [#373]
- `DynamicPin` with dynamically changed mode, remove `AF` constants [#372]
- `count_down` constructor for `Timer` -> `CountDownTimer` without start [#382]
- Implementation of RTIC Monotonic for TIM2 & TIM5 under `rtic` feature [#380] [#390]
- `IoPin` for `Output<OpenDrain>> <-> Input<Floating>>` [#374]
- `IoPin` for `Output<PushPull>> <-> Input<PullUp>> and Input<PullDown>>` [#389]
- Add `internal_pull_down` to `Pin<Output<OpenDrain>>` and `Pin<Alternate<PushPull>>` for symmetry
  with `internal_pull_up` [#399]
- Added `peripheral` for DMA read access to peripheral [#396]
- Added ADC2+ADC3 implementations to DMA Transfer [#396]
- Added `reference_voltage` to Adc [#396]

[#372]: https://github.com/stm32-rs/stm32f4xx-hal/pull/372
[#373]: https://github.com/stm32-rs/stm32f4xx-hal/pull/373
[#396]: https://github.com/stm32-rs/stm32f4xx-hal/pull/396
[#374]: https://github.com/stm32-rs/stm32f4xx-hal/pull/374
[#380]: https://github.com/stm32-rs/stm32f4xx-hal/pull/380
[#381]: https://github.com/stm32-rs/stm32f4xx-hal/pull/381
[#382]: https://github.com/stm32-rs/stm32f4xx-hal/pull/382
[#389]: https://github.com/stm32-rs/stm32f4xx-hal/pull/389
[#390]: https://github.com/stm32-rs/stm32f4xx-hal/pull/390
[#399]: https://github.com/stm32-rs/stm32f4xx-hal/pull/399

### Changed

- Correct default mode for debugger pins [#405]
- Move `embedded-hal` implementations to subdirs [#404]
- Qei macro cleanups  [#403]
- Update RTIC to 1.0 [#401]
- Finish SDIO data transmission before querying card status in `write_block` [#395]
- SDIO: Rewrite loop conditions to silence clippy
- Unify alternate pin constraints [#393]
- Prevent overflow when optimizing SAI PLL [#419]
- [breaking-change] Use `&Clocks` instead of `Clocks` [#387]
- Split and rename `GetBusFreq` -> `BusClock`, `BusTimerClock` [#386]
- [breaking-change] Remove `Can::new_unchecked`. Add `Can::tx` and `Can::rx` [#384]
- [breaking-change] Make `Alternate` generic over `Otype` instead of separate `Alternate` and `AlternateOD` [#383]
- [breaking-change] Bump `stm32f4` to 0.14. Update RTIC based examples to use `rtic` 0.6 [#367]
- [breaking-change] Bump `bxcan` to 0.6 [#371]
- fix #362: ADC voltage conversion might be incorrect [#397]
- [breaking-change] Change `Pin<Output<OpenDrain>>::internal_pull_up` signature from `(&mut self, _: bool) -> ()`
  to `(self, _: bool) -> Self`. [#399]

[#367]: https://github.com/stm32-rs/stm32f4xx-hal/pull/367
[#371]: https://github.com/stm32-rs/stm32f4xx-hal/pull/371
[#383]: https://github.com/stm32-rs/stm32f4xx-hal/pull/383
[#384]: https://github.com/stm32-rs/stm32f4xx-hal/pull/384
[#386]: https://github.com/stm32-rs/stm32f4xx-hal/pull/386
[#387]: https://github.com/stm32-rs/stm32f4xx-hal/pull/387
[#393]: https://github.com/stm32-rs/stm32f4xx-hal/pull/393
[#395]: https://github.com/stm32-rs/stm32f4xx-hal/pull/395
[#397]: https://github.com/stm32-rs/stm32f4xx-hal/pull/397
[#401]: https://github.com/stm32-rs/stm32f4xx-hal/pull/401
[#403]: https://github.com/stm32-rs/stm32f4xx-hal/pull/403
[#404]: https://github.com/stm32-rs/stm32f4xx-hal/pull/404
[#405]: https://github.com/stm32-rs/stm32f4xx-hal/pull/405
[#419]: https://github.com/stm32-rs/stm32f4xx-hal/pull/419

## [v0.10.1] - 2021-10-26

- Fix `cortex-m-rt` dependency

## [v0.10.0] - 2021-09-22

### Added

- PWM channels for timers 9-14 [#364]
- `ws2812_spi` example [#363]
- `AsRef/AsMut<Rx/Tx` for Serial [#355]
- `spi::Transactional` [#356]
- `IoPin` for `Output<OpenDrain>` and `Output<PushPull>> <-> Input<Floating>>` [#356]
- I2c `Mode` with duty_cycle [#353]
- Simple docs in gpio. `into_<output>_in_state`, `with_<output>_in_state` [#351]
- Weaker constrains for examples [#351]
- Deprecate `stm32` alias. [#351]
- Temporary change pin mode [#346]
- More badges in README [#345]
- `RccBus` & `GetBusFreq` traits. `AHBx`, `APBx` structures [#342]
- Filler `NoPin` type [#340]
- Add inherent impl of `PwmPin` methods on `PwmChannel`s.
- `Serial:tx` and `Serial::rx` that take only 1 pin [#332]
- Instead of `Alternate<AF1>` you can just use `Alternate<1>` [#328]
- `PinState` and `get/set_state` [#325]
- Inherent methods for infallible digital operations [#325]
- Generic `into_alternate` and `into_alternate_open_drain`. Non-generic ones are deprecated [#266]
- `PinExt` trait. Make `ExtiPin` implementation generic [#323]
- `Enable`, `LPEnable` and `Reset` traits in `rcc`. Implemented for all used peripherals [#311]
- Features corresponding to peripherals [#311]
- Improved documentation of rng and prelude [#303]
- Added an example of integration with RTIC [#295]
- Added internal pullup configuration for the AlternateOD pin type [#298]
- Added USART support for sending and receiving 9-bit words [#299]
- Added support for I2S communication using SPI peripherals, and two examples [#265]
- Added support for some LCD controllers using the Flexible Static Memory
  Controller / Flexible Memory Controller [#297]
- Added `DelayMs` / `DelayUs` impls for TIM2/TIM5 [#309]
- Added an example for using the new FSMC interface with the provided
  `display-interface` driver and the `st7789` driver on a F413Discovery board [#302]
- Derive `Eq`, `PartialEq`, `Copy` and `Clone` for error types [#306]
- Added open-drain pin mode support for PWM output [#313]
- Added missing error flags for dma streams [#318]
- Added PWM input capability to all compatible timers [#271]
- Bidi mode support for SPI [#349]
- Added `listen` and `unlisten` for RX- and TX-only USART [#357]
- Added function for clearing the idle line interrupt in USART [#357]
- Added flash driver [#347]
- Added `gpio::gpiox::Pxi::downgrade2` method [#323]
- Added DMA support for SPI [#319]

[#265]: https://github.com/stm32-rs/stm32f4xx-hal/pull/265
[#266]: https://github.com/stm32-rs/stm32f4xx-hal/pull/266
[#271]: https://github.com/stm32-rs/stm32f4xx-hal/pull/271
[#295]: https://github.com/stm32-rs/stm32f4xx-hal/pull/295
[#297]: https://github.com/stm32-rs/stm32f4xx-hal/pull/297
[#298]: https://github.com/stm32-rs/stm32f4xx-hal/pull/298
[#302]: https://github.com/stm32-rs/stm32f4xx-hal/pull/302
[#303]: https://github.com/stm32-rs/stm32f4xx-hal/pull/303
[#306]: https://github.com/stm32-rs/stm32f4xx-hal/pull/306
[#309]: https://github.com/stm32-rs/stm32f4xx-hal/pull/309
[#311]: https://github.com/stm32-rs/stm32f4xx-hal/pull/311
[#313]: https://github.com/stm32-rs/stm32f4xx-hal/pull/313
[#325]: https://github.com/stm32-rs/stm32f4xx-hal/pull/325
[#328]: https://github.com/stm32-rs/stm32f4xx-hal/pull/328
[#318]: https://github.com/stm32-rs/stm32f4xx-hal/pull/318
[#319]: https://github.com/stm32-rs/stm32f4xx-hal/pull/319
[#322]: https://github.com/stm32-rs/stm32f4xx-hal/pull/322
[#323]: https://github.com/stm32-rs/stm32f4xx-hal/pull/323
[#332]: https://github.com/stm32-rs/stm32f4xx-hal/pull/332
[#340]: https://github.com/stm32-rs/stm32f4xx-hal/pull/340
[#342]: https://github.com/stm32-rs/stm32f4xx-hal/pull/342
[#345]: https://github.com/stm32-rs/stm32f4xx-hal/pull/345
[#346]: https://github.com/stm32-rs/stm32f4xx-hal/pull/346
[#347]: https://github.com/stm32-rs/stm32f4xx-hal/pull/347
[#349]: https://github.com/stm32-rs/stm32f4xx-hal/pull/349
[#351]: https://github.com/stm32-rs/stm32f4xx-hal/pull/351
[#353]: https://github.com/stm32-rs/stm32f4xx-hal/pull/353
[#355]: https://github.com/stm32-rs/stm32f4xx-hal/pull/355
[#356]: https://github.com/stm32-rs/stm32f4xx-hal/pull/356
[#357]: https://github.com/stm32-rs/stm32f4xx-hal/pull/357
[#363]: https://github.com/stm32-rs/stm32f4xx-hal/pull/363

### Changed

- Bumped a few dependencies and fixed example fallout [#365]
- Uncommented two TIM5 channels for STM32F410 [#364]
- Update examples with `embedded-graphics`,
  remove deprecated `I2s::i2sx` [#358]
- `into_alternate()` may be omitted now for `Serial`, `Spi`, `I2s`, `I2c` [#359]
- [breaking-change] 115_200 bps for Serial by default [#355]
- Move `Tx`, `Rx` structures into `Serial` [#355]
- Update `embedded-hal` dependendency [#356]
- [breaking-change] `into_<output>` fns set pin in `Low` state by default [#351]
- Use manual impls for blocking spi instead of `Default` [#356]
- Split `Stream` trait on `Stream` and `StreamISR`,
  use const generics for `Stream` and `Channel` [#341]
- [breaking-change] `Timer::new` now just initializes peripheral,
  use `.start_count_down` to start count, `pwm` or `delay` on `Timer` struct [#337]
- Add `Spi::new`, `I2s::new, `spi::Instance` and deprecate `Spi:spix`,
  deprecate `Serial::usartx`, remove deprecated `I2c::i2cx` [#330]
- Deprecate `free` in favour of `release` [#333]
- Clean features in `serial`, `spi`, `i2c`, `timer` [#331], [#334]
- Internal implementation of GPIO Pin API changed to use Const Generics [#266]
- Update the sdio driver to match the changes in the PAC [#294]
- Update README.md with current information [#293]
- Updated serial driver to use 32-bit reads and writes when accessing the USART data register [#299]
- Add possibility to use DMA with the ADC abstraction, add example for ADC with DMA [#258]
- Remove unsafe code from ADC DMA example [#301]
- [breaking-change] DMA: Memory to peripheral transfers now only require `StaticReadBuffer` [#257].
- Rename erased `Pin` to `EPin`, partially erased `PXx` to `PEPin`, `PX` to `Pin` [#339]
- [breaking-change] `gpio::Edge::{RISING, FALLING, RISING_FALLING}` are renamed to `Rising`, `Falling`, `RisingFalling`, respectively [#343]

[#266]: https://github.com/stm32-rs/stm32f4xx-hal/pull/266
[#293]: https://github.com/stm32-rs/stm32f4xx-hal/pull/293
[#294]: https://github.com/stm32-rs/stm32f4xx-hal/pull/294
[#299]: https://github.com/stm32-rs/stm32f4xx-hal/pull/299
[#258]: https://github.com/stm32-rs/stm32f4xx-hal/pull/258
[#257]: https://github.com/stm32-rs/stm32f4xx-hal/pull/257
[#301]: https://github.com/stm32-rs/stm32f4xx-hal/pull/301
[#330]: https://github.com/stm32-rs/stm32f4xx-hal/pull/330
[#331]: https://github.com/stm32-rs/stm32f4xx-hal/pull/331
[#333]: https://github.com/stm32-rs/stm32f4xx-hal/pull/333
[#334]: https://github.com/stm32-rs/stm32f4xx-hal/pull/334
[#337]: https://github.com/stm32-rs/stm32f4xx-hal/pull/337
[#339]: https://github.com/stm32-rs/stm32f4xx-hal/pull/339
[#341]: https://github.com/stm32-rs/stm32f4xx-hal/pull/341
[#343]: https://github.com/stm32-rs/stm32f4xx-hal/pull/343
[#349]: https://github.com/stm32-rs/stm32f4xx-hal/pull/349
[#351]: https://github.com/stm32-rs/stm32f4xx-hal/pull/351
[#355]: https://github.com/stm32-rs/stm32f4xx-hal/pull/355
[#356]: https://github.com/stm32-rs/stm32f4xx-hal/pull/356
[#358]: https://github.com/stm32-rs/stm32f4xx-hal/pull/358
[#359]: https://github.com/stm32-rs/stm32f4xx-hal/pull/359
[#364]: https://github.com/stm32-rs/stm32f4xx-hal/pull/364
[#365]: https://github.com/stm32-rs/stm32f4xx-hal/pull/365

### Fixed

- Fixed typo in string representation in DMAError type [#305]
- Corrected pin definitions for the Flexible Static Memory Controller / Flexible Memory Controller
  LCD interface [#312]
- Eliminated `channel_impl` macro warnings caused by unused ident [#323]

[#305]: https://github.com/stm32-rs/stm32f4xx-hal/pull/305
[#312]: https://github.com/stm32-rs/stm32f4xx-hal/pull/312
[#323]: https://github.com/stm32-rs/stm32f4xx-hal/pull/323

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
- Allow specification of ADC reference voltage in ADC configuration structure
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

[#231]: https://github.com/stm32-rs/stm32f4xx-hal/pull/231
[#262]: https://github.com/stm32-rs/stm32f4xx-hal/pull/262
[#263]: https://github.com/stm32-rs/stm32f4xx-hal/pull/263
[#278]: https://github.com/stm32-rs/stm32f4xx-hal/issues/278
[#273]: https://github.com/stm32-rs/stm32f4xx-hal/pull/273
[#286]: https://github.com/stm32-rs/stm32f4xx-hal/pull/286

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

- Made the `rt` feature of stm32f4 optional.

### Fixed

- Avoid overwriting the cache bits in `flash.acr`.

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

- Support longer Delay without asserting.

## v0.1.0 - 2018-10-02

### Added

- Support for stm32f407 and stm32f429.

[Unreleased]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.17.1...HEAD
[v0.17.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.17.0...v0.17.1
[v0.17.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.16.2...v0.17.0
[v0.16.2]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.16.1...v0.16.2
[v0.16.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.16.0...v0.16.1
[v0.16.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.15.0...v0.16.0
[v0.15.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.14.0...v0.15.0
[v0.14.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.13.2...v0.14.0
[v0.13.2]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.13.1...v0.13.2
[v0.13.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.13.0...v0.13.1
[v0.13.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.11.1...v0.12.0
[v0.11.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.11.0...v0.11.1
[v0.11.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.10.1...v0.11.0
[v0.10.1]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.10.0...v0.10.1
[v0.10.0]: https://github.com/stm32-rs/stm32f4xx-hal/compare/v0.9.0...v0.10.0
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
