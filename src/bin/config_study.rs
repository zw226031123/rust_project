use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_FILE: &str = "config.toml";
fn main() {
    let c = parse_config(CONFIG_FILE);
    println!("{:#?}", c);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app: ApplicationConfig,
    pub search: SearchConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationConfig {
    pub port: i32,
    pub page_size: i32,
}

pub fn parse_config(file_name: &str) -> Config {
    let content = fs::read_to_string(file_name).expect("Failed to open TOML config file");
    toml::from_str(&content).expect("Failed to parse TOML config file")
}
