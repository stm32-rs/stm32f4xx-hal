# Paint Touch Logging + Flash Verification

## TL;DR

> **Quick Summary**: Add defmt touch coordinate logging to the f469disco-paint example, rebuild with defmt, flash to hardware via `probe-rs run`, and verify defmt RTT log streaming works.
> 
> **Deliverables**:
> - Touch x,y coordinate logging in `f469disco-paint.rs` (both raw and adjusted values)
> - Working defmt log output via `probe-rs run` (replacing broken `probe-rs attach`)
> - Hardware-verified paint demo with live touch debugging
> 
> **Estimated Effort**: Quick (< 15 minutes)
> **Parallel Execution**: NO — strictly sequential (edit → build → deploy)
> **Critical Path**: Task 1 → Task 2 → Task 3

---

## Context

### Original Request
User wants to flash the paint demo and add defmt logging for touch x,y coordinates so they can see exactly what's happening during touch interaction.

### Interview Summary
**Key Discussions**:
- Touch coordinates exist at lines 193-194 but are never logged
- Previous `probe-rs attach` approach failed with "no version symbol" error
- `probe-rs run` should work as it flashes AND runs with full ELF context

**Research Findings**:
- All existing defmt calls in paint.rs use `#[cfg(feature = "defmt")]` guard pattern (lines 121-122, 146-147, 161-162, 170-171, 178-179)
- `#![deny(warnings)]` on line 13 makes the cfg guard MANDATORY
- defmt imports already present (lines 18-19, 22-23)
- Build features confirmed: `stm32f469,stm32-fmc,dsihost,framebuffer,defmt`
- Remote board reachable at `ubuntu@192.168.13.246`, probe-rs 0.31.0
- `DEFMT_LOG = "info"` set in `.cargo/config.toml`

### Metis Review
**Identified Gaps** (addressed):
- Log volume concern (100+ lines/sec during drawing) → Acceptable for debugging, note for future
- `point.x`/`point.y` types → Confirmed numeric primitives via `as i32` casts on L193-194
- SSH TTY allocation needed for Ctrl+C propagation → Use `ssh -t` flag
- Doc comment on line 10 slightly stale (missing `dsihost`) → Out of scope, don't fix

---

## Work Objectives

### Core Objective
Add real-time touch coordinate logging via defmt so the user can see raw and adjusted x,y values during paint interaction, and get the defmt RTT log pipeline working via `probe-rs run`.

### Concrete Deliverables
- 2 new lines in `examples/f469disco-paint.rs` (cfg guard + defmt::info!)
- Flashed binary on remote STM32F469I-DISCO board
- Working defmt RTT log stream showing "Paint ready" message

### Definition of Done
- [x] Build succeeds with defmt feature: exit code 0
- [x] `probe-rs run` shows defmt log output including "Paint ready" message
- [x] Touch coordinate logs appear when screen is touched (inherent hardware verification)

### Must Have
- `#[cfg(feature = "defmt")]` guard on the logging line (mandatory due to `#![deny(warnings)]`)
- Both raw (`point.x`, `point.y`) AND adjusted (`x`, `y`) coordinates in the log message
- Use `probe-rs run` (NOT `probe-rs attach`) for defmt output

### Must NOT Have (Guardrails)
- DO NOT add any new `use` imports — defmt is already accessible
- DO NOT modify any lines other than the 2-line insertion after line 194
- DO NOT change `.cargo/config.toml`, `Cargo.toml`, or any other file
- DO NOT add extra logging (palette changes, brush size, FPS, frame counter, etc.)
- DO NOT "fix" or adjust TOUCH_X_OFFSET / TOUCH_Y_OFFSET calibration values
- DO NOT modify `f469disco-touch-debug.rs` (explicit user constraint)
- DO NOT change the doc comment on line 10 even though it's slightly stale

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.
> The only exception: physical screen touch to trigger coordinate logs (inherent hardware limitation).

### Test Decision
- **Infrastructure exists**: YES (defmt + probe-rs)
- **Automated tests**: None (embedded hardware example, no unit test framework)
- **Framework**: N/A
- **Verification method**: Build success + defmt RTT log stream confirmation

### QA Policy
Every task includes agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Build verification**: Bash — compile, check exit code
- **Deploy verification**: Bash (SSH/SCP) — transfer binary, flash, observe defmt output
- **defmt pipeline**: Bash (SSH) — `probe-rs run` output captures "Paint ready" message

---

## Execution Strategy

### Sequential Execution (No Parallelism Possible)

```
Wave 1 (Start Immediately):
└── Task 1: Add defmt touch logging (2 lines) [quick]

Wave 2 (After Wave 1):
└── Task 2: Build + verify binary [quick]

Wave 3 (After Wave 2):
└── Task 3: Deploy + verify defmt RTT stream [quick]

Critical Path: Task 1 → Task 2 → Task 3
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1    | —         | 2      | 1    |
| 2    | 1         | 3      | 2    |
| 3    | 2         | —      | 3    |

### Agent Dispatch Summary

- **Wave 1**: 1 task — T1 → `quick`
- **Wave 2**: 1 task — T2 → `quick`
- **Wave 3**: 1 task — T3 → `quick`

---

## TODOs

- [x] 1. Add defmt touch coordinate logging to paint example

  **What to do**:
  - Open `examples/f469disco-paint.rs`
  - After line 194 (`let y = (point.y as i32 + TOUCH_Y_OFFSET).clamp(0, HEIGHT as i32 - 1);`), insert a blank line followed by exactly 2 lines:
    ```rust
                    #[cfg(feature = "defmt")]
                    defmt::info!("touch: raw=({}, {}) adj=({}, {})", point.x, point.y, x, y);
    ```
  - The indentation must match the surrounding code (20 spaces — 5 levels of 4-space indent)
  - Follow the exact same 2-line pattern used at lines 121-122, 146-147, 161-162, 170-171, 178-179 in the same file

  **Must NOT do**:
  - DO NOT add any `use` imports — defmt is already accessible via existing imports on lines 18-19
  - DO NOT modify any other lines in the file
  - DO NOT remove or change the `#![deny(warnings)]` on line 13
  - DO NOT add additional logging (palette changes, brush info, FPS, etc.)
  - DO NOT touch calibration offsets (TOUCH_X_OFFSET, TOUCH_Y_OFFSET)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
    - Trivial 2-line insertion, no special domain skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (solo)
  - **Blocks**: Task 2
  - **Blocked By**: None

  **References**:

  **Pattern References** (existing code to follow):
  - `examples/f469disco-paint.rs:121-122` — Exact 2-line `#[cfg(feature = "defmt")]` + `defmt::info!(...)` pattern to copy
  - `examples/f469disco-paint.rs:178-179` — Most recent defmt log before the insertion point ("Paint ready" message)
  - `examples/f469disco-paint.rs:193-194` — The lines AFTER which to insert (raw `point.x`/`point.y` and adjusted `x`/`y` calculations)

  **API/Type References**:
  - `examples/f469disco-paint.rs:192` — `touch.get_touch(&mut i2c, 1)` returns a `Point` with `x` and `y` fields (numeric primitives cast to `i32` on L193-194)

  **Acceptance Criteria**:
  - [ ] `grep -n 'defmt::info.*touch.*raw' examples/f469disco-paint.rs` shows the new line
  - [ ] `grep -B1 'defmt::info.*touch.*raw' examples/f469disco-paint.rs` shows `#[cfg(feature = "defmt")]` on the preceding line
  - [ ] No other lines in the file were modified (verify with `git diff examples/f469disco-paint.rs` — should show only 3 insertions: blank line + cfg + defmt::info)

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Defmt touch logging line inserted correctly
    Tool: Bash (grep)
    Preconditions: File has been edited
    Steps:
      1. Run: grep -n 'defmt::info.*touch.*raw' examples/f469disco-paint.rs
      2. Run: grep -B1 'defmt::info.*touch.*raw' examples/f469disco-paint.rs
      3. Run: git diff --stat examples/f469disco-paint.rs
    Expected Result:
      - Step 1 returns exactly 1 line containing: defmt::info!("touch: raw=({}, {}) adj=({}, {})", point.x, point.y, x, y);
      - Step 2 shows #[cfg(feature = "defmt")] on the line before
      - Step 3 shows 1 file changed, ~3 insertions(+)
    Failure Indicators: grep returns no matches; more than 3 lines changed; missing cfg guard
    Evidence: .sisyphus/evidence/task-1-defmt-logging-inserted.txt

  Scenario: Build succeeds with AND without defmt feature
    Tool: Bash
    Preconditions: Touch logging lines inserted
    Steps:
      1. Run: cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf 2>&1; echo "EXIT:$?"
      2. Run: cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer" --target thumbv7em-none-eabihf 2>&1; echo "EXIT:$?"
    Expected Result: Both builds exit with EXIT:0
    Failure Indicators: Compilation error mentioning defmt, unused import, or unresolved symbol
    Evidence: .sisyphus/evidence/task-1-build-both-features.txt
  ```

  **Commit**: YES
  - Message: `feat(examples): add defmt touch coordinate logging to paint example`
  - Files: `examples/f469disco-paint.rs`
  - Pre-commit: `cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf`

- [x] 2. Build release binary with defmt and verify

  **What to do**:
  - Run the full build command: `cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf`
  - Verify exit code is 0
  - Verify the binary exists at `target/thumbv7em-none-eabihf/release/examples/f469disco-paint`
  - NOTE: This may already be done as part of Task 1's QA. If Task 1 already built successfully, just confirm the binary exists and skip the redundant build.

  **Must NOT do**:
  - DO NOT change any build features or target
  - DO NOT modify Cargo.toml or .cargo/config.toml

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (solo)
  - **Blocks**: Task 3
  - **Blocked By**: Task 1

  **References**:
  - `examples/f469disco-paint.rs:10` — Doc comment showing build command
  - `.cargo/config.toml` — Contains `DEFMT_LOG = "info"` and linker flags including `-Tdefmt.x`

  **Acceptance Criteria**:
  - [ ] Build exits with code 0
  - [ ] Binary exists: `ls -la target/thumbv7em-none-eabihf/release/examples/f469disco-paint` succeeds

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Release binary builds successfully with defmt
    Tool: Bash
    Preconditions: Task 1 edits committed
    Steps:
      1. Run: cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf 2>&1; echo "EXIT:$?"
      2. Run: ls -la target/thumbv7em-none-eabihf/release/examples/f469disco-paint
    Expected Result: EXIT:0; file exists with non-zero size
    Failure Indicators: Non-zero exit code; file not found
    Evidence: .sisyphus/evidence/task-2-build-success.txt
  ```

  **Commit**: NO (no file changes — build only)

- [x] 3. Deploy to remote board and verify defmt RTT log stream

  **What to do**:
  - SCP the binary to the remote board:
    ```bash
    scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/f469disco-paint
    ```
  - Flash and run with `probe-rs run` (NOT `probe-rs attach` which has the "no version symbol" bug):
    ```bash
    ssh -t -o ConnectTimeout=10 ubuntu@192.168.13.246 '. ~/.cargo/env && DEFMT_LOG=info probe-rs run --chip STM32F469NIHx /tmp/f469disco-paint'
    ```
  - Watch for defmt output — should see initialization messages:
    - "Initializing SDRAM..."
    - "Configuring LTDC and initializing display..."
    - "Initializing I2C touch controller..."
    - "FT6X06 touch initialized"
    - "Paint ready — touch to draw, tap palette bar to change color"
  - Touch coordinate logs (`touch: raw=(...) adj=(...)`) will appear when the screen is physically touched — this cannot be automated but the defmt pipeline is confirmed working if "Paint ready" appears
  - Use `timeout 30` or Ctrl+C to stop after confirming logs work
  - If `probe-rs run` hangs or shows no output, try:
    - `DEFMT_LOG=trace` instead of `info`
    - Check if binary has defmt sections: `readelf -S /tmp/f469disco-paint | grep defmt`
    - Try `probe-rs run --protocol swd --chip STM32F469NIHx /tmp/f469disco-paint`

  **Must NOT do**:
  - DO NOT use `probe-rs attach` (broken with "no version symbol" error)
  - DO NOT change the chip identifier (must be `STM32F469NIHx`)
  - DO NOT modify any files on the remote board beyond `/tmp/f469disco-paint`

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (solo)
  - **Blocks**: None (final task)
  - **Blocked By**: Task 2

  **References**:
  - Remote board: `ubuntu@192.168.13.246`, chip `STM32F469NIHx`, probe-rs 0.31.0
  - SSH auth: key-based, use `-o BatchMode=yes` for non-interactive, `-t` for TTY allocation
  - `.cargo/config.toml` — `DEFMT_LOG = "info"` (but set explicitly on remote command too)

  **Acceptance Criteria**:
  - [ ] SCP succeeds (exit code 0)
  - [ ] `probe-rs run` starts and shows RTT initialization
  - [ ] At least the "Paint ready" defmt message appears in output (confirms full pipeline)

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Binary deployed and defmt RTT stream working
    Tool: Bash (SSH + SCP)
    Preconditions: Binary built successfully in Task 2
    Steps:
      1. Run: scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/f469disco-paint; echo "SCP_EXIT:$?"
      2. Run: ssh -o ConnectTimeout=10 ubuntu@192.168.13.246 '. ~/.cargo/env && timeout 20 probe-rs run --chip STM32F469NIHx /tmp/f469disco-paint 2>&1; echo "RUN_EXIT:$?"'
    Expected Result:
      - SCP_EXIT:0
      - Output contains "Paint ready" (or at minimum "Initializing SDRAM")
      - RUN_EXIT may be 124 (timeout) which is fine — it means probe-rs was running and streaming
    Failure Indicators:
      - SCP_EXIT non-zero (network issue)
      - No defmt output at all ("no version symbol" error reappearing)
      - probe-rs crashes or refuses to connect
    Evidence: .sisyphus/evidence/task-3-defmt-rtt-output.txt

  Scenario: Fallback — verify defmt symbols in binary if RTT fails
    Tool: Bash
    Preconditions: probe-rs run showed no defmt output
    Steps:
      1. Run: ssh ubuntu@192.168.13.246 'readelf -S /tmp/f469disco-paint 2>/dev/null | grep -i defmt || echo NO_DEFMT_SECTIONS'
      2. Run: ssh ubuntu@192.168.13.246 '. ~/.cargo/env && DEFMT_LOG=trace probe-rs run --chip STM32F469NIHx /tmp/f469disco-paint 2>&1 | head -20'
    Expected Result: defmt sections present in ELF; DEFMT_LOG=trace may show more output
    Failure Indicators: NO_DEFMT_SECTIONS → binary was built without defmt feature
    Evidence: .sisyphus/evidence/task-3-defmt-fallback.txt
  ```

  **Commit**: NO (deployment only, no code changes)

---

## Final Verification Wave

> Since this is a trivial 3-task sequential plan, the final QA is embedded in Task 3.
> The "Paint ready" defmt log message confirms the entire pipeline works end-to-end.

---

## Commit Strategy

- **Task 1**: `feat(examples): add defmt touch coordinate logging to paint example` — `examples/f469disco-paint.rs`
  - Pre-commit: `cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf`

---

## Success Criteria

### Verification Commands
```bash
# Build succeeds
cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,dsihost,framebuffer,defmt" --target thumbv7em-none-eabihf
# Expected: exit code 0

# Deploy and verify defmt
scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/
ssh -t ubuntu@192.168.13.246 'DEFMT_LOG=info . ~/.cargo/env && probe-rs run --chip STM32F469NIHx /tmp/f469disco-paint'
# Expected: defmt output including "Paint ready" message
```

### Final Checklist
- [x] 2-line defmt logging insertion present in paint.rs
- [x] `#[cfg(feature = "defmt")]` guard on the logging line
- [x] Build succeeds with defmt feature
- [x] `probe-rs run` shows defmt RTT output
- [x] "Paint ready" message visible in logs
