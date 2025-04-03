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
        Self {}
    }
}

impl GeyserPlugin for LiquidRoutePlugin {
    fn name(&self) -> &'static str {
        "LiquidRoutePlugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        Ok(())
    }

    fn on_unload(&mut self) {
        // No resources to clean up
    }

    fn update_account(
        &self,
        _account: ReplicaAccountInfoVersions,
        _slot: u64,
        _is_startup: bool,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn update_slot_status(
        &self,
        _slot: u64,
        _parent: Option<u64>,
        _status: &SlotStatus,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &self,
        _block_info: ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_transaction(
        &self,
        _transaction: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_entry(
        &self,
        _entry: ReplicaEntryInfoVersions,
    ) -> PluginResult<()> {
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