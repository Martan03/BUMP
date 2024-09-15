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
    volume: f32,
    mute: bool,
    sink: Sink,
    symph: SymphOptions,
}

impl Player {
    pub fn new(lib: &Library) -> Self {
        let mut plr = Self {
            playlist: Playlist::new(0..lib.len()).current(0),
            state: PlayState::Paused,
            volume: 1.0,
            mute: false,
            sink: Default::default(),
            symph: Default::default(),
        };
        _ = plr.load_sink(lib, 0);
        plr
    }

    /// Toggles playback state when play None, else set it based on the value
    pub fn play_pause<T>(&mut self, play: T) -> Result<(), Error>
    where
        T: Into<Option<bool>>,
    {
        let play = play.into().unwrap_or(!self.state.is_playing());
        self.sink.play(play)?;
        self.state = PlayState::play(play);
        Ok(())
    }

    /// Shuffles the current playlist
    pub fn shuffle(&mut self) {
        self.playlist.shuffle();
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

    /// Sets playback volume to given value
    pub fn volume(&mut self, volume: f32) -> Result<(), Error> {
        let volume = volume.clamp(0.0, 1.0);
        self.sink.volume(volume)?;
        self.volume = volume;
        Ok(())
    }

    /// Mutes/unmutes the playback
    pub fn mute<T>(&mut self, mute: T) -> Result<(), Error>
    where
        T: Into<Option<bool>>,
    {
        let mute: bool = mute.into().unwrap_or(!self.mute);
        let volume = mute.then_some(0.0).unwrap_or(self.volume);
        self.sink.volume(volume)?;
        self.mute = mute;
        Ok(())
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
