use std::net::IpAddr;

use serde_json::from_slice;
use tokio::{
    io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, WriteHalf},
    net::{TcpListener, TcpStream},
    sync::{mpsc::UnboundedSender, oneshot},
};

use crate::core::audio::PlayMsg;

use super::RecvMsg;

pub struct Server {
    sender: UnboundedSender<RecvMsg>,
}

impl Server {
    /// Creates new [`Server`]
    pub fn new(sender: UnboundedSender<RecvMsg>) -> Self {
        Self { sender }
    }

    /// Starts the server on the given ip and port number
    pub async fn start<T>(&self, ip: T, port: u16)
    where
        T: Into<IpAddr>,
    {
        let addr = format!("{}:{}", ip.into(), port);
        let listener = TcpListener::bind(&addr).await.unwrap();
        println!("Server listening on: {}", addr);

        loop {
            let Ok((stream, _)) = listener.accept().await else {
                continue;
            };
            let sender = self.sender.clone();
            tokio::spawn(Self::handle_client(stream, sender));
        }
    }
}

impl Server {
    /// Handles the client
    async fn handle_client(
        stream: TcpStream,
        sender: UnboundedSender<RecvMsg>,
    ) {
        let (reader, mut writer) = split(stream);
        let mut reader = BufReader::new(reader);
        let mut buffer = Vec::new();

        loop {
            match reader.read_until(b'\n', &mut buffer).await {
                Ok(0) => break,
                Ok(_) => {
                    Self::handle_client_recv(&mut writer, &sender, &mut buffer)
                        .await
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {e}");
                    break;
                }
            }
        }
    }

    /// Handles the message received from the client
    async fn handle_client_recv(
        stream: &mut WriteHalf<TcpStream>,
        sender: &UnboundedSender<RecvMsg>,
        buffer: &mut Vec<u8>,
    ) {
        let msgs = match from_slice::<Vec<PlayMsg>>(buffer) {
            Ok(msg) => msg,
            Err(e) => {
                buffer.clear();
                eprintln!("{e}");
                return;
            }
        };

        buffer.clear();
        for msg in msgs {
            let (res_sender, res_recv) = oneshot::channel();
            sender.send(RecvMsg::new(msg, res_sender)).unwrap();

            if let Ok(res) = res_recv.await {
                stream.write_all(res.as_bytes()).await.unwrap();
            }
        }
    }
}
