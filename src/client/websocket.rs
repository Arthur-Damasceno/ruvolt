use {
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream},
};

use crate::Result;

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct WebSocketClient {
    stream: Stream,
}

impl WebSocketClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;

        Ok(Self { stream })
    }
}
