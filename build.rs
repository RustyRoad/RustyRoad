use std::env;

fn main() {
    if cfg!(target_os = "windows") {
        let _pg_path = match env::var("POSTGRES_LIB_PATH") {
            Ok(val) => val,
            Err(_) => panic!("Could not find environment variable POSTGRES_LIB_PATH. Make sure it's set and valid."),
        };
    } else if cfg!(target_os = "linux") {
        // Try to use pkg-config to find libpq
        if let Err(e) = pkg_config::probe_library("libpq") {
            println!("Failed to find libpq with pkg_config: {}", e);
            // If pkg-config fails, try falling back to default lib search paths
            println!("cargo:rustc-link-search=native=/usr/lib");
            println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/lib");
        }
    }
}
