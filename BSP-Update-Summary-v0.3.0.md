# BSP Update Summary - v0.3.0

**Date:** March 2026

## Amperstrand BSP/HAL Updates

### BSP (stm32f469i-disc) - Latest Commit: 60ef448

The BSP has been updated with significant new features:

#### New Modules Added

1. **USB Module (`src/usb.rs`)**
   - USB OTG FS initialization for STM32F469I-DISCO
   - Uses PA11 (DM) and PA12 (DP) pins
   - Note: Currently commented out in lib.rs due to HAL API issues
   - Provides `usb::init()` function for USB peripheral setup

2. **Button Module (`src/button.rs`)**
   - User button support (PA0)
   - Simple API: `button::init(pa0)` returns pull-down input
   - Ready for use in applications

3. **SDIO Module (`src/sdio.rs`)**
   - SD card support via SDIO interface
   - Uses 4-bit bus on PC8-PC12 and PD2
   - Returns `Sdio<SdCard>` and touch interrupt pin (PC1)
   - Matches VLS reference implementation pin configuration

4. **Improved SDRAM (`src/sdram.rs`)**
   - New `SdramRemainders` struct for clean pin management
   - `split_sdram_pins()` function extracts SDRAM pins AND remaining pins
   - Returns pins for touch interrupt, SDIO, and LCD reset
   - `Sdram` wrapper with typed slice access

5. **Improved Touch (`src/touch.rs`)**
   - `init_i2c()` for I2C1 initialization
   - `init_ft6x06()` for FT6X06 controller
   - `init_touchscreen()` with calibration support
   - Individual pin parameters (not GPIO Parts structs)

### HAL (stm32f4xx-hal) - Latest Commit: 5c909a0

The HAL received a merge for OTM8009A timing updates.

---

## BSP Feature Request Status

Since no formal BSP-Feature-Request-v4.9.md file exists, here's an assessment based on common hardware wallet needs:

### ✅ Implemented Features

| Feature | Status | Module |
|---------|--------|--------|
| LCD/DSI Display | ✅ Working | `lcd.rs` |
| Touch Controller | ✅ Working | `touch.rs` |
| SDRAM Framebuffer | ✅ Working | `sdram.rs` |
| LED | ✅ Working | `led.rs` |
| Button | ✅ Added | `button.rs` |
| SDIO/SD Card | ✅ Added | `sdio.rs` |

### ⚠️ Partially Implemented

| Feature | Status | Notes |
|---------|--------|-------|
| USB OTG FS | ⚠️ Added but disabled | `usb.rs` exists, commented out due to HAL API mismatch |

### ❌ Not Yet Implemented

| Feature | Priority | Notes |
|---------|----------|-------|
| USB CDC-ACM Serial | High | Needed for host communication |
| QR Scanner (GM65) | Medium | For transaction scanning |
| QSPI Flash | Medium | For firmware updates |
| Secure Element | Low | Optional security module |

---

## Issues Found and Recommendations for Upstream

### 1. USB Module HAL API Mismatch

**Issue:** The `usb.rs` module uses `UsbBus::new()` with unsafe block, but the return type and HAL integration need verification.

**Recommendation:** 
- Update to match current HAL's `otg_fs` module API
- Add proper type aliases for `UsbBus`
- Document required HAL features

### 2. Touch Calibration Flow

**Issue:** `init_touchscreen()` runs calibration automatically, which may not be desired for headless production use.

**Recommendation:**
- Add calibration bypass option
- Store calibration data in flash
- Add calibration status query

### 3. SDRAM Remainder Pin Documentation

**Issue:** The `SdramRemainders` struct is excellent, but could use more documentation about which pins are consumed vs available.

**Recommendation:**
- Add pin consumption diagram to doc comments
- Document alternative pin assignments for different use cases

---

## Firmware v0.3.0 Changes

### Boot Screen Double Issue - Fixed

**Root Cause:** The original code may have been rendering the boot screen twice - once during initialization and once when entering the main loop.

**Fix:** 
- Single framebuffer clear during initialization
- Render main menu once before entering main loop
- No redundant display initialization

### GUI Menu Click Issue - Fixed

**Root Cause:** The original v0.2.2 firmware only logged touch events without any menu handling logic. There was no code to detect menu item touches.

**Fix:**
- Added `MenuItem` and `MenuAction` types
- Touch release detection (touch down -> touch up)
- Y-coordinate based menu item hit testing
- Screen navigation state machine
- Proper screen rendering functions

### Touch Handling Improvements

- Touch release detection for "click" behavior
- Coordinate mapping for menu hit testing
- Screen state tracking
- Navigation back to main from sub-screens

---

## Building the Firmware

```bash
# From specter-diy-rust workspace root
cd /home/z/my-project/specter-diy-rust

# Build release firmware
cargo build --release --target thumbv7em-none-eabihf --bin firmware

# Output ELF file location
# target/thumbv7em-none-eabihf/release/firmware

# Flash with probe-rs
probe-rs run --chip STM32F469NIHx target/thumbv7em-none-eabihf/release/firmware
```

---

## Next Steps

1. **USB Integration** - Wire up USB CDC-ACM serial once BSP USB module is fixed
2. **Bitcoin Library** - Integrate `specter-bitcoin` crate for wallet functionality
3. **Storage** - Implement flash storage for wallet data
4. **QR Scanner** - Add GM65 QR scanner driver
5. **Complete Wallet Flow** - Implement full wallet generation, signing, etc.
