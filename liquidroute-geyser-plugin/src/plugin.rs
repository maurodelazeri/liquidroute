use {
    crate::{
        config::{Config, LiquidRouteConfig},
        version::plugin_version,
    },
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
        ReplicaEntryInfoVersions, ReplicaTransactionInfoVersions, Result as PluginResult,
        SlotStatus,
    },
    log::{info, error, debug, LevelFilter},
};
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Extremely minimal implementation to avoid memory allocation issues
#[derive(Debug)]
pub struct PluginInner {
    is_shutdown: AtomicBool,
}

#[derive(Debug)]
pub struct LiquidRoutePlugin {
    inner: Arc<PluginInner>,
}

impl LiquidRoutePlugin {
    pub fn new(_config: Config) -> Result<Self, GeyserPluginError> {
        let _ = crate::debug_log_to_file("Creating absolutely minimal plugin to avoid memory issues");
        
        let inner = Arc::new(PluginInner {
            is_shutdown: AtomicBool::new(false),
        });

        let _ = crate::debug_log_to_file("LiquidRoute plugin initialized with minimal configuration");

        Ok(Self { inner })
    }
}

impl GeyserPlugin for LiquidRoutePlugin {
    fn name(&self) -> &'static str {
        "LiquidRoutePlugin"
    }
    
    fn setup_logger(&self, logger: &'static dyn log::Log, level: LevelFilter) -> PluginResult<()> {
        let _ = crate::debug_log_to_file(&format!("Setting up logger with level: {:?}", level));
        
        // Simple no-op logging setup
        log::set_max_level(level);
        
        // Just return OK regardless of what happens
        Ok(())
    }

    fn on_load(&mut self, config_file: &str, is_reload: bool) -> PluginResult<()> {
        let _ = crate::debug_log_to_file(&format!("Loading plugin from: {}, reload: {}", config_file, is_reload));
        Ok(())
    }

    fn on_unload(&mut self) {
        let _ = crate::debug_log_to_file("Unloading plugin");
        self.inner.is_shutdown.store(true, Ordering::SeqCst);
    }

    fn update_account(
        &self,
        _account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if self.inner.is_shutdown.load(Ordering::SeqCst) {
            return Ok(());
        }
        
        // Do absolutely nothing with accounts to avoid memory issues
        if slot % 10000 == 0 {
            let _ = crate::debug_log_to_file(&format!("Received account update for slot: {}, startup: {}", slot, is_startup));
        }
        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: u64,
        _parent: Option<u64>,
        status: &SlotStatus,
    ) -> PluginResult<()> {
        if self.inner.is_shutdown.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Log only occasionally to avoid too much I/O
        if slot % 10000 == 0 {
            let _ = crate::debug_log_to_file(&format!("Slot status update: slot={}, status={:?}", slot, status));
        }
        Ok(())
    }

    fn notify_block_metadata(
        &self,
        _block_info: ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        // Do absolutely nothing with block data to avoid memory issues
        Ok(())
    }

    fn notify_transaction(
        &self,
        _transaction_info: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> PluginResult<()> {
        // Do absolutely nothing with transaction data to avoid memory issues
        Ok(())
    }

    fn notify_entry(
        &self,
        _entry_info: ReplicaEntryInfoVersions,
    ) -> PluginResult<()> {
        // Do absolutely nothing with entry data to avoid memory issues
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        // Disable account notifications entirely
        false
    }

    fn transaction_notifications_enabled(&self) -> bool {
        // Disable transaction notifications entirely
        false
    }
    
    fn entry_notifications_enabled(&self) -> bool {
        // Disable entry notifications entirely
        false
    }
    
    fn notify_end_of_startup(&self) -> PluginResult<()> {
        let _ = crate::debug_log_to_file("End of startup notification received");
        Ok(())
    }
}

// Super minimal dummy plugin that does nothing in case there's an error
#[derive(Debug)]
struct DummyPlugin;

impl GeyserPlugin for DummyPlugin {
    fn name(&self) -> &'static str {
        "DummyPlugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        let _ = crate::debug_log_to_file("Dummy plugin loaded due to error");
        Ok(())
    }

    fn on_unload(&mut self) {
        let _ = crate::debug_log_to_file("Unloading dummy plugin");
    }

    fn update_account(&self, _account: ReplicaAccountInfoVersions, _slot: u64, _is_startup: bool) -> PluginResult<()> {
        Ok(())
    }

    fn update_slot_status(&self, _slot: u64, _parent: Option<u64>, _status: &SlotStatus) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(&self, _block_info: ReplicaBlockInfoVersions) -> PluginResult<()> {
        Ok(())
    }

    fn notify_transaction(&self, _transaction: ReplicaTransactionInfoVersions, _slot: u64) -> PluginResult<()> {
        Ok(())
    }

    fn notify_entry(&self, _entry: ReplicaEntryInfoVersions) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        false
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
    
    fn entry_notifications_enabled(&self) -> bool {
        false
    }
    
    fn setup_logger(&self, _logger: &'static dyn log::Log, _level: LevelFilter) -> PluginResult<()> {
        Ok(())
    }
    
    fn notify_end_of_startup(&self) -> PluginResult<()> {
        Ok(())
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPlugin pointer.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    // Initialize debug logging
    crate::init_debug_log();
    let _ = crate::debug_log_to_file("_create_plugin called");
    
    // Try to read any config file but ignore the contents
    let config_file = std::env::var("LIQUIDROUTE_GEYSER_PLUGIN_CONFIG")
        .unwrap_or_else(|_| "/tmp/liquidroute.json".to_string());
    
    let _ = crate::debug_log_to_file(&format!("Config file location (not used): {}", config_file));
    
    // Create a minimal config to avoid memory issues
    let config = Config {
        libpath: "unused".to_string(),
        log: Default::default(),
        liquidroute: Default::default(),
    };

    match LiquidRoutePlugin::new(config) {
        Ok(plugin) => {
            let _ = crate::debug_log_to_file("Successfully created minimal plugin");
            Box::into_raw(Box::new(plugin))
        },
        Err(err) => {
            let error_msg = format!("Failed to create plugin: {}", err);
            let _ = crate::debug_log_to_file(&error_msg);
            
            // Always return a dummy plugin to avoid crashes
            Box::into_raw(Box::new(DummyPlugin {}))
        }
    }
}