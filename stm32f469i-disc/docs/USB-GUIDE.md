# USB Device Guide

The STM32F469I-DISCO board includes a USB OTG Full-Speed peripheral that can operate in device mode. This guide covers how to use the BSP's USB module to create USB devices such as CDC-ACM serial ports.

## Overview

- **Peripheral**: USB OTG FS (Full-Speed)
- **Feature Flag**: `usb_fs`
- **Pins**: PA11 (DM), PA12 (DP)
- **Endpoint Count**: 6 endpoints (F469-specific)
- **FIFO Depth**: 320 words

The BSP provides a convenience wrapper around the HAL's USB implementation, handling pin configuration and peripheral initialization.

## Quick Start

### Enable the Feature

Add the `usb_fs` feature to your `Cargo.toml`:

```toml
[dependencies.stm32f469i-disc]
features = ["usb_fs"]
```

### Basic Initialization

```rust
use stm32f469i_disc::{prelude::*, usb};

let dp = pac::Peripherals::take().unwrap();
let mut rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(48.MHz()).require_pll48clk());

let gpioa = dp.GPIOA.split(&mut rcc);

// Initialize USB using BSP helper
let usb = usb::init(
    (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
    gpioa.pa11,
    gpioa.pa12,
    &rcc.clocks,
);
```

## API Reference

### `usb::init()`

Initializes the USB OTG FS peripheral with the correct pin alternate functions.

**Signature:**
```rust
pub fn init(
    periphs: (
        pac::OTG_FS_GLOBAL,
        pac::OTG_FS_DEVICE,
        pac::OTG_FS_PWRCLK,
    ),
    pa11: gpio::PA11,  // USB DM
    pa12: gpio::PA12,  // USB DP
    clocks: &rcc::Clocks,
) -> USB
```

**Returns:** A `USB` struct implementing `UsbPeripheral`, ready for use with `UsbBus::new()`.

The function configures:
- PA11 as USB DM (Data Minus)
- PA12 as USB DP (Data Plus)
- Alternate function mode for USB operation

## Building CDC-ACM Serial

The most common USB device class is CDC-ACM, which creates a virtual serial port. Here's how to build one:

### Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
static_cell = "2"
usb-device = "0.3"
usbd-serial = "0.2"
```

### Complete CDC-ACM Example

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use static_cell::ConstStaticCell;
use stm32f469i_disc::{pac, prelude::*, usb};
use stm32f4xx_hal::otg_fs::UsbBus;
use usb_device::prelude::*;

// Endpoint memory must be statically allocated
static EP_MEMORY: ConstStaticCell<[u32; 1024]> = ConstStaticCell::new([0; 1024]);

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Critical: 48MHz PLL48CLK required for USB
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(48.MHz())
            .require_pll48clk()
    );

    let gpioa = dp.GPIOA.split(&mut rcc);

    // Initialize USB using BSP helper
    let usb = usb::init(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        gpioa.pa11,
        gpioa.pa12,
        &rcc.clocks,
    );

    // Create USB bus with endpoint memory
    let usb_bus = UsbBus::new(usb, EP_MEMORY.take());

    // Create CDC-ACM serial port
    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    // Create USB device
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(usbd_serial::USB_CLASS_CDC)
        .strings(&[StringDescriptors::default()
            .manufacturer("STM32F469")
            .product("CDC Serial")
            .serial_number("DISCO1")])
        .unwrap()
        .build();

    loop {
        // Poll USB device
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // Read and echo back
        let mut buf = [0u8; 64];
        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back (uppercased)
                for byte in buf[..count].iter_mut() {
                    if byte.is_ascii_lowercase() {
                        *byte &= !0x20;
                    }
                }
                let _ = serial.write(&buf[..count]);
            }
            _ => {}
        }
    }
}
```

## Common Issues

### 48MHz Clock Requirement

USB requires a precise 48MHz clock. Use `require_pll48clk()` in your RCC config:

```rust
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(48.MHz())
        .require_pll48clk()  // Required for USB!
);
```

Without this, USB will not enumerate correctly.

### Endpoint Memory Allocation

The USB peripheral needs dedicated SRAM for endpoint buffers. This must be statically allocated:

```rust
use static_cell::ConstStaticCell;
static EP_MEMORY: ConstStaticCell<[u32; 1024]> = ConstStaticCell::new([0; 1024]);
```

1024 words (4KB) is sufficient for most CDC-ACM applications. For multiple endpoints or bulk transfers, you may need more.

### Interrupt Handling (Optional)

For interrupt-driven USB instead of polling, enable the USB interrupt in your device:

```rust
// In your interrupt configuration
dp.OTG_FS_GLOBAL.gintmsk.write(|w| {
    w.usbrstm().set_bit()  // USB reset
    .enumdnem().set_bit()  // Enumeration done
    .oepint().set_bit()    // OUT endpoint
    .iepint().set_bit()    // IN endpoint
});
```

For RTIC applications, route the `OTG_FS` interrupt to your task.

### Feature Conflicts

Only one USB feature can be enabled at a time:
- `usb_fs` - Full-Speed (use this for STM32F469I-DISCO)
- `usb_hs` - High-Speed (not available on this board's connector)

## Hardware Notes

### USB Connector

The USB connector is located on the back side of the board, near the display connector. It's a micro-USB OTG connector.

### Pin Assignment

| Pin | Function | Notes |
|-----|----------|-------|
| PA11 | USB_DM | Data Minus |
| PA12 | USB_DP | Data Plus |

These pins are hard-wired to the USB connector and cannot be changed.

### Power Modes

The STM32F469I-DISCO USB operates in bus-powered mode. The board receives power from the USB connection when plugged in.

### Limitations

- Only device mode is supported by the HAL
- OTG host mode is not available in the current implementation
- High-Speed USB is not exposed on this board's connector

## Related Examples

The HAL repository includes a complete CDC-ACM example at:
- `examples/usb-serial-poll.rs` - Polling-based CDC serial port

## Troubleshooting

| Symptom | Likely Cause | Solution |
|---------|--------------|----------|
| Device not recognized | Missing 48MHz clock | Add `require_pll48clk()` to RCC config |
| Enumeration fails | EP memory not allocated | Add static `EP_MEMORY` allocation |
| Crash on init | Wrong feature enabled | Use `usb_fs`, not `usb_hs` |
| Intermittent connection | HSE crystal issues | Verify HSE frequency matches config |
