use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // Print the plugin version during build
    println!("cargo:rustc-env=LIQUIDROUTE_PLUGIN_VERSION={}", env!("CARGO_PKG_VERSION"));
    
    Ok(())
}