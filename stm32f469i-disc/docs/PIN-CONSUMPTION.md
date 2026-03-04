# GPIO Pin Consumption - STM32F469I-DISCO

## Overview

The STM32F469I-DISCO development board routes many GPIO pins to the on-board 16MB SDRAM memory. This consumes approximately 50 pins across six GPIO ports (C, D, E, F, G, H, I), leaving limited pins available for other peripherals.

Understanding which pins are consumed and which remain available is critical when designing applications that need to use additional peripherals like SDIO, touch input, or custom GPIO functions.

### Key Takeaway

After SDRAM initialization, the `split_sdram_pins()` function returns:
- SDRAM pin tuple for memory controller use
- `SdramRemainders` struct with pins for touch/SDIO
- PH7 for LCD reset

## Pin Consumption by Port

### Port A

| Pin | Function | Notes |
|-----|----------|-------|
| PA0 | User Button | Active high, use pull-down |
| PA11 | USB DM | USB OTG FS |
| PA12 | USB DP | USB OTG FS |

**Available:** PA1-PA10, PA13-PA15 (not routed to onboard peripherals)

### Port B

| Pin | Function | Notes |
|-----|----------|-------|
| PB8 | I2C1 SCL | Touch controller I2C clock |
| PB9 | I2C1 SDA | Touch controller I2C data |

**Available:** PB0-PB7, PB10-PB15 (not routed to onboard peripherals)

### Port C

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PC0 | SDNWE | - | SDRAM write enable |
| PC1 | - | Touch INT | FT6X06 interrupt (active low) |
| PC8 | - | SDIO D0 | SD card data 0 |
| PC9 | - | SDIO D1 | SD card data 1 |
| PC10 | - | SDIO D2 | SD card data 2 |
| PC11 | - | SDIO D3 | SD card data 3 |
| PC12 | - | SDIO CLK | SD card clock |

**Available:** PC2-PC7, PC13-PC15

### Port D

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PD0 | D2 | - | SDRAM data bit 2 |
| PD1 | D3 | - | SDRAM data bit 3 |
| PD2 | - | SDIO CMD | SD card command |
| PD4 | - | LED LD2 | Orange LED |
| PD5 | - | LED LD3 | Red LED |
| PD8 | D13 | - | SDRAM data bit 13 |
| PD9 | D14 | - | SDRAM data bit 14 |
| PD10 | D15 | - | SDRAM data bit 15 |
| PD14 | D0 | - | SDRAM data bit 0 |
| PD15 | D1 | - | SDRAM data bit 1 |

**Available:** PD3, PD6, PD7, PD11-PD13

### Port E

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PE0 | NBL0 | - | SDRAM byte lane 0 enable |
| PE1 | NBL1 | - | SDRAM byte lane 1 enable |
| PE7 | D4 | - | SDRAM data bit 4 |
| PE8 | D5 | - | SDRAM data bit 5 |
| PE9 | D6 | - | SDRAM data bit 6 |
| PE10 | D7 | - | SDRAM data bit 7 |
| PE11 | D8 | - | SDRAM data bit 8 |
| PE12 | D9 | - | SDRAM data bit 9 |
| PE13 | D10 | - | SDRAM data bit 10 |
| PE14 | D11 | - | SDRAM data bit 11 |
| PE15 | D12 | - | SDRAM data bit 12 |

**Available:** PE2-PE6

### Port F

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PF0 | A0 | - | SDRAM address bit 0 |
| PF1 | A1 | - | SDRAM address bit 1 |
| PF2 | A2 | - | SDRAM address bit 2 |
| PF3 | A3 | - | SDRAM address bit 3 |
| PF4 | A4 | - | SDRAM address bit 4 |
| PF5 | A5 | - | SDRAM address bit 5 |
| PF11 | SDNRAS | - | SDRAM row address strobe |
| PF12 | A6 | - | SDRAM address bit 6 |
| PF13 | A7 | - | SDRAM address bit 7 |
| PF14 | A8 | - | SDRAM address bit 8 |
| PF15 | A9 | - | SDRAM address bit 9 |

**Available:** PF6-PF10

### Port G

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PG0 | A10 | - | SDRAM address bit 10 |
| PG1 | A11 | - | SDRAM address bit 11 |
| PG4 | BA0 | - | SDRAM bank address 0 |
| PG5 | BA1 | - | SDRAM bank address 1 |
| PG6 | - | LED LD1 | Green LED |
| PG8 | SDCLK | - | SDRAM clock |
| PG15 | SDNCAS | - | SDRAM column address strobe |

**Available:** PG2, PG3, PG7, PG9-PG14

### Port H

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PH2 | SDCKE0 | - | SDRAM clock enable |
| PH3 | SDNE0 | - | SDRAM chip select |
| PH7 | - | LCD RST | LCD reset (output) |
| PH8 | D16 | - | SDRAM data bit 16 |
| PH9 | D17 | - | SDRAM data bit 17 |
| PH10 | D18 | - | SDRAM data bit 18 |
| PH11 | D19 | - | SDRAM data bit 19 |
| PH12 | D20 | - | SDRAM data bit 20 |
| PH13 | D21 | - | SDRAM data bit 21 |
| PH14 | D22 | - | SDRAM data bit 22 |
| PH15 | D23 | - | SDRAM data bit 23 |

**Available:** PH0, PH1, PH4-PH6

### Port I

| Pin | SDRAM | Other | Notes |
|-----|-------|-------|-------|
| PI0 | D24 | - | SDRAM data bit 24 |
| PI1 | D25 | - | SDRAM data bit 25 |
| PI2 | D26 | - | SDRAM data bit 26 |
| PI3 | D27 | - | SDRAM data bit 27 |
| PI4 | NBL2 | - | SDRAM byte lane 2 enable |
| PI5 | NBL3 | - | SDRAM byte lane 3 enable |
| PI6 | D28 | - | SDRAM data bit 28 |
| PI7 | D29 | - | SDRAM data bit 29 |
| PI9 | D30 | - | SDRAM data bit 30 |
| PI10 | D31 | - | SDRAM data bit 31 |

**Available:** PI8, PI11

### Port K

| Pin | Function | Notes |
|-----|----------|-------|
| PK3 | LED LD4 | Blue LED |

**Available:** PK0-PK2, PK4-PK7

## SDRAM Pin Categories

### Address Bus (12 pins)

```
A0-A5:  PF0, PF1, PF2, PF3, PF4, PF5
A6-A9:  PF12, PF13, PF14, PF15
A10-A11: PG0, PG1
```

### Bank Select (2 pins)

```
BA0: PG4
BA1: PG5
```

### Data Bus (32 pins)

```
D0-D1:   PD14, PD15
D2-D3:   PD0, PD1
D4-D12:  PE7, PE8, PE9, PE10, PE11, PE12, PE13, PE14, PE15
D13-D15: PD8, PD9, PD10
D16-D23: PH8, PH9, PH10, PH11, PH12, PH13, PH14, PH15
D24-D27: PI0, PI1, PI2, PI3
D28-D29: PI6, PI7
D30-D31: PI9, PI10
```

### Byte Lane Enables (4 pins)

```
NBL0: PE0
NBL1: PE1
NBL2: PI4
NBL3: PI5
```

### Control Signals (6 pins)

```
SDNWE:   PC0   (Write enable)
SDNRAS:  PF11  (Row address strobe)
SDNCAS:  PG15  (Column address strobe)
SDCLK:   PG8   (Clock)
SDCKE0:  PH2   (Clock enable)
SDNE0:   PH3   (Chip select)
```

## SdramRemainders Structure

The `split_sdram_pins()` function returns remaining pins in a struct:

```rust
pub struct SdramRemainders {
    /// Touch interrupt (FT6X06) - configure as pull-down input
    pub pc1: hal::gpio::PC1<hal::gpio::Input>,
    /// SDIO data lines - configure as alternate with pull-up
    pub pc8: hal::gpio::PC8<hal::gpio::Input>,
    pub pc9: hal::gpio::PC9<hal::gpio::Input>,
    pub pc10: hal::gpio::PC10<hal::gpio::Input>,
    pub pc11: hal::gpio::PC11<hal::gpio::Input>,
    pub pc12: hal::gpio::PC12<hal::gpio::Input>,
    /// SDIO command - configure as alternate with pull-up
    pub pd2: hal::gpio::PD2<hal::gpio::Input>,
}
```

Plus PH7 is returned separately for LCD reset.

## Visual Pin Map

```
Port A: [0:BTN] [1-10:FREE] [11:USB] [12:USB] [13-15:FREE]
                      ↑           ↑
                   JTAG/SWD   USB OTG FS

Port B: [0-7:FREE] [8:I2C_SCL] [9:I2C_SDA] [10-15:FREE]
                              ↑
                           Touch I2C

Port C: [0:SDRAM] [1:TOUCH] [2-7:FREE] [8-12:SDIO] [13-15:FREE]
          ↑          ↑                      ↑
        SDNWE    Touch INT              SD Card

Port D: [0-1:SDRAM] [2:SDIO] [3:FREE] [4-5:LED] [6-7:FREE] [8-15:SDRAM]
           ↑            ↑                ↑
        D2,D3        CMD            Orange,Red

Port E: [0-1:SDRAM] [2-6:FREE] [7-15:SDRAM]
          ↑                         ↑
       NBL0,1                   D4-D12

Port F: [0-5:SDRAM] [6-10:FREE] [11:SDRAM] [12-15:SDRAM]
          ↑              ↑          ↑           ↑
       A0-A5          FREE      SDNRAS      A6-A9

Port G: [0-1:SDRAM] [2-3:FREE] [4-5:SDRAM] [6:LED] [7:FREE] [8:SDRAM] [9-14:FREE] [15:SDRAM]
          ↑                           ↑        ↑              ↑                        ↑
       A10,A11                    BA0,BA1   Green          SDCLK                   SDNCAS

Port H: [0-1:FREE] [2-3:SDRAM] [4-6:FREE] [7:LCD] [8-15:SDRAM]
                       ↑                       ↑       ↑
                   SDCKE0,SDNE0              RST   D16-D23

Port I: [0-3:SDRAM] [4-5:SDRAM] [6-7:SDRAM] [8:FREE] [9-10:SDRAM] [11:FREE]
           ↑            ↑            ↑                  ↑
        D24-D27      NBL2,3       D28,D29            D30,D31

Port K: [0-2:FREE] [3:LED] [4-7:FREE]
                       ↑
                    Blue

Legend:
  SDRAM = Consumed by SDRAM interface
  FREE  = Available for application use
  LED   = User LED output
  BTN   = User button input
  SDIO  = SD card interface
  USB   = USB OTG FS
  I2C   = I2C1 for touch controller
```

## Pin Conflict Matrix

| Feature | Required Pins | Conflicts With |
|---------|---------------|----------------|
| SDRAM | PC0, PD0,1,8,9,10,14,15, PE0,1,7-15, PF0-5,11-15, PG0,1,4,5,8,15, PH2,3,8-15, PI0-7,9,10 | Any use of these pins |
| LCD | PH7 | - (returned separately) |
| Touch | PB8, PB9 (I2C), PC1 (INT) | - |
| SDIO | PC8-12, PD2 | - |
| USB FS | PA11, PA12 | - |
| User Button | PA0 | - |
| LEDs | PD4, PD5, PG6, PK3 | - |

### Compatible Peripherals

All on-board peripherals can coexist since their pins do not overlap:
- SDRAM + Touch + SDIO + USB + LEDs + Button = All work together

### What Cannot Be Used Together

Nothing on this board conflicts! The BSP design ensures all on-board peripherals use non-overlapping pins.

## Usage Example

```rust
use stm32f469i_disc::{sdram, lcd, touch, sdio};

// Split GPIO ports
let gpioc = dp.GPIOC.split(&mut rcc);
let gpiod = dp.GPIOD.split(&mut rcc);
let gpioe = dp.GPIOE.split(&mut rcc);
let gpiof = dp.GPIOF.split(&mut rcc);
let gpiog = dp.GPIOG.split(&mut rcc);
let gpioh = dp.GPIOH.split(&mut rcc);
let gpioi = dp.GPIOI.split(&mut rcc);

// Get SDRAM pins and remaining pins
let (sdram_pins, remainders, ph7) = sdram::split_sdram_pins(
    gpioc, gpiod, gpioe, gpiof, gpiog, gpioh, gpioi
);

// Initialize SDRAM
let mut sdram = sdram::Sdram::new(dp.FMC, sdram_pins, &rcc.clocks, &mut delay);

// LCD reset is on PH7
let mut lcd_reset = ph7.into_push_pull_output();

// SDIO uses PC8-12 and PD2 from remainders
let (sdio, touch_int) = sdio::init(dp.SDIO, remainders, &mut rcc);
// touch_int is PC1

// Touch I2C is on PB8/PB9 (separate from SDRAM remainders)
let gpiob = dp.GPIOB.split(&mut rcc);
let i2c = touch::init_i2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut rcc);
let mut touch = touch::init_ft6x06(&i2c, touch_int);
```

## Pin Summary

| Port | Total Pins | SDRAM | Other Used | Available |
|------|-----------|-------|------------|-----------|
| A | 16 | 0 | 3 (BTN, USB) | 13 |
| B | 16 | 0 | 2 (I2C) | 14 |
| C | 16 | 1 | 6 (Touch, SDIO) | 9 |
| D | 16 | 7 | 3 (SDIO, LED) | 6 |
| E | 16 | 11 | 0 | 5 |
| F | 16 | 11 | 0 | 5 |
| G | 16 | 6 | 1 (LED) | 9 |
| H | 16 | 10 | 1 (LCD) | 5 |
| I | 12 | 10 | 0 | 2 |
| K | 8 | 0 | 1 (LED) | 7 |
| **Total** | **148** | **52** | **17** | **75** |

The SDRAM consumes 52 pins, leaving 96 pins for other uses (75 of which are on ports A-K).
