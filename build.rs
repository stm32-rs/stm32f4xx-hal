use std::env;

#[derive(Clone, Copy, Debug)]
enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match (self.next(), self.next()) {
            (Some(res), None) => Ok(res),
            (None, _) => Err(GetOneError::None),
            _ => Err(GetOneError::Multiple),
        }
    }
}

fn main() {
    let _chip_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_STM32F4"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No stm32xx Cargo feature enabled"),
        Err(GetOneError::Multiple) => panic!("Multiple stm32xx Cargo features enabled"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase();

    // Only generate memory.x when building the HAL as the primary package,
    // not when building as a dependency (e.g., for BSPs that have their own memory.x)
    let is_primary = env::var("CARGO_PRIMARY_PACKAGE").is_ok();
    
    if is_primary {
        let out = &std::path::PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let memory_x = include_bytes!("memory.x");
        std::fs::write(out.join("memory.x"), memory_x).unwrap();
        println!("cargo:rustc-link-search={}", out.display());
    }
    
    println!("cargo:rerun-if-changed=memory.x");
}
