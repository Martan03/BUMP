use std::{path::PathBuf, time::Duration};

use audiotags::Tag;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Song {
    path: PathBuf,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    year: Option<i32>,
    length: Option<Duration>,
    genre: Option<String>,
}

impl Song {
    /// Creates new song from the given file by reading its metadata
    pub fn from_file(file: &PathBuf) -> Result<Self, Error> {
        let tag = Tag::new().read_from_path(file)?;

        Ok(Self {
            path: file.to_owned(),
            title: tag.title().map(String::from),
            artist: tag.artist().map(String::from),
            album: tag.album_title().map(String::from),
            year: tag.year(),
            length: tag.duration().map(Duration::from_secs_f64),
            genre: tag.genre().map(String::from),
        })
    }

    /// Gets the path to the song
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets the song title
    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    /// Gets the artist of the song
    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    /// Gets the album of the song
    pub fn album(&self) -> &Option<String> {
        &self.album
    }

    /// Gets the release year of the song
    pub fn year(&self) -> Option<i32> {
        self.year
    }

    /// Gets the duration of the song
    pub fn length(&self) -> &Option<Duration> {
        &self.length
    }

    /// Gets the genre of the song
    pub fn genre(&self) -> &Option<String> {
        &self.genre
    }
}
