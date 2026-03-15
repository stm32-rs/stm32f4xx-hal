# Remote STM32F469 Development Setup

This document explains how to remotely flash and monitor firmware on an STM32F469NIHx board connected to a remote machine.

---

## Hardware Setup

```
┌─────────────────┐      USB      ┌─────────────────┐      Network      ┌─────────────────┐
│   Development   │◄────────────►│   Remote Host   │◄─────────────────►│   STM32F469     │
│     Machine     │               │                 │                   │   Board         │
│   (Your PC)     │               │ ubuntu@192.168  │                   │   + ST-Link     │
└─────────────────┘               │    .13.246      │                   └─────────────────┘
                                  └─────────────────┘
```

The STM32F469 board is connected via ST-Link debugger to the remote host. You SSH into the remote host to flash and monitor.

---

## Remote Host Details

| Property | Value |
|----------|-------|
| IP Address | `192.168.13.246` |
| Username | `ubuntu` |
| SSH Command | `ssh ubuntu@192.168.13.246` |

---

## Prerequisites on Remote Host

The remote host must have:

1. **probe-rs** installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install probe-rs
   ```

2. **udev rules** for ST-Link (optional but recommended):
   ```bash
   # Create udev rule
   echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="374b", MODE="0666"' | sudo tee /etc/udev/rules.d/99-stlink.rules
   sudo udevadm control --reload-rules
   ```

3. **Verify probe is detected**:
   ```bash
   ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs list"
   ```
   
   Expected output:
   ```
   The following debug probes were found:
   [0]: STLink V2-1 -- 0483:374b:066FFF515786534867184152 (ST-LINK)
   ```

---

## Target Chip

| Property | Value |
|----------|-------|
| MCU | STM32F469NIHx |
| Core | Cortex-M4F @ 180MHz |
| Flash | 2MB |
| RAM | 384KB |
| probe-rs chip ID | `STM32F469NIHx` |

---

## Flashing Firmware

### Step 1: Copy ELF to Remote Host

```bash
scp firmware.elf ubuntu@192.168.13.246:/tmp/
```

### Step 2: Flash via probe-rs

```bash
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/firmware.elf && probe-rs reset --chip STM32F469NIHx"
```

**What this does:**
- `probe-rs download` - Flashes the ELF to the MCU
- `probe-rs reset` - Resets the MCU to start execution

---

## Monitoring Logs (defmt/RTT)

The firmware must be built with `defmt` and `defmt-rtt` for logging.

### Build Configuration (Required)

In `Cargo.toml`:
```toml
[dependencies]
defmt = "0.3"
defmt-rtt = "0.4"

[profile.release]
debug = 2           # REQUIRED for source locations in logs
opt-level = "s"
lto = true
```

In `main.rs`:
```rust
use defmt_rtt as _;  // MUST be included for RTT to work

fn main() -> ! {
    defmt::info!("Firmware starting...");
    // ...
}
```

### Capture Logs with probe-rs run

The best way to capture logs is using `probe-rs run` which flashes, resets, and attaches in one command:

```bash
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 120 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf"
```

**Parameters explained:**
- `timeout 120` - Stop after 120 seconds
- `probe-rs run` - Flash and attach for logging
- `--chip STM32F469NIHx` - Target chip
- `--log-format full` - Show full log format with timestamps
- `--rtt-scan-memory` - Scan memory to find RTT control block

### Why NOT probe-rs attach

`probe-rs attach` connects to an already-running MCU and often misses boot logs. Use `probe-rs run` instead.

```bash
# ❌ DON'T USE - misses boot logs
probe-rs attach --chip STM32F469NIHx /tmp/firmware.elf

# ✅ USE THIS - captures everything from boot
probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf
```

---

## Complete Workflow

### One Command (Flash + Monitor)

```bash
scp firmware.elf ubuntu@192.168.13.246:/tmp/ && \
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 120 probe-rs run --chip STM32F469NIHx --log-format full --rtt-scan-memory /tmp/firmware.elf 2>&1"
```

### Two Commands (Flash, then Monitor)

```bash
# 1. Flash only
scp firmware.elf ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/firmware.elf && probe-rs reset --chip STM32F469NIHx"

# 2. Monitor (separate SSH session)
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && timeout 60 probe-rs attach --chip STM32F469NIHx /tmp/firmware.elf"
```

---

## Troubleshooting

### "interface is busy" Error

A previous probe-rs process is still running:

```bash
ssh ubuntu@192.168.13.246 "pkill -9 -f 'probe-rs'"
```

Then retry the flash command.

### No Log Output

1. **Check `debug = 2`** in release profile
2. **Check `use defmt_rtt as _;`** is in main.rs
3. **Use `probe-rs run`** not `probe-rs attach`
4. **Add `--rtt-scan-memory`** flag

### "Insufficient DWARF info" Warning

The firmware was compiled without debug info. Add to `Cargo.toml`:

```toml
[profile.release]
debug = 2
```

### Probe Not Found

```bash
# Check USB connection
ssh ubuntu@192.168.13.246 "lsusb | grep -i st"

# Check probe-rs can see it
ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs list"
```

---

## Quick Reference Card

```bash
# Remote host
HOST="ubuntu@192.168.13.246"
CHIP="STM32F469NIHx"
ELF="/tmp/firmware.elf"

# Copy firmware
scp firmware.elf $HOST:/tmp/

# Flash and monitor (one command)
ssh $HOST ". ~/.cargo/env && timeout 120 probe-rs run --chip $CHIP --log-format full --rtt-scan-memory $ELF"

# Flash only
ssh $HOST ". ~/.cargo/env && probe-rs download --chip $CHIP $ELF && probe-rs reset --chip $CHIP"

# Kill stale probe-rs
ssh $HOST "pkill -9 -f probe-rs"

# Check probe
ssh $HOST ". ~/.cargo/env && probe-rs list"
```

---

## Notes

- The board uses an **ST-Link V2-1** debugger (built into the board)
- The remote host runs **probe-rs 0.31.0**
- Firmware must use **defmt 0.3** and **defmt-rtt 0.4** for logging
- Boot logs are captured best with `probe-rs run` + `--rtt-scan-memory`
