use std::net::IpAddr;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{core::audio::PlayMsg, error::Error};

pub struct Client;

impl Client {
    /// Sends given message to the server
    #[tokio::main]
    pub async fn send(
        ip: IpAddr,
        port: u16,
        msgs: Vec<PlayMsg>,
    ) -> Result<(), Error> {
        let mut stream = TcpStream::connect(format!("{ip}:{port}")).await?;

        let json = serde_json::to_string(&msgs)?;
        stream.write_all(format!("{json}\n").as_bytes()).await?;
        Ok(())
    }
}
