use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Constains bump configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    dirs: Vec<PathBuf>,
    extensions: Vec<String>,
    recursive_search: bool,
    start_load: bool,

    lib_path: PathBuf,
}

impl Config {
    /// Loads config from the config directory
    pub fn load() -> Self {
        let dir = Config::path();
        let conf = match std::fs::read_to_string(&dir) {
            Ok(c) => serde_json::from_str::<Self>(&c).ok(),
            Err(_) => None,
        };

        match conf {
            Some(conf) => conf,
            None => {
                let conf = Config::default();
                _ = conf.save();
                conf
            }
        }
    }

    /// Saves current config to the file
    pub fn save(&self) -> Result<(), Error> {
        let mut path = Config::dir();
        std::fs::create_dir_all(&path)?;

        path.push("config.json");
        std::fs::File::create(&path)?;

        let json = serde_json::to_string_pretty::<Self>(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Gets song source directories
    pub fn dirs(&self) -> &Vec<PathBuf> {
        &self.dirs
    }

    /// Gets extensions list to be loaded
    pub fn extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    /// Returns true when recursive search is allowed, else false
    pub fn recursive_search(&self) -> bool {
        self.recursive_search
    }

    /// Returns true when start load is allowed, else false
    pub fn _start_load(&self) -> bool {
        self.start_load
    }

    /// Gets the library path
    pub fn lib_path(&self) -> &PathBuf {
        &self.lib_path
    }
}

impl Config {
    /// Gets the config directory
    fn dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or(PathBuf::from("."))
            .join(Config::app_id())
    }

    /// Gets the config file path
    fn path() -> PathBuf {
        Config::dir().join("config.json")
    }

    /// Gets the app id based on the compile mode
    fn app_id() -> String {
        if cfg!(debug_assertions) {
            "bump_debug".to_string()
        } else {
            "bump".to_string()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dirs: Config::default_dirs(),
            extensions: Config::default_extensions(),
            recursive_search: true,
            start_load: true,
            lib_path: Config::default_lib_path(),
        }
    }
}

// Default function for each attribute
impl Config {
    fn default_dirs() -> Vec<PathBuf> {
        let dir = dirs::audio_dir().unwrap_or(PathBuf::from("."));
        vec![dir]
    }

    fn default_extensions() -> Vec<String> {
        vec!["mp3".to_owned(), "flac".to_owned(), "m4a".to_owned()]
    }

    fn default_lib_path() -> PathBuf {
        Config::dir().join("library.json")
    }
}
