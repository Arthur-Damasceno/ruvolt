use std::sync::Arc;

use crate::{
    error::Error,
    http::HttpClient,
    models::events::{ClientToServerEvent, ServerToClientEvent},
    websocket::WebSocketClient,
    Context, EventHandler, EventHandlerExt, Result,
};

/// API wrapper to interact with Revolt.
#[derive(Debug)]
pub struct Client<T: EventHandler> {
    event_handler: Arc<T>,
    ws_client: WebSocketClient,
    http_client: Arc<HttpClient>,
    token: String,
}

impl<T: EventHandler> Client<T> {
    /// Create a new client and connect to the server.
    pub async fn new(event_handler: T, token: impl Into<String>) -> Result<Self> {
        let ws_client = WebSocketClient::connect().await?;
        let token = token.into();
        let http_client = Arc::new(HttpClient::new(&token));

        Ok(Self {
            event_handler: Arc::new(event_handler),
            http_client,
            token,
            ws_client,
        })
    }

    /// Start listening for server events.
    pub async fn listen(&mut self) -> Result {
        self.authenticate().await?;

        let cx = Context::new(self.http_client.clone());

        loop {
            if let Err(err) = self.ws_client.check_heartbeat().await {
                self.event_handler.error(err).await;
            }

            if let Some(event) = self.ws_client.accept().await {
                let event_handler = self.event_handler.clone();
                let cx = cx.clone();

                tokio::spawn(async move {
                    match event {
                        Ok(event) => event_handler.handle(cx, event).await,
                        Err(err) => event_handler.error(err).await,
                    }
                });
            } else {
                return Ok(());
            }
        }
    }

    async fn authenticate(&mut self) -> Result {
        self.ws_client
            .send(ClientToServerEvent::Authenticate {
                token: self.token.clone(),
            })
            .await?;

        let event = self.ws_client.accept().await.ok_or(Error::Unknown(
            "The server closed the connection unexpectedly".into(),
        ))??;

        match event {
            ServerToClientEvent::Authenticated => Ok(()),
            ServerToClientEvent::Error { error } => Err(error.into()),
            event => Err(Error::Unknown(format!(
                "Unexpected event after authentication: {:?}",
                event,
            ))),
        }
    }
}
