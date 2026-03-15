# Feedback for v3.7 - HARDWARE CAPABILITIES LOGGING

**Status**: ✅ WORKING - New hardware capability detection

**Test Date**: March 2026

---

## Summary

v3.7 adds hardware capability detection and logging at boot, showing which features are available on the device. The firmware correctly identifies available and missing hardware.

---

## What's New in v3.7

### Hardware Capabilities Logging

```
INFO  [HW] Hardware capabilities:
INFO  [HW]   Display:   YES
INFO  [HW]   Touch:     YES
INFO  [HW]   USB:       YES
INFO  [HW]   RNG:       YES
INFO  [HW]   Camera:    NO (QR scanning unavailable)
INFO  [HW]   SD Card:   NO
INFO  [HW]   Battery:   NO
```

### Hardware Limitations Warnings

```
WARN  [HW] LIMITATION: No camera - QR code scanning not available
WARN  [HW] LIMITATION: No SD card - backup to SD not available
WARN  [HW] LIMITATION: No battery - requires external power
```

### Menu Listing at Boot

```
INFO  Menu: Home, WalletGen, LoadWallet,
INFO       SignTx, Settings, About
```

### Enhanced Heartbeat

Now includes wallet state:
```
INFO  [HEARTBEAT] frame=500 screen=Home touches=0 dirty=false wallet=NoWallet
```

After navigating to SignTx:
```
INFO  [HEARTBEAT] frame=6000 screen=SignTx touches=8 dirty=false wallet=Loaded
```

---

## What Works

| Feature | Status | Notes |
|---------|--------|-------|
| Boot | ✅ | Clean, ~4s |
| Display | ✅ | LTDC layer enabled |
| Touch | ✅ | 15 events, no panic |
| Navigation | ✅ | Home → LoadWallet → Home → WalletGen → Home → SignTx |
| RNG | ✅ | Ready after 15 iterations |
| Hardware detection | ✅ | NEW - Correctly identifies capabilities |
| Heartbeat | ✅ | Every 500 frames with wallet state |
| Dirty flag | ✅ | `dirty=false` - no unnecessary redraws |
| Stability | ✅ | 7000+ frames, no crash |

---

## Boot Log

```
INFO  Specter-DIY Rust Firmware v3.7
INFO  [BOOT] Taking peripherals...
INFO  [RNG] Enabling clock...
INFO  [RCC] SYSCLK=168000000 Hz
INFO  [RNG] Ready after 15 iterations
INFO  [GPIO] Configuring...
INFO  [LCD] Reset...
INFO  [LCD] Reset OK
INFO  [LED] ON
INFO  [SDRAM] Initializing...
INFO  [SDRAM] Base = 0xc0000000
INFO  [FB] Clearing...
INFO  [DISPLAY] Initializing LTDC/DSI...
INFO  Detected LCD controller: Nt35510
INFO  Display initialized successfully
INFO  [LTDC] Configuring layer L1...
INFO  [LTDC] Enabling layer L1...
INFO  [LTDC] Layer enabled
INFO  [TOUCH] Initializing...
INFO  [HW] Hardware capabilities:
INFO  [HW]   Display:   YES
INFO  [HW]   Touch:     YES
INFO  [HW]   USB:       YES
INFO  [HW]   RNG:       YES
INFO  [HW]   Camera:    NO (QR scanning unavailable)
INFO  [HW]   SD Card:   NO
INFO  [HW]   Battery:   NO
WARN  [HW] LIMITATION: No camera - QR code scanning not available
WARN  [HW] LIMITATION: No SD card - backup to SD not available
WARN  [HW] LIMITATION: No battery - requires external power
INFO  [GUI] Initializing...
INFO  READY - Starting main loop
INFO  Menu: Home, WalletGen, LoadWallet,
INFO       SignTx, Settings, About
```

---

## Hardware Detection Summary

| Capability | Detected | Impact |
|------------|----------|--------|
| Display | ✅ YES | UI available |
| Touch | ✅ YES | Touch navigation works |
| USB | ✅ YES | USB communication possible |
| RNG | ✅ YES | Hardware random number generation |
| Camera | ❌ NO | QR code scanning unavailable |
| SD Card | ❌ NO | Backup to SD not available |
| Battery | ❌ NO | Requires external power |

---

## Touch Events

15 touch events logged without any panic:
```
INFO  [TOUCH] Event #1: (94, 233)
INFO  [TOUCH] Release at (94, 233)
INFO  [NAV] Home -> LoadWallet
...
INFO  [NAV] LoadWallet -> Home
...
INFO  [NAV] Home -> WalletGen
...
INFO  [NAV] WalletGen -> Home
...
INFO  [NAV] Home -> SignTx
```

---

## Navigation Flow

User navigated through:
1. Home → LoadWallet
2. LoadWallet → Home
3. Home → WalletGen
4. WalletGen → Home
5. Home → SignTx

---

## Wallet State Tracking

Heartbeat now shows wallet state:
- `wallet=NoWallet` - No wallet loaded (initial state)
- `wallet=Loaded` - Wallet loaded (after navigating to SignTx)

---

## Session Statistics

- Touch events: 15
- Frames: 7000+
- Screens visited: Home, LoadWallet, WalletGen, SignTx
- Exit: User requested (SIGTERM)
- Crashes: None
- Dirty flag: `false` (no unnecessary redraws)

---

## Version Comparison

| Feature | v3.5 | v3.6 | v3.7 |
|---------|------|------|------|
| Hardware detection | ❌ | ❌ | ✅ |
| Menu listing | ❌ | ❌ | ✅ |
| Wallet state in heartbeat | ❌ | ❌ | ✅ |
| Dirty flag | ❌ | ✅ | ✅ |
| Display | ✅ | ✅ | ✅ |
| Touch | ✅ | ✅ | ✅ |

---

## Observations

### Positive
1. **Hardware detection works** - Correctly identifies available hardware
2. **Clear limitations logged** - User knows what features are unavailable
3. **Menu listing helpful** - Shows available screens at boot
4. **Wallet state tracking** - Useful for debugging

### Still Missing (vs v2.5)
1. **USB initialization logging** - `[USB] Init...` / `[USB] OK` / `[USB] Connected`
2. **SD card initialization logging** - Even though not available, should log detection attempt
3. **Frame-by-frame logging** - Useful for debugging (optional)

### Known Limitations (Hardware)
1. **No camera** - QR scanning not available
2. **No SD card** - Backup to SD not available  
3. **No battery** - Requires external power

---

## Recommendations for v3.8

1. **Add USB initialization logging** - Even if detected as available, log the init sequence
2. **Test USB communication** - Verify USB actually works with host PC
3. **Consider double buffering** - To eliminate remaining flicker on screen changes (see `troubleshoot_flicker.md`)

---

*Report generated from v3.7 testing session*

---

## Update: Second Test Run

A second test of v3.7 revealed additional features not logged in the first run:

### USB Status Changed
```
INFO  [HW]   USB:       YES (stub)
```
USB is now marked as "(stub)" - indicating it's a stub implementation, not fully functional.

### Bitcoin Features (NEW)
```
INFO  Bitcoin: BIP39, BIP32, Addresses, PSBT
```
This line shows the Bitcoin-related features supported:
- **BIP39**: Mnemonic seed phrases (12/24 word backups)
- **BIP32**: Hierarchical Deterministic wallets (HD wallets)
- **Addresses**: Bitcoin address generation
- **PSBT**: Partially Signed Bitcoin Transactions

### Session Statistics (Second Run)
- Touch events: 13
- Frames: 1000+
- Screens visited: Home, WalletGen, SignTx
- Exit: USB probe disconnection
- Crashes: None

---

*Updated from second v3.7 testing session*
