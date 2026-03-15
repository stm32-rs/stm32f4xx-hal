# Learning Notes

## RAM/HEAP Size Fix

**Date**: 2026-03-02

### Problem
The F412 chip only has 256K RAM, but the constants claimed 320K. This would cause the allocator to claim memory beyond physical RAM.

### Solution
Replaced hardcoded `RAM_SIZE` and `HEAP_SIZE` constants with cfg-conditional versions:

```rust
/// Total configured RAM size from `memory.x` (build.rs generates per-chip)
#[cfg(feature = "stm32f412")]
const RAM_SIZE: usize = 256 * 1024;
#[cfg(not(feature = "stm32f412"))]
const RAM_SIZE: usize = 320 * 1024;

/// Size of the heap in bytes
#[cfg(feature = "stm32f412")]
pub const HEAP_SIZE: usize = 222 * 1024;  // 256K - ~34K data+stack
#[cfg(not(feature = "stm32f412"))]
pub const HEAP_SIZE: usize = 286 * 1024;  // 320K - ~34K data+stack
```

### Verification
All 3 targets compile successfully:
- `--features stm32f412` → RAM_SIZE = 256K, HEAP_SIZE = 222K
- `--features stm32f413` → RAM_SIZE = 320K, HEAP_SIZE = 286K
- `--features stm32f469` → RAM_SIZE = 320K, HEAP_SIZE = 286K

### Memory Layout
| Board | RAM | HEAP | Stack+Data |
|-------|-----|------|------------|
| F412 | 256K | 222K | ~34K |
| F413 | 320K | 286K | ~34K |
| F469 | 320K | 286K | ~34K |
