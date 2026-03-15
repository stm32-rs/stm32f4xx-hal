# Upstream PR Preparation: Squash, Test, Submit

## TL;DR

> **Quick Summary**: Prepare the VLS `stm32f469` branch for submission as a draft PR to upstream (`gitlab.com/lightning-signer/validating-lightning-signer`). Squash 20 commits into ~4 clean, logical, reviewer-friendly commits, re-test on hardware, then submit.

> **Deliverables**:
> - Clean commit history with ~4 squashed commits
> - All three targets compile (f412, f413, f469)
> - Hardware-tested on STM32F469 Discovery (test + bench binaries)
> - Draft PR submitted to upstream VLS repository

> **Estimated Effort**: Medium
> **Parallel Execution**: Limited (sequential git operations)
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 4 → Task 5

---

## Context

### Original Request
Prepare the VLS `stm32f469` branch for upstream submission. The branch has 19 commits that need to be organized into logical groups for easier review. The end goal is a draft PR that upstream maintainers can review.

### Repository State

| Repo | Location | Remote | Branch |
|------|----------|--------|--------|
| **HAL** | `/Users/macbook/src/stm32f4xx-hal/` | `github.com/Amperstrand/stm32f4xx-hal` | `pr2-f469disco-examples` |
| **VLS** | `/Users/macbook/src/stm32f4xx-hal/validating-lightning-signer/` | `gitlab.com/Amperstrand1/validating-lightning-signer` | `stm32f469` |
| **Upstream VLS** | (remote only) | `gitlab.com/lightning-signer/validating-lightning-signer` | `main` |

### Current Branch State

```
                     upstream/main (c54d3e3e) ← both branches fork from here
                              │
                              │
                        stm32f469 (19 commits)
                        6a97c885 [CLEAN - ready to squash]
```

- `upstream/main` is at `c54d3e3e`
- `stm32f469` has 19 commits ahead of upstream, 0 behind
- **stm32f469 is already cleanly on top of upstream/main** → no rebase needed, just squash

### 19 Commits to Squash

| Commit | Message | Group |
|--------|---------|-------|
| `c7efce1f` | refactor(stm32): split device.rs into modular per-board architecture | **A: Board Architecture** |
| `a8f74431` | refactor(stm32): migrate F412/F413 from st7789 to mipidsi, fix RAM_SIZE | **A: Board Architecture** |
| `9bfd6bf0` | chore(deps): update embedded-hal to 1.0, graphics to 0.8, remove hal-02 alias | **A: Board Architecture** |
| `5bbaf419` | feat(stm32): add STM32F469 Discovery board support | **A: Board Architecture** |
| `ee1eb364` | fix: update to mipidsi 0.8 API to embedded-hal 1.0 | **A: Board Architecture** |
| `e685f700` | fix(stm32): correct mipidsi 0.8 API usage for F412/F413 | **A: Board Architecture** |
| `e2a89696` | refactor(stm32): compute UI layout from screen dimensions | **A: Board Architecture** |
| `01df07d8` | fix(stm32): patch ft6x06 to prevent multi-touch panic | **A: Board Architecture** |
| `e360ea07` | fix(stm32): improve SDXC card compatibility | **B: SD Card Reliability** |
| `d974fea1` | fix(stm32): require SD card, add FAT probe and GPT partition support | **B: SD Card Reliability** |
| `3a6f347e` | feat(stm32): add GPT partition support for SDXC cards | **B: SD Card Reliability** |
| `a7bdd817` | fix(stm32): use 400KHz SDIO clock for SDXC card compatibility | **B: SD Card Reliability** |
| `de140275` | feat(sdio): switch to 1MHz for SDXC data transfer | **B: SD Card Reliability** |
| `1cfa093f` | fix(stm32): add missing sdcard functions and Button import | **B: SD Card Reliability** |
| `2707c7d4` | fix(stm32): add Button import, fix README fallback language | **B: SD Card Reliability** |
| `5394e735` | test(stm32): add long-run soak test binary | **C: Tests & Benchmarks** |
| `6a97c885` | feat(bench): add SD card filesystem benchmarks | **C: Tests & Benchmarks** |
| `e7ac9ee5` | docs(stm32): add BSP migration architecture plan | **D: Documentation** |
| `009c2087` | docs(sdcard): add comprehensive SDIO clock speed documentation | **D: Documentation** |

### Dirty Working Tree
- `vls-signer-stm32/README.md` — benchmark number corrections (11 lines)
  - Crypto times were wrong: showed 0.35ms instead of actual 10.73ms
  - SD card write speeds corrected: 328 KB/s → 104 KB/s

### HAL/BSP Dependency
VLS `Cargo.toml` pins to:
- `stm32f4xx-hal` → `rev = "45cfa4e700d0e36adda089ecff93b424e24cef68"` on `github.com/Amperstrand/stm32f4xx-hal`
- `stm32f469i-disc` → same repo, same rev

This commit includes the `set_bus()` public API needed for 1MHz clock switch.

### Hardware Test Results (verified on STM32F469 Discovery)

**Crypto:**
- Sign ECDSA: 10.73ms/op
- Verify ECDSA: 10.84ms/op
- SHA256: 0.05ms/op
- Secp256k1 Create: 0.07ms/op

**SD Card (1MHz, FAT32, 64GB SDXC):**
- Write: 104-107 KB/s
- Read: 328 KB/s

---

## Work Objectives

### Core Objective
Transform 19+1 commits into a clean, reviewer-friendly commit history ready for upstream submission.

### Concrete Deliverables
- Squashed branch with ~4 logical commits
- All three targets compile cleanly
- Hardware test passed on STM32F469 Discovery (test + bench)
- Draft PR submitted to upstream

### Definition of Done
- [ ] Dirty README committed
- [ ] 20 commits squashed into ~4 logical commits
- [ ] Backup branch created before squash
- [ ] All three targets compile: f412, f413, f469
- [ ] `test` binary runs on hardware
- [ ] `bench` binary runs on hardware
- [ ] Force-pushed to origin
- [ ] Draft PR created on GitLab

### Must Have
- Clean commit messages following conventional commits
- Logical grouping (architecture, SD card, tests, docs)
- Hardware verification before submission
- HAL/BSP pinned to published git commits

### Must NOT Have (Guardrails)
- No rebase onto upstream/main (already on top)
- No changes to HAL repo (already published at 45cfa4e)
- No changes to Cargo.toml dependencies (already pinned correctly)
- No skipping hardware test
- No force-push without backup branch

---

## Squash Plan

### Target Commit Structure

| # | Squashed Commit | Source Commits (oldest→newest) | Description |
|---|-----------------|-------------------------------|-------------|
| **A** | `feat(stm32): add F469 board support with modular device architecture` | c7efce1f, a8f74431, 9bfd6bf0, 5bbaf419, ee1eb364, e685f700, e2a89696, 01df07d8 | Split device.rs, add F469 support, mipidsi migration, touch fix |
| **B** | `feat(stm32): add reliable SD card support for SDXC cards` | e360ea07, d974fea1, 3a6f347e, a7bdd817, de140275, 1cfa093f, 2707c7d4 | GPT/FAT probing, SDXC clock optimization, 1MHz data transfer |
| **C** | `test(stm32): add soak test and SD card benchmarks` | 5394e735, 6a97c885 | Soak test binary, SD card benchmarks |
| **D** | `docs(stm32): add BSP architecture plan and SDIO documentation` | e7ac9ee5, 009c2087, + dirty README | BSP migration plan, SDIO clock docs, benchmark results |

### Commit Messages (Draft)

**Commit A:**
```
feat(stm32): add F469 board support with modular device architecture

- Split device.rs into per-board modules (f412.rs, f413.rs, f469.rs)
- Add STM32F469I-DISCO board support with DSI display
- Migrate F412/F413 from st7789 to mipidsi driver
- Update to embedded-hal 1.0 and mipidsi 0.8
- Fix ft6x06 multi-touch panic
- Compute UI layout from screen dimensions
```

**Commit B:**
```
feat(stm32): add reliable SD card support for SDXC cards

- Add GPT partition probing for SDXC cards
- Implement FAT boot sector detection
- Use 400KHz SDIO clock for init (SD spec compliance)
- Switch to 1MHz for data transfer (reliability optimization)
- Add sdcard module with block device adapter
```

**Commit C:**
```
test(stm32): add soak test and SD card benchmarks

- Add long-run soak test binary for stability testing
- Add SD card filesystem benchmarks (read/write/seek)
- Document benchmark results in README
```

**Commit D:**
```
docs(stm32): add BSP architecture plan and SDIO documentation

- Add BSP migration architecture plan document
- Add comprehensive SDIO clock speed documentation
- Document hardware-verified benchmark results
```

---

## Execution Strategy

### Wave 1: Pre-Squash Preparation
```
├── Task 1: Commit dirty README fix [quick]
├── Task 2: Create backup branch [quick]
```

### Wave 2: Squash
```
├── Task 3: Interactive rebase to squash commits [deep]
```

### Wave 3: Verification
```
├── Task 4: Compile all three targets [unspecified-high]
├── Task 5: Hardware test on STM32F469 [unspecified-high]
```

### Wave 4: Submit
```
├── Task 6: Force-push to origin [quick]
├── Task 7: Create draft PR [quick]
```

---

## TODOs

- [x] 1. Commit dirty README fix

  **What to do**:
  - Stage and commit the benchmark number corrections in `vls-signer-stm32/README.md`
  - The corrections fix crypto times (0.35ms → 10.73ms) and SD write speeds (328 KB/s → 104 KB/s)

  **Commands**:
  ```bash
  cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
  git add vls-signer-stm32/README.md
  git commit -m "docs(stm32): correct benchmark numbers in README"
  ```

  **Acceptance Criteria**:
  - [ ] `git status` shows clean working tree
  - [ ] Commit created with message "docs(stm32): correct benchmark numbers in README"

  **Commit**: YES
  - Message: `docs(stm32): correct benchmark numbers in README`
  - Files: `vls-signer-stm32/README.md`

- [x] 2. Create backup branch

  **What to do**:
  - Create a safety backup branch before squash operation
  - This allows recovery if squash goes wrong

  **Commands**:
  ```bash
  cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
  git branch backup/stm32f469-pre-squash
  git push origin backup/stm32f469-pre-squash
  ```

  **Acceptance Criteria**:
  - [ ] Branch `backup/stm32f469-pre-squash` exists locally
  - [ ] Branch pushed to origin

  **Commit**: NO (branch operation only)

- [x] 3. Squash 20 commits into ~4 logical commits

  **What to do**:
  - Use interactive rebase to squash commits into 4 logical groups
  - Follow the squash plan in the "Squash Plan" section above
  - Rewrite commit messages to be reviewer-friendly

  **Process**:
  ```bash
  cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
  git rebase -i upstream/main
  ```

  In the editor:
  1. Mark commits for squash according to the plan
  2. Edit commit messages to match the draft messages above
  3. Save and complete rebase

  **Squash Sequence** (from oldest to newest):
  - Commits c7efce1f through 01df07d8 → **pick c7efce1f, squash rest** → Commit A
  - Commits e360ea07 through 2707c7d4 → **pick e360ea07, squash rest** → Commit B
  - Commits 5394e735, 6a97c885 → **pick 5394e735, squash 6a97c885** → Commit C
  - Commits e7ac9ee5, 009c2087, +dirty README commit → **pick e7ac9ee5, squash rest** → Commit D

  **Acceptance Criteria**:
  - [ ] Branch has exactly 4 commits
  - [ ] Commit messages follow conventional commits format
  - [ ] `git log --oneline upstream/main..HEAD` shows 4 commits

  **Commit**: YES (rebase rewrites history)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires careful git history manipulation with specific commit ordering
  - **Skills**: [] (git operations are manual, not delegated)

- [x] 4. Compile all three targets

  **What to do**:
  - Verify all three board targets compile after squash
  - This catches any issues from the rebase

  **Commands**:
  ```bash
  cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
  cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf
  cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf
  cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf
  ```

  **Acceptance Criteria**:
  - [ ] F412 target compiles with exit code 0
  - [ ] F413 target compiles with exit code 0
  - [ ] F469 target compiles with exit code 0

  **Commit**: NO (verification only)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Compilation verification for embedded targets
  - **Skills**: []

- [x] 5. Hardware test on STM32F469 Discovery

  **What to do**:
  - Flash and run both `test` and `bench` binaries on the hardware
  - Verify SD card operations work correctly
  - Verify crypto benchmarks produce expected results

  **Remote Board**: `ssh ubuntu@192.168.13.246`

  **Commands**:
  ```bash
  # On remote machine
  cd ~/src/validating-lightning-signer-stm32f469/vls-signer-stm32

  # Copy binaries from local (after local build)
  # Build on local machine first, then:

  # Flash and run test binary
  ~/.cargo/bin/probe-rs run --chip STM32F469NIHx target/thumbv7em-none-eabihf/release/test

  # Flash and run bench binary
  ~/.cargo/bin/probe-rs run --chip STM32F469NIHx target/thumbv7em-none-eabihf/release/bench
  ```

  **Acceptance Criteria**:
  - [ ] `test` binary runs without errors
  - [ ] `bench` binary produces expected benchmark numbers
  - [ ] SD card operations succeed (read/write)

  **Commit**: NO (verification only)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Hardware testing requires careful execution
  - **Skills**: []

- [x] 6. Force-push to origin

  **What to do**:
  - Force-push the squashed branch to origin
  - This updates the remote with the clean commit history

  **Commands**:
  ```bash
  cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
  git push --force-with-lease origin stm32f469
  ```

  **Acceptance Criteria**:
  - [ ] Force-push succeeds
  - [ ] `git log origin/stm32f469 --oneline | head -4` shows 4 commits

  **Commit**: NO (push operation only)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single git push command
  - **Skills**: []

- [x] 7. Create draft PR

  **What to do**:
  - Create a draft PR on GitLab targeting upstream main
  - Use `gh` CLI or GitLab web interface

  **PR Title**: `feat(stm32): add STM32F469 Discovery board support`

  **PR Description** (draft):
  ```markdown
  ## Summary

  This PR adds support for the STM32F469I-DISCO board to the VLS signer, along with reliability improvements for SD card operations.

  ## Changes

  - **Board Architecture**: Modular device abstraction with per-board modules (f412, f413, f469)
  - **SD Card Support**: Reliable SDXC card support with GPT partition probing and FAT filesystem
  - **Tests & Benchmarks**: Soak test and SD card performance benchmarks
  - **Documentation**: BSP architecture plan and SDIO clock documentation

  ## Hardware Tested

  - STM32F469I-DISCO with 64GB SDXC card
  - Both `test` and `bench` binaries verified

  ## Dependencies

  This PR depends on HAL and BSP changes published at:
  - `github.com/Amperstrand/stm32f4xx-hal` @ `45cfa4e`
  - `stm32f469i-disc` BSP (same repo)

  ## Notes

  - F412 and F413 targets compile but are not hardware-tested (no hardware available)
  - SDIO clock set to 1MHz for reliability (see `src/sdcard.rs` for rationale)
  ```

  **Commands**:
  ```bash
  # Using glab CLI (if available)
  cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
  glab mr create --draft --title "feat(stm32): add STM32F469 Discovery board support" \
    --description "$(cat <<'EOF'
  [PR description above]
  EOF
  )" \
    --target-branch main \
    --source-branch stm32f469 \
    --target-repo lightning-signer/validating-lightning-signer
  ```

  **Acceptance Criteria**:
  - [ ] Draft PR created on GitLab
  - [ ] PR targets `lightning-signer/validating-lightning-signer:main`
  - [ ] PR is marked as draft

  **Commit**: NO (PR creation only)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single CLI command or web form
  - **Skills**: []

---

## Success Criteria

### Final Checklist
- [ ] All "Must Have" items present
- [ ] All "Must NOT Have" items avoided
- [ ] 4 clean commits on stm32f469 branch
- [ ] All three targets compile
- [ ] Hardware test passed
- [ ] Draft PR submitted

### Verification Commands
```bash
# Check commit count
cd /Users/macbook/src/stm32f4xx-hal/validating-lightning-signer
git log --oneline upstream/main..HEAD | wc -l  # Expected: 4

# Check each target compiles
cargo build --release -p vls-signer-stm32 --features stm32f412 --target thumbv7em-none-eabihf
cargo build --release -p vls-signer-stm32 --features stm32f413 --target thumbv7em-none-eabihf
cargo build --release -p vls-signer-stm32 --features stm32f469 --target thumbv7em-none-eabihf

# Check backup exists
git branch -l 'backup/stm32f469-pre-squash'
```
