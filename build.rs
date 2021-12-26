const DEVICES: [&str; 17] = [
    "stm32f401",
    "stm32f405",
    "stm32f407",
    "stm32f410",
    "stm32f411",
    "stm32f412",
    "stm32f413",
    "stm32f415",
    "stm32f417",
    "stm32f423",
    "stm32f427",
    "stm32f429",
    "stm32f437",
    "stm32f439",
    "stm32f446",
    "stm32f469",
    "stm32f479",
];

fn main() {
    let features = DEVICES
        .iter()
        .map(|d| format!("CARGO_FEATURE_{}", d.to_uppercase()))
        .collect::<Vec<_>>();
    assert!(
        features
            .iter()
            .map(|f| i32::from(std::env::var_os(f).is_some()))
            .sum::<i32>()
            == 1,
        "This crate requires one of the following device features enabled:\n\t{}",
        DEVICES.join("\n\t")
    );
}
