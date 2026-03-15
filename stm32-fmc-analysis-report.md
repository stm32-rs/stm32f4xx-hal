# STM32-FMC Analysis Report: Relevance to STM32F469 Display Work

**Date:** 2026-03-01  
**Author:** Sisyphus (AI Research Agent)  
**Purpose:** Evaluate `stm32-fmc` crate and its relevance to STM32F469-DISCO display support work

---

## Executive Summary

**Key Finding:** The `stm32-fmc` crate is **already integrated and actively used** in both the HAL (`stm32f4xx-hal`) and the BSP (`stm32f469i-disc`). It is **critical infrastructure** for the F469 display work, providing the SDRAM controller that enables large framebuffers for the 480×800 display.

| Aspect | Status | Recommendation |
|--------|--------|----------------|
| **Already integrated** | ✅ Yes | Keep current integration |
| **Used for display** | ✅ Yes | Display examples use SDRAM framebuffers |
| **Version** | 0.4.0 | Current, maintained |
| **Relevance** | **HIGH** | Essential for display work |

**Action Required:** No new dependency needed. Continue using the existing `stm32-fmc` integration. Focus efforts on optimizing the display pipeline, not on adding FMC support.

---

## 1. What is `stm32-fmc`?

### 1.1 Overview

`stm32-fmc` is a **Hardware Abstraction Layer (HAL)** for the **Flexible Memory Controller (FMC)** and **Flexible Static Memory Controller (FSMC)** found in STM32 microcontrollers.

**Repository:** https://github.com/stm32-rs/stm32-fmc  
**Current Version:** 0.4.0  
**License:** MIT/Apache-2.0  
**Maintenance Status:** Active (embedded-hal 1.0 compatible)

### 1.2 Features Provided

| Feature | Support Level | Description |
|---------|---------------|-------------|
| **SDRAM** | ✅ Production-ready | Full SDRAM initialization, timing configuration, memory access |
| **NAND Flash** | ✅ Supported | NAND flash device support |
| **SRAM/PSRAM** | ⚠️ Partial | Some support via FSMC |
| **NOR Flash** | ⚠️ Partial | Some support via FSMC |
| **LCD (8080/6800)** | Via FSMC | Parallel LCD interfaces via FSMC banks |

### 1.3 Supported STM32 Families

- **STM32F4:** F417, F427, F429, F437, F439, **F446, F469, F479**
- **STM32F7:** Full support
- **STM32G4:** FSMC support
- **STM32H7:** Full FMC support

---

## 2. STM32F469 FMC Hardware Context

### 2.1 STM32F469 FMC Capabilities

The STM32F469 represents the **"Advanced Line"** of the STM32F4 series, specifically optimized for high-end GUIs:

| Feature | STM32F429 | STM32F469 |
|---------|-----------|-----------|
| **Display Interface** | LTDC only (parallel RGB) | LTDC + **MIPI-DSI** |
| **FMC Data Bus** | 32-bit SDRAM | 32-bit SDRAM (Enhanced FIFO) |
| **Internal RAM** | 256 KB | **384 KB** |
| **SDRAM Banks** | 2 | 2 |

### 2.2 STM32F469I-DISCO Board SDRAM

The board includes a **128-Mbit (16 MB) SDRAM** (typically **ISSI IS42S32400F-6**):

| Specification | Value |
|---------------|-------|
| **Size** | 16 MB (128 Mbit) |
| **Bus Width** | 32-bit |
| **Address Space** | `0xC000_0000` (Bank 5) |
| **Speed Grade** | 100 MHz (Grade 6) |
| **CAS Latency** | 3 cycles |
| **Organization** | 4 banks × 12 rows × 8 columns × 32 bits |

### 2.3 Display Memory Requirements

For the 480×800 DSI display:

| Format | Bytes/Pixel | Framebuffer Size | Fits in Internal RAM? |
|--------|-------------|------------------|----------------------|
| RGB565 | 2 | 768 KB | ❌ No (384 KB available) |
| ARGB8888 | 4 | 1.536 MB | ❌ No |
| Double-buffered RGB565 | 2 | 1.536 MB | ❌ No |

**Critical Insight:** The internal SRAM (384 KB) is **insufficient** for any single full-screen framebuffer. **SDRAM via FMC is mandatory** for the F469 display.

---

## 3. Current Integration Status

### 3.1 Integration in `stm32f4xx-hal`

The HAL integrates `stm32-fmc` in `src/fmc.rs`:

```rust
// src/fmc.rs - HAL wrapper for stm32-fmc
use stm32_fmc::FmcPeripheral;
use stm32_fmc::{AddressPinSet, PinsSdram, Sdram, SdramChip, SdramPinSet, SdramTargetBank};

pub struct FMC {
    pub fmc: FMC_PER,
    hclk: Hertz,
}

// FmcExt trait provides sdram() and sdram_unchecked() methods
impl FmcExt for FMC_PER {
    fn sdram<BANK, ADDR, PINS, CHIP>(self, pins: PINS, chip: CHIP, clocks: &Clocks) -> Sdram<FMC, CHIP>;
}
```

**Feature Flags:**
- `fmc` — Enables FMC support (STM32F469, F429, etc.)
- `fsmc` — Enables FSMC support (STM32F407, etc.)

### 3.2 Integration in `stm32f469i-disc` BSP

The BSP crate provides a convenient SDRAM abstraction:

```rust
// stm32f469i-disc/src/sdram.rs
use stm32_fmc::devices::is42s32400f_6;

pub struct Sdram {
    pub mem: *mut u32,      // Pointer to SDRAM base (0xC000_0000)
    pub words: usize,       // 4,194,304 words = 16 MB
}

impl Sdram {
    pub fn new(fmc: FMC, pins: PINS, clocks: &Clocks, delay: &mut SysDelay) -> Self {
        Self {
            mem: fmc.sdram(pins, is42s32400f_6::Is42s32400f6 {}, clocks).init(delay),
            words: 16 * 1024 * 1024 / mem::size_of::<u32>(),
        }
    }
}
```

**Device Definition:**
- The `stm32-fmc` crate includes `is42s32400f_6` — **the exact SDRAM chip on the F469-DISCO**

---

## 4. How FMC Relates to the Display Pipeline

### 4.1 Complete Display Data Path

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         STM32F469 Display Pipeline                      │
└─────────────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌────────────┐
│   SDRAM      │───▶│     LTDC     │───▶│     DSI      │───▶│   LCD      │
│  (16 MB)     │    │  Controller  │    │    Host      │    │  Panel     │
│  via FMC     │    │              │    │              │    │NT35510/    │
│0xC0000000    │    │  Reads FB    │    │ MIPI D-PHY   │    │OTM8009A    │
└──────────────┘    └──────────────┘    └──────────────┘    └────────────┘
       ▲                   │
       │                   │
       │              ┌────┴────┐
       │              │  DMA2D  │
       └──────────────│ Accel.  │──────────────────────────┐
                      └─────────┘                          │
                      (Fast fills,                          │
                       color conversion,                   │
                       blending)                           │
                                                           │
                      ┌────────────────┐                   │
                      │      CPU        │◀──────────────────┘
                      │  (application)  │    (can also write directly)
                      └────────────────┘
```

### 4.2 Memory Bandwidth Analysis

**Display Refresh Bandwidth:**
- 480 × 800 × 60 Hz × 3 bytes (RGB888) = **~69 MB/s** for single layer
- 480 × 800 × 60 Hz × 2 bytes (RGB565) = **~46 MB/s** for single layer

**FMC Theoretical Bandwidth:**
- 32-bit bus @ 90 MHz = **360 MB/s** (theoretical max)
- With SDRAM overhead (refresh, precharge, row activation) ≈ **250-300 MB/s** (effective)

**Headroom:**
- Single layer RGB565: ~15-20% of FMC bandwidth
- Single layer RGB888: ~23-28% of FMC bandwidth
- Double-buffered RGB565: ~30-40% of FMC bandwidth

**Conclusion:** FMC has sufficient bandwidth for the display with room for CPU/DMA2D operations.

---

## 5. Current Display Examples Using SDRAM

### 5.1 Evidence from Code

The BSP examples **already use SDRAM-backed framebuffers**:

```rust
// display_hello_eg.rs (lines 60-71)
defmt::info!("Initializing SDRAM...");
let sdram = Sdram::new(
    dp.FMC,
    sdram_pins! {gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi},
    &rcc.clocks,
    &mut delay,
);

let buffer: &'static mut [u16] =
    unsafe { &mut *core::ptr::slice_from_raw_parts_mut(sdram.mem as *mut u16, lcd::FB_SIZE) };
let mut fb = LtdcFramebuffer::new(buffer, lcd::WIDTH, lcd::HEIGHT);
```

### 5.2 Examples Using SDRAM Framebuffers

| Example | File | SDRAM Usage |
|---------|------|-------------|
| `display_hello_eg` | `stm32f469i-disc/examples/display_hello_eg.rs` | ✅ SDRAM framebuffer |
| `display_dsi_lcd` | `stm32f469i-disc/examples/display_dsi_lcd.rs` | ✅ SDRAM framebuffer |
| `display_touch` | `stm32f469i-disc/examples/display_touch.rs` | ✅ SDRAM framebuffer |
| `fmc_sdram_test` | `stm32f469i-disc/examples/fmc_sdram_test.rs` | ✅ SDRAM test only |

---

## 6. Relationship to Current Display Work

### 6.1 What the Display Work Needs

Based on the research document `research-f469-display-pipeline.md` and status report:

| Need | Provided By | Status |
|------|-------------|--------|
| **SDRAM controller** | `stm32-fmc` | ✅ Already integrated |
| **SDRAM chip driver** | `stm32-fmc::devices::is42s32400f_6` | ✅ Already integrated |
| **Framebuffer storage** | SDRAM via FMC | ✅ Already used in examples |
| **LTDC controller** | `stm32f4xx-hal::ltdc` | ✅ Implemented |
| **DSI host** | `stm32f4xx-hal::dsi` | ✅ Implemented |
| **Panel drivers** | `nt35510`, `otm8009a` crates | ✅ Integrated in BSP |

### 6.2 What's Already Working

1. **SDRAM initialization** via `stm32-fmc`
2. **Framebuffer allocation** in SDRAM at `0xC000_0000`
3. **LTDC reading from SDRAM** for display refresh
4. **embedded-graphics** integration with `LtdcFramebuffer`

### 6.3 Known Issues from Display Research

From `research-f469-display-pipeline.md`:

| Issue | Root Cause | FMC Relevance |
|-------|------------|---------------|
| RGB565 1/3 horizontal shift | DSI/LTDC color coding mismatch | **None** — FMC works correctly |
| ARGB8888 required for DSI | DSI wrapper reads 24-bit, LTDC outputs 32-bit | **None** — FMC works correctly |
| NT35510 timing mismatch | Using OTM8009A timings for NT35510 | **None** — FMC works correctly |

**Key Insight:** The current display issues are in the **DSI/LTDC configuration**, not FMC. The FMC/SDRAM layer is functioning correctly.

---

## 7. Recommendations

### 7.1 Immediate Actions

| Action | Priority | Effort | Impact |
|--------|----------|--------|--------|
| **Keep `stm32-fmc` as-is** | N/A | None | Current integration is correct |
| **Fix DSI color coding** | HIGH | 2-4 hours | Solves RGB565 shift issue |
| **Add NT35510-specific timings** | MEDIUM | 1-2 hours | Improves panel compatibility |
| **Document SDRAM framebuffer usage** | LOW | 1 hour | Helps future developers |

### 7.2 Do NOT Do

- ❌ **Do not replace `stm32-fmc`** — It's the correct, maintained solution
- ❌ **Do not write custom FMC code** — `stm32-fmc` handles all edge cases
- ❌ **Do not investigate FMC for display issues** — FMC is not the problem

### 7.3 Future Considerations

| Consideration | Relevance | When to Act |
|---------------|-----------|-------------|
| **DMA2D acceleration for SDRAM** | HIGH | When implementing double-buffering |
| **SDRAM power management** | LOW | If battery power is needed |
| **Memory-mapped file storage** | LOW | If NAND Flash support needed |
| **FMC LCD interface (8080)** | LOW | Only if using non-DSI displays |

---

## 8. Technical Debt Assessment

### 8.1 FMC-Related Debt

| Item | Location | Severity | Notes |
|------|----------|----------|-------|
| `unsafe` in DSI PLL config | `src/dsi.rs`, BSP `lcd.rs` | LOW | Not FMC-related |
| Missing `DrawTarget` for ARGB8888 | `src/ltdc.rs` | MEDIUM | Blocks ARGB8888 usage |
| Hardcoded timings | BSP `lcd.rs` | LOW | Uses same timings for both panels |

### 8.2 Not FMC-Related

The FMC/SDRAM implementation is clean and follows best practices. No technical debt found in this area.

---

## 9. Summary

### 9.1 Key Conclusions

1. **`stm32-fmc` is already integrated** and correctly configured for the STM32F469I-DISCO
2. **SDRAM-backed framebuffers are already used** in display examples
3. **FMC is NOT the source of any current display issues** — the problems are in DSI/LTDC configuration
4. **The integration is production-ready** — `stm32-fmc` v0.4.0 is stable and maintained

### 9.2 Actionable Steps

1. **Focus display debugging on DSI/LTDC**, not FMC
2. **Implement ARGB8888 DrawTarget** in `src/ltdc.rs` to enable proper DSI color path
3. **Add NT35510-specific timing constants** for better panel compatibility
4. **Continue using `stm32-fmc`** — no changes needed

### 9.3 Verdict

> **The `stm32-fmc` crate is critical infrastructure that is already properly integrated. It should be kept as a dependency and requires no modifications. All display-related efforts should focus on the DSI/LTDC/panel layers, not the FMC/SDRAM layer.**

---

## Appendix A: File References

### FMC-Related Files in Codebase

| File | Purpose |
|------|---------|
| `stm32-fmc/Cargo.toml` | Crate definition (v0.4.0) |
| `stm32-fmc/src/lib.rs` | Core traits (`FmcPeripheral`, etc.) |
| `stm32-fmc/src/sdram.rs` | SDRAM controller implementation |
| `stm32-fmc/src/devices/is42s32400f.rs` | F469-DISCO SDRAM chip definition |
| `src/fmc.rs` | HAL wrapper integrating `stm32-fmc` |
| `stm32f469i-disc/src/sdram.rs` | BSP SDRAM convenience wrapper |
| `examples/fmc-sdram.rs` | HAL-level SDRAM example |
| `stm32f469i-disc/examples/fmc_sdram_test.rs` | BSP SDRAM test |

### Display-Related Files Using SDRAM

| File | Purpose |
|------|---------|
| `stm32f469i-disc/examples/display_hello_eg.rs` | embedded-graphics with SDRAM FB |
| `stm32f469i-disc/examples/display_dsi_lcd.rs` | DSI LCD with SDRAM FB |
| `stm32f469i-disc/examples/display_touch.rs` | Touch with SDRAM FB |

---

## Appendix B: References

### Official Documentation

- [AN4861](https://www.st.com/resource/en/application_note/an4861-introduction-to-lcdtft-display-controller-ltdc-on-stm32-mcus-stmicroelectronics.pdf) — LTDC Introduction
- [AN4757](https://www.st.com/resource/en/application_note/an4757-handling-mipi-dsi-on-stm32-microcontrollers-stmicroelectronics.pdf) — MIPI-DSI on STM32
- [UM1932](https://www.st.com/resource/en/user_manual/um1932-discovery-kit-with-stm32f469ni-mcu-stmicroelectronics.pdf) — F469-DISCO User Manual
- [DS11189](https://www.st.com/resource/en/datasheet/stm32f469be.pdf) — STM32F469 Datasheet

### Code Repositories

- [stm32-rs/stm32-fmc](https://github.com/stm32-rs/stm32-fmc) — FMC HAL crate
- [stm32-rs/stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) — STM32F4 HAL

### Internal Documents

- `research-f469-display-pipeline.md` — Display pipeline analysis
- `.sisyphus/reports/f469-display-status-report.md` — Display status report
- `f469-disco-revc-board-support.md` — Board revision analysis

---

*Report generated by Sisyphus AI Agent*  
*Analysis Mode: Complete*  
*Code Changes Made: None (analysis only)*
