# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# Plan: F469I-DISCO Touch Driver Upgrade & Demo Cleanup

## TL;DR
> Upgrade to ft6x06-rs v0.3.0 for robust touch handling, then create a clean demo implementation following HAL patterns.

## Status: ✅ PHASE 1-3 COMPLETE

## Context

### Current State
- ✅ Display working (NT35510 LCD, color/BER patterns)
- ✅ **Touch working with ft6x06-rs v0.3.0** (UPGRADED!)
- ✅ No workarounds needed - clean API

### Why Upgrade?
The newer `ft6x06-rs` v0.3.0 crate (by DogeDark):
- Proper error handling (no internal panics) ✅
- Async support via embedded-hal-async
- Cleaner API with `get_touch_event()` method ✅
- Active development (2024 vs 2022)

## Work Objectives

1. ✅ **Upgrade touch driver** to ft6x06-rs v0.3.0
2. ✅ **Verify touch works** with new crate
3. ⏳ **Clean up demo code** following HAL conventions (Phase 4)
4. ⏳ **Document** for future developers (Phase 4)

## TODOs

### Phase 1: Research & Preparation ✅

- [x] 1.1 Research ft6x06-rs v0.3.0 API
  - Struct: `FT6x06<I2C>` (not `Ft6X06`)
  - Constructor: `FT6x06::new(i2c)` - takes ownership, no addr/irq params
  - Method: `get_touch_event()` returns `Result<Option<TouchEvent>, DriverError>`
  - TouchPoint: `primary_point.x`, `primary_point.y`, `primary_point.weight`
  - **Key difference**: Only supports I2C address 0x38

- [x] 1.2 Check dependency compatibility
  - ✅ stm32f4xx-hal uses embedded-hal 1.0 (compatible)
  - ✅ async features are optional (using sync-driver)

### Phase 2: Upgrade Implementation ✅

- [x] 2.1 Add ft6x06-rs dependency
  
  **File**: `Cargo.toml`
  ```toml
  # Added:
  ft6x06-rs = { version = "0.3.0", default-features = false, features = ["sync-driver"] }
  ```

- [x] 2.2 Update example imports and initialization

  **File**: `examples/f469disco-lcd-test.rs`
  
  Changes made:
  - `use ft6x06::Ft6X06` → `use ft6x06_rs::FT6x06`
  - `Ft6X06::new(&i2c, touch_addr, ts_int)` → `FT6x06::new(i2c)`
  - Driver now takes ownership of I2C

- [x] 2.3 Remove td_status workaround
  - ✅ Removed `td_status()` workaround completely
  - ✅ Using `get_touch_event()` directly
  - ✅ No more masking/guard code needed

### Phase 3: Verification ✅

- [x] 3.1 Build and flash updated example
  ```bash
  cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"
  # Build succeeded!
  ```

- [x] 3.2 Monitor logs for touch events
  - ✅ Touch coordinates detected: x=384-472, y=280-426
  - ✅ Touch releases detected
  - ✅ 20+ toggles in 30 seconds

- [x] 3.3 Verify all touch functionality:
  - ✅ Touch coordinates accurate
  - ✅ Toggle works (Color ↔ BER)
  - ✅ LED feedback works
  - ✅ **No panics**

### Phase 4: Demo Cleanup (Optional Future Work)

- [ ] 4.1 Extract display initialization into reusable helper
  - Consider `DisplaySdram` pattern from other examples
  - Make LCD controller selection cleaner

- [ ] 4.2 Add proper error types
  - Define `LcdError` enum if needed
  - Use `?` operator instead of `unwrap()/expect()`

- [ ] 4.3 Add documentation comments
  - Explain board revision differences
  - Document touch behavior
  - Add usage examples

- [ ] 4.4 Consider feature flags
  - `nt35510-only` for B08 boards
  - `otm8009a-only` for older boards
  - `touch` for optional touch support

## Success Criteria

| Criteria | Status |
|----------|--------|
| Touch works without panics | ✅ VERIFIED |
| Clean API usage | ✅ No workarounds |
| Code follows HAL patterns | ✅ Matches style |
| Documented | ⏳ Phase 4 |

## API Migration Summary

| Old (ft6x06 v0.1.2) | New (ft6x06-rs v0.3.0) |
|---------------------|------------------------|
| `use ft6x06::Ft6X06` | `use ft6x06_rs::FT6x06` |
| `Ft6X06::new(&i2c, addr, irq_pin)` | `FT6x06::new(i2c)` |
| `touch.detect_touch(&mut i2c)` | `touch.get_touch_event()` |
| `touch.get_touch(&mut i2c, 1)` | `event.primary_point` |
| `touch.td_status(&mut i2c)` | Not needed |
| `touch.ts_calibration()` | Not available |
| Multiple I2C addresses | Only 0x38 supported |

## Risk Mitigation

| Risk | Mitigation | Status |
|------|------------|--------|
| ft6x06-rs incompatible with embedded-hal version | Pin to compatible version | ✅ Compatible |
| API too different | Keep old code as backup | ✅ Migrated successfully |
| Touch still fails | Report issue upstream | ✅ Works perfectly |

## Notes

- ~~Keep the `td_status()` workaround as a fallback~~ → Not needed!
- Legacy address (0x2A) no longer supported - B08 boards only
- I2C ownership transfer is the key architectural change
