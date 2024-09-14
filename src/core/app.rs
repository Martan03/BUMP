use std::path::PathBuf;

use tokio::sync::mpsc::UnboundedReceiver;

use super::{
    audio::{player::Player, PlayMsg},
    library::Library,
    server::RecvMsg,
};

pub struct App {
    player: Player,
    lib: Library,
    msg_queue: UnboundedReceiver<RecvMsg>,
}

impl App {
    /// Creates new [`App`]
    pub fn new(msg_queue: UnboundedReceiver<RecvMsg>) -> Self {
        let lib = Library::load(&PathBuf::from("."));
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
        match msg.cmd {
            PlayMsg::Play => _ = self.player.play(true),
            PlayMsg::Pause => _ = self.player.play(false),
            PlayMsg::PlayPause => todo!(),
            PlayMsg::Prev => _ = self.player.prev(&self.lib, None),
            PlayMsg::Next => _ = self.player.next(&self.lib, None),
        }
    }
}
