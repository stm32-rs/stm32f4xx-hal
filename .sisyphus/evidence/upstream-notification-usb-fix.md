# USB BSP Module Fix - Ready for Testing

## Summary

The USB OTG FS module in the STM32F469I-DISCO BSP has been fixed to use the correct HAL API. The previous implementation was broken and used an incorrect `UsbBus::new()` API. The fix now returns a `USB` struct that users can pass to `UsbBus::new(usb, ep_memory)`.

## Commit to Test

| Field | Value |
|-------|-------|
| **Commit Hash** | `50e65d32ef333456132396785a453415e778b2f8` |
| **Short Hash** | `50e65d3` |
| **Branch** | `pr2-f469disco-examples` |
| **Repository** | `git@github.com:Amperstrand/stm32f4xx-hal.git` |

## How to Test

### 1. Add the HAL as a dependency with path override

In your project's `Cargo.toml`:

```toml
[dependencies.stm32f4xx-hal]
git = "https://github.com/Amperstrand/stm32f4xx-hal.git"
branch = "pr2-f469disco-examples"
features = ["stm32f469", "usb_fs"]
```

Or with specific commit:

```toml
[dependencies.stm32f4xx-hal]
git = "https://github.com/Amperstrand/stm32f4xx-hal.git"
rev = "50e65d3"
features = ["stm32f469", "usb_fs"]
```

### 2. Use the BSP USB module

```rust
use stm32f469i_disc::usb;

// In your initialization code:
let gpioa = dp.GPIOA.split(&mut rcc);

let usb = usb::init(
    (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
    gpioa.pa11,
    gpioa.pa12,
    &rcc.clocks,
);

// Then create UsbBus with your own endpoint memory:
use stm32f4xx_hal::otg_fs::UsbBus;
use static_cell::ConstStaticCell;
static EP_MEMORY: ConstStaticCell<[u32; 1024]> = ConstStaticCell::new([0; 1024]);

let usb_bus = UsbBus::new(usb, EP_MEMORY.take());
```

## What Was Fixed

### Before (Broken)
```rust
pub fn init(...) -> UsbBus {
    unsafe { UsbBus::new((_otg_fs_global, _otg_fs_device, _otg_fs_pwrclk)) }
}
```

**Problems:**
- Returned `UsbBus` instead of `USB` struct
- Used incorrect `UsbBus::new()` signature (HAL's `UsbBus::new()` requires `(USB, ep_memory)`)
- Used `unsafe` block unnecessarily

### After (Fixed)
```rust
pub fn init(
    periphs: (
        hal::pac::OTG_FS_GLOBAL,
        hal::pac::OTG_FS_DEVICE,
        hal::pac::OTG_FS_PWRCLK,
    ),
    pa11: hal::gpio::PA11,
    pa12: hal::gpio::PA12,
    clocks: &hal::rcc::Clocks,
) -> USB {
    USB::new(periphs, (pa11, pa12), clocks)
}
```

**Improvements:**
- Returns `USB` struct matching HAL's API
- Uses correct `USB::new()` constructor
- No unsafe code
- Follows the pattern from HAL's `examples/usb-serial-poll.rs`
- Properly feature-gated with `#[cfg(feature = "usb_fs")]`

## Verification

- ✅ `cargo build --lib` succeeds in BSP
- ✅ No LSP diagnostics
- ✅ Commit pushed to `origin/pr2-f469disco-examples`

## Related

- HAL USB struct definition: `src/otg_fs.rs`
- Reference example: `examples/usb-serial-poll.rs`
- BSP module: `stm32f469i-disc/src/usb.rs`

---

*Generated: 2026-03-03*
*Commit: 50e65d32ef333456132396785a453415e778b2f8*
