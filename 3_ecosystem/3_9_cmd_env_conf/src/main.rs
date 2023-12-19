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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LogLevel;
    use std::time::Duration;

    #[test]
    fn test_default_values() {
        let config: Result<AppConfig, _> = Config::builder()
            .add_source(File::new("config.toml", FileFormat::Toml))
            .build()
            .and_then(|conf| conf.try_deserialize());
        assert!(config.is_ok());
        let config = config.unwrap();

        assert!(!config.mode.debug);

        assert_eq!(config.server.external_url, "http://127.0.0.1");
        assert_eq!(config.server.http_port, 8081);
        assert_eq!(config.server.grpc_port, 8082);
        assert_eq!(config.server.healthz_port, 10025);
        assert_eq!(config.server.metrics_port, 9199);

        assert_eq!(config.db.mysql.host, "127.0.0.1");
        assert_eq!(config.db.mysql.port, 3306);
        assert_eq!(config.db.mysql.dating, "default");
        assert_eq!(config.db.mysql.user, "root");
        assert_eq!(config.db.mysql.pass, "");

        assert_eq!(config.db.mysql.connections.max_idle, 30);
        assert_eq!(config.db.mysql.connections.max_open, 30);

        assert_eq!(config.log.app.level, LogLevel::Info);

        assert_eq!(config.background.watchdog.period, Duration::from_secs(5));
        assert_eq!(config.background.watchdog.limit, 10);
        assert_eq!(
            config.background.watchdog.lock_timeout,
            Duration::from_secs(4)
        );
    }
}
