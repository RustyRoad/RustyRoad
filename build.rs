extern crate pkg_config;
use std::env;
use std::process::Command;

fn main() {
    println!("VCPKG_ROOT: {:?}", env::var("VCPKG_ROOT"));
    println!("POSTGRES_LIB_PATH: {:?}", env::var("POSTGRES_LIB_PATH"));
    if cfg!(target_os = "windows") {
        let pg_path = match env::var("POSTGRES_LIB_PATH") {
            Ok(val) => val,
            Err(_) => panic!("Could not find environment variable POSTGRES_LIB_PATH. Make sure it's set and valid."),
        };

        println!("cargo:rustc-link-search=native={}", pg_path);

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

    println!("cargo:rustc-link-lib=libpq");

    Command::new("cmd")
        .args(&["/C", "nvm use 18.17.1 && npm run build"])
        .current_dir("./grapesjs-tailwind")
        .status()
        .unwrap();
}