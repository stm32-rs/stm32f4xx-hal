use crate::pac::RCC;

pub struct MainPll {
    pub use_pll: bool,
    pub pllsysclk: Option<u32>,
    pub pll48clk: Option<u32>,
    /// "M" divisor, required for the other PLLs on some MCUs.
    pub m: Option<u32>,
    /// "R" output, required for I2S on STM32F410.
    pub plli2sclk: Option<u32>,
}

impl MainPll {
    pub fn fast_setup(
        pllsrcclk: u32,
        use_hse: bool,
        pllsysclk: Option<u32>,
        pll48clk: bool,
    ) -> MainPll {
        let sysclk = pllsysclk.unwrap_or(pllsrcclk);
        if pllsysclk.is_none() && !pll48clk {
            // Even if we do not use the main PLL, we still need to set the PLL source as that setting
            // applies to the I2S and SAI PLLs as well.
            unsafe { &*RCC::ptr() }
                .pllcfgr
                .write(|w| w.pllsrc().bit(use_hse));

            return MainPll {
                use_pll: false,
                pllsysclk: None,
                pll48clk: None,
                m: None,
                plli2sclk: None,
            };
        }
        // Input divisor from PLL source clock, must result to frequency in
        // the range from 1 to 2 MHz
        let pllm_min = (pllsrcclk + 1_999_999) / 2_000_000;
        let pllm_max = pllsrcclk / 1_000_000;

        // Sysclk output divisor must be one of 2, 4, 6 or 8
        let sysclk_div = core::cmp::min(8, (432_000_000 / sysclk) & !1);

        let target_freq = if pll48clk {
            48_000_000
        } else {
            sysclk * sysclk_div
        };

        // Find the lowest pllm value that minimize the difference between
        // target frequency and the real vco_out frequency.
        let pllm = (pllm_min..=pllm_max)
            .min_by_key(|pllm| {
                let vco_in = pllsrcclk / pllm;
                let plln = target_freq / vco_in;
                target_freq - vco_in * plln
            })
            .unwrap();

        let vco_in = pllsrcclk / pllm;
        assert!((1_000_000..=2_000_000).contains(&vco_in));

        // Main scaler, must result in >= 100MHz (>= 192MHz for F401)
        // and <= 432MHz, min 50, max 432
        let plln = if pll48clk {
            // try the different valid pllq according to the valid
            // main scaller values, and take the best
            let pllq = (4..=9)
                .min_by_key(|pllq| {
                    let plln = 48_000_000 * pllq / vco_in;
                    let pll48_diff = 48_000_000 - vco_in * plln / pllq;
                    let sysclk_diff = (sysclk as i32 - (vco_in * plln / sysclk_div) as i32).abs();
                    (pll48_diff, sysclk_diff)
                })
                .unwrap();
            48_000_000 * pllq / vco_in
        } else {
            sysclk * sysclk_div / vco_in
        };
        let pllp = (sysclk_div / 2) - 1;

        let pllq = (vco_in * plln + 47_999_999) / 48_000_000;
        let real_pll48clk = vco_in * plln / pllq;

        unsafe { &*RCC::ptr() }.pllcfgr.write(|w| unsafe {
            w.pllm().bits(pllm as u8);
            w.plln().bits(plln as u16);
            w.pllp().bits(pllp as u8);
            w.pllq().bits(pllq as u8);
            w.pllsrc().bit(use_hse)
        });

        let real_pllsysclk = vco_in * plln / sysclk_div;

        MainPll {
            use_pll: true,
            pllsysclk: Some(real_pllsysclk),
            pll48clk: if pll48clk { Some(real_pll48clk) } else { None },
            m: Some(pllm),
            plli2sclk: None,
        }
    }

    #[cfg(feature = "gpio-f410")]
    pub fn setup_with_i2s(
        pllsrcclk: u32,
        use_hse: bool,
        pllsysclk: Option<u32>,
        pll48clk: bool,
        plli2sclk: u32,
    ) -> MainPll {
        use super::{SYSCLK_MAX, SYSCLK_MIN};

        // Input divisor from PLL source clock, must result to frequency in
        // the range from 1 to 2 MHz
        let pllm_min = (pllsrcclk + 1_999_999) / 2_000_000;
        let pllm_max = pllsrcclk / 1_000_000;

        let (pllm, plln, pllp, pllq, pllr, _) = (pllm_min..=pllm_max)
            .filter_map(|m| {
                let vco_in = pllsrcclk / m;

                // The VCO output must be within 100 and 432 MHz.
                let plln_min = (100_000_000 + vco_in - 1) / vco_in;
                let plln_max = 432_000_000 / vco_in;

                (plln_min..=plln_max)
                    .filter_map(|n| {
                        let vco_out = vco_in * n;

                        // The "P" divider value must be even (2, 4, 6, 8).
                        let p = if let Some(pllsysclk) = pllsysclk {
                            let (p, p_output, p_error) = Self::best_divider(
                                vco_out,
                                SYSCLK_MIN * 2,
                                pllsysclk * 2,
                                SYSCLK_MAX * 2,
                                1,
                                4,
                            )?;
                            Some((p * 2, p_output / 2, p_error / 2))
                        } else {
                            None
                        };

                        // The 48 MHz clock must be accurate within 0.25% for USB.
                        let q = if pll48clk {
                            Some(Self::best_divider(
                                vco_out, 47_880_000, 48_000_000, 48_120_000, 2, 15,
                            )?)
                        } else {
                            None
                        };

                        // We do not set any accuracy requirements for I2S, as on F410 this frequency is
                        // provided on a best-effort basis.
                        // TODO: What is the maximum valid input frequency for I2S?
                        let r = Self::best_divider(vco_out, 0, plli2sclk, u32::MAX, 2, 15)?;

                        let error = p.map(|(_, _, error)| error).unwrap_or(0)
                            + p.map(|(_, _, error)| error).unwrap_or(0)
                            + r.2;

                        Some((m, n, p.map(|p| p.0), q.map(|q| q.0), r.0, error))
                    })
                    .min_by_key(|(_, _, _, _, _, error)| *error)
            })
            .min_by_key(|(_, _, _, _, _, error)| *error)
            .expect("could not find a valid main PLL configuration");

        unsafe { &*RCC::ptr() }.pllcfgr.write(|w| unsafe {
            w.pllm().bits(pllm as u8);
            w.plln().bits(plln as u16);
            if let Some(pllp) = pllp {
                w.pllp().bits(pllp as u8 / 2 - 1);
            }
            if let Some(pllq) = pllq {
                w.pllq().bits(pllq as u8);
            }
            w.pllr().bits(pllr as u8);
            w.pllsrc().bit(use_hse)
        });

        let real_pllsysclk = pllp.map(|pllp| pllsrcclk / pllm * plln / pllp);
        let real_pll48clk = pllq.map(|pllq| pllsrcclk / pllm * plln / pllq);

        MainPll {
            use_pll: true,
            pllsysclk: real_pllsysclk,
            pll48clk: real_pll48clk,
            m: Some(pllm),
            plli2sclk: None,
        }
    }

    #[cfg(feature = "gpio-f410")]
    fn best_divider(
        vco_out: u32,
        min: u32,
        target: u32,
        max: u32,
        min_div: u32,
        max_div: u32,
    ) -> Option<(u32, u32, u32)> {
        let div = (vco_out + target / 2) / target;
        let min_div = u32::max(
            min_div,
            if max != 0 {
                (vco_out + max - 1) / max
            } else {
                0
            },
        );
        let max_div = u32::min(max_div, if min != 0 { vco_out / min } else { u32::MAX });
        if min_div > max_div {
            return None;
        }
        let div = u32::min(u32::max(div, min_div), max_div);
        let output = vco_out / div;
        let error = (output as i32 - target as i32).abs() as u32;
        Some((div, output, error))
    }
}

#[cfg(not(feature = "gpio-f410"))]
pub struct I2sPll {
    pub use_pll: bool,
    /// "M" divisor, required for the other PLLs on some MCUs.
    pub m: Option<u32>,
    /// PLL I2S clock output.
    pub plli2sclk: Option<u32>,
}

#[cfg(not(feature = "gpio-f410"))]
impl I2sPll {
    pub fn unused() -> I2sPll {
        I2sPll {
            use_pll: false,
            m: None,
            plli2sclk: None,
        }
    }

    pub fn setup(pllsrcclk: u32, plli2sclk: Option<u32>) -> I2sPll {
        let target = if let Some(clk) = plli2sclk {
            clk
        } else {
            return Self::unused();
        };
        // Input divisor from PLL source clock, must result to frequency in
        // the range from 1 to 2 MHz
        let pllm_min = (pllsrcclk + 1_999_999) / 2_000_000;
        let pllm_max = pllsrcclk / 1_000_000;
        let (pll, config, _) = (pllm_min..=pllm_max)
            .map(|m| Self::optimize_fixed_m(pllsrcclk, m, target))
            .min_by_key(|(_, _, error)| *error)
            .expect("no suitable I2S PLL configuration found");
        Self::apply_config(config);
        pll
    }

    #[cfg(any(
        feature = "gpio-f401",
        feature = "gpio-f417",
        feature = "gpio-f427",
        feature = "gpio-f469",
    ))]
    pub fn setup_shared_m(pllsrcclk: u32, m: Option<u32>, plli2sclk: Option<u32>) -> I2sPll {
        // "m" is None if the main PLL is not in use.
        let m = if let Some(m) = m {
            m
        } else {
            return Self::setup(pllsrcclk, plli2sclk);
        };
        let target = if let Some(clk) = plli2sclk {
            clk
        } else {
            return Self::unused();
        };
        let (pll, config, _) = Self::optimize_fixed_m(pllsrcclk, m, target);
        Self::apply_config(config);
        pll
    }

    fn optimize_fixed_m(pllsrcclk: u32, m: u32, plli2sclk: u32) -> (I2sPll, SingleOutputPll, u32) {
        let (config, real_plli2sclk, error) =
            SingleOutputPll::optimize(pllsrcclk, m, plli2sclk, 2, 7)
                .expect("did not find any valid I2S PLL config");
        (
            I2sPll {
                use_pll: true,
                m: Some(config.m as u32),
                plli2sclk: Some(real_plli2sclk),
            },
            config,
            error,
        )
    }

    #[cfg(not(any(
        feature = "gpio-f411",
        feature = "gpio-f412",
        feature = "gpio-f413",
        feature = "gpio-f446",
    )))]
    fn apply_config(config: SingleOutputPll) {
        let rcc = unsafe { &*RCC::ptr() };
        // "M" may have been written before, but the value is identical.
        rcc.pllcfgr
            .modify(|_, w| unsafe { w.pllm().bits(config.m) });
        rcc.plli2scfgr
            .modify(|_, w| unsafe { w.plli2sn().bits(config.n).plli2sr().bits(config.outdiv) });
    }
    #[cfg(any(
        feature = "gpio-f411",
        feature = "gpio-f412",
        feature = "gpio-f413",
        feature = "gpio-f446",
    ))]
    fn apply_config(config: SingleOutputPll) {
        let rcc = unsafe { &*RCC::ptr() };
        rcc.plli2scfgr.modify(|_, w| unsafe {
            w.plli2sm()
                .bits(config.m)
                .plli2sn()
                .bits(config.n)
                .plli2sr()
                .bits(config.outdiv)
        });
    }
}

#[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469",))]
pub struct SaiPll {
    pub use_pll: bool,
    /// SAI clock (PLL output divided by the SAI clock divider).
    pub sai_clk: Option<u32>,
}

#[cfg(any(feature = "gpio-f427", feature = "gpio-f446", feature = "gpio-f469",))]
impl SaiPll {
    pub fn unused() -> SaiPll {
        SaiPll {
            use_pll: false,
            sai_clk: None,
        }
    }

    pub fn setup(pllsrcclk: u32, sai_clk: Option<u32>) -> SaiPll {
        let target = if let Some(clk) = sai_clk {
            clk
        } else {
            return Self::unused();
        };
        // Input divisor from PLL source clock, must result to frequency in
        // the range from 1 to 2 MHz
        let pllm_min = (pllsrcclk + 1_999_999) / 2_000_000;
        let pllm_max = pllsrcclk / 1_000_000;
        let (pll, config, saidiv, _) = (pllm_min..=pllm_max)
            .map(|m| Self::optimize_fixed_m(pllsrcclk, m, target))
            .min_by_key(|(_, _, _, error)| *error)
            .expect("no suitable SAI PLL configuration found");
        Self::apply_config(config, saidiv);
        pll
    }

    #[cfg(any(feature = "gpio-f427", feature = "gpio-f469",))]
    pub fn setup_shared_m(pllsrcclk: u32, m: Option<u32>, sai_clk: Option<u32>) -> SaiPll {
        // "m" is None if both other PLLs are not in use.
        let m = if let Some(m) = m {
            m
        } else {
            return Self::setup(pllsrcclk, sai_clk);
        };
        let target = if let Some(clk) = sai_clk {
            clk
        } else {
            return Self::unused();
        };
        let (pll, config, saidiv, _) = Self::optimize_fixed_m(pllsrcclk, m, target);
        Self::apply_config(config, saidiv);
        pll
    }

    fn optimize_fixed_m(
        pllsrcclk: u32,
        m: u32,
        sai_clk: u32,
    ) -> (SaiPll, SingleOutputPll, u32, u32) {
        // NOTE: This code tests lots of configurations due to the nested loops for the two
        // dividers. A smarter approach can probably speed up the search.
        let (config, saidiv, real_sai_clk, error) = (1..=32)
            .filter_map(|saidiv| {
                let target = sai_clk * saidiv;
                let (config, real_sai_clk, error) =
                    SingleOutputPll::optimize(pllsrcclk, m, target, 2, 15)?;
                Some((config, saidiv, real_sai_clk, error))
            })
            .min_by_key(|(_, _, _, error)| *error)
            .expect("no suitable I2S PLL configuration found");
        (
            SaiPll {
                use_pll: true,
                sai_clk: Some(real_sai_clk),
            },
            config,
            saidiv,
            error,
        )
    }

    #[cfg(not(feature = "gpio-f446"))]
    fn apply_config(config: SingleOutputPll, saidiv: u32) {
        let rcc = unsafe { &*RCC::ptr() };
        rcc.dckcfgr
            .modify(|_, w| w.pllsaidivq().bits(saidiv as u8 - 1));
        // "M" may have been written before, but the value is identical.
        rcc.pllcfgr
            .modify(|_, w| unsafe { w.pllm().bits(config.m) });
        rcc.pllsaicfgr
            .modify(|_, w| unsafe { w.pllsain().bits(config.n).pllsaiq().bits(config.outdiv) });
    }
    #[cfg(feature = "gpio-f446")]
    fn apply_config(config: SingleOutputPll, saidiv: u32) {
        let rcc = unsafe { &*RCC::ptr() };
        rcc.dckcfgr
            .modify(|_, w| w.pllsaidivq().bits(saidiv as u8 - 1));
        rcc.pllsaicfgr.modify(|_, w| unsafe {
            w.pllsaim()
                .bits(config.m)
                .pllsain()
                .bits(config.n)
                .pllsaiq()
                .bits(config.outdiv)
        });
    }
}

#[cfg(not(feature = "gpio-f410"))]
struct SingleOutputPll {
    m: u8,
    n: u16,
    outdiv: u8,
}

#[cfg(not(feature = "gpio-f410"))]
impl SingleOutputPll {
    fn optimize(
        pllsrcclk: u32,
        m: u32,
        target: u32,
        min_div: u32,
        max_div: u32,
    ) -> Option<(SingleOutputPll, u32, u32)> {
        let vco_in = pllsrcclk / m;

        // We loop through the possible divider values to find the best configuration. Looping
        // through all possible "N" values would result in more iterations.
        let (n, outdiv, output, error) = (min_div..=max_div)
            .filter_map(|outdiv| {
                let target_vco_out = match target.checked_mul(outdiv) {
                    Some(x) => x,
                    None => return None,
                };
                let n = (target_vco_out + (vco_in >> 1)) / vco_in;
                let vco_out = vco_in * n;
                if !(100_000_000..=432_000_000).contains(&vco_out) {
                    return None;
                }
                let output = vco_out / outdiv;
                let error = (output as i32 - target as i32).unsigned_abs();
                Some((n, outdiv, output, error))
            })
            .min_by_key(|(_, _, _, error)| *error)?;
        Some((
            SingleOutputPll {
                m: m as u8,
                n: n as u16,
                outdiv: outdiv as u8,
            },
            output,
            error,
        ))
    }
}
