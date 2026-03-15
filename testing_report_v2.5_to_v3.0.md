# Specter-DIY Firmware Testing Report

**Device**: STM32F469NIHx (Specter-DIY Hardware Wallet)  
**Test Method**: Remote flashing via `probe-rs run` with RTT logging  
**Remote Board**: `ubuntu@192.168.13.246`  
**Test Date**: March 2026

---

## Executive Summary

| Version | Status | Screen | Touch | RNG | Frames Before Issue |
|---------|--------|--------|-------|-----|---------------------|
| **v2.5** | ✅ WORKING | Black ✅ | Works ✅ | HW ✅ | 2000+ (no issue) |
| **v2.6** | ❌ BROKEN | N/A | Panic ❌ | N/A | 0 (crash on touch) |
| **v2.7** | ❌ BROKEN | Red ❌ | Panic ❌ | HW ✅ | 0 (crash on touch) |
| **v2.8** | ❌ BROKEN | Blank ❌ | N/A | Hang ❌ | 0 (hangs at RNG) |
| **v2.9** | ❌ BROKEN | Red ❌ | Panic ❌ | SW ✅ | 76 |
| **v3.0** | ❌ BROKEN | Red ❌ | Panic ❌ | SW ✅ | 187 |

**Conclusion**: v2.5 is the only fully working version. All subsequent versions have regressions.

---

## Version Details

---

## v2.5 - WORKING (Baseline)

### Status: ✅ FULLY WORKING

### User Observation
> "2.5 works great"

### Key Features Working
- All screens render correctly (Home, About, Settings, WalletGen)
- Touch works reliably without freezing
- Hardware RNG works
- Wallet generation completes
- Navigation between screens works
- USB connects successfully

### Boot Log
```
INFO  ========================================
INFO  Specter-DIY v2.5
INFO  BIP39/BIP32 + GUI Framework
INFO  ========================================
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] Enabling RNG clock...
INFO  [RCC] Configuring system clocks...
INFO  [RCC] SYSCLK=168000000 Hz
INFO  [GPIO] Configuring ports...
INFO  [GPIO] OK
INFO  [LCD] Reset sequence...
INFO  [LCD] Reset OK
INFO  [LED] Init...
INFO  [LED] ON
INFO  [RNG] Hardware RNG initialization...
INFO  [RNG] new() - enabling RNG clock...
INFO  [RNG] RCC AHB2ENR.RNGEN set
INFO  [RNG] AHB2ENR = 0x00000040
INFO  [RNG] Clock enabled OK
INFO  [RNG] Enabling RNG peripheral...
INFO  [RNG] CR = 0x00000004
INFO  [RNG] RNG peripheral enabled OK
INFO  [RNG] SR = 0x00000001 (DRDY=1 CECS=0 SECS=0)
INFO  [RNG] Initialization complete
INFO  [RNG] Test OK: 0xf53addb8
INFO  [SDRAM] Initialization...
INFO  [SDRAM] ptr=0xc0000000
INFO  [FB] Framebuffer setup...
INFO  [DISPLAY] LCD init...
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
INFO  [DISPLAY] Layer configured
INFO  [TOUCH] Init...
INFO  [TOUCH] OK
INFO  [USB] Init...
INFO  [USB] OK
INFO  [SD] Init...
INFO  [SD] Init complete
INFO  ========================================
INFO  INIT COMPLETE - Starting UI
INFO  ========================================
INFO  [FRAME] #1 - Rendering screen Home
INFO  [FRAME] #1 - Render complete
INFO  [USB] Connected - state=Configured
```

### Touch Events (Working)
```
INFO  [TOUCH] Event #1: 1 finger(s) at (309, 417)
INFO  [TOUCH] screen=(309, 417) fingers=1
INFO  [HOME] touch at (309, 417)
INFO  [HOME] menu item 4 selected: About
INFO  [NAV] -> About
INFO  [FRAME] #294 - Rendering screen About
```

### Wallet Generation (Working)
```
INFO  [WALLET] init: WalletGen screen
INFO  [NAV] -> WalletGen
INFO  [RNG] Generating entropy for wallet...
INFO  [RNG] Entropy generated, passing to wallet screen...
INFO  [WALLET] Generated mnemonic with checksum
INFO  [WALLET] Continue button pressed, advancing page
```

### Key Implementation Details
- **SYSCLK**: 168MHz
- **RNG**: Hardware RNG with proper clock initialization
- **Touch**: Safe wrapper for FT6X06 (no panic on multi-touch)
- **Framebuffer**: Cleared with black (0x0000)
- **Debug**: `debug = 2` enabled (proper file:line in logs)

---

## v2.6 - BROKEN

### Status: ❌ BROKEN - FT6X06 Panic

### User Observation
> (Crashes immediately on first touch)

### Boot Log
```
WARN  Insufficient DWARF info; compile your program with `debug = 2` to enable location info.
INFO  ========================================
INFO  Specter-DIY Rust Firmware v2.6
INFO  ========================================
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
INFO  Display: Nt35510
INFO  Touch initialized!
INFO  ========================================
INFO  Ready! Touch screen to interact
INFO  ========================================
INFO  Touch #1
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### Crash Stack Trace
```
Frame 9: Ft6X06::detect_touch @ ft6x06-0.1.2/src/lib.rs:332:9
Frame 12: __cortex_m_rt_main @ main.rs
```

### Issues Identified
1. **FT6X06 Panic**: Crashes on first touch
2. **Missing debug info**: `<invalid location: defmt frame-index: X>` instead of file:line

### Root Cause
The `ft6x06-0.1.2` crate has an assertion that panics when touch count exceeds `FT6X06_MAX_NB_TOUCH`.

---

## v2.7 - BROKEN

### Status: ❌ BROKEN - Red Screen + FT6X06 Panic

### User Observation
> "2.7 shows red screen. led blinks on touch initially until it eventually crashes."

### Boot Log
```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v2.7
INFO  ========================================
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] Configuring clocks...
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [GPIO] Configuring ports...
INFO  [GPIO] Ports configured
INFO  [LCD] Reset sequence...
INFO  [LCD] Reset complete
INFO  [SDRAM] Initializing...
INFO  [SDRAM] Initialized at 0xc0000000
INFO  [SDRAM] Clearing framebuffer...
INFO  [SDRAM] Framebuffer ready
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
INFO  [TOUCH] Initializing FT6X06...
INFO  [TOUCH] FT6X06 initialized OK
INFO  [DISPLAY] Configuring layer...
INFO  [DISPLAY] Layer configured
INFO  [GUI] Initializing screen manager...
INFO  [GUI] Screen manager ready
INFO  ========================================
INFO  Ready! Touch screen to interact
INFO  ========================================
INFO  [TOUCH] #1 at (4095, 3840)
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### Issues Identified
1. **Red Screen**: Framebuffer cleared with wrong color (0xF800 red instead of 0x0000 black)
2. **FT6X06 Panic**: Crashes on touch
3. **Wrong Touch Coordinates**: `(4095, 3840)` are max/invalid values
4. **Clock Change**: SYSCLK changed from 168MHz → 180MHz

### Changes from v2.5
| Setting | v2.5 | v2.7 |
|---------|------|------|
| SYSCLK | 168MHz | 180MHz |
| Screen Color | Black | Red |
| Touch Coords | Correct | (4095, 3840) |

---

## v2.8 - BROKEN

### Status: ❌ BROKEN - RNG Initialization Hang

### User Observation
> "2.8 shows a blank screen and does not react to touch."

### Boot Log
```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v2.8
INFO  ========================================
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] Configuring clocks...
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [GPIO] Configuring ports...
INFO  [GPIO] Ports configured
INFO  [RNG] Hardware RNG initialization...
(HANGS HERE INDEFINITELY)
```

### Issues Identified
1. **RNG Hang**: Firmware blocks forever at RNG initialization
2. **Blank Screen**: Display never initialized
3. **No Touch**: Never reaches touch initialization

### Root Cause Analysis
The hardware RNG on STM32F469 requires:
1. RNG clock enabled via RCC AHB2ENR.RNGEN
2. RNG peripheral enabled via RNG_CR.RNGEN
3. Wait for RNG_SR.DRDY (data ready) flag

If any step fails, the code blocks forever waiting for DRDY.

### Recommendation
Add timeout to RNG initialization:
```rust
let timeout = 100_000;
for _ in 0..timeout {
    if RNG.sr.read().drdy().bit_is_set() {
        return Ok(Self);
    }
}
Err(RngError::Timeout)
```

---

## v2.9 - BROKEN

### Status: ❌ BROKEN - Red Screen + FT6X06 Panic (RNG Workaround Works)

### User Observation
> "2.9 shows the red screen again. touchs seem to register"

### Boot Log
```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v2.9
INFO  ========================================
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] Configuring clocks...
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [RNG] Using software entropy (HW RNG needs clock fix)
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
INFO  [SDRAM] Clearing framebuffer (384000 pixels)...
INFO  [SDRAM] Cleared 0 pixels...
INFO  [SDRAM] Cleared 100000 pixels...
INFO  [SDRAM] Cleared 200000 pixels...
INFO  [SDRAM] Cleared 300000 pixels...
INFO  [SDRAM] Framebuffer ready
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
INFO  [TOUCH] Initializing FT6X06...
INFO  [TOUCH] FT6X06 initialized OK
INFO  [DISPLAY] Configuring layer...
INFO  [DISPLAY] Layer configured
INFO  [GUI] Initializing screen manager...
INFO  [GUI] Screen manager ready
INFO  ========================================
INFO  Ready! Touch screen to interact
INFO  ========================================
INFO  [FRAME] #0 - Rendering screen Home
INFO  [FRAME] #0 - Render complete
INFO  [TOUCH] #1 at (4095, 3840)
INFO  [FRAME] #1 - Rendering screen Home
...
INFO  [FRAME] #76 - Render complete
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### Improvements Over v2.8
- **RNG Workaround**: Uses software entropy instead of hanging
- **Progress**: Renders 76 frames before crash

### Issues Identified
1. **Red Screen**: Same as v2.7
2. **FT6X06 Panic**: Still crashes on touch
3. **Wrong Touch Coordinates**: `(4095, 3840)` still invalid

---

## v3.0 - BROKEN

### Status: ❌ BROKEN - Red Screen + FT6X06 Panic

### User Observation
> "v3 also shows the red screen"

### Boot Log
```
INFO  ========================================
INFO  Specter-DIY Rust Firmware v3.0
INFO  ========================================
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] Configuring clocks...
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [RNG] Using software entropy (HW RNG needs clock fix)
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
INFO  [SDRAM] Clearing framebuffer...
INFO  [SDRAM] Framebuffer ready
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
INFO  [TOUCH] Initializing FT6X06...
INFO  [TOUCH] FT6X06 initialized OK
INFO  [DISPLAY] Configuring layer...
INFO  [DISPLAY] Layer configured
INFO  [GUI] Initializing screen manager...
INFO  [GUI] Screen manager ready
INFO  ========================================
INFO  Ready! Touch screen to interact
INFO  ========================================
INFO  [FRAME] #0 - Rendering screen Home
INFO  [FRAME] #0 - Render complete
INFO  [TOUCH] Event #1: 1 finger(s) at (4095, 3840)
INFO  [FRAME] #1 - Rendering screen Home
...
INFO  [FRAME] #187 - Rendering screen Home
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### Improvements Over v2.9
- **More frames**: 187 frames before crash (vs 76 in v2.9)
- **Touch event logging**: Shows `Event #1: 1 finger(s) at (4095, 3840)`

### Issues Identified
1. **Red Screen**: Still not fixed
2. **FT6X06 Panic**: Still crashes
3. **Wrong Touch Coordinates**: Still `(4095, 3840)`

---

## Summary of Regressions

### Issues by Version

| Issue | v2.5 | v2.6 | v2.7 | v2.8 | v2.9 | v3.0 |
|-------|------|------|------|------|------|------|
| FT6X06 Panic | ✅ No | ❌ Yes | ❌ Yes | N/A | ❌ Yes | ❌ Yes |
| Red Screen | ✅ No | N/A | ❌ Yes | N/A | ❌ Yes | ❌ Yes |
| RNG Hang | ✅ No | N/A | N/A | ❌ Yes | ✅ Fixed | ✅ Fixed |
| Wrong Touch Coords | ✅ No | N/A | ❌ Yes | N/A | ❌ Yes | ❌ Yes |
| Missing debug=2 | ✅ No | ❌ Yes | ✅ No | ✅ No | ✅ No | ✅ No |
| Clock Changed | 168MHz | N/A | 180MHz | 180MHz | 180MHz | 180MHz |

### Key Bugs to Fix

#### 1. FT6X06 Multi-Touch Panic (CRITICAL)
**Affects**: v2.6, v2.7, v2.9, v3.0

The `ft6x06-0.1.2` crate panics when touch count > `FT6X06_MAX_NB_TOUCH`.

**Solution**: Create a safe wrapper that clamps touch count:
```rust
pub fn detect_touch_safe(&mut self) -> Option<TouchEvent> {
    let touches = ft6x06::read(self.i2c).ok()?;
    let safe_count = touches.len().min(FT6X06_MAX_TOUCHES);
    // ... return clamped event
}
```

**Reference**: v2.5 implementation works correctly.

#### 2. Red Screen (HIGH)
**Affects**: v2.7, v2.9, v3.0

Framebuffer is cleared with red (0xF800) instead of black (0x0000) in RGB565 format.

**Solution**:
```rust
// In framebuffer clear:
for pixel in framebuffer.iter_mut() {
    *pixel = 0x0000; // Black, NOT 0xF800 (red)
}
```

#### 3. RNG Initialization Hang (HIGH)
**Affects**: v2.8

Hardware RNG blocks forever waiting for DRDY flag.

**Solution**: Add timeout or use software entropy fallback (as v2.9/v3.0 do).

#### 4. Wrong Touch Coordinates (MEDIUM)
**Affects**: v2.7, v2.9, v3.0

Touch reports `(4095, 3840)` which are max/invalid values.

**Possible causes**:
- I2C read returning invalid data
- Coordinate transform not applied
- Touch controller not properly configured

**Reference**: v2.5 reports correct coordinates like `(309, 417)`.

---

## Recommendations for Next Version

### Must Fix (Blocking)
1. **Copy FT6X06 safe wrapper from v2.5** - This is the critical fix
2. **Fix framebuffer clear color** - Change 0xF800 to 0x0000

### Should Fix
3. **Investigate touch coordinate mapping** - Compare with v2.5 implementation
4. **Fix hardware RNG** - Enable clock properly or keep software fallback

### Nice to Have
5. **Keep SYSCLK at 168MHz** (v2.5 value) unless there's a reason for 180MHz
6. **Ensure `debug = 2`** in Cargo.toml for proper log locations

---

## Test Environment

```
Remote Board: ubuntu@192.168.13.246
Chip: STM32F469NIHx
Display Controller: NT35510 (B08 revision)
Touch Controller: FT6X06

Flash Command:
scp firmware.elf ubuntu@192.168.13.246:/tmp/ && \
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 120 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf"
```

---

## Feedback Files

Individual feedback files have been created:
- `feedback_for_v2.5.md` - Working reference
- `feedback_for_v2.6.md` - FT6X06 panic
- `feedback_for_v2.7.md` - Red screen + panic
- `feedback_for_v2.8.md` - RNG hang
- `feedback_for_v2.9.md` - Red screen + panic (RNG workaround)
- `feedback_for_v3.0.md` - Red screen + panic

---

*Report generated from live hardware testing session*
