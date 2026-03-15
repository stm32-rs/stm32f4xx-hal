# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# Embedded-Graphics Hello World on STM32F469I-DISCO

## TL;DR

> **Quick Summary**: Add `LtdcFramebuffer` struct with `DrawTarget` impl to `src/ltdc.rs` behind a `framebuffer` feature flag, implement 5 missing `DisplayController` methods that 4 broken examples depend on, fix an ARGB4444 bug, register all examples in `Cargo.toml`, and create a new hello-world embedded-graphics example.
> 
> **Deliverables**:
> - `LtdcFramebuffer<T>` struct + `DrawTarget` impl in `src/ltdc.rs` (feature-gated)
> - `DisplayController::new_dsi()` constructor (no pins, no HSE/PLL — for DSI-driven boards)
> - `DisplayController::layer_buffer_mut()` — borrow layer buffer for direct pixel access
> - `DisplayController::set_layer_transparency()` — set layer global alpha
> - `DisplayController::set_layer_buffer_address()` — change layer framebuffer pointer
> - `DisplayController::set_color_keying()` — enable/set color key on a layer
> - ARGB4444 bytes-per-pixel bug fix (16 → 2)
> - 5 `[[example]]` entries in `Cargo.toml`
> - New `examples/f469disco-hello-eg.rs` — renders text + shapes via embedded-graphics
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 3 waves
> **Critical Path**: Task 1 (Cargo.toml) → Task 2 (ltdc.rs APIs) → Task 3 (LtdcFramebuffer) → Task 5 (hello-eg example) → Task 7 (cargo check)

---

## Context

### Original Request
Get embedded-graphics working on the STM32F469I-DISCO display. The board already has simple touch and display working via DSI. Goal: create a hello-world example rendering text/shapes with embedded-graphics, using a HAL-level `DrawTarget` implementation.

### Interview Summary
**Key Discussions**:
- **Scope**: HAL crate implementation + fix broken examples + new hello-world example
- **Location**: `LtdcFramebuffer` and `DrawTarget` live in `src/ltdc.rs`, not example code
- **Feature gating**: Behind `framebuffer` feature flag using `embedded-graphics-core` as optional dep
- **Color conversion**: Use `IntoStorage` trait (not manual bit-packing like the F7 example)
- **Ownership**: `LtdcFramebuffer` owns `&'static mut [T]` independently, with `into_inner()` to release

**Research Findings**:
- 4 broken examples reference 6 non-existent APIs — they were written ahead of the HAL impl
- `examples/ltdc-screen/screen.rs` has a working `DrawTarget` pattern but uses manual RGB565 bit-packing — we improve on this with `IntoStorage`
- `display_init.rs` calls `new_dsi()` at line 192 which doesn't exist — only `new()` exists (requires pins + HSE)
- ARGB4444 byte_per_pixel is 16 instead of 2 at `src/ltdc.rs:427` — obvious bug
- `embedded-graphics 0.8.1` already in dev-deps; need `embedded-graphics-core 0.4` as optional HAL dep

### Metis Review
**Identified Gaps** (addressed):
- Ownership model clarified: `LtdcFramebuffer` is standalone, not tied to `DisplayController`
- `layer_buffer_mut()` returns `Option<&mut [T]>` (safe borrow), not `Option<&'static mut [T]>`
- `DrawTarget::Error = core::convert::Infallible` — drawing to a framebuffer buffer can't fail
- Bounds-check must silently discard out-of-bounds pixels (DrawTarget contract)
- No DMA2D optimizations in this PR (keep it simple)

---

## Work Objectives

### Core Objective
Enable embedded-graphics rendering on the STM32F469I-DISCO display by adding `DrawTarget` support to the HAL crate and fixing all broken examples.

### Concrete Deliverables
- Modified `src/ltdc.rs` with `LtdcFramebuffer`, `DrawTarget`, and 5 new `DisplayController` methods
- Modified `Cargo.toml` with `embedded-graphics-core` dep, `framebuffer` feature, and 5 example entries
- New `examples/f469disco-hello-eg.rs`
- All 5 f469disco examples compile with `cargo check`

### Definition of Done
- [ ] `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-hello-eg` passes
- [ ] `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-paint` passes
- [ ] `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-image-slider` passes
- [ ] `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-animated-layers` passes
- [ ] `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-slideshow` passes
- [ ] `cargo check --target thumbv7em-none-eabihf --features="stm32f469,defmt" --example f469disco-lcd-test` still passes (no regression)

### Must Have
- `LtdcFramebuffer<T>` with `DrawTarget` impl using `IntoStorage`
- `new_dsi()` constructor that skips pin config and PLLSAI (DSI host handles clocking)
- `layer_buffer_mut()` returning `Option<&mut [T]>`
- `set_layer_transparency(layer, alpha_u8)`
- `set_layer_buffer_address(layer, address_u32)`
- `set_color_keying(layer, rgb_u32)`
- ARGB4444 bug fix
- All 5 examples registered in Cargo.toml
- Hello-world example with at least text + colored rectangle

### Must NOT Have (Guardrails)
- Do NOT implement DMA2D-accelerated `fill_solid` / `fill_contiguous` (future work)
- Do NOT modify `examples/f469disco-lcd-test.rs` (working example, leave it alone)
- Do NOT add `embedded-graphics` (full crate) as HAL dependency — use `embedded-graphics-core` only
- Do NOT change the `display_init.rs` API beyond what's needed (it already calls `new_dsi()` correctly)
- Do NOT add lifetime parameters to `DisplayController` — keep `&'static mut [T]` ownership pattern
- Do NOT touch `examples/f469disco/nt35510.rs` or `examples/f469disco/images.rs` (they're fine)
- Do NOT use manual RGB565 bit-packing — use `IntoStorage` trait
- Do NOT make `LtdcFramebuffer` depend on `DisplayController` — they are independent types

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (no_std embedded target — no test runner)
- **Automated tests**: None (cross-compiled `no_std` — cannot run unit tests)
- **Framework**: None
- **Verification method**: `cargo check` compilation verification + code review

### QA Policy
Every task MUST include agent-executed QA scenarios. For this embedded project, verification is compilation-based:
- **All tasks**: `cargo check --target thumbv7em-none-eabihf` with appropriate features
- **Evidence**: Save cargo check output to `.sisyphus/evidence/task-{N}-*.txt`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — independent changes):
├── Task 1: Cargo.toml changes (dep + feature + examples) [quick]
├── Task 2: DisplayController new methods (new_dsi, layer_buffer_mut, set_layer_transparency, set_layer_buffer_address, set_color_keying) + ARGB4444 bug fix [deep]
└── Task 3: LtdcFramebuffer struct + DrawTarget impl [deep]

Wave 2 (Examples — depend on Wave 1):
├── Task 4: Fix all 4 broken examples (verify they now compile) [quick]
└── Task 5: Create f469disco-hello-eg.rs [unspecified-high]

Wave 3 (Verification — depends on everything):
└── Task 6: Full compilation check of ALL examples + regression check [quick]

Wave FINAL (Independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Compilation QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 2 → Task 3 → Task 5 → Task 6 → F1-F4
Parallel Speedup: Wave 1 runs 3 tasks in parallel, Wave 2 runs 2 in parallel
Max Concurrent: 3 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 2, 3, 4, 5 |
| 2 | 1 | 4, 5, 6 |
| 3 | 1 | 4, 5, 6 |
| 4 | 1, 2, 3 | 6 |
| 5 | 1, 2, 3 | 6 |
| 6 | 4, 5 | F1-F4 |
| F1-F4 | 6 | — |

### Agent Dispatch Summary

- **Wave 1**: 3 tasks — T1 → `quick`, T2 → `deep`, T3 → `deep`
- **Wave 2**: 2 tasks — T4 → `quick`, T5 → `unspecified-high`
- **Wave 3**: 1 task — T6 → `quick`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. Cargo.toml: Add `embedded-graphics-core` dependency, `framebuffer` feature, and `[[example]]` entries

  **What to do**:
  1. Add `embedded-graphics-core` as optional dependency in `[dependencies]` section:
     ```toml
     [dependencies.embedded-graphics-core]
     version = "0.4"
     optional = true
     ```
  2. Add `framebuffer` feature in `[features]` section (after the chip features, near other optional feature flags):
     ```toml
     framebuffer = ["dep:embedded-graphics-core"]
     ```
  3. Add 5 `[[example]]` entries at the end of the file (after the existing `fmc-sdram` entry):
     ```toml
     [[example]]
     name = "f469disco-paint"
     required-features = ["stm32f469", "stm32-fmc", "framebuffer", "defmt"]

     [[example]]
     name = "f469disco-image-slider"
     required-features = ["stm32f469", "stm32-fmc", "defmt"]

     [[example]]
     name = "f469disco-animated-layers"
     required-features = ["stm32f469", "stm32-fmc", "defmt"]

     [[example]]
     name = "f469disco-slideshow"
     required-features = ["stm32f469", "stm32-fmc", "defmt"]

     [[example]]
     name = "f469disco-hello-eg"
     required-features = ["stm32f469", "stm32-fmc", "framebuffer", "defmt"]
     ```
  Note: `f469disco-paint` and `f469disco-hello-eg` need `framebuffer` because they use `LtdcFramebuffer`. The other 3 examples (image-slider, animated-layers, slideshow) do NOT use `LtdcFramebuffer` — they use `DisplayController` methods directly, so they don't need `framebuffer`.

  **Must NOT do**:
  - Do NOT add `embedded-graphics` (full crate) as a HAL dependency — only `embedded-graphics-core`
  - Do NOT modify existing `[[example]]` entries
  - Do NOT add `framebuffer` to any chip feature set (it's independent)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple TOML additions — 3 edits to known locations
  - **Skills**: `[]`
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed — no git operations

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Tasks 2, 3, 4, 5, 6
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `Cargo.toml:91-93` — Example of optional dependency format (`stm32_i2s_v12x`)
  - `Cargo.toml:626-628` — Existing `f469disco-lcd-test` example entry (pattern to follow)
  - `Cargo.toml:790-792` — Existing `fmc-sdram` example entry (last entry, insert after this)

  **API/Type References**:
  - `Cargo.toml:131-155` — Features section (where to add `framebuffer` feature)
  - `Cargo.toml:154` — `stm32f469` feature definition (note: does NOT auto-enable `ltdc` — that's only on `stm32f429/439`)

  **External References**:
  - `embedded-graphics-core` crate on crates.io — version 0.4.x is current stable

  **WHY Each Reference Matters**:
  - `Cargo.toml:91-93`: Copy the `[dependencies.X]` + `optional = true` pattern exactly
  - `Cargo.toml:626-628`: Match the existing `[[example]]` format for consistency
  - `Cargo.toml:131-155`: Insert `framebuffer` feature near line 155 after chip features

  **Acceptance Criteria**:
  - [ ] `embedded-graphics-core` appears in `[dependencies]` with `optional = true`
  - [ ] `framebuffer` feature exists with `["dep:embedded-graphics-core"]`
  - [ ] 5 `[[example]]` entries present for f469disco-paint, f469disco-image-slider, f469disco-animated-layers, f469disco-slideshow, f469disco-hello-eg

  **QA Scenarios:**

  ```
  Scenario: Feature flag parses correctly
    Tool: Bash
    Steps:
      1. Run: cargo metadata --format-version=1 --features=framebuffer 2>&1 | head -1
      2. Assert: exit code 0 (no TOML parse errors)
    Expected Result: cargo metadata succeeds without error
    Failure Indicators: TOML parse error, unknown feature, dependency resolution failure
    Evidence: .sisyphus/evidence/task-1-feature-parse.txt

  Scenario: Example entries are registered
    Tool: Bash
    Steps:
      1. Run: grep -c 'f469disco' Cargo.toml
      2. Assert: count includes at least 6 matches (5 new + 1 existing lcd-test)
    Expected Result: All 6 f469disco examples registered
    Failure Indicators: Missing [[example]] entries
    Evidence: .sisyphus/evidence/task-1-example-entries.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `feat(ltdc): add embedded-graphics-core dep and framebuffer feature`
  - Files: `Cargo.toml`

- [x] 2. DisplayController: Add `new_dsi()`, `layer_buffer_mut()`, `set_layer_transparency()`, `set_layer_buffer_address()`, `set_color_keying()` + fix ARGB4444 bug

  **What to do**:

  All changes in `src/ltdc.rs` inside the `impl<T: 'static + SupportedWord> DisplayController<T>` block (after line 208).

  **A. Fix ARGB4444 bug** (line 427):
  Change `PixelFormat::ARGB4444 => 16` to `PixelFormat::ARGB4444 => 2` in the `byte_per_pixel` match in `config_layer()`. ARGB4444 is 16 bits = 2 bytes per pixel.

  **B. Add `new_dsi()` constructor** (insert after `new()` at ~line 353):
  ```rust
  /// Create a DisplayController for DSI-driven displays.
  ///
  /// Unlike [`new()`](Self::new), this constructor does not configure LTDC pins
  /// or PLLSAI. On DSI boards the DSI host drives the pixel clock and data
  /// lines, so LTDC only needs its timing registers set up.
  pub fn new_dsi(
      ltdc: LTDC,
      dma2d: DMA2D,
      pixel_format: PixelFormat,
      config: DisplayConfig,
  ) -> DisplayController<T> {
      // Safety: enable and reset LTDC and DMA2D peripherals
      unsafe {
          LTDC::enable_unchecked();
          LTDC::reset_unchecked();
          DMA2D::enable_unchecked();
          DMA2D::reset_unchecked();
      }

      // Screen constants
      let total_width: u16 =
          config.h_sync + config.h_back_porch + config.active_width + config.h_front_porch - 1;
      let total_height: u16 =
          config.v_sync + config.v_back_porch + config.active_height + config.v_front_porch - 1;

      // Configure LTDC timing registers (same as new())
      ltdc.sscr().write(|w| {
          w.hsw().set(config.h_sync - 1);
          w.vsh().set(config.v_sync - 1)
      });
      ltdc.bpcr().write(|w| {
          w.ahbp().set(config.h_sync + config.h_back_porch - 1);
          w.avbp().set(config.v_sync + config.v_back_porch - 1)
      });
      ltdc.awcr().write(|w| {
          w.aaw().set(config.h_sync + config.h_back_porch + config.active_width - 1);
          w.aah().set(config.v_sync + config.v_back_porch + config.active_height - 1)
      });
      ltdc.twcr().write(|w| {
          w.totalw().set(total_width);
          w.totalh().set(total_height)
      });

      // Configure LTDC signals polarity
      ltdc.gcr().write(|w| {
          w.hspol().bit(config.h_sync_pol);
          w.vspol().bit(config.v_sync_pol);
          w.depol().bit(config.no_data_enable_pol);
          w.pcpol().bit(config.pixel_clock_pol)
      });

      // Background color
      ltdc.bccr().write(|w| unsafe { w.bits(0xAAAAAAAA) });

      // Reload and enable
      ltdc.srcr().modify(|_, w| w.imr().set_bit());
      ltdc.gcr().modify(|_, w| w.ltdcen().set_bit().den().set_bit());
      ltdc.srcr().modify(|_, w| w.imr().set_bit());

      DisplayController {
          _ltdc: ltdc,
          _dma2d: dma2d,
          config,
          buffer1: None,
          buffer2: None,
          pixel_format,
      }
  }
  ```
  Key difference from `new()`: NO pin configuration, NO PLLSAI setup. The DSI host (initialized separately) provides the pixel clock. Only LTDC timing + polarity + enable.

  **C. Add `layer_buffer_mut()`** (insert after `draw_pixel()`):
  ```rust
  /// Get a mutable reference to the layer's framebuffer.
  ///
  /// Returns `None` if the layer has not been configured with [`config_layer()`](Self::config_layer).
  pub fn layer_buffer_mut(&mut self, layer: Layer) -> Option<&mut [T]> {
      match layer {
          Layer::L1 => self.buffer1.as_deref_mut(),
          Layer::L2 => self.buffer2.as_deref_mut(),
      }
  }
  ```

  **D. Add `set_layer_transparency()`** (insert after `layer_buffer_mut()`):
  ```rust
  /// Set the global alpha (transparency) for a layer.
  ///
  /// `alpha`: 0 = fully transparent, 255 = fully opaque.
  /// Takes effect after [`reload()`](Self::reload).
  pub fn set_layer_transparency(&self, layer: Layer, alpha: u8) {
      self._ltdc
          .layer(layer as usize)
          .cacr()
          .write(|w| w.consta().set(alpha as u16));
      self.reload();
  }
  ```
  Note: calls `self.reload()` for immediate effect (matches how the examples use it in tight animation loops).

  **E. Add `set_layer_buffer_address()`** (insert after `set_layer_transparency()`):
  ```rust
  /// Change the framebuffer address for a layer.
  ///
  /// This can be used for double-buffering by swapping between two
  /// pre-allocated framebuffers. Takes effect after [`reload()`](Self::reload).
  pub fn set_layer_buffer_address(&self, layer: Layer, address: u32) {
      self._ltdc
          .layer(layer as usize)
          .cfbar()
          .write(|w| w.cfbadd().set(address));
      self.reload();
  }
  ```

  **F. Add `set_color_keying()`** (insert after `set_layer_buffer_address()`):
  ```rust
  /// Enable color keying on a layer.
  ///
  /// Pixels matching `color_key` (RGB888 format, 24-bit) become fully
  /// transparent, allowing the layer below to show through.
  /// Takes effect after [`reload()`](Self::reload).
  pub fn set_color_keying(&mut self, layer: Layer, color_key: u32) {
      let l = self._ltdc.layer(layer as usize);
      l.ckcr().write(|w| unsafe { w.bits(color_key & 0x00FF_FFFF) });
      l.cr().modify(|_, w| w.colken().set_bit());
      self.reload();
  }
  ```

  **Must NOT do**:
  - Do NOT modify the existing `new()` constructor
  - Do NOT change `DisplayController` struct fields (no new fields needed)
  - Do NOT add DMA2D optimizations
  - Do NOT change `config_layer()` signature

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Multiple method implementations in embedded HAL code requiring register-level accuracy
  - **Skills**: `[]`
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed — no git operations in this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Tasks 4, 5, 6
  - **Blocked By**: Task 1 (Cargo.toml must exist with correct deps first)

  **References**:

  **Pattern References**:
  - `src/ltdc.rs:208-353` — Existing `new()` constructor. `new_dsi()` duplicates the timing/polarity/enable parts but skips PLLSAI setup (lines 239-301) and pin handling (line 213: `_pins: Option<LtdcPins>`)
  - `src/ltdc.rs:361-468` — `config_layer()` method. Shows register access pattern for layers: `self._ltdc.layer(layer as usize)` to get layer registers, then `.cacr()`, `.cfbar()`, `.ckcr()`, `.cr()` etc.
  - `src/ltdc.rs:479-488` — `draw_pixel()` method. Shows `buffer1`/`buffer2` access pattern: `match layer { Layer::L1 => self.buffer1.as_mut().unwrap(), ... }`
  - `src/ltdc.rs:549-553` — `reload()` method. Called by the new methods for immediate effect.

  **API/Type References**:
  - `src/ltdc.rs:193-206` — `DisplayController<T>` struct fields. New methods use `_ltdc`, `buffer1`, `buffer2`, `config`.
  - `src/ltdc.rs:40-43` — `Layer` enum (`L1 = 0`, `L2 = 1`). Used as `layer as usize` for register indexing.
  - `src/ltdc.rs:562-571` — `PixelFormat` enum. Referenced in bug fix.

  **WHY Each Reference Matters**:
  - `src/ltdc.rs:208-353`: Copy timing register setup code for `new_dsi()`, but omit lines 239-301 (PLLSAI) and the `_pins`/`hse` parameters.
  - `src/ltdc.rs:361-468`: Understand the layer register access pattern (`self._ltdc.layer(...)`) used by new methods.
  - `src/ltdc.rs:427`: Bug line — change `16` to `2` for ARGB4444 bytes per pixel.

  **Acceptance Criteria**:
  - [ ] `PixelFormat::ARGB4444 => 2` at the byte_per_pixel match (was 16)
  - [ ] `new_dsi()` method exists and compiles
  - [ ] `layer_buffer_mut()` method exists returning `Option<&mut [T]>`
  - [ ] `set_layer_transparency()` method exists and writes `cacr` register
  - [ ] `set_layer_buffer_address()` method exists and writes `cfbar` register
  - [ ] `set_color_keying()` method exists, writes `ckcr` and enables `colken`

  **QA Scenarios:**

  ```
  Scenario: ltdc.rs compiles with stm32f469 feature
    Tool: Bash
    Steps:
      1. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469" 2>&1
      2. Assert: exit code 0
    Expected Result: Zero compilation errors for the ltdc module
    Failure Indicators: Any error mentioning ltdc.rs, DisplayController, or new method names
    Evidence: .sisyphus/evidence/task-2-ltdc-compile.txt

  Scenario: ARGB4444 bug is fixed
    Tool: Bash
    Steps:
      1. Run: grep 'ARGB4444 =>' src/ltdc.rs
      2. Assert: all matches show `=> 2` for the byte_per_pixel context (not `=> 16`)
    Expected Result: ARGB4444 maps to 2 bytes per pixel in config_layer()
    Failure Indicators: Any line showing `ARGB4444 => 16` in byte_per_pixel match
    Evidence: .sisyphus/evidence/task-2-argb4444-fix.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `feat(ltdc): add DisplayController DSI methods and fix ARGB4444 bug`
  - Files: `src/ltdc.rs`

---

- [x] 3. LtdcFramebuffer struct + DrawTarget impl (feature-gated)

  **What to do**:

  Add to `src/ltdc.rs`, OUTSIDE the `impl DisplayController` block (after line 554, near the end of the file, before `PixelFormat` enum). Gate everything behind `#[cfg(feature = "framebuffer")]`.

  **A. Add conditional import at top of file** (after existing `use` statements, ~line 13):
  ```rust
  #[cfg(feature = "framebuffer")]
  use embedded_graphics_core::{
      draw_target::DrawTarget,
      geometry::{OriginDimensions, Size},
      pixelcolor::{IntoStorage, Rgb565},
      Pixel,
  };
  ```

  **B. Add `LtdcFramebuffer` struct** (after `DisplayController` impl block):
  ```rust
  /// A framebuffer wrapper that implements [`DrawTarget`] for use with
  /// `embedded-graphics`.
  ///
  /// `LtdcFramebuffer` owns a `&'static mut [T]` SDRAM buffer and provides
  /// pixel-level drawing via the `embedded-graphics` `DrawTarget` trait.
  ///
  /// # Usage
  ///
  /// ```ignore
  /// let mut fb = LtdcFramebuffer::new(buffer, 480, 800);
  /// fb.clear(Rgb565::BLACK).ok();
  /// // ... draw with embedded-graphics ...
  /// let buffer = fb.into_inner();
  /// display_ctrl.config_layer(Layer::L1, buffer, PixelFormat::RGB565);
  /// ```
  #[cfg(feature = "framebuffer")]
  pub struct LtdcFramebuffer<T: 'static + SupportedWord> {
      buf: &'static mut [T],
      width: u16,
      height: u16,
  }
  ```

  **C. Add impl block with `new()` and `into_inner()`**:
  ```rust
  #[cfg(feature = "framebuffer")]
  impl<T: 'static + SupportedWord> LtdcFramebuffer<T> {
      /// Create a new framebuffer wrapper.
      ///
      /// # Panics
      ///
      /// Panics if `buf.len() != width * height`.
      pub fn new(buf: &'static mut [T], width: u16, height: u16) -> Self {
          assert!(buf.len() == (width as usize) * (height as usize));
          Self { buf, width, height }
      }

      /// Consume the framebuffer and return the underlying buffer.
      pub fn into_inner(self) -> &'static mut [T] {
          self.buf
      }
  }
  ```

  **D. Add `DrawTarget` impl for `LtdcFramebuffer<u16>`** (RGB565 only):
  ```rust
  #[cfg(feature = "framebuffer")]
  impl DrawTarget for LtdcFramebuffer<u16> {
      type Color = Rgb565;
      type Error = core::convert::Infallible;

      fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
      where
          I: IntoIterator<Item = Pixel<Self::Color>>,
      {
          let w = self.width as i32;
          let h = self.height as i32;
          for Pixel(coord, color) in pixels.into_iter() {
              let (x, y): (i32, i32) = coord.into();
              if x >= 0 && x < w && y >= 0 && y < h {
                  self.buf[y as usize * self.width as usize + x as usize] = color.into_storage();
              }
          }
          Ok(())
      }
  }
  ```
  Key: uses `color.into_storage()` (not manual bit-packing). Silently discards out-of-bounds pixels (DrawTarget contract).

  **E. Add `OriginDimensions` impl**:
  ```rust
  #[cfg(feature = "framebuffer")]
  impl<T: 'static + SupportedWord> OriginDimensions for LtdcFramebuffer<T> {
      fn size(&self) -> Size {
          Size::new(self.width as u32, self.height as u32)
      }
  }
  ```

  **Must NOT do**:
  - Do NOT implement `DrawTarget` for `u32` or `u8` — only `u16` (RGB565) for now
  - Do NOT add DMA2D-accelerated `fill_solid` / `fill_contiguous` overrides
  - Do NOT make `LtdcFramebuffer` depend on `DisplayController`
  - Do NOT use manual RGB565 bit-packing — use `IntoStorage`

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Trait implementation requiring correct embedded-graphics API conformance
  - **Skills**: `[]`
  - **Skills Evaluated but Omitted**:
    - `git-master`: Not needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Tasks 4, 5, 6
  - **Blocked By**: Task 1 (Cargo.toml must have the `embedded-graphics-core` dependency)

  **References**:

  **Pattern References**:
  - `examples/ltdc-screen/screen.rs:48-78` — Existing `DrawTarget` impl for `Stm32F7DiscoDisplay<u16>`. Our implementation follows the same structure but uses `IntoStorage` instead of manual bit-packing (lines 61-63). Also uses the same `core::convert::Infallible` error type (line 50) and same bounds-check pattern (line 60).
  - `src/ltdc.rs:193-206` — `DisplayController<T>` struct. `LtdcFramebuffer` uses the same `T: 'static + SupportedWord` bounds and `&'static mut [T]` buffer type.
  - `examples/f469disco-paint.rs:121-134` — How the example uses `LtdcFramebuffer`: `new()`, `clear()`, `fill_solid()`, `into_inner()`. This is the API contract we must satisfy.

  **API/Type References**:
  - `src/ltdc.rs:573-576` — `SupportedWord` trait and impls (`u8`, `u16`, `u32`). Used as bounds.
  - `embedded-graphics-core` API: `DrawTarget` trait requires `type Color`, `type Error`, `fn draw_iter()`. `OriginDimensions` requires `fn size() -> Size`.

  **External References**:
  - `embedded-graphics-core` DrawTarget docs: https://docs.rs/embedded-graphics-core/0.4.0/embedded_graphics_core/draw_target/trait.DrawTarget.html
  - `IntoStorage` docs: https://docs.rs/embedded-graphics-core/0.4.0/embedded_graphics_core/pixelcolor/trait.IntoStorage.html

  **WHY Each Reference Matters**:
  - `screen.rs:48-78`: Proves the pattern works. We improve on it with `IntoStorage`.
  - `f469disco-paint.rs:121-134`: This is our primary consumer — the API must match what paint expects.

  **Acceptance Criteria**:
  - [ ] `LtdcFramebuffer<T>` struct exists with `buf`, `width`, `height` fields
  - [ ] `LtdcFramebuffer::new()` panics on wrong buffer size
  - [ ] `LtdcFramebuffer::into_inner()` returns `&'static mut [T]`
  - [ ] `DrawTarget for LtdcFramebuffer<u16>` uses `color.into_storage()`
  - [ ] `DrawTarget` silently discards out-of-bounds pixels (no panic)
  - [ ] `OriginDimensions` returns correct `Size`
  - [ ] All gated behind `#[cfg(feature = "framebuffer")]`

  **QA Scenarios:**

  ```
  Scenario: LtdcFramebuffer compiles with framebuffer feature
    Tool: Bash
    Steps:
      1. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,framebuffer" 2>&1
      2. Assert: exit code 0, no errors mentioning LtdcFramebuffer or DrawTarget
    Expected Result: Module compiles cleanly with the feature enabled
    Failure Indicators: Missing trait imports, type mismatches, unresolved `IntoStorage`
    Evidence: .sisyphus/evidence/task-3-framebuffer-compile.txt

  Scenario: LtdcFramebuffer is NOT compiled without framebuffer feature
    Tool: Bash
    Steps:
      1. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469" 2>&1
      2. Assert: exit code 0 (compiles without framebuffer feature)
      3. Run: grep 'LtdcFramebuffer' in compiled output should show nothing
    Expected Result: Feature gating works — no embedded-graphics-core dependency without feature
    Failure Indicators: Compilation error about missing embedded-graphics-core
    Evidence: .sisyphus/evidence/task-3-no-feature.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `feat(ltdc): add LtdcFramebuffer with DrawTarget impl`
  - Files: `src/ltdc.rs`

- [x] 4. Verify all 4 broken examples compile (no code changes needed if Tasks 1-3 are correct)

  **What to do**:

  The 4 broken examples (`f469disco-paint`, `f469disco-image-slider`, `f469disco-animated-layers`, `f469disco-slideshow`) reference APIs that Tasks 1-3 implement. This task verifies they compile. If any fail, diagnose and fix the issue.

  1. Run `cargo check` for each of the 4 examples individually
  2. If any fail, read the error carefully:
     - Missing import in the example? → Check what the example imports vs what we export
     - Method signature mismatch? → Fix the method in `src/ltdc.rs` to match example usage
     - Type mismatch? → Check `consta().set()` accepts `u16` vs `u8`, etc.
  3. Do NOT change the examples unless absolutely necessary (they define the API contract)
  4. If an example uses an API pattern we haven't implemented, add the missing piece to `src/ltdc.rs`

  **Specific things to verify per example**:
  - `f469disco-paint.rs`: Uses `LtdcFramebuffer::new()`, `.clear()`, `.fill_solid()`, `.into_inner()`, `display_ctrl.layer_buffer_mut(Layer::L1)`, and `display_init::init_ltdc_rgb565()` which calls `new_dsi()`
  - `f469disco-image-slider.rs`: Uses `display_ctrl.set_layer_buffer_address(Layer::L1, addr_u32)`, `init_ltdc_rgb565()` → `new_dsi()`
  - `f469disco-animated-layers.rs`: Uses `display_ctrl.layer_buffer_mut(Layer::L2)`, `display_ctrl.set_color_keying(Layer::L2, 0x000000)`, `init_ltdc_rgb565()` → `new_dsi()`
  - `f469disco-slideshow.rs`: Uses `display_ctrl.layer_buffer_mut(Layer::L1)`, `display_ctrl.layer_buffer_mut(Layer::L2)`, `display_ctrl.set_layer_transparency(Layer::L2, alpha_u8)`, `init_ltdc_rgb565()` → `new_dsi()`

  **Must NOT do**:
  - Do NOT rewrite examples — they are the specification
  - Do NOT modify `display_init.rs` (it already calls `new_dsi()` correctly)
  - Do NOT touch `f469disco-lcd-test.rs`

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Primarily compilation verification; fixes (if any) are small signature adjustments
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 5)
  - **Blocks**: Task 6
  - **Blocked By**: Tasks 1, 2, 3

  **References**:

  **Pattern References**:
  - `examples/f469disco-paint.rs:27-28` — Imports `ltdc::LtdcFramebuffer` from HAL. Verifies our export path.
  - `examples/f469disco-paint.rs:121-134` — `LtdcFramebuffer` usage: `new(buffer, WIDTH, HEIGHT)`, `.clear()`, `.fill_solid()`, `.into_inner()`
  - `examples/f469disco-paint.rs:182` — `display_ctrl.layer_buffer_mut(Layer::L1)` call site
  - `examples/f469disco-image-slider.rs:177,182` — `display_ctrl.set_layer_buffer_address(Layer::L1, buf_b_addr)` call sites
  - `examples/f469disco-animated-layers.rs:126` — `display_ctrl.set_color_keying(Layer::L2, 0x000000)` call site
  - `examples/f469disco-animated-layers.rs:139` — `display_ctrl.layer_buffer_mut(Layer::L2)` call site
  - `examples/f469disco-slideshow.rs:116-117` — `display_ctrl.set_layer_transparency(Layer::L1, 255)` call sites
  - `examples/f469disco-slideshow.rs:138,150` — `display_ctrl.layer_buffer_mut()` call sites
  - `examples/f469disco/display_init.rs:192` — `DisplayController::<u16>::new_dsi(ltdc, dma2d, PixelFormat::RGB565, DISPLAY_CONFIG)` call site

  **WHY Each Reference Matters**:
  - These are the actual call sites that define the required method signatures. If compilation fails, compare call site arguments against our method signatures.

  **Acceptance Criteria**:
  - [ ] `cargo check --example f469disco-paint` passes
  - [ ] `cargo check --example f469disco-image-slider` passes
  - [ ] `cargo check --example f469disco-animated-layers` passes
  - [ ] `cargo check --example f469disco-slideshow` passes

  **QA Scenarios:**

  ```
  Scenario: All 4 broken examples now compile
    Tool: Bash
    Steps:
      1. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-paint 2>&1
      2. Assert: exit code 0
      3. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,defmt" --example f469disco-image-slider 2>&1
      4. Assert: exit code 0
      5. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,defmt" --example f469disco-animated-layers 2>&1
      6. Assert: exit code 0
      7. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,defmt" --example f469disco-slideshow 2>&1
      8. Assert: exit code 0
    Expected Result: All 4 examples compile with zero errors
    Failure Indicators: Any `error[E...]` in output mentioning missing methods, wrong types, or unresolved imports
    Evidence: .sisyphus/evidence/task-4-examples-compile.txt

  Scenario: Existing lcd-test example still works (regression)
    Tool: Bash
    Steps:
      1. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,defmt" --example f469disco-lcd-test 2>&1
      2. Assert: exit code 0
    Expected Result: No regression on existing working example
    Failure Indicators: Any compilation error in lcd-test
    Evidence: .sisyphus/evidence/task-4-lcd-test-regression.txt
  ```

  **Commit**: NO (verification only — no new files if everything works)

---

- [x] 5. Create `examples/f469disco-hello-eg.rs` — embedded-graphics hello world

  **What to do**:

  Create a NEW file `examples/f469disco-hello-eg.rs` that renders text and shapes on the STM32F469I-DISCO display using `LtdcFramebuffer` and `embedded-graphics`.

  The example structure follows the pattern from `f469disco-paint.rs` (same board setup, SDRAM init, DSI init, LTDC config) but the main loop renders static embedded-graphics content instead of touch-driven painting.

  **Example outline**:
  ```rust
  //! STM32F469I-DISCO embedded-graphics hello world.
  //!
  //! Renders text and colored shapes on the DSI display using the HAL
  //! `LtdcFramebuffer` `DrawTarget` implementation.
  //!
  //! Build:
  //! ```bash
  //! cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
  //! ```
  
  #![deny(warnings)]
  #![no_main]
  #![no_std]
  
  use cortex_m_rt::entry;
  use defmt_rtt as _;
  use panic_probe as _;
  
  use stm32f4xx_hal::{self as hal, rcc::Config};
  use hal::{
      fmc::FmcExt,
      gpio::alt::fmc as fmc_alt,
      ltdc::{Layer, LtdcFramebuffer},
      pac::{CorePeripherals, Peripherals},
      prelude::*,
  };
  
  use embedded_graphics::{
      mono_font::{ascii::FONT_10X20, MonoTextStyle},
      pixelcolor::Rgb565,
      prelude::*,
      primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
      text::Text,
  };
  
  #[path = "f469disco/display_init.rs"]
  mod display_init;
  use display_init::{FB_SIZE, HEIGHT, WIDTH};
  use stm32_fmc::devices::is42s32400f_6;
  
  // ... same fmc_pins! macro as other f469disco examples ...
  // ... same SDRAM init, DSI init, panel init as paint.rs ...
  // ... then:
  
  // Draw with embedded-graphics
  let mut fb = LtdcFramebuffer::new(buffer, WIDTH, HEIGHT);
  fb.clear(Rgb565::BLACK).ok();
  
  // Title text
  let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
  Text::new("Hello embedded-graphics!", Point::new(40, 60), text_style)
      .draw(&mut fb).ok();
  
  // Colored rectangle
  Rectangle::new(Point::new(50, 100), Size::new(200, 100))
      .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
      .draw(&mut fb).ok();
  
  // Circle
  Circle::new(Point::new(300, 100), 80)
      .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
      .draw(&mut fb).ok();
  
  // Triangle
  Triangle::new(
      Point::new(50, 400),
      Point::new(200, 300),
      Point::new(350, 400),
  )
  .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
  .draw(&mut fb).ok();
  
  let buffer = fb.into_inner();
  
  // Configure LTDC and display
  let mut display_ctrl = display_init::init_ltdc_rgb565(dp.LTDC, dp.DMA2D);
  display_ctrl.config_layer(Layer::L1, buffer, hal::ltdc::PixelFormat::RGB565);
  display_ctrl.enable_layer(Layer::L1);
  display_ctrl.reload();
  
  defmt::info!("Hello embedded-graphics! Display ready.");
  loop {
      cortex_m::asm::wfi();
  }
  ```

  **Key implementation notes**:
  - Copy the FMC/SDRAM/DSI/panel initialization EXACTLY from `f469disco-paint.rs` (lines 67-143). Do NOT innovate on the init sequence.
  - Use `embedded_graphics` (the full crate, which is a dev-dependency) for rendering primitives and text
  - Import `LtdcFramebuffer` from the HAL crate (it's our new API)
  - The main loop is just `wfi()` — the display content is static (drawn once before LTDC config)
  - This example does NOT need touch support (no I2C, no FT6X06)
  - The example does NOT need `gpiob` (used for I2C in paint example) but still needs all other GPIO ports for FMC pins

  **Must NOT do**:
  - Do NOT add touch support — this is a simple display-only hello world
  - Do NOT use `display_ctrl.draw_pixel()` — draw through `LtdcFramebuffer` only
  - Do NOT import or depend on `images.rs` — this example is self-contained

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: New file creation requiring careful integration with board init code
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 4)
  - **Blocks**: Task 6
  - **Blocked By**: Tasks 1, 2, 3

  **References**:

  **Pattern References**:
  - `examples/f469disco-paint.rs:1-143` — **PRIMARY TEMPLATE**. Copy the entire board init sequence (imports, fmc_pins! macro, SDRAM init, DSI init, panel init). The hello-eg example differs only in what happens after SDRAM is initialized (drawing with embedded-graphics instead of touch loop).
  - `examples/f469disco-paint.rs:121-134` — `LtdcFramebuffer` usage pattern: `new()` → `clear()` → draw → `into_inner()` → `config_layer()`
  - `examples/f469disco-paint.rs:32-37` — `embedded_graphics_core` imports. Hello-eg uses `embedded_graphics` (full crate with `mono_font`, `Text`, primitives) from dev-deps instead.

  **API/Type References**:
  - `embedded_graphics` dev-dep API: `Text::new()`, `Rectangle::new()`, `Circle::new()`, `Triangle::new()`, `MonoTextStyle::new()`, `PrimitiveStyle::with_fill()`
  - `examples/f469disco/display_init.rs:24-26` — `WIDTH=480`, `HEIGHT=800`, `FB_SIZE=384000` constants imported by all examples

  **External References**:
  - embedded-graphics drawing docs: https://docs.rs/embedded-graphics/0.8.1/embedded_graphics/
  - `FONT_10X20` is a built-in monospace font suitable for 480×800 display (10px wide characters)

  **WHY Each Reference Matters**:
  - `f469disco-paint.rs:1-143`: The board init is identical across all f469disco examples. Do not reinvent it.
  - `embedded_graphics` docs: Needed for correct `Text`/`Rectangle`/`Circle`/`Triangle` API usage.

  **Acceptance Criteria**:
  - [ ] File `examples/f469disco-hello-eg.rs` exists
  - [ ] Renders at least: one text string, one rectangle, one circle
  - [ ] Uses `LtdcFramebuffer::new()`, `.clear()`, `.into_inner()`
  - [ ] Uses `embedded_graphics` drawing primitives (not raw pixel writes)
  - [ ] Board init copied from paint.rs (FMC, SDRAM, DSI, panel)
  - [ ] Main loop is `wfi()` (static display)
  - [ ] No touch/I2C code

  **QA Scenarios:**

  ```
  Scenario: Hello-eg example compiles
    Tool: Bash
    Steps:
      1. Run: cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-hello-eg 2>&1
      2. Assert: exit code 0, no errors
    Expected Result: New example compiles cleanly
    Failure Indicators: Missing imports, type mismatches, undefined `LtdcFramebuffer`
    Evidence: .sisyphus/evidence/task-5-hello-eg-compile.txt

  Scenario: Example uses embedded-graphics API correctly
    Tool: Bash
    Steps:
      1. Run: grep -c 'draw(&mut fb)' examples/f469disco-hello-eg.rs
      2. Assert: count >= 3 (text + rectangle + circle at minimum)
      3. Run: grep 'LtdcFramebuffer::new' examples/f469disco-hello-eg.rs
      4. Assert: match found
      5. Run: grep 'into_inner' examples/f469disco-hello-eg.rs
      6. Assert: match found
    Expected Result: Example uses LtdcFramebuffer and draws multiple primitives
    Failure Indicators: Missing draw calls, no LtdcFramebuffer usage
    Evidence: .sisyphus/evidence/task-5-hello-eg-api-usage.txt
  ```

  **Commit**: YES
  - Message: `feat(examples): add f469disco embedded-graphics hello world`
  - Files: `examples/f469disco-hello-eg.rs`

- [x] 6. Full compilation verification of ALL examples + regression check

  **What to do**:

  Run `cargo check` for all 6 examples to verify everything works together. This is the final integration check.

  1. Check all 5 f469disco examples (4 fixed + 1 new):
     ```bash
     cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-hello-eg
     cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-paint
     cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,defmt" --example f469disco-image-slider
     cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,defmt" --example f469disco-animated-layers
     cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,defmt" --example f469disco-slideshow
     ```
  2. Regression check (existing example):
     ```bash
     cargo check --target thumbv7em-none-eabihf --features="stm32f469,defmt" --example f469disco-lcd-test
     ```
  3. Cross-feature isolation check:
     ```bash
     cargo check --target thumbv7em-none-eabihf --features="stm32f407,framebuffer"
     ```
     This verifies that the `framebuffer` feature works on a non-f469 target (just adds the types, no examples needed).

  **Must NOT do**:
  - Do NOT modify any source files in this task (it's verification only)
  - If issues are found, report them for the relevant task to fix

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running cargo check commands and capturing output
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential after Wave 2)
  - **Blocks**: F1, F2, F3, F4
  - **Blocked By**: Tasks 4, 5

  **References**:
  - All example files from Tasks 1-5
  - `Cargo.toml` for feature flags and example entries

  **Acceptance Criteria**:
  - [ ] All 5 f469disco examples compile with zero errors
  - [ ] `f469disco-lcd-test` compiles (regression)
  - [ ] `framebuffer` feature works on stm32f407 target
  - [ ] Zero warnings across all checks

  **QA Scenarios:**

  ```
  Scenario: Full compilation sweep
    Tool: Bash
    Steps:
      1. Run all 7 cargo check commands listed above
      2. Assert: all exit code 0
      3. Count total warnings across all outputs
      4. Assert: 0 warnings
    Expected Result: Clean compilation across all examples and feature combinations
    Failure Indicators: Any error or warning in any check
    Evidence: .sisyphus/evidence/task-6-full-check.txt
  ```

  **Commit**: NO (verification only)

---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt"`. Review all changed files for: `as any`/`#[allow(unused)]` overuse, empty match arms, commented-out code, unused imports. Check for AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Compilation QA** — `unspecified-high`
  Run `cargo check` for ALL 6 examples (5 f469disco + lcd-test regression). Capture full output. Verify zero warnings. Check that `framebuffer` feature doesn't break non-f469 builds: `cargo check --target thumbv7em-none-eabihf --features="stm32f407,framebuffer"`.
  Output: `Examples [N/N pass] | Warnings [N] | Cross-feature [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **After Wave 1**: `feat(ltdc): add LtdcFramebuffer with DrawTarget and DisplayController DSI methods` — src/ltdc.rs, Cargo.toml
- **After Wave 2**: `feat(examples): add f469disco hello-eg example and fix broken examples` — examples/*.rs, Cargo.toml
- **Alternative**: Single commit after Wave 3 if preferred: `feat(ltdc): add embedded-graphics DrawTarget support and f469disco examples`

---

## Success Criteria

### Verification Commands
```bash
# All examples must compile (zero errors, zero warnings)
cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-hello-eg
cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-paint
cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-image-slider
cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-animated-layers
cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt" --example f469disco-slideshow

# Regression: existing example still works
cargo check --target thumbv7em-none-eabihf --features="stm32f469,defmt" --example f469disco-lcd-test

# Feature isolation: framebuffer feature doesn't break other targets
cargo check --target thumbv7em-none-eabihf --features="stm32f407,framebuffer"
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All 6 examples compile
- [ ] No regressions on existing examples
