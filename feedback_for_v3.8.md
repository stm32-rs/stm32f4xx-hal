# Feedback for v3.8 - DISPLAY + TOUCH TEST FIRMWARE

**Status**: ⚠️ TEST FIRMWARE - Not full wallet implementation

**Test Date**: March 2026

---

## Summary

v3.8 is a **simplified test firmware** focused only on display and touch hardware validation. It is NOT the full wallet firmware. This appears to be a diagnostic/hardware test build.

---

## Key Observations

### 1. Missing Debug Info

```
WARN  Insufficient DWARF info; compile your program with `debug = 2` to enable location info.
```

All logs show `<invalid location: defmt frame-index: X>` instead of proper file:line:
```
INFO  Specter-DIY Rust Firmware
└─ <mod path> @ └─ <invalid location: defmt frame-index: 24>:0
```

**Issue**: Compiled without `debug = 2` in Cargo.toml

### 2. Different Firmware Type

```
INFO  Specter-DIY Rust Firmware
INFO  Display + Touch Test
```

This is a **hardware test firmware**, not the full wallet application.

### 3. No Version Number

Unlike v3.5-v3.7 which logged `Specter-DIY Rust Firmware v3.X`, this version has no version number.

### 4. Missing Features (Compared to v3.7)

| Feature | v3.7 | v3.8 |
|---------|------|------|
| Version logging | ✅ `v3.7` | ❌ No version |
| RNG initialization | ✅ | ❌ Missing |
| Hardware capabilities | ✅ | ❌ Missing |
| Bitcoin features | ✅ BIP39/BIP32/PSBT | ❌ Missing |
| GUI/Wallet | ✅ | ❌ Missing |
| Heartbeat | ✅ Every 500 frames | ❌ Missing |
| Menu system | ✅ | ❌ Missing |
| Navigation | ✅ | ❌ Missing |
| Wallet state | ✅ | ❌ Missing |
| Dirty flag | ✅ | ❌ Missing |
| Debug info | ✅ file:line | ❌ `<invalid location>` |

---

## Boot Log

```
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
```

---

## What Works

| Feature | Status | Notes |
|---------|--------|-------|
| Boot | ✅ | Completes successfully |
| SDRAM | ✅ | 0xc0000000 |
| Display init | ✅ | NT35510 detected |
| Test pattern | ✅ | Drawn before display init |
| Touch init | ✅ | FT6X06 at 0x38 |
| LED | ✅ | Initialized |
| Stability | ✅ | No crashes |

---

## What's Missing / Not Working

| Feature | Status | Notes |
|---------|--------|-------|
| Version number | ❌ | Not logged |
| Debug info | ❌ | `debug = 2` missing |
| RNG | ❌ | Not initialized |
| Wallet/GUI | ❌ | Not included |
| Heartbeat | ❌ | No periodic logging |
| Touch events | ❓ | None logged (may need user input) |
| USB | ❌ | Not initialized |
| SD card | ❌ | Not initialized |

---

## Boot Sequence Analysis

v3.8 has a simpler boot sequence:

```
1. LCD reset
2. LED init
3. SDRAM init
4. Draw test pattern
5. Display init (DSI, NT35510)
6. Touch init (I2C1, FT6X06)
7. Wait for touch (LED blink on touch)
```

vs v3.7 full boot:

```
1. Peripherals
2. RNG
3. Clocks
4. GPIO
5. LCD reset
6. LED
7. SDRAM
8. Framebuffer clear
9. Display init
10. LTDC layer config
11. Touch init
12. Hardware capabilities log
13. GUI init
14. Main loop with heartbeat
```

---

## Touch Functionality

The firmware says:
```
INFO  Ready! Touch screen to blink LED
```

No touch events were logged. Either:
1. User did not touch the screen during the test
2. Touch event logging is not implemented
3. Touch handler only blinks LED without logging

---

## Comparison: v3.7 vs v3.8

| Aspect | v3.7 | v3.8 |
|--------|------|------|
| Purpose | Full wallet | Hardware test |
| Lines of log | ~50+ | ~25 |
| Features | All wallet features | Display + Touch only |
| Debug info | ✅ Proper | ❌ Missing |
| Test pattern | ❌ No | ✅ Yes |
| Boot time | ~4s | ~4s |

---

## Issues to Fix

### Critical
1. **Add `debug = 2`** to Cargo.toml for proper log locations

### If This Should Be Full Wallet
2. **Add version number** to boot log
3. **Include RNG initialization**
4. **Include GUI/wallet code**
5. **Include heartbeat logging**

### If This Is Intentional Test Firmware
- Document as test/hardware validation build
- Consider adding version like "v3.8-test"

---

## Session Statistics

- Touch events: 0 logged
- Frames: N/A (no heartbeat)
- Exit: User requested (SIGTERM)
- Crashes: None

---

## Conclusion

v3.8 appears to be a **hardware test firmware** for validating display and touch functionality. It successfully initializes display (NT35510) and touch (FT6X06) without crashes.

**If this was intended to be the full wallet firmware**, it is missing:
- Version number
- Debug info (`debug = 2`)
- RNG initialization
- Wallet/GUI functionality
- Heartbeat logging

**Recommendation**: Clarify if this is a test build or if features were accidentally removed.

---

*Report generated from v3.8 testing session*
