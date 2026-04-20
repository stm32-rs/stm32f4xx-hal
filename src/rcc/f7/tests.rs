use fugit::{HertzU32 as Hertz, RateExtU32};

use super::{Config, FreqRequest};

fn build_request(sysclk: u32, use_pll48clk: bool) -> FreqRequest {
    let p = Some((sysclk - 1, sysclk + 1));
    let q = if use_pll48clk {
        Some((48_000_000 - 120_000, 48_000_000 + 120_000))
    } else {
        None
    };
    FreqRequest { p, q }
}

fn check(hse: u32, sysclk: u32, use_pll48clk: bool) {
    let request = build_request(sysclk, use_pll48clk);
    let (m, n, p, q) =
        Config::calculate_mnpq(hse, request).expect("Can't calculate PLL parameters");

    let pll_in = hse;

    if m < 2 || m > 63 {
        panic!("Invalid PLL M value: {}", m);
    }

    let vco_in = pll_in / m;
    if vco_in < 1_000_000 || vco_in > 2_000_000 {
        panic!("Invalid VCO input frequency: {}", vco_in);
    }

    if n < 50 || n > 432 {
        panic!("Invalid PLL N value: {}", n);
    }

    let vco = ((pll_in as u64) * (n as u64) / (m as u64)) as u32;
    if vco < 100_000_000 || vco > 432_000_000 {
        panic!("Invalid VCO frequency: {}", vco);
    }

    let p = p.expect("PLL P value should be defined!");
    if [2, 4, 6, 8].iter().find(|v| **v == p).is_none() {
        panic!("Invalid PLL P value: {}", p);
    }

    let p_freq = vco / p;
    if p_freq > 216_000_000 {
        panic!("Invalid PLL P frequency: {}", p_freq);
    }
    if p_freq < (sysclk - 1) || p_freq > (sysclk + 1) {
        panic!(
            "Invalid PLL P frequency: {} (requested sysclk {})",
            p_freq, sysclk
        );
    }

    if use_pll48clk && q.is_none() {
        panic!("PLL Q value should be defined!");
    }
    if let Some(q) = q {
        if q < 2 || q > 15 {
            panic!("Invalid PLL Q value: {}", q);
        }
        if use_pll48clk {
            let q_freq = vco / q;
            if q_freq < (48_000_000 - 120_000) || q_freq > (48_000_000 + 120_000) {
                panic!("Invalid PLL Q frequency: {}", q_freq);
            }
        }
    }
}

#[test]
fn test_pll_calc1() {
    check(25_000_000, 48_000_000, false);
}

#[test]
fn test_pll_calc1_usb() {
    check(25_000_000, 48_000_000, true);
}

#[test]
fn test_pll_calc2() {
    check(12_000_000, 48_000_000, false);
}

#[test]
fn test_pll_calc2_usb() {
    check(12_000_000, 48_000_000, true);
}

#[test]
fn test_pll_calc3() {
    check(12_000_000, 216_000_000, false);
}

#[test]
fn test_pll_calc3_usb() {
    check(12_000_000, 216_000_000, true);
}

#[test]
fn test_rcc_calc1() {
    use super::{HSEClock, HSEClockMode, MCO1, MCO2, MCOPRE, PLL48CLK, PLLP, PLLSAIP};

    let cfgr = Config {
        hse: None,
        hse_bypass: false,
        hclk: None,
        sysclk: None,
        pclk1: None,
        pclk2: None,
        lse: None,
        lsi: None,
        use_pll: false,
        pll48clk: None,
        pllm: 2,
        plln: 50,
        pllp: PLLP::Div2,
        pllq: 2,
        use_pllsai: false,
        pllsain: 192,
        pllsaip: PLLSAIP::Div2,
        pllsaiq: 2,
        use_plli2s: false,
        plli2sr: 2,
        plli2sq: 2,
        plli2sn: 192,
        mco1: MCO1::Hsi,
        mco1pre: MCOPRE::Div1_no_div,
        mco2: MCO2::Sysclk,
        mco2pre: MCOPRE::Div1_no_div,
    };

    let mut cfgr = cfgr
        .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
        .use_pll()
        .use_pll48clk(PLL48CLK::Pllq)
        .sysclk(216.MHz());
    cfgr.pll_configure();

    assert_eq!(cfgr.hse.unwrap().freq, Hertz::MHz(25));

    let (clocks, _config) = cfgr.calculate_clocks();
    assert_eq!(clocks.sysclk().raw(), 216_000_000);
    assert!(clocks.is_pll48clk_valid());
}

#[test]
fn test_rcc_calc2() {
    use super::{HSEClock, HSEClockMode, MCO1, MCO2, MCOPRE, PLL48CLK, PLLP, PLLSAIP};

    let cfgr = Config {
        hse: None,
        hse_bypass: false,
        hclk: None,
        sysclk: None,
        pclk1: None,
        pclk2: None,
        lse: None,
        lsi: None,
        use_pll: false,
        pll48clk: None,
        pllm: 2,
        plln: 50,
        pllp: PLLP::Div2,
        pllq: 2,
        use_pllsai: false,
        pllsain: 192,
        pllsaip: PLLSAIP::Div2,
        pllsaiq: 2,
        use_plli2s: false,
        plli2sr: 2,
        plli2sq: 2,
        plli2sn: 192,
        mco1: MCO1::Hsi,
        mco1pre: MCOPRE::Div1_no_div,
        mco2: MCO2::Sysclk,
        mco2pre: MCOPRE::Div1_no_div,
    };

    let mut cfgr = cfgr
        .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
        .use_pll48clk(PLL48CLK::Pllq)
        .sysclk(216.MHz());
    cfgr.pll_configure();

    assert_eq!(cfgr.hse.unwrap().freq, Hertz::MHz(25));

    let (clocks, _config) = cfgr.calculate_clocks();
    assert_eq!(clocks.sysclk().raw(), 216_000_000);
    assert!(clocks.is_pll48clk_valid());
}

#[test]
fn test_rcc_calc3() {
    use super::{HSEClock, HSEClockMode, MCO1, MCO2, MCOPRE, PLL48CLK, PLLP, PLLSAIP};

    let cfgr = Config {
        hse: None,
        hse_bypass: false,
        hclk: None,
        sysclk: None,
        pclk1: None,
        pclk2: None,
        lse: None,
        lsi: None,
        use_pll: false,
        pll48clk: None,
        pllm: 2,
        plln: 50,
        pllp: PLLP::Div2,
        pllq: 2,
        use_pllsai: false,
        pllsain: 192,
        pllsaip: PLLSAIP::Div2,
        pllsaiq: 2,
        use_plli2s: false,
        plli2sr: 2,
        plli2sq: 2,
        plli2sn: 192,
        mco1: MCO1::Hsi,
        mco1pre: MCOPRE::Div1_no_div,
        mco2: MCO2::Sysclk,
        mco2pre: MCOPRE::Div1_no_div,
    };

    let mut cfgr = cfgr
        .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
        .use_pll48clk(PLL48CLK::Pllq)
        .set_defaults();
    cfgr.pll_configure();

    assert_eq!(cfgr.hse.unwrap().freq, Hertz::MHz(25));

    let (clocks, _config) = cfgr.calculate_clocks();
    assert_eq!(clocks.sysclk().raw(), 216_000_000);
    assert!(clocks.is_pll48clk_valid());
}

#[test]
fn test_rcc_default() {
    use super::{MCO1, MCO2, MCOPRE, PLLP, PLLSAIP};

    let mut cfgr = Config {
        hse: None,
        hse_bypass: false,
        hclk: None,
        sysclk: None,
        pclk1: None,
        pclk2: None,
        lse: None,
        lsi: None,
        use_pll: false,
        pll48clk: None,
        pllm: 2,
        plln: 50,
        pllp: PLLP::Div2,
        pllq: 2,
        use_pllsai: false,
        pllsain: 192,
        pllsaip: PLLSAIP::Div2,
        pllsaiq: 2,
        use_plli2s: false,
        plli2sr: 2,
        plli2sq: 2,
        plli2sn: 192,
        mco1: MCO1::Hsi,
        mco1pre: MCOPRE::Div1_no_div,
        mco2: MCO2::Sysclk,
        mco2pre: MCOPRE::Div1_no_div,
    };

    cfgr.pll_configure();
    assert!(!cfgr.use_pll);
    let (clocks, _config) = cfgr.calculate_clocks();
    assert_eq!(clocks.sysclk().raw(), 16_000_000);
}
