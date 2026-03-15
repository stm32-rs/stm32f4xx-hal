# F469-Disco RevC Board Support Analysis

> **Source Commit:** `f5a6772d79fcb622ecd95873eb9a61af6a542d4c`  
> **Repository:** diybitcoinhardware/f469-disco  
> **Description:** Adds support for new board revision (RevC) with different LCD display controller

---

## Table of Contents

1. [Overview](#overview)
2. [Hardware Changes](#hardware-changes)
3. [Board Detection Mechanism](#board-detection-mechanism)
4. [Display Parameters Comparison](#display-parameters-comparison)
5. [Code Architecture](#code-architecture)
6. [Files Modified](#files-modified)
7. [Implementation Details](#implementation-details)

---

## Overview

This commit adds support for a **new board revision (RevC)** of the STM32F469I-Discovery board that uses a different LCD display panel. The implementation features:

- **Runtime detection** of connected display type
- **Dual display driver support** (OTM8009A and NT35510)
- **Automatic fallback** to correct driver based on detection
- **Shared initialization code** with conditional driver selection

---

## Hardware Changes

### Original Board (Pre-RevC)
- **Display Panel:** KoD KM-040TMP-02-0621 (WVGA)
- **LCD Controller:** OTM8009A
- **Resolution:** 480×800 (portrait) / 800×480 (landscape)
- **Interface:** DSI (Display Serial Interface) with 2 data lanes

### RevC Board
- **Display Panel:** Frida Techshine 3K138 (WVGA)
- **LCD Controller:** NT35510
- **Resolution:** 480×800 (portrait) / 800×480 (landscape)
- **Interface:** DSI (Display Serial Interface) with 2 data lanes

**Key Difference:** While both displays have the same resolution, they use different LCD controller ICs with different timing requirements and initialization sequences.

---

## Board Detection Mechanism

### Detection Strategy

The system uses **DSI (Display Serial Interface) read commands** to query the display's identification register. This is done before the LCD driver initialization to determine which controller is present.

### Detection Flow

```
┌─────────────────────────────────────────────────────────────────┐
│              BSP_LCD_InitEx(orientation, is_revc)               │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
                    ┌───────────────────────┐
                    │  Initialize DSI/LTDC  │
                    │  with default timings │
                    └───────────────────────┘
                                │
                                ▼
                    ┌───────────────────────────┐
                    │ if (is_revc == 0) {       │
                    │   BSP_LCD_ReadDisplayModel│
                    │ }                         │
                    └───────────────────────────┘
                                │
                    ┌───────────┴───────────┐
                    ▼                       ▼
          ┌─────────────────┐      ┌─────────────────┐
          │ arr[0] != 0     │      │ arr[0] == 0     │
          │ (OTM8009A)      │      │ (NT35510/RevC)  │
          └─────────────────┘      └─────────────────┘
                    │                       │
                    ▼                       ▼
          ┌─────────────────┐      ┌─────────────────────────┐
          │ OTM8009A_Init() │      │ Re-call BSP_LCD_InitEx  │
          │ (Original)      │      │ with is_revc = 1        │
          └─────────────────┘      │ Then NT35510_Init()     │
                                   └─────────────────────────┘
```

### Detection Code

```c
// In BSP_LCD_InitEx() - stm32469i_discovery_lcd.c
if(is_revc == 0){
    uint8_t arr[5] = { 0 };
    BSP_LCD_ReadDisplayModel(arr, sizeof(arr));
    if(arr[0] == 0){
      return BSP_LCD_InitEx(orientation, 1);  // Re-initialize for RevC
    }
}
```

### Display Model Read Function

```c
uint32_t BSP_LCD_ReadDisplayModel(uint8_t * arr, uint16_t len){
  // Enable Bus Turn-Around for read operation
  HAL_StatusTypeDef status = HAL_DSI_ConfigFlowControl(&(hdsi_eval), DSI_FLOW_CONTROL_BTA);
  if (status != HAL_OK){
    return LCD_ERROR;
  }
 
  // Read from DSI register 0xA1 (RDDDBS - Read DDB Start)
  // This returns display manufacturer/model identification data
  status = HAL_DSI_Read(&(hdsi_eval), LCD_OTM8009A_ID, arr, len, 
                        DSI_GEN_SHORT_PKT_READ_P1, 0, (uint8_t[]){0xA1, 0});
  if (status != HAL_OK){
    return LCD_ERROR;
  }
  return LCD_OK;
}
```

### How Detection Works

1. **DSI Read Command:** Sends a generic short packet read to register `0xA1` (RDDDBS - Read DDB Start)
2. **Response Analysis:** 
   - **OTM8009A (Original):** Returns non-zero first byte
   - **NT35510 (RevC):** Returns `0x00` as first byte
3. **Recursive Initialization:** If RevC is detected, the function re-calls itself with `is_revc=1`
4. **Driver Selection:** Based on `is_revc` flag, appropriate timing and driver are selected

---

## Display Parameters Comparison

### OTM8009A (Original Board)

#### Portrait Mode (480×800)

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Width** | 480 pixels | Horizontal active pixels |
| **Height** | 800 pixels | Vertical active lines |
| **HSYNC** | 2 | Horizontal sync pulse width (in lcdClk) |
| **HBP** | 34 | Horizontal back porch (in lcdClk) |
| **HFP** | 34 | Horizontal front porch (in lcdClk) |
| **VSYNC** | 1 | Vertical sync pulse width (in lines) |
| **VBP** | 15 | Vertical back porch (in lines) |
| **VFP** | 16 | Vertical front porch (in lines) |
| **Frequency Divider** | 2 | LCD clock divider |

#### Landscape Mode (800×480)

| Parameter | Value | Derived From |
|-----------|-------|--------------|
| **Width** | 800 pixels | - |
| **Height** | 480 pixels | - |
| **HSYNC** | 1 | = Portrait VSYNC |
| **HBP** | 15 | = Portrait VBP |
| **HFP** | 16 | = Portrait VFP |
| **VSYNC** | 2 | = Portrait HSYNC |
| **VBP** | 34 | = Portrait HBP |
| **VFP** | 34 | = Portrait HFP |

```c
// From otm8009a.h
#define  OTM8009A_480X800_WIDTH             ((uint16_t)480)
#define  OTM8009A_480X800_HEIGHT            ((uint16_t)800)
#define  OTM8009A_480X800_HSYNC             ((uint16_t)2)
#define  OTM8009A_480X800_HBP               ((uint16_t)34)
#define  OTM8009A_480X800_HFP               ((uint16_t)34)
#define  OTM8009A_480X800_VSYNC             ((uint16_t)1)
#define  OTM8009A_480X800_VBP               ((uint16_t)15)
#define  OTM8009A_480X800_VFP               ((uint16_t)16)
#define  OTM8009A_480X800_FREQUENCY_DIVIDER  2
```

---

### NT35510 (RevC Board)

#### Portrait Mode (480×800)

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Width** | 480 pixels | Horizontal active pixels |
| **Height** | 800 pixels | Vertical active lines |
| **HSYNC** | 2 | Horizontal sync pulse width (in lcdClk) |
| **HBP** | 34 | Horizontal back porch (in lcdClk) |
| **HFP** | 34 | Horizontal front porch (in lcdClk) |
| **VSYNC** | 120 | Vertical sync pulse width (in lines) |
| **VBP** | 150 | Vertical back porch (in lines) |
| **VFP** | 150 | Vertical front porch (in lines) |
| **Frequency Divider** | 2 | LCD clock divider |

#### Landscape Mode (800×480)

| Parameter | Value | Derived From |
|-----------|-------|--------------|
| **Width** | 800 pixels | - |
| **Height** | 480 pixels | - |
| **HSYNC** | 120 | = Portrait VSYNC |
| **HBP** | 150 | = Portrait VBP |
| **HFP** | 150 | = Portrait VFP |
| **VSYNC** | 2 | = Portrait HSYNC |
| **VBP** | 34 | = Portrait HBP |
| **VFP** | 34 | = Portrait HFP |

```c
// From nt35510.h
#define  NT35510_480X800_WIDTH             ((uint16_t)480)
#define  NT35510_480X800_HEIGHT            ((uint16_t)800)
#define  NT35510_480X800_HSYNC             ((uint16_t)2)
#define  NT35510_480X800_HBP               ((uint16_t)34)
#define  NT35510_480X800_HFP               ((uint16_t)34)
#define  NT35510_480X800_VSYNC             ((uint16_t)120)
#define  NT35510_480X800_VBP               ((uint16_t)150)
#define  NT35510_480X800_VFP               ((uint16_t)150)
#define  NT35510_480X800_FREQUENCY_DIVIDER  2
```

---

### Timing Differences Summary

| Parameter | OTM8009A | NT35510 | Difference |
|-----------|----------|---------|------------|
| **VSYNC** | 1 | 120 | **120× larger** |
| **VBP** | 15 | 150 | **10× larger** |
| **VFP** | 16 | 150 | **~9× larger** |

**Key Insight:** The NT35510 requires significantly larger vertical timing margins (VSYNC, VBP, VFP). This is likely due to:
- Different internal display processing architecture
- Different panel timing requirements
- Additional time needed for internal operations

---

## Code Architecture

### Initialization Sequence

```
┌────────────────────────────────────────────────────────────┐
│                    BSP_LCD_Init()                          │
│         (calls BSP_LCD_InitEx with defaults)               │
└────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────┐
│              BSP_LCD_InitEx(orientation, is_revc)          │
└────────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│ DSI PLL Init  │ │ DSI Init      │ │ LTDC Init     │
└───────────────┘ └───────────────┘ └───────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────┐
│              Select Timing Parameters                       │
│  if (is_revc) { NT35510 timings } else { OTM8009A timings }│
└────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────┐
│              Display Detection (if is_revc == 0)           │
│  BSP_LCD_ReadDisplayModel() → returns arr[0]               │
└────────────────────────────────────────────────────────────┘
                          │
            ┌─────────────┴─────────────┐
            ▼                           ▼
    ┌───────────────┐          ┌───────────────┐
    │ arr[0] != 0   │          │ arr[0] == 0   │
    │ OTM8009A      │          │ NT35510/RevC  │
    └───────────────┘          └───────────────┘
            │                           │
            ▼                           ▼
    ┌───────────────┐          ┌───────────────┐
    │OTM8009A_Init()│          │ Re-init with  │
    │               │          │ is_revc=1     │
    └───────────────┘          └───────────────┘
                                        │
                                        ▼
                                ┌───────────────┐
                                │NT35510_Init() │
                                └───────────────┘
```

### Timing Selection Code

```c
// In BSP_LCD_InitEx() - stm32469i_discovery_lcd.c
if(is_revc){
    VSA  = NT35510_480X800_VSYNC;
    VBP  = NT35510_480X800_VBP;
    VFP  = NT35510_480X800_VFP;
    HSA  = NT35510_480X800_HSYNC;
    HBP  = NT35510_480X800_HBP;
    HFP  = NT35510_480X800_HFP;
}else{
    VSA  = OTM8009A_480X800_VSYNC;
    VBP  = OTM8009A_480X800_VBP;
    VFP  = OTM8009A_480X800_VFP;
    HSA  = OTM8009A_480X800_HSYNC;
    HBP  = OTM8009A_480X800_HBP;
    HFP  = OTM8009A_480X800_HFP;
}
```

### Driver Selection Code

```c
// In BSP_LCD_InitEx() - stm32469i_discovery_lcd.c
if(is_revc){
    /* Initialize the NT35510 LCD Display IC Driver (TechShine LCD IC Driver) */
    NT35510_Init(NT35510_FORMAT_RGB888, orientation);
}else{
    /* Initialize the OTM8009A LCD Display IC Driver (KoD LCD IC Driver) */
    OTM8009A_Init(OTM8009A_FORMAT_RGB888, orientation);
}
```

---

## Files Modified

| File | Purpose |
|------|---------|
| `stm32469i_discovery_lcd.c` | LCD initialization, timing selection, display detection |
| `stm32469i_discovery_lcd.h` | Function prototypes, includes for both drivers |
| `nt35510.c` | **NEW** - NT35510 LCD controller driver implementation |
| `nt35510.h` | **NEW** - NT35510 constants, timing parameters, commands |
| `ft6x06.c` | Touch controller driver (license update, minor changes) |
| `ft6x06.h` | Touch controller header (license update, minor changes) |
| `display.c` | MicroPython display module (uses BSP_LCD_InitEx) |
| `micropython.mk` | Build configuration (added NT35510 source files) |

---

## Implementation Details

### DSI Configuration

Both displays use DSI (Display Serial Interface) with the following common configuration:

```c
hdsivideo_handle.VirtualChannelID = LCD_OTM8009A_ID;
hdsivideo_handle.ColorCoding = LCD_DSI_PIXEL_DATA_FMT_RBG888;
hdsivideo_handle.VSPolarity = DSI_VSYNC_ACTIVE_HIGH;
hdsivideo_handle.HSPolarity = DSI_HSYNC_ACTIVE_HIGH;
hdsivideo_handle.DEPolarity = DSI_DATA_ENABLE_ACTIVE_HIGH;
hdsivideo_handle.Mode = DSI_VID_MODE_BURST;
hdsivideo_handle.NullPacketSize = 0xFFF;
hdsivideo_handle.NumberOfChunks = 0;
hdsivideo_handle.PacketSize = HACT;  // Depends on orientation
```

### Clock Configuration

```c
// DSI PLL configuration (common for both displays)
dsiPllInit.PLLNDIV  = 125;
dsiPllInit.PLLIDF   = DSI_PLL_IN_DIV2;
dsiPllInit.PLLODF   = DSI_PLL_OUT_DIV1;
laneByteClk_kHz = 62500; /* 500 MHz / 8 = 62.5 MHz = 62500 kHz */

// LTDC clock configuration
// PLLSAI_VCO = 384 MHz
// PLLLCDCLK = 384 MHz / 7 = 54.857 MHz  
// LTDC clock = 54.857 MHz / 2 = 27.429 MHz
PeriphClkInitStruct.PLLSAI.PLLSAIN = 384;
PeriphClkInitStruct.PLLSAI.PLLSAIR = 7;
PeriphClkInitStruct.PLLSAIDivR = RCC_PLLSAIDIVR_2;
```

### Frame Buffer

```c
#define LCD_FB_START_ADDRESS       ((uint32_t)0xC0000000)  // SDRAM base address
```

---

## Usage in MicroPython

The display module exposes initialization through `BSP_LCD_InitEx`:

```python
# Initialize display
import udisplay
udisplay.display_init()

# Set rotation (triggers re-initialization)
# 0 = Portrait, 1 = Landscape
udisplay.set_rotation(0)

# Update display (call in main loop)
udisplay.update(dt_ms)

# Turn display on/off
udisplay.on()
udisplay.off()
```

---

## Key Takeaways for Similar Implementations

1. **Runtime Detection:** Use DSI read commands to query display identification registers before driver initialization

2. **Recursive Init Pattern:** Use a flag parameter (`is_revc`) to allow re-initialization with different parameters after detection

3. **Timing Abstraction:** Define timing parameters as constants in header files for easy modification and clarity

4. **Common Infrastructure:** Share DSI/LTDC initialization code between display variants; only differ in:
   - Timing parameters (VSYNC, VBP, VFP, HSYNC, HBP, HFP)
   - LCD controller-specific initialization sequences

5. **Graceful Fallback:** Default to original board configuration, detect newer revision, and re-initialize if needed

---

## References

- **STM32F469I-Discovery Board:** STM32F469NI microcontroller with DSI and LTDC peripherals
- **DSI Specification:** MIPI Display Serial Interface
- **OTM8009A Datasheet:** KoD KM-040TMP-02-0621 display driver IC
- **NT35510 Datasheet:** Frida Techshine 3K138 display driver IC

---

*Document generated from commit analysis: f5a6772d79fcb622ecd95873eb9a61af6a542d4c*
