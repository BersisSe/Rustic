use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    pub title: String,
    pub theme: String,
    pub template_path: String,
}

impl Config {
    pub fn load(config_path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&content)?)  
    }
    pub fn new(base_url: String, title: String, theme: String ,template_path: String) -> Config {
        return Config {
            base_url,
            title,
            theme,
            template_path
        };
    }
}
