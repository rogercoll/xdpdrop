use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) ipv4s: Vec<String>,
    pub(crate) dns: Vec<String>,
}

impl Config {
    pub(crate) fn new(file_path: &str) -> Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
