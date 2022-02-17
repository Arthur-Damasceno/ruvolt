use {
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream},
};

use crate::Result;

const REVOLT_WS_API: &str = "wss://ws.revolt.chat";

#[derive(Debug)]
pub struct WebSocketClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketClient {
    pub async fn connect() -> Result<Self> {
        let (stream, _) = connect_async(REVOLT_WS_API).await?;

        Ok(Self { stream })
    }
}
