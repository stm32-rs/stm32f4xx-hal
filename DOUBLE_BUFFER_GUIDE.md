# Double Buffering for STM32F469I-DISCO — Implementation Guide

Thanks for the kind words about the BSP! Glad the FT6X06 fix is working for you.

The good news is the HAL already has everything you need for double buffering — including a working reference example. Here's the minimal setup:

---

## 1. Allocate Two Framebuffers in SDRAM

Your 16MB SDRAM at `0xC0000000` has plenty of room. For 480×800 RGB565, each buffer is 768KB.

```rust
const WIDTH: usize = 480;
const HEIGHT: usize = 800;
const FB_SIZE: usize = WIDTH * HEIGHT;  // 384,000 u16s (768 KB)

// After SDRAM init
let base_ptr = sdram.init(&mut delay) as *mut u16;

// Buffer A at SDRAM base
let buf_a: &'static mut [u16] = 
    unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr, FB_SIZE) };

// Buffer B right after
let buf_b: &'static mut [u16] = 
    unsafe { &mut *core::ptr::slice_from_raw_parts_mut(base_ptr.add(FB_SIZE), FB_SIZE) };

// Store addresses for swapping
let buf_a_addr = base_ptr as u32;
let buf_b_addr = unsafe { base_ptr.add(FB_SIZE) } as u32;
```

---

## 2. Configure LTDC Layer

```rust
let (mut display_ctrl, _controller) = board::init_display_full(
    dp.DSI, dp.LTDC, dp.DMA2D,
    &mut rcc, &mut delay,
    board::BoardHint::Unknown,
    PixelFormat::RGB565,
);

// Start with buffer A
display_ctrl.config_layer(Layer::L1, buf_a, hal::ltdc::PixelFormat::RGB565);
display_ctrl.enable_layer(Layer::L1);
display_ctrl.reload();
```

---

## 3. Double Buffer Swap Helper

```rust
struct DoubleBuffer {
    front_addr: u32,
    back_addr: u32,
}

impl DoubleBuffer {
    fn new(a: u32, b: u32) -> Self {
        Self { front_addr: a, back_addr: b }
    }
    
    /// Get back buffer for drawing (NOT currently displayed)
    fn back_buffer(&self) -> &'static mut [u16] {
        unsafe { 
            core::slice::from_raw_parts_mut(self.back_addr as *mut u16, FB_SIZE) 
        }
    }
    
    /// Swap buffers (VSYNC-synchronized)
    fn swap(&mut self, display_ctrl: &LtdcFramebuffer<..., PixelFormat>) {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        display_ctrl.set_layer_buffer_address(Layer::L1, self.back_addr);
        core::mem::swap(&mut self.front_addr, &mut self.back_addr);
    }
}
```

---

## 4. Render Loop

```rust
let mut dbl_buf = DoubleBuffer::new(buf_a_addr, buf_b_addr);

loop {
    // Draw to back buffer
    let back = dbl_buf.back_buffer();
    
    // Clear
    for pixel in back.iter_mut() {
        *pixel = 0x0000; // Black
    }
    
    // Draw your UI (embedded-graphics works on top of raw slice)
    draw_wallet_ui(back);
    
    // Swap — happens on next VSYNC, no tearing
    dbl_buf.swap(&display_ctrl);
    
    // Frame timing
    delay.delay_ms(16);
}
```

---

## VSYNC Synchronization

The key method is `set_layer_buffer_address()` in `src/ltdc.rs`. It internally calls `reload_on_vblank()`, so the address change takes effect during vertical blanking — no tearing.

For TE (Tearing Effect) pin synchronization with NT35510, you'd need that wired up, but LTDC VSYNC is sufficient for this board.

---

## Reference Implementation

See the complete working example:

```
examples/f469disco-image-slider.rs
```

Lines 107-110: Buffer allocation  
Lines 186-196: Swap logic

Build with:
```bash
cargo build --release --example f469disco-image-slider \
    --features="stm32f469,stm32-fmc,framebuffer,defmt"
```

---

## Memory Layout

```
SDRAM @ 0xC0000000 (16 MB):
┌─────────────────────────┐
│ Buffer A  (768 KB)      │  0xC0000000
├─────────────────────────┤
│ Buffer B  (768 KB)      │  0xC0177000
├─────────────────────────┤
│ Available (~14.5 MB)    │
└─────────────────────────┘
```

---

This should eliminate your flickering completely. The draw-to-back-then-swap pattern means users never see partial renders — just instant frame transitions.

Happy to help test if you run into any issues!
