fn main() {
    // Set the LIQUIDROUTE_PLUGIN_VERSION environment variable
    // for use in version.rs
    println!("cargo:rustc-env=LIQUIDROUTE_PLUGIN_VERSION=0.1.0");
}