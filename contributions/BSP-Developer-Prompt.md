# Prompt for stm32f469i-disc BSP Developers

## Summary

We're building a Bitcoin hardware wallet (Specter-DIY) for STM32F469I-DISCO using Rust. Our firmware is working well (v4.6), but we had to implement ~600 lines of display/hardware code that would benefit the community if added to the BSP.

---

## Request: Display Support Module

The BSP currently lacks display support. For GUI applications, this is essential. Would you consider adding a `display` module?

### What We Need

```rust
// Ideal BSP API
pub mod display {
    pub struct LcdDisplay {
        // Handles DSI + LTDC + panel init
    }

    impl LcdDisplay {
        /// Full initialization sequence for NT35510/OTM8009A
        pub fn init(dsi: DSI, ltdc: LTDC, dma2d: DMA2D, rcc: &mut Rcc, delay: &mut impl DelayMs<u32>) -> Result<Self, Error>;

        /// Which controller was detected
        pub fn controller(&self) -> LcdController;

        /// Access to framebuffer in SDRAM
        pub fn framebuffer(&mut self) -> &mut [u16];
    }

    pub enum LcdController {
        Nt35510,  // B08 revision
        Otm8009a, // B07 and earlier
    }
}
```

### Why This Helps

1. **Reduced boilerplate**: Every GUI project needs the same DSI/LTDC setup
2. **Proper defaults**: Panel timing, clock config, PHY timers are tricky
3. **Controller detection**: Runtime detection of NT35510 vs OTM8009A prevents issues

---

## Request: Safe Touch Wrapper

The `ft6x06` crate panics when touch count > 2. A safe wrapper in the BSP would prevent crashes:

```rust
pub mod touch {
    pub struct SafeTouch { /* ... */ }

    impl SafeTouch {
        /// Non-blocking read, clamps touch count to prevent panic
        pub fn read(&mut self, i2c: &mut I2c) -> Option<TouchEvent>;
    }

    pub struct TouchEvent {
        pub x: u16,
        pub y: u16,
        pub count: u8,  // Clamped to 2 max
    }
}
```

---

## Optional: USB VCP

If you have bandwidth, USB virtual COM port support would be valuable:

- Uses OTG FS on PA11/PA12
- Useful for debugging and host communication
- Amperstrand BSP has a working implementation that could be referenced

---

## Our Test Results

| Component | Status | Notes |
|-----------|--------|-------|
| RNG | ✅ | PLL48CLK 48MHz working |
| SDRAM | ✅ | 16MB at 0xc0000000 |
| Display | ✅ | NT35510, 480x800 |
| Touch | ✅ | Safe wrapper prevents panic |
| Frame Loop | ✅ | 300+ frames tested |
| USB VCP | ❌ | Not implemented yet |

**Known Issue**: Display flickering due to lack of double buffering. We work around this with dirty region tracking.

---

## Files We Could Contribute

If helpful, we can contribute our working implementations:

1. `board/mod.rs` - Full DSI/LTDC/panel init (~400 lines)
2. Touch safe wrapper (~50 lines)
3. Display timing constants for both panel revisions

---

## Contact

Let us know if this is something you'd like to add to the BSP. We're happy to help with testing or contribute code!

---

**References**:
- STM32F469I-DISCO User Manual (UM1932)
- NT35510 datasheet (B08 revision)
- stm32f4xx-hal: https://docs.rs/stm32f4xx-hal
