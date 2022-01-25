use std::sync::Arc;

use crate::{
    entities::{ClientToServerEvent, ServerToClientEvent},
    error::Error,
    websocket::WebSocketClient,
    EventHandler, Result,
};

/// API wrapper to interact with Revolt.
pub struct Client<T: EventHandler> {
    event_handler: Arc<T>,
    ws_client: WebSocketClient,
}

impl<T: EventHandler> Client<T> {
    /// Create a new client and connect to the server.
    pub async fn new(event_handler: T) -> Result<Self> {
        let ws_client = WebSocketClient::connect("wss://ws.revolt.chat").await?;

        Ok(Self {
            event_handler: Arc::new(event_handler),
            ws_client,
        })
    }

    async fn authenticate(&mut self, token: String) -> Result {
        self.ws_client
            .send(ClientToServerEvent::Authenticate {
                token: token.clone(),
            })
            .await?;

        let event = self.ws_client.recv().await?;

        match event {
            ServerToClientEvent::Authenticated => Ok(()),
            ServerToClientEvent::Error { error } => Err(Error::Authentication(error)),
            event => Err(Error::Unknown(format!(
                "Unexpected event after authentication: {:?}",
                event,
            ))),
        }
    }
}
