# F469-DISCO Bug Fixes: DSI Color Mismatch + ft6x06 Panic + Stability Hardening

## TL;DR

> **Quick Summary**: Fix two confirmed runtime bugs on STM32F469-DISCO hardware — (1) DSI↔LTDC color coding mismatch causing pixel offset/wrapping and (2) ft6x06 crate panic causing "display freeze" — plus memory.x RAM size correction and stability hardening. Empirical hardware verification at each step.
> 
> **Deliverables**:
> - DSI color coding corrected (`SixteenBitsConfig1`) — display pixels align 1:1 with touch coordinates
> - ft6x06 panic guard prevents assertion crash — display runs stable for 5+ minutes
> - All fixes verified on physical hardware via SSH+probe-rs remote flash
> - defmt diagnostic logs proving each fix works
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 2 waves after serial diagnostic phase
> **Critical Path**: Task 0 (connectivity) ✅ → Task 1 (diagnostic) ✅ → Task 2 (DSI color fix + memory.x) → Task 3 (force_rx_low_power) → Task 4 (ft6x06 guard) → Tasks 5-7 (parallel hardening) → Task 8 (soak test)

---

## Context

### Original Request
Fix two runtime bugs observed on STM32F469-DISCO hardware before continuing the upstream merge work plan.

### Bug Analysis Summary (Updated After Task 1 Diagnostics)

**Bug 1: "~20% Offset / Wrapping" — ROOT CAUSE CONFIRMED: DSI↔LTDC Color Coding Mismatch**
- ✅ **Touch coordinates are correct** — Task 1 diagonal swipe proved: raw=(0,799)→(474,2), mapping 1:1 to screen pixels
- ✅ ST BSP `TS_SWAP_NONE` confirmed correct for portrait 480×800
- ❌ **The visual offset is caused by DSI color coding mismatch**, NOT touch coordinates
- `board.rs:224-225` sets `ColorCoding::TwentyFourBits` (3 bytes/pixel on DSI link)
- But LTDC is configured with `PixelFormat::RGB565` (2 bytes/pixel)
- In Video Burst mode, DSI reads 3 bytes per pixel where LTDC provides 2 → 2/3 ratio shift → wrapping
- **FIX**: Change both `color_coding_host` and `color_coding_wrapper` to `ColorCoding::SixteenBitsConfig1`

**Bug 2: "Display Freeze/Crash" — ROOT CAUSE CONFIRMED: ft6x06 Crate Panic**
- ✅ After ~200+ touch samples, ft6x06 crate panics at `lib.rs:332`:
  `assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8`
- `detect_touch()` reads garbage I2C value for ntouch > 2 → assertion panic → HardFault
- **THIS is the "display freeze" bug**, not a DSI issue
- **FIX**: Add pre-read guard on register 0x02 (TD_STATUS) before calling `detect_touch()`

**Additional Finding: memory.x 384K RAM is Wrong**
- STM32F469NIH6 has 320K contiguous SRAM, not 384K
- Setting 384K places `_stack_start` at non-existent memory → immediate HardFault on deep stacks
- Currently uncommitted — must be fixed to 320K (grouped with Task 2)

**Remaining speculative stability causes (lower priority, Tasks 3/5/6/7):**
1. `force_rx_low_power(true)` never cleared after panel init (very likely)
2. DSI error interrupts disabled, error flags never cleared (very likely)
3. LTDC IMR reload races with active scan (likely)
4. DMA2D completion not awaited (moderate)
- These remain in the plan as hardening tasks but are NOT confirmed root causes
### Interview Summary
**Key Decisions**:
- Fix bugs BEFORE continuing upstream merge
- RGB565 (u16) is the working pixel format — ARGB8888 causes HardFault
- Remote board at `ubuntu@192.168.13.246` for testing
- Build: `cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`
- Flash: `scp` to remote + `probe-rs run --connect-under-reset --chip STM32F469NIHx`

### Metis Review
**Key Findings**:
- ST BSP uses `TS_SWAP_NONE` for portrait mode — **CONFIRMED correct by Task 1 diagnostics**. No axis swap needed.
- I2C timeout is a HAL-level change affecting all users. Use example-level wrapper instead.
- Apply and verify fixes one at a time, not all at once.
- `force_rx_low_power(false)` must be added BEFORE `AllInHighSpeed` switch.
- LTDC VBR should only be used for buffer address swap, keep IMR for initial config.
- Add diagnostic defmt logging at each fix point.

---

## Work Objectives

### Core Objective
Make the F469-DISCO paint demo reliably touchable with correct coordinates and stable display that doesn't freeze, using empirical hardware verification for each fix.

### Concrete Deliverables
- `examples/f469disco-paint.rs` with correct touch coordinate mapping
- `examples/f469disco/board.rs` with `force_rx_low_power(false)` fix
- Example-level I2C timeout wrapper (not HAL change)
- Diagnostic defmt logging for touch and DSI stability
- Verified stable 5+ minute continuous operation on hardware

### Definition of Done
- [ ] Touch all 4 corners of display → defmt shows coordinates near (0,0), (479,0), (0,799), (479,799) respectively
- [ ] Draw across full screen area → lines appear where finger touches (no offset)
- [ ] Tap each of 8 palette colors → correct color selected
- [ ] Run continuous touch drawing for 5+ minutes → no freeze, no corruption, no black screen
- [ ] `cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"` → success
- [ ] `cargo build --release --examples --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"` → success (no regressions)

### Must Have
- Empirical touch coordinate data from hardware BEFORE applying any swap/offset
- `force_rx_low_power(false)` after panel init in board.rs (both paths)
- Example-level I2C timeout (not HAL i2c.rs change)
- defmt diagnostic logging at each fix point
- Each fix verified on hardware individually

### Must NOT Have (Guardrails)
- **No changes to `src/i2c.rs`** — use example-level wrapper instead (HAL change impacts all users)
- **No blind axis swap** — must be based on empirical corner-touch data
- **No ARGB8888 changes** — stay on RGB565 (ARGB8888 causes HardFault)
- **No `Peripherals::steal()` refactoring** — low priority, high risk
- **No framebuffer pointer model changes** — low priority
- **No double-buffering implementation** — out of scope
- **No touch gesture support** — only coordinate accuracy
- **No DSI interrupt handler implementation** — only diagnostic flag reading
- **No batch-applying all fixes** — one at a time with hardware verification

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed via SSH+defmt.
> Acceptance criteria requiring "user manually tests on screen" are FORBIDDEN.

### Test Decision
- **Infrastructure exists**: NO (embedded no_std — no test framework on target)
- **Automated tests**: None (hardware verification only)
- **Framework**: N/A

### QA Policy
Every task uses SSH+probe-rs remote flash with defmt output analysis.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Hardware verification**: SSH to ubuntu@192.168.13.246, SCP binary, probe-rs run, capture defmt output
- **Build verification**: Local cargo build with all required features
- **Stability verification**: Timed soak test with defmt timestamp analysis

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 0 (Serial — Diagnostic Baseline):
├── Task 0: Verify probe-rs connectivity [quick]
├── Task 1: Touch diagnostic flash — raw coordinate logging [quick]
└── Task 2: Apply touch coordinate fix based on empirical data [quick]

Wave 1 (Serial — Primary Stability Fix):
└── Task 3: force_rx_low_power(false) after panel init [quick]

Wave 2 (Parallel — Secondary Stability Fixes):
├── Task 4: Example-level I2C timeout wrapper [deep]
├── Task 5: LTDC VBR reload for buffer swap [quick]
├── Task 6: DSI error flag diagnostic logging [quick]
└── Task 7: DMA2D completion wait in draw_rectangle [quick]

Wave 3 (Serial — Final Verification):
└── Task 8: Full soak test — 5+ minute continuous operation [unspecified-high]

Wave FINAL (Parallel — 4 reviewers):
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Hardware QA — full test on physical board [unspecified-high]
└── Task F4: Scope fidelity check [deep]

Critical Path: Task 0 → Task 1 → Task 2 → Task 3 → Task 4-7 → Task 8 → F1-F4
Parallel Speedup: ~40% faster than fully sequential
Max Concurrent: 4 (Wave 2)
```

### Dependency Matrix

| Task | Blocked By | Blocks |
|------|-----------|--------|
| 0 | — | 1, 2, 3, 4, 5, 6, 7 |
| 1 | 0 | 2 |
| 2 | 1 | 8 |
| 3 | 0 | 8 |
| 4 | 0 | 8 |
| 5 | 0 | 8 |
| 6 | 0 | 8 |
| 7 | 0 | 8 |
| 8 | 2, 3, 4, 5, 6, 7 | F1-F4 |
| F1-F4 | 8 | — |

### Agent Dispatch Summary

- **Wave 0**: 3 tasks — T0 → `quick`, T1 → `quick`, T2 → `quick`
- **Wave 1**: 1 task — T3 → `quick`
- **Wave 2**: 4 tasks — T4 → `deep`, T5 → `quick`, T6 → `quick`, T7 → `quick`
- **Wave 3**: 1 task — T8 → `unspecified-high`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 0. Verify Probe-RS Connectivity to Remote Board

  **What to do**:
  - SSH into `ubuntu@192.168.13.246` and run `probe-rs list` to confirm the STM32F469 debug probe is visible
  - Run `probe-rs info --chip STM32F469NIHx` to confirm chip communication
  - If probe-rs is not installed or probe not found, troubleshoot before proceeding

  **Must NOT do**:
  - Do not flash anything yet — connectivity check only

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single SSH command to verify connectivity
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `playwright`: No browser needed for SSH commands

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 0 — Serial diagnostic
  - **Blocks**: All subsequent tasks (1-8)
  - **Blocked By**: None (first task)

  **References**:

  **Pattern References**:
  - `tools/f469disco-display-build.sh` — Contains the SCP + probe-rs flash command pattern used in this project

  **External References**:
  - probe-rs docs: `https://probe.rs/docs/getting-started/probe-setup/`

  **WHY Each Reference Matters**:
  - The build script shows the exact SSH/SCP/probe-rs command sequence that works for this project

  **Acceptance Criteria**:
  - [ ] `ssh ubuntu@192.168.13.246 "probe-rs list"` → shows at least one connected probe
  - [ ] `ssh ubuntu@192.168.13.246 "probe-rs info --chip STM32F469NIHx"` → shows chip info without error

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Probe-RS lists connected debug probe
    Tool: Bash (SSH)
    Preconditions: SSH access to ubuntu@192.168.13.246 with key auth
    Steps:
      1. Run: ssh ubuntu@192.168.13.246 "probe-rs list"
      2. Assert output contains probe identifier (e.g., "STLink" or "CMSIS-DAP")
    Expected Result: At least one probe listed, no connection errors
    Failure Indicators: "No probes found", SSH connection refused, timeout
    Evidence: .sisyphus/evidence/task-0-probe-connectivity.txt
  ```

  **Commit**: NO

---

- [x] 1. Touch Diagnostic Flash — Raw Coordinate Logging

  **What to do**:
  - Temporarily add enhanced defmt logging to `examples/f469disco-paint.rs` that prints raw FT6X06 coordinates with explicit corner labels
  - The logging should print: `"TOUCH DIAG: raw_x={} raw_y={} (FT6X06 registers)"` on every touch event
  - Also print the current TOUCH_X_OFFSET and TOUCH_Y_OFFSET values and the computed screen coordinates
  - Build, flash to hardware via SSH+SCP, and capture defmt output
  - Instruct the tester (or use instructions for manual test): touch the 4 corners of the display in order: top-left, top-right, bottom-left, bottom-right, then touch each palette bar
  - **Analyze the defmt output** to determine:
    - Whether axes need swapping (does raw_x range match WIDTH=480 or HEIGHT=800?)
    - Whether any axis needs inversion
    - What offset values are needed
  - **Record findings** in `.sisyphus/evidence/task-1-touch-diagnostic.txt`
  - **CRITICAL**: Do NOT apply fixes yet — this task is diagnostic only. Revert any temporary logging changes after capturing data.

  **Must NOT do**:
  - Do NOT apply axis swap or offset changes in this task
  - Do NOT modify any file other than f469disco-paint.rs (and revert those changes)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small temporary code change + SSH flash + log analysis
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 0 — Serial diagnostic (after Task 0)
  - **Blocks**: Task 2 (touch fix depends on diagnostic data)
  - **Blocked By**: Task 0 (needs probe connectivity)

  **References**:

  **Pattern References**:
  - `examples/f469disco-paint.rs:192-197` — Current touch handling code with existing defmt logging at line 197
  - `examples/f469disco-paint.rs:22-23` — TOUCH_X_OFFSET and TOUCH_Y_OFFSET constants
  - `examples/f469disco-paint.rs:16-17` — WIDTH=480, HEIGHT=800 display constants

  **API/Type References**:
  - FT6X06 `get_touch()` returns `TouchPoint { x: u16, y: u16 }` — raw 12-bit values from registers 0x03-0x06
  - FT6X06 constants: `FT6X06_MAX_X_LENGTH=800`, `FT6X06_MAX_Y_LENGTH=480`

  **External References**:
  - ST BSP `stm32469i_discovery_ts.c` uses `TS_SWAP_NONE` for portrait mode — suggests NO swap needed for 480×800
  - ST BSP GitHub: `https://github.com/STMicroelectronics/32f469idiscovery-bsp`
  - FT6X06 crate `display_touch.rs` example swaps X/Y with comment "The coordinates are flipped" — but may target different orientation

  **WHY Each Reference Matters**:
  - paint.rs:192-197 shows where touch coords are consumed and what logging already exists
  - FT6X06 constants reveal expected raw value ranges (x up to 800, y up to 480)
  - ST BSP is ground truth for correct axis handling on this exact board+orientation

  **Acceptance Criteria**:
  - [ ] Enhanced defmt logging added (temporarily) to paint example
  - [ ] Binary flashed to hardware and defmt output captured
  - [ ] Raw coordinate data from touching 4 corners recorded in evidence file
  - [ ] Analysis written: axis swap needed? inversion needed? offset values?
  - [ ] Temporary logging changes reverted after data capture

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Capture raw touch coordinates from all 4 display corners
    Tool: Bash (SSH + probe-rs)
    Preconditions: Task 0 passed. Enhanced logging build ready.
    Steps:
      1. Build: cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"
      2. SCP: scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/
      3. Flash + capture: ssh ubuntu@192.168.13.246 "timeout 60 probe-rs run --connect-under-reset --chip STM32F469NIHx /tmp/f469disco-paint" | tee .sisyphus/evidence/task-1-touch-diagnostic.txt
      4. Wait for "Panel initialized" in defmt output
      5. Touch top-left corner of display → observe raw_x, raw_y values
      6. Touch top-right corner → observe values
      7. Touch bottom-left corner → observe values
      8. Touch bottom-right corner → observe values
    Expected Result: Evidence file contains raw coordinate data for all 4 corners
    Failure Indicators: No defmt output, probe-rs connection failure, no touch events logged
    Evidence: .sisyphus/evidence/task-1-touch-diagnostic.txt

  Scenario: Determine axis mapping from diagnostic data
    Tool: Manual analysis of evidence file
    Preconditions: task-1-touch-diagnostic.txt contains raw corner data
    Steps:
      1. Read evidence file
      2. For top-left corner: check if raw_x≈0 and raw_y≈0 (no swap) or raw_x≈0 and raw_y≈479 (swap+invert) etc.
      3. For bottom-right corner: check if raw_x≈479 and raw_y≈799 (no swap) or reversed
      4. Determine: swap needed? inversion needed? offset values?
    Expected Result: Clear determination of axis mapping written as analysis comment in evidence file
    Failure Indicators: Ambiguous data, coordinates not near expected ranges
    Evidence: .sisyphus/evidence/task-1-touch-analysis.txt
  ```

  **Commit**: NO (diagnostic only, changes reverted)

---

- [x] 2. Fix DSI Color Coding Mismatch + memory.x RAM Size

  **What to do**:
  - **ROOT CAUSE IDENTIFIED**: Task 1 diagnostics confirmed raw touch coordinates are correct (0,799)→(474,2) maps 1:1 to pixels. The visual "~20% offset + wrap" bug is caused by a **DSI ↔ LTDC pixel format mismatch**.
  - **The mismatch**: In `examples/f469disco/board.rs` line 224-225, the DSI host is configured with `ColorCoding::TwentyFourBits` (3 bytes/pixel on DSI link), but the LTDC layer is configured with `PixelFormat::RGB565` (2 bytes/pixel in framebuffer). The DSI wrapper reads 3 bytes per pixel from the LTDC output but LTDC only provides 2 bytes, creating a 2/3 ratio shift that wraps around the screen.
  - **Fix**: Change DSI color coding to match the LTDC pixel format. When using RGB565 framebuffer, set:
    ```rust
    color_coding_host: ColorCoding::SixteenBitsConfig1,
    color_coding_wrapper: ColorCoding::SixteenBitsConfig1,
    ```
  - `SixteenBitsConfig1` = 0b000 in DSI_LCOLCR/DSI_WCFGR.COLMUX, which tells the DSI wrapper to expect 16-bit (RGB565) pixels from LTDC — matching exactly.
  - **IMPORTANT**: Check whether a loosely-coupled (adapted-command mode) conversion applies. ST's BSP uses `LCD_DSI_PIXEL_DATA_FMT_RBG888` on the DSI side even with RGB565 LTDC because the DSI hardware can convert. However, our code uses **Video Burst mode** (line 216-218: `DsiMode::Video { mode: DsiVideoMode::Burst }`), and in video mode the DSI typically passes pixels through without conversion. The `SixteenBitsConfig1` approach is the correct fix for video mode.
  - **Verification**: After fixing, a diagonal bottom-left→top-right swipe should draw a line from bottom-left to top-right on screen with NO offset and NO wrapping.
  - **Also fix memory.x**: Change RAM from 384K (wrong — causes HardFault) to 320K. The STM32F469NIH6 has SRAM1(160K)+SRAM2(16K)+SRAM3(64K)+SRAM4(64K)=304K contiguous from 0x20000000, plus backup SRAM. Conservative value: 320K per ST datasheet DS11189 table 2. Comment: `/* STM32F469NIH6: 2 MiB Flash, 320 KiB SRAM */`.
  - Build, flash, verify: (a) display shows correct colors with no shift, (b) touch drawing is pixel-accurate, (c) no HardFault from memory.x.

  **Must NOT do**:
  - Do NOT change the LTDC pixel format (keep RGB565)
  - Do NOT change the framebuffer indexing at line 211 — `y * WIDTH + x` is correct
  - Do NOT try ARGB8888 — it causes HardFault on this board
  - Do NOT modify the touch coordinate handling — it was confirmed correct by Task 1
  - Do NOT set RAM to 384K — only 320K or less is valid

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: DSI ↔ LTDC color coding interaction is subtle; agent must understand the video mode pixel pipeline and verify the fix doesn't break panel init
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 0 — Serial (after Task 1)
  - **Blocks**: Task 8 (soak test)
  - **Blocked By**: Task 1 (needed diagnostic data; now complete)

  **References**:

  **Pattern References**:
  - `examples/f469disco/board.rs:224-225` — **THE BUG**: `color_coding_host: ColorCoding::TwentyFourBits` and `color_coding_wrapper: ColorCoding::TwentyFourBits` — must change both to `SixteenBitsConfig1`
  - `examples/f469disco/board.rs:216-218` — `DsiMode::Video { mode: DsiVideoMode::Burst }` — confirms video mode, so DSI does NOT convert pixel format
  - `src/dsi.rs:132-138` — `ColorCoding` enum: `SixteenBitsConfig1 = 0b000`, `TwentyFourBits = 0b101` — the values that get written to DSI registers
  - `src/dsi.rs:409` — Where `color_coding_host` is written to DSI_LCOLCR register
  - `src/dsi.rs:414` — Where `color_coding_wrapper` is written to DSI_WCFGR.COLMUX
  - `src/ltdc.rs:507-512` — LTDC CFBLR register: sets pitch = `width * 2` (bytes) for RGB565 — this is correct and must NOT change
  - `examples/f469disco-paint.rs:211` — Framebuffer indexing `buf[py * WIDTH + px]` — this is correct for WIDTH=480 stride
  - `memory.x` — Currently `LENGTH = 384K` (line 5) — must change to `320K`

  **Evidence References (from Task 1)**:
  - `.sisyphus/evidence/task-1-touch-analysis.txt` — Proves touch coords are correct; bug is in display pipeline
  - `.sisyphus/evidence/task-1-swipe-diagonal.txt` — Raw diagonal swipe data: (0,799)→(474,2)

  **External References**:
  - ST BSP `stm32469i_discovery_lcd.c` — Uses `LCD_DSI_PIXEL_DATA_FMT_RBG888` but in adapted-command mode (NOT video burst mode)
  - STM32F4 Reference Manual RM0386, DSI chapter — LCOLCR and WCFGR register definitions
  - ST Datasheet DS11189, Table 2 — STM32F469NIH6 memory map: SRAM sizes
  - Oracle session `ses_3604a445bffeVPckPp6n19GzNA` — Analyzed reference working code vs our broken code; confirmed DSI color coding is the fix
  - Oracle session `ses_36057924effevarzFeNKBDDBcf` — Validated DSI color coding mismatch as root cause of 1/3 display shift

  **WHY Each Reference Matters**:
  - board.rs:224-225 is THE bug — the 2 lines that must change
  - dsi.rs:132-138 shows the correct enum variant to use and its register value
  - dsi.rs:409,414 show exactly which registers get programmed — confirms both host and wrapper must match
  - ltdc.rs:507-512 proves the LTDC side is correct (pitch = 960 = 480*2) and must not change
  - Task 1 evidence proves the touch coords are innocent — the display pipeline is the culprit
  - Oracle sessions provide the analytical chain that identified this root cause

  **Acceptance Criteria**:
  - [ ] `color_coding_host` changed from `TwentyFourBits` to `SixteenBitsConfig1` in board.rs
  - [ ] `color_coding_wrapper` changed from `TwentyFourBits` to `SixteenBitsConfig1` in board.rs
  - [ ] `memory.x` RAM changed from `384K` to `320K` with corrected comment
  - [ ] Build succeeds: `cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`
  - [ ] Flashed to hardware: display shows correct colors with no horizontal shift or wrapping
  - [ ] Diagonal swipe draws a line from bottom-left to top-right with no offset
  - [ ] Palette taps select correct colors (leftmost bar = first color)

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Display shows correctly without 1/3 shift or wrapping
    Tool: Bash (SSH + probe-rs)
    Preconditions: DSI color coding fixed to SixteenBitsConfig1. memory.x fixed to 320K.
    Steps:
      1. Build: cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"
      2. SCP to remote and flash with probe-rs
      3. Observe display initialization via defmt — palette bars should be visible and correctly colored
      4. Draw a diagonal line from bottom-left corner to top-right corner
      5. Assert defmt shows touch raw=(~0, ~799) at start and raw=(~479, ~0) at end
      6. Assert the drawn line appears where finger touches — no horizontal offset, no wrapping
    Expected Result: Line starts at bottom-left, ends at top-right, follows finger exactly
    Failure Indicators: Line shifted right, line wraps to other side of screen, colors wrong/corrupted, display blank
    Evidence: .sisyphus/evidence/task-2-dsi-color-fix-verified.txt

  Scenario: memory.x doesn't cause HardFault
    Tool: Bash (SSH + probe-rs)
    Preconditions: memory.x set to RAM=320K.
    Steps:
      1. Build and flash paint example
      2. Assert defmt output appears within 5 seconds (no immediate HardFault)
      3. Assert "Initializing SDRAM" message appears (init sequence progresses)
    Expected Result: Board boots normally, init messages appear in defmt
    Failure Indicators: No defmt output (HardFault on boot), probe-rs shows crash at _stack_start
    Evidence: .sisyphus/evidence/task-2-memory-x-boot.txt

  Scenario: Palette color selection works correctly with new DSI color coding
    Tool: Bash (SSH + probe-rs)
    Preconditions: DSI color fix applied and flashed.
    Steps:
      1. Tap the leftmost palette bar (top of screen, left side)
      2. Assert defmt shows y < PALETTE_H (48) and x near 0-59
      3. Draw a stroke — should be black (first palette color)
      4. Tap the rightmost palette bar (top of screen, right side)
      5. Assert defmt shows y < PALETTE_H and x near 420-479
      6. Draw a stroke — should be white (last palette color)
    Expected Result: Each palette region maps to correct color index (0-7), colors display correctly
    Failure Indicators: Wrong palette index, colors appear shifted/wrong on display
    Evidence: .sisyphus/evidence/task-2-palette-verified.txt
  ```

  **Evidence to Capture**:
  - [ ] task-2-dsi-color-fix-verified.txt — Diagonal swipe defmt output showing correct pixel-accurate drawing
  - [ ] task-2-memory-x-boot.txt — Boot log proving no HardFault
  - [ ] task-2-palette-verified.txt — Palette tap log proving correct color selection

  **Commit**: YES (two commits)
  - Commit A: `fix(f469disco): fix DSI color coding mismatch causing 1/3 display shift`
    - Files: `examples/f469disco/board.rs`
    - Pre-commit: `cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`
  - Commit B: `fix: correct memory.x RAM size for STM32F469NIH6 (320K, not 384K)`
    - Files: `memory.x`


- [x] 3. Clear `force_rx_low_power` After Panel Init in board.rs

  **What to do**:
  - In `examples/f469disco/board.rs`, function `init_panel()` (line ~299): add `dsi_host.force_rx_low_power(false);` BEFORE the `set_command_mode_transmission_kind(AllInHighSpeed)` call
  - In `examples/f469disco/board.rs`, function `init_display_full()` (line ~405): same fix — add `dsi_host.force_rx_low_power(false);` BEFORE `AllInHighSpeed` switch
  - Also check `examples/f469disco-lcd-test.rs` if it has the same pattern — fix there too
  - The issue: `force_rx_low_power(true)` is set at lines 273 and 379 before panel init, but never cleared. The DSI WPCR1.FLPRXLPM bit remains permanently set, which forces LP receive even after switching to HS mode. This can cause the DSI link to become unresponsive over time.
  - Add a defmt log after clearing: `defmt::info!("force_rx_low_power cleared — DSI HS mode fully active");`
  - Build, flash, verify display still initializes correctly AND stays alive for 2+ minutes

  **Must NOT do**:
  - Do NOT change `src/dsi.rs` — only modify example/board code
  - Do NOT remove the `force_rx_low_power(true)` before panel init — it's needed during LP command phase
  - Do NOT change the init order — only ADD the clearing call

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 2-3 line addition in board.rs at known locations
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 — Primary stability fix (after touch fix)
  - **Blocks**: Task 8 (soak test)
  - **Blocked By**: Task 0 (needs probe connectivity)

  **References**:

  **Pattern References**:
  - `examples/f469disco/board.rs:272-273` — `force_rx_low_power(true)` set in `init_panel()`
  - `examples/f469disco/board.rs:299` — `set_command_mode_transmission_kind(AllInHighSpeed)` — add clearing BEFORE this line
  - `examples/f469disco/board.rs:378-379` — `force_rx_low_power(true)` set in `init_display_full()`
  - `examples/f469disco/board.rs:405` — Same `AllInHighSpeed` switch in `init_display_full()` — add clearing BEFORE this
  - `src/dsi.rs:479-481` — Implementation of `force_rx_low_power()` — writes WPCR1.FLPRXLPM bit

  **External References**:
  - ST AN4860 DSI Application Note — documents FLPRXLPM bit behavior and when to clear
  - ST BSP `stm32469i_discovery_lcd.c` — check if ST clears FLPRXLPM after init (ground truth)

  **WHY Each Reference Matters**:
  - board.rs:273,299 are the exact locations where the bug exists — LP power forced on but never cleared
  - dsi.rs:479-481 confirms the API exists and what register bit it touches (WPCR1.FLPRXLPM)
  - ST AN4860 explains the intended usage pattern for this bit

  **Acceptance Criteria**:
  - [ ] `force_rx_low_power(false)` added before `AllInHighSpeed` in `init_panel()`
  - [ ] `force_rx_low_power(false)` added before `AllInHighSpeed` in `init_display_full()`
  - [ ] lcd-test.rs fixed if same pattern exists
  - [ ] Build succeeds for all examples
  - [ ] Display still initializes correctly on hardware
  - [ ] Display stays alive for 2+ minutes of normal use

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Display initializes correctly with force_rx_low_power fix
    Tool: Bash (SSH + probe-rs)
    Preconditions: Fix applied to board.rs (both functions).
    Steps:
      1. Build: cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"
      2. SCP and flash to hardware
      3. Wait for defmt output
      4. Assert: "force_rx_low_power cleared" message appears in defmt log
      5. Assert: "Panel initialized, DSI in high-speed mode" appears after clearing
      6. Assert: Display shows paint UI (palette bar visible)
    Expected Result: Display initializes normally, new log message confirms FLPRXLPM cleared
    Failure Indicators: Display blank/corrupt, no "cleared" message, init hangs
    Evidence: .sisyphus/evidence/task-3-flprxlpm-fix.txt

  Scenario: Display remains stable after force_rx_low_power fix
    Tool: Bash (SSH + probe-rs)
    Preconditions: Fix applied and flashed.
    Steps:
      1. Flash paint example
      2. Let run for 2 minutes while periodically touching screen
      3. Monitor defmt output for errors or silence (silence = hang)
    Expected Result: Continuous defmt touch output for 2+ minutes, no gaps > 10s
    Failure Indicators: defmt output stops (hang), display corruption, touch stops responding
    Evidence: .sisyphus/evidence/task-3-stability-2min.txt
  ```

  **Commit**: YES
  - Message: `fix(f469disco): clear force_rx_low_power after panel init`
  - Files: `examples/f469disco/board.rs`, optionally `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo build --release --examples --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`

---

- [x] 4. Add ft6x06 Panic Guard + Example-Level I2C Timeout Wrapper

  **What to do**:
  - **PRIMARY FIX — ft6x06 ntouch panic guard**: Task 1 confirmed the "display freeze" is actually a panic in the ft6x06 crate at `lib.rs:332`: `assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8`. After ~200 touch samples, `detect_touch()` reads a garbage I2C value for number of touches (ntouch > 2), hits the assertion, and panics → HardFault → board appears frozen.
  - **Guard implementation**: Before calling `touch.detect_touch(&mut i2c)`, read the raw TD_STATUS register (0x02) via I2C to validate ntouch. If the value > 2 (FT6X06_MAX_NB_TOUCH), skip the frame and log a warning instead of letting the crate panic.
  - Alternatively, wrap the `detect_touch` + `get_touch` calls in a more defensive pattern:
    ```rust
    // Instead of: let num = touch.detect_touch(&mut i2c)?;
    // Use a guard that catches invalid ntouch before the crate asserts:
    let num = match touch.detect_touch(&mut i2c) {
        Ok(n) if n <= 2 => n,
        Ok(n) => {
            defmt::warn!("ft6x06: invalid ntouch={}, skipping", n);
            continue;
        }
        Err(_) => { continue; }
    };
    ```
  - **NOTE**: The ft6x06 crate's `detect_touch` may panic internally before returning. If so, we need to read register 0x02 directly via `i2c.write_read(0x38, &[0x02], &mut buf)` and validate BEFORE calling the crate function.
  - **SECONDARY FIX — I2C timeout / watchdog**: The I2C bus can also get stuck due to unbounded busy-waits in `src/i2c.rs` (which we MUST NOT modify). Add an IWDG (Independent Watchdog) with ~2 second timeout as a last-resort recovery. If the I2C hangs, the watchdog resets the board.
  - **Do NOT modify `src/i2c.rs`** — this is shared HAL code affecting all users
  - Consider: SysTick-based software timeout around I2C calls (option A) or simple IWDG watchdog (option B, simpler and more robust). IWDG is preferred because it catches ANY hang, not just I2C.

  **Must NOT do**:
  - **MUST NOT modify `src/i2c.rs`** — this constraint is non-negotiable
  - Do NOT modify the ft6x06 crate source (external dependency)
  - Do NOT add complex retry logic — simple guard + skip is sufficient
  - Do NOT use DMA for I2C (out of scope)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Must understand ft6x06 register-level behavior, I2C failure modes, and IWDG; the panic guard needs careful placement to run before the crate's internal assertion
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7)
  - **Blocks**: Task 8 (soak test)
  - **Blocked By**: Task 0 (needs probe connectivity)

  **References**:

  **Pattern References**:
  - `examples/f469disco-paint.rs:182-188` — Current touch detect loop: `touch.detect_touch(&mut i2c)` called with `.ok()` but the PANIC happens INSIDE detect_touch before it can return Err
  - `examples/f469disco-paint.rs:192` — `touch.get_touch(&mut i2c, 1)` — also calls into crate internals that may panic
  - `src/i2c.rs:256-264` — `wait_for_flag` function with unbounded `while` loop (reference only — shows why IWDG is needed)
  - `src/i2c.rs:387-418` — Read operation with 6+ sequential busy-waits (reference only)

  **API/Type References**:
  - FT6X06 register 0x02 (TD_STATUS) — number of touch points, bits 3:0. Valid range: 0-2. Values > 2 indicate I2C bus error / garbage read.
  - `ft6x06::Ft6X06::detect_touch()` — reads register 0x02, asserts ntouch <= 2, PANICS if violated
  - `stm32f4xx_hal::watchdog::IndependentWatchdog` — IWDG peripheral for last-resort recovery

  **Evidence References**:
  - `.sisyphus/evidence/task-1-swipe-diagonal.txt` — Contains the crash log: `assertion failed: ntouch <= FT6X06_MAX_NB_TOUCH as u8`
  - `.sisyphus/notepads/f469-bug-fixes/learnings.md:16-21` — Documents the ft6x06 panic as THE crash cause

  **External References**:
  - FT6X06 datasheet — TD_STATUS register (0x02) description
  - ft6x06 crate source `lib.rs:332` — The assertion that panics: `assert!(ntouch <= FT6X06_MAX_NB_TOUCH as u8)`
  - STM32F4 Reference Manual, IWDG chapter — Watchdog configuration

  **WHY Each Reference Matters**:
  - paint.rs:182-188 is the exact code being guarded — the panic happens inside detect_touch
  - FT6X06 register 0x02 is the raw source of the garbage value — reading it directly lets us validate before the crate asserts
  - Task 1 evidence proves this is a real crash (not theoretical) — happened after ~200 touches

  **Acceptance Criteria**:
  - [ ] `src/i2c.rs` is UNCHANGED (verify with `git diff src/i2c.rs` → empty)
  - [ ] ft6x06 ntouch garbage values no longer cause panic (guarded before assertion)
  - [ ] defmt warning logged when garbage ntouch detected (not panic)
  - [ ] IWDG watchdog initialized as last-resort hang recovery
  - [ ] System continues running after garbage ntouch (touch frame skipped, not halted)
  - [ ] Build succeeds for all examples

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Paint demo runs for 200+ touches without panic
    Tool: Bash (SSH + probe-rs)
    Preconditions: ft6x06 panic guard + IWDG added to paint.rs.
    Steps:
      1. Build and flash paint example
      2. Draw continuously for 2-3 minutes (should generate 200+ touch events)
      3. Monitor defmt output — count touch events
      4. Assert: no panic, no HardFault, no reset
      5. If any "ft6x06: invalid ntouch" warnings appear, count them
    Expected Result: 200+ touch events processed, system still running. Occasional ntouch warnings acceptable (< 5%).
    Failure Indicators: Panic message, defmt output stops, board resets unexpectedly
    Evidence: .sisyphus/evidence/task-4-panic-guard-stress.txt

  Scenario: Normal touch operation unaffected by guard
    Tool: Bash (SSH + probe-rs)
    Preconditions: Guard added to paint.rs.
    Steps:
      1. Build and flash paint example
      2. Touch screen normally for 30 seconds
      3. Monitor defmt output — should show normal touch coordinates
      4. Assert: No spurious warnings during light normal use
    Expected Result: Normal touch operation, zero or near-zero warnings
    Failure Indicators: Spurious warnings during normal use, touch not working
    Evidence: .sisyphus/evidence/task-4-normal-touch.txt

  Scenario: Verify src/i2c.rs was not modified
    Tool: Bash (git)
    Preconditions: All changes applied.
    Steps:
      1. Run: git diff src/i2c.rs
      2. Assert: Empty output (no changes)
    Expected Result: src/i2c.rs is identical to committed version
    Failure Indicators: Any diff output
    Evidence: .sisyphus/evidence/task-4-i2c-unchanged.txt
  ```

  **Commit**: YES
  - Message: `fix(f469disco): add ft6x06 panic guard and IWDG watchdog for touch stability`
  - Files: `examples/f469disco-paint.rs`
  - Pre-commit: `cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`
---

- [x] 5. LTDC: Add VBR Reload for Buffer Address Swap

  **What to do**:
  - In `src/ltdc.rs`, add a new public method `reload_on_vblank(&mut self)` alongside the existing `reload()` method
  - The existing `reload()` at lines 590-595 uses IMR (Immediate Reload) — keep it as-is for initial configuration
  - The new `reload_on_vblank()` should set the VBR (Vertical Blanking Reload) bit instead of IMR
  - In `examples/f469disco-paint.rs` and any example that swaps framebuffer addresses mid-operation, change `reload()` calls to `reload_on_vblank()`
  - IMR during active scan can race with the LTDC scanline, causing tearing or corruption
  - VBR waits for the vertical blanking period, ensuring atomic reload
  - **Keep IMR for initial setup** (before first frame is displayed) and **use VBR for runtime changes**

  **Must NOT do**:
  - Do NOT remove the existing `reload()` method — it's needed for initial config
  - Do NOT change all reload sites globally — only runtime buffer swaps
  - Do NOT implement double-buffering (out of scope)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Add one new method mirroring existing `reload()` but with VBR bit
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 6, 7)
  - **Blocks**: Task 8 (soak test)
  - **Blocked By**: Task 0 (needs probe connectivity for verification)

  **References**:

  **Pattern References**:
  - `src/ltdc.rs:590-595` — Existing `reload()` method using IMR: `self.ltdc.srcr.modify(|_, w| w.imr().set_bit())`
  - `src/ltdc.rs:677-679` — `draw_rectangle` calls `self.reload()` after DMA2D operation
  - `src/ltdc.rs:345,351,414,417` — Other reload sites in initial configuration (keep IMR)

  **API/Type References**:
  - STM32F4 LTDC SRCR register: IMR (bit 0) = immediate reload, VBR (bit 1) = vertical blanking reload

  **External References**:
  - STM32F4 Reference Manual, LTDC chapter — SRCR register description
  - ST HAL `stm32f4xx_hal_ltdc.c` — uses both `LTDC_RELOAD_IMMEDIATE` and `LTDC_RELOAD_VERTICAL_BLANKING`
  - ST community thread on diagonal tearing: `https://community.st.com/t5/stm32-mcus-touchgfx-and-gui/diagonal-tearing-on-lcd-dsi-videomode-doublebuffering-on/td-p/669397`

  **WHY Each Reference Matters**:
  - ltdc.rs:590-595 is the existing reload method to mirror for VBR
  - ltdc.rs:677-679 is the draw_rectangle that should use VBR to avoid tearing during active scan
  - ST HAL shows both reload types exist for a reason — different use cases

  **Acceptance Criteria**:
  - [ ] New `reload_on_vblank()` public method added to ltdc.rs
  - [ ] `draw_rectangle` uses `reload_on_vblank()` instead of `reload()`
  - [ ] Existing `reload()` method unchanged
  - [ ] Build succeeds for all examples and all MCU targets
  - [ ] No visible tearing when drawing rapidly on hardware

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: VBR reload method exists and compiles
    Tool: Bash
    Preconditions: reload_on_vblank() added to ltdc.rs.
    Steps:
      1. Run: cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"
      2. Assert: Build succeeds
      3. Run: grep -n "reload_on_vblank" src/ltdc.rs
      4. Assert: Method definition found
    Expected Result: Method exists and builds without error
    Failure Indicators: Compilation error, method not found
    Evidence: .sisyphus/evidence/task-5-vbr-build.txt

  Scenario: Rapid drawing shows no corruption
    Tool: Bash (SSH + probe-rs)
    Preconditions: VBR fix applied and flashed.
    Steps:
      1. Flash paint example
      2. Draw rapid strokes across screen for 30 seconds
      3. Monitor defmt for any error messages
    Expected Result: Smooth drawing, no visual artifacts in defmt logs, no freeze
    Failure Indicators: defmt errors, drawing stops, display corrupts
    Evidence: .sisyphus/evidence/task-5-vbr-stability.txt
  ```

  **Commit**: YES (grouped with Task 7)
  - Message: `fix(ltdc): add VBR reload and DMA2D completion wait`
  - Files: `src/ltdc.rs`
  - Pre-commit: `cargo build --release --examples --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`

---

- [x] 6. DSI Error Flag Diagnostic Logging

  **What to do**:
  - In `examples/f469disco/board.rs`, change `DsiInterrupts::None` (line ~223) to enable error interrupts or at minimum, enable error flag accumulation
  - In the paint example's main loop (`examples/f469disco-paint.rs`), add periodic DSI error register reads:
    - Read ISR0 and ISR1 registers from the DSI peripheral every ~100 loop iterations
    - If any error flags are set, log them via defmt: `defmt::warn!("DSI errors: ISR0={:#x} ISR1={:#x}", isr0, isr1);`
    - Clear the flags after reading to prevent accumulation
  - This is diagnostic — we're adding visibility into whether DSI protocol errors occur during operation
  - The DSI error flags can reveal: ACK errors, PHY errors, ECC errors, CRC errors, packet size errors
  - If DSI errors accumulate without clearing, the DSI link may become unresponsive

  **Must NOT do**:
  - Do NOT implement a full DSI interrupt handler (ISR)
  - Do NOT add complex error recovery logic — just log and clear
  - Do NOT modify `src/dsi.rs` API — use raw register access if needed

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Add register reads and defmt logging in main loop
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 7)
  - **Blocks**: Task 8 (soak test)
  - **Blocked By**: Task 0 (needs probe connectivity for verification)

  **References**:

  **Pattern References**:
  - `examples/f469disco/board.rs:223` — `DsiInterrupts::None` hardcoded — where to change interrupt config
  - `src/dsi.rs:249-252` — DSI interrupt enable implementation — shows what interrupt flags are available
  - `examples/f469disco-paint.rs:170-210` — Main loop where periodic DSI reads should be added

  **API/Type References**:
  - STM32F4 DSI_ISR0/DSI_ISR1 registers — error flag locations
  - `stm32f4::stm32f469::DSI` register block — for raw register access

  **External References**:
  - ST AN4860: DSI error flag descriptions and clearing procedure
  - STM32F4 Reference Manual, DSI chapter — ISR0/ISR1 register bit definitions

  **WHY Each Reference Matters**:
  - board.rs:223 is where interrupts are currently disabled — the root of the visibility problem
  - dsi.rs:249-252 shows the API for enabling interrupts (may provide simpler path than raw registers)
  - The main loop is where periodic reads belong — low overhead, high diagnostic value

  **Acceptance Criteria**:
  - [ ] DSI error flags read periodically in paint main loop
  - [ ] defmt warning logged if any error flags set
  - [ ] Error flags cleared after reading
  - [ ] Build succeeds for all examples
  - [ ] Normal operation shows zero or very few DSI errors in defmt

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: DSI error diagnostic logging compiles and runs
    Tool: Bash (SSH + probe-rs)
    Preconditions: DSI error reads added to paint.rs main loop.
    Steps:
      1. Build and flash paint example
      2. Run for 60 seconds with light touch interaction
      3. Monitor defmt output for DSI error warnings
    Expected Result: Either zero DSI errors (healthy link) or error messages revealing specific flags
    Failure Indicators: Build failure, runtime crash from register access
    Evidence: .sisyphus/evidence/task-6-dsi-errors.txt
  ```

  **Commit**: YES
  - Message: `fix(f469disco): add DSI error diagnostic logging`
  - Files: `examples/f469disco-paint.rs`, `examples/f469disco/board.rs`
  - Pre-commit: `cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`

---

- [x] 7. DMA2D Completion Wait in `draw_rectangle`

  **What to do**:
  - In `src/ltdc.rs`, function `draw_rectangle()` (line ~622-674), add a DMA2D completion wait after starting the transfer
  - Currently (line ~673): DMA2D transfer is started with `cr.modify(|_, w| w.start().set_bit())` and the function returns immediately
  - Add a busy-wait after start: `while self.dma2d.cr.read().start().bit_is_set() {}`
  - This ensures the DMA2D transfer completes before the CPU can modify the framebuffer again
  - Without this wait, the next draw operation could race with an in-progress DMA2D transfer, corrupting the framebuffer
  - The DMA2D transfer for a typical rectangle is very fast (microseconds), so the wait is negligible

  **Must NOT do**:
  - Do NOT add DMA2D interrupt handling (out of scope)
  - Do NOT change the DMA2D configuration or mode
  - Do NOT modify any other function in ltdc.rs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: One-line addition after DMA2D start
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 6)
  - **Blocks**: Task 8 (soak test)
  - **Blocked By**: Task 0 (needs probe connectivity for verification)

  **References**:

  **Pattern References**:
  - `src/ltdc.rs:670-674` — DMA2D start in `draw_rectangle()`: `cr.modify(|_, w| w.start().set_bit())` followed by immediate return
  - `src/ltdc.rs:622` — Start of `draw_rectangle` function for full context

  **External References**:
  - STM32F4 Reference Manual, DMA2D chapter — CR.START bit is self-clearing on completion
  - ST HAL `stm32f4xx_hal_dma2d.c` — `HAL_DMA2D_Start()` followed by `HAL_DMA2D_PollForTransfer()` pattern

  **WHY Each Reference Matters**:
  - ltdc.rs:670-674 is the exact code being modified — shows the missing completion wait
  - ST HAL confirms the start-then-poll pattern is standard for DMA2D

  **Acceptance Criteria**:
  - [ ] DMA2D completion wait added after `start().set_bit()` in `draw_rectangle()`
  - [ ] Build succeeds for all examples and MCU targets
  - [ ] Rapid drawing on hardware shows no framebuffer corruption

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: DMA2D completion wait doesn't regress drawing performance
    Tool: Bash (SSH + probe-rs)
    Preconditions: DMA2D wait added to ltdc.rs draw_rectangle.
    Steps:
      1. Build and flash paint example
      2. Draw rapid strokes for 30 seconds
      3. Monitor defmt output — touch events should still arrive at normal rate
    Expected Result: Drawing feels responsive, no visible lag, no corruption
    Failure Indicators: Visible lag in drawing, touch events queuing up, timeout warnings
    Evidence: .sisyphus/evidence/task-7-dma2d-wait.txt
  ```

  **Commit**: YES (grouped with Task 5)
  - Message: `fix(ltdc): add VBR reload and DMA2D completion wait`
  - Files: `src/ltdc.rs`
  - Pre-commit: `cargo build --release --examples --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`

---

- [x] 8. Full Soak Test — 5+ Minute Continuous Operation

  **What to do**:
  - This is a verification-only task — no code changes
  - Flash the final build (with ALL fixes from Tasks 2-7) to hardware
  - Run continuous touch drawing for 5+ minutes
  - Capture full defmt log output to evidence file
  - Analyze defmt log for:
    - Touch coordinate accuracy (no drift, no wrapping)
    - I2C timeout warnings (count, frequency)
    - DSI error flags (count, types)
    - Continuous operation (no gaps > 10s in defmt output = no hang)
  - If any issues found, document them for targeted follow-up

  **Must NOT do**:
  - Do NOT make code changes in this task
  - Do NOT declare success if ANY hang occurs (even if recovered by watchdog)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Requires patience, careful log analysis, and judgment about stability
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 — Serial (after all fixes)
  - **Blocks**: Final verification wave (F1-F4)
  - **Blocked By**: Tasks 2, 3, 4, 5, 6, 7 (all fixes must be applied)

  **References**:

  **Pattern References**:
  - `.sisyphus/evidence/task-2-touch-fix-verified.txt` — Touch accuracy baseline from Task 2
  - `.sisyphus/evidence/task-3-stability-2min.txt` — 2-minute stability baseline from Task 3
  - `.sisyphus/evidence/task-6-dsi-errors.txt` — DSI error baseline from Task 6

  **WHY Each Reference Matters**:
  - Previous evidence files provide comparison baselines — the soak test should show equal or better metrics

  **Acceptance Criteria**:
  - [ ] Paint example runs for 5+ minutes without any hang
  - [ ] Touch coordinates remain accurate throughout (no drift)
  - [ ] I2C timeout warnings < 5 in 5 minutes (infrequent, not systemic)
  - [ ] DSI error flags either zero or low-frequency and non-escalating
  - [ ] defmt output continuous — no gaps > 10 seconds
  - [ ] All evidence files saved

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: 5-minute continuous operation soak test
    Tool: Bash (SSH + probe-rs)
    Preconditions: All fixes from Tasks 2-7 applied. Final build flashed.
    Steps:
      1. Build: cargo build --release --example f469disco-paint --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"
      2. SCP to remote: scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/
      3. Flash and capture: ssh ubuntu@192.168.13.246 "timeout 360 probe-rs run --connect-under-reset --chip STM32F469NIHx /tmp/f469disco-paint" | tee .sisyphus/evidence/task-8-soak-test.txt
      4. During the 5-6 minute run: touch screen periodically, draw strokes, tap palette
      5. After timeout or 5+ minutes, analyze log:
         - Count touch events: grep -c "touch:" task-8-soak-test.txt (should be high)
         - Count I2C timeouts: grep -c "I2C.*timeout" task-8-soak-test.txt (should be < 5)
         - Count DSI errors: grep -c "DSI error" task-8-soak-test.txt (should be low)
         - Check for gaps: analyze timestamps for any > 10s gaps
    Expected Result: 5+ minutes of continuous operation with touch events, < 5 I2C timeouts, no escalating DSI errors, no hangs
    Failure Indicators: defmt output stops before 5 minutes, > 10 I2C timeouts, DSI errors escalating, any gap > 10s
    Evidence: .sisyphus/evidence/task-8-soak-test.txt

  Scenario: Touch accuracy maintained throughout soak test
    Tool: Analysis of evidence file
    Preconditions: task-8-soak-test.txt captured.
    Steps:
      1. Extract first 5 touch coordinates and last 5 touch coordinates from log
      2. Compare ranges — coordinates should use similar screen ranges
      3. No coordinates should be stuck at 0 or max (indicates hang/error)
    Expected Result: Touch coordinates consistent throughout, full screen range used
    Failure Indicators: Coordinates drift, get stuck, or show different patterns early vs late
    Evidence: .sisyphus/evidence/task-8-touch-stability-analysis.txt
  ```

  **Commit**: NO (memory.x already committed with Task 2; soak test is verification only)
  - Note: This commit captures the previously-uncommitted memory.x fix along with the soak test verification

---
## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, check defmt evidence). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo build --release --examples --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"`. Review all changed files for: `as any`, empty catches, `unwrap()` in main loop, unused imports. Check for AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Hardware QA — Full Test on Physical Board** — `unspecified-high` (+ no extra skills needed, uses SSH)
  Flash final build to hardware. Verify: touch all 4 corners → correct coordinates in defmt. Draw lines across full screen → no offset. Tap palette → correct colors. Run 5+ minutes → no freeze. Capture all defmt output as evidence.
  Output: `Touch [PASS/FAIL] | Palette [PASS/FAIL] | Stability [N minutes] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git diff). Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check `src/i2c.rs` was NOT modified. Check no ARGB8888 changes. Check no `Peripherals::steal()` changes.
  Output: `Tasks [N/N compliant] | Forbidden Changes [CLEAN/N issues] | VERDICT`

---

## Commit Strategy

After all fixes verified:
- **Commit 1**: `fix(f469disco): correct DSI color coding for RGB565 + fix memory.x RAM size` — examples/f469disco/board.rs, memory.x
- **Commit 2**: `fix(f469disco): clear force_rx_low_power after panel init` — examples/f469disco/board.rs
- **Commit 3**: `fix(f469disco): add ft6x06 panic guard + IWDG watchdog` — examples/f469disco-paint.rs
- **Commit 4**: `fix(ltdc): use VBR reload for buffer swap` — src/ltdc.rs
- **Commit 5**: `chore(f469disco): add DSI error diagnostic logging` — examples/f469disco-paint.rs
- **Commit 6**: `fix(f469disco): add DMA2D completion wait in draw_rectangle` — examples/f469disco-paint.rs or examples/f469disco/board.rs

---

## Success Criteria

### Verification Commands
```bash
# Build all examples
cargo build --release --examples --features "stm32f469,stm32-fmc,framebuffer,dsihost,defmt"
# Expected: Finished release [optimized] target(s)

# Flash and run paint demo
scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/
ssh ubuntu@192.168.13.246 "probe-rs run --connect-under-reset --chip STM32F469NIHx /tmp/f469disco-paint"
# Expected: Panel initialized, touch coordinates correct, no freeze after 5+ minutes
```

### Final Checklist
- [x] DSI color coding set to `SixteenBitsConfig1` — display pixels align 1:1 with touch (no offset/wrap)
- [x] memory.x RAM size corrected to 320K (no HardFault on deep stacks)
- [x] ft6x06 panic guard prevents ntouch assertion crash (no "display freeze")
- [x] Display stable for 5+ minutes of continuous touch drawing
- [x] Touch coordinates map correctly across full 480×800 screen
- [x] All "Must Have" items present
- [x] All "Must NOT Have" items absent (especially no src/i2c.rs changes)
- [x] All examples build without errors or new warnings
- [x] All evidence files saved in `.sisyphus/evidence/`
