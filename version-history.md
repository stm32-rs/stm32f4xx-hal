# Firmware Version History — Specter-DIY STM32F469NIHx

Summary of all firmware versions tested on real hardware.

---

## Quick Reference

| Version | Status | Key Features | Issues |
|---------|--------|--------------|--------|
| v1.5 | ✅ | Boot, RNG, touch safe, USB | Color bars only, no GUI |
| v2.2 | ✅ | GUI framework, menu selection | Render log flooding |
| v2.3 | ✅ | Wallet generation flow | None |
| v2.5 | ✅ | All screens, navigation | None - best version |
| v2.6 | ❌ | None | No debug, FT6X06 panic, missing features |
| v2.7 | ⚠️ | Boot logging, source locations | Missing RNG/USB/UI logging |
| v2.8 | ⚠️ | Boots, no crash | No debug info, minimal logging |

---

## Best Version: v2.5

v2.5 had everything working:
- Full boot logging with all prefixes
- RNG initialization and test
- USB initialization and connection
- Touch event logging
- Home screen with 5 menu items
- Wallet generation (full flow)
- Settings and About screens
- Forward and back navigation
- Heartbeat every 500 frames
- Clean source locations

---

## Current Version: v2.7

v2.7 is a clean restart with:
- ✅ Source locations working
- ✅ Boot logging restored
- ✅ Boots fast (2.9s)
- ✅ SYSCLK at 180MHz
- ✅ No crashes
- ❌ Missing RNG initialization
- ❌ Missing USB initialization
- ❌ Missing touch event logging
- ❌ Missing UI logging
- ❌ Missing heartbeat

---

## Next Version: v2.8

Use `feedback-prompt.md` to add:
1. RNG initialization with test
2. USB initialization with connection logging
3. Touch event logging
4. Home screen with 5 menu items
5. Screen navigation
6. Wallet generation flow
7. Frame rendering logs
8. Heartbeat every 500 frames

---

## Critical Rules

1. **Always** use `debug = 2` in release profile
2. **Never** remove FT6X06 safe touch wrapper
3. **Always** log with prefixes: `[BOOT]`, `[RCC]`, `[RNG]`, `[USB]`, `[TOUCH]`, `[HOME]`, `[NAV]`, `[FRAME]`, `[HEARTBEAT]`
4. **Never** remove working features when updating

---

## Command Reference

```bash
# Flash and capture logs
scp firmware.elf ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 120 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf"
```
