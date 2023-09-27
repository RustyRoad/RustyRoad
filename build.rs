use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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

    Command::new("cmd")
        .args(&["/C", "nvm use 18.17.1 && npm run build"])
        .current_dir("./grapesjs-tailwind")
        .status()
        .unwrap();

    // install grapesjs-cli
    Command::new("cmd")
        .args(&["/C", "npm install -g grapesjs-cli"])
        .status()
        .unwrap();

  
        // Get the path to the directory containing Cargo.toml
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not set");
    
    // Print the manifest directory
    println!("Manifest dir: {:?}", manifest_dir);

    // Construct the absolute path to the source file
    let source_path = Path::new(&manifest_dir).join("grapesjs-tailwind/dist/grapesjs-tailwind.min.js");
    
    // Print the absolute path of the source file.
    println!("Source file path: {:?}", source_path);

    // Check whether the source file exists and print a message.
    if source_path.exists() {
        println!("Source file exists.");
    } else {
        println!("Source file does not exist.");
    }
    
    // Perform the file copy operation.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("grapesjs-tailwind.min.js");
    fs::copy(&source_path, &dest_path)
        .unwrap_or_else(|err| panic!("Failed to copy file: {}", err));
}
