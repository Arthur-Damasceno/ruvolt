use {
    futures_util::{SinkExt, StreamExt},
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{
    entities::{ClientToServerEvent, ServerToClientEvent},
    error::Error,
    Result,
};

#[derive(Debug)]
pub struct WebSocketClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
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

    pub async fn recv(&mut self) -> Result<ServerToClientEvent> {
        let msg = self.stream.next().await.unwrap()?;

        let event = match msg {
            Message::Text(msg) => serde_json::from_str(&msg).map_err(|err| {
                Error::Unknown(format!(
                    "Cannot deserialize unexpected event from server: {}",
                    err
                ))
            })?,
            _ => unimplemented!(),
        };

        Ok(event)
    }
}
