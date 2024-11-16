use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::fs;



#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub canonical: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaDataCollection {
    pub pages: HashMap<String, MetaData>,
}


impl MetaDataCollection {
    pub fn load(file_path: &str) -> io::Result<Self> {
        let file_content = fs::read_to_string(file_path)?;
        let metadata: MetaDataCollection = serde_json::from_str(&file_content)?;
        Ok(metadata)
    }
}