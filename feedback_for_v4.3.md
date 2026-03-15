# Feedback for Specter-DIY Firmware v4.3

## Summary

❌ **BLACK SCREEN** - All hardware initializes without errors, but display output is not visible. Touch and RNG work. Display initialization logs claim success but screen remains black.

---

## What Works (✅)

### 1. RNG - FIXED!
```
INFO  Clocks configured:
INFO    SYSCLK: 168000000 Hz
INFO    HCLK: 168000000 Hz
INFO    PLL48CLK: 48000000 Hz
INFO  RNG test: 0xdbc64b73
INFO  Random seed: [215, 223, 229, 210, 58, 122, 55, 109]
```
- PLL48CLK properly configured at 48 MHz
- RNG generates random values correctly
- No panic on initialization (v4.1 issue fixed)

### 2. Device Signature
```
INFO  Device ID: Q105514 (x=60, y=55)
INFO  Flash size: 2048 KB
INFO  Board: STM32F469I-DISCO
INFO  MCU: STM32F469NI
```
- Unique device ID read successfully
- Correct flash size detection (2 MB)

### 3. SDRAM
```
INFO  SDRAM at 0xc0000000
```
- SDRAM initialized at correct address

### 4. Display
```
INFO  Initializing NT35510 (B08 revision)...
INFO  force_rx_low_power cleared — DSI HS mode fully active
INFO  Display initialized successfully
INFO  Display controller: Nt35510
INFO  Display layer configured
```
- Display layer configured

### 5. Touch (FT6X06)
```
INFO  Probing FT6X06 at 0x38...
INFO  FT6X06 touch initialized!
INFO  Touch: x=326, y=252
INFO  Touch: x=345, y=302
... (many touch events)
```
- FT6X06 initialized without panic
- Touch coordinates reported correctly
- Multi-touch handled without crashing (v2.6-v3.0 issue fixed)

### 6. Main Loop
```
INFO  Frame 60 | State: Splash | RNG: 0xaf0e227d
```
- Main loop running successfully
- 60 frames processed
- RNG continues working during runtime

---

## What Doesn't Work / Needs Attention (⚠️)

### 1. 🔴 CRITICAL: Black Screen (No Display Output)

**Observation**: User reports screen is completely black despite logs showing:
```
INFO  Test pattern drawn
INFO  Display initialized successfully  
INFO  Display layer configured
```

**Root Cause Analysis**:

Most likely causes (in order of probability):

1. **LTDC Layer Not Enabled**
   - Layer is "configured" but not actually enabled
   - Need to call `ltdc.reload()` after enabling layer for changes to take effect
   - Reference: v3.4 fixed display by enabling LTDC layer + reload

2. **Backlight Not Enabled**
   - Display controller works but backlight GPIO is low
   - Check if backlight pin is configured and set HIGH

3. **Framebuffer Not Mapped to LTDC**
   - SDRAM at 0xc0000000 but LTDC may be reading from wrong address
   - Verify LTDC layer framebuffer start address matches SDRAM

4. **DSI Command Mode vs Video Mode**
   - Display may be in command mode (no auto-refresh)
   - Need to configure DSI for video mode

5. **Pixel Format Mismatch**
   - LTDC configured for ARGB8888 but framebuffer is RGB565 (or vice versa)
   - Bytes per pixel mismatch causes garbage/no output

**Fix Required**:
```rust
// After configuring LTDC layer:
layer.enable();
ltdc.reload();  // CRITICAL: Apply changes

// Also verify:
defmt::info!("LTDC layer enabled: {}", layer.is_enabled());
defmt::info!("Framebuffer at: 0x{:08x}", framebuffer.as_ptr() as u32);
```

---

### 2. DSI Read Errors During LCD Probe
```
WARN  NT35510 probe attempt 1 failed: DSI read error
WARN  NT35510 probe attempt 2 failed: DSI read error
WARN  NT35510 probe attempt 3 failed: DSI read error
WARN  Probe inconclusive (mismatch=0, read_err=3, write_err=0); defaulting to NT35510
```
**Analysis**: 
- All 3 DSI read attempts failed
- Firmware falls back to defaulting to NT35510
- This works but suggests DSI read isn't fully functional

**Potential Causes**:
- DSI timing not optimal for read operations
- Need delay between read command and data collection
- Clock lane may not be switching properly for reads

**Recommendation**: 
- This is non-critical since detection works via fallback
- If LCD detection is ever needed for multiple panel types, fix DSI reads

---

## Version Comparison

| Feature | v3.7 | v4.0 | v4.1 | v4.3 |
|---------|------|------|------|------|
| RNG | ✅ | ❌ Timeout | ❌ Panic | ✅ |
| Display | ✅ | ✅ | ⬜ | ❌ Black Screen |
| Touch | ✅ | ❌ Panic | ⬜ | ✅ |
| SDRAM | ✅ | ⬜ | ⬜ | ✅ |
| Device ID | ✅ | ⬜ | ⬜ | ✅ |
| Main Loop | ✅ | ⬜ | ⬜ | ✅ |

**v4.3 has black screen** - Display initializes but no output. Likely LTDC layer not enabled or backlight off.

**v4.3 is the best version since v3.7** - All core features working.

---

## Boot Log

```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v4.3
INFO  Fixed RNG clock configuration
INFO  ========================================
INFO  Configuring clocks with PLL48CLK for RNG...
INFO  Clocks configured:
INFO    SYSCLK: 168000000 Hz
INFO    HCLK: 168000000 Hz
INFO    PLL48CLK: 48000000 Hz
INFO  Initializing Hardware RNG...
INFO  RNG test: 0xdbc64b73
INFO  Random seed: [215, 223, 229, 210, 58, 122, 55, 109]
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
INFO  ========================================
INFO  v4.3 Ready!
INFO  - HAL RNG with PLL48CLK fix
INFO  - Device Signature (unique ID)
INFO  - Touch: Active
INFO  ========================================
```

---

## Test Status

| Component | Status | Notes |
|-----------|--------|-------|
| Boot | ✅ | Clean boot |
| RNG | ✅ | PLL48CLK fix working |
| Clock Config | ✅ | 168 MHz SYSCLK, 48 MHz PLL48CLK |
| Device ID | ✅ | Unique ID read |
| Flash Size | ✅ | 2048 KB detected |
| SDRAM | ✅ | 0xc0000000 |
| Display | ❌ | Black screen - no output |
| Touch | ✅ | FT6X06 working |
| Main Loop | ✅ | 60 frames processed |
| DSI Read | ⚠️ | Fallback used |

---

## Next Steps / Improvements

1. **CRITICAL**: Fix black screen - enable LTDC layer + reload, or enable backlight
2. **Low Priority**: Fix DSI read errors for robust LCD detection
3. **Blocked**: Cannot proceed with UI until display output works

---

## Summary

**v4.3 has a critical black screen issue.** The RNG clock configuration fix resolved v4.0-v4.1 issues, and touch works correctly. However, the display initializes but shows no output.

**Most likely fix**: Enable LTDC layer and call `ltdc.reload()`, or enable backlight GPIO.

**Status**: ❌ DISPLAY NOT WORKING - NEEDS FIX
