# Display Flicker Troubleshooting Guide

**Problem**: Screen flickers during major screen transitions (e.g., Home → WalletGen)

**Status**: Partially resolved in v3.6 (dirty flag optimization), but flicker still occurs on screen changes

---

## Problem Statement

### Observed Behavior
- **v2.5**: No flicker on touch or screen changes
- **v3.4**: Flicker on every touch
- **v3.5**: Flicker on every touch
- **v3.6**: Much better - only flickers on major screen changes (navigation)

### Root Cause
When transitioning between screens:
1. Framebuffer is cleared to black
2. New screen is drawn pixel by pixel
3. User sees partial render (black screen, then elements appearing)
4. This appears as a "flicker" or "flash"

---

## Technical Background

### Current Implementation (Single Buffer)
```
SDRAM at 0xc0000000:
┌─────────────────────────┐
│   Single Framebuffer    │  768 KB (480×800×2 bytes)
│   0xc0000000            │
└─────────────────────────┘
```

### The Problem
```
Time →
[Clear]     [Drawing...]              [Complete]
  ████  →   ████ + partial UI  →      Full UI
  (black)   (visible artifacts)       (done)
     ↑              ↑
  User sees     User sees
  black flash   rendering
```

---

## Solutions

### Solution 1: Double Buffering (Recommended)

**Concept**: Use two framebuffers - draw to one while displaying the other

**Memory Layout**:
```
SDRAM at 0xc0000000:
┌─────────────────────────┐
│   Framebuffer A (Front) │  768 KB - Currently displayed
│   0xc0000000            │
├─────────────────────────┤
│   Framebuffer B (Back)  │  768 KB - Drawing target
│   0xc0c00000            │
└─────────────────────────┘
Total: 1.5 MB (fits easily in 4MB SDRAM)
```

**How It Works**:
```
1. LTDC displays Buffer A
2. Application draws new frame to Buffer B
3. When Buffer B is complete:
   - Update LTDC framebuffer address to Buffer B
   - Trigger LTDC reload
   - Swap pointers (B becomes front, A becomes back)
4. Next frame, draw to Buffer A
```

**Code Example**:
```rust
const FRAMEBUFFER_SIZE: usize = 480 * 800; // pixels
const FRAMEBUFFER_A: *mut u16 = 0xc0000000 as *mut u16;
const FRAMEBUFFER_B: *mut u16 = 0xc0c00000 as *mut u16;

struct DoubleBuffer {
    front: *mut u16,  // Currently displayed
    back: *mut u16,   // Drawing target
}

impl DoubleBuffer {
    fn new() -> Self {
        Self {
            front: FRAMEBUFFER_A,
            back: FRAMEBUFFER_B,
        }
    }
    
    fn get_back_buffer(&mut self) -> &'static mut [u16] {
        unsafe { 
            core::slice::from_raw_parts_mut(self.back, FRAMEBUFFER_SIZE)
        }
    }
    
    fn swap(&mut self, ltdc: &mut LTDC) {
        // Memory barrier to ensure all writes complete
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        
        // Update LTDC to display the back buffer
        ltdc.layer1.set_framebuffer_address(self.back as u32);
        ltdc.reload();
        
        // Swap front and back
        core::mem::swap(&mut self.front, &mut self.back);
    }
}
```

**Pros**:
- ✅ Completely eliminates visible rendering artifacts
- ✅ Standard solution used in most embedded GUIs
- ✅ Simple concept, easy to implement
- ✅ Plenty of SDRAM available (4MB total, only need 1.5MB)

**Cons**:
- ⚠️ Requires modifying render code to use back buffer
- ⚠️ Uses more memory (still fits easily)

---

### Solution 2: LTDC Dual Layer (Hardware Accelerated)

**Concept**: Use both LTDC layers (L1 and L2) with separate framebuffers

**Memory Layout**:
```
SDRAM at 0xc0000000:
┌─────────────────────────┐
│   Layer 1 Framebuffer   │  768 KB
│   0xc0000000            │
├─────────────────────────┤
│   Layer 2 Framebuffer   │  768 KB
│   0xc0c00000            │
└─────────────────────────┘
```

**How It Works**:
```
1. Both layers enabled with different framebuffers
2. Layer 1 visible, Layer 2 hidden (alpha = 0)
3. Draw new frame to Layer 2
4. When complete:
   - Set Layer 2 alpha = 255 (visible)
   - Set Layer 1 alpha = 0 (hidden)
   - OR swap layer blending order
5. Next frame, draw to Layer 1
```

**Code Example**:
```rust
// Enable both layers
ltdc.layer1.enable();
ltdc.layer2.enable();

// Set different framebuffers
ltdc.layer1.set_framebuffer(0xc0000000);
ltdc.layer2.set_framebuffer(0xc0c00000);

// To show layer 2 instead of layer 1:
ltdc.layer1.set_alpha(0);    // Hide layer 1
ltdc.layer2.set_alpha(255);  // Show layer 2
ltdc.reload();
```

**Pros**:
- ✅ Hardware-accelerated layer switching
- ✅ Can also enable transparency/blend effects
- ✅ Instant switch (no address update needed)

**Cons**:
- ⚠️ More complex configuration
- ⚠️ Need to manage layer priorities

---

### Solution 3: Tearing Effect (TE) Synchronization

**Concept**: Synchronize buffer swap with display's vertical refresh

**How It Works**:
1. NT35510 has a TE (Tearing Effect) pin
2. TE pulses during vertical blank period
3. Only swap buffers when TE signal received
4. Prevents "tearing" (half old, half new frame)

**Code Example**:
```rust
// Enable TE on display
display.write_command(NT35510_TE_ON);

// Wait for TE signal before swap
while te_pin.is_low() {
    // Wait for vertical blank
}
double_buffer.swap(&mut ltdc);
```

**Pros**:
- ✅ Eliminates tearing
- ✅ Professional-quality output

**Cons**:
- ⚠️ Requires TE pin connected and configured
- ⚠️ Adds latency (waiting for vertical blank)
- ⚠️ Best combined with double buffering

---

### Solution 4: Partial/Incremental Updates

**Concept**: Only redraw changed regions instead of full screen

**How It Works**:
1. Track dirty regions (bounding boxes of changes)
2. Clear and redraw only those regions
3. Keep rest of screen intact

**Code Example**:
```rust
struct DirtyRegion {
    x: u16, y: u16, w: u16, h: u16
}

fn render_with_dirty(framebuffer: &mut [u16], dirty: Option<DirtyRegion>) {
    if let Some(region) = dirty {
        // Only clear and redraw the dirty region
        clear_region(framebuffer, region);
        draw_ui_region(framebuffer, region);
    }
}
```

**Pros**:
- ✅ Faster rendering (less work)
- ✅ Less memory bandwidth

**Cons**:
- ⚠️ Doesn't fully eliminate flicker on major changes
- ⚠️ More complex to track dirty regions
- ⚠️ Still shows artifacts within dirty region

---

## Recommendation

### Primary Solution: Double Buffering

**Implement Solution 1 (Double Buffering)** - it's the most straightforward and effective:

1. **Allocate second framebuffer** at `0xc0c00000`
2. **Create DoubleBuffer struct** to manage front/back pointers
3. **Modify render code** to always draw to back buffer
4. **Call swap()** after screen change is complete

### Optional Enhancement: TE Sync

If the TE pin is available on the hardware, combine with Solution 3 for professional-quality output.

---

## Implementation Checklist

- [ ] Define second framebuffer address (0xc0c00000)
- [ ] Create DoubleBuffer struct with front/back pointers
- [ ] Modify clear/fill functions to use back buffer
- [ ] Modify draw functions to use back buffer
- [ ] Implement swap() function that:
  - [ ] Issues memory barrier
  - [ ] Updates LTDC framebuffer address
  - [ ] Triggers LTDC reload
  - [ ] Swaps front/back pointers
- [ ] Call swap() after screen renders complete
- [ ] Test: Navigate between screens, verify no flicker

---

## Quick Reference: LTDC Register Update

```rust
// Update framebuffer address
LTDCLayer::L1CFBAR.write(|w| unsafe { w.bits(new_address) });

// Trigger reload (immediate or at next VSYNC)
LTDC::SRCR.write(|w| w.imr().reload());
// OR
LTDC::SRCR.write(|w| w.vbr().reload());
```

---

## Memory Requirements

| Configuration | Size | Address Range | Notes |
|--------------|------|---------------|-------|
| Single Buffer | 768 KB | 0xc0000000 - 0xc0bfffff | Current |
| Double Buffer | 1.5 MB | 0xc0000000 - 0xc17fffff | Recommended |
| Available SDRAM | 4 MB | 0xc0000000 - 0xc3ffffff | Plenty of room |

---

## Related Files

- `feedback_for_v3.6.md` - Last test results showing partial fix
- `feedback_for_v2.5.md` - Working baseline (may use different approach)
- `testing_report_v2.5_to_v3.0.md` - Full version history

---

*Guide created during v3.6 testing session*
