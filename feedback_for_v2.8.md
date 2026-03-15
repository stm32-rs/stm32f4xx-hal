# Feedback for v2.8 - HANGS ON RNG INIT

**Status**: ❌ BROKEN - Hangs at RNG initialization

## What Works
- ✅ Boot starts
- ✅ Peripherals taken
- ✅ RCC clock config (SYSCLK=180MHz)
- ✅ GPIO configured
- ✅ LCD reset

## What's Broken

### CRITICAL: RNG Initialization Hangs
```
INFO  [RNG] Hardware RNG initialization...
```
The firmware **never proceeds past this line**. It hangs indefinitely.

### Blank Screen
- Display never initialized
- No touch functionality
- Screen remains blank

## Root Cause Analysis

The hardware RNG on STM32F469 requires:
1. RNG clock enabled via RCC AHB2ENR.RNGEN
2. RNG peripheral enabled via RNG_CR.RNGEN
3. Wait for RNG_SR.DRDY (data ready) flag

If any of these fails (e.g., clock not properly configured), the code will block forever waiting for DRDY.

## Fixes Required

### Option 1: Add Timeout to RNG Init
```rust
pub fn new(rcc: &mut RCC) -> Result<Self, RngError> {
    // Enable clock
    rcc.ahb2enr.modify(|_, w| w.rngen().set_bit());
    
    // Enable RNG
    RNG.cr.modify(|_, w| w.rngen().set_bit());
    
    // Wait for ready WITH TIMEOUT
    let timeout = 100_000;
    for _ in 0..timeout {
        if RNG.sr.read().drdy().bit_is_set() {
            return Ok(Self { _private: () });
        }
    }
    
    Err(RngError::Timeout)
}
```

### Option 2: Use Software Entropy (Fallback)
Like v2.9 does:
```rust
INFO  [RNG] Using software entropy (HW RNG needs clock fix)
```

### Option 3: Fix Clock Configuration
At 180MHz SYSCLK, verify:
- PLL configuration is correct
- AHB2 prescaler is correct
- RNG clock source is valid

## Comparison with v2.5
| Feature | v2.5 | v2.8 |
|---------|------|------|
| RNG Init | ✅ Works | ❌ Hangs |
| SYSCLK | 168MHz | 180MHz |
| Screen | ✅ Works | ❌ Never reached |

---

**Conclusion**: v2.8 hangs at RNG init. Either fix clock/RNG config or use software entropy fallback like v2.9.
