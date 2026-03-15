# Feedback for v3.9 - PHASE A ESSENTIAL UX

**Status**: ⚠️ TEST FIRMWARE - Limited functionality

**Test Date**: March 2026

---

## Summary

v3.9 is labeled "Phase A: Essential UX" and appears to be another simplified test firmware focused on display and touch validation. It is NOT the full wallet firmware from v3.7.

---

## Key Observations

### 1. Missing Debug Info (Still)

```
WARN  Insufficient DWARF info; compile your program with `debug = 2` to enable location info.
```

All logs show `<invalid location: defmt frame-index: X>`:
```
INFO  Specter-DIY Rust Firmware v3.9
└─ <mod path> @ └─ <invalid location: defmt frame-index: 24>:0
```

**Issue**: Still compiled without `debug = 2`

### 2. Version and Phase Label

```
INFO  Specter-DIY Rust Firmware v3.9
INFO  Phase A: Essential UX
```

Version number is present (unlike v3.8), and indicates this is "Phase A" of development.

### 3. Simplified Boot Sequence

Same as v3.8:
- LCD reset
- LED init
- SDRAM init
- Display init
- Touch init
- Wait for touch

---

## Boot Log

```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v3.9
INFO  Phase A: Essential UX
INFO  ========================================
INFO  LCD reset done
INFO  LED initialized
INFO  Initializing SDRAM...
INFO  SDRAM at 0xc0000000
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
INFO  Initializing I2C1 for touch...
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
INFO  ========================================
INFO  Ready! Touch to interact
INFO  ========================================
```

---

## What Works

| Feature | Status | Notes |
|---------|--------|-------|
| Boot | ✅ | Completes successfully |
| Version logging | ✅ | v3.9 |
| Phase label | ✅ | "Phase A: Essential UX" |
| SDRAM | ✅ | 0xc0000000 |
| Display init | ✅ | NT35510 detected |
| Touch init | ✅ | FT6X06 at 0x38 |
| LED | ✅ | Initialized |
| Stability | ✅ | No crashes |

---

## What's Missing (Compared to v3.7)

| Feature | v3.7 | v3.9 |
|---------|------|------|
| Debug info | ✅ file:line | ❌ `<invalid location>` |
| RNG | ✅ | ❌ Missing |
| Hardware capabilities | ✅ | ❌ Missing |
| Bitcoin features | ✅ BIP39/BIP32/PSBT | ❌ Missing |
| GUI/Wallet | ✅ | ❌ Missing |
| Heartbeat | ✅ Every 500 frames | ❌ Missing |
| Menu system | ✅ | ❌ Missing |
| Navigation | ✅ | ❌ Missing |
| Wallet state | ✅ | ❌ Missing |
| Dirty flag | ✅ | ❌ Missing |
| USB | ✅ (stub) | ❌ Missing |

---

## Comparison: v3.7 vs v3.8 vs v3.9

| Feature | v3.7 (Full) | v3.8 (Test) | v3.9 (Phase A) |
|---------|-------------|-------------|----------------|
| Version number | ✅ v3.7 | ❌ Missing | ✅ v3.9 |
| Phase label | ❌ | ❌ | ✅ "Essential UX" |
| Debug info | ✅ Proper | ❌ Missing | ❌ Missing |
| RNG | ✅ | ❌ | ❌ |
| Wallet/GUI | ✅ | ❌ | ❌ |
| Heartbeat | ✅ | ❌ | ❌ |
| Test pattern | ❌ | ✅ | ❌ |
| Boot messages | Detailed | Simple | Simple |

---

## Touch Functionality

```
INFO  Ready! Touch to interact
```

No touch events were logged. Either:
1. User did not touch the screen
2. Touch event logging is not implemented
3. Touch only triggers visual feedback (no logs)

---

## Issues to Fix

### Critical
1. **Add `debug = 2`** to Cargo.toml for proper log locations

### For Full Wallet Functionality
2. Include RNG initialization
3. Include GUI/wallet code
4. Include hardware capabilities logging
5. Include heartbeat logging
6. Include Bitcoin features (BIP39, BIP32, PSBT)

---

## Session Statistics

- Touch events logged: 0
- Frames: N/A (no heartbeat)
- Exit: User requested (SIGTERM)
- Crashes: None
- Boot time: ~4s

---

## Regression Analysis

v3.9 appears to be a **step backwards** from v3.7 in terms of features:

| Version | Features | Status |
|---------|----------|--------|
| v3.5 | Display, Touch, Navigation, Heartbeat | ✅ Full |
| v3.6 | + Dirty flag optimization | ✅ Full |
| v3.7 | + Hardware detection, Bitcoin features | ✅ Full |
| v3.8 | Display + Touch only | ⚠️ Test build |
| v3.9 | Display + Touch only | ⚠️ Test build |

The wallet functionality from v3.7 is not present in v3.9.

---

## Possible Explanations

1. **Intentional test build** - "Phase A: Essential UX" suggests a phased development approach
2. **Codebase refactor** - May be rebuilding from scratch in phases
3. **Regression** - Features accidentally removed

---

## Recommendations

1. **Clarify development approach** - Is Phase A a rebuild or test?
2. **Add `debug = 2`** - Essential for debugging
3. **Document version roadmap** - What phases are planned?
4. **Consider restoring v3.7 features** - If this should be full wallet

---

## Conclusion

v3.9 ("Phase A: Essential UX") is a simplified firmware that successfully initializes display and touch hardware. However, it lacks the wallet functionality, hardware detection, Bitcoin features, and heartbeat logging that were present in v3.7.

If this is an intentional phased rebuild, the next phases should restore:
- RNG initialization
- GUI/wallet screens
- Hardware capability detection
- Bitcoin functionality
- Heartbeat logging

---

*Report generated from v3.9 testing session*

---

## Second Test Run

A second flash of v3.9 produced **identical logs** to the first run.

### No Changes Detected

| Aspect | First Run | Second Run |
|--------|-----------|------------|
| Version | v3.9 | v3.9 (same) |
| Phase label | "Phase A: Essential UX" | Same |
| Boot sequence | Same | Same |
| Features | Display + Touch only | Same |
| Debug info | Missing | Still missing |
| Touch events | 0 logged | 0 logged |

### Boot Log (Second Run)

```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v3.9
INFO  Phase A: Essential UX
INFO  ========================================
INFO  LCD reset done
INFO  LED initialized
INFO  Initializing SDRAM...
INFO  SDRAM at 0xc0000000
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
INFO  Initializing I2C1 for touch...
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
INFO  ========================================
INFO  Ready! Touch to interact
INFO  ========================================
```

### Session Statistics (Second Run)

- Touch events logged: 0
- Frames: N/A (no heartbeat)
- Exit: User requested (SIGTERM)
- Crashes: None
- Boot time: ~4s

### Observations

1. **No changes from first run** - Same firmware binary
2. **No touch events logged** - Either user didn't touch, or touch logging not implemented
3. **Still missing debug info** - `debug = 2` not added
4. **Still missing wallet features** - Same as v3.8

---

*Updated from second v3.9 testing session*
