# F469-DISCO Display Pipeline Research

## Problem Statement

The STM32F469I-DISCO (Rev B08, NT35510 panel) shows a **1/3 horizontal display shift** when using RGB565 framebuffer. The right 2/3 of each scanline wraps into the next line, creating a diagonal tear pattern.

**Root cause**: The DSI wrapper reads 3 bytes/pixel (RGB888) from a framebuffer that stores 2 bytes/pixel (RGB565). Each 480-pixel scanline is 960 bytes in memory but the DSI reads 1440 bytes, overflowing 480 bytes into the next line = exactly 1/3 of the width.

## Key Question

Can we use an **RGB565 framebuffer** (2 bytes/pixel, `u16`) with the NT35510 panel over DSI? This matters because:
- `embedded_graphics` `DrawTarget` is only implemented for `LtdcFramebuffer<u16>` (Rgb565) in our HAL
- We cannot modify `src/ltdc.rs` (constraint)
- ARGB8888 (4 bytes/pixel) doubles memory usage: 1.536MB vs 768KB

## What We Tested

| Config | DSI Host (LCOLCR) | DSI Wrapper (WCFGR) | LTDC Layer | Panel COLMOD | Result |
|--------|-------------------|---------------------|------------|--------------|--------|
| Original | TwentyFourBits (5) | TwentyFourBits (5) | RGB565 | RGB888 (0x77) | **1/3 shift** — DSI reads 3 bpp from 2 bpp buffer |
| Option C | SixteenBitsConfig1 (0) | SixteenBitsConfig1 (0) | RGB565 | RGB565 (0x55) | **DSI broken** — all reads/writes fail, no communication |

## Reference Implementations Studied

### 1. STM32CubeF4 BSP (canonical, `/tmp/f469-disco`)
**Source**: `stm32469i_discovery_lcd.c`, `BSP_LCD_InitEx()`

| Stage | Configuration | Value |
|-------|--------------|-------|
| Framebuffer | ARGB8888 | `uint32_t*`, 4 bytes/pixel |
| LTDC layer | `LTDC_PIXEL_FORMAT_ARGB8888` | Register value 0 |
| DSI color coding | `LCD_DSI_PIXEL_DATA_FMT_RBG888` = `DSI_RGB888` | Both LCOLCR and WCFGR = 5 |
| NT35510 COLMOD | `NT35510_FORMAT_RGB888` | 0x77 |
| OTM8009A COLMOD | `OTM8009A_FORMAT_RGB888` | 0x77 |

**Key observation**: The BSP has NO RGB565 display path. RGB565 only appears in DMA2D color conversion helpers. The LTDC→DSI→Panel pipeline is always ARGB8888→RGB888→RGB888.

### 2. ChuckM's stm32f469i demos (`github.com/ChuckM/stm32f469i`)
**Source**: `demos/ltdc/ltdc.c`

| Stage | Configuration | Value |
|-------|--------------|-------|
| Framebuffer | ARGB8888 | `uint32_t*`, 4 bytes/pixel |
| LTDC layer | `LTDC_L1PFCR = 0` | ARGB8888 |
| DSI wrapper | `DSI_WCFGR COLMUX = 5` | RGB888 |
| DSI host | `DSI_LCOLCR = 5` | RGB888 |
| Panel COLMOD | 0x77 | RGB888 |

**Key observation**: Has a `#ifdef LCD_COLOR_FORMAT_RGB565` conditional that writes COLMOD 0x55 to the panel, BUT does NOT change LTDC or DSI registers to match. This means ChuckM's RGB565 path would have the same 1/3 shift bug — it's dead/broken code.

### 3. eez-open LVGL demo (`github.com/eez-open/stm32f469i-disco-lvgl-demo`)
**Source**: `screen_driver.c`, BSP LCD driver

| Stage | Configuration | Value |
|-------|--------------|-------|
| LVGL color depth | `LV_COLOR_DEPTH 32` | ARGB8888 |
| LTDC layer | `LTDC_PIXEL_FORMAT_ARGB8888` | 4 bytes/pixel |
| DSI color coding | `DSI_RGB888` | RGB888 |
| NT35510 COLMOD | `NT35510_FORMAT_RGB888` | 0x77 |

### 4. RT-Thread LVGL demo
**Note**: Targets OTM8009A (RevA/B), not NT35510. Uses `LV_COLOR_DEPTH 16` with DMA2D RGB565 transfers, but the underlying BSP still initializes DSI+LTDC for the OTM8009A panel.

## Register Deep Dive

### How ColorCoding Maps to Registers (from our Rust HAL `src/dsi.rs`)

```rust
// DSI Host color coding → DSI_LCOLCR register, COLC bits
dsi.lcolcr().modify(|_, w| w.colc().bits(color_coding_host as u8));

// DSI Wrapper color coding → DSI_WCFGR register, COLMUX bits
dsi.wcfgr().modify(|_, w| w.colmux().bits(color_coding_wrapper as u8));
```

The STM32 HAL (`stm32f4xx_hal_dsi.c`) always sets BOTH registers to the SAME value:
```c
// HAL_DSI_ConfigVideoMode()
hdsi->Instance->LCOLCR &= ~DSI_LCOLCR_COLC;
hdsi->Instance->LCOLCR |= VidCfg->ColorCoding;              // LCOLCR.COLC
hdsi->Instance->WCFGR &= ~DSI_WCFGR_COLMUX;
hdsi->Instance->WCFGR |= ((VidCfg->ColorCoding)<<1U);       // WCFGR.COLMUX
```

### ColorCoding enum values
```
SixteenBitsConfig1  = 0b000  (0) — RGB565, Config 1
SixteenBitsConfig2  = 0b001  (1) — RGB565, Config 2
SixteenBitsConfig3  = 0b010  (2) — RGB565, Config 3
EighteenBitsConfig1 = 0b011  (3) — RGB666, loosely packed
EighteenBitsConfig2 = 0b100  (4) — RGB666, packed
TwentyFourBits      = 0b101  (5) — RGB888
```

### Why SixteenBitsConfig1 Broke DSI Communication

When we set both LCOLCR and WCFGR to SixteenBitsConfig1, DSI read/write commands (used for panel probe and init) also failed. Hypotheses:

1. **The DSI command mode also uses color coding bits** — changing them may affect LP command transmission timing/framing
2. **The NT35510 panel's DSI receiver may not support 16-bit mode** — the panel might reject packets or fail to sync
3. **The DSI wrapper's COLMUX affects the LTDC→DSI byte lane mapping for ALL traffic** — not just pixel data but also command packets flowing through the wrapper

The ST BSP never uses SixteenBitsConfig1 with DSI on F469. There's no documented evidence of RGB565 working over DSI on this hardware.

## The LTDC→DSI Pipeline Conversion

The LTDC controller itself has a pixel format register (LxPFCR) per layer. When LTDC is bridged to DSI (not driving an external parallel display), the data path is:

```
SDRAM → LTDC (reads pixels per LxPFCR format) → DSI Wrapper (COLMUX) → DSI Host (LCOLCR) → PHY → Panel
```

The LTDC reads pixels from memory according to LxPFCR:
- `ARGB8888` (value 0): reads 4 bytes, extracts R[7:0], G[7:0], B[7:0], A[7:0]
- `RGB565` (value 2): reads 2 bytes, extracts R[4:0], G[5:0], B[4:0]

The DSI Wrapper (COLMUX) then takes the LTDC output and packs it into DSI packets:
- `COLMUX=5` (RGB888): expects 24 bits per pixel from LTDC
- `COLMUX=0` (RGB565): expects 16 bits per pixel from LTDC

**Critical**: When LTDC outputs ARGB8888 (32 bits) but COLMUX is set to RGB888 (24 bits), the alpha channel is **automatically stripped** by the LTDC→DSI bridge. This is the documented and proven working path.

When LTDC outputs RGB565 (16 bits) but COLMUX is set to RGB888 (24 bits), there's a **byte count mismatch** and the DSI reads extra bytes from the next scanline = 1/3 shift.

## Conclusions

### Finding 1: ARGB8888 is the ONLY proven working framebuffer format for DSI on STM32F469

Every single working reference implementation uses:
- **LTDC: ARGB8888** (4 bytes/pixel)
- **DSI: RGB888** (COLMUX=5, LCOLCR=5)
- **Panel: RGB888** (COLMOD=0x77)

No implementation has RGB565 working over DSI. ChuckM's conditional RGB565 code is broken (doesn't change DSI registers).

### Finding 2: The `embedded_graphics` compatibility problem

Our HAL (`src/ltdc.rs`) only implements `DrawTarget` for `LtdcFramebuffer<u16>` (Rgb565). To use ARGB8888 framebuffers, we need `DrawTarget` for `LtdcFramebuffer<u32>`.

**Options to solve this:**

#### Option A: Add `DrawTarget` impl for `LtdcFramebuffer<u32>` in `src/ltdc.rs`
- **Pros**: Clean, idiomatic, enables embedded_graphics with ARGB8888
- **Cons**: Requires modifying `src/` (currently constrained)
- **Effort**: ~20 lines of code
- **Color type**: Could use `embedded_graphics_core::pixelcolor::Rgb888` mapped to ARGB8888 u32

#### Option B: Create a wrapper in the example's `board.rs`
- **Pros**: No src/ changes needed
- **Cons**: Wrapper is example-specific, not reusable
- **Effort**: ~30 lines

#### Option C: Lift the src/ constraint and add DrawTarget properly
- **Pros**: Best long-term solution, benefits all users of the HAL
- **Cons**: Changes HAL source
- **Effort**: Small, well-contained change

### Finding 3: NT35510 vertical timing may also need updating

Reference BSP uses different vertical timing for NT35510:

| Parameter | Our code | ST BSP Reference |
|-----------|----------|-----------------|
| VSYNC | 1 | 120 |
| VBP | 15 | 150 |
| VFP | 16 | 150 |

These are defined as `NT35510_480X800_VSYNC`, `NT35510_480X800_VBP`, `NT35510_480X800_VFP` in the BSP headers. Our timings match the OTM8009A values. While the display may work with shorter porches, using the NT35510-specific timings could improve stability.

## Recommended Fix

1. **Change LTDC pixel format to ARGB8888** (u32 framebuffer)
2. **Keep DSI at TwentyFourBits** (RGB888) for both host and wrapper
3. **Keep NT35510 panel init at RGB888** (`panel.init()`, not `panel.init_rgb565()`)
4. **Add `DrawTarget` impl for `LtdcFramebuffer<u32>`** in `src/ltdc.rs` — this is a small, well-contained change that makes ARGB8888 usable with embedded_graphics
5. **Consider updating NT35510 vertical timing** to match the BSP reference values

## References

- STM32CubeF4 BSP: `/tmp/f469-disco/usermods/udisplay_f469/BSP_DISCO_F469NI/`
- ChuckM demos: `github.com/ChuckM/stm32f469i/demos/ltdc/`
- eez-open LVGL: `github.com/eez-open/stm32f469i-disco-lvgl-demo`
- RT-Thread: `github.com/RT-Thread/rt-thread/bsp/stm32/stm32f469-st-disco/`
- RM0386 (STM32F469 Reference Manual): DSI registers in Section 21
- AN4860: DSI Host on STM32F4xx and STM32F7xx
