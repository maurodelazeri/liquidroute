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
    
    #[error("Invalid config file path: {0}")]
    InvalidPath(String),
}

/// Log configuration
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LogConfig {
    /// Log level (e.g., info, debug, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,
    
    /// Optional log file path
    pub file: Option<String>,
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

impl LiquidRouteConfig {
    /// Validate and clamp configuration values to safe defaults
    pub fn validate(&mut self) {
        // Ensure thread_count is within reasonable limits (1-4)
        if self.thread_count == 0 {
            self.thread_count = 1;
        } else if self.thread_count > 4 {
            self.thread_count = 4;
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_config() -> LogConfig {
    LogConfig {
        level: default_log_level(),
        file: None,
    }
}

fn default_track_token_accounts() -> bool {
    true
}

fn default_thread_count() -> usize {
    // Default to 1 thread to minimize resource usage
    1
}

impl Config {
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let mut file = File::open(&path).map_err(|e| {
            let _ = crate::debug_log_to_file(&format!("Failed to open config file {}: {}", path_str, e));
            ConfigError::FileOpen(e)
        })?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            let _ = crate::debug_log_to_file(&format!("Failed to read config file {}: {}", path_str, e));
            ConfigError::FileRead(e)
        })?;
        
        // Try parsing with serde_json first, with detailed error logging
        match serde_json::from_str(&contents) {
            Ok(config) => {
                let _ = crate::debug_log_to_file(&format!("Successfully parsed config from {}", path_str));
                Ok(config)
            },
            Err(e) => {
                let _ = crate::debug_log_to_file(&format!("Failed to parse config file {}: {}", path_str, e));
                
                // Try parsing with json5 as a fallback (more lenient JSON parser)
                match json5::from_str(&contents) {
                    Ok(config) => {
                        let _ = crate::debug_log_to_file("Successfully parsed config using json5 fallback");
                        Ok(config)
                    },
                    Err(_) => Err(ConfigError::Parse(e)),
                }
            }
        }
    }
}