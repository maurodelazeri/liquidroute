pub mod plugin;
pub mod version;

#[no_mangle]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn agave_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin {
    Box::into_raw(Box::new(plugin::LiquidRoutePlugin::new()))
}