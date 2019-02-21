/**
 * Load arachnid config json.
 */

use std::fs::File;
use std::io::prelude::*;
use self::serde::Deserialize;
extern crate serde;
extern crate serde_json;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub gecko_driver_host: String,
    pub gecko_driver_port: String,
    pub gecko_driver_capabilities: serde_json::Value
}

pub fn load_config() -> Config {
    let mut file = File::open("config.json")
        .expect("Couldn't find config.json. Please set one up!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't parse config.json");
    let data: Config = serde_json::from_str(&contents).expect("Couldn't parse config.json");
    data
}
