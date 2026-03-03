## F469 Display Stack Cleanup Plan

### Goal
Separate generic MCU HAL functionality from board-specific BSP functionality, and keep touch/display dependencies robust for real hardware.

### Layering Rules
1. `stm32f4xx-hal` keeps only generic, reusable peripheral abstractions.
2. `stm32f469i-disc` keeps board wiring, panel detection policy, and on-board component setup.
3. External display/touch controller crates stay external (`nt35510`, `otm8009a`, `ft6x06`).

### Scope by Crate
#### `stm32f4xx-hal` (upstream-targeted)
- Keep DSI host primitives and LTDC framebuffer APIs.
- Keep generic display-controller integration points.
- Remove or avoid board-specific init flows and panel-specific policy.

#### `stm32f469i-disc` (board support)
- Own pin mappings and bring-up helpers for LCD/touch.
- Own NT35510 vs OTM8009A runtime detection policy.
- Own FT6X06 init and touch convenience APIs.

### Branch Strategy
1. Use `pr1-core-dsi-ltdc` as HAL upstream base (generic only).
2. Keep board integration work in BSP-focused branches (`pr2-f469disco-examples` and follow-ups).
3. Split future PRs by concern:
   - PR A: HAL generic DSI/LTDC API changes.
   - PR B: BSP board wiring and examples.
   - PR C: touch robustness and diagnostics.

### FT6X06 Stability Requirement
The FT6X06 driver can panic on spurious multi-touch values without the PR #5 fix. The repository must use commit `cc352f80b12fd985da4c4847771a26ebc03ece62` so `detect_touch` clamps count instead of asserting.

### Execution Checklist
1. Pin `ft6x06` directly to commit `cc352f80b12fd985da4c4847771a26ebc03ece62` in each standalone STM32F469 consumer manifest.
2. Keep touch-driver handling in BSP (`stm32f469i-disc`), not HAL core.
3. Ensure HAL F469 examples use BSP display helpers rather than local board modules.
4. Verify BSP + F469 examples build with the pinned touch dependency.
5. Keep HAL PRs free of board-specific logic.

### Done Criteria
- HAL branch diff contains only generic peripheral functionality.
- BSP branch diff contains board-specific display/touch policy.
- FT6X06 panic path is neutralized via commit-pinned dependency in active manifests.
