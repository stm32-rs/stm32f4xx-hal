# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# F469DISCO Cleanup & Hardware Verification

## TL;DR

> **Quick Summary**: Delete the dead `display_init.rs` shim, verify all 4 remaining display examples on real hardware, and commit the complete f469disco HAL display module migration.
> 
> **Deliverables**:
> - Deletion of `examples/f469disco/display_init.rs` (confirmed 100% dead code)
> - Hardware verification of 4 examples: paint, image-slider, animated-layers, slideshow
> - Clean git commit of all changes (8 modified + 1 new + 1 deleted file)
> - Final Verification Wave (F1-F4) re-run with all passing
> 
> **Estimated Effort**: Short
> **Parallel Execution**: NO — strictly sequential pipeline
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → F1-F4

---

## Context

### Original Request
Complete the f469disco display HAL migration by cleaning up dead code, verifying all display examples on real hardware, and committing the clean state. This is the final phase of the "Embedded-Graphics Hello World on STM32F469I-DISCO" effort.

### Previous Work (Tasks 1-6 from parent plan)
All implementation tasks are DONE:
- **Task 1**: Cargo.toml — `embedded-graphics-core` dep, `framebuffer` feature, example entries ✅
- **Task 2**: `DisplayController` — `new_dsi()`, `layer_buffer_mut()`, `set_layer_transparency()`, `set_layer_buffer_address()`, `set_color_keying()`, ARGB4444 fix ✅
- **Task 3**: `LtdcFramebuffer` — `DrawTarget` impl ✅
- **Task 4**: All 4 broken examples compile ✅
- **Task 5**: `f469disco-hello-eg.rs` created ✅
- **Task 6**: Full compilation verification ✅

### Hardware Verification Status
- `f469disco-lcd-test` — **WORKING** (verified on hardware, DO NOT MODIFY)
- `f469disco-hello-eg` — **WORKING** (verified on hardware with current HAL import code)
- `f469disco-paint` — **Compiles, NOT YET verified on hardware**
- `f469disco-image-slider` — **Compiles, NOT YET verified on hardware**
- `f469disco-animated-layers` — **Compiles, NOT YET verified on hardware**
- `f469disco-slideshow` — **Compiles, NOT YET verified on hardware**

### Metis Review
**Identified Gaps** (addressed):
- `display_init.rs` is 100% dead code — confirmed no `#[path]` references anywhere → DELETE entirely
- `probe-rs` not on default PATH on remote → every SSH command must set PATH
- Must use explicit `git add` per file — untracked dirs (`dist/`, `.sisyphus/`, `STM32CubeF4/`, `stm32f7xx-hal/`) must NOT be committed
- Flash from `target/` dir (unstripped ELF with defmt info), not from `dist/` (stripped)
- Hardware examples run `loop { wfi() }` forever — use `timeout` with `probe-rs run` (exit code 124 = success)

---

## Work Objectives

### Core Objective
Clean up the dead `display_init.rs` file, verify all 4 unverified examples work on the real STM32F469I-DISCO board, and create a clean atomic git commit capturing the entire f469disco HAL display module migration.

### Concrete Deliverables
- `examples/f469disco/display_init.rs` deleted (was 393 lines of dead code)
- 4 examples hardware-verified: defmt output captured showing successful DSI init, panel detect, no crash
- 1 git commit with 8 modified files + 1 new file + 1 deleted file

### Definition of Done
- [ ] `examples/f469disco/display_init.rs` does NOT exist
- [ ] All 6 f469disco examples compile in release mode
- [ ] 4 examples (paint, image-slider, animated-layers, slideshow) flash and run on STM32F469NIHx without panic
- [ ] Git working tree is clean (no uncommitted changes to tracked files)
- [ ] Git commit includes the file deletion and all modified/new files
- [ ] F1-F4 final verification wave passes

### Must Have
- Delete `examples/f469disco/display_init.rs`
- Hardware defmt output for each of the 4 examples showing successful init
- Atomic git commit with explicit file staging

### Must NOT Have (Guardrails)
- Do NOT modify `examples/f469disco-lcd-test.rs`
- Do NOT modify `examples/f469disco/nt35510.rs`
- Do NOT modify `examples/f469disco/images.rs`
- Do NOT use `git add .` or `git add -A` — untracked work dirs must not be committed
- Do NOT commit `dist/`, `.sisyphus/`, `STM32CubeF4/`, `stm32f7xx-hal/`, or `*.md` work notes
- Do NOT flash stripped binaries from `dist/` — use unstripped ELF from `target/`

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (no_std embedded target)
- **Automated tests**: None (cross-compiled no_std)
- **Framework**: None
- **Verification method**: Compilation + hardware flash with defmt capture

### QA Policy
Every task includes agent-executed QA scenarios:
- **Build tasks**: `cargo build --release` with appropriate features
- **Hardware tasks**: `probe-rs run` with timeout, defmt output capture
- **Git tasks**: `git show --stat`, `git diff`, `git status` verification
- Evidence saved to `.sisyphus/evidence/task-{N}-*.txt`

### Remote Board Configuration
- **Host**: `ubuntu@192.168.13.246`
- **Chip**: `STM32F469NIHx`
- **Board revision**: B08 (NT35510 display)
- **PATH**: Must set `export PATH=$PATH:$HOME/.cargo/bin` in every SSH command
- **SSH pattern**: `ssh -o ConnectTimeout=10 -o BatchMode=yes ubuntu@192.168.13.246 ". ~/.cargo/env && <command>"`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
└── Task 1: Delete display_init.rs [quick]

Wave 2 (After Wave 1):
└── Task 2: Rebuild all 6 examples in release mode [quick]

Wave 3 (After Wave 2):
└── Task 3: Flash & verify 4 examples on hardware (SEQUENTIAL within task) [deep]

Wave 4 (After Wave 3):
└── Task 4: Git commit with explicit staging [quick]

Wave 5 (After Wave 4):
└── Task 5: Re-run hello-eg on hardware (regression check after commit) [quick]

Wave FINAL (After Wave 5):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Hardware QA summary (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → F1-F4
Parallelization: None in main pipeline (sequential), 4-way parallel in FINAL
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 2 |
| 2 | 1 | 3 |
| 3 | 2 | 4 |
| 4 | 3 | 5 |
| 5 | 4 | F1-F4 |
| F1-F4 | 5 | — |

### Agent Dispatch Summary

- **Wave 1**: 1 task — T1 → `quick`
- **Wave 2**: 1 task — T2 → `quick`
- **Wave 3**: 1 task — T3 → `deep`
- **Wave 4**: 1 task — T4 → `quick` + `git-master`
- **Wave 5**: 1 task — T5 → `quick`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs


- [ ] 1. Delete `examples/f469disco/display_init.rs`

  **What to do**:
  - Delete the file `examples/f469disco/display_init.rs`
  - This file is 100% dead code: it contains a triple-pasted re-export shim (lines 1-23) and ~370 lines of old implementation
  - NO example references it via `#[path]` — all 5 examples import from the HAL directly: `use stm32f4xx_hal::display::f469disco as display_init;`
  - Verify deletion: `test ! -f examples/f469disco/display_init.rs`
  - Verify no dead references: `grep -r "display_init\.rs" examples/ 2>/dev/null | wc -l` → must be 0

  **Must NOT do**:
  - Do NOT delete `examples/f469disco/images.rs` — used by 3 examples via `#[path]`
  - Do NOT delete `examples/f469disco/nt35510.rs` — used by lcd-test via `#[path]`
  - Do NOT delete the `examples/f469disco/` directory — it still contains `images.rs` and `nt35510.rs`

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file deletion, trivial task
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (standalone)
  - **Blocks**: Task 2
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `examples/f469disco/display_init.rs` — The file to delete. Has 393 lines of dead code: triple re-export shim on lines 1-23, then old implementation from line 25 onward.

  **API/Type References**:
  - None — this is a deletion task

  **WHY Each Reference Matters**:
  - The file is confirmed dead via `grep -r '#\[path.*display_init' examples/` returning zero matches
  - All examples use `use stm32f4xx_hal::display::f469disco as display_init;` instead

  **Acceptance Criteria**:
  - [ ] File `examples/f469disco/display_init.rs` does NOT exist
  - [ ] `examples/f469disco/images.rs` still exists
  - [ ] `examples/f469disco/nt35510.rs` still exists
  - [ ] `grep -r "display_init\.rs" examples/` returns no matches

  **QA Scenarios:**

  ```
  Scenario: display_init.rs is deleted, sibling files preserved
    Tool: Bash
    Steps:
      1. Run: test ! -f examples/f469disco/display_init.rs && echo "DELETED" || echo "STILL EXISTS"
      2. Assert: output is "DELETED"
      3. Run: test -f examples/f469disco/images.rs && echo "PRESERVED" || echo "MISSING"
      4. Assert: output is "PRESERVED"
      5. Run: test -f examples/f469disco/nt35510.rs && echo "PRESERVED" || echo "MISSING"
      6. Assert: output is "PRESERVED"
      7. Run: grep -r "display_init\.rs" examples/ 2>/dev/null | wc -l
      8. Assert: output is "0"
    Expected Result: Dead file deleted, sibling files untouched
    Failure Indicators: File still exists, or sibling files were accidentally deleted
    Evidence: .sisyphus/evidence/task-1-deletion.txt
  ```

  **Commit**: NO (part of Task 4 atomic commit)

- [ ] 2. Rebuild all 6 examples in release mode

  **What to do**:
  - Build all 6 f469disco examples in release mode to confirm `display_init.rs` deletion breaks nothing
  - Use these exact commands:
    ```bash
    cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo build --release --example f469disco-image-slider --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo build --release --example f469disco-animated-layers --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo build --release --example f469disco-slideshow --features="stm32f469,stm32-fmc,framebuffer,defmt"
    cargo build --release --example f469disco-lcd-test --features="stm32f469,dsihost,defmt"
    ```
  - All 6 must exit with code 0 and produce binaries in `target/thumbv7em-none-eabihf/release/examples/`
  - Note: `lcd-test` uses different features (`dsihost` not `stm32-fmc,framebuffer`)

  **Must NOT do**:
  - Do NOT modify any source files — this is build verification only
  - If any build fails, report the error but do NOT fix it in this task (escalate)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Running 6 cargo build commands and checking exit codes
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 3
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `Cargo.toml` — Example entries with required-features define which features each example needs
  - `.cargo/config.toml` — Build target configuration (thumbv7em-none-eabihf)

  **WHY Each Reference Matters**:
  - Features must match exactly what each example's `required-features` specifies in Cargo.toml

  **Acceptance Criteria**:
  - [ ] All 6 `cargo build --release` commands exit with code 0
  - [ ] Binaries exist at `target/thumbv7em-none-eabihf/release/examples/f469disco-*`
  - [ ] No compilation errors or warnings related to `display_init`

  **QA Scenarios:**

  ```
  Scenario: All 6 examples build successfully in release mode
    Tool: Bash
    Steps:
      1. Run: cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      2. Assert: exit code 0, output contains "Finished"
      3. Run: cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      4. Assert: exit code 0, output contains "Finished"
      5. Run: cargo build --release --example f469disco-image-slider --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      6. Assert: exit code 0, output contains "Finished"
      7. Run: cargo build --release --example f469disco-animated-layers --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      8. Assert: exit code 0, output contains "Finished"
      9. Run: cargo build --release --example f469disco-slideshow --features="stm32f469,stm32-fmc,framebuffer,defmt" 2>&1
      10. Assert: exit code 0, output contains "Finished"
      11. Run: cargo build --release --example f469disco-lcd-test --features="stm32f469,dsihost,defmt" 2>&1
      12. Assert: exit code 0, output contains "Finished"
    Expected Result: All 6 release binaries built without errors
    Failure Indicators: Any error[E...] in output, missing symbols, unresolved imports
    Evidence: .sisyphus/evidence/task-2-release-build.txt
  ```

  **Commit**: NO (build verification only)

- [ ] 3. Flash and verify 4 examples on real hardware

  **What to do**:
  - For EACH of the 4 unverified examples, perform this sequence SEQUENTIALLY (one at a time):
    1. Transfer the unstripped ELF binary from `target/thumbv7em-none-eabihf/release/examples/` to the remote board:
       ```bash
       scp target/thumbv7em-none-eabihf/release/examples/<EXAMPLE_NAME> ubuntu@192.168.13.246:/tmp/
       ```
    2. Flash and capture defmt output (15-second timeout):
       ```bash
       ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/<EXAMPLE_NAME> && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/<EXAMPLE_NAME> 2>&1"
       ```
    3. Save defmt output to evidence file
    4. Verify output contains DSI/display init messages and NO panic/hardfault
  - Examples to verify (in this order):
    1. `f469disco-paint`
    2. `f469disco-image-slider`
    3. `f469disco-animated-layers`
    4. `f469disco-slideshow`
  - **JTAG recovery**: If `probe-rs` fails with `JtagNoDeviceConnected`, use `--connect-under-reset`:
    ```bash
    probe-rs download --chip STM32F469NIHx --connect-under-reset /tmp/<EXAMPLE_NAME>
    ```

  **Must NOT do**:
  - Do NOT flash `f469disco-lcd-test` — already verified, must not be touched
  - Do NOT flash `f469disco-hello-eg` — already verified on hardware
  - Do NOT use binaries from `dist/` — use unstripped ELF from `target/`
  - Do NOT flash multiple examples simultaneously

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Hardware-in-the-loop verification requiring careful sequencing and error recovery
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential pipeline)
  - **Blocks**: Task 4
  - **Blocked By**: Task 2

  **References**:

  **Pattern References**:
  - Previous hello-eg hardware verification (from session context) — exact same flash/verify pattern was used successfully
  - SSH command pattern: `ssh ubuntu@192.168.13.246 ". ~/.cargo/env && <command>"`
  - JTAG recovery pattern: `probe-rs download --chip STM32F469NIHx --connect-under-reset /tmp/<name>`

  **Expected defmt output pattern** (based on hello-eg verified run):
  ```
  [INFO ] Initializing SDRAM...
  [INFO ] Initializing DSI...
  [WARN ] NT35510 probe attempt N failed: DSI read error    ← acceptable (retries)
  [WARN ] Probe inconclusive; defaulting to NT35510          ← acceptable (B08 board)
  [INFO ] Detected LCD controller: Nt35510
  [INFO ] Initializing NT35510 (B08 revision)...
  [INFO ] Display initialized successfully
  ```

  **Per-example expected behavior**:
  - `f469disco-paint`: Initializes display + touch. Shows paint canvas. May log touch events if finger on screen. Expected to see `Initializing FT6X06 touch controller` or similar.
  - `f469disco-image-slider`: Initializes display. Generates patterns in SDRAM. Shows first image. Logs pattern generation.
  - `f469disco-animated-layers`: Initializes display with 2 layers. Animated rectangle bouncing. May log layer config.
  - `f469disco-slideshow`: Initializes display with 2 layers. Crossfade animation between patterns. May log transparency changes.

  **Acceptance Criteria**:
  - [ ] `f469disco-paint` flashed and defmt output captured — no panic
  - [ ] `f469disco-image-slider` flashed and defmt output captured — no panic
  - [ ] `f469disco-animated-layers` flashed and defmt output captured — no panic
  - [ ] `f469disco-slideshow` flashed and defmt output captured — no panic
  - [ ] All 4 defmt outputs show DSI initialization messages
  - [ ] None of the 4 outputs contain "panic" or "HardFault"

  **QA Scenarios:**

  ```
  Scenario: f469disco-paint runs on hardware without crash
    Tool: Bash (SSH + probe-rs)
    Preconditions: Binary built at target/thumbv7em-none-eabihf/release/examples/f469disco-paint
    Steps:
      1. scp target/thumbv7em-none-eabihf/release/examples/f469disco-paint ubuntu@192.168.13.246:/tmp/
      2. ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-paint && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-paint 2>&1"
      3. Assert: output contains "SDRAM" or "DSI" or "Display" (init messages present)
      4. Assert: output does NOT contain "panicked" or "HardFault" or "BELOW292"
    Expected Result: Paint example initializes display and enters main loop without crash
    Failure Indicators: panic message, HardFault, probe-rs connection error, no output at all
    Evidence: .sisyphus/evidence/task-3-paint-hw.txt

  Scenario: f469disco-image-slider runs on hardware without crash
    Tool: Bash (SSH + probe-rs)
    Preconditions: Binary built at target/thumbv7em-none-eabihf/release/examples/f469disco-image-slider
    Steps:
      1. scp target/thumbv7em-none-eabihf/release/examples/f469disco-image-slider ubuntu@192.168.13.246:/tmp/
      2. ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-image-slider && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-image-slider 2>&1"
      3. Assert: output contains init messages, no panic
    Expected Result: Image slider initializes and shows first pattern
    Failure Indicators: panic, HardFault, probe-rs error
    Evidence: .sisyphus/evidence/task-3-image-slider-hw.txt

  Scenario: f469disco-animated-layers runs on hardware without crash
    Tool: Bash (SSH + probe-rs)
    Preconditions: Binary built at target/thumbv7em-none-eabihf/release/examples/f469disco-animated-layers
    Steps:
      1. scp target/thumbv7em-none-eabihf/release/examples/f469disco-animated-layers ubuntu@192.168.13.246:/tmp/
      2. ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-animated-layers && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-animated-layers 2>&1"
      3. Assert: output contains init messages, no panic
    Expected Result: Animated layers initializes 2-layer display and starts animation
    Failure Indicators: panic, HardFault, probe-rs error
    Evidence: .sisyphus/evidence/task-3-animated-layers-hw.txt

  Scenario: f469disco-slideshow runs on hardware without crash
    Tool: Bash (SSH + probe-rs)
    Preconditions: Binary built at target/thumbv7em-none-eabihf/release/examples/f469disco-slideshow
    Steps:
      1. scp target/thumbv7em-none-eabihf/release/examples/f469disco-slideshow ubuntu@192.168.13.246:/tmp/
      2. ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-slideshow && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-slideshow 2>&1"
      3. Assert: output contains init messages, no panic
    Expected Result: Slideshow initializes and starts crossfade animation
    Failure Indicators: panic, HardFault, probe-rs error
    Evidence: .sisyphus/evidence/task-3-slideshow-hw.txt
  ```

  **Evidence to Capture:**
  - [ ] task-3-paint-hw.txt — full defmt output from paint example
  - [ ] task-3-image-slider-hw.txt — full defmt output from image-slider example
  - [ ] task-3-animated-layers-hw.txt — full defmt output from animated-layers example
  - [ ] task-3-slideshow-hw.txt — full defmt output from slideshow example

  **Commit**: NO (hardware verification only)

- [x] 4. Git commit all changes (atomic)
  **What to do**:
  - Stage ONLY the specific files that are part of this migration. Use explicit `git add` for each:
    ```bash
    git add Cargo.toml
    git add src/display/mod.rs
    git add src/display/f469disco.rs
    git add examples/f469disco-hello-eg.rs
    git add examples/f469disco-animated-layers.rs
    git add examples/f469disco-image-slider.rs
    git add examples/f469disco-paint.rs
    git add examples/f469disco-slideshow.rs
    git rm examples/f469disco/display_init.rs
    ```
  - Commit with message: `feat(display): add f469disco HAL module, migrate examples, remove dead display_init shim`
  - Verify commit with:
    ```bash
    git show --stat HEAD
    git diff --name-only HEAD  # must be empty for tracked files
    git status  # only untracked work dirs should remain
    ```

  **Must NOT do**:
  - NEVER use `git add .` or `git add -A` — untracked dirs must not be committed:
    - `dist/` (built binaries)
    - `.sisyphus/` (plan files)
    - `STM32CubeF4/` (reference firmware)
    - `stm32f7xx-hal/` (reference HAL)
    - `*.md` work notes (f469-disco-revc-board-support.md, lis.md, notes-vscode.md)
    - `tools/` (build scripts)
  - Do NOT amend any previous commits
  - Do NOT push to remote (user will push when ready)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Git staging and commit — straightforward operations
  - **Skills**: [`git-master`]
    - `git-master`: Safe git operations with precise staging

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 5, F1-F4
  - **Blocked By**: Task 3

  **References**:

  **Pattern References**:
  - Previous commits in this branch: `git log --oneline -5`
    - `625855a feat(examples): add f469disco display examples with NT35510/OTM8009A autodetection`
    - `5e71df9 feat(cargo): add framebuffer feature and f469disco example entries`
    - `0bcae14 feat(ltdc): add DisplayController DSI methods, LtdcFramebuffer, and NT35510 HAL driver`
  - These follow `type(scope): description` format — new commit should match

  **Acceptance Criteria**:
  - [ ] `git show --stat HEAD` includes all 9 files (8 modified + 1 new) plus 1 deletion
  - [ ] `git diff --name-only HEAD` returns empty (no uncommitted tracked changes)
  - [ ] `git status` shows only untracked work directories
  - [ ] Commit message follows `feat(scope): description` pattern
  - [ ] `display_init.rs` deletion appears in commit
  - [ ] `f469disco.rs` addition appears in commit

  **QA Scenarios:**

  ```
  Scenario: Git commit includes exactly the right files
    Tool: Bash
    Steps:
      1. Run: git show --stat HEAD --format=''
      2. Assert: output includes 'examples/f469disco/display_init.rs' (deletion)
      3. Assert: output includes 'src/display/f469disco.rs' (addition)
      4. Assert: output includes 'Cargo.toml'
      5. Assert: output includes all 5 example files
      6. Assert: output includes 'src/display/mod.rs'
      7. Run: git diff --name-only HEAD
      8. Assert: empty output (no uncommitted tracked changes)
      9. Run: git status --porcelain | grep -v '^??' | wc -l
      10. Assert: output is "0" (no staged/modified tracked files)
    Expected Result: Clean commit with exactly the migration files
    Failure Indicators: Missing files in commit, uncommitted changes remaining, untracked files accidentally staged
    Evidence: .sisyphus/evidence/task-4-git-commit.txt

  Scenario: No untracked work files were committed
    Tool: Bash
    Steps:
      1. Run: git show --stat HEAD --format='' | grep -E 'dist/|.sisyphus/|STM32CubeF4/|stm32f7xx-hal/|tools/|\.md' | grep -v 'display/mod.rs' | grep -v 'f469disco.rs'
      2. Assert: empty output (none of these should appear in commit)
    Expected Result: No work files leaked into the commit
    Failure Indicators: Any dist/, .sisyphus/, STM32CubeF4/ or work note files in commit
    Evidence: .sisyphus/evidence/task-4-no-leak.txt
  ```

  **Commit**: YES (this IS the commit task)
  - Message: `feat(display): add f469disco HAL module, migrate examples, remove dead display_init shim`
  - Files: see explicit list above

- [x] 5. Post-commit regression check — re-verify hello-eg on hardware

  **What to do**:
  - Rebuild and re-flash `f469disco-hello-eg` after the commit to confirm no regression:
    ```bash
    cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
    scp target/thumbv7em-none-eabihf/release/examples/f469disco-hello-eg ubuntu@192.168.13.246:/tmp/
    ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-hello-eg && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-hello-eg 2>&1"
    ```
  - Verify defmt output shows same successful init pattern as before
  - This confirms the commit didn't break anything

  **Must NOT do**:
  - Do NOT modify any files — this is verification only

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single build + flash + verify
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5
  - **Blocks**: F1-F4
  - **Blocked By**: Task 4

  **References**:
  - Previous hello-eg verification from earlier session — same commands, same expected output

  **Acceptance Criteria**:
  - [ ] `f469disco-hello-eg` builds in release mode
  - [ ] Flashes to hardware without error
  - [ ] defmt output shows "Display initialized successfully" and "Hello embedded-graphics!"
  - [ ] No panic or HardFault in output

  **QA Scenarios:**

  ```
  Scenario: hello-eg works after commit (regression check)
    Tool: Bash (SSH + probe-rs)
    Steps:
      1. cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
      2. scp target/thumbv7em-none-eabihf/release/examples/f469disco-hello-eg ubuntu@192.168.13.246:/tmp/
      3. ssh ubuntu@192.168.13.246 ". ~/.cargo/env && probe-rs download --chip STM32F469NIHx /tmp/f469disco-hello-eg && probe-rs reset --chip STM32F469NIHx && sleep 1 && timeout 15 probe-rs attach --chip STM32F469NIHx /tmp/f469disco-hello-eg 2>&1"
      4. Assert: output contains "Hello embedded-graphics"
      5. Assert: output does NOT contain "panicked" or "HardFault"
    Expected Result: hello-eg still works after commit
    Failure Indicators: build failure, flash error, panic on hardware
    Evidence: .sisyphus/evidence/task-5-hello-eg-regression.txt
  ```

  **Commit**: NO (verification only)

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (check file deletion, check defmt evidence, check git commit). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt"`. Verify no dead code references to `display_init.rs` remain (`grep -r "display_init\.rs" examples/`). Check that `src/display/f469disco.rs` has no `#[allow(dead_code)]` that shouldn't be there. Verify all modified example files use HAL import pattern consistently.
  Output: `Build [PASS/FAIL] | Dead Code [CLEAN/N refs] | Import Pattern [CONSISTENT/N issues] | VERDICT`

- [ ] F3. **Hardware QA Summary** — `unspecified-high`
  Read all `.sisyphus/evidence/task-3-*.txt` files. For each of the 4 examples: verify defmt output shows DSI init + panel detection + no panic. Verify `probe-rs` exit was clean (exit code 0 or 124). Cross-check that the board revision (B08) and panel type (NT35510) are consistent across all runs. Flag any anomalies.
  Output: `Examples Verified [N/4] | Panel Consistent [YES/NO] | Anomalies [NONE/list] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", verify actual changes match spec. Verify `display_init.rs` is deleted. Verify NO modifications to `lcd-test.rs`, `nt35510.rs`, `images.rs`. Check git commit includes exactly the right files (8 modified + 1 new + 1 deleted). Detect any unaccounted changes. Verify no untracked work files were committed.
  Output: `Tasks [N/N compliant] | Protected Files [CLEAN/N issues] | Commit Scope [CORRECT/N issues] | VERDICT`

---

## Commit Strategy

- **Single atomic commit after Wave 4**: `feat(display): add f469disco HAL module, migrate examples, remove dead display_init shim`
- **Files to stage** (explicit list):
  - `git add Cargo.toml`
  - `git add src/display/mod.rs`
  - `git add src/display/f469disco.rs`
  - `git add examples/f469disco-hello-eg.rs`
  - `git add examples/f469disco-animated-layers.rs`
  - `git add examples/f469disco-image-slider.rs`
  - `git add examples/f469disco-paint.rs`
  - `git add examples/f469disco-slideshow.rs`
  - `git rm examples/f469disco/display_init.rs`
- **Pre-commit check**: `cargo check --target thumbv7em-none-eabihf --features="stm32f469,stm32-fmc,framebuffer,defmt"`

---

## Success Criteria

### Verification Commands
```bash
# File deletion verified
test ! -f examples/f469disco/display_init.rs && echo "PASS" || echo "FAIL"

# No dead references
grep -r "display_init\.rs" examples/ 2>/dev/null | wc -l  # Expected: 0

# All 6 examples compile
cargo build --release --example f469disco-hello-eg --features="stm32f469,stm32-fmc,framebuffer,defmt"
cargo build --release --example f469disco-paint --features="stm32f469,stm32-fmc,framebuffer,defmt"
cargo build --release --example f469disco-image-slider --features="stm32f469,stm32-fmc,framebuffer,defmt"
cargo build --release --example f469disco-animated-layers --features="stm32f469,stm32-fmc,framebuffer,defmt"
cargo build --release --example f469disco-slideshow --features="stm32f469,stm32-fmc,framebuffer,defmt"
cargo build --release --example f469disco-lcd-test --features="stm32f469,dsihost,defmt"

# Git clean
git diff --name-only HEAD  # Expected: empty
git show --stat HEAD  # Expected: includes display_init.rs deletion + f469disco.rs addition
```

### Final Checklist
- [ ] `display_init.rs` deleted
- [ ] All 6 examples compile in release mode
- [ ] 4 examples verified on hardware (defmt evidence captured)
- [ ] No `display_init.rs` references remain in codebase
- [ ] Git commit is clean and includes exactly the right files
- [ ] All "Must NOT Have" absent
- [ ] F1-F4 all APPROVE
