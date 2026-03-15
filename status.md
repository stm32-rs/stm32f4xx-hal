# STM32F4xx HAL/BSP/VLS Integration Status

**Last Updated**: 2026-03-06

## Overview

BSP has been successfully separated from the HAL repository into its own standalone repository. All three projects (HAL, BSP, VLS) are updated and pushed to their respective remotes.

---

## Repository Status

### 1. HAL Repository (`stm32f4xx-hal`)

| Property | Value |
|----------|-------|
| **Remote** | `git@github.com:Amperstrand/stm32f4xx-hal.git` |
| **Branch** | `pr2-f469disco-examples` |
| **Head Commit** | `2581e30` |
| **Status** | ✅ Clean (pushed) |

**Recent Commits:**
```
2581e30 chore: remove BSP from HAL repo, Also adding to gitignore
45cfa4e feat(sdio): make set_bus public for Sdio<SdCard>
8239f21 fix(sdio): add software timeouts to block read/write loops
9c5b5ce docs(bsp): add USB guide, pin consumption docs, and CDC-ACM example
4dc2967 feat(bsp): add stm32f469i-disc board support package
```

**Key Changes:**
- BSP folder (`stm32f469i-disc/`) removed from git tracking
- `stm32f469i-disc/` added to `.gitignore`

---

### 2. BSP Repository (`stm32f469i-disc`)

| Property | Value |
|----------|-------|
| **Remote** | `git@github.com:Amperstrand/stm32f469i-disc.git` |
| **Branch** | `main` |
| **Head Commit** | `a53da07` |
| **Status** | ✅ Clean (pushed) |

**Recent Commits:**
```
a53da07 fix: reference HAL via git instead of local path
607014e feat: add sdio, button, usb modules; update for VLS integration
```

**Key Changes:**
- Now references HAL via git (`github.com/Amperstrand/stm32f4xx-hal`) at rev `2581e30`
- No longer uses relative path (`path = ".."`)

---

### 3. VLS Repository (`validating-lightning-signer`)

| Property | Value |
|----------|-------|
| **Remote** | `git@gitlab.com:Amperstrand1/validating-lightning-signer.git` |
| **Branch** | `stm32f469` |
| **Head Commit** | `bf08a1c1` |
| **MR** | [!835](https://gitlab.com/lightning-signer/validating-lightning-signer/-/merge_requests/835) |
| **Status** | ✅ Clean (pushed) |

**Recent Commits:**
```
bf08a1c1 fix(stm32): separate BSP into its own repository
5a41282b Merge branch validating-lightning-signer:main into stm32f469
5f211a80 docs(stm32): add BSP architecture plan and benchmark documentation
546c53aa test(stm32): add soak test and SD card benchmarks
ab03ec7e feat(stm32): add reliable SD card support for SDXC cards
```

**Dependency References:**
- HAL: `github.com/Amperstrand/stm32f4xx-hal` @ `2581e30`
- BSP: `github.com/Amperstrand/stm32f469i-disc` @ `a53da07`

---

## Dependency Graph

```
VLS (vls-signer-stm32)
├── stm32f4xx-hal @ 2581e30 (Amperstrand/stm32f4xx-hal)
└── stm32f469i-disc @ a53da07 (Amperstrand/stm32f469i-disc)
    └── stm32f4xx-hal @ 2581e30 (Amperstrand/stm32f4xx-hal)
```

---

## Build Verification

| Target | Status |
|--------|--------|
| `stm32f469` (default) | ✅ Compiles |
| `stm32f412` | ✅ Compiles |
| `stm32f413` | ✅ Compiles |

---

## Cleanup Items

### Local Only (not tracked)

The HAL working directory has many untracked documentation/notes files:

| Type | Files |
|------|-------|
| Feedback notes | `feedback_for_v*.md`, `feedback-all.md` |
| Documentation drafts | `BSP-*.md`, `*-GUIDE.md`, `research-*.md` |
| Test reports | `testing_report_*.md` |
| Reference repos | `STM32CubeF4/`, `stm32f429i-disc/`, `stm32f7xx-hal/` |
| VLS checkout | `validating-lightning-signer/` |
| Misc | `contributions/`, `nt35510/`, various `.md` and `.sh` files |

**Recommendation**: These are local working notes and don't need to be committed. Consider:
- Adding to `.gitignore` if persistent
- Or deleting if no longer needed

### VLS Cargo.lock

The `vls-signer-stm32/Cargo.lock` has local changes (auto-generated from build). This is expected behavior and doesn't require action unless you want to commit the updated lockfile.

---

## Next Steps for Upstream Submission

1. **Squash commits** in VLS MR !835 (optional, depending on upstream preference)
2. **Test on hardware** (STM32F469 Discovery board)
3. **Submit draft PR** to upstream VLS repository
4. **HAL PR**: The `pr2-f469disco-examples` branch may need separate upstream submission to `stm32-rs/stm32f4xx-hal`
5. **BSP PR**: Consider submitting BSP to `stm32-rs` organization (e.g., `stm32-rs/stm32f469i-disc`)

---

## Commit Reference Summary

| Repo | Commit | Description |
|------|--------|-------------|
| HAL | `2581e30` | BSP removed, added to .gitignore |
| BSP | `a53da07` | References HAL via git |
| VLS | `bf08a1c1` | Points to separate HAL and BSP repos |
