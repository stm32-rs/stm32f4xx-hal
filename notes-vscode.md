# Touch Issue Analysis - STM32F469I-DISCO LCD Test

## Implementation Status

✅ **COMPLETED** - Comprehensive diagnostic logging infrastructure added to `examples/f469disco-lcd-test.rs`

### What Was Added

#### 1. TouchEvent Struct
Captures full state snapshot after each touch detection:
- Poll counter (sequential numbering)
- Detected touch count (`num`)
- Coordinates (x, y) and delta from previous touch
- State flags: `touch_active`, `waiting_for_release`, `touch_hold_ticks`
- Error codes: `detect_error`, `read_error` (0=ok, 1=error, 2=timeout)
- Gate decision: All 3 conditions evaluated separately + toggle result
- Toggle execution flag

#### 2. TouchHistory Ring Buffer
- Last 32 touch events in circular buffer
- `dump_last_n()` function for post-mortem analysis
- Provides complete sequence of all detection failures with context

#### 3. Phase-Based Logging (6 Phases)
Each touch poll is logged with structured phases:

- **PHASE 1: DETECT** - `detect_touch()` call result with attempt count
  ```
  DETECT attempt 1/3: success/error [2ms delay per retry]
  ```

- **PHASE 2: READ** - `get_touch()` coordinate read with delta calculation
  ```
  DETECT: num=1, READ: x=240, y=400, delta=(+2,-1), detect_err=0, read_err=0
  ```

- **PHASE 3: STATE MACHINE** - Touch active/release/timeout transitions
  ```
  TIMEOUT: FT6x06 held num>0 for 500ms, forcing release
  RELEASE: num=0, held for 100ms
  ```

- **PHASE 4: GATE DECISION** - Each toggle condition evaluated
  ```
  GATE: num>0=true, !active_before=true, !wait=true → ✓ TOGGLE
  or
  GATE: num>0=true, !active_before=false, !wait=true → ✗ BLOCKED
    └─ REASON: touch_active=true (not rising edge, still held)
  ```

- **PHASE 5: TOGGLE EXECUTION** - Confirms toggle or details why blocked
  ```
  ✓ TOGGLE #1: Color test at x=240, y=400
  or
  ✗ BLOCKED: see GATE phase for reason
  ```

- **PHASE 6: REGISTER DUMP** - FT6x06 register values (periodic)
  ```
  FT6x06 REG: TD_STATUS=0x01 at poll #5 [ready for expansion]
  ```

#### 4. Diagnostic Tracking Variables
- `poll_count` - Sequential poll number (1, 2, 3, ...)
- `last_coord_x/y` - Previous touch coordinates for delta calculation
- `total_toggles` - Total toggle count for correlation
- `register_dump_counter` - Triggers periodic FT6x06 register reads

---

## Testing Plan

### Test 3.1: Rapid-Fire Taps at Same Location

**Setup:**
```bash
cd /Users/macbook/src/stm32f4xx-hal
cargo run --release --example f469disco-lcd-test --features="stm32f469,defmt" 2>&1 | tee test-3.1-log.txt
```

**Procedure:**
1. Display boots and shows Color test pattern
2. Tap LCD screen at same location 5 times quickly (tap, lift, tap, lift, ~300ms intervals)
3. Observe RTT output
4. Let system idle for 10 seconds to see any state recovery

**Expected Output Signature (SUCCESS):**
```
[Poll 10] DETECT: num=1, READ: x=240, y=400, ...
          GATE: num>0=✓, !active=✓, !wait=✓ → ✓ TOGGLE #1: Color
[Poll 11-40] GATE: num=0, no touch [release]
[Poll 50] DETECT: num=1, READ: x=238, y=401, delta=(−2,+1), ...
          GATE: num>0=✓, !active=✓, !wait=✓ → ✓ TOGGLE #2: BER
[Poll 51-80] GATE: num=0, no touch [release]
[Poll 90] DETECT: num=1, READ: x=241, y=399, delta=(+3,−2), ...
          GATE: num>0=✓, !active=✓, !wait=✓ → ✓ TOGGLE #3: Color
```

**Expected Output Signature (FAILURE #1: Gate Blocks on Edge)**
```
[Poll 10] GATE: num=1 ✓, !active=✗ [REASON: touch_active=true, still held], !wait=✓ → ✗ BLOCKED
          └─ REASON: not rising edge
[Poll 11-50] GATE: num=1 ✓, !active=✗, !wait=✓ → ✗ BLOCKED [continuous]
[Poll 51] TIMEOUT: FT6x06 held num>0 for 500ms
          waiting_for_release=true [suppression active]
[Poll 52+] GATE: num>0 ✓ but waiting=✗ → ✗ BLOCKED
```
**Root Cause:** FT6x06 not reporting `num=0` between taps, OR reading stale data

**Expected Output Signature (FAILURE #2: Timeout Too Aggressive)**
```
[Poll 10] GATE: num=1 ✓, !active=✓, !wait=✓ → ✓ TOGGLE #1
[Poll 11-25] State machine counting up...
[Poll 25] TIMEOUT: FT6x06 held num>0 for 500ms
          └─ Last coord: x=240, y=400 (held for 500ms)
          waiting_for_release=true [suppression active]
[Poll 26] DETECT: num=0 [but suppression still active]
[Poll 50] DETECT: num=1 again [new tap]
          GATE: waiting_for_release=true → ✗ BLOCKED
```
**Root Cause:** Timeout fired before finger was released, preventing next toggle even after lift

### Test 3.2: Tap with Complete Finger Lift

**Setup:** Same as 3.1

**Procedure:**
1. Tap, fully lift (wait 150+ms), tap, repeat 5 times
2. Watch for `num=0` appearing between taps
3. Verify each tap toggles 

**Expected Output Signature (SUCCESS):**
```
[Poll 10] DETECT: num=1, GATE: ✓ TOGGLE #1
[Poll 11-20] DETECT: num=0 [tap held]
[Poll 21-30] DETECT: num=0 [finger lifting]
[Poll 31] DETECT: num=0, RELEASE: detected, normal recovery
[Poll 40+] User lifts finger
[Poll 50] DETECT: num=1, GATE: ✓ TOGGLE #2
```

**Expected Output Signature (FAILURE: num Never Returns to 0)**
```
[Poll 10] DETECT: num=1, GATE: ✓ TOGGLE #1
[Poll 11+] DETECT: num=1 [continuous, never goes to 0!]
[Poll 25] TIMEOUT: FT6x06 held num>0 for 500ms
          waiting_for_release=true
[Poll 26+] DETECT: num=1 still, but gate blocked by waiting_for_release
```
**Root Cause:** FT6x06 stuck reporting num>0 even after finger lifted

### Test 3.3: Long Hold (600ms+)

**Procedure:**
1. Press touch, hold for 600ms, release
2. Immediately try another tap
3. Watch for timeout and recovery

**Expected Output Signature (SUCCESS):**
```
[Poll 10] DETECT: num=1, GATE: ✓ TOGGLE #1
[Poll 25] TIMEOUT: FT6x06 held num>0 for 500ms, forcing release
[Poll 26] DETECT: num=0, RELEASE: detected, suppression clears
[Poll 40+] Finger lifted
[Poll 50] DETECT: num=1, GATE: ✓ TOGGLE #2 [recovers after timeout]
```

**Expected Output Signature (FAILURE: Recovery Stuck)**
```
[Poll 10] TIMEOUT: FT6x06 held num>0, forced release
          waiting_for_release=true [suppression active]
[Poll 26-100] DETECT: num=1 (or num=0), but waiting=true blocks all toggles
[Poll 50] New tap: GATE: waiting=✗ → ✗ BLOCKED [can't toggle until waiting clears]
```
**Root Cause:** Suppression doesn't clear because num never goes to 0

### Test 3.4: Coordinate Drift During Hold

**Procedure:**
1. Press touch, slowly drag 50px, release
2. Watch coordinates update during drag
3. Verify no huge jumps (indicates I2C corruption)

**Expected Output Signature:**
```
[Poll 10] READ: x=240, y=400, delta=(0,0)
[Poll 11] READ: x=242, y=402, delta=(+2,+2) [smooth drift]
[Poll 12] READ: x=244, y=404, delta=(+2,+2) [consistent]
[Poll 13] READ: x=246, y=406, delta=(+2,+2) [smooth...]
[Poll 14] READ: x=288, y=445, delta=(+42,+39) [reached 50px]
```

**Expected Output Signature (FAILURE: Coords Stale or Jump)**
```
[Poll 10] READ: x=240, y=400
[Poll 11-15] READ: x=240, y=400 [NO CHANGE - stale data!]
or
[Poll 10] READ: x=240, y=400
[Poll 11] READ: x=240, y=400
[Poll 12] READ: x=500, y=700 [HUGE JUMP - corruption!]
```
**Root Cause:** FT6x06 FIFO overflow or I2C read incomplete

### Test 3.5: I2C Bus Stress

**Procedure:**
1. Rapid-fire taps while LCD is actively updating display
2. Look for I2C read errors correlated with timeouts
3. Check if errors propagate to state corruption

**Expected Output Signature (SUCCESS):**
```
[Poll 10] DETECT attempt 1/3: success
[Poll 50] DETECT attempt 1/3: success
[Poll 100] DETECT attempt 1/3: success [all succeed on first try]
```

**Expected Output Signature (FAILURE: I2C Contention)**
```
[Poll 10] DETECT attempt 1/3: error
[Poll 10] DETECT attempt 2/3: error
[Poll 10] DETECT attempt 3/3: error [retry madness]
          └─ DETECT timed out after 3 attempts
[Poll 11] READ: also fails
[Poll 12] Coordinates missing from log
→ Followed by TIMEOUT or gate decision logged without coordinate context
```
**Root Cause:** I2C bus hung, SDA/SCL stuck, or clock stretching timeout

---

## Post-Mortem Analysis Template

After each test run, save the log and analyze:

```
# Test 3.1 Analysis
Date: 2026-02-25
Board: STM32F469I-DISCO B08 (NT35510)
Test: Rapid-fire taps at same location

Observations:
- Total toggles: [count from logs]
- First toggle: ✓ (poll #10)
- Subsequent toggles: ✓/✗
- Pattern: [describe what % of taps worked]

Gate Blocking Sequence:
- num>0 checks: [% pass]
- !active_before checks: [% pass]
- !waiting_for_release checks: [% pass]

Failure Signature: [which condition fails most]
```

### Root Cause Decision Tree

```
START: Taps work 1st time, then stuck?
│
├─ GATE: num>0 ✗ (num never changes)
│  └─ FT6x06 STATUS: Check if stuck reporting num=0
│
├─ GATE: !active ✗ (touch_active stays true)
│  └─ NUM HISTORY: Look for if num ever goes back to 0
│     ├─ YES → touch_active tracking logic broken
│     └─ NO → FT6x06 not releasing, proceed below
│
├─ GATE: !waiting ✗ (waiting_for_release=true)
│  └─ TIMEOUT or normal sequence?
│     ├─ TIMEOUT occurred → FT6x06 held num>0 for 500ms
│     │  └─ Did num go to 0 after timeout?
│     │     ├─ YES (eventually) → timeout too aggressive, increase TOUCH_HOLD_TIMEOUT_TICKS
│     │     └─ NO (never) → FT6x06 stuck contact, needs reset
│     └─ NO TIMEOUT → touch released correctly, what's waiting_for_release doing on?
│
└─ All gates pass but toggle doesn't happen?
   └─ Bug in toggle execution (unlikely, check toggle_happened flag)
```

---

## How to Extract and Analyze Logs

### 1. Capture Log to File
```bash
# Start test, capture RTT output to file
cargo run --release --example f469disco-lcd-test --features="stm32f469,defmt" 2>&1 | tee test-run.txt
```

### 2. Extract Touch Sequence
```bash
# Show all DETECT/GATE/TOGGLE lines
grep -E "DETECT|GATE|TOGGLE|TIMEOUT|RELEASE" test-run.txt > touch-sequence.txt

# Count toggles
grep "✓ TOGGLE" test-run.txt | wc -l

# Find when blocking starts
grep "✗ BLOCKED" test-run.txt | head -5

# Show error rate
grep -E "detect_err|read_err" test-run.txt | sort | uniq -c
```

### 3. Timeline View
```bash
# Show time-ordered sequence (if timestamps available)
grep "Poll #" test-run.txt | head -30
```

### 4. Failure Context
```bash
# When toggle fails, show 5 polls before/after
grep -B5 -A5 "✗ BLOCKED" test-run.txt | head -50
```

---

## Root Cause Summary

Based on the initial notes.md analysis, the primary suspects are:

| Suspect | Confidence | Failure Signature | Fix |
|---------|------------|-------------------|-----|
| Movement-based rearm | **HIGH** | Old code version; not in current impl | *(already removed)* |
| FT6x06 stuck contact | **HIGH** | num never goes to 0 after tap | Check FT6x06 reset, I2C read completeness |
| Timeout too aggressive | **MEDIUM** | TIMEOUT fires, then waiting_for_release blocks all | Increase TOUCH_HOLD_TIMEOUT_TICKS to 50 (1000ms) |
| I2C bus corruption | **MEDIUM** | Frequent READ/DETECT errors → coord stale → timeout | Check I2C pull-ups, reduce speed, add recovery |
| Edge detection flaw | **LOW** | touch_active stuck true even when num=0 | Review state machine logic |

---

## Execution Checklist

- [ ] Compile: `cargo check --example f469disco-lcd-test --features="stm32f469,defmt"`
- [ ] Deploy to board via CubeProgrammer or probe-run
- [ ] Run **Test 3.1** (rapid taps same location) - **PRIMARY TEST**
- [ ] Save output to `test-3.1-log.txt`
- [ ] Analyze with decision tree above
- [ ] Document failure signature
- [ ] If Test 3.1 shows gate blocking → Root cause identified
- [ ] If Test 3.1 passes → Run **Test 3.2** (with finger lift)
- [ ] Continue through Tests 3.3, 3.4, 3.5 as needed
- [ ] Implement fix based on root cause
- [ ] Retest with same 5 scenarios
- [ ] Verify no regressions

---

## Next Steps

1. **Deploy** the updated example to your board
2. **Run Test 3.1** and capture logs
3. **Analyze output** using the decision tree
4. **Report which condition fails** (gate decision or timing)
5. **Implement targeted fix** (FT6x06 reset, timeout adjust, or gate logic change)
6. **Re-test** all 5 scenarios

The comprehensive logging will make the root cause **immediately obvious** from the gate decision logs.

