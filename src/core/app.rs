use tokio::sync::mpsc::UnboundedReceiver;

use crate::server::RecvMsg;

use super::{
    audio::{player::Player, PlayMsg},
    config::Config,
    library::Library,
};

pub struct App {
    player: Player,
    lib: Library,
    msg_queue: UnboundedReceiver<RecvMsg>,
}

impl App {
    /// Creates new [`App`]
    pub fn new(msg_queue: UnboundedReceiver<RecvMsg>) -> Self {
        let config = Config::load();
        let lib = Library::load(&config);
        Self {
            player: Player::new(&lib),
            lib,
            msg_queue,
        }
    }

    /// Runs the main loop of the [`App`]
    pub async fn run(&mut self) {
        while let Some(msg) = self.msg_queue.recv().await {
            self.handle_msg(msg).await;
        }
    }

    /// Handles message received from the server
    async fn handle_msg(&mut self, msg: RecvMsg) {
        _ = match msg.cmd {
            PlayMsg::Play => self.player.play_pause(true),
            PlayMsg::Pause => self.player.play_pause(false),
            PlayMsg::PlayPause => self.player.play_pause(None),
            PlayMsg::Prev => self.player.prev(&self.lib, None),
            PlayMsg::Next => self.player.next(&self.lib, None),
            PlayMsg::Volume(vol) => self.player.volume(vol),
            PlayMsg::Mute => self.player.mute(None),
            PlayMsg::Shuffle => Ok(self.player.shuffle()),
        };
    }
}
