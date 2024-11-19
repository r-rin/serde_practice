use std::{error::Error, fs::File, io::Read};

use serde::{Serialize, Deserialize};
use serde_json;
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: u32,
    price: u32,
    duration: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: u32,
    duration: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: u32,
    shard_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug)]
enum RequestType {
    success,
    error,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Debug {
    duration: String,
    at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    r#type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut file = File::open("request.json")?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    let request: Request = serde_json::from_str(&json_str)?;
    let yaml_str = serde_yaml::to_string(&request)?;
    dbg!(yaml_str);
    Ok(())
}
