# LiquidRoute Geyser Plugin

A Solana Geyser plugin for processing DEX-related on-chain data in real-time to support the LiquidRoute advanced DEX aggregator with the AHMO-NG (Adaptive Hierarchical Multi-Optimizer - Next Generation) algorithm.

## Overview

This plugin connects directly to Solana validators to monitor DEX account updates, transactions, and slot status, enabling ultra-low latency DEX aggregation for optimal trade routing. It provides the backend infrastructure for the LiquidRoute DEX aggregator.

## Features

- Real-time monitoring of DEX-related accounts
- Efficient parsing of DEX pool states and order books
- Support for multiple DEX protocols (Raydium, Orca, OpenBook, Phoenix, etc.)
- Optimized for low-latency price impact calculations
- Direct validator integration for minimal overhead
- Implementation of the AHMO-NG algorithm for optimal trade routing

## Build

```bash
cargo build --release
```

The build process will create a shared library (`.so` on Linux, `.dylib` on macOS) located at `target/release/liquidroute_geyser_plugin.*`.

## Configuration

Create a config file based on the example `config.json`:

```json
{
  "libpath": "/path/to/liquidroute_geyser_plugin.so",
  "log": {
    "level": "info"
  },
  "liquidroute": {
    "track_token_accounts": true,
    "thread_count": 8
  }
}
```

- `libpath`: Path to the compiled plugin library (must be absolute)
- `log`: Logging configuration
  - `level`: Log level (info, debug, warn, error)
- `liquidroute`: Plugin-specific configuration
  - `track_token_accounts`: Whether to track token account updates
  - `thread_count`: Number of worker threads for processing updates

## Validation

Validate your configuration with:

```bash
cargo run --bin config-check -- --config /path/to/config.json
```

## Usage with Solana Validator

1. Build the plugin
2. Copy the shared library to your desired location
3. Create a configuration file
4. Add the plugin to your validator configuration:

```bash
solana-validator ... --geyser-plugin-config /path/to/config.json
```

## License

Apache 2.0
