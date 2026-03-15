# STM32F469I-DISCO Display & Touch Development Notes

## Session Summary: 2026-02-25

### What's Working Now ✅

| Component | Status | Notes |
|-----------|--------|-------|
| **Board Unlock** | ✅ | Cleared Specter-DIY RDP/WRP protection via STM32CubeProgrammer |
| **Flash Access** | ✅ | st-flash and probe-rs both work on remote |
| **LED Blinky** | ✅ | Basic firmware execution confirmed |
| **LCD Display** | ✅ | Color/BER test patterns working (NT35510, B08 revision) |
| **Touch Input** | ✅ | Toggle on touch working (after td_status bypass) |
| **Remote Deploy** | ✅ | SSH → ubuntu@192.168.13.246 with probe-rs |

### Key Technical Findings

#### 1. FT6x06 Crate Panic Bug

**Problem**: `ft6x06` v0.1.2 panics when touch controller returns `ntouch > 2`:
```rust
// In ft6x06-0.1.2/src/lib.rs:332
assert!(ntouch <= FT6X06_MAX_NB_TOUCH as u8);  // PANICS!
```

**Root Cause**: The crate uses `assert!()` internally instead of returning `Result::Err`.

**Workaround Applied**: Call `td_status()` directly instead of `detect_touch()`:
```rust
// BEFORE (panics):
let num = match touch.detect_touch(&mut i2c) { ... };

// AFTER (works):
let num = match touch.td_status(&mut i2c) {
    Ok(n) => {
        let touch_count = n & 0x0F;  // Mask garbage bits
        if touch_count > 2 { continue; }  // Guard before using
        touch_count
    }
    ...
};
```

#### 2. Board Hardware Configuration

| Component | Details |
|-----------|---------|
| **LCD Controller** | NT35510 (B08 revision) - auto-detected |
| **Touch Controller** | FT6x06 via I2C (addr: 0x38) |
| **DSI Config** | Double-lane, 27.429 MHz LTDC, HSE 8MHz |
| **Display** | 480x800 portrait |

#### 3. Remote Flash Setup

```bash
# Build locally
cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"

# Deploy to remote
scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test && probe-rs reset --chip STM32F469NIHx"

# Monitor logs
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs attach --chip STM32F469NIHx /tmp/f469disco-lcd-test"
```

---

## Roadmap: Next Steps

### Phase 1: Upgrade Touch Driver (Priority: HIGH)

**Goal**: Replace buggy `ft6x06` v0.1.2 with modern `ft6x06-rs` v0.3.0

| Step | Action | Status |
|------|--------|--------|
| 1.1 | Research `ft6x06-rs` v0.3.0 API differences | Pending |
| 1.2 | Update Cargo.toml dependency | Pending |
| 1.3 | Adapt example code to new API | Pending |
| 1.4 | Test touch functionality | Pending |

**Questions to Answer**:
- Does v0.3.0 handle invalid touch counts gracefully?
- Is API compatible with embedded-hal 1.0?
- Does it support async (for future work)?

### Phase 2: Clean Demo Implementation

**Goal**: Production-quality F469I-DISCO demo following HAL conventions

| Step | Action | Status |
|------|--------|--------|
| 2.1 | Review existing HAL examples for patterns | Pending |
| 2.2 | Extract display init into reusable module | Pending |
| 2.3 | Clean up touch handling with proper error types | Pending |
| 2.4 | Add documentation and comments | Pending |
| 2.5 | Consider feature flags for different demos | Pending |

### Phase 3: Integration Tests

| Step | Action | Status |
|------|--------|--------|
| 3.1 | Add CI build check for F469 examples | Pending |
| 3.2 | Document board setup requirements | Pending |
| 3.3 | Create troubleshooting guide | Pending |

---

## Technical Debt / Known Issues

1. **DSI Read Errors**: NT35510 probe fails with DSI read errors but defaults correctly
   - May be timing-related
   - Works despite errors

2. **Touch Coordinate Noise**: Touch coordinates can be noisy (rapid fire events)
   - Consider debouncing
   - May need calibration

3. **Board Detection**: `BoardHint::Unknown` from touch probe
   - I2C address detection may be unreliable at init

---

## Branch Status

- **Current Branch**: `master` (local, ahead of origin by 4 commits)
- **Working Example**: `examples/f469disco-lcd-test.rs`
- **Uncommitted Changes**: Touch fix in `f469disco-lcd-test.rs`

---

## Reference Links

- [ft6x06-rs v0.3.0](https://github.com/DogeDark/ft6x06-rs) - Modern driver with async support
- [ft6x06 v0.1.2](https://github.com/Srg213/ft6x06) - Original buggy driver
- [STM32F469 Datasheet](https://www.st.com/resource/en/datasheet/stm32f469ni.pdf)
- [NT35510 Datasheet](https://www.newhavendisplay.com/appnotes/datasheets/LCDs/NT35510.pdf)

---

## Session: 2026-02-26 - ST Spec Timing Investigation

### Experiment: ST Spec Timings

**Hypothesis**: Using ST's official NT35510 timing values from stm32-nt35510 BSP would be more correct than OTM8009A-style timings.

**ST Spec Values Tested**:
| Parameter | OTM8009A (Working) | ST Spec NT35510 (Tested) |
|-----------|---------------------|--------------------------|
| v_sync | 1 | 120 |
| v_back_porch | 15 | 150 |
| v_front_porch | 16 | 150 |
| **Total V lines** | 832 | 1220 (+46%) |

### Result: FAILED ❌

- **Display**: Garbled/noise output
- **Touch**: Still working (confirmed toggles #36-47)
- **DSI/LTDC**: Init sequence completed without errors

### Root Cause Analysis

1. **Frame Size Mismatch**:
   - ST spec creates 46% larger vertical frame (1220 vs 832 lines)
   - DSI timing calculations in `src/dsi.rs` depend on frame size
   - Larger frame may cause HLINE calculation errors

2. **Timing Calculations Affected**:
   - `dsi.rs:323-355` uses display config values for DSI video mode timing
   - Byte clock cycle calculations may overflow or misalign
   - LTDC pixel clock (27.429 kHz) doesn't match new frame timing

3. **Why Touch Still Works**:
   - Touch is I2C-based (FT6x06), completely independent of display timing
   - DSI/LTDC timing changes don't affect I2C bus

4. **Why Specter-DIY Uses These Timings**:
   - May have different PLLSAI/DSI clock configurations
   - Different display panel variant tolerances
   - Their initialization sequence may compensate

### Conclusion

**Stick with OTM8009A-style tight timings for NT35510.**

The ST spec timings are likely designed for a different panel variant or require
different DSI/LTDC clock configurations that we haven't implemented.

```rust
// WORKING: Use these for NT35510
v_back_porch: 15,
v_front_porch: 16,
v_sync: 1,
```

### Files Modified

- `examples/f469disco-lcd-test.rs` - Reverted to working timings
- `.sisyphus/notes/f469-development.md` - This documentation

