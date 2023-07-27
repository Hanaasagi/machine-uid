use std::env;
fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    if target_os.is_ok() && target_os.unwrap() == "windows" {
        use cc::Build;
        // println!("cargo:rustc-link-lib=Kernel.a");
        Build::new()
            .file("src/win.cpp")
            .cpp_link_stdlib("stdc++")
            .compile("machine-uid");
    }
}
