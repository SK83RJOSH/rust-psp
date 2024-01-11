use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=psp.ld");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    if env::var("CARGO_FEATURE_STUB_ONLY").is_ok() {
        return;
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("psp.ld");
    std::fs::copy("psp.ld", out_file).unwrap();

    println!("cargo:rustc-link-arg=-Tpsp.ld");
    println!("cargo:rustc-link-search={}", out_dir);
}
