# Black Screen Issue - v4.3

## Problem

Firmware v4.3 initializes successfully without panics, but the display shows a **black screen**. All logs indicate success:

```
INFO  Test pattern drawn
INFO  Display initialized successfully
INFO  Display controller: Nt35510
INFO  Display layer configured
```

However, no actual output is visible on the screen.

---

## Hardware Context

- **Board**: STM32F469I-DISCO
- **Display**: 480x800 LCD via DSI
- **Controller**: NT35510 (B08 revision)
- **Framebuffer**: SDRAM at 0xc0000000

---

## Root Cause Analysis (Most Likely → Least Likely)

### 1. LTDC Layer Not Enabled (Most Likely)

The log says "Display layer configured" but **configured ≠ enabled**.

On STM32F4, LTDC layer changes don't take effect until:
1. The layer is explicitly enabled
2. A reload is triggered to apply the shadow registers

**Fix:**
```rust
// After configuring the LTDC layer:
layer.enable();
ltdc.reload();  // CRITICAL: Applies layer configuration to hardware
```

**Reference:** This same issue was fixed in v3.4. The v3.4 fix enabled the LTDC layer and called reload.

### 2. Backlight Not Enabled

The display controller and LTDC may be working correctly, but the backlight is off.

**Fix:**
```rust
// Configure backlight GPIO and set HIGH
let mut backlight = gpiog.pg13.into_push_pull_output();
backlight.set_high();
```

Check the STM32F469I-DISCO schematic for the actual backlight pin.

### 3. Framebuffer Address Mismatch

LTDC may be reading from the wrong address.

**Fix:**
```rust
// Verify framebuffer address matches LTDC config
let fb_address = framebuffer.as_ptr() as u32;
defmt::info!("Framebuffer at: 0x{:08x}", fb_address);

// LTDC layer configuration should use same address
layer.set_framebuffer(fb_address);
```

### 4. DSI Video Mode Not Enabled

If DSI is in command mode, the display won't auto-refresh.

**Fix:**
```rust
// Ensure DSI is configured for video mode, not command mode
// Video mode sends continuous pixel data
```

### 5. Pixel Format Mismatch

LTDC configured for different pixel format than what's written to framebuffer.

**Common mismatch:**
- LTDC expects ARGB8888 (4 bytes/pixel)
- Framebuffer contains RGB565 (2 bytes/pixel)

---

## Debugging Steps

Add these logs to identify the issue:

```rust
// After LTDC initialization
defmt::info!("LTDC enabled: {}", ltdc.is_enabled());
defmt::info!("LTDC layer 1 enabled: {}", layer1.is_enabled());

// Framebuffer info
defmt::info!("Framebuffer address: 0x{:08x}", fb_ptr as u32);
defmt::info!("Framebuffer size: {} bytes", fb_size);

// Backlight status
defmt::info!("Backlight pin state: {}", backlight.is_set_high());

// Write test pattern to first few pixels
let fb = framebuffer.as_mut_ptr() as *mut u16;
for i in 0..100 {
    unsafe { *fb.add(i) = 0xF800 }; // Red in RGB565
}
defmt::info!("Test pattern written to framebuffer");
```

---

## Known Working Code (v3.7 Reference)

v3.7 display output worked. Compare v4.3 display initialization with v3.7 to find what's different:

1. LTDC enable sequence
2. Layer enable + reload
3. Backlight GPIO
4. DSI configuration

---

## Quick Fix Checklist

- [ ] Call `layer.enable()` after configuring layer
- [ ] Call `ltdc.reload()` after enabling layer
- [ ] Verify backlight GPIO is set HIGH
- [ ] Confirm framebuffer address matches LTDC config
- [ ] Check pixel format consistency (RGB565 vs ARGB8888)
- [ ] Verify DSI is in video mode

---

## Expected Result After Fix

Screen should show:
1. Test pattern (if drawn)
2. Or solid color if framebuffer cleared
3. Touch coordinates should still work (they already do in v4.3)

---

## Files to Check/Modify

1. `firmware/src/main.rs` - Display initialization around line 315-331
2. `firmware/src/board/mod.rs` - `init_display_full()` function
3. Look for LTDC layer configuration and enable sequence
