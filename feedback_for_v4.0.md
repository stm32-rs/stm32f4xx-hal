# Feedback for v4.0 - HARDWARE ABSTRACTION LAYER

**Status**: ❌ BROKEN - FT6X06 panic + RNG timeout

**Test Date**: March 2026

---

## Summary

v4.0 introduces a new "Hardware Abstraction Layer" architecture with proper debug info, but has two critical issues:
1. **RNG initialization timeout** - Returns 0x00000000
2. **FT6X06 panic** - Same crash as v2.6-v3.0

---

## What's New in v4.0

### 1. Hardware Abstraction Layer
```
INFO  Specter-DIY Rust Firmware v4.0
INFO  Hardware Abstraction Layer
```

### 2. Debug Info FIXED! ✅
```
INFO  Initializing Hardware RNG...
└─ firmware::__cortex_m_rt_main @ /home/z/.../firmware/src/main.rs:163
ERROR RNG initialization timeout!
└─ specter_hal::rng::{impl#0}::init @ /home/z/.../hal/src/rng.rs:42
```

Proper file:line locations now shown! No more `<invalid location>`.

### 3. New Components Initialized

| Component | Status | Notes |
|-----------|--------|-------|
| Hardware RNG | ⚠️ Timeout | Returns 0x00000000 |
| Flash Storage | ⚠️ Not initialized | magic=0xffffffff |
| USB Serial | ✅ Stub mode | Driver created |
| Display | ✅ | NT35510 |
| Touch | ✅ Init | Then PANIC |

### 4. Test Pattern
```
INFO  Drawing test pattern...
INFO  Test pattern drawn
```

### 5. Detailed Board Info
```
INFO  Board: STM32F469I-DISCO
INFO  MCU: STM32F469NI
INFO  Display: 480x800
INFO  Flash: 131072 bytes
```

---

## Boot Log

```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v4.0
INFO  Hardware Abstraction Layer
INFO  ========================================
INFO  Initializing Hardware RNG...
ERROR RNG initialization timeout!
WARN  RNG not initialized, initializing now...
ERROR RNG initialization timeout!
ERROR RNG timeout!
INFO  RNG test: 0x00000000
WARN  RNG not initialized, initializing now...
ERROR RNG initialization timeout!
ERROR RNG timeout!
WARN  RNG not initialized, initializing now...
ERROR RNG initialization timeout!
ERROR RNG timeout!
INFO  Random seed: [0, 0, 0, 0, 0, 0, 0, 0]
INFO  Initializing Flash Storage...
INFO  Flash storage not initialized (magic=0xffffffff)
INFO  Flash: not initialized
INFO  Initializing USB Serial...
INFO  USB: Serial driver created
INFO  USB: Driver initialized (stub mode)
INFO  LCD reset done
INFO  LED initialized
INFO  Initializing SDRAM...
INFO  SDRAM at 0xc0000000
INFO  Drawing test pattern...
INFO  Test pattern drawn
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
INFO  Display layer configured
INFO  Initializing I2C1 for touch...
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
INFO  Board: STM32F469I-DISCO
INFO  MCU: STM32F469NI
INFO  Display: 480x800
INFO  Flash: 131072 bytes
INFO  ========================================
INFO  v4.0 Ready!
INFO  - Hardware RNG: Active
INFO  - Flash Storage: Ready
INFO  - USB Serial: Ready
INFO  - Touch: Active
INFO  ========================================
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

---

## What Works

| Feature | Status | Notes |
|---------|--------|-------|
| Boot | ✅ | Progresses to "Ready!" |
| Debug info | ✅ | Proper file:line now! |
| SDRAM | ✅ | 0xc0000000 |
| Display init | ✅ | NT35510 detected |
| Display layer | ✅ | Configured |
| Touch init | ✅ | FT6X06 at 0x38 |
| USB Serial | ✅ | Stub mode |
| Test pattern | ✅ | Drawn |
| Board info | ✅ | Logged |
| LED | ✅ | Initialized |

---

## What's Broken

### 1. RNG Initialization Timeout (CRITICAL)

```
ERROR RNG initialization timeout!
ERROR RNG timeout!
INFO  RNG test: 0x00000000
INFO  Random seed: [0, 0, 0, 0, 0, 0, 0, 0]
```

**Problem**: RNG never becomes ready, returns all zeros.

**Location**: `specter_hal::rng::{impl#0}::init @ hal/src/rng.rs:42`

**Impact**: 
- No secure random numbers
- Wallet generation would be insecure
- Random seed is all zeros

**Possible causes**:
- RNG clock not enabled before timeout check
- Wrong timeout value
- Clock configuration issue

### 2. FT6X06 Multi-Touch Panic (CRITICAL)

```
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

**Problem**: Same crash as v2.6-v3.0. The FT6X06 crate panics on multi-touch.

**Stack trace**:
```
Frame 9: <unknown function @ 0x0800386c>
         ft6x06-0.1.2/src/lib.rs:332:9
Frame 12: __cortex_m_rt_main
           firmware/src/main.rs:389:14
```

**Impact**: Firmware crashes on first touch event.

### 3. Flash Storage Not Initialized

```
INFO  Flash storage not initialized (magic=0xffffffff)
INFO  Flash: not initialized
```

**Problem**: Flash storage is blank/unformatted.

**Impact**: 
- No persistent wallet storage
- Settings not saved
- May need initialization routine

---

## Version Comparison

| Feature | v3.7 | v3.8/v3.9 | v4.0 |
|---------|------|-----------|------|
| Debug info | ✅ | ❌ | ✅ |
| RNG | ✅ | ❌ | ❌ Timeout |
| Wallet/GUI | ✅ | ❌ | ❌ |
| FT6X06 panic | ✅ No | N/A | ❌ Yes |
| Flash storage | ❌ | ❌ | ⚠️ Not init |
| USB Serial | ⚠️ Stub | ❌ | ⚠️ Stub |
| Test pattern | ❌ | ✅/❌ | ✅ |
| HAL architecture | ❌ | ❌ | ✅ |

---

## Architecture Changes

v4.0 appears to have restructured the codebase:

```
Old structure:
- firmware/src/main.rs
- firmware/src/board/mod.rs

New structure:
- firmware/src/main.rs
- hal/src/rng.rs       (new HAL module)
- hal/src/flash.rs     (new HAL module)
- hal/src/usb.rs       (new HAL module)
```

This is a major refactoring with a Hardware Abstraction Layer.

---

## Fixes Required

### Fix 1: RNG Initialization (CRITICAL)

The RNG timeout suggests the clock is not being enabled or the peripheral is not ready.

```rust
// Check hal/src/rng.rs:42
// Ensure:
// 1. RCC AHB2ENR.RNGEN is set BEFORE waiting
// 2. Timeout is reasonable (10000+ iterations)
// 3. RNG_CR.RNGEN is set
```

Reference: v3.7 RNG initialization works:
```
INFO  [RNG] Ready after 15 iterations
```

### Fix 2: FT6X06 Safe Wrapper (CRITICAL)

Must implement safe wrapper like v3.5-v3.7:

```rust
// In touch handling code (main.rs:389)
pub fn detect_touch_safe(&mut self) -> Option<TouchEvent> {
    let touches = ft6x06.read()?;
    let safe_count = touches.len().min(FT6X06_MAX_TOUCHES);
    // ... handle safely without panic
}
```

### Fix 3: Flash Storage Initialization

Need to initialize/format flash on first use:
```rust
// Check magic value, if 0xffffffff, initialize storage
if magic == 0xffffffff {
    flash.format();
}
```

---

## Session Statistics

- Exit: PANIC (FT6X06 assertion)
- Touch events: 1 (then crash)
- Frames: N/A
- Boot completion: ✅ Reached "v4.0 Ready!"

---

## Improvements from v3.9

| Aspect | v3.9 | v4.0 |
|--------|------|------|
| Debug info | ❌ Missing | ✅ Proper file:line |
| RNG | ❌ Missing | ⚠️ Present but broken |
| Flash storage | ❌ Missing | ⚠️ Present but uninitialized |
| USB Serial | ❌ Missing | ⚠️ Present (stub) |
| Architecture | Simple | HAL-based |

---

## Conclusion

v4.0 is a major architectural refactoring with the new Hardware Abstraction Layer. It has:

**Improvements:**
- ✅ Debug info now working (`debug = 2`)
- ✅ New HAL architecture
- ✅ Flash storage module
- ✅ USB serial module
- ✅ Board info logging

**Regressions:**
- ❌ RNG broken (timeout)
- ❌ FT6X06 panic returns (same as v2.6-v3.0)
- ❌ Wallet/GUI not present

**Priority fixes:**
1. Fix RNG initialization (clock timing)
2. Add FT6X06 safe wrapper (from v3.5-v3.7)
3. Initialize flash storage on first use

---

*Report generated from v4.0 testing session*
