use std::{
    fs::read_dir,
    ops::{Index, IndexMut},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{core::config::Config, error::Error};

use super::Song;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Library {
    songs: Vec<Song>,
}

impl Library {
    /// Loads the library (returns empty library on error)
    pub fn load(config: &Config) -> Library {
        let mut lib = match std::fs::read_to_string(config.lib_path()) {
            Ok(l) => serde_json::from_str::<Library>(&l).unwrap_or_default(),
            Err(_) => Library::default(),
        };
        _ = lib.find(config);
        lib
    }

    /// Saves the library to the given path
    pub fn _save(&self, path: &PathBuf) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::File::create(path)?;

        let json = serde_json::to_string::<Library>(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn find(&mut self, config: &Config) -> Result<(), Error> {
        self.songs = vec![];
        for dir in config.dirs() {
            self._find(config, &dir)?;
        }
        Ok(())
    }

    /// Gets length of the library
    pub fn len(&self) -> usize {
        self.songs.len()
    }
}

impl Library {
    fn _find(&mut self, config: &Config, dir: &PathBuf) -> Result<(), Error> {
        for entry in read_dir(dir)? {
            let path = entry?.path();

            if path.is_dir() && config.recursive_search() {
                self._find(config, &path)?;
                continue;
            }

            let Some(ext) = path.extension() else {
                continue;
            };
            let ext = ext.to_string_lossy();
            if !config.extensions().iter().any(|e| e == ext.as_ref()) {
                continue;
            }

            let song = Song::from_file(&path)?;
            self.songs.push(song);
        }
        Ok(())
    }
}

impl Index<usize> for Library {
    type Output = Song;

    fn index(&self, index: usize) -> &Self::Output {
        &self.songs[index]
    }
}

impl IndexMut<usize> for Library {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.songs[index]
    }
}
