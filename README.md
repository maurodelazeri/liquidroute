# LiquidRoute

$ /home/batman/agave/target/release/agave-validator --ledger /home/solana/ledger plugin list
Currently the following plugins are loaded:

1. DummyPlugin

/home/batman/agave/target/release/agave-validator --ledger /home/solana/ledger plugin unload DummyPlugin
Successfully unloaded plugin: DummyPlugin

/home/batman/agave/target/release/agave-validator --ledger /home/solana/ledger plugin list
There are currently no plugins loaded

/home/batman/agave/target/release/agave-validator --ledger /home/solana/ledger plugin load liquidroute.json
Successfully loaded plugin: DummyPlugin

$agave-validator --ledger /home/solana/ledger plugin reload DummyPlugin geyser-plugin-test/plugin.json
Successfully reloaded plugin: DummyPlugin

# At this point, I modified `on_load` to return an `Err` and recompiled.

$agave-validator --ledger /home/solana/ledger plugin reload DummyPlugin geyser-plugin-test/plugin.json
Failed to reload plugin DummyPlugin: JsonRpcError(Error { code: InvalidRequest, message: "Failed to start new plugin (previous plugin was dropped!): Error updating account. Error message: (())", data: None })

/home/batman/agave/target/release/agave-validator --ledger /home/solana/ledger plugin list
There are currently no plugins loaded
