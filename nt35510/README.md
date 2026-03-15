# nt35510

Standalone `no_std` NT35510 DSI LCD controller driver.

This crate is transport-agnostic and accepts any DSI host implementing
`embedded_display_controller::dsi::DsiHostCtrlIo`.

## Usage

```rust
use embedded_display_controller::dsi::DsiHostCtrlIo;
use embedded_hal::delay::DelayNs;
use nt35510::{ColorFormat, Mode, Nt35510, Nt35510Config};

fn init_display(dsi: &mut impl DsiHostCtrlIo, delay: &mut impl DelayNs) {
    let mut panel = Nt35510::new();
    let _ = panel.probe(dsi, delay);

    let config = Nt35510Config {
        mode: Mode::Portrait,
        color_format: ColorFormat::Rgb565,
        ..Nt35510Config::default()
    };
    panel.init_with_config(dsi, delay, config).unwrap();
}
```

`Nt35510Config::default()` matches STM32F469I-DISCO tested settings
(portrait, RGB565, 480x800). Landscape mode is available but currently untested.
`init()` and `init_rgb565()` remain convenience wrappers.
