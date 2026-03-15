# Feedback for v2.6 - CRASH ON TOUCH

**Status**: ❌ BROKEN - Panics on touch input

## What Works
- ✅ Boot sequence completes
- ✅ SDRAM initialization at 0xc0000000
- ✅ Display initialization (NT35510)
- ✅ Touch initialization
- ✅ Display shows "Ready! Touch screen to interact"

## What's Broken

### CRITICAL: FT6X06 Multi-Touch Panic
**First touch causes immediate crash:**

```
INFO  Touch #1
ERROR panicked at /home/z/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### Root Cause
The `ft6x06-0.1.2` crate has an assertion that panics when `ntouch > FT6X06_MAX_NB_TOUCH`. This happens because:

1. The FT6X06 controller can report more touches than the constant allows
2. The crate doesn't handle this gracefully - it panics instead of clamping
3. This is triggered on the **very first touch event**

### Missing Debug Info
```
WARN Insufficient DWARF info; compile your program with `debug = 2` to enable location info.
```
Log messages show `<invalid location: defmt frame-index: X>` instead of proper file:line.

## Fix Required

### Option 1: Create a Safe Wrapper (Recommended)
Create a wrapper around the FT6X06 driver that:
```rust
// In touch.rs or similar
pub fn read_touch_safe(i2c: &mut I2c) -> Option<TouchEvent> {
    let touches = ft6x06::read(i2c).ok()?;
    // Clamp touch count to safe value
    let safe_count = touches.len().min(FT6X06_MAX_TOUCHES);
    if safe_count > 0 {
        Some(TouchEvent {
            x: touches[0].x,
            y: touches[0].y,
            finger_count: safe_count as u8,
        })
    } else {
        None
    }
}
```

### Option 2: Patch or Replace ft6x06 Crate
- Fork the crate and remove/fix the assertion
- Use a different touch driver library
- Add error handling instead of panic

### Also Fix: Add `debug = 2` to Cargo.toml
```toml
[profile.release]
debug = 2
```

## Version Info
- Version: v2.6
- Chip: STM32F469NIHx
- Display Controller: NT35510
- Touch Controller: FT6X06

## Comparison with v2.5
| Feature | v2.5 | v2.6 |
|---------|------|------|
| Boot | ✅ | ✅ |
| Display | ✅ | ✅ |
| Touch Init | ✅ | ✅ |
| Touch Events | ✅ | ❌ PANIC |
| debug = 2 | ✅ | ❌ |

---

**Conclusion**: v2.6 is unusable due to FT6X06 panic. Must use safe wrapper like v2.5 does.
