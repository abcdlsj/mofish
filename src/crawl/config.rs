use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub link: String,
    pub refresh: i64,
}

pub fn read_config(file_path: String) -> Result<Vec<Site>> {
    let config = std::fs::read_to_string(file_path)?;
    let sites: Vec<Site> = serde_json::from_str(&config)?;

    Ok(sites)
}
