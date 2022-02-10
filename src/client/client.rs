use {std::sync::Arc, tokio::task};

use crate::{
    error::Error,
    http::HttpClient,
    models::events::{ClientToServerEvent, ServerToClientEvent},
    websocket::{self, WebSocketClient},
    Context, EventHandler, EventHandlerExt, Result,
};

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
    pub async fn listen(self, token: &str) -> Result {
        self.authenticate(token).await?;

        let http_client = Arc::new(HttpClient::new(token));
        let user = http_client.get("users/@me").await?;
        let cx = Context::new(self.ws_client.clone(), http_client, user);

        websocket::heartbeat(self.ws_client.clone());

        while let Some(event) = self.ws_client.accept().await {
            let event_handler = self.event_handler.clone();
            let cx = cx.clone();

            task::spawn(async move {
                match event {
                    Ok(event) => event_handler.handle(cx, event).await,
                    Err(err) => event_handler.error(err).await,
                }
            });
        }

        Ok(())
    }

    async fn authenticate(&self, token: &str) -> Result {
        self.ws_client
            .publish(ClientToServerEvent::auth(token))
            .await?;

        let event = self
            .ws_client
            .accept()
            .await
            .expect("The server closed the connection unexpectedly")?;

        match event {
            ServerToClientEvent::Authenticated => Ok(()),
            ServerToClientEvent::Error { error } => Err(Error::from(error)),
            event => Err(Error::Unknown(format!(
                "Unexpected event after authentication: {:?}",
                event,
            ))),
        }
    }
}
