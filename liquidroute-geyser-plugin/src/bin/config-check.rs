use {
    anyhow::{anyhow, Result},
    clap::Parser,
    liquidroute_geyser_plugin::config::Config,
    std::path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[arg(short, long)]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let config_path = args.config;
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