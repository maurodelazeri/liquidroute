# LiquidRoute Geyser Plugin

A Solana Geyser plugin for processing DEX-related on-chain data in real-time to support the LiquidRoute advanced DEX aggregator.

## Overview

This plugin connects directly to Solana validators to monitor DEX account updates, transactions, and slot status, enabling ultra-low latency DEX aggregation for optimal trade routing.

## Features

- Real-time monitoring of DEX-related accounts
- Efficient parsing of DEX pool states and order books
- Support for multiple DEX protocols (Raydium, Orca, OpenBook, Phoenix, etc.)
- Optimized for low-latency price impact calculations
- Direct validator integration for minimal overhead

## Build

```bash
cargo build --release
```

## Configuration

Create a config file based on the example `config.json`:

```json
{
  "libpath": "libsolana_geyser_plugin.so",
  "track_token_accounts": true,
  "thread_count": 8
}
```

## Validation

Validate your configuration with:

```bash
cargo run --bin config-check -- --config /path/to/config.json
```

## Usage with Solana Validator

1. Build the plugin
2. Set the environment variable to point to your config:
   ```
   export LIQUIDROUTE_GEYSER_PLUGIN_CONFIG=/path/to/config.json
   ```
3. Configure the Solana validator to use this plugin in `validator.json`

## License

Apache 2.0