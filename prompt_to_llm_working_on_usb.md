# Context for LLM Working on USB (STM32F469I-DISCO)

## What Was Done

Added USB documentation and a working CDC-ACM example to the BSP.

## Where to Find It

```
Repository: https://github.com/Amperstrand/stm32f4xx-hal
Branch:     pr2-f469disco-examples
Commit:     9c5b5ce
```

## Files You Should Read

1. **`stm32f469i-disc/src/usb.rs`** — BSP USB module, provides `usb::init()`
2. **`stm32f469i-disc/docs/USB-GUIDE.md`** — Clock requirements, CDC-ACM pattern, troubleshooting
3. **`stm32f469i-disc/examples/usb_cdc_serial.rs`** — Working CDC-ACM example
4. **`stm32f469i-disc/src/lib.rs`** — BSP module exports (line 16: `#[cfg(feature = "usb_fs")] pub mod usb;`)

## Critical Facts

### USB Module IS Functional
Previous documentation (BSP-Update-Summary-v0.3.0.md) incorrectly stated USB was "commented out due to HAL API issues". This was wrong. The module works.

### 48MHz Clock Requirement
USB requires `require_pll48clk()` in RCC config. Without this, USB will not enumerate.

```rust
let mut rcc = dp.RCC.freeze(
    hal::rcc::Config::hse(8.MHz())
        .sysclk(168.MHz())
        .require_pll48clk(),  // REQUIRED for USB
);
```

### Pins Used
- PA11 = USB DM
- PA12 = USB DP

### Feature Flag
Enable with `--features usb_fs` when building.

## How to Use the Example

```bash
# Build
cargo build --manifest-path stm32f469i-disc/Cargo.toml \
  --example usb_cdc_serial --features usb_fs --release

# Flash (with probe-rs)
probe-rs run --chip STM32F469NIHx \
  target/thumbv7em-none-eabihf/release/examples/usb_cdc_serial

# Connect (appears as /dev/tty.usbmodem* or /dev/ttyACM0)
# Type characters, they echo back in uppercase
```

## What's Missing / Next Steps

1. **USB interrupt handling** — Current example uses polling. For production, add OTG_FS interrupt.
2. **USB serial receive buffer** — Current example has no buffering.
3. **Multiple USB classes** — Only CDC-ACM is demonstrated.

## HAL USB Reference

The HAL has generic USB examples in `/examples/`:
- `usb-serial-poll.rs` — Polling-based CDC
- `usb-serial-irq.rs` — Interrupt-based CDC
- `rtic-usb-cdc-echo.rs` — RTIC framework

These are chip-agnostic. The BSP example is board-specific.

## Pin Constraint Warning

SDRAM consumes 52 pins. See `stm32f469i-disc/docs/PIN-CONSUMPTION.md` before adding peripherals.

## Build Verification

```bash
# Verify USB example compiles
cargo check --manifest-path stm32f469i-disc/Cargo.toml \
  --example usb_cdc_serial --features usb_fs
# Should exit 0 with no errors
```
