# Firmware Next Steps — Specter-DIY STM32F469NIHx

## Current Status (CONFIRMED WORKING on real hardware)

Everything below has been verified on a physical STM32F469NIHx board via probe-rs:

- **Boot**: Clean, ~4s to full init
- **RNG**: Hardware RNG works, produces valid random values, no hang
- **SDRAM**: Verified at 0xC0000000
- **Display**: NT35510 (B08 board), 480×800 portrait, DSI HS mode, color bar test pattern visible
- **Framebuffer**: 480×800 in SDRAM, LTDC layer configured
- **Touch**: FT6X06 working, safe multi-touch wrapper active, 50ms debounce, Y calibration 780→799
- **LED**: Responds to touch (blinks on touch event)
- **USB**: OTG FS connected and enumerated

The test pattern (color bars) is displayed. Touch input is detected and causes LED blinks. No crashes observed.

## What To Build Next

The hardware abstraction layer is done. Now build the actual Specter-DIY wallet UI and functionality on top of it. Proceed in this order:

### Step 1: Basic UI Framework

Replace the color bar test pattern with an actual UI rendering system:

- Implement a simple framebuffer drawing API: `fill_rect`, `draw_text`, `draw_line`, `clear_screen`
- Use a bitmap font (embedded in firmware, no filesystem needed) — 8×16 monospace is fine to start
- Implement a screen/page abstraction: each screen is a struct that can `render(&self, fb: &mut Framebuffer)` and `handle_touch(&mut self, x: u16, y: u16) -> Action`
- Colors: white text on black background (OLED-style), with a blue accent for selected items

### Step 2: Navigation and Menu System

Build a simple menu system using the touch input:

- Home screen with a list of menu items (vertically stacked buttons, full-width)
- Menu items: "Generate Wallet", "Load Wallet", "Sign Transaction", "Settings", "About"
- Touch a menu item → navigate to that screen
- Back button (top-left corner touch zone, or a rendered "← Back" button)
- Keep it simple: no animations, no scrolling yet — just full-screen page transitions

### Step 3: Wallet Generation (First Real Feature)

Implement BIP39 mnemonic generation using the hardware RNG:

- Generate 256 bits of entropy from the hardware RNG (already working)
- Convert to a 24-word BIP39 mnemonic (embed the BIP39 English wordlist as a const array)
- Display the 24 words on screen (paginated: 6 words per page, 4 pages, swipe or tap to advance)
- "Confirm Backup" screen: ask user to verify 3 random words by selecting from 4 choices each
- On confirmation: derive the master key (BIP32) and store in RAM (NOT flash, for now)

### Step 4: USB Communication

Implement basic USB HID or CDC communication for host interaction:

- Enumerate as a USB HID device (or CDC serial — whichever is simpler with the existing USB setup)
- Accept JSON-formatted commands from the host
- Start with a simple ping/pong: host sends `{"method": "ping"}`, device responds `{"result": "pong"}`
- Then add `get_xpub` command: returns the master xpub derived from the loaded wallet

## Constraints

- **No heap allocation** — use fixed-size buffers, `heapless` collections
- **No filesystem** — everything in RAM or const. Flash storage comes later
- **No std** — this is `#![no_std]` embedded Rust
- **Keep defmt logging** — log every significant state transition, but use `defmt::debug!` for high-frequency events (touch polling) and `defmt::info!` for lifecycle events
- **Don't break what works** — the boot sequence, RNG, display init, and touch init are proven. Don't refactor them. Build on top.
- **Build profile**: keep `debug = 2` in release profile for defmt source locations

## Hardware Reference

- **MCU**: STM32F469NIHx, 180MHz Cortex-M4F, 2MB flash, 384KB SRAM
- **External SDRAM**: 16MB at 0xC0000000 (framebuffer lives here)
- **Display**: 480×800 portrait, NT35510 controller, MIPI-DSI interface
- **Touch**: FT6X06 capacitive, I2C, raw X 0–479, raw Y 0–780
- **USB**: OTG FS (PA11/PA12)
- **SD Card**: SDIO interface (present pin checked, not currently inserted)
- **LED**: GPIO-driven, currently used for touch feedback

## Dependencies You Already Have

- `defmt` + `defmt-rtt` for logging
- `cortex-m-rt` for runtime
- `stm32f4xx-hal` or register-level PAC for peripherals
- `ft6x06` crate (with your safe multi-touch wrapper on top)
- `heapless` for fixed-size collections
- USB OTG driver (already enumerating)

## What NOT To Do

- Don't add `alloc` or a heap allocator
- Don't refactor the boot sequence or peripheral init — it works
- Don't add a graphics library like `embedded-graphics` unless you can justify the flash cost — raw framebuffer writes are fine for this UI
- Don't implement flash storage yet — RAM-only for now
- Don't try to implement full Bitcoin signing yet — get the UI and key generation working first
