# Feedback for Specter-DIY Firmware v4.1

## Summary

❌ **BROKEN** - Firmware panics immediately during RNG initialization due to clock configuration assertion failure.

---

## What Works (✅)

1. **Compilation & Flashing**: Firmware compiles and flashes successfully
2. **Debug Info**: Line numbers and file paths are visible in logs (fixed from v3.8)
3. **Early Logging**: Boot banner and initial messages display correctly

---

## What Doesn't Work (❌)

### 1. RNG Clock Configuration - CRITICAL

**Panic Location**: `stm32f4xx-hal/src/rng.rs:91`

```
ERROR panicked at /home/z/.cargo/git/checkouts/stm32f4xx-hal-ada826ebd427e1ab/dc928d7/src/rng.rs:91:13:
assertion failed: rng_clk >= (hclk / 16)
```

**Root Cause**: The RNG peripheral requires a minimum clock frequency. The assertion:
- `rng_clk >= (hclk / 16)`
- If HCLK = 168 MHz (max for STM32F469), RNG clock must be >= 10.5 MHz
- If HCLK = 144 MHz, RNG clock must be >= 9 MHz

**Problem**: The RNG is likely being clocked from a source that's too slow (maybe HSI at 16 MHz divided down, or PLL output incorrectly configured).

### Fix Required

1. **Check RCC Clock Configuration**:
   - Ensure PLL48CLK (used for RNG) is properly configured
   - For STM32F4, RNG typically uses PLL48CLK (48 MHz USB clock)
   - Alternative: Use HSI16 or HSE directly if available

2. **STM32F4xx HAL RNG Clock Source Options**:
   - `RNG_CLK_HSI48` - Internal 48 MHz (if available)
   - `RNG_CLK_PLL48CLK` - From PLL (common for USB)
   - Check `Rcc.cfgr` and `Rcc.pllcfgr` settings

3. **Code Fix Example**:
   ```rust
   // Before creating Rng, ensure PLL48CLK is enabled
   let clocks = rcc.cfgr
       .use_hse(8.mhz())  // External crystal
       .sysclk(168.mhz()) // Max for STM32F469
       .hclk(168.mhz())
       .pclk1(42.mhz())
       .pclk2(84.mhz())
       .require_pll48clk()  // IMPORTANT: Enable 48MHz PLL for RNG/USB
       .freeze(&mut flash.acr);
   
   // Then RNG should work
   let mut rng = dp.RNG.constrain(&clocks);
   ```

4. **Alternative Workaround** (if PLL48CLK unavailable):
   - Use HSI (16 MHz internal oscillator) as RNG source
   - Lower HCLK frequency to satisfy `rng_clk >= hclk/16`

---

## Boot Log

```
INFO  ========================================
└─ firmware::__cortex_m_rt_main @ /home/z/my-project/specter-diy-rust/firmware/src/main.rs:152 
INFO  Specter-DIY Rust Firmware v4.1
└─ firmware::__cortex_m_rt_main @ /home/z/my-project/specter-diy-rust/firmware/src/main.rs:153 
INFO  Using stm32f4xx-hal drivers directly
└─ firmware::__cortex_m_rt_main @ /home/z/my-project/specter-diy-rust/firmware/src/main.rs:154 
INFO  ========================================
└─ firmware::__cortex_m_rt_main @ /home/z/my-project/specter-diy-rust/firmware/src/main.rs:155 
INFO  Initializing Hardware RNG...
└─ firmware::__cortex_m_rt_main @ /home/z/my-project/specter-diy-rust/firmware/src/main.rs:165 
ERROR panicked at .../stm32f4xx-hal-.../src/rng.rs:91:13:
assertion failed: rng_clk >= (hclk / 16)
```

---

## Stack Trace

```
Frame 0: HardFault_ @ 0x0800179e
Frame 1: HardFault <Cause: Escalated UsageFault <Cause: Undefined instruction>>
...
Frame 11: constrain @ stm32f4xx-hal/src/rng.rs:83:9
Frame 12: __cortex_m_rt_main @ firmware/src/main.rs:166:29
```

The crash occurs at `main.rs:166` when calling `dp.RNG.constrain(&clocks)`.

---

## Comparison with Previous Versions

| Version | RNG Status | Notes |
|---------|------------|-------|
| v3.7 | ✅ Working | Proper clock config |
| v4.0 | ❌ Timeout | Returned 0x00000000 |
| v4.1 | ❌ Panic | Clock assertion failure |

v4.1 is **worse** than v4.0 - now it panics instead of just timing out.

---

## Priority Fixes

1. **HIGH**: Fix RCC clock configuration to provide valid RNG clock
   - Add `.require_pll48clk()` to clock config
   - Or use alternative clock source

2. **MEDIUM**: Add graceful error handling instead of panic
   - Return `Result<Rng, RngError>` instead of panicking
   - Allow firmware to boot with software RNG fallback

3. **LOW**: Validate clock config at compile time if possible

---

## Files to Check/Modify

1. `firmware/src/main.rs` - Line 166: RNG initialization
2. Clock configuration (likely in main.rs or separate config module)
3. Check if using stm32f4xx-hal from git - commit `dc928d7`

---

## Test Status

- **Boot**: ✅ Started
- **RNG**: ❌ Panic at initialization
- **Display**: ⬜ Not reached
- **Touch**: ⬜ Not reached
- **SD Card**: ⬜ Not reached
- **Flash Storage**: ⬜ Not reached

**Firmware crashes before any other hardware can be tested.**
