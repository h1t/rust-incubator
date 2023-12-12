mod config;

use ::config::{Config, File, FileFormat};
use clap::{ArgAction, Parser};
use config::Config as AppConfig;
use std::{error::Error, path::PathBuf};

/// Prints its configuration to STDOUT
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enables debug mode
    #[clap(short, long, action = ArgAction::Count)]
    debug: u8,

    /// Path to configuration file
    #[clap(short, long, env = "CONF_FILE", default_value = "config.toml")]
    conf: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { debug: _, conf } = Args::parse();
    let config_path = conf.to_str().ok_or("bad format of path to conf")?;
    let config: AppConfig = Config::builder()
        .add_source(File::new(config_path, FileFormat::Toml))
        .build()?
        .try_deserialize()?;

    println!("{:#?}", config);

    Ok(())
}
