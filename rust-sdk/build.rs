fn main() {
    let version = if let Ok(version) = std::env::var("CARGO_PKG_VERSION") {
        version
    } else {
        "unknown".to_string()
    };

    println!("cargo:rustc-env=APP_VERSION={}", version);
}