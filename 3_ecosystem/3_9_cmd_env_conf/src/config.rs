use serde::Deserialize;
use smart_default::SmartDefault;
use std::time::Duration;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub mode: Mode,
    pub server: Server,
    pub db: Db,
    pub log: Log,
    pub background: Background,
}

#[derive(Debug, Deserialize, Default)]
pub struct Mode {
    #[serde(default)]
    pub debug: bool,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Server {
    #[default = "http://127.0.0.1"]
    pub external_url: String,

    #[default = 8081]
    pub http_port: u16,

    #[default = 8082]
    pub grpc_port: u16,

    #[default = 10025]
    pub healthz_port: u16,

    #[default = 9199]
    pub metrics_port: u16,
}

#[derive(Debug, Deserialize, Default)]
pub struct Db {
    pub mysql: Mysql,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Mysql {
    #[default = "127.0.0.1"]
    pub host: String,

    #[default = 3306]
    pub port: u16,

    #[default = "default"]
    pub dating: String,

    #[default = "root"]
    pub user: String,

    pub pass: String,

    pub connections: Connections,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Connections {
    #[default = 30]
    pub max_idle: u64,

    #[default = 30]
    pub max_open: u64,
}

#[derive(Debug, Deserialize, Default)]
pub struct Log {
    pub app: App,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,

    #[default]
    Info,

    Debug,
    Trace,
}

#[derive(Debug, Deserialize, Default)]
pub struct App {
    #[serde(default)]
    pub level: LogLevel,
}

#[derive(Debug, Deserialize, Default)]
pub struct Background {
    pub watchdog: Watchdog,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Watchdog {
    #[serde(with = "humantime_serde")]
    #[default(Duration::from_secs(5))]
    pub period: Duration,

    #[default = 10]
    pub limit: u64,

    #[serde(with = "humantime_serde")]
    #[default(Duration::from_secs(4))]
    pub lock_timeout: Duration,
}
