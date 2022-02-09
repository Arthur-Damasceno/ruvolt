use {
    futures_util::SinkExt,
    tokio::{net::TcpStream, sync::Mutex},
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{models::events::ClientToServerEvent, Result};

pub const REVOLT_WS_API: &str = "wss://ws.revolt.chat";

#[derive(Debug)]
pub struct WebSocketClient(Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>);

impl WebSocketClient {
    pub async fn connect() -> Result<Self> {
        let (stream, _) = connect_async(REVOLT_WS_API).await?;

        Ok(Self(Mutex::new(stream)))
    }

    pub async fn publish(&self, event: ClientToServerEvent) -> Result {
        let text = serde_json::to_string(&event).unwrap();

        self.send(text).await?;

        Ok(())
    }

    async fn send(&self, text: String) -> Result {
        self.0.lock().await.send(Message::Text(text)).await?;

        Ok(())
    }
}
