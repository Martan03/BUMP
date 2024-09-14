use tokio::sync::oneshot;

use super::audio::PlayMsg;

#[allow(clippy::module_inception)]
mod server;

pub use server::*;

/// Wraps the message received on the server and adds sender for the response
pub struct RecvMsg {
    pub cmd: PlayMsg,
    pub sender: oneshot::Sender<String>,
}

impl RecvMsg {
    /// Creates new [`RecvMsg`]
    pub fn new(cmd: PlayMsg, sender: oneshot::Sender<String>) -> Self {
        Self { cmd, sender }
    }
}
