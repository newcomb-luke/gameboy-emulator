use std::path::{Path, PathBuf};

use preferences::{AppInfo, Preferences};
use serde::{Deserialize, Serialize};

const APP_INFO: AppInfo = AppInfo { name: "Dotra", author: "Luke N" };
const RECENTS_KEY: &str = "recents";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RomEntry {
    name: String,
    path: PathBuf
}

impl RomEntry {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into()
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recents {
    roms: Vec<RomEntry>
}

impl Recents {
    pub fn new() -> Self {
        Self { roms: vec![] }
    }

    pub fn roms(&self) -> &Vec<RomEntry> {
        &self.roms
    }

    pub fn add_if_not_present(&mut self, rom: RomEntry) {
        if self.roms.iter().find(|e| e.path == rom.path).is_none() {
            self.roms.push(rom);
        }
    }

    pub fn remove_missing(&mut self) {
        self.roms.retain(|e| e.path.exists());
    }
}

pub fn get_recents() -> Recents {
    if let Ok(mut recents) = Recents::load(&APP_INFO, RECENTS_KEY) {
        // Remove any ROMs which may have been moved/deleted since last opened
        recents.remove_missing();
        
        // Re-save it in case anything changed
        recents.save(&APP_INFO, RECENTS_KEY).unwrap();

        recents
    } else {
        let empty = Recents::new();

        empty.save(&APP_INFO, RECENTS_KEY).unwrap();

        empty
    }
}

pub fn save_recents(recents: &Recents) {
    recents.save(&APP_INFO, RECENTS_KEY).unwrap()
}