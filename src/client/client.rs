use {
    std::{sync::Arc, time::Duration},
    tokio::time::interval,
};

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
        let token = token.into();
        let ws_client = WebSocketClient::connect().await?;
        let http_client = Arc::new(HttpClient::new(&token));

        Ok(Self {
            event_handler: Arc::new(event_handler),
            ws_client,
            http_client,
            token,
        })
    }

    /// Start listening for server events.
    pub async fn listen(&mut self) -> Result {
        self.authenticate().await?;

        let user = self.http_client.get("users/@me").await?;
        let cx = Context::new(self.http_client.clone(), user);
        let mut heartbeat_interval = interval(Duration::from_secs(20));

        while let Some(event) = self
            .ws_client
            .accept_or_heartbeat(&mut heartbeat_interval)
            .await
        {
            let cx = cx.clone();
            let event_handler = self.event_handler.clone();

            tokio::spawn(async move {
                match event {
                    Ok(event) => event_handler.handle(cx, event).await,
                    Err(err) => event_handler.error(err).await,
                }
            });
        }

        Ok(())
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
            ServerToClientEvent::Error { error } => Err(Error::from(error)),
            event => Err(Error::Unknown(format!(
                "Unexpected event after authentication: {:?}",
                event
            ))),
        }
    }
}
