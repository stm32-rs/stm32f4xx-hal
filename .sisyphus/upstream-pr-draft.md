# Draft Merge Request: STM32F469 Discovery Board Support

## Direct URL to Create MR

1. Go to: https://gitlab.com/lightning-signer/validating-lightning-signer/-/merge_requests/new

2. Select:
   - **Source**: `Amperstrand1/validating-lightning-signer` / `stm32f469`
   - **Target**: `lightning-signer/validating-lightning-signer` / `main`

3. Mark as **Draft** (checkbox or prefix title with "Draft:")

---

## Title

```
Draft: feat(stm32): add STM32F469 Discovery board support
```

---

## Description

```markdown
## Summary

This PR adds support for the STM32F469I-DISCO board to the VLS signer, along with reliability improvements for SD card operations.

## Changes

### 1. Board Architecture (`a329b063`)
- Split `device.rs` into per-board modules (f412.rs, f413.rs, f469.rs)
- Add STM32F469I-DISCO board support with DSI display and SDRAM
- Implement narrow-waist `BoardIo` pattern for board-agnostic initialization
- Migrate F412/F413 from st7789 to mipidsi display driver
- Update to embedded-hal 1.0 and mipidsi 0.8

### 2. SD Card Support (`ab03ec7e`)
- Add `sdcard` module with block device adapter for embedded-sdmmc
- Implement GPT partition probing for SDXC cards
- Add FAT boot sector detection for partitionless cards
- SDIO clock optimization: 400KHz init → 1MHz data transfer
- Comprehensive SDIO documentation in source

### 3. Tests & Benchmarks (`546c53aa`)
- Add soak test binary for long-running stability testing
- Add SD card filesystem benchmarks
- Hardware-verified benchmark results in README

### 4. Documentation (`5f211a80`)
- Add BSP migration architecture plan
- Update README with build instructions and benchmarks

## Hardware Tested

- **Board**: STM32F469I-DISCO
- **SD Card**: 64GB SDXC, FAT32
- **Tests**: Both `test` and `bench` binaries verified
- **Results**:
  - Crypto: Sign 10.73ms, Verify 10.84ms, SHA256 0.05ms
  - SD Card: Write 104 KB/s, Read 328 KB/s @ 1MHz

## Dependencies

This PR depends on HAL and BSP changes published at:
- `stm32f4xx-hal` @ `45cfa4e` (https://github.com/Amperstrand/stm32f4xx-hal)
- `stm32f469i-disc` BSP (same repository)

## Notes

- F412 and F413 targets compile but are not hardware-tested (no hardware available)
- SDIO clock set to 1MHz for reliability (see `src/sdcard.rs` for rationale with references to SD spec)
- Draft PR for initial review and feedback

## Commits

1. `a329b063` feat(stm32): add F469 board support with modular device architecture
2. `ab03ec7e` feat(stm32): add reliable SD card support for SDXC cards
3. `546c53aa` test(stm32): add soak test and SD card benchmarks
4. `5f211a80` docs(stm32): add BSP architecture plan and benchmark documentation
```

---

## After Creating MR

Once you've created the MR on GitLab, you can authenticate `glab` for future use:

```bash
glab auth login
# Select: gitlab.com
# Select: HTTPS
# Paste your GitLab Personal Access Token (needs api scope)
```

Then you can manage the MR from CLI:
```bash
glab mr view
glab mr note "Ready for review"
```

---

## Quick Reference

| Item | Value |
|------|-------|
| Source Branch | `Amperstrand1/validating-lightning-signer:stm32f469` |
| Target Branch | `lightning-signer/validating-lightning-signer:main` |
| Commit Count | 4 |
| Latest Commit | `5f211a80` |
| Backup Branch | `backup/stm32f469-pre-squash` |
