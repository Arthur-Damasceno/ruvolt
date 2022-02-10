use std::sync::Arc;

use crate::{websocket::WebSocketClient, EventHandler, Result};

/// API wrapper to interact with Revolt.
#[derive(Debug)]
pub struct Client<T: EventHandler> {
    event_handler: Arc<T>,
    ws_client: Arc<WebSocketClient>,
}

impl<T: EventHandler> Client<T> {
    /// Create a new client and connect to the server.
    pub async fn new(event_handler: T) -> Result<Self> {
        let ws_client = WebSocketClient::connect().await?;

        Ok(Self {
            event_handler: Arc::new(event_handler),
            ws_client: Arc::new(ws_client),
        })
    }

    /// Start listening for server events.
    pub async fn listen(self, _token: impl Into<String>) -> Result {
        Ok(())
    }
}
