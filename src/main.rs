use core::app::App;
use std::{net::Ipv4Addr, process};

use args::Args;
use error::Error;
use server::{Client, Server};
use termint::{enums::Color, widgets::StrSpanExtension};
use tokio::sync::mpsc::unbounded_channel;

mod args;
mod core;
mod error;
mod server;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {e}", "Error:".fg(Color::Red));
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(std::env::args())?;
    if args.actions.is_empty() {
        run_app();
        return Ok(());
    }

    Client::send(args.ip, args.port, args.actions)
}

#[tokio::main]
async fn run_app() {
    let (sender, receiver) = unbounded_channel();

    let mut app = App::new(receiver);

    let server = Server::new(sender);
    tokio::spawn(async move {
        server.start(Ipv4Addr::new(127, 0, 0, 1), 8080).await;
    });

    app.run().await;
}
