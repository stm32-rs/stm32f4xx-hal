## 2026-02-27 - Task 8

- Used a fresh branch from `upstream/master` (`pr1-core-dsi-ltdc`) and manual file application instead of cherry-pick to avoid importing noisy fixup history.
- Kept PR1 as exactly four files (`Cargo.toml`, `CHANGELOG.md`, `src/dsi.rs`, `src/ltdc.rs`) to preserve clean upstream review boundaries.
- Chose two commits (DSI-focused + LTDC/Cargo/CHANGELOG-focused) to keep commits logically atomic while avoiding splitting `Cargo.toml` across commits.
