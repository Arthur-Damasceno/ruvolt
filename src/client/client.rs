use crate::{
    entities::{ClientToServerEvent, ServerToClientEvent},
    error::Error,
    websocket::WebSocketClient,
    Result,
};

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
