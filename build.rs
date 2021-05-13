#![allow(missing_docs)]

fn main() {
    let is_apple = std::env::var("CARGO_CFG_TARGET_OS")
        .map(|t| matches!(t.as_str(), "ios" | "macos"))
        .unwrap();

    if is_apple {
        cc::Build::new()
            .file("src/apple_locale.m")
            .compile("locale");

        println!("cargo:rustc-link-lib=framework=Foundation");
    }

    println!("cargo:rerun-if-changed=src/apple_locale.m");
    println!("cargo:rerun-if-env-changed=TARGET");
}
