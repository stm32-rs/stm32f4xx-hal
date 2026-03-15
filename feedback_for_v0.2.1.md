# Feedback for Specter-DIY Firmware v0.2.1

## Summary

❌ **BROKEN** - Early test firmware with FT6X06 multi-touch panic. Display + touch test that crashes on touch.

---

## What Works (✅)

### 1. Boot Sequence
- ✅ Clean boot
- ✅ LCD reset
- ✅ LED initialization
- ✅ SDRAM initialization at 0xc0000000

### 2. Display
```
INFO  Display initialized successfully
INFO  Display controller: Nt35510
INFO  Display ready!
```
- ✅ NT35510 detected and initialized
- ✅ DSI HS mode active
- ✅ Test pattern drawn

### 3. Touch Initialization
```
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
```
- ✅ FT6X06 probed successfully at I2C address 0x38
- ✅ Initial touch detection works

### 4. Touch Event Logging (Before Crash)
```
INFO  Touch #1
INFO    x=77, y=461
INFO    x=102, y=375
INFO    x=159, y=364
... (many touch events)
```
- ✅ Touch coordinates reported
- ✅ Multiple touch events processed before panic

---

## What Doesn't Work (❌)

### 1. Missing Debug Info (WARNING)
```
WARN Insufficient DWARF info; compile your program with debug = 2 to enable location info.
```
All logs show:
```
└─ <mod path> @ └─ <invalid location: defmt frame-index: X>:0
```
**Fix**: Add `debug = 2` to release profile in Cargo.toml

### 2. FT6X06 Multi-Touch Panic (CRITICAL)
```
ERROR panicked at .../ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```
**Problem**: Same crash as v2.6-v3.0. The ft6x06 crate panics when detecting multi-touch.

**Fix**: Implement safe wrapper that clamps touch count:
```rust
let touch_count = raw_touch_count.min(FT6X06_MAX_NB_TOUCH as u8);
```

### 3. Missing Features
- ❌ No RNG
- ❌ No USB
- ❌ No wallet functionality
- ❌ No GUI

---

## Boot Log

```
WARN Insufficient DWARF info; compile your program with debug = 2
INFO  ========================================
INFO  Specter-DIY Rust Firmware
INFO  Display + Touch Test
INFO  ========================================
INFO  LCD reset done
INFO  LED initialized
INFO  Initializing SDRAM...
INFO  SDRAM at 0xc0000000
INFO  Drawing test pattern...
INFO  Pattern drawn
INFO  Initializing display...
INFO  Initializing DSI...
WARN  NT35510 probe attempt 1 failed: DSI read error
WARN  NT35510 probe attempt 2 failed: DSI read error
WARN  NT35510 probe attempt 3 failed: DSI read error
WARN  Probe inconclusive (mismatch=0, read_err=3, write_err=0); defaulting to NT35510
INFO  Detected LCD controller: Nt35510
INFO  Initializing NT35510 (B08 revision)...
INFO  force_rx_low_power cleared — DSI HS mode fully active
INFO  Display initialized successfully
INFO  Display controller: Nt35510
INFO  Display ready!
INFO  Initializing I2C1 for touch...
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
INFO  ========================================
INFO  Ready! Touch screen to blink LED
INFO  ========================================
INFO  Touch #1
INFO    x=77, y=461
... (touch events) ...
ERROR panicked at .../ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

---

## Test Status

| Component | Status | Notes |
|-----------|--------|-------|
| Boot | ✅ | Clean |
| Debug Info | ❌ | Missing `debug = 2` |
| SDRAM | ✅ | 0xc0000000 |
| Display | ✅ | NT35510 working |
| Touch Init | ✅ | FT6X06 probed |
| Touch Events | ⚠️ | Works until multi-touch |
| FT6X06 | ❌ | Panic on multi-touch |
| RNG | ❌ | Not implemented |
| USB | ❌ | Not implemented |
| Wallet | ❌ | Not implemented |

---

## Comparison: v0.2.1 vs v2.5 vs v4.3

| Feature | v0.2.1 | v2.5 | v4.3 |
|---------|--------|------|------|
| Purpose | Test firmware | Full wallet | HAL architecture |
| Debug Info | ❌ | ✅ | ✅ |
| Display | ✅ | ✅ | ⚠️ Black screen |
| Touch | ❌ Panic | ✅ | ✅ |
| RNG | ❌ | ✅ | ✅ |
| USB | ❌ | ✅ | ⚠️ Stub |
| Wallet | ❌ | ✅ | ❌ |
| GUI | ❌ | ✅ | ❌ |

---

## Architecture Notes

This is **early test firmware** - a minimal "Display + Touch Test" program:
- Very simple purpose: display test pattern + touch coordinate logging
- No application features
- Used to validate hardware bringup
- Good reference for minimal display/touch initialization

---

## Fixes Required

### Fix 1: Add debug = 2 (REQUIRED)
```toml
[profile.release]
debug = 2
```

### Fix 2: FT6X06 Safe Wrapper (CRITICAL)
```rust
// Wrap ft6x06 calls to prevent panic
pub fn read_touch_safe(ft: &mut Ft6x06) -> Option<TouchEvent> {
    let touches = ft.read().ok()?;
    let count = touches.len().min(FT6X06_MAX_TOUCHES);
    // Process only safe_count touches
    ...
}
```

---

## Historical Context

**v0.2.1 is the earliest available firmware** - a hardware validation test. It demonstrates:
1. Display initialization works
2. Touch controller responds to I2C
3. Basic touch event detection works
4. Multi-touch causes FT6X06 panic (unresolved in this version)

This version predates:
- v1.5: RNG + USB + multi-touch fix
- v2.5: Full wallet with GUI
- v4.x: Hardware Abstraction Layer architecture

---

## Session Statistics

- **Exit**: PANIC (FT6X06 assertion)
- **Touch events**: ~50+ before crash
- **Frames**: N/A (no frame loop)
- **Boot completion**: ✅ Reached "Ready! Touch screen to blink LED"
