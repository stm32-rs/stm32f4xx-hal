# Feedback for Specter-DIY Firmware v4.5

## Summary

⚠️ **PARTIALLY WORKING** - Display shows briefly then freezes. Touch holding "wakes" it with flickering. Likely entering low-power mode or WFI loop without proper display refresh.

✅ **WORKING** - Black screen fix applied! All core hardware working with proper LTDC layer enable + reload. Backlight enabled.

---

## What Works (✅)

### 1. RNG - Fully Working
```
INFO  Clocks configured:
INFO    SYSCLK: 168000000 Hz
INFO    HCLK: 168000000 Hz
INFO    PLL48CLK: 48000000 Hz
INFO  RNG test: 0xcf4f748b
INFO  Random seed: [147, 191, 182, 223, 114, 230, 219, 229]
```
- PLL48CLK at 48 MHz (correct)
- RNG generates valid random values
- No timeout or panic

### 2. Device Signature
```
INFO  Device ID: Q105514 (x=60, y=55)
INFO  Flash size: 2048 KB
INFO  Board: STM32F469I-DISCO
INFO  MCU: STM32F469NI
```
- Unique device ID read
- Correct board detection

### 3. SDRAM
```
INFO  SDRAM at 0xc0000000
```
- Memory initialized correctly

### 4. Display - FIXED! 🎉
```
INFO  Test pattern drawn (BLUE background)
INFO  Layer L1 configured
INFO  Layer L1 enabled
INFO  LTDC reload complete
INFO  Display layer configured and ready
INFO  - Display ON + Backlight enabled
```

**What was fixed (vs v4.3 black screen):**
- ✅ Layer explicitly enabled (`Layer L1 enabled`)
- ✅ LTDC reload called (`LTDC reload complete`)
- ✅ Test pattern explicitly mentions color (`BLUE background`)
- ✅ Backlight enabled (`Display ON + Backlight enabled`)
- ✅ Framebuffer address logged (`0xc0000000`)
- ✅ Framebuffer size logged (`768000 bytes`)

### 5. Touch (FT6X06)
```
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
```
- Touch initialized without panic
- No multi-touch crash

### 6. Debug Info
- ✅ All logs show proper file:line locations
- ✅ No `<invalid location>` errors

---

## 🔴 CRITICAL ISSUE: Display Freezes After Boot (User Report)

**User Observation:**
> "After the blue background shows for a second, it sort of stops again. Only when I hold down the screen does it start loading. And it flickers. When I release the touch it stops again."

**Symptoms:**
1. Blue background appears briefly at boot
2. Display freezes/stops updating
3. Holding touch on screen causes it to "wake up" and continue
4. Flickering occurs during touch-hold
5. Releasing touch causes it to stop again

**Root Cause Analysis:**

Most likely causes (in order of probability):

### 1. WFI (Wait For Interrupt) Without Display Refresh
The main loop may be using `wfi()` to save power, but this halts the CPU without refreshing the display:
```rust
loop {
    wfi();  // CPU sleeps until interrupt
    // Display not refreshing during sleep!
}
```

Touch interrupt wakes the CPU, causing display to update momentarily.

**Fix:** Either remove `wfi()` or ensure display refresh happens independently (DMA/DSI video mode).

### 2. DSI Command Mode vs Video Mode
If DSI is in command mode, display needs continuous refresh commands. In video mode, it refreshes automatically.

**Fix:** Ensure DSI is configured for video mode with continuous refresh.

### 3. LTDC Not Continuously Scanning
LTDC may be configured for single-frame rather than continuous scanning.

**Fix:** Ensure LTDC is in continuous scan mode.

### 4. Framebuffer Not Being Updated
Main loop may be idle, not writing new frames to framebuffer.

**Fix:** Add continuous frame updates or animation in main loop.

**Recommended Fix:**
```rust
loop {
    // Option 1: Remove wfi() entirely for now
    // wfi();  // REMOVE THIS
    
    // Option 2: Poll touch and update display continuously
    if let Some(touch) = touch.read() {
        // Handle touch
    }
    
    // Always refresh something to keep display alive
    // Or configure DSI for video mode auto-refresh
}
```

---

## What Doesn't Work / Not Yet Implemented (⚠️)

### 1. DSI Read Errors (Non-Critical)
```
WARN  NT35510 probe attempt 1 failed: DSI read error
WARN  NT35510 probe attempt 2 failed: DSI read error
WARN  NT35510 probe attempt 3 failed: DSI read error
```
- Same as all previous versions
- Falls back to NT35510 (works fine)
- Low priority fix

### 2. Missing Features (Not Yet Implemented)
- ❌ No USB serial
- ❌ No SD card
- ❌ No wallet functionality
- ❌ No GUI

---

## Boot Log

```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v4.5
INFO  Black Screen Fix + Industry Bitcoin Crates
INFO  ========================================
INFO  Configuring clocks with PLL48CLK for RNG...
INFO  Clocks configured:
INFO    SYSCLK: 168000000 Hz
INFO    HCLK: 168000000 Hz
INFO    PLL48CLK: 48000000 Hz
INFO  Initializing Hardware RNG...
INFO  RNG test: 0xcf4f748b
INFO  Random seed: [147, 191, 182, 223, 114, 230, 219, 229]
INFO  Reading device signature...
INFO  Device ID: Q105514 (x=60, y=55)
INFO  Flash size: 2048 KB
INFO  Board: STM32F469I-DISCO
INFO  MCU: STM32F469NI
INFO  Display: 480x800
INFO  LCD reset done
INFO  LED initialized
INFO  Initializing SDRAM...
INFO  SDRAM at 0xc0000000
INFO  Drawing test pattern...
INFO  Test pattern drawn (BLUE background)
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
INFO  Framebuffer address: 0xc0000000
INFO  Framebuffer size: 768000 bytes (480x800 @ 2 bytes/pixel)
INFO  Layer L1 configured
INFO  Layer L1 enabled
INFO  LTDC reload complete
INFO  Display layer configured and ready
INFO  Initializing I2C1 for touch...
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
INFO  ========================================
INFO  v4.5 Ready!
INFO  - Display ON + Backlight enabled
INFO  - Industry-standard Bitcoin crates
INFO  - Touch: Active
INFO  ========================================
```

---

## Version Comparison

| Feature | v4.3 | v4.5 |
|---------|------|------|
| RNG | ✅ | ✅ |
| Display Init | ❌ Black screen | ✅ Fixed |
| Display Runtime | N/A | ⚠️ Freezes without touch |
| Touch | ✅ | ✅ (wakes display) |
| Flicker | N/A | ⚠️ Yes |
| Layer Enabled | ❌ Missing | ✅ Explicit |
| LTDC Reload | ❌ Missing | ✅ Complete |
| Backlight | ❌ Unknown | ✅ Enabled |
| Layer Enabled | ❌ Missing | ✅ Explicit |
| LTDC Reload | ❌ Missing | ✅ Complete |
| Backlight | ❌ Unknown | ✅ Enabled |
| Debug Info | ✅ | ✅ |
| USB | ❌ | ❌ |
| Wallet | ❌ | ❌ |

---

## Test Status

| Component | Status | Notes |
|-----------|--------|-------|
| Boot | ✅ | Clean |
| Clock Config | ✅ | PLL48CLK 48 MHz |
| RNG | ✅ | Working |
| Device ID | ✅ | Read correctly |
| SDRAM | ✅ | 0xc0000000 |
| Display Init | ✅ | LTDC enabled + reload |
| Display Runtime | ⚠️ | **FREEZES - needs wfi() fix** |
| Flicker | ⚠️ | Yes during touch-hold |
| Backlight | ✅ | Enabled |
| Touch | ✅ | FT6X06 working (wakes display) |
| USB | ❌ | Not implemented |
| SD Card | ❌ | Not implemented |
| Wallet | ❌ | Not implemented |

---

## Key Fixes Applied in v4.5

### Black Screen Fix
The v4.3 black screen was caused by missing LTDC layer enable. v4.5 adds:

1. **Layer Enable**: `Layer L1 enabled`
2. **LTDC Reload**: `LTDC reload complete` - applies configuration to hardware
3. **Backlight**: Explicitly enabled
4. **Debug Logging**: Framebuffer address and size logged

This matches the fix pattern documented in `black_screen_fix.md`.

### Display Initialization Sequence
```
INFO  Test pattern drawn (BLUE background)  ← Draw to framebuffer
INFO  Layer L1 configured                    ← Configure layer
INFO  Layer L1 enabled                       ← ENABLE layer (was missing)
INFO  LTDC reload complete                   ← APPLY changes (was missing)
INFO  Display layer configured and ready     ← Verified
```

---

## Session Statistics

- **Exit**: Clean (SIGTERM by user)
- **Crashes**: None
- **Touch events**: Not logged (no interaction during session)
- **Boot completion**: ✅ Reached "v4.5 Ready!"

---

## Next Steps

1. **USB Serial** - Add USB communication
2. **SD Card** - Add storage support
3. **GUI Framework** - Port from v2.5 or implement new
4. **Wallet Features** - BIP39/BIP32 implementation
5. **Touch Event Logging** - Log touch coordinates in main loop

---

## Summary

**v4.5 has a critical display freeze issue.**

The black screen from v4.3 is fixed (LTDC layer enabled + reload), but the display now freezes after boot. The main loop appears to use `wfi()` or similar power-saving that halts display refresh. Touch temporarily "wakes" the display with flickering.

**Priority Fix:** Remove `wfi()` from main loop or configure DSI for video mode auto-refresh.

**Status**: ⚠️ DISPLAY INIT WORKS BUT FREEZES - NEEDS MAIN LOOP FIX
