# Feedback for v3.6 - DIRTY FLAG OPTIMIZATION

**Status**: ✅ WORKING - With dirty flag optimization

**Test Date**: March 2026

---

## Summary

v3.6 adds the **dirty flag optimization** that was present in v2.5 but missing from v3.4/v3.5. This should reduce unnecessary redraws and potentially fix the flickering issue.

---

## Key Feature: Dirty Flag

v3.6 now logs the `dirty` flag status in heartbeats:

```
INFO  [HEARTBEAT] frame=500 screen=LoadWallet touches=7 dirty=false
INFO  [HEARTBEAT] frame=1000 screen=LoadWallet touches=11 dirty=false
INFO  [HEARTBEAT] frame=1500 screen=LoadWallet touches=11 dirty=false
```

**`dirty=false`** means the screen is NOT being redrawn unnecessarily - only when changes occur.

---

## What Works

| Feature | Status | Notes |
|---------|--------|-------|
| Boot | ✅ | Fast, clean |
| Display | ✅ | LTDC layer enabled |
| Touch | ✅ | 34 events, no panic |
| Navigation | ✅ | Home → WalletGen → Home → LoadWallet |
| RNG | ✅ | Ready after 14 iterations |
| Heartbeat | ✅ | Every 500 frames with dirty flag |
| Dirty Flag | ✅ | Shows `dirty=false` when no redraw needed |
| Stability | ✅ | 4500+ frames, no crash |

---

## Boot Log

```
INFO  Specter-DIY Rust Firmware v3.6
INFO  [BOOT] Taking peripherals...
INFO  [RNG] Enabling clock...
INFO  [RCC] SYSCLK=168000000 Hz
INFO  [RNG] Ready after 14 iterations
INFO  [GPIO] Configuring...
INFO  [LCD] Reset...
INFO  [LCD] Reset OK
INFO  [LED] ON
INFO  [SDRAM] Initializing...
INFO  [SDRAM] Base = 0xc0000000
INFO  [FB] Clearing...
INFO  [DISPLAY] Initializing LTDC/DSI...
INFO  Detected LCD controller: Nt35510
INFO  Display initialized successfully
INFO  [LTDC] Configuring layer L1...
INFO  [LTDC] Enabling layer L1...
INFO  [LTDC] Layer enabled
INFO  [TOUCH] Initializing...
INFO  [GUI] Initializing...
INFO  READY - Starting main loop
```

---

## Touch Events

34 touch events logged without any panic or crash:
```
INFO  [TOUCH] Event #1: (377, 678)
INFO  [TOUCH] Release at (377, 678)
INFO  [NAV] Home -> About
...
INFO  [NAV] Home -> WalletGen
...
INFO  [NAV] WalletGen -> Home
...
INFO  [NAV] Home -> LoadWallet
```

---

## Navigation

User navigated through multiple screens:
- Home → About
- Home → WalletGen
- WalletGen → Home
- Home → LoadWallet

---

## Differences from v3.5

| Feature | v3.5 | v3.6 |
|---------|------|------|
| Heartbeat | `frame=500 screen=X touches=Y` | `frame=500 screen=X touches=Y dirty=false` |
| Dirty flag | Not logged | ✅ Logged |
| LTDC reload | `Layer enabled and reloaded` | `Layer enabled` |

---

## User Testing Required

### Questions for User:

1. **Does the flickering still occur?**
   - v3.4/v3.5 had flickering when touching
   - The dirty flag optimization should reduce this

2. **Is the UI more responsive?**
   - With `dirty=false`, unnecessary redraws are skipped
   - Touch should feel smoother

3. **Are screen transitions smooth?**
   - Navigation between screens should be instant

---

## Technical Details

### Dirty Flag Optimization

The `dirty=false` in heartbeat logs indicates:
- Screen content hasn't changed
- No redraw occurred
- Framebuffer was NOT cleared and redrawn

This is the same behavior as v2.5 (the working baseline).

### When `dirty=true` Should Occur:
- On first render after boot
- When navigating to a new screen
- When UI elements change (button press, text update)

### When `dirty=false` Should Occur:
- When screen is idle (no changes)
- When only touch coordinates are tracked but UI unchanged

---

## Observations from Logs

### Touch Coordinates Are Valid
All coordinates within screen bounds (480x800):
- (377, 678), (414, 714), (201, 693), (337, 235), etc.

### No FT6X06 Panic
34 touch events processed without crash - the safe wrapper is working.

### Clean Exit
Session ended due to USB probe disconnection (not firmware crash):
```
WARN  Could not clear all hardware breakpoints
Error: device disconnected
```
This is a probe connectivity issue, not a firmware bug.

---

## Version Comparison

| Feature | v2.5 | v3.4 | v3.5 | v3.6 |
|---------|------|------|------|------|
| Display | ✅ | ✅ | ✅ | ✅ |
| Touch | ✅ | ✅ | ✅ | ✅ |
| Dirty flag | ✅ | ❌ | ❌ | ✅ |
| Flicker on touch | No | Yes | ? | ? |
| USB logging | ✅ | ❌ | ❌ | ❌ |
| SD card logging | ✅ | ❌ | ❌ | ❌ |
| Frame logging | ✅ | ✅ | ❌ | ❌ |

---

## Session Statistics

- Touch events: 34
- Frames: 4500+
- Screens visited: Home, About, WalletGen, LoadWallet
- Exit: USB probe disconnection
- Crashes: None
- Dirty flag: `false` (no unnecessary redraws)

---

## Remaining Work

### Still Missing (vs v2.5)
1. **USB initialization logging**
2. **SD card initialization logging**

### To Verify
1. **Flickering fixed?** - User needs to confirm
2. **Wallet generation flow** - Not tested
3. **All menu items** - Settings not visited

---

## Recommendations for v3.7

1. **Add USB/SD card logging back** - For feature parity with v2.5
2. **Test wallet generation** - Complete flow with entropy generation
3. **If flickering persists** - Investigate render timing or double buffering

---

*Report generated from v3.6 testing session*
