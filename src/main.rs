use std::{fs::File, io::Read, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_yaml::to_string as to_yaml;
use toml::to_string as to_toml;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() {
    let mut file = File::open("./src/request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json::from_str(&json_str).unwrap();
    let yaml_str = to_yaml(&request).unwrap();
    println!("YAML:\n{}", yaml_str);
    let toml_str = to_toml(&request).unwrap();
    println!("TOML:\n{}", toml_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut file = File::open("./src/request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();

        assert_eq!(request.request_type, RequestType::Success);

        assert_eq!(
            request.stream.user_id.to_string(),
            "8d234120-0bda-49b2-b7e0-fbd3912f6cbf"
        );
        assert_eq!(request.stream.is_private, false);
        assert_eq!(request.stream.settings, 45345);
        assert_eq!(
            request.stream.shard_url.to_string(),
            "https://n3.example.com/sapi"
        );

        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.public_tariff.price, 100);
        assert_eq!(
            request.stream.public_tariff.duration,
            Duration::from_secs(3600)
        );
        assert_eq!(
            request.stream.public_tariff.description,
            "test public tariff"
        );

        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(
            request.stream.private_tariff.duration,
            Duration::from_secs(60)
        );
        assert_eq!(
            request.stream.private_tariff.description,
            "test private tariff"
        );

        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[0].price, 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
        assert_eq!(request.gifts[1].id, 2);
        assert_eq!(request.gifts[1].price, 3);
        assert_eq!(request.gifts[1].description, "Gift 2");

        assert_eq!(request.debug.duration, Duration::from_millis(234));
        assert_eq!(
            request.debug.at,
            "2019-06-28T08:35:46+00:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        );
    }
}
