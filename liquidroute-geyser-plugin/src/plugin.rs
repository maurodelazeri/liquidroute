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
    log::{info, error, debug},
};
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
        info!("Initializing LiquidRoute plugin");

        // Initialize logger if needed
        // Note: For Solana validators, logging is typically already configured
        // We don't attempt to set it up again, just log at appropriate level
        info!("LiquidRoute config: {:?}", config.liquidroute);

        // Create tokio runtime with configured thread count
        let runtime = Builder::new_current_thread()
            .enable_all()
            .thread_name_fn(get_thread_name)
            .build()
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;

        let inner = Arc::new(PluginInner {
            runtime,
            is_shutdown: AtomicBool::new(false),
            config: config.liquidroute,
        });

        info!("LiquidRoute plugin initialization complete: {}", plugin_version());

        Ok(Self { inner })
    }
}

impl GeyserPlugin for LiquidRoutePlugin {
    fn name(&self) -> &'static str {
        "LiquidRoutePlugin"
    }

    fn on_load(&mut self, config_file: &str, is_reload: bool) -> PluginResult<()> {
        info!("Loading LiquidRoute plugin from config: {}, reload: {}", config_file, is_reload);
        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading LiquidRoute plugin");
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

        debug!("Account update received for slot: {}, startup: {}", slot, is_startup);

        // Here we would process the account update, but for now just log a placeholder message
        if self.inner.config.track_token_accounts {
            debug!("Processing token account update (placeholder)");
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

        debug!("Slot status update: slot={}, parent={:?}, status={:?}", slot, parent, status);

        // Process slot status update placeholder
        match status {
            SlotStatus::Processed => {
                debug!("Processed slot: {}", slot);
            }
            SlotStatus::Confirmed => {
                debug!("Confirmed slot: {}", slot);
            }
            SlotStatus::Rooted => {
                debug!("Rooted slot: {}", slot);
            }
            SlotStatus::FirstShredReceived => {
                debug!("First shred received for slot: {}", slot);
            }
            SlotStatus::Completed => {
                debug!("Completed slot: {}", slot);
            }
            SlotStatus::CreatedBank => {
                debug!("Created bank for slot: {}", slot);
            }
            SlotStatus::Dead(reason) => {
                debug!("Dead slot: {}, reason: {}", slot, reason);
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

        debug!("Block metadata notification received");

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

        debug!("Transaction notification for slot: {}", slot);

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

        debug!("Entry notification received");

        // Process entry placeholder

        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
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
    let config_file = std::env::var("LIQUIDROUTE_GEYSER_PLUGIN_CONFIG")
        .unwrap_or_else(|_| "config/liquidroute.json".to_string());
    
    let _ = crate::debug_log_to_file(&format!("Reading config from: {}", config_file));

    let config = match Config::read_from(&config_file) {
        Ok(config) => {
            let _ = crate::debug_log_to_file(&format!("Successfully read config: {:?}", config));
            config
        },
        Err(err) => {
            let error_msg = format!("Failed to read config from {}: {}", config_file, err);
            let _ = crate::debug_log_to_file(&error_msg);
            error!("{}", error_msg);
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
            error!("{}", error_msg);
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
        error!("Dummy plugin loaded - this indicates a configuration error");
        Ok(())
    }

    fn on_unload(&mut self) {}

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
}