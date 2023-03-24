fn main() {
    // Specify the library search path
    println!("cargo:rustc-link-search=native=C:\\Users\\riley\\vcpkg\\installed\\x64-windows\\lib");
    // Specify the library to link
    println!("cargo:rustc-link-lib=libpq");
}
