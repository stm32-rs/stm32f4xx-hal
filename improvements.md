# Firmware Version History & Feedback — Specter-DIY STM32F469NIHx

Live hardware testing feedback for each firmware version on a real STM32F469NIHx board.

---

## Remote Setup

- **Remote Host:** `ubuntu@192.168.13.246`
- **Chip:** `STM32F469NIHx`
- **Probe:** ST-Link V2-1

**Flash and monitor:**
```bash
scp firmware.elf ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 120 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf"
```

---

## Version Summary

| Version | Status | Boot | RNG | USB | UI | Wallet | Notes |
|---------|--------|------|-----|-----|-----|--------|-------|
| v1.5 | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | Baseline, color bars only |
| v2.2 | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | GUI works, render log flooding |
| v2.3 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | Full wallet flow |
| v2.5 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | **Best version** - all screens |
| v2.6 | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | No debug, FT6X06 panic |
| v2.7 | ⚠️ | ✅ | ❌ | ❌ | ⚠️ | ❌ | Missing RNG/USB/logging |
| v2.8 | ❌ | ⚠️ | ❌ | - | - | - | **Hangs at RNG init** |

---

## Detailed Version Notes

### v1.5 — Baseline Working
```
INFO  Specter-DIY v1.5
INFO  Multi-touch fix + RNG timeout
INFO  [RCC] SYSCLK=168000000 Hz
INFO  [RNG] Test value: 0x042692dc
INFO  [USB] OK
INFO  INIT COMPLETE!
```
- ✅ All hardware working
- ✅ Multi-touch safe wrapper
- ✅ RNG with timeout
- ❌ No GUI (color bars only)

---

### v2.2 — GUI Framework
```
INFO  Specter-DIY v2.2
INFO  [UI] render: Home screen (×200+)
INFO  [HOME] menu item 1 selected: Load Wallet
```
- ✅ UI framework working
- ✅ Menu selection works
- ⚠️ Render log flooding (every frame)

---

### v2.3 — Full Wallet Flow
```
INFO  Specter-DIY v2.3
INFO  [RNG] Test OK: 0xa10a7dd8
INFO  [HEARTBEAT] frame=500 screen=Home
INFO  [NAV] -> WalletGen
INFO  [WALLET] Generated mnemonic with checksum
INFO  [WALLET] Verification complete, passed=false
```
- ✅ Render flooding fixed
- ✅ Heartbeat every 500 frames
- ✅ Full wallet generation flow

---

### v2.5 — Best Version (All Features)
```
INFO  Specter-DIY v2.5
INFO  [NAV] -> Settings
INFO  [NAV] <- Home
INFO  [NAV] -> About
```
- ✅ All 5 menu items work
- ✅ Forward navigation (`->`)
- ✅ Back navigation (`<-`)
- ✅ Settings screen
- ✅ About screen
- ✅ Clean logging

---

### v2.6 — REGRESSED
```
WARN Insufficient DWARF info; compile with debug = 2
ERROR panicked at ft6x06-0.1.2/src/lib.rs:332:9
assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8
```
- ❌ No source locations (missing `debug = 2`)
- ❌ FT6X06 panic on first touch
- ❌ Started from wrong codebase

---

### v2.7 — Partial
```
INFO  Specter-DIY v2.7
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [TOUCH] FT6X06 initialized OK
INFO  [GUI] Screen manager ready
```
- ✅ Source locations restored
- ✅ Faster boot (2.9s)
- ✅ SYSCLK 180MHz
- ❌ No RNG init
- ❌ No USB init
- ❌ No touch/ UI logging

---

### v2.8 — HANGS AT RNG
```
INFO  Specter-DIY v2.8
INFO  [RCC] SYSCLK=180000000 Hz
INFO  [RNG] Hardware RNG initialization...
<HANGS - no further output>
```
- ✅ Source locations working
- ❌ **RNG initialization hangs**
- ❌ Never reaches display/touch/USB/GUI

**Fix:** Add timeout to RNG wait loop (see v1.5 for working implementation)

---

## Critical Rules for Firmware Development

### 1. Always Use `debug = 2`
```toml
[profile.release]
debug = 2           # REQUIRED for defmt source locations
```

### 2. Never Remove FT6X06 Safe Wrapper
The ft6x06 crate panics on multi-touch. Always clamp:
```rust
let touch_count = raw_touch_count.min(FT6X06_MAX_NB_TOUCH as u8);
```

### 3. RNG Must Have Timeout
```rust
let mut timeout = 10_000_000;  // ~100ms
while !rng.sr.read().drdy().bit() && timeout > 0 {
    timeout -= 1;
}
if timeout == 0 {
    defmt::warn!("[RNG] Timeout");
    // Handle error
}
```

### 4. Log With Prefixes
- `[BOOT]` - Peripheral init
- `[RCC]` - Clock config
- `[RNG]` - RNG init
- `[GPIO]` - GPIO config
- `[SDRAM]` - Memory init
- `[DISPLAY]` - Display
- `[TOUCH]` - Touch events
- `[USB]` - USB status
- `[GUI]` - UI framework
- `[HOME]` - Home screen
- `[NAV]` - Navigation
- `[FRAME]` - Rendering
- `[HEARTBEAT]` - Alive check

### 5. Don't Remove Working Features
Always verify existing features still work after changes.

---

## Next Version Requirements

Based on v2.5 (best version), add:
1. ✅ RNG initialization with timeout
2. ✅ USB initialization with connection logging
3. ✅ Touch event logging
4. ✅ Home screen with menu items
5. ✅ Screen navigation (forward + back)
6. ✅ Wallet generation flow
7. ✅ Settings and About screens
8. ✅ Frame rendering logs (on change only)
9. ✅ Heartbeat every 500 frames

---

## Build Configuration

```toml
[profile.release]
debug = 2           # REQUIRED
opt-level = "s"
lto = true
codegen-units = 1
```

---

## Command Reference

```bash
# Flash and monitor
scp firmware.elf ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 120 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf"

# Kill stale probe-rs
ssh ubuntu@192.168.13.246 "pkill -9 -f probe-rs"
```
