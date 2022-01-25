use {
    futures_util::SinkExt,
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{entities::ClientToServerEvent, Result};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct WebSocketClient {
    stream: Stream,
}

impl WebSocketClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;

        Ok(Self { stream })
    }

    pub async fn send(&mut self, event: ClientToServerEvent) -> Result {
        let msg = Message::Text(serde_json::to_string(&event).unwrap());

        self.stream.send(msg).await?;

        Ok(())
    }
}
