use serde::Deserialize;
use std::fs;
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Config {
    version: String,
    services: Vec<Service>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Service {
    name: String,
    image: String,
    // #[serde(default)]
    // #[serde(default = Some(default_ports))]
    ports: Option<Vec<String>>,
}
#[allow(dead_code)]
fn default_ports() -> Vec<String> {
    vec!["8080".to_string()]
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string("config-test.yml")?;
    let config: Config = serde_yaml::from_str(&yaml_str)?;

    println!("{:#?}", config);
    Ok(())
}
