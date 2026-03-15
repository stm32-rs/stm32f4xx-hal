# Feedback for v3.1 - RED SCREEN + FT6X06 PANIC

**Status**: ❌ BROKEN - Red screen persists + FT6X06 panic

**Test Date**: March 2026

---

## ⚠️ CRITICAL: v2.5 WORKS - USE AS REFERENCE

**v2.5 is the ONLY version that works correctly.** The firmware author should:
1. Compare v3.1 code against v2.5
2. Copy the working framebuffer fill implementation from v2.5
3. Copy the working FT6X06 safe wrapper from v2.5
4. Copy the working touch coordinate handling from v2.5

| Feature | v2.5 | v3.1 | Fix |
|---------|------|------|-----|
| Screen Color | ✅ Black | ❌ Red | **Copy from v2.5** |
| Touch | ✅ Works | ❌ Panic | **Copy FT6X06 wrapper from v2.5** |
| Touch Coords | ✅ Correct (e.g., 309, 417) | ❌ (4095, 3840) | **Copy from v2.5** |
| Frame Rendering | ✅ Home screen visible | ❌ Nothing visible | **Copy from v2.5** |
| Test Pattern | ✅ N/A (not needed) | ❌ Not visible | Remove or fix |

---

## User Observations

> "board is still red and even after multiple touches i never see the home screen rendered. i did not see a test pattern."
> 
> "version 2.5 works"

---

## What Works

| Feature | Status | Notes |
|---------|--------|-------|
| Boot sequence | ✅ | Completes successfully |
| RNG | ✅ | Software entropy works |
| Display init | ✅ | NT35510 detected |
| Touch init | ✅ | FT6X06 initialized |
| Frame rendering | ✅ | 176 frames before crash |
| No crash without touch | ✅ | Timeout exit, not panic (when not touched) |
| debug = 2 | ✅ | Proper file:line in logs |

## What's Broken

### 1. RED SCREEN (CRITICAL - NOT FIXED)

**Log claims black, but screen is red:**
```
INFO  [SDRAM] Clearing framebuffer to BLACK (0x0000)...
INFO  [FILL] Setting all 384000 pixels to 0x0000
INFO  [SDRAM] Framebuffer ready, first pixel = 0x0000
...
INFO  [DISPLAY] Drawing test pattern to framebuffer...
INFO  [TEST] Drawing test pattern...
INFO  [TEST] Test pattern complete - first pixel = 0xf800
...
INFO  [DISPLAY] Clearing to black for GUI...
INFO  [FILL] Setting all 384000 pixels to 0x0000
```

**Analysis:**
- The `fill_color` function logs that it's setting pixels to 0x0000
- But user sees RED screen
- Test pattern draws with 0xf800 (red in RGB565)
- The subsequent "clear to black" doesn't actually clear

**Possible causes:**
1. **fill_color function broken** - Not actually writing to memory
2. **Wrong memory address** - Writing to wrong location
3. **Cache issue** - Writes not flushed to SDRAM
4. **Display reading wrong buffer** - LTDC pointing to wrong address
5. **Compiler optimization** - Loop being optimized away

**Recommended debugging:**
```rust
// Add verification after fill:
let first_10: [u16; 10] = core::array::from_fn(|i| framebuffer[i]);
info!("First 10 pixels after fill: {:?}", first_10);

// Also try volatile writes:
for pixel in framebuffer.iter_mut() {
    core::ptr::write_volatile(pixel, 0x0000);
}
```

### 2. FT6X06 Multi-Touch Panic (NOT FIXED)

**Still crashes on touch:**
```
INFO  [TOUCH] Event #1: 1 finger(s) at (4095, 3840)
...
INFO  [FRAME] #176 - Rendering screen Home
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

**Analysis:**
- v3.1 does NOT include the FT6X06 safe wrapper from v2.5
- Same panic as v2.6, v2.7, v2.9, v3.0
- Touch coordinates still wrong: `(4095, 3840)`

### 3. Wrong Touch Coordinates (NOT FIXED)

```
INFO  [TOUCH] Event #1: 1 finger(s) at (4095, 3840)
```

- 4095 = 12-bit max value (0xFFF)
- 3840 = Not a standard max
- Indicates raw/unprocessed touch data

---

## Boot Log

```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v3.1
INFO  ========================================
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] Configuring clocks...
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [RNG] Using software entropy
INFO  [RNG] Entropy seed initialized
INFO  [GPIO] Configuring ports...
INFO  [GPIO] Ports configured
INFO  [LCD] Reset sequence...
INFO  [LCD] Reset complete
INFO  [LED] LED initialized
INFO  [SDRAM] Initializing...
INFO  [SDRAM] Configuring FMC...
INFO  [SDRAM] Initializing memory...
INFO  [SDRAM] Initialized at 0xc0000000
INFO  [SDRAM] Clearing framebuffer to BLACK (0x0000)...
INFO  [FILL] Setting all 384000 pixels to 0x0000
INFO  [SDRAM] Framebuffer ready, first pixel = 0x0000
INFO  [DISPLAY] Initializing LTDC/DSI...
INFO  Initializing DSI...
WARN  NT35510 probe attempt 1 failed: DSI read error
WARN  NT35510 probe attempt 2 failed: DSI read error
WARN  NT35510 probe attempt 3 failed: DSI read error
WARN  Probe inconclusive (mismatch=0, read_err=3, write_err=0); defaulting to NT35510
INFO  Detected LCD controller: Nt35510
INFO  Initializing NT35510 (B08 revision)...
INFO  force_rx_low_power cleared — DSI HS mode fully active
INFO  Display initialized successfully
INFO  [DISPLAY] Controller: Nt35510
INFO  [DISPLAY] Drawing test pattern to framebuffer...
INFO  [TEST] Drawing test pattern...
INFO  [TEST] Test pattern complete - first pixel = 0xf800
INFO  [DISPLAY] Configuring layer...
INFO  [DISPLAY] Layer configured, test pattern should be visible
INFO  [DISPLAY] Waiting 2 seconds for test pattern observation...
INFO  [DISPLAY] Clearing to black for GUI...
INFO  [FILL] Setting all 384000 pixels to 0x0000
INFO  [TOUCH] Initializing FT6X06...
INFO  [TOUCH] FT6X06 initialized OK
INFO  [GUI] Initializing screen manager...
INFO  [GUI] Screen manager ready
INFO  ========================================
INFO  Ready! Touch screen to interact
INFO  ========================================
INFO  [FRAME] #0 - Rendering screen Home
INFO  [FRAME] #0 - Render complete
INFO  [TOUCH] Event #1: 1 finger(s) at (4095, 3840)
...
INFO  [FRAME] #176 - Rendering screen Home
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

---

## Test Pattern Debug

v3.1 added test pattern code:
```
INFO  [DISPLAY] Drawing test pattern to framebuffer...
INFO  [TEST] Drawing test pattern...
INFO  [TEST] Test pattern complete - first pixel = 0xf800
INFO  [DISPLAY] Waiting 2 seconds for test pattern observation...
```

**Question for user:** Did you see the test pattern during the 2-second wait? What did it look like?

---

## Fixes Required

### Fix 1: fill_color Function (CRITICAL)

The fill_color function is NOT actually writing to the framebuffer. Try:

```rust
// Option A: Use volatile writes
pub fn fill_color(framebuffer: &mut [u16], color: u16) {
    info!("[FILL] Setting all {} pixels to 0x{:04x}", framebuffer.len(), color);
    for (i, pixel) in framebuffer.iter_mut().enumerate() {
        core::ptr::write_volatile(pixel, color);
    }
    
    // Verify
    let first = unsafe { core::ptr::read_volatile(framebuffer.as_ptr()) };
    info!("[FILL] Verification - first pixel = 0x{:04x}", first);
}

// Option B: Use memset-like approach
pub fn fill_color(framebuffer: &mut [u16], color: u16) {
    let ptr = framebuffer.as_mut_ptr();
    let len = framebuffer.len();
    unsafe {
        core::ptr::write_bytes(ptr, color as u8, len * 2);
    }
}
```

### Fix 2: FT6X06 Safe Wrapper (CRITICAL)

**MUST copy from v2.5** - see `feedback_for_v2.5.md` for reference.

### Fix 3: Touch Coordinate Mapping

Investigate why coordinates are (4095, 3840):
- Check I2C read function
- Verify coordinate transform
- Compare with v2.5 touch handling

---

## Version Comparison

| Feature | v2.5 | v3.1 |
|---------|------|------|
| Screen Color | ✅ Black | ❌ Red (log says black) |
| fill_color | ✅ Works | ❌ Broken |
| Touch | ✅ Works | ❌ Panic |
| Touch Coords | ✅ Correct | ❌ (4095, 3840) |
| FT6X06 Safe | ✅ Yes | ❌ No |
| Boot | ✅ | ✅ |
| RNG | ✅ HW | ✅ SW |

---

## Session Statistics

- Frames rendered: 176
- Touch events: 1 logged
- Exit reason: FT6X06 panic
- Test pattern: Added but user sees red

---

## Action Items for Next Version

### REQUIRED: Copy Working Code from v2.5

The v2.5 implementation works. Do NOT try to debug the v3.1 code - instead:

1. **[CRITICAL] Framebuffer Fill**
   - Open v2.5 source code
   - Find the framebuffer initialization/clear code
   - Copy that exact implementation to v3.2
   - v2.5 produces BLACK screen, v3.1 produces RED screen

2. **[CRITICAL] FT6X06 Touch Handler**
   - Open v2.5 source code
   - Find the FT6X06 touch detection code
   - Copy the safe wrapper that handles multi-touch
   - v2.5 does NOT panic on touch, v3.1 DOES panic

3. **[CRITICAL] Touch Coordinate Mapping**
   - Open v2.5 source code
   - Find the touch coordinate handling
   - v2.5 reports correct coords like (309, 417)
   - v3.1 reports wrong coords (4095, 3840)

4. **[OPTIONAL] Remove Test Pattern**
   - Test pattern not visible anyway
   - Adds complexity without benefit
   - v2.5 doesn't have test pattern and works fine

### Do NOT:
- Do NOT try to fix fill_color with volatile writes - just copy v2.5's approach
- Do NOT try to add timeout to FT6X06 - just copy v2.5's safe wrapper
- Do NOT add more debugging - v2.5 already works, copy it

---

## Files to Reference

- `feedback_for_v2.5.md` - Documents what works in v2.5
- `testing_report_v2.5_to_v3.0.md` - Full comparison of all versions
- v2.5 source code - The reference implementation

---

*Feedback generated from live hardware testing*
