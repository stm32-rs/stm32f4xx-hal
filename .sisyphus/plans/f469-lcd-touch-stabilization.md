# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# F469 DISCO LCD + Touch Stabilization

## TL;DR

> **Quick Summary**: Rewrite the touch state machine in `f469disco-lcd-test.rs` to use a simple 2-state (Idle/Pressed) model with time-only debounce, matching the proven `display-touch.rs` pattern. Display init is already working — touch is the only broken part.
> 
> **Deliverables**:
> - Rewritten touch loop that toggles reliably on every tap
> - Clean diagnostic logging (coordinates, mode changes, health)
> - Autodetection of NT35510 vs OTM8009A preserved
> - Remote flash + RTT evidence of working touch
> 
> **Estimated Effort**: Quick (single focused session)
> **Parallel Execution**: NO — sequential (single file, iterative test cycle)
> **Critical Path**: Rewrite touch → Build → Flash → Verify → Commit

---

## Context

### Original Request
Get STM32F469I-DISCO display AND touch working reliably on B08 (NT35510) boards.

### Current State
- **Display**: WORKS — color/BER test patterns visible
- **Touch**: BROKEN — first toggle works, then permanently stuck
- **Root cause**: Complex 9-variable touch state machine with coordinate-based blocking creates logic lockout (confirmed by Oracle analysis)
- **Infrastructure**: Remote flash pipeline via CubeProgrammer to `ubuntu@192.168.13.246` works

### Research Findings
1. **display-touch.rs** (same repo, known working): Uses ZERO state tracking — just polls `detect_touch` → `get_touch` → acts. No debounce, no edge detection, no movement tracking.
2. **Oracle analysis**: "Most likely this is a logic lockout. `moved_since_last_toggle` effectively disallows toggling when next press is near same spot. Simplify to 2 states: Idle and Pressed."
3. **FT6x06 timing**: Max report rate 80-100Hz. Current 10ms poll (100Hz) is at the limit. Should use ≥15ms.
4. **FT6x06 reset**: `display-touch.rs` uses `long_hard_reset()` from ft6x06 crate. Current code does NOT — potential initialization issue.
5. **F7 disco screen example**: Display-only, no touch. Uses clean DisplayController + DrawTarget pattern.

---

## Work Objectives

### Core Objective
Replace the broken 9-variable touch state machine with a simple 2-state model that toggles display mode on every tap, reliably.

### Concrete Deliverables
- `examples/f469disco-lcd-test.rs` — rewritten touch loop
- `examples/f469disco/nt35510.rs` — minor cleanup only (init works)
- `notes.md` — updated with results
- Git commit of working state

### Definition of Done
- [ ] `cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"` passes with zero warnings
- [ ] Display shows color test pattern on boot
- [ ] Touch toggles between color/BER on EVERY tap (not just first)
- [ ] Tapping same spot repeatedly works (no coordinate blocking)
- [ ] RTT logs show touch coordinates and mode transitions

### Must Have
- Simple Idle/Pressed touch state machine
- Time-only debounce (no coordinate checks)
- NT35510/OTM8009A autodetection preserved
- defmt logging of touch events and mode changes
- Poll interval ≥15ms

### Must NOT Have (Guardrails)
- NO coordinate-based toggle blocking (`moved_since_last_toggle` REMOVED)
- NO `last_toggle_point` variable
- NO `stuck_touch_ticks` / forced release logic
- NO `TOUCH_REARM_MOVE_PX` constant
- NO more than 3 touch state variables total
- NO timer-based auto-switching (user-driven only)

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** for build/flash. Visual touch verification requires user.

### Test Decision
- **Infrastructure exists**: YES (remote flash + RTT)
- **Automated tests**: None (embedded hardware)
- **Framework**: N/A

### QA Policy
- Build verification: `cargo build` must pass
- Flash verification: CubeProgrammer reports success
- Runtime verification: RTT logs + user visual confirmation

---

## TODOs

- [ ] 1. Rewrite touch state machine in f469disco-lcd-test.rs

  **What to do**:
  - Remove ALL coordinate-based touch state variables: `last_touch_point`, `last_toggle_point`, `stuck_touch_ticks`, `TOUCH_REARM_MOVE_PX`, `TOUCH_STUCK_WARN_TICKS`, `TOUCH_STUCK_FORCE_RELEASE_TICKS`, `TOUCH_COORD_LOG_INTERVAL_TICKS`
  - Replace with simple 2-state model:
    ```rust
    // State: just two variables
    let mut touch_active = false;       // Is finger currently on screen?
    let mut debounce_remaining = 0u8;   // Time-only cooldown after toggle
    
    // Constants
    const DEBOUNCE_TICKS: u8 = 15;     // ~300ms at 20ms poll = prevents double-tap
    const POLL_INTERVAL_MS: u32 = 20;  // Stay below 100Hz FT6x06 limit
    ```
  - Toggle logic:
    ```rust
    if num > 0 && !touch_active && debounce_remaining == 0 {
        // Rising edge: finger just touched
        toggle_display_mode();
        debounce_remaining = DEBOUNCE_TICKS;
        touch_active = true;
    } else if num > 0 {
        touch_active = true;  // Still touching
    } else {
        touch_active = false; // Released
    }
    if debounce_remaining > 0 { debounce_remaining -= 1; }
    ```
  - Keep touch coordinate logging (throttled, for diagnostics)
  - Keep error retry logic for detect_touch/get_touch (3 retries with 500µs delay)
  - Change `delay.delay_ms(10u32)` to `delay.delay_ms(20u32)`
  - Keep board hint detection and touch address selection
  - Keep LCD controller autodetection unchanged

  **Must NOT do**:
  - Do NOT change NT35510 init sequence (it works)
  - Do NOT change DSI/LTDC configuration (it works)
  - Do NOT add coordinate-based conditions to toggle logic
  - Do NOT add more than 3 touch state variables

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires careful state machine redesign with understanding of embedded timing
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Tasks 2, 3, 4
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `examples/display-touch.rs:173-210` — Known working touch polling pattern (ZERO state, just poll and act)
  - `examples/f469disco-lcd-test.rs:326-499` — Current broken touch loop to replace
  - `examples/f469disco-lcd-test.rs:63-69` — Constants to remove

  **API/Type References**:
  - `ft6x06::Ft6X06::detect_touch(&mut i2c) -> Result<u8, E>` — Returns touch count
  - `ft6x06::Ft6X06::get_touch(&mut i2c, 1) -> Result<TouchState, E>` — Returns x, y, weight, misc

  **External References**:
  - Oracle recommendation: "Simplify to 2 states: Idle and Pressed. Toggle ONLY on Idle→Pressed transition. Return to Idle ONLY when num==0. Keep debounce as time-only."
  - FT6x06 datasheet: max 100Hz report rate → poll ≥15ms

  **Acceptance Criteria**:
  - [ ] `cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"` passes with zero example warnings
  - [ ] No `last_toggle_point`, `stuck_touch_ticks`, `TOUCH_REARM_MOVE_PX` in code
  - [ ] Touch state uses ≤3 variables
  - [ ] Poll interval is 20ms

  **QA Scenarios**:

  ```
  Scenario: Build passes cleanly
    Tool: Bash
    Steps:
      1. Run: cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"
      2. Check exit code is 0
      3. Check no warnings from example file (lib warnings OK)
    Expected Result: Build succeeds, no example-level warnings
    Evidence: .sisyphus/evidence/task-1-build.txt

  Scenario: No banned state variables remain
    Tool: Bash (grep)
    Steps:
      1. grep -n "last_toggle_point\|stuck_touch_ticks\|TOUCH_REARM_MOVE_PX\|TOUCH_STUCK\|moved_since_last_toggle\|TOUCH_COORD_LOG_INTERVAL" examples/f469disco-lcd-test.rs
      2. Verify zero matches
    Expected Result: No matches found
    Evidence: .sisyphus/evidence/task-1-grep-banned.txt
  ```

  **Commit**: YES
  - Message: `fix(display): simplify touch state machine to 2-state model`
  - Files: `examples/f469disco-lcd-test.rs`
  - Pre-commit: `cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"`

---

- [ ] 2. Flash and verify touch works on hardware

  **What to do**:
  - Copy built ELF to remote host via scp
  - Flash via CubeProgrammer
  - Ask user to test: tap repeatedly, confirm toggles work every time
  - If RTT capture works (probe-rs), capture logs

  **Must NOT do**:
  - Do NOT modify code in this task
  - Do NOT skip user verification

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocked By**: Task 1

  **References**:
  - Flash command: `ssh ubuntu@192.168.13.246 "/home/ubuntu/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI -c port=SWD mode=UR -d /tmp/f469disco-lcd-test.elf -v -rst"`

  **Acceptance Criteria**:
  - [ ] CubeProgrammer reports "Download verified successfully"
  - [ ] User confirms display shows color test pattern
  - [ ] User confirms touch toggles work repeatedly (not just once)

  **QA Scenarios**:

  ```
  Scenario: Flash succeeds
    Tool: Bash (ssh)
    Steps:
      1. scp firmware to remote /tmp/
      2. Run CubeProgrammer flash command
      3. Check output contains "Download verified successfully"
    Expected Result: Flash and verify pass
    Evidence: .sisyphus/evidence/task-2-flash.txt

  Scenario: Touch works repeatedly
    Tool: User visual verification
    Steps:
      1. Ask user to tap screen 5+ times
      2. Each tap should toggle between color and BER patterns
      3. Tapping same spot should work
    Expected Result: All taps toggle correctly
    Evidence: User confirmation in chat
  ```

  **Commit**: NO (verification only)

---

- [ ] 3. Update notes.md with results

  **What to do**:
  - Update "Current Status Summary" section
  - Mark touch as WORKING (if verified)
  - Document the fix: removed coordinate-based blocking, simplified to 2-state
  - Update "Next Session Checklist" to reflect completed items

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Blocked By**: Task 2

  **Acceptance Criteria**:
  - [ ] notes.md reflects current working state

  **Commit**: YES (group with task 1 if not yet committed)
  - Message: `docs: update notes.md with touch fix results`
  - Files: `notes.md`

---

## Final Verification Wave

- [ ] F1. **Build Verification** — `quick`
  Run `cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"`. Verify exit code 0 and no example warnings.
  Output: `Build [PASS/FAIL] | Warnings [0/N] | VERDICT`

- [ ] F2. **Code Quality Check** — `quick`
  Grep for banned patterns (last_toggle_point, stuck_touch_ticks, TOUCH_REARM_MOVE_PX, moved_since_last_toggle). Count touch state variables. Verify ≤3.
  Output: `Banned patterns [0] | State vars [N] | VERDICT`

---

## Commit Strategy

- **Task 1**: `fix(display): simplify touch state machine to 2-state model` — examples/f469disco-lcd-test.rs
- **Task 3**: `docs: update notes.md with touch fix results` — notes.md
- (Can be combined into single commit if preferred)

---

## Success Criteria

### Verification Commands
```bash
cargo build --release --example f469disco-lcd-test --features="stm32f469,defmt"  # Expected: exit 0, no example warnings
grep -c "last_toggle_point\|stuck_touch_ticks\|TOUCH_REARM_MOVE_PX" examples/f469disco-lcd-test.rs  # Expected: 0
```

### Final Checklist
- [ ] Display shows color test pattern on boot
- [ ] Touch toggles on EVERY tap (not just first)
- [ ] Same-spot tapping works
- [ ] No coordinate-based toggle blocking in code
- [ ] Poll interval ≥15ms
- [ ] ≤3 touch state variables
