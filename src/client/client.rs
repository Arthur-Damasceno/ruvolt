use crate::{websocket::WebSocketClient, Result};

/// API wrapper to interact with Revolt.
#[derive(Debug)]
pub struct Client {
    ws_client: WebSocketClient,
}

impl Client {
    /// Create a new client and connect to the server.
    pub async fn new() -> Result<Self> {
        let ws_client = WebSocketClient::connect("wss://ws.revolt.chat").await?;

        Ok(Self { ws_client })
    }
}
