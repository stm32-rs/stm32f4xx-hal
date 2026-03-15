# v3.4 Status Report - DISPLAY WORKING!

**Status**: ✅ WORKING - First functional version since v2.5

**Test Date**: March 2026

---

## Executive Summary

**v3.4 is the first version since v2.5 that has a working display.** After 6 broken versions (v2.6 through v3.3), the display now shows content correctly.

### Version History Summary

| Version | Display | Touch | RNG | Status |
|---------|---------|-------|-----|--------|
| v2.5 | ✅ Works | ✅ Works | ✅ HW | ✅ **BASELINE** |
| v2.6 | N/A | ❌ Panic | N/A | ❌ Broken |
| v2.7 | ❌ Red | ❌ Panic | ✅ HW | ❌ Broken |
| v2.8 | ❌ Blank | N/A | ❌ Hang | ❌ Broken |
| v2.9 | ❌ Red | ❌ Panic | ✅ SW | ❌ Broken |
| v3.0 | ❌ Red | ❌ Panic | ✅ SW | ❌ Broken |
| v3.1 | ❌ Red | ❌ Panic | ✅ SW | ❌ Broken |
| v3.2 | ❌ Red | ✅ Works | ✅ HW | ⚠️ Partial |
| v3.3 | ❌ Red | ✅ Works | ✅ HW | ⚠️ Partial |
| **v3.4** | ✅ **Works** | ✅ Works | ✅ HW | ✅ **WORKING** |

---

## What Works in v3.4

### ✅ Display Output
- Screen shows GREEN during boot test
- Screen shows checkerboard pattern
- Screen shows RED during test
- Screen shows BLACK (proper clear)
- Home screen / UI is visible
- User confirmed: "yes" to seeing all test patterns

### ✅ Touch Handler
- 36 touch events logged without panic
- No FT6X06 assertion failure
- Touch coordinates are valid (e.g., (283, 161))
- Touch release detection works

### ✅ Hardware RNG
- `[RNG] Test OK: 0xb6eebba3`
- Ready after 15 iterations
- Using 168MHz SYSCLK (same as v2.5)

### ✅ Navigation
- Home → WalletGen → Home → LoadWallet all logged
- Screen transitions work correctly

### ✅ Stability
- 6000+ frames without crash
- Heartbeat every 500 frames
- Clean shutdown (user requested)

### ✅ Boot Sequence
```
GREEN (3 sec) → Checkerboard → RED → BLACK → GUI
```

---

## What Was Fixed

The key fix was in the LTDC layer configuration:
```
[LTDC] Configuring layer L1 with framebuffer at 0xc0000000
[LTDC] Enabling layer L1...
[LTDC] Reloading display...
```

The display tests confirmed framebuffer writes are now visible:
- GREEN fill: `[FILL] Done - pixel[0]=0x07e0 pixel[383999]=0x07e0`
- RED fill: `[FILL] Done - pixel[0]=0xf800 pixel[383999]=0xf800`
- BLACK fill: `[FILL] Done - pixel[0]=0x0000 pixel[383999]=0x0000`

---

## Potential Remaining Issues

### 1. USB Not Tested
Logs don't show USB initialization or connection:
```
INFO  [USB] Init...
INFO  [USB] OK
INFO  [USB] Connected - state=Configured
```
This was present in v2.5 but not logged in v3.4.

**Action**: Verify USB connectivity works

### 2. SD Card Not Tested
Logs don't show SD card initialization:
```
INFO  [SD] Init...
INFO  [SD] Init complete
```
This was present in v2.5 but not logged in v3.4.

**Action**: Verify SD card works

### 3. Wallet Generation Flow Not Tested
User navigated to WalletGen but didn't complete the flow.
- Need to test entropy generation
- Need to test mnemonic display
- Need to test backup verification

**Action**: Test complete wallet generation flow

### 4. Settings/About Screens Not Tested
User navigated between Home, WalletGen, and LoadWallet but not Settings or About.

**Action**: Test all menu items

### 5. DSI Read Errors Still Present
```
WARN  NT35510 probe attempt 1 failed: DSI read error
WARN  NT35510 probe attempt 2 failed: DSI read error
WARN  NT35510 probe attempt 3 failed: DSI read error
WARN  Probe inconclusive (mismatch=0, read_err=3, write_err=0); defaulting to NT35510
```
These warnings appear in ALL versions including v2.5. This is expected behavior - the probe fails but display works.

**Action**: None required (cosmetic)

---

## Remaining Work

### High Priority
1. **Test USB connectivity** - Verify device appears on USB
2. **Test SD card** - Verify storage works
3. **Test wallet generation end-to-end** - Create a new wallet

### Medium Priority
4. **Test all screens** - Settings, About, Sign Transaction
5. **Test back navigation** - Ensure user can go back from all screens
6. **Test multi-touch** - Verify FT6X06 safe wrapper handles 2+ fingers

### Low Priority
7. **Remove test pattern code** - Now that display works, the boot test (GREEN, checkerboard, RED) can be removed for production
8. **Clean up debug logging** - Reduce verbosity for production

---

## Comparison: v2.5 vs v3.4

| Feature | v2.5 | v3.4 | Notes |
|---------|------|------|-------|
| Display | ✅ | ✅ | Fixed! |
| Touch | ✅ | ✅ | Works |
| RNG | ✅ HW | ✅ HW | Works |
| Navigation | ✅ | ✅ | Works |
| USB | ✅ Logged | ❓ Not logged | Needs test |
| SD Card | ✅ Logged | ❓ Not logged | Needs test |
| Wallet Gen | ✅ Tested | ❓ Not tested | Needs test |
| SYSCLK | 168MHz | 168MHz | Same |
| Boot time | ~6.6s | ~3.0s | v3.4 faster |

---

## Recommended Next Steps

### For v3.5 (Next Version)

1. **Remove boot test sequence** (GREEN → checkerboard → RED)
   - Keep only BLACK clear before GUI
   - Faster boot to usable UI

2. **Add USB logging back**
   - Verify USB initialization
   - Log connection state

3. **Add SD card logging back**
   - Verify SD card initialization
   - Log if card present/absent

4. **Test plan for v3.5:**
   - Boot and verify UI loads quickly
   - Test wallet generation complete flow
   - Test all menu items
   - Test USB enumeration on host PC
   - Test SD card detection

---

## Technical Notes

### What Fixed the Display

The key difference in v3.4 is the LTDC layer configuration:

```rust
// v3.4 adds explicit layer enable:
[LTDC] Configuring layer L1 with framebuffer at 0xc0000000
[LTDC] Enabling layer L1...        // <-- THIS WAS MISSING
[LTDC] Reloading display...        // <-- THIS WAS MISSING
```

Previous versions configured the layer but may not have:
1. Explicitly enabled the layer
2. Reloaded the LTDC shadow registers

### Framebuffer Verification

v3.4 uses `fill_color_ptr` with verification:
```rust
INFO  [FILL] Done - pixel[0]=0x07e0 pixel[383999]=0x07e0
```
This confirms both first and last pixels are written correctly.

---

## Files Generated During Testing

- `feedback_for_v2.5.md` - Working baseline reference
- `feedback_for_v2.6.md` - FT6X06 panic
- `feedback_for_v2.7.md` - Red screen + panic
- `feedback_for_v2.8.md` - RNG hang
- `feedback_for_v2.9.md` - Red screen + panic
- `feedback_for_v3.0.md` - Red screen + panic
- `feedback_for_v3.1.md` - Red screen + panic
- `feedback_for_v3.1_v3.2_combined.md` - Display debugging
- `testing_report_v2.5_to_v3.0.md` - Full version history

---

## Conclusion

**v3.4 is functional and ready for feature testing.**

The display issue that plagued v2.6 through v3.3 has been resolved. The firmware now boots, displays the UI, responds to touch, and navigates between screens without crashing.

**Remaining work**: Verify USB, SD card, and complete wallet generation flow.

---

*Status report generated from v3.4 testing session*
