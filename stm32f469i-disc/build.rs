use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Only generate memory.x when building the HAL as the primary package,
    // not when building as a dependency (e.g., for BSPs that have their own memory.x)
    let is_primary = env::var("CARGO_PRIMARY_PACKAGE").is_ok();

    if is_primary {
        // Put the linker script somewhere the linker can find it
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("memory.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}
