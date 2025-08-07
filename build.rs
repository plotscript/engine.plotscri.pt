use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/parser/grammar.pest");
    
    // Set version info for CLI
    println!("cargo:rustc-env=RUSTC_VERSION={}", env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()));
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()));
    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string()));
}