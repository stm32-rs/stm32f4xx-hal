# Firmware Feedback for Next Version — Specter-DIY STM32F469NIHx

## Current Status: v2.7

v2.7 boots successfully and has good logging infrastructure. Here's what's working and what needs to be added.

---

## ✅ What's Working in v2.7

Boot sequence with proper logging:
```
INFO  Specter-DIY Rust Firmware v2.7
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [GPIO] Ports configured
INFO  [LCD] Reset complete
INFO  [SDRAM] Initialized at 0xc0000000
INFO  [SDRAM] Framebuffer ready
INFO  [DISPLAY] Controller: Nt35510
INFO  [TOUCH] FT6X06 initialized OK
INFO  [GUI] Screen manager ready
INFO  Ready! Touch screen to interact
```

- ✅ Source locations working (file:line)
- ✅ Boot logging with prefixes
- ✅ Boots in ~2.9s
- ✅ SYSCLK at 180MHz
- ✅ No crashes

---

## 🔴 What's Missing (Compared to v2.5)

v2.5 had a complete wallet firmware with UI, wallet generation, and USB. v2.7 is missing these features. Add them all:

### 1. RNG Initialization

Add hardware RNG initialization with a test read:

```rust
// After GPIO configuration, add:
defmt::info!("[RNG] Hardware RNG initialization...");

// Enable RNG clock
device.RCC.ahb2enr.modify(|_, w| w.rngen().set_bit());
defmt::info!("[RNG] Clock enabled");

// Create RNG instance and test
let mut rng = HardwareRng::new(device.RNG);
let test_val = rng.gen_u32();
defmt::info!("[RNG] Test OK: {:#x}", test_val);
```

---

### 2. USB Initialization

Add USB OTG FS initialization:

```rust
// After display/touch init, add:
defmt::info!("[USB] Init...");

let usb_bus = UsbBus::new(device.OTG_FS_GLOBAL, device.OTG_FS_DEVICE, device.OTG_FS_PWRCLK);
let mut serial = usbd_serial::SerialPort::new(&usb_bus);
let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x8888))
    .strings(&[StringDescriptors::default()
        .manufacturer("Specter")
        .product("Specter-DIY")
        .serial_number("v2.8")])
    .unwrap()
    .device_class(usbd_serial::DEVICE_CLASS)
    .build();

defmt::info!("[USB] OK");
```

In the main loop, poll USB and log connection:
```rust
if usb_dev.poll(&mut [&mut serial]) {
    defmt::info!("[USB] Connected - state=Configured");
    // Handle USB data...
}
```

---

### 3. Touch Event Logging

Log every touch event with coordinates:

```rust
// In main loop:
let mut touch_event_count: u32 = 0;

if let Some((x, y, fingers)) = touch.read() {
    touch_event_count += 1;
    defmt::info!("[TOUCH] Event #{}: {} finger(s) at ({}, {})", 
        touch_event_count, fingers, x, y);
    defmt::info!("[TOUCH] screen=({}, {}) fingers={}", x, y, fingers);
}
```

---

### 4. Home Screen with Menu Items

Create a home screen with these menu items:

```rust
enum MenuItem {
    GenerateWallet,
    LoadWallet,
    SignTransaction,
    Settings,
    About,
}

impl MenuItem {
    fn name(&self) -> &'static str {
        match self {
            MenuItem::GenerateWallet => "Generate Wallet",
            MenuItem::LoadWallet => "Load Wallet",
            MenuItem::SignTransaction => "Sign Transaction",
            MenuItem::Settings => "Settings",
            MenuItem::About => "About",
        }
    }
}
```

Render menu and log selection:
```rust
fn render_home_screen(fb: &mut Framebuffer, menu_items: &[MenuItem], selected: usize) {
    defmt::info!("[FRAME] #{} - Rendering screen Home", frame_count);
    // Draw menu items...
}

fn handle_touch(x: u16, y: u16, menu_items: &[MenuItem]) {
    defmt::info!("[HOME] touch at ({}, {})", x, y);
    
    // Determine which item was touched
    if let Some(item) = get_touched_item(x, y, menu_items) {
        defmt::info!("[HOME] menu item {} selected: {}", item.index, item.name());
    }
}
```

---

### 5. Screen Navigation

Implement screen navigation with logging:

```rust
enum Screen {
    Home,
    WalletGen(WalletGenState),
    LoadWallet,
    SignTx,
    Settings,
    About,
}

fn navigate_to(screen: Screen) {
    defmt::info!("[NAV] -> {:?}", screen_name(&screen));
    current_screen = screen;
}

fn go_back() {
    defmt::info!("[NAV] <- {:?}", screen_name(&previous_screen));
    current_screen = previous_screen;
}
```

---

### 6. Wallet Generation Flow

Implement the full wallet generation flow:

```rust
enum WalletGenState {
    Start,
    DisplayWords { page: u8 },  // 4 pages, 6 words each
    ConfirmBackup { step: u8 }, // 3 verification steps
    Complete { passed: bool },
}

fn generate_wallet(rng: &mut HardwareRng) -> [u8; 32] {
    defmt::info!("[RNG] Generating entropy for wallet...");
    let mut entropy = [0u8; 32];
    rng.fill_bytes(&mut entropy);
    defmt::info!("[RNG] Entropy generated, passing to wallet screen...");
    entropy
}

fn generate_mnemonic(entropy: &[u8; 32]) -> Vec<&'static str, 24> {
    // BIP39 mnemonic generation
    let words = bip39::from_entropy(entropy);
    defmt::info!("[WALLET] Generated mnemonic with checksum");
    words
}

fn advance_page(state: &mut WalletGenState) {
    if let WalletGenState::DisplayWords { page } = state {
        if *page < 3 {
            *page += 1;
            defmt::info!("[WALLET] Continue button pressed, advancing page");
        } else {
            *state = WalletGenState::ConfirmBackup { step: 0 };
            defmt::info!("[WALLET] Starting backup verification");
        }
    }
}

fn verify_word(step: u8, selected_option: u8, correct: bool) {
    defmt::info!("[WALLET] Verify step {}: option {} selected (correct={})", 
        step, selected_option, correct);
}
```

---

### 7. Heartbeat Logging

Add periodic heartbeat to show the firmware is alive:

```rust
let mut frame_count: u32 = 0;
let mut last_heartbeat: u32 = 0;

loop {
    frame_count += 1;
    
    // Heartbeat every 500 frames
    if frame_count - last_heartbeat >= 500 {
        defmt::info!("[HEARTBEAT] frame={} screen={:?} dirty={}", 
            frame_count, current_screen, dirty);
        last_heartbeat = frame_count;
    }
    
    // ... rest of main loop
}
```

---

### 8. Settings and About Screens

Simple placeholder screens:

```rust
fn render_settings_screen(fb: &mut Framebuffer) {
    defmt::info!("[FRAME] #{} - Rendering screen Settings", frame_count);
    // Draw settings options
}

fn render_about_screen(fb: &mut Framebuffer) {
    defmt::info!("[FRAME] #{} - Rendering screen About", frame_count);
    // Draw version info: "Specter-DIY v2.8"
}
```

---

## Expected Log Output

When complete, the firmware should produce logs like this:

```
INFO  Specter-DIY Rust Firmware v2.8
INFO  [BOOT] Taking peripherals...
INFO  [BOOT] Peripherals OK
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [GPIO] Ports configured
INFO  [RNG] Hardware RNG initialization...
INFO  [RNG] Test OK: 0x12345678
INFO  [SDRAM] Initialized at 0xc0000000
INFO  [DISPLAY] Controller: Nt35510
INFO  [TOUCH] FT6X06 initialized OK
INFO  [USB] Init...
INFO  [USB] OK
INFO  [GUI] Screen manager ready
INFO  Ready! Touch screen to interact
INFO  [USB] Connected - state=Configured
INFO  [FRAME] #1 - Rendering screen Home
INFO  [HEARTBEAT] frame=500 screen=Home dirty=false
INFO  [TOUCH] Event #1: 1 finger(s) at (192, 124)
INFO  [HOME] touch at (192, 124)
INFO  [HOME] menu item 0 selected: Generate Wallet
INFO  [NAV] -> WalletGen
INFO  [FRAME] #50 - Rendering screen WalletGen
INFO  [RNG] Generating entropy for wallet...
INFO  [WALLET] Generated mnemonic with checksum
INFO  [WALLET] Continue button pressed, advancing page
...
```

---

## Build Configuration (Keep This)

```toml
[profile.release]
debug = 2           # REQUIRED for defmt source locations
opt-level = "s"
lto = true
codegen-units = 1
```

---

## Summary Checklist

Add all of these in the next version:

- [ ] RNG initialization with test logging
- [ ] USB initialization with connection logging
- [ ] Touch event logging with coordinates
- [ ] Home screen with 5 menu items
- [ ] Screen navigation (Home, WalletGen, LoadWallet, SignTx, Settings, About)
- [ ] Wallet generation flow (entropy → mnemonic → display → verify → complete)
- [ ] Frame rendering logs
- [ ] Heartbeat every 500 frames
- [ ] Back navigation from screens

Target version: **v2.8**
