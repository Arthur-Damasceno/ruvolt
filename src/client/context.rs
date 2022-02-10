use std::sync::Arc;

use crate::{
    http::HttpClient,
    models::{events::ClientToServerEvent, User},
    websocket::WebSocketClient,
    Result,
};

/// A struct for general utilities and wrapper for the http client for fetch entities from the API.
#[derive(Debug, Clone)]
pub struct Context {
    ws_client: Arc<WebSocketClient>,
    /// A http client.
    pub http_client: Arc<HttpClient>,
    /// The current user.
    pub user: User,
}

impl Context {
    pub(crate) fn new(
        ws_client: Arc<WebSocketClient>,
        http_client: Arc<HttpClient>,
        user: User,
    ) -> Self {
        Self {
            ws_client,
            http_client,
            user,
        }
    }

    /// Send an event to the server.
    ///
    /// # Panics
    ///
    /// Panics if the event is [Ping](ClientToServerEvent::Ping) or [Authenticate](ClientToServerEvent::Authenticate).
    pub async fn send(&self, event: ClientToServerEvent) -> Result {
        match event {
            ClientToServerEvent::Ping { .. } | ClientToServerEvent::Authenticate { .. } => {
                panic!("{:?} event is handled by the client", event);
            }
            event => self.ws_client.publish(event).await,
        }
    }

    /// Close the websocket connection.
    pub async fn close(&self) -> Result {
        self.ws_client.close().await?;

        Ok(())
    }
}
