use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    recents: Vec<PathBuf>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            recents: Vec::new()
        }
    }
}

pub fn read_config() -> Config {
    if let Some(dirs) = ProjectDirs::from("xyz", "Luke N", "GameBoy Emulator") {
        let config_dir_path = dirs.config_local_dir();
        let config_file_path = config_dir_path.join("config.json");

        if config_file_path.exists() {
            if let Ok(config_file) = std::fs::File::open(config_file_path) {
                if let Ok(config) = serde_json::from_reader(config_file) {
                    config
                } else {
                    Config::default()
                }
            } else {
                Config::default()
            }
        } else {
            if let Ok(config_file) = std::fs::File::open(config_file_path) {
                todo!()
            }

            todo!()
        }
    } else {
        Config::default()
    }
}