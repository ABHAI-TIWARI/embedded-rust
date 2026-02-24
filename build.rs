// Cargo build script for embedded linker setup.
fn main() {
    // Pass linker script argument required by esp-hal applications.
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
}
