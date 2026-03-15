## Notepad: nt35510-crate-migration

### Learnings
(None yet)

### Decisions
- Branch: NT35510 (user chose, not pr2)
- Start with lcd-test (self-contained, no board.rs)
- Hardware test only lcd-test + hello-eg (covers both code paths)
- defmt panic: drop error from message (external Error lacks defmt::Format)

### Issues
(None yet)

### Problems
(None yet)

## Task 1: f469disco-lcd-test migration (completed 2026-02-27)

### Disambiguation issues with dual trait bounds
- `&mut impl TraitA + TraitB` requires parens: `&mut (impl TraitA + TraitB)` 
- When both `embedded-hal-0.2` and `embedded-hal-1.0` are in scope, `delay_us` is ambiguous
- Solution: use fully-qualified syntax: `embedded_hal_02::blocking::delay::DelayUs::<u32>::delay_us(delay, value)`

### external crate `nt35510 = "0.1.0"` API differences from local module
- Error type is `nt35510::Error` (not `Nt35510Error`)
- Variants: `DsiRead`, `DsiWrite`, `ProbeMismatch(u8)`, `InvalidDimensions`
- `probe()` returns `Result<(), nt35510::Error>` — need to cover all 4 variants in match
- `Error` does NOT implement `defmt::Format` — cannot use `{:?}` in `defmt::panic!`

### Import needed
- `use embedded_hal::delay::DelayNs;` required to use trait in function signature bounds
- Task 4/5 migration note: importing `embedded_hal::delay::DelayNs` in `board.rs` can make `delay_ms`/`delay_us` method calls ambiguous with embedded-hal 0.2 traits; use fully-qualified `embedded_hal_02::blocking::delay::{DelayUs, DelayMs}` calls where needed.
