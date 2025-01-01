use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Playlist {
    songs: Vec<usize>,
    current: Option<usize>,
}

impl Playlist {
    /// Creates new playlist with given songs
    pub fn new<T>(songs: T) -> Self
    where
        T: IntoIterator<Item = usize>,
    {
        Self {
            songs: songs.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Sets current song to the given song id
    pub fn current(mut self, id: usize) -> Self {
        self.current = Some(id);
        self
    }

    /// Shuffles the playlist
    pub fn shuffle(&mut self) {
        let id = self.current.map(|v| self.songs[v]);

        let mut rng = rand::thread_rng();
        self.songs.shuffle(&mut rng);

        let Some(id) = id else {
            return;
        };

        if let Some(cur) = self.songs.iter().position(|&x| x == id) {
            self.songs.swap(cur, 0);
            self.current = Some(0);
        }
    }

    /// Sets current to the nth previous song and returns current
    pub fn prev(&mut self, num: usize) -> Option<usize> {
        self.current = Some(self.current?.saturating_sub(num));
        self.current
    }

    /// Sets current to the nth next song and returns current
    pub fn next(&mut self, num: usize) -> Option<usize> {
        let cur = self.current? + num;
        if cur >= self.songs.len() {
            return None;
        }
        self.current = Some(cur);
        self.current
    }
}