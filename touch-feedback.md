# Firmware Feedback — v2.9 Touch & Display Issues

## Problem Summary

**Version:** v2.9  
**Status:** ⚠️ Boots successfully but touch doesn't work, screen shows red

---

## Observed Behavior

### What Works
```
INFO  Specter-DIY Rust Firmware v2.9
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [SDRAM] Initialized at 0xc0000000
INFO  [DISPLAY] Controller: Nt35510
INFO  [TOUCH] FT6X06 initialized OK
INFO  [GUI] Screen manager ready
INFO  Ready! Touch screen to interact
INFO  [FRAME] #0 - Rendering screen Home
INFO  [FRAME] #0 - Render complete
```

- ✅ Boot completes (~2.94s)
- ✅ All hardware initialized
- ✅ Frame #0 renders

### What Doesn't Work
- ❌ **Screen shows RED** instead of home screen UI
- ❌ **No touch events** when touching screen
- ❌ **No heartbeat logs** after boot
- ❌ **No frame #1, #2, etc.** - only frame #0 logged

---

## Root Cause Analysis

### Issue 1: Red Screen

The framebuffer is being cleared but apparently to the **wrong color**. The user sees red instead of the home screen.

**Likely cause:** Framebuffer clear using wrong color value

```rust
// WRONG - This is probably what's happening:
for pixel in buffer.iter_mut() {
    *pixel = 0xF800;  // Red in RGB565!
}

// CORRECT - Should be:
for pixel in buffer.iter_mut() {
    *pixel = 0x0000;  // Black in RGB565
}
```

Or the home screen render function isn't actually drawing anything visible.

---

### Issue 2: No Touch Events

After `[FRAME] #0 - Render complete`, there are:
- ❌ No `[TOUCH] Event` logs
- ❌ No `[HEARTBEAT]` logs
- ❌ No subsequent `[FRAME]` logs

**This means the main loop is likely:**
1. Not running at all (crashed/hung after frame #0)
2. Running but not logging anything
3. Running extremely slowly

---

## Required Fixes

### Fix 1: Add Heartbeat Logging (CRITICAL)

Add this to prove the main loop is running:

```rust
loop {
    frame += 1;
    
    // Heartbeat every 500 frames
    if frame % 500 == 0 {
        defmt::info!("[HEARTBEAT] frame={} screen=Home", frame);
    }
    
    // ... rest of loop
}
```

**Without heartbeat, we can't tell if the loop is running.**

---

### Fix 2: Fix Framebuffer Clear Color

Ensure framebuffer is cleared to **black**, not red:

```rust
// In framebuffer clear function:
fn clear(&mut self) {
    defmt::info!("[FB] Clearing to black (0x0000)");
    for pixel in self.buffer.iter_mut() {
        *pixel = 0x0000;  // BLACK in RGB565
    }
}
```

**Common mistakes:**
- `0xFF00` = Red in some pixel formats
- `0xF800` = Red in RGB565
- `0x00FF` = Also red-ish depending on format

---

### Fix 3: Add Touch Event Logging

```rust
loop {
    // Poll touch
    match touch.read() {
        Some((x, y, fingers)) => {
            defmt::info!("[TOUCH] Event: {} finger(s) at ({}, {})", fingers, x, y);
            // Handle touch...
        }
        None => {
            // No touch - this is normal, don't log
        }
    }
}
```

---

### Fix 4: Ensure Home Screen Renders

```rust
impl HomeScreen {
    fn render(&self, fb: &mut Framebuffer) {
        defmt::info!("[HOME] Starting render");
        
        // Clear to BLACK first
        fb.clear();  // Should clear to 0x0000
        
        // Draw SOMETHING visible
        fb.fill_rect(0, 0, 480, 800, 0x0000);  // Black background
        fb.draw_text(100, 100, "Specter-DIY", 0xFFFF);  // White text
        
        defmt::info!("[HOME] Render complete");
    }
}
```

---

## Expected Log Output (After Fixes)

```
INFO  Ready! Touch screen to interact
INFO  [FRAME] #0 - Rendering screen Home
INFO  [HOME] Starting render
INFO  [HOME] Render complete
INFO  [FRAME] #0 - Render complete
INFO  [HEARTBEAT] frame=500 screen=Home
INFO  [HEARTBEAT] frame=1000 screen=Home
INFO  [TOUCH] Event: 1 finger(s) at (240, 400)
INFO  [HOME] touch at (240, 400)
INFO  [HEARTBEAT] frame=1500 screen=Home
```

---

## Diagnostic Checklist

- [ ] Add heartbeat every 500 frames
- [ ] Fix framebuffer clear to black (0x0000)
- [ ] Add touch event logging
- [ ] Verify home screen draws visible content
- [ ] Verify main loop continues after frame #0

---

## Quick Test

After fixes, we should see:
1. **Heartbeats** every few seconds → loop is running
2. **Black screen** with UI → framebuffer correct
3. **Touch events** when touching → touch working

If we see heartbeats but no touch events → touch polling broken
If we see no heartbeats → main loop crashed/hung
If screen still red → framebuffer clear still wrong

---

## Priority

**CRITICAL** - Device is unusable without working touch and display.
