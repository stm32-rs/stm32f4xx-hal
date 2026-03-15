# Combined Feedback for v3.1 and v3.2 - DISPLAY NOT WORKING

**Status**: ❌ BROKEN - Screen stays RED, no UI visible

**Test Date**: March 2026

---

## ⚠️ CRITICAL: v2.5 WORKS - USE AS REFERENCE

**v2.5 is the ONLY version that displays the UI correctly.**

| Feature | v2.5 | v3.1 | v3.2 | Fix |
|---------|------|------|------|-----|
| Screen Display | ✅ Black + UI visible | ❌ Red, no UI | ❌ Red, no UI | **Compare with v2.5** |
| Touch Handler | ✅ Works | ❌ Panic | ✅ Works (91 events) | ✅ Fixed in v3.2 |
| Touch Coords | ✅ Correct | ❌ (4095, 3840) | ✅ Correct | ✅ Fixed in v3.2 |
| RNG | ✅ HW Works | ✅ SW fallback | ✅ HW Works | ✅ Fixed in v3.2 |
| FT6X06 Panic | ✅ No panic | ❌ Panics | ✅ No panic | ✅ Fixed in v3.2 |
| SYSCLK | 168MHz | 180MHz | 168MHz | ✅ Fixed in v3.2 |
| Heartbeat | ✅ Works | ❌ Missing | ✅ Works | ✅ Fixed in v3.2 |

---

## Summary: What's Fixed vs Still Broken

### ✅ FIXED in v3.2 (Keep These Changes)
1. **FT6X06 Touch Handler** - No more panic, 91 touch events logged
2. **Touch Coordinates** - Now reports real values like (402, 291) instead of (4095, 3840)
3. **Hardware RNG** - Works again: `[RNG] Test OK: 0x3377aad3`
4. **SYSCLK** - Back to 168MHz (same as v2.5)
5. **Heartbeat** - Every 500 frames
6. **Navigation** - `[NAV] -> SignTx` logged correctly
7. **LED** - Lights on touch, off on release

### ❌ STILL BROKEN (Priority Fix for v3.3)
1. **RED SCREEN** - Screen stays red, no UI visible
2. **No Test Pattern Visible** - 2-second test pattern not visible
3. **No Home Screen** - Cannot see menus or UI elements

---

## User Observations

### v3.1
> "board is still red and even after multiple touches i never see the home screen rendered. i did not see a test pattern."
> "version 2.5 works"

### v3.2
> "screen is still red. can not see home screen or menus. could not see any test pattern during the 2-second wait."
> "led lights up when i touch and turns off when i let go."

---

## v3.2 Boot Log (The Working Parts)

```
INFO  Specter-DIY Rust Firmware v3.2
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RNG] Enabling RNG clock in AHB2ENR (before freeze)...
INFO  [RNG] AHB2ENR = 0x00000040
INFO  [RCC] Configuring clocks...
INFO  [RCC] SYSCLK=168000000 Hz
INFO  [RNG] Enabling RNG peripheral...
INFO  [RNG] Ready after 16 iterations
INFO  [RNG] Test OK: 0x3377aad3
INFO  [GPIO] Configuring ports...
INFO  [GPIO] OK
INFO  [LCD] Reset sequence...
INFO  [LCD] Reset OK
INFO  [LED] ON
INFO  [SDRAM] Initialization...
INFO  [SDRAM] ptr=0xc0000000
INFO  [FB] Clearing framebuffer to BLACK (0x0000)...
INFO  [FB] First pixel = 0x0000
INFO  [DISPLAY] LCD init...
INFO  Initializing DSI...
INFO  Detected LCD controller: Nt35510
INFO  Initializing NT35510 (B08 revision)...
INFO  Display initialized successfully
INFO  [TEST] Drawing test pattern...
INFO  [TEST] Test pattern complete - first pixel = 0xf800
INFO  [DISPLAY] Layer configured - test pattern should be visible
INFO  [TEST] Waiting 2 seconds for observation...
INFO  [DISPLAY] Clearing to black for GUI...
INFO  [TOUCH] Init...
INFO  [TOUCH] OK - test read: count=0 x=0 y=0
INFO  [GUI] Initializing screen manager...
INFO  [GUI] Screen manager ready
INFO  INIT COMPLETE - Starting UI
INFO  [FRAME] #1 - Rendering screen Home
INFO  [FRAME] #1 - Render complete
INFO  [HEARTBEAT] frame=500 screen=Home touches=0
INFO  [TOUCH] Event #1: (402, 291)
INFO  [TOUCH] Release at (402, 291)
INFO  [NAV] -> SignTx
...
INFO  [HEARTBEAT] frame=6000 screen=SignTx touches=91
```

---

## Root Cause Analysis: RED SCREEN

### The Problem
- Logs claim framebuffer is cleared to BLACK (0x0000)
- Logs claim test pattern is drawn
- Logs claim GUI renders
- **BUT screen shows RED**

### What This Means
The framebuffer writes are happening (code runs without error), but the display is NOT showing what's in the framebuffer. This is likely a **display configuration issue**, not a framebuffer write issue.

### Possible Causes (In Priority Order)

1. **LTDC Layer Framebuffer Address Wrong**
   - LTDC may be reading from a different address than where framebuffer is written
   - Check: Does LTDC layer point to 0xc0000000?
   - Compare: v2.5 LTDC configuration vs v3.2

2. **Pixel Format Mismatch**
   - Framebuffer written in one format, LTDC expects another
   - v2.5 uses RGB565 (16-bit)
   - Check: Is v3.2 using the same pixel format?

3. **Display Controller Register Issue**
   - NT35510 may have a register that affects display color
   - Some displays have a "fill color" register independent of framebuffer

4. **Framebuffer Not Being Flushed**
   - Writes to SDRAM may be cached and not reaching display
   - v2.5 may have cache flushing that v3.2 is missing

5. **Layer Not Enabled**
   - LTDC layer may not be properly enabled
   - Display falls back to some default color

---

## Priority Fixes for v3.3

### 🔴 CRITICAL: Fix Display Output

**DO NOT debug the framebuffer fill function** - it's writing correctly (first pixel = 0x0000 confirmed in logs).

**DO compare the LTDC/DSI configuration with v2.5:**

```rust
// Check these in v2.5 vs v3.2:
// 1. LTDC layer framebuffer address (should be 0xc0000000)
// 2. LTDC pixel format (should be RGB565 / L8 based on config)
// 3. LTDC layer enable status
// 4. DSI configuration
// 5. Any cache flush operations after framebuffer writes
```

### What to Compare Line-by-Line with v2.5

1. **LTDC Layer Configuration**
   - Look at the LTDC layer setup code
   - Compare framebuffer address, pixel format, window size
   - Check if layer is enabled

2. **DSI Initialization**
   - Compare DSI init sequence
   - Check for any differences in commands sent to NT35510

3. **Framebuffer Pointer**
   - Verify the same address (0xc0000000) is used everywhere
   - Check that LTDC is reading from this address

4. **Any `unsafe` blocks or volatile operations**
   - v2.5 may have memory barriers or volatile writes
   - These ensure writes reach the display

### Code to Add for Debugging

```rust
// After LTDC layer config, verify:
let layer1_ba = LTDC.layer1[0].read().bits();
info!("[LTDC] Layer 1 buffer address = 0x{:08x}", layer1_ba);

// Verify pixel format
let pf = LTDC.layer1[0].pf.read().bits();
info!("[LTDC] Pixel format = 0x{:02x}", pf);

// Verify layer is enabled
let cr = LTDC.layer1[0].cr.read().bits();
info!("[LTDC] Layer CR = 0x{:08x} (enabled={})", cr, cr & 1);
```

---

## What NOT to Change for v3.3

### Keep These v3.2 Implementations
- ✅ FT6X06 touch handler (no panic)
- ✅ Touch coordinate handling
- ✅ Hardware RNG initialization
- ✅ SYSCLK at 168MHz
- ✅ Heartbeat logging
- ✅ Navigation logging
- ✅ LED control

---

## Test Pattern Observation

Both v3.1 and v3.2 have test pattern code:
```
INFO  [TEST] Drawing test pattern...
INFO  [TEST] Test pattern complete - first pixel = 0xf800
INFO  [TEST] Waiting 2 seconds for observation...
```

**User cannot see the test pattern.** This confirms the issue is display output, not GUI rendering.

If the test pattern (which sets first pixel to 0xf800 = RED) was visible, user would see something during the 2-second wait. They see nothing - just red screen the whole time.

---

## Files to Reference

- `feedback_for_v2.5.md` - Documents the working v2.5 implementation
- `testing_report_v2.5_to_v3.0.md` - Full version comparison
- **v2.5 source code** - The reference implementation that produces BLACK screen and visible UI

---

## Action Checklist for v3.3

- [ ] Compare LTDC layer configuration (address, format, enable) with v2.5
- [ ] Compare DSI initialization with v2.5
- [ ] Add debug logging for LTDC registers
- [ ] Verify framebuffer address used by LTDC matches SDRAM base (0xc0000000)
- [ ] Check for any cache flush/memory barrier operations in v2.5 missing in v3.2
- [ ] DO NOT change touch handler, RNG, or other working code

---

*Combined feedback from v3.1 and v3.2 testing sessions*
