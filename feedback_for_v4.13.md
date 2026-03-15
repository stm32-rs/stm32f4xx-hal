# Feedback for Specter-DIY Firmware v4.13

## Summary

❌ **MAJOR REGRESSION** - v4.13 appears to be the same as v0.2.1 (early test firmware). Lost all v4.x improvements.

---

## 🔴 Critical Issues

### 1. Missing `debug = 2`
```
WARN Insufficient DWARF info; compile your program with debug = 2
└─ <mod path> @ └─ <invalid location: defmt frame-index: 9>:0
```
All logs show `<invalid location>` - no file:line info.

### 2. No Version Number
```
INFO  Specter-DIY Rust Firmware
INFO  Display + Touch Test
```
- No version string (should say "v4.13")
- Says "Display + Touch Test" - same as v0.2.1

### 3. FT6X06 Multi-Touch Panic
```
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```
- Same crash as v0.2.1, v2.6-v3.0
- Safe wrapper removed

### 4. Missing All v4.x Features
- ❌ No RNG
- ❌ No USB
- ❌ No Bitcoin keys
- ❌ No frame loop
- ❌ No state machine
- ❌ No double buffering

---

## What Works (✅)

Only basic hardware init:
```
INFO  LCD reset done
INFO  LED initialized
INFO  SDRAM at 0xc0000000
INFO  Display initialized successfully
INFO  Display controller: Nt35510
INFO  FT6X06 touch initialized!
```

- Display initializes
- Touch initializes (until multi-touch panic)
- SDRAM works

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
WARN  Probe inconclusive; defaulting to NT35510
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
INFO    x=129, y=669
INFO    x=276, y=305
INFO    x=218, y=516
INFO    x=182, y=512
INFO    x=160, y=519
INFO    x=179, y=556
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

---

## Version Comparison

| Feature | v4.12 | v4.13 |
|---------|-------|-------|
| Debug Info | ✅ | ❌ Missing |
| Version String | ✅ "v4.12" | ❌ Missing |
| RNG | ✅ | ❌ Missing |
| USB | ✅ | ❌ Missing |
| Bitcoin Keys | ✅ | ❌ Missing |
| Frame Loop | ✅ 840+ | ❌ None |
| Double Buffer | ✅ | ❌ Single |
| FT6X06 Safe | ✅ | ❌ Panic |
| State Machine | ✅ | ❌ None |

---

## Analysis: What Happened?

**v4.13 appears to be identical to v0.2.1** (the early test firmware):

| Log Entry | v0.2.1 | v4.13 |
|-----------|--------|-------|
| Banner | "Specter-DIY Rust Firmware" | Same |
| Subtitle | "Display + Touch Test" | Same |
| Exit | "Ready! Touch screen to blink LED" | Same |
| Panic | ft6x06-0.1.2/lib.rs:332 | Same |

**Possible causes:**
1. Wrong file uploaded as v4.13
2. Build system pointed to wrong branch/tag
3. Major merge/revert error
4. Testing old code path

---

## Session Statistics

| Metric | Value |
|--------|-------|
| Frames | 0 (no frame loop) |
| Touch events | 6 (then panic) |
| Exit | PANIC (FT6X06 assertion) |
| Debug info | Missing |

---

## Required Fixes

### Critical
1. **Verify build source** - Check if correct branch/tag was built
2. **Add `debug = 2`** to release profile
3. **Add FT6X06 safe wrapper** - clamp touch count

### Restore Missing Features
1. RNG with PLL48CLK
2. USB CDC-ACM
3. Bitcoin key derivation
4. Frame loop
5. State machine
6. Double buffering
7. Version string

---

## Summary

**v4.13 is a complete regression.**

It appears to be the same code as v0.2.1 (the earliest test firmware), losing all improvements from v4.0 through v4.12:
- No debug info
- No RNG
- No USB
- No Bitcoin features
- No frame loop
- FT6X06 panic on multi-touch

**Recommendation**: Verify the build source and rebuild from the v4.12 codebase.

**Status**: ❌ REGRESSION - DO NOT USE
