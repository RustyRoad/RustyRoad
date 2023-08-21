extern crate pkg_config;

fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native=C:\\Program Files (x86)\\vcpkg\\installed\\x64-windows\\lib");
    } else if cfg!(target_os = "linux") {
        // Try to use pkg-config to find libpq
        if let Err(e) = pkg_config::probe_library("libpq") {
            println!("Failed to find libpq with pkg-config: {}", e);
            // If pkg-config fails, you can fallback to the hard-coded paths:
            println!("cargo:rustc-link-search=native=/usr/lib/");
            println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
            println!("cargo:rustc-link-search=native=/usr/local/lib/");
            println!("cargo:rustc-link-search=native=/lib/");
        }
    }

    println!("cargo:rustc-link-lib=libpq");
}
