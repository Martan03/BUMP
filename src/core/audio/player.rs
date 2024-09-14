use std::fs::File;

use raplay::{
    source::{symph::SymphOptions, Symph},
    Sink,
};

use crate::{
    core::library::{Library, Playlist},
    error::Error,
};

use super::PlayState;

pub struct Player {
    playlist: Playlist,
    state: PlayState,
    sink: Sink,
    symph: SymphOptions,
}

impl Player {
    pub fn new(lib: &Library) -> Self {
        let mut plr = Self {
            playlist: Playlist::new(0..lib.len()).current(0),
            state: PlayState::Paused,
            sink: Default::default(),
            symph: Default::default(),
        };
        _ = plr.load_sink(lib, 0);
        plr
    }

    /// Sets playing state based on the given bool value
    pub fn play(&mut self, play: bool) -> Result<(), Error> {
        self.sink.play(play)?;
        self.state = PlayState::play(play);
        Ok(())
    }

    /// Plays previous nth song
    pub fn prev(
        &mut self,
        lib: &Library,
        num: Option<usize>,
    ) -> Result<(), Error> {
        let cur = self.playlist.prev(num.unwrap_or(1));
        self.play_on(lib, cur)
    }

    /// Plays next nth song
    pub fn next(
        &mut self,
        lib: &Library,
        num: Option<usize>,
    ) -> Result<(), Error> {
        let cur = self.playlist.next(num.unwrap_or(1));
        self.play_on(lib, cur)
    }
}

impl Player {
    /// Plays song on given id
    fn play_on(
        &mut self,
        lib: &Library,
        id: Option<usize>,
    ) -> Result<(), Error> {
        match id {
            Some(id) => self.load_sink(lib, id)?,
            None => {
                self.state = PlayState::Stopped;
                self.sink.hard_pause()?;
            }
        }
        Ok(())
    }

    /// Loads the song on given id to the sink
    fn load_sink(&mut self, lib: &Library, id: usize) -> Result<(), Error> {
        let file = File::open(lib[id].path())?;
        let src = Symph::try_new(file, &self.symph)?;

        self.sink.load(src, self.state.is_playing())?;
        Ok(())
    }
}
