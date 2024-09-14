use std::{
    ops::{Index, IndexMut},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::Song;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Library {
    songs: Vec<Song>,
}

impl Library {
    /// Loads the library from given path (returns empty library on error)
    pub fn load(path: &PathBuf) -> Library {
        let mut lib = match std::fs::read_to_string(path) {
            Ok(l) => serde_json::from_str::<Library>(&l).unwrap_or_default(),
            Err(_) => Library::default(),
        };
        if let Ok(song) = Song::from_file(&PathBuf::from(
            "/home/martan03/Music/OneRepublic - Human (Deluxe)/01 - Run.mp3",
        )) {
            lib.songs.push(song);
        }
        lib
    }

    /// Saves the library to the given path
    pub fn save(&self, path: &PathBuf) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::File::create(path)?;

        let json = serde_json::to_string::<Library>(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Gets length of the library
    pub fn len(&self) -> usize {
        self.songs.len()
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
