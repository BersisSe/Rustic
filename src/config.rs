use clap::builder::Str;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub title: String,
    pub theme: String
}

impl Config {
    pub fn load(config_path: &str) -> Self {
        let content = fs::read_to_string(config_path).expect("Unable to read config file");
        serde_json::from_str(&content).expect("Invalid JSON in config file")
    }
    pub fn new(base_url: String, title: String, ) -> Config{
        return Config{
            base_url,
            title,
            theme: "default".to_string()
        };
    }
}