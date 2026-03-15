# Feedback for v2.7 - RED SCREEN + TOUCH PANIC

**Status**: ❌ BROKEN - Red screen + FT6X06 panic

## What Works
- ✅ Boot sequence completes
- ✅ Display initialization (NT35510)
- ✅ Touch initialization
- ✅ LED blinks on touch initially

## What's Broken

### 1. Red Screen
- Framebuffer clear using wrong color
- Screen shows red instead of black/clear background

### 2. FT6X06 Multi-Touch Panic (CRITICAL)
**Crashes on touch:**
```
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### 3. Wrong Touch Coordinates
```
[TOUCH] #1 at (4095, 3840)
```
- These are near-maximum values, indicating coordinate mapping is broken

### 4. Clock Speed Changed
- SYSCLK changed from 168MHz (v2.5) → 180MHz (v2.7)
- This may affect other timings

## Fixes Required

### Fix 1: Framebuffer Clear Color
In the framebuffer clear code, use black (0x0000) instead of red:
```rust
// WRONG - this is red in RGB565
for pixel in framebuffer.iter_mut() {
    *pixel = 0xF800; // Red
}

// CORRECT - black
for pixel in framebuffer.iter_mut() {
    *pixel = 0x0000; // Black
}
```

### Fix 2: FT6X06 Safe Wrapper
Create a safe wrapper that clamps touch count (see v2.6 feedback for details)

### Fix 3: Touch Coordinate Mapping
The raw coordinates (4095, 3840) suggest:
- I2C read returning invalid data, OR
- Coordinate transform not applied, OR
- Touch controller not properly initialized

## Comparison with v2.5
| Feature | v2.5 | v2.7 |
|---------|------|------|
| Screen Color | ✅ Black | ❌ Red |
| Touch Events | ✅ Works | ❌ Panic |
| Touch Coords | ✅ Correct | ❌ (4095, 3840) |
| SYSCLK | 168MHz | 180MHz |

---

**Conclusion**: v2.7 introduces red screen bug and still has FT6X06 panic. Revert screen color to v2.5 implementation.
