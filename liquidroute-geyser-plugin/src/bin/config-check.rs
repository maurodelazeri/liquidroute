use {
    anyhow::{anyhow, Result},
    clap::{App, Arg},
    liquidroute_geyser_plugin::config::Config,
    std::path::PathBuf,
};

fn main() -> Result<()> {
    let matches = App::new("LiquidRoute Config Checker")
        .version(env!("CARGO_PKG_VERSION"))
        .author("LiquidRoute Team <info@liquidroute.com>")
        .about("Validates LiquidRoute Geyser plugin configuration")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Path to the config file")
                .required(true)
        )
        .get_matches();
    
    let config_path = PathBuf::from(matches.value_of("config").unwrap());
    if !config_path.exists() {
        return Err(anyhow!("Config file not found: {:?}", config_path));
    }
    
    match Config::read_from(&config_path) {
        Ok(config) => {
            println!("Configuration is valid!");
            println!("libpath: {}", config.libpath);
            println!("log level: {}", config.log.level);
            println!("LiquidRoute configuration:");
            println!("  track_token_accounts: {}", config.liquidroute.track_token_accounts);
            println!("  thread_count: {}", config.liquidroute.thread_count);
            Ok(())
        }
        Err(err) => {
            Err(anyhow!("Invalid configuration: {}", err))
        }
    }
}