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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    // General plugin configuration
    #[serde(default = "default_libpath")]
    pub libpath: String,
    
    // DEX-specific configuration
    #[serde(default)]
    pub track_token_accounts: bool,
    
    // Performance configuration
    #[serde(default = "default_thread_count")]
    pub thread_count: usize,
}

fn default_libpath() -> String {
    "libsolana_geyser_plugin.so".to_string()
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