# HAL/BSP Cleanup: Close PR #867, Push BSP, Clean Sisyphus

## TL;DR

> **Quick Summary**: Close PR #867 on GitHub (board examples belong in BSP), push the unpushed BSP commit, delete 11 stale sisyphus plans, close the boulder, and verify both HAL and BSP build cleanly. Optionally clean up pr2 branch.
> 
> **Deliverables**:
> - PR #867 closed on GitHub with explanatory comment
> - BSP commit ba99894 pushed to GitHub (stm32f469i-disc)
> - 11 stale sisyphus plans deleted, 4 kept
> - Boulder cleared (f412-f413-mipidsi-migration completed)
> - HAL verified building on pr1 branch
> - BSP verified building with all examples
> 
> **Estimated Effort**: Short (~30 min)
> **Parallel Execution**: YES — 3 waves + final verification
> **Critical Path**: Push BSP → Close PR → Clean sisyphus → Verify builds → Final review

---

## Context

### Original Request
User wants to clean up the stm32f4xx-hal fork so board-specific F469-Discovery code lives only in the stm32f469i-disc BSP, not in the HAL. The goal is a clean state where upstream stm32-rs would accept PR #866 (core DSI/LTDC drivers) and all board examples live in the BSP where they belong.

### Interview Summary
**Key Discussions**:
- PR #866 (pr1-core-dsi-ltdc) stays untouched — core DSI/LTDC driver, DRAFT on stm32-rs
- PR #867 (pr2-f469disco-examples) should be CLOSED — board examples belong in BSP
- The minimal f469disco-lcd-test.rs already exists on upstream/master (merged previously) — no loss from closing PR #867
- BSP already has full equivalents: lcd.rs (394 lines with NT35510/OTM8009A auto-detection), touch.rs, sdram.rs, 5 examples
- BSP has 1 unpushed commit (ba99894) that needs pushing
- 11 stale sisyphus plans to delete, 4 to keep
- Local master branch (10 commits ahead) is deferred — out of scope
- Verification: cargo check/build only, no hardware flash

**Research Findings**:
- stm32-rs convention: HALs contain chip-level peripheral drivers only; board-specific wiring goes in BSPs; panel drivers are external crates
- BSP already depends on `nt35510 = "0.1.0"` and `otm8009a = "0.1"` and uses them in lcd.rs for panel auto-detection
- PR #867 has zero comments and zero reviews — safe to close without losing feedback
- pr2 diff vs pr1: +11 files changed, +1960 insertions — ALL board-specific (examples, board.rs, images.rs, memory.x, nt35510 dev-dep)
- The f469disco-lcd-test.rs on upstream/master has `required-features = ["stm32f469", "defmt"]` — pr2 changed this to `["stm32f469", "stm32-fmc", "dsihost"]` but since we're abandoning pr2, the upstream version stays as-is

### Metis Review
**Identified Gaps** (addressed):
- Must explicitly defer local master cleanup (10 unpushed commits) — added to guardrails
- Need exact PR closure comment text — drafted in task
- BSP remote must be verified before push — added precondition check
- Boulder.json needs specific format for clearing — specified in task
- memory.x changed in pr2 (128K/32K → 2048K/320K) — no action needed since pr2 is being abandoned, upstream has correct values
- Should decide on pr2 branch cleanup (local + remote) — included as optional tasks

---

## Work Objectives

### Core Objective
Close PR #867, push the BSP to GitHub, clean up stale sisyphus state, and verify everything builds. Leave a clean, well-organized state across both repositories.

### Concrete Deliverables
- PR #867 state: CLOSED on GitHub
- BSP repository: fully pushed, `git status` shows "up to date"
- Sisyphus plans: 4 files remaining (upstream-merge.md, vls-f469-port.md, f412-f413-mipidsi-migration.md, hal-bsp-cleanup.md)
- Boulder: cleared/idle
- Build verification: both HAL and BSP compile without errors

### Definition of Done
- [ ] `gh pr view 867 --json state` → `"CLOSED"`
- [ ] `cd stm32f469i-disc && git status` → "Your branch is up to date with 'origin/main'"
- [ ] `ls .sisyphus/plans/ | wc -l` → `4` (after removing hal-bsp-cleanup.md too once complete)
- [ ] `cargo check --features stm32f469,dsihost` → exits 0 (on pr1 branch)
- [ ] `cd stm32f469i-disc && cargo build --examples` → exits 0

### Must Have
- PR #867 closed with explanatory comment about BSP
- BSP commit pushed to origin
- Stale sisyphus plans removed
- Both repos verified building

### Must NOT Have (Guardrails)
- MUST NOT touch PR #866 (pr1-core-dsi-ltdc) — no rebases, edits, or force pushes
- MUST NOT modify any code on pr1 or upstream/master branches
- MUST NOT push the 10 local master commits to origin (deferred to another session)
- MUST NOT attempt to "improve" or "fix" anything discovered during cleanup
- MUST NOT restructure BSP location (it lives nested in HAL dir — that's fine for now)
- MUST NOT add new features, examples, or code during this cleanup
- MUST NOT modify the existing upstream f469disco-lcd-test.rs
- MUST NOT delete f413disco-lcd-ferris.rs (different board, out of scope)

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (cargo check/build available)
- **Automated tests**: None — this is a cleanup task, not code changes
- **Framework**: cargo check / cargo build --examples
- **Approach**: Verify existing code compiles, not write new tests

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Git operations**: Use Bash — verify with git status/log/remote commands
- **GitHub operations**: Use Bash (gh CLI) — verify PR state
- **Build verification**: Use Bash (cargo) — verify exit codes and output
- **File operations**: Use Bash (ls/cat) — verify file presence/absence

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — independent operations):
├── Task 1: Push BSP commit to GitHub [quick]
├── Task 2: Close PR #867 on GitHub [quick]
├── Task 3: Delete 11 stale sisyphus plans [quick]
└── Task 4: Close boulder (clear boulder.json) [quick]

Wave 2 (After Wave 1 — build verification):
├── Task 5: Verify HAL builds on pr1 branch [quick]
└── Task 6: Verify BSP builds with all examples [quick]

Wave 3 (After Wave 2 — optional branch cleanup):
├── Task 7: Delete remote pr2 branch [quick]
└── Task 8: Delete or reset local pr2 branch [quick]

Wave FINAL (After ALL tasks — independent review):
├── Task F1: Plan compliance audit [quick]
└── Task F2: State verification [quick]

Critical Path: Task 1 → Task 6 (BSP must be pushed before build verify makes sense)
Parallel Speedup: Wave 1 all parallel, Wave 2 parallel, Wave 3 parallel
Max Concurrent: 4 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 (Push BSP) | — | 6 |
| 2 (Close PR) | — | 7 |
| 3 (Delete plans) | — | F1 |
| 4 (Close boulder) | — | F1 |
| 5 (Verify HAL) | — | F1 |
| 6 (Verify BSP) | 1 | F1 |
| 7 (Delete remote pr2) | 2 | 8 |
| 8 (Delete local pr2) | 7 | F1 |
| F1 (Compliance) | 1-6 | — |
| F2 (State verify) | 1-6 | — |

### Agent Dispatch Summary

- **Wave 1**: **4 parallel** — T1 `quick` + `git-master`, T2 `quick` + `git-master`, T3 `quick`, T4 `quick`
- **Wave 2**: **2 parallel** — T5 `quick`, T6 `quick`
- **Wave 3**: **2 parallel** — T7 `quick` + `git-master`, T8 `quick` + `git-master`
- **FINAL**: **2 parallel** — F1 `quick`, F2 `quick`

---

## TODOs


- [ ] 1. Push BSP commit to GitHub

  **What to do**:
  - Verify BSP remote is correct: `cd stm32f469i-disc && git remote -v` (expect `origin  git@github.com:Amperstrand/stm32f469i-disc.git`)
  - Verify no upstream changes: `git fetch origin && git status` (expect "ahead of origin/main by 1 commit")
  - Push: `git push origin main`
  - Verify: `git status` shows "Your branch is up to date with 'origin/main'"

  **Must NOT do**:
  - Do NOT force push
  - Do NOT rebase or amend the commit
  - Do NOT modify any files in the BSP before pushing

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Git push operation with pre-push verification

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Task 6 (BSP build verification)
  - **Blocked By**: None

  **References**:
  - `stm32f469i-disc/` — BSP repo root, nested inside HAL directory
  - Unpushed commit: `ba99894` — `feat(bsp): add LCD and touch initialization to stm32f469i-disc BSP`
  - Remote: `git@github.com:Amperstrand/stm32f469i-disc.git` (origin)

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: BSP push succeeds
    Tool: Bash
    Preconditions: Working directory is stm32f469i-disc/
    Steps:
      1. git remote -v → expect origin points to Amperstrand/stm32f469i-disc.git
      2. git fetch origin
      3. git status → expect "ahead of 'origin/main' by 1 commit"
      4. git push origin main → expect success (no rejection)
      5. git status → expect "Your branch is up to date with 'origin/main'"
    Expected Result: Push succeeds, branch is up to date
    Failure Indicators: Push rejected, authentication error, merge conflict
    Evidence: .sisyphus/evidence/task-1-bsp-push.txt

  Scenario: BSP push rejected (remote has new commits)
    Tool: Bash
    Preconditions: git fetch shows divergence
    Steps:
      1. If git status shows divergence after fetch, STOP and report
      2. Do NOT force push
    Expected Result: Task pauses and reports the divergence for human decision
    Evidence: .sisyphus/evidence/task-1-bsp-push-rejected.txt
  ```

  **Commit**: NO (git push only, no new commits)

- [ ] 2. Close PR #867 on GitHub with comment

  **What to do**:
  - Close PR #867 using gh CLI with explanatory comment
  - Exact command:
    ```bash
    gh pr close 867 --repo stm32-rs/stm32f4xx-hal --comment "Closing this PR — the board-specific examples and board module belong in the [stm32f469i-disc BSP](https://github.com/Amperstrand/stm32f469i-disc) rather than the HAL.

    The BSP already has equivalent (and more complete) implementations:
    - LCD initialization with NT35510/OTM8009A auto-detection
    - Touch support (ft6x06)
    - SDRAM configuration
    - 5 working examples

    The core DSI/LTDC driver changes remain in PR #866."
    ```
  - Verify the PR is closed: `gh pr view 867 --repo stm32-rs/stm32f4xx-hal --json state`

  **Must NOT do**:
  - Do NOT delete the PR (just close it)
  - Do NOT modify PR #866 in any way
  - Do NOT edit the PR description or title before closing

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: GitHub PR operations via gh CLI

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Task 7 (remote branch deletion)
  - **Blocked By**: None

  **References**:
  - PR #867: `feat(examples): F469-Disco display examples with panel auto-detection` — from `Amperstrand:pr2-f469disco-examples` → `stm32-rs/stm32f4xx-hal:master`
  - PR #866: `feat(dsi/ltdc): DSI write commands, LTDC DSI constructor, and framebuffer DrawTarget` — DO NOT TOUCH

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: PR #867 closed with comment
    Tool: Bash (gh CLI)
    Preconditions: gh is authenticated, PR #867 is currently OPEN or DRAFT
    Steps:
      1. gh pr view 867 --repo stm32-rs/stm32f4xx-hal --json state → confirm not already closed
      2. gh pr close 867 --repo stm32-rs/stm32f4xx-hal --comment "[comment text above]"
      3. gh pr view 867 --repo stm32-rs/stm32f4xx-hal --json state → expect {"state":"CLOSED"}
      4. gh pr view 866 --repo stm32-rs/stm32f4xx-hal --json state → expect still OPEN/DRAFT (not accidentally touched)
    Expected Result: PR #867 state is CLOSED, PR #866 unchanged
    Failure Indicators: gh auth error, PR already closed, wrong PR number
    Evidence: .sisyphus/evidence/task-2-pr-close.txt
  ```

  **Commit**: NO (GitHub API operation only)

---

- [ ] 3. Delete 11 stale sisyphus plans

  **What to do**:
  - Delete the following 11 plan files from `.sisyphus/plans/`:
    ```
    align-with-c-bsp.md
    embedded-graphics-hello-world.md
    f469-lcd-touch-stabilization.md
    f469disco-cleanup-verify.md
    fix-touch-panic.md
    nt35510-pr-improvements.md
    pr843-submission.md
    upgrade-touch-driver.md
    nt35510-crate-migration.md
    paint-touch-logging.md
    f469-bug-fixes.md
    ```
  - Keep these 4 plan files:
    ```
    upstream-merge.md          (resumable — upstream sync work)
    vls-f469-port.md           (resumable — VLS porting, separate scope)
    f412-f413-mipidsi-migration.md  (completed — boulder reference)
    hal-bsp-cleanup.md         (THIS plan — active)
    ```
  - Verify: `ls .sisyphus/plans/ | wc -l` → 4

  **Must NOT do**:
  - Do NOT delete upstream-merge.md, vls-f469-port.md, f412-f413-mipidsi-migration.md, or hal-bsp-cleanup.md
  - Do NOT modify the content of any kept plans
  - Do NOT create new plan files

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: F1 (compliance audit)
  - **Blocked By**: None

  **References**:
  - `.sisyphus/plans/` — directory containing all 15 plan files
  - All 11 files to delete have been triaged as either cancelled (marked with ❌ CANCELLED headers from 2026-02-27) or superseded by this plan

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Correct plans deleted, correct plans kept
    Tool: Bash
    Preconditions: .sisyphus/plans/ contains 15 files
    Steps:
      1. ls .sisyphus/plans/ → verify 15 files exist before deletion
      2. rm .sisyphus/plans/{align-with-c-bsp,embedded-graphics-hello-world,f469-lcd-touch-stabilization,f469disco-cleanup-verify,fix-touch-panic,nt35510-pr-improvements,pr843-submission,upgrade-touch-driver,nt35510-crate-migration,paint-touch-logging,f469-bug-fixes}.md
      3. ls .sisyphus/plans/ → expect exactly 4 files
      4. ls .sisyphus/plans/ → expect: f412-f413-mipidsi-migration.md, hal-bsp-cleanup.md, upstream-merge.md, vls-f469-port.md
    Expected Result: 11 files deleted, 4 files remain
    Failure Indicators: Wrong file count, missing kept file, file not found error
    Evidence: .sisyphus/evidence/task-3-plans-cleanup.txt

  Scenario: Accidentally deleting a kept plan
    Tool: Bash
    Preconditions: Before deletion
    Steps:
      1. Verify each of the 4 kept files exists BEFORE any deletion
      2. After deletion, verify each of the 4 kept files still exists
    Expected Result: All 4 kept files present before and after
    Evidence: .sisyphus/evidence/task-3-plans-kept-verify.txt
  ```

  **Commit**: YES
  - Message: `chore(sisyphus): delete 11 stale plans`
  - Files: `.sisyphus/plans/*.md` (11 deleted files)
  - Pre-commit: `ls .sisyphus/plans/ | wc -l` → 4

- [ ] 4. Close boulder (clear boulder.json)

  **What to do**:
  - Read current boulder.json to confirm it references f412-f413-mipidsi-migration
  - Update boulder.json to idle state: `{"active_plan": null, "status": "idle"}`
  - Verify: `cat .sisyphus/boulder.json` shows idle state

  **Must NOT do**:
  - Do NOT delete boulder.json (it should exist in idle state)
  - Do NOT modify any plan files

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: F1 (compliance audit)
  - **Blocked By**: None

  **References**:
  - `.sisyphus/boulder.json` — current content references `f412-f413-mipidsi-migration` with status `in_progress`
  - This boulder's plan (f412-f413-mipidsi-migration.md) has all tasks completed — it should be closed

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Boulder cleared to idle
    Tool: Bash
    Preconditions: boulder.json exists with active plan
    Steps:
      1. cat .sisyphus/boulder.json → confirm it currently references f412-f413-mipidsi-migration
      2. Write new content: {"active_plan": null, "status": "idle"}
      3. cat .sisyphus/boulder.json → expect null active_plan and idle status
    Expected Result: Boulder is in idle state
    Failure Indicators: boulder.json doesn't exist, content not valid JSON
    Evidence: .sisyphus/evidence/task-4-boulder-clear.txt
  ```

  **Commit**: YES (grouped with Task 3)
  - Message: `chore(sisyphus): delete 11 stale plans, close boulder`
  - Files: `.sisyphus/boulder.json`

- [ ] 5. Verify HAL builds on pr1 branch

  **What to do**:
  - Checkout pr1-core-dsi-ltdc branch
  - Run `cargo check --features stm32f469,dsihost`
  - Verify exit code is 0
  - Return to pr2 branch afterward (or leave on pr1 — doesn't matter since pr2 is abandoned)

  **Must NOT do**:
  - Do NOT modify any files on pr1
  - Do NOT commit anything to pr1
  - Do NOT run cargo build (check is sufficient and faster)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 6)
  - **Blocks**: F1 (compliance audit)
  - **Blocked By**: None (pr1 is independent of BSP push)

  **References**:
  - Branch `pr1-core-dsi-ltdc` — contains 2 commits on top of upstream/master: DSI write commands + LTDC DSI constructor
  - `src/dsi.rs` — DSI peripheral driver
  - `src/ltdc.rs` — LTDC peripheral driver with DSI constructor and framebuffer support

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: HAL compiles on pr1 branch
    Tool: Bash
    Preconditions: stm32f4xx-hal repo root
    Steps:
      1. git checkout pr1-core-dsi-ltdc
      2. cargo check --features stm32f469,dsihost 2>&1
      3. echo $? → expect 0
    Expected Result: cargo check exits with 0, no errors
    Failure Indicators: Compilation errors, missing features, unresolved imports
    Evidence: .sisyphus/evidence/task-5-hal-build.txt
  ```

  **Commit**: NO (verification only)

- [ ] 6. Verify BSP builds with all examples

  **What to do**:
  - Change to BSP directory: `cd stm32f469i-disc`
  - Run `cargo build --examples`
  - Verify exit code is 0
  - All 5 examples should compile: display_dsi_lcd, display_hello_eg, display_touch, fmc_sdram_test, gpio_hal_blinky

  **Must NOT do**:
  - Do NOT modify any BSP files
  - Do NOT run on hardware (cargo build only)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 5)
  - **Blocks**: F1 (compliance audit)
  - **Blocked By**: Task 1 (BSP must be pushed first, to confirm no push issues affect build)

  **References**:
  - `stm32f469i-disc/` — BSP repo root
  - `stm32f469i-disc/Cargo.toml` — BSP dependencies (uses HAL via `path = ".."`, so HAL code matters)
  - `stm32f469i-disc/examples/` — 5 example files
  - BSP depends on HAL via path dependency — if HAL branch changes, BSP build may break. Since we're not changing HAL code, this should be fine.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: BSP compiles all examples
    Tool: Bash
    Preconditions: stm32f469i-disc/ directory, HAL branch has the code BSP depends on
    Steps:
      1. cd stm32f469i-disc
      2. cargo build --examples 2>&1
      3. echo $? → expect 0
    Expected Result: All 5 examples compile without errors
    Failure Indicators: Compilation errors, missing dependencies, linker errors
    Evidence: .sisyphus/evidence/task-6-bsp-build.txt

  Scenario: BSP path dependency resolves correctly
    Tool: Bash
    Preconditions: HAL must be on a branch that has DSI/LTDC support
    Steps:
      1. grep 'path = ".."' stm32f469i-disc/Cargo.toml → confirm HAL is a path dep
      2. Note which HAL branch is checked out — BSP build depends on it
      3. If HAL is on pr1 (from Task 5), BSP should still build since pr1 has DSI/LTDC
    Expected Result: Path dependency resolves and BSP builds
    Evidence: .sisyphus/evidence/task-6-bsp-path-dep.txt
  ```

  **Commit**: NO (verification only)

- [ ] 7. Delete remote pr2 branch (optional but recommended)

  **What to do**:
  - Delete the remote branch: `git push origin --delete pr2-f469disco-examples`
  - Verify: `git branch -r | grep pr2` returns nothing

  **Must NOT do**:
  - Do NOT delete any other remote branches
  - Do NOT delete pr1-core-dsi-ltdc remote branch
  - Only proceed if Task 2 (PR closure) completed successfully

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Remote branch management

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 8)
  - **Blocks**: Task 8 (local branch deletion)
  - **Blocked By**: Task 2 (PR must be closed first)

  **References**:
  - Remote branch: `origin/pr2-f469disco-examples` on `github.com:Amperstrand/stm32f4xx-hal.git`
  - PR #867 must be CLOSED before branch deletion

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Remote pr2 branch deleted
    Tool: Bash
    Preconditions: PR #867 is CLOSED (Task 2 complete)
    Steps:
      1. git branch -r | grep pr2 → confirm remote branch exists
      2. git push origin --delete pr2-f469disco-examples
      3. git branch -r | grep pr2 → expect no output
    Expected Result: Remote branch no longer exists
    Failure Indicators: Permission denied, branch not found
    Evidence: .sisyphus/evidence/task-7-remote-branch-delete.txt
  ```

  **Commit**: NO (remote operation only)

- [ ] 8. Delete local pr2 branch

  **What to do**:
  - Switch to pr1 branch first: `git checkout pr1-core-dsi-ltdc`
  - Delete local pr2 branch: `git branch -D pr2-f469disco-examples`
  - Verify: `git branch | grep pr2` returns nothing

  **Must NOT do**:
  - Do NOT delete pr1-core-dsi-ltdc local branch
  - Do NOT delete master branch
  - Make sure you're NOT on pr2 when deleting it

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]
    - `git-master`: Local branch management

  **Parallelization**:
  - **Can Run In Parallel**: NO (depends on Task 7)
  - **Parallel Group**: Wave 3 (after Task 7)
  - **Blocks**: F1 (compliance audit)
  - **Blocked By**: Task 7 (delete remote first to avoid confusion)

  **References**:
  - Local branch: `pr2-f469disco-examples` — currently checked out, has 1 commit on top of pr1
  - Must checkout different branch before deletion

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Local pr2 branch deleted
    Tool: Bash
    Preconditions: Not on pr2 branch, Task 7 complete
    Steps:
      1. git checkout pr1-core-dsi-ltdc
      2. git branch | grep pr2 → confirm local branch exists
      3. git branch -D pr2-f469disco-examples
      4. git branch | grep pr2 → expect no output
      5. git branch --show-current → expect pr1-core-dsi-ltdc
    Expected Result: Local pr2 branch deleted, now on pr1
    Failure Indicators: Cannot delete checked-out branch, branch not found
    Evidence: .sisyphus/evidence/task-8-local-branch-delete.txt
  ```

  **Commit**: NO (branch operation only)

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 2 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit**
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": verify it wasn't violated. Check evidence files exist in .sisyphus/evidence/.
  - **Recommended Agent Profile**: Category `quick`, Skills: []
  - **Parallelization**: Can run in parallel with F2. Blocked by Tasks 1-6.
  - Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **State Verification**
  Verify final state of all repositories: BSP pushed and up-to-date, PR #867 closed, sisyphus plans count correct, boulder cleared, HAL and BSP build clean.
  - **Recommended Agent Profile**: Category `quick`, Skills: [`git-master`]
  - **Parallelization**: Can run in parallel with F1. Blocked by Tasks 1-6.
  - Output: `BSP [PUSHED/NOT] | PR867 [CLOSED/OPEN] | Plans [N files] | Boulder [IDLE/ACTIVE] | HAL Build [PASS/FAIL] | BSP Build [PASS/FAIL] | VERDICT`

---

## Commit Strategy

No commits needed — this is a cleanup of GitHub state, sisyphus files, and build verification. No source code changes.

If sisyphus plan deletion should be committed:
- **Message**: `chore(sisyphus): delete 11 stale plans, close boulder`
- **Files**: `.sisyphus/plans/*.md` (11 deleted), `.sisyphus/boulder.json` (updated)

---

## Success Criteria

### Verification Commands
```bash
# PR #867 is closed
gh pr view 867 --json state  # Expected: {"state":"CLOSED"}

# BSP is pushed
cd stm32f469i-disc && git status  # Expected: "Your branch is up to date with 'origin/main'"

# Sisyphus plans cleaned (4 remain, including this plan)
ls .sisyphus/plans/ | wc -l  # Expected: 4

# Boulder is cleared
cat .sisyphus/boulder.json  # Expected: idle/null active plan

# HAL builds
git checkout pr1-core-dsi-ltdc && cargo check --features stm32f469,dsihost  # Expected: exit 0

# BSP builds
cd stm32f469i-disc && cargo build --examples  # Expected: exit 0
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] PR #866 completely untouched
- [ ] No code changes made anywhere
- [ ] Local master still 10 commits ahead (deferred, not touched)
