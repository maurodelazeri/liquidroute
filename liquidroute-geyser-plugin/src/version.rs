pub const LIQUIDROUTE_PLUGIN_VERSION: &str = env!("LIQUIDROUTE_PLUGIN_VERSION");

pub fn plugin_version() -> String {
    format!("LiquidRoute Geyser Plugin {}", LIQUIDROUTE_PLUGIN_VERSION)
}