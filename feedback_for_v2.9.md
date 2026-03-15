# Feedback for v2.9 - RED SCREEN + TOUCH PANIC

**Status**: ❌ BROKEN - Red screen + FT6X06 panic (but RNG workaround works)

## What Works
- ✅ Boot sequence completes
- ✅ RNG workaround (software entropy) - no hang like v2.8!
- ✅ Display initialization (NT35510)
- ✅ Touch initialization
- ✅ Frame rendering (76 frames before crash)
- ✅ Touch registers initially (LED feedback works)

## What's Broken

### 1. Red Screen
- Framebuffer clear using wrong color (0xF800 red instead of 0x0000 black)
- Same issue as v2.7

### 2. FT6X06 Multi-Touch Panic (CRITICAL)
**Crashes on touch after ~76 frames:**
```
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

### 3. Wrong Touch Coordinates
```
[TOUCH] #1 at (4095, 3840)
```
- These are max/invalid values
- Coordinate mapping or I2C read issue

## Improvements Over v2.8

### RNG Workaround
```
INFO  [RNG] Using software entropy (HW RNG needs clock fix)
INFO  [RNG] Entropy seed initialized
```
- Uses software entropy instead of hardware RNG
- Avoids the v2.8 hang

### Progress
```
INFO  Ready! Touch screen to interact
INFO  [FRAME] #0 - Rendering screen Home
...
INFO  [FRAME] #76 - Render complete
```
- Gets further than v2.8 (which hangs at RNG init)
- Renders 76 frames before crash

## Fixes Required

### Fix 1: Framebuffer Clear Color
```rust
// In framebuffer clear:
*pixel = 0x0000; // Black, not 0xF800 (red)
```

### Fix 2: FT6X06 Safe Wrapper (CRITICAL)
Must create safe wrapper that clamps touch count:
```rust
pub fn detect_touch_safe(&mut self) -> Option<TouchEvent> {
    // Read touches, clamp count to max 2
    let ntouch = self.read_touch_count().min(2);
    // ... rest of implementation
}
```

### Fix 3: Touch Coordinate Mapping
Investigate why coordinates are (4095, 3840):
- Check I2C read
- Verify coordinate transform
- Check touch controller registers

## Version Comparison
| Feature | v2.5 | v2.8 | v2.9 |
|---------|------|------|------|
| RNG | ✅ HW | ❌ Hang | ✅ SW fallback |
| Screen Color | ✅ Black | N/A | ❌ Red |
| Touch | ✅ Works | N/A | ❌ Panic |
| Boot | ✅ | ❌ Hangs | ✅ |
| Frames | 2000+ | 0 | 76 |

---

**Conclusion**: v2.9 makes progress (software RNG workaround) but still has red screen and FT6X06 panic. Fix framebuffer color and add FT6X06 safe wrapper to reach v2.5 stability.
