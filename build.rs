use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=src");
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=Kernel32");
        Build::new().file("src/win.cpp").compile("machine-uid");
    }
}
