use {
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
        ReplicaEntryInfoVersions, ReplicaTransactionInfoVersions, Result as PluginResult,
        SlotStatus,
    },
    log::LevelFilter,
};

#[derive(Debug)]
pub struct LiquidRoutePlugin;

impl LiquidRoutePlugin {
    pub fn new() -> Self {
        println!("LiquidRoutePlugin: Creating new instance");
        Self {}
    }
}

impl GeyserPlugin for LiquidRoutePlugin {
    fn name(&self) -> &'static str {
        "LiquidRoutePlugin"
    }

    fn on_load(&mut self, config_file: &str, is_reload: bool) -> PluginResult<()> {
        println!("LiquidRoutePlugin: Loaded with config: {}, reload: {}", config_file, is_reload);
        Ok(())
    }

    fn on_unload(&mut self) {
        println!("LiquidRoutePlugin: Unloading");
    }

    fn update_account(
        &self,
        _account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        // Only log during startup phase and only every 10,000 accounts to avoid spam
        if is_startup && slot % 10000 == 0 {
            println!("LiquidRoutePlugin: Processing account at slot: {}", slot);
        }
        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: u64,
        parent: Option<u64>,
        status: &SlotStatus,
    ) -> PluginResult<()> {
        // Log only every 10,000 slots to avoid spam
        if slot % 10000 == 0 {
            println!("LiquidRoutePlugin: Slot {} (parent: {:?}) status: {:?}",
                     slot, parent, status);
        }
        Ok(())
    }

    fn notify_block_metadata(
        &self,
        block_info: ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        // Log only occasionally based on slot to avoid spam
        if let ReplicaBlockInfoVersions::V0_0_1(info) = &block_info {
            // Use the slot field which is available
            if info.slot % 10000 == 0 {
                println!("LiquidRoutePlugin: Block at slot {} processed", info.slot);
            }
        }
        Ok(())
    }

    fn notify_transaction(
        &self,
        _transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> PluginResult<()> {
        // Only log occasional transactions
        if slot % 100000 == 0 {
            println!("LiquidRoutePlugin: Transaction processed at slot {}", slot);
        }
        Ok(())
    }

    fn notify_entry(
        &self,
        _entry: ReplicaEntryInfoVersions,
    ) -> PluginResult<()> {
        // No logging here to avoid spam
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        // This is still false to minimize load, but we'll log when this is queried
        println!("LiquidRoutePlugin: Validator checked account_data_notifications_enabled");
        false
    }

    fn transaction_notifications_enabled(&self) -> bool {
        println!("LiquidRoutePlugin: Validator checked transaction_notifications_enabled");
        false
    }

    fn entry_notifications_enabled(&self) -> bool {
        println!("LiquidRoutePlugin: Validator checked entry_notifications_enabled");
        false
    }

    fn setup_logger(&self, _logger: &'static dyn log::Log, level: LevelFilter) -> PluginResult<()> {
        println!("LiquidRoutePlugin: Logger setup with level {:?}", level);
        Ok(())
    }

    fn notify_end_of_startup(&self) -> PluginResult<()> {
        println!("LiquidRoutePlugin: End of startup notification received");
        Ok(())
    }
}