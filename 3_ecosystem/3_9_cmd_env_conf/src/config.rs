use serde::Deserialize;
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

#[derive(Debug, Deserialize, Default)]
pub struct Server {
    #[serde(default = "external_url")]
    pub external_url: String,

    #[serde(default = "http_port")]
    pub http_port: u16,

    #[serde(default = "grpc_port")]
    pub grpc_port: u16,

    #[serde(default = "healthz_port")]
    pub healthz_port: u16,

    #[serde(default = "metrics_port")]
    pub metrics_port: u16,
}

#[derive(Debug, Deserialize, Default)]
pub struct Db {
    pub mysql: Mysql,
}

#[derive(Debug, Deserialize, Default)]
pub struct Mysql {
    #[serde(default = "mysql_host")]
    pub host: String,

    #[serde(default = "mysql_port")]
    pub port: u16,

    #[serde(default = "mysql_dating")]
    pub dating: String,

    #[serde(default = "mysql_user")]
    pub user: String,

    #[serde(default)]
    pub pass: String,

    //#[serde(default)]
    pub connections: Connections,
}

#[derive(Debug, Deserialize, Default)]
pub struct Connections {
    #[serde(default = "connections_max_idle")]
    pub max_idle: u64,

    #[serde(default = "connections_max_open")]
    pub max_open: u64,
}

#[derive(Debug, Deserialize, Default)]
pub struct Log {
    pub app: App,
}

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug, Deserialize, Default)]
pub struct Watchdog {
    #[serde(with = "humantime_serde")]
    #[serde(default = "watchdog_period")]
    pub period: Duration,

    #[serde(default = "watchdog_limit")]
    pub limit: u64,

    #[serde(with = "humantime_serde")]
    #[serde(default = "watchdog_lock_timeout")]
    pub lock_timeout: Duration,
}

fn external_url() -> String {
    "http://127.0.0.1".to_string()
}

fn http_port() -> u16 {
    8081
}

fn grpc_port() -> u16 {
    8082
}

fn healthz_port() -> u16 {
    10025
}

fn metrics_port() -> u16 {
    9199
}

fn mysql_host() -> String {
    "127.0.0.1".to_string()
}

fn mysql_port() -> u16 {
    3306
}

fn mysql_dating() -> String {
    "default".to_string()
}
fn mysql_user() -> String {
    "root".to_string()
}

fn connections_max_idle() -> u64 {
    30
}

fn connections_max_open() -> u64 {
    30
}

fn watchdog_limit() -> u64 {
    10
}

fn watchdog_period() -> Duration {
    Duration::from_secs(5)
}

fn watchdog_lock_timeout() -> Duration {
    Duration::from_secs(4)
}
