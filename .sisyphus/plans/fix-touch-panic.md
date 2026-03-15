# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# Fix: FT6x06 Touch Panic on Invalid Count

## TL;DR
> Add guard for invalid touch count (>2) from FT6x06 controller to prevent panic and allow touch to work.

## Context

### Problem
The `ft6x06` crate panics when the touch controller returns `ntouch > 2`:
```
[ERROR] panicked at ft6x06-0.1.2/src/lib.rs:332:9:
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```

This causes the touch feature to freeze immediately when touched.

### Root Cause
The FT6x06 touch controller can return garbage data (touch count > 2). The crate asserts instead of returning an error.

## Work Objectives

- Add validation guard before using touch count
- Treat invalid touch count as an error (skip and continue)
- Log warning when invalid data detected

## TODOs

- [x] 1. Add touch count validation guard

  **File**: `examples/f469disco-lcd-test.rs`
  
  **What to do**:
  Around line 324, change:
  ```rust
  let num = match touch.detect_touch(&mut i2c) {
      Ok(n) => n,
      Err(_) => {
          touch_err_count += 1;
          ...
      }
  };
  ```
  
  To:
  ```rust
  let num = match touch.detect_touch(&mut i2c) {
      Ok(n) => {
          // Guard against invalid touch count from FT6x06 (crate panics if n > 2)
          if n > 2 {
              touch_err_count += 1;
              if touch_err_count % 64 == 1 {
                  defmt::warn!("Invalid touch count {} (err_count={})", n, touch_err_count);
              }
              prev_num = 0;
              touch_hold_ticks = 0;
              delay.delay_ms(20u32);
              continue;
          }
          n
      }
      Err(_) => {
          touch_err_count += 1;
          ...
      }
  };
  ```

- [x] 2. Build and flash to verify

  ```bash
  cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"
  scp target/thumbv7em-none-eabihf/release/examples/f469disco-lcd-test ubuntu@192.168.13.246:/tmp/
  ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-lcd-test && probe-rs reset --chip STM32F469NIHx"
  ```

## Success Criteria
- Touch works without freezing
- Touch toggles between color/BER patterns
- LED lights up while touching
