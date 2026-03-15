# ❌ CANCELLED — Superseded by upstream-merge.md

> Cancelled on: 2026-02-27

---

# NT35510 PR Improvement Plan

## 📋 Session Resume Guide

> **How to Resume This Planning Session**
>
> In a future session, tell Prometheus:
> ```
> Resume the NT35510 PR improvement plan. Read .sisyphus/plans/nt35510-pr-improvements.md
> ```
>
> Or reference this session ID: `ses_36c6fdbb4ffeGPjkHeQfefQeaL`

---

## 🎯 Key Takeaways from Planning Session

### Critical Discoveries

1. **PR Scope Creep**: The PR grew from "NT35510 support" to 33 files (2,131 additions) including:
   - Complete `src/display/` module (framebuffer, SPI transport, SDRAM helper)
   - Three board support crates (`boards/f469disco`, `f429disco`, `f413disco`)
   - Build orchestration tool (`xtask/`)
   - New feature flags (`spi_display`, `framebuffer`)

2. **Maintainer Objection**: burrbull explicitly objected to example-related feature flags in main Cargo.toml. The original `nt35510-only`/`otm8009a-only` flags were removed, but `spi_display`/`framebuffer` were added.

3. **Architecture Precedent**: `otm8009a` exists as a **separate crate** on crates.io (only 2 dependencies). This is the pattern NT35510 should follow long-term.

4. **Commit Noise**: 25 of 55 commits are AI-generated noise ("Initial plan" commits + merge commits from copilot sub-PRs).

### Decisions Made

| Decision | Choice | Rationale |
|----------|--------|-----------|
| PR Structure | Split into 3 PRs | Easier to review, higher merge chance |
| Architecture | Let maintainer decide | User prefers "working first, iterate later" |
| NT35510 location | Example-only for now | Can become separate crate later |
| Hardware test | BEFORE any code changes | Validates current state works |

### Maintainer Feedback Summary

**burrbull (maintainer)**:
> "I'm not happy with example related features here. Maybe we should restructure examples directory to have its own Cargo.toml. Integrate xtask maybe like in esp-rs/esp-hal?"

**tegimeki (reviewer with B01/OTM8009A board)**:
- OTM8009A returns RDID1=0x40 (different from NT35510's 0x00) ✓ Detection works
- Black stripe on OTM8009A with `otm8009a-only` - **FIXED**
- Touch timeout spam - **FIXED** (throttling added)

### What Works (Already Implemented)

- ✅ Auto-detection of LCD controller (RDID1 probe)
- ✅ Touch support with error throttling
- ✅ Controller-specific DSI timing
- ✅ Double-init guard in NT35510 driver
- ✅ Well-documented code with detection table

### What Needs Work

- ⚠️ Hardware testing on B08 (NT35510) - **YOU ARE DOING THIS TOMORROW**
- ⚠️ Split PR into focused chunks
- ⚠️ Squash AI-generated commits
- ⚠️ Address `GenericShortP0/P1/P2` stubs in `src/dsi.rs:639+`
- ⚠️ Consider removing untested board crates (f413disco, f429disco)

---

## Work Objectives

### Core Objective
Transform a bloated 33-file PR into 3 focused, reviewable PRs that each have a clear purpose and higher chance of merge.

### Concrete Deliverables
1. **PR A (hal-dsi-improvements)**: DSI commands + LTDC DSI constructor
2. **PR B (display-ecosystem)**: Display module + boards/ + xtask/
3. **PR C (nt35510-support)**: NT35510 driver + auto-detection example

### Definition of Done
- [ ] Hardware tested on STM32F469I-DISCO B08 (NT35510)
- [ ] PR A created and CI passes
- [ ] PR B created and CI passes
- [ ] PR C created and CI passes
- [ ] All PRs have clean commit history (no "Initial plan" commits)

### Must Have
- Hardware testing before any code changes
- Backup branch before squashing
- Each PR compiles independently

### Must NOT Have (Guardrails)
- DO NOT include untested board crates (f413disco, f429disco) without hardware
- DO NOT add CI/workflow changes without maintainer agreement
- DO NOT squash all commits into single commit
- DO NOT modify NT35510 driver before hardware test

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: NO (embedded, no unit tests)
- **Automated tests**: None
- **Agent-Executed QA**: Compilation checks only

### QA Policy
Every task includes compilation verification:
```bash
# Core HAL compiles
cargo check --features="stm32f469,dsihost,ltdc"
# Example compiles
cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
```

---

## Execution Strategy

### Sequential Execution (PRs depend on each other)

```
Gate 0 (Before Any Work):
└── Hardware Test on STM32F469I-DISCO B08

PR A (hal-dsi-improvements) - Foundation:
├── Task A1: Create backup branch
├── Task A2: Create PR A branch from master
├── Task A3: Cherry-pick DSI improvements
├── Task A4: Cherry-pick LTDC improvements  
├── Task A5: Fix GenericShortP0/P1/P2 stubs
├── Task A6: Squash to ~3 logical commits
└── Task A7: Push PR A, verify CI

PR B (display-ecosystem) - Infrastructure:
├── Task B1: Create PR B branch from PR A
├── Task B2: Add src/display/ module
├── Task B3: Add boards/ crates
├── Task B4: Add xtask/
├── Task B5: Add feature flags to Cargo.toml
├── Task B6: Squash to ~4 logical commits
└── Task B7: Push PR B, verify CI

PR C (nt35510-support) - The Original Goal:
├── Task C1: Create PR C branch from PR A
├── Task C2: Add NT35510 driver
├── Task C3: Add auto-detection example
├── Task C4: Update CHANGELOG
├── Task C5: Squash to ~3 logical commits
└── Task C6: Push PR C, verify CI

Final:
└── Task F1: Update PR descriptions, request review
```

### Dependency Matrix

- **A1-A7**: No dependencies (foundation)
- **B1-B7**: Depends on PR A being ready
- **C1-C6**: Depends on PR A being ready (PR B optional)
- **F1**: Depends on all PRs being ready

### Agent Dispatch Summary

- **Gate 0**: User task (hardware testing)
- **PR A**: 7 tasks → `quick` for git operations, `unspecified-low` for code fixes
- **PR B**: 7 tasks → `unspecified-low` for infrastructure setup
- **PR C**: 6 tasks → `quick` for git, `unspecified-low` for documentation
- **Final**: 1 task → `quick`

---

## TODOs

### Gate 0: Hardware Testing (USER TASK - Do This First)

- [ ] 0. Test on STM32F469I-DISCO B08 (NT35510)

  **What to do**:
  - Connect board via ST-Link/USB
  - Run: `cargo run --release --example f469disco-lcd-test --features="stm32f469,defmt"`
  - Verify LCD shows color pattern
  - Verify touch toggles pattern
  - Verify no panic/hang for 60+ seconds

  **QA Scenarios**:
  ```
  Scenario: LCD initializes correctly
    Tool: Hardware observation
    Steps:
      1. Flash board with example
      2. Observe defmt output: "Detected LCD controller: Nt35510"
      3. Observe screen shows color bars
    Expected Result: Color pattern visible on screen
    Evidence: Photo/screenshot of display
  ```

  **Must NOT do**:
  - Do NOT modify any code before this test passes

  **Parallelization**:
  - Can Run In Parallel: NO - blocks all other tasks
  - Blocks: All PR A/B/C tasks
  - Blocked By: None

---

### PR A: HAL Improvements (Foundation)

- [ ] A1. Create backup branch

  **What to do**:
  - Run: `git branch NT35510-backup`
  - Run: `git branch NT35510-backup-$(date +%Y%m%d)`
  - Verify backup exists with `git branch`

  **QA Scenarios**:
  ```
  Scenario: Backup branch created
    Tool: Bash
    Steps:
      1. git branch | grep NT35510-backup
    Expected Result: Branch name appears in output
    Evidence: Terminal output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`
  - Skills: [] (git commands only)

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: A2-A7
  - Blocked By: Gate 0

- [ ] A2. Create PR A branch from master

  **What to do**:
  - Run: `git checkout master && git pull origin master`
  - Run: `git checkout -b hal-dsi-improvements`
  - This will be the branch for PR A

  **QA Scenarios**:
  ```
  Scenario: Branch created from master
    Tool: Bash
    Steps:
      1. git rev-parse --abbrev-ref HEAD
    Expected Result: "hal-dsi-improvements"
    Evidence: Terminal output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: A3-A7
  - Blocked By: A1

- [ ] A3. Cherry-pick DSI improvements from NT35510 branch

  **What to do**:
  - Identify commits that modified `src/dsi.rs`
  - Cherry-pick those commits to hal-dsi-improvements branch
  - Commits to look for:
    - "Implement missing DSI commands"
    - "Refactor DSI module docs"
    - Any DSI-related fixes

  **References**:
  - `src/dsi.rs` - DSI module with DsiHostCtrlIo impl

  **QA Scenarios**:
  ```
  Scenario: DSI improvements cherry-picked
    Tool: Bash
    Steps:
      1. git log --oneline | grep -i dsi
      2. cargo check --features="stm32f469,dsihost"
    Expected Result: DSI-related commits present, code compiles
    Evidence: git log output, cargo check exit code 0
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: A4
  - Blocked By: A2

- [ ] A4. Cherry-pick LTDC improvements from NT35510 branch

  **What to do**:
  - Identify commits that modified `src/ltdc.rs`
  - Cherry-pick the `new_dsi()` constructor addition
  - Ensure LTDC compiles with DSI feature

  **References**:
  - `src/ltdc.rs` - LTDC module with DSI constructor

  **QA Scenarios**:
  ```
  Scenario: LTDC improvements cherry-picked
    Tool: Bash
    Steps:
      1. git log --oneline | grep -i ltdc
      2. cargo check --features="stm32f469,ltdc"
    Expected Result: LTDC-related commits present, code compiles
    Evidence: git log output, cargo check exit code 0
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: A5
  - Blocked By: A3

- [ ] A5. Fix GenericShortP0/P1/P2 stub implementations in dsi.rs

  **What to do**:
  - Locate `GenericShortP0`, `GenericShortP1`, `GenericShortP2` in src/dsi.rs
  - Current stubs write (0, 0, discriminant) - need proper parameter passing
  - Either implement correctly or document as `todo!()` with clear note

  **References**:
  - `src/dsi.rs:630-638` - GenericShort stub implementations

  **QA Scenarios**:
  ```
  Scenario: GenericShort implementations fixed or documented
    Tool: Bash
    Steps:
      1. grep -n "GenericShort" src/dsi.rs
      2. cargo check --features="stm32f469,dsihost"
    Expected Result: No broken stub code, compiles successfully
    Evidence: grep output, cargo check exit code 0
  ```

  **Recommended Agent Profile**:
  - Category: `unspecified-low`
  - Skills: []

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: A6
  - Blocked By: A4

- [ ] A6. Squash commits to ~3 logical commits

  **What to do**:
  - Run interactive rebase: `git rebase -i master`
  - Squash all cherry-picked commits into 3 logical groups:
    1. DSI improvements
    2. LTDC improvements
    3. Documentation
  - Remove all "Initial plan" commits
  - Remove all merge commits

  **QA Scenarios**:
  ```
  Scenario: Clean commit history
    Tool: Bash
    Steps:
      1. git log --oneline master..HEAD | wc -l
      2. git log --oneline master..HEAD | grep -c "Initial plan"
      3. git log --oneline master..HEAD | grep -c "Merge pull"
    Expected Result: 3-5 commits, 0 "Initial plan", 0 merge commits
    Evidence: git log output
  ```

  **Must NOT do**:
  - Do NOT squash into single commit
  - Do NOT lose the logical separation of changes

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: A7
  - Blocked By: A5

- [ ] A7. Push PR A branch and verify CI

  **What to do**:
  - Run: `git push origin hal-dsi-improvements -u`
  - Create PR on GitHub (draft initially)
  - Wait for CI to pass
  - PR title: "feat(dsi): implement missing DCS commands and LTDC DSI constructor"

  **QA Scenarios**:
  ```
  Scenario: PR A created and CI passes
    Tool: Bash (gh CLI)
    Steps:
      1. gh pr list --head hal-dsi-improvements
      2. gh pr checks <pr-number>
    Expected Result: PR exists, all checks pass
    Evidence: gh output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: PR B and PR C tasks
  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: PR B and PR C tasks
  - Blocked By: A6

---

### PR B: Display Ecosystem (Infrastructure)

- [ ] B1. Create PR B branch from PR A

  **What to do**:
  - Run: `git checkout hal-dsi-improvements && git checkout -b display-ecosystem`
  - This branch will contain display module + boards + xtask

  **QA Scenarios**:
  ```
  Scenario: Branch created from PR A
    Tool: Bash
    Steps:
      1. git rev-parse --abbrev-ref HEAD
    Expected Result: "display-ecosystem"
    Evidence: Terminal output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: B2-B7
  - Blocked By: A7

- [ ] B2. Add src/display/ module

  **What to do**:
  - Copy `src/display/` directory from NT35510 branch
  - Include: framebuffer.rs, mod.rs, sdram.rs, spi.rs
  - Update src/lib.rs to export display module

  **References**:
  - `src/display/framebuffer.rs` - LtdcFramebuffer with DrawTarget impl
  - `src/display/spi.rs` - SPI display transport
  - `src/display/sdram.rs` - DisplaySdram helper

  **QA Scenarios**:
  ```
  Scenario: Display module compiles
    Tool: Bash
    Steps:
      1. cargo check --features="stm32f469,dsihost,ltdc,framebuffer,stm32-fmc"
    Expected Result: exit code 0
    Evidence: cargo output
  ```

  **Recommended Agent Profile**:
  - Category: `unspecified-low`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: B3
  - Blocked By: B1

- [ ] B3. Add boards/ crates

  **What to do**:
  - Copy `boards/` directory from NT35510 branch
  - Include: f469disco/, f429disco/, f413disco/
  - Note: f413disco and f429disco are UNTESTED - may want to exclude

  **References**:
  - `boards/f469disco/` - F469 board examples crate
  - `boards/f429disco/` - F429 board examples (untested)
  - `boards/f413disco/` - F413 board examples (untested)

  **QA Scenarios**:
  ```
  Scenario: Board crates exist
    Tool: Bash
    Steps:
      1. ls boards/
    Expected Result: f469disco, f429disco, f413disco directories present
    Evidence: ls output
  ```

  **Recommended Agent Profile**:
  - Category: `unspecified-low`

  **Parallelization**:
  - Can Run In Parallel: YES (with B4)
  - Blocks: B5
  - Blocked By: B2

- [ ] B4. Add xtask/ build tool

  **What to do**:
  - Copy `xtask/` directory from NT35510 branch
  - This is the build orchestration tool
  - Update README.md with board crate documentation

  **References**:
  - `xtask/src/main.rs` - Build orchestration logic

  **QA Scenarios**:
  ```
  Scenario: xtask compiles
    Tool: Bash
    Steps:
      1. cd xtask && cargo check
    Expected Result: exit code 0
    Evidence: cargo output
  ```

  **Recommended Agent Profile**:
  - Category: `unspecified-low`

  **Parallelization**:
  - Can Run In Parallel: YES (with B3)
  - Blocks: B5
  - Blocked By: B2

- [ ] B5. Add feature flags to Cargo.toml

  **What to do**:
  - Add `spi_display` feature with `dep:display-interface` dependency
  - Add `framebuffer` feature with `dep:embedded-graphics-core` dependency
  - Ensure features are properly gated

  **References**:
  - `Cargo.toml` lines 500-510 - Feature definitions

  **QA Scenarios**:
  ```
  Scenario: Feature flags work
    Tool: Bash
    Steps:
      1. cargo check --features="stm32f469,framebuffer"
    Expected Result: exit code 0
    Evidence: cargo output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: B6
  - Blocked By: B3, B4

- [ ] B6. Squash commits to ~4 logical commits

  **What to do**:
  - Run interactive rebase: `git rebase -i hal-dsi-improvements`
  - Squash into 4 logical groups:
    1. Display framebuffer module
    2. SPI display transport
    3. SDRAM helper
    4. xtask + board crates
  - Remove all "Initial plan" and merge commits

  **QA Scenarios**:
  ```
  Scenario: Clean commit history for PR B
    Tool: Bash
    Steps:
      1. git log --oneline hal-dsi-improvements..HEAD | wc -l
    Expected Result: 4-6 commits
    Evidence: git log output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: B7
  - Blocked By: B5

- [ ] B7. Push PR B branch and verify CI

  **What to do**:
  - Run: `git push origin display-ecosystem -u`
  - Create PR on GitHub (draft)
  - Wait for CI to pass
  - PR title: "feat(display): add framebuffer module, SPI transport, and board examples"

  **QA Scenarios**:
  ```
  Scenario: PR B created and CI passes
    Tool: Bash (gh CLI)
    Steps:
      1. gh pr list --head display-ecosystem
    Expected Result: PR exists
    Evidence: gh output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: None (PR B is optional dependency for PR C)
  - Blocked By: B6

---

### PR C: NT35510 Support (The Original Goal)

- [ ] C1. Create PR C branch from PR A

  **What to do**:
  - Run: `git checkout hal-dsi-improvements && git checkout -b nt35510-support`
  - This branch depends on PR A only (not PR B)

  **QA Scenarios**:
  ```
  Scenario: Branch created from PR A
    Tool: Bash
    Steps:
      1. git rev-parse --abbrev-ref HEAD
    Expected Result: "nt35510-support"
    Evidence: Terminal output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: C2-C6
  - Blocked By: A7

- [ ] C2. Add NT35510 driver

  **What to do**:
  - Copy `examples/f469disco/nt35510.rs` from NT35510 branch
  - Ensure it compiles with PR A's DSI improvements
  - The driver should be minimal (142 lines as-is)

  **References**:
  - `examples/f469disco/nt35510.rs` - NT35510 DSI driver

  **QA Scenarios**:
  ```
  Scenario: NT35510 driver compiles
    Tool: Bash
    Steps:
      1. cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
    Expected Result: exit code 0
    Evidence: cargo output
  ```

  **Recommended Agent Profile**:
  - Category: `unspecified-low`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: C3
  - Blocked By: C1

- [ ] C3. Add auto-detection example

  **What to do**:
  - Copy updated `examples/f469disco-lcd-test.rs` from NT35510 branch
  - The example should auto-detect NT35510 vs OTM8009A
  - Include touch support

  **References**:
  - `examples/f469disco-lcd-test.rs` - Main example with auto-detection

  **QA Scenarios**:
  ```
  Scenario: Example compiles
    Tool: Bash
    Steps:
      1. cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
    Expected Result: exit code 0
    Evidence: cargo output
  ```

  **Recommended Agent Profile**:
  - Category: `unspecified-low`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: C4
  - Blocked By: C2

- [ ] C4. Update CHANGELOG

  **What to do**:
  - Add entry under `[Unreleased]` section
  - Brief description of NT35510 support
  - Reference issue #842

  **References**:
  - `CHANGELOG.md` - Change log

  **QA Scenarios**:
  ```
  Scenario: CHANGELOG updated
    Tool: Bash
    Steps:
      1. grep -c "NT35510" CHANGELOG.md
    Expected Result: count > 0
    Evidence: grep output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: C5
  - Blocked By: C3

- [ ] C5. Squash commits to ~3 logical commits

  **What to do**:
  - Run interactive rebase: `git rebase -i hal-dsi-improvements`
  - Squash into 3 logical groups:
    1. NT35510 driver
    2. Auto-detection example
    3. CHANGELOG
  - Remove all "Initial plan" and merge commits

  **QA Scenarios**:
  ```
  Scenario: Clean commit history for PR C
    Tool: Bash
    Steps:
      1. git log --oneline hal-dsi-improvements..HEAD | wc -l
    Expected Result: 3-4 commits
    Evidence: git log output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: C6
  - Blocked By: C4

- [ ] C6. Push PR C branch and verify CI

  **What to do**:
  - Run: `git push origin nt35510-support -u`
  - Create PR on GitHub (draft initially)
  - Wait for CI to pass
  - PR title: "feat(f469disco): add NT35510 LCD controller support with auto-detection"
  - PR description should:
    - Explain the B08 vs B07 board difference
    - Document RDID1 detection approach
    - Note that hardware was tested

  **QA Scenarios**:
  ```
  Scenario: PR C created and CI passes
    Tool: Bash (gh CLI)
    Steps:
      1. gh pr list --head nt35510-support
    Expected Result: PR exists
    Evidence: gh output
  ```

  **Recommended Agent Profile**:
  - Category: `quick`

  **Parallelization**:
  - Can Run In Parallel: NO
  - Blocks: None
  - Blocked By: C5

---
## Commit Strategy

**PR A Commits** (3):
1. `feat(dsi): implement missing DCS commands and add DsiHostCtrlIo trait impl`
2. `feat(ltdc): add DSI-compatible constructor for video mode displays`
3. `docs(dsi): improve documentation and add usage examples`

**PR B Commits** (4):
1. `feat(display): add framebuffer module with embedded-graphics support`
2. `feat(display): add SPI display transport module`
3. `feat(display): add SDRAM helper for display framebuffers`
4. `feat(xtask): add board-specific example build orchestration`

**PR C Commits** (3):
1. `feat(example): add NT35510 LCD controller driver for F469I-DISCO B08`
2. `feat(example): add auto-detection for NT35510/OTM8009A controllers`
3. `docs: update CHANGELOG for NT35510 support`

---

## Success Criteria

### Verification Commands
```bash
# PR A compiles
cargo check --features="stm32f469,dsihost,ltdc"
# Expected: exit code 0

# PR B compiles with display features
cargo check --features="stm32f469,dsihost,ltdc,framebuffer,stm32-fmc"
# Expected: exit code 0

# PR C example compiles
cargo check --example f469disco-lcd-test --features="stm32f469,defmt"
# Expected: exit code 0

# No "Initial plan" commits
git log --oneline origin/pr-a-branch --not origin/master | grep -c "Initial plan"
# Expected: 0
```

### Final Checklist
- [ ] Hardware tested on B08 (NT35510)
- [ ] PR A: CI passes, 3 commits, focused scope
- [ ] PR B: CI passes, 4 commits, documented architecture
- [ ] PR C: CI passes, 3 commits, working example
- [ ] All PRs have clear descriptions
