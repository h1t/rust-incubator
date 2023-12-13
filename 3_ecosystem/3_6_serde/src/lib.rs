use serde::{Deserialize, Serialize};
use std::time::Duration;
use time::serde::iso8601;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Request {
    #[serde(rename = "type")]
    request_type: String,

    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,

    #[serde(with = "iso8601")]
    at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Gift {
    id: i64,
    price: i64,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: i64,
    shard_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicTariff {
    id: i64,
    price: i64,

    #[serde(with = "humantime_serde")]
    duration: Duration,

    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateTariff {
    client_price: i64,

    #[serde(with = "humantime_serde")]
    duration: Duration,

    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_request() -> Request {
        serde_json::from_str(include_str!("../request.json")).expect("can't parse request")
    }

    #[test]
    fn test_yaml() {
        let request = get_request();
        let yaml_data = serde_yaml::to_string(&request).expect("can't serialize yaml");
        let yaml_request = serde_yaml::from_str(&yaml_data).expect("can't parse yaml");

        assert_eq!(request, yaml_request);
    }

    #[test]
    fn test_toml() {
        let request = get_request();
        let toml_data = toml::to_string(&request).expect("can't serialize toml");
        let toml_request = toml::from_str(&toml_data).expect("can't parse toml");

        assert_eq!(request, toml_request);
    }
}
