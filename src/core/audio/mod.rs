use serde::{Deserialize, Serialize};

pub mod player;

/// Represents state of the playback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayState {
    // No song is loaded nor playing
    #[default]
    Stopped,
    // Song is playing
    Playing,
    // Song is loaded but paused
    Paused,
}

impl PlayState {
    /// Gets state based on the bool value
    pub fn play(play: bool) -> Self {
        if play {
            Self::Playing
        } else {
            Self::Paused
        }
    }

    /// Returns true when playing state
    pub fn is_playing(&self) -> bool {
        self == &Self::Playing
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PlayMsg {
    Play(Option<bool>),
    Next(Option<usize>),
    Prev(Option<usize>),
    Volume(f32),
    Mute,
    Shuffle,
}
