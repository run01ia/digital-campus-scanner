use std::fs;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub camera: CameraConfig,
    pub wifi: WifiConfig,
}


#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub output_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub enabled: bool,
    pub device_index: i32,
    pub image_format: String,
}

#[derive(Debug, Deserialize)]
pub struct WifiConfig {
    pub enabled: bool,
    pub scan_interval_ms: u64,
    pub scan_wait_ms: u64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
