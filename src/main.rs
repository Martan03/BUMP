use core::{app::App, server::Server};
use std::net::Ipv4Addr;

use tokio::sync::mpsc::unbounded_channel;

mod core;
mod error;

fn main() {
    run();
}

#[tokio::main]
async fn run() {
    let (sender, receiver) = unbounded_channel();

    let mut app = App::new(receiver);

    let server = Server::new(sender);
    tokio::spawn(async move {
        server.start(Ipv4Addr::new(127, 0, 0, 1), 8080).await;
    });

    app.run().await;
}
