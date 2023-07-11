fn main() {
    #[cfg(target_os = "windows")]
    {
        use cc::Build;
        println!("cargo:rustc-link-lib=Kernel32");
        Build::new().file("src/win.cpp").compile("machine-uid");
    }
}
