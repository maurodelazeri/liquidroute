use {
    serde::{Deserialize, Serialize},
    std::{fs::File, io::Read, path::Path},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to open config file: {0}")]
    FileOpen(#[source] std::io::Error),
    
    #[error("Failed to read config file: {0}")]
    FileRead(#[source] std::io::Error),
    
    #[error("Failed to parse config file: {0}")]
    Parse(#[source] serde_json::Error),
}

/// Log configuration
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LogConfig {
    /// Log level (e.g., info, debug, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,
}

/// Main plugin configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Path to the plugin shared library
    pub libpath: String,
    
    /// Logging configuration
    #[serde(default = "default_log_config")]
    pub log: LogConfig,
    
    /// LiquidRoute specific configuration
    #[serde(default)]
    pub liquidroute: LiquidRouteConfig,
}

/// LiquidRoute specific configuration
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LiquidRouteConfig {
    /// Whether to track token accounts
    #[serde(default = "default_track_token_accounts")]
    pub track_token_accounts: bool,
    
    /// Number of worker threads
    #[serde(default = "default_thread_count")]
    pub thread_count: usize,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_config() -> LogConfig {
    LogConfig {
        level: default_log_level(),
    }
}

fn default_track_token_accounts() -> bool {
    true
}

fn default_thread_count() -> usize {
    4
}

impl Config {
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let mut file = File::open(path).map_err(ConfigError::FileOpen)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(ConfigError::FileRead)?;
        
        serde_json::from_str(&contents).map_err(ConfigError::Parse)
    }
}