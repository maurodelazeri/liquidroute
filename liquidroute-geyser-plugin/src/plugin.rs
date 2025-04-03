use {
    crate::{
        config::{Config, LiquidRouteConfig},
        get_thread_name,
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
use tokio::runtime::{Builder, Runtime};

#[derive(Debug)]
pub struct PluginInner {
    #[allow(dead_code)]
    runtime: Runtime,  // Will be used in future implementations for async processing
    is_shutdown: AtomicBool,
    config: LiquidRouteConfig,
}

#[derive(Debug)]
pub struct LiquidRoutePlugin {
    inner: Arc<PluginInner>,
}

impl LiquidRoutePlugin {
    pub fn new(config: Config) -> Result<Self, GeyserPluginError> {
        let _ = crate::debug_log_to_file("LiquidRoutePlugin::new called");
        
        // Try to initialize logging and catch any panics
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            info!("Initializing LiquidRoute plugin");
            info!("LiquidRoute config: {:?}", config.liquidroute);
        }));
        
        if result.is_err() {
            let _ = crate::debug_log_to_file("Logging initialization caused a panic, continuing without standard logging");
        }

        // Create tokio runtime with configured thread count
        let runtime = match Builder::new_current_thread()
            .enable_all()
            .thread_name_fn(get_thread_name)
            .build() {
                Ok(runtime) => {
                    let _ = crate::debug_log_to_file("Successfully created tokio runtime");
                    runtime
                },
                Err(e) => {
                    let error_msg = format!("Failed to create tokio runtime: {}", e);
                    let _ = crate::debug_log_to_file(&error_msg);
                    return Err(GeyserPluginError::AccountsUpdateError { 
                        msg: format!("Failed to create tokio runtime: {}", e) 
                    });
                }
            };

        let inner = Arc::new(PluginInner {
            runtime,
            is_shutdown: AtomicBool::new(false),
            config: config.liquidroute,
        });

        let _ = crate::debug_log_to_file(&format!("LiquidRoute plugin initialization complete: {}", plugin_version()));

        Ok(Self { inner })
    }
}

impl GeyserPlugin for LiquidRoutePlugin {
    fn name(&self) -> &'static str {
        "LiquidRoutePlugin"
    }
    
    fn setup_logger(&self, logger: &'static dyn log::Log, level: LevelFilter) -> PluginResult<()> {
        let _ = crate::debug_log_to_file(&format!("Setting up logger with level: {:?}", level));
        
        // Try to register the logger provided by Agave
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            log::set_max_level(level);
            if let Err(err) = log::set_logger(logger) {
                let msg = format!("Failed to set logger: {}", err);
                let _ = crate::debug_log_to_file(&msg);
                // Convert to a custom error message instead of wrapping the SetLoggerError
                // because SetLoggerError doesn't implement std::error::Error
                return Err(GeyserPluginError::SlotStatusUpdateError { 
                    msg: format!("Failed to set logger: {}", err) 
                });
            }
            Ok(())
        }));
        
        match result {
            Ok(setup_result) => setup_result,
            Err(_) => {
                let _ = crate::debug_log_to_file("Logger setup caused a panic, continuing with debug file logging only");
                Ok(()) // Don't propagate the error since we have our fallback logging
            }
        }
    }

    fn on_load(&mut self, config_file: &str, is_reload: bool) -> PluginResult<()> {
        let _ = crate::debug_log_to_file(&format!("Loading LiquidRoute plugin from config: {}, reload: {}", config_file, is_reload));
        
        // Try to log using standard logger, but catch any panics
        let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
            info!("Loading LiquidRoute plugin from config: {}, reload: {}", config_file, is_reload);
        }));
        
        Ok(())
    }

    fn on_unload(&mut self) {
        let _ = crate::debug_log_to_file("Unloading LiquidRoute plugin");
        
        // Try to log using standard logger, but catch any panics
        let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
            info!("Unloading LiquidRoute plugin");
        }));
        
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

        let _ = crate::debug_log_to_file(&format!("Account update received for slot: {}, startup: {}", slot, is_startup));
        
        // Safely try to log via standard logger
        let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
            debug!("Account update received for slot: {}, startup: {}", slot, is_startup);
        }));

        // Here we would process the account update, but for now just log a placeholder message
        if self.inner.config.track_token_accounts {
            let _ = crate::debug_log_to_file("Processing token account update (placeholder)");
        }

        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: u64,
        parent: Option<u64>,
        status: &SlotStatus,
    ) -> PluginResult<()> {
        if self.inner.is_shutdown.load(Ordering::SeqCst) {
            return Ok(());
        }

        let _ = crate::debug_log_to_file(&format!("Slot status update: slot={}, parent={:?}, status={:?}", slot, parent, status));
        
        // Process slot status update placeholder in a panic-safe manner
        match status {
            SlotStatus::Processed => {
                let _ = crate::debug_log_to_file(&format!("Processed slot: {}", slot));
            }
            SlotStatus::Confirmed => {
                let _ = crate::debug_log_to_file(&format!("Confirmed slot: {}", slot));
            }
            SlotStatus::Rooted => {
                let _ = crate::debug_log_to_file(&format!("Rooted slot: {}", slot));
            }
            SlotStatus::FirstShredReceived => {
                let _ = crate::debug_log_to_file(&format!("First shred received for slot: {}", slot));
            }
            SlotStatus::Completed => {
                let _ = crate::debug_log_to_file(&format!("Completed slot: {}", slot));
            }
            SlotStatus::CreatedBank => {
                let _ = crate::debug_log_to_file(&format!("Created bank for slot: {}", slot));
            }
            SlotStatus::Dead(reason) => {
                let _ = crate::debug_log_to_file(&format!("Dead slot: {}, reason: {}", slot, reason));
            }
        }

        Ok(())
    }

    fn notify_block_metadata(
        &self,
        _block_info: ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        if self.inner.is_shutdown.load(Ordering::SeqCst) {
            return Ok(());
        }

        let _ = crate::debug_log_to_file("Block metadata notification received");

        // Process block metadata placeholder
        // In the future we would analyze this for DEX-related blocks

        Ok(())
    }

    fn notify_transaction(
        &self,
        _transaction_info: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> PluginResult<()> {
        if self.inner.is_shutdown.load(Ordering::SeqCst) {
            return Ok(());
        }

        let _ = crate::debug_log_to_file(&format!("Transaction notification for slot: {}", slot));

        // Process transaction placeholder
        // In the future, this is where we would analyze DEX transactions

        Ok(())
    }

    fn notify_entry(
        &self,
        _entry_info: ReplicaEntryInfoVersions,
    ) -> PluginResult<()> {
        if self.inner.is_shutdown.load(Ordering::SeqCst) {
            return Ok(());
        }

        let _ = crate::debug_log_to_file("Entry notification received");

        // Process entry placeholder

        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
    
    fn entry_notifications_enabled(&self) -> bool {
        false
    }
    
    fn notify_end_of_startup(&self) -> PluginResult<()> {
        let _ = crate::debug_log_to_file("End of startup notification received");
        Ok(())
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the LiquidRoutePlugin pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    // Initialize debug logging
    crate::init_debug_log();
    let _ = crate::debug_log_to_file("_create_plugin called");
    
    // Try to find config in different locations
    let config_paths = [
        std::env::var("LIQUIDROUTE_GEYSER_PLUGIN_CONFIG").ok(),
        Some("config/liquidroute.json".to_string()),
        Some("/etc/agave/liquidroute.json".to_string()),
        Some("liquidroute.json".to_string())
    ];
    
    let mut config_error = String::new();
    let mut config = None;
    
    for path in config_paths.iter().flatten() {
        let _ = crate::debug_log_to_file(&format!("Trying to read config from: {}", path));
        
        match Config::read_from(path) {
            Ok(cfg) => {
                let _ = crate::debug_log_to_file(&format!("Successfully read config from {}: {:?}", path, cfg));
                config = Some(cfg);
                break;
            },
            Err(err) => {
                let msg = format!("Failed to read config from {}: {}", path, err);
                let _ = crate::debug_log_to_file(&msg);
                config_error = msg;
            }
        }
    }
    
    let config = match config {
        Some(config) => config,
        None => {
            let error_msg = format!("Failed to read config from any location. Last error: {}", config_error);
            let _ = crate::debug_log_to_file(&error_msg);
            
            // Try to log using standard logger, but catch any panics
            let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
                error!("{}", error_msg);
            }));
            
            return Box::into_raw(Box::new(DummyPlugin {}));
        }
    };

    match LiquidRoutePlugin::new(config) {
        Ok(plugin) => {
            let _ = crate::debug_log_to_file("Successfully created plugin");
            Box::into_raw(Box::new(plugin))
        },
        Err(err) => {
            let error_msg = format!("Failed to create plugin: {}", err);
            let _ = crate::debug_log_to_file(&error_msg);
            
            // Try to log using standard logger, but catch any panics
            let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
                error!("{}", error_msg);
            }));
            
            Box::into_raw(Box::new(DummyPlugin {}))
        }
    }
}

/// A dummy plugin that does nothing, used as a fallback when plugin creation fails
#[derive(Debug)]
struct DummyPlugin;

impl GeyserPlugin for DummyPlugin {
    fn name(&self) -> &'static str {
        "DummyPlugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        let _ = crate::debug_log_to_file("Dummy plugin loaded - this indicates a configuration error");
        
        // Try to log using standard logger, but catch any panics
        let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
            error!("Dummy plugin loaded - this indicates a configuration error");
        }));
        
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